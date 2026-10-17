/** 📮️ App channel reply ledger: the request sequence, the queue of commands waiting for their reply and the handle `enqueue` that sends them. */

import type { PluginDispatchHintV1 } from "@semio-tech/framework";
import type { LocalInteractionQueryCommand } from "@semio-tech/framework-replication";
import type { AppCommandValue, AppFrameValue } from "../../🪢️wire-contracts/🟦️.ts";
import { encodeAppCommand } from "../🎛️command-codec/🟦️.ts";
import type { ChannelLifecycle } from "../🔌️channel-lifecycle/🟦️.ts";
import { appChannelFrameBelongsTo, type AppChannelHandle, appChannelReplySequence, type AppChannelRequestSequence, appChannelTransactionReply, type AppChannelTransactionReply } from "../🔢️request-sequence/🟦️.ts";

//#region 🔖️AppChannelClient
/** ⏳️ One command (or one local interaction query receipt) that still waits for its reply frames. */
type PendingReply = { readonly seq: number; readonly queryReceipt: boolean; readonly transaction: AppChannelTransactionReply | null; readonly resolve: (frames: AppFrameValue[]) => void; readonly reject: (error: unknown) => void };

/** 🧾️ Owns the request sequence and the FIFO of pending replies of one app instance: every command is numbered here, queued on the
 * handle and parked until the pump hands back the frames that answer its `seq`. */
export class ReplyLedger {
  private readonly handle: AppChannelHandle;
  private readonly sequence: AppChannelRequestSequence;
  private readonly instanceId: number;
  private readonly lifecycle: ChannelLifecycle;
  private readonly pending: PendingReply[] = [];

  constructor(handle: AppChannelHandle, sequence: AppChannelRequestSequence, instanceId: number, lifecycle: ChannelLifecycle) {
    this.handle = handle;
    this.sequence = sequence;
    this.instanceId = instanceId;
    this.lifecycle = lifecycle;
  }

  /** 🔢️ How many replies are still awaited. */
  get size(): number {
    return this.pending.length;
  }

  /** 🎟️ The next command sequence of the handle-lifetime owner. */
  nextSeq(): number {
    return this.sequence.nextSequence();
  }

  /** 🔎️ Admits one local interaction query: its request id and the two receipt sequences it reserves. */
  nextQuery(): ReturnType<AppChannelRequestSequence["nextQuery"]> {
    return this.sequence.nextQuery();
  }

  /** 🔀️ Queues one encoded command and resolves with every frame its matching {@link TurnOutcome}
   * carries — see {@link AppChannelClient}'s own header doc for how the reply gets correlated back to this call.
   * `dispatch` ({@link PluginDispatchHintV1}) is forwarded verbatim to {@link PluginWasmHandle.enqueue};
   * only {@link InteractionCommands.command} ever passes one — `configure`/`readDocument`/… are maintenance and stay
   * unordered (arrival order). */
  send(command: AppCommandValue, dispatch?: PluginDispatchHintV1): Promise<AppFrameValue[]> {
    if (this.lifecycle.closed) return Promise.reject(new Error("app-channel.disposed"));
    return new Promise<AppFrameValue[]>((resolve, reject) => {
      const wireSequence = Object.values(command)[0]!.seq;
      const seq = Number(wireSequence);
      if (!Number.isSafeInteger(seq) || seq < 0) { reject(new Error("app-channel.invalid-sequence-owner")); return; }
      const waiter = { seq, queryReceipt: false, transaction: appChannelTransactionReply(command), resolve, reject };
      this.pending.push(waiter);
      try { this.handle.enqueue(this.instanceId, [encodeAppCommand(command)], dispatch); }
      catch (error) {
        const index = this.pending.indexOf(waiter);
        if (index !== -1) this.pending.splice(index, 1);
        reject(error);
      }
    });
  }

  /** 📃️ Queues one local interaction query command; `failed` hears a refused enqueue and a rejected receipt. */
  sendQuery(seq: number, command: LocalInteractionQueryCommand, failed: (error: unknown) => void): void {
    this.pending.push({ seq, queryReceipt: true, transaction: null, resolve: () => {}, reject: failed });
    try { this.handle.enqueue(this.instanceId, [encodeAppCommand({ LocalInteractionQuery: { seq, command } })]); }
    catch (error) {
      const index = this.pending.findIndex((waiter) => waiter.seq === seq);
      if (index !== -1) this.pending.splice(index, 1);
      failed(error);
    }
  }

  /** 🚨️ Rejects the oldest waiter with the error its outcome carried. */
  rejectOldest(error: unknown): void {
    this.pending.shift()?.reject(error);
  }

  /** 🧨️ Rejects every command waiter (query receipts keep their own lifecycle) with one shared error. */
  rejectCommands(error: unknown): void {
    this.removeCommands(() => error);
  }

  /** 🪦️ Rejects every command waiter because the queue was retired. */
  retireCommands(): void {
    this.removeCommands(() => new Error("app-channel.disposed"));
  }

  /** 🧹️ Forgets the query receipts of a finished local interaction query. */
  dropQueryReceipts(): void {
    for (let index = this.pending.length - 1; index >= 0; index -= 1) {
      if (this.pending[index]!.queryReceipt) this.pending.splice(index, 1);
    }
  }

  /** 🔒️ Rejects every waiter because the outcome stream ended. */
  closeAll(): void {
    for (const waiter of this.pending.splice(0)) waiter.reject(new Error("app-channel.closed"));
  }

  /** 🤝️ Hands the ordinary reply frames of one outcome to the waiters they answer, oldest first: `capture` sees each waiter's reply before it resolves. */
  settle(ordinary: readonly AppFrameValue[], capture: (reply: readonly AppFrameValue[]) => void): void {
    const correlated = new Set(ordinary.flatMap((frame) => { const sequence = appChannelReplySequence(frame); return sequence === null ? [] : [sequence]; }));
    for (let index = 0; index < this.pending.length;) {
      const waiter = this.pending[index]!;
      if (!correlated.has(waiter.seq)) { index += 1; continue; }
      this.pending.splice(index, 1);
      const reply = ordinary.filter((frame) => appChannelFrameBelongsTo(frame, waiter.seq, waiter.transaction));
      capture(reply);
      waiter.resolve(reply);
    }
  }

  private removeCommands(failure: () => unknown): void {
    for (let index = this.pending.length - 1; index >= 0; index -= 1) {
      if (!this.pending[index]!.queryReceipt) this.pending.splice(index, 1)[0]!.reject(failure());
    }
  }
}
//#endregion 🔖️AppChannelClient
