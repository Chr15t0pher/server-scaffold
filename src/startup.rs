use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{health_check, subscriptions};

async fn index(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let web_data = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            .app_data(web_data.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
