use std::fs;

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
    if (operation_res.is_err()) {
      let error_message = format!("error on line {} -> {}",
        original_line, operation_res.unwrap_err());
      return Err(error_message)
    }
  }

  Ok(rom_data)
}
