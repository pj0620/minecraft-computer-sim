use crate::{buses::Buses, data_path::blocks::DataPath};

pub fn jmp0(buses: &mut Buses, data_path: &mut DataPath) -> Result<(), String> {
  // Store Jump Address in RM
  buses.fbus.rom_en = true;
  buses.fbus.rm_en = true;
  buses.fbus.rm_rw = true;
  data_path.update(buses)?;
  buses.reset();

  // Move Jump Address from RM into Program Counter
  buses.fbus.pc_dbus_set = true;
  buses.fbus.rm_en = true;
  data_path.update(buses)?;
  buses.reset();

  Ok(())
}