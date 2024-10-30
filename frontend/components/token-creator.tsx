'use client';

import React from 'react';
import { useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";

type TokenStandard = "original" | "token2022";
type NFTStandard = "tokenMetadata" | "core" | "bubblegum";
type TokenType = "fungible" | "nft";

export function TokenCreator() {
  const { connected } = useWallet();
  const [tokenType, setTokenType] = useState<TokenType>("fungible");
  const [tokenStandard, setTokenStandard] = useState<TokenStandard>("original");
  const [nftStandard, setNftStandard] = useState<NFTStandard>("tokenMetadata");

  if (!connected) {
    return (
      <div className="text-center text-gray-400">
        Please connect your wallet to create tokens
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto space-y-6 bg-gray-800 p-6 rounded-lg">
      <h2 className="text-2xl font-bold">Create New Token</h2>
      
      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-2">Token Type</label>
          <select
            className="w-full bg-gray-700 rounded p-2"
            value={tokenType}
            onChange={(e) => setTokenType(e.target.value as TokenType)}
          >
            <option value="fungible">Fungible Token</option>
            <option value="nft">NFT</option>
          </select>
        </div>

        {tokenType === "fungible" ? (
          <div>
            <label className="block text-sm font-medium mb-2">Token Standard</label>
            <select
              className="w-full bg-gray-700 rounded p-2"
              value={tokenStandard}
              onChange={(e) => setTokenStandard(e.target.value as TokenStandard)}
            >
              <option value="original">Original</option>
              <option value="token2022">Token-2022</option>
            </select>
          </div>
        ) : (
          <div>
            <label className="block text-sm font-medium mb-2">NFT Standard</label>
            <select
              className="w-full bg-gray-700 rounded p-2"
              value={nftStandard}
              onChange={(e) => setNftStandard(e.target.value as NFTStandard)}
            >
              <option value="tokenMetadata">Token Metadata</option>
              <option value="core">Core</option>
              <option value="bubblegum">Bubblegum</option>
            </select>
          </div>
        )}

        <button className="w-full bg-purple-600 hover:bg-purple-700 py-2 rounded">
          Create Token
        </button>
      </div>
    </div>
  );
} 