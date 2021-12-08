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
  const connection = new anchor.web3.Connection(
    anchor.web3.clusterApiUrl("devnet"),
    "confirmed"
  );

  let allProjects: Project[] = [];

  interface Project {
    project_id: number;
    representative: PublicKey;
    current_amount: number;
    goal_amount: number;
    deadline: number;
    achieved: boolean;
  }

  before(async () => {
    let newUser = Keypair.generate();
    let airdropSignature = await connection.requestAirdrop(
      newUser.publicKey,
      LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSignature);

    let newProject: Project;
    newProject = {
      project_id: 0,
      representative: newUser.publicKey,
      current_amount: 0,
      goal_amount: 100,
      deadline: Date.now(),
      achieved: false,
    };
    allProjects.push(newProject);
  });

  it("Initialize", async () => {
    // Add your test here.

    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
