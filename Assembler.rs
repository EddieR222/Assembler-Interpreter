use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct AssemblerInterpreter {
    registers: Vec<Register>,
}

impl AssemblerInterpreter {
    pub fn new() -> AssemblerInterpreter {
        let registers = Vec::new();
        let assembler = AssemblerInterpreter { registers: registers };
        return assembler;
    }

    pub fn interpret(input: &str) -> Option<String> {
        // println!("Given Input: {}", input);
        let mut tokens: Vec<&str> = input.split("\n").collect();
        tokens.retain(|s| s != &"");
        // println!("Tokens: {:?}", tokens);
        let mut assembler = AssemblerInterpreter::new();
    

        let mut pc: usize = 0;
        let mut code_and_message: Vec<&str> = Vec::new();
        let mut message: &str = "0";
        let mut function_call: &str = "0";
        let mut compared_values: Vec<&str> = Vec::new();
        let mut split_tokens: Vec<&str> = Vec::new();
        let mut pc_returns: Vec<usize> = Vec::new();

        let mut labels: HashMap<&str, usize> = HashMap::new();
        let mut final_msg: String = String::new();

        //Printing assembly pretty
        let mut j: usize = 0;
        for token in tokens.iter() {
            code_and_message = token.split(";").collect();
            if !code_and_message[0].contains(|c:char| c.is_alphabetic()) {
                continue;
            }
            if code_and_message[0].contains(":") && !code_and_message[0].contains("msg") {
                // println!("{}", code_and_message[0]);
                continue;
            }
            // println!("{}:{}", j, token.trim());
            j += 1;
        }








        let mut i: usize = 0;
        while i < tokens.len() {
            code_and_message = tokens[i].split(";").collect();
            if code_and_message[0].contains(":") && !code_and_message[0].contains(&"msg") {
                labels.insert(code_and_message[0].trim().trim_end_matches(":"), i);
                tokens.remove(i);
                continue;
            }
            if !code_and_message[0].contains(|c:char| c.is_alphabetic()) {
                tokens.remove(i);
                continue;
            }
            i += 1;
        }
        // println!("Labels: {:?}", labels);
        // println!("Tokens Now: {:?}", tokens);

    


        while !tokens[pc].contains("end") {
            
            code_and_message = tokens[pc].split(";").collect();
            // println!("Comments: {:?}", code_and_message);
            if code_and_message.len() > 1 {
                // If length > 1, we have a message (always on the right)
                message = code_and_message[1];
            }
            function_call = code_and_message[0];
            if function_call == "" { pc +=  1; continue;} // if only message, skip
            // Now that we have a function, we need to see the function call
            split_tokens = function_call.split_ascii_whitespace().collect();
            match split_tokens[0] {
                "inc" => {assembler.inc(split_tokens[1]);}
                "dec" => {assembler.dec(split_tokens[1]);}
                "add" => {assembler.add(split_tokens[1].trim_end_matches(","), split_tokens[2]);}
                "sub" => {assembler.sub(split_tokens[1].trim_end_matches(","), split_tokens[2]);}
                "div" => {assembler.div(split_tokens[1].trim_end_matches(","), split_tokens[2]);}
                "mul" => {assembler.mul(split_tokens[1].trim_end_matches(","), split_tokens[2]);}
                "mov" => {assembler.mov(split_tokens[1].trim_end_matches(","), split_tokens[2]);}
                "jmp" => {
                    pc = *labels.get(split_tokens[1]).unwrap();
                    continue;
                }
                "cmp" => {
                    compared_values.push(split_tokens[1].trim_end_matches(","));
                    compared_values.push(split_tokens[2]);
                }
                "jne" => {
                    let (x, y) = assembler.get_compared_values(&mut compared_values);
                    if x != y {
                        pc = *labels.get(split_tokens[1]).unwrap();
                        continue;
                    }
                }
                "je" => {
                    let (x, y) = assembler.get_compared_values(&mut compared_values);
                    if x == y {
                        pc = *labels.get(split_tokens[1]).unwrap();
                        continue;
                    }
                }
                "jge" => {
                    let (x, y) = assembler.get_compared_values(&mut compared_values);
                    if x >= y {
                        pc = *labels.get(split_tokens[1]).unwrap();
                        continue;
                    }
                }
                "jg" => {
                    let (x, y) = assembler.get_compared_values(&mut compared_values);
                    if x >= y {
                        pc = *labels.get(split_tokens[1]).unwrap();
                        continue;
                    }
                }
                "jle" => {
                    let (x, y) = assembler.get_compared_values(&mut compared_values);
                    if x <= y {
                        pc = *labels.get(split_tokens[1]).unwrap();
                        continue;
                    }
                }
                "jl" => {
                    let (x, y) = assembler.get_compared_values(&mut compared_values);
                    if x < y {
                        pc = *labels.get(split_tokens[1]).unwrap();
                        continue;
                    }
                }
                "call" => {
                    pc += 1;
                    pc_returns.push(pc);
                    pc = *labels.get(split_tokens[1]).unwrap();
                    continue;
                }
                "msg" => {
                    let function_call_t = function_call.trim();
                    final_msg = assembler.prepare_msg(function_call_t);
                    
                }
                "ret" => {
                    let return_pc = pc_returns.pop();
                    if return_pc.is_none() {
                        continue;
                    } else {
                        pc = return_pc.unwrap();
                        continue;
                    }
                }
                _ => {println!("unknown function");}
            } // end of match statement
            // Once we reach here, we have gone through and performed the current function call
            pc += 1;
            if pc > tokens.len() - 1 {
                return None;
            }
        } // end of while loop
        // Once we reach here, we have reached the end command
        return Some(final_msg.trim().to_string());
    }

