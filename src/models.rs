use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Insertable,Serialize, Deserialize, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUsers {
    pub first_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
}

#[derive(Debug, Queryable, Selectable,Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::signup_shopkeepers)]
pub struct NewSignupShopkeepers {
    pub first_name: Option<String>,
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub shop_name: Option<String>,
    pub shop_address: Option<String>,
    pub city: Option<String>,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct Users {
    pub id: i32,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub phone_number: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::signup_shopkeepers)]
pub struct SignupShopkeepers {
    pub id: i32,
    pub first_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub shop_name: Option<String>,
    pub shop_address: Option<String>,
    pub city: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login{
    pub username_or_email: String,
    pub password: String
}