use axum::{response::Html, routing::get, Router};
use metacall;
use tokio::runtime::Runtime;

fn main() {
    match metacall::initialize() {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        _ => println!("MetaCall initialized"),
    }

    let scripts = ["App.tsx".to_string()];

    if let Err(e) = metacall::load_from_file("ts", &scripts) {
        panic!("{}", e);
    } else {
        println!("scripts loaded");
    }

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let app = Router::new().route("/", get(root));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await.unwrap();
        println!(
            "listening on {}",
            listener.local_addr().unwrap().to_string()
        );
        axum::serve(listener, app).await.unwrap();
    });
}

async fn root() -> Html<String> {
    match metacall::metacall("Hello", &[metacall::Any::Str("Metacall!".to_string())]) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(ret) => match ret {
            metacall::Any::Str(message) => Html(message),
            _ => Html("<h1>Not a Valid HTML</h1>".to_string()),
        },
    }
}
