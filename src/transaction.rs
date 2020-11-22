use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub tx_id: u32,
    pub input: Vec<Txin>,
    pub output: Vec<Txout>,
}

impl Transaction {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Txin {
    pub sequence: u32,
    pub vout: u32,
    pub pub_key: [u8; 32],
    pub signature: Vec<u8>,
}

impl Txin {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Txout {
    pub pub_key_hash: [u8; 32],
    pub vout: u32,
}

impl Txout {

}
