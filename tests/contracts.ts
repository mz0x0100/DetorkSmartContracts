import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("escrow", () => {
  it("initialize_escrow", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Escrow;

    const client = anchor.web3.Keypair.generate();
    const freelancer = anchor.web3.Keypair.generate();

    const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("detork-escrow"),
        client.publicKey.toBuffer(),
        freelancer.publicKey.toBuffer(),
      ],
      program.programId
    );
    const amount = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL * 0.001);

    await program.methods
      .initialize(amount)
      .accounts({
        client: client.publicKey,
        freelancer: freelancer.publicKey,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([client])
      .rpc();
  });
});
