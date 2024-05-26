use crate::buses::Buses;

pub trait DataPathBlock {
  fn update(&mut self, buses: &mut Buses);
}