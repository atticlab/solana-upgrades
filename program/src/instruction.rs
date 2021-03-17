//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction, InstructionError},
    pubkey::Pubkey,
};

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
pub struct V2ToV1UpgradeData {
    pub array: [u8; 64],
    pub key_2: Pubkey,
}

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum UpgradeInstruction {
    /// Initializes account with data.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The account to initialize.
    InitV1(InitArgsV1),

    /// Gets account and validates it is proper V1 account (version and size)
    ///
    /// Accounts expected by this instruction:
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
    /// Accounts expected by this instruction:
    /// 1. [writable] initialized original account of v1 version
    /// 2. [writable] next version account not yet initialized
    /// 3. [signer]
    UpgradeV1ToV2(V2ToV1UpgradeData),
}

impl UpgradeInstruction {
    pub fn initialize_v1(
        program_id: &Pubkey,
        account: &Pubkey,
        data: InitArgsV1,
        signer: &Pubkey,
    ) -> Result<Instruction, InstructionError> {
        let data = UpgradeInstruction::InitV1(data);
        let accounts = vec![
            AccountMeta::new(*account, false),
            AccountMeta::new_readonly(*signer, true),
        ];

        Ok(Instruction::new_with_borsh(*program_id, &data, accounts))
    }

    pub fn initialize_v2(
        program_id: &Pubkey,
        account: &Pubkey,
        data: InitArgsV2,
        signer: &Pubkey,
    ) -> Result<Instruction, InstructionError> {
        let data = UpgradeInstruction::InitV2(data);
        let accounts = vec![
            AccountMeta::new(*account, false),
            AccountMeta::new_readonly(*signer, true),
        ];
        Ok(Instruction::new_with_borsh(*program_id, &data, accounts))
    }

    pub fn use_v1(
        program_id: &Pubkey,
        account: &Pubkey,
        signer: &Pubkey,
    ) -> Result<Instruction, InstructionError> {
        let accounts = vec![
            AccountMeta::new(*account, false),
            AccountMeta::new_readonly(*signer, true),
        ];
        Ok(Instruction::new_with_borsh(
            *program_id,
            &UpgradeInstruction::UseV1,
            accounts,
        ))
    }

    pub fn use_v2(
        program_id: &Pubkey,
        account: &Pubkey,
        signer: &Pubkey,
    ) -> Result<Instruction, InstructionError> {
        let accounts = vec![
            AccountMeta::new(*account, false),
            AccountMeta::new_readonly(*signer, true),
        ];
        Ok(Instruction::new_with_borsh(
            *program_id,
            &UpgradeInstruction::UseV2,
            accounts,
        ))
    }

    pub fn upgrade_v1_to_v2(
        program_id: &Pubkey,
        old: &Pubkey,
        new: &Pubkey,
        data: V2ToV1UpgradeData,
        signer: &Pubkey,
    ) -> Result<Instruction, InstructionError> {
        let accounts = vec![
            AccountMeta::new(*old, false),
            AccountMeta::new(*new, false),
            AccountMeta::new_readonly(*signer, true),
        ];
        Ok(Instruction::new_with_borsh(
            *program_id,
            &UpgradeInstruction::UpgradeV1ToV2(data),
            accounts,
        ))
    }
}
