use core::result::Result;
use core::result::Result::Ok;
use solana_program::instruction::{AccountMeta, Instruction, InstructionError};
use solana_program::pubkey::Pubkey;
use crate::legacy_instruction::UpgradeInstruction;
use crate::legacy_instruction::legacy_instruction_structs::{InitArgsV1, InitArgsV2, V1ToV2UpgradeData};
//use solana_upgrade::legacy_instruction::{InitArgsV1, InitArgsV2, UpgradeInstruction, V1ToV2UpgradeData};

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

pub fn new_ix_upgrade_v1_to_v2(
    program_id: &Pubkey,
    old: &Pubkey,
    new: &Pubkey,
    difference: V1ToV2UpgradeData,
    signer: &Pubkey,
) -> Result<Instruction, InstructionError> {
    let accounts = vec![
        AccountMeta::new(*old, false),
        AccountMeta::new(*new, false),
        AccountMeta::new_readonly(*signer, true),
    ];
    Ok(Instruction::new_with_borsh(
        *program_id,
        &UpgradeInstruction::UpgradeV1ToV2(difference),
        accounts,
    ))
}
