
import {
  Connection,
  Keypair,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
  Transaction
} from '@solana/web3.js';

export const signAndSendTransactionInstructionsModified = async (
  // sign and send transaction
  connection: Connection,
  signers: Keypair[],
  feePayer: Keypair,
  instructions: TransactionInstruction[],
  confirmIt?: boolean,
): Promise<string> => {
  let latestBlockhash = await connection.getLatestBlockhash();
  const messageV0 = new TransactionMessage({
    payerKey: feePayer.publicKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions,
  }).compileToV0Message([]);
  const tx = new VersionedTransaction(messageV0);
  tx.sign(signers);
  const tx_id = await connection.sendTransaction(tx);
  if (confirmIt) {
    const strategy = {
      signature: tx_id,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      blockhash: latestBlockhash.blockhash,
    };
    await connection.confirmTransaction(strategy, "confirmed");
  }
  return tx_id;
};


export const signAndSendLegacyTransaction = async (
  // sign and send transaction
  connection: Connection,
  signers: Keypair[],
  feePayer: Keypair,
  instructions: TransactionInstruction[],
  confirmIt?: boolean,
): Promise<string> => {
  let latestBlockhash = await connection.getLatestBlockhash();
  const tx = new Transaction({
    feePayer: feePayer.publicKey,
    lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    blockhash: latestBlockhash.blockhash,
  }).add(...instructions);
  const tx_id = await connection.sendTransaction(tx, signers, { skipPreflight: false });
  console.log(tx_id)
  if (confirmIt) {
    const strategy = {
      signature: tx_id,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      blockhash: latestBlockhash.blockhash,
    };
    await connection.confirmTransaction(strategy, "confirmed");
  }
  return tx_id;
};


// Function to convert a string to a Uint8Array
export const stringToUint8Array = (str: string, size: number): number[] => {
  const arr = new Uint8Array(size);
  for (let i = 0; i < str.length && i < size; i++) {
    arr[i] = str.charCodeAt(i);
  }
  return Array.from(arr);
};