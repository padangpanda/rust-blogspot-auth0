use crate::schema::*;
use serde::{Deserialize, Serialize};

use super::account::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Account, foreign_key = "account_id")]
#[table_name = "settings"]
pub struct Setting {
    pub id: i32,
    pub friendly_name: String,
    pub logo_url: String,
    pub support_email: String,
    pub support_url: String,
    pub environment_tag: String,
    pub default_audience: String,
    pub default_directory: String,
    pub default_error_page: String,
    pub default_error_page_url: String,
    pub default_language: String,
    pub account_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "settings"]
pub struct NewSetting<'a> {
    pub friendly_name: &'a str,
    pub logo_url: &'a str,
    pub support_email: &'a str,
    pub support_url: &'a str,
    pub environment_tag: &'a str,
    pub default_audience: &'a str,
    pub default_directory: &'a str,
    pub default_error_page: &'a str,
    pub default_error_page_url: &'a str,
    pub default_language: &'a str,
    pub account_id: &'a i32,
}