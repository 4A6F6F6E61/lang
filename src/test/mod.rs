#![cfg(test)]

mod general {
    use crate::lexer::Lexer;
    #[test]
    fn lexer_new() -> () {
        use std::fs::read_to_string;

        let mut lexer = Lexer::new();
        let code = read_to_string("./testing.lang").expect("Should have been able to read the file");
        lexer.parse(code);
        println!("{:#?}", lexer.ast);
    }

    #[test]
    fn cxx_testing() {
        use crate::{log, printx, transpiler::Cxx, PrintT};

        let mut cxx = Cxx::new();
        cxx.run("./testing.lang");
        log!(Info, f("\n{}", cxx.buffer));
    }
}

mod cxx {
    use crate::test::{test_cxx};
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
}
use {std::{fs::read_to_string, path::Path}, crate::{log, printx, PrintT, transpiler::Cxx}};

pub fn load_file<P>(file: P) -> Option<String>
where
P: AsRef<Path>,
{
    let mut out = String::new();
    if let Ok(code) = read_to_string(file) {
        code.lines().for_each(|line| {
            out.push_str(&format!("{}\n", line.trim()));
        });
        Some(out.clone())
    } else {
        log!(CXX, "Unable to read file");
        None
    }
}

fn test_cxx(test: &str) {
    let mut cxx = Cxx::new();
    cxx.run(&format!("./src/examples/{test}.lang"));
    let mut code1 = String::from(cxx.buffer.trim());
    code1.push_str("\n");
    if let Some(code2) = load_file(&format!("./src/examples/out/cxx/{test}.cxx")) {
        assert_eq!(code1, code2);
    }
}