pub mod instruction;
use instruction::StudentInfo;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let instruction = StudentInfo::unpack(instruction_data)?;
    match instruction {
        StudentInfo::Introduction { name, message } => {
            add_student_info(program_id, accounts, name, message)
        }
    }
}
#[allow(unused)]
pub fn add_student_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    msg!("Hello everyone!!");
    msg!("My name is {}", name);
    msg!("Message: {}", message);

    Ok(())
}
