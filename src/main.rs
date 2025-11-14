#[path ="./lib/mod.rs"]
pub mod lib;

use std::env;
use lib::base;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();
    base::handle_arguments(argv, argc);
}
