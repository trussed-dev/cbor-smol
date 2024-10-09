use super::error::{Error, Result};
use serde::ser;
use serde::Serialize;

use core::mem;

use crate::consts::*;

pub trait Writer {
    /// The type of error returned when a write operation fails.
    type Error: Into<Error>;

    /// Attempts to write an entire buffer into this write.
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

impl<'a> Writer for &'a mut [u8] {
    type Error = Error;
    fn write_all<'b>(&'b mut self, buf: &[u8]) -> Result<()> {
        let l = buf.len();
        if self.len() < l {
            // This buffer will not fit in our slice
            return Err(Error::SerializeBufferFull(0));
        }
        let (current, rem) = mem::take(self).split_at_mut(l);
        current.copy_from_slice(buf);
        *self = rem;
        Ok(())
    }
}

#[cfg(feature = "heapless-bytes-v0-3")]
impl<const N: usize> Writer for heapless_bytes_v0_3::Bytes<N> {
    type Error = Error;
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.extend_from_slice(buf)
            .or(Err(Error::SerializeBufferFull(self.len())))
    }
}

#[cfg(feature = "heapless-bytes-v0-4")]
impl<const N: usize> Writer for heapless_bytes_v0_4::Bytes<N> {
    type Error = Error;
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.extend_from_slice(buf)
            .or(Err(Error::SerializeBufferFull(self.len())))
    }
}

#[cfg(feature = "heapless-v0-7")]
impl<const N: usize> Writer for heapless_v0_7::Vec<u8, N> {
    type Error = Error;
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.extend_from_slice(buf)
            .or(Err(Error::SerializeBufferFull(self.len())))
    }
}

#[cfg(feature = "heapless-v0-8")]
impl<const N: usize> Writer for heapless_v0_8::Vec<u8, N> {
    type Error = Error;
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.extend_from_slice(buf)
            .or(Err(Error::SerializeBufferFull(self.len())))
    }
}

impl<'a, T: Writer> Writer for &'a mut T {
    type Error = T::Error;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        (**self).write_all(buf)
    }
}

struct WrittenWriter<W> {
    writer: W,
    written: usize,
}

impl<W: Writer> Writer for WrittenWriter<W> {
    type Error = W::Error;

    fn write_all(&mut self, buf: &[u8]) -> core::result::Result<(), Self::Error> {
        self.written += buf.len();
        self.writer.write_all(buf)
    }
}

pub struct Serializer<W> {
    inner: WrittenWriter<W>,
}

