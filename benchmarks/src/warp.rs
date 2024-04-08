use metacall::{metacall, switch, loaders};
use warp::{reply::html, Filter};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _metacall = switch::initialize().unwrap(); 
    loaders::from_single_file("ts", "App.tsx"); 

    let routes = warp::any().map(|| {
        html(
            match metacall::<String>("Hello", ["emslgjnse!".to_string()]) {
            Err(e) => {
                format!("Cannot load component: {e:?}")
            }
            Ok(ret) => match ret {
                message => message,
                _ => "<h1>Not a Valid HTML</h1>".to_string(),
            },
        })
    });
    let addr = ([127, 0, 0, 1], 8083);
    println!("running on {addr:?}");
    warp::serve(routes).run(addr).await;
}
