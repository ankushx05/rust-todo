use std::env;
use std::path::Path;
use std::sync::Arc;

use axum::routing::get;
use axum::{Extension, Router};

mod db;
mod handlers;
mod model;
mod repository;
mod service;

use dotenv::dotenv;
use handlers::{create_todo, delete_todo, get_todo, list_todos, update_todo};
use repository::TodoRepo;
use service::TodoService;
use sqlx::migrate::Migrator;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use model::{NewTodo, Todo, UpdateTodo};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::list_todos,
        handlers::create_todo,
        handlers::get_todo,
        handlers::update_todo,
        handlers::delete_todo
    ),
    components(schemas(Todo, NewTodo, UpdateTodo)),
    info(title = "Todo API", version = "0.3.0")
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // connect to database
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = db::create_pool(&database_url)
        .await
        .expect("🚫 Failed to connect to database");
    println!("{}", "✅ Database Connected");

    // run migrations
    let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
    migrator.run(&pool).await.unwrap();
    println!("Migrations applied");

    // create repo and service
    let repo = TodoRepo::new(pool.clone());
    let svc = Arc::new(TodoService::new(repo));

    // build router and routes here
    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .layer(Extension(svc.clone()));

    // run server
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8080").await {
        Ok(listener) => {
            println!("🚀 Server listening on http://0.0.0.0:8080");
            listener
        }
        Err(e) => {
            eprintln!("🚫 Failed to bind address: {}", e);
            std::process::exit(1);
        }
    };

    axum::serve(listener, app).await.unwrap()
}
