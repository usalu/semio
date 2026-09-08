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

}
