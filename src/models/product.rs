use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::AsChangeset;
use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::schema::products;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = products)]
pub(crate) struct Product {
    pub(crate) id: String,
    name: String,
    description: Option<String>,
    catagory: Option<String>,
    price: Option<BigDecimal>,
    inventory: i32,
    last_updated: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
    images: Option<Vec<Option<String>>>,
    price_id: Option<String>,
    active: bool,
}

#[derive(Debug, Clone, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = products)]
pub(crate) struct NewProduct {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    catagory: Option<String>,
    pub(crate) price: Option<BigDecimal>,
    inventory: i32,
    pub(crate) images: Option<Vec<Option<String>>>,
    price_id: Option<String>,
    active: bool,
}

// used to get a list of ids from the client
#[derive(Debug, Deserialize)]
pub(crate) struct ProductIds {
    pub(crate) ids: String,
}

impl Product {
    pub(crate) fn new(
        stripe_product: stripe::Product,
    ) -> Self
    {
        Self {
            id: stripe_product.id.to_string(),
            name: stripe_product.name.unwrap(),
            description: stripe_product.description,
            catagory: match stripe_product.metadata {
                Some(metadata) => metadata.get("catagory").map(|s| s.to_string()),
                None => None,
            },
            price: None,
            inventory: 0,
            last_updated: None,
            created_at: None,
            images: stripe_product.images.map(|images| {
                images.into_iter().map(|image| Some(image)).collect()
            }),
            price_id: None,
            active: stripe_product.active.unwrap(),
        }
    }
}