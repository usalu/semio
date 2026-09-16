type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { createBoundedMailbox } = dependencies;

  const { describe, expect, it } = vitest;

  describe("createBoundedMailbox", () => {
    it("overflow is rejected (not a silent drop) when nothing lower-priority exists to evict", () => {
      const mailbox = createBoundedMailbox<string>(2);
      expect(mailbox.enqueue({ lane: "Maintenance", payload: "a" })).toEqual({ kind: "accept" });
      expect(mailbox.enqueue({ lane: "Maintenance", payload: "b" })).toEqual({ kind: "accept" });
      expect(mailbox.enqueue({ lane: "Maintenance", payload: "c" })).toEqual({ kind: "rejected" });
      expect(mailbox.length).toBe(2);
    });

    it("coalescing collapses same-key entries latest-wins, preserving queue position", () => {
      const mailbox = createBoundedMailbox<number>(10);
      for (let i = 0; i < 200; i++) {
        const backpressure = mailbox.enqueue({ lane: "Interactive", coalesce: "pointer-move", payload: i });
        expect(backpressure.kind === "accept" || backpressure.kind === "coalesced").toBe(true);
      }
      expect(mailbox.length).toBe(1);
      expect(mailbox.popNext()?.payload).toBe(199);
    });

    it("lane priority beats FIFO order on popNext", () => {
      const mailbox = createBoundedMailbox<string>(10);
      mailbox.enqueue({ lane: "Maintenance", payload: "low" });
      mailbox.enqueue({ lane: "Background", payload: "mid" });
      mailbox.enqueue({ lane: "Interactive", payload: "high" });
      expect(mailbox.popNext()?.lane).toBe("Interactive");
      expect(mailbox.popNext()?.lane).toBe("Background");
      expect(mailbox.popNext()?.lane).toBe("Maintenance");
      expect(mailbox.isEmpty).toBe(true);
    });

    it("dropped backpressure reports the evicted lane, admitting the higher-priority incomer", () => {
      const mailbox = createBoundedMailbox<string>(2);
      mailbox.enqueue({ lane: "Maintenance", payload: "a" });
      mailbox.enqueue({ lane: "Background", payload: "b" });
      const backpressure = mailbox.enqueue({ lane: "Interactive", payload: "c" });
      expect(backpressure).toEqual({ kind: "dropped", lane: "Maintenance" });
      expect(mailbox.length).toBe(2);
      expect(mailbox.popNext()?.payload).toBe("c");
      expect(mailbox.popNext()?.payload).toBe("b");
    });
  });

  describe("createBoundedMailbox causal order (INPUT-CAUSALITY-LEDGER §2 B)", () => {
    const drain = <T>(mailbox: ReturnType<typeof createBoundedMailbox<T>>): T[] => {
      const out: T[] = [];
      let envelope: { payload: T } | undefined;
      while ((envelope = mailbox.popNext()) !== undefined) out.push(envelope.payload);
      return out;
    };

    it("(a) without `order` nothing changes: pure arrival order within a lane", () => {
      const mailbox = createBoundedMailbox<string>(10);
      for (const payload of ["u1", "u2", "u3", "u4"]) mailbox.enqueue({ lane: "Interactive", payload });
      expect(drain(mailbox)).toEqual(["u1", "u2", "u3", "u4"]);
    });

    it("(b) an ordered envelope never overtakes unordered envelopes queued ahead of every larger-ordered one", () => {
      // ordered-only rule: three unordered turns, then an ordered one with a small key — there is
      // no queued envelope with a larger `order` to land in front of, so it appends (arrival order).
      const mailbox = createBoundedMailbox<string>(10);
      mailbox.enqueue({ lane: "Interactive", payload: "u1" });
      mailbox.enqueue({ lane: "Interactive", payload: "u2" });
      mailbox.enqueue({ lane: "Interactive", payload: "u3" });
      expect(mailbox.enqueue({ lane: "Interactive", order: 0, payload: "o0" })).toEqual({ kind: "accept" });
      expect(drain(mailbox)).toEqual(["u1", "u2", "u3", "o0"]);
    });

    it("(b') L2: a follow-up of input N lands before the queued input N+1 and ahead of maintenance queued behind it", () => {
      const mailbox = createBoundedMailbox<string>(10);
      mailbox.enqueue({ lane: "Interactive", order: 10, payload: "input-10" });
      mailbox.enqueue({ lane: "Interactive", order: 11, payload: "input-11" });
      mailbox.enqueue({ lane: "Interactive", payload: "maintenance-after-11" });
      mailbox.enqueue({ lane: "Interactive", order: 12, payload: "input-12" });
      // guest follow-up caused by input 10 arrives last, keyed causedBy = 10
      expect(mailbox.enqueue({ lane: "Interactive", order: 10, payload: "followup-of-10" })).toEqual({ kind: "accept" });
      expect(drain(mailbox)).toEqual(["input-10", "followup-of-10", "input-11", "maintenance-after-11", "input-12"]);
    });

    it("(c) two ordered envelopes dequeue by `order` regardless of arrival", () => {
      const mailbox = createBoundedMailbox<string>(10);
      mailbox.enqueue({ lane: "Interactive", order: 7, payload: "o7" });
      mailbox.enqueue({ lane: "Interactive", order: 3, payload: "o3" });
      mailbox.enqueue({ lane: "Interactive", order: 5, payload: "o5" });
      expect(mailbox.length).toBe(3);
      expect(drain(mailbox)).toEqual(["o3", "o5", "o7"]);
    });

    it("(d) equal `order` keeps arrival order", () => {
      const mailbox = createBoundedMailbox<string>(10);
      mailbox.enqueue({ lane: "Interactive", order: 4, payload: "first" });
      mailbox.enqueue({ lane: "Interactive", order: 4, payload: "second" });
      mailbox.enqueue({ lane: "Interactive", order: 4, payload: "third" });
      mailbox.enqueue({ lane: "Interactive", order: 2, payload: "earlier-key" });
      expect(drain(mailbox)).toEqual(["earlier-key", "first", "second", "third"]);
    });

    it("(e) a coalesced replacement keeps the original position and takes the newer `order` when given", () => {
      const mailbox = createBoundedMailbox<string>(10);
      mailbox.enqueue({ lane: "Interactive", order: 1, payload: "o1" });
      mailbox.enqueue({ lane: "Interactive", coalesce: "pointer-move", order: 2, payload: "move-a" });
      mailbox.enqueue({ lane: "Interactive", order: 3, payload: "o3" });
      // newer sample with a much larger key must NOT move behind o3 — position wins
      expect(mailbox.enqueue({ lane: "Interactive", coalesce: "pointer-move", order: 99, payload: "move-b" })).toEqual({ kind: "coalesced" });
      expect(mailbox.length).toBe(3);
      expect(mailbox.popNext()?.payload).toBe("o1");
      const replaced = mailbox.popNext();
      expect(replaced?.payload).toBe("move-b");
      expect(replaced?.order).toBe(99);
      expect(mailbox.popNext()?.payload).toBe("o3");
    });

    it("(e') a coalesced replacement without `order` retains the original's key in place", () => {
      const mailbox = createBoundedMailbox<string>(10);
      mailbox.enqueue({ lane: "Interactive", coalesce: "k", order: 5, payload: "old" });
      mailbox.enqueue({ lane: "Interactive", order: 8, payload: "o8" });
      expect(mailbox.enqueue({ lane: "Interactive", coalesce: "k", payload: "new" })).toEqual({ kind: "coalesced" });
      // the retained key 5 still anchors later ordered insertions: 6 lands after it, before 8
      mailbox.enqueue({ lane: "Interactive", order: 6, payload: "o6" });
      const head = mailbox.popNext();
      expect(head?.payload).toBe("new");
      expect(head?.order).toBe(5);
      expect(drain(mailbox)).toEqual(["o6", "o8"]);
    });

    it("(f) lanes still outrank `order`: a smaller key in a lower lane waits for the higher lane", () => {
      const mailbox = createBoundedMailbox<string>(10);
      mailbox.enqueue({ lane: "Background", order: 0, payload: "bg-0" });
      mailbox.enqueue({ lane: "Interactive", order: 50, payload: "hi-50" });
      mailbox.enqueue({ lane: "UserVisible", order: 1, payload: "uv-1" });
      mailbox.enqueue({ lane: "Interactive", order: 40, payload: "hi-40" });
      expect(drain(mailbox)).toEqual(["hi-40", "hi-50", "uv-1", "bg-0"]);
    });

    it("capacity semantics are unchanged for ordered envelopes: reject with nothing to evict, evict the victim lane's head otherwise", () => {
      const full = createBoundedMailbox<string>(2);
      expect(full.enqueue({ lane: "Maintenance", order: 2, payload: "a" })).toEqual({ kind: "accept" });
      expect(full.enqueue({ lane: "Maintenance", order: 1, payload: "b" })).toEqual({ kind: "accept" });
      expect(full.enqueue({ lane: "Maintenance", order: 0, payload: "c" })).toEqual({ kind: "rejected" });
      expect(full.length).toBe(2);
      expect(drain(full)).toEqual(["b", "a"]);

      const evicting = createBoundedMailbox<string>(2);
      evicting.enqueue({ lane: "Maintenance", order: 9, payload: "m9" });
      evicting.enqueue({ lane: "Maintenance", order: 1, payload: "m1" }); // now the Maintenance head
      expect(evicting.enqueue({ lane: "Interactive", order: 5, payload: "i5" })).toEqual({ kind: "dropped", lane: "Maintenance" });
      expect(evicting.length).toBe(2);
      expect(drain(evicting)).toEqual(["i5", "m9"]);
    });
  });

}
