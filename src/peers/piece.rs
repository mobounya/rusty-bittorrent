use tokio_util::bytes::{Buf, BytesMut};
use tokio_util::codec::Decoder;
use crate::peers::BLOCK_MAX;

pub struct Piece {
    pub index : u32,
    pub begin : u32,
    pub block : Vec<u8>
}

pub struct PieceDecoder {}

impl Piece {
    pub fn new(index : u32, begin : u32, block : Vec<u8>) -> Self {
        Self {
            index,
            begin,
            block,
        }
    }
}

impl PieceDecoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Decoder for PieceDecoder {
    type Item = Piece;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 * 2 {
            return Ok(None);
        }
        let mut index_bytes = [0u8; 4];
        index_bytes.copy_from_slice(&src[..4]);
        let index = u32::from_be_bytes(index_bytes);
        src.advance(4);

        let mut begin_bytes = [0u8; 4];
        begin_bytes.copy_from_slice(&src[..4]);
        let begin = u32::from_be_bytes(begin_bytes);
        src.advance(4);

        assert!(src.len() <= BLOCK_MAX as usize);

        let block = src.to_vec();
        src.advance(block.len());

        Ok(Some(
            Piece::new(index, begin, block)
        ))
    }
}