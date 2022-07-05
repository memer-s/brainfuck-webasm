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
      format!("{{\"program_counter\": {},\n \"memory\": {:?},\n \"memory_counter\": {},\n \"depth\": {}}}", 
         self.program_counter, 
         self.memory,
         self.memory_counter,
         self.loop_depth
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
               let mut i = 0;
               self.loop_depth = 0;
               loop {
                  if self.memory[(self.program_counter-i) as usize] == b'[' {
                     self.loop_depth += 1
                  }
                  i+=1;
                  if self.program_counter-i==0 {
                     break;
                  }
               }
               if self.memory[self.memory_counter as usize] == 0 {
                  let mut i = 0;
                  loop {
                     if self.memory[i] == b']' {
                        self.program_counter = (i-1) as u8;
                        self.loop_depth-=1;
                        break;
                     }
                     i+=1;
                  }
               }
               self.program_counter+=1;
            },
            b']' => {
               let mut i = 0;
               let mut n = self.loop_depth;
               loop {
                  if self.code[(self.program_counter-i) as usize] == b'[' {
                     if n == 1 {
                        self.program_counter = self.program_counter-(i+1);
                        break;
                     }
                     n-=1;
                  }
                  i+=1;
               }
               self.program_counter+=1;
            }
            _ => {
               log(&format!("Could not Interpret character at index: {}", self.memory_counter));
            }
         };
      }
   }
}
