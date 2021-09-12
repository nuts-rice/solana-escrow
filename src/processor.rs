use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program::invoke, program_error::ProgramError, program_pack::{Pack, IsInitialized}, pubkey::Pubkey, sysvar::{rent::Rent, Sysvar}};

use crate::instruction::{error::EscrowError, instruction::EscrowInstruction, state::Escrow};

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_escrow(accounts, amount, program_id)
            }
        }
    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let temp_token_account = next_account_info(account_info_iter)?;

        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let escrow_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }

        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.data.borrow())?;
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        escrow_info.is_initialized = true;
        escrow_info.initializer_pubkey = *initializer.key;
        escrow_info.temp_token_account_pubkey = *temp_token_account.key;
        escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
        escrow_info.expected_amount = amount;

        Escrow::pack(escrow_info, &mut escrow_account.data.borrow_mut())?;
        //Create a Program Derived Address here by passing an array of seeds and the program_id
        //into the find_program_address function
        //Get back a new pda and bump_seed
        let (pda, bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

        //PDAs are public keys that are derived from the program_id and the seed
        //As well as having been pushed off the ed25519 curve by the bump seed (nonce)
        let token_program = next_account_info(account_info_iter)?;
        //set_authority is builder function to create instruction to transfer ownership
        let owner_change_ix = spl_token::instruction::set_authority(
            //Add a check here that the token program is really the account of the token program
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initalizer.key]
        )?;

        msg!("Calling the token program to transfer token account ownership...");
        //Include signed initalizer account here for CPI, the signature is extended to CPI
        invoke(
            //Because party A signed the InitEscrow transaction, program can make the token program set_authority CPI
            //and include her pubkey as a signer pubkey
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }
}
