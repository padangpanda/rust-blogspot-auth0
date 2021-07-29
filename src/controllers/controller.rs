// #![allow(unused)]
// extern crate bcrypt;
use crate::{
    helpers::constants,
    Pool,
    models::{
        response::{LoginResponse, ServiceError, ResponseBody},
        models::{NewUser, User, InputUserRegister, InputUserLogin, Claims}
    },
    schema::users,
    schema::users::dsl::*,
    diesel::{QueryDsl, RunQueryDsl, ExpressionMethods}
};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{DateTime, Duration, Utc};
use bcrypt::{DEFAULT_COST, hash, verify};
use actix_web::{web, http::StatusCode};
use diesel::dsl::{delete, insert_into};
use validator::{Validate, ValidationErrors};

pub fn register_handler(
    db: web::Data<Pool>,
    item: web::Form<InputUserRegister>,
) -> Result<ResponseBody<String>, ValidationErrors> {
    let conn = db.get().unwrap();
    let hashed = hash(&item.password, DEFAULT_COST).unwrap();
    let new_user = NewUser {
        name: &item.name,
        password: &hashed,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    
    match item.validate() {
        Ok(_) => {
            let res: User = insert_into(users).values(new_user).get_result(&conn).unwrap();
            
            Ok(ResponseBody::new("You are successfully registered", res.email.to_string()))
        },
        Err(err) => Err(err)
    }
}

pub fn login_handler(
    db: web::Data<Pool>,
    item: web::Form<InputUserLogin>
) -> Result<LoginResponse, ServiceError> {
    let conn = db.get().unwrap();
    match users::table.filter(users::email.eq(item.email.to_string())).get_result::<User>(&conn) {
        Ok(found_user) => {
            println!("{:?}", &found_user);
            match verify(&item.password, &found_user.password) {
                Ok(true) => {
                    // println!("{}", a);
                    let key = "ayambetelor".as_bytes();
                    let expired: DateTime<Utc> = Utc::now() + Duration::days(1);
                    let my_claims = Claims {
                        email: found_user.email.to_string(),
                        exp: expired.timestamp() as usize
                    };
                    let token = encode(
                        &Header::default(),
                        &my_claims,
                        &EncodingKey::from_secret(key)
                    ).unwrap();
                    Ok(LoginResponse {
                        username: found_user.name.to_string(),
                        token,
                        email: found_user.email.to_string(),
                    })
                },
                Ok(false) => Err(ServiceError::new(StatusCode::UNAUTHORIZED, constants::MESSAGE_LOGIN_FAILED.to_string())),
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
        },
        Err(_) => Err(ServiceError::new(StatusCode::UNAUTHORIZED, constants::MESSAGE_USER_NOT_FOUND.to_string()))
    
    }
}

pub fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

pub fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

pub fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}