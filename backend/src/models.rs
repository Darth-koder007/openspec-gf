use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub email: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kudo {
    pub id: i64,
    #[serde(rename = "senderEmail")]
    pub sender_email: String,
    #[serde(rename = "recipientEmail")]
    pub recipient_email: String,
    pub message: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
}
