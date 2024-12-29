#[cfg(test)]
pub mod exact {
    use crate::parse_json;

    #[test]
    fn simple_obj() {
        let json_str = r#"
        {
            "name": "youssef",
            "age": "22"
        }
        "#;

        match parse_json(json_str) {
            Ok(json_value) => {
                assert_eq!(
                    json_value.to_string(),
                    "{\"name\":\"youssef\",\"age\":\"22\"}"
                );
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }


    #[test]
    fn number_values() {
        let json_str = r#"
        {
            "age": 22,
            "height": 1.8,
            "negative": -452
        }
        "#;

        match parse_json(json_str) {
            Ok(json_value) => {
                assert_eq!(
                    json_value.to_string(),
                    "{\"age\":22,\"height\":1.8,\"negative\":-42}"
                );
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[test]
    fn empty_object() {
        let json_str = "{}";

        match parse_json(json_str) {
            Ok(json_value) => {
                assert_eq!(json_value.to_string(), "{}");
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

}

