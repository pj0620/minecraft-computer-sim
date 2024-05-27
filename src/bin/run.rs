extern crate minecraft_computer_sim;

use std::env;
use std::time::Duration;

use minecraft_computer_sim::buses::Buses;
// use minecraft_computer_sim::control_plane::blocks::{doDecode, doExecute, doFetch};
use minecraft_computer_sim::control_plane::stages::do_fetch;
use minecraft_computer_sim::data_path::blocks::DataPath;
use minecraft_computer_sim::data_path::memory::{ProgramCounter, Rom};
use minecraft_computer_sim::{data_path::primitives::DataPathBlock, loader::file_loader::load_file};

const CLOCK_FREQ: f32 = 0.25;

fn main() -> Result<(), String> {
  let args: Vec<String> = env::args().collect();
  let program_path = &args[1];
  println!("running following program: {program_path}");
  let rom_img = load_file(&program_path)?;
  println!("final rom image: {:?}", rom_img);

  let wait_duration = Duration::from_secs_f32(0.33 / CLOCK_FREQ);

  let mut buses: Buses = Buses::new();

  let program_counter = ProgramCounter::new();
  let rom = Rom::new(program_counter, rom_img);
  let blocks: Vec<Box<dyn DataPathBlock>>  = vec![
    Box::new(rom)
  ];

  let mut data_path = DataPath::new(blocks);

  loop {
    let cir = do_fetch(&mut buses, &mut data_path);
    std::thread::sleep(wait_duration);

    println!("Found current instruction = {cir}");

    // doDecode();
    // std::thread::sleep(wait_duration);

    // doExecute();
    // std::thread::sleep(wait_duration);
  }
}
