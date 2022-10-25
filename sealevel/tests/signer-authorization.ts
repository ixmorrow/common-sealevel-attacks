import * as anchor from "@project-serum/anchor"
import { AnchorError, Program, ProgramError } from "@project-serum/anchor"
import { LAMPORTS_PER_SOL } from "@solana/web3.js"
import { assert, expect } from "chai"
import { SignerAuthorization } from "../target/types/signer_authorization"

describe("sealevel", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace
    .SignerAuthorization as Program<SignerAuthorization>

  const kp = anchor.web3.Keypair.generate()

  before(async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        kp.publicKey,
        1 * LAMPORTS_PER_SOL
      ),
      "confirmed"
    )
  })

  it("insecure with signer", async () => {
    const tx = await program.methods
      .insecure()
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .transaction()

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [kp])
  })

  it("insecure without signer", async () => {
    await program.methods
      .insecure()
      .accounts({
        authority: kp.publicKey,
      })
      .rpc()
  })

  it("secure with signer", async () => {
    const tx = await program.methods
      .secure()
      .accounts({
        authority: kp.publicKey,
      })
      .transaction()

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [kp])
  })

  it("secure without signer", async () => {
    try {
      await program.methods
        .secure()
        .accounts({
          authority: kp.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
    }
  })

  it("recommended with signer", async () => {
    const tx = await program.methods
      .recommended()
      .accounts({
        authority: kp.publicKey,
      })
      .transaction()

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [kp])
  })

  it("recommended without signer", async () => {
    try {
      await program.methods
        .recommended()
        .accounts({
          authority: kp.publicKey,
        })
        .rpc()
      assert.ok(false)
    } catch (err) {
      expect(err)
    }
  })
})
