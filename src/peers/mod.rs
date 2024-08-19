mod tracker_response;
mod handshake;

use std::io::{Read, Write};
use std::net::TcpStream;
pub use tracker_response::*;

use crate::metainfo::TorrentMetaInfo;
use crate::peers::handshake::Handshake;

pub struct Peers {
    metainfo : TorrentMetaInfo,
    peer_id : String,
    port : String, // 6881-6889
    uploaded : u64,
    downloaded : u64,
    left: u64,
    compact: bool
}

impl Peers {
    pub fn new(metainfo : TorrentMetaInfo) -> Self {
        // ad-hoc: length is not available in all torrent files, so I try to deduce the length
        let length = match metainfo.info.length {
            None => (metainfo.info.pieces.len() / 20) as u64 * metainfo.info.piece_length,
            Some(len) => len
        };
        Peers {
            metainfo,
            peer_id: "13374313374313374369".to_string(),
            port: "6882".to_string(),
            uploaded: 0,
            downloaded: 0,
            left: length,
            compact: true
        }
    }

    pub async fn discover(&self) -> Result<TrackerResponse, Box<dyn std::error::Error>> {
        let url = self.construct_url();
        let body = reqwest::get(url)
            .await?
            .bytes()
            .await?;
        let decoded_value : TrackerResponse = serde_bencode::from_bytes(&*body).expect("could not decode tracker response");
        Ok(decoded_value)
    }

    // https://wiki.theory.org/BitTorrentSpecification#Handshake
    pub fn handshake(&self, peer_ip : &String) -> Result<Handshake, Box<dyn std::error::Error>> {
        let mut handshake : Handshake = Handshake::new(<[u8; 20]>::try_from(self.metainfo.info.hash_raw()).unwrap(),
                                                      <[u8; 20]>::try_from(self.peer_id.as_bytes()).unwrap());
        {
            let mut stream = TcpStream::connect(peer_ip)?;
            let handshake_bytes = bytemuck::bytes_of(&handshake);
            assert_eq!(handshake_bytes.len(), size_of::<Handshake>());
            stream.write(handshake_bytes)?;

            let mut peer_handshake_bytes: [u8; size_of::<Handshake>()] = [0; size_of::<Handshake>()];
            let bytes_read = stream.read(&mut peer_handshake_bytes)?;
            assert_eq!(bytes_read, size_of::<Handshake>());
            handshake = *bytemuck::from_bytes(&peer_handshake_bytes);
        }
        Ok(handshake)
    }


    fn urlencode<T: ToString>(data : &T) -> String {
        urlencoding::encode(&data.to_string()).to_string()
    }

    fn construct_url(&self) -> String {
        let tracker_url = self.metainfo.announce.clone();
        let urlencoded_bencoded_info_hash = self.metainfo.urlencode_info_hash();
        let urlencoded_peer_id = Self::urlencode(&self.peer_id);
        let urlencoded_port = Self::urlencode(&self.port);
        let urlencoded_uploaded = Self::urlencode(&self.uploaded);
        let urlencoded_downloaded = Self::urlencode(&self.downloaded);
        let urlencoded_left = Self::urlencode(&self.left);
        let urlencoded_compact = match self.compact {
            true => "1".to_string(),
            false => "0".to_string()
        };
        tracker_url + "?info_hash=" + &*urlencoded_bencoded_info_hash + "&peer_id=" + &*urlencoded_peer_id
            + "&port=" + &*urlencoded_port + "&uploaded=" + &*urlencoded_uploaded + "&downloaded=" + &*urlencoded_downloaded
            + "&left=" + &*urlencoded_left + "&compact=" + &*urlencoded_compact
    }
}