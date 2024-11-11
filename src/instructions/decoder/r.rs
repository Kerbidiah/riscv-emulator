use crate::cpu_core::register::Register;
use crate::instructions::formats::InstructionFormat;

use bitbybit::{self, bitfield};
use arbitrary_int::{u3, u7};

#[derive(Debug)]
pub struct RType {
	pub format: InstructionFormat,
	pub opcode: u7,
	pub rd: Register,
	pub rs1: Register,
	pub rs2: Register,
	pub funct3: u3,
	pub funct7: u7,
}

impl From<RTypeCompact> for RType {
	fn from(value: RTypeCompact) -> Self {
		Self {
			format: value.opcode().into(),
			opcode: value.opcode(),
			rd: value.rd(),
			rs1: value.rs1(),
			rs2: value.rs2(),
			funct3: value.funct3(),
			funct7: value.funct7()
		}
	}
}

impl From<u32> for RType {
	fn from(value: u32) -> Self {
		RTypeCompact::from(value).into()
	}
}

#[bitfield(u32)]
pub struct RTypeCompact {
	#[bits(0..=6, r)]
	opcode: u7,

	#[bits(7..=11, r)]
	rd: Register,

	#[bits(12..=14, r)]
	funct3: u3,

	#[bits(15..=19, r)]
	rs1: Register,

	#[bits(20..=24, r)]
	rs2: Register,

	#[bits(25..=31, r)]
	funct7: u7,
}

impl From<u32> for RTypeCompact {
	#[inline]
	fn from(value: u32) -> Self {
		Self::new_with_raw_value(value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decode_r_0() {
		// add x13, x10, x11
		let instr_compact = RTypeCompact::new_with_raw_value(0x00b506b3);
		let instr: RType = instr_compact.into();

		// testing compact
		assert_eq!(instr_compact.opcode(), u7::new(0b011_0011));
		assert_eq!(instr_compact.rd(), Register::X13);
		assert_eq!(instr_compact.rs1(), Register::X10);
		assert_eq!(instr_compact.rs2(), Register::X11);
		assert_eq!(instr_compact.funct3(), u3::new(0));
		assert_eq!(instr_compact.funct7(), u7::new(0));

		// testing normal
		assert_eq!(instr.format, InstructionFormat::R);
		assert_eq!(instr_compact.opcode(), instr.opcode);
		assert_eq!(instr_compact.rd(), instr.rd);
		assert_eq!(instr_compact.rs1(), instr.rs1);
		assert_eq!(instr_compact.rs2(), instr.rs2);
		assert_eq!(instr_compact.funct3(), instr.funct3);
		assert_eq!(instr_compact.funct7(), instr.funct7);
	}


	#[ignore = "add more tests later"]
	#[test]
	fn decode_r_1() {
		todo!() // add more tests
	}

}