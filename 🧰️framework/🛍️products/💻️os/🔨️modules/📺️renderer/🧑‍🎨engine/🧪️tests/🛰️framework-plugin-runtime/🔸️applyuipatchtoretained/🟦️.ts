/** 🔸️ The applyuipatchtoretained tests of the framework plugin runtime suite. */
import { describe, expect, it } from "vitest";
import { type UiNodeRecord } from "@semio-tech/framework";
import { applyUiPatchToRetained, decodeWirePatchOps, serializePerActor } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🔹 Registers the applyuipatchtoretained tests of the framework plugin runtime suite. */
export function registerApplyuipatchtoretainedTests(): void {
  describe("applyUiPatchToRetained", () => {
    const leaf = (id: number, value: string): UiNodeRecord => ({
      id,
      key: `leaf-${id}`,
      component: { type: "text", value, emphasize: null, dataAttributes: null },
      layout: { kind: "leaf", width: "hug", height: "hug" },
      style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
      activity: "idle",
      disabled: false,
      transition: null,
      accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
      bindings: [],
      menu: null,
      children: [],
    });

    it("applies the semantic upsert and set-root operations emitted by the actor WIT boundary", async () => {
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const ops = decodeWirePatchOps([
        {
          tag: "upsert",
          val: {
            node: Array.from(encodePackValue({ id: 0, key: "leaf-0", component: leaf(0, "a").component, layout: leaf(0, "a").layout, style: {}, activity: "idle", accessibility: {} })),
          },
        },
        { tag: "set-root", val: 0n },
      ]);
      const result = applyUiPatchToRetained(null, { surface: "s", revision: 1n, baseRevision: 0n, ops });
      expect(result.desynced).toBe(false);
      expect(result.surface?.revision).toBe(1);
      expect(result.surface?.nodes.get(0)?.component).toEqual({ type: "text", value: "a", emphasize: null, dataAttributes: null });
      expect(result.surface?.nodes.get(0)?.children).toEqual([]);
    });

    it("projects lossless pack integer carriers onto exact node ids, children and revisions", async () => {
      const { encodePackValue, packUInt } = await import("@semio-tech/framework-os");
      const ops = decodeWirePatchOps([
        {
          tag: "upsert",
          val: {
            node: Array.from(encodePackValue({ id: packUInt(1n), key: "leaf-1", component: leaf(1, "a").component, layout: leaf(1, "a").layout, style: {}, activity: "idle", accessibility: {}, children: [packUInt(3n)] })),
          },
        },
        { tag: "upsert", val: { node: Array.from(encodePackValue({ id: packUInt(3n), key: "leaf-3", component: leaf(3, "b").component, layout: leaf(3, "b").layout, style: {}, activity: "idle", accessibility: {} })) } },
        { tag: "set-children", val: { node: 1n, children: [3n] } },
        { tag: "set-root", val: 1n },
      ]);
      expect(ops.map((op) => op.type)).toEqual(["upsert", "upsert", "setChildren", "setRoot"]);
      expect(ops[0]).toMatchObject({ type: "upsert", id: 1, children: [3] });
      expect(ops[1]).toMatchObject({ type: "upsert", id: 3 });
      expect(ops[3]).toEqual({ type: "setRoot", id: 1 });
      const result = applyUiPatchToRetained(null, { surface: "s", revision: 1n, baseRevision: 0n, ops });
      expect(result.desynced).toBe(false);
      expect(result.surface?.nodes.get(1)?.children).toEqual([3]);
      expect(result.surface?.nodes.get(3)?.component).toEqual({ type: "text", value: "b", emphasize: null, dataAttributes: null });
      expect(() => decodeWirePatchOps([{ tag: "upsert", val: { node: Array.from(encodePackValue({ id: packUInt(2n ** 60n), key: "huge" })) } }])).toThrow(/upsert\.node\.id/u);
    });

    it("applies incremental semantic field updates with a matching base revision", () => {
      const first = applyUiPatchToRetained(null, {
        surface: "s",
        revision: 1,
        baseRevision: 0,
        ops: [
          { type: "upsert", ...leaf(0, "a") },
          { type: "setRoot", id: 0 },
        ],
      });
      const result = applyUiPatchToRetained(first.surface, { surface: "s", revision: 2, baseRevision: 1, ops: [{ type: "setComponent", id: 0, component: { type: "text", value: "b", emphasize: null, dataAttributes: null } }] });
      expect(result.desynced).toBe(false);
      expect(result.surface?.revision).toBe(2);
      expect(result.surface?.nodes.get(0)?.component).toEqual({ type: "text", value: "b", emphasize: null, dataAttributes: null });
    });

    it("keeps the previous body when a semantic patch has a stale base revision", () => {
      const { surface: previous } = applyUiPatchToRetained(null, {
        surface: "s",
        revision: 1,
        baseRevision: 0,
        ops: [
          { type: "upsert", ...leaf(0, "a") },
          { type: "setRoot", id: 0 },
        ],
      });
      const result = applyUiPatchToRetained(previous, { surface: "s", revision: 2, baseRevision: 0, ops: [{ type: "setComponent", id: 0, component: { type: "text", value: "b", emphasize: null, dataAttributes: null } }] });
      expect(result.desynced).toBe(true);
      expect(result.surface).toBe(previous);
    });

    it("keeps the previous body when a patch violates the retained document graph", () => {
      const { surface: previous } = applyUiPatchToRetained(null, {
        surface: "s",
        revision: 1,
        baseRevision: 0,
        ops: [
          { type: "upsert", ...leaf(0, "a") },
          { type: "setRoot", id: 0 },
        ],
      });
      const result = applyUiPatchToRetained(previous, { surface: "s", revision: 2, baseRevision: 1, ops: [{ type: "setChildren", id: 0, children: [99] }] });
      expect(result.desynced).toBe(true);
      expect(result.surface).toBe(previous);
    });
  });
  it("parses a typed InvocationResponse, including requestedEffects, from a plugin handle-action response", async () => {
    const { parseInvocationResponse } = await import("@semio-tech/framework");
    const response = parseInvocationResponse(
      JSON.stringify({
        output: null,
        mutations: [{ diff: { payload: { schemaId: "draw.operation", document: { id: "forest" } } } }],
        inverseGroup: { invocationId: "setActiveExample:1:0", mutations: [], inverseMutations: [] },
        requestedEffects: [{ navigate: { uri: "/spaces/forest" } }],
      }),
    );
    expect(response.mutations).toHaveLength(1);
    expect(response.requestedEffects).toEqual([{ navigate: { uri: "/spaces/forest" } }]);
  });
  it("falls back to an empty InvocationResponse for malformed handle-action JSON", async () => {
    const { parseInvocationResponse } = await import("@semio-tech/framework");
    expect(parseInvocationResponse("not json")).toEqual({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } });
    expect(parseInvocationResponse(JSON.stringify({ output: null }))).toEqual({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } });
  });
  it("serializePerActor queues concurrent turns for the same actor one at a time, never overlapping", async () => {
    let inFlight = 0;
    let maxInFlight = 0;
    const runOne = () =>
      serializePerActor("actor-1", async () => {
        inFlight += 1;
        maxInFlight = Math.max(maxInFlight, inFlight);
        await new Promise((resolve) => setTimeout(resolve, 5));
        inFlight -= 1;
        return "done";
      });
    const results = await Promise.all([runOne(), runOne(), runOne()]);
    expect(maxInFlight).toBe(1);
    expect(results).toEqual(["done", "done", "done"]);
  });
  it("serializePerActor keys independently per actor — different actors run concurrently, not queued behind each other", async () => {
    let concurrentAcrossActors = 0;
    let maxConcurrentAcrossActors = 0;
    const runOn = (actorId: string) =>
      serializePerActor(actorId, async () => {
        concurrentAcrossActors += 1;
        maxConcurrentAcrossActors = Math.max(maxConcurrentAcrossActors, concurrentAcrossActors);
        await new Promise((resolve) => setTimeout(resolve, 5));
        concurrentAcrossActors -= 1;
      });
    await Promise.all([runOn("actor-a"), runOn("actor-b")]);
    expect(maxConcurrentAcrossActors).toBe(2);
  });
  it("serializePerActor keeps queuing subsequent turns after an earlier one rejects", async () => {
    const order: string[] = [];
    const failing = serializePerActor("actor-2", async () => {
      order.push("first");
      throw new Error("turn faulted");
    });
    const succeeding = serializePerActor("actor-2", async () => {
      order.push("second");
      return "ok";
    });
    await expect(failing).rejects.toThrow("turn faulted");
    await expect(succeeding).resolves.toBe("ok");
    expect(order).toEqual(["first", "second"]);
  });
}
