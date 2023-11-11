use crate::math::math::Math;

pub type INSTRUCTION = u8;
pub const MOV: INSTRUCTION = 0x01;
pub const ANDB: INSTRUCTION = 0x02;
pub const ORB: INSTRUCTION = 0x03;
pub const XORB: INSTRUCTION = 0x04;
pub const NOT: INSTRUCTION = 0x05;
pub const SHL: INSTRUCTION = 0x06;
pub const SHR: INSTRUCTION = 0x07;
pub const ADD: INSTRUCTION = 0x08;
pub const INC: INSTRUCTION = 0x09;
pub const SUB: INSTRUCTION = 0x0A;
pub const DEC: INSTRUCTION = 0x0B;
pub const MUL: INSTRUCTION = 0x0C;
pub const CMP: INSTRUCTION = 0x0D;
pub const HALT: INSTRUCTION = 0x15;


pub trait ALU {
    fn execute(opcode: u8, output_register: u8, operand_register_1: u8, operand_register_2: u8, registers: &mut [u8; 8]);
}

pub struct EumulatedALU;

impl ALU for EumulatedALU {
    fn execute(opcode: u8, output_register: u8, operand_register_1: u8, operand_register_2: u8, registers: &mut [u8; 8]) {    
        match opcode {
            MOV => {
                registers[output_register as usize] = operand_register_1
            }
            ANDB => {
                registers[output_register as usize] = registers[operand_register_1 as usize] & registers[operand_register_2 as usize]
            }
            ORB => {
                registers[output_register as usize] = registers[operand_register_1 as usize] | registers[operand_register_2 as usize]
            }
            XORB => {
                registers[output_register as usize] = registers[output_register as usize] ^ registers[operand_register_2 as usize]
            }
            NOT => {
                registers[output_register as usize] = !registers[operand_register_1 as usize]
            }
            SHL => {
                registers[output_register as usize] = registers[operand_register_1 as usize] << registers[operand_register_2 as usize]
            }
            SHR => {
                registers[output_register as usize] = registers[operand_register_1 as usize] >> registers[operand_register_2 as usize]
            }
            ADD => {
                registers[output_register as usize] = Math::full_adder_8bit(registers[operand_register_1 as usize], registers[operand_register_2 as usize])
            }
            INC => {
                registers[output_register as usize] = Math::full_adder_8bit(registers[operand_register_1 as usize], 0b00000001)
            }
            SUB => {
                registers[output_register as usize] = Math::full_subtractor_8bit(registers[operand_register_1 as usize], registers[operand_register_2 as usize])
            }
            DEC => {
                registers[output_register as usize] = Math::full_subtractor_8bit(registers[operand_register_1 as usize], 0b00000001)
            }
            MUL => {
                registers[output_register as usize] = Math::multiply(registers[operand_register_1 as usize], registers[operand_register_2 as usize])
            }
            CMP => {
                if registers[operand_register_1 as usize] == registers[operand_register_2 as usize] {
                    registers[output_register as usize] = 1;
                } else {
                    registers[output_register as usize] = 0;
                } 
            }
            0x0E => unimplemented!(),
            0x0F => unimplemented!(),
            0x10 => unimplemented!(),
            0x11 => unimplemented!(),
            0x12 => unimplemented!(),
            0x13 => unimplemented!(),
            0x14 => unimplemented!(),
            _ => ()
        }
    }
}