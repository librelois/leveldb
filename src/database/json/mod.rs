
use super::Database;
use super::options::{ReadOptions,WriteOptions};
use super::error::Error;
use serialize::{json, Encodable};
use serialize::json::Json;
use serialize::json::Encoder;
use serialize::json::from_reader;
use std::io::BufReader;
use std::io;

pub trait Interface<'a, T:Encodable<json::Encoder<'a>,io::IoError>> {
  fn put(&mut self,
         options: WriteOptions,
         key: &T,
         value: &T)
        -> Result<(), Error>;
  fn delete(&mut self,
           options: WriteOptions,
           key: &T) -> Result<(), Error>;
  fn get(&mut self,
         options: ReadOptions,
         key: &T) -> Result<Option<Json>, Error>;
}

impl<'a, T: Encodable<json::Encoder<'a>, io::IoError>> Interface<'a, T> for Database {
  fn put(&mut self,
        options: WriteOptions,
        key: &T,
        value: &T) -> Result<(), Error> {
    let encoded_key = json::Encoder::buffer_encode(&key);
    let encoded_val = json::Encoder::buffer_encode(&value);
    self.put_binary(options, encoded_key.as_slice(), encoded_val.as_slice())
  }
  fn delete(&mut self,
            options: WriteOptions,
            key: &T) -> Result<(), Error> {
    let encoded_key = json::Encoder::buffer_encode(&key);
    self.delete_binary(options, encoded_key.as_slice())
  }
  fn get(&mut self,
         options: ReadOptions,
         key: &T) -> Result<Option<Json>, Error> {
    let encoded_key = json::Encoder::buffer_encode(&key);
    let result = self.get_binary(options, encoded_key.as_slice());
    match result {
      Err(error) => { Err(error) },
      Ok(opt) => {
        match opt {
          None => { Ok(None) },
          Some(binary) => {
            let mut reader = BufReader::new(binary.as_slice());
            match from_reader(&mut reader) {
              Ok(json) => { Ok(Some(json)) },
              Err(_) => { Err( Error::new(from_str("json parsing failed").unwrap()) ) }
            }
          }
        }
      }
    }
  }
}
