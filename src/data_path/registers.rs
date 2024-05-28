use crate::buses::{Buses, Fbus};

use super::blocks::DataPathBlock;

pub enum DataSource {
  DBUS,
  ABUS
}

// Utility functions for defining new registers
type DataSourceFn = fn(buses: &Buses) -> u8;
const FROM_DBUS: DataSourceFn = |buses: &Buses| buses.dbus;
// const FROM_ABUS: DataSourceFn = |buses: &Buses| buses.abus;

type DataSinkFn = fn(buses: &mut Buses, new_data: u8) -> ();
const TO_DBUS: DataSinkFn = |buses: &mut Buses, new_data: u8| buses.dbus = new_data;
const TO_ABUS: DataSinkFn = |buses: &mut Buses, new_data: u8| buses.abus = new_data;

pub struct Register {
  data: u8,
  rw_flag_func: fn(fbus: &Fbus) -> bool,
  en_flag_func: fn(fbus: &Fbus) -> bool,
  input_func: DataSourceFn,
  output_func: DataSinkFn
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
}

pub struct RT { register: Register }
impl RT  {
  pub fn new() -> Self {
    RT {
      register: Register { 
        data: 0, 
        rw_flag_func: |fbus: &Fbus| fbus.rt_rw,
        en_flag_func: |fbus: &Fbus| fbus.rt_en,
        input_func: FROM_DBUS,
        output_func: TO_DBUS
      }
    }
  }
}
impl DataPathBlock for RT {
  fn update(&mut self, buses: &mut Buses) -> Result<(), String> {self.register.update(buses); Ok(())}
}

pub struct RM { register: Register }
impl RM  {
  pub fn new() -> Self {
    RM {
      register: Register { 
        data: 0, 
        rw_flag_func: |fbus: &Fbus| fbus.rm_rw,
        en_flag_func: |fbus: &Fbus| fbus.rm_en,
        input_func: FROM_DBUS,
        output_func: TO_ABUS
      }
    }
  }
}
impl DataPathBlock for RM {
  fn update(&mut self, buses: &mut Buses) -> Result<(), String> {self.register.update(buses); Ok(())}
}