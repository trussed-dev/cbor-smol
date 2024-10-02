#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate delog;
generate_macros!();

pub(crate) mod consts;
pub mod de;
pub mod error;
pub mod ser;

pub use error::{Error, Result};
use ser::Writer;

/// Serialize an object to a `Writer`
///
/// Returns the amount of bytes written to the writer
pub fn cbor_serialize_to<'a, T: ?Sized + serde::Serialize, W: Writer>(
    object: &T,
    writer: W,
) -> Result<usize> {
    let mut serializer = ser::Serializer::new(writer);
    object.serialize(&mut serializer)?;
    Ok(serializer.written())
}

// kudos to postcard, this is much nicer than returning size
pub fn cbor_serialize<'a, T: ?Sized + serde::Serialize>(
    object: &T,
    buffer: &'a mut [u8],
) -> Result<&'a [u8]> {
    let mut buf = &mut *buffer;
    let written = cbor_serialize_to(object, &mut buf)?;
    Ok(&buffer[..written])
}

#[cfg(feature = "heapless-bytes-v0-3")]
/// Append serialization of object to existing bytes, returning length of serialized object.
pub fn cbor_serialize_extending_bytes<T: ?Sized + serde::Serialize, const N: usize>(
    object: &T,
    bytes: &mut heapless_bytes_v0_3::Bytes<N>,
) -> Result<usize> {
    let len_before = bytes.len();
    let mut ser = ser::Serializer::new(bytes);

    object.serialize(&mut ser)?;

    Ok(ser.into_inner().len() - len_before)
}

#[cfg(feature = "heapless-bytes-v0-3")]
/// Serialize object into newly allocated Bytes.
pub fn cbor_serialize_bytes<T: ?Sized + serde::Serialize, const N: usize>(
    object: &T,
) -> Result<heapless_bytes_v0_3::Bytes<N>> {
    let mut data = heapless_bytes_v0_3::Bytes::<N>::new();
    cbor_serialize_extending_bytes(object, &mut data)?;
    Ok(data)
}

pub fn cbor_deserialize<'de, T: serde::Deserialize<'de>>(buffer: &'de [u8]) -> Result<T> {
    // cortex_m_semihosting::hprintln!("deserializing {:?}", buffer).ok();
    de::from_bytes(buffer)
}
