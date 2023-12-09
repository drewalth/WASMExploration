use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

#[no_mangle]
pub extern "C" fn greet() {
    // This will print to the console in WebAssembly
    println!("Hello, world!");
}

#[wasm_bindgen]
pub fn rust_function() {
    console::log_1(&"This message is from Rust!".into());
}