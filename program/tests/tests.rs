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
use legacy_instruction::legacy_instructions_implementation::*;
use solana_upgrade::legacy_instruction::legacy_instructions_implementation;
use solana_upgrade::legacy_instruction::legacy_instruction_structs::{InitArgsV1, InitArgsV2};

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "solana-upgrade",
        solana_upgrade::id(),
        processor!(solana_upgrade::processor::Processor::process_instruction),
    )
}


#[tokio::test]
async fn upgrade_flow() {
    let v1_account = Keypair::new();
    let old = v1_account.pubkey();
    let v2_account = Keypair::new();
    let new = v2_account.pubkey();
    let cluster = program_test().start_with_context().await;
    let payer = cluster.payer;
    let mut banks_client = cluster.banks_client;

    //// v1
    let data = InitArgsV1 {
        key: Pubkey::new_unique(),
        num: 33,
        num_2: 666,
    };
    let rent = banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(StateV1::LEN as usize); // <= pay attention
    let transaction =
        create_v1_transaction(&payer, v1_account, lamports, data, cluster.last_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    //// v2
    let data = InitArgsV2 {
        key: Pubkey::new_unique(),
        num: 33,
        num_2: 666,
        array: [0; 64],
        key_2: Pubkey::new_unique(),
    };
    let rent = banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(StateV2::LEN as usize);
    let transaction =
        create_v2_transaction(&payer, v2_account, lamports, data, cluster.last_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // check version and states of different accounts
    let transaction = create_use_transaction(&payer, old, new, cluster.last_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // upgrade v1 to v2 via copy into fresh account
    let data = legacy_instruction::legacy_instruction_structs::V1ToV2UpgradeData {
        key_2: Pubkey::new_unique(),
        array: [42; 64],
    };

    let lamports = rent.minimum_balance(StateV2::LEN as usize);

    let (transaction, _new_account) =
        create_upgrade_transaction(&payer, old, data, lamports, cluster.last_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}



// obsolete \/
fn create_v1_transaction(
    payer: &Keypair,
    account: Keypair,
    lamports: u64,
    data: InitArgsV1,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account( // <- pay attention
                &payer.pubkey(),
                &account.pubkey(),
                lamports,
                StateV1::LEN as u64,
                &solana_upgrade::id(),
            ),
            legacy_instructions_implementation::initialize_v1(
                &solana_upgrade::id(),
                &account.pubkey(),
                data,
                &payer.pubkey(),
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, &account], recent_blockhash);
    transaction
}


fn create_v2_transaction(
    payer: &Keypair,
    account: Keypair,
    lamports: u64,
    data: InitArgsV2,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &account.pubkey(),
                lamports,
                StateV2::LEN as u64,
                &solana_upgrade::id(),
            ),
            initialize_v2(
                &solana_upgrade::id(),
                &account.pubkey(),
                data,
                &payer.pubkey(),
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, &account], recent_blockhash);
    transaction
}

fn create_use_transaction(
    payer: &Keypair,
    old: Pubkey,
    new: Pubkey,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[
            use_v1(&solana_upgrade::id(), &old, &payer.pubkey()).unwrap(),
            use_v2(&solana_upgrade::id(), &new, &payer.pubkey()).unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], recent_blockhash);
    transaction
}

fn create_upgrade_transaction(
    payer: &Keypair,
    old: Pubkey,
    new_difference: legacy_instruction::legacy_instruction_structs::V1ToV2UpgradeData,
    lamports: u64,
    recent_blockhash: solana_program::hash::Hash,
) -> (Transaction, Pubkey) {
    let new = Keypair::new();
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &new.pubkey(),
                lamports,
                StateV2::LEN as u64,
                &solana_upgrade::id(),
            ),
            new_ix_upgrade_v1_to_v2(
                &solana_upgrade::id(),
                &old,
                &new.pubkey(),
                new_difference,
                &payer.pubkey(),
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, &new], recent_blockhash);
    (transaction, new.pubkey())
}

