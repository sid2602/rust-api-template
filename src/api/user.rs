use actix_web::{get, post, web::{self}, App, HttpResponse, HttpServer, Responder, middleware::Logger,};
use sqlx::{Pool, Postgres, Executor, postgres::PgQueryResult, Error, query_as};

use crate::model::user::User;

pub type DbPool = Pool<Postgres>;


#[get("/user")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {


    let users = query_as!(User, "SELECT * FROM users").fetch_all(&**pool).await;

    if let Ok(users) = users {
     return HttpResponse::Ok().json(users);
    }

    return HttpResponse::Ok().body("Error");
}