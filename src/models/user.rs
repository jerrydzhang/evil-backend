use diesel::prelude::{Insertable, Queryable, Identifiable};
use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Queryable, Insertable)]
#[diesel(table_name = users)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) email: String,
    pub(crate) roles: Option<Vec<Option<String>>>,
    pub(crate) stripe_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SubmitRoles {
    pub(crate) user_id: String,
    pub(crate) stripe_customer_id: String,
    pub(crate) roles: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct UserId {
    pub(crate) id: String,
}