mod aaa;

#[cfg(test)]
mod tests {
    use super::aaa;

    #[test]
    fn mytest1() {
        assert_eq!(2 + 2, 4);
        aaa::say_hello();
    }

    #[test]
    #[should_panic(expected="test panic")]
    fn test2() {
        panic!("test panic");
    }
}