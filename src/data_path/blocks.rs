use crate::buses::Buses;

use super::primitives::DataPathBlock;

pub struct DataPath {
  blocks: Vec<Box<dyn DataPathBlock>>,
}

impl DataPath {
  pub fn new(blocks: Vec<Box<dyn DataPathBlock>>) -> Self {
    DataPath {
      blocks
    }
  }

  pub fn update(&mut self, buses: &mut Buses) -> () {
    for block in &mut self.blocks {
      block.update(buses)
    }
  }
}
