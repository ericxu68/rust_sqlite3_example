pub fn build_message() -> String {
    "Hello, Rust world!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_message() {
        assert_eq!("Hello, Rust world!".to_string(), build_message());
    }
}
