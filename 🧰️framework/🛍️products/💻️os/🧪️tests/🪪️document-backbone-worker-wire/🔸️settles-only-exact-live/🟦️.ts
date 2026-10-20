/** 🔸️ The settles only exact live tests of the document backbone worker wire suite. */
import { expect, it, vi } from "vitest";
import { source } from "../../🧮️directory-client-helpers/🟦️.ts";

/** 🔹 Registers the settles only exact live tests of the document backbone worker wire suite. */
export function registerSettlesOnlyExactLiveTests(): void {
  it("settles only the exact live browser intent mailbox and retires pending work", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const { BrowserActorActionMailboxV1, browserActorActionRefusalReasonV1 } = await import("@semio-tech/framework/browser-bundle/action-handoff/requests");
    const { actionSequence: _sequence, ...owner } = fixture.request;
    const intent = { ...fixture.uiIntent, seq: BigInt(fixture.uiIntent.seq) };
    const sent: any[] = [];
    const mailbox = new BrowserActorActionMailboxV1((request) => { sent.push(request); }, 1000);
    try {
      const pending = mailbox.dispatchIntent(owner, "map", intent);
      expect(sent).toHaveLength(1);
      expect(sent[0].actionSequence).toBe(1);
      expect(mailbox.pending()).toBe(1);
      const queued = mailbox.dispatchIntent(owner, "map", intent);
      expect(sent).toHaveLength(1);
      expect(mailbox.pending()).toBe(2);
      expect(mailbox.settle({ ...fixture.acknowledged, actionSequence: 2 })).toBe(false);
      for (const hostile of fixture.hostileResults.slice(0, 3)) expect(mailbox.settle({ ...hostile, actionSequence: 1 })).toBe(false);
      expect(mailbox.settle(fixture.acknowledged)).toBe(false);
      const acknowledged = { ...fixture.acknowledged, actionSequence: 1 };
      expect(mailbox.settle(acknowledged)).toBe(true);
      expect(equal(await pending, acknowledged)).toBe(true);
      expect(mailbox.settle(acknowledged)).toBe(false);
      expect(sent).toHaveLength(2);
      expect(sent[1].actionSequence).toBe(2);
      expect(mailbox.pending()).toBe(1);
      const queuedReceipt = { ...fixture.acknowledged, actionSequence: 2 };
      expect(mailbox.settle(queuedReceipt)).toBe(true);
      expect(equal(await queued, queuedReceipt)).toBe(true);
      expect(mailbox.pending()).toBe(0);
      const next = mailbox.dispatchIntent(owner, "map", intent);
      const retired = expect(next).rejects.toThrow("retired");
      expect(sent[2].actionSequence).toBe(3);
      mailbox.close("retired");
      await retired;
      expect(mailbox.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(false);
      await expect(mailbox.dispatchIntent(owner, "map", intent)).rejects.toThrow("closed");
      expect(mailbox.pending()).toBe(0);
    } finally {
      mailbox.close("test cleanup");
    }
    const rapidSent: number[] = [];
    const rapid = new BrowserActorActionMailboxV1((request) => { rapidSent.push(request.actionSequence); });
    try {
      const settledOrder: number[] = [];
      const commands = [1, 2, 3].map((sequence) => rapid.dispatchCommand(owner, fixture.actionInvocation, fixture.commandViewState).then((result) => { settledOrder.push(sequence); return result; }));
      expect(rapidSent).toEqual([1]);
      expect(rapid.pending()).toBe(3);
      expect(rapid.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(false);
      expect(rapid.settle({ ...fixture.acknowledged, actionSequence: 2 })).toBe(false);
      expect(rapid.settle({ ...fixture.acknowledged, actionSequence: 1 })).toBe(true);
      expect(rapidSent).toEqual([1, 2]);
      expect(rapid.settle({ ...fixture.acknowledged, actionSequence: 2 })).toBe(true);
      expect(rapidSent).toEqual([1, 2, 3]);
      expect(rapid.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(true);
      const results = await Promise.all(commands);
      expect(results.map((result) => result.actionSequence)).toEqual([1, 2, 3]);
      expect(settledOrder).toEqual([1, 2, 3]);
      expect(rapid.pending()).toBe(0);
    } finally {
      rapid.close("test cleanup");
    }
    const boundedSent: number[] = [];
    const bounded = new BrowserActorActionMailboxV1((request) => { boundedSent.push(request.actionSequence); }, 1000, 1);
    try {
      const first = bounded.dispatchCommand(owner, fixture.actionInvocation, fixture.commandViewState);
      const second = bounded.dispatchCommand(owner, fixture.actionInvocation, fixture.commandViewState);
      const overflow = bounded.dispatchCommand(owner, fixture.actionInvocation, fixture.commandViewState);
      await expect(overflow).rejects.toThrow("queue full");
      expect(await overflow.catch((error) => browserActorActionRefusalReasonV1(error))).toBe("queue-full");
      expect(boundedSent).toEqual([1]);
      expect(bounded.pending()).toBe(2);
      const firstClosed = expect(first).rejects.toThrow("owner retired");
      const secondClosed = expect(second).rejects.toThrow("owner retired");
      bounded.close("browser-actor-action: owner retired");
      await firstClosed;
      await secondClosed;
      expect(await first.catch((error) => browserActorActionRefusalReasonV1(error))).toBe("owner-mismatch");
      expect(bounded.pending()).toBe(0);
      expect(bounded.settle({ ...fixture.acknowledged, actionSequence: 1 })).toBe(false);
    } finally {
      bounded.close("test cleanup");
    }
    expect(() => new BrowserActorActionMailboxV1(() => {}, 1000, 0)).toThrow("invalid capacity");
    expect(browserActorActionRefusalReasonV1(new Error("browser-actor-action: mailbox closed"))).toBe("dispatch-failed");
    expect(browserActorActionRefusalReasonV1(new Error("browser-actor-action: completion unconfirmed"))).toBe("dispatch-failed");
    expect(browserActorActionRefusalReasonV1(new Error("action-owner-mismatch"))).toBe("owner-mismatch");
    expect(browserActorActionRefusalReasonV1(new Error("action-busy"))).toBe("owner-mismatch");
    expect(browserActorActionRefusalReasonV1(new Error("action-state-unconfirmed"))).toBe("not-applied");
    expect(browserActorActionRefusalReasonV1(new Error("action-catching-up"))).toBe("catching-up");
    expect(browserActorActionRefusalReasonV1(new Error("action-refused"))).toBe("dispatch-failed");
    expect(browserActorActionRefusalReasonV1(new Error("action-command-ingress-refused"))).toBe("dispatch-failed");
    expect(browserActorActionRefusalReasonV1(new Error("action-guest-refused"))).toBeNull();
    expect(browserActorActionRefusalReasonV1(new Error("enqueue failed"))).toBeNull();
    expect(browserActorActionRefusalReasonV1(42)).toBeNull();
    const failing = new BrowserActorActionMailboxV1(() => { throw new Error("enqueue failed"); });
    await expect(failing.dispatchIntent(owner, "map", intent)).rejects.toThrow("enqueue failed");
    expect(failing.pending()).toBe(0);
    failing.close("test cleanup");
    const retried: number[] = [];
    const retry = new BrowserActorActionMailboxV1((request) => { retried.push(request.actionSequence); });
    try {
      await expect(retry.dispatchIntent(owner, "map", { ...intent, revision: 4 })).rejects.toThrow("stale surface");
      expect(retried).toEqual([]);
      const refused = expect(retry.dispatchIntent(owner, "map", intent)).rejects.toThrow(fixture.rejected.reason);
      expect(retried).toEqual([2]);
      expect(retry.settle({ ...fixture.rejected, actionSequence: 2 })).toBe(true);
      await refused;
      const next = retry.dispatchIntent(owner, "map", intent);
      expect(retried).toEqual([2, 3]);
      expect(retry.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(true);
      await next;
    } finally {
      retry.close("test cleanup");
    }
    vi.useFakeTimers();
    const timedSent: number[] = [];
    const timed = new BrowserActorActionMailboxV1((request) => { timedSent.push(request.actionSequence); }, 10);
    try {
      const expired = expect(timed.dispatchIntent(owner, "map", intent)).rejects.toThrow("unconfirmed");
      const behind = timed.dispatchIntent(owner, "map", intent);
      expect(timedSent).toEqual([1]);
      await vi.advanceTimersByTimeAsync(9);
      expect(timedSent).toEqual([1]);
      await vi.advanceTimersByTimeAsync(1);
      await expired;
      expect(timedSent).toEqual([1, 2]);
      expect(timed.pending()).toBe(1);
      await vi.advanceTimersByTimeAsync(9);
      expect(timed.settle({ ...fixture.acknowledged, actionSequence: 2 })).toBe(true);
      expect((await behind).actionSequence).toBe(2);
      await vi.advanceTimersByTimeAsync(10);
      expect(timed.pending()).toBe(0);
      const later = timed.dispatchIntent(owner, "map", intent);
      expect(timedSent).toEqual([1, 2, 3]);
      expect(timed.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(true);
      await later;
    } finally {
      timed.close("test cleanup");
      vi.useRealTimers();
    }
  });
}
