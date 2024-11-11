use std::ops::{Index, IndexMut};

use bitbybit::{self, bitenum};
use arbitrary_int;

#[derive(Debug, Default)]
pub struct Registers {
	// x0: u32, // should always be 0
	dummy_register: u32, // exists so X0 writes can write to something
	x1: u32,
	x2: u32,
	x3: u32,
	x4: u32,
	x5: u32,
	x6: u32,
	x7: u32,
	x8: u32,
	x9: u32,
	x10: u32,
	x11: u32,
	x12: u32,
	x13: u32,
	x14: u32,
	x15: u32,
	x16: u32,
	x17: u32,
	x18: u32,
	x19: u32,
	x20: u32,
	x21: u32,
	x22: u32,
	x23: u32,
	x24: u32,
	x25: u32,
	x26: u32,
	x27: u32,
	x28: u32,
	x29: u32,
	x30: u32,
	x31: u32,

}

impl Index<Register> for Registers {
	type Output = u32;

	fn index(&self, index: Register) -> &Self::Output {
		match index {
			Register::X0 => &0,
			Register::X1 => &self.x1,
			Register::X2 => &self.x2,
			Register::X3 => &self.x3,
			Register::X4 => &self.x4,
			Register::X5 => &self.x5,
			Register::X6 => &self.x6,
			Register::X7 => &self.x7,
			Register::X8 => &self.x8,
			Register::X9 => &self.x9,
			Register::X10 => &self.x10,
			Register::X11 => &self.x11,
			Register::X12 => &self.x12,
			Register::X13 => &self.x13,
			Register::X14 => &self.x14,
			Register::X15 => &self.x15,
			Register::X16 => &self.x16,
			Register::X17 => &self.x17,
			Register::X18 => &self.x18,
			Register::X19 => &self.x19,
			Register::X20 => &self.x20,
			Register::X21 => &self.x21,
			Register::X22 => &self.x22,
			Register::X23 => &self.x23,
			Register::X24 => &self.x24,
			Register::X25 => &self.x25,
			Register::X26 => &self.x26,
			Register::X27 => &self.x27,
			Register::X28 => &self.x28,
			Register::X29 => &self.x29,
			Register::X30 => &self.x30,
			Register::X31 => &self.x31,
		}
	}
}

impl IndexMut<Register> for Registers {
	fn index_mut(&mut self, index: Register) -> &mut Self::Output {
		match index {
			Register::X0 => &mut self.dummy_register,
			Register::X1 => &mut self.x1,
			Register::X2 => &mut self.x2,
			Register::X3 => &mut self.x3,
			Register::X4 => &mut self.x4,
			Register::X5 => &mut self.x5,
			Register::X6 => &mut self.x6,
			Register::X7 => &mut self.x7,
			Register::X8 => &mut self.x8,
			Register::X9 => &mut self.x9,
			Register::X10 => &mut self.x10,
			Register::X11 => &mut self.x11,
			Register::X12 => &mut self.x12,
			Register::X13 => &mut self.x13,
			Register::X14 => &mut self.x14,
			Register::X15 => &mut self.x15,
			Register::X16 => &mut self.x16,
			Register::X17 => &mut self.x17,
			Register::X18 => &mut self.x18,
			Register::X19 => &mut self.x19,
			Register::X20 => &mut self.x20,
			Register::X21 => &mut self.x21,
			Register::X22 => &mut self.x22,
			Register::X23 => &mut self.x23,
			Register::X24 => &mut self.x24,
			Register::X25 => &mut self.x25,
			Register::X26 => &mut self.x26,
			Register::X27 => &mut self.x27,
			Register::X28 => &mut self.x28,
			Register::X29 => &mut self.x29,
			Register::X30 => &mut self.x30,
			Register::X31 => &mut self.x31,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
#[bitenum(u5, exhaustive = true)]
pub enum Register {
	X0 = 0,
	X1 = 1,
	X2 = 2,
	X3 = 3,
	X4 = 4,
	X5 = 5,
	X6 = 6,
	X7 = 7,
	X8 = 8,
	X9 = 9,
	X10 = 10,
	X11 = 11,
	X12 = 12,
	X13 = 13,
	X14 = 14,
	X15 = 15,
	X16 = 16,
	X17 = 17,
	X18 = 18,
	X19 = 19,
	X20 = 20,
	X21 = 21,
	X22 = 22,
	X23 = 23,
	X24 = 24,
	X25 = 25,
	X26 = 26,
	X27 = 27,
	X28 = 28,
	X29 = 29,
	X30 = 30,
	X31 = 31,
}

impl Register {
	const MASK: u8 = 0b0001_1111;
}

// TODO: is there a better way?
impl From<u8> for Register {
	fn from(value: u8) -> Self {
		match value & Self::MASK {
			0 => Register::X0,
			1 => Register::X1,
			2 => Register::X2,
			3 => Register::X3,
			4 => Register::X4,
			5 => Register::X5,
			6 => Register::X6,
			7 => Register::X7,
			8 => Register::X8,
			9 => Register::X9,
			10 => Register::X10,
			11 => Register::X11,
			12 => Register::X12,
			13 => Register::X13,
			14 => Register::X14,
			15 => Register::X15,
			16 => Register::X16,
			17 => Register::X17,
			18 => Register::X18,
			19 => Register::X19,
			20 => Register::X20,
			21 => Register::X21,
			22 => Register::X22,
			23 => Register::X23,
			24 => Register::X24,
			25 => Register::X25,
			26 => Register::X26,
			27 => Register::X27,
			28 => Register::X28,
			29 => Register::X29,
			30 => Register::X30,
			31 => Register::X31,
			_ => unreachable!()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::prelude::*;

	#[test]
	fn u8_to_register() {
		assert_eq!(Register::X0, 0.into());
		assert_eq!(Register::X1, 1.into());
		assert_eq!(Register::X2, 2.into());
		assert_eq!(Register::X3, 3.into());
		assert_eq!(Register::X4, 4.into());
		assert_eq!(Register::X5, 5.into());
		assert_eq!(Register::X6, 6.into());
		assert_eq!(Register::X7, 7.into());
		assert_eq!(Register::X8, 8.into());
		assert_eq!(Register::X9, 9.into());
		assert_eq!(Register::X10, 10.into());
		assert_eq!(Register::X11, 11.into());
		assert_eq!(Register::X12, 12.into());
		assert_eq!(Register::X13, 13.into());
		assert_eq!(Register::X14, 14.into());
		assert_eq!(Register::X15, 15.into());
		assert_eq!(Register::X16, 16.into());
		assert_eq!(Register::X17, 17.into());
		assert_eq!(Register::X18, 18.into());
		assert_eq!(Register::X19, 19.into());
		assert_eq!(Register::X20, 20.into());
		assert_eq!(Register::X21, 21.into());
		assert_eq!(Register::X22, 22.into());
		assert_eq!(Register::X23, 23.into());
		assert_eq!(Register::X24, 24.into());
		assert_eq!(Register::X25, 25.into());
		assert_eq!(Register::X26, 26.into());
		assert_eq!(Register::X27, 27.into());
		assert_eq!(Register::X28, 28.into());
		assert_eq!(Register::X29, 29.into());
		assert_eq!(Register::X30, 30.into());
		assert_eq!(Register::X31, 31.into());
	}

	#[test]
	fn register_write_test() {
		let mut regs = Registers::default();
		let mut rng = thread_rng();

		// test the 0 register
		regs[Register::X0] = rng.gen();
		assert_eq!(regs[Register::X0], 0);

		// test the other registers
		for r in 1..32 {
			let reg = r.into();
			let val = rng.gen();

			regs[reg] = val;
			assert_eq!(regs[reg], val);
		}
	}
}