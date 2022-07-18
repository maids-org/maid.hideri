use diesel::prelude::*;
use super::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_user(
    uid: i32,
    conn: &PgConnection
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn create_user(uid: i32, language: &str, conn: &PgConnection
) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: uid,
        lang: language.to_owned()
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}