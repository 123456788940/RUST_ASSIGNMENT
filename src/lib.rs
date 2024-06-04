use solana_program::program_pack::Pack;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::{
    instruction::{mint_to, transfer},
    state::{Mint, Account},
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction_byte, rest) = instruction_data.split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match instruction_byte {
        0 => mint_token(program_id, accounts, rest),
        1 => transfer_token(program_id, accounts, rest),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn mint_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mint_info = next_account_info(accounts_iter)?;
    let to_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;
    let token_program_info = next_account_info(accounts_iter)?;

    let mint = Mint::unpack(&mint_info.data.borrow())?;
    let amount = instruction_data
        .get(..8)
        .ok_or(ProgramError::InvalidInstructionData)?
        .clone();

    let mint_to_ix = mint_to(
        token_program_info.key,
        mint_info.key,
        to_info.key,
        authority_info.key,
        &[],
        u64::from_le_bytes(amount.try_into().unwrap()),
    )?;
    solana_program::program::invoke(
        &mint_to_ix,
        &[
            mint_info.clone(),
            to_info.clone(),
            authority_info.clone(),
            token_program_info.clone(),
        ],
    )?;

    Ok(())
}

fn transfer_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let from_info = next_account_info(accounts_iter)?;
    let to_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;
    let token_program_info = next_account_info(accounts_iter)?;

    let amount = instruction_data
        .get(..8)
        .ok_or(ProgramError::InvalidInstructionData)?
        .clone();

    let transfer_ix = transfer(
        token_program_info.key,
        from_info.key,
        to_info.key,
        authority_info.key,
        &[],
        u64::from_le_bytes(amount.try_into().unwrap()),
    )?;
    solana_program::program::invoke(
        &transfer_ix,
        &[
            from_info.clone(),
            to_info.clone(),
            authority_info.clone(),
            token_program_info.clone(),
        ],
    )?;

    Ok(())
}
