#![allow(dead_code)]

use colored::Colorize;
use std::cell::RefCell;

thread_local! {
    pub static LEXER_ERROR_COUNT: RefCell<usize> = RefCell::new(0usize);
}

pub fn lexer_error() {
    LEXER_ERROR_COUNT.with(|count| {
        *count.borrow_mut() += 1;
    });
}

pub enum PrintT {
    Error,
    Lexer,
    Syntax,
    Info,
    Cpu,
    Clear,
}

pub fn printx(type_: PrintT, message: &str) {
    let prefix = match type_ {
        PrintT::Error => format!("[Error]: ").red(),
        PrintT::Info => format!("[Info]: ").green(),
        PrintT::Lexer => format!("[Lexer]: ").blue(),
        PrintT::Cpu => format!("[Cpu]: ").yellow(),
        PrintT::Syntax => format!("[Syntax]: ").yellow(),
        PrintT::Clear => "".to_string().white(),
    };
    match type_ {
        PrintT::Clear => {
            print!("{}{}", prefix, message);
        }
        _ => {
            print!("{}{}\n", prefix, message);
        }
    };
}
#[macro_export]
macro_rules! log {
    (Error, f($($format:tt)*)) => {
        use crate::lib::{printx, PrintT};
        printx(PrintT::Error, format!($($format)*).as_str());
    };
    (Error, $($str:tt)*) => {
        //use crate::cpu::{printx, PrintT};
        printx(PrintT::Error, $($str)*);
    };
    (LexerError, f($($format:tt)*)) => {
        use crate::lib::{printx, PrintT, lexer_error};
        printx(PrintT::Error, format!($($format)*).as_str());
        lexer_error();
    };
    (LexerError, $($str:tt)*) => {
        use crate::lib::{printx, PrintT, lexer_error};
        printx(PrintT::Error, $($str)*);
        lexer_error();
    };
    (Info, f($($format:tt)*)) => {
        use crate::lib::{printx, PrintT};
        printx(PrintT::Info, format!($($format)*).as_str());
    };
    (Info, $($str:tt)*) => {
        printx(PrintT::Info, $($str)*);
    };
    (Lexer, f($($format:tt)*)) => {
        use crate::lib::{printx, PrintT};
        printx(PrintT::Lexer, format!($($format)*).as_str());
    };
    (Lexer, $($str:tt)*) => {
        use crate::lib::{printx, PrintT};
        printx(PrintT::Lexer, $($str)*);
    };
    (Cpu, f($($format:tt)*)) => {
        printx(PrintT::Cpu, format!($($format)*).as_str());
    };
    (Cpu, $($str:tt)*) => {
        use crate::lib::{printx, PrintT};
        printx(PrintT::Cpu, $($str)*);
    };
    (Syntax, f($($format:tt)*)) => {
        use crate::lib::{printx, PrintT};
        printx(PrintT::Syntax, format!($($format)*).as_str());
    };
    (Syntax, $($str:tt)*) => {
        use crate::lib::{printx, PrintT};
        printx(PrintT::Syntax, $($str)*);
    };
    (Clear, f($($format:tt)*)) => {
        printx(PrintT::Clear, format!($($format)*).as_str());
    };
    (Clear, $($str:tt)*) => {
        printx(PrintT::Clear, $($str)*);
    };
}
