import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("escrow", () => {
  it("initialize_escrow", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Escrow;

    const client = anchor.web3.Keypair.generate();
    const freelancer = anchor.web3.Keypair.generate();

    // Airdrop to client
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        client.publicKey,
        anchor.web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    );

    const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("detork-escrow"), // make sure this matches the Rust seeds exactly
        client.publicKey.toBuffer(),
        freelancer.publicKey.toBuffer(),
      ],
      program.programId
    );

    const amount = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL / 1000); // 0.001 SOL
    const orderId = "3949023543adf3e";

    await program.methods
      .initialize(amount, orderId)
      .accounts({
        client: client.publicKey,
        freelancer: freelancer.publicKey,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([client])
      .rpc();

    const vault = await program.account.vault.fetch(vaultPda);
    console.log("Vault state:", vault);
  });
});
