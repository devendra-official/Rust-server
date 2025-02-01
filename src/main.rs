mod db;
mod middleware;
mod models;
mod services;
mod utils;

use std::env;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use db::connection::connect_db;
use dotenv::dotenv;
use middleware::jwt_middleware::JwtHandle;
use models::error::AppRes;
use services::{
    blog_service::{blog_upload, get_blog},
    user_service::{login, signup},
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();

    let pool = connect_db().await.unwrap();

    let upload = env::var("UPLOAD_DIR").unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .service(Files::new("/files", &upload).show_files_listing())
            .wrap(Cors::default())
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                let message = match err {
                    actix_web::error::JsonPayloadError::ContentType => {
                        String::from("Invalid content type, expected application/json")
                    }
                    actix_web::error::JsonPayloadError::Deserialize(ref err) => {
                        format!("Invalid JSON format {}", err)
                    }
                    _ => String::from("Invalid JSON payload"),
                };
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(AppRes::new(&message)),
                )
                .into()
            }))
            .service(
                web::scope("/blog")
                    .wrap(JwtHandle)
                    .route("/upload", web::post().to(blog_upload))
                    .route("/get-blogs", web::get().to(get_blog))
            )
            .route("/users/login", web::post().to(login))
            .route("/users/signup", web::post().to(signup))
            .default_service(
                web::route()
                    .to(|| async { HttpResponse::NotFound().json(AppRes::new("Page Not Found")) }),
            )
    })
    .bind((
        host.as_str(),
        port.parse::<u16>().expect("Invalid PORT number"),
    ))
    .expect("server failed to run");
    server.run().await
}
