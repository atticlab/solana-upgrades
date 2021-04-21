//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use crate::legacy_instruction::legacy_instruction_structs::{InitArgsV1, InitArgsV2, V1ToV2UpgradeData};
use crate::state::FibonacciSequence;
//use crate::state::FibonacciSequence;

pub mod legacy_instruction_structs;
pub mod     legacy_instructions_implementation;

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum UpgradeInstruction {
    /// Initializes account with data.
    ///
    /// Accounts expected by this legacy_instruction:
    ///
    ///   0. `[writable]` The account to initialize.
    InitV1(InitArgsV1),

    /// Gets account and validates it is proper V1 account (version and size)
    ///
    /// Accounts expected by this legacy_instruction:
    ///
    ///   0. `[]` The account to validate.    
    UseV1,

    ///////// after upgrade of program ///////
    /// Initializes V2 account
    InitV2(InitArgsV2),

    /// Use V2 account
    UseV2,

    /// Updates account from V1 to V2.
    /// Reads old account and initializes next version account.
    /// Accounts expected by this legacy_instruction:
    /// 1. [writable] initialized original account of v1 version
    /// 2. [writable] next version account not yet initialized
    /// 3. [signer]
    UpgradeV1ToV2(V1ToV2UpgradeData),

    /// Updates account from V1 to V2.
    /// 1. [read-only] [u8]
    InitFibonacci(FibonacciSequence),
}

pub fn initialize_fibonacci(program_id: &Pubkey, fib_seq: FibonacciSequence, account: &Pubkey, signer: &Pubkey) -> Instruction  {
    let initialized_fib_seq = UpgradeInstruction::InitFibonacci(fib_seq);
    let accounts = vec![
        AccountMeta::new(*account, false),
        AccountMeta::new_readonly(*signer, true),
    ];
    Instruction::new_with_borsh(*program_id, &initialized_fib_seq, accounts)
}

