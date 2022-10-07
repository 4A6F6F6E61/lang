#![allow(dead_code)]

use crate::{
    lexer::{Function, Lexer, Token},
    log, printx,
    transpiler::Cxx,
    PrintT,
};
use std::{fs::read_to_string, path::Path};

impl Cxx {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn run<P>(&mut self, file: P)
    where
        P: AsRef<Path>,
    {
        let mut lexer = Lexer::new();
        if let Ok(code) = read_to_string(file) {
            lexer.parse(code);
            dbg!(&lexer.ast);
            self.transpile(lexer.ast);
        } else {
            log!(CXX, "Unable to read file");
        }
    }

    fn transpile(&mut self, ast: Vec<Token>) {
        let mut ast_iter = ast.iter().peekable();
        while ast_iter.peek().is_some() {
            let nt = ast_iter.next().unwrap();
            match nt {
                Token::Const(x) => {
                    let (name, value) = (x.name.clone(), x.value.clone());
                    self.buffer
                        .push_str(&format!("const auto {name} = {value};\n"));
                }
                Token::Global(x) => {
                    let (name, value) = (x.name.clone(), x.value.clone());
                    self.buffer.push_str(&format!("auto {name} = {value};\n"));
                }
                Token::Function(x) => {
                    self.function(x);
                }
                _ => {
                    log!(Error, "unexpected Token");
                }
            }
        }
    }
    fn function(&mut self, x: &Function) {
        // TODO: Handle Arguemnt Return types
        let (_name, _arguments, _return_type) =
            (x.name.clone(), x.arguments.clone(), x.return_type.clone());
    }
}
