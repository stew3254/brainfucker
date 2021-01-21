use anyhow::{anyhow, Error};
use std::fs;
use std::io::{stdin, Stdin, Read};
use crate::parser::Command;

mod parser;

#[derive(Debug)]
struct ProgramStack {
  program: Vec<Command>,
  inst_index: usize,
  tape: Vec<u8>,
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
      tape: vec![0; 32],
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
          if self.tape_index < self.tape.len() {
            // Make sure to reserve enough space
            self.tape.reserve(self.tape.len() - self.tape_index);
            // Write out rest of 0 bytes
            for i in 0..self.tape.len() {
              self.tape[i] = 0;
            }
          }
          self.tape[self.tape_index] = self.tape[self.tape_index].wrapping_add(1);
        }
        Command::DecrementValue => {
          self.tape[self.tape_index] = self.tape[self.tape_index].wrapping_sub(1);
        }
        Command::Output => print!("{}", self.tape[self.tape_index]),
        Command::Input => {
          self.tape[self.tape_index] = match self.stdin.lock().bytes().next() {
            Some(b) =>  {
              match b {
                Ok(x) => x, 
                Err(e) => return Err(anyhow!(e)),
              }
            }
            None => return Err(anyhow!("No input could be obtained")),
          }
        }
        Command::JumpForward(j) => {
          if self.tape[self.tape_index] == 0 {
            // Going to increase instruction pointer by 1 more after so this is fine
            self.inst_index = j.pair_index;
          }
        }
        Command::JumpBackward(j) => {
          if self.tape[self.tape_index] != 0 {
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
