import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { expect } from "chai"
import { DuplicateMutableAccounts } from "../target/types/duplicate_mutable_accounts"

describe("duplicate-mutable-accounts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace
    .DuplicateMutableAccounts as Program<DuplicateMutableAccounts>

  const accountOne = anchor.web3.Keypair.generate()
  const accountTwo = anchor.web3.Keypair.generate()

  it("Initialized Account One", async () => {
    await program.methods
      .initialize()
      .accounts({
        user: accountOne.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([accountOne])
      .rpc()
  })

  it("Initialized Account Two", async () => {
    await program.methods
      .initialize()
      .accounts({
        user: accountTwo.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([accountTwo])
      .rpc()
  })

  it("insecure", async () => {
    await program.methods
      .insecure(new anchor.BN(1), new anchor.BN(2))
      .accounts({
        userA: accountOne.publicKey,
        userB: accountOne.publicKey,
      })
      .rpc()

    const account = await program.account.user.fetch(accountOne.publicKey)
    console.log(account.data)
  })

  it("secure", async () => {
    try {
      await program.methods
        .secure(new anchor.BN(1), new anchor.BN(2))
        .accounts({
          userA: accountOne.publicKey,
          userB: accountOne.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      console.log(err)
    }
  })

  it("recommended", async () => {
    try {
      await program.methods
        .recommended(new anchor.BN(1), new anchor.BN(2))
        .accounts({
          userA: accountOne.publicKey,
          userB: accountOne.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      console.log(err)
    }
  })
})