    pub fn mov(&mut self, x: &str, y: &str) {
        /* 
        x - registers to copy value into
        y - value or value of register to copy into register x
        ==========Function Equivalent============
                        x = y
        =========================================
        */

        // First we search to see if register already exists, if not then we create one
        // But if it does exist, we just replace it's value with the value in the input
        let register = self.find_register(x.trim());
        if register.is_none() { 
            let mut value: f64 = 0.0;
            let possible_register = self.find_register(y.trim());
            if possible_register.is_none() {
                value = y.parse::<f64>().unwrap();
            } else {
                value = possible_register.unwrap().value;
            }
            let new_register = Register::new(x.to_string(), value);
            self.registers.push(new_register);
        } else {
            let mut y_value: &str= "0";
            let possible_register = self.find_register(y.trim());
            if possible_register.is_none() {
                self.replace_value(x, y_value);
            } else {
                let y_f64  = possible_register.unwrap().value;
                self.replace_value(x, &y_f64.to_string());
            }
        }
    }

    pub fn inc(&mut self, register: &str) {
        /* 
        x - register to increment by one
        =========Function Equivalent===========
                        x += 1
        ======================================
        */
        self.update_value(register, 1.0, "+");
    }

    pub fn dec(&mut self, register: &str) {
        /* 
        x - register to decrement by one
        =========Function Equivalent===========
                        x -= 1
        =======================================
        */
        self.update_value(register, 1.0, "-");
    }

    pub fn add(&mut self, register: &str, y: &str) {
        /* 
        x - register to add y to
        y - value or value of register to add by
        =========Function Equivalent===========
                        x += y
        =======================================
        */
        let possible_register = self.find_register(y.trim());
        if possible_register.is_none() {
            let value_f64 = y.parse::<f64>().unwrap();
            self.update_value(register, value_f64, "+");
        } else {
            let confirmed_register = possible_register.unwrap();
            self.update_value(register, confirmed_register.value, "+");
        }
    }

    pub fn sub(&mut self, register: &str, y: &str) {
        /* 
        x - register to sub y to
        y - value or value of register to sub by
        =========Function Equivalent===========
                        x -= y
        =======================================
        */
        let possible_register = self.find_register(y.trim());
        if possible_register.is_none() {
            let value_f64 = y.parse::<f64>().unwrap();
            self.update_value(register, value_f64, "-");
        } else {
            let confirmed_register = possible_register.unwrap();
            self.update_value(register, confirmed_register.value, "-");
        }
    }

    pub fn mul(&mut self, register: &str, y: &str) {
        /* 
        x - register to mul y to
        y - value or value of register to mul by
        =========Function Equivalent===========
                        x *= y
        =======================================
        */
        let possible_register = self.find_register(y.trim());
        if possible_register.is_none() {
            let value_f64 = y.parse::<f64>().unwrap();
            self.update_value(register, value_f64, "*");
        } else {
            let confirmed_register = possible_register.unwrap();
            self.update_value(register, confirmed_register.value, "*");
        }
    }

