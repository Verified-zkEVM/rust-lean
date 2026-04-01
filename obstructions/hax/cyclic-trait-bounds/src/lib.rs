//! Minimal reproducer: two traits with cyclic associated type bounds.
//! Hax generates invalid Lean where declaration order matters.
//!
//! Upstream: cryspen/hax#1773, cryspen/hax#1886
//!
//! Pattern source: Plonky3's PrimeCharacteristicRing / PrimeField
//! mutual dependency, and more generally any codec/serialization
//! trait pair where Encoder::Decoded: Decoder and vice versa.

pub trait Encoder: Sized {
    type Decoded: Decoder<Encoded = Self>;
    fn encode(input: &Self::Decoded) -> Self;
}

pub trait Decoder: Sized {
    type Encoded: Encoder<Decoded = Self>;
    fn decode(input: &Self::Encoded) -> Self;
}

pub struct Bytes(pub Vec<u8>);
pub struct Message(pub String);

impl Encoder for Bytes {
    type Decoded = Message;
    fn encode(input: &Message) -> Self {
        Bytes(input.0.as_bytes().to_vec())
    }
}

impl Decoder for Message {
    type Encoded = Bytes;
    fn decode(input: &Bytes) -> Self {
        Message(String::from_utf8_lossy(&input.0).into_owned())
    }
}

pub fn round_trip(msg: &Message) -> Message {
    let encoded = Bytes::encode(msg);
    Message::decode(&encoded)
}
