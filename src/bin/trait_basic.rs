#![allow(unused)]

struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {file_path}")
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {file_path}")
    }
}

fn main() {}