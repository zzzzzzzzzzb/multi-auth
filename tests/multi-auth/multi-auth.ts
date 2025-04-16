import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { MultiAuthProgram } from "../../target/types/multi_auth_program";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .MultiAuthProgram as anchor.Program<MultiAuthProgram>;

  let mockNFTAccount = new web3.Keypair();
  let chain1_id = 1;
  let nft1_addr = mockNFTAccount.publicKey;
  let nft1_id = 1;

  let [auth_account1] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      program.provider.publicKey.toBuffer(),
      nft1_addr.toBuffer(),
      new BN(nft1_id).toArrayLike(Buffer, "le", 8),
      new BN(chain1_id).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );
  let [auth_account2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      program.provider.publicKey.toBuffer(),
      nft1_addr.toBuffer(),
      new BN(nft1_id).toArrayLike(Buffer, "le", 8),
      new BN(chain1_id).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  it("register", async () => {
    // Send transaction
    const txHash = await program.methods
      .register(nft1_addr, new BN(nft1_id), new BN(chain1_id))
      .accounts({
        authStatusAccount: auth_account1,
        owner: program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      // .signers([program.provider.wallet.payer])
      .rpc();
    console.log(`register Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await program.provider.connection.confirmTransaction(txHash);

    // Fetch the created account
    const data = await program.account.authStatusAccount.fetch(auth_account1);

    console.log("On-chain data is:", data);
    const preBalance = await program.provider.connection.getBalance(
      program.provider.publicKey
    );
    console.log("pre balance is :", preBalance);
    // Check whether the data on-chain is equal to local 'data'
    assert.equal(data.srcNft.toBase58(), nft1_addr.toBase58());

    const unregisterHash = await program.methods
      .unregister(nft1_addr, new BN(nft1_id), new BN(chain1_id))
      .accounts({
        authStatusAccount: auth_account2,
        owner: program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      // .signers([program.provider.wallet.payer])
      .rpc();
    console.log(
      `unregister Use 'solana confirm -v ${unregisterHash}' to see the logs`
    );

    const postBalance = await program.provider.connection.getBalance(
      program.provider.publicKey
    );
    console.log("post balance is :", postBalance);
    assert(postBalance > preBalance);
    // const data1 = await program.account.authStatusAccount.fetch(
    //     auth_account2
    // );
    // console.log("after delete data is:", data1);
    // console.log("after delete nft_address is: ", data1.srcNft);
  });
});
