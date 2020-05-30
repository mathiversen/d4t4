//! [![github]](https://github.com/mathiversen/d4t4)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//!
//! # D4t4
//!
//! **WIP - work in progress, use at your own risk**
//!
//! A JSON superset
//!
//! ## Features
//! - JSON-compatible
//! - Objects & arrays may have trailing comma
//! - Object keys may not need quotes
//! - Single & multiline comments are allowed
//! - Strings may use double and/or single quotes
//! - Values can be referenced from other parts of the object tree
//!
//! ## Example
//! ```rust
//!     use d4t4::parse;
//!
//!     fn main() {
//!         let data = r#"{
//!            /*
//!                Look how nice, we can separate re-useable data!
//!            */
//!            padding: {
//!                s: '1px',
//!                m: '2px',
//!                l: '3px', /* note: triling comma */
//!            },
//!            color: {
//!                red: '#fed7d7',
//!                green: '#c6f6d5',
//!                blue: '#bee3f8',
//!            },
//!            objects: [
//!                {
//!                    border: "1px solid &{color.red}",
//!                    padding: "&{padding.s}",
//!                },
//!                {
//!                    border: "1px solid &{color.green}",
//!                    padding: "&{padding.m}",
//!                },
//!                {
//!                    border: "1px solid &{color.blue}",
//!                    padding: "&{padding.l}",
//!                },
//!            ]
//!         }"#;
//!
//!         let json = parse(data).ok().unwrap();
//!         assert_eq!(json["objects"][0]["padding"], "1px");
//!     }
//! ```
//! ## Contributions
//! I would love to get some feedback if you find my little project useful. Please feel free to highlight issues with my code or submit a PR in case you want to improve it.

#![allow(clippy::needless_doctest_main)]

mod error;
mod parser;
mod tokenizer;

pub use crate::error::Error;
pub use crate::parser::parse;
pub use anyhow::Result;
pub use serde_json::Value;
