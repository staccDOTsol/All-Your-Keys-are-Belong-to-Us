use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("BjQ3BpvtHL6oLjGRQZbdpc5p4U5phJFrQLYWQLJQSyAC");
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
        let bump: &[u8] = &[ctx.accounts.our_one_of_us.bump];
        
        solana_program::program::invoke_signed(&ix, &[ooo.to_account_info().clone()],
            &[seeds, &[bump]]  
        )?;
        
        Ok(())
    }

    pub fn bid(ctx: Context<Bid>, new_authority: Pubkey, bid: u64) -> Result<()> {
   
        let from_account = ctx.accounts.authority.to_account_info();
        let to_account = ctx.accounts.one_of_us.to_account_info();
        let og_authority = ctx.accounts.og_authority.to_account_info();
        let stacc = ctx.accounts.stacc.to_account_info();
        let our_one_of_us = &mut ctx.accounts.our_one_of_us;
        assert!(bid > our_one_of_us.lamports, "Bid must be greater than the current lamports");

        let ix_transfers_1 = solana_program::system_instruction::transfer(
            from_account.key,
            to_account.key,
            bid / 3
        );

        let ix_transfers_2 = solana_program::system_instruction::transfer(
            from_account.key,
            og_authority.key,
            bid / 3
        );

        let ix_transfers_3 = solana_program::system_instruction::transfer(
            from_account.key,
            stacc.key,
            bid / 3
        );

        solana_program::program::invoke(&ix_transfers_1, &[from_account.clone(), to_account.clone()])?;
        solana_program::program::invoke(&ix_transfers_2, &[from_account.clone(), og_authority.clone()])?;
        solana_program::program::invoke(&ix_transfers_3, &[from_account.clone(), stacc.clone()])?;

        our_one_of_us.authority = new_authority;
        our_one_of_us.lamports = bid;
        our_one_of_us.time_remaining = Clock::get()?.unix_timestamp + 24 * 60 * 60;
        Ok(())
    }

}
#[account]
pub struct OurOneOfUs {
    pub authority: Pubkey, // 32
    pub og_authority: Pubkey, // 32
    pub bump: u8, // 1 
    pub lamports: u64,  // 8 
    pub time_remaining: i64, // 8
    // = 81
}
#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, address = Pubkey::from_str("7ihN8QaTfNoDTRTQGULCzbUT3PHwPDTu5Brcu4iT2paP").unwrap())]
    /// CHECK: stacc 
    pub stacc: AccountInfo<'info>,
    /// CHECK: one_of_us 
    #[account(mut)]
    pub one_of_us: AccountInfo<'info>,
    /// CHECK: our_one_of_us
    pub og_authority: AccountInfo<'info>,
    #[account(mut,
        seeds = [b"one-of-us".as_ref(),
        one_of_us.key.as_ref(),
        authority.key.as_ref()],
        bump)]
    pub our_one_of_us: Account<'info, OurOneOfUs>,
    pub system_program: Program<'info, System>,
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
        constraint = our_one_of_us.authority == *authority.key,
        constraint = our_one_of_us.time_remaining < Clock::get()?.unix_timestamp,
        close = stacc)]
    pub our_one_of_us: Account<'info, OurOneOfUs>,
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
        space = 8 + 81)]
    pub our_one_of_us: Account<'info, OurOneOfUs>,
    pub system_program: Program<'info, System>,
}
