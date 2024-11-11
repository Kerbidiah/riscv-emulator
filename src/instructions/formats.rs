// only impelementing I and M extentions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionFormat {
	R,
	I,
	S,
	B,
	U,
	J
}

impl InstructionFormat {
	pub const MASK: u8 = 0b0111_1111;
}

impl From<u8> for InstructionFormat {
	fn from(value: u8) -> Self {
		let opcode = value & Self::MASK;
		match opcode {
			0b0011_0011 => Self::R,
			0b0001_0011 | 0b0000_0011 | 0b0110_0111 | 0b0111_0011 => Self::I,
			0b0010_0011 => Self::S,
			0b0110_0011 => Self::B,
			0b0011_0111 | 0b0001_0111 => Self::U,
			0b01101111 => Self::J,

			0b1000_0000..=u8::MAX => unreachable!(),
			_ => unimplemented!()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use InstructionFormat as InstrFmt;

	#[test]
	fn opcode_conversion() {
		assert_eq!(InstrFmt::R, 0xB3.into());

		assert_eq!(InstrFmt::I, 0x93.into());
		assert_eq!(InstrFmt::I, 0x03.into());
		assert_eq!(InstrFmt::I, 0x67.into());
		assert_eq!(InstrFmt::I, 0x73.into());

		assert_eq!(InstrFmt::S, 0xA3.into());
	
		assert_eq!(InstrFmt::B, 0x63.into());

		assert_eq!(InstrFmt::U, 0xB7.into());
		assert_eq!(InstrFmt::U, 0x97.into());
		
		assert_eq!(InstrFmt::J, 0xEF.into());
	}
}