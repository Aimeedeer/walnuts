use crate::blockheader::BlockHeader;
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use chrono::DateTime;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, txs: Vec<Transaction>) -> Self {
	Block {
	    header,
	    txs,	    
	}
    }

    /// Create the genesis block
    /// TODO: move const to another file
    /// (config or const file)
    pub fn genesis_init() -> Self {
	let mut prev_block_hash = Sha256::new();
	prev_block_hash.update(b"The genesis block");

	let prev_block_hash: [u8; 32] = prev_block_hash.finalize().into();
	let time = DateTime::parse_from_str(
	    "2020 Nov 10 10:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z").unwrap();
	let time = time.timestamp_millis();
	// println!("The timestamp is {:?}", time);

	let nonce = 1;

	// Create the genesis block header
	let genesis_block_header = BlockHeader::new(prev_block_hash, time, nonce);

	// Create the genesis block
	// TODO: create txs
	let genesis = Block::new(genesis_block_header, Vec::new());
	// println!("New block is created: {:?}", new_block.block_hash());
	genesis
    }
    
    pub fn hash(&self) -> &[u8; 32] {
	&self.header.hash()
    }
}

