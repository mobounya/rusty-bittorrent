mod tracker_response;
mod handshake;
mod peer_message;
mod request;
mod piece;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::collections::HashMap;
use std::fs;
use sha1::{Digest, Sha1};
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};
pub use tracker_response::*;

use crate::metainfo::TorrentMetaInfo;
use crate::peers::handshake::Handshake;
use crate::peers::peer_message::{MessageID, PeerMessage, PeerMessageDecoder, PeerMessageEncoder};
use crate::peers::piece::PieceDecoder;
use crate::peers::request::{Request, RequestEncoder};

pub const BLOCK_MAX : u64 = 16 * 1024;

pub struct Peers {
    metainfo : TorrentMetaInfo,
    peer_id : String,
    port : String, // 6881-6889
    uploaded : u64,
    downloaded : u64,
    left : u64,
    compact : bool,
    peers_connections : HashMap<String, TcpStream>,
    pieces_hash : Vec<String>,
}

impl Peers {
    pub fn new(metainfo : TorrentMetaInfo) -> Self {
        // ad-hoc: length is not available in all torrent files, so I try to deduce the length
        let length = match metainfo.info.length {
            None => (metainfo.info.pieces.len() / 20) as u64 * metainfo.info.piece_length,
            Some(len) => len
        };
        let mut pieces_hash : Vec<String> = vec![];
        for index in (0..metainfo.info.pieces.len()).step_by(20) {
            let raw_hash = &metainfo.info.pieces[index..index + 20];
            pieces_hash.push(base16ct::lower::encode_string(raw_hash));
        }

        Peers {
            metainfo,
            peer_id: "13374313374313374369".to_string(),
            port: "6882".to_string(),
            uploaded: 0,
            downloaded: 0,
            left: length,
            compact: true,
            peers_connections: HashMap::new(),
            pieces_hash,
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
    pub fn handshake(&mut self, peer_ip : &String) -> Result<Handshake, Box<dyn std::error::Error>> {
        let handshake : Handshake = Handshake::new(<[u8; 20]>::try_from(self.metainfo.info.hash_raw()).unwrap(),
                                                      <[u8; 20]>::try_from(self.peer_id.as_bytes()).unwrap());
        let mut stream = TcpStream::connect(peer_ip)?;
        let handshake_bytes = bytemuck::bytes_of(&handshake);
        assert_eq!(handshake_bytes.len(), size_of::<Handshake>());
        stream.write(handshake_bytes)?;

        let mut peer_handshake_bytes: [u8; size_of::<Handshake>()] = [0; size_of::<Handshake>()];
        let bytes_read = stream.read(&mut peer_handshake_bytes)?;
        assert_eq!(bytes_read, size_of::<Handshake>());
        let handshake : Handshake = *bytemuck::from_bytes(&peer_handshake_bytes);

        self.peers_connections.insert(base16ct::lower::encode_string(&handshake.peer_id), stream);
        Ok(handshake)
    }

    pub fn download_piece(&mut self, peer_ip : &String, piece_index : usize) -> Result<(), Box<dyn std::error::Error>> {
        assert!(piece_index < self.pieces_hash.len());

        let handshake = self.handshake(peer_ip)?;
        let mut tcp_stream = self.peers_connections.get(&base16ct::lower::encode_string(&handshake.peer_id)).unwrap();

        let piece_length = if piece_index == (self.pieces_hash.len() - 1) {
            match self.metainfo.info.length.unwrap() % self.metainfo.info.piece_length {
                0 => self.metainfo.info.piece_length,
                len => len,
            }
        } else {
            self.metainfo.info.piece_length
        };

        let mut buffer : BytesMut = BytesMut::new();
        let mut received_bitfield_peer_message = false;
        let mut received_no_data_counter = 0;
        let mut downloaded_piece_data : Vec<u8> = vec![0; piece_length as usize];

        let mut received_data : usize = 0;
        loop {
            let mut stream_buffer : [u8; 1024] = [0; 1024];
            loop {
                let bytes_read = tcp_stream.read(&mut stream_buffer)?;
                if bytes_read > 0 {
                    buffer.extend_from_slice(&stream_buffer[..bytes_read]);
                    break;
                } else {
                    received_no_data_counter += 1;
                    if received_no_data_counter == 5 {
                        panic!("Stopped receiving data")
                    }
                }
            }

            // try to decode all the bytes in the buffer if you can
            loop {
                let peer_message = PeerMessageDecoder::new().decode(&mut buffer)?;
                match peer_message {
                    None => break,
                    Some(peer_message) => {
                        match peer_message.message_id {
                            MessageID::Bitfield => {
                                if received_bitfield_peer_message {
                                    panic!("Received Bitfield peer message twice");
                                }
                                received_bitfield_peer_message = true;
                                let peer_message = PeerMessage::new(MessageID::Interested, None)?;
                                let mut buffer : BytesMut = BytesMut::new();
                                let _ = PeerMessageEncoder::new().encode(peer_message, &mut buffer)?;
                                tcp_stream.write(buffer.as_ref())?;
                            },
                            MessageID::UnChoke => {
                                let nblocks = (piece_length + (BLOCK_MAX - 1)) / BLOCK_MAX;
                                for block in 0..nblocks {
                                    let block_length;
                                    if block == nblocks - 1 {
                                        block_length = piece_length - (block * BLOCK_MAX);
                                    } else {
                                        block_length = BLOCK_MAX;
                                    }
                                    let piece = Request::new(piece_index as u32, (block * BLOCK_MAX) as u32, block_length as u32);
                                    let mut piece_bytes : BytesMut = BytesMut::new();
                                    RequestEncoder::new().encode(piece, &mut piece_bytes)?;

                                    let request_message_peer = PeerMessage::new(MessageID::Request, Some(piece_bytes.to_vec()))?;
                                    let mut request_message_peer_bytes : BytesMut = BytesMut::new();
                                    PeerMessageEncoder::new().encode(request_message_peer, &mut request_message_peer_bytes)?;

                                    tcp_stream.write(request_message_peer_bytes.as_ref())?;
                                }
                            },
                            MessageID::Piece => {
                                let payload = peer_message.payload.expect("Piece message should have a payload");
                                let mut payload_raw_bytes = BytesMut::from(payload.as_slice());
                                let piece = PieceDecoder::new().decode(&mut payload_raw_bytes)?.expect("Failed decoding payload");
                                assert_eq!(piece.index as usize, piece_index);
                                downloaded_piece_data.splice(piece.begin as usize..piece.begin as usize + piece.block.len(), piece.block.clone());
                                received_data += piece.block.len();

                                if received_data == piece_length as usize {
                                    let hash = Sha1::digest(&downloaded_piece_data);
                                    assert_eq!(base16ct::lower::encode_string(&hash), self.pieces_hash[piece_index]);
                                    let filename = "/tmp/piece-".to_string() + piece_index.to_string().as_str();
                                    Peers::write_raw_bytes_to_file(&filename, &downloaded_piece_data);
                                    println!("Wrote {received_data} bytes to file '{filename}'");
                                    return Ok(());
                                }
                            },
                            _ => panic!("panic")
                        }
                    }
                }

            }
        }
    }

    fn write_raw_bytes_to_file(filename : &String, raw_data : &Vec<u8>) {
        let mut file = fs::File::create(filename).expect("Could not create file for piece");
        file.write_all(raw_data.as_slice()).expect("Could not write to piece file");
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
