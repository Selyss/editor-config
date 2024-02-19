//! An [editorconfig](https://editorconfig.org/) parsing library.
//!
//! This crate serves as a thin wrapper over the editorconfig specification.
//!
//! ## Example
//!
//! ```toml
//! [*]
//! end_of_line = lf
//! ```
//!
//! Retrieving a property value can be done by providing a section name and property name.
//!
//! ```rust
//! use editor_config::parser::EditorConfig;
//!
//!
//! let editorconfig_path = "tests/test_data/.editorconfig";
//!
//! let editorconfig = EditorConfig::from_file(editorconfig_path).unwrap();
//!
//! assert_eq!(editorconfig.get_property("*", "end_of_line"), Some(&String::from("lf")));
//! ```
//!
//! This crate only parses keys and returns the values, it does not impose any implementation details on the editor.
//!
//! The editorconfig file must be provided to the parser, it does not search the file system.
//!
//! Unknown/invalid properties or property values are ignored as per the core library specification. If the parser encounters one, it
//! will return `None`.
//!
//! If a property value is returned, it has been checked and narrowed to confirm it is valid.

pub mod parser;
