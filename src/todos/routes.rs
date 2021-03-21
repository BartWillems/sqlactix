use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use actix_web::{get, post};

use crate::error::Error;
use crate::todos::Todo;
use crate::State;

#[get("/todos")]
async fn load(state: Data<State>) -> Result<HttpResponse, Error> {
    let todos = Todo::load(&state.db).await?;
    Ok(HttpResponse::Ok().json(todos))
}

#[get("/todos/{id}")]
async fn find(id: Path<i64>, state: Data<State>) -> Result<HttpResponse, Error> {
    let todos = Todo::find(id.into_inner(), &state.db).await?;
    Ok(HttpResponse::Ok().json(todos))
}

#[post("/todos")]
async fn create(todo: Json<String>, state: Data<State>) -> Result<HttpResponse, Error> {
    let todo = Todo::create(&todo, &state.db).await?;

    Ok(HttpResponse::Created().json(todo))
}

pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(load);
    cfg.service(find);
    cfg.service(create);
}
