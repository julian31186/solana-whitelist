import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Whitelist } from "../target/types/whitelist";
import { PublicKey } from "@solana/web3.js";


describe("whitelist", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet;
  const program = anchor.workspace.Whitelist as Program<Whitelist>;



  it("Create Whitelist!", async () => {
    const [whitelist,whitelistBump] = await anchor.web3.PublicKey.findProgramAddress([wallet.publicKey.toBytes()], program.programId)
    const tx = await program.methods.initializeWhitelist(wallet.publicKey).accounts({
      whitelistingAccount: whitelist,
      user: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      

  }).rpc();
    console.log("Your transaction signature", tx);
  });



  


});
