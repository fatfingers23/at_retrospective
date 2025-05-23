// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `dev.baileytownsend.retrospective.day` namespace.
use atrium_api::types::TryFromUnknown;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecordData {
    ///An array of retrospective countables that happened in your day.
    pub countables: Vec<Countable>,
    ///An array of retrospective events that happened in your day.
    pub events: Vec<Event>,
    ///A text summary of your day, 300 character limit so it can be used as a post as well
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub summary: core::option::Option<String>,
}
pub type Record = atrium_api::types::Object<RecordData>;
impl From<atrium_api::types::Unknown> for RecordData {
    fn from(value: atrium_api::types::Unknown) -> Self {
        Self::try_from_unknown(value).unwrap()
    }
}
///Something that happened in your day that can be counted. Steps, water consumed, etc.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CountableData {
    ///The count for the event. This is the number of times the event happened.
    pub count: i64,
    ///a slugable key for the countable item using underscores instead of spaces. ounces_water, cookies_eaten, etc.
    pub key: String,
}
pub type Countable = atrium_api::types::Object<CountableData>;
///Something that happened in your day. Maybe you read a book, told a funny joke, or went on a fun outing. This is a record of that event.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EventData {
    ///What was the event? Watched a movie, what movie? Went to the beach?
    pub description: String,
    ///a slugable key for the event item using underscores instead of spaces. Like movies_watched, books_read, or restaurants_tried.
    pub key: String,
}
pub type Event = atrium_api::types::Object<EventData>;
