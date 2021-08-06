use crate::schema::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use passwords::analyzer;

// models for table schema

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, PartialEq, Associations)]
// pub struct Account {
//     pub id: i32,
//     pub name: String,
//     pub email: String,
//     pub password: String,
//     pub account_type: String,
//     pub tenant_domain: String,
//     pub region: String,
//     pub environment_tag: String,
//     pub provider: String,
//     pub created_at: chrono::NaiveDateTime,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
// #[belongs_to(Account, foreign_key = "account_id")]
// #[table_name = "applications"]
// pub struct Application {
//     pub id: i32,
//     pub name: String,
//     pub client_id: String,
//     pub app_type: String,
//     pub domain: String,
//     pub client_secret: String,
//     pub description: String,
//     pub logo_url: String,
//     pub token_auth_method: String,
//     pub app_login_url: String,
//     pub callback_url: String,
//     pub logout_url: String,
//     pub web_origin: String,
//     pub cors: String,
//     pub id_token_exp: i32,
//     pub reuse_interval: i32,
//     pub abs_lifetime: i32,
//     pub inactivity_lifetime: i32,
//     pub account_id: i32,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
// #[belongs_to(Account, foreign_key = "account_id")]
// #[table_name = "apis"]
// pub struct Api {
//     pub id: i32,
//     pub name: String,
//     pub api_id: String,
//     pub identifier: String,
//     pub token_exp: i32,
//     pub token_exp_browser: i32,
//     pub sign_algorithm: String,
//     pub rbac: bool,
//     pub permission_acc_token: bool,
//     pub allow_skip_user: bool,
//     pub allow_off_acc: bool,
//     pub account_id: i32,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
// #[belongs_to(Account, foreign_key = "account_id")]
// #[table_name = "users"]
// pub struct User {
//     pub id: i32,
//     pub email: String,
//     pub password: String,
//     pub latest_login: chrono::NaiveDateTime,
//     pub connection: String,
//     pub provider: String,
//     pub is_social: bool,
//     pub picture: String,
//     pub updated_at: chrono::NaiveDateTime,
//     pub blocked: bool,
//     pub blocked_for: String,
//     pub guardian_authenticators: String,
//     pub account_id: i32,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
// #[belongs_to(Account, foreign_key = "account_id")]
// #[table_name = "roles"]
// pub struct Role {
//     pub id: i32,
//     pub name: String,
//     pub description: String,
//     pub account_id: i32,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
// #[belongs_to(Account, foreign_key = "account_id")]
// #[table_name = "settings"]
// pub struct Setting {
//     pub id: i32,
//     pub friendly_name: String,
//     pub logo_url: String,
//     pub support_email: String,
//     pub support_url: String,
//     pub environment_tag: String,
//     pub default_audience: String,
//     pub default_directory: String,
//     pub default_error_page: String,
//     pub default_error_page_url: String,
//     pub default_language: String,
//     pub account_id: i32,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
// #[belongs_to(User, foreign_key = "user_id")]
// #[belongs_to(Role, foreign_key = "role_id")]
// #[table_name = "user_role"]
// pub struct UserRole {
//     pub id: i32, 
//     pub user_id: i32,
//     pub role_id: i32,
// }

// models for inserting data into table (database)

// #[derive(Insertable, Debug)]
// #[table_name = "accounts"]
// pub struct NewAccount<'a> {
//     pub name: &'a str,
//     pub email: &'a str,
//     pub password: &'a str,
//     pub account_type: &'a str,
//     pub tenant_domain: &'a str,
//     pub region: &'a str,
//     pub environment_tag: &'a str,
//     pub provider: &'a str,
//     pub created_at: chrono::NaiveDateTime,
// }

// #[derive(Insertable, Debug)]
// #[table_name = "applications"]
// pub struct NewApplication<'a> {
//     pub name: &'a str,
//     pub client_id: &'a str,
//     pub app_type: &'a str,
//     pub domain: &'a str,
//     pub client_secret: &'a str,
//     pub description: &'a str,
//     pub logo_url: &'a str,
//     pub token_auth_method: &'a str,
//     pub app_login_url: &'a str,
//     pub callback_url: &'a str,
//     pub logout_url: &'a str,
//     pub web_origin: &'a str,
//     pub cors: &'a str,
//     pub id_token_exp: &'a i32,
//     pub reuse_interval: &'a i32,
//     pub abs_lifetime: &'a i32,
//     pub inactivity_lifetime: &'a i32,
//     pub account_id: &'a i32,
// }

// #[derive(Insertable, Debug)]
// #[table_name = "apis"]
// pub struct NewApi<'a> {
//     pub name: &'a str,
//     pub api_id: &'a str,
//     pub identifier: &'a str,
//     pub token_exp: &'a i32,
//     pub token_exp_browser: &'a i32,
//     pub sign_algorithm: &'a str,
//     pub rbac: &'a bool,
//     pub permission_acc_token: &'a bool,
//     pub allow_skip_user: &'a bool,
//     pub allow_off_acc: &'a bool,
//     pub account_id: &'a i32,
// }

// #[derive(Insertable, Debug)]
// #[table_name = "users"]
// pub struct NewUser<'a> {
//     pub email: &'a str,
//     pub password: &'a str,
//     pub latest_login: chrono::NaiveDateTime,
//     pub connection: &'a str,
//     pub provider: &'a str,
//     pub is_social: &'a bool,
//     pub picture: &'a str,
//     pub updated_at: chrono::NaiveDateTime,
//     pub blocked: &'a bool,
//     pub blocked_for: &'a str,
//     pub guardian_authenticators: &'a str,
//     pub account_id: &'a i32,
// }

// #[derive(Insertable, Debug)]
// #[table_name = "roles"]
// pub struct NewRole<'a> {
//     pub name: &'a str,
//     pub description: &'a str,
//     pub account_id: &'a i32,
// }

// #[derive(Insertable, Debug)]
// #[table_name = "settings"]
// pub struct NewSetting<'a> {
//     pub friendly_name: &'a str,
//     pub logo_url: &'a str,
//     pub support_email: &'a str,
//     pub support_url: &'a str,
//     pub environment_tag: &'a str,
//     pub default_audience: &'a str,
//     pub default_directory: &'a str,
//     pub default_error_page: &'a str,
//     pub default_error_page_url: &'a str,
//     pub default_language: &'a str,
//     pub account_id: &'a i32,
// }

// #[derive(Insertable, Debug)]
// #[table_name = "user_role"]
// pub struct NewUserRole<'a> {
//     pub user_id: &'a i32,
//     pub role_id: &'a i32,
// }

// models for request body from client

// #[derive(Debug, Serialize, Deserialize, Validate)]
// pub struct InputAccountRegister {
//     pub name: String,
//     #[validate(email)]
//     pub email: String,
//     #[validate(length(min = 8), custom = "contain_everything")]
//     pub password: String,
// }

// fn contain_everything(password: &str) -> Result<(), ValidationError> {
//     let tobe_checked = analyzer::analyze(password);
//     let uppercase = tobe_checked.uppercase_letters_count();
//     let lowercase = tobe_checked.lowercase_letters_count();
//     let number = tobe_checked.numbers_count();

//     if uppercase >= 1 && number >= 1 && lowercase >= 1 {
//         return Ok(())
//     }
    
//     return Err(ValidationError::new("Password must contain uppercase lowercase and numbers!"));
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct InputAccountLogin {
//     pub email: String,
//     pub password: String
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     pub email: String,
//     pub exp: usize,
// }