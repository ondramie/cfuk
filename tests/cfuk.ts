import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Cfuk } from "../target/types/cfuk";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("cfuk", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Cfuk as Program<Cfuk>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });


  it("Creates the CFUK token", async () => {
    // Generate a new keypair for the token mint
    const mintKeypair = anchor.web3.Keypair.generate();
    
    // Initialize token
    await program.methods
      .initializeToken(new anchor.BN(1000000000)) // 1 billion tokens
      .accounts({
        mint: mintKeypair.publicKey,
        authority: provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Token created! Mint address:", mintKeypair.publicKey.toString());
  });

});
