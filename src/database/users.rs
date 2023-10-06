use diesel::{PgConnection, RunQueryDsl, QueryDsl};

use crate::{models::user::User, errors::error::AppError, schema::users::dsl::*};

pub(crate) fn db_create_user (
    conn: &mut PgConnection,
    new_user: User,
) -> Result<User, AppError> {
    let user = diesel::insert_into(users)
        .values(&new_user)
        .on_conflict_do_nothing()
        .get_result::<User>(conn)?;

    Ok(user)
}

pub(crate) fn db_delete_user (
    conn: &mut PgConnection,
    user_id: String,
) -> Result<usize, AppError> {
    let deleted_user = diesel::delete(users.find(user_id))
        .execute(conn)?;

    Ok(deleted_user)
}
