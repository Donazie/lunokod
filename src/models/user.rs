use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct User {
    userid: i32,
    username: String,
    useremail: String,
    userpassword: String,
    useradmin: bool,
    usercreated: NaiveDateTime,
    useractive: Option<NaiveDateTime>,
}
