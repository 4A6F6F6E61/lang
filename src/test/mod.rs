#![cfg(test)]
pub mod utils;

mod general {
    use crate::lexer::Lexer;
    #[test]
    fn lexer_new() -> () {
        use std::fs::read_to_string;

        let mut lexer = Lexer::new();
        let code =
            read_to_string("./testing.lang").expect("Should have been able to read the file");
        lexer.parse(code);
        println!("{:#?}", lexer.ast);
    }

    #[test]
    fn cxx_testing() {
        use crate::{log, printx, transpiler::*, PrintT};

        let cxx = &mut cxx::new();
        run(cxx, "./testing.lang");
        log!(Info, f("\n{}", cxx.buffer));
    }
}

mod cxx {
    use crate::test::utils::test_cxx;
    #[test]
    fn expression() {
        test_cxx("expression");
    }
    #[test]
    fn global_const() {
        test_cxx("global_const");
    }
    #[test]
    fn global_let() {
        test_cxx("global_let");
    }
    #[test]
    fn if_() {
        test_cxx("if");
    }
    #[test]
    fn let_() {
        test_cxx("let");
    }
    #[test]
    fn loop_function() {
        test_cxx("loop_function");
    }
    #[test]
    fn function() {
        test_cxx("main");
    }
    #[test]
    fn generator() {
        test_cxx("generator");
    }
}

mod functional {
    #[test]
    fn fn_cxx() {
        use crate::{log, printx, transpiler::*, PrintT};

        let t = &mut cxx::new();
        run(t, &format!("./src/examples/generator.lang"));
        log!(Info, f("\n{}", t.buffer));
    }
}
