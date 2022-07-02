use std::fmt::format;

use wasm_bindgen::prelude::*;

pub fn log(s: &String) {
    web_sys::console::log_1(&s[..].into())
}

#[wasm_bindgen]
struct BF {
    code: String,
    program_counter: u8,
    memory: [u8; 30]
}

#[wasm_bindgen]
impl BF {
    pub fn new() -> BF {
        let code = String::from("");
        BF {
            code,
            program_counter: 0,
            memory: [0; 30]
        }
    }

    pub fn set_code(&mut self, c: String) {
        self.code = c.to_string();
        log(&format!("Setting code to: {}", self.code))
    }

    pub fn print(&self) {
        log(&format!("PC: {}, Memory: {:?}", self.program_counter, self.memory));
    }
}
