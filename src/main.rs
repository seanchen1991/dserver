use actix_web::{
    web, 
    App, 
    HttpServer, 
    HttpResponse,
    Responder
};

const PORT: &'static str = "127.0.0.1:8000";

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    }) 
    .bind(PORT)?
    .run()
    .await
}
