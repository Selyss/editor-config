# editor-config

An editorconfig parsing library.

This crate serves as a thin wrapper over the editorconfig specification.

# Example

```rust
let editorconfig_path = "tests/test_data/.editorconfig";

let editorconfig = EditorConfig::from_file(editorconfig_path).unwrap();

if let Some(indent_style) = editorconfig.get_property("*", "end_of_line") {
	println!("EOL: {}", indent_style);
} else {
	println!("End of line not specified.");
}
```

This crate only parses keys and returns the values, it does not impose any implementation details on the editor.

The editorconfig file must be provided to the parser, it does not search the file system.
