use transistor_emulation::LogicGate;

use crate::math::math::{NativeMath, Math};

pub trait ALU {
    fn execute(opcode: u8, output_register: u8, operand_register: u8, registers: &mut [u8; 8]);
}

pub enum Mode {
    Native = 0,
    Emulated
}

pub struct EumulatedALU;

impl ALU for EumulatedALU {
    fn execute(opcode: u8, output_register: u8, operand_register: u8, registers: &mut [u8; 8]) {
        match opcode {
            0b000000000 => {
                registers[output_register as usize] = operand_register
            },
            0b00000001 => {
                registers[output_register as usize] = LogicGate::AND(registers[output_register as usize], registers[operand_register as usize])
            },
            0b00000010 => {
                registers[output_register as usize] = LogicGate::OR(registers[output_register as usize], registers[operand_register as usize])
            }
            0b00000011 => {
                registers[output_register as usize] = Math::full_adder_8bit(registers[output_register as usize], registers[operand_register as usize])
            },
            0b00000101 => {
                registers[output_register as usize] = Math::full_adder_8bit(registers[output_register as usize], 0b00000001)
            },
            0b00000100 => {
                registers[output_register as usize] = Math::full_subtractor_8bit(registers[output_register as usize], registers[operand_register as usize])
            },
            0b00000110 => {
                registers[output_register as usize] = Math::full_subtractor_8bit(registers[output_register as usize], 0b00000001)
            },
            0b00000111 => {
                registers[output_register as usize] = Math::multiply(registers[output_register as usize], registers[operand_register as usize])
            }
            _ => ()
        }
    }
}

pub struct NativeALU;

impl ALU for NativeALU {
    fn execute(opcode: u8, output_register: u8, operand_register: u8, registers: &mut [u8; 8]) {
        match opcode {
            0b000000000 => {
                registers[output_register as usize] = operand_register
            },
            0b00000001 => {
                registers[output_register as usize] = LogicGate::AND(registers[output_register as usize], registers[operand_register as usize])
            },
            0b00000010 => {
                registers[output_register as usize] = LogicGate::OR(registers[output_register as usize], registers[operand_register as usize])
            }
            0b00000011 => {
                registers[output_register as usize] = NativeMath::add(registers[output_register as usize], registers[operand_register as usize])
            },
            0b00000100 => {
                registers[output_register as usize] = NativeMath::subtract(registers[output_register as usize], registers[operand_register as usize])
            },
            0b00000101 => {
                registers[output_register as usize] = NativeMath::increment(registers[output_register as usize])
            },
            0b00000110 => {
                registers[output_register as usize] = NativeMath::decrement(registers[output_register as usize])
            },
            0b00000111 => {
                registers[output_register as usize] = NativeMath::multiply(registers[output_register as usize], registers[operand_register as usize])
            },
            0b00001000 => {
                registers[output_register as usize] = NativeMath::divide(registers[output_register as usize], registers[operand_register as usize])
            },
            _ => ()
        }
    }
}