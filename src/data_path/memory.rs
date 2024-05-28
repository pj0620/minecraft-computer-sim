use crate::buses::Buses;

use super::blocks::DataPathBlock;

pub struct ProgramCounter {
  pub buf_value: u8,
  pub value: u8,
  pub mem_space: u8
}

impl ProgramCounter {
  pub fn new() -> Self {
    ProgramCounter {
      buf_value: 0,
      value: 0,
      mem_space: 0
    }
  }
}

impl DataPathBlock for ProgramCounter {
  fn update(&mut self, buses: &mut Buses) {
    println!("PC={}, MEM_SPACE={}", self.value, self.mem_space);

    // increment PC buffer
    if buses.fbus.pc_buf_set {
      self.buf_value = (self.value + 1) % 16;
    }

    // set PC to buffer
    if buses.fbus.pc_set {

      // flip memory banks if at the top of memory
      if self.value == 15 && self.buf_value == 0 {
        self.mem_space = (self.mem_space + 1) % 2;
      }

      self.value = self.buf_value;
    }

    else if buses.fbus.pc_dbus_set {
      self.value = buses.dbus;
    }
  }
}

pub struct Rom {
  program_counter: ProgramCounter,
  data: [u8; 32]
}

impl Rom  {
  pub fn new(program_counter: ProgramCounter, data: [u8; 32]) -> Self {
    Rom {
      program_counter,
      data
    }
  }
}

impl DataPathBlock for Rom {
  fn update(&mut self, buses: &mut Buses) {

    // Update Program Counter
    self.program_counter.update(buses);

    // if rom_en is true, put data on data bus
    if buses.fbus.rom_en {
      buses.dbus = self.data[self.program_counter.value as usize];
    }
  }
}

pub struct Ram {
  data: [u8; 16]
}

impl Ram  {
  pub fn new(data: [u8; 16]) -> Self {
    Ram {
      data
    }
  }
}

impl DataPathBlock for Ram {
  fn update(&mut self, buses: &mut Buses) {

    if buses.fbus.ram_en && buses.fbus.ram_rw {
      self.data[buses.abus as usize] = buses.dbus;
    }

    else if buses.fbus.ram_en && !buses.fbus.ram_rw {
      buses.dbus = self.data[buses.abus as usize];
    }
  }
}
