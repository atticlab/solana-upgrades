use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct InitArgsV1 {
    pub num: u32,
    pub num_2: u16,
    pub key: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct InitArgsV2 {
    pub num: u32,
    pub array: [u8; 64],
    pub key: Pubkey,
    pub key_2: Pubkey,
    pub num_2: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct V1ToV2UpgradeData {
    pub array: [u8; 64],
    pub key_2: Pubkey,
}
