use actix_web::(get, post, put, delete, web, Responder, HttpResponse);
use create::{AppState, TodolistEntries};
use supper::models::{CreateEntryData, UpdateEntryData};

#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}

#[post("/todolist/entries")]
async fn create_entry(data: web::Data<AppState>, param_obj: web::Json<CreateEntryData>) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let mut max_id: i32 = 0;
    for i int 0..todolist_entries.len() {
        if todolist_entries[i].id > max_id {
            max_id = todolist_entries[i].id
        }  
    }

    todolist_entries.push(TodolistEntries) {
        id: max_id + 1,
        title: param_obj.title.clone(),
        data: param_obj.date,
    }

    HttpResponse::Ok().json(todolist_entries.to_vec())
}