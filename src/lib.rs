use actix_web::{
    dev::Server,
    web, 
    App, 
    HttpServer, 
    HttpResponse,
};

const PORT: &'static str = "127.0.0.1:8000";

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    }) 
    .bind(PORT)?
    .run();

    Ok(server)
}
