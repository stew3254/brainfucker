use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct Jump {
  pub index: usize,
  pub pair_index: usize,
}

#[derive(Debug)]
pub enum Command {
  IncrementPtr,
  DecrementPtr,
  IncrementValue,
  DecrementValue,
  Output,
  Input,
  JumpForward(Jump),
  JumpBackward(Jump),
}

// Quickly parse program lint before trying to run
pub fn parse(p: Vec<u8>) -> Result<Vec<Command>, Error> {
  let mut program: Vec<Command> = Vec::new();
  let mut jump_stack: Vec<usize> = Vec::new();
  for c in p {
    match c {
      b'>' => program.push(Command::IncrementPtr),
      b'<' => program.push(Command::DecrementPtr),
      b'+' => program.push(Command::IncrementValue),
      b'-' => program.push(Command::DecrementValue),
      b'.' => program.push(Command::Output),
      b',' => program.push(Command::Input),
      b'[' => {
        // Keep location in program saved for when we find a matching brace
        jump_stack.push(program.len());
        
        // Add jump position to program
        program.push(
          Command::JumpForward(
            // Set index of where we are in the program
            Jump {
              index: if program.len() > 0 {
                program.len()-1
              } else {
                0
              },
              pair_index: 0,
            }
          )
        );
      }
      b']' => {
        let len = program.len();
        let mut pair_index: usize = 0;
        match jump_stack.pop() {
          Some(i) => {
            if let Command::JumpForward(j)  = &mut program[i] {
              j.pair_index = len;
              pair_index = j.index;
            }
            program.push(
              Command::JumpBackward(
                // Set index of where we are in the program
                Jump {
                  index: len,
                  pair_index,
                }
              )
            );
          }
          None => return Err(anyhow!("unbalanced number of brackets: too many ']'"))
        }
      }
      // Ignore all other characters since they are basically comments
      _ => {}
    }
  }
  if jump_stack.len() != 0 {
    Err(anyhow!("unbalanced number of brackets: too many '['"))
  } else {
    Ok(program)
  }
}
