use anchor_lang::prelude::*;

/// Struct quản lý Staking Pool
#[account]
pub struct StakingPool {
    /// Authority của staking pool
    pub authority: Pubkey,
    
    /// Mint của token được stake
    pub token_mint: Pubkey,
    
    /// Mint của reward token
    pub reward_mint: Pubkey,
    
    /// Reward rate per second
    pub reward_rate: u64,
    
    /// Tổng số token đã được stake
    pub total_staked: u64,
    
    /// Bump seed
    pub bump: u8,
}

impl StakingPool {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 8 + 1;
}

/// Struct quản lý thông tin stake của user
#[account]
pub struct UserStake {
    /// Owner của stake
    pub owner: Pubkey,
    
    /// Staking pool address
    pub staking_pool: Pubkey,
    
    /// Số lượng token đã stake
    pub staked_amount: u64,
    
    /// Thời gian bắt đầu stake
    pub stake_start_time: i64,
    
    /// Reward đã claim cuối cùng
    pub last_claim_time: i64,
    
    /// Tổng reward đã claim
    pub total_claimed: u64,
    
    /// Bump seed
    pub bump: u8,
}

impl UserStake {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 1;
}