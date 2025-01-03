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