#![cfg(feature = "test-bpf")]

use solana_program::{pubkey::Pubkey, system_instruction};
use solana_program_test::{processor, ProgramTest, tokio, BanksClient};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use solana_program::instruction::Instruction;

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
    let rent_cost = rent.minimum_balance(DummyState::LEN as usize);

    create_and_process_transaction_with_small_allocated_space_size(&v1_account, cluster.last_blockhash, &payer, &mut banks_client, rent_cost).await;
    assert_default_fibonacci_sequence_length_is_two(&v1_account, &mut banks_client).await;
    increase_fibonacci_sequence_locally(&v1_account, &mut banks_client).await;
    assert_global_fibonacci_sequence_has_not_changed(&v1_account, &mut banks_client).await;
    create_and_process_transaction_with_small_allocated_space_size(&v1_account, cluster.last_blockhash, &payer, &mut banks_client, rent_cost).await;
    expect_failing_transaction_when_increased_space(&v1_account, cluster.last_blockhash, &payer, &mut banks_client, rent_cost).await;
}

async fn create_and_process_transaction_with_small_allocated_space_size(v1_account: &Keypair, recent_blockhash: solana_program::hash::Hash, payer: &Keypair, banks_client: &mut BanksClient, rent_cost: u64) {
    let transaction2 = create_transaction_with_space(&payer, &v1_account, rent_cost, recent_blockhash,
                                                     2);
    let tx_result = banks_client.process_transaction(transaction2).await;
    tx_result.unwrap();
}

async fn expect_failing_transaction_when_increased_space(v1_account: &Keypair, recent_blockhash: solana_program::hash::Hash, payer: &Keypair, banks_client: &mut BanksClient, lamports: u64) {
    let transaction2 = create_transaction_with_space(&payer, &v1_account, lamports, recent_blockhash,
                                                     3);

    let tx_result = banks_client.process_transaction(transaction2).await;
    // As expected transaction failes Transaction failed custom_transaction_error 0x0
    // It is not possible to process_transaction when space is increased
    tx_result.unwrap_err();
}

async fn assert_default_fibonacci_sequence_length_is_two(v1_account: &Keypair, banks_client: &mut BanksClient) {
    let acc_option = banks_client.get_account(v1_account.pubkey()).await.unwrap();
    let acc = acc_option.unwrap();
    assert_eq!(2, acc.data.len());
    assert_eq!(vec![0, 0], acc.data);

}

async fn increase_fibonacci_sequence_locally(v1_account: &Keypair, banks_client: &mut BanksClient) {
    {
        // get account and verify that it holds fibonacci
        let account_option = banks_client.get_account(v1_account.pubkey()).await.unwrap();
        let mut acc_after_tx = account_option.unwrap();

        assert_eq!(2, acc_after_tx.data.len());

        // I can set data to four elements but space=2
        // how is that possible?
        // Answer: it is modified in user space, not internal Solana space
        let fibonacci: Vec<u8> = vec![1, 1, 2, 3];
        acc_after_tx.data = fibonacci;
        assert_eq!(4, acc_after_tx.data.len());
    }
}

async fn assert_global_fibonacci_sequence_has_not_changed(v1_account: &Keypair, banks_client: &mut BanksClient) {
    {
        let account_option = banks_client.get_account(v1_account.pubkey()).await.unwrap();
        let acc_after_fibonacci_seq_was_modified = account_option.unwrap();
        // as you can see even though we set acc data to [1, 1, 2, 4], its length is still 2
        assert_eq!(2, acc_after_fibonacci_seq_was_modified.data.len());
    }
}


fn create_transaction_with_space(
    payer: &Keypair,
    account: &Keypair,
    lamports: u64,
    recent_blockhash: solana_program::hash::Hash,
    space: u64,
) -> Transaction {
    let create_acc_sys_ix = create_acc_system_ix(&payer, &account, lamports, space);
    let mut transaction = Transaction::new_with_payer(
        &[
            create_acc_sys_ix,
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, &account], recent_blockhash);
    transaction
}

pub fn create_acc_system_ix(payer: &&Keypair, account: &Keypair, rent: u64, space: u64) -> Instruction {
    system_instruction::create_account( // <- pay attention
                                        &payer.pubkey(),
                                        &account.pubkey(),
                                        rent,
                                        space,
                                        &solana_upgrade::id(),
    )
}


pub struct DummyState {
    pub state_version: u8,
    pub num: u32,
    pub num_2: u16,
    pub key: Pubkey,
}



impl DummyState {
    pub const LEN: usize = 1 + 4 + 2 + 32;
}