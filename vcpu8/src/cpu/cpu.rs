use crate::alu::alu::{ALU, EumulatedALU};

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
    registers: [u8; 8]
}

impl CPU {
    pub fn new() -> Self {
        CPU { registers: [0; 8] }
    }

    pub fn execute(&mut self, program: Vec<(u8, u8, u8)>) -> u8 {
        println!("Before Execution: {:?}", self.registers);
        for i in program {
            let (opcode, output_register, operand_register) = i;

            // HALT
            if opcode == 0x15 {
                break;
            }
            
           EumulatedALU::execute(opcode, output_register, operand_register, &mut self.registers)
        }
        println!("After Execution: {:?}", self.registers);

        self.registers[Registers::R0 as usize]
    }
}