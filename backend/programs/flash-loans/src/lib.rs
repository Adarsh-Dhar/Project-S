use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("5sf3oNJKEARM9ia6yew3xGVcct8244xHxLEPQ4F2cFQb");

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

    pub fn lend(ctx: Context<Lend>, amount: u64) -> ProgramResult {
        let txn = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.lender.key(), 
            &ctx.accounts.pool.key(), 
            amount
        );

        anchor_lang::solana_program::program::invoke(&txn, &[ctx.accounts.lender.to_account_info(), ctx.accounts.pool.to_account_info()])?;
        let pool = &mut ctx.accounts.pool;

        pool.balance += amount;

        let lender_account = &mut ctx.accounts.lender_account;
        lender_account.lender = ctx.accounts.lender.key();
        lender_account.amount_lended += amount;
        Ok(())
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> ProgramResult {
        let txn = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.pool.key(), 
            &ctx.accounts.borrower.key(), 
            amount);
        anchor_lang::solana_program::program::invoke(&txn, &[ctx.accounts.pool.to_account_info(), ctx.accounts.borrower.to_account_info()])?;
        let pool = &mut ctx.accounts.pool;

        pool.balance -= amount;
        pool.borrowed += amount;

        let borrower_account = &mut ctx.accounts.borrower_account;
        borrower_account.borrower = ctx.accounts.borrower.key();
        borrower_account.amount_borrowed += amount;
        Ok(())
    }

    pub fn repay(ctx: Context<Repay>, amount: u64) -> ProgramResult {
        let txn = anchor_lang::solana_program::system_instruction::transfer(&ctx.accounts.repayer.key(), &ctx.accounts.pool.key(), amount);
        anchor_lang::solana_program::program::invoke(&txn, &[ctx.accounts.repayer.to_account_info(), ctx.accounts.pool.to_account_info()])?;
        let pool = &mut ctx.accounts.pool;

        pool.balance += amount;
        pool.borrowed -= amount;

        let borrower_account = &mut ctx.accounts.borrower_account;
        borrower_account.amount_borrowed -= amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let txn = anchor_lang::solana_program::system_instruction::transfer(&ctx.accounts.pool.key(), &ctx.accounts.withdrawer.key(), amount);
        anchor_lang::solana_program::program::invoke(&txn, &[ctx.accounts.pool.to_account_info(), ctx.accounts.withdrawer.to_account_info()])?;
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
    #[account(init, payer=owner, space=5000, seeds=[b"myaccount", owner.key().as_ref()], bump)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Lend<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(init, payer=lender, space=5000, seeds=[b"lenderaccount", lender.key().as_ref()], bump)]
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
    pub utilization: u8,
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

pub const POOL_UTILIZATION_HIGH: u8 = 0;
pub const POOL_UTILIZATION_MEDIUM: u8 = 1;
pub const POOL_UTILIZATION_LOW: u8 = 2;

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_create_account() {
        let program_id = Pubkey::default();
        let owner_key = Pubkey::new_unique();
        let mut lamports = 1000000;
        let mut data = vec![0; 5000];
        
        let owner_account = AccountInfo::new(
            &owner_key,
            true,
            true,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            Epoch::default(),
        );

        let my_account_key = Pubkey::new_unique();
        let mut my_account_lamports = 0;
        let mut my_account_data = vec![0; 5000];
        
        let my_account = AccountInfo::new(
            &my_account_key,
            false,
            true,
            &mut my_account_lamports,
            &mut my_account_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let system_program_key = Pubkey::new_unique();
        let mut system_lamports = 0;
        let mut system_data = vec![];
        let system_program = AccountInfo::new(
            &system_program_key,
            false,
            false,
            &mut system_lamports,
            &mut system_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let accounts = CreateAccount {
            my_account: Account::try_from(&my_account).unwrap(),
            owner: Signer::try_from(&owner_account).unwrap(),
            system_program: Program::try_from(&system_program).unwrap(),
        };

        let ctx = Context::new(program_id, accounts, &[]);
        let result = super::backend::create_account(ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lend() {
        let program_id = Pubkey::default();
        
        // Setup pool account
        let pool_key = Pubkey::new_unique();
        let mut pool_lamports = 0;
        let mut pool_data = vec![0; 5000];
        let pool_account = AccountInfo::new(
            &pool_key,
            false,
            true,
            &mut pool_lamports,
            &mut pool_data,
            &program_id,
            false,
            Epoch::default(),
        );

        // Setup lender account
        let lender_key = Pubkey::new_unique();
        let mut lender_lamports = 1000000;
        let mut lender_data = vec![0; 5000];
        let lender_account_info = AccountInfo::new(
            &lender_key,
            true,
            true,
            &mut lender_lamports,
            &mut lender_data,
            &program_id,
            false,
            Epoch::default(),
        );

        // Setup lender's tracking account
        let lender_tracking_key = Pubkey::new_unique();
        let mut lender_tracking_lamports = 0;
        let mut lender_tracking_data = vec![0; 5000];
        let lender_tracking_account = AccountInfo::new(
            &lender_tracking_key,
            false,
            true,
            &mut lender_tracking_lamports,
            &mut lender_tracking_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let system_program_key = Pubkey::new_unique();
        let mut system_lamports = 0;
        let mut system_data = vec![];
        let system_program = AccountInfo::new(
            &system_program_key,
            false,
            false,
            &mut system_lamports,
            &mut system_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let accounts = Lend {
            pool: Account::try_from(&pool_account).unwrap(),
            lender_account: Account::try_from(&lender_tracking_account).unwrap(),
            lender: Signer::try_from(&lender_account_info).unwrap(),
            system_program: Program::try_from(&system_program).unwrap(),
        };

        let ctx = Context::new(program_id, accounts, &[]);
        let result = super::backend::lend(ctx, 100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_borrow() {
        let program_id = Pubkey::default();
        
        // Setup pool account with initial balance
        let pool_key = Pubkey::new_unique();
        let mut pool_lamports = 5000000;
        let mut pool_data = vec![0; 5000];
        let pool_account = AccountInfo::new(
            &pool_key,
            false,
            true,
            &mut pool_lamports,
            &mut pool_data,
            &program_id,
            false,
            Epoch::default(),
        );

        // Setup borrower account
        let borrower_key = Pubkey::new_unique();
        let mut borrower_lamports = 1000000;
        let mut borrower_data = vec![0; 5000];
        let borrower_account_info = AccountInfo::new(
            &borrower_key,
            true,
            true,
            &mut borrower_lamports,
            &mut borrower_data,
            &program_id,
            false,
            Epoch::default(),
        );

        // Setup borrower's tracking account
        let borrower_tracking_key = Pubkey::new_unique();
        let mut borrower_tracking_lamports = 0;
        let mut borrower_tracking_data = vec![0; 5000];
        let borrower_tracking_account = AccountInfo::new(
            &borrower_tracking_key,
            false,
            true,
            &mut borrower_tracking_lamports,
            &mut borrower_tracking_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let system_program_key = Pubkey::new_unique();
        let mut system_lamports = 0;
        let mut system_data = vec![];
        let system_program = AccountInfo::new(
            &system_program_key,
            false,
            false,
            &mut system_lamports,
            &mut system_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let accounts = Borrow {
            pool: Account::try_from(&pool_account).unwrap(),
            borrower_account: Account::try_from(&borrower_tracking_account).unwrap(),
            borrower: Signer::try_from(&borrower_account_info).unwrap(),
            system_program: Program::try_from(&system_program).unwrap(),
        };

        let ctx = Context::new(program_id, accounts, &[]);
        let result = super::backend::borrow(ctx, 1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_repay() {
        let program_id = Pubkey::default();
        
        // Setup pool account
        let pool_key = Pubkey::new_unique();
        let mut pool_lamports = 0;
        let mut pool_data = vec![0; 5000];
        let pool_account = AccountInfo::new(
            &pool_key,
            false,
            true,
            &mut pool_lamports,
            &mut pool_data,
            &program_id,
            false,
            Epoch::default(),
        );

        // Setup borrower's tracking account with borrowed amount
        let borrower_account_key = Pubkey::new_unique();
        let mut borrower_account_lamports = 0;
        let mut borrower_account_data = vec![0; 5000];
        let borrower_tracking_account = AccountInfo::new(
            &borrower_account_key,
            false,
            true,
            &mut borrower_account_lamports,
            &mut borrower_account_data,
            &program_id,
            false,
            Epoch::default(),
        );

        // Setup repayer account
        let repayer_key = Pubkey::new_unique();
        let mut repayer_lamports = 1000000;
        let mut repayer_data = vec![0; 5000];
        let repayer_account = AccountInfo::new(
            &repayer_key,
            true,
            true,
            &mut repayer_lamports,
            &mut repayer_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let system_program_key = Pubkey::new_unique();
        let mut system_lamports = 0;
        let mut system_data = vec![];
        let system_program = AccountInfo::new(
            &system_program_key,
            false,
            false,
            &mut system_lamports,
            &mut system_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let accounts = Repay {
            pool: Account::try_from(&pool_account).unwrap(),
            borrower_account: Account::try_from(&borrower_tracking_account).unwrap(),
            repayer: Signer::try_from(&repayer_account).unwrap(),
            system_program: Program::try_from(&system_program).unwrap(),
        };

        let ctx = Context::new(program_id, accounts, &[]);
        let result = super::backend::repay(ctx, 500);
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw() {
        let program_id = Pubkey::default();
        
        // Setup pool account with balance
        let pool_key = Pubkey::new_unique();
        let mut pool_lamports = 5000000;
        let mut pool_data = vec![0; 5000];
        let pool_account = AccountInfo::new(
            &pool_key,
            false,
            true,
            &mut pool_lamports,
            &mut pool_data,
            &program_id,
            false,
            Epoch::default(),
        );

        // Setup lender's tracking account with deposited amount
        let lender_account_key = Pubkey::new_unique();
        let mut lender_account_lamports = 0;
        let mut lender_account_data = vec![0; 5000];
        let lender_tracking_account = AccountInfo::new(
            &lender_account_key,
            false,
            true,
            &mut lender_account_lamports,
            &mut lender_account_data,
            &program_id,
            false,
            Epoch::default(),
        );

        // Setup withdrawer account
        let withdrawer_key = Pubkey::new_unique();
        let mut withdrawer_lamports = 1000000;
        let mut withdrawer_data = vec![0; 5000];
        let withdrawer_account = AccountInfo::new(
            &withdrawer_key,
            true,
            true,
            &mut withdrawer_lamports,
            &mut withdrawer_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let system_program_key = Pubkey::new_unique();
        let mut system_lamports = 0;
        let mut system_data = vec![];
        let system_program = AccountInfo::new(
            &system_program_key,
            false,
            false,
            &mut system_lamports,
            &mut system_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let accounts = Withdraw {
            pool: Account::try_from(&pool_account).unwrap(),
            lender_account: Account::try_from(&lender_tracking_account).unwrap(),
            withdrawer: Signer::try_from(&withdrawer_account).unwrap(),
            system_program: Program::try_from(&system_program).unwrap(),
        };

        let ctx = Context::new(program_id, accounts, &[]);
        let result = super::backend::withdraw(ctx, 1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pool_utilization() {
        assert_eq!(POOL_UTILIZATION_HIGH, 0);
        assert_eq!(POOL_UTILIZATION_MEDIUM, 1);
        assert_eq!(POOL_UTILIZATION_LOW, 2);
    }
}
