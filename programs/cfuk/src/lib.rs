use anchor_lang::prelude::*;

// public key
declare_id!("Fou1bcLSpMZfbNkjZC6b6qyPRp9477z9knhBHL1i5jSx");

#[program]
pub mod cfuk {
    use super::*;

    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        total_supply: u64,
    ) -> Result<()> {
        // Initialize the mint
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            9, // 9 decimals
            ctx.accounts.authority.key,
            Some(ctx.accounts.authority.key),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
