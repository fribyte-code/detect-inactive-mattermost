use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;

pub type UsersRoot = Vec<User>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    #[serde(rename = "create_at")]
    pub create_at: i64,
    #[serde(rename = "update_at")]
    pub update_at: i64,
    #[serde(rename = "delete_at")]
    pub delete_at: i64,
    pub username: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    pub nickname: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsRoot {
    pub order: Vec<String>,
    pub posts: HashMap<String, PostProperties>,

    #[serde(rename = "next_post_id")]
    pub next_post_id: String,
    #[serde(rename = "prev_post_id")]
    pub prev_post_id: String,
    #[serde(rename = "has_next")]
    pub has_next: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostProperties {
    pub id: String,
    #[serde(rename = "create_at")]
    pub create_at: i64,
    #[serde(rename = "update_at")]
    pub update_at: i64,
    #[serde(rename = "delete_at")]
    pub delete_at: i64,
    #[serde(rename = "edit_at")]
    pub edit_at: i64,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    #[serde(rename = "root_id")]
    pub root_id: String,
    #[serde(rename = "original_id")]
    pub original_id: String,
    pub message: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub hashtags: String,

    pub metadata: Option<Metadata>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub reactions: Option<Vec<Reaction>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "post_id")]
    pub post_id: String,
    #[serde(rename = "emoji_name")]
    pub emoji_name: String,
    #[serde(rename = "create_at")]
    pub create_at: i64,
}

