import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PiggyBank } from "../target/types/piggy_bank";

describe("piggy_bank", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.piggyBank as Program<PiggyBank>;
  const user = anchor.web3.Keypair.generate();
  it("Is initialized!", async () => {
    const amount = new anchor.BN(0.5 * anchor.web3.LAMPORTS_PER_SOL);
    const tx = await program.methods
      .initialize(amount)
      .accounts({ signer: user.publicKey })
      .signers([user])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
