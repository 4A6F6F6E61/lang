#![cfg(test)]

#[test]
fn lexer_new() -> () {
    use crate::lexer::Lexer;
    use std::fs::read_to_string;

    let mut lexer = Lexer::new();
    let code = read_to_string("./testing.lang").expect("Should have been able to read the file");
    lexer.parse(code);
    println!("{:#?}", lexer.ast);
}

#[test]
fn cxx() {
    use crate::{log, printx, transpiler::Cxx, PrintT};

    let mut cxx = Cxx::new();
    cxx.run("./testing.lang");
    log!(Info, f("\n{}", cxx.buffer));
}
