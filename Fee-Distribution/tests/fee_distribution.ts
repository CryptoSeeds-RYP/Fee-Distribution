import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FeeDistribution } from "../target/types/fee_distribution";
import { assert } from "chai";

describe("fee-distribution", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.FeeDistribution as Program<FeeDistribution>;

  it("Should distribute fees correctly", async () => {
    const tx = await program.methods
      .distributeFees(new anchor.BN(1000000)) // Test with 1,000,000 lamports
      .rpc();

    console.log("Transaction signature:", tx);
    assert.ok(tx);
  });

  it("Should allow holders to harvest rewards", async () => {
    const tx = await program.methods.harvestRewards().rpc();
    console.log("Harvest Transaction Signature:", tx);
    assert.ok(tx);
  });

  it("Should expire unclaimed rewards after 1 year", async () => {
    const tx = await program.methods.expireUnclaimedRewards().rpc();
    console.log("Expire Rewards Transaction Signature:", tx);
    assert.ok(tx);
  });
});
