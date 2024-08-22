use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct File {
    length : i64,
    md5sum : Option<String>,
    path : Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Info {
    #[serde(rename = "piece length")]
    pub piece_length : u64,
    pub pieces : ByteBuf,
    private : Option<u8>,
    name : String,
    pub length : Option<u64>,
    md5sum : Option<String>,
    files : Option<Vec<File>>
}

// https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TorrentMetaInfo {
    pub info : Info,
    pub announce : String,
    #[serde(rename = "announce-list")]
    pub announce_list : Option<Vec<Vec<String>>>,
    #[serde(rename = "creation date")]
    pub creation_date : Option<i64>,
    pub comment : Option<String>,
    #[serde(rename = "created by")]
    pub created_by : Option<String>,
    pub encoding : Option<String>,
}

fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

impl TorrentMetaInfo {
    pub fn urlencode_info_hash(&self) -> String {
        let info_hash_raw = self.info.hash_raw();
        urlencoding::encode_binary(&*info_hash_raw).to_string()
    }
}

impl Display for TorrentMetaInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let info_hash = self.info.hash_base16();
        writeln!(f, "Tracker url: {}", self.announce)?;
        writeln!(f, "name: {}", self.info.name)?;
        writeln!(f, "piece length: {}", self.info.piece_length)?;
        if let Some(length) = self.info.length {
            writeln!(f, "length: {}", length)?;
        } else {
            writeln!(f, "length: None")?;
        }
        writeln!(f, "info hash: {}", info_hash)?;
        if let Some(md5sum) = self.info.md5sum.clone() {
            write!(f, "md5sum: {}", md5sum)?;
        } else {
            writeln!(f, "md5sum: None")?;
        }
        writeln!(f, "pieces: ")?;
        for offset in (0..self.info.pieces.len()).step_by(20) {
            let raw_hash_bytes = &self.info.pieces[offset..offset + 20];
            writeln!(f, "{}", bytes_to_hex_string(raw_hash_bytes))?;
        }
        write!(f, "")
    }
}

impl Info {
    pub fn hash_base16(&self) -> String {
        let hash = Sha1::digest(serde_bencode::to_bytes(self).unwrap());
        base16ct::lower::encode_string(&hash)
    }

    pub fn hash_raw(&self) -> Vec<u8> {
        Sha1::digest(serde_bencode::to_bytes(self).unwrap()).to_vec()
    }
}
