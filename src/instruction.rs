use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {
    //Starts the trade by creating and populating escrow account and transferring ownership

    //Accounts expected:
    //
    //0. '[signer]' the acount of the person initializing escrow
    //1. '[writable]' temp token account that should be created prior to this instruction and owned by the initalizer
    //2. '[]' the initalizers token account for the token they will recieve *should* the trade go through
    //3. '[writable]' the escrow account, hold all the necessary info about the trade
    //4. '[]' the rent sysvar
    //5. '[]' the token program

    InitEscrow {
        //Amount party A expects to recieve of token Y
        amount: u64
    }
}

impl EscrowInstruction {
    //unpacks a byte buffer into a [EscrowInstruction] enum.EscrowInstruction.html
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let(tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {

            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _  => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            //Memory safety closure pattern here
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
