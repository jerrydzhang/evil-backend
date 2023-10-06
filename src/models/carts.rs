use std::collections::HashMap;

use diesel::{prelude::{Queryable, Insertable}, AsChangeset};
use serde::{Serialize, Deserialize};

use crate::schema::carts;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = carts)]
pub(crate) struct CartItem {
    pub(crate) id: i32,
    user_id: String,
    pub(crate) product_id: i32,
    pub(crate) quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = carts)]
pub(crate) struct NewCartItem {
    pub(crate) user_id: String,
    pub(crate) product_id: i32,
    pub(crate) quantity: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CartSubmit {
    pub(crate) user_id: String,
    pub(crate) cart: HashMap<i32, i32>,
}