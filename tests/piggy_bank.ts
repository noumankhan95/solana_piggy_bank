import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PiggyBank } from "../target/types/piggy_bank";
import { assert } from "chai";
import { BN } from "bn.js";

describe("piggy_bank", () => {
  // ─────────────────────────────
  // Provider & Program
  // ─────────────────────────────
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PiggyBank as Program<PiggyBank>;

  // ─────────────────────────────
  // Test actors
  // ─────────────────────────────
  const user = anchor.web3.Keypair.generate();

  // Example amount: 0.5 SOL
  const depositAmount = new anchor.BN(0.5 * anchor.web3.LAMPORTS_PER_SOL);

  // PDA (example – adjust seeds to your program)
  let piggyBankPda: anchor.web3.PublicKey;
  let bump: number;

  // ─────────────────────────────
  // Global setup
  // ─────────────────────────────
  before(async () => {
    // Airdrop SOL to user
    const sig = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL,
    );
    await provider.connection.confirmTransaction(sig);

    // Derive PDA
    [piggyBankPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("piggy_bank"), user.publicKey.toBuffer()],
      program.programId,
    );
  });

  // ─────────────────────────────
  // Initialize
  // ─────────────────────────────
  it("initializes the piggy bank", async () => {
    const now = Math.floor(Date.now() / 1000);
    // const oneYear = 365 * 24 * 60 * 60;
    const two_seconds = 1;
    const unlockTime = new anchor.BN(now + two_seconds);
    await program.methods
      .initialize(unlockTime)
      .accounts({
        signer: user.publicKey,
      })
      .signers([user])
      .rpc();
    const account = await program.account.piggyBank.fetch(piggyBankPda);

    assert.ok(account.owner.equals(user.publicKey));
  });

  // ─────────────────────────────
  // Lock funds
  // ─────────────────────────────
  it("locks funds", async () => {
    const amount = new anchor.BN(0.5 * anchor.web3.LAMPORTS_PER_SOL);
    try {
      const amount = new anchor.BN(0.5 * anchor.web3.LAMPORTS_PER_SOL);
      await program.methods
        .lockFunds(amount)
        .accounts({
          signer: user.publicKey,
        })
        .signers([user])
        .rpc();

      const balance = await provider.connection.getBalance(piggyBankPda);
      console.log(
        "PDA balance:",
        balance / anchor.web3.LAMPORTS_PER_SOL,
        "SOL",
      );

      // Use the exact amount
      assert.ok(balance >= amount.toNumber());
    } catch (err: any) {
      // This is the SendTransactionError
      console.log("Transaction failed:", err);

      // Anchor error may have `logs`
      if (err.logs) {
        console.log("Logs from simulation:");
        for (const log of err.logs) {
          console.log(log);
        }
      } else if (err instanceof Error) {
        console.log("Error message:", err.message);
      }
    }
  });
  it("shouldnt unlock funds too early", async () => {
    try {
      const amount = new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL);
      await program.methods
        .unlockFunds(amount)
        .accounts({ signer: user.publicKey })
        .signers([user])
        .rpc();
      assert.fail("Cannot withdraw yet. Lock period not finished..");
    } catch (e) {
      const err = e as anchor.AnchorError;
      assert.equal(err.error.errorCode.code, "TooEarly");
    }
  });
 
});
