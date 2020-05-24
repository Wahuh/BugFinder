use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};

#[InputObject]
pub struct NewIssue {
    pub title: String,
    pub description: String,
}

#[SimpleObject]
pub struct Issue {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub category: Option<String>,
}

#[SimpleObject]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}
