use std::env;
use crate::metainfo::{Parser, TorrentMetaInfo};
use crate::peers::Peers;

mod metainfo;
mod peers;

fn parse_torrent_file(torrent_file_path : &String) -> TorrentMetaInfo {
    let parser = Parser::new(torrent_file_path.clone());
    match parser.parse() {
        Ok(parsed_value) => parsed_value,
        Err(_) => panic!("Could not decode file: {}", torrent_file_path)
    }
}

fn custom_assert(result: bool, panic_message: &str) {
    if result == false {
        panic!("{}", panic_message);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1].to_lowercase() == "info" {
        custom_assert(args.len() == 3, "usage: info [TORRENT_FILE_PATH]");
        let torrent_file_path = args[2].clone();
        let metainfo = parse_torrent_file(&torrent_file_path);
        print!("{}", metainfo);
    } else if args[1].to_lowercase() == "peers" {
        custom_assert(args.len() == 3, "usage: peers [TORRENT_FILE_PATH]");
        let torrent_file_path = args[2].clone();
        let metainfo = parse_torrent_file(&torrent_file_path);
        let peers = Peers::new(metainfo).discover().await.unwrap();
        println!("{}", peers);
    } else if args[1].to_lowercase() == "handshake" {
        custom_assert(args.len() == 4, "usage: handshake [TORRENT_FILE_PATH] PEER_IP:PEER_PORT");
        let torrent_file_path = args[2].clone();
        let peer_address = args[3].clone();
        let metainfo = parse_torrent_file(&torrent_file_path);
        let mut peers = Peers::new(metainfo);
        let handshake = peers.handshake(&peer_address).unwrap();
        println!("Peer ID: {}", base16ct::lower::encode_string(&handshake.peer_id));
        println!("Info hash: {}", base16ct::lower::encode_string(&handshake.info_hash));
    } else if args[1].to_lowercase() == "download_piece" {
        custom_assert(args.len() == 4, "usage: download_piece [TORRENT_FILE_PATH] PIECE_INDEX");
        let peer_address = String::from("167.71.141.80:51470");
        let torrent_file_path = args[2].clone();
        let piece_index : usize = args[3].parse::<usize>().expect("piece index is not a valid number");
        let metainfo = parse_torrent_file(&torrent_file_path);
        let mut peers = Peers::new(metainfo);
        peers.download_piece(&peer_address, piece_index).expect(format!("failed to download piece {piece_index}").as_str());
    }
}
