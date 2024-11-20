mod errors;
mod tests;
use crate::errors::{bail, Errors};
use anyhow::Context;
use core::fmt;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

const O_BRAC: u8 = b'{';
const C_BRAC: u8 = b'}';
const O_ARR: u8 = b'[';
const C_ARR: u8 = b']';
const QOUTE: u8 = b'"';
const COLUMN: u8 = b':';
const COMMA: u8 = b',';

#[derive(Debug)]
struct Object {
    data: HashMap<String, Box<Value>>,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (k, v) in &self.data {
            write!(f, "{} : {}", k, v)?
        }
        Ok(())
    }
}
#[derive(Debug)]
enum Value {
    Text(String),
    Array(Vec<Object>),
    Obj(Object),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Text(s) => {
                write!(f, "{}", s)
            }
            Value::Array(vec) => {
                write!(f, "{:?}", vec)
            }
            Value::Obj(object) => {
                write!(f, "{}", object)
            }
        }
    }
}

struct Reader<T> {
    r: BufReader<T>,
}
impl<T> Reader<T>
where
    T: Read,
{
    fn match_char(&mut self, char: u8) -> anyhow::Result<()> {
        let mut buf = Vec::new();
        self.r.read_until(char, &mut buf).context("match_char")?;
        trim_spaces(&mut buf);
        if buf.len() == 0 {
            bail(Errors::NotFound)?
        }
        if char != buf[0] {
            eprintln!(
                "Expected {:?}  Found {:?} \n",
                char::from(char),
                char::from(buf[0])
            );
            bail(Errors::CorruptFile)?
        }

        Ok(())
    }

    fn read_key(&mut self) -> anyhow::Result<String> {
        let mut buf = Vec::new();
        self.match_char(QOUTE)?;

        self.r.read_until(QOUTE, &mut buf)?;

        self.match_char(COLUMN)?;

        Ok(String::from_utf8_lossy(&buf[..buf.len() - 1]).to_string())
    }

    fn read_value(&mut self) -> anyhow::Result<Value> {
        let mut buf = Vec::new();
        let read = self.next_char()?;
        match read {
            QOUTE => {
                self.r.read_until(QOUTE, &mut buf)?;
                Ok(Value::Text(
                    String::from_utf8_lossy(&buf[..buf.len() - 1]).to_string(),
                ))
            }

            O_BRAC => {
                let obj = self.read_object()?;
                Ok(Value::Obj(obj))
            }

            O_ARR => {
                todo!();
            }
            _ => bail(Errors::CorruptFile)?,
        }

        //self.match_char(COMMA)?;
    }

    fn read_object(&mut self) -> anyhow::Result<Object> {
        let mut j = Object {
            data: HashMap::new(),
        };
        loop {
            let key = self.read_key()?;
            let value = self.read_value()?;
            j.data.insert(key, Box::new(value));

            let read = self.next_char()?;

            match read {
                COMMA => {} //continue
                C_BRAC => {
                    break;
                }
                _ => bail(Errors::CorruptFile)?,
            }
        }
        Ok(j)
    }
    fn next_char(&mut self) -> anyhow::Result<u8> {
        let mut buf = [b' '; 1];
        while buf[0] == b' ' || buf[0] == b'\n' || buf[0] == b'\t' {
            self.r.read_exact(&mut buf).context("next_char")?;
        }
        Ok(buf[0])
    }
}
fn trim_spaces(buffer: &mut Vec<u8>) {
    buffer.retain(|x| *x != b' ' && *x != b'\n' && *x != b'\t');
    buffer.retain(|x| *x != b' ');
}





fn main() -> anyhow::Result<()> {
    let f = File::open("examples/oomlanda.json").context("opening file")?;
    let r = BufReader::new(f);
    let mut reader = Reader { r };

    reader.match_char(O_BRAC)?;
    let j = reader.read_object()?;
    println!("{:?}", j);
    Ok(())
}
