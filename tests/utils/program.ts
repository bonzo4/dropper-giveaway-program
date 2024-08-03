import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DropperGiveaway } from "../../target/types/dropper_giveaway";

export function getProgram() {
    // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DropperGiveaway as Program<DropperGiveaway>;
  return program;
}