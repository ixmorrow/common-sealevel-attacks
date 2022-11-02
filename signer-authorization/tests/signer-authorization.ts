import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { expect } from "chai"
import { SignerAuthorization } from "../target/types/signer_authorization"

describe("signer-authorization", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace
    .SignerAuthorization as Program<SignerAuthorization>

  const keypair = anchor.web3.Keypair.generate()

  before(async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        keypair.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    )
  })

  it("insecure with signer", async () => {
    const tx = await program.methods
      .insecure()
      .accounts({
        authority: keypair.publicKey,
      })
      .transaction()

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
      keypair,
    ])
  })

  it("insecure without signer", async () => {
    await program.methods
      .insecure()
      .accounts({
        authority: keypair.publicKey,
      })
      .rpc()
  })

  it("secure with signer", async () => {
    const tx = await program.methods
      .secure()
      .accounts({
        authority: keypair.publicKey,
      })
      .transaction()

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
      keypair,
    ])
  })

  it("secure without signer", async () => {
    try {
      await program.methods
        .secure()
        .accounts({
          authority: keypair.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      console.log(err)
    }
  })

  it("recommended with signer", async () => {
    const tx = await program.methods
      .recommended()
      .accounts({
        authority: keypair.publicKey,
      })
      .transaction()

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
      keypair,
    ])
  })

  it("recommended without signer", async () => {
    try {
      await program.methods
        .recommended()
        .accounts({
          authority: keypair.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      console.log(err)
    }
  })
})
