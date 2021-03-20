use actix_web::get;
use actix_web::web::Data;
use actix_web::HttpResponse;
use sqlx::{Pool, Postgres};

use crate::error::Error;

#[derive(Serialize)]
struct Stats {
    idle_db_connections: usize,
}

#[get("/stats")]
async fn stats(pool: Data<Pool<Postgres>>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Stats {
        idle_db_connections: pool.num_idle(),
    }))
}

pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(stats);
}
