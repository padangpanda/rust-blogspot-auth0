use crate::schema::*;
use serde::{Deserialize, Serialize};

use super::account::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Account, foreign_key = "account_id")]
#[table_name = "apis"]
pub struct Api {
    pub id: i32,
    pub name: String,
    pub api_id: String,
    pub identifier: String,
    pub token_exp: i32,
    pub token_exp_browser: i32,
    pub sign_algorithm: String,
    pub rbac: bool,
    pub permission_acc_token: bool,
    pub allow_skip_user: bool,
    pub allow_off_acc: bool,
    pub account_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "apis"]
pub struct NewApi<'a> {
    pub name: &'a str,
    pub api_id: &'a str,
    pub identifier: &'a str,
    pub token_exp: &'a i32,
    pub token_exp_browser: &'a i32,
    pub sign_algorithm: &'a str,
    pub rbac: &'a bool,
    pub permission_acc_token: &'a bool,
    pub allow_skip_user: &'a bool,
    pub allow_off_acc: &'a bool,
    pub account_id: &'a i32,
}