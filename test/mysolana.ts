import * as anchor from "@project-serum/anchor/";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";

import { Mysolana } from "../target/types/mysolana";

// describe("Mysolanaapp", () => {
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.Provider.env());

  // const program = anchor.workspace. as Program<Mysolana>;

  // it("Is initialized!", async () => {
    // Add your test here.
  //   const tx = await program.rpc.initialize({});
  //   console.log("Your transaction signature", tx);
  // });
  // const provider = anchor.Provider.env();
  // anchor.setProvider(provider);
  // const program = anchor.workspace.Mysolanaapp;
  // it("It initializes the account", async () => {
  //   const baseAccount = anchor.web3.Keypair.generate();
  //   await program.rpc.initialize("Hello World", {
  //     accounts: {
  //       baseAccount: baseAccount.publicKey,
  //       user: provider.wallet.publicKey,
  //       systemProgram: SystemProgram.programId,
  //     },
  //     signers: [baseAccount],
  //   });

  //   const account = await program.account.baseAccount.fetch(
  //     baseAccount.publicKey
  //   );
  //   console.log("Data: ", account.data);
  //   assert.ok(account.data === "Hello World");
  //   _baseAccount = baseAccount;
  // });

  // it("Updates a previously created account", async () => {
  //   const baseAccount = _baseAccount;

  //   await program.rpc.update("Some new data", {
  //     accounts: {
  //       baseAccount: baseAccount.publicKey,
  //     },
  //   });

  //   const account = await program.account.baseAccount.fetch(
  //     baseAccount.publicKey
  //   );
  //   console.log("Updated data: ", account.data);
  //   assert.ok(account.data === "Some new data");
  //   console.log("all account data:", account);
  //   console.log("All data: ", account.dataList);
  //   assert.ok(account.dataList.length === 2);
  // });
// });
