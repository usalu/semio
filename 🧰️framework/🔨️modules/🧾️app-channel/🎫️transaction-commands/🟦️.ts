/** 🎫️ App channel transaction commands: prepare, commit, roll back and group undo or redo of one member of a cross-artifact transaction. */

import type { AppFrameValue } from "../../🪢️wire-contracts/🟦️.ts";
import type { ReplyLedger } from "../📮️reply-ledger/🟦️.ts";

//#region 🔖️AppChannelClient
/** 🧮️ Sends one member's half of a transaction; the coordinator in `PluginRuntime/🟦️.tsx` drives the fan-out. */
export class TransactionCommands {
  private readonly ledger: ReplyLedger;

  constructor(ledger: ReplyLedger) {
    this.ledger = ledger;
  }

  //#region 🔖️Transaction
  /** 🏷️ `TransactionPrepare`, owner-mutation wire form (contract freeze §2/§5.3): `mutationId` +
   * `payload` set, `preparedOps` empty. Sent when `ArtifactMutationRouter` resolves the mutation to
   * its OWNING plugin. */
  async prepareOwner(txnId: string, mutationId: string, payload: Uint8Array): Promise<AppFrameValue[]> {
    return this.ledger.send({
      transactionPrepare: { seq: this.ledger.nextSeq(), txn_id: txnId, mutation_id: mutationId, payload: Array.from(payload), prepared_ops: [], origin: [], prepared_child_ops: [] },
    });
  }

  /** 🗂️ `TransactionPrepare`, pre-planned wire form: `preparedOps`/`origin` set, `mutationId`
   * empty. Sent to a CONTRIBUTED-mutation target (after the host has already called the contributor's
   * `contributor.artifact-mutation-plan`) or to any member the coordinator is re-batching several
   * already-known ops onto in one call — see `PluginRuntime/🟦️.tsx`'s `TransactionCoordinator`. */
  async preparePlanned(txnId: string, preparedOps: readonly Uint8Array[], origin: Uint8Array): Promise<AppFrameValue[]> {
    return this.ledger.send({
      transactionPrepare: {
        seq: this.ledger.nextSeq(),
        txn_id: txnId,
        mutation_id: "",
        payload: [],
        prepared_ops: preparedOps.map((op) => Array.from(op)),
        origin: Array.from(origin),
        prepared_child_ops: [],
      },
    });
  }

  /** ✅️ Commits the prepared transaction. */
  async commit(txnId: string): Promise<AppFrameValue[]> {
    return this.ledger.send({ transactionCommit: { seq: this.ledger.nextSeq(), txn_id: txnId } });
  }

  /** ↩️ Rolls the prepared transaction back. */
  async rollback(txnId: string): Promise<AppFrameValue[]> {
    return this.ledger.send({ transactionRollback: { seq: this.ledger.nextSeq(), txn_id: txnId } });
  }

  /** 🎁️ Group undo — fans out to every member of `groupId` (contract freeze §5.7); this call is one
   * member's half, the coordinator drives the fan-out. */
  async undo(groupId: string): Promise<AppFrameValue[]> {
    return this.ledger.send({ transactionUndo: { seq: this.ledger.nextSeq(), group_id: groupId } });
  }

  /** 🔁️ Group redo — the mirror of {@link undo}. */
  async redo(groupId: string): Promise<AppFrameValue[]> {
    return this.ledger.send({ transactionRedo: { seq: this.ledger.nextSeq(), group_id: groupId } });
  }
  //#endregion 🔖️Transaction
}
//#endregion 🔖️AppChannelClient
