use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
    admin: bool,
    created: NaiveDateTime,
    active: Option<NaiveDateTime>,
}
