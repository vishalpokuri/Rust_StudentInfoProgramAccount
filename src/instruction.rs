use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum StudentInfo {
    Introduction { name: String, message: String },
}

#[derive(BorshDeserialize)]
struct StudentInfoDeserialiser {
    name: String,
    message: String,
}

impl StudentInfo {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidAccountData)?;

        let payload = StudentInfoDeserialiser::try_from_slice(&rest)?;
        Ok(match variant {
            0 => StudentInfo::Introduction {
                name: payload.name,
                message: payload.message,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
