use anyhow::{anyhow, Error};

// Quickly parse program lint before trying to run
pub fn parse(p: &String) -> Result<(), Error> {
  let mut index_stack: Vec<char> = Vec::new();
  let mut loop_stack: Vec<char> = Vec::new();
  for c in p.chars() {
    match c {
      '>' => index_stack.push(c),
      '<' => {
        if let None = index_stack.pop() {
          return Err(anyhow!("cannot index negative memory on tape"));
        }
      }
      '[' => loop_stack.push(c),
      ']' => {
        if let None = loop_stack.pop() {
          return Err(anyhow!("unbalanced number of brackets: too many ']'"));
        }
      }
      // No special rules for these yet
      '+' => {}
      '-' => {}
      '.' => {}
      ',' => {}

      // Ignore all other characters since they are basically comments
      _ => {}
    }
  }
  if loop_stack.len() != 0 {
    Err(anyhow!("unbalanced number of brackets: too many '['"))
  } else {
    Ok(())
  }
}
