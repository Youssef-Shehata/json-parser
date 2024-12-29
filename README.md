# Json-Parser 

This is a simple library that parses json files or text into usable rust objects, its still in development and not ready for production use.

## Features

- Parses JSON strings into a `JsonValue` enum.
- Supports JSON data types: `null`, `boolean`, `number`, `string`, `array`, and `object`.
- Includes error handling for invalid JSON strings.
- Serializes `JsonValue` enum back into JSON strings.

## Usage

```rust

use your_json_parser_lib::parse_json;

fn main() {
    let json_str = r#"
    {
        "name": "Gojo Saturo",
        "age": 29,
        "alive": "its complicated"
    }
    "#;

    match parse_json(json_str) {
        Ok(json_value) => println!("{:?}", json_value),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
## API

`JsonValue`
The `JsonValue` enum represents a JSON value and can be one of the following variants:

- Null
- Bool(bool)
- Number(f64)
- String(String)
- Array(Vec<JsonValue>)
- Object(HashMap<String, JsonValue>)

`parse_json`
The `parse_json` function takes a JSON string slice and returns a `Result<JsonValue, String>`:

```rust
Copy code
fn parse_json(json: &str) -> Result<JsonValue, String>;
```


## Example

For now to parse a JSON file, you can use the following example code:

```rust
Copy code
use std::fs::File;
use std::io::Read;
use your_json_parser_lib::parse_json;

fn main() {
    let mut file_content = String::new();
    let mut file = File::open("examples/sample.json").expect("Error opening file");
    file.read_to_string(&mut file_content).expect("Error reading file");

    match parse_json(&file_content) {
        Ok(json_value) => println!("{:?}", json_value),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
## Error Handling
The parser will return an Err(String) if it encounters an invalid JSON string. The error message will provide details about the issue.

## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue on GitHub.

## License
This project is licensed under the MIT License.

