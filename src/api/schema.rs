use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCompletionResponse {
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
    pub logprobs: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub content: String,
    pub role: String,
    #[serde(rename = "tool_calls")]
    pub tool_calls: Value,
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
    #[serde(rename = "avg_tok_per_sec")]
    pub avg_tok_per_sec: f64,
    #[serde(rename = "avg_prompt_tok_per_sec")]
    pub avg_prompt_tok_per_sec: f64,
    #[serde(rename = "avg_compl_tok_per_sec")]
    pub avg_compl_tok_per_sec: f64,
    #[serde(rename = "total_time_sec")]
    pub total_time_sec: f64,
    #[serde(rename = "total_prompt_time_sec")]
    pub total_prompt_time_sec: f64,
    #[serde(rename = "total_completion_time_sec")]
    pub total_completion_time_sec: f64,
}
