#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

use actix_web::{middleware, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::env;

mod error;
mod stats;
mod todos;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = Pool::<Postgres>::connect(&database_url).await?;

    // Seems broken for now, use sqlx-cli in the meantime
    // info!("running migrations...");
    // sqlx::migrate!("./migrations").run(&db_pool).await?;
    // info!("completed");

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .configure(todos::routes)
            .configure(stats::routes)
    })
    .bind("localhost:8080")?
    .run()
    .await?;

    Ok(())
}
