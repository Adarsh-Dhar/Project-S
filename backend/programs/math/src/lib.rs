use anchor_lang::prelude::*;

declare_id!("APSRuc2SBwPNwfLE9wqHCowvZnjQnt2CMF8jvGw5hzwi");

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
