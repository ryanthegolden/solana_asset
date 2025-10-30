use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, SetAuthority},
};

use crate::state::*;

/// Tạo token mới với mint account và metadata
pub fn create_token(
    ctx: Context<CreateToken>,
    params: TokenInitParams,
) -> Result<()> {
    // Validate input parameters
    params.validate()?;

    let token_manager = &mut ctx.accounts.token_manager;
    let clock = Clock::get()?;

    // Khởi tạo TokenManager với thông tin từ params
    token_manager.authority = ctx.accounts.authority.key();
    token_manager.token_mint = ctx.accounts.token_mint.key();
    token_manager.name = params.name.clone();
    token_manager.symbol = params.symbol.clone();
    token_manager.uri = params.uri;
    token_manager.decimals = params.decimals;
    token_manager.max_supply = params.max_supply;
    token_manager.current_supply = 0;
    token_manager.is_mintable = params.is_mintable;
    token_manager.is_freezable = params.is_freezable;
    token_manager.is_burnable = params.is_burnable;
    token_manager.transfer_fee_basis_points = params.transfer_fee_basis_points;
    token_manager.fee_recipient = params.fee_recipient;
    token_manager.created_at = clock.unix_timestamp;
    token_manager.bump = ctx.bumps.token_manager;

    // Nếu token không thể freeze, xóa freeze authority
    if !params.is_freezable {
        let token_mint_key = ctx.accounts.token_mint.key();
        let seeds = &[
            b"token_manager",
            token_mint_key.as_ref(),
            &[token_manager.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = SetAuthority {
            current_authority: token_manager.to_account_info(),
            account_or_mint: ctx.accounts.token_mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        
        anchor_spl::token::set_authority(
            cpi_ctx,
            anchor_spl::token::spl_token::instruction::AuthorityType::FreezeAccount,
            None,
        )?;
    }

    msg!(
        "Token created successfully: {} ({})", 
        token_manager.name, 
        token_manager.symbol
    );
    msg!("Token mint: {}", token_manager.token_mint);
    msg!("Decimals: {}", token_manager.decimals);
    msg!("Max supply: {}", token_manager.max_supply);
    msg!("Is freezable: {}", token_manager.is_freezable);

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: TokenInitParams)]
pub struct CreateToken<'info> {
    /// Authority tạo và quản lý token
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Token Manager PDA để lưu trữ metadata
    #[account(
        init,
        payer = authority,
        space = TokenManager::LEN,
        seeds = [b"token_manager", token_mint.key().as_ref()],
        bump
    )]
    pub token_manager: Account<'info, TokenManager>,

    /// Mint account cho token
    #[account(
        init,
        payer = authority,
        mint::decimals = params.decimals,
        mint::authority = token_manager,
        mint::freeze_authority = token_manager,
    )]
    pub token_mint: Account<'info, Mint>,

    /// Token program
    pub token_program: Program<'info, Token>,
    
    /// System program
    pub system_program: Program<'info, System>,
    
    /// Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}