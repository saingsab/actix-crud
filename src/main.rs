use secrecy::ExposeSecret;
use actix_web::{get, web, App, HttpServer};
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use sqlx::Connection;
use sqlx::Row;

mod todolist;
use todolist::services;

struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntries>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TodolistEntries {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

#[tokio::main]
// async fn main() -> std::io::Result<()> {
async fn main() -> Result<(), Box<dyn Error>> {

    let url = "postgres://postgres:password@localhost:5432/todolist";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum")
                .fetch_one(&pool)
                .await?;

    let sum: i32 = res.get("sum");
    println!("1 + 1 = {}", sum);

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