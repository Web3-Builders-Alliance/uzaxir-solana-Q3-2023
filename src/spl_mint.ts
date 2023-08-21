const { getOrCreateAssociatedTokenAccount, mintTo } = require("@solana/spl-token");
const { PublicKey, Keypair, Connection } = require("@solana/web3.js");
const wallet = require("../dev-wallet.json");


async function mintTokens() {
  const token_decimals = 1_000_000n;
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");
  const owner = Keypair.fromSecretKey(new Uint8Array(wallet));
  const mint = new PublicKey("DqNVUsBsyv881u2gGrXMRvievkd5WtcrpSaFCC7BiHix");

  const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    owner,
    mint,
    owner.publicKey
  );

  console.log(`Associated Token Account Address: ${associatedTokenAccount.address.toBase58()}`);

  const mintTxn = await mintTo(
    connection,
    owner,
    mint,
    associatedTokenAccount.address,
    owner,
    token_decimals
  );

  console.log(`Mint Txn ID: ${mintTxn}`);


}

mintTokens()
