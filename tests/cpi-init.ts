import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { CpiInit } from "../target/types/cpi_init"
import { CalleeProgram } from "../target/types/callee_program"
import { PublicKey } from "@solana/web3.js"

describe("cpi-init", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.CpiInit as Program<CpiInit>
  const calleeProgram = anchor.workspace.CalleeProgram as Program<CalleeProgram>

  const [PDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("SEED")],
    calleeProgram.programId
  )

  it("Is initialized!", async () => {
    const tx = await program.methods
      .cpi()
      .accounts({
        data: PDA,
        user: provider.wallet.publicKey,
        calleeProgram: calleeProgram.programId,
      })
      .rpc()
    console.log("Your transaction signature", tx)
  })
})
