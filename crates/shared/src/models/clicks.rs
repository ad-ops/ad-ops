use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Clicks {
    pub amount: u32,
    pub last_click_ts: DateTime<Utc>,
}