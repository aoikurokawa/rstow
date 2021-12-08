import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CrowdFunding } from "../target/types/crowd_funding";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
} from "@solana/web3.js";

describe("CrowdFunding", () => {
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.Provider.env());
  // let IDL = anchor
  const program: anchor.Program = anchor.workspace.CrowdFunding;
  const provider = anchor.Provider.env();
  const wallet = provider.wallet as anchor.Wallet;

  interface Project {
    project_id: number;
    representative: PublicKey;
    current_amount: number;
    goal_amount: number;
    deadline: number;
    achieved: boolean;
  }

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
