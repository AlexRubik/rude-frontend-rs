import React, { useMemo, useCallback } from 'react';
import ReactDOM from 'react-dom/client';
import { ConnectionProvider, WalletProvider, useWallet } from '@solana/wallet-adapter-react';
import {
  BaseWalletMultiButton,
  WalletModalProvider,
} from '@solana/wallet-adapter-react-ui';
import { VersionedTransaction } from '@solana/web3.js';
import * as buffer from "buffer";
window.Buffer = buffer.Buffer;

// Default styles that can be overridden by your app
require('./styles.css');

const LABELS = {
  'change-wallet': 'Change wallet',
  connecting: 'Connecting ...',
  'copy-address': 'Copy address',
  copied: 'Copied',
  disconnect: 'Disconnect',
  'has-wallet': 'Connect Wallet',
  'no-wallet': 'Connect Wallet',
};

export const Wallet = () => {
  // Use Solana mainnet by default, can be changed to devnet or testnet
  const endpoint = "https://api.mainnet-beta.solana.com";
  
  const wallets = useMemo(
    () => [
      // Wallets will auto-detect from browser extensions
    ],
    []
  );
  
  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect={true}>
        <WalletModalProvider>
          <BaseWalletMultiButton labels={LABELS} />
          <Dispatcher />
          <Disconnect />
          <SignTransaction />
          <SignMessage />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};

function MountWalletAdapter() {
  const container = document.getElementById("ore-wallet-adapter");
  if (container) {
    const root = ReactDOM.createRoot(container);
    root.render(<Wallet />);
  }
}
window.MountWalletAdapter = MountWalletAdapter;

// Dispatch wallet public key changes to Rust
function Dispatcher() {
  const { publicKey } = useWallet();
  
  useMemo(() => {
    let msg;
    if (publicKey) {
      msg = publicKey.toBuffer().toJSON().data;
    } else {
      msg = null;
    }
    
    try {
      const event = new CustomEvent(
        "ore-pubkey",
        {
          detail: {
            pubkey: msg
          }
        }
      );
      window.dispatchEvent(event);
    } catch (err) {
      console.error('Error dispatching pubkey:', err);
    }
    
    return;
  }, [publicKey]);
  
  return null;
}

// Disconnect wallet function
function Disconnect() {
  const { publicKey, disconnect } = useWallet();
  
  const callback = useCallback(async (_) => {
    try {
      await disconnect();
    } catch (err) {
      console.error('Error disconnecting wallet:', err);
    }
  }, [publicKey, disconnect]);
  
  window.OreWalletDisconnecter = callback;
  
  return null;
}

// Sign transaction function
function SignTransaction() {
  const { publicKey, signTransaction } = useWallet();
  
  const callback = useCallback(async (msg) => {
    try {
      if (!signTransaction) {
        throw new Error('Wallet does not support transaction signing');
      }
      
      const tx = VersionedTransaction.deserialize(
        Buffer.from(msg.b64, "base64")
      );
      
      const signed = await signTransaction(tx);
      const b64 = Buffer.from(signed.serialize()).toString("base64");
      
      return b64;
    } catch (err) {
      console.error('Error signing transaction:', err);
      throw err;
    }
  }, [publicKey, signTransaction]);
  
  window.OreTxSigner = callback;
  
  return null;
}

// Sign message function
function SignMessage() {
  const { publicKey, signMessage } = useWallet();
  
  const callback = useCallback(async (msg) => {
    try {
      if (!signMessage) {
        throw new Error('Wallet does not support message signing');
      }
      
      const sig = await signMessage(
        Buffer.from(msg.b64, "base64")
      );
      
      const b64 = Buffer.from(sig).toString("base64");
      
      return b64;
    } catch (err) {
      console.error('Error signing message:', err);
      throw err;
    }
  }, [publicKey, signMessage]);
  
  window.OreMsgSigner = callback;
  
  return null;
}
