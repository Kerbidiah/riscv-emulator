// only impelementing I and M extentions

use arbitrary_int::u7;

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
	// op code bit patterns

	const R0: u7 = u7::new(0b011_0011);

	const I0: u7 = u7::new(0b001_0011);
	const I1: u7 = u7::new(0b000_0011);
	const I2: u7 = u7::new(0b110_0111);
	const I3: u7 = u7::new(0b111_0011);

	const S0: u7 = u7::new(0b010_0011);

	const B0: u7 = u7::new(0b110_0011);
	
	const U0: u7 = u7::new(0b011_0111);
	const U1: u7 = u7::new(0b001_0111);

	const J0: u7 = u7::new(0b1101111);
}

impl From<u7> for InstructionFormat {
	fn from(opcode: u7) -> Self {
		match opcode {
			Self::R0 => Self::R,
			Self::I0 | Self::I1 | Self::I2 | Self::I3 => Self::I,
			Self::S0 => Self::S,
			Self::B0 => Self::B,
			Self::U0 | Self::U1 => Self::U,
			Self::J0 => Self::J,
			_ => unimplemented!()
		}
	}
}

impl From<u8> for InstructionFormat {
	#[inline]
	fn from(value: u8) -> Self {
		u7::extract_u8(value, 0).into()
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