#[derive(Queryable, Serialize)]
pub struct Player {
    agent: i32,
    game: i32,
    rank: i32,
}
