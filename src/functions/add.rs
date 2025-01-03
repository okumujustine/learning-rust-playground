pub fn add_five(num: u32) -> u32 {
    num + 5
}

pub fn add_10(num: u8) -> u8 {
    num + 10
}

#[cfg(test)]
mod test {
    use crate::functions::add::{add_10, add_five};

    #[test]
    fn test_add_five() {
        assert_eq!(add_five(50), 55);
    }

    #[test]
    fn test_add_10 () {
        assert_eq!(add_10(11), 21);
    }
}