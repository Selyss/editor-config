// takes in a file and deserializes like a toml file
#[derive(Debug)]
pub struct EditorConfig {
    indent_style: Option<String>,
    indent_size: Option<String>, // string because of a "tab" option
    tab_width: Option<u32>,
    end_of_line: Option<String>,
    charset: Option<String>,
    trim_trailing_whitespace: Option<bool>,
    insert_final_newline: Option<bool>,
    root: Option<bool>,
}

impl EditorConfig {}
