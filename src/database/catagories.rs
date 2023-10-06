use diesel::{PgConnection, RunQueryDsl, QueryDsl};

use crate::models::catagory::{Catagory, NewCatagory};
use crate::errors::error::AppError;
use crate::schema::catagories::dsl::*;


pub(crate) fn db_get_all_catagories (
    conn: &mut PgConnection,
) -> Result<Option<Vec<Catagory>>, AppError> {
    let all_catagories = catagories.load::<Catagory>(conn)?;

    Ok(Some(all_catagories))
}

pub(crate) fn db_create_catagory (
    conn: &mut PgConnection,
    new_catagory: NewCatagory,
) -> Result<Catagory, AppError> {
    let catagory = diesel::insert_into(catagories)
        .values(&new_catagory)
        .get_result::<Catagory>(conn)?;

    Ok(catagory)
}

pub(crate) fn db_delete_catagory (
    conn: &mut PgConnection,
    catagory_id: i32,
) -> Result<usize, AppError> {
    let deleted_catagory = diesel::delete(catagories.find(catagory_id))
        .execute(conn)?;

    Ok(deleted_catagory)
}