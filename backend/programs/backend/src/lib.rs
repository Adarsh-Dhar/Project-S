use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

#[program]
pub mod backend {
    use super::*;

    #[cfg(not(feature = "program"))]
    pub fn lend(ctx: Context<Lend>, amount: u64, token: Pubkey) -> ProgramResult {
        let pool = &mut ctx.accounts.pool;
        pool.token = token;
        pool.amount = amount;
        pool.lender = ctx.accounts.lender.key();
        Ok(())
    }

    #[cfg(not(feature = "program"))]
    pub fn borrow(ctx: Context<Lend>, amount: u64, token: Pubkey) -> ProgramResult {
        let pool = &mut ctx.accounts.pool;
        pool.token = token;
        pool.amount = amount;
        pool.lender = ctx.accounts.lender.key();
        Ok(())
    }

    #[cfg(not(feature = "program"))]
    pub fn repay(ctx: Context<Lend>, amount: u64, token: Pubkey) -> ProgramResult {
        let pool = &mut ctx.accounts.pool;
        pool.token = token;
        pool.amount = amount;
        pool.lender = ctx.accounts.lender.key();
        Ok(())
    }

    #[cfg(not(feature = "program"))]
    pub fn withdraw(ctx: Context<Lend>, amount: u64, token: Pubkey) -> ProgramResult {
        let pool = &mut ctx.accounts.pool;
        pool.token = token;
        pool.amount = amount;
        pool.lender = ctx.accounts.lender.key();
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Lend<'info> {
    #[account(init, payer=user, space=5000, seeds=[b"bankaccount", user.key().as_ref()], bump)]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(init, payer=user, space=5000, seeds=[b"bankaccount", user.key().as_ref()], bump)]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(init, payer=user, space=5000, seeds=[b"bankaccount", user.key().as_ref()], bump)]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(init, payer=user, space=5000, seeds=[b"bankaccount", user.key().as_ref()], bump)]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub name: String,
    pub balance: u64,
    pub owner: Pubkey,
}