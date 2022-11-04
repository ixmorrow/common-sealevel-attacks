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

  it("insecure with signer", async () => {
    await program.methods
      .insecure()
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .rpc()
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
    await program.methods
      .secure()
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .rpc()
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
      // console.log(err)
    }
  })

  it("recommended with signer", async () => {
    await program.methods
      .recommended()
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .rpc()
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
      // console.log(err)
    }
  })
})
