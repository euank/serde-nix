use std::fmt;
use std::io;
use std::string::String;

use serde::ser::{self, Impossible, Serialize};
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

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

fn escape(s: &str) -> Result<String> {
    let mut result = String::new();
    result += "\"";
    for c in s.chars() {
        result += &escape_char(&c)?;
    }

    result += "\"";
    Ok(result)
}

// Escape the given string into a nix map key. Omit quoting for alphanumeric keys.
fn escape_map_key(s: &str) -> Result<String> {
    if s.chars().all(char::is_alphanumeric) {
        Ok(s.to_string())
    } else {
        escape(s)
    }
}

fn escape_char(c: &char) -> Result<String> {
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

    fn serialize_bool(self, value: bool) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn serialize_char(self, value: char) -> Result<()> {
        self.serialize_str(&value.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        write!(self.writer, "{}", escape(value)?)?;
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(value.len()))?;
        for byte in value {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_unit(self) -> Result<()> {
        write!(self.writer, "null")?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        write!(self.writer, "null")?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
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
    ) -> Result<()>
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

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(NixExpr::Map { ser: self })
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_str(variant)?;
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        write!(self.writer, "{{ ")?;
        Ok(NixExpr::Map { ser: self })
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
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

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            NixExpr::Map { ref mut ser } => value.serialize(&mut **ser),
            _ => unreachable!(),
        }
    }

    fn end(self) -> Result<()> {
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

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleStruct for NixExpr<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleVariant for NixExpr<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeMap for NixExpr<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            NixExpr::Map { ref mut ser } => key.serialize(MapKeySerializer { ser: *ser }),
            _ => unreachable!(),
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
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

    fn end(self) -> Result<()> {
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

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match *self {
            NixExpr::Map { .. } => ser::SerializeMap::serialize_entry(self, key, value),
            NixExpr::Number { ref mut ser } => value.serialize(&mut **ser),
            NixExpr::RawValue { .. } => unreachable!(),
        }
    }

    fn end(self) -> Result<()> {
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

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
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

    fn end(self) -> Result<()> {
        Ok(())
    }
}

struct MapKeySerializer<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
}

impl<'a, W> ser::Serializer for MapKeySerializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        write!(self.ser.writer, "{}", escape_map_key(value)?)?;
        Ok(())
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.ser.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, _value: bool) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_i8(self, _value: i8) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_i16(self, _value: i16) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_i32(self, _value: i32) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_i64(self, _value: i64) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_u8(self, _value: u8) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_u16(self, _value: u16) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_u32(self, _value: u32) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_u64(self, _value: u64) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_f32(self, _value: f32) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_f64(self, _value: f64) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_char(self, value: char) -> Result<()> {
        self.serialize_str(&value.to_string())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_unit(self) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::MapKeyMustBeAString)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::MapKeyMustBeAString)
    }

    fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + fmt::Display,
    {
        self.ser.collect_str(value)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("nix strings may not contain null bytes")]
    UnencodableNullString,
    #[error("nix map keys must be strings")]
    MapKeyMustBeAString,
    #[error("{0}")]
    Custom(String),
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(format!("{}", msg))
    }
}

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let mut v = Vec::new();
    to_writer(&mut v, value)?;
    Ok(String::from_utf8(v).unwrap())
}
