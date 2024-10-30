'use client';

import React from 'react';
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import Link from "next/link";

export function Navbar() {
  return (
    <nav className="border-b border-gray-800">
      <div className="container mx-auto px-4 py-4 flex justify-between items-center">
        <div className="flex items-center space-x-8">
          <Link href="/" className="text-xl font-bold">
            Token Manager
          </Link>
          <div className="hidden md:flex space-x-6">
            <Link href="/tokens" className="hover:text-purple-400">
              Tokens
            </Link>
            <Link href="/nfts" className="hover:text-purple-400">
              NFTs
            </Link>
            <Link href="/collections" className="hover:text-purple-400">
              Collections
            </Link>
          </div>
        </div>
        <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700" />
      </div>
    </nav>
  );
} 