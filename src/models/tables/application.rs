use crate::schema::*;
use serde::{Deserialize, Serialize};

use super::account::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Account, foreign_key = "account_id")]
#[table_name = "applications"]
pub struct Application {
    pub id: i32,
    pub name: String,
    pub client_id: String,
    pub app_type: String,
    pub domain: String,
    pub client_secret: String,
    pub description: String,
    pub logo_url: String,
    pub token_auth_method: String,
    pub app_login_url: String,
    pub callback_url: String,
    pub logout_url: String,
    pub web_origin: String,
    pub cors: String,
    pub id_token_exp: i32,
    pub reuse_interval: i32,
    pub abs_lifetime: i32,
    pub inactivity_lifetime: i32,
    pub account_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "applications"]
pub struct NewApplication<'a> {
    pub name: &'a str,
    pub client_id: &'a str,
    pub app_type: &'a str,
    pub domain: &'a str,
    pub client_secret: &'a str,
    pub description: &'a str,
    pub logo_url: &'a str,
    pub token_auth_method: &'a str,
    pub app_login_url: &'a str,
    pub callback_url: &'a str,
    pub logout_url: &'a str,
    pub web_origin: &'a str,
    pub cors: &'a str,
    pub id_token_exp: &'a i32,
    pub reuse_interval: &'a i32,
    pub abs_lifetime: &'a i32,
    pub inactivity_lifetime: &'a i32,
    pub account_id: &'a i32,
}