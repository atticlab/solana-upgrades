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
    instruction::{self},
    state::*,
};

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

    //// v1
    let data = instruction::InitArgsV1 {
        key: Pubkey::new_unique(),
        num: 33,
        num_2: 666,
    };
    let rent = banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(StateV1::LEN as usize); // <= pay attention
    //let transaction =
        //create_transaction_with_state(&payer, v1_account, lamports, data, cluster.last_blockhash);
  //  banks_client.process_transaction(transaction).await.unwrap();


}

/*
fn create_transaction_with_state (
    payer: &Keypair,
    account: Keypair,
    lamports: u64,
    data: instruction::InitArgsV1,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let create_acc_sys_ix =
        system_instruction::create_account( // <- pay attention
      &payer.pubkey(),
      &account.pubkey(),
        lamports,
        StateWithBytes::LEN as u64,
&solana_upgrade::id(),
    );
  /*  let initialize_acc_with_fib_sequence = instruction::initialize_fibonacci(
        &solana_upgrade::id(),
        &account.pubkey(),
        data,
        &payer.pubkey(),
    );*/
   /* let mut transaction = Transaction::new_with_payer(
        &[
            create_acc_sys_ix,
            initialize_acc_with_fib_sequence.unwrap(),
        ],
        Some(&payer.pubkey()),
    );*/
    transaction.sign(&[payer, &account], recent_blockhash);
    transaction
}
*/