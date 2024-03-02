use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Like {
    pub id: String,
    pub created_at: DateTime<Utc>,
}
