use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Vote {
    pub question_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
}

