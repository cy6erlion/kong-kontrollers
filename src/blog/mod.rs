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

mod database;
mod inputs;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct BlogPost {
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
