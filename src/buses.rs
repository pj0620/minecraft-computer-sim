pub struct Fbus {

  // Program Counter
  pub pc_set: bool,
  pub pc_buf_set: bool,

  // ROM
  pub rom_en: bool,

  // RAM
  pub ram_en: bool,
  pub ram_rw: bool,

}
impl Fbus {
  pub fn new() -> Self {
    Fbus {
      // Program Counter
      pc_set: false,
      pc_buf_set: false,

      // ROM
      rom_en: false,

      // RAM
      ram_en: false,
      ram_rw: false,
    }
  }
}

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
}