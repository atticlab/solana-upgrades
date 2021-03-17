//! State transition types
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
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
