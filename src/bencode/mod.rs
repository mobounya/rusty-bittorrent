mod decoder;
pub use decoder::*;

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use super::*;

    #[test]
    fn valid_short_byte_string() {
        let bencode_byte_string = "3:hel".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_value) => {
                match decoded_value {
                    Value::String(decoded_byte_string) => {
                        assert_eq!(decoded_byte_string, "hel");
                    }
                    _ => {
                        panic!("Decoded value should be a String");
                    }
                }
            }
            Err(_) => {
                panic!("Error decoding value");
            }
        } 
    }

    #[test]
    fn valid_long_byte_string() {
        let bencode_byte_string = "13:rusty_torrent".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_value) => {
                match decoded_value {
                    Value::String(decoded_byte_string) => {
                        assert_eq!(decoded_byte_string, "rusty_torrent");
                    }
                    _ => {
                        panic!("Decoded value should be a String");
                    }
                }
            }
            Err(_) => {
                panic!("Error decoding value");
            }
        }
    }

    #[test]
    fn valid_too_long_byte_string() {
        // len("rusty_torrent") = 13
        let bencode_byte_string = "5:rusty_torrent".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_value) => {
                match decoded_value {
                    Value::String(decoded_byte_string) => {
                        assert_eq!(decoded_byte_string, "rusty");
                    }
                    _ => {
                        panic!("Decoded value should be a String");
                    }
                }
            }
            Err(_) => {
                panic!("Error decoding value");
            }
        }
    }

    #[test]
    fn not_bencoded_byte_string() {
        let bencode_byte_string = "foo_bar".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => {
                panic!("Invalid bencoded value should return an error");
            }
            Err(err) => {
                assert_eq!(err, DecoderError::NotBencodedData);
            }
        }
    }

    #[test]
    fn not_bencoded_byte_string_2() {
        let bencode_byte_string = "5_foobar".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => {
                panic!("Invalid bencoded value should return an error");
            }
            Err(err) => {
                assert_eq!(err, DecoderError::InvalidByteString);
            }
        }
    }

    #[test]
    fn invalid_byte_string_length() {
        let bencode_byte_string = "4:to".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => {
                panic!("Invalid bencoded value should return an error");
            }
            Err(err) => {
                assert_eq!(err, DecoderError::InvalidByteStringLength);
            }
        }
    }
}