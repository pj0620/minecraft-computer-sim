#[derive(Debug)]
pub struct Fbus {

  // Program Counter
  pub pc_set: bool,
  pub pc_buf_set: bool,
  pub pc_dbus_set: bool,
  pub pc_mem_set_0: bool,
  pub pc_mem_set_1: bool,

  // ROM
  pub rom_en: bool,

  // RAM
  pub ram_en: bool,
  pub ram_rw: bool,

  // RT
  pub rt_en: bool,
  pub rt_rw: bool,

  // RM
  pub rm_en: bool,
  pub rm_rw: bool,

}
impl Fbus {
  pub fn new() -> Self {
    Fbus {
      // Program Counter
      pc_set: false,
      pc_buf_set: false,
      pc_dbus_set: false,
      pc_mem_set_0: false,
      pc_mem_set_1: false,

      // ROM
      rom_en: false,

      // RAM
      ram_en: false,
      ram_rw: false,

      rt_en: false,
      rt_rw: false,

      rm_en: false,
      rm_rw: false,
    }
  }

  pub fn reset_flags(&mut self) {
    self.pc_set = false;
    self.pc_buf_set = false;
    self.pc_dbus_set = false;
    self.pc_mem_set_0 = false;
    self.pc_mem_set_1 = false;
    self.rom_en = false;
    self.ram_en = false;
    self.ram_rw = false;
    self.rt_en = false;
    self.rt_rw = false;
    self.rm_en = false;
    self.rm_rw = false;
  }
}

#[derive(Debug)]
pub struct Buses {
  pub dbus: u8,
  pub abus: u8,
  pub fbus: Fbus
}

impl Buses {
  pub fn new() -> Self {
    Buses {
      dbus: 0,
      abus: 0,
      fbus: Fbus::new()
    }
  }

  pub fn reset(&mut self) {
    self.abus = 0;
    self.dbus = 0;
    self.fbus.reset_flags();
  }
}