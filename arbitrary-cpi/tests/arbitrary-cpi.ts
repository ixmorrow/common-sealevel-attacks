import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { ArbitraryCpi } from "../target/types/arbitrary_cpi"
import { CalleeProgram } from "../target/types/callee_program"
import { CalleeProgramFake } from "./../target/types/callee_program_fake"

describe("arbitrary-cpi", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.ArbitraryCpi as Program<ArbitraryCpi>
  const calleeProgram = anchor.workspace.CalleeProgram as Program<CalleeProgram>
  const calleeProgramFake = anchor.workspace
    .CalleeProgramFake as Program<CalleeProgramFake>

  const data = anchor.web3.Keypair.generate()
  const dataFake = anchor.web3.Keypair.generate()

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .cpi()
      .accounts({
        data: data.publicKey,
        user: provider.wallet.publicKey,
        calleeProgram: calleeProgram.programId,
      })
      .signers([data])
      .rpc()
    console.log("Your transaction signature", tx)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .cpi()
      .accounts({
        data: dataFake.publicKey,
        user: provider.wallet.publicKey,
        calleeProgram: calleeProgramFake.programId,
      })
      .signers([dataFake])
      .rpc()
    console.log("Your transaction signature", tx)
  })
})
