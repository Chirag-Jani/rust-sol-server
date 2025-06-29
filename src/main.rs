use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::Query};
use serde::Deserialize;

#[derive(Deserialize)]
struct BalanceQuery {
    address: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/greet")]
async fn greet(name: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

#[get("/balance")]
async fn balance(query: Query<BalanceQuery>) -> impl Responder {
    HttpResponse::Ok().body(format!("Balance of {} is 100", query.address))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(greet).service(balance))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
