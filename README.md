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

    rite, fungame
tooooor:~$ cat JARehRjGUkkEShpjzfuV4ERJS25j8XhamL776FAktNGm.json
[36,132,1,157,79,179,165,2,69,242,223,53,76,66,8,112,78,153,60,182,89,155,230,116,219,53,190,54,192,137,158,1,255,0,198,221,91,179,95,217,235,252,230,235,184,236,83,33,125,83,29,240,249,54,193,84,181,105,175,234,16,224,11,206]
but all y'all alr knew that
all-your-keys-are-belong-tous
Our one of us GFgAKBoNCs1786TUAsK9QBcPbzqoVazFntLHxyCRjP14
Your belongtous signature https://solscan.io/tx/LoszYE2W8q8n6D19fX7sQakLRNhSvLxhYrE8tReCGyQyoSxWh4AknsDFozFdDqsrSqripNC5hNiixidrCXrarNm
bid 7tKXmz9SGKfP1fZQ2HpxTDo5mA9sqfMEZuEhYWU5fnD7QifBvYXeFb5BEhVzFdZK8Jh2BVYMYsJszFiY2LgDpR8
bid X21fnRR4LaLj2hyeeoYWBJqpxS28DGACBTarPiQLBNBnMFRGSSZDmLYbnG7uYMxEcrvZW6e6Cgu1STyTEQzBsAu
may the best dev-savvy person win
each bid must % 3 == 0
each bid must > previous bid
each bid pays 1/3 to og_authority (me in this case)
each bid pays 1/3 to stacc (always me)
each bid pays 1/3 to the address being auctioned (jare...gm in this case)
each bid resets the timer to now + 24hr
if the timer exhausts, the current authority can BackToYou which is a misnomer or sommat innit
anywho
absolute lunacy
https://github.com/staccDOTsol/All-Your-Keys-are-Belong-to-Us

~derp~

Man oh man

nfts are finally cool

there's a cool fuckin exploit on solana being abused but I see much cooler applications for it.  1. if your private key for a wallet (owned by system program) signs,  a program can reassign ownership of that address  2. prior, users will load native sol, tokens, nfts of value to a wallet. 3. next, users assign the address to the program in the same tx as an nft is minted hilariously containing the private key as an attribute in the metadata 4. now, the authority field of the pda for the basket of value<->nft<->pda trio is assigned to the nft 5. on burning via the program or transfer hook (wen) the native sol is credited to the burner, as burning assures ownership of the nft 6. on interacting with the program with an unburnt nft, the program asserts valid ownership and allows ix via the program to trx the non-native sol value elsewhere  herein I believe these are notes of value, baskets of value, and overall the first time the solana ecosystem has seen nfts worth anything more than rent.
