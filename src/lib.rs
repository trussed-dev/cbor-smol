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
pub fn cbor_serialize_to<T: ?Sized + serde::Serialize, W: Writer>(
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

pub fn cbor_deserialize<'de, T: serde::Deserialize<'de>>(buffer: &'de [u8]) -> Result<T> {
    // cortex_m_semihosting::hprintln!("deserializing {:?}", buffer).ok();
    de::from_bytes(buffer)
}
