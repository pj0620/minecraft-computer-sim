use crate::{buses::Buses, data_path::blocks::DataPath};

use super::jump::jmp0;

pub type ExecuteFunc = fn(buses: &mut Buses, data_path: &mut DataPath) -> ();

pub static EXECUTE_FUNCS: [ExecuteFunc; 1] = [
  jmp0
];