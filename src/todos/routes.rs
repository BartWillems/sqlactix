use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use actix_web::{get, post};
use sqlx::{Pool, Postgres};

use crate::error::Error;
use crate::todos::Todo;

#[get("/todos")]
async fn load(pool: Data<Pool<Postgres>>) -> Result<HttpResponse, Error> {
    let todos = Todo::load(&pool).await?;
    Ok(HttpResponse::Ok().json(todos))
}

#[get("/todos/{id}")]
async fn find(id: Path<i64>, pool: Data<Pool<Postgres>>) -> Result<HttpResponse, Error> {
    let todos = Todo::find(id.into_inner(), &pool).await?;
    Ok(HttpResponse::Ok().json(todos))
}

#[post("/todos")]
async fn create(todo: Json<String>, pool: Data<Pool<Postgres>>) -> Result<HttpResponse, Error> {
    let todo = Todo::create(&todo, &pool).await?;

    Ok(HttpResponse::Created().json(todo))
}

pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(load);
    cfg.service(find);
    cfg.service(create);
}
