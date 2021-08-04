use std::fmt;
use std::io;
use std::string::String;

use serde::ser::{self, Serialize};
use thiserror::Error;

#[derive(Debug)]
pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    pub fn new(writer: W) -> Self {
        Serializer { writer }
    }
}

fn escape(s: &str) -> Result<String, Error> {
    let mut result = String::new();
    result += "\"";
    for c in s.chars() {
        result += &escape_char(&c)?;
    }

    result += "\"";
    Ok(result)
}

fn escape_char(c: &char) -> Result<String, Error> {
    Ok(match c {
        '\0' => return Err(Error::UnencodableNullString),
        '\n' => "\n".to_string(),
        '\t' => "\t".to_string(),
        '\r' => "\r".to_string(),
        '"' => "\\\"".to_string(),
        '$' => "''$".to_string(),
        c => c.to_string(),
    })
}

impl<'a, W> serde::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = NixExpr<'a, W>;
    type SerializeTuple = NixExpr<'a, W>;
    type SerializeTupleStruct = NixExpr<'a, W>;
    type SerializeTupleVariant = NixExpr<'a, W>;
    type SerializeMap = NixExpr<'a, W>;
    type SerializeStruct = NixExpr<'a, W>;
    type SerializeStructVariant = NixExpr<'a, W>;

    fn serialize_bool(self, value: bool) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<(), Error> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_char(self, value: char) -> Result<(), Error> {
        self.serialize_str(&value.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<(), Error> {
        write!(self.writer, "{}", escape(value)?)?;
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<(), Error> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(value.len()))?;
        for byte in value {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_unit(self) -> Result<(), Error> {
        write!(self.writer, "null")?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> {
        write!(self.writer, "null")?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
    ) -> Result<(), Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        write!(self.writer, "{{ ")?;
        self.serialize_str(variant)?;
        write!(self.writer, " = ")?;
        value.serialize(&mut *self)?;
        write!(self.writer, "; }}")?;
        Ok(())
    }

    fn serialize_none(self) -> Result<(), Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Ok(NixExpr::Map { ser: self })
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        self.serialize_str(variant)?;
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        write!(self.writer, "{{ ")?;
        Ok(NixExpr::Map { ser: self })
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        self.serialize_str(variant)?;
        self.serialize_map(Some(len))
    }
}

pub enum NixExpr<'a, W> {
    Map { ser: &'a mut Serializer<W> },
    Number { ser: &'a mut Serializer<W> },
    RawValue { ser: &'a mut Serializer<W> },
}

impl<'a, W> ser::SerializeSeq for NixExpr<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            NixExpr::Map { ref mut ser } => value.serialize(&mut **ser),
            _ => unreachable!(),
        }
    }

    fn end(self) -> Result<(), Error> {
        match self {
            NixExpr::Map { ser } => {
                write!(ser.writer, "]")?;
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

impl<'a, W> ser::SerializeTuple for NixExpr<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleStruct for NixExpr<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleVariant for NixExpr<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeMap for NixExpr<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            NixExpr::Map { ref mut ser } => key.serialize(&mut **ser),
            _ => unreachable!(),
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            NixExpr::Map { ref mut ser } => {
                write!(ser.writer, " = ")?;
                value.serialize(&mut **ser)?;
                write!(ser.writer, "; ")?;
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn end(self) -> Result<(), Error> {
        match self {
            NixExpr::Map { ser } => {
                write!(ser.writer, "}}")?;
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

impl<'a, W> ser::SerializeStruct for NixExpr<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            NixExpr::Map { .. } => ser::SerializeMap::serialize_entry(self, key, value),
            NixExpr::Number { ref mut ser } => value.serialize(&mut **ser),
            NixExpr::RawValue { .. } => unreachable!(),
        }
    }

    fn end(self) -> Result<(), Error> {
        match self {
            NixExpr::Map { .. } => ser::SerializeMap::end(self),
            _ => Ok(()),
        }
    }
}

impl<'a, W> ser::SerializeStructVariant for NixExpr<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            NixExpr::Map { .. } | NixExpr::Number { .. } => {
                ser::SerializeStruct::serialize_field(self, key, value)
            }
            NixExpr::RawValue { .. } => unreachable!(),
        }
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("nix strings may not contain null bytes")]
    UnencodableNullString,
    #[error("{0}")]
    Custom(String),
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(format!("{}", msg))
    }
}

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<(), Error>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: ?Sized + Serialize,
{
    let mut v = Vec::new();
    to_writer(&mut v, value)?;
    Ok(String::from_utf8(v).unwrap())
}
