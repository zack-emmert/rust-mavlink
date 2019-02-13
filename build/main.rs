#![recursion_limit="256"]
#[macro_use]
extern crate quote;

extern crate crc16;
extern crate xml;

mod parser;

use std::env;
use std::fs::File;
use std::path::Path;

pub fn main() {
    let src_dir = env::current_dir().unwrap();
    let in_path = Path::new(&src_dir).join("combined.xml");
    let mut inf = File::open(&in_path).unwrap();
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("combined.rs");
    let mut outf = File::create(&dest_path).unwrap();

    parser::generate(&mut inf, &mut outf);

    // Dont run build.rs unless it is changed
    println!("cargo:rerun-if-changed=build.rs");
}
