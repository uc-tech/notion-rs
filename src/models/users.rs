use crate::ids::UserId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserCommon {
    pub id: UserId,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Person {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Bot {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged, rename_all = "snake_case")]
pub enum User {
    Person(PersonUser),
    Bot(BotUser),
    Unknown(UnknownUser),
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct UnknownUser {
    #[serde(flatten)]
    pub common: UserCommon,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct PersonUser {
    #[serde(flatten)]
    pub common: UserCommon,
    pub person: Person,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct BotUser {
    #[serde(flatten)]
    pub common: UserCommon,
    pub bot: Bot,
}
