use std::{error, fs};

use super::ops::{OpDefinition, OPERATIONS};

fn get_operation(operation_name: &str) -> Result<&OpDefinition, String> {
  for op in &OPERATIONS {
    if (op.name == operation_name) {
      return Ok(op);
    }
  }
  let error_msg = format!("invalid operation {}", operation_name);
  Err(error_msg)
}

pub fn load_file(program_path: &str) -> Result<[u8; 32], String> {
  let error_message = format!("Cannot read file {}", program_path);
  let content = fs::read_to_string(program_path)
    .expect(&error_message);
  let lines: Vec<&str> = content.lines()
    .map(|x| x.trim())
    .collect();

  // let line_num: usize = 0;
  let mut rom_data = [0; 32];
  for (original_line, line) in lines.iter().enumerate() {
    if (line.is_empty() || line.starts_with("#")) {
      continue;
    }

    let line_arr: Vec<&str> = line.split_whitespace().collect();
    let operation_res = get_operation(line_arr[0]);
    let op = get_operation(line_arr[0])
      .map_err(|error| {
        format!("error on line {} -> {}", original_line, error)
      })?;


    let operands = op.operands;
    if line_arr.len() as u8 != operands + 1 {
      return Err(
        format!("error on line {} -> incorrect operands for {} expected {} recieved {}", 
          original_line, op.name, op.operands, line_arr.len() - 1)
      );
    }
  }

  Ok(rom_data)
}
