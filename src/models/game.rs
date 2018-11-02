use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Game {
    id: i32,
    user: i32,
    started: NaiveDateTime,
    finished: NaiveDateTime,
}
