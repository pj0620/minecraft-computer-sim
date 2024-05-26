use crate::buses::Fbus;

pub trait ControlPlaneBlock {
  fn update(&self, fbus: &mut Fbus);
}