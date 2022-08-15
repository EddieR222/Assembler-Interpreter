mod Assembler;
use Assembler::*;

fn main() {
    
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn function_calls_work() {
        let result = 
        AssemblerInterpreter::interpret(
            "\n; My first program\n
            mov  a, 5\n
            inc  a\n
            call function\n
            msg  '(5+1)/2 = ', a    ; output message\n
            end\n
            \n
            function:\n  
            div  a, 2\n    
            ret\n");
        assert_eq!(result.unwrap(), "(5+1)/2 = 3");
    }

    #[test]
    fn function_calls_work_part_2() {
        let result = 
        AssemblerInterpreter::interpret(
            "\n
            mov   a, 5\n
            mov   b, a\n
            mov   c, a\n
            call  proc_fact\n
            call  print\n
            end\n
            \n
            proc_fact:\n    
            dec   b\n    
            mul   c, b\n    
            cmp   b, 1\n    
            jne   proc_fact\n    
            ret\n
            \n
            print:\n    
            msg   a, '! = ', c ; output text\n    
            ret\n"
        );
        assert_eq!(result.unwrap(), "5! = 120");
    }

    #[test]
    fn fib_series() {
        let result = 
        AssemblerInterpreter::interpret(
            "\n
            mov   a, 8            ; value\n
            mov   b, 0            ; next\n
            mov   c, 0            ; counter\n
            mov   d, 0            ; first\n
            mov   e, 1            ; second\n
            call  proc_fib\n
            call  print\n
            end\n
            \n
            proc_fib:\n    
            cmp   c, 2\n    
            jl    func_0\n    
            mov   b, d\n    
            add   b, e\n    
            mov   d, e\n    
            mov   e, b\n    
            inc   c\n    
            cmp   c, a\n    
            jle   proc_fib\n    
            ret\n
            \n
            func_0:\n   
            mov   b, c\n    
            inc   c\n    
            jmp   proc_fib\n
            \n
            print:\n    
            msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text\n    
            ret\n",
        );
        assert_eq!(result.unwrap(), "Term 8 of Fibonacci series is: 21");
    }

    #[test]
    fn mod_assembly() {
        let result = 
        AssemblerInterpreter::interpret(
            "\nmov   a, 11           ; 
            value1\nmov   b, 3            ; 
            value2\n
            call  mod_func\n
            msg   'mod(', a, ', ', b, ') = ', d        ; output\n
            end\n\n; Mod function\n
            mod_func:\n    
            mov   c, a        ; temp1\n    
            div   c, b\n    
            mul   c, b\n    
            mov   d, a        ; temp2\n    
            sub   d, c\n    
            ret\n"
        );
        assert_eq!(result.unwrap(), "mod(11, 3) = 0");
    }

    #[test]
    fn null_return() {
        let result = 
        AssemblerInterpreter::interpret("\n
        call  func1\n
        call  print\n
        end\n\n
        func1:\n    
        call  func2\n    
        ret\n
        \n
        func2:\n    
        ret\n\n
        print:\n    
        msg 'This program should return null'\n");
        assert_eq!(result, None);
    }

    #[test]
    fn testing_more() {
        let result = 
        AssemblerInterpreter::interpret(
            "\n
            mov   a, 2            ; value1\n
            mov   b, 10           ; value2\n
            mov   c, a            ; temp1\n
            mov   d, b            ; temp2\n
            call  proc_func\n
            call  print\n
            end\n\n
            proc_func:\n    
            cmp   d, 1\n    
            je    continue\n    
            mul   c, a\n    
            dec   d\n    
            call  proc_func\n\n
            continue:\n    
            ret\n\n
            print:\n    
            msg a, '^', b, ' = ', c\n    
            ret\n"
        );
        assert_eq!(result.unwrap(), "2^10 = 1024");

    }
}
