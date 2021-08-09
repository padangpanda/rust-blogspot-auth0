#![allow(unused_imports)]
// extern crate bcrypt;
use crate::{
    helpers::constants,
    Pool,
    models::{
        response::{LoginResponse, ServiceError, ResponseBody},
        tables::account::{NewAccount, Account, InputAccountRegister, InputAccountLogin, Claims},
        tables::setting::{NewSetting, Setting}
    },
    schema::{accounts, settings},
    schema::accounts::dsl::*,
    schema::settings::dsl::*,
    diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, select}
};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{DateTime, Duration, Utc};
use bcrypt::{hash, verify};
use actix_web::{web, http::StatusCode};
use diesel::dsl::{insert_into, exists};
use validator::{Validate};

pub fn register_handler(
    db: web::Data<Pool>,
    item: web::Form<InputAccountRegister>,
) -> Result<ResponseBody<String>, ServiceError> {
    let conn = db.get().unwrap();
    let hashed = hash(&item.password, 8).unwrap();
    let first_domain = &item.tenant_domain;
    let temp_region = &item.region;
    let tenant = format!("{}.{}.auth0.com", first_domain, temp_region);
    let environment = "development";
    let provid = "database";
    let new_account = NewAccount {
        name: &item.name,
        password: &hashed,
        email: &item.email,
        account_type: &item.account_type,
        tenant_domain: &tenant,
        region: &item.region,
        environment_tag: &environment,
        provider: &provid,
        created_at: chrono::Local::now().naive_local(),
    };

    match select(exists(accounts::table.filter(email.eq(new_account.email.to_string())))).get_result(&conn) {
        Ok(true) => Err(ServiceError::new(StatusCode::BAD_REQUEST, constants::MESSAGE_SIGNUP_EMAIL_ALREADY_USED.to_string())),
        Ok(false) => match item.validate() {
            Ok(_) => {
                let res: Account = insert_into(accounts).values(new_account).get_result(&conn).unwrap();
                
                let new_setting = NewSetting {
                    friendly_name: "",
                    logo_url: "https://cdn.icon-icons.com/icons2/2248/PNG/512/cat_icon_138789.png",
                    support_email: "",
                    support_url: "",
                    environment_tag: &environment,
                    default_audience: "",
                    default_directory: "",
                    default_error_page: "generic",
                    default_error_page_url: "",
                    default_language: "en",
                    account_id: &res.id,
                };

                let _: Setting = insert_into(settings).values(new_setting).get_result(&conn).unwrap();
            

                Ok(ResponseBody::new("You are successfully registered", res.email.to_string()))
            },
            Err(_) => Err(ServiceError::new(StatusCode::BAD_REQUEST, constants::MESSAGE_VALIDATION_ERROR.to_string()))
        },
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
    }
}

pub fn login_handler(
    db: web::Data<Pool>,
    item: web::Form<InputAccountLogin>
) -> Result<LoginResponse, ServiceError> {
    let conn = db.get().unwrap();
    match accounts::table.filter(accounts::email.eq(item.email.to_string())).get_result::<Account>(&conn) {
        Ok(found_account) => {
            match verify(&item.password, &found_account.password) {
                Ok(true) => {
                    let key: String = std::env::var("TOKEN_KEY").expect("TOKEN_KEY must be set");
                    let expired: DateTime<Utc> = Utc::now() + Duration::days(1);
                    let my_claims = Claims {
                        email: found_account.email.to_string(),
                        exp: expired.timestamp() as usize
                    };
                    let token = encode(
                        &Header::default(),
                        &my_claims,
                        &EncodingKey::from_secret(key.as_bytes())
                    ).unwrap();
                    Ok(LoginResponse {
                        username: found_account.name.to_string(),
                        token,
                        email: found_account.email.to_string(),
                    })
                },
                Ok(false) => Err(ServiceError::new(StatusCode::UNAUTHORIZED, constants::MESSAGE_LOGIN_FAILED.to_string())),
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
        },
        Err(_) => Err(ServiceError::new(StatusCode::UNAUTHORIZED, constants::MESSAGE_USER_NOT_FOUND.to_string()))
    
    }
}
