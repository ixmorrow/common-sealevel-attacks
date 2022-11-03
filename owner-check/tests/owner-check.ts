import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { OwnerCheck } from "../target/types/owner_check"
import { Clone } from "../target/types/clone"
import { expect } from "chai"

describe("owner-check", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.OwnerCheck as Program<OwnerCheck>
  const programClone = anchor.workspace.Clone as Program<Clone>

  const keypair = anchor.web3.Keypair.generate()
  const keypairClone = anchor.web3.Keypair.generate()

  it("Initialize Account", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        user: keypair.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([keypair])
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await programClone.methods
      .initialize()
      .accounts({
        user: keypairClone.publicKey,
        payer: provider.wallet.publicKey,
      })
      .signers([keypairClone])
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("insecure no owner check", async () => {
    await program.methods
      .insecure()
      .accounts({
        user: keypair.publicKey,
      })
      .rpc()
  })

  it("insecure no owner check", async () => {
    await program.methods
      .insecure()
      .accounts({
        user: keypairClone.publicKey,
      })
      .rpc()
  })

  it("secure owner check", async () => {
    await program.methods
      .secure()
      .accounts({
        user: keypair.publicKey,
      })
      .rpc()
  })

  it("secure owner check", async () => {
    try {
      await program.methods
        .secure()
        .accounts({
          user: keypairClone.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      // console.log(err)
    }
  })

  it("recommended owner check", async () => {
    await program.methods
      .recommended()
      .accounts({
        user: keypair.publicKey,
      })
      .rpc()
  })

  it("recommended owner check", async () => {
    try {
      await program.methods
        .recommended()
        .accounts({
          user: keypairClone.publicKey,
        })
        .rpc()
    } catch (err) {
      expect(err)
      // console.log(err)
    }
  })
})
