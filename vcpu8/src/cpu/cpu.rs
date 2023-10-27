use crate::alu::alu::{ALU, Mode, EumulatedALU};

pub enum Registers {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7
}

pub struct CPU {
    mode: Mode,
    registers: [u8; 8]
}

impl CPU {
    pub fn new(mode: Mode) -> Self {
        CPU { mode: mode, registers: [0; 8] }
    }
    pub fn execute(&mut self, program: Vec<(u8, u8, u8)>) -> u8 {
        println!("Before Execution: {:?}", self.registers);
        for i in program {
            let (opcode, output_register, operand_register) = i;

            // HALT
            if opcode == 0x15 {
                break;
            }
            
            match self.mode {
                Mode::Emulated => EumulatedALU::execute(opcode, output_register, operand_register, &mut self.registers),
                Mode::Native => unimplemented!("Use emulated mode.")
            }
        }
        println!("After Execution: {:?}", self.registers);

        self.registers[Registers::R0 as usize]
    }
}