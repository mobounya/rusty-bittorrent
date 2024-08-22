use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use std::io::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageID {
    Choke = 0,
    UnChoke = 1,
    Interested = 2,
    NotInterested = 3,
    Have = 4,
    Bitfield = 5,
    Request = 6,
    Piece = 7,
    Cancel = 8,
    Port = 9
}

#[derive(Debug, Clone)]
pub struct PeerMessage {
    pub message_id : MessageID,
    pub payload : Option<Vec<u8>>
}

pub struct PeerMessageDecoder {}
pub struct PeerMessageEncoder {}


impl MessageID {
    pub fn should_have_payload(&self) -> bool {
        match self {
            MessageID::Choke => false,
            MessageID::UnChoke => false,
            MessageID::Interested => false,
            MessageID::NotInterested => false,
            _ => true
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            MessageID::Choke => 0,
            MessageID::UnChoke => 1,
            MessageID::Interested => 2,
            MessageID::NotInterested => 3,
            MessageID::Have => 4,
            MessageID::Bitfield => 5,
            MessageID::Request => 6,
            MessageID::Piece => 7,
            MessageID::Cancel => 8,
            MessageID::Port => 9
        }
    }

    pub fn from_u8(message_id : u8) -> Result<MessageID, Error> {
        match message_id {
            0 => Ok(MessageID::Choke),
            1 => Ok(MessageID::UnChoke),
            2 => Ok(MessageID::Interested),
            3 => Ok(MessageID::NotInterested),
            4 => Ok(MessageID::Have),
            5 => Ok(MessageID::Bitfield),
            6 => Ok(MessageID::Request),
            7 => Ok(MessageID::Piece),
            8 => Ok(MessageID::Cancel),
            9 => Ok(MessageID::Port),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid message id: {message_id}"),
            ))
        }
    }
}

impl PeerMessage {
    pub fn new(message_id : MessageID, payload : Option<Vec<u8>>) -> Result<Self, Error> {
        if message_id.should_have_payload() {
            if let None = payload {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Peer message with message id {} should have a payload", message_id.to_u8()),
                ));
            }
            if let Some(ref payload) = payload {
                if payload.is_empty() {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Peer message with message id {} should have a payload", message_id.to_u8()),
                    ));
                }
            }
        } else {
            if let Some(ref payload) = payload {
                if payload.is_empty() == false {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Peer message with message id {} should not have a payload", message_id.to_u8()),
                    ));
                }
            }
        }

        Ok(Self { message_id, payload })
    }

    pub fn size(&self) -> usize {
        let mut size = 1usize; // 1 for message_id
        if let Some(ref payload) = self.payload {
            size += payload.len();
        }
        size
    }
}

impl PeerMessageDecoder {
    pub fn new() -> Self {
        PeerMessageDecoder {}
    }
}

impl PeerMessageEncoder {
    pub fn new() -> Self {
        PeerMessageEncoder {}
    }
}

impl Decoder for PeerMessageDecoder {
    type Item = PeerMessage;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }

        let mut length_prefix_bytes: [u8; 4] = [0; 4];
        length_prefix_bytes.copy_from_slice(&src[..4]);
        let length_prefix = u32::from_be_bytes(length_prefix_bytes);

        // This is a keep-alive message, discard it for now.
        if length_prefix == 0 {
            println!("Received keep-alive message");
            src.advance(4);
            return self.decode(src);
        }
        if src.len() < (4 + length_prefix) as usize {
            return Ok(None);
        }
        src.advance(4);

        let mut message_id_byte = [0u8; 1];
        message_id_byte.copy_from_slice(&src[..1]);
        src.advance(1);

        let message_id = MessageID::from_u8(u8::from_be_bytes(message_id_byte))?;

        // Some type of messages do not have a payload
        if message_id.should_have_payload() == false {
            return Ok(Some(
                PeerMessage {
                    message_id,
                    payload: None,
                }
            ));
        }

        let payload: Vec<u8> = src[..length_prefix as usize - 1usize].to_vec();
        src.advance(length_prefix as usize - 1usize);

        Ok(Some(
            PeerMessage {
                message_id,
                payload: Some(payload),
            }
        ))
    }
}

impl Encoder<PeerMessage> for PeerMessageEncoder {
    type Error = std::io::Error;

    fn encode(&mut self, item: PeerMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if dst.capacity() < item.size() + 4 {
            dst.reserve(item.size() + 4);
        }
        dst.put_u32_ne(item.size() as u32);
        dst.put_u8(item.message_id.to_u8());
        if let Some(payload) = item.payload {
            dst.extend_from_slice(&payload);
        }
        Ok(())
    }
}
