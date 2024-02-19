pub mod runtime;

use boa_engine::{Context, Source};
use crate::runtime::logger::prepare_logger;
use std::env::args;
use std::fs;

fn main() {
    let file_path = args().nth(1).unwrap();
    let content = fs::read(file_path).unwrap();

    let mut context = Context::default();

    match context.register_global_builtin_callable("puts", 0, prepare_logger()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Uncaught Error: {}", e);
            std::process::exit(1);
        }
    }

    context.eval(Source::from_bytes(&content)).unwrap();
}
