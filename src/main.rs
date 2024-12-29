mod tests;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};
#[derive(Debug)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
impl JsonValue {
    fn to_string(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(","))
            }
            JsonValue::Object(obj) => {
                let members: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", members.join(","))
            }
        }
    }
}
fn parse_json(json: &str) -> Result<JsonValue, String> {
    let mut chars = json.chars().peekable();
    parse_value(&mut chars)
}

fn parse_value<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    skip_whitespace(chars);
    match chars.peek() {
        Some('{') => parse_object(chars),
        Some('[') => parse_array(chars),
        Some('"') => parse_string(chars),
        Some('t') | Some('f') => parse_bool(chars),
        Some('n') => parse_null(chars),
        Some(c) if c.is_digit(10) || *c == '-' => parse_number(chars),
        _ => Err("Unexpected character".to_string()),
    }
}
fn skip_whitespace<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) {
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}
fn parse_object<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    let mut obj = HashMap::new();

    chars.next();

    skip_whitespace(chars);

    if chars.peek() == Some(&'}') {
        chars.next();
        return Ok(JsonValue::Object(obj));
    }

    loop {
        skip_whitespace(chars);
        let key = match parse_string(chars)? {
            JsonValue::String(s) => s,
            _ => return Err("Expected string key".to_string()),
        };

        skip_whitespace(chars);

        if chars.next() != Some(':') {
            return Err("Expected ':'".to_string());
        }

        let value = parse_value(chars)?;

        obj.insert(key, value);

        skip_whitespace(chars);

        match chars.next() {
            Some('}') => break,
            Some(',') => continue,
            _ => return Err("Expected ',' or '}'".to_string()),
        }
    }
    Ok(JsonValue::Object(obj))
}

fn parse_array<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    todo!()
}

fn parse_string<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    let mut s = String::new();
    chars.next(); 
    while let Some(c) = chars.next() {
        match c {
            '"' => return Ok(JsonValue::String(s)),
            '\\' => todo!("escaping letters"),
            _ => s.push(c),
        }
    }
    Err("Unterminated string".to_string())
}

fn parse_bool<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    todo!()
}

fn parse_null<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    todo!()
}

fn parse_number<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<JsonValue, String> {
    todo!()
}

fn main() {
    let mut str = String::new();
    let mut f = File::open("examples/oomlanda.json").expect("error opening file");
    let _ = f.read_to_string(&mut str);

    match parse_json(&str) {
        Ok(json_value) => println!("{:?}", json_value),
        Err(e) => eprintln!("Error: {}", e),
    }
}
