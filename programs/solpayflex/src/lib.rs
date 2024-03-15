use anchor_lang::prelude::*;

declare_id!("vjVYwt9ssMraEBQhbzfNr4bmdLvNN87inq4NSN9ai9J");

#[program]
pub mod solpayflex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
