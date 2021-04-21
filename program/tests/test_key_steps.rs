#![cfg(feature = "test-bpf")]

use solana_program::{pubkey::Pubkey, system_instruction};
use solana_upgrade::state::{DeprecatedEscrow, FibonacciSequence};
use solana_program_test::{processor, ProgramTest, tokio, ProgramTestContext};
use solana_sdk::signature::{Keypair, Signer};
use solana_program::instruction::Instruction;
use solana_sdk::rent::Rent;


#[tokio::test]
async fn test() {
    //
    let cluster = program_test().start_with_context().await;
    let payer = cluster.payer;
    let mut banks_client = cluster.banks_client;
    let account_keypair= Keypair::new();
    let rent = banks_client.get_rent().await.unwrap();
    let price_to_exist_for_two_years = rent.minimum_balance(FibonacciSequence::LEN as usize); // <= pay attention

    // написать прототип контракта, который хранит аккаунт, в котором тупо хранится набор байт
    let account = create_acc_system_ix(&payer, &account_keypair, price_to_exist_for_two_years);

    let escrow_info = DeprecatedEscrow {
        account_with_fibonacci_sequence: Default::default()
    };
}

pub fn create_acc_system_ix(payer: &Keypair, account: &Keypair, rent: u64) -> Instruction {
    system_instruction::create_account( // <- pay attention
                                        &payer.pubkey(),
                                        &account.pubkey(),
                                        rent,
                                        FibonacciSequence::LEN as u64,
                                        &solana_upgrade::id(),
    )
}

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "solana-upgrade",
        solana_upgrade::id(),
        processor!(solana_upgrade::processor::Processor::process_instruction),
    )
}
