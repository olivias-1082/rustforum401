use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub id: String,
    pub from_user_id: u16,
    pub to_user_id: u16,
    pub content: String,
    pub create_time: NaiveDateTime
}