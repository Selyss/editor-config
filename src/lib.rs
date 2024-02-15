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
//! This crate only parses and returns the values, it does not impose any implementation details on the editor
//! 
//! The file must be provided to the parser, it does not search the filesystem.

pub mod parser;
