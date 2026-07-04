//! Uci
//!
#![doc = r#""#]

#![allow(dead_code)]
#[macro_use]
pub mod serde_utils;
pub mod choices;
pub mod common;
pub mod elements;
pub mod enums;
pub mod traits;
pub mod types;

pub const SCHEMA_VERSION: &'static str = r#"005.0a.ASK-20260423-f1380e7"#;

