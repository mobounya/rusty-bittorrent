use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum DecoderError {
    NotBencodedData,
    InvalidByteStringLength,
    InvalidByteString,
    InvalidInteger,
    DictionaryKeyIsNotAString,
    ValueNotPresentInDictionary,
    ListFormatError
}

pub struct Decoder {
    encoded_data : String
}

// https://wiki.theory.org/BitTorrentSpecification#Bencoding
impl Decoder {
    pub fn new(encoded_data: String) -> Self {
        Decoder {
            encoded_data,
        }
    }

    pub fn decode(&mut self) -> Result<Value, DecoderError> {
        if self.encoded_data.chars().next().unwrap().is_digit(10) {
            let decoded_value = self.decode_bencoded_byte_string()?;
            return Ok(Value::from(decoded_value.to_string()));
        } else if self.encoded_data.chars().next().unwrap() == 'i' {
            let decoded_value  = self.decode_bencoded_integer()?;
            return Ok(Value::from(decoded_value));
        } else if self.encoded_data.chars().next().unwrap() == 'd' {
            let decoded_value  = self.decode_bencoded_dictionary()?;
            return Ok(Value::from(decoded_value));
        } else if self.encoded_data.chars().next().unwrap() == 'l' {
            let decoded_value  = self.decode_bencoded_list()?;
            return Ok(Value::from(decoded_value));
        }
        return Err(DecoderError::NotBencodedData);
    }

    // https://wiki.theory.org/BitTorrentSpecification#Byte_Strings
    fn decode_bencoded_byte_string(&mut self) -> Result<String, DecoderError> {
        let encoded_value = self.encoded_data.clone();
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

        let string = encoded_value[colon_index + 1..colon_index + 1 + number].to_string().clone();
        let chars_processed = number_string.len() + 1 + string.len();
        self.encoded_data = String::from(&encoded_value[chars_processed..]).clone();

        return Ok(string);
    }

    // https://wiki.theory.org/BitTorrentSpecification#Integers
    fn decode_bencoded_integer(&mut self) -> Result<i64, DecoderError> {
        let encoded_value = self.encoded_data.clone();
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
        let chars_processed = integer_part_string.len() + 2;
        self.encoded_data = encoded_value[chars_processed..].to_string();
        return Ok(integer_part_number);
    }

    // https://wiki.theory.org/BitTorrentSpecification#Lists
    fn decode_bencoded_list(&mut self) -> Result<Vec<Value>, DecoderError> {
        let mut list : Vec<Value> = vec!();
        let mut found_end_delimeter = false;

        if self.encoded_data == "le" {
            return Ok(list);
        }

        self.encoded_data = self.encoded_data[1..].to_string().clone();
        while !self.encoded_data.is_empty() {
            let decoded_value = self.decode()?;
            list.push(decoded_value);

            if !self.encoded_data.is_empty() && self.encoded_data.chars().next().unwrap() == 'e' {
                found_end_delimeter = true;
                break;
            }
        }

        if !found_end_delimeter {
            return Err(DecoderError::ListFormatError);
        }

        return Ok(list);
    }

    // https://wiki.theory.org/BitTorrentSpecification#Dictionaries
    fn decode_bencoded_dictionary(&mut self) -> Result<serde_json::Map<String, Value>, DecoderError> {
        let mut dict = serde_json::Map::<String, Value>::new();

        if self.encoded_data == "de" {
            return Ok(dict);
        }

        self.encoded_data = self.encoded_data[1..].to_string().clone();

        while !self.encoded_data.is_empty() {
            let decoded_key = self.decode()?;
            if !decoded_key.is_string() {
                return Err(DecoderError::DictionaryKeyIsNotAString);
            }
            if self.encoded_data == "e" {
                return Err(DecoderError::ValueNotPresentInDictionary);
            }
            let decoded_value= self.decode()?;

            dict.insert(decoded_key.to_string(), decoded_value.clone());

            if self.encoded_data.chars().next().unwrap() == 'e' {
                break;
            }
        }

        return Ok(dict);
    }
}
