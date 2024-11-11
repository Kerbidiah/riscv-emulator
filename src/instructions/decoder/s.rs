use crate::cpu_core::register::Register;
use crate::instructions::formats::InstructionFormat;

use bitbybit::{self, bitfield};
use arbitrary_int::{u3, u7, u12};

#[derive(Debug)]
pub struct SType {
	pub format: InstructionFormat,
	pub opcode: u7,
	pub rs1: Register, // address
	pub rs2: Register, // value
	pub funct3: u3,
	pub imm: u12 // acts as offset from address
}

impl SType {
	/// sign extends imm
	pub fn imm_val(&self) -> i32 {
		super::imm::sign_extend_i32(self.imm)
	}
}

impl From<STypeCompact> for SType {
	fn from(value: STypeCompact) -> Self {
		Self {
			format: value.opcode().into(),
			opcode: value.opcode(),
			rs1: value.rs1(),
			rs2: value.rs2(),
			funct3: value.funct3(),
			imm: value.imm()
		}
	}
}

impl From<u32> for SType {
	fn from(value: u32) -> Self {
		STypeCompact::from(value).into()
	}
}

#[bitfield(u32)]
pub struct STypeCompact {
	#[bits(0..=6, r)]
	opcode: u7,

	#[bits([7..=11, 25..=31], r)]
	imm: u12,

	#[bits(12..=14, r)]
	funct3: u3,

	#[bits(15..=19, r)]
	rs1: Register,

	#[bits(20..=24, r)]
	rs2: Register,
}

impl From<u32> for STypeCompact {
	#[inline]
	fn from(value: u32) -> Self {
		Self::new_with_raw_value(value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decode_s_0() {
		// sw x19, 2000(x15)
		let instr_compact = STypeCompact::new_with_raw_value(0x7d37a823);
		let instr: SType = instr_compact.into();

		// testing compact
		assert_eq!(instr_compact.opcode(), u7::new(0b010_0011));
		assert_eq!(instr_compact.rs1(), Register::X15);
		assert_eq!(instr_compact.rs2(), Register::X19);
		assert_eq!(instr_compact.funct3(), u3::new(0x2));
		assert_eq!(instr_compact.imm(), u12::new(2000));

		// testing normal
		assert_eq!(instr.format, InstructionFormat::S);
		assert_eq!(instr_compact.opcode(), instr.opcode);
		assert_eq!(instr_compact.rs1(), instr.rs1);
		assert_eq!(instr_compact.rs2(), instr.rs2);
		assert_eq!(instr_compact.funct3(), instr.funct3);
		assert_eq!(instr_compact.imm(), instr.imm);
	}


	#[ignore = "add more tests later"]
	#[test]
	fn decode_s_1() {
		todo!() // add more tests
	}

}