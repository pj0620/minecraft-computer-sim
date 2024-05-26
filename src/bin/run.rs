extern crate minecraft_computer_sim;

use std::time::Duration;

use minecraft_computer_sim::buses::Buses;
// use minecraft_computer_sim::control_plane::blocks::{doDecode, doExecute, doFetch};
use minecraft_computer_sim::control_plane::blocks::doFetch;
use minecraft_computer_sim::data_path::blocks::DataPath;
use minecraft_computer_sim::data_path::memory::{ProgramCounter, Rom};
use minecraft_computer_sim::data_path::primitives::DataPathBlock;

const CLOCK_FREQ: f32 = 0.25;

fn main() {
  let wait_duration = Duration::from_secs_f32(0.33 / CLOCK_FREQ);

  let mut buses: Buses = Buses::new();

  let program_counter = ProgramCounter::new();
  let rom = Rom { program_counter: &program_counter };
  let data_path_blocks: Vec<Box<dyn DataPathBlock>>  = vec![
    Box::new(program_counter),
    Box::new(rom)
  ];

  let data_path = DataPath::new(data_path_blocks);

  loop {
    doFetch(&mut buses, &data_path);
    std::thread::sleep(wait_duration);

    // doDecode();
    // std::thread::sleep(wait_duration);

    // doExecute();
    // std::thread::sleep(wait_duration);
  }
}
