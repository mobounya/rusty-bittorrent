use std::env;
use crate::metainfo::{Parser};
use crate::peers::Peers;

mod metainfo;
mod peers;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run [command] [TORRENT_FILE_PATH]");
        return;
    }
    if args[1] == "info" {
        let torrent_file_path = args[2].clone();
        let parser = Parser::new(torrent_file_path.clone());
        let metainfo = match parser.parse() {
            Ok(parsed_value) => parsed_value,
            Err(_) => {
                eprintln!("Could not decode file: {}", torrent_file_path);
                return;
            }
        };
        print!("{}", metainfo);
    } else if args[1] == "peers" {
        let torrent_file_path = args[2].clone();
        let parser = Parser::new(torrent_file_path.clone());
        let metainfo = match parser.parse() {
            Ok(parsed_value) => parsed_value,
            Err(_) => {
                eprintln!("Could not decode file: {}", torrent_file_path);
                return;
            }
        };
        let peers = Peers::new(metainfo).discover().await.unwrap();
        println!("{}", peers);
    }
}
