use axum::extract::Path;

pub async fn greet(Path((greeting, nom)): Path<(String, String)>) -> String { 
    format!("{}, {}", greeting, nom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Path;

    #[tokio::test]
    async fn test_greet() {
        let greeting = Path(("hello".to_string(), "chennai".to_string()));
        assert_eq!(greet(greeting).await, "hello, chennai");
            
    }
}

