use std::{
    fmt::{self, Display},
    fs::File,
    io::{stderr, BufRead, BufReader, Read},
};

use anyhow::{anyhow, bail, Context};

enum Errors {
    ERROR_CORRUPT,
    UNBALANCED_BRACKETS,
}
impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Errors::ERROR_CORRUPT => write!(f, " Json File Corrupted"),
            Errors::UNBALANCED_BRACKETS => write!(f, " Unbalanced brackets in file"),
        }
    }
}
#[derive(PartialEq)]
enum Tokens {
    OBracket,
    CBracket,
    OArr,
    CArr,
}
struct Object {
    key: String,
    value: Box<Value>,
}
enum Value {
    Obj(Object),
    Array(Vec<Value>),
    Val(f64),
    Text(String),
}

struct JParser {
    data: Object,
}
trait Parse {
    fn parse(&self) -> Object;
}
impl Parse for String {
    fn parse(&self) -> Object {
        todo!();
    }
}

impl Parse for File {
    fn parse(&self) -> Object {
        todo!();
        let val = Value::Val(9.0);
        return Object {
            key: String::new(),
            value: Box::new(val),
        };
    }
}

fn main() -> anyhow::Result<()> {
    let f = File::open("src/examples/oomlanda.txt").context("opening file")?;

    let mut buf = Vec::new();
    let mut stack = Vec::new();
    let mut reader = BufReader::new(f);
    let n = reader.read_until(b'{', &mut buf).context("reading file")?;
    if n > 1 && String::from_utf8_lossy(&buf).trim_start() != "{" {
        anyhow::bail!("{}", Errors::ERROR_CORRUPT);
    }
    stack.push(Tokens::OBracket);
    buf.clear();
    reader.read_until(b':', &mut buf).context("reading file")?;

    println!("{}", n);

    if !stack.is_empty() {
        anyhow::bail!("{}", Errors::UNBALANCED_BRACKETS);
    }

    Ok(())
}

fn parse_token(token: Tokens, stack: &mut Vec<Tokens>) -> anyhow::Result<()> {
    match token {
        Tokens::OBracket => stack.push(token),
        Tokens::CBracket => {
            let p = stack.pop();
            if p.is_none() || p.is_some() && p.unwrap() != Tokens::OBracket{
                anyhow::bail!("{}", Errors::UNBALANCED_BRACKETS);
            }
        },
        Tokens::OArr => stack.push(token),
        Tokens::CArr => {
            let p = stack.pop();
            if p.is_none() || p.is_some() && p.unwrap() != Tokens::OArr {
                anyhow::bail!("{}", Errors::UNBALANCED_BRACKETS);
            }
        }
    }
    Ok(())
}
