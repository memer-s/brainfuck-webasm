use wasm_bindgen::prelude::*;

pub fn log(s: &String) {
   web_sys::console::log_1(&s[..].into())
}

#[wasm_bindgen]
struct BF {
   code: Vec<u8>,
   code_len: u8,
   program_counter: u8,
   memory: [u8; 256],
   memory_counter: u8,
}

#[wasm_bindgen]
impl BF {
   pub fn new() -> BF {
      BF {
         code: vec![],
         code_len: 0,
         program_counter: 0,
         memory: [0; 256],
         memory_counter: 0
      }
   }

   pub fn set_code(&mut self, c: String) {
      log(&format!("Writing code: \n{}.\n\nto program memory.", &c.replace(" ", "")));
      self.code = c.into_bytes();
      self.code_len = self.code.len() as u8;
   }

   pub fn print(&self) {
      log(&format!("PC: {}/{}, Memory: \n{:?}", self.program_counter, self.code_len, self.memory));
   }

   pub fn step(&mut self) {
      if self.code_len == self.program_counter {
         log(&String::from("EOF!"))
      } else {
         match self.code[self.program_counter as usize] {
            b'+' => {
               self.memory[self.memory_counter as usize]+=1;
               self.program_counter+=1;
            },
            b'-' => {
               self.memory[self.memory_counter as usize]-=1;
               self.program_counter+=1;
            },
            b'>' => {
               self.memory_counter+=1;
               self.program_counter+=1;
            }
            b'<' => {
               self.memory_counter-=1;
               self.program_counter+=1;
            }
            _ => {
               log(&format!("Could not Interpret character at index: {}", self.memory_counter))
            }
         };
      }
   }
}
