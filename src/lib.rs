use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

pub fn log(s: &String) {
   web_sys::console::log_1(&s[..].into())
}

#[wasm_bindgen]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook))
}

#[wasm_bindgen]
struct BF {
   code: Vec<u8>,
   code_len: u8,
   program_counter: u8,
   memory: [u8; 256],
   memory_counter: u8,
   stack: Vec<u8>,
   loop_indices: Vec<u8>,
   loop_depth: u8
}

#[wasm_bindgen]
impl BF {
   pub fn new() -> BF {
      BF {
         code: vec![],
         code_len: 0,
         program_counter: 0,
         memory: [0; 256],
         memory_counter: 0,
         stack: vec![],
         loop_indices: vec![],
         loop_depth: 0
      }
   }
    
   pub fn set_code(&mut self, mut c: String) {
      log(&format!("Writing code: \n{}.\n\nto program memory", &c));
      c = c.replace(" ", "");
      c = c.replace("\n", "");
      c = c.replace("\t", "");
      self.code = c.into_bytes();
      self.code_len = self.code.len() as u8;
      self.program_counter = 0;
      self.memory = [0; 256];
      self.memory_counter = 0;
      self.loop_depth = 0;
   }

   pub fn print(&self) {
      log(&format!("PC: {}/{}, Memory: \n{:?}", self.program_counter, self.code_len, self.memory));
   }

   pub fn get_state(&self) -> String {
      format!("{{\n \"program_counter\": {},\n \"memory\": {:?},\n \"memory_counter\": {},\n \"depth\": {},\n \"stack\": {:?}\n}}", 
         self.program_counter, 
         self.memory,
         self.memory_counter,
         self.loop_depth,
         self.stack
      )
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
            b'[' => {
               if self.memory[self.memory_counter as usize] != 0 {
                  self.stack.push(self.program_counter);
               } else {
                  let mut depth = 0;
                  let mut i = 0;
                  loop {

                     if i+self.program_counter as usize == self.code.len()-1 {
                        break;
                     }

                     match self.code[self.program_counter as usize+i] {
                        b'[' => {
                           depth+=1;
                        }
                        b']' => {
                           depth -= 1;
                           if depth == 0 {
                              self.program_counter = self.program_counter+1+i as u8;
                              break;
                           }
                        }
                        _ => {}
                     }
                     i+=1
                  }
               }
               self.program_counter+=1;
            },
            b']' => {
               if self.stack.len()>0 {
                  self.program_counter = self.stack[self.stack.len()-1];
                  self.stack.pop();
               }
               else {
                  self.program_counter+=2
               }
            }
            _ => {
               log(&format!("Could not Interpret character at index: {}", self.memory_counter));
            }
         };
      }
   }
}
