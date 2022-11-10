import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { expect } from "chai"
import { Initialization } from "../target/types/initialization"

describe("initialization", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const wallet = anchor.workspace.Initialization.provider.wallet
  const walletTwo = anchor.web3.Keypair.generate()

  const program = anchor.workspace.Initialization as Program<Initialization>
  const userInsecure = anchor.web3.Keypair.generate()
  const userSecure = anchor.web3.Keypair.generate()
  const userRecommended = anchor.web3.Keypair.generate()

  before(async () => {
    const tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: userInsecure.publicKey,
        space: 32,
        lamports: await provider.connection.getMinimumBalanceForRentExemption(
          32
        ),
        programId: program.programId,
      }),
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: userSecure.publicKey,
        space: 1 + 32,
        lamports: await provider.connection.getMinimumBalanceForRentExemption(
          1 + 32
        ),
        programId: program.programId,
      })
    )

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
      wallet.payer,
      userInsecure,
      userSecure,
    ])

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        walletTwo.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    )
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .insecure()
      .accounts({
        user: userInsecure.publicKey,
      })
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .insecure()
      .accounts({
        user: userInsecure.publicKey,
        authority: walletTwo.publicKey,
      })
      .transaction()
    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
      walletTwo,
    ])
  })

  it("Is initialized!", async () => {
    const tx = await program.methods
      .secure()
      .accounts({
        user: userSecure.publicKey,
      })
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Is initialized!", async () => {
    try {
      // Add your test here.
      const tx = await program.methods
        .secure()
        .accounts({
          user: userSecure.publicKey,
          authority: walletTwo.publicKey,
        })
        .transaction()
      await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
        walletTwo,
      ])
    } catch (err) {
      expect(err)
      console.log(err)
    }
  })

  it("Is initialized!", async () => {
    const tx = await program.methods
      .recommended()
      .accounts({
        user: userRecommended.publicKey,
      })
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Is initialized!", async () => {
    try {
      // Add your test here.
      const tx = await program.methods
        .recommended()
        .accounts({
          user: userRecommended.publicKey,
          authority: walletTwo.publicKey,
        })
        .transaction()
      await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
        walletTwo,
      ])
    } catch (err) {
      expect(err)
      console.log(err)
    }
  })
})
