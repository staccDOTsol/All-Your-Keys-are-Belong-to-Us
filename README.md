# All Your Keys are Belong To Us


HEY

The coolest bit about this is that now me, you, and anyone on Solami can say FCK YOU to people who have compromised your wallets. 

Instead of trying your best-i-best to build cool 
@jito_sol
 bots to execute your 
@JupiterExchange
 launchpad tx faster, or in my stupid case liquidate Tour De Sun winnings on jare...gm after hakkers went n robbed me blind of a key (or upwards of a dozen) I leaked on github.

Now, for the wee fee of 0.001 to wrap n 0.001 to unwrap you can secure your lost-and-sad private keys, mnemonics or whatever u went n had compromised by ~Gibbing it to the staccomatic~ program, 

allYourKeysAreBelongTous

https://solscan.io/tx/kEhPqZqmcmpPg91cJL9UboQZMDASVgLbLGNniQ1JkER6FGv7JCJyT684XfHxa8mtxnQfEG9vATvByJTFdbZDL2A
https://solscan.io/tx/2jNpKQiad2Fobrafjv8KtWqQNwa6JZtnRxyLimTZVJtWSQ3kYT8bXGgY9M9eGYg8GUbsGSZGbkziCYH4X5HQo8Bd

@aeyakovenko
 how do I verify n enshrine this thing so I can't muck off with everyone's shiz or hold em ransom


yea remind me to put these bytes back on chain
or
extend the program size
or sommat



    pub fn belong_to_us(ctx: Context<BelongToUs>) -> Result<()> {
        // change the owner of the account via assign
        let ooo = &ctx.accounts.one_of_us;
        // EdwardsPoint to publickey ?
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

        let one_of_us = &mut ctx.accounts.our_one_of_us; 
        one_of_us.authority = *ctx.accounts.authority.key;
        one_of_us.bump = one_of_us.bump;    
        one_of_us.lamports = 1_000_000_u64;
        one_of_us.time_remaining = Clock::get()?.unix_timestamp + 24 * 60 * 60;
        one_of_us.og_authority = *ctx.accounts.one_of_us.key;
        msg!("one_of_us: {:?}", one_of_us.authority);
        
        Ok(())
    }

    innit