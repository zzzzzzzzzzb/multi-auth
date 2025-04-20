import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {SolanaNft} from "../target/types/solana_nft";
import {expect} from "chai";
import {
    findMetadataPda,
    findMasterEditionV2Pda,
} from "@metaplex-foundation/js";
import {MPL_TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata";

describe("solana-nft", () => {
    // 配置 Anchor 提供者
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.SolanaNft as Program<SolanaNft>;
    const payer = anchor.web3.Keypair.generate();

    // 元数据
    const nftName = "测试 NFT";
    const nftSymbol = "TNFT";
    const nftUri = "https://example.com/metadata.json";

    it("Mint NFT", async () => {
        // 创建铸币账户
        const mintKeypair = anchor.web3.Keypair.generate();
        console.log(mintKeypair.publicKey.toBase58())

        const metadataAddress = findMetadataPda(mintKeypair.publicKey);
        const masterEditionAddress = findMasterEditionV2Pda(mintKeypair.publicKey);
        const associatedTokenAddress = anchor.utils.token.associatedAddress({
            mint: mintKeypair.publicKey,
            owner: payer.publicKey,
        });

        // 调用 mint_nft 指令
        const tx = await program.methods
            .mintNft(nftName, nftSymbol, nftUri)
            .accounts({
                payer: payer.publicKey,
                metadataAccount: metadataAddress,
                editionAccount: masterEditionAddress,
                mintAccount: mintKeypair.publicKey,
                associatedTokenAccount: associatedTokenAddress,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                associatedTokenProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([payer, mintKeypair])
            .rpc();

        console.log("交易签名:", tx);

        // 验证 NFT 已被铸造
        const tokenAccount = await provider.connection.getTokenAccountBalance(
            associatedTokenAddress
        );
        expect(tokenAccount.value.uiAmount).to.equal(1);
    });
});
