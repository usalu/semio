/** 📮️ Bounded, in-order action replies owned by one displayed browser-actor lifetime. */
import type { ActionInvocation, CommandInvocation, UiIntent, PluginViewState } from "@semio-tech/framework";
import { browserActorActionOwnerMatchesV1, parseBrowserActorActionResultV1, type BrowserActorActionOwnerV1, type BrowserActorActionRequestV1, type BrowserActorActionResultV1 } from "../🟦️.ts";
import { createBrowserActorAppCommandRequestV1 } from "../🎛️command/🟦️.ts";
import { createBrowserActorUiIntentRequestV1 } from "../🧭️intent/🟦️.ts";

/** 🎟️ One issued action: its bytes, its waiter, and a completion timer that starts only when it is sent. */
type QueuedActionV1 = {
  readonly request: BrowserActorActionRequestV1;
  readonly resolve: (result: BrowserActorActionResultV1) => void;
  readonly reject: (error: Error) => void;
  timer: ReturnType<typeof setTimeout> | null;
};

/** 🚦️ Why the mailbox path refused an input, in the shell's input-ledger refusal vocabulary (L1/L5). */
export type BrowserActorActionRefusalReasonV1 = "queue-full" | "owner-mismatch" | "dispatch-failed";

/** 🧭️ Maps a mailbox or worker rejection onto the refusal vocabulary; `null` for errors this transport did not author. */
export function browserActorActionRefusalReasonV1(error: unknown): BrowserActorActionRefusalReasonV1 | null {
  const message = error instanceof Error ? error.message : typeof error === "string" ? error : null;
  if (message === null) return null;
  if (message.includes("queue full")) return "queue-full";
  if (/action-owner-mismatch|action-busy|owner retired/u.test(message)) return "owner-mismatch";
  if (/mailbox closed|completion unconfirmed|action-state-unconfirmed|action-refused|action-command-ingress-/u.test(message)) return "dispatch-failed";
  return null;
}

/** 🪪️ A bounded FIFO of intents: one in flight at a time, exact receipt matching, and synchronous retirement admission. */
export class BrowserActorActionMailboxV1 {
  private sequence = 0;
  private inFlight: QueuedActionV1 | null = null;
  private readonly queue: QueuedActionV1[] = [];
  private closed = false;

  /** 🧮️ `capacity` bounds the requests WAITING behind the in-flight one; `timeoutMs` bounds each request from its send, not its issue. */
  constructor(private readonly send: (request: BrowserActorActionRequestV1) => void, private readonly timeoutMs = 30_000, private readonly capacity = 32) {
    if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1 || timeoutMs > 300_000) throw new Error("browser-actor-action: invalid timeout");
    if (!Number.isSafeInteger(capacity) || capacity < 1 || capacity > 1_024) throw new Error("browser-actor-action: invalid capacity");
  }

  /** 🎯️ Captures an intent before handing its immutable bytes to the worker transport. */
  async dispatchIntent(owner: Omit<BrowserActorActionOwnerV1, "actionSequence">, windowKindId: string, intent: UiIntent): Promise<BrowserActorActionResultV1> {
    return await this.dispatchRequest(actionSequence => createBrowserActorUiIntentRequestV1({ ...owner, actionSequence }, windowKindId, intent));
  }

  /** 🎛️ Captures a complete catalog invocation and view state on the same in-order owner. */
  async dispatchCommand(owner: Omit<BrowserActorActionOwnerV1, "actionSequence">, invocation: ActionInvocation | CommandInvocation, viewState: PluginViewState): Promise<BrowserActorActionResultV1> {
    return await this.dispatchRequest(actionSequence => createBrowserActorAppCommandRequestV1({ ...owner, actionSequence }, invocation, viewState));
  }

  /** 📊️ Requests not yet settled: the waiting queue plus the one in flight. */
  pending(): number {
    return this.queue.length + (this.inFlight === null ? 0 : 1);
  }

  /** 🎫️ Never throws: `closed` and `queue full` are typed rejections; the sequence is assigned at issue and the send happens in FIFO order. */
  private dispatchRequest(create: (actionSequence: number) => BrowserActorActionRequestV1): Promise<BrowserActorActionResultV1> {
    if (this.closed) return Promise.reject(new Error("browser-actor-action: mailbox closed"));
    if (this.queue.length >= this.capacity) return Promise.reject(new Error("browser-actor-action: queue full"));
    if (this.sequence === Number.MAX_SAFE_INTEGER) return Promise.reject(new Error("browser-actor-action: sequence exhausted"));
    let request: BrowserActorActionRequestV1;
    try {
      request = create(++this.sequence);
    } catch (error) {
      return Promise.reject(error instanceof Error ? error : new Error(String(error)));
    }
    return new Promise<BrowserActorActionResultV1>((resolve, reject) => {
      this.queue.push({ request, resolve, reject, timer: null });
      this.pump();
    });
  }

  /** 🚚️ Sends the head of the queue when nothing is in flight; a synchronous transport failure settles that request and moves on. */
  private pump(): void {
    while (!this.closed && this.inFlight === null) {
      const next = this.queue.shift();
      if (next === undefined) return;
      this.inFlight = next;
      next.timer = setTimeout(() => this.expire(next), this.timeoutMs);
      try {
        this.send(next.request);
        return;
      } catch (error) {
        if (this.inFlight !== next) return;
        this.retire(next);
        next.reject(error instanceof Error ? error : new Error(String(error)));
      }
    }
  }

  /** ⏱️ A send whose receipt never arrived settles alone; the queue behind it is not punished. */
  private expire(entry: QueuedActionV1): void {
    if (this.closed || this.inFlight !== entry) return;
    this.retire(entry);
    entry.reject(new Error("browser-actor-action: completion unconfirmed"));
    this.pump();
  }

  /** 🧹️ Clears the in-flight slot and its timer without settling the waiter. */
  private retire(entry: QueuedActionV1): void {
    if (this.inFlight === entry) this.inFlight = null;
    if (entry.timer !== null) clearTimeout(entry.timer);
    entry.timer = null;
  }

  /** 📬️ Foreign, stale, replayed, malformed, and queued-but-unsent dispositions cannot settle the in-flight request. */
  settle(value: unknown): boolean {
    if (this.closed || this.inFlight === null) return false;
    let result: BrowserActorActionResultV1;
    try {
      result = parseBrowserActorActionResultV1(value);
    } catch {
      return false;
    }
    const current = this.inFlight;
    if (!browserActorActionOwnerMatchesV1(current.request, result)) return false;
    this.retire(current);
    if (result.outcome === "rejected") current.reject(new Error(result.reason));
    else current.resolve(result);
    this.pump();
    return true;
  }

  /** 🛑️ Refuses new work immediately and rejects the in-flight waiter and every queued one, once, without claiming guest completion. */
  close(reason: string): void {
    if (this.closed) return;
    this.closed = true;
    const waiters = this.inFlight === null ? this.queue.splice(0) : [this.inFlight, ...this.queue.splice(0)];
    this.inFlight = null;
    for (const waiter of waiters) {
      if (waiter.timer !== null) clearTimeout(waiter.timer);
      waiter.timer = null;
      waiter.reject(new Error(reason));
    }
  }
}
