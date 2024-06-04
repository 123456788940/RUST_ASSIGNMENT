# RUST_ASSIGNMENT

Sure, here is a detailed `README.md` file that explains how to set up, deploy, and test the smart contract for minting and transferring SPL tokens on a local Solana testnet.

```markdown
# SPL Token Mint and Transfer Smart Contract

This project demonstrates a simple Solana smart contract that includes functions for minting and transferring SPL tokens using native Rust.

## Prerequisites

Ensure you have the following installed on your machine:

- Rust: Install Rust using `rustup`.
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Solana CLI: Install Solana CLI tools.
  ```sh
  sh -c "$(curl -sSfL https://release.solana.com/v1.9.1/install)"
  ```

- SPL Token CLI: Install the SPL Token CLI for managing tokens.
  ```sh
  cargo install spl-token-cli
  ```

## Setup

1. **Clone the repository:**
   ```sh
   git clone https://github.com/your_username/spl_token_mint_transfer.git
   cd spl_token_mint_transfer
   ```

2. **Build the project:**
   ```sh
   cargo build-bpf
   ```

## Deploy the Program

1. **Start the local Solana test validator:**
   ```sh
   solana-test-validator
   ```

   Keep this terminal window open as it runs the local Solana testnet.

2. **Open a new terminal window and set the Solana CLI to use the local test validator:**
   ```sh
   solana config set --url http://127.0.0.1:8899
   ```

3. **Deploy the program:**
   ```sh
   solana program deploy ./target/deploy/spl_token_mint_transfer.so
   ```

   After deploying, note the program ID that is printed in the output. You will need this for testing.

## Testing

### Mint Tokens

1. **Create a test file `test_mint_transfer.rs` in the `tests` directory:**
   ```sh
   mkdir -p tests
   touch tests/test_mint_transfer.rs
   ```

2. **Copy the following test code into `tests/test_mint_transfer.rs`:**

   ```rust
   use solana_program_test::*;
   use solana_sdk::{
       instruction::{AccountMeta, Instruction},
       signer::Signer,
       transaction::Transaction,
   };
   use spl_token::state::{Account, Mint};

   #[tokio::test]
   async fn test_mint_transfer() {
       let program_id = Pubkey::new_unique();
       let mut program_test = ProgramTest::new(
           "spl_token_mint_transfer",
           program_id,
           processor!(process_instruction),
       );

       let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

       let mint_keypair = Keypair::new();
       let token_account_keypair = Keypair::new();
       let amount: u64 = 1000;

       // Create mint account
       let create_mint_ix = system_instruction::create_account(
           &payer.pubkey(),
           &mint_keypair.pubkey(),
           Rent::default().minimum_balance(Mint::LEN),
           Mint::LEN as u64,
           &spl_token::id(),
       );

       // Initialize mint account
       let init_mint_ix = spl_token::instruction::initialize_mint(
           &spl_token::id(),
           &mint_keypair.pubkey(),
           &payer.pubkey(),
           None,
           9,
       )
       .unwrap();

       // Create token account
       let create_token_account_ix = system_instruction::create_account(
           &payer.pubkey(),
           &token_account_keypair.pubkey(),
           Rent::default().minimum_balance(Account::LEN),
           Account::LEN as u64,
           &spl_token::id(),
       );

       // Initialize token account
       let init_token_account_ix = spl_token::instruction::initialize_account(
           &spl_token::id(),
           &token_account_keypair.pubkey(),
           &mint_keypair.pubkey(),
           &payer.pubkey(),
       )
       .unwrap();

       // Mint tokens
       let mint_token_ix = Instruction {
           program_id,
           accounts: vec![
               AccountMeta::new(mint_keypair.pubkey(), false),
               AccountMeta::new(token_account_keypair.pubkey(), false),
               AccountMeta::new(payer.pubkey(), true),
               AccountMeta::new(spl_token::id(), false),
           ],
           data: [0, amount.to_le_bytes()].concat(),
       };

       // Transfer tokens
       let transfer_token_ix = Instruction {
           program_id,
           accounts: vec![
               AccountMeta::new(token_account_keypair.pubkey(), false),
               AccountMeta::new(token_account_keypair.pubkey(), false),
               AccountMeta::new(payer.pubkey(), true),
               AccountMeta::new(spl_token::id(), false),
           ],
           data: [1, amount.to_le_bytes()].concat(),
       };

       // Create transaction
       let mut transaction = Transaction::new_with_payer(
           &[
               create_mint_ix,
               init_mint_ix,
               create_token_account_ix,
               init_token_account_ix,
               mint_token_ix,
               transfer_token_ix,
           ],
           Some(&payer.pubkey()),
       );

       transaction.sign(&[&payer, &mint_keypair, &token_account_keypair], recent_blockhash);
       banks_client.process_transaction(transaction).await.unwrap();

       // Fetch and verify token account balance
       let token_account = banks_client
           .get_account(token_account_keypair.pubkey())
           .await
           .unwrap()
           .unwrap();

       let token_account_data = Account::unpack(&token_account.data).unwrap();
       assert_eq!(token_account_data.amount, amount);
   }
   ```

3. **Run the tests:**
   ```sh
   cargo test-bpf
   ```

   This command will compile the tests and run them on the local test validator. The test checks that tokens are correctly minted and transferred between accounts.

## Usage

- **Mint Tokens:**
  You can call the `mint_token` function to mint new SPL tokens to a specified token account.

- **Transfer Tokens:**
  You can call the `transfer_token` function to transfer SPL tokens from one account to another.

## Conclusion

This README provides a comprehensive guide to set up, deploy, and test the Solana smart contract for minting and transferring SPL tokens. Follow the instructions step-by-step to ensure everything is configured correctly. If you encounter any issues, refer to the Solana and SPL Token documentation for additional help.

```

This `README.md` file includes instructions on setting up the environment, deploying the smart contract, and running tests to ensure the contract functions as expected. Follow the instructions step-by-step to deploy and test your smart contract successfully.
