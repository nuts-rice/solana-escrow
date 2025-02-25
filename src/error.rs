use thiserror::Error;


use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    //Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempty")]
    NotRentExempt,
}
//Implement our programs EscrowError to a ProgramError
impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
