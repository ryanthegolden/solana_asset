use anchor_lang::prelude::*;

/// Struct quản lý thông tin token
#[account]
pub struct TokenManager {
    /// Authority có quyền quản lý token (mint, freeze, etc.)
    pub authority: Pubkey,
    
    /// Mint address của token
    pub token_mint: Pubkey,
    
    /// Tên của token
    pub name: String,
    
    /// Symbol của token (ví dụ: "DOGE", "SHIB")
    pub symbol: String,
    
    /// URI chứa metadata của token (logo, description, etc.)
    pub uri: String,
    
    /// Số thập phân của token (thường là 6 hoặc 9)
    pub decimals: u8,
    
    /// Tổng supply tối đa có thể mint
    pub max_supply: u64,
    
    /// Số lượng token đã được mint
    pub current_supply: u64,
    
    /// Có thể mint thêm hay không
    pub is_mintable: bool,
    
    /// Có thể freeze account hay không
    pub is_freezable: bool,
    
    /// Token có thể burn hay không
    pub is_burnable: bool,
    
    /// Phí cho mỗi lần transfer (basis points, 100 = 1%)
    pub transfer_fee_basis_points: u16,
    
    /// Account nhận phí transfer
    pub fee_recipient: Option<Pubkey>,
    
    /// Thời gian tạo token
    pub created_at: i64,
    
    /// Bump seed cho PDA
    pub bump: u8,
}

impl TokenManager {
    /// Kích thước cần thiết cho account
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // token_mint
        4 + 32 + // name (max 32 chars)
        4 + 10 + // symbol (max 10 chars)
        4 + 200 + // uri (max 200 chars)
        1 + // decimals
        8 + // max_supply
        8 + // current_supply
        1 + // is_mintable
        1 + // is_freezable
        1 + // is_burnable
        2 + // transfer_fee_basis_points
        1 + 32 + // fee_recipient (Option<Pubkey>)
        8 + // created_at
        1; // bump
}

/// Struct cho việc khởi tạo TokenManager
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenInitParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub max_supply: u64,
    pub is_mintable: bool,
    pub is_freezable: bool,
    pub is_burnable: bool,
    pub transfer_fee_basis_points: u16,
    pub fee_recipient: Option<Pubkey>,
}

impl TokenInitParams {
    /// Validate input parameters
    pub fn validate(&self) -> Result<()> {
        require!(self.name.len() <= 32, TokenError::NameTooLong);
        require!(self.symbol.len() <= 10, TokenError::SymbolTooLong);
        require!(self.uri.len() <= 200, TokenError::UriTooLong);
        require!(self.decimals <= 9, TokenError::InvalidDecimals);
        require!(self.max_supply > 0, TokenError::InvalidMaxSupply);
        require!(
            self.transfer_fee_basis_points <= 10000, 
            TokenError::InvalidTransferFee
        );
        Ok(())
    }
}

/// Custom errors cho token operations
#[error_code]
pub enum TokenError {
    #[msg("Token name too long (max 32 characters)")]
    NameTooLong,
    
    #[msg("Token symbol too long (max 10 characters)")]
    SymbolTooLong,
    
    #[msg("URI too long (max 200 characters)")]
    UriTooLong,
    
    #[msg("Invalid decimals (max 9)")]
    InvalidDecimals,
    
    #[msg("Invalid max supply (must be > 0)")]
    InvalidMaxSupply,
    
    #[msg("Invalid transfer fee (max 100%)")]
    InvalidTransferFee,
    
    #[msg("Token is not mintable")]
    TokenNotMintable,
    
    #[msg("Exceeds max supply")]
    ExceedsMaxSupply,
    
    #[msg("Insufficient token supply")]
    InsufficientSupply,
    
    #[msg("Token is not burnable")]
    TokenNotBurnable,
    
    #[msg("Unauthorized")]
    Unauthorized,
}