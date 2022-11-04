import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { OwnerCheck } from "../target/types/owner_check"
import { Clone } from "../target/types/clone"
import { expect } from "chai"

describe("owner-check", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.OwnerCheck as Program<OwnerCheck>
  const programClone = anchor.workspace.Clone as Program<Clone>

  const account = anchor.web3.Keypair.generate()
  const accountClone = anchor.web3.Keypair.generate()

  it("Initialize Account", async () => {
    await program.methods
      .initialize()
      .accounts({
        data: account.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([account])
      .rpc()
  })

  it("Initialize Account Clone", async () => {
    await programClone.methods
      .initialize()
      .accounts({
        data: accountClone.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([accountClone])
      .rpc()
  })

  it("insecure", async () => {
    await program.methods
      .insecure()
      .accounts({
        data: account.publicKey,
      })
      .rpc()
  })

  it("insecure", async () => {
    await program.methods
      .insecure()
      .accounts({
        data: accountClone.publicKey,
      })
      .rpc()
  })

  it("secure", async () => {
    await program.methods
      .secure()
      .accounts({
        data: account.publicKey,
      })
      .rpc()
  })

  it("secure", async () => {
    try {
      await program.methods
        .secure()
        .accounts({
          data: accountClone.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      // console.log(err)
    }
  })

  it("recommended", async () => {
    await program.methods
      .recommended()
      .accounts({
        data: account.publicKey,
      })
      .rpc()
  })

  it("recommended", async () => {
    try {
      await program.methods
        .recommended()
        .accounts({
          data: accountClone.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      // console.log(err)
    }
  })
})
