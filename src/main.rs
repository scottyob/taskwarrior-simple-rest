extern crate actix_web;

use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use std::{env, io};

mod task;

use task::{Task, Tasks};

#[get("/")]
async fn list() -> Result<impl Responder> {
    let tasks = Tasks::get().await?;
    Ok(web::Json(tasks))
}

#[post("/new")]
async fn new(web::Json(task): web::Json<Task>) -> Result<impl Responder> {
    let task = Tasks::add(task).await.expect("Unable to add Task");
    Ok(task)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| App::new().service(list).service(new))
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
