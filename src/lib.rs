#![allow(warnings)]

use solana_program::{
    entrypoint,
    msg,
    pubkey::Pubkey,
    account_info::{
        AccountInfo,
        next_account_info,
    },
    program_error::ProgramError,
};

entrypoint!(process_instruction);

fn sha1(data: &[u8]) -> [u8; 20] {
    use sha1::{Sha1, Digest};
    let mut hasher = Sha1::new();

    // process input message
    hasher.update(data);

    hasher.finalize().into()
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let input1_len = usize::from(u16::from_le_bytes([instruction_data[0], instruction_data[1]]));
    let input2_len = usize::from(u16::from_le_bytes([instruction_data[2], instruction_data[3]]));
    let input1: &[u8] = &instruction_data[4..4+input1_len];
    let input2: &[u8] = &instruction_data[4+input1_len..4+input1_len+input2_len];

    if input1 == input2 {
        msg!("input was the same");
        return Err(ProgramError::InvalidArgument)?;
    }

    if sha1(input1) != sha1(input2) {
        msg!("input1: {:?}", input1);
        msg!("input2: {:?}", input2);
        msg!("input1 sha1: {:?}", sha1(input1));
        msg!("input2 sha1: {:?}", sha1(input2));
        msg!("sha1 did not match");
        return Err(ProgramError::InvalidArgument)?;
    }

    // pay out the bounty
    let mut account_info_iter = accounts.iter();
    let bounty_source: &AccountInfo = next_account_info(&mut account_info_iter)?;
    let bounty_dest: &AccountInfo = next_account_info(&mut account_info_iter)?;


    let initial_bounty_amount: u64 = bounty_source.try_lamports()?;
    let initial_bounty_dest_amount: u64 = bounty_dest.try_lamports()?;

    **bounty_dest.try_borrow_mut_lamports()? = initial_bounty_dest_amount
        .checked_add(initial_bounty_amount)
        .ok_or(ProgramError::InvalidArgument)?;
    **bounty_source.try_borrow_mut_lamports()? = 0;

    Ok(())
}
