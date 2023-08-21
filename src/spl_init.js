"use strict";
var __awaiter = (this && this.__awaiter) || function(thisArg, _arguments, P, generator) {
  function adopt(value) { return value instanceof P ? value : new P(function(resolve) { resolve(value); }); }
  return new (P || (P = Promise))(function(resolve, reject) {
    function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
    function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
    function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
    step((generator = generator.apply(thisArg, _arguments || [])).next());
  });
};
const { Keypair, Connection } = require("@solana/web3.js");
const { createMint } = require('@solana/spl-token');
const wallet = require("../dev-wallet.json");
function initializeMint() {
  return __awaiter(this, void 0, void 0, function*() {
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
    const owner = Keypair.fromSecretKey(new Uint8Array(wallet));
    const mintAccount = Keypair.generate();
    const mintAuthority = owner;
    const decimals = 6;
    const mint = yield createMint(connection, owner, owner.publicKey, null, decimals);
    console.log(mint);
  });
}
initializeMint();
