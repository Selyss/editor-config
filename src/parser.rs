use ini::Ini;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

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

// NOTE: invalid properties should never error.
// https://github.com/editorconfig/editorconfig/wiki/How-to-create-an-EditorConfig-plugin#handling-unsupported-editorconfig-properties
fn validate_property<'a>(property: &str, value: &'a String) -> Option<&'a String> {
    match property {
        "indent_style" => match value.as_str() {
            "tab" | "space" => Some(value),
            _ => None,
        },
        "indent_size" => match value.as_str() {
            "tab" => Some(value),
            _ => value.parse::<u32>().ok().map(|_| value),
        },
        "tab_width" => value.parse::<u32>().ok().map(|_| value),
        "end_of_line" => match value.as_str() {
            "lf" | "cr" | "crlf" => Some(value),
            _ => None,
        },
        "charset" => match value.as_str() {
            "latin1" | "utf-8" | "utf-8-bom" | "utf-16be" | "utf-16le" => Some(value),
            _ => None,
        },
        "trim_trailing_whitespace" | "insert_final_newline" | "root" => {
            value.parse::<bool>().ok().map(|_| value)
        }
        "max_line_length" => match value.as_str() {
            "off" => Some(value),
            _ => value.parse::<u32>().ok().map(|_| value),
        },
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
