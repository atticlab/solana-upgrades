//#![cfg(feature = "test-bpf")]

use solana_program::{instruction::InstructionError, pubkey::Pubkey, system_instruction};
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::{Transaction, TransactionError},
    transport::TransportError,
};
use solana_upgrade::{instruction, state::Mint};

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "solana-upgrade",
        spl_nft_erc_721::id(),
        processor!(spl_nft_erc_721::processor::Processor::process_instruction),
    )
}

#[tokio::test]
async fn initialize_mint_ok() {
    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
    let mint_account = Keypair::new();
    let data = instruction::MintData::new("KC", "Kitty").unwrap();
    let rent = banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(Mint::LEN as usize);
    let transaction =
        create_mint_transaction(payer, mint_account, lamports, data, recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn initialize_mint_not_rent_exempt() {
    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
    let mint_account = Keypair::new();
    let data = instruction::MintData::new("KC", "Kitty").unwrap();
    let transaction = create_mint_transaction(payer, mint_account, 0, data, recent_blockhash);
    let result = banks_client
        .process_transaction(transaction)
        .await
        .err()
        .unwrap();
    assert!(matches!(
        result,
        TransportError::TransactionError(TransactionError::InstructionError(
            1,
            InstructionError::AccountNotRentExempt
        ))
    ));
}

fn create_mint_transaction(
    payer: Keypair,
    mint_account: Keypair,
    lamports: u64,
    data: instruction::MintData,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &mint_account.pubkey(),
                lamports,
                Mint::LEN,
                &spl_nft_erc_721::id(),
            ),
            instruction::initialize_mint(
                &spl_nft_erc_721::id(),
                &mint_account.pubkey(),
                data,
                &payer.pubkey(),
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &mint_account], recent_blockhash);
    transaction
}
