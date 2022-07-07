// Author: github.com/memer-s.
// Check out the wikipedia article on brainfuck if you are interested.

use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;


// Helper functions.
pub fn log(s: &String) {
   web_sys::console::log_1(&s[..].into())
}

pub fn err(s: &String) {
   web_sys::console::error_1(&s[..].into())
}

// -------------------------------

// This function runs after initialization. 
// It reroutes the Rust errors to the Javascript console
#[wasm_bindgen]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}


// -------------------------------

#[wasm_bindgen]
struct BF {
   code: Vec<u8>,
   program_counter: u8,
   memory: [u8; 256],
   memory_counter: u8,
   stack: Vec<u8>,
}

#[wasm_bindgen]
impl BF {
   pub fn new() -> BF {
      BF {
         code: vec![],
         program_counter: 0,
         memory: [0; 256],
         memory_counter: 0,
         stack: vec![],
      }
   }
   
   // This function is called with the code passed as a string,
   // it then parses the string into a Vector<u8>.
   // The function also sets the program_counter and the 
   // memory_counter to 0, and it initializes the memory: [u8; 256].
   pub fn set_code(&mut self, mut c: String) {
      self.code = vec![];
      c.into_bytes().iter().for_each(|char| {
         match char {
             b'+' => self.code.push(*char),
             b'-' => self.code.push(*char),
             b'>' => self.code.push(*char),
             b'<' => self.code.push(*char),
             b'[' => self.code.push(*char),
             b']' => self.code.push(*char),
             b'.' => self.code.push(*char),
             _ => {}
         }
      });
      log(&format!("Writing code: \n{:?}.\n\nto program memory", self.code));
      self.program_counter = 0;
      self.memory = [0; 256];
      self.memory_counter = 0;
   }

   // This function prints the internal state 
   // of the interpreter to the JS console
   pub fn print(&self) {
      log(&format!("PC: {}/{}, Memory: \n{:?}", self.program_counter, self.code.len(), self.memory));
   }

   // This function gets the internal state of the interpreter
   // as a stringified JSON object.
   pub fn get_state(&self) -> String {
      format!("{{\n \"program_counter\": {},\n \"memory\": {:?},\n \"memory_counter\": {},\n \"stack\": {:?}\n}}", 
         self.program_counter, 
         self.memory,
         self.memory_counter,
         self.stack
      )
   }
   
   // This runs the code at the program_counter. And increments the program_counter.
   // The "," operation is not yet implemented.
   pub fn step(&mut self) {
      if self.code.len() == self.program_counter as usize {
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
               if self.memory[self.memory_counter as usize] == 0 {
                  let mut i = 0;
                  let mut depth = 0;
                  loop {
                     match self.code[i+self.program_counter as usize] {
                        b'[' => {
                           depth += 1;
                        }
                        b']' => {
                           depth -= 1;
                           if depth == 0 {
                              self.program_counter = self.program_counter + i as u8;
                              break;
                           }
                        }
                        _ => {}
                     }
                     i+=1;
                  }
               }
               else {
                  self.program_counter+=1;
               }
            },
            b']' => {
               if self.memory[self.memory_counter as usize] != 0 {
                  let mut i = 0;
                  let mut depth = 0;
                  loop {
                     match self.code[self.program_counter as usize - i] {
                        b'[' => {
                           depth -= 1;
                           if depth == 0 {
                              self.program_counter = self.program_counter - i as u8;
                              break;
                           }
                        }
                        b']' => {
                           depth += 1;
                        }
                        _ => {}
                     }
                     i+=1;
                  }
               }
               else {
                  self.program_counter+=1;
               }
            },
            b'.' => {
               let doc = web_sys::window().unwrap().document().unwrap();
               let el = doc.get_element_by_id("cout").unwrap();
               let text = el.text_content().unwrap();
               if self.memory[self.memory_counter as usize].is_ascii() {
                   el.set_inner_html(
                       &format!("{}{}",text,std::str::from_utf8(&[self.memory[self.memory_counter as usize]]).unwrap())
                       );
               } else {
                   err(&format!("Could not convert current memory cell to ascii. [{}] is not valid ascii.", self.memory[self.memory_counter as usize]))
               }
               self.program_counter+=1
            }
            _ => {
               log(&format!("Could not Interpret character at index: {}", self.memory_counter));
            }
         };
      }
   }
}
