use actix_web::{App, HttpServer, Responder, HttpResponse, web::get};


async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    HttpServer::new(|| {
        App::new().route("/", get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
