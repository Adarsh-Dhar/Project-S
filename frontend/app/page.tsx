'use client';

import React from 'react';
import { TokenCreator } from "@/components/token-creator";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

export default function Home() {
  return (
    <div className="space-y-8">
      <section className="text-center space-y-4">
        <h1 className="text-4xl font-bold">Solana Token Manager</h1>
        <p className="text-xl text-gray-400">
          Create and manage your Solana tokens and NFTs
        </p>
        <div className="flex justify-center">
          <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700" />
        </div>
      </section>
      <TokenCreator />
    </div>
  );
}
