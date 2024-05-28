extern crate minecraft_computer_sim;

use std::env;
use std::time::Duration;

use minecraft_computer_sim::buses::Buses;
// use minecraft_computer_sim::control_plane::blocks::{doDecode, doExecute, doFetch};
use minecraft_computer_sim::control_plane::stages::{do_decode, do_execute, do_fetch};
use minecraft_computer_sim::data_path::blocks::{DataPath, DataPathBlock};
use minecraft_computer_sim::data_path::memory::{ProgramCounter, Ram, Rom};
use minecraft_computer_sim::data_path::registers::{RM, RT};
use minecraft_computer_sim::loader::file_loader::load_file;


const CLOCK_FREQ: f32 = 0.25;

fn main() -> Result<(), String> {
  let args: Vec<String> = env::args().collect();
  // let program_path = &args[1];
  let program_path = "programs/simple.mc";

  println!("running following program: {program_path}");
  let rom_img = load_file(&program_path)?;
  println!("final rom image: {:?}", rom_img);

  let wait_duration = Duration::from_secs_f32(0.33 / CLOCK_FREQ);

  let mut buses: Buses = Buses::new();

  let program_counter = ProgramCounter::new();
  let rom = Rom::new(program_counter, rom_img);
  let ram = Ram::new([0; 16]);
  let blocks: Vec<Box<dyn DataPathBlock>>  = vec![
    Box::new(rom),
    Box::new(ram),
    Box::new(RT::new()),
    Box::new(RM::new())
  ];

  let mut data_path = DataPath::new(blocks);

  loop {
    let cir = do_fetch(&mut buses, &mut data_path);
    std::thread::sleep(wait_duration);

    println!("Found current instruction = {cir}");

    let exec_func = do_decode(cir)?;
    std::thread::sleep(wait_duration);

    do_execute(&mut buses, &mut data_path, exec_func);
    std::thread::sleep(wait_duration);
  }
}
