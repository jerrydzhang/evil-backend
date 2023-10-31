use std::collections::HashMap;

use diesel::{prelude::{Queryable, Insertable, Associations, Identifiable}, AsChangeset};
use serde::{Serialize, Deserialize};

use crate::models::{user::User, product::Product};
use crate::schema::carts;

#[derive(Debug, Clone, Serialize, Associations, Deserialize, Queryable, Identifiable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = carts)]
pub(crate) struct CartItem {
    pub(crate) id: i32,
    user_id: String,
    pub(crate) product_id: String,
    pub(crate) quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = carts)]
pub(crate) struct NewCartItem {
    pub(crate) user_id: String,
    pub(crate) product_id: String,
    pub(crate) quantity: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CartSubmit {
    pub(crate) user_id: String,
    pub(crate) cart: HashMap<String, i32>,
}