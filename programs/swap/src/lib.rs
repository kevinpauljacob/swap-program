pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("XSXz1iFkpGjKXKbcfmKsN4Kcj6PWSH2pBfAs9p9GVFz");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>, id: u64, amount_offered: u64, amount_requested: u64, expiration: i64) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&ctx, amount_offered)?;
        instructions::make_offer::save_offer(ctx, id, amount_offered, amount_requested, expiration)
    }
}
