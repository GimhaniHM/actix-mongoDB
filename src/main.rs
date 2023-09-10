
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn init() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("🚀 Server started successfully");
    HttpServer::new(move|| App::new().service(init))
        .bind(("localhost", 8080))?
        .run()
        .await

}
