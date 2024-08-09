mod decoder;
pub use decoder::*;

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use super::*;

    #[test]
    fn valid_short_byte_string() {
        let bencode_byte_string = "3:hel".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
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
        let mut decoder = Decoder::new(bencode_byte_string);
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
        let mut decoder = Decoder::new(bencode_byte_string);
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
        let mut decoder = Decoder::new(bencode_byte_string);
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
        let mut decoder = Decoder::new(bencode_byte_string);
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
        let mut decoder = Decoder::new(bencode_byte_string);
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
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_value) => {
                match decoded_value {
                    Value::Number(decoded_integer) => {
                        assert_eq!(decoded_integer.as_i64().unwrap(), 0);
                    }
                    _ => {
                        panic!("Decoded value should be a Number");
                    }
                }
            }
            Err(_err) => {
                panic!("Decoding valid integer should not return an error");
            }
        }
    }

    #[test]
    fn valid_positive_integer() {
        let bencode_byte_string = "i+42e".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_value) => {
                match decoded_value {
                    Value::Number(decoded_integer) => {
                        assert_eq!(decoded_integer.as_i64().unwrap(), 42);
                    }
                    _ => {
                        panic!("Decoded value should be a Number");
                    }
                }
            }
            Err(_err) => {
                panic!("Decoding a valid integer should not return an error");
            }
        }
    }

    #[test]
    fn valid_negative_integer() {
        let bencode_byte_string = "i-42e".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_value) => {
                match decoded_value {
                    Value::Number(decoded_integer) => {
                        assert_eq!(decoded_integer.as_i64().unwrap(), -42);
                    }
                    _ => {
                        panic!("Decoded value should be a Number");
                    }
                }
            }
            Err(_err) => panic!("Decoding a valid integer should not return an error")
        }
    }

    #[test]
    fn invalid_positive_zero_integer() {
        let bencode_byte_string = "i+0e".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode a positive 0"),
            Err(err) => assert_eq!(err, DecoderError::InvalidInteger),
        }
    }

    #[test]
    fn invalid_negative_zero_integer() {
        let bencode_byte_string = "i-0e".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode a negative 0"),
            Err(err) => assert_eq!(err, DecoderError::InvalidInteger),
        }
    }

    #[test]
    fn invalid_begin_delimiter_integer() {
        let bencode_byte_string = "42e".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode invalid integer"),
            Err(err) => assert_eq!(err, DecoderError::InvalidByteString),
        }
    }

    #[test]
    fn invalid_end_delimiter_integer() {
        let bencode_byte_string = "i42".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode invalid integer"),
            Err(err) => assert_eq!(err, DecoderError::InvalidInteger),
        }
    }

    #[test]
    fn invalid_start_with_zero_integer() {
        let bencode_byte_string = "i011e".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode invalid integer"),
            Err(err) => assert_eq!(err, DecoderError::InvalidInteger),
        }
    }

    #[test]
    fn valid_empty_dictionary() {
        let bencode_byte_string = "de".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_dict) => {
                match decoded_dict {
                    Value::Object(decoded_dict) => assert_eq!(decoded_dict.len(), 0),
                    _ => panic!("Decoded value should be an Object")
                }
            },
            Err(_) => panic!("Decoding an empty dictionary should not return an error")
        }
    }

    #[test]
    fn valid_dictionary_with_key_and_one_byte_string() {
        let bencode_byte_string = "d5:hello5:helloe".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_dict) => {
                match decoded_dict {
                    Value::Object(decoded_dict) => {
                        // FIXME: should be hello instead of "hello"
                        match decoded_dict.get("\"hello\"") {
                            None => panic!("Dictionary should have a key 'hello'"),
                            Some(value) => {
                                match value {
                                    Value::String(val) => {
                                        assert_eq!(val, "hello");
                                    },
                                    _ => panic!("Dictionary should have a value of type String with key 'hello'")
                                }
                            }
                        };
                    },
                    _ => panic!("Decoded value should be an Object")
                }
            },
            Err(_) => panic!("Decoding a valid dictionary should not return an error")
        }
    }

    #[test]
    fn valid_dictionary_with_key_and_one_integer() {
        let bencode_byte_string = "d5:helloi42ee".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_dict) => {
                match decoded_dict {
                    Value::Object(decoded_dict) => {
                        // FIXME: should be hello instead of "hello"
                        match decoded_dict.get("\"hello\"") {
                            None => panic!("Dictionary should have a key 'hello'"),
                            Some(value) => {
                                match value {
                                    Value::Number(val) => {
                                        assert_eq!(val.as_i64().unwrap(), 42);
                                    },
                                    _ => panic!("Dictionary should have a value of type Number with key 'hello'")
                                }
                            }
                        };
                    },
                    _ => panic!("Decoded value should be an Object")
                }
            },
            Err(_) => panic!("Decoding a valid dictionary should not return an error")
        }
    }

    #[test]
    fn valid_dictionary_with_2_keys_2_values() {
        let bencode_byte_string = "d4:key1i1e4:key2i2ee".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(decoded_value) => {
                match decoded_value {
                    Value::Object(decoded_dict) => {
                        match decoded_dict.get("\"key1\"") {
                            Some(value) => {
                                match value {
                                    Value::Number(integer_value) => assert_eq!(integer_value.as_i64().unwrap(), 1),
                                    _ => panic!("Dictionary value for \"key1\" should be of type Number")
                                }
                            }
                            None => panic!("Dictionary should have a value for \"key1\"")
                        }
                        match decoded_dict.get("\"key2\"") {
                            Some(value) => {
                                match value {
                                    Value::Number(integer_value) => assert_eq!(integer_value.as_i64().unwrap(), 2),
                                    _ => panic!("Dictionary value for \"key2\" should be of type Number")
                                }
                            }
                            None => panic!("Dictionary should have a value for \"key2\"")
                        }
                    },
                    _ => panic!("Decoded dictionary should be of type Object")
                }
            },
            Err(_) => panic!("Decoding a valid dictionary should not return an error")
        }
    }

    #[test]
    fn invalid_dictionary_key() {
        let bencode_byte_string = "di42ei1337ee".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode Dictionary with a key that is not a String"),
            Err(err) => assert_eq!(err, DecoderError::DictionaryKeyIsNotAString)
        }
    }

    #[test]
    fn invalid_dictionary_with_key_and_no_value() {
        let bencode_byte_string = "d4:key1e".to_string();
        let mut decoder = Decoder::new(bencode_byte_string);
        match decoder.decode() {
            Ok(_) => panic!("Should not decode Dictionary with key but no value"),
            Err(err) => assert_eq!(err, DecoderError::ValueNotPresentInDictionary)
        }
    }
}