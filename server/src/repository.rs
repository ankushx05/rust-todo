use crate::model::{NewTodo, Todo, UpdateTodo};
use anyhow::{Ok, Result};
use sqlx::PgPool;
use uuid::Uuid;

pub trait TodoRepository {
    async fn list(&self) -> Result<Vec<Todo>>;
    async fn create(&self, new: NewTodo) -> Result<Todo>;
    async fn get(&self, id: Uuid) -> Result<Option<Todo>>;
    async fn update(&self, id: Uuid, upd: UpdateTodo) -> Result<Option<Todo>>;
    async fn delete(&self, id: Uuid) -> Result<bool>;
}

#[derive(Debug)]
pub struct TodoRepo {
    pub pool: PgPool,
}

impl TodoRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TodoRepository for TodoRepo {
    async fn create(&self, new: NewTodo) -> Result<Todo> {
        let id = Uuid::new_v4();
        let rec = sqlx::query_as::<_, Todo>("INSERT INTO todos (id, title, completed) VALUES ($1, $2, $3) RETURNING id, title, completed")
        .bind(id)
        .bind(new.title)
        .bind(false)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn list(&self) -> Result<Vec<Todo>> {
        let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
            .fetch_all(&self.pool)
            .await?;
        Ok(todos)
    }

    async fn get(&self, id: Uuid) -> Result<Option<Todo>> {
        let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(todo)
    }

    async fn update(&self, id: Uuid, upd: UpdateTodo) -> Result<Option<Todo>> {
        let exiting = self.get(id).await?;
        if exiting.is_none() {
            return Ok(None);
        };

        let mut title = exiting.as_ref().unwrap().title.clone();
        let mut completed = exiting.as_ref().unwrap().completed;

        if let Some(t) = upd.title {
            title = t
        };
        if let Some(c) = upd.completed {
            completed = c
        };

        let rec = sqlx::query_as::<_,Todo>(
            "UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING id, title, completed"
        ).bind(title)
        .bind(completed)
        .fetch_one(&self.pool).await?;

        Ok(Some(rec))
    }

    async fn delete(&self, id: Uuid) -> Result<bool> {
        let res = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(res.rows_affected() > 0)
    }
}
