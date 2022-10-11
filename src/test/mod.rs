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
mod syntax {
    #[test]
    fn test_main() {
        use crate::{log, printx, transpiler::Cxx, PrintT};

        let mut cxx = Cxx::new();
        cxx.run("./src/examples/main.lang");
        log!(Info, f("\n{}", cxx.buffer));
        assert_eq!(
            cxx.buffer.trim(),
            "int main ()
{
return 0 ;
}"
        );
    }
    #[test]
    fn test_if() {
        use crate::{log, printx, transpiler::Cxx, PrintT};

        let mut cxx = Cxx::new();
        cxx.run("./src/examples/if.lang");
        log!(Info, f("\n{}", cxx.buffer));
        assert_eq!(
            cxx.buffer.trim(),
            "int main ()
{
auto test = true;
if (test)
{
test =false ;
}
}"
        );
    }
    #[test]
    fn test_let() {
        use crate::{log, printx, transpiler::Cxx, PrintT};

        let mut cxx = Cxx::new();
        cxx.run("./src/examples/let.lang");
        log!(Info, f("\n{}", cxx.buffer));
        assert_eq!(
            cxx.buffer.trim(),
            "int main ()
{
auto test = 10;
}"
        );
    }
    #[test]
    fn test_loop_function() {
        use crate::{log, printx, transpiler::Cxx, PrintT};

        let mut cxx = Cxx::new();
        cxx.run("./src/examples/loop_function.lang");
        log!(Info, f("\n{}", cxx.buffer));
        assert_eq!(
            cxx.buffer.trim(),
            "int main ()
{
do {
} while (1);
}"
        );
    }
    #[test]
    fn test_global_let() {
        use crate::{log, printx, transpiler::Cxx, PrintT};

        let mut cxx = Cxx::new();
        cxx.run("./src/examples/global_let.lang");
        log!(Info, f("\n{}", cxx.buffer));
        assert_eq!(
            cxx.buffer.trim(),
            "auto test = 69;
int main ()
{
}"
        );
    }
    #[test]
    fn test_global_const() {
        use crate::{log, printx, transpiler::Cxx, PrintT};

        let mut cxx = Cxx::new();
        cxx.run("./src/examples/global_const.lang");
        log!(Info, f("\n{}", cxx.buffer));
        assert_eq!(
            cxx.buffer.trim(),
            "const auto test = 69;
int main ()
{
}"
        );
    }
}
