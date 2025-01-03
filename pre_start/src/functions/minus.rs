/**
 * Adds parameter to 10
 * # Parameters:
 * num : u8
 * # Example:
 * ```
 * let x = 90;
 * let result = subtract_10(100);
 * ```
 * 
 */
pub fn subtract_10(num: u8) -> u8 {
    num - 10
}

#[cfg(test)]
mod test {
    use crate::functions::minus::subtract_10;

    #[test]
    fn test_subtract_10() {
        assert_eq!(subtract_10(10), 0);
    }
}
