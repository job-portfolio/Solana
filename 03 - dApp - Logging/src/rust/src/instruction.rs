use std::convert::TryInto;
use solana_program::{
  program_error::ProgramError
};

#[derive(Debug)]
pub enum Instruction {
  Increment,
  Decrement,
  SetValue(u32)
}

impl Instruction {
  pub fn decode( input: &[u8] ) -> Result<Self, ProgramError> {
    let (&inst, data) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    match inst {
      0 => return Ok(Instruction::Increment),
      1 => return Ok(Instruction::Decrement),
      2 => {
        if data.len() != 4 {
          return Err(ProgramError::InvalidInstructionData);
        }
        let v: Result<[u8; 4], _> = data[..4].try_into();
        match v {
          Ok(i) => {
            return Ok(Instruction::SetValue(u32::from_le_bytes(i)));
          },
          _ => {
            return Err(ProgramError::InvalidInstructionData);
          }
        }
      },
      _ => Err(ProgramError::InvalidInstructionData)
    }
  }
}

