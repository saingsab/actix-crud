use crate::{
    model::TodoModel,
    schema::{CreateTodoSchema, FilterOptions},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/todos")]
pub async fn todo_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        TodoModel,
        "SELECT * FROM tbl_todolist ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all note items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let notes = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });
    HttpResponse::Ok().json(json_response)
}

// Merge the Route Functions
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(todo_list_handler);

    conf.service(scope);
}