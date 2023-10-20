use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods, result::Error};
use diesel::prelude::*;


use crate::{models::user::User, schema::users::dsl::*};

pub(crate) fn db_create_user (
    conn: &mut PgConnection,
    new_user: User,
) -> Result<User, Error> {
    let user = diesel::insert_into(users)
        .values(&new_user)
        .on_conflict_do_nothing()
        .get_result::<User>(conn)?;

    Ok(user)
}

pub(crate) fn db_update_user (
    conn: &mut PgConnection,
    user_id: String,
    stripe: String,
    new_roles: Vec<String>,
) -> Result<User, Error> {
    let user = diesel::update(users.find(user_id))
        .set((stripe_id.eq(stripe), roles.eq(new_roles)))
        .get_result::<User>(conn)?;

    Ok(user)
}

pub(crate) fn db_delete_user (
    conn: &mut PgConnection,
    user_id: String,
) -> Result<usize, Error> {
    let deleted_user = diesel::delete(users.find(user_id))
        .execute(conn)?;

    Ok(deleted_user)
}

pub(crate) fn db_get_user (
    conn: &mut PgConnection,
    user_id: String,
) -> Result<Option<User>, Error> {
    let user = users.find(user_id)
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub(crate) fn db_user_stripe_to_user_id (
    conn: &mut PgConnection,
    stripe_user_id: String,
) -> Result<Option<User>, Error> {
    let user = users.filter(stripe_id.eq(stripe_user_id))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}