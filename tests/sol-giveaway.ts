import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import { getProgram } from "./utils/program";
import { getSolGiveawayPda } from "./utils/pdas";
import {
  getManagerKeypair,
  getOwnerKeypair,
  getUserKeypair,
} from "./utils/wallets";
import { expect } from "chai";

describe("Sol Giveaway", () => {
  const program = getProgram();
  const giveawayId = -1;

  const giveawayPDA = getSolGiveawayPda(program, giveawayId);
  const owner = getOwnerKeypair();
  const manager = getManagerKeypair();
  const user = getUserKeypair();

  it("It creates a sol giveaway", async () => {
    const tx = await program.methods
      .createSolGiveaway({
        giveawayId: new BN(giveawayId),
        lamportsAmount: new BN(0.25 * Math.pow(10, 9)),
        winnersAmount: new BN(3),
      })
      .signers([owner])
      .rpc()
      .catch((err) => console.log(err));

    const giveaway = await program.account.solGiveaway.fetch(giveawayPDA);

    expect(giveaway);
  });

  it("It fails to pay out sol", async () => {
    try {
      await program.methods
        .payoutSolGiveaway(new BN(giveawayId))
        .signers([user])
        .accounts({ signer: user.publicKey })
        .rpc();
    } catch (err) {
      expect(err);
    }
  });

  it("Sets sol winners", async () => {
    await program.methods
      .setSolGiveawayWinners({
        winnerKeys: [user.publicKey, user.publicKey, user.publicKey],
        giveawayId: new BN(giveawayId),
      })
      .signers([manager])
      .accounts({ signer: manager.publicKey })
      .rpc()
      .catch((err) => console.log(err));

    const giveaway = await program.account.solGiveaway.fetch(giveawayPDA);

    expect(giveaway.winners.length).to.equal(giveaway.winnersAmount.toNumber());
  });

  it("it pays out sol", async () => {
    await program.methods
      .payoutSolGiveaway(new BN(giveawayId))
      .signers([manager])
      .accounts({ signer: manager.publicKey, winnerAccount: user.publicKey })
      .rpc()
      .catch((err) => console.log(err));

    const giveaway = await program.account.solGiveaway.fetch(giveawayPDA);

    expect(giveaway.winners.length).to.equal(
      giveaway.winnersAmount.toNumber() - 1
    );
  });

  it("it claims sol", async () => {
    await program.methods
      .claimSolGiveaway(new BN(giveawayId))
      .signers([user])
      .accounts({ signer: user.publicKey })
      .rpc()
      .catch((err) => console.log(err));

    const giveaway = await program.account.solGiveaway.fetch(giveawayPDA);

    expect(giveaway.winners.length).to.equal(
      giveaway.winnersAmount.toNumber() - 2
    );
  });

  it("repos unclaimed sol", async () => {
    await program.methods
      .repoSolGiveaway(new BN(giveawayId))
      .signers([manager])
      .accountsPartial({
        signer: manager.publicKey,
        destinationAccount: owner.publicKey,
      })
      .rpc()
      .catch((err) => console.log(err));

    const giveaway = await program.account.solGiveaway.fetch(giveawayPDA);

    expect(giveaway.winners.length).to.equal(0);
  });

  it("It fails to pay out sol 2", async () => {
    try {
      await program.methods
        .payoutSolGiveaway(new BN(giveawayId))
        .signers([owner])
        .accountsPartial({
          signer: owner.publicKey,
          winnerAccount: user.publicKey,
        })
        .rpc();
    } catch (err) {
      expect(err);
    }
  });
});
