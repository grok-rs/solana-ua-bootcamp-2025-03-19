import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import {
  airdropIfRequired,
  getCustomErrorMessage,
} from "@solana-developers/helpers";
import { systemProgramErrors } from "./system-program-errors";
import assert from "node:assert";

describe("favorites", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Writes our favourites to the blockchain", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Here's what we want to write to the blockchain
    const favoriteNumber = new anchor.BN(23);
    const favoriteColor = "red";

    // Make a transaction to write to the blockchain
    let tx: string | null = null;
    try {
      tx = await program.methods
        // Call the set_favorites instruction handler
        .setFavorites(favoriteNumber, favoriteColor)
        .accounts({
          user: user.publicKey,
          // Note that both `favorites` and `system_program` are added
          // automatically.
        })
        // Sign the transaction
        .signers([user])
        // Send the transaction to the cluster or RPC
        .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, rawError.message)
      );
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] =
      web3.PublicKey.findProgramAddressSync(
        [Buffer.from("favorites"), user.publicKey.toBuffer()],
        program.programId
      );

    // And make sure it matches!
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    assert.equal(dataFromPda.color, favoriteColor);
    assert.equal(dataFromPda.number.toNumber(), favoriteNumber.toNumber());
  });

  it("Updates the favourites on the blockchain", async () => {
    // Generate a new user keypair
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    // Airdrop SOL to the user if their balance is below 0.5 SOL, requesting up to 1 SOL
    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Step 1: Set initial favorites
    const initialNumber = new anchor.BN(23);
    const initialColor = "red";
    let tx;
    try {
      tx = await program.methods
        .setFavorites(initialNumber, initialColor)
        .accounts({ user: user.publicKey })
        .signers([user])
        .rpc();
      console.log(`Set favorites tx: ${tx}`);
    } catch (error) {
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, error.message)
      );
    }

    // Calculate the PDA for the user's favorites
    const [favoritesPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );

    // Verify initial favorites
    let data = await program.account.favorites.fetch(favoritesPda);
    assert.equal(
      data.number.toNumber(),
      initialNumber.toNumber(),
      "Initial number mismatch"
    );
    assert.equal(data.color, initialColor, "Initial color mismatch");

    // Step 2: Update only the number
    const updatedNumber = new anchor.BN(100);
    try {
      tx = await program.methods
        .updateFavorites({ number: updatedNumber, color: null })
        .accounts({ user: user.publicKey })
        .signers([user])
        .rpc();
      console.log(`Update number tx: ${tx}`);
    } catch (error) {
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, error.message)
      );
    }

    // Verify number updated, color unchanged
    data = await program.account.favorites.fetch(favoritesPda);
    assert.equal(
      data.number.toNumber(),
      updatedNumber.toNumber(),
      "Number update failed"
    );
    assert.equal(data.color, initialColor, "Color should not have changed");

    // Step 3: Update only the color
    const updatedColor = "green";
    try {
      tx = await program.methods
        .updateFavorites({ number: null, color: updatedColor })
        .accounts({ user: user.publicKey })
        .signers([user])
        .rpc();
      console.log(`Update color tx: ${tx}`);
    } catch (error) {
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, error.message)
      );
    }

    // Verify color updated, number unchanged
    data = await program.account.favorites.fetch(favoritesPda);
    assert.equal(
      data.number.toNumber(),
      updatedNumber.toNumber(),
      "Number should not have changed"
    );
    assert.equal(data.color, updatedColor, "Color update failed");

    // Step 4: Update both number and color
    const finalNumber = new anchor.BN(42);
    const finalColor = "blue";
    try {
      tx = await program.methods
        .updateFavorites({ number: finalNumber, color: finalColor })
        .accounts({ user: user.publicKey })
        .signers([user])
        .rpc();
      console.log(`Update both tx: ${tx}`);
    } catch (error) {
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, error.message)
      );
    }

    // Verify both updated
    data = await program.account.favorites.fetch(favoritesPda);
    assert.equal(
      data.number.toNumber(),
      finalNumber.toNumber(),
      "Final number update failed"
    );
    assert.equal(data.color, finalColor, "Final color update failed");
  });
});
