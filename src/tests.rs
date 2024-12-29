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
                println!("{:?}", json_value);

                assert_eq!(json_value.to_string(), "{\"name\":\"youssef\",\"age\":\"22\"}");
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
