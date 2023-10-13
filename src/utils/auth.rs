use actix_identity::Identity;
use actix_web::web;

use crate::{models::dbpool::PgPool, database::users::db_get_user};

// verify the identity of the user and check if they have the required role
pub(crate) fn verify_identity(
    pool: web::Data<PgPool>,
    user: Identity,
    role: Vec<&str>,
) -> bool {
    let mut conn = pool.get().unwrap();

    // get the user from the database
    let db_user = db_get_user(&mut conn, user.id().unwrap()).unwrap();
    
    // if the user exists
    if let Some(db_user) = db_user {
        
        // if the user has roles
        if let Some(roles) = db_user.roles {
            
            // convert the roles to a vector of strings
            let roles: Vec<String> = roles.iter().flatten().map(|r| r.to_owned()).collect();

            // if the user has the required roles
            if role.iter().any(|r| roles.contains(&r.to_string())) {
                return true;
            }
        }
    }
    false
}