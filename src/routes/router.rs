use crate::controllers::controller::*;
use crate::Pool;
use crate::models::models::{InputUserRegister, InputUserLogin};
use actix_web::{web, Error, HttpResponse};

// Handler for POST /register
pub async fn register(
    db: web::Data<Pool>,
    item: web::Form<InputUserRegister>,
) -> Result<HttpResponse, Error> {
    match register_handler(db, item) {
        Ok(result) => Ok(HttpResponse::Created().json(result)),
        Err(e) => Ok(HttpResponse::BadRequest().json(e))
    }
}

pub async fn login(
    db: web::Data<Pool>,
    item: web::Form<InputUserLogin>,
) -> Result<HttpResponse, Error> {
    match login_handler(db, item) {
        Ok(loggedin) => Ok(HttpResponse::Ok().json(loggedin)),
        Err(e) => Ok(e.response())
    }
    
}

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}


pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for DELETE /users/{id}
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_user(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

