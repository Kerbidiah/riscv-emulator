use crate::cpu_core::register::Register;

#[derive(Debug)]
struct IType {
	opcode: u8,
	rd: Register,
	rs1: Register,
	funct3: u8,
	imm: u16
}