use anchor_lang::prelude::*;
declare_id!("GAChMFE4jNfB7XXfx6dEoPGWV7UxNRRdxois4FcmBVxe");

#[program]
pub mod whitelist {
    use super::*;

    pub fn initialize_whitelist(ctx: Context<InitializeWhitelist>, authority: Pubkey) -> Result<()> {
        let whitelist = &mut ctx.accounts.whitelisting_account;
        whitelist.counter = 0;
        Ok(())
    }

    pub fn add_to_whitelist(ctx: Context<AddWhiteListAddress>, address: Pubkey) -> Result<()> {
      

        //whitelist.counter.checked_add(1).unwrap();
        Ok(())
    }

}


//Data Validators
#[derive(Accounts)]
pub struct AddWhiteListAddress<'info> {         
    #[account(init,seeds=[user.key().as_ref()] , bump, payer = user, space = 8 + 32 + 8)]
    pub list_account: Account<'info, WhiteListedAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
#[account(init, seeds=[user.key().as_ref()] , bump,  payer = user, space = 8 + 32 + 8)]
    pub whitelisting_account: Account<'info, WhiteListingAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

pub system_program: Program<'info, System>,
}



//Data Structures
#[account] 
pub struct WhiteListingAccount {
    pub authority: Pubkey,
    pub counter: u64,
}

#[account]
pub struct WhiteListedAccount {
    pub address: Pubkey,
}

