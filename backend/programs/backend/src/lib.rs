use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    associated_token::AssociatedToken,
};

declare_id!("LendingProtoco111111111111111111111111111111");

#[program]
pub mod lending_protocol {
    use super::*;

    // Initialize lending pool
    pub fn initialize_lending_pool(
        ctx: Context<InitializeLendingPool>,
        min_collateral_ratio: u64,
        interest_rate: u64,
    ) -> Result<()> {
        let lending_pool = &mut ctx.accounts.lending_pool;
        lending_pool.authority = ctx.accounts.authority.key();
        lending_pool.token_mint = ctx.accounts.token_mint.key();
        lending_pool.vault = ctx.accounts.vault.key();
        lending_pool.min_collateral_ratio = min_collateral_ratio;
        lending_pool.interest_rate = interest_rate;
        lending_pool.total_deposits = 0;
        lending_pool.total_borrows = 0;
        Ok(())
    }

    // Deposit tokens into lending pool
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
    ) -> Result<()> {
        // Transfer tokens from user to vault
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            amount,
        )?;

        // Update user deposit account
        let user_deposit = &mut ctx.accounts.user_deposit;
        user_deposit.owner = ctx.accounts.user.key();
        user_deposit.lending_pool = ctx.accounts.lending_pool.key();
        user_deposit.amount = user_deposit.amount.checked_add(amount).unwrap();

        // Update lending pool
        let lending_pool = &mut ctx.accounts.lending_pool;
        lending_pool.total_deposits = lending_pool.total_deposits.checked_add(amount).unwrap();

        Ok(())
    }

    // Borrow tokens from lending pool
    pub fn borrow(
        ctx: Context<Borrow>,
        amount: u64,
    ) -> Result<()> {
        let lending_pool = &mut ctx.accounts.lending_pool;
        let collateral = &ctx.accounts.collateral_account;
        
        // Check collateral ratio
        let required_collateral = amount
            .checked_mul(lending_pool.min_collateral_ratio)
            .unwrap()
            .checked_div(100)
            .unwrap();
        
        require!(
            collateral.amount >= required_collateral,
            LendingError::InsufficientCollateral
        );

        // Transfer tokens from vault to user
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.lending_pool.to_account_info(),
                },
            ),
            amount,
        )?;

        // Update loan account
        let loan = &mut ctx.accounts.loan;
        loan.borrower = ctx.accounts.user.key();
        loan.lending_pool = ctx.accounts.lending_pool.key();
        loan.collateral_account = ctx.accounts.collateral_account.key();
        loan.amount = amount;
        loan.interest_rate = lending_pool.interest_rate;
        loan.last_update = Clock::get()?.unix_timestamp;

        // Update lending pool
        lending_pool.total_borrows = lending_pool.total_borrows.checked_add(amount).unwrap();

        Ok(())
    }

    // Repay borrowed tokens
    pub fn repay(
        ctx: Context<Repay>,
        amount: u64,
    ) -> Result<()> {
        let loan = &mut ctx.accounts.loan;
        
        // Calculate interest
        let time_elapsed = Clock::get()?.unix_timestamp.checked_sub(loan.last_update).unwrap();
        let interest = loan.amount
            .checked_mul(loan.interest_rate as u64)
            .unwrap()
            .checked_mul(time_elapsed as u64)
            .unwrap()
            .checked_div(365 * 24 * 60 * 60 * 100)
            .unwrap();
        
        let total_due = loan.amount.checked_add(interest).unwrap();
        require!(amount <= total_due, LendingError::InvalidRepaymentAmount);

        // Transfer tokens from user to vault
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            amount,
        )?;

        // Update loan account
        loan.amount = loan.amount.checked_sub(amount).unwrap();
        loan.last_update = Clock::get()?.unix_timestamp;

        // Update lending pool
        let lending_pool = &mut ctx.accounts.lending_pool;
        lending_pool.total_borrows = lending_pool.total_borrows.checked_sub(amount).unwrap();

        Ok(())
    }

    // Liquidate undercollateralized position
    pub fn liquidate(
        ctx: Context<Liquidate>,
        amount: u64,
    ) -> Result<()> {
        let lending_pool = &ctx.accounts.lending_pool;
        let loan = &ctx.accounts.loan;
        let collateral = &ctx.accounts.collateral_account;

        // Check if position is undercollateralized
        let required_collateral = loan.amount
            .checked_mul(lending_pool.min_collateral_ratio)
            .unwrap()
            .checked_div(100)
            .unwrap();
        
        require!(
            collateral.amount < required_collateral,
            LendingError::PositionNotLiquidatable
        );

        // Transfer collateral to liquidator
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.collateral_account.to_account_info(),
                    to: ctx.accounts.liquidator_token_account.to_account_info(),
                    authority: ctx.accounts.lending_pool.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLendingPool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + LendingPool::LEN
    )]
    pub lending_pool: Account<'info, LendingPool>,
    
    pub token_mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = lending_pool,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserDeposit::LEN,
        seeds = [
            b"user_deposit",
            user.key().as_ref(),
            lending_pool.key().as_ref(),
        ],
        bump
    )]
    pub user_deposit: Account<'info, UserDeposit>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub collateral_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = user,
        space = 8 + Loan::LEN
    )]
    pub loan: Account<'info, Loan>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub loan: Account<'info, Loan>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,
    
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    
    #[account(mut)]
    pub loan: Account<'info, Loan>,
    
    #[account(mut)]
    pub collateral_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub liquidator_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct LendingPool {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub vault: Pubkey,
    pub min_collateral_ratio: u64,
    pub interest_rate: u64,
    pub total_deposits: u64,
    pub total_borrows: u64,
}

#[account]
pub struct UserDeposit {
    pub owner: Pubkey,
    pub lending_pool: Pubkey,
    pub amount: u64,
}

#[account]
pub struct Loan {
    pub borrower: Pubkey,
    pub lending_pool: Pubkey,
    pub collateral_account: Pubkey,
    pub amount: u64,
    pub interest_rate: u64,
    pub last_update: i64,
}

impl LendingPool {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8 + 8 + 8;
}

impl UserDeposit {
    pub const LEN: usize = 32 + 32 + 8;
}

impl Loan {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8 + 8;
}

#[error_code]
pub enum LendingError {
    #[msg("Insufficient collateral for loan")]
    InsufficientCollateral,
    #[msg("Invalid repayment amount")]
    InvalidRepaymentAmount,
    #[msg("Position is not liquidatable")]
    PositionNotLiquidatable,
}
