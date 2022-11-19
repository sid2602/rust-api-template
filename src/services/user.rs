use actix_web::web;
use sqlx::{query_as, Error};

use crate::{model::user::User, config::db::DbPool};

pub struct UserService {
}

impl UserService {
    pub async fn get_users(pool: web::Data<DbPool>) -> Result<Vec<User>, Error>{
        query_as!(User, "SELECT * FROM users").fetch_all(&**pool).await
    }
}