impl<W: Writer> Serializer<W> {
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer {
            inner: WrittenWriter { writer, written: 0 },
        }
    }

    pub fn written(&self) -> usize {
        self.inner.written
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.inner.writer
    }

    #[inline]
    fn write_u8(&mut self, major: u8, value: u8) -> Result<()> {
        if value <= 0x17 {
            self.inner.write_all(&[major << MAJOR_OFFSET | value])
        } else {
            let buf = [major << MAJOR_OFFSET | 24, value];
            self.inner.write_all(&buf)
        }
        .map_err(|e| e.into())
    }

    #[inline]
    fn write_u16(&mut self, major: u8, value: u16) -> Result<()> {
        if value <= u16::from(u8::max_value()) {
            self.write_u8(major, value as u8)
        } else {
            let mut buf = [major << MAJOR_OFFSET | 25, 0, 0];
            buf[1..].copy_from_slice(&value.to_be_bytes());
            self.inner.write_all(&buf).map_err(|e| e.into())
        }
    }

    #[inline]
    fn write_u32(&mut self, major: u8, value: u32) -> Result<()> {
        if value <= u32::from(u16::max_value()) {
            self.write_u16(major, value as u16)
        } else {
            let mut buf = [major << MAJOR_OFFSET | 26, 0, 0, 0, 0];
            buf[1..].copy_from_slice(&value.to_be_bytes());
            self.inner.write_all(&buf).map_err(|e| e.into())
        }
    }

    #[inline]
    fn write_u64(&mut self, major: u8, value: u64) -> Result<()> {
        if value <= u64::from(u32::max_value()) {
            self.write_u32(major, value as u32)
        } else {
            let mut buf = [major << MAJOR_OFFSET | 27, 0, 0, 0, 0, 0, 0, 0, 0];
            buf[1..].copy_from_slice(&value.to_be_bytes());
            self.inner.write_all(&buf).map_err(|e| e.into())
        }
    }

    #[inline]
    fn serialize_collection(
        &mut self,
        major: u8,
        len: Option<usize>,
    ) -> Result<CollectionSerializer<'_, W>> {
        let needs_eof = match len {
            Some(len) => {
                self.write_u64(major, len as u64)?;
                false
            }
            None => {
                self.inner
                    .write_all(&[major << MAJOR_OFFSET | 31])
                    .map_err(|e| e.into())?;
                true
            }
        };

        Ok(CollectionSerializer {
            ser: self,
            needs_eof,
        })
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Writer,
{
    type Ok = ();

    type Error = Error;

    type SerializeSeq = CollectionSerializer<'a, W>;
    type SerializeTuple = &'a mut Serializer<W>;
    type SerializeTupleStruct = &'a mut Serializer<W>;
    type SerializeTupleVariant = &'a mut Serializer<W>;
    type SerializeMap = CollectionSerializer<'a, W>;
    type SerializeStruct = &'a mut Serializer<W>;
    type SerializeStructVariant = &'a mut Serializer<W>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        let value = if value { VALUE_TRUE } else { VALUE_FALSE };
        self.inner.write_all(&[value]).map_err(|e| e.into())
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        let sign = (value >> 7) as u8;
        let major_type = sign & 0x1;
        let bits = sign ^ (value as u8);
        self.write_u8(major_type, bits)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        let sign = (value >> 15) as u16;
        let major_type = (sign & 0x1) as u8;
        let bits = sign ^ (value as u16);
        self.write_u16(major_type, bits)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        let sign = (value >> 31) as u32;
        let major_type = (sign & 0x1) as u8;
        let bits = sign ^ (value as u32);
        self.write_u32(major_type, bits)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        let sign = (value >> 63) as u64;
        let major_type = (sign & 0x1) as u8;
        let bits = sign ^ (value as u64);
        self.write_u64(major_type, bits)
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<()> {
        self.write_u8(MAJOR_POSINT, value)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<()> {
        self.write_u16(MAJOR_POSINT, value)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<()> {
        self.write_u32(MAJOR_POSINT, value)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<()> {
        self.write_u64(MAJOR_POSINT, value)
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        todo!("serialize_f32 not implemented");
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        todo!("serialize_f64 not implemented");
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<()> {
        // A char encoded as UTF-8 takes 4 bytes at most.
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        self.write_u64(MAJOR_STR, value.len() as u64)?;
        self.inner.write_all(value.as_bytes()).map_err(|e| e.into())
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        self.write_u64(MAJOR_BYTES, value.len() as u64)?;
        self.inner.write_all(value).map_err(|e| e.into())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.inner.write_all(&[VALUE_NULL]).map_err(|e| e.into())
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        self.serialize_none()
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        // if self.packed {
        self.serialize_u32(variant_index)
        // } else {
        //     self.serialize_str(variant)
        // }
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        // if name == CBOR_NEWTYPE_NAME {
        //     for tag in get_tag().into_iter() {
        //         self.write_u64(6, tag)?;
        //     }
        // }
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        // if self.enum_as_map {
        //     self.write_u64(5, 1u64)?;
        //     variant.serialize(&mut *self)?;
        // } else {
        self.write_u64(MAJOR_ARRAY, 2)?;
        self.serialize_unit_variant(name, variant_index, variant)?;
        // }
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<CollectionSerializer<'a, W>> {
        self.serialize_collection(MAJOR_ARRAY, len)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<&'a mut Serializer<W>> {
        self.write_u64(MAJOR_ARRAY, len as u64)?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<&'a mut Serializer<W>> {
        self.serialize_tuple(len)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<&'a mut Serializer<W>> {
        // if self.enum_as_map {
        //     self.write_u64(5, 1u64)?;
        //     variant.serialize(&mut *self)?;
        //     self.serialize_tuple(len)
        // } else {
        self.write_u64(MAJOR_ARRAY, (len + 1) as u64)?;
        self.serialize_unit_variant(name, variant_index, variant)?;
        Ok(self)
        // }
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<CollectionSerializer<'a, W>> {
        self.serialize_collection(MAJOR_MAP, len)
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.write_u64(MAJOR_MAP, len as u64)?;
        Ok(self)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        // if self.enum_as_map {
        //     self.write_u64(5, 1u64)?;
        // } else {
        self.write_u64(MAJOR_ARRAY, 2)?;
        // }
        self.serialize_unit_variant(name, variant_index, variant)?;
        self.serialize_struct(name, len)
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: core::fmt::Display,
    {
        unreachable!()
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: Writer,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: Writer,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: Writer,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: Writer,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: Writer,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[doc(hidden)]
pub struct CollectionSerializer<'a, W> {
    ser: &'a mut Serializer<W>,
    needs_eof: bool,
}

impl<'a, W> CollectionSerializer<'a, W>
where
    W: Writer,
{
    #[inline]
    fn end_inner(self) -> Result<()> {
        if self.needs_eof {
            self.ser.inner.write_all(&[0xff]).map_err(|e| e.into())
        } else {
            Ok(())
        }
    }
}

impl<'a, W> ser::SerializeSeq for CollectionSerializer<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.end_inner()
    }
}

impl<'a, W> ser::SerializeMap for CollectionSerializer<'a, W>
where
    W: Writer,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(&mut *self.ser)
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.end_inner()
    }
}
