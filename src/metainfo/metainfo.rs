use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct File {
    length : i64,
    md5sum : Option<String>,
    path : Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Info {
    #[serde(rename = "piece length")]
    piece_length : i64,
    pieces : ByteBuf,
    private : Option<u8>,
    name : String,
    length : Option<u64>,
    md5sum : Option<String>,
    files : Option<Vec<File>>
}

// https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TorrentMetaInfo {
    info : Info,
    announce : String,
    #[serde(rename = "announce-list")]
    announce_list : Option<Vec<Vec<String>>>,
    #[serde(rename = "creation date")]
    creation_date : Option<i64>,
    comment : Option<String>,
    #[serde(rename = "created by")]
    created_by : Option<String>,
    encoding : Option<String>,
}
