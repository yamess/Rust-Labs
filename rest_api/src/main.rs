use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use log::info;

use rest_api::setup_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger();
    info!("Starting server");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::MergeOnly,
            ))
            .service(index)
            .service(hello)
            .route("/echo", web::get().to(echo))
    })
    .bind(("127.0.0.1", 8000))?
    .workers(4)
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Echo page")
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Index page")
}
