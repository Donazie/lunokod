use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct User {
    userid: i32,
    username: String,
    useremail: String,
    userpassword: String,
    useradmin: i32,
    usercreated: NaiveDateTime,
    useractive: Option<NaiveDateTime>,
}
