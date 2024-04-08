use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use metacall::{loaders, metacall, switch};
use tokio::runtime::Runtime;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(metacall::<String>("Hello", [String::from("Metacall!")]).unwrap())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let _metacall = switch::initialize().unwrap();
    loaders::from_single_file("ts", "App.tsx").unwrap();

    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
