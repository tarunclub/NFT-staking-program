use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    entrypoint,
    entrypoint::ProgramResult,
};
use crate::processor;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, instruction_data)?;
    Ok(())
}