/** 📨️ The envelope lane ordering, granted budget, wire adoption, command ingress and frame parity tests. */
import { readModuleTreeSource } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { flushAdmission } from "../🧰️settlement-test-kit/🟦️.ts";
import type { Lane, ShardBudget, ShardClientSettlementTestKit, ShardEnvelope, ShardFrame } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🗂 Registers the lane ordering, granted budget tracker, grant and envelope wire adoption, fixed command ingress pages and frame parity tests. */
export function registerEnvelopeOrderingAndFrameTests(kit: ShardClientSettlementTestKit): void {
  const { ACTOR_BYTE_PAGE_BYTES, BUDGET, MAINTENANCE_LANE_DEFAULT_BUDGET, SHARD_FRAME_VARIANT_FIELDS, activateActor, createGrantedBudgetTracker, createShardCommandIngressPages, describe, expect, harness, interpretShardFrame, it, orderEnvelopesByLane, testSource } = kit;
  function makeEnvelope(to: string, lane: Lane, seq: number, kind = "wake"): ShardEnvelope {
    return { to, from: { kind: "kernel" }, lane, seq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: {} } };
  }
  describe("orderEnvelopesByLane", () => {
    it("sorts by Lane priority, not arrival order, stable within a tied lane", () => {
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2), makeEnvelope("a", "Maintenance", 3), makeEnvelope("a", "Interactive", 4)];
      const ordered = orderEnvelopesByLane(envelopes);
      expect(ordered.map((envelope) => envelope.seq)).toEqual([2, 4, 1, 3]);
    });

    it("is a no-op for an already-lane-sorted, single-lane batch", () => {
      const envelopes = [makeEnvelope("a", "Interactive", 1), makeEnvelope("a", "Interactive", 2)];
      expect(orderEnvelopesByLane(envelopes).map((envelope) => envelope.seq)).toEqual([1, 2]);
    });
  });
  describe("GrantedBudgetTracker + interpretShardFrame", () => {
    it("a Grant records its budget and hands back envelopes in lane-priority order", () => {
      const tracker = createGrantedBudgetTracker();
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2)];
      const grantBudget: ShardBudget = { ...BUDGET, fuel: 999 };
      const result = interpretShardFrame({ kind: "Grant", actor: "a", budget: grantBudget, envelopes }, tracker);
      expect(result).toEqual({ action: "runEnvelopes", actor: "a", budget: grantBudget, envelopes: [envelopes[1], envelopes[0]] });
      expect(tracker.granted("a")).toBe(grantBudget);
    });

    it("an Envelope with no prior Grant runs under the Maintenance-lane default, never an invented constant", () => {
      const tracker = createGrantedBudgetTracker();
      const lonelyEnvelope = makeEnvelope("never-granted", "Interactive", 1);
      const result = interpretShardFrame({ kind: "Envelope", envelope: lonelyEnvelope }, tracker);
      expect(result).toEqual({ action: "runEnvelopes", actor: "never-granted", budget: MAINTENANCE_LANE_DEFAULT_BUDGET, envelopes: [lonelyEnvelope] });
    });

    it("an Envelope AFTER a Grant for the same actor runs under THAT granted budget — proving the old constant no longer influences it", () => {
      const tracker = createGrantedBudgetTracker();
      const grantBudget: ShardBudget = { ...BUDGET, fuel: 42 };
      interpretShardFrame({ kind: "Grant", actor: "a", budget: grantBudget, envelopes: [] }, tracker);
      const followUp = makeEnvelope("a", "Interactive", 5);
      const result = interpretShardFrame({ kind: "Envelope", envelope: followUp }, tracker);
      expect(result.action).toBe("runEnvelopes");
      expect((result as { readonly budget: ShardBudget }).budget).toBe(grantBudget);
      expect((result as { readonly budget: ShardBudget }).budget).not.toBe(MAINTENANCE_LANE_DEFAULT_BUDGET);
    });

    it("Register/Unregister are pure bookkeeping; Unregister forgets a previously granted budget", () => {
      const tracker = createGrantedBudgetTracker();
      interpretShardFrame({ kind: "Grant", actor: "a", budget: BUDGET, envelopes: [] }, tracker);
      expect(tracker.granted("a")).toBe(BUDGET);
      expect(interpretShardFrame({ kind: "Register", actor: "a" }, tracker)).toEqual({ action: "register", actor: "a" });
      expect(interpretShardFrame({ kind: "Unregister", actor: "a" }, tracker)).toEqual({ action: "unregister", actor: "a" });
      expect(tracker.granted("a")).toEqual(MAINTENANCE_LANE_DEFAULT_BUDGET); // forgotten — back to the floor, not a stale grant
    });

    it("an unknown/future frame variant resolves to 'unknown' instead of throwing (forward-compat)", () => {
      const tracker = createGrantedBudgetTracker();
      const futureFrame = { kind: "Checkpoint", actor: "a" } as unknown as ShardFrame;
      expect(() => interpretShardFrame(futureFrame, tracker)).not.toThrow();
      expect(interpretShardFrame(futureFrame, tracker)).toEqual({ action: "unknown", frame: futureFrame });
    });
  });
  describe("ShardClient.grant / ShardClient.envelope wire adoption", () => {
    it("grant() sends a ShardFrame::Grant frame with envelopes pre-sorted by lane, budget carried alongside them", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("a", "https://x/a.js", [], BUDGET);
      await flushAdmission();
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;

      const grantBudget: ShardBudget = { ...BUDGET, fuel: 12345 };
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2)];
      void client.grant("a", grantBudget, envelopes);

      const sent = workers[0]!.sent[1] as { readonly kind: string; readonly frame: ShardFrame };
      expect(sent.kind).toBe("frame");
      expect(sent.frame.kind).toBe("Grant");
      const grantFrame = sent.frame as { readonly kind: "Grant"; readonly actor: string; readonly budget: ShardBudget; readonly envelopes: readonly ShardEnvelope[] };
      expect(grantFrame.budget).toBe(grantBudget);
      expect(grantFrame.envelopes.map((envelope) => envelope.seq)).toEqual([2, 1]); // Interactive dispatched before Background
    });

    it("envelope() sends a ShardFrame::Envelope frame with NO budget field on the wire at all", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("a", "https://x/a.js", [], BUDGET);
      await flushAdmission();
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;

      void client.envelope(makeEnvelope("a", "Interactive", 1));
      const sent = workers[0]!.sent[1] as { readonly kind: string; readonly frame: ShardFrame };
      expect(sent.kind).toBe("frame");
      expect(sent.frame.kind).toBe("Envelope");
      expect(Object.keys(sent.frame)).toEqual(["kind", "envelope"]); // structurally budget-less on the wire
    });

    it("turn()/activate() keep working completely unchanged alongside the new frame wire (incremental adoption really is incremental)", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("legacy", "https://x/legacy.js", [], BUDGET);
      await flushAdmission();
      const activateMsg = workers[0]!.sent[0] as { readonly kind: string; readonly requestId: string };
      expect(activateMsg.kind).toBe("activate");
      workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;

      const turnPromise = client.turn("legacy", [{ kind: "wake", payload: {} }], BUDGET);
      const turnMsg = workers[0]!.sent[1] as { readonly kind: string; readonly requestId: string };
      expect(turnMsg.kind).toBe("turn");
      workers[0]!.deliver({ kind: "result", requestId: turnMsg.requestId, ok: true, value: { effects: [] } });
      await expect(turnPromise).resolves.toEqual({ effects: [] });
    });
  });
  describe("fixed command ingress pages", () => {
    it("matches a DataView little-endian oracle across a full page boundary", () => {
      const command = Uint8Array.from({ length: ACTOR_BYTE_PAGE_BYTES + 5 }, (_, index) => index & 0xff);
      const pages = createShardCommandIngressPages({ owner: 7n, generation: 11n, commandIndex: 1, commandCount: 3, instance: 13, seq: 17n, command });
      expect(pages).toHaveLength(2);
      expect(pages.map((page) => page.page.length)).toEqual([ACTOR_BYTE_PAGE_BYTES, 5]);
      expect(pages.map((page) => Object.keys(page))).toEqual([["cursor", "bytes", "page"], ["cursor", "bytes", "page"]]);
      expect(pages[0]!.cursor).toMatchObject({ owner: 7n, generation: 11n, commandIndex: 1, commandCount: 3, instance: 13, seq: 17n, kind: 0, pageIndex: 0, pageCount: 2 });
      const oracle = new DataView(command.buffer, command.byteOffset, command.byteLength);
      expect(pages[0]!.page.block00.word0).toBe(oracle.getBigUint64(0, true));
      expect(pages[0]!.page.block63.word7).toBe(oracle.getBigUint64(ACTOR_BYTE_PAGE_BYTES - 8, true));
      expect(pages[1]!.page.block00.word0).toBe(0x0000000403020100n);
      expect(pages[1]!.page.block00.word1).toBe(0n);
      expect(pages[1]!.page.block63.word7).toBe(0n);
      expect(pages[0]!.bytes.length).toBe(pages[0]!.page.length);
      expect(pages[1]!.bytes.length).toBe(pages[1]!.page.length);
    });

    it("defines reactor page.length so wasm poll can destructure a bytes-shaped ingress", () => {
      const page = createShardCommandIngressPages({ owner: 1n, generation: 1n, commandIndex: 0, commandCount: 1, instance: 1, seq: 1n, command: Uint8Array.of(9, 8, 7) })[0]!;
      expect(page.page).toEqual(expect.objectContaining({ length: 3 }));
      const { length } = page.page;
      expect(length).toBe(3);
      expect(page.bytes).toEqual(Uint8Array.of(9, 8, 7));
    });

    it("forwards the fixed page as the dedicated turn argument", async () => {
      const { client, workers } = harness(1);
      await activateActor(client, workers, "paged");
      const page = createShardCommandIngressPages({ owner: 1n, generation: 1n, commandIndex: 0, commandCount: 1, instance: 1, seq: 1n, command: Uint8Array.of(9, 8, 7) })[0]!;
      void client.turn("paged", [], BUDGET, page);
      expect(workers[0]!.sent.at(-1)).toMatchObject({ kind: "turn", actorId: "paged", events: [], commandPage: page });
      expect(Object.keys(page)).toEqual(["cursor", "bytes", "page"]);
    });
  });
  describe("ShardFrame parity with Rust component.rs", () => {
    it("TS ShardFrame variant/field names match the live Rust enum in 🖥️host/🧵️shard/🦀️.rs", async () => {
      const { readFileSync } = await import("node:fs");
      const rustUrl = new URL("../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs", testSource.url);
      const source = readModuleTreeSource(rustUrl);
      const enumMatch = source.match(/pub enum ShardFrame \{([\s\S]*?)\n\}\s*\n\s*impl ShardFrame/);
      expect(enumMatch).not.toBeNull(); // [TRACE] `pub enum ShardFrame { ... } impl ShardFrame` shape not found — Rust source changed, update this test's regex
      const body = enumMatch![1]!.replace(/\/\/\/.*$/gm, "").replace(/\/\/.*$/gm, "");
      const variantPattern = /(\w+)\s*(?:\{([^{}]*)\}|\(([^()]*)\))?\s*,/g;
      const rustVariants: Array<{ readonly name: string; readonly fields: readonly string[] | null }> = [];
      let match: RegExpExecArray | null;
      while ((match = variantPattern.exec(body)) !== null) {
        const [, name, structFields, tupleType] = match;
        if (structFields !== undefined) {
          const fields = structFields
            .split(",")
            .map((part) => part.trim())
            .filter((part) => part.length > 0)
            .map((part) => part.split(":")[0]!.trim());
          rustVariants.push({ name: name!, fields });
        } else if (tupleType !== undefined) {
          rustVariants.push({ name: name!, fields: null });
        }
      }

      // Same variant NAMES, in the same order, as this file's own runtime twin.
      expect(rustVariants.map((variant) => variant.name)).toEqual(SHARD_FRAME_VARIANT_FIELDS.map((variant) => variant.kind));
      for (const rustVariant of rustVariants) {
        if (rustVariant.fields === null) continue; // tuple variant (`Envelope(Envelope)`) — Rust has no field name to diff against; see SHARD_FRAME_VARIANT_FIELDS's own doc
        const tsVariant = SHARD_FRAME_VARIANT_FIELDS.find((variant) => variant.kind === rustVariant.name)!;
        expect(tsVariant.fields).toEqual(rustVariant.fields);
      }
    });
  });
}
