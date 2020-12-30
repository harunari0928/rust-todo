use actix_web::ResponseError;
use actix_web::{get, App, HttpResponse, HttpServer};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {}

impl ResponseError for MyError {}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello world!";

    Ok(HttpResponse::Ok().body(response_body))
}
