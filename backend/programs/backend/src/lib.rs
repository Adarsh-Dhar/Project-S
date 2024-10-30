use anchor_lang::prelude::*;

declare_id!("4BHxBawaxpmPGv4Ehb81wUg72fixS2T3VMTbfZ7HM5bu");

#[program]
pub mod backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
