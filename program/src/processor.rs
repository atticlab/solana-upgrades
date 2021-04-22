//! Program state processor

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    pubkey::Pubkey,
};


/// Program state handler.
pub struct Processor {}

impl Processor {

    /// Processes an legacy_instruction
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _input: &[u8],
    ) -> ProgramResult {
        Ok(())
    }
}
