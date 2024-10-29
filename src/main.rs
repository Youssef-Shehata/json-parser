use std::{
    fmt::{self},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Ok};

static mut STACK: Vec<Tokens> = <Vec<Tokens>>::new();

enum Errors {
    FileCorrupt,
    UnbalancedBrackets,
}
fn bail<T>(e: Errors) -> anyhow::Result<T> {
    anyhow::bail!("{}", e)
}
impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Errors::FileCorrupt => write!(f, " Json File Corrupted"),
            Errors::UnbalancedBrackets => write!(f, " Unbalanced brackets in file"),
        }
    }
}
#[derive(PartialEq, Clone)]
enum Tokens {
    OBracket,
    CBracket,
    OArr,
    CArr,
    Comma,
}
struct Pair {
    key: String,
    value: Value,
}
struct Object {
    data: Vec<Pair>,
}
enum Value {
    Obj(Object),
    Array(Vec<Value>),
    Number(f64),
    Text(String),
}

#[derive(PartialEq)]
enum State {
    Start,
    Token(Tokens),
    Object,
    Success,
}
impl State {
    fn next(&self, reader: &mut BufReader<File>, buf: &mut Vec<u8>) -> anyhow::Result<State> {
        match self {
            State::Start => {
                let n = reader.read_until(b'{', buf).context("reading file")?;
                if n > 1 && String::from_utf8_lossy(&buf).trim_start() != "{" {
                    bail(Errors::FileCorrupt)?
                }
                Ok(Self::Object)
            }

            State::Token(t) => {
                unsafe {
                    STACK.push(t.clone());
                };
                match t {
                    Tokens::OBracket => Ok(State::Object),
                    Tokens::CBracket => unsafe {
                        let p = STACK.pop();
                        if p.is_none() || p.is_some() && p.unwrap() != Tokens::OBracket {
                            bail(Errors::UnbalancedBrackets)?
                        }
                        Ok(State::Token(Tokens::Comma))
                    },
                    Tokens::OArr => Ok(State::Object),
                    Tokens::CArr => unsafe {
                        let p = STACK.pop();
                        if p.is_none() || p.is_some() && p.unwrap() != Tokens::OArr {
                            bail(Errors::UnbalancedBrackets)?
                        }
                        Ok(State::Token(Tokens::Comma))
                    },
                    //keep read the next line and decide wether its a key or end of object 
                    Tokens::Comma => todo!(),
                }
            }
            //read the key , value and store them in the global struct , idk how yet 
            State::Object => todo!(),

            State::Success => Ok(State::Success),
        }
    }
}


fn main() -> anyhow::Result<()> {
    let f = File::open("examples/oomlanda.txt").context("opening file")?;

    let mut reader = BufReader::new(f);
    let mut s = State::Start;
    let mut buf = Vec::new();
    while s != State::Success{
        s = s.next(&mut reader, &mut buf)?;
    }

    unsafe {
        if !STACK.is_empty() {
            anyhow::bail!("{}", Errors::UnbalancedBrackets);
        }
    }
    Ok(())
}

