use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    match metacall::metacall("Hello", &[metacall::Any::Str("Metacall!".to_string())]) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(ret) => match ret {
            metacall::Any::Str(message) => HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(message),
            _ => HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body("<h1>Not a Valid HTML</h1>"),
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match metacall::initialize() {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        _ => println!("MetaCall initialized"),
    }

    let scripts = ["App.tsx".to_string()];

    if let Err(e) = metacall::load_from_file("ts", &scripts) {
        println!("{}", e);
        panic!();
    }
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
