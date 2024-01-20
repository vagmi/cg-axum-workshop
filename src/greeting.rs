use axum::extract::Path;

pub async fn greet(Path(name): Path<String>) -> String { 
    format!("hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Path;

    #[tokio::test]
    async fn test_greet() {
        assert_eq!(
            greet(Path("chennai geeks".to_string())).await, 
            "hello chennai geeks");
    }
}

