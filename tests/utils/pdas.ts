import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DropperGiveaway } from "../../target/types/dropper_giveaway";
import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";

export function getSolGiveawayPda(
  program: Program<DropperGiveaway>,
  giveawayId: number,
  signer: PublicKey
) {
  const [giveawayPDA] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("sol_giveaway"),
      new BN(giveawayId).toBuffer("le", 8),
      signer.toBuffer(),
    ],
    program.programId
  );

  return giveawayPDA;
}

export function getSplGiveawayPda(
  program: Program<DropperGiveaway>,
  giveawayId: number,
  signer: PublicKey
) {
  const [giveawayPDA] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("spl_giveaway"),
      new BN(giveawayId).toBuffer("le", 8),
      signer.toBuffer(),
    ],
    program.programId
  );

  return giveawayPDA;
}
