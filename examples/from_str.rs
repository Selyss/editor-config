//! Example of getting an editorconfig from a string

use editor_config::parser::EditorConfig;

fn main() {
    let editorconfig_str = r#"
        [*]
        indent_style = tab
        indent_size = 4

        [*.js]
        indent_style = space
    "#;

    let editorconfig: EditorConfig = editorconfig_str.parse().unwrap();

    if let Some(indent_style) = editorconfig.get_property("*.js", "indent_style") {
        println!("Indent Style: {}", indent_style);
    } else {
        println!("Indent style not specified.");
    }
}
