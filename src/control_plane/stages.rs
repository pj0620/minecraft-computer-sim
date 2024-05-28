use crate::{buses::Buses, control_plane::execution::EXECUTE_FUNCS, data_path::blocks::DataPath};

use super::execution::ExecuteFunc;

pub fn do_fetch(buses: &mut Buses, data_path: &mut DataPath) -> u8 {
  println!("[Control Plane] Performing Fetch");

  // Grab Current Instruction
  buses.fbus.rom_en = true;
  data_path.update(buses);
  let cir = buses.dbus;
  buses.reset();

  // Set PC Buffer
  buses.fbus.pc_buf_set = true;
  data_path.update(buses);
  buses.reset();

  // Set PC
  buses.fbus.pc_set = true;
  data_path.update(buses);
  buses.reset();

  return cir;
}

pub fn do_decode(cir: u8) -> Result<ExecuteFunc, String> {
  println!("[Control Plane] Performing Decode");
  
  if cir < EXECUTE_FUNCS.len() as u8 {
    return Ok(EXECUTE_FUNCS[cir as usize]);
  }

  let error_msg = format!("no executor matched for opcode = {cir}");
  Err(error_msg) 
}

pub fn do_execute(buses: &mut Buses, data_path: &mut DataPath, execute_func: ExecuteFunc) {
  println!("[Control Plane] Performing Execute");

  execute_func(buses, data_path);
}
