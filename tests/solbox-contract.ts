import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolboxContract } from "../target/types/solbox_contract";
import { assert, expect } from "chai";

describe("solbox-contract-test", () => {
  // Configure the provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolboxContractTest as Program<SolboxContract>;

  let userAccount = anchor.web3.Keypair.generate();
  let referralAccount1 = anchor.web3.Keypair.generate();
  let referralAccount2 = anchor.web3.Keypair.generate();
  let referralAccount3 = anchor.web3.Keypair.generate(); // This one should trigger spillover

  before(async () => {
    // Fund user account for transactions
    const airdropSig = await provider.connection.requestAirdrop(
      userAccount.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);
  });

  it("Initializes the contract successfully", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .rpc();
    
    console.log("✅ Contract Initialized - Transaction Signature:", tx);
  });

  it("Registers a new user", async () => {
    const tx = await program.methods
      .registerUser("TestUser")
      .accounts({
        user: userAccount.publicKey,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userAccount])
      .rpc();

    console.log("✅ User Registered - Transaction Signature:", tx);

    // Fetch user account data
    const userData = await program.account.user.fetch(userAccount.publicKey);
    expect(userData.username).to.equal("TestUser");
  });

  it("Places first referral", async () => {
    const tx = await program.methods
      .placeReferral(userAccount.publicKey)
      .accounts({
        referralAccount: referralAccount1.publicKey,
        user: userAccount.publicKey,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([referralAccount1])
      .rpc();

    console.log("✅ First Referral Placed - Transaction Signature:", tx);

    // Fetch referral data
    const referralData = await program.account.referral.fetch(referralAccount1.publicKey);
    assert(referralData.referredBy.equals(userAccount.publicKey), "Referral mismatch");
  });

  it("Places second referral", async () => {
    const tx = await program.methods
      .placeReferral(userAccount.publicKey)
      .accounts({
        referralAccount: referralAccount2.publicKey,
        user: userAccount.publicKey,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([referralAccount2])
      .rpc();

    console.log("✅ Second Referral Placed - Transaction Signature:", tx);
  });

  it("Fails on third referral due to spillover", async () => {
    try {
      await program.methods
        .placeReferral(userAccount.publicKey)
        .accounts({
          referralAccount: referralAccount3.publicKey,
          user: userAccount.publicKey,
          payer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([referralAccount3])
        .rpc();

      assert.fail("Referral should not be placed due to spillover limit");
    } catch (err) {
      expect(err.message).to.include("No available slot for referral placement");
      console.log("✅ Spillover logic correctly prevented additional referral");
    }
  });

  it("Fails to register an existing user", async () => {
    try {
      await program.methods
        .registerUser("TestUser")
        .accounts({
          user: userAccount.publicKey,
          payer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([userAccount])
        .rpc();
      assert.fail("User should not be able to register twice");
    } catch (err) {
      expect(err.message).to.include("User already exists");
      console.log("✅ Properly prevented duplicate registration");
    }
  });
});
