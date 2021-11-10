use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("<p style='background: #565656'><img src='https://actix.rs/img/logo-large.png' width='250'></p>")
}

#[post("/")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// fn get_server_port() -> u16 {
//     env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("HOST").expect("Host not set");
    let host = env::var("PORT").expect("Port not set");
    HttpServer::new(|| {
        
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}