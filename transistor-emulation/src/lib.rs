pub struct LogicGate;

impl LogicGate {
    #[allow(non_snake_case)]
    pub fn AND(input1: u8, input2: u8) -> u8 {
        input1 & input2
    }

    #[allow(non_snake_case)]
    pub fn OR(input1: u8, input2: u8) -> u8 {
        input1 | input2
    }

    #[allow(non_snake_case)]
    pub fn NOT(input: u8) -> u8 {
        match input {
            1 => 0,
            0 => 1,
            _ => 0
        }
    }

    #[allow(non_snake_case)]
    pub fn NAND(input1: u8, input2: u8) -> u8 {
        let and_output = LogicGate::AND(input1, input2);
        LogicGate::NOT(and_output)
    }

    #[allow(non_snake_case)]
    pub fn NOR(input1: u8, input2: u8) -> u8 {
        let or_output = LogicGate::OR(input1, input2);
        LogicGate::NOT(or_output)
    }

    #[allow(non_snake_case)]
    pub fn XOR(input1: u8, input2: u8) -> u8 {
        let not_output1 = LogicGate::NOT(input1);
        let and_output1 = LogicGate::AND(not_output1, input2);

        let not_output2 = LogicGate::NOT(input2);
        let and_output2 = LogicGate::AND(input1, not_output2);

        LogicGate::OR(and_output1, and_output2)
    }

    #[allow(non_snake_case)]
    pub fn XNOR(inpu1: u8, input2: u8) -> u8 {
        let xor_output = LogicGate::XOR(inpu1, input2);
        LogicGate::NOT(xor_output)
    }

    #[allow(non_snake_case)]
    pub fn HIGH_XOR(input1: u8, input2: u8) -> u8 {
        input1 ^ input2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and_test() {
        let result1 = LogicGate::AND(0, 0);
        let result2 = LogicGate::AND(0, 1);
        let result3 = LogicGate::AND(1, 0);
        let result4 = LogicGate::AND(1, 1);
        assert_eq!(result1, 0);
        assert_eq!(result2, 0);
        assert_eq!(result3, 0);
        assert_eq!(result4, 1);
    }

    #[test]
    fn or_test() {
        let result1 = LogicGate::OR(0, 0);
        let result2 = LogicGate::OR(0, 1);
        let result3 = LogicGate::OR(1, 0);
        let result4 = LogicGate::OR(1, 1);
        assert_eq!(result1, 0);
        assert_eq!(result2, 1);
        assert_eq!(result3, 1);
        assert_eq!(result4, 1);
    }

    #[test]
    fn not_test() {
        let result1 = LogicGate::NOT(0);
        let result2 = LogicGate::NOT(1);
        assert_eq!(result1, 1);
        assert_eq!(result2, 0);
    }

    #[test]
    fn nand_test() {
        let result1 = LogicGate::NAND(0, 0);
        let result2 = LogicGate::NAND(0, 1);
        let result3 = LogicGate::NAND(1, 0);
        let result4 = LogicGate::NAND(1, 1);
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
        assert_eq!(result3, 1);
        assert_eq!(result4, 0);
    }

    #[test]
    fn nor_test() {
        let result1 = LogicGate::NOR(0, 0);
        let result2 = LogicGate::NOR(0, 1);
        let result3 = LogicGate::NOR(1, 0);
        let result4 = LogicGate::NOR(1, 1);
        assert_eq!(result1, 1);
        assert_eq!(result2, 0);
        assert_eq!(result3, 0);
        assert_eq!(result4, 0);
    }

    #[test]
    fn xor_test() {
        let result1 = LogicGate::XOR(0, 0);
        let result2 = LogicGate::XOR(0, 1);
        let result3 = LogicGate::XOR(1, 0);
        let result4 = LogicGate::XOR(1, 1);
        assert_eq!(result1, 0);
        assert_eq!(result2, 1);
        assert_eq!(result3, 1);
        assert_eq!(result4, 0);
    }

    #[test]
    fn xnor_test() {
        let result1 = LogicGate::XNOR(0, 0);
        let result2 = LogicGate::XNOR(0, 1);
        let result3 = LogicGate::XNOR(1, 0);
        let result4 = LogicGate::XNOR(1, 1);
        assert_eq!(result1, 1);
        assert_eq!(result2, 0);
        assert_eq!(result3, 0);
        assert_eq!(result4, 1);
    }

    #[test]
    fn higher_level_xor_test() {
        let result1 = LogicGate::HIGH_XOR(42, 42);
        assert_eq!(result1, 0);
    }
}
