pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

use crate::error::ErrorCode;

declare_id!("XSXz1iFkpGjKXKbcfmKsN4Kcj6PWSH2pBfAs9p9GVFz");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>, id: u64, amount_offered: u64, amount_requested: u64, expiration: i64) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&ctx, amount_offered)?;
        instructions::make_offer::save_offer(ctx, id, amount_offered, amount_requested, expiration)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {

        let current_timestamp = Clock::get()?.unix_timestamp;
        require!(
            ctx.accounts.offer.expiration > current_timestamp,
            ErrorCode::OfferExpired
        );

        require!(
            &ctx.accounts.offer.is_active,
            ErrorCode::OfferNotActive
        );

        instructions::take_offer::send_wanted_tokens_to_maker(&ctx)?;
        instructions::take_offer::withdraw_and_close_vault(ctx)
    }

    pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
        instructions::cancel_offer::withdraw_and_close_vault(ctx)
    }
}
