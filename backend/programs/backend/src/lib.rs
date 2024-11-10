use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

#[program]
pub mod backend {
    use super::*;

    #[cfg(not(feature = "program"))]
    pub fn create_account(ctx: Context<CreateAccount>) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.owner = ctx.accounts.owner.key();
        my_account.amount_borrowed = 0;
        my_account.collateral_deposited = 0;
        my_account.amount_lended = 0;
        Ok(())
    }



    #[cfg(not(feature = "program"))]
    pub fn lend(ctx: Context<Lend>, amount: u64) -> ProgramResult {
        let txn = anchor_lang::solana_program::system_instruction::transfer(ctx.accounts.lender.key(), ctx.accounts.pool.key(), amount);
        anchor_lang::solana_program::invoke(txn, &[ctx.accounts.lender.to_account_info(), ctx.accounts.pool.to_account_info()])?;
        let pool = &mut ctx.accounts.pool;

        pool.amount += amount;
        pool.lender = ctx.accounts.lender.key();

        let lender_account = &mut ctx.accounts.lender_account;
        lender_account.lender = ctx.accounts.lender.key();
        lender_account.amount_lended += amount;
        Ok(())
    }

    #[cfg(not(feature = "program"))]
    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> ProgramResult {
        let txn = anchor_lang::solana_program::system_instruction::transfer(ctx.accounts.pool.key(), ctx.accounts.borrower.key(), amount);
        anchor_lang::solana_program::invoke(txn, &[ctx.accounts.pool.to_account_info(), ctx.accounts.borrower.to_account_info()])?;
        let pool = &mut ctx.accounts.pool;

        pool.balance -= amount;
        pool.borrowed += amount;

        let borrower_account = &mut ctx.accounts.borrower_account;
        borrower_account.borrower = ctx.accounts.borrower.key();
        borrower_account.amount_borrowed += amount;
        Ok(())
    }

    #[cfg(not(feature = "program"))]
    pub fn repay(ctx: Context<Repay>, amount: u64) -> ProgramResult {
        let txn = anchor_lang::solana_program::system_instruction::transfer(ctx.accounts.repayer.key(), ctx.accounts.pool.key(), amount);
        anchor_lang::solana_program::invoke(txn, &[ctx.accounts.repayer.to_account_info(), ctx.accounts.pool.to_account_info()])?;
        let pool = &mut ctx.accounts.pool;

        pool.balance += amount;
        pool.borrowed -= amount;

        let borrower_account = &mut ctx.accounts.borrower_account;
        borrower_account.amount_borrowed -= amount;
        Ok(())
    }

    #[cfg(not(feature = "program"))]
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let txn = anchor_lang::solana_program::system_instruction::transfer(ctx.accounts.pool.key(), ctx.accounts.withdrawer.key(), amount);
        anchor_lang::solana_program::invoke(txn, &[ctx.accounts.pool.to_account_info(), ctx.accounts.withdrawer.to_account_info()])?;
        let pool = &mut ctx.accounts.pool;

        pool.balance -= amount;
        pool.liquidity -= amount;

        let lender_account = &mut ctx.accounts.lender_account;
        lender_account.amount_lended -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAccount<'info> {
    #[account(init, payer=user, space=5000, seeds=[b"bankaccount", user.key().as_ref()], bump)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Lend<'info> {
    #[account(init, payer=user, space=5000, seeds=[b"poolaccount", user.key().as_ref()], bump)]
    pub pool: Account<'info, Pool>,
    #[account(init, payer=user, space=5000, seeds=[b"lenderaccount", user.key().as_ref()], bump)]
    pub lender_account: Account<'info, LenderAccount>,
    #[account(mut)]
    pub lender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(init, payer=borrower, space=5000, seeds=[b"borroweraccount", borrower.key().as_ref()], bump)]
    pub borrower_account: Account<'info, BorrowerAccount>,
    #[account(mut)]
    pub borrower: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub borrower_account: Account<'info, BorrowerAccount>,
    #[account(mut)]
    pub repayer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub lender_account: Account<'info, LenderAccount>,
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub name: String,
    pub balance: u64,
    pub owner: Pubkey,
    pub utilization: PoolUtilization,
    pub liquidity: u64,
    pub borrowed: u64,
}

#[account]
pub struct LenderAccount {
   pub lender: Pubkey,
   pub amount_lended: u64,
}

#[account]
pub struct BorrowerAccount {
   pub borrower: Pubkey,
   pub amount_borrowed: u64,
}

#[account]
pub struct MyAccount {
    pub owner: Pubkey,
    pub amount_borrowed: u64,
    pub collateral_deposited: u64,
    pub amount_lended: u64,

}

pub enum PoolUtilization {
    High,
    Medium,
    Low,
}
