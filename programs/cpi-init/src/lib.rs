use anchor_lang::prelude::*;
use callee_program::cpi::accounts::Initialize;
use callee_program::program::CalleeProgram;

declare_id!("CeePq5JvarpFoJdAMLUn2wBdC7Q8p19NVjZxVqH6YXYg");

#[program]
pub mod cpi_init {
    use super::*;

    pub fn cpi(ctx: Context<Cpi>) -> Result<()> {
        let (pda, _bump_seed) =
            Pubkey::find_program_address(&["SEED".as_ref()], &ctx.accounts.callee_program.key());

        if pda != ctx.accounts.data.key() {
            return err!(ErrorCode::IncorrectPDA);
        }

        let cpi_program = ctx.accounts.callee_program.to_account_info();
        let cpi_accounts = Initialize {
            data: ctx.accounts.data.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        callee_program::cpi::initialize(cpi_ctx).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Cpi<'info> {
    /// CHECK:
    #[account(mut)]
    pub data: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub callee_program: Program<'info, CalleeProgram>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Incorrect PDA")]
    IncorrectPDA,
}
