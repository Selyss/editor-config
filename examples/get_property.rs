//! An example of getting an editorconfig property

use editor_config::parser::EditorConfig;

fn main() {
    let editorconfig_path = "tests/test_data/.editorconfig";
    let editorconfig = EditorConfig::from_file(editorconfig_path).unwrap();

    if let Some(indent_style) = editorconfig.get_property("*", "end_of_line") {
        println!("EOL: {}", indent_style);
    } else {
        println!("End of line not specified.");
    }
}
