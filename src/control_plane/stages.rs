use crate::{buses::Buses, data_path::blocks::DataPath};

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

pub fn doDecode() {
  println!("[Control Plane] Performing Decode");
}

pub fn doExecute() {
  println!("[Control Plane] Performing Execute");
}
