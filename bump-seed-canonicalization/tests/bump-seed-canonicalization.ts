import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import {
  createProgramAddressSync,
  findProgramAddressSync,
} from "@project-serum/anchor/dist/cjs/utils/pubkey"
import { expect } from "chai"
import { BumpSeedCanonicalization } from "../target/types/bump_seed_canonicalization"

describe("bump-seed-canonicalization", () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace
    .BumpSeedCanonicalization as Program<BumpSeedCanonicalization>

  const [pda, bump] = findProgramAddressSync([], program.programId)
  console.log(pda.toString())
  console.log(bump)

  const pda_not_canonical = createProgramAddressSync(
    [Buffer.from([251])],
    program.programId
  )
  console.log(pda_not_canonical.toString())

  it("Initialize PDA using canonical bump", async () => {
    await program.methods.initialize(bump).accounts({ pda: pda }).rpc()
  })

  it("Initialize PDA using non-canonical bump", async () => {
    await program.methods
      .initialize(251)
      .accounts({ pda: pda_not_canonical })
      .rpc()
  })

  it("insecure", async () => {
    await program.methods.insecure(bump).accounts({ pda: pda }).rpc()
  })

  it("insecure", async () => {
    await program.methods
      .insecure(251)
      .accounts({ pda: pda_not_canonical })
      .rpc()
  })

  it("secure", async () => {
    await program.methods.secure(bump).accounts({ pda: pda }).rpc()
  })

  it("secure", async () => {
    try {
      await program.methods
        .secure(251)
        .accounts({ pda: pda_not_canonical })
        .rpc()
    } catch (err) {
      expect(err)
      console.log(err)
    }
  })

  it("recommended", async () => {
    await program.methods.recommended().accounts({ pda: pda }).rpc()
  })

  it("recommended", async () => {
    try {
      await program.methods
        .recommended()
        .accounts({ pda: pda_not_canonical })
        .rpc()
    } catch (err) {
      expect(err)
      console.log(err)
    }
  })
})
