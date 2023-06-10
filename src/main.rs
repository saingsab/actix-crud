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
    /*
    begin commit way
    let mut tx = conn.begin().await?;
    after the statment then 

    tx.rollback().await?;
    */
    let query = "INSERT INTO tbl_todolist (id, title) VALUES ($1, $2)";

    let _id = Uuid::new_v4();
    sqlx::query(query)
    .bind(&_id)
    .bind(&todolist.title)
    .execute(pool)
    .await?;
Ok(())
}

async fn updateTask(todolist: &TodolistEntries, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE tbl_todolist
                SET title = $2 
                WHERE id = ($1::uuid)";

    let _id = "3d516ed8-c656-498c-87b1-46b34e7c7c25";
    sqlx::query(query)
    .bind(&_id)
    .bind(&todolist.title)  
    .execute(pool)
    .await?;
Ok(())
}

async fn deleteTask(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    // DELETE FROM table_name WHERE condition;
    let query = "DELETE FROM tbl_todolist
                 WHERE id = ($1::uuid)";

    let _id = "3d516ed8-c656-498c-87b1-46b34e7c7c25";
    sqlx::query(query)
    .bind(&_id)
    .execute(pool)
    .await?;
Ok(())
}

#[tokio::main]
// async fn main() -> std::io::Result<()> {
async fn main() -> Result<(), Box<dyn Error>> {

    let url = "postgres://postgres:password@localhost:5432/todolist";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    
    // let dt = Utc::now();
    let todo = TodolistEntries {
        title: "Learn rust".to_string(),
    };

    // createTask(&todo, &pool).await?;
    // updateTask(&todo, &pool).await?;
    deleteTask(&pool).await?;

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