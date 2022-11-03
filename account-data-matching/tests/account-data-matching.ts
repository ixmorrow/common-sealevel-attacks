import * as anchor from "@project-serum/anchor"
import * as spl from "@solana/spl-token"
import { Program } from "@project-serum/anchor"
import { AccountDataMatching } from "../target/types/account_data_matching"
import { expect } from "chai"

describe("account-data-matching", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace
    .AccountDataMatching as Program<AccountDataMatching>

  const connection = anchor.getProvider().connection
  const wallet = anchor.workspace.AccountDataMatching.provider.wallet

  const keypair = anchor.web3.Keypair.generate()

  let mint: anchor.web3.PublicKey
  let accountOne: anchor.web3.PublicKey
  let accountTwo: anchor.web3.PublicKey

  before(async () => {
    mint = await spl.createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      1
    )

    accountOne = await spl.createAccount(
      connection,
      wallet.payer,
      mint,
      wallet.publicKey
    )

    accountTwo = await spl.createAccount(
      connection,
      wallet.payer,
      mint,
      keypair.publicKey
    )
  })

  it("Insecure", async () => {
    await program.methods
      .insecure()
      .accounts({
        token: accountOne,
        authority: wallet.publicKey,
      })
      .rpc()
  })

  it("Insecure", async () => {
    await program.methods
      .insecure()
      .accounts({
        token: accountTwo,
        authority: wallet.publicKey,
      })
      .rpc()
  })

  it("Secure", async () => {
    await program.methods
      .secure()
      .accounts({
        token: accountOne,
        authority: wallet.publicKey,
      })
      .rpc()
  })

  it("Secure", async () => {
    try {
      await program.methods
        .secure()
        .accounts({
          token: accountTwo,
          authority: wallet.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      // console.log(err)
    }
  })

  it("Recommended", async () => {
    await program.methods
      .recommended()
      .accounts({
        token: accountOne,
        authority: wallet.publicKey,
      })
      .rpc()
  })

  it("Recommended", async () => {
    try {
      await program.methods
        .recommended()
        .accounts({
          token: accountTwo,
          authority: wallet.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      // console.log(err)
    }
  })
})
