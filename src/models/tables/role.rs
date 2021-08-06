use crate::schema::*;
use serde::{Deserialize, Serialize};

use super::account::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Account, foreign_key = "account_id")]
#[table_name = "roles"]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub account_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "roles"]
pub struct NewRole<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub account_id: &'a i32,
}