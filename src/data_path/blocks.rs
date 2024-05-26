use crate::buses::Buses;

use super::primitives::DataPathBlock;

pub struct DataPath {
  blocks: Vec<Box<dyn DataPathBlock>>,
}

impl DataPath {
  pub fn new(blocks: Vec<Box<dyn DataPathBlock>>) -> Self {
    DataPath {
      blocks: blocks
    }
  }

  pub fn update(&self, buses: &mut Buses) -> () {}
}
