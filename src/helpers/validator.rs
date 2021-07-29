use actix_web::web;
use crate::Pool;
use validator::ValidationError;

use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::diesel::{
    QueryDsl, RunQueryDsl, ExpressionMethods
};
use crate::models::models::User;

// pub type

pub fn unique_email(pool: web::Data<Pool>, input_email: &str) -> Result<(), ValidationError> { 
    let conn = pool.get().unwrap();
    match users::table.filter(users::email.eq(input_email.to_string())).load::<User>(&conn) {
        Ok(_) => Err(ValidationError::new("test error unique email")),
        Err(_) => Ok(())
    }
}