use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, TodolistEntries};
use super::models::{CreateEntryData, UpdateEntryData};
use sqlx::Row;
use uuid::Uuid;

#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    let url = "postgres://postgres:password@localhost:5432/todolist";
    let pool = sqlx::postgres::PgPool::connect(url).await;

    let query = "INSERT INTO tbl_todolist (id, title) VALUES ($1, $2)";

    let _id = Uuid::new_v4();
    sqlx::query(query)
        .bind(&_id)
        .bind(&todolist.title)
        .execute(&pool)
        .await;

    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}

// #[post("/todolist/entries")]
// async fn create_entry(data: web::Data<AppState>, param_obj: web::Json<CreateEntryData>) -> impl Responder {
//     let mut todolist_entries = data.todolist_entries.lock().unwrap();
//     let mut max_id: i32 = 0;
//     for i in 0..todolist_entries.len() {
//         if todolist_entries[i].id > max_id {
//             max_id = todolist_entries[i].id
//         }
//     }

//     todolist_entries.push(TodolistEntries {
//         id: max_id + 1,
//         title: param_obj.title.clone(),
//         create_at: param_obj.date,
//     });

//     HttpResponse::Ok().json(todolist_entries.to_vec())
// }

// #[put("/todolist/entries/{id}")]
// async fn update_entry(data: web::Data<AppState>, path: web::Path<i32>, param_obj: web::Json<UpdateEntryData>) -> impl Responder {
//     let id = path.into_inner();
//     let mut todolist_entries = data.todolist_entries.lock().unwrap();
//     for i in 0..todolist_entries.len() {
//         if todolist_entries[i].id == id {
//             todolist_entries[i].title = param_obj.title.clone();
//             break;
//         }
//     }

//     HttpResponse::Ok().json(todolist_entries.to_vec())
// }

// #[delete("/todolist/entries/{id}")]
// async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
//     let mut todolist_entries = data.todolist_entries.lock().unwrap();

//     let id = path.into_inner();
//     *todolist_entries = todolist_entries.to_vec().into_iter().filter(|x| x.id != id).collect();

//     HttpResponse::Ok().json(todolist_entries.to_vec())
// }

// look like export the module in node
pub fn config (cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries);
    //    .service(create_entry)
    //    .service(update_entry)
    //    .service(delete_entry);
}