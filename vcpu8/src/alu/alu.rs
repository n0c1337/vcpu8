use crate::math::math::Math;

pub trait ALU {
    fn execute(opcode: u8, output_register: u8, operand_register: u8, registers: &mut [u8; 8]);
}

pub enum Mode {
    Emulated = 0,
    #[deprecated(note="Not implemented anymore, use Mode::Emulated")]
    Native
}

pub struct EumulatedALU;

impl ALU for EumulatedALU {
    fn execute(opcode: u8, output_register: u8, operand_register: u8, registers: &mut [u8; 8]) {
        match opcode {
            0x01 => {
                registers[output_register as usize] = operand_register
            }
            0x02 => {
                registers[output_register as usize] = registers[output_register as usize] & registers[operand_register as usize]
            }
            0x03 => {
                registers[output_register as usize] = registers[output_register as usize] | registers[operand_register as usize]
            }
            0x04 => {
                registers[output_register as usize] = registers[output_register as usize] ^ registers[operand_register as usize]
            }
            0x05 => {
                registers[output_register as usize] = !registers[output_register as usize]
            }
            0x06 => {
                registers[output_register as usize] = registers[output_register as usize] << registers[operand_register as usize]
            }
            0x07 => {
                registers[output_register as usize] = registers[output_register as usize] >> registers[operand_register as usize]
            }
            0x08 => {
                registers[output_register as usize] = Math::full_adder_8bit(registers[output_register as usize], registers[operand_register as usize])
            }
            0x09 => {
                registers[output_register as usize] = Math::full_adder_8bit(registers[output_register as usize], 0b00000001)
            }
            0x0A => {
                registers[output_register as usize] = Math::full_subtractor_8bit(registers[output_register as usize], registers[operand_register as usize])
            }
            0x0B => {
                registers[output_register as usize] = Math::full_subtractor_8bit(registers[output_register as usize], 0b00000001)
            }
            0x0C => {
                registers[output_register as usize] = Math::multiply(registers[output_register as usize], registers[operand_register as usize])
            }
            0x0D => {
                if registers[output_register as usize] == registers[operand_register as usize] {
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