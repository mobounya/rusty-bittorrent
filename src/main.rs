use crate::metainfo::Parser;

mod metainfo;

fn main() {
    let parser = Parser::new("./sample.torrent".to_string());
    let meta_info = parser.parse().expect("");
    println!("{:?}", meta_info);
}
