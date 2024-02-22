import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AllYourKeysAreBelongTous } from "../target/types/all_your_keys_are_belong_tous";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import fs from 'fs'
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
const dedKey = [36,132,1,157,79,179,165,2,69,242,223,53,76,66,8,112,78,153,60,182,89,155,230,116,219,53,190,54,192,137,158,1,255,0,198,221,91,179,95,217,235,252,230,  235,184,236,83,33,125,83,29,240,249,54,193,84,181,105,175,234,16,224,11,206]
  
describe("all-your-keys-are-belong-tous", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = new Program(
    JSON.parse(fs.readFileSync('target/idl/all_your_keys_are_belong_tous.json').toString()) as anchor.Idl,
    new PublicKey("BjQ3BpvtHL6oLjGRQZbdpc5p4U5phJFrQLYWQLJQSyAC"),
    anchor.getProvider()
  )

  it("Is initialized!", async () => {
    // Add your test here.
    const authority = anchor.web3.Keypair.fromSecretKey(new Uint8Array(
      JSON.parse(fs.readFileSync('/Users/jd/7i.json').toString())
    ));
    const stacc = authority 
    const oneOfUs = Keypair.fromSecretKey(new Uint8Array(dedKey));
    const ourOneOfUs = PublicKey.findProgramAddressSync([
      Buffer.from("one-of-us"),
      oneOfUs.publicKey.toBuffer(),
      authority.publicKey.toBuffer()
    ], program.programId);
    console.log("Our one of us", ourOneOfUs[0].toBase58());
    

/*
    const tx = await program.methods.belongToUs()    .accounts({
      authority: authority.publicKey,
      ourOneOfUs: ourOneOfUs[0],
      oneOfUs: oneOfUs.publicKey,
      stacc: stacc.publicKey,
      systemProgram: SystemProgram.programId
    })
    .signers([authority, oneOfUs])
    .rpc();
    
    console.log("Your belongtous signature", tx);
    const tx2 = await program.methods.backToYou()
    .accounts({
      authority: authority.publicKey,
      ourOneOfUs: ourOneOfUs[0],
      oneOfUs: oneOfUs.publicKey,
      stacc: stacc.publicKey,
      systemProgram: SystemProgram.programId
    })
    .signers([authority, oneOfUs])
    .rpc({skipPreflight: true});

    console.log("Your backtoyou signature", tx2);
*/
// what number over 1_000_000 is mod 3 == 0 ?
// 1_000_001 % 3 == 1
// 1_000_002 % 3 == 2
// 1_000_003 % 3 == 0
  
    const bid = await program.methods.bid(SystemProgram.programId, new BN(2_000_006))
    .accounts({
      authority: authority.publicKey,
      stacc: stacc.publicKey,
      oneOfUs: oneOfUs.publicKey,
      ogAuthority: authority.publicKey,
      ourOneOfUs: ourOneOfUs[0],
      systemProgram: SystemProgram.programId
    })
    .preInstructions([
      anchor.web3.SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: oneOfUs.publicKey,
        lamports: 1_000_001
      })
    ])
    .signers([authority])
    .rpc({skipPreflight: true});
    console.log('bid', bid);  
  });
});
