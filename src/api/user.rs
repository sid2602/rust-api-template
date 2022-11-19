use actix_web::{get, web::{self}, HttpResponse,};

use crate::{config::{CustomError::{CustomError, map_sqlx_error}, db::DbPool}, services::user::UserService};

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, CustomError> {

    let response = UserService::get_users(pool).await;

    match response {
         Ok(resp) => return Ok(HttpResponse::Ok().json(resp)),
         Err(error) => return  Err(map_sqlx_error(error))
    }
}