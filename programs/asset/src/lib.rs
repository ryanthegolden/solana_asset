use anchor_lang::prelude::*;

declare_id!("CsgfQjdQEoVHUFUfbzjqoetNFWjemSNKr4Wd7mJpeNaH");

#[program]
pub mod asset {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
