//! Example of how to version and upgrade account data

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("pKSLLSUCsqEsqURK9MhDopYZfij6GRWt7oPfC5DwtAq");

/// Current program version
pub const PROGRAM_VERSION: u8 = 1;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;
