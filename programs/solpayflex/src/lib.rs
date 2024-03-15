use anchor_lang::prelude::*;

declare_id!("vjVYwt9ssMraEBQhbzfNr4bmdLvNN87inq4NSN9ai9J");

#[program]
pub mod solpayflex {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    #[account]
    pub struct solpayflex {
        pub buyer: Pubkey,
        pub seller: Pubkey,
        pub item: Pubkey,
        pub amount: u64,
        pub paid: u64, // amount paid at the moment
        pub created_at: u64,
    }

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> ProgramResult {
        let solpayflex = ctx.
    }
}

#[derive(Accounts)]
pub struct Initialize {}
