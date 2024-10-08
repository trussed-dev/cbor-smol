searchState.loadedDescShard("cbor_smol", 0, "Returns a mutable slice view.\nReturns an immutable slice view.\nSerialize object into newly allocated Bytes.\nAppend serialization of object to existing bytes, …\nWrap existing bytes in a <code>Bytes&lt;N&gt;</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nUnwraps the Vec&lt;u8, N&gt;, same as <code>into_vec</code>.\nUnwraps the Vec&lt;u8, N&gt;, same as <code>into_inner</code>.\nConstruct a new, empty <code>Bytes&lt;N&gt;</code>.\nFallible conversion into differently sized byte buffer.\nLow-noise conversion between lengths.\nSome APIs offer an interface of the form …\nA structure for deserializing a cbor-smol message.\nReturns the argument unchanged.\nDeserialize a message of type <code>T</code> from a byte slice. The …\nObtain a Deserializer from a slice of bytes\nCalls <code>U::from(self)</code>.\nDeserialize a message of type <code>T</code> from a byte slice. The …\nFound a bool that wasn’t 0xf4 or 0xf5\nCould not parse an enum\nExpected a i16, was too large\nExpected a i32, was too large\nExpected a i64, was too large\nExpected a i8, was too large\nExpected a different major type\nExpected a u16\nExpected a u32\nExpected a u64\nExpected a u8\nTried to parse invalid utf-8\nExpected a NULL marker\nValue may be valid, but not encoded in minimal way\nHit the end of buffer, expected more data\nContains the error value\nThis is the error type used by cbor-smol\nInexistent slice-to-array cast error. Used here to avoid …\nThis is a feature that cbor-smol intends to support, but …\nContains the success value\nThis is the Result type used by cbor-smol.\nSerde Deserialization Error\nSerde Missing required value\nSerde Serialization Error\nThe serialize buffer is full\nThis is a feature that cbor-smol will never implement\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThe type of error returned when a write operation fails.\nReturns the number of bytes written to the underlying …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nUnwrap the <code>Writer</code> from the <code>Serializer</code>.\nReturns the underlying slice.\nWraps a mutable slice so it can be used as a <code>Writer</code>.\nAttempts to write an entire buffer into this write.")