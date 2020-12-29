use actix_web::{
    web, 
    App, 
    HttpRequest, 
    HttpServer, 
    Responder
};

const PORT: &'static str = "127.0.0.1:8000";

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info()
        .get("name")
        .unwrap_or("World");

    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    }) 
    .bind(PORT)
    .run()
    .await
}
