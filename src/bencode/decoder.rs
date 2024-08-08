use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum DecoderError {
    NotBencodedData,
    InvalidByteStringLength,
    InvalidByteString,
    InvalidInteger,
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

    pub fn decode(&self) -> Result<Value, DecoderError> {
        if self.encoded_data.chars().next().unwrap().is_digit(10) {
            let (decoded_value, _to_skip) = self.decode_bencoded_byte_string()?;
            return Ok(Value::from(decoded_value.to_string()));
        } else if self.encoded_data.chars().next().unwrap() == 'i' {
            let (decoded_value, _to_skip) = self.decode_bencoded_integer()?;
            return Ok(Value::from(decoded_value));
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

    // https://wiki.theory.org/BitTorrentSpecification#Integers
    fn decode_bencoded_integer(&self) -> Result<(i64, usize), DecoderError> {
        let encoded_value = &self.encoded_data;
        let end_delimiter_index = match encoded_value.find("e") {
            Some(value) => value,
            None => return Err(DecoderError::InvalidInteger),
        };

        let integer_part_string = &encoded_value[1..end_delimiter_index];

        if integer_part_string.len() > 1 && &integer_part_string[0..2] == "-0" {
            return Err(DecoderError::InvalidInteger);
        }
        if integer_part_string.len() > 1 && &integer_part_string[0..2] == "+0" {
            return Err(DecoderError::InvalidInteger);
        }
        if integer_part_string.len() > 1 && integer_part_string.chars().next().unwrap() == '0' {
            return Err(DecoderError::InvalidInteger);
        }
        let integer_part_number = match integer_part_string.parse::<i64>() {
            Ok(value) => value,
            Err(_) => return Err(DecoderError::InvalidInteger)
        };
        return Ok((integer_part_number, integer_part_string.len() + 2));
    }
}
