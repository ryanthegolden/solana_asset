use anchor_lang::prelude::*;

// Import modules
pub mod instructions;
pub mod state;

// Re-export for easier access
pub use instructions::*;
pub use state::*;

declare_id!("CsgfQjdQEoVHUFUfbzjqoetNFWjemSNKr4Wd7mJpeNaH");

#[program]
pub mod asset {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    /// Tạo token mới với mint account và metadata
    pub fn create_token(
        ctx: Context<CreateToken>,
        params: TokenInitParams,
    ) -> Result<()> {
        instructions::create_token(ctx, params)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
