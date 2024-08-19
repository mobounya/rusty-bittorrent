use bytemuck::Pod;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
// https://wiki.theory.org/BitTorrentSpecification#Handshake
pub struct Handshake {
    pub length : u8,
    pub protocol : [u8; 19],
    pub reserved : [u8; 8],
    pub info_hash : [u8; 20],
    pub peer_id : [u8; 20]
}

// Implement the bytemuck traits
unsafe impl bytemuck::Zeroable for Handshake {}
unsafe impl Pod for Handshake {}

impl Handshake {
    pub fn new(info_hash : [u8; 20], peer_id : [u8; 20]) -> Self {
        Self {
            length: 19,
            protocol: *b"BitTorrent protocol",
            reserved: [0; 8],
            info_hash,
            peer_id
        }
    }
}
