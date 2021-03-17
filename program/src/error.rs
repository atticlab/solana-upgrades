//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the upgrade program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum UpgradeError {
    #[error("Invalid data version")]
    InvalidVersion,
}
impl From<UpgradeError> for ProgramError {
    fn from(e: UpgradeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for UpgradeError {
    fn type_of() -> &'static str {
        "UpgradeError"
    }
}

impl PrintProgramError for UpgradeError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        let error = self.to_string();
        msg!(&error[..]);
    }
}
