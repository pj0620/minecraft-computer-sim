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
  fn update(&mut self, buses: &mut Buses) -> Result<(), String> {
    println!("PC={}, MEM_SPACE={}", self.value, self.mem_space);

    if buses.fbus.pc_mem_set_0 && buses.fbus.pc_mem_set_1 {
      return Err(String::from("both pc_mem_set_0 and pc_mem_set_1 are true, it is impossible to set MEM_SPACE = 0 and MEM_SPACE = 1"))
    }

    if buses.fbus.pc_mem_set_0 {
      self.mem_space = 0;
    }

    if buses.fbus.pc_mem_set_1 {
      self.mem_space = 1;
    }

    if buses.fbus.pc_buf_set && buses.fbus.pc_set {
      return Err(String::from("both pc_buf_set and pc_set flags are set. This can lead to an infinite loop."));
    }

    // increment PC buffer
    if buses.fbus.pc_buf_set {
      self.buf_value = (self.value + 1) % 16;
    }

    if buses.fbus.pc_set && buses.fbus.pc_dbus_set {
      return Err(String::from("both pc_set and pc_dbus_set flags are set. It is not clear whether PC should be taken from PC_BUF or DBUS."));
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

    Ok(())
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
  fn update(&mut self, buses: &mut Buses) -> Result<(), String> {

    // Update Program Counter
    self.program_counter.update(buses)?;

    // if rom_en is true, put data on data bus
    if buses.fbus.rom_en {
      let addr = self.program_counter.value + 16 * self.program_counter.mem_space;
      buses.dbus = self.data[addr as usize];
    }

    Ok(())
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
  fn update(&mut self, buses: &mut Buses) -> Result<(), String>  {

    if buses.fbus.ram_en && buses.fbus.ram_rw {
      self.data[buses.abus as usize] = buses.dbus;
    }

    else if buses.fbus.ram_en && !buses.fbus.ram_rw {
      buses.dbus = self.data[buses.abus as usize];
    }

    Ok(())
  }
}
