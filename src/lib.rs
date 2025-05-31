use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use std::net::TcpListener;

#[get("/{greeting}/{name}")]
async fn greet_with_name(path: web::Path<(String, String)>) -> impl Responder {
    let (greeting, name) = path.into_inner();
    format!("{} {}!", greeting, name)
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(tcp_listener: Option<TcpListener>) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(greet_with_name))
        .listen(tcp_listener.unwrap_or(TcpListener::bind("127.0.0.1:8000")?))?
        .run();

    Ok(server)
}
