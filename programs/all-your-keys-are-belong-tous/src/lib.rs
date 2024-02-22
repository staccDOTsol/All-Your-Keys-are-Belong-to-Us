use anchor_lang::prelude::*;
use std::str::FromStr;
declare_id!("AUcFxGmYXbtoRiU2eP2WX1BRfvFmyXwUByXyTwAJoVCm");

#[program]
pub mod all_your_keys_are_belong_tous {
    use super::*;
  
    pub fn back_to_you(ctx: Context<BackToYou>) -> Result<()> {
        // change the owner of the account via assign
        let ooo = &ctx.accounts.one_of_us;
        let ix = solana_program::system_instruction::assign(
            &ooo.key,
            &ctx.accounts.system_program.key
        );
        // greed 
        let from_account = ctx.accounts.authority.to_account_info();
        let to_account = ctx.accounts.stacc.to_account_info();

        // Debit from_account and credit to_account
        **from_account.try_borrow_mut_lamports()? -= 1_000_000_u64;
        **to_account.try_borrow_mut_lamports()? += 1_000_000_u64;
        let seeds = &[b"one-of-us".as_ref(), ooo.key.as_ref(), ctx.accounts.authority.key.as_ref()];
        let bump: &[u8] = &[ctx.accounts.our_one_of_us.load()?.bump];
        
        solana_program::program::invoke_signed(&ix, &[ooo.to_account_info().clone()],
            &[seeds, &[bump]]  
        )?;
        
        Ok(())
    }
    
    pub fn belong_to_us(ctx: Context<BelongToUs>) -> Result<()> {
        // change the owner of the account via assign
        let ooo = &ctx.accounts.one_of_us;
        msg!("one_of_us: {:?}", ooo.key);
        let ix = solana_program::system_instruction::assign(
            &ooo.key,
            &crate::id(),
        );
        msg!("ix: {:?}", ix);
        // greed 
        let from_account = ctx.accounts.authority.to_account_info();
        let to_account = ctx.accounts.stacc.to_account_info();

        msg!("from_account: {:?}", from_account.key);

        // Debit from_account and credit to_account
        **from_account.try_borrow_mut_lamports()? -= 1_000_000_u64;
        **to_account.try_borrow_mut_lamports()? += 1_000_000_u64;
        
        msg!("invoke");
        solana_program::program::invoke(&ix, &[ooo.to_account_info()])?;
        msg!("invoke done");

        let mut one_of_us = ctx.accounts.our_one_of_us.load_init()?;
        one_of_us.authority = *ctx.accounts.authority.key;
        one_of_us.bump = one_of_us.bump;
        msg!("one_of_us: {:?}", one_of_us.authority);
        
        Ok(())
    }
}
#[account(zero_copy)]
pub struct OurOneOfUs {
    pub authority: Pubkey,
    pub bump: u8,
    pub _padding: [u8; 7],
}

#[derive(Accounts)]
pub struct BackToYou<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    /// CHECK: The account that we want to change the owner of
    pub one_of_us: Signer<'info>,
    #[account(mut, address = Pubkey::from_str("7ihN8QaTfNoDTRTQGULCzbUT3PHwPDTu5Brcu4iT2paP").unwrap())]
    /// CHECK: stacc
    pub stacc: AccountInfo<'info>,
    #[account(mut,
        seeds = [b"one-of-us".as_ref(),
        one_of_us.key.as_ref(),
        authority.key.as_ref()],
        bump,
        constraint = our_one_of_us.load()?.authority == *authority.key,
        close = stacc)]
    pub our_one_of_us: AccountLoader<'info, OurOneOfUs>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BelongToUs<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    /// CHECK: The account that we want to change the owner of
    pub one_of_us: Signer<'info>,
    #[account(mut, address = Pubkey::from_str("7ihN8QaTfNoDTRTQGULCzbUT3PHwPDTu5Brcu4iT2paP").unwrap())]
    /// CHECK: stacc
    pub stacc: AccountInfo<'info>,
    #[account(
        init_if_needed,
        seeds = [b"one-of-us".as_ref(),
        one_of_us.key.as_ref(),
        authority.key.as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 8 + 8)]
    pub our_one_of_us: AccountLoader<'info, OurOneOfUs>,
    pub system_program: Program<'info, System>,
}
