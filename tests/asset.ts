import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Asset } from "../target/types/asset";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

// Token program ID constant
const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

describe("asset", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.asset as Program<Asset>;
  const provider = anchor.getProvider();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Creates a token", async () => {
    const tokenMint = Keypair.generate();
    const authority = provider.wallet;

    // Find the token manager PDA
    const [tokenManager] = PublicKey.findProgramAddressSync(
      [Buffer.from("token_manager"), tokenMint.publicKey.toBuffer()],
      program.programId
    );

    const params = {
      name: "Test Token",
      symbol: "TEST",
      uri: "https://example.com/token.json",
      decimals: 9,
      maxSupply: new anchor.BN("1000000000000000000"), // 1 billion tokens
      isMintable: true,
      isFreezable: true,
      isBurnable: true,
      transferFeeBasisPoints: 100, // 1%
      feeRecipient: null,
    };

    const tx = await program.methods
      .createToken(params)
      .accountsPartial({
        authority: authority.publicKey,
        tokenManager: tokenManager,
        tokenMint: tokenMint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([tokenMint])
      .rpc();

    console.log("Create token transaction signature:", tx);

    // Verify the token manager was created correctly
    const tokenManagerAccount = await program.account.tokenManager.fetch(tokenManager);
    console.log("Token Manager:", tokenManagerAccount);

    // Assertions
    expect(tokenManagerAccount.name).to.equal("Test Token");
    expect(tokenManagerAccount.symbol).to.equal("TEST");
    expect(tokenManagerAccount.decimals).to.equal(9);
    expect(tokenManagerAccount.isMintable).to.be.true;
    expect(tokenManagerAccount.isFreezable).to.be.true;
    expect(tokenManagerAccount.isBurnable).to.be.true;
    expect(tokenManagerAccount.transferFeeBasisPoints).to.equal(100);
    expect(tokenManagerAccount.currentSupply.toString()).to.equal("0");
  });
});
