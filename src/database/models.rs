
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::database::mongox::MongoRepo;
pub struct Data {
pub database : MongoRepo
} 
// User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
#[derive(Debug, Serialize, Deserialize)]
pub struct RGB{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextwithICon{
    #[serde( skip_serializing_if = "Option::is_none")]
    pub text:Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub icon:Option<String>
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Embed{
    #[serde( skip_serializing_if = "Option::is_none")]
    pub title:Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub color:Option<RGB>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub description:Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub image:Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub thumbnail:Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub footer:Option<TextwithICon>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub auth:Option<TextwithICon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinLeave{
    pub id:u64,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub format:Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub image:Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub embed: Option<Embed>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub image_editting:Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GuildData{
    #[serde( skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub join: Option<JoinLeave>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub leave: Option<JoinLeave>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub music_dash: Option<u64>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub vc :Option<u64>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub choose: Option<HashMap<String, String>>,
}

