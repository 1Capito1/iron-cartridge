/// Isolates a specific bit from a byte, useful for bitflag checks
/// # Examples
///
/// ```rust
///        use iron_cartridge::isolate_bit_u8;
///        let value: u8 = 0b10101101;
///        let expected = [1, 0, 1, 1, 0, 1, 0, 1];
///        let mut actual = [0; 8];
///
///        for offset in 0..8u8 {
///            actual[offset as usize] = isolate_bit_u8(value, offset);
///        }
///        assert_eq!(actual, expected);
/// ```
pub fn isolate_bit_u8(val: u8, offset: u8) -> u8 {
    (val >> offset) & 0x1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn isolate_bit() {
        let value: u8 = 0b10101101;
        let expected = [1, 0, 1, 1, 0, 1, 0, 1];
        let mut actual = [0; 8];

        for offset in 0..8u8 {
            actual[offset as usize] = isolate_bit_u8(value, offset);
        }

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn rotate_left() {
        let value: u8 = 0b10000000_u8.rotate_left(1);
        let expected: u8 = 0b00000001;
        assert_eq!(value, expected);
    }
}
