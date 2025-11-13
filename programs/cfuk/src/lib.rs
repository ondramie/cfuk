use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("Fou1bcLSpMZfbNkjZC6b6qyPRp9477z9knhBHL1i5jSx");

#[program]
pub mod cfuk {
    use super::*;

    // Initial settings for distribution
    const CAMP_ALLOCATION: u64 = 700_000_000; // 70%
    const TEAM_ALLOCATION: u64 = 200_000_000; // 20%
    const TREASURY_ALLOCATION: u64 = 100_000_000; // 10%

    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            9,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()),
        )
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        payer = mint_authority,
        mint::decimals = 9,
        mint::authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
