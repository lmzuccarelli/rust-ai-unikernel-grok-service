use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrokResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: i64,
    pub model: String,
    #[serde(rename = "system_fingerprint")]
    pub system_fingerprint: String,
    pub object: String,
    pub usage: Usage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    pub index: i64,
    pub message: Message,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}
