use anyhow::{anyhow, Error};
use std::fs;
use std::io::{stdin, Stdin, Read};
use crate::parser::Command;
use std::collections::HashMap;

mod parser;

#[derive(Debug)]
struct ProgramStack {
  program: Vec<Command>,
  inst_index: usize,
  tape: HashMap<usize, u8>,
  tape_index: usize,
  stdin: Stdin,
}

impl ProgramStack {
  pub fn from(program: Vec<u8>) -> Result<ProgramStack, Error> {
    if program.len() == 0 {
      return Err(anyhow::anyhow!("Program cannot be of length 0"))
    }
    
    // See if the program is parsable
    let p = match parser::parse(program) {
      Ok(p) => p,
      Err(e) => return Err(e),
    };

    Ok(ProgramStack {
      program: p,
      inst_index: 0,
      tape: HashMap::new(),
      tape_index: 0,
      stdin: stdin(),
    })
  }

  pub fn from_file(filename: String) -> Result<ProgramStack, Error> {
    let program = match fs::read(filename) {
      Ok(s) => s,
      Err(e) => return Err(anyhow!(e)),
    };
    ProgramStack::from(program)
  }
  
  pub fn run(&mut self) -> Result<(), Error> {
    let len = self.program.len();
    loop {
      if self.inst_index >= len {
        return Ok(())
      }
      match &self.program[self.inst_index] {
        Command::IncrementPtr => {self.tape_index += 1;}
        Command::DecrementPtr => {
          // Make sure no negative bounds are indexed
          if self.tape_index > 0 {
            self.tape_index -= 1;
          } else {
            return Err(anyhow!("Tried to index a negative bound!"))
          }
        }
        Command::IncrementValue => {
          // Add value into tape if it's not 0 or change it if it's already there
          match self.tape.get_mut(&self.tape_index) {
            Some(b) => {*b = b.wrapping_add(1);}
            None => {self.tape.insert(self.tape_index, 1);}
          }
        }
        Command::DecrementValue => {
          // Remove value into tape if it's going to be 0 or change it if it's already there
          // or add it if it isn't
          let mut remove_key = false;
          match self.tape.get_mut(&self.tape_index) {
            Some(b) => {
              // Remove key since it would go to 0
              if *b == 1 {
                remove_key = true;
              } else {
                *b = b.wrapping_sub(1);
              }
            },
            None => {self.tape.insert(self.tape_index, 255);}
          }
          if remove_key {
            self.tape.remove(&self.tape_index);
          }
        }
        Command::Output => {
          print!(
            "{}",
            match self.tape.get(&self.tape_index) {
              Some(b) => *b,
              None => 0,
            } as char
          )
        }
        Command::Input => {
          match self.stdin.lock().bytes().next() {
            Some(b) =>  {
              match b {
                Ok(x) => self.tape.insert(self.tape_index, x),
                Err(e) => return Err(anyhow!(e)),
              };
            }
            None => return Err(anyhow!("No input could be obtained")),
          }
        }
        Command::JumpForward(j) => {
          if let None = self.tape.get(&self.tape_index) {
            // Going to increase instruction pointer by 1 more after so this is fine
            self.inst_index = j.pair_index;
          }
        }
        Command::JumpBackward(j) => {
          if let Some(_) = self.tape.get(&self.tape_index) {
            // Going to increase instruction pointer by 1 more after so this is fine
            self.inst_index = j.pair_index;
          }
        }
      }
      // Bump up instruction pointer by 1
      self.inst_index += 1;
    }
  }
  
  pub fn show_tape(&self) {
    println!("{:?}", self.tape);
  }
}

fn main() {
  match ProgramStack::from_file(String::from("test.bf")) {
    Ok(mut p) => {
      p.run().unwrap();
      p.show_tape();
    },
    Err(e) => println!("{}", e),
  };
}
