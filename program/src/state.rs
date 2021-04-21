use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;


//use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

// escrow

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct FibonacciSequence {
    pub fib: [u8; 2],
}


impl FibonacciSequence {
    pub const LEN: usize = 2;
    pub const FIB_SEQUENCE: [u8; Self::LEN] = [255 as u8, 254 as u8];
}











pub struct DeprecatedEscrow {
    pub account_with_fibonacci_sequence: Pubkey,
}

impl DeprecatedEscrow {
    pub const LEN: usize = 32;
}
