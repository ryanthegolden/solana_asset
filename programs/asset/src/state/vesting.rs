use anchor_lang::prelude::*;

/// Struct quản lý lịch trình vesting
#[account]
pub struct VestingSchedule {
    /// Người thụ hưởng
    pub beneficiary: Pubkey,
    
    /// Token mint
    pub token_mint: Pubkey,
    
    /// Tổng số token được vest
    pub total_amount: u64,
    
    /// Thời gian bắt đầu vest
    pub start_time: i64,
    
    /// Thời gian cliff (trước đó không thể claim)
    pub cliff_time: i64,
    
    /// Tổng thời gian vest
    pub duration: i64,
    
    /// Số token đã được release
    pub released_amount: u64,
    
    /// Authority có thể revoke
    pub authority: Pubkey,
    
    /// Có thể revoke hay không
    pub revocable: bool,
    
    /// Bump seed
    pub bump: u8,
}

impl VestingSchedule {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 32 + 1 + 1;
}