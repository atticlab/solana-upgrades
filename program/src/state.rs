use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// escrow
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct FibonacciSequence {
    pub fib: [u8; 2],
}

impl FibonacciSequence {
    pub const LEN: usize = 2;
    pub const FIB_SEQUENCE: [u8; Self::LEN] = [255 as u8, 254 as u8];
}
