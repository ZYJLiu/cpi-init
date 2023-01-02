use anchor_lang::prelude::*;

declare_id!("FAexH7kpt2AdCkvTJ9YFEautWFQiNptdhuMoWKCiPzAM");

const SEED: &str = "SEED";

#[program]
pub mod callee_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds=[SEED.as_bytes()], bump, payer = user, space = 8 + 8)]
    pub data: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data {
    pub data: u64,
}
