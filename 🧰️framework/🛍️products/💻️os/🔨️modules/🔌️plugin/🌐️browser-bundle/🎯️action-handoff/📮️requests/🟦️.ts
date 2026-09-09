/** 📮️ Bounded action replies owned by one displayed browser-actor lifetime. */
import type { ActionInvocation, CommandInvocation, UiIntent, ViewModel } from "@semio-tech/framework";
import { browserActorActionOwnerMatchesV1, parseBrowserActorActionResultV1, type BrowserActorActionOwnerV1, type BrowserActorActionRequestV1, type BrowserActorActionResultV1 } from "../🟦️.ts";
import { createBrowserActorAppCommandRequestV1 } from "../🎛️command/🟦️.ts";
import { createBrowserActorUiIntentRequestV1 } from "../🧭️intent/🟦️.ts";

type PendingActionV1 = {
  readonly request: BrowserActorActionRequestV1;
  readonly timer: ReturnType<typeof setTimeout>;
  readonly resolve: (result: BrowserActorActionResultV1) => void;
  readonly reject: (error: Error) => void;
};

/** 🪪️ One in-flight intent, exact receipt matching, and synchronous retirement admission. */
export class BrowserActorActionMailboxV1 {
  private sequence = 0;
  private pending: PendingActionV1 | null = null;
  private closed = false;

  constructor(private readonly send: (request: BrowserActorActionRequestV1) => void, private readonly timeoutMs = 30_000) {
    if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1 || timeoutMs > 300_000) throw new Error("browser-actor-action: invalid timeout");
  }

  /** 🎯️ Captures an intent before handing its immutable bytes to the worker transport. */
  async dispatchIntent(owner: Omit<BrowserActorActionOwnerV1, "actionSequence">, windowKindId: string, intent: UiIntent): Promise<BrowserActorActionResultV1> {
    return await this.dispatchRequest(actionSequence => createBrowserActorUiIntentRequestV1({ ...owner, actionSequence }, windowKindId, intent));
  }

  /** 🎛️ Captures a complete catalog invocation and view state on the same single-flight owner. */
  async dispatchCommand(owner: Omit<BrowserActorActionOwnerV1, "actionSequence">, invocation: ActionInvocation | CommandInvocation, viewState: ViewModel): Promise<BrowserActorActionResultV1> {
    return await this.dispatchRequest(actionSequence => createBrowserActorAppCommandRequestV1({ ...owner, actionSequence }, invocation, viewState));
  }

  private dispatchRequest(create: (actionSequence: number) => BrowserActorActionRequestV1): Promise<BrowserActorActionResultV1> {
    if (this.closed) throw new Error("browser-actor-action: mailbox closed");
    if (this.pending !== null) throw new Error("browser-actor-action: another action pending");
    if (this.sequence === Number.MAX_SAFE_INTEGER) throw new Error("browser-actor-action: sequence exhausted");
    const request = create(++this.sequence);
    return new Promise<BrowserActorActionResultV1>((resolve, reject) => {
      const timer = setTimeout(() => this.close("browser-actor-action: completion unconfirmed"), this.timeoutMs);
      const pending = { request, timer, resolve, reject };
      this.pending = pending;
      try {
        this.send(request);
      } catch (error) {
        if (this.pending !== pending) return;
        this.pending = null;
        clearTimeout(timer);
        reject(error instanceof Error ? error : new Error(String(error)));
      }
    });
  }

  /** 📬️ Foreign, stale, replayed, and malformed dispositions cannot settle the current request. */
  settle(value: unknown): boolean {
    if (this.closed || this.pending === null) return false;
    let result: BrowserActorActionResultV1;
    try {
      result = parseBrowserActorActionResultV1(value);
    } catch {
      return false;
    }
    if (!browserActorActionOwnerMatchesV1(this.pending.request, result)) return false;
    const pending = this.pending;
    this.pending = null;
    clearTimeout(pending.timer);
    if (result.outcome === "rejected") pending.reject(new Error(result.reason));
    else pending.resolve(result);
    return true;
  }

  /** 🛑️ Refuses new work immediately and rejects the retained waiter without claiming guest completion. */
  close(reason: string): void {
    if (this.closed) return;
    this.closed = true;
    const pending = this.pending;
    this.pending = null;
    if (pending === null) return;
    clearTimeout(pending.timer);
    pending.reject(new Error(reason));
  }
}
