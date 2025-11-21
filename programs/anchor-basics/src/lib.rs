use anchor_lang::prelude::*;

declare_id!("DwobFKwx9HBRAqmbxRx6eRpUCdpkSfU3hU6PvfrXrB9t");

#[program]
pub mod anchor_basics {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
