import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { Connection, Transaction } from "@solana/web3.js"
import { expect } from "chai"
import { ClosingAccounts } from "../target/types/closing_accounts"

describe("closing-accounts", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const wallet = anchor.workspace.ClosingAccounts.provider.wallet
  const program = anchor.workspace.ClosingAccounts as Program<ClosingAccounts>
  const user = anchor.web3.Keypair.generate()
  const userTwo = anchor.web3.Keypair.generate()
  const userThree = anchor.web3.Keypair.generate()
  const userFour = anchor.web3.Keypair.generate()

  it("Init One", async () => {
    await program.methods
      .initialize()
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc()
  })

  it("Insecure Close", async () => {
    const ix1 = await program.methods
      .insecureClose()
      .accounts({
        user: user.publicKey,
        destination: wallet.publicKey,
      })
      .instruction()

    const ix2 = anchor.web3.SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: user.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(
        8 + 32
      ),
    })

    const tx = new Transaction().add(ix1, ix2)

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
      wallet.payer,
      // user,
    ])

    const account = await program.account.user.fetch(user.publicKey)
    console.log(account.authority.toString())
  })

  it("Init Two", async () => {
    await program.methods
      .initialize()
      .accounts({
        user: userTwo.publicKey,
      })
      .signers([userTwo])
      .rpc()
  })

  it("Insecure Close", async () => {
    const ix1 = await program.methods
      .insecureCloseStill()
      .accounts({
        user: userTwo.publicKey,
        destination: wallet.publicKey,
      })
      .instruction()

    const ix2 = anchor.web3.SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: userTwo.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(
        8 + 32
      ),
    })

    const tx = new Transaction().add(ix1, ix2)

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
      wallet.payer,
      // userTwo,
    ])

    const account = await program.account.user.fetch(userTwo.publicKey)
    console.log(account)
  })

  it("Init Three", async () => {
    await program.methods
      .initialize()
      .accounts({
        user: userThree.publicKey,
      })
      .signers([userThree])
      .rpc()
  })

  it("Insecure Close", async () => {
    const ix1 = await program.methods
      .insecureCloseStillStill()
      .accounts({
        user: userThree.publicKey,
        destination: wallet.publicKey,
      })
      .instruction()

    const ix2 = anchor.web3.SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: userThree.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(
        8 + 32
      ),
    })

    const tx = new Transaction().add(ix1, ix2)

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
      wallet.payer,
      // userTwo,
    ])

    const account = await program.account.user.fetch(userThree.publicKey)
    console.log(account.authority.toString())
  })

  // not working
  it("Secure Close", async () => {
    // not changing discriminator
    const ix1 = await program.methods
      .insecureCloseStillStill()
      .accounts({
        user: userThree.publicKey,
        destination: wallet.publicKey,
      })
      .instruction()

    const ix2 = await program.methods
      .forceDefund()
      .accounts({
        user: userThree.publicKey,
        destination: wallet.publicKey,
      })
      .instruction()

    const ix3 = anchor.web3.SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: userThree.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(
        8 + 32
      ),
    })

    const tx = new Transaction().add(ix1, ix2, ix3)

    await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
      wallet.payer,
      // userTwo,
    ])

    const account = await program.account.user.fetch(userThree.publicKey)
    console.log(account.authority.toString())
  })

  it("Is initialized!", async () => {
    await program.methods
      .initialize()
      .accounts({
        user: userFour.publicKey,
      })
      .signers([userFour])
      .rpc()
  })

  it("Close", async () => {
    const ix1 = await program.methods
      .recommendedClose()
      .accounts({
        user: userFour.publicKey,
        destination: wallet.publicKey,
      })
      .instruction()

    const ix2 = anchor.web3.SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: userFour.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(
        8 + 32
      ),
    })

    const tx = new Transaction().add(ix1, ix2)

    try {
      await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [
        wallet.payer,
        // userTwo,
      ])
    } catch (err) {
      expect(err)
    }
  })
})
