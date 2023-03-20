mod memory;
mod parser;
mod program;
mod utils;
mod jacklib;
mod charmap;
mod compiler;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    console_log!("Hi there!");
}