    pub fn div(&mut self, register: &str, y: &str) {
        /* 
        x - register to divide y to
        y - value or value of register to divide by
        =========Function Equivalent===========
                        x -= y
        =======================================
        */
        let possible_register = self.find_register(y.trim());
        if possible_register.is_none() {
            let value_f64 = y.parse::<f64>().unwrap();
            self.update_value(register, value_f64, "/");
        } else {
            let confirmed_register = possible_register.unwrap();
            self.update_value(register, confirmed_register.value, "/");
        }
    }

    pub fn find_register(&self, x: &str) -> Option<Register> {
        let registers = self.registers.clone();
        for register in &registers {
            if register.name == x { 
                return Some(register.clone());
            }
        }
        return None;
    }

    pub fn replace_value(&mut self, register: &str, value:&str) {
        for mut reg in self.registers.iter_mut() {
            if reg.name == register {
                reg.value = value.parse::<f64>().unwrap();
            }
        }
    }

    pub fn update_value(&mut self, register: &str, value: f64, operator:&str) {
        for mut reg in self.registers.iter_mut() {
            if reg.name == register {
                match operator {
                    "+" => { reg.value += value; }
                    "-" => { reg.value -= value; }
                    "/" => { reg.value /= value; }
                    "*" => { reg.value *= value; }
                    _ => { println!("Something Went Wrong"); }
                }
            }
        }
    }

    pub fn get_compared_values(&self, compared_values: &mut Vec<&str>) -> (f64, f64) {
        let mut y:f64 = 0.0;
        let mut x:f64 = 0.0;

        let y_str = compared_values.pop().unwrap();
        let possible_register = self.find_register(y_str);
        if possible_register.is_none() {
            y = y_str.parse::<f64>().unwrap();
        } else {
            y = possible_register.unwrap().value;
        }
        let x_str = compared_values.pop().unwrap();
        let possible_register = self.find_register(x_str);
        if possible_register.is_none() {
            x = x_str.parse::<f64>().unwrap();
        } else {
            x = possible_register.unwrap().value;
        }
        return (x, y);
    
    }

    pub fn prepare_msg(&self, msg: &str) -> String{
        let function_call_t = msg.trim();
        let without_msg = function_call_t.trim_start_matches("msg ");
        let pure_msg = without_msg.trim();
        let mut final_msg = "".to_string();
        let mut buffering = false;
        let mut register_buffering = true;
        let mut buffer = String::new();
        let mut register = String::new();
        for c in pure_msg.chars() {
            if c == '\'' && !buffering {
                register_buffering = false;
                buffering = true;
                continue;
            }
            if c == '\'' && buffering {
                buffering = false;
                final_msg += &buffer;
                buffer.drain(..);
                continue;
            }
            if buffering{
                buffer.push(c);
                continue;
            }
            if c == ',' && !buffering && !register_buffering {
                register_buffering = true;
                continue;
            }
            if c == ',' && !buffering && register_buffering {
                register_buffering = false;
                let possible_register = self.find_register(register.trim());
                if possible_register.is_none() {
                    final_msg += &register.trim();
                } else {
                    final_msg += &possible_register.unwrap().value.to_string();
                }
                register.drain(..);
            }
            if register_buffering {
                register.push(c);
                continue;
            }
        } // end of for loop
        if !register.is_empty() {
            let possible_register = self.find_register(register.trim());
                if possible_register.is_none() {
                    final_msg += &register.trim();
                } else {
                    final_msg += &possible_register.unwrap().value.to_string();
                }
                register.drain(..);
        }
        if !buffer.is_empty() {
            final_msg += &buffer;
        }
        return final_msg;
    }

}


#[derive(Debug, Clone, PartialEq)]
pub struct Register {
    pub name: String,
    pub value: f64,
}

impl Register {
    pub fn new(name: String, value: f64) -> Register {
        let new_register = Register { name: name, value: value };
        return new_register;
    }

}

pub fn count_million(mut starting_num: u128) -> u128 {
    while starting_num < 18446744073709551315 {
        starting_num += 1;
    }
    return starting_num;
}
