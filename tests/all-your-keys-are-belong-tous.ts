import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AllYourKeysAreBelongTous } from "../target/types/all_your_keys_are_belong_tous";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import fs from 'fs'
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
const dedKey = "hwr9RkgWBrqFKf6bKXpdUu4EBFEgj6u6ZZbRctK3kYSzA1xqrGRoFu4tpii27Dh58qiyvx8EcYmW6PSciprkfCZ";
describe("all-your-keys-are-belong-tous", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = new Program(
    JSON.parse(fs.readFileSync('target/idl/all_your_keys_are_belong_tous.json').toString()) as anchor.Idl,
    new PublicKey("AUcFxGmYXbtoRiU2eP2WX1BRfvFmyXwUByXyTwAJoVCm"),
    anchor.getProvider()
  )

  it("Is initialized!", async () => {
    // Add your test here.
    const authority = anchor.web3.Keypair.fromSecretKey(new Uint8Array(
      JSON.parse(fs.readFileSync('/Users/jd/7i.json').toString())
    ));
    const stacc = authority 
    const oneOfUs = Keypair.fromSecretKey(bs58.decode(dedKey));
    const ourOneOfUs = PublicKey.findProgramAddressSync([
      Buffer.from("one-of-us"),
      oneOfUs.publicKey.toBuffer(),
      authority.publicKey.toBuffer()
    ], program.programId);
    console.log("Our one of us", ourOneOfUs[0].toBase58());
/*

    const tx = await program.methods.belongToUs()
    .accounts({
      authority: authority.publicKey,
      ourOneOfUs: ourOneOfUs[0],
      oneOfUs: oneOfUs.publicKey,
      stacc: stacc.publicKey,
      systemProgram: SystemProgram.programId
    })
    .signers([authority, oneOfUs])
    .rpc({skipPreflight: true});
    
    console.log("Your belongtous signature", tx);
*/
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

  });
});
