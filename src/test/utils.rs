use {
    crate::{log, printx, transpiler::*, PrintT},
    std::{fs::read_to_string, path::Path},
};

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

pub fn test_cxx(test: &str) {
    let cxx = &mut cxx::new();
    run(cxx, &format!("./src/examples/{test}.lang"));
    let mut code1 = String::from(cxx.buffer.trim());
    code1.push_str("\n");
    log!(Info, f("\n{code1}"));
    if let Some(code2) = load_file(&format!("./src/examples/out/cxx/{test}.cxx")) {
        assert_eq!(code1, code2);
    }
}
