use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("info")
    );

    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(("127.0.0.1", 9000))?
        .workers(2)
        .run();

    server.await
}