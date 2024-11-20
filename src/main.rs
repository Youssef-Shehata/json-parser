mod errors;
mod tests;
use crate::errors::{bail, Errors};
use anyhow::Context;
use std::{
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
struct JsonObject {
    data: Vec<Object>
}

#[derive(Debug)]
struct Object{
    key: String,
    value: String,
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
        self.r.read_until(char, &mut buf)?;
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

    fn read_value(&mut self) -> anyhow::Result<String> {
        let mut buf = Vec::new();
        self.match_char(QOUTE)?;

        self.r.read_until(QOUTE, &mut buf)?;

        //self.match_char(COMMA)?;

        Ok(String::from_utf8_lossy(&buf[..buf.len() - 1]).to_string())
    }

    fn read_object(&mut self) -> anyhow::Result<JsonObject> {
        let mut j = JsonObject{data:Vec::new()};
        self.match_char(O_BRAC)?;
        
        loop {
            let key = self.read_key()?;
            let value = self.read_value()?;

            println!("{:?} : {:?}", key, value);
            j.data.push(Object{key,value});
            println!("");

            let read = self.next_char()?;

            match read {
                C_BRAC => {
                    break;
                }
                COMMA => {}//continue
                _ => bail(Errors::CorruptFile)?,
            }
        }

        Ok(j)
    }
    fn next_char(&mut self) -> anyhow::Result<u8> {
        let mut buf = [b' '; 1];
        while buf[0] == b' ' || buf[0] == b'\n' || buf[0] == b'\t' {
            self.r.read_exact(&mut buf)?;
        }
        Ok(buf[0])
    }
}
fn trim_spaces(buffer: &mut Vec<u8>) {
    buffer.retain(|x| *x != b' ' && *x != b'\n' && *x != b'\t');
    buffer.retain(|x| *x != b' ');
}
fn main() -> anyhow::Result<()> {
    let f = File::open("examples/oomlanda.txt").context("opening file")?;
    let r = BufReader::new(f);
    let mut reader = Reader { r };

    loop {
       let j =  reader.read_object()?;
       println!("OBJECCCCCCCCCCTOOOOO : {:?}" ,j);
    }
    //println!("{:?} : {:?}", k, v);
    Ok(())
}
