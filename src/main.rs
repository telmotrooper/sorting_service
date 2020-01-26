use actix_web::{web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use actix_web::get;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
//            .route("/{name}", web::get().to(greet))
            .service(hello)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}
