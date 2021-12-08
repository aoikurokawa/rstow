import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CrowdFunding } from "../target/types/crowd_funding";

describe("CrowdFunding", () => {
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.Provider.env());
  // let IDL = anchor
  const program: anchor.Program = anchor.workspace.CrowdFunding;
  const provider = anchor.Provider.env();
  const wallet = provider.wallet as anchor.Wallet;
  

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
