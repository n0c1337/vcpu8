pub mod math;
pub mod alu;
pub mod cpu;

#[cfg(test)]
mod tests {
    use crate::math::math::Math;

    #[test]
    fn half_adder_test() {
        let result1 = Math::half_adder(0, 0);
        let result2 = Math::half_adder(0, 1);
        let result3 = Math::half_adder(1, 0);
        let result4 = Math::half_adder(1, 1);
        assert_eq!(result1, (0, 0));
        assert_eq!(result2, (1, 0));
        assert_eq!(result3, (1, 0));
        assert_eq!(result4, (0, 1));
    }

    #[test]
    fn full_adder_test() {
        let result1 = Math::full_adder(0, 0, 0);
        let result2 = Math::full_adder(0, 0, 1);
        let result3 = Math::full_adder(0, 1, 0);
        let result4 = Math::full_adder(0, 1, 1);
        println!("{:?}", result4);

        let result5 = Math::full_adder(1, 0, 0);
        let result6 = Math::full_adder(1, 0, 1);
        let result7 = Math::full_adder(1, 1, 0);
        let result8 = Math::full_adder(1, 1, 1);

        assert_eq!(result1, (0, 0));
        assert_eq!(result2, (1, 0));
        assert_eq!(result3, (1, 0));
        assert_eq!(result4, (0, 1));

        assert_eq!(result5, (1, 0));
        assert_eq!(result6, (0, 1));
        assert_eq!(result7, (0, 1));
        assert_eq!(result8, (1, 1));
    }

    #[test]
    fn full_adder_test_8bit() {
        let result = Math::full_adder_8bit(0b00000010, 0b00000100);
        let result2 = Math::full_adder_8bit(0b10101010, 0b00110011);
        assert_eq!(result, 0b00000110);
        assert_eq!(result2, 0b11011101)
    }
}