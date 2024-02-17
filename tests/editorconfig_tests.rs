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
    // parse the default editorconfig from the editorconfig website
    fn test_file_parse() {
        let editorconfig_path = "tests/test_data/.editorconfig";
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
        assert_eq!(
            editorconfig.get_property("*.py", "indent_style"),
            Some(&String::from("space"))
        );
        assert_eq!(
            editorconfig.get_property("*.py", "indent_size"),
            Some(&String::from("4"))
        );
        assert_eq!(
            editorconfig.get_property("Makefile", "indent_style"),
            Some(&String::from("tab"))
        );
        assert_eq!(
            editorconfig.get_property("lib/**.js", "indent_style"),
            Some(&String::from("space"))
        );
        assert_eq!(
            editorconfig.get_property("lib/**.js", "indent_size"),
            Some(&String::from("2"))
        );
        assert_eq!(
            editorconfig.get_property("{package.json,.travis.yml}", "indent_style"),
            Some(&String::from("space"))
        );
        assert_eq!(
            editorconfig.get_property("{package.json,.travis.yml}", "indent_size"),
            Some(&String::from("2"))
        );
    }
    // INFO: invalid keys are implicitly none, since the user would need to request an invalid key
    // to get an invalid key
    #[test]
    fn test_invalid_value() {
        let editorconfig_str = r#"
            [*]
            indent_size = big
        "#;

        let editorconfig: EditorConfig = editorconfig_str.parse().unwrap();
        assert_eq!(editorconfig.get_property("*", "indent_size"), None);
    }
}
