pub mod float_ops;
pub mod double_ops;

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let instruction_type = instruction_data[0];
    
    if instruction_data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let a_bytes: [u8; 4] = instruction_data[1..5].try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let b_bytes: [u8; 4] = instruction_data[5..9].try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    
    let a = f32::from_le_bytes(a_bytes);
    let b = f32::from_le_bytes(b_bytes);

    match instruction_type {
        0 => {
            // Add
            let result = float_ops::add_floats(a, b);
            msg!("Add: {} + {} = {}", a, b, result);
        }
        1 => {
            // Multiply
            let result = float_ops::multiply_floats(a, b);
            msg!("Multiply: {} * {} = {}", a, b, result);
        }
        2 => {
            // Divide
            match float_ops::divide_floats(a, b) {
                Ok(result) => {
                    msg!("Divide: {} / {} = {}", a, b, result);
                }
                Err(_) => {
                    return Err(ProgramError::InvalidArgument);
                }
            }
        }
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}