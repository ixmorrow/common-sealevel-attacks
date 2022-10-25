import * as anchor from "@project-serum/anchor"
import * as spl from "@solana/spl-token"
import { AnchorError, Program, ProgramError } from "@project-serum/anchor"
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import { assert, expect } from "chai"
import { OwnerCheck } from "../target/types/owner_check"

describe("sealevel", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.OwnerCheck as Program<OwnerCheck>
  const connection = anchor.getProvider().connection
  const wallet = anchor.workspace.OwnerCheck.provider.wallet

  const kp = anchor.web3.Keypair.generate()

  let mint: PublicKey
  let account: PublicKey

  before(async () => {
    await connection.confirmTransaction(
      await connection.requestAirdrop(kp.publicKey, 1 * LAMPORTS_PER_SOL),
      "confirmed"
    )

    mint = await spl.createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      1
    )

    account = await spl.createAccount(
      connection,
      wallet.payer,
      mint,
      wallet.publicKey
    )
  })

  // it("fake account", async () => {
  //   const tx = await program.methods
  //     .init()
  //     .accounts({
  //       test: kp.publicKey,
  //       authority: wallet.publicKey,
  //       mint: mint,
  //     })
  //     .signers([kp])
  //     .rpc()

  //   const account = await program.account.test.fetch(kp.publicKey)
  //   console.log(account)
  // })

  it("insecure", async () => {
    const tx = await program.methods
      .insecure()
      .accounts({
        // token: kp.publicKey,
        token: account,
        authority: wallet.publicKey,
      })
      .rpc()
  })

  it("secure", async () => {
    const tx = await program.methods
      .secure()
      .accounts({
        // token: kp.publicKey,
        token: account,
        authority: wallet.publicKey,
      })
      .rpc()
  })

  it("recommended", async () => {
    const tx = await program.methods
      .secure()
      .accounts({
        // token: kp.publicKey,
        token: account,
        authority: wallet.publicKey,
      })
      .rpc()
  })
})
