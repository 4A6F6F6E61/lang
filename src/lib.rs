#![allow(dead_code)]

use colored::Colorize;
use std::cell::RefCell;

pub mod lexer;
mod test;
pub mod transpiler;

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
    LexerError,
    Lexer,
    Syntax,
    Info,
    Cpu,
    Clear,
    Test,
    CXX,
}

pub fn printx(type_: PrintT, message: &str) {
    let prefix = match type_ {
        PrintT::Error => String::from("ERROR: ").red(),
        PrintT::LexerError => String::from("LEXER: ").red(),
        PrintT::Info => format!("INFO: ").green(),
        PrintT::Syntax => format!("SYNTAX: ").yellow(),
        PrintT::Lexer => format!("LEXER: ").blue(),
        PrintT::Cpu => format!("CPU: ").yellow(),
        PrintT::Test => format!("TEST: ").yellow(),
        PrintT::Clear => "".to_string().white(),
        // --------------
        // languages
        // --------------
        PrintT::CXX => format!("CXX: ").yellow(),
    };
    match type_ {
        PrintT::Clear => {
            print!("{}{}", prefix, message);
        }
        PrintT::Error | PrintT::LexerError => {
            eprintln!("{}{}", prefix, message)
        }
        _ => {
            print!("{}{}\n", prefix, message);
        }
    };
}
#[macro_export]
macro_rules! log {
    (Error, f($($format:tt)*)) => {
        printx(PrintT::Error, format!($($format)*).as_str());
    };
    (Error, $($str:tt)*) => {
        printx(PrintT::Error, $($str)*);
    };
    (LexerError, f($($format:tt)*)) => {
        printx(PrintT::LexerError, format!($($format)*).as_str());
        lexer_error();
    };
    (LexerError, $($str:tt)*) => {
        printx(PrintT::LexerError, $($str)*);
        lexer_error();
    };
    (Info, f($($format:tt)*)) => {
        printx(PrintT::Info, format!($($format)*).as_str());
    };
    (Info, $($str:tt)*) => {
        printx(PrintT::Info, $($str)*);
    };
    (Lexer, f($($format:tt)*)) => {
        printx(PrintT::Lexer, format!($($format)*).as_str());
    };
    (Lexer, $($str:tt)*) => {
        printx(PrintT::Lexer, $($str)*);
    };
    (Cpu, f($($format:tt)*)) => {
        printx(PrintT::Cpu, format!($($format)*).as_str());
    };
    (Cpu, $($str:tt)*) => {
        printx(PrintT::Cpu, $($str)*);
    };
    (Syntax, f($($format:tt)*)) => {
        printx(PrintT::Syntax, format!($($format)*).as_str());
    };
    (Syntax, $($str:tt)*) => {
        printx(PrintT::Syntax, $($str)*);
    };
    (Clear, f($($format:tt)*)) => {
        printx(PrintT::Clear, format!($($format)*).as_str());
    };
    (Clear, $($str:tt)*) => {
        printx(PrintT::Clear, $($str)*);
    };
    (CXX, f($($format:tt)*)) => {
        printx(PrintT::CXX, format!($($format)*).as_str());
    };
    (CXX, $($str:tt)*) => {

        printx(PrintT::CXX, $($str)*);
    };
    (Test, f($($format:tt)*)) => {
        printx(PrintT::Test, format!($($format)*).as_str());
    };
    (Test, $($str:tt)*) => {
        printx(PrintT::Test, $($str)*);
    };
}

#[macro_export]
macro_rules! notwasm {
    ($($x:tt)*) => {
        #[cfg(not(target_arch = "wasm32"))]
        {
            $($x)*
        }
    };
}