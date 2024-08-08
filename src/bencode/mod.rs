mod decoder;
pub use decoder::*;

#[cfg(test)]
mod tests {
    use serde_json::{Number, Value};
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

    #[test]
    fn valid_single_digit_integer() {
        let bencode_byte_string = "i0e".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_value) => {
                match decoded_value {
                    Value::Number(decoded_integer) => {
                        assert_eq!(decoded_integer, Number::from(0));
                    }
                    _ => {
                        panic!("Decoded value should be a Number");
                    }
                }
            }
            Err(err) => {
                panic!("Decoding valid integer should not return an error");
            }
        }
    }

    #[test]
    fn valid_positive_integer() {
        let bencode_byte_string = "i+42e".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_value) => {
                match decoded_value {
                    Value::Number(decoded_integer) => {
                        assert_eq!(decoded_integer, Number::from(42));
                    }
                    _ => {
                        panic!("Decoded value should be a Number");
                    }
                }
            }
            Err(err) => {
                panic!("Decoding a valid integer should not return an error");
            }
        }
    }

    #[test]
    fn valid_negative_integer() {
        let bencode_byte_string = "i-42e".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_value) => {
                match decoded_value {
                    Value::Number(decoded_integer) => {
                        assert_eq!(decoded_integer, Number::from(-42));
                    }
                    _ => {
                        panic!("Decoded value should be a Number");
                    }
                }
            }
            Err(err) => {
                panic!("Decoding a valid integer should not return an error");
            }
        }
    }

    #[test]
    fn invalid_positive_zero_integer() {
        let bencode_byte_string = "i+0e".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode a positive 0"),
            Err(err) => assert_eq!(err, DecoderError::InvalidInteger),
        }
    }

    #[test]
    fn invalid_negative_zero_integer() {
        let bencode_byte_string = "i-0e".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode a negative 0"),
            Err(err) => assert_eq!(err, DecoderError::InvalidInteger),
        }
    }

    #[test]
    fn invalid_begin_delimiter_integer() {
        let bencode_byte_string = "42e".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode invalid integer"),
            Err(err) => assert_eq!(err, DecoderError::InvalidByteString),
        }
    }

    #[test]
    fn invalid_end_delimiter_integer() {
        let bencode_byte_string = "i42".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode invalid integer"),
            Err(err) => assert_eq!(err, DecoderError::InvalidInteger),
        }
    }

    #[test]
    fn invalid_start_with_zero_integer() {
        let bencode_byte_string = "i011e".to_string();
        let decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode invalid integer"),
            Err(err) => assert_eq!(err, DecoderError::InvalidInteger),
        }
    }
}