use crate::schema::*;
use serde::{Deserialize, Serialize};

use super::{user::*, role::*};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Role, foreign_key = "role_id")]
#[table_name = "user_role"]
pub struct UserRole {
    pub id: i32, 
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "user_role"]
pub struct NewUserRole<'a> {
    pub user_id: &'a i32,
    pub role_id: &'a i32,
}