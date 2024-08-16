use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

// https://wiki.theory.org/BitTorrentSpecification#Tracker_Response
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TrackerResponse {
    #[serde(rename = "failure reason")]
    failure_reason : Option<String>,
    #[serde(rename = "warning reason")]
    warning_message : Option<String>,
    interval : u64,
    #[serde(rename = "min interval")]
    min_interval : Option<u64>,
    #[serde(rename = "tracker id")]
    tracker_id : Option<String>,
    complete : u64,
    incomplete : u64,
    peers : ByteBuf
}

impl Display for TrackerResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let peers = self.peers();
        let mut it = peers.iter().peekable();
        writeln!(f, "Peers:")?;
        while let Some(peer_ip) = it.next() {
            if it.peek().is_none() {
                write!(f, "{}", peer_ip)?;
            } else {
                writeln!(f, "{}", peer_ip)?;
            }
        }
        write!(f, "")
    }
}

impl TrackerResponse {
    pub fn peers(&self) -> Vec<String> {
        let mut i = 0;
        let mut peers_ips : Vec<String> = vec![];
        while i < self.peers.len() {
            let ip_as_string = Self::ip_bytes_to_ip_string(&self.peers[i..i+6].to_vec());
            peers_ips.push(ip_as_string);
            i += 6;
        }
        peers_ips
    }

    pub fn ip_bytes_to_ip_string(bytes : &Vec<u8>) -> String {
        assert!(bytes.len() >= 6);
        let mut ip_string = String::from("");
        for i in 0..4 {
            let byte = bytes[i];
            ip_string.push_str(&*byte.to_string());
            if i < 3 {
                ip_string.push('.');
            } else {
                ip_string.push(':');
            }
        }
        let port = u16::from_be_bytes([bytes[4], bytes[5]]);
        ip_string.push_str(port.to_string().as_str());
        ip_string
    }
}
