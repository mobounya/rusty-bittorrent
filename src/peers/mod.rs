mod tracker_response;
pub use tracker_response::*;

use crate::metainfo::TorrentMetaInfo;

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
        let length = metainfo.info.length.unwrap();
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

    pub async fn discover(&self) -> Result<TrackerResponse, Box<dyn std::error::Error>>{
        let url = self.construct_url();
        let body = reqwest::get(url)
            .await?
            .bytes()
            .await?;
        let decoded_value : TrackerResponse = serde_bencode::from_bytes(&*body).expect("could not decode tracker response");
        Ok(decoded_value)
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