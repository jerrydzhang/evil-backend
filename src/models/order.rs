use diesel::{prelude::{Queryable, Insertable}, AsChangeset};
use serde::{Serialize, Deserialize};

use crate::schema::orders;

use super::product::Product;


#[derive(Debug, Clone, Serialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = orders)]
pub(crate) struct Order {
    pub(crate) id: String,
    pub(crate) user_id: String,
    pub(crate) products: serde_json::Value,
    pub(crate) status: String,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) created_at: chrono::NaiveDateTime,
    pub(crate) updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Default, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = orders)]
pub(crate) struct NewOrder {
    pub(crate) id: Option<String>,
    pub(crate) user_id: Option<String>,
    pub(crate) products: Option<serde_json::Value>,
    pub(crate) status: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) created_at: Option<chrono::NaiveDateTime>,
    pub(crate) updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ExpandedOrder {
    pub(crate) id: String,
    pub(crate) user_id: String,
    pub(crate) products: Vec<OrderProduct>,
    pub(crate) status: String,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) created_at: chrono::NaiveDateTime,
    pub(crate) updated_at: chrono::NaiveDateTime,
}

impl ExpandedOrder{
    pub(crate) fn new(
        order: Order,  
        products: Vec<OrderProduct>,   
    ) -> Self {
        Self {
            id: order.id,
            user_id: order.user_id,
            products: products,
            status: order.status,
            name: order.name,
            address: order.address,
            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct OrderProduct {
    product: Product,
    quantity: i32,
}

impl OrderProduct {
    pub(crate) fn new(
        product: Product,
        quantity: i32,
    ) -> Self {
        Self {
            product: product,
            quantity: quantity,
        }
    }
}