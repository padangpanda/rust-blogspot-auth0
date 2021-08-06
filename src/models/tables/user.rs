use crate::schema::*;
use serde::{Deserialize, Serialize};

use super::account::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Account, foreign_key = "account_id")]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub latest_login: chrono::NaiveDateTime,
    pub connection: String,
    pub provider: String,
    pub is_social: bool,
    pub picture: String,
    pub updated_at: chrono::NaiveDateTime,
    pub blocked: bool,
    pub blocked_for: String,
    pub guardian_authenticators: String,
    pub account_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub latest_login: chrono::NaiveDateTime,
    pub connection: &'a str,
    pub provider: &'a str,
    pub is_social: &'a bool,
    pub picture: &'a str,
    pub updated_at: chrono::NaiveDateTime,
    pub blocked: &'a bool,
    pub blocked_for: &'a str,
    pub guardian_authenticators: &'a str,
    pub account_id: &'a i32,
}