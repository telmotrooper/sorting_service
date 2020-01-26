use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}
