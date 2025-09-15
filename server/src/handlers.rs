use std::sync::Arc;

use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

use crate::{
    model::{NewTodo, Todo, UpdateTodo},
    repository::TodoRepo,
    service::TodoService,
};

pub type SharedTodoService = Arc<TodoService<TodoRepo>>;

#[utoipa::path(
    get,
    path = "/todos",
    responses((status = 200, description = "List todos", body = [Todo]))
)]
pub async fn list_todos(Extension(svc): Extension<SharedTodoService>) -> impl IntoResponse {
    match svc.list().await {
        Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/todos",
    request_body = NewTodo,
    responses((status = 201,description ="Created",body = Todo))
)]
pub async fn create_todo(
    Extension(svc): Extension<SharedTodoService>,
    Json(payload): Json<NewTodo>,
) -> impl IntoResponse {
    match svc.create(payload).await {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    params(("id" = Uuid, Path, description = "Todo id", example = "d290f1ee-6c54-4b01-90e6-d701748f0851")),
    responses(
        (status = 200, description = "Get todo", body = Todo), 
        (status = 404, description = "Not found")
    )
)]
pub async fn get_todo(
    Extension(svc): Extension<SharedTodoService>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match svc.get(id).await {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    request_body = UpdateTodo,
    params(("id" = Uuid, Path, description = "Todo id")),
    responses(
        (status = 200, description = "Updated", body = Todo),
        (status = 404, description = "Not found")
    )
)]
pub async fn update_todo(
    Extension(svc): Extension<SharedTodoService>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> impl IntoResponse {
    match svc.update(id, payload).await {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not Found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    params(("id"= Uuid ,Path, description = "Todo Id")),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not found")
    )
)]
pub async fn delete_todo(
    Extension(svc): Extension<SharedTodoService>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match svc.delete(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, "Not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)).into_response(),
    }
}
