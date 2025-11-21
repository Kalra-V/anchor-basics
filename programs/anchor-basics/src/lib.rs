use anchor_lang::prelude::*;

declare_id!("DwobFKwx9HBRAqmbxRx6eRpUCdpkSfU3hU6PvfrXrB9t");

#[program]
pub mod anchor_basics {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        return Ok(());
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        return Ok(());
    }
}

// Accounts struct for initialize
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 8,
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, GmCounter>,

    pub system_program: Program<'info, System>
}

// Accounts struct for increment
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"counter", user.key().as_ref()], bump)]
    pub counter: Account<'info, GmCounter>,

    pub user: Signer<'info>
}

// PDA Account storing the count
#[account]
pub struct GmCounter {
    pub count: u64
}
