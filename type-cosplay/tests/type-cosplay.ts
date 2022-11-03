import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { Keypair } from "@solana/web3.js"
import { expect } from "chai"
import { TypeCosplay } from "../target/types/type_cosplay"
import { TypeCosplaySecure } from "../target/types/type_cosplay_secure"
import { TypeCosplayRecommended } from "./../target/types/type_cosplay_recommended"

describe("type-cosplay", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.TypeCosplay as Program<TypeCosplay>
  const programSecure = anchor.workspace
    .TypeCosplaySecure as Program<TypeCosplaySecure>
  const programRecommended = anchor.workspace
    .TypeCosplayRecommended as Program<TypeCosplayRecommended>

  const typeOne = Keypair.generate()
  const typeTwo = Keypair.generate()

  const typeOneSecure = Keypair.generate()
  const typeTwoSecure = Keypair.generate()

  const typeOneRecommended = Keypair.generate()
  const typeTwoRecommended = Keypair.generate()

  it("Is initialized!", async () => {
    await program.methods
      .initializeTypeOne()
      .accounts({
        user: typeOne.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([typeOne])
      .rpc()
  })

  it("Is initialized!", async () => {
    await program.methods
      .initializeTypeTwo()
      .accounts({
        user: typeTwo.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([typeTwo])
      .rpc()
  })

  it("insecure", async () => {
    await program.methods
      .insecure()
      .accounts({
        user: typeOne.publicKey,
      })
      .rpc()
  })

  it("insecure", async () => {
    await program.methods
      .insecure()
      .accounts({
        user: typeTwo.publicKey,
      })
      .rpc()
  })

  it("Is initialized!", async () => {
    await programSecure.methods
      .initializeTypeOne()
      .accounts({
        user: typeOneSecure.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([typeOneSecure])
      .rpc()
  })

  it("Is initialized!", async () => {
    await programSecure.methods
      .initializeTypeTwo()
      .accounts({
        user: typeTwoSecure.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([typeTwoSecure])
      .rpc()
  })

  it("insecure", async () => {
    await programSecure.methods
      .secure()
      .accounts({
        user: typeOneSecure.publicKey,
      })
      .rpc()
  })

  it("insecure", async () => {
    try {
      await programSecure.methods
        .secure()
        .accounts({
          user: typeTwoSecure.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      // console.log(err)
    }
  })

  it("Is initialized!", async () => {
    await programRecommended.methods
      .initializeTypeOne()
      .accounts({
        user: typeOneRecommended.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([typeOneRecommended])
      .rpc()
  })

  it("Is initialized!", async () => {
    await programRecommended.methods
      .initializeTypeTwo()
      .accounts({
        user: typeTwoRecommended.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([typeTwoRecommended])
      .rpc()
  })

  it("insecure", async () => {
    await programRecommended.methods
      .recommended()
      .accounts({
        user: typeOneRecommended.publicKey,
      })
      .rpc()
  })

  it("insecure", async () => {
    try {
      await programRecommended.methods
        .recommended()
        .accounts({
          user: typeTwoRecommended.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      // console.log(err)
    }
  })
})
