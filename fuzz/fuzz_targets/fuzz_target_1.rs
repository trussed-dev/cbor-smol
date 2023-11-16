#![no_main]

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Arbitrary, Serialize, Deserialize)]
enum AllEnums {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Struct(Struct),
    Array([Struct; 4]),
    Option(Option<Struct>),
    Vec(Vec<Struct>),
    Bytes(#[serde(with = "serde_bytes")] Vec<u8>),
    String(String),
    Tuple((Struct, Struct)),
    TupleVariant(Struct, Struct),
    TupleVariantBytes(Struct, Struct, #[serde(with = "serde_bytes")] Vec<u8>),
    StructVariant {
        x: Struct,
        y: Struct,
    },
    StructVariantBytes {
        x: Struct,
        y: Struct,
        #[serde(with = "serde_bytes")]
        z: Vec<u8>,
    },
}

#[derive(Debug, PartialEq, Arbitrary, Serialize, Deserialize)]
struct Struct {
    a: Box<AllEnums>,
    b: Box<AllEnums>,
}

/// Workaround https://github.com/rust-fuzz/arbitrary/issues/144
#[derive(Debug)]
struct Input<'i>(AllEnums, &'i [u8]);

impl<'i> Arbitrary<'i> for Input<'i> {
    fn arbitrary(u: &mut Unstructured<'i>) -> Result<Self, arbitrary::Error> {
        Ok(Self(AllEnums::arbitrary(u)?, Arbitrary::arbitrary(u)?))
    }

    fn arbitrary_take_rest(mut u: Unstructured<'i>) -> Result<Self, arbitrary::Error> {
        Ok(Self(
            AllEnums::arbitrary(&mut u)?,
            Arbitrary::arbitrary_take_rest(u)?,
        ))
    }
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (0, None)
    }
}

fuzz_target!(|data: Input<'_>| {
    let bytes = data.1;
    let data = data.0;
    let _res: Option<AllEnums> = cbor_smol::cbor_deserialize(&bytes).ok();
    let mut buffer = vec![0; 1024 * 20];
    let res = cbor_smol::cbor_serialize(&data, &mut buffer).unwrap();
    cbor_smol::cbor_deserialize(&res)
        .map(|b: AllEnums| {
            assert_eq!(data, b);
        })
        .map_err(|err| {
            let v: Result<serde_cbor::Value, _> = serde_cbor::from_slice(&res);
            panic!(
                "Failed to deserialize: {:?}\n\
            input: {:#?}\n\
            data: {:02x?}\n\
            serde_cbor gives: {:#?}\n",
                err, data, res, v
            );
        })
        .ok();
});
