use anchor_lang::prelude::*;

declare_id!("3Dqx282m1twoCYo97aFoJMk3CJusjC4MwzVf3qB7cEe2");

#[program]
pub mod auto_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.auto_pda_deriv.data = 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8+8,
        seeds = [
            "no-more-bullshit".as_bytes(),
        ],
        bump,
    )]
    auto_pda_deriv: Account<'info, MyAccount>,

    #[account(mut)]
    payer: Signer<'info>,
    
    system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount {
    data: u64,
}