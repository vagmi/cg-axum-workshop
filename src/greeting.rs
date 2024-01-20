use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    title: String,
}

pub async fn greet(Path((greeting, nom)): Path<(String, String)>) -> String { 
    format!("{}, {}", greeting, nom)
}

pub async fn create_todo(Json(create_todo): Json<CreateTodo>) -> Json<Todo> {
    Json(Todo {
        id: 1,
        title: create_todo.title,
        completed: false,
    })
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


    #[tokio::test]
    async fn test_create_todo() {
        let todo_req= Json(CreateTodo {
            title: "hello".to_string(),
        });
        let todo = create_todo(todo_req).await;
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "hello");
        assert_eq!(todo.completed, false);
    }
}

