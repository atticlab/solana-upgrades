//! State transition types
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub struct StructureMy {
    pub my_bytes: [u8],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct StateWithBytes {
    pub size: usize,
    pub my_bytes: [u8; 2],
    pub my_vec: Vec<u8>,
}

impl StateWithBytes {
    pub const LEN: usize = 2;
    pub const FIB_SEQUENCE: [u8; Self::LEN] = [255 as u8, 254 as u8];
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq)]
pub enum StateVersion {
    Uninitialized,
    V1,
    V2,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct StateV1 {
    pub state_version: StateVersion,
    pub num: u32,
    pub num_2: u16,
    pub key: Pubkey,
}



impl StateV1 {
    pub const LEN: usize = 1 + 4 + 2 + 32;
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct StateV2 {
    pub state_version: StateVersion,
    pub num: u32,
    pub array: [u8; 64],
    pub key: Pubkey,
    pub key_2: Pubkey,
    pub num_2: u64,
}

impl StateV2 {
    pub const LEN: usize = 1 + 4 + 64 + 32 + 32 + 8;
}

