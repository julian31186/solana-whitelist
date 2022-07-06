import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Whitelist } from "../target/types/whitelist";
import { PublicKey } from "@solana/web3.js";

describe("whitelist", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet;
  const program = anchor.workspace.Whitelist as Program<Whitelist>;

  const key = new PublicKey("BWFYCRtbx3sZjkNoafkVZk49ecMXkQzkA5gC8TGAFYzw");

  it("Create Whitelist!", async () => {
    const [whitelist, whitelistBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [wallet.publicKey.toBytes(), key.toBuffer()],
        program.programId
      );
    const tx = await program.methods
      .initializeWhitelist(key)
      .accounts({
        whitelistingAccount: whitelist,
        user: wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Fetch a Whitelist!", async () => {
        const [whitelist, whitelistBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [wallet.publicKey.toBytes(), key.toBuffer()],
        program.programId
      );

      const whitelistData = await program.account.whiteListingAccount.fetch(whitelist);
      
      console.log(whitelistData.authority.toString(), whitelistData.key.toString());

  });
});
