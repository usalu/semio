type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { TurnScheduler } = dependencies;

  const { describe, expect, it, vi } = vitest;

  /** 🧪️ A deferred promise the test controls the settlement of, so `runTurn` can simulate real
   * async work without a real sleep — resolve/reject is driven by the test, not a timer. */
  function deferred<T>(): { readonly promise: Promise<T>; readonly resolve: (value: T) => void; readonly reject: (error: unknown) => void } {
    let resolve!: (value: T) => void;
    let reject!: (error: unknown) => void;
    const promise = new Promise<T>((res, rej) => {
      resolve = res;
      reject = rej;
    });
    return { promise, resolve, reject };
  }

  const flush = () => new Promise<void>((resolve) => queueMicrotask(() => queueMicrotask(resolve)));

  function harness<TPayload = string>(mailboxCapacity = 10) {
    const order: Array<{ actorId: string; payload: TPayload }> = [];
    const running = new Map<string, { readonly resolve: () => void; readonly reject: (error: unknown) => void }>();
    const scheduler = new TurnScheduler<TPayload, undefined>({
      mailboxCapacity,
      budgetFor: () => undefined,
      runTurn: (actorId, payload) => {
        order.push({ actorId, payload });
        const { promise, resolve, reject } = deferred<void>();
        running.set(actorId, { resolve, reject });
        return promise;
      },
    });
    return { scheduler, order, settle: (actorId: string) => running.get(actorId)?.resolve(), fail: (actorId: string, error: unknown) => running.get(actorId)?.reject(error) };
  }

  describe("TurnScheduler lane priority", () => {
    it("dispatches by lane priority, not arrival order, when a batch lands before the first pick", async () => {
      const { scheduler, order } = harness();
      scheduler.enqueue("low", { lane: "Background", payload: "low-1" });
      scheduler.enqueue("high", { lane: "Interactive", payload: "high-1" });
      scheduler.enqueue("mid", { lane: "UserVisible", payload: "mid-1" });
      await flush();
      expect(order.map((entry) => entry.actorId)).toEqual(["high", "mid", "low"]);
    });
  });

  describe("TurnScheduler per-actor ordering under interleaving", () => {
    it("never starts an actor's next turn before its current one settles, even while other actors interleave", async () => {
      const { scheduler, order, settle } = harness();
      scheduler.enqueue("a", { lane: "Interactive", payload: "a-1" });
      await flush();
      expect(order.map((entry) => entry.actorId)).toEqual(["a"]);

      // queue a-2 (behind a-1, still running) and a higher-lane turn for "b"
      scheduler.enqueue("a", { lane: "Interactive", payload: "a-2" });
      scheduler.enqueue("b", { lane: "Interactive", payload: "b-1" });
      await flush();
      // "a" is busy (a-1 still in flight) so only "b" can start; a-2 must not jump ahead of a-1
      expect(order.map((entry) => entry.actorId)).toEqual(["a", "b"]);
      expect(scheduler.isBusy("a")).toBe(true);

      settle("a");
      await flush();
      expect(order.map((entry) => entry.actorId)).toEqual(["a", "b", "a"]);
      expect(order[2]!.payload).toBe("a-2");

      settle("b");
      settle("a");
      await flush();
    });
  });

  describe("TurnScheduler coalescing", () => {
    it("collapses a burst of same-key envelopes to one queued turn, never 200 deep", async () => {
      const { scheduler, order, settle } = harness<number>();
      for (let i = 0; i < 200; i++) {
        const backpressure = scheduler.enqueue("pointer", { lane: "Interactive", coalesce: "pointer-move", payload: i });
        expect(backpressure.kind === "accept" || backpressure.kind === "coalesced").toBe(true);
      }
      expect(scheduler.pendingCount("pointer")).toBe(1);
      await flush();
      expect(order).toHaveLength(1);
      expect(order[0]!.payload).toBe(199);
      settle("pointer");
    });
  });

  describe("TurnScheduler backpressure at the cap", () => {
    it("rejected surfaces synchronously at the cap instead of the queue growing past it", () => {
      const { scheduler } = harness(2);
      expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "a" })).toEqual({ kind: "accept" });
      expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "b" })).toEqual({ kind: "accept" });
      // same lane, no coalesce key, nothing lower-priority to evict -> rejected, not silently dropped
      expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "c" })).toEqual({ kind: "rejected" });
      expect(scheduler.pendingCount("full")).toBe(2);
    });
  });

  describe("TurnScheduler cancellation", () => {
    it("cancels only queued turns, leaving an in-flight one to settle on its own", async () => {
      const { scheduler, order, settle } = harness();
      scheduler.enqueue("x", { lane: "Interactive", payload: "x-1" });
      await flush();
      expect(order.map((e) => e.payload)).toEqual(["x-1"]); // x-1 now in flight

      scheduler.enqueue("x", { lane: "Interactive", payload: "x-2" });
      scheduler.enqueue("x", { lane: "Background", payload: "x-3" });
      expect(scheduler.pendingCount("x")).toBe(2);

      const cancelled = scheduler.cancelQueued("x");
      expect(cancelled).toBe(2);
      expect(scheduler.pendingCount("x")).toBe(0);

      settle("x");
      await flush();
      // only x-1 ever ran — x-2/x-3 were cancelled before dispatch
      expect(order.map((e) => e.payload)).toEqual(["x-1"]);
    });

    it("teardownActor cancels queued work and forgets the actor so a later enqueue starts fresh", async () => {
      const { scheduler, order } = harness();
      const cancelledPayloads: string[] = [];
      scheduler.enqueue("y", { lane: "Interactive", payload: "y-1" });
      scheduler.enqueue("y", { lane: "Interactive", payload: "y-2" });
      const cancelled = scheduler.teardownActor("y", (payload) => cancelledPayloads.push(payload));
      expect(cancelled).toBe(2);
      expect(cancelledPayloads).toEqual(["y-1", "y-2"]);
      expect(scheduler.pendingCount("y")).toBe(0);
      await flush();
      expect(order).toHaveLength(0); // nothing ever dispatched — torn down before the first pump
    });
  });

  describe("TurnScheduler onTurnError", () => {
    it("reports a rejected runTurn instead of throwing out of the pump loop, and keeps draining", async () => {
      const errors: Array<{ actorId: string; error: unknown }> = [];
      const scheduler = new TurnScheduler<string, undefined>({
        mailboxCapacity: 5,
        budgetFor: () => undefined,
        runTurn: async (actorId, payload) => {
          if (payload === "boom") throw new Error("turn failed");
        },
        onTurnError: (actorId, error) => errors.push({ actorId, error }),
      });
      scheduler.enqueue("z", { lane: "Interactive", payload: "boom" });
      await flush();
      expect(errors).toHaveLength(1);
      expect(errors[0]!.actorId).toBe("z");
      expect(scheduler.isBusy("z")).toBe(false); // failure still frees the actor for its next turn
    });
  });

  void vi;

}
