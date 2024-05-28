use crate::buses::Buses;


pub trait DataPathBlock {
  fn update(&mut self, buses: &mut Buses);
}

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

    for _ in 0..2 {
      for block in &mut self.blocks {
        block.update(buses)
      }
    }
  }
}