import { Connection, PublicKey } from "@solana/web3.js";
import {
  SEED_AFFAIR_LIST,
  SEED_ESCROW,
  SEED_LENDER,
  SEED_AFFAIR,
  SEED_RENTAL,
  SEED_THREAD,
  SEED_AUTHORITY_THREAD,
  CLOCKWORK_PROGRAM_ID,
} from "./constants";
import { PROGRAM_ID } from "./generated";

export function findAffairList() {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_AFFAIR_LIST)],
    PROGRAM_ID,
  );
}

export function findVault() {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_ESCROW)],
    PROGRAM_ID,
  );
}

export function findThreadAuthority() {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_AUTHORITY_THREAD)],
    PROGRAM_ID,
  );
}

export function findAffair(authority: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_AFFAIR), authority.toBuffer()],
    PROGRAM_ID,
  );
}

export function findLender(authority: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_LENDER), authority.toBuffer()],
    PROGRAM_ID,
  );
}

export function findRentEscrow(lenderAccount: PublicKey, clientAccount: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_ESCROW), lenderAccount.toBuffer(), clientAccount.toBuffer()],
    PROGRAM_ID,
  );
}

export function findRentAccount(lenderAccount: PublicKey, clientAccount: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_RENTAL), lenderAccount.toBuffer(), clientAccount.toBuffer()],
    PROGRAM_ID,
  );
}

export function findRentalThreadId(clientAccount: PublicKey, affairAccount: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_THREAD), clientAccount.toBuffer(), affairAccount.toBuffer()],
    PROGRAM_ID,
  );
}

export function findAffairThreadId(threadAuthority: PublicKey, affairAccount: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_THREAD), threadAuthority.toBuffer(), affairAccount.toBuffer()],
    PROGRAM_ID,
  );
}
export function findClockworkThreadAccount(threadAuthority: PublicKey, threadId: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_THREAD), threadAuthority.toBuffer(), threadId.toBuffer()],
    CLOCKWORK_PROGRAM_ID,
  );
}
