use actix_web::{get, post, web::{self}, App, HttpResponse, HttpServer, Responder, middleware::Logger,};
use sqlx::{Pool, Postgres, Executor, postgres::PgQueryResult, Error, query_as};

use crate::{model::user::User, config::CustomError::{CustomError, map_io_error}};

pub type DbPool = Pool<Postgres>;


#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, CustomError> {


    let response = query_as!(User, "SELECT * FROM users").fetch_all(&**pool).await;

    match response {
         Ok(resp) => return Ok(HttpResponse::Ok().json(resp)),
         Err(error) => return  Err(CustomError::Unknown)
    }
}