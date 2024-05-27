use crate::buses::Buses;

use super::primitives::DataPathBlock;

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

      println!("PC={}, MEM_SPACE={}", self.value, self.mem_space);
    }
  }
}

pub struct Rom {
  program_counter: ProgramCounter
}

impl Rom  {
  pub fn new(program_counter: ProgramCounter) -> Self {
    Rom {
      program_counter
    }
  }
}

impl DataPathBlock for Rom {
  fn update(&mut self, buses: &mut Buses) {

    // Update Program Counter
    self.program_counter.update(buses);
  }
}
