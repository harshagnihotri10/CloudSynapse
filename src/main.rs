use actix_web::{get, post, web, App, HttpServer, HttpResponse};
use serde_json::json;
mod lib;

#[get("/dependencies")]
async fn get_dependencies() -> HttpResponse {
    let deps = lib::parse_makefile("Makefile").unwrap();
    HttpResponse::Ok().json(deps)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_dependencies)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
