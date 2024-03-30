import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solot } from "../target/types/solot";
import * as spl from "@solana/spl-token"
import { assert } from "chai"
import { Metaplex } from "@metaplex-foundation/js"
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata"

describe("solot", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solot as Program<Solot>;
  const wallet = anchor.workspace.Solot.provider.wallet
  const user = new anchor.web3.Keypair
  const connection = program.provider.connection
  const metaplex = Metaplex.make(connection)
  const metadataProgram = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")

  const [tokenMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("reward")],
    program.programId
  )
  console.log(`Token mint pda ${tokenMintPDA.toString()}`)
  const solotTokenPoolAccount = spl.getAssociatedTokenAddressSync(
    tokenMintPDA,
    wallet.publicKey
  )
  const metadata = {
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
    name: "Solana Lottery",
    symbol: "SOLOT",
  }

  it("create mint",async()=>{
    const tokenMintMetadataPDA = await metaplex
    .nfts()
    .pdas()
    .metadata({ mint: tokenMintPDA })

    const tx = await program.methods
      .createMint(metadata.uri, metadata.name, metadata.symbol)
      .accounts({
        tokenMint: tokenMintPDA,
        metadataAccount: tokenMintMetadataPDA,
        tokenMetadataProgram: metadataProgram,
      })
      .rpc();
      console.log("Your transaction signature", tx);

  })
  it("initialize lottery", async () => {
    // Add your test here.
    const tx = await program.methods
      .initializeLottery().
      accounts({
        solotData: user.publicKey,
        tokenMint:tokenMintPDA,
        solotTokenPoolAccount:solotTokenPoolAccount,
    })
    .signers([user])
    .rpc();
    console.log("Your transaction signature", tx);

    const solotData = await program.account.solotData.fetch(user.publicKey) 

  });
});
