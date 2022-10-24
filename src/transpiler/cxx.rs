use super::*;

pub type Cxx = Transpiler;
pub fn new() -> Cxx {
    Transpiler {
        buffer: String::new(),
        imports: vec![],
        target_lang: TranspilerLang::Cxx,
    }
}
