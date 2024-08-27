use serde::{Deserialize, Serialize};


// LoongArch ISA
#[derive(Debug, Serialize, Deserialize)]
pub struct LAISA {
  pub insts: Vec<LAInst>,
  pub info: String,
}

// LoongArch Instruction
#[derive(Debug, Serialize, Deserialize)]
pub struct LAInst {
  pub name: String,
  pub format: Vec<String>,
  pub postfix: Vec<String>,
  pub detail: String
}