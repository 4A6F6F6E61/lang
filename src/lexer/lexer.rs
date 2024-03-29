#![allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
use {
    indicatif::{
        ProgressBar,
        ProgressState,
        ProgressStyle
    },
    std::fmt::Write,
};
use {
    crate::{lexer::*, lexer_error, log, printx, PrintT, notwasm},
    rand::Rng,
    std::vec,
};

// -----------------------------------------------------------------------
// Lexer implementation
// -----------------------------------------------------------------------
impl Lexer {
    pub fn new() -> Lexer {
        return Lexer {
            tmp_ast: vec![],
            ast: vec![],
            strings: vec![],
            #[cfg(not(target_arch = "wasm32"))]
            progress_bar: ProgressBar::new(100),
            #[cfg(not(target_arch = "wasm32"))]
            progress: 0,
            brackets: Brackets {
                round: 0,
                square: 0,
                braces: 0,
            },
        };
    }
    // --------------------------------
    // Progressbar setup
    // --------------------------------
    #[cfg(not(target_arch = "wasm32"))]
    pub fn setup_pb(&mut self) {
        self.progress_bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("#-"),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn finish_pb(&mut self) {
        self.progress_bar
            .finish_with_message("Finished parsing tokens");
    }
    // --------------------------------
    // String vector generation
    // --------------------------------
    fn generate_strings(&mut self, code: String) {
        self.strings.clear();
        let line_count = code.lines().count();
        if line_count != 0 {
            code.lines().enumerate().for_each(|(i, line)| {
                notwasm! {
                    self.progress_bar.set_position(
                        ((i as f32/line_count as f32 * 100f32)*0.25) as u64
                    );
                }
                let mut temp_string = String::new();
                self.strings.push(vec![]);
                let mut chars = line.chars().peekable();
                while chars.peek().is_some() {
                    let char = chars.next().unwrap();
                    match char {
                        ' ' => {
                            if temp_string.len() > 0 {
                                self.strings[i].push(temp_string);
                                temp_string = String::new();
                            }
                        }
                        '(' | ')' | '{' | '}' | '[' | ']' | ',' | '=' | '+' | '-' | '*' | '"'
                        | '#' => {
                            if temp_string.len() > 0 {
                                self.strings[i].push(temp_string);
                                temp_string = String::new();
                            }

                            self.strings[i].push(char.to_string());
                        }
                        '/' => {
                            if temp_string.len() > 0 {
                                self.strings[i].push(temp_string);
                                temp_string = String::new();
                            }

                            if chars.peek() == Some(&'/') {
                                chars.next();
                                self.strings[i].push("//".to_string());
                            } else {
                                self.strings[i].push(char.to_string());
                            }
                        }
                        _ => temp_string.push(char),
                    }
                }
                if temp_string.len() > 0 {
                    self.strings[i].push(temp_string);
                } else {
                    self.strings[i].push("".to_string());
                }
            });
        } else {
            log!(Lexer, "Please provide some code");
        }
    }
    // --------------------------------
    // Parsing
    // --------------------------------
    pub fn parse(&mut self, code: String) {
        use super::token::*;
        
        log!(Lexer, "Started parsing tokens...");
        notwasm! {
            self.setup_pb();
        }

        let mut rng = rand::thread_rng();
        notwasm! {self.progress_bar.set_position(0);}
        self.top_level(code);
        let mut p = 1;
        let len = self.tmp_ast.len();
        for node in self.tmp_ast.clone() {
            notwasm! {
                self.progress_bar.set_position(
                    (((p as f32/len as f32 * 100f32)*0.5)+50f32) as u64
                );
                p += 1;
            }
            match node {
                Token::Function(func) => {
                    let f = Token::Function(Function {
                        lines: self.low_level(
                            func.tmp_lines.clone(),
                            func.start_ln.clone(),
                            rng.gen_range(0..=i32::MAX),
                        ),
                        ..func
                    });
                    self.ast.push(f);
                }
                Token::LoopFunction(func) => {
                    let f = Token::LoopFunction(Function {
                        lines: self.low_level(
                            func.tmp_lines.clone(),
                            func.start_ln.clone(),
                            rng.gen_range(0..=i32::MAX),
                        ),
                        ..func
                    });
                    self.ast.push(f);
                }
                Token::GeneratorFunction(func) => {
                    let f = Token::GeneratorFunction(Function {
                        lines: self.low_level(
                            func.tmp_lines.clone(),
                            func.start_ln.clone(),
                            rng.gen_range(0..=i32::MAX),
                        ),
                        ..func
                    });
                    self.ast.push(f);
                }
                _ => {
                    self.ast.push(node.to_owned());
                }
            }
        }
        notwasm! {self.progress_bar.set_position(10000);}
        if self.brackets.braces > 0 {}
    }
    pub fn top_level(&mut self, code: String) {
        use super::token::*;
        self.generate_strings(code); // generates a 2D string vector
                                     // ------------------------------
                                     // This is for toplevel only
                                     // ------------------------------
        let mut line_iter = self.strings.iter().peekable();
        let mut line_number = 0;
        let mut p = 1;
        let mut fn_type = FunctionType::Function;
        let len = self.strings.len();
        while line_iter.peek().is_some() {
            notwasm! {
                self.progress_bar.set_position(
                    (((p as f32/len as f32 * 100f32)*0.5)+25f32) as u64
                );
                p += 1;
            }
            line_number += 1;
            let next_line = line_iter.next().unwrap();
            let mut string_iter = next_line.iter().peekable();
            while string_iter.peek().is_some() {
                let _str = string_iter.next().unwrap();
                match _str.as_str() {
                    "loop" => {
                        fn_type = FunctionType::Loop;
                    }
                    "gen" => {
                        fn_type = FunctionType::Generator;
                    }
                    "fn" => {
                        self.brackets.braces = 0;
                        self.brackets.round = 0;
                        self.brackets.square = 0;
                        let syntax_fn = || match fn_type {
                            FunctionType::Function => {
                                log!(Syntax, "\nfn `name` (`arguments`) {\n`code`\n}");
                            }
                            FunctionType::Loop => {
                                log!(Syntax, "\nloop fn `name` (`arguments`) {\n`code`\n}");
                            }
                            FunctionType::Generator => {
                                log!(Syntax, "\ngen fn `name` (`arguments`) {\n`code`\n}");
                            }
                        };
                        if let (Some(fn_name), Some(fn_op_br)) =
                            (string_iter.next(), string_iter.next())
                        {
                            if fn_name == "(" {
                                log!(
                                    LexerError,
                                    f("Expected function name at line {line_number}")
                                );
                                syntax_fn();
                            } else if fn_op_br != "(" {
                                log!(LexerError, f("Expected opening bracket after function name but got `{fn_op_br}` at line {line_number}"));
                                syntax_fn();
                            } else {
                                self.brackets.round += 1;
                                /* --------------------------------------------------
                                 *  Parse Function Arguments
                                 * --------------------------------------------------*/
                                let mut arguments: Vec<String> = vec![];
                                while string_iter.peek().is_some() {
                                    if string_iter.peek().unwrap().as_str() == ")" {
                                        self.brackets.round -= 1;
                                        string_iter.next();
                                        break;
                                    } else if string_iter.peek().unwrap().as_str() == "(" {
                                        self.brackets.round += 1;
                                        log!(
                                            LexerError,
                                            f("Unexpected opening bracket at line {line_number}")
                                        );
                                    }
                                    arguments.push(string_iter.next().unwrap().to_string());
                                }
                                /* --------------------------------------------------
                                 *  Turn the Function Arguments string Vector
                                 *  into a vector of `Arg` structs
                                 * --------------------------------------------------*/
                                let mut args: Vec<Arg> = vec![];
                                let mut arg_iter = arguments.iter().peekable();
                                while arg_iter.peek().is_some() {
                                    let x = arg_iter.next().unwrap();
                                    if x.is_empty() {
                                        log!(
                                            LexerError,
                                            f("Expected argument `name` at line {line_number}")
                                        );
                                    } else {
                                        let mut tmp = x.chars();
                                        tmp.next_back();
                                        let name = tmp.collect();
                                        let mut type_ = String::new();
                                        if !x.ends_with(":") {
                                            log!(
                                                LexerError,
                                                f(
                                                    "{:?} Expected `:` at line {line_number}",
                                                    arg_iter.peek()
                                                )
                                            );
                                        } else {
                                            while arg_iter.peek() != Some(&&",".to_string())
                                                && arg_iter.peek().is_some()
                                            {
                                                type_.push_str(arg_iter.next().unwrap());
                                            }
                                            if type_.is_empty() {
                                                log!(LexerError, f("Expected argument `type` at line {line_number}"));
                                            } else {
                                                arg_iter.next();
                                                args.push(Arg { name, type_ })
                                            }
                                        }
                                    }
                                }

                                if let Some(op_braces) = string_iter.next() {
                                    if op_braces != "{" && op_braces != ":" {
                                        log!(LexerError, f("Expected opening braces or colon but found `{op_braces}` at line {line_number}"));
                                    } else {
                                        /* --------------------------------------------------
                                         *  Parse Function Return Type
                                         * --------------------------------------------------*/
                                        let mut return_type = "void".to_string();
                                        if op_braces == ":" {
                                            // Check for return type
                                            if let Some(type_) = string_iter.next() {
                                                return_type = type_.to_owned();
                                            } else {
                                                log!(
                                                    LexerError,
                                                    f("Expected return type at line {line_number}")
                                                );
                                            }
                                            if let Some(op_braces_) = string_iter.next() {
                                                if op_braces_ != "{" {
                                                    log!(LexerError, f("Expected opening braces found `{op_braces_}` at line {line_number}"));
                                                }
                                            }
                                        }
                                        /* --------------------------------------------------
                                         *  Parse Function Body
                                         * --------------------------------------------------*/
                                        self.brackets.braces += 1;
                                        let mut fn_body: Vec<Vec<String>> = vec![];
                                        let mut function_parsed: bool = false;
                                        while line_iter.peek().is_some() {
                                            if function_parsed {
                                                break;
                                            }
                                            string_iter =
                                                line_iter.next().unwrap().iter().peekable();
                                            let mut temp = vec![];
                                            while string_iter.peek().is_some() {
                                                let current_string = string_iter.next().unwrap();
                                                match current_string.as_str() {
                                                    "{" => {
                                                        self.brackets.braces += 1;
                                                        temp.push("{".to_string());
                                                    }
                                                    "}" => {
                                                        self.brackets.braces -= 1;
                                                        if self.brackets.braces == 0 {
                                                            let func = Function {
                                                                name: fn_name.to_owned(),
                                                                arguments: args.clone(),
                                                                return_type: return_type.clone(),
                                                                lines: vec![],
                                                                tmp_lines: fn_body.clone(),
                                                                start_ln: line_number,
                                                            };
                                                            match fn_type {
                                                                FunctionType::Function => {
                                                                    self.tmp_ast.push(
                                                                        Token::Function(func),
                                                                    );
                                                                    function_parsed = true;
                                                                }
                                                                FunctionType::Loop => {
                                                                    self.tmp_ast.push(
                                                                        Token::LoopFunction(func),
                                                                    );
                                                                    function_parsed = true;
                                                                }
                                                                FunctionType::Generator => {
                                                                    self.tmp_ast.push(
                                                                        Token::GeneratorFunction(
                                                                            func,
                                                                        ),
                                                                    );
                                                                    function_parsed = true;
                                                                }
                                                            }
                                                            fn_type = FunctionType::Function;
                                                            break;
                                                        } else {
                                                            fn_body.push(vec!["}".to_string()]);
                                                        }
                                                    }
                                                    _ => {
                                                        temp.push(current_string.to_owned());
                                                    }
                                                }
                                            }
                                            fn_body.push(temp);
                                        }
                                        if !function_parsed {
                                            log!(
                                                Error,
                                                f("Unable to parse function at line {line_number}")
                                            );
                                        }
                                    }
                                } else {
                                    log!(
                                        LexerError,
                                        f("Expected opening braces at line {line_number}")
                                    );
                                }
                            }
                        } else {
                            log!(LexerError, f("Expected function name and opening bracket at line {line_number}"));
                            syntax_fn();
                        }
                    }
                    "//" => {
                        let comment = next_line.join(" ");
                        self.tmp_ast.push(Token::Comment(comment));
                        break;
                    }
                    "const" | "global" => {
                        let syntax = || {
                            if _str == "const" {
                                log!(Syntax, "const `name` = `value`");
                            } else {
                                log!(Syntax, "global `name` = `value`");
                            }
                        };
                        if let (Some(name), Some(equals)) = (string_iter.next(), string_iter.next())
                        {
                            //TODO: Check for multidiemensional Array
                            if equals == "=" {
                                let expression: Vec<&String> = string_iter.clone().collect();
                                while string_iter.peek().is_some() {
                                    string_iter.next();
                                }
                                if expression.contains(&&"[".to_string())
                                    && !expression.contains(&&"]".to_string())
                                {
                                    log!(LexerError, f("Creating a multiline Array with the let binding is not supported. Line: {line_number}"));
                                } else {
                                    let exp = generate_expression(expression, line_number);
                                    if _str == "const" {
                                        self.tmp_ast.push(Token::Const(Let {
                                            name: name.to_owned(),
                                            exp,
                                        }));
                                    } else {
                                        self.tmp_ast.push(Token::Global(Let {
                                            name: name.to_owned(),
                                            exp,
                                        }));
                                    }
                                }
                            } else {
                                log!(LexerError, f("Expected `=` at line {line_number}"));
                                syntax();
                            }
                        } else {
                            log!(LexerError, f("Wrong Syntax at line {line_number}"));
                            syntax();
                        }
                    }
                    "#" => {
                        if let Some(two) = string_iter.next() {
                            if two == "include" {
                                if let Some(path) = string_iter.next() {
                                    self.tmp_ast.push(Token::CImport(format!("{path}")));
                                } else {
                                    log!(LexerError, f("Expected `path` at line {line_number}"));
                                }
                            } else {
                                log!(LexerError, f("Unimplimented `{two}` at line {line_number}"));
                            }
                        } else {
                            log!(LexerError, f("Expected `something` at line {line_number}"));
                        }
                    }
                    "" => {}
                    _ => {
                        self.tmp_ast.push(Token::Unknown(_str.to_string()));
                        log!(LexerError, f("Unexpected token {_str} at line {line_number}"));
                    }
                }
            }
        }
        // ------------------------------
    }

    pub fn low_level(&mut self, code: Vec<Vec<String>>, start_ln: i32, id: i32) -> Vec<Line> {
        use super::token::*;
        let mut lines = vec![];
        let mut as_string: String;
        let mut line_iter = code.iter().peekable();
        let mut line_number = start_ln;
        while line_iter.peek().is_some() {
            let mut tokens = vec![];
            line_number += 1;
            let next_line = line_iter.next().unwrap();
            as_string = next_line.join(" ");
            if next_line.is_empty() {
                continue;
            }
            let mut string_iter = next_line.iter().peekable();
            while string_iter.peek().is_some() {
                let string = string_iter.next().unwrap().as_str();
                match string {
                    "let" => {
                        let syntax = || {
                            log!(Syntax, "let `name` = `expression`");
                        };
                        if let (Some(name), Some(equals)) = (string_iter.next(), string_iter.next())
                        {
                            //TODO: Check for multidiemensional Array
                            if equals == "=" {
                                let expression: Vec<&String> = string_iter.clone().collect();
                                while string_iter.peek().is_some() {
                                    string_iter.next();
                                }
                                if expression.contains(&&"[".to_string())
                                    && !expression.contains(&&"]".to_string())
                                {
                                    log!(LexerError, f("Creating a multiline Array with the let binding is not supported. Line: {line_number}"));
                                } else {
                                    tokens.push(Token::Var(Let {
                                        name: name.to_owned(),
                                        exp: generate_expression(expression, line_number),
                                    }));
                                }
                            } else {
                                log!(LexerError, f("Expected `=` at line {line_number}"));
                                syntax();
                            }
                        } else {
                            log!(LexerError, f("Wrong Syntax at line {line_number}"));
                            syntax();
                        }
                    }
                    "if" => {
                        let syntax = || {
                            log!(Syntax, "\nif `condition` {\n   `code`\n}");
                        };
                        let mut condition_v: Vec<&String> = vec![];
                        let mut then: bool = false;
                        while string_iter.peek().is_some() {
                            let nt = string_iter.next().unwrap();
                            if nt == "{" {
                                then = true;
                                self.brackets.braces += 1;
                                break;
                            }
                            condition_v.push(nt);
                        }
                        if !then {
                            log!(LexerError, f("Expected `{{` at line {line_number}"));
                            syntax();
                        } else {
                            if condition_v.is_empty() {
                                log!(LexerError, f("Expected condition at line {line_number}"));
                                syntax();
                            } else {
                                tokens.push(Token::If(If::new(
                                    generate_expression(condition_v, line_number),
                                    id,
                                    self.brackets.braces,
                                )));
                            }
                        }
                    }
                    "else" => {
                        if let Some(nt) = string_iter.next() {
                            match nt.as_str() {
                                "if" => {
                                    let syntax = || {
                                        log!(Syntax, "\n} else if `condition` {\n   `code`\n}");
                                    };
                                    let mut condition_v: Vec<&String> = vec![];
                                    let mut then: bool = false;
                                    while string_iter.peek().is_some() {
                                        let nt = string_iter.next().unwrap();
                                        if nt == "{" {
                                            then = true;
                                            self.brackets.braces += 1;
                                            break;
                                        }
                                        condition_v.push(nt);
                                    }
                                    if !then {
                                        log!(LexerError, f("Expected `{{` at line {line_number}"));
                                        syntax();
                                    } else {
                                        if condition_v.is_empty() {
                                            log!(
                                                LexerError,
                                                f("Expected condition at line {line_number}")
                                            );
                                            syntax();
                                        } else {
                                            tokens.push(Token::ElseIf(If::new(
                                                generate_expression(condition_v, line_number),
                                                id,
                                                self.brackets.braces,
                                            )));
                                        }
                                    }
                                }
                                "{" => {
                                    tokens.push(Token::Else(Else::new(id, self.brackets.braces)));
                                    self.brackets.braces += 1;
                                }
                                _ => {
                                    log!(LexerError, f("Expected `{{` or `if` but found {nt} at line {line_number}"));
                                }
                            }
                        } else {
                            log!(LexerError, f("Expected `{{` or `if` at line {line_number}"));
                        }
                    }
                    "for" => {
                        let syntax = || {
                            log!(
                                Syntax,
                                "\nfor `var_name` in `iterator/Expression` {\n   `code`\n}"
                            );
                        };
                        if let Some(var_name) = string_iter.next() {
                            let mut iter_exp: Vec<&String> = vec![];
                            let mut then: bool = false;
                            let mut _in: bool = false;
                            if string_iter.peek().is_some()
                                && string_iter.next().unwrap() == &"in".to_string()
                            {
                                while string_iter.peek().is_some() {
                                    let nt = string_iter.next().unwrap();

                                    if nt == "{" {
                                        then = true;
                                        self.brackets.braces += 1;
                                        break;
                                    }
                                    iter_exp.push(nt);
                                }
                                if !then {
                                    log!(LexerError, f("Expected `{{` at line {line_number}"));
                                    syntax();
                                } else {
                                    if iter_exp.is_empty() {
                                        log!(
                                            LexerError,
                                            f("Expected iterator or expression at line {line_number}")
                                        );
                                        syntax();
                                    } else {
                                        tokens.push(Token::For(For::new(
                                            var_name.to_owned(),
                                            generate_expression(iter_exp, line_number),
                                            id,
                                            self.brackets.braces,
                                        )));
                                    }
                                }
                            } else {
                                log!(LexerError, f("Expected `in` at line {line_number}"));
                                syntax();
                            }
                        } else {
                            log!(LexerError, f("Expected var_name at line {line_number}"));
                            syntax();
                        }
                    }
                    "}" => {
                        tokens.push(Token::End(Br::new(id, self.brackets.braces)));
                        self.brackets.braces -= 1;
                    }
                    "[" => {
                        tokens.push(Token::OpenSqBr(Br::new(id, self.brackets.square)));
                        self.brackets.square += 1;
                    }
                    "]" => {
                        tokens.push(Token::CloseSqBr(Br::new(id, self.brackets.square)));
                        self.brackets.square -= 1;
                    }
                    "(" => {
                        tokens.push(Token::OpenRoBr(Br::new(id, self.brackets.round)));
                        self.brackets.round += 1;
                    }
                    ")" => {
                        tokens.push(Token::CloseRoBr(Br::new(id, self.brackets.round)));
                        self.brackets.round -= 1;
                    }
                    "," => {
                        tokens.push(Token::Comma);
                    }
                    "loop" => {
                        if let Some(nt) = string_iter.next() {
                            if nt == "{" {
                                self.brackets.braces += 1;
                                tokens.push(Token::Loop(Loop {
                                    id: self.brackets.braces,
                                }));
                            } else {
                                log!(LexerError, f("Expected `{{` at line {line_number}"));
                            }
                        }
                    }
                    "" => {}
                    "yield" => {
                        if let Some(_) = string_iter.peek() {
                            let exp = string_iter.clone().collect();
                            while string_iter.peek().is_some() {
                                string_iter.next();
                            }
                            tokens.push(Token::Yield(generate_expression(exp, line_number)));
                        } else {
                            log!(
                                LexerError,
                                f("Expected expression after `yield` at line {line_number}")
                            );
                        }
                    }
                    "return" => {
                        if let Some(_) = string_iter.peek() {
                            let exp = string_iter.clone().collect();
                            while string_iter.peek().is_some() {
                                string_iter.next();
                            }
                            tokens.push(Token::Return(generate_expression(exp, line_number)));
                        } else {
                            log!(
                                LexerError,
                                f("Expected expression after `return` at line {line_number}")
                            );
                        }
                    }
                    _ => {
                        if let Some(nt) = string_iter.peek() {
                            match nt.as_str() {
                                "=" => {
                                    string_iter.next();
                                    if let Some(_) = string_iter.peek() {
                                        let exp = string_iter.clone().collect();
                                        while string_iter.peek().is_some() {
                                            string_iter.next();
                                        }
                                        tokens.push(Token::Assign(token::Assign {
                                            var: string.to_owned(),
                                            exp: generate_expression(exp, line_number),
                                        }));
                                    } else {
                                        log!(
                                            LexerError,
                                            f("Expected expression after assignment at line {line_number}")
                                        );
                                    }
                                }
                                _ => {
                                    tokens.push(Token::Generic(String::from(string)));
                                }
                            }
                        } else {
                            tokens.push(Token::Generic(String::from(string)));
                        }
                    }
                }
            }
            lines.push(Line { tokens, as_string });
        }
        lines
    }
}

pub fn generate_expression(strings: Vec<&String>, _line_number: i32) -> token::Expression {
    let mut expression = vec![];

    use super::token::expression::*;

    for string in strings {
        match string.as_str() {
            "+" => {
                expression.push(Token::Operator(Operator::Plus));
            }
            "-" => {
                expression.push(Token::Operator(Operator::Minus));
            }
            "*" => {
                expression.push(Token::Operator(Operator::Mul));
            }
            "/" => {
                expression.push(Token::Operator(Operator::Div));
            }
            "<<" => {
                expression.push(Token::Operator(Operator::BitShiftLeft));
            }
            ">>" => {
                expression.push(Token::Operator(Operator::BitShiftRight));
            }
            "==" => {
                expression.push(Token::Operator(Operator::Equals));
            }
            "|>" => {
                expression.push(Token::Operator(Operator::Pipe));
            }
            "|" => {
                expression.push(Token::Operator(Operator::BitOr));
            }
            "&" => {
                expression.push(Token::Operator(Operator::BitAnd));
            }
            "||" => {
                expression.push(Token::Operator(Operator::Or));
            }
            "&&" => {
                expression.push(Token::Operator(Operator::And));
            }
            _ => {
                expression.push(Token::ExpVal(string.to_owned()));
            }
        }
    }

    expression
}
