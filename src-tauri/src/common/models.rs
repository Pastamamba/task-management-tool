use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggedHour {
    pub date: NaiveDateTime,
    pub minutes: i32,
}
