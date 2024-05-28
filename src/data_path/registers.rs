use crate::buses::{Buses, Fbus};

use super::blocks::DataPathBlock;

// Utility functions for defining new registers
pub type DataSourceFn = fn(buses: &Buses) -> u8;
pub const FROM_DBUS: DataSourceFn = |buses: &Buses| buses.dbus;
pub const FROM_ABUS: DataSourceFn = |buses: &Buses| buses.abus;

pub type DataSinkFn = fn(buses: &mut Buses, new_data: u8) -> ();
pub const TO_DBUS: DataSinkFn = |buses: &mut Buses, new_data: u8| buses.dbus = new_data;
pub const TO_ABUS: DataSinkFn = |buses: &mut Buses, new_data: u8| buses.abus = new_data;

pub struct Register {
  data: u8,
  rw_flag_func: fn(fbus: &Fbus) -> bool,
  en_flag_func: fn(fbus: &Fbus) -> bool,
  input_func: DataSourceFn,
  output_func: DataSinkFn
}

impl DataPathBlock for Register {
  fn update(&mut self, buses: &mut Buses) -> Result<(), String> {
    self.update(buses); 
    Ok(())
  }
}

impl Register {
  pub fn update(&mut self, buses: &mut Buses) {
    let rw_flag = (self.rw_flag_func)(&buses.fbus);
    let en_flag = (self.en_flag_func)(&buses.fbus);

    let new_input = (self.input_func)(&buses);

    if rw_flag && en_flag {
      self.data = new_input;
    }

    else if !rw_flag && en_flag {
      (self.output_func)(buses, self.data);
    }
  }

  pub fn new(
    rw_flag_func: fn(fbus: &Fbus) -> bool,
    en_flag_func: fn(fbus: &Fbus) -> bool,
    input_func: DataSourceFn,
    output_func: DataSinkFn
  ) -> Self{
    Register {
      data: 0,
      rw_flag_func,
      en_flag_func,
      input_func,
      output_func
    }
  }
}
