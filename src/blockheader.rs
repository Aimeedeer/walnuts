use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub prev_block_hash: [u8; 32],
    pub time: i64,
    pub nonce: u32,
}

impl BlockHeader {
    pub fn new(prev_block_hash: [u8; 32], time: i64, nonce: u32) -> Self {
        BlockHeader {
            prev_block_hash,
            time,
            nonce,
        }
    }

    pub fn hash(&self) -> &[u8; 32] {
        let mut hasher = Sha256::default();
        hasher.reset();

	&self.prev_block_hash
    }
}
