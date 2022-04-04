use std::net::TcpListener;

use news_letter::run;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("unable to bind to port 8000");

    run(listener)?.await
}