use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    pub id: String,
    pub user_id: u16,
    pub content: String,
    pub username: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}