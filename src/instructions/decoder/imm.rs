use std::ops::Not;

use arbitrary_int as arb;

pub trait SignExtendable: arb::Number + Into<u32> + Copy + Not<Output = Self> {}
impl<T> SignExtendable for T where T: arb::Number + Into<u32> + Copy + Not<Output = T> {}


pub fn sign_extend_u32<T: SignExtendable>(imm: T) -> u32 {
	if bool::from(arb::u1::extract_u32(imm.into(), T::BITS - 1)) { // first bit is 1
		u32::MAX ^ imm.not().into()
	} else {
		imm.into()
	}
}

pub fn sign_extend_i32<T: SignExtendable>(imm: T) -> i32 {
	sign_extend_u32(imm) as i32
}

#[cfg(test)]
mod tests {
	use super::{sign_extend_u32, sign_extend_i32};
	use super::arb::{u1, u12};

	#[test]
	fn sign_extention_u1() {
		let a = u1::new(0);
		let b = u1::new(1);

		assert_eq!(sign_extend_u32(a), u32::MIN);
		assert_eq!(sign_extend_u32(b), u32::MAX);
	}

	#[test]
	fn sign_extention_u12() {
		let a = u12::new(2047);
		let b = u12::new(0b1000_0000_0000); // -2048
		let c = u12::new(4095); // all ones, ans: -1

		assert_eq!(sign_extend_i32(a), 2047);
		assert_eq!(sign_extend_i32(b), -2048);
		assert_eq!(sign_extend_i32(c), -1);
	}
}