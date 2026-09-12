use anchor_lang::prelude::*;

declare_id!("Bassxf6opGq61Br6BrKDyfU3SCdWEdC7RN8gcZpRGpCB");

#[program]
pub mod nexora {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
    ) -> Result<()> {
        let nexora = &mut ctx.accounts.nexora;
        
        nexora.name = name;
        nexora.authority = ctx.accounts.authority.key();

        Ok(())
    }

    pub fn update_name(
        ctx: Context<UpdateName>,
        new_name: String,
    ) -> Result<()> {
        let nexora = &mut ctx.accounts.nexora;

        nexora.name = new_name;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 64 + 32)]
    pub nexora: Account<'info, NexoraData>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateName<'info> {
    #[account(mut)]
    pub nexora: Account<'info, NexoraData>,
}

#[account]
pub struct NexoraData {
    pub name: String,
    pub authority: Pubkey,
}
