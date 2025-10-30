use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, MintTo, Transfer, Burn},
};

use crate::state::*;

// Token instruction implementations will go here
// - create_token
// - mint_tokens  
// - transfer_tokens
// - burn_tokens