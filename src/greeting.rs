pub async fn greet() -> String { 
    "hello world".to_string() 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_greet() {
        assert_eq!(greet().await, "hello world");
    }
}

