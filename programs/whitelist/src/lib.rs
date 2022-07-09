use anchor_lang::prelude::*;
declare_id!("9HpLH6sDQNfphnvaoy4KBfmpQyVfcwkvxEYDhriFavJP");

#[program]
pub mod whitelist {
    use super::*;

    pub fn initialize_whitelist(
        ctx: Context<InitializeWhitelist>,
        key: Pubkey,
        _project: String
    ) -> Result<()> {
        let whitelist = &mut ctx.accounts.whitelisting_account;
        whitelist.key = key;
        whitelist.counter = 0;
        whitelist.authority = ctx.accounts.user.key();
        Ok(())
    }
}

//Data Validators
#[derive(Accounts)]
#[instruction(key: Pubkey , project: String)]


pub struct InitializeWhitelist<'info> {
    #[account(init, seeds=[user.key().as_ref(), key.key().as_ref(), project.as_bytes()] , bump,  payer = user, space = 8 + 32 + 8 + 32)]
    pub whitelisting_account: Account<'info, WhiteListingAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
  
//Data Structures
#[account]
pub struct WhiteListingAccount {
    pub authority: Pubkey,
    pub key: Pubkey,
    pub counter: u64,
}
