mod errors;
mod tests;
use anyhow::{Context, Error, Ok};
use std::{
    fmt::{self, format}, fs::File, io::{BufRead, BufReader, Cursor, Read}, 
};
use crate::errors::{Errors, bail};

const O_BRAC: u8 = b'{';
const C_BRAC: u8 = b'}';
const O_ARR: u8 = b'[';
const C_ARR: u8 = b']';
const QOUTE: u8 = b'"';
const COLUMN: u8 = b':';
const COMMA: u8 = b',';

#[derive(Debug)]
enum Things{
    Key(String),
    Value(String),
}

fn match_char(expected: u8, c: u8) -> anyhow::Result<()>{
    if c != expected {
        bail(Errors::CorruptFile)?
    }
    Ok(())
}

fn trim_spaces(buffer : &mut Vec<u8>){
    buffer.retain(|x| *x != b' ' );
}
fn read_column(reader: &mut BufReader<File>)->anyhow::Result<()>{
    let mut buf = Vec::new();
    reader.read_until(COLUMN , &mut buf)?;
    trim_spaces(&mut buf);
    if buf[0] != b':'{
        bail(Errors::CorruptFile)?
    }
    Ok(())
}
fn read_key(reader: &mut BufReader<File>) -> anyhow::Result<String> {
    let mut buf = Vec::new();
    reader.read_until(QOUTE, &mut buf)?;
    trim_spaces(&mut buf);
    match_char(QOUTE, buf[0])?;

    buf.clear();

    reader.read_until(QOUTE, &mut buf)?;
    read_column(reader)?;
    buf.remove(buf.len()-1);
    Ok(String::from_utf8_lossy(&buf).to_string())

}

fn main() -> anyhow::Result<()> {
    let f = File::open("examples/oomlanda.txt").context("opening file")?;
    let mut reader = BufReader::new(f);
    let mut result :Vec<Things>= Vec::new();
    let key = read_key(&mut reader)?;
    result.push(Things::Key(key));



    print!("things :: {:?}" , result);
    Ok(())
}
