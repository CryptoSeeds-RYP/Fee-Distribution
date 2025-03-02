use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[program]
mod fee_distribution {
    use super::*;

    pub fn distribute_fees(ctx: Context<DistributeFees>, amount: u64) -> Result<()> {
        let holders_share = amount * 33 / 100;
        let stakers_share = amount * 34 / 100;
        let treasury_share = amount * 33 / 100;

        // Store holders' rewards for manual harvesting
        ctx.accounts.holders_rewards_account.amount += holders_share;

        // Store stakers' rewards for manual harvesting
        ctx.accounts.stakers_rewards_account.amount += stakers_share;

        // Transfer treasury share immediately
        token::transfer(
            ctx.accounts.treasury_transfer_ctx(),
            treasury_share,
        )?;

        Ok(())
    }

    pub fn harvest_rewards(ctx: Context<HarvestRewards>, user: Pubkey) -> Result<()> {
        let user_rewards = ctx.accounts.holders_rewards_account.amount;
        require!(user_rewards > 0, ErrorCode::NoRewardsAvailable);

        token::transfer(
            ctx.accounts.harvest_transfer_ctx(),
            user_rewards,
        )?;

        // Reset user's rewards balance after harvesting
        ctx.accounts.holders_rewards_account.amount = 0;

        Ok(())
    }

    pub fn expire_unclaimed_rewards(ctx: Context<ExpireRewards>) -> Result<()> {
        let current_timestamp = Clock::get()?.unix_timestamp;
        let one_year = 365 * 24 * 60 * 60;

        if current_timestamp - ctx.accounts.holders_rewards_account.last_updated > one_year {
            let expired_rewards = ctx.accounts.holders_rewards_account.amount;
            ctx.accounts.holders_rewards_account.amount = 0;
            
            // Redistribute expired rewards to treasury
            token::transfer(
                ctx.accounts.expired_rewards_transfer_ctx(),
                expired_rewards,
            )?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributeFees<'info> {
    #[account(mut)]
    pub source_wallet: Signer<'info>,

    #[account(mut)]
    pub holders_rewards_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stakers_rewards_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_wallet: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct HarvestRewards<'info> {
    #[account(mut)]
    pub user_wallet: Signer<'info>,
    #[account(mut)]
    pub holders_rewards_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ExpireRewards<'info> {
    #[account(mut)]
    pub holders_rewards_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
