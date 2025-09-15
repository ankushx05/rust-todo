use crate::{
    model::{NewTodo, Todo, UpdateTodo},
    repository::TodoRepository,
};
use anyhow::Result;
use uuid::Uuid;

pub struct TodoService<R: TodoRepository> {
    pub repo: R,
}

impl<R: TodoRepository> TodoService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> Result<Vec<Todo>> {
        self.repo.list().await
    }

    pub async fn create(&self, new: NewTodo) -> Result<Todo> {
        self.repo.create(new).await
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Todo>> {
        self.repo.get(id).await
    }

    pub async fn update(&self, id: Uuid, upd: UpdateTodo) -> Result<Option<Todo>> {
        self.repo.update(id, upd).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        self.repo.delete(id).await
    }
}
