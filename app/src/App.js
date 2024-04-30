import './App.css';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import { useState } from 'react';
import * as anchor from '@project-serum/anchor';
import { Connection, PublicKey } from '@solana/web3.js';
import { useAnchorWallet, WalletProvider, ConnectionProvider } from '@solana/wallet-adapter-react';
import { Metaplex } from "@metaplex-foundation/js"
import {PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata"
import * as spl from "@solana/spl-token"
import idl from './idl.json';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
require('@solana/wallet-adapter-react-ui/styles.css');



const wallets = [new PhantomWalletAdapter()]
const programID = new PublicKey(idl.metadata.address);

function App() {
  // const [value, setValue] = useState(null);
  const wallet = useAnchorWallet();

  async function getProvider() {
    /* create the provider and return it to the caller */
    /* network set to local network for now */
    const network = "http://127.0.0.1:8899";
    const connection = new Connection(network, []);

    const metaplex = Metaplex.make(connection)
    const provider = new anchor.AnchorProvider(
      connection, wallet, [],
    );
    return provider, metaplex;
  }
  async function createMint() {
    const {provider, metaplex} = await getProvider()
    /* create the program interface combining the idl, program ID, and provider */
    const program = new anchor.Program(idl, programID, provider);
    const [tokenMintPDA] = await PublicKey.findProgramAddressSync([Buffer.from("reward")], program.programId)
    console.log("Token Mint: ", tokenMintPDA);
    const solotTokenPoolAccount = spl.getAssociatedTokenAddressSync(
      tokenMintPDA,
      wallet.publicKey,
    )
    const tokenMintMetadataPDA = await metaplex
    .nfts()
    .pdas()
    .metadata({ mint: tokenMintPDA })

    const metadata = {
      uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",  // 需要替换
      name: "Solana Lottery",
      symbol: "SOLOT",
    }
    try {
      /* interact with the program via rpc */
      const sig = await program.methods
      .createMint(metadata.uri, metadata.name, metadata.symbol)
      .accounts({
        tokenMint: tokenMintPDA,
        metadataAccount: tokenMintMetadataPDA,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .rpc();
      console.log("Your transaction signature", sig);
    } catch (err) {
      console.log("Transaction error: ", err);
    }
  }

  if (!wallet.connected) {
    /* If the user's wallet is not connected, display connect wallet button. */
    return (
      <div style={{ display: 'flex', justifyContent: 'center', marginTop:'100px' }}>
        <WalletMultiButton />
      </div>
    )
  } else {
    return (
      <div className="App">
        <div>
          {
            <button onClick={createMint}>Create Mint</button>
          }
        </div>
      </div>
    );
  }
}

  // return (
  //   <div className="App">
  //     <header className="App-header">
  //       <img src={logo} className="App-logo" alt="logo" />
  //       <p>
  //         Edit <code>src/App.js</code> and save to reload.
  //       </p>
  //       <a
  //         className="App-link"
  //         href="https://reactjs.org"
  //         target="_blank"
  //         rel="noopener noreferrer"
  //       >
  //         Learn React
  //       </a>
  //     </header>
  //   </div>
  // );
// }
const AppWithProvider = () => (
  <ConnectionProvider endpoint="http://127.0.0.1:8899">
    <WalletProvider wallets={wallets} autoConnect>
      <WalletModalProvider>
        <App />
      </WalletModalProvider>
    </WalletProvider>
  </ConnectionProvider>
)

export default AppWithProvider;

