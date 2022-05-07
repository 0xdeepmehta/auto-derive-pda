import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AutoPda } from "../target/types/auto_pda";

describe("auto_pda", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AutoPda as Program<AutoPda>;

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize();
    const keys = await tx.pubkeys();
    console.log(keys.autoPdaDeriv.toString());

    const txHash = await tx.rpc();
    console.log("Your transaction signature", txHash);

    const pdaAccountData = (await program.account.myAccount.fetch(keys.autoPdaDeriv)).data
    console.log(pdaAccountData.toNumber())
  });
});
