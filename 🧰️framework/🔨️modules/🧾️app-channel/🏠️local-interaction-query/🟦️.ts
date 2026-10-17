/** 🏠️ App channel local interaction query: the single native-owned paged read of the local interaction slice, its cancellation and its ACK-owned page lifecycle. */

import type { LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryCommand, LocalInteractionQueryReply } from "@semio-tech/framework-replication";
import type { ChannelLifecycle } from "../🔌️channel-lifecycle/🟦️.ts";
import type { ReplyLedger } from "../📮️reply-ledger/🟦️.ts";
import { type LocalInteractionClientQuery, sameLocalInteractionQuery } from "../🔢️request-sequence/🟦️.ts";

//#region 🔖️AppChannelClient
//#region 🏠️LocalInteractionQuery
/** 🔍️ Owns the one local interaction query a client may run at a time. Each page remains native-owned until its consumer resolves; completion waits for exact native root retirement. */
export class LocalInteractionQuery {
  private readonly instanceId: number;
  private readonly ledger: ReplyLedger;
  private readonly lifecycle: ChannelLifecycle;
  private readonly settled: () => void;
  private query: LocalInteractionClientQuery | null = null;

  constructor(instanceId: number, ledger: ReplyLedger, lifecycle: ChannelLifecycle, settled: () => void) {
    this.instanceId = instanceId;
    this.ledger = ledger;
    this.lifecycle = lifecycle;
    this.settled = settled;
  }

  /** 🛌️ `true` while no query is running. */
  get idle(): boolean {
    return this.query === null;
  }

  /** 📃️ Each page remains native-owned until its consumer resolves; completion waits for exact native root retirement. */
  read(consume: (page: LocalInteractionPage) => Promise<void>, signal?: AbortSignal): Promise<LocalInteractionIdentity> {
    if (this.lifecycle.closed) return Promise.reject(new Error("app-channel.disposed"));
    if (this.query) return Promise.reject(new Error("local-interaction.busy"));
    if (signal?.aborted) return Promise.reject(new Error("local-interaction.cancelled"));
    let admission: ReturnType<ReplyLedger["nextQuery"]>;
    try { admission = this.ledger.nextQuery(); }
    catch (error) { return Promise.reject(error); }
    const requestId = admission.request;
    return new Promise((resolve, reject) => {
      const abort = () => this.cancel(new Error("local-interaction.cancelled"));
      this.query = { requestId, cancelSequence: admission.cancelSequence, consume, resolve, reject, signal, abort, token: null, nextOrdinal: 0n, consuming: false, cancelled: false, terminalConsumed: false, failure: null };
      signal?.addEventListener("abort", abort, { once: true });
      this.send(admission.sequence, { kind: "read", requestId });
    });
  }

  /** 🪦️ Cancels the running query because its client retired. */
  retire(): void {
    if (this.query) this.cancel(new Error("local-interaction.disposed"));
  }

  /** 🛑️ Marks the running query cancelled and asks native to retire its root when it already has a token. */
  cancel(error: unknown): void {
    const query = this.query;
    if (!query || query.cancelled) return;
    query.cancelled = true;
    query.failure = error;
    if (query.token) this.send(query.cancelSequence, { kind: "cancel", token: query.token });
  }

  /** 🏁️ Ends the running query: drops its receipts, settles its promise and lets the client finish a pending disposal. */
  finish(error: unknown = null): void {
    const query = this.query;
    if (!query) return;
    this.query = null;
    this.ledger.dropQueryReceipts();
    query.signal?.removeEventListener("abort", query.abort);
    const failure = query.failure ?? error;
    if (failure !== null) query.reject(failure);
    else if (query.token && query.terminalConsumed) query.resolve(query.token.identity);
    else query.reject(new Error("local-interaction.incomplete-close"));
    this.settled();
  }

  /** 📨️ Applies one native reply to the running query: admission, one page at a time in ordinal order, then exact root retirement. */
  receive(reply: LocalInteractionQueryReply): void {
    const query = this.query;
    if (!query) return;
    if (reply.kind === "rejected") {
      if (reply.requestId === query.requestId) this.finish(new Error(`local-interaction.${reply.code}`));
      return;
    }
    if (reply.kind === "started") {
      if (reply.token.requestId !== query.requestId || reply.token.identity.appInstanceId !== this.instanceId || query.token !== null) return;
      query.token = reply.token;
      if (query.cancelled) this.send(query.cancelSequence, { kind: "cancel", token: query.token });
      return;
    }
    const token = reply.kind === "page" ? reply.page : reply.token;
    if (!query.token || !sameLocalInteractionQuery(query.token, token)) return;
    if (reply.kind === "closed") {
      if (!query.cancelled && (!query.terminalConsumed || token.ordinal !== query.token.ordinal)) return;
      this.finish(reply.cancelled ? new Error("local-interaction.cancelled") : null);
      return;
    }
    if (query.cancelled || query.consuming || token.ordinal !== query.nextOrdinal.toString()) return;
    query.token = { requestId: token.requestId, queryGeneration: token.queryGeneration, identity: token.identity, ordinal: token.ordinal };
    query.consuming = true;
    void Promise.resolve().then(() => query.consume(reply.page)).then(() => {
      if (this.query !== query) return;
      query.consuming = false;
      if (query.cancelled) return;
      query.terminalConsumed = reply.page.terminal;
      query.nextOrdinal += 1n;
      let seq: number;
      try { seq = this.ledger.nextSeq(); }
      catch (error) { this.cancel(error); return; }
      this.send(seq, { kind: "acknowledge", token: query.token! });
    }, (error: unknown) => {
      if (this.query !== query) return;
      query.consuming = false;
      this.cancel(error);
    });
  }

  private send(seq: number, command: LocalInteractionQueryCommand): void {
    this.ledger.sendQuery(seq, command, (error: unknown) => this.finish(error));
  }
}
//#endregion 🏠️LocalInteractionQuery
//#endregion 🔖️AppChannelClient
