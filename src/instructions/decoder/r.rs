use crate::cpu_core::register::Register;

#[derive(Debug)]
struct RType {
	opcode: u8,
	rd: Register,
	rs1: Register,
	rs2: Register,
	funct3: u8,
	funct7: u8,
}