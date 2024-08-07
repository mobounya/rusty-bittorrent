use serde_json::Value::{String as JsonString};

#[derive(PartialEq, Eq, Debug)]
pub enum DecoderError {
    NotBencodedData,
    InvalidByteStringLength,
    InvalidByteString
}

pub struct Decoder {
    encoded_data : String,
}

// https://wiki.theory.org/BitTorrentSpecification#Bencoding
impl Decoder {
    pub fn new(encoded_data: String) -> Self {
        return Decoder {
            encoded_data,
        };
    }

    pub fn decode(&self) -> Result<serde_json::Value, DecoderError> {
        if self.encoded_data.chars().next().unwrap().is_digit(10) {
            let (decoded_value, _to_skip) = self.decode_bencoded_byte_string()?;
            return Ok(JsonString(decoded_value.to_string()));
        }
        return Err(DecoderError::NotBencodedData);
    }

    // https://wiki.theory.org/BitTorrentSpecification#Byte_Strings
    fn decode_bencoded_byte_string(&self) -> Result<(&str, usize), DecoderError> {
        let encoded_value = &self.encoded_data;
        let colon_index = match encoded_value.find(':') {
            Some(index) => index,
            None => return Err(DecoderError::InvalidByteString),
        };
        let number_string = &encoded_value[..colon_index];
        let number = match number_string.parse::<usize>() {
            Ok(v) => v,
            Err(_) => return Err(DecoderError::InvalidByteString),
        };
        if encoded_value[colon_index + 1..].len() < number {
            return Err(DecoderError::InvalidByteStringLength);
        }
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number];
        return Ok((string, number_string.len() + 1 + string.len()));
    }
}
