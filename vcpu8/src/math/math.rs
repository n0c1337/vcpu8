pub struct Math;
pub struct NativeMath;

impl Math {
    // TODO: Rewrite half and full adder to use my own logicgate implementation
    pub fn half_adder(a: u8, b: u8) -> (u8, u8) {
        let sum_out = a ^ b;
        let carry_out = a & b;
        (sum_out, carry_out)
    }
    
    pub fn full_adder(a: u8, b: u8, cin: u8) -> (u8, u8) {
        let (sum_out1, carry_out1) = Math::half_adder(a, b);
        let (sum_out2, carry_out2) = Math::half_adder(sum_out1, cin);
    
        let sum_out = sum_out2;
        let carry_out = carry_out1 | carry_out2;
    
        (sum_out, carry_out)
    }

    // TODO: Implement shift operation in LogicGate module
    pub fn full_adder_8bit(number_1: u8, number_2: u8) -> u8 {
        let mut result = 0;
        let mut carry = 0;

        // Iterate over each bit in the 8 bit number
        for i in 0..8 {
            // Get bit at position i for number_1 and number_2
            let a = (number_1 >> i) & 1;
            let b = (number_2 >> i) & 1;

            // Call full_adder function which can add 2 single bit
            let (sum, c) = Math::full_adder(a, b, carry);
            result |= sum << i;
            // Update carry for next iteration
            carry = c;
        }

        // Add the last carry bit to the result
        result |= (u16::from(carry) << 8) as u8;
        result
    }

    pub fn multiply(number_1: u8, number_2: u8) -> u8 {
        let mut result = 0;
        for _ in 0..number_2 {
            result += Math::full_adder_8bit(number_1, 0b0)
        }
        result
    }

    pub fn half_subtractor(a: u8, b: u8) -> (u8, u8) {
        let difference_out= a ^ b;
        let borrow_out = !a & b;

        (difference_out, borrow_out)
    }

    pub fn full_subtractor(a: u8, b: u8, bin: u8) -> (u8, u8) {
        let (diffrence_out1, borrow_out1) = Math::half_subtractor(a, b);
        let (diffrence_out2, borrow_out2) = Math::half_subtractor(diffrence_out1, bin);

        let difference_out = diffrence_out2;
        let borrow_out = borrow_out1 | borrow_out2;
    
        (difference_out, borrow_out)
    }

    pub fn full_subtractor_8bit(number_1: u8, number_2: u8) -> u8 {
        let mut result = 0;
        let mut borrow = 0;

        // Iterate over each bit in the 8 bit number
        for i in 0..8 {
            // Get bit at position i for number_1 and number_2
            let a = (number_1 >> i) & 1;
            let b = (number_2 >> i) & 1;

             // Call full_subtractor function which can add 2 single bit
            let (difference, c) = Math::full_subtractor(a, b, borrow);
            result |= difference << i;
            // Update borrow for next iteration
            borrow = c;
        }

        result |= (u16::from(borrow) << 8) as u8;
        result
    }
}

// Lazy approach
impl NativeMath {
    pub fn add(number_1: u8, number_2: u8) -> u8 {
        number_1 + number_2
    }

    pub fn subtract(number_1: u8, number_2: u8) -> u8 {
        number_1 - number_2
    }

    pub fn increment(number: u8) -> u8 {
        number + 1
    }

    pub fn decrement(number: u8) -> u8 {
        number - 1
    }

    pub fn multiply(number_1: u8, number_2: u8) -> u8 {
        number_1 * number_2
    }

    pub fn divide(number_1: u8, number_2: u8) -> u8 {
        number_1 / number_2
    }
}