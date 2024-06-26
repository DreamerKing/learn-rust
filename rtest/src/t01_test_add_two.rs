pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn it_add_two() {
    assert_eq!(4, add_two(2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add_two2() {
        assert_ne!(4, add_two(3))
    }
}

