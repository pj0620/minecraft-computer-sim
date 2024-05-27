#[derive(Debug)]
pub struct OpDefinition {
  pub operands: u8,
  pub name: &'static str,
  pub op_code: u8
}

pub const OPERATIONS: [OpDefinition; 4] = [
  OpDefinition { op_code: 0, operands: 1, name: "jmp0" },
  OpDefinition { op_code: 1, operands: 1, name: "jmp1" },
  OpDefinition { op_code: 2, operands: 1, name: "setR0" },
  OpDefinition { op_code: 3, operands: 1, name: "setR1" }
];
