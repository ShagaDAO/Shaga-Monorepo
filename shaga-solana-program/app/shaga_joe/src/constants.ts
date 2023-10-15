import { PublicKey } from "@solana/web3.js";

export const SEED_AFFAIR_LIST = "affair_list";
export const SEED_ESCROW = "escrow";
export const SEED_LENDER = "lender";
export const SEED_AFFAIR = "affair";
export const SEED_RENTAL = "rental";
export const SEED_THREAD = "thread";
export const SEED_AUTHORITY_THREAD = "authority_thread";

export const SOLANA_NATIVE_MINT = new PublicKey(
  "So11111111111111111111111111111111111111112",
);
export const SPL_TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
);

export const CLOCKWORK_PROGRAM_ID = new PublicKey("CLoCKyJ6DXBJqqu2VWx9RLbgnwwR6BMHHuyasVmfMzBh")