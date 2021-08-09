use crate::controllers::controller_account::*;
use crate::Pool;
use crate::models::tables::account::{InputAccountRegister, InputAccountLogin};
use actix_web::{web, Error, HttpResponse};

// Handler for POST /register
pub async fn register(
    db: web::Data<Pool>,
    item: web::Form<InputAccountRegister>,
) -> Result<HttpResponse, Error> {
    match register_handler(db, item) {
        Ok(result) => Ok(HttpResponse::Created().json(result)),
        Err(e) => Ok(e.response())
    }
}

pub async fn login(
    db: web::Data<Pool>,
    item: web::Form<InputAccountLogin>,
) -> Result<HttpResponse, Error> {
    match login_handler(db, item) {
        Ok(loggedin) => Ok(HttpResponse::Ok().json(loggedin)),
        Err(e) => Ok(e.response())
    }
    
}

