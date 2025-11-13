import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("cfuk", () => {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Cast the program to any to bypass type checking
  const program = anchor.workspace.Cfuk as unknown as Program;

  it("Initializes the mint", async () => {
    const mintKeypair = anchor.web3.Keypair.generate();

    try {
      const accounts = {
        mint: mintKeypair.publicKey,
        mintAuthority: provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      };

      const tx = await program.methods
        .initializeMint()
        .accountsStrict(accounts)
        .signers([mintKeypair])
        .rpc();

      console.log("Your transaction signature", tx);
      console.log("Mint address:", mintKeypair.publicKey.toString());
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
});
