use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::AsChangeset;
use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::schema::products;

#[derive(Debug, Default, Clone, Serialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = products)]
pub(crate) struct Product {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) category: Option<String>,
    pub(crate) price: Option<BigDecimal>,
    pub(crate) inventory: Option<i32>,
    pub(crate) last_updated: Option<NaiveDateTime>,
    pub(crate) created_at: Option<NaiveDateTime>,
    pub(crate) images: Option<Vec<Option<String>>>,
    pub(crate) price_id: Option<String>,
    pub(crate) active: bool,
    pub(crate) variant_id: i32,
}

impl Product {
    pub(crate) fn new(
        stripe_product: stripe::Product,
    ) -> Self {
        Self {
            id: stripe_product.id.to_string(),
            name: stripe_product.name.unwrap(),
            description: stripe_product.description,
            category: match stripe_product.metadata.clone() {
                Some(metadata) => metadata.get("category").map(|s| s.to_string()),
                None => None,
            },
            price: None,
            inventory: match stripe_product.metadata.clone() {
                Some(metadata) => Some(metadata.get("inventory").map(|s| s.parse::<i32>().unwrap()).unwrap_or(0)),
                None => None,
            },
            last_updated: None,
            created_at: None,
            images: stripe_product.images.map(|images| {
                images.into_iter().map(|image| Some(image)).collect()
            }),
            price_id: None,
            active: stripe_product.active.unwrap(),
            variant_id: match stripe_product.metadata.clone() {
                Some(metadata) => Some(metadata.get("variant_id").map(|s| s.parse::<i32>().unwrap()).unwrap_or(0)),
                None => None,
            }.unwrap_or(0),
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = products)]
pub(crate) struct NewProduct {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) category: Option<String>,
    pub(crate) price: Option<BigDecimal>,
    pub(crate) inventory: Option<i32>,
    pub(crate) last_updated: Option<NaiveDateTime>,
    pub(crate) created_at: Option<NaiveDateTime>,
    pub(crate) images: Option<Vec<Option<String>>>,
    pub(crate) price_id: Option<String>,
    pub(crate) active: Option<bool>,
    pub(crate) variant_id: Option<i32>,
}

impl NewProduct {
    pub(crate) fn new(
        stripe_product: stripe::Product,
    ) -> Self {
        Self {
            id: Some(stripe_product.id.to_string()),
            name: Some(stripe_product.name.unwrap()),
            description: stripe_product.description,
            category: match stripe_product.metadata.clone() {
                Some(metadata) => metadata.get("category").map(|s| s.to_string()),
                None => None,
            },
            price: None,
            inventory: match stripe_product.metadata.clone() {
                Some(metadata) => match metadata.get("inventory").map(|s| s.parse::<i32>().unwrap()) {
                    Some(inventory) => Some(inventory),
                    None => None,
                },
                None => None,
            },
            last_updated: None,
            created_at: None,
            images: stripe_product.images.map(|images| {
                images.into_iter().map(|image| Some(image)).collect()
            }),
            price_id: None,
            active: Some(stripe_product.active.unwrap()),
            variant_id: match stripe_product.metadata.clone() {
                Some(metadata) => match metadata.get("variant_id").map(|s| s.parse::<i32>().unwrap()) {
                    Some(variant_id) => Some(variant_id),
                    None => None,
                },
                None => None,
            },
        }
    }
}

// used to get a list of ids from the client
#[derive(Debug, Deserialize)]
pub(crate) struct ProductIds {
    pub(crate) ids: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct NewProductPayload {
    pub(crate) name: String,
    pub(crate) inventory: i32,
    pub(crate) description: Option<String>,
    pub(crate) image: String,
    pub(crate) category: String,
    pub(crate) price: BigDecimal,
    pub(crate) variant_id: i32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpdatePayload {
    pub(crate) name: String,
    pub(crate) inventory: i32,
    pub(crate) description: String,
    pub(crate) images: Vec<String>,
    pub(crate) is_active: bool,
}