use actix_web::{get, App, HttpServer, Responder};
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    let podname = match env::var("POD_NAME") {
        Ok(val) => val,
        Err(err) => err.to_string(),
    };
    format!("{}", podname)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 80))?
        .run()
        .await
}

#[test]
fn test() {
    println!("hoge")
}