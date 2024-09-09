import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import { getProgram } from "./utils/program";
import { getSplGiveawayPda } from "./utils/pdas";
import { PublicKey } from "@solana/web3.js";
import {
  getManagerKeypair,
  getOwnerKeypair,
  getUserKeypair,
} from "./utils/wallets";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { expect } from "chai";

describe("Spl Giveaway", () => {
  const mint = new PublicKey("295D2K8jNxvvLGA3Uv1KWQ2cPD1ptHJuP44bMmUYBKzs");

  const owner = getOwnerKeypair();
  const manager = getManagerKeypair();
  const user = getUserKeypair();

  const program = getProgram();
  const giveawayId = -24;

  const giveawayPDA = getSplGiveawayPda(program, giveawayId, owner.publicKey);

  // const splAssociatedAccount = getAssociatedTokenAddressSync(
  //   mint,
  //   owner.publicKey
  // );

  it("It creates a spl giveaway", async () => {
    await program.methods
      .createSplGiveaway({
        giveawayId: new BN(giveawayId),
        rewardAmount: new BN(0.1 * Math.pow(10, 9)),
        winnersAmount: new BN(3),
      })
      .signers([owner])
      .accounts({
        tokenMint: mint,
      })
      .rpc()
      .catch((err) => console.log(err));

    const giveaway = await program.account.splGiveaway.fetch(giveawayPDA);

    expect(giveaway);
  });

  it("It fails to pay out spl", async () => {
    try {
      await program.methods
        .payoutSplGiveaway(new BN(giveawayId), owner.publicKey)
        .signers([user])
        .accounts({
          signer: user.publicKey,
          winnerAccount: user.publicKey,
          tokenMint: mint,
        })
        .rpc();
    } catch (err) {
      expect(err);
    }
  });

  it("selects winners", async () => {
    await program.methods
      .setSplGiveawayWinners({
        winnerKeys: [user.publicKey, user.publicKey],
        giveawayId: new BN(giveawayId),
        creatorKey: owner.publicKey,
      })
      .signers([manager])
      .accounts({
        signer: manager.publicKey,
      })
      .rpc()
      .catch((err) => console.log(err));

    const giveaway = await program.account.splGiveaway.fetch(giveawayPDA);

    expect(giveaway.winners.length).to.equal(giveaway.winnersAmount.toNumber());
  });

  it("it pays out spl", async () => {
    await program.methods
      .payoutSplGiveaway(new BN(giveawayId), owner.publicKey)
      .signers([manager])
      .accounts({
        signer: manager.publicKey,
        winnerAccount: user.publicKey,
        tokenMint: mint,
      })
      .rpc()
      .catch((err) => console.log(err));

    const giveaway = await program.account.splGiveaway.fetch(giveawayPDA);

    expect(giveaway.winners.length).to.equal(
      giveaway.winnersAmount.toNumber() - 1
    );
  });

  it("it claims spl", async () => {
    await program.methods
      .claimSplGiveaway(new BN(giveawayId), owner.publicKey)
      .signers([user, owner])
      .accountsPartial({
        signer: user.publicKey,
        tokenMint: mint,
      })
      .rpc()
      .catch((err) => console.log(err));

    const giveaway = await program.account.splGiveaway.fetch(giveawayPDA);

    expect(giveaway.winners.length).to.equal(
      giveaway.winnersAmount.toNumber() - 2
    );
  });

  it("repos unclaimed spl", async () => {
    await program.methods
      .repoSplGiveaway(new BN(giveawayId), owner.publicKey)
      .signers([manager])
      .accountsPartial({
        signer: manager.publicKey,
        destinationAccount: owner.publicKey,
        tokenMint: mint,
      })
      .rpc()
      .catch((err) => console.log(err));

    const giveaway = await program.account.splGiveaway.fetch(giveawayPDA);

    expect(giveaway.winners.length).to.equal(0);
  });

  it("It fails to pay out spl 2", async () => {
    try {
      await program.methods
        .payoutSplGiveaway(new BN(giveawayId), owner.publicKey)
        .signers([manager])
        .accounts({
          signer: manager.publicKey,
          winnerAccount: user.publicKey,
          tokenMint: mint,
        })
        .rpc();
    } catch (err) {
      expect(err);
    }
  });
});
