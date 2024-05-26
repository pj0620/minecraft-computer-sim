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
    if (buses.fbus.pc_buf_set) {
      self.buf_value = (self.value + 1) % 16;
    }

    if (buses.fbus.pc_set) {
      if (self.value == 15 && self.buf_value == 0) {
        self.mem_space = (self.mem_space + 1) % 2;
      }

      self.value = self.buf_value;
    }
  }
}

pub struct Rom {
  pub program_counter: Box<ProgramCounter>
}

impl DataPathBlock for Rom {
  fn update(&mut self, buses: &mut Buses) {
    
  }
}
