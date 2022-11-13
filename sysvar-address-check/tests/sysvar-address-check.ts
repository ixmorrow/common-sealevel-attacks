import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { SYSVAR_RENT_PUBKEY } from "@solana/web3.js"
import { expect } from "chai"
import { SysvarAddressCheck } from "../target/types/sysvar_address_check"

describe("sysvar-address-check", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace
    .SysvarAddressCheck as Program<SysvarAddressCheck>

  const keypair = anchor.web3.Keypair.generate()

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .insecure()
      .accounts({ rent: SYSVAR_RENT_PUBKEY })
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .secure()
      .accounts({ rent: SYSVAR_RENT_PUBKEY })
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .recommended()
      .accounts({ rent: SYSVAR_RENT_PUBKEY })
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .insecure()
      .accounts({ rent: keypair.publicKey })
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Is initialized!", async () => {
    try {
      // Add your test here.
      const tx = await program.methods
        .secure()
        .accounts({ rent: keypair.publicKey })
        .rpc()
      console.log("Your transaction signature", tx)
    } catch (err) {
      expect(err)
      console.log(err)
    }
  })

  it("Is initialized!", async () => {
    try {
      // Add your test here.
      const tx = await program.methods
        .recommended()
        .accounts({ rent: keypair.publicKey })
        .rpc()
      console.log("Your transaction signature", tx)
    } catch (err) {
      expect(err)
      console.log(err)
    }
  })
})
