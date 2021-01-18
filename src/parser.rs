use crate::Error;
use crate::Error::InvalidProgram;

pub fn parse(p: &String) -> Result<(), Error>{
  for c in p.chars() {
    match c {
      '>' => {}
      '<' => {}
      '+' => {}
      '-' => {}
      '.' => {}
      ',' => {}
      '[' => {}
      ']' => {}
      c if c.is_whitespace() => {}
      _ => {
        return Err(InvalidProgram {
          expected: "valid character",
          found: "invalid character",
        })
      }
    }
  }
}