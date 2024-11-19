use anchor_lang::prelude::*;

declare_id!("GL8LwbtRyXp52HjS2QgQJFyZY7SGda629oVon8cwU1iL");

pub mod instructions;
use instructions::*;

pub use whirlpool_cpi::state::{FeeTier, Whirlpool};

#[program]
pub mod whirlpool_quoter {

    use super::*;

    pub fn quote_exact_in(
        ctx: Context<QuoteExactInSingle>,
        amount: u64,
        slippage_bps: u16,
    ) -> Result<u64> {
        msg!("quote_exact_in {:?}, {:?}", amount, slippage_bps);
        msg!("whirlpool {:?}", ctx.accounts.whirlpool.key());
        msg!(
            "token_owner_account_a {:?}",
            ctx.accounts.token_owner_account_a.key()
        );
        msg!(
            "token_owner_account_b {:?}",
            ctx.accounts.token_owner_account_b.key()
        );
        Ok(1000)
    }
}
