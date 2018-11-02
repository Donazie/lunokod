use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Agent {
    agentid: i32,
    agentuser: i32,
    agentcode: String,
    agentcreated: NaiveDateTime,
    agentmodified: Option<NaiveDateTime>,
}
