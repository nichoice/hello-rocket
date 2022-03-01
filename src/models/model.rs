use rocket::serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};

// use chrono::{Utc, DateTime, prelude::*};

use super::super::schema::users;


#[derive(Serialize, Deserialize, Clone, Debug, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>
}