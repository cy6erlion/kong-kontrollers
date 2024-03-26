//! # Blog kontroller
//!
//! The `blog` __kontroller__ allows an admin account (from the `account`
//! kontroller) to post posts, delete, edit blog posts. The kontroller
//!  allows the blog posts to be read publicly by anyone.
//!
//! > To use the blog kontroller, enable it with the `blog` Cargo
//! > feature.
//!
//! The `blog` kontroller depends on the `login` kontroller,
//! enabling the `blog` kontroller automatically enables the
//! `login` kontroller.

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

pub use inputs::CreateBlogInput;

pub mod create;
pub mod database;
pub mod delete;
pub mod get;
pub mod get_all;
pub mod inputs;
pub mod update;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct DatabaseBlogPostInput {
    /// The title of the blog post
    pub title: String,
    /// The subtitle of the blog post
    pub subtitle: Option<String>,
    /// The overview of the blog post
    pub overview: Option<String>,
    /// The username of the blog author
    pub author: Option<String>,
    /// The path to the cover image of the blog post
    pub cover: Option<String>,
    /// The actual content of the blog post
    pub content: String,
    /// The date when the blog post was published
    pub date: Option<DateTime<Utc>>,
}

impl From<CreateBlogInput> for DatabaseBlogPostInput {
    fn from(value: CreateBlogInput) -> Self {
        DatabaseBlogPostInput {
            title: value.title,
            subtitle: value.subtitle,
            overview: value.overview,
            author: value.author,
            cover: value.cover,
            content: value.content,
            // TODO: don't hardcode this, allow to use date from
            // user input
            date: Some(Utc::now()),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct BlogPost {
    /// The Identifier of the blog post, the Rust Type is `i64`
    pub id: i64,
    /// The title of the blog post
    pub title: String,
    /// The subtitle of the blog post
    pub subtitle: Option<String>,
    /// The overview of the blog post
    pub overview: Option<String>,
    /// The username of the blog author
    pub author: Option<String>,
    /// The path to the cover image of the blog post
    pub cover: Option<String>,
    /// The actual content of the blog post
    pub content: String,
    /// The date when the blog post was published
    pub date: Option<DateTime<Utc>>,
}
