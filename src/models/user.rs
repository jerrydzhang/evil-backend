use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) email: String,
}