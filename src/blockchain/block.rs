use std::fmt::{ self, Debug, Formatter };
use crate::BlkHash;

pub struct Blk {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlkHash,
    pub prev_blk: BlkHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Blk {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "blk[{}]: {} at {} w/ {}", &self.index, &hex::encode(&self.hash), &self.timestamp, &self.payload )
    }
}

impl Blk {
    pub fn new (
        index: u32,
        timestamp: u128,
        prev_blk: BlkHash,
        nonce: u64,
        payload: String,
    ) -> Self {
        Blk {
            index,
            timestamp,
            hash: vec![0; 32], // move away from sha256 eventually
            prev_blk, 
            nonce,
            payload,
        }
    }
}