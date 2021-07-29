// extern crate passwords;

use crate::schema::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use passwords::analyzer;

// use crate::helpers::validator::unique_email;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct InputUserRegister {
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8), custom = "contain_everything")]
    pub password: String,
}

fn contain_everything(password: &str) -> Result<(), ValidationError> {
    let tobe_checked = analyzer::analyze(password);
    let uppercase = tobe_checked.uppercase_letters_count();
    let lowercase = tobe_checked.lowercase_letters_count();
    let number = tobe_checked.numbers_count();

    if uppercase >= 1 && number >= 1 && lowercase >= 1 {
        return Ok(())
    }
    
    return Err(ValidationError::new("Password must contain uppercase lowercase and numbers!"));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUserLogin {
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub exp: usize,
}