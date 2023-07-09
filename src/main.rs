use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("App is running!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(healthz)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
