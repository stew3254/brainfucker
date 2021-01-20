use anyhow::{anyhow, Error};
use std::fs;

mod parser;

#[derive(Debug)]
enum Direction {
  Forward,
  Backward,
}

#[derive(Debug)]
struct Loop {
  instr_ptr: i32,
  jump_dir: Direction,
}

#[derive(Debug)]
struct ProgramStack {
  program: String,
  inst_ptr: i32,
  data: Vec<i8>,
  data_ptr: i32,
  stack: Vec<Loop>,
}

impl ProgramStack {
  pub fn from(program: String) -> Result<ProgramStack, Error> {
    match program.len() {
      0 => Err(anyhow::anyhow!("Program cannot be of length 0")),
      _ => {
        // See if the program is parsable
        if let Err(p) = parser::parse(&program) {
          return Err(p);
        }

        Ok(ProgramStack {
          program,
          inst_ptr: 0,
          data: Vec::with_capacity(32),
          data_ptr: 0,
          stack: Vec::new(),
        })
      }
    }
  }

  pub fn from_file(filename: String) -> Result<ProgramStack, Error> {
    let program = match fs::read_to_string(filename) {
      Ok(s) => s,
      Err(e) => return Err(anyhow!(e)),
    };
    ProgramStack::from(program)
  }
  
  pub fn run(&self) -> Result<(), Error> {
    Ok(())
  }

}

fn main() {
  println!("{:?}", ProgramStack::from_file(String::from("test.bf")));
}
