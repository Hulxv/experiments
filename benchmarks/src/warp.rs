use metacall;
use warp::{reply::html, Filter};

#[tokio::main]
async fn main() {
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
    let routes = warp::any().map(|| {
        match metacall::metacall("Hello", &[metacall::Any::Str("emslgjnse!".to_string())]) {
            Err(e) => {
                println!("{}", e);
                panic!();
            }
            Ok(ret) => html(match ret {
                metacall::Any::Str(message) => message,
                _ => "<h1>Not a Valid HTML</h1>".to_string(),
            }),
        }
    });

    warp::serve(routes).run(([127, 0, 0, 1], 8083)).await;
}
