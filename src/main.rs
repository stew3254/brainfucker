use std::fmt;

mod parser;

#[derive(thiserror::Error, Debug)]
enum Error {
  #[error("invalid program (expected {expected:?}, go {found:?})")]
  InvalidProgram {
    expected: String,
    found: String,
  }
}

#[derive(Debug)]
struct ProgramStack {
  program: String,
  inst_ptr: i32,
  data: Vec<i32>,
  data_ptr: i32,
}

impl ProgramStack {
  pub fn from(program: String) -> Result<ProgramStack, Error> {
    match program.len() {
      0 => Err(
        Error::InvalidProgram {
          expected: String::from("length to be greater than 0"),
          found: String::new()
        }
      ),
      _ => {
        if let Ok(_) = parser::parse(&program) {
          Ok(
            ProgramStack {
              program,
              inst_ptr: 0,
              data: vec![],
              data_ptr: 0
            }
          )
        }
      }
    }
  }
}

fn main() {
  ProgramStack::from(String::from(""));
}
