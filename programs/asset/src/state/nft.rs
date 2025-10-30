use anchor_lang::prelude::*;

/// Struct quản lý NFT Collection
#[account]
pub struct NftCollection {
    /// Authority của collection
    pub authority: Pubkey,
    
    /// Collection mint
    pub collection_mint: Pubkey,
    
    /// Collection metadata account
    pub collection_metadata: Pubkey,
    
    /// Collection master edition
    pub collection_master_edition: Pubkey,
    
    /// Tổng supply tối đa
    pub total_supply: u32,
    
    /// Số NFT đã mint
    pub minted_count: u32,
    
    /// Collection name
    pub name: String,
    
    /// Collection symbol
    pub symbol: String,
    
    /// Base URI cho metadata
    pub base_uri: String,
    
    /// Royalty percentage (basis points)
    pub royalty_basis_points: u16,
    
    /// Creator address nhận royalty
    pub royalty_recipient: Pubkey,
    
    /// Bump seed
    pub bump: u8,
}

impl NftCollection {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 4 + 4 + 
        4 + 32 + 4 + 10 + 4 + 200 + 2 + 32 + 1;
}