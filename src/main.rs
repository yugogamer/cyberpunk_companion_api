use actix_web::web::Data;
use actix_web::{get, middleware, App, HttpResponse, HttpServer};

mod utils;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = utils::config::Config::new();
    let pool = utils::database::connect_to_database(&config).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Compress::default())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
