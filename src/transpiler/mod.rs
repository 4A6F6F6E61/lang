#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct Cxx {
    pub buffer: String,
    pub imports: Vec<String>,
}
mod cxx;
