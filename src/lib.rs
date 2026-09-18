use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Bassxf6opGq61Br6rBKDyfU3SCDwEdC7RN8gcZpRGpCB");

#[program]
pub mod nexora {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.nexora_account.balance = 0;
        msg!("Nexora initialized");
        Ok(())
    }

    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
    ) -> Result<()> {
        let cpi_accounts: Transfer = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi_ctx: CpiContext<Transfer> = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );

        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(
            init,
            payer = user,
            space = 8 + 8
        )]
        pub nexora_account: Account<'info, NexoraAccount>,

        #[account(mut)]
        pub user: Signer<'info>,

        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct TransferTokens<'info> {
        #[account(
            mut,
            constraint = from.owner == owner.key(),
            constraint = from.mint == to.mint,
        )]
        pub from: Account<'info, TokenAccount>,

        #[account(mut)]
        pub to: Account<'info, TokenAccount>,

        pub owner: Signer<'info>,

        pub token_program: Program<'info, Token>,
    }

    #[account]
    pub struct NexoraAccount {
        pub balance: u64,
    }
} 

