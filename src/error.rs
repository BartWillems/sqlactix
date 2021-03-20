use std::fmt;

use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "TODO task already exists: {}", _0)]
    Duplicate(String),
    #[display(fmt = "unkown database error: {:?}", _0)]
    Unknown(Option<String>),
}

#[derive(Serialize)]
struct ErrorResponse<T: fmt::Display + fmt::Debug> {
    error: String,
    description: Option<T>,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Duplicate(ref message) => HttpResponse::Conflict().json(ErrorResponse {
                error: String::from("conflict"),
                description: Some(message),
            }),
            Error::Unknown(ref message) => {
                error!("Unknown error occured: {:?}", message);
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Error {
        match error {
            sqlx::Error::Database(err) => {
                err.downcast_ref::<sqlx::postgres::PgDatabaseError>().into()
            }
            _ => Error::Unknown(None),
        }
    }
}

impl From<&sqlx::postgres::PgDatabaseError> for Error {
    fn from(error: &sqlx::postgres::PgDatabaseError) -> Error {
        match error.code() {
            "23505" => Error::Duplicate(format!(
                "resource already exists in {}",
                error.table().unwrap_or("_unknown_table")
            )),
            _ => Error::Unknown(Some(error.to_string())),
        }
    }
}
