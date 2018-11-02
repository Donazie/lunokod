use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Agent {
    id: i32,
    user: i32,
    code: String,
    created: NaiveDateTime,
    modified: Option<NaiveDateTime>,
}
