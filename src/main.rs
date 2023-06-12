use actix_web::{web, App, HttpRequest, HttpResponse, Responder, HttpServer};
use sqlx::{postgres::PgPool, Pool};

#[derive(Debug)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

async fn get_todos(db: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = db
        .stream("SELECT id, title, completed FROM todos", &[])
        .await?
        .collect::<Vec<_>>()?;

    Ok(todos)
}


async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}