use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: u16,
    pub qq: String,
    pub email: String,
    pub username: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}