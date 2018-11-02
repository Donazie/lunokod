use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Game {
    gameid: i32,
    gameuser: i32,
    gamestarted: NaiveDateTime,
    gamefinished: NaiveDateTime,
}
