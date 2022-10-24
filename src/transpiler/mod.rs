#![allow(dead_code)]
use {
    crate::{
        lexer::{
            token::{expression::Operator, Expression, Function},
            FunctionType, Lexer, Token,
        },
        log, printx, PrintT,
    },
    std::{fs::read_to_string, path::Path},
};

pub mod cxx;

#[derive(Clone, Debug)]
pub enum TranspilerLang {
    Cxx,
}

#[derive(Clone, Debug)]
pub struct Transpiler {
    pub buffer: String,
    pub imports: Vec<String>,
    pub target_lang: TranspilerLang,
}

pub fn run<P>(this: &mut Transpiler, file: P)
where
    P: AsRef<Path>,
{
    let mut lexer = Lexer::new();
    if let Ok(code) = read_to_string(file) {
        lexer.parse(code);
        //dbg!(&lexer.ast);
        transpile(this, lexer.ast);
    } else {
        log!(CXX, "Unable to read file");
    }
}

pub fn transpile(this: &mut Transpiler, ast: Vec<Token>) {
    let mut ast_iter = ast.iter().peekable();
    while ast_iter.peek().is_some() {
        let nt = ast_iter.next().unwrap();
        match nt {
            Token::Const(x) => {
                let (name, exp) = (x.name.clone(), x.exp.clone());
                this.buffer.push_str(&format!(
                    "const auto {name} = {};\n",
                    expression(this.clone().target_lang, exp)
                ));
            }
            Token::Global(x) => {
                let (name, exp) = (x.name.clone(), x.exp.clone());
                this.buffer.push_str(&format!(
                    "auto {name} = {};\n",
                    expression(this.clone().target_lang, exp)
                ));
            }
            Token::Function(x) => {
                function(this, x, FunctionType::Function);
            }
            Token::LoopFunction(x) => {
                function(this, x, FunctionType::Loop);
            }
            Token::GeneratorFunction(x) => {
                if !this.imports.contains(&"vector".to_string()) {
                    this.imports.push("vector".to_string());
                    this.buffer.push_str("#include <vector>\n\n");
                }
                function(this, x, FunctionType::Generator);
            }
            Token::CImport(ci) => {
                if !this.imports.contains(ci) {
                    this.buffer.push_str(&format!("#include {ci}\n"));
                    this.imports.push(ci.to_owned());
                }
            }
            Token::Comment(_) => {}
            _ => {
                log!(Error, f("unexpected Token"));
                dbg!(&nt);
            }
        }
    }
}

pub fn function(this: &mut Transpiler, x: &Function, type_: FunctionType) {
    let (name, arguments, mut return_type) =
        (x.name.clone(), x.arguments.clone(), x.return_type.clone());

    let mut args_v = vec![];

    for arg in arguments {
        args_v.push(format!("{} {}", arg.type_, arg.name));
    }

    let args = args_v.join(", ");

    if type_ == FunctionType::Generator {
        return_type = format!("std::vector<{return_type}>");
    }

    this.buffer
        .push_str(&format!("{return_type} {name} ({args})\n{{\n"));

    match type_ {
        FunctionType::Loop => this.buffer.push_str("do {\n"),
        FunctionType::Generator => this.buffer.push_str("std::vector<int> dfjfjfdjfndjfnjd;\n"),
        _ => {}
    }
    dbg!(&x.lines);

    for line in x.lines.clone() {
        let mut token_iter = line.tokens.iter().peekable();
        let mut semic = false;
        while token_iter.peek().is_some() {
            let token = token_iter.next().unwrap();
            match token {
                Token::If(_if) => {
                    let condition = expression(this.clone().target_lang, _if.condition.clone());
                    this.buffer.push_str(&format!("if ({condition})\n{{\n"));
                }
                Token::ElseIf(_if) => {
                    let condition = expression(this.clone().target_lang, _if.condition.clone());
                    this.buffer
                        .push_str(&format!("else if ({condition})\n{{\n"));
                }
                Token::Assign(_assign) => {
                    let name = _assign.var.clone();
                    let exp = expression(this.clone().target_lang, _assign.exp.clone());
                    this.buffer.push_str(&format!("{name} = {exp};\n"));
                }
                Token::Return(_return) => {
                    let exp = expression(this.clone().target_lang, _return.clone());
                    this.buffer.push_str(&format!("return {exp};\n"));
                }
                Token::Yield(_yield) => {
                    let exp = expression(this.clone().target_lang, _yield.clone());
                    this.buffer
                        .push_str(&format!("dfjfjfdjfndjfnjd.push_back({exp});\n"));
                }
                Token::Var(_var) => {
                    let (name, exp) = (&_var.name, _var.exp.clone());
                    this.buffer.push_str(&format!(
                        "auto {name} = {};\n",
                        expression(this.clone().target_lang, exp)
                    ));
                }
                Token::End(_) => {
                    this.buffer.push_str("}\n");
                }
                Token::Else(_) => {
                    this.buffer.push_str("else{\n");
                }
                Token::Empty => {
                    this.buffer.push_str("\n");
                }
                Token::Generic(s) => {
                    this.buffer.push_str(&format!("{s} "));
                    semic = true;
                }
                Token::OpenRoBr(_) => {
                    this.buffer.push_str(&format!("("));
                    semic = true;
                }
                Token::CloseRoBr(_) => {
                    this.buffer.push_str(&format!(")"));
                    semic = true;
                }
                _ => {
                    log!(Error, "Unexpected Token");
                    dbg!(&token);
                }
            }
        }
        if semic {
            this.buffer.push_str(";\n");
        }
    }

    match type_ {
        FunctionType::Loop => this.buffer.push_str("} while (1);\n}\n"),
        FunctionType::Function => this.buffer.push_str("}\n"),
        FunctionType::Generator => this.buffer.push_str("return dfjfjfdjfndjfnjd;\n}\n"),
    }
}

pub fn expression(_this: TranspilerLang, exp: Expression) -> String {
    let mut string = String::new();
    for x in exp {
        match x {
            Token::Operator(o) => match o {
                Operator::Plus => {
                    string.push_str(&format!("+"));
                }
                Operator::Minus => {
                    string.push_str(&format!("-"));
                }
                Operator::Mul => {
                    string.push_str(&format!("*"));
                }
                Operator::Div => {
                    string.push_str(&format!("/"));
                }
                Operator::BitShiftLeft => {
                    string.push_str(&format!("<<"));
                }
                Operator::BitShiftRight => {
                    string.push_str(&format!(">>"));
                }
                Operator::Equals => {
                    string.push_str(&format!("=="));
                }
                Operator::And => {
                    string.push_str(&format!("&&"));
                }
                Operator::Or => {
                    string.push_str(&format!("||"));
                }
                Operator::BitAnd => {
                    string.push_str(&format!("&"));
                }
                Operator::BitOr => {
                    string.push_str(&format!("|"));
                }
                Operator::Pipe => {
                    log!(CXX, "Operator::Pipe : not yet implemented");
                }
            },
            Token::ExpVal(s) => {
                string.push_str(&s);
            }
            _ => {
                log!(Error, "Unexpected token in Expression");
            }
        }
    }
    string
}
