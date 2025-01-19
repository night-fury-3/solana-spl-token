use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

declare_id!("9YwTUcrmYvuAmrrmvNvkVZ6z2mYkQRdfxGC4prsV5gcK");

#[program]
pub mod spl_token_transfer {
    use super::*;

    pub fn transfer_tokens(  
        ctx: Context<TransferTokens>,  
        amount: u64, // Amount of tokens to transfer  
    ) -> Result<()> {  
        // Perform the token transfer  
        token::transfer(ctx.accounts.into_transfer_context(), amount)?;  

        Ok(())  
    } 
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {  
    /// Source token account (the sender)  
    #[account(  
        mut,  
        constraint = from.owner.eq(&authority.key()) // Use eq method for comparison  
    )]   
    /// CHECK: This account is checked that the owner matches authority;   
    /// hence, access checks are enforced by the constraints imposed.  
    pub from: AccountInfo<'info>,  

    /// Destination token account (the receiver)  
    #[account(mut)]
    /// CHECK: This account is checked that the owner matches authority;   
    /// hence, access checks are enforced by the constraints imposed.  
    pub to: AccountInfo<'info>,  

    /// The wallet or account that authorizes the transfer  
    pub authority: Signer<'info>,  

    /// Token program that handles the token transfer  
    pub token_program: Program<'info, Token>,  
}  

impl<'info> TransferTokens<'info> {  
    pub fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {  
        let cpi_accounts = Transfer {  
            from: self.from.to_account_info(),  
            to: self.to.to_account_info(),  
            authority: self.authority.to_account_info(),  
        };  
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)  
    }  
}
