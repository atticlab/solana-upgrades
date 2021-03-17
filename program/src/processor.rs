//! Program state processor

use crate::{error::UpgradeError, instruction::*, state::*};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    pub fn process_initialize_v1(
        program_id: &Pubkey,
        account: &AccountInfo,
        input: InitArgsV1,
        signer: &AccountInfo,
    ) -> ProgramResult {
        validate_program(&program_id, &account)?;
        validate_signer(signer)?;

        let mut account_data = account.try_borrow_mut_data()?;
        if StateV1::LEN > account_data.len() {
            return Err(ProgramError::AccountDataTooSmall);
        }

        let mut state = StateV1::try_from_slice(&account_data)?;
        if state.state_version != StateVersion::Uninitialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        state.state_version = StateVersion::V1;
        state.num = input.num;
        state.num_2 = input.num_2;
        state.key = input.key;

        state.serialize(&mut account_data.as_mut())?;
        Ok(())
    }

    pub fn process_initialize_v2(
        program_id: &Pubkey,
        account: &AccountInfo,
        input: InitArgsV2,
        signer: &AccountInfo,
    ) -> ProgramResult {
        validate_program(&program_id, &account)?;
        validate_signer(signer)?;

        let mut account_data = account.try_borrow_mut_data()?;
        if StateV2::LEN > account_data.len() {
            return Err(ProgramError::AccountDataTooSmall);
        }

        let mut state = StateV2::try_from_slice(&account_data)?;
        if state.state_version != StateVersion::Uninitialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        state.state_version = StateVersion::V2;
        state.num = input.num;
        state.num_2 = input.num_2;
        state.key = input.key;
        state.key_2 = input.key_2;
        state.array = input.array;

        state.serialize(&mut account_data.as_mut())?;
        Ok(())
    }

    pub fn process_use_v1(program_id: &Pubkey, account: &AccountInfo) -> ProgramResult {
        validate_program(&program_id, &account)?;

        let account_data = account.try_borrow_data()?;
        if StateV1::LEN > account_data.len() {
            return Err(ProgramError::AccountDataTooSmall);
        }

        let state = StateV1::try_from_slice(&account_data)?;
        if state.state_version != StateVersion::V1 {
            return Err(UpgradeError::InvalidVersion.into());
        }

        Ok(())
    }

    pub fn process_use_v2(program_id: &Pubkey, account: &AccountInfo) -> ProgramResult {
        validate_program(&program_id, &account)?;

        let account_data = account.try_borrow_data()?;
        if StateV2::LEN > account_data.len() {
            return Err(ProgramError::AccountDataTooSmall);
        }

        let state = StateV2::try_from_slice(&account_data)?;
        if state.state_version != StateVersion::V2 {
            return Err(UpgradeError::InvalidVersion.into());
        }

        Ok(())
    }

    pub fn process_upgrade_v1_to_v2(
        program_id: &Pubkey,
        old: &AccountInfo,
        new: &AccountInfo,
        input: V2ToV1UpgradeData,
        signer: &AccountInfo,
    ) -> ProgramResult {
        validate_signer(&signer)?;
        validate_program(&program_id, &old)?;
        validate_program(&program_id, &new)?;

        let old_account_data = old.try_borrow_data()?;
        if StateV1::LEN > old_account_data.len() {
            return Err(ProgramError::AccountDataTooSmall);
        }

        let old_state = StateV1::try_from_slice(&old_account_data)?;
        if old_state.state_version != StateVersion::V1 {
            return Err(UpgradeError::InvalidVersion.into());
        }

        let mut new_account_data = new.try_borrow_mut_data()?;
        if StateV2::LEN > new_account_data.len() {
            return Err(ProgramError::AccountDataTooSmall);
        }

        let mut new_state = StateV2::try_from_slice(&new_account_data)?;
        if new_state.state_version != StateVersion::Uninitialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        new_state.state_version = StateVersion::V2;
        new_state.num = old_state.num;
        new_state.num_2 = old_state.num_2 as u64;
        new_state.key = old_state.key;
        new_state.key_2 = input.key_2;
        new_state.array = input.array;

        new_state.serialize(&mut new_account_data.as_mut())?;

        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = UpgradeInstruction::try_from_slice(input)
            .or(Err(ProgramError::InvalidInstructionData))?;
        match instruction {
            UpgradeInstruction::InitV1(data) => {
                if let [account, signer, ..] = accounts {
                    Self::process_initialize_v1(program_id, account, data, signer)
                } else {
                    Err(ProgramError::NotEnoughAccountKeys)
                }
            }
            UpgradeInstruction::InitV2(data) => {
                if let [account, signer, ..] = accounts {
                    Self::process_initialize_v2(program_id, account, data, signer)
                } else {
                    Err(ProgramError::NotEnoughAccountKeys)
                }
            }
            UpgradeInstruction::UseV1 => {
                if let [account, ..] = accounts {
                    Self::process_use_v1(program_id, account)
                } else {
                    Err(ProgramError::NotEnoughAccountKeys)
                }
            }
            UpgradeInstruction::UseV2 => {
                if let [account, ..] = accounts {
                    Self::process_use_v2(program_id, account)
                } else {
                    Err(ProgramError::NotEnoughAccountKeys)
                }
            }
            UpgradeInstruction::UpgradeV1ToV2(data) => {
                if let [old, new, signer, ..] = accounts {
                    Self::process_upgrade_v1_to_v2(program_id, old, new, data, signer)
                } else {
                    Err(ProgramError::NotEnoughAccountKeys)
                }
            }
        }
    }
}

fn validate_program(program_id: &Pubkey, account: &AccountInfo) -> ProgramResult {
    if program_id != account.owner {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}

fn validate_signer(account: &AccountInfo) -> ProgramResult {
    if !account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Ok(())
}
