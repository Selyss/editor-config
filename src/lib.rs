//! An [editorconfig]-parsing library
//!
//! Editorconfig is a standardized configuration file for text editors
//!
//!
//! ```toml
//! [*]
//! indent_style = tab
//! indent_size = 4
//!
//! [*.js]
//! indent_style = space
//!
//! ## Parsing editorconfig
//!

pub mod parser;
