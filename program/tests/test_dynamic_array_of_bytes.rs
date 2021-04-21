#![cfg(feature = "test-bpf")]

use solana_program::{pubkey::Pubkey, system_instruction};
use solana_program_test::{
    tokio,
    ProgramTest,
    processor,
};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_upgrade::{
    legacy_instruction::{self},
    legacy_state::*,
};
use legacy_instruction::legacy_instruction_structs::InitArgsV1;
use solana_upgrade::legacy_instruction::{initialize_fibonacci};
use solana_upgrade::state::FibonacciSequence;
use solana_upgrade::solana_program::instruction::Instruction;

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "solana-upgrade",
        solana_upgrade::id(),
        processor!(solana_upgrade::processor::Processor::process_instruction),
    )
}

#[tokio::test]
async fn upgrade_my_flow() {
    let v1_account = Keypair::new();
    let cluster = program_test().start_with_context().await;
    let payer = cluster.payer;
    let mut banks_client = cluster.banks_client;

    let rent = banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(StateV1::LEN as usize); // <= pay attention
    let transaction =
        create_transaction_with_state(&payer, v1_account, lamports, cluster.last_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();


}


fn create_transaction_with_state (
    payer: &Keypair,
    account: Keypair,
    lamports: u64,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let create_acc_sys_ix = create_acc_system_ix(&payer, &account, lamports);
    let initialize_acc_with_fib_sequence = create_fibonacci_ix(&payer, &account);
    let mut transaction = Transaction::new_with_payer(
        &[
            create_acc_sys_ix,
            initialize_acc_with_fib_sequence,
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, &account], recent_blockhash);
    transaction
}

fn create_fibonacci_ix(payer: &&Keypair, account: &Keypair) -> Instruction {
    let fib_seq_data = FibonacciSequence{fib:[1, 1]};

    initialize_fibonacci(
        &solana_upgrade::id(),
        fib_seq_data,
        &account.pubkey(),
        &payer.pubkey(),
    )
}

fn create_acc_system_ix(payer: &&Keypair, account: &Keypair, lamports: u64) -> Instruction {
    system_instruction::create_account( // <- pay attention
                                        &payer.pubkey(),
                                        &account.pubkey(),
                                        lamports,
                                        FibonacciSequence::LEN as u64,
                                        &solana_upgrade::id(),
    )
}
