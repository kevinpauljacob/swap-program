use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The offer has already expired.")]
    OfferExpired,

    #[msg("The offer is no longer active.")]
    OfferNotActive,
}

