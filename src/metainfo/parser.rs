use std::fs;
use std::path::PathBuf;
use crate::metainfo::TorrentMetaInfo;

#[derive(Debug)]
pub enum ParserError {
    InvalidBencodedData,
    #[allow(dead_code)]
    CannotReadFile(String)
}

pub struct Parser {
    file_path : PathBuf,
    file_name : String
}

impl Parser {
    pub fn new(path : String) -> Self {
        let file_path = PathBuf::from(path);
        Parser {
            file_path: file_path.clone(),
            file_name: file_path.file_name().unwrap().to_str().unwrap().to_string(),
        }
    }

    pub fn parse(&self) -> Result<TorrentMetaInfo, ParserError> {
        let file_content = match fs::read(&self.file_path) {
            Ok(value) => value,
            Err(_) => return Err(ParserError::CannotReadFile(self.file_name.clone()))
        };
        let deserialized : TorrentMetaInfo = match serde_bencode::from_bytes(&file_content) {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                return Err(ParserError::InvalidBencodedData)
            }
        };

        Ok(deserialized)
    }
}
