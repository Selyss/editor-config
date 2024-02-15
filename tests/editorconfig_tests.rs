#[cfg(test)]
mod tests {
    use editor_config::parser::EditorConfig;

    #[test]
    fn test_str_parse() {
        let editorconfig_str = r#"
            [*]
            indent_style = tab
            indent_size = 4

            [*.js]
            indent_style = space
        "#;

        let editorconfig: EditorConfig = editorconfig_str.parse().unwrap();

        assert_eq!(editorconfig.config.len(), 2);
        assert_eq!(
            editorconfig.get_property("*", "indent_style"),
            Some(&String::from("tab"))
        );
        assert_eq!(
            editorconfig.get_property("*.js", "indent_style"),
            Some(&String::from("space"))
        );
    }

    #[test]
    fn test_file_parse() {
        let editorconfig_path = "tests/test_data/.editorconfig";
        let editorconfig = EditorConfig::from_file(editorconfig_path).unwrap();

        assert_eq!(editorconfig.config.len(), 2);
        assert_eq!(
            editorconfig.get_property("*", "indent_style"),
            Some(&String::from("tab"))
        );
        assert_eq!(
            editorconfig.get_property("*.js", "indent_style"),
            Some(&String::from("space"))
        );
    }

    #[test]
    fn test_example_file_parse() {
        let editorconfig_path = "tests/test_data/.editorconfig_example";
        let editorconfig = EditorConfig::from_file(editorconfig_path).unwrap();

        assert_eq!(editorconfig.config.len(), 6);
        assert_eq!(
            editorconfig.get_property("*", "end_of_line"),
            Some(&String::from("lf"))
        );
        assert_eq!(
            editorconfig.get_property("*", "insert_final_newline"),
            Some(&String::from("true"))
        );
        assert_eq!(
            editorconfig.get_property("*.{js,py}", "charset"),
            Some(&String::from("utf-8"))
        );
    }
}
