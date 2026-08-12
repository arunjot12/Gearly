use crate::models::{NewSignupShopkeepers, NewUsers, SignupShopkeepers, Users};
use crate::schema::{signup_shopkeepers, users};
use diesel::{insert_into, mysql::MysqlConnection, prelude::*};

#[derive(Debug,thiserror::Error)]
pub enum AppError {
    #[error("invalid creditionals")]
    InvalidCreditionals,

    #[error("User Not Found")]
    UserNotFound,

    #[error("Database error")]
    Database(#[from]diesel::result::Error),

    #[error("Internal Server Error")]
    Internal
}

pub fn handle_customer_signup(
    connection: &mut MysqlConnection,
    customer: &NewUsers,
) -> Result<String, String> {
    let check_customer_number = users::table
        .select(Users::as_select())
        .filter(users::phone_number.eq(&customer.phone_number))
        .first(connection)
        .optional();

    match check_customer_number {
        Ok(_) => {
            println!("Validation is successfully")
        }
        Err(err) => return Err(err.to_string()),
    }
    let insert_result = insert_into(users::table)
        .values(customer)
        .execute(connection);

    match insert_result {
        Ok(_) => Ok("✅ Shopkeeper registered successfully!".to_string()),
        Err(err) => return Err(err.to_string()),
    }
}

pub fn handle_shopkeeper_signup(
    connection: &mut MysqlConnection,
    shopkeeper: &NewSignupShopkeepers,
) -> Result<String, String> {
    if let Some(ref phone) = shopkeeper.phone_number {
        let check_shopkeeper_number: Result<Option<SignupShopkeepers>, _> =
            signup_shopkeepers::table
                .select(SignupShopkeepers::as_select())
                .filter(signup_shopkeepers::phone_number.eq(phone))
                .first(connection)
                .optional();

        match check_shopkeeper_number {
            Ok(_) => {
                println!("Validation is successfully")
            }
            Err(err) => return Err(err.to_string()),
        }
    }

    let insert_result = insert_into(signup_shopkeepers::table)
        .values(shopkeeper)
        .execute(connection);

    match insert_result {
        Ok(_) => Ok("✅ Shopkeeper registered successfully!".to_string()),
        Err(err) => return Err(err.to_string()),
    }
}
