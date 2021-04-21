#![cfg(feature = "test-bpf")]

use solana_program::{pubkey::Pubkey, system_instruction};
use solana_program_test::{
    processor,
    ProgramTest,
    tokio,
};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use solana_upgrade::{
    legacy_state::*,
};
use solana_upgrade::solana_program::instruction::Instruction;
use solana_upgrade::state::FibonacciSequence;

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "solana-upgrade",
        solana_upgrade::id(),
        processor!(solana_upgrade::processor::Processor::process_instruction),
    )
}

#[tokio::test]
async fn test_dynamic_fibonacci_sequence() {
    let v1_account = Keypair::new();
    let cluster = program_test().start_with_context().await;
    let payer = cluster.payer;
    let mut banks_client = cluster.banks_client;

    let rent = banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(StateV1::LEN as usize); // <= pay attention
    let create_account = create_transaction_with_state(&payer, &v1_account, lamports, cluster.last_blockhash);
    banks_client.process_transaction(create_account).await.unwrap();

    {
        let acc_option = banks_client.get_account(v1_account.pubkey()).await.unwrap();
        let mut acc = acc_option.unwrap();
        let fibonacci: Vec<u8> = vec![1, 1];
        acc.data = fibonacci;
        assert_eq!(2, acc.data.len());
    }
    {
        // get account and verify that it holds fibonacci
        let account_option = banks_client.get_account(v1_account.pubkey()).await.unwrap();
        let mut acc_after_tx = account_option.unwrap();

        assert_eq!(2, acc_after_tx.data.len());
        let fibonacci: Vec<u8> = vec![1, 1, 2, 3];
        acc_after_tx.data = fibonacci;
        assert_eq!(4, acc_after_tx.data.len());
    }

    //let transaction2 = create_transaction_with_state2(&payer, &v1_account, lamports, cluster.last_blockhash);

//    banks_client.process_transaction(transaction2).await.unwrap();
}


fn create_transaction_with_state (
    payer: &Keypair,
    account: &Keypair,
    lamports: u64,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let create_acc_sys_ix = create_acc_system_ix(&payer, &account, lamports);
    let mut transaction = Transaction::new_with_payer(
        &[
            create_acc_sys_ix,
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, &account], recent_blockhash);
    transaction
}
fn create_transaction_with_state2 (
    payer: &Keypair,
    account: &Keypair,
    lamports: u64,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let create_acc_sys_ix = create_acc_system_ix(&payer, &account, lamports);
    let mut transaction = Transaction::new_with_payer(
        &[
            create_acc_sys_ix,
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, &account], recent_blockhash);
    transaction
}

pub fn create_acc_system_ix(payer: &&Keypair, account: &Keypair, rent: u64) -> Instruction {
    system_instruction::create_account( // <- pay attention
                                        &payer.pubkey(),
                                        &account.pubkey(),
                                        rent,
                                        FibonacciSequence::LEN as u64,
                                        &solana_upgrade::id(),
    )
}
