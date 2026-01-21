use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use solana_security_txt::security_txt;

declare_id!("Fou1bcLSpMZfbNkjZC6b6qyPRp9477z9knhBHL1i5jSx");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "CFUK Token",
    project_url: "https://github.com/ondramie/cfuk",
    contacts: "email:m.e.ondras@proton.me",
    policy: "https://github.com/ondramie/cfuk/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/ondramie/cfuk"
}

// Token distribution (1 billion total supply)
pub const DECIMALS: u8 = 9;
pub const CAMP_ALLOCATION: u64 = 700_000_000_000_000_000; // 700M tokens (70%)
pub const TEAM_ALLOCATION: u64 = 200_000_000_000_000_000; // 200M tokens (20%)
pub const TREASURY_ALLOCATION: u64 = 100_000_000_000_000_000; // 100M tokens (10%)

#[program]
pub mod cfuk {
    use super::*;

    pub fn initialize_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        // Mint is initialized by Anchor's #[account(init, mint::decimals, mint::authority)] constraint
        Ok(())
    }

    pub fn distribute(ctx: Context<Distribute>) -> Result<()> {
        // Mint to camp wallet (70%)
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.camp_token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            CAMP_ALLOCATION,
        )?;

        // Mint to team wallet (20%)
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.team_token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            TEAM_ALLOCATION,
        )?;

        // Mint to treasury wallet (10%)
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.treasury_token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            TREASURY_ALLOCATION,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        payer = mint_authority,
        mint::decimals = DECIMALS,
        mint::authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub camp_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub team_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
