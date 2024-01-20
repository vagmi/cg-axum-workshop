pub fn greet(name: &str) -> String { 
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("chennai geeks"), "Hello, chennai geeks!");
    }

    #[test]
    fn test_greet2() {
        assert_eq!(greet("world"), "Hello, world!");
    }
}
