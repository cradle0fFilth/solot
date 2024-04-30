import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solot } from "../target/types/solot";
import * as spl from "@solana/spl-token"
import { assert } from "chai"
import { Metaplex } from "@metaplex-foundation/js"
import {PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata"

describe("solot", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solot as Program<Solot>
  // const owner = anchor.workspace.Solot.provider.wallet
  const owner = anchor.Wallet.local().payer
  const connection = program.provider.connection
  const metaplex = Metaplex.make(connection)
  const solotData = new anchor.web3.Keypair
  const lossLotteryTickets = new anchor.web3.Keypair
  const winLotteryTickets = new anchor.web3.Keypair
  // const metadataProgram = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
  const [tokenMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("reward")],
    program.programId
  )
  console.log(`Token mint pda ${tokenMintPDA.toString()}`)
  const solotTokenPoolAccount = spl.getAssociatedTokenAddressSync(
    tokenMintPDA,
    owner.publicKey
  )
  const metadata = {
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",  // 需要替换
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
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .rpc();
      console.log("Your transaction signature", tx);
      const mintInfo = await spl.getMint(connection, tokenMintPDA)
      assert.equal(mintInfo.supply, BigInt(0))
  })
  it("initialize lottery", async () => {
    // Add your test here.
    const tx = await program.methods
      .initializeLottery().
      accounts({
        solotData: owner.publicKey,
        tokenMint:tokenMintPDA,
        solotTokenPoolAccount:solotTokenPoolAccount,
    })
    .signers([owner])
    .rpc();
    console.log("Your transaction signature", tx);

    const solotData = await program.account.solotData.fetch(owner.publicKey)

  });
});
