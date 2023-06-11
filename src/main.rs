use actix_web::{web, App, HttpRequest, HttpResponse, Responder, HttpServer};

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