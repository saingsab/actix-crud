use secrecy::ExposeSecret;
use actix_web::{get, web::{self, to}, App, HttpServer};
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use sqlx::Row;
use uuid::Uuid;
use chrono::Utc;

mod todolist;
use todolist::services;

struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntries>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TodolistEntries {
    title: String,
}

#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

async fn createTask(todolist: &TodolistEntries, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO tbl_todolist (id, title) VALUES ($1, $2)";

    let _id = Uuid::new_v4();
    sqlx::query(query)
    .bind(&_id)
    .bind(&todolist.title)
    .execute(pool)
    .await?;
Ok(())
}

#[tokio::main]
// async fn main() -> std::io::Result<()> {
async fn main() -> Result<(), Box<dyn Error>> {

    let url = "postgres://postgres:password@localhost:5432/todolist";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    
    let dt = Utc::now();
    let todo = TodolistEntries {
        title: "chean pong tea".to_string(),
    };

    createTask(&todo, &pool).await?;

    Ok(())

    // let app_data = web::Data::new(AppState {
    //     todolist_entries: Mutex::new(vec![])
    // });

    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(app_data.clone())
    //         .service(index)
    //         .configure(services::config)
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await
}