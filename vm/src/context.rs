use crate::bits;

pub struct Context {
    pub chain: Chain,
    pub block: Block,
    pub tx: Tx,
}

pub struct Chain {
    pub chain_id: bits::U256,
}

pub struct Block {
    pub number: bits::U256,
}

pub struct Tx {
    pub from: bits::Address,
}