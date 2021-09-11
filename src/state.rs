use solana_program::pubkey::Pubkey;

use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_refs, array_refs, muts_array_refs};

//Sate file is responsible for:
//1) defining state objects that the processor can use
//2) serializing and deserializing such objects from and into arrays of u8

pub struct Escrow {
//is_initialiezed determines whether a given account escrow account is already in use
    pub is_initialized: bool,
//Save temp_token_account_pubkey so that when party B takes the trade the escrow program
//can send tokens from the account at temp_token_account_pubkey to party B account
//We save temp_token_account_pubkey here so that party B doesn't pass in
    pub initializer_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
//It's the programs responsibilty to check that recieved accounts == expected accounts
    pub initializer_token_to_receive_account_pubkey: Pubkey,
    pub expected_amount: u64,
}
//Implementing Sealed, which is Solana's Sized trait
impl Sealed for Escrow{}

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }}

impl Pack for Escrow {
    //Defining LEN: size of our type
    //length of the struct by adding the sizes of individual data types
    const LEN: usize = 105;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Escrow::LEN];
        let (
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_recieve_account_pubkey,
            expected_amount,
        ) = array_refs![src, 1, 32, 32, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        //Ok result for Escrow
        Ok(Escrow {
            is_initialized,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(*initializer_token_to_recieve_account_pubkey),
            expected_amount: u64::from_le_bytes(*expected_amount),
        })


    }

    //Pass in &self here after unpack_from_slice
    //unpack_from_slice was a static constructor returning a new instance of an escrow struct
    //Here, we pack that instance of Escrow, then serde into the dst slice
    //Basically decoded from unpack, encode with pack. ez :)
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Escrow::LEN];
        let (
            is_initialized_dst,
            initializer_pubkey_dst,
            temp_token_account_pubkey_dst,
            initializer_token_to_receive_account_pubkey_dst,
            expected_amount_dst,
        ) = muts_array_refs![dst, 1, 32, 32, 32, 8];

        let Escrow (
            is_initalized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_recieve_account_pubkey,
            expected_amount,
        ) = self;

        is_initialized_dst[0] = *is_initalized as u8;
            initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
            temp_token_account_pubkey_dst.copy_from_slice(initializer_token_to_recieve_account_pubkey.as_ref());
            initializer_token_to_receive_account_pubkey_dst.copy_from_slice(initializer_token_to_recieve_account_pubkey.as_ref());
            *expected_amount_dst = expected_amount.to_le_bytes();
        }
}
