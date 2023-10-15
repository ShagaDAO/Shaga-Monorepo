import { Connection } from '@solana/web3.js';
import { PublicKey } from '@solana/web3.js';
import { Affair, AffairPayload, RentalTerminationAuthority, createCreateAffairInstruction, createEndRentalInstruction, createInitializeInstruction, createInitializeLenderInstruction, createStartRentalInstruction, createTerminateAffairInstruction, createTerminateVacantAffairInstruction } from '../generated';
import {
  findAffairList,
  findVault,
  findThreadAuthority,
  findAffairThreadId,
  findRentalThreadId,
  findClockworkThreadAccount,
  findRentAccount,
  findRentEscrow,
  findLender,
  findAffair,
} from '../pda';
import { CLOCKWORK_PROGRAM_ID } from '../constants';

export function initializeShagaAccounts(payer: PublicKey) {

  const [affairsList] = findAffairList();
  const [vault] = findVault();
  const [threadAuthority] = findThreadAuthority();

  const initializeAccountsIx = createInitializeInstruction(
    {
      payer: payer,
      affairsList,
      vault,
      threadAuthority,
    }
  )
  return initializeAccountsIx
}

export function createLender(payer: PublicKey) {

  const [lender] = findLender(payer);

  const createLenderIx = createInitializeLenderInstruction(
    {
      payer: payer,
      lender,
    }
  )
  return createLenderIx
}

export function createAffair(authority: PublicKey, affairPayload: AffairPayload) {
  const [affair] = findAffair(authority);
  console.log('affair', affair.toBase58())
  const [lender] = findLender(authority);
  const [affairsList] = findAffairList();
  const [vault] = findVault();
  const [threadAuthority] = findThreadAuthority();
  const [threadId] = findAffairThreadId(threadAuthority, affair);
  const [affairClockworkThread] = findClockworkThreadAccount(threadAuthority, threadId);

  const createAffairIx = createCreateAffairInstruction(
    {
      authority,
      lender,
      affair,
      affairsList,
      vault,
      threadAuthority,
      affairClockworkThread,
      clockworkProgram: CLOCKWORK_PROGRAM_ID
    },
    {
      payload: affairPayload
    }
  )
  return createAffairIx

}

export async function startRental(connection: Connection, client: PublicKey, affair: PublicKey, rentalTerminationTime: number) {
  const affairData = await Affair.fromAccountAddress(connection, affair);
  const [lender] = findLender(affairData.authority);
  const [affairsList] = findAffairList();
  const [vault] = findVault();
  const [threadAuthority] = findThreadAuthority();
  console.log(threadAuthority.toBase58())
  const [escrow] = findRentEscrow(lender, client);
  const [rental] = findRentAccount(lender, client);
  const [threadId] = findRentalThreadId(threadAuthority, rental);
  const [rentalClockworkThread] = findClockworkThreadAccount(threadAuthority, threadId);

  const startRentalIx = createStartRentalInstruction(
    {
      client,
      lender,
      affair,
      affairsList,
      vault,
      escrow,
      rental,
      threadAuthority,
      rentalClockworkThread,
      clockworkProgram: CLOCKWORK_PROGRAM_ID
    },
    { // time in UTC timestamp since epoch
      rentalTerminationTime
    }
  )
  return startRentalIx
}
export async function endRental(client: PublicKey, affair: PublicKey) {
  const connection = new Connection(process.env.RPC_URL || "");
  const affairData = await Affair.fromAccountAddress(connection, affair);
  const [lender] = findLender(affairData.authority);
  const [affairsList] = findAffairList();
  const [vault] = findVault();
  const [threadAuthority] = findThreadAuthority();
  const [escrow] = findRentEscrow(lender, client);
  const [rental] = findRentAccount(lender, client);
  const [threadId] = findRentalThreadId(threadAuthority, rental);
  const [rentalClockworkThread] = findClockworkThreadAccount(threadAuthority, threadId);

  const endRentalIx = createEndRentalInstruction(
    {
      signer: client,
      client,
      lender,
      affair,
      affairsList,
      vault,
      escrow,
      rental,
      threadAuthority,
      rentalClockworkThread,
      clockworkProgram: CLOCKWORK_PROGRAM_ID
    },
    {
      terminationBy: RentalTerminationAuthority.Client
    }
  )
  return endRentalIx

}

export async function terminateAffair(connection: Connection, authority: PublicKey, affair: PublicKey, vacant?: boolean) {
  let [lender] = findLender(authority);
  const [affairsList] = findAffairList();
  const [vault] = findVault();
  const [threadAuthority] = findThreadAuthority();
  const [threadId] = findAffairThreadId(threadAuthority, affair);
  const [affairClockworkThread] = findClockworkThreadAccount(threadAuthority, threadId);
  if (vacant) {
    const terminateAffairIx = createTerminateVacantAffairInstruction(
      {
        signer: authority,
        authority,
        lender,
        affair,
        affairsList,
        vault,
        threadAuthority,
        affairClockworkThread,
        clockworkProgram: CLOCKWORK_PROGRAM_ID
      }
    )
    return terminateAffairIx
  }

  const affairData = await Affair.fromAccountAddress(connection, affair);
  [lender] = findLender(affairData.authority);
  const [escrow] = findRentEscrow(lender, affairData.client);
  const [rental] = findRentAccount(lender, affairData.client);
  const [threadIdRental] = findRentalThreadId(threadAuthority, rental);
  const [rentalClockworkThread] = findClockworkThreadAccount(threadAuthority, threadIdRental);

  const terminateAffairIx = createTerminateAffairInstruction(
    {
      authority,
      client: affairData.client,
      lender,
      affair,
      affairsList,
      vault,
      escrow,
      rental,
      threadAuthority,
      affairClockworkThread,
      rentalClockworkThread,
      clockworkProgram: CLOCKWORK_PROGRAM_ID
    }
  )
  return terminateAffairIx
}