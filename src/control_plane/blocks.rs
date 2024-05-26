use crate::{buses::Buses, data_path::blocks::DataPath};

pub fn doFetch(buses: &mut Buses, data_path: &DataPath) -> u8 {
  println!("[Control Plane] Performing Fetch");

  buses.fbus.rom_en = true;
  data_path.update(buses);
  let cir = buses.dbus;

  print!("found current instruction = {cir}");
  return 0;
}

pub fn doDecode() {
  println!("[Control Plane] Performing Decode");
}

pub fn doExecute() {
  println!("[Control Plane] Performing Execute");
}
