use ini::Ini;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Section {
    IndentStyle(String),
    IndentSize(String), // can have value tab
    TabWidth(u32),
    EndOfLine(String),
    Charset(String),
    TrimTrailingWhitespace(bool),
    InsertFinalNewline(bool),
    MaxLineLength(String), // limited support
                           // Root(bool),
}

pub struct EditorConfig {
    pub config: HashMap<String, HashMap<String, String>>,
}

impl EditorConfig {
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let contents: String = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
        EditorConfig::from_str(&contents)
    }
    pub fn get_property(&self, section: &str, property: &str) -> Option<&String> {
        self.config
            .get(section)
            .and_then(|props| props.get(property))
            .and_then(|value| validate_property(property, value))
    }
}

fn validate_property<'a>(property: &str, value: &'a String) -> Option<&'a String> {
    match property {
        "tab_width" => value.parse::<u32>().ok().map(|_| value),
        "trim_trailing_whitespace" | "insert_final_newline" | "root" => {
            value.parse::<bool>().ok().map(|_| value)
        }
        _ => Some(value),
    }
}

impl FromStr for EditorConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = Ini::load_from_str(s).map_err(|e| e.to_string())?;
        let mut config = HashMap::new();
        for (section, properties) in parsed.iter() {
            let section_name = section.as_ref().map_or("*".to_string(), |s| s.to_string());
            let section_props = properties
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect();
            config.insert(section_name, section_props);
        }

        Ok(EditorConfig { config })
    }
}
