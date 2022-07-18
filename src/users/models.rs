use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub lang: String,
}