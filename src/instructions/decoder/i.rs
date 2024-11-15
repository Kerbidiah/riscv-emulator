use crate::cpu_core::register::Register;
use crate::instructions::formats::InstructionFormat;

use bitbybit::{self, bitfield};
use arbitrary_int::{u12, u3, u7};

#[derive(Debug)]
pub struct IType {
	pub format: InstructionFormat,
	pub opcode: u7,
	pub rd: Register,
	pub rs1: Register,
	pub funct3: u3,
	pub imm: u12,
}

impl IType {
	/// sign extends imm
	pub fn imm_val(&self) -> i32 {
		super::imm::sign_extend_i32(self.imm)
	}
}

impl From<ITypeCompact> for IType {
	fn from(value: ITypeCompact) -> Self {
		Self {
			format: value.opcode().into(),
			opcode: value.opcode(),
			rd: value.rd(),
			rs1: value.rs1(),
			funct3: value.funct3(),
			imm: value.imm()
		}
	}
}

impl From<u32> for IType {
	fn from(value: u32) -> Self {
		ITypeCompact::from(value).into()
	}
}

#[bitfield(u32)]
pub struct ITypeCompact {
	#[bits(0..=6, r)]
	opcode: u7,

	#[bits(7..=11, r)]
	rd: Register,

	#[bits(12..=14, r)]
	funct3: u3,

	#[bits(15..=19, r)]
	rs1: Register,

	#[bits(20..=31, r)]
	imm: u12
}

impl From<u32> for ITypeCompact {
	#[inline]
	fn from(value: u32) -> Self {
		Self::new_with_raw_value(value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decode_i_0() {
		// andi x11, x13, 999
		let instr_compact = ITypeCompact::new_with_raw_value(0x3e76f593);
		let instr: IType = instr_compact.into();

		// testing compact
		assert_eq!(instr_compact.opcode(), u7::new(0b001_0011));
		assert_eq!(instr_compact.rd(), Register::X11);
		assert_eq!(instr_compact.rs1(), Register::X13);
		assert_eq!(instr_compact.funct3(), u3::new(0x7));
		assert_eq!(instr_compact.imm(), u12::new(999));

		// testing normal
		assert_eq!(instr.format, InstructionFormat::I);
		assert_eq!(instr_compact.opcode(), instr.opcode);
		assert_eq!(instr_compact.rd(), instr.rd);
		assert_eq!(instr_compact.rs1(), instr.rs1);
		assert_eq!(instr_compact.funct3(), instr.funct3);
		assert_eq!(instr_compact.imm(), instr.imm);
	}


	#[ignore = "add more tests later"]
	#[test]
	fn decode_i_1() {
		todo!() // add more tests
	}
}