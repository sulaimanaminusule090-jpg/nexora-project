use anchor_lang::prelude::*;

declare_id!("YOUR_PROGRAM_ID");

#[program]
pub mod nexora {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Nexora Solana Program Initialized");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
