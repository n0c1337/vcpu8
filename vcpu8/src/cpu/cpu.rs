use crate::{alu::alu::{ALU, EumulatedALU}, HALT};

pub type Register = u8;
pub const RD: Register = 0; // integer register destination
pub const R1: Register = 1;
pub const R2: Register = 2;
pub const R3: Register = 3;
pub const R4: Register = 4;
pub const R5: Register = 5;
pub const R6: Register = 6;
pub const R7: Register = 7;

pub struct CPU {
    registers: [u8; 8],
    floating_point_register: [f32; 4]
}

impl CPU {
    pub fn new() -> Self {
        CPU { registers: [0; 8], floating_point_register: [0.0; 4] }
    }

    pub fn execute(&mut self, program: Vec<(u8, u8, u8, u8)>) -> u8 {
        println!("Before Execution: {:?}", self.registers);
        for i in program {
            let (opcode, output_register, operand_register_1, operand_register_2) = i;

            if opcode == HALT { break; }
            
           EumulatedALU::execute(opcode, output_register, operand_register_1, operand_register_2, &mut self.registers)
        }
        println!("After Execution: {:?}", self.registers);

        self.registers[RD as usize]
    }
}