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
    #[test]
    fn arrays() {
        let json_str = r#"{"arr":["1","2","3"]}"#;
        let json_str2 = r#"{"arr":[{"1":"111"},{"2":["2","22"}]}"#;
        let json_str3 = r#"{"arr":[false,true]}"#;

        let tests = vec![json_str, json_str2, json_str3];

        for test in tests.iter() {
            match parse_json(test) {
                Ok(json_value) => {
                    assert_eq!(json_value.to_string(), test.trim());
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    #[test]
    fn bools() {
        let json_str = r#"{"bool": true,
                            "bool2":false}"#;

        match parse_json(json_str) {
            Ok(json_value) => {
                assert_eq!(json_value.to_string(), r#"{"bool":true,"bool2":false}"#);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
