const { Keypair, Connection } = require("@solana/web3.js");
const { createMint } = require('@solana/spl-token');
const wallet = require("../dev-wallet.json");
async function initializeMint() {
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const owner = Keypair.fromSecretKey(new Uint8Array(wallet));
  const mintAccount = Keypair.generate();

  const mintAuthority = owner;
  const decimals = 6;

  const mint = await createMint(
    connection,
    owner,
    owner.publicKey,
    null,
    decimals
  );

  console.log(mint);
}

initializeMint()

// Mint on Devnet: https://explorer.solana.com/address/DqNVUsBsyv881u2gGrXMRvievkd5WtcrpSaFCC7BiHix?cluster=devnet
