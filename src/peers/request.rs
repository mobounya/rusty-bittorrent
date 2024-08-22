use tokio_util::bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct Request {
    index : [u8; 4],
    begin : [u8; 4],
    length : [u8; 4],
}
pub struct RequestDecoder {}
pub struct RequestEncoder {}

impl Request {
    pub fn new(index : u32, begin : u32, length : u32) -> Self {
        Request {
            index: index.to_be_bytes(),
            begin: begin.to_be_bytes(),
            length: length.to_be_bytes()
        }
    }

    pub fn size() -> usize {
        4 * 3
    }
}

impl RequestDecoder {
    pub fn new() -> Self {
        RequestDecoder {}
    }
}

impl RequestEncoder {
    pub fn new() -> Self {
        RequestEncoder {}
    }
}

impl Decoder for RequestDecoder {
    type Item = Request;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // index, begin, and length are each a 32-bit integer.
        if src.len() < 4 * 3 {
            return Ok(None);
        }
        let mut index_bytes = [0u8; 4];
        index_bytes.copy_from_slice(&src[..4]);
        src.advance(4);
        let index = u32::from_be_bytes(index_bytes);

        let mut begin_bytes = [0u8; 4];
        begin_bytes.copy_from_slice(&src[..4]);
        src.advance(4);
        let begin = u32::from_be_bytes(begin_bytes);

        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&src[..4]);
        src.advance(4);
        let length = u32::from_be_bytes(length_bytes);

        Ok(Some(Request::new(index, begin, length)))
    }
}

impl Encoder<Request> for RequestEncoder {
    type Error = std::io::Error;

    fn encode(&mut self, item: Request, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if dst.capacity() < Request::size() {
            dst.reserve(Request::size());
        }
        dst.extend_from_slice(&item.index);
        dst.extend_from_slice(&item.begin);
        dst.extend_from_slice(&item.length);

        Ok(())
    }
}
