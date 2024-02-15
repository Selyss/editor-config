use ini::Ini;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

pub struct EditorConfig {
    config: HashMap<String, HashMap<String, String>>,
}

impl EditorConfig {
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let contents = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
        EditorConfig::from_str(&contents)
    }
}

impl FromStr for EditorConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = Ini::load_from_str(s).map_err(|e| e.to_string())?;
        let config = parsed
            .iter()
            .map(|(section, properties)| {
                let section_name = section.expect("Error: cannot parse section").to_string();
                let section_props = properties
                    .iter()
                    .map(|(key, value)| (key.to_string(), value.to_string()))
                    .collect();
                (section_name, section_props)
            })
            .collect();

        Ok(EditorConfig { config })
    }
}

// // takes in a file and deserializes like a toml file
// #[derive(Debug)]
// pub struct EditorConfig {
//     indent_style: Option<String>,
//     indent_size: Option<String>, // string because of a "tab" option
//     tab_width: Option<u32>,
//     end_of_line: Option<String>,
//     charset: Option<String>,
//     trim_trailing_whitespace: Option<bool>,
//     insert_final_newline: Option<bool>,
//     root: Option<bool>,
// }
//
// impl EditorConfig {}
