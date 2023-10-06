use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::AsChangeset;
use diesel::prelude::{Insertable, Queryable, Associations};
use serde::{Serialize, Deserialize};

use crate::schema::products;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Associations)]
#[belongs_to(super::catagory::Catagory)]
#[diesel(table_name = products)]
pub(crate) struct Product {
    id: i32,
    name: String,
    description: Option<String>,
    catagory_id: i32,
    price: BigDecimal,
    inventory: i32,
    last_updated: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = products)]
pub(crate) struct NewProduct {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) catagory_id: i32,
    pub(crate) price: BigDecimal,
    pub(crate) inventory: i32,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DisplayProduct {
    id: i32,
    name: String,
    description: Option<String>,
    catagory_id: i32,
    catagory: String,
    price: BigDecimal,
    inventory: i32,
    last_updated: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
}

// used to get a list of ids from the client
#[derive(Debug, Deserialize)]
pub(crate) struct Ids {
    pub(crate) ids: String,
}

impl DisplayProduct {
    pub(crate) fn new(
        product: Product,
        catagory: String,
    ) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            catagory_id: product.catagory_id,
            catagory,
            price: product.price,
            inventory: product.inventory,
            last_updated: product.last_updated,
            created_at: product.created_at,
        }
    }
}