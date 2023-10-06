use diesel::{prelude::{Insertable, Queryable}, AsChangeset};
use serde::{Serialize, Deserialize};

use crate::schema::catagories;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = catagories)]
pub(crate) struct Catagory {
    id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = catagories)]
pub(crate) struct NewCatagory {
    pub(crate) name: String,
}