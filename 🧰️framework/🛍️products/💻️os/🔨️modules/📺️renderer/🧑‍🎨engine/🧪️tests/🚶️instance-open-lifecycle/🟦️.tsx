/** 🚶 Instance-open retained UI lifecycle. */
import type { UiNodeRecord } from "@semio-tech/framework";
import type { RetainedSurface, WireTurnResult } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { createPluginRuntimeSupport, type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerInstanceOpenLifecycleTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it, vi } = vitest;
  const { applyUiPatchToRetained, encodeActorUiPatchReceipt, encodePackValue, patchAckEvents, pluginSurfaceRef, retainedSurfaceHash, retainedSurfaceId, retainedSurfaceToBuiltNode, retainedSurfaceToSnapshot, retainedUiRefreshResponse, uiRefreshSectionUnchanged, retainedWindowByActor, retainTurnUiPatches, SURFACE_RENDER_FAULT, uiRefreshBodyKeys } = dependencies;
  const { encodeFaultBytes } = createPluginRuntimeSupport(dependencies);
    describe("instance-open retained UI lifecycle", () => {
      it("preserves document effects returned by a refresh even before any surface is retained", async () => {
        const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧪️tests/🧫️fixtures/🔣️.json");
        const response = retainedUiRefreshResponse(7, { viewState: {} }, new Map(), [{ tag: "load-document", val: { docPack: fixture.effects.loadDocument.pack, spr: fixture.effects.loadDocument.spr } }]);
        expect(response.requestedEffects).toEqual([{ loadDocument: fixture.effects.loadDocument }]);
      });
  
      it("reports a refresh fault frame instead of returning an unchanged surface", async () => {
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧪️tests/🧫️fixtures/🔣️.json");
        const payload = encodeAppFrame({ Error: { in_reply_to: null, fault: Array.from(encodeFaultBytes(fixture.wire.fault)), report: [] } });
        expect(() => retainedUiRefreshResponse(7, { viewState: {} }, new Map(), [{ tag: "send-message", val: { target: { tag: "shell", val: "7" }, payload } }])).toThrow(fixture.wire.fault);
      });

      /** 🖼️ A surface-render fault is ONE surface's terminal report, not the turn's answer. The guest emits
       * it on whatever turn its reconciler happened to terminate on — a `DuplicateSiblingKey` in the
       * framework ToolRun panel reached the turn that was carrying a `flow-extension-brep` evaluation
       * completion and took every one of its effects down with it, so no evaluation result ever landed
       * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The report stays loud; the turn keeps its effects. */
      it("scopes a surface-render fault to its surface and keeps the same turn's other effects", async () => {
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧪️tests/🧫️fixtures/🔣️.json");
        const payload = encodeAppFrame({ Error: { in_reply_to: null, fault: Array.from(encodeFaultBytes(SURFACE_RENDER_FAULT)), report: [] } });
        const reported = vi.spyOn(console, "error").mockImplementation(() => {});
        try {
          const response = retainedUiRefreshResponse(7, { viewState: {} }, new Map(), [
            { tag: "send-message", val: { target: { tag: "shell", val: "7" }, payload } },
            { tag: "load-document", val: { docPack: fixture.effects.loadDocument.pack, spr: fixture.effects.loadDocument.spr } },
          ]);
          expect(response.requestedEffects).toEqual([{ loadDocument: fixture.effects.loadDocument }]);
          expect(reported.mock.calls.flat().join(" ")).toContain(SURFACE_RENDER_FAULT);
        } finally {
          reported.mockRestore();
        }
      });

      it("refreshes window and panel surfaces from the language-agnostic ownership cases", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🔄️surface-refresh.json");
        for (const testCase of fixture.cases) {
          const bodyKeys = uiRefreshBodyKeys(testCase.request);
          expect(bodyKeys, testCase.name).toEqual(testCase.bodyKeys);
          const retained = new Map<string, RetainedSurface>();
          for (const target of [...(testCase.request.windows ?? []), ...(testCase.request.panels ?? [])]) {
            const bodyKey = target.bodyKey ?? target.key;
            const root: UiNodeRecord = {
              id: 0, key: bodyKey, component: { type: "text", value: bodyKey, emphasize: null, dataAttributes: null },
              layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
              transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [],
            };
            const { surface, desynced } = applyUiPatchToRetained(null, {
              surface: target.key, revision: 1, baseRevision: 0,
              ops: [{ type: "upsert", ...root }, { type: "setRoot", id: 0 }],
            });
            expect(desynced).toBe(false);
            retained.set(retainedSurfaceId(7, target.key), surface!);
          }
          const response = retainedUiRefreshResponse(7, testCase.request, retained);
          expect(response.windows?.map((entry) => entry.key)).toEqual(testCase.windowKeys);
          expect(response.panels?.map((entry) => entry.key)).toEqual(testCase.panelKeys);
          for (const entry of [...(response.windows ?? []), ...(response.panels ?? [])]) {
            expect(entry.value).toMatchObject({ component: { type: "text" } });
            expect(entry.hash).not.toBe("");
          }
        }
      });

      /** 🪞️ Ticket 26/09/02 wave B4. `buildUiRefreshRequest` stamps every requested entry with the
       * host's cached hash so an unchanged body can answer without a payload; before this law both
       * projectors ignored it and rebuilt every requested tree on every refresh — measured on the live
       * Nakagin document switch at 51 projections / 1 275 retained-node walks for ONE switch, and,
       * worse, a fresh `BuiltNode` per unchanged body defeats `mergeRecordPreservingIdentity` and
       * re-reconciles the whole shell subtree. The clause that matters is `value === undefined` with
       * the hash echoed back: that is what lets the shell keep its exact object reference. */
      it("omits the payload of a requested body whose hash the host already holds and re-projects only the one that changed", () => {
        const body = (key: string, value: string): UiNodeRecord => ({
          id: 0, key, component: { type: "text", value, emphasize: null, dataAttributes: null },
          layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
          transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [],
        });
        const publish = (previous: RetainedSurface | null, key: string, revision: number, value: string): RetainedSurface => {
          const { surface, desynced } = applyUiPatchToRetained(previous, { surface: key, revision, baseRevision: revision - 1, ops: [{ type: "upsert", ...body(key, value) }, { type: "setRoot", id: 0 }] });
          expect(desynced).toBe(false);
          return surface!;
        };
        const retained = new Map<string, RetainedSurface>([
          [retainedSurfaceId(7, "steady"), publish(null, "steady", 1, "steady-one")],
          [retainedSurfaceId(7, "moving"), publish(null, "moving", 1, "moving-one")],
        ]);
        const request = { viewState: {}, windows: [{ key: "steady", bodyKey: "steady" }, { key: "moving", bodyKey: "moving" }] };
        const first = retainedUiRefreshResponse(7, request, retained);
        expect(first.windows?.map((entry) => entry.value !== undefined)).toEqual([true, true]);
        const hashes = Object.fromEntries((first.windows ?? []).map((entry) => [entry.key, entry.hash]));

        const echoed = retainedUiRefreshResponse(7, { ...request, windows: request.windows.map((target) => ({ ...target, hash: hashes[target.key] })) }, retained);
        expect(echoed.windows).toEqual([{ key: "steady", hash: hashes.steady }, { key: "moving", hash: hashes.moving }]);
        expect(echoed.windows?.every((entry) => entry.value === undefined)).toBe(true);

        retained.set(retainedSurfaceId(7, "moving"), publish(retained.get(retainedSurfaceId(7, "moving"))!, "moving", 2, "moving-two"));
        const partial = retainedUiRefreshResponse(7, { ...request, windows: request.windows.map((target) => ({ ...target, hash: hashes[target.key] })) }, retained);
        expect(partial.windows?.find((entry) => entry.key === "steady")).toEqual({ key: "steady", hash: hashes.steady });
        const moved = partial.windows?.find((entry) => entry.key === "moving");
        expect(moved?.hash).not.toBe(hashes.moving);
        expect(moved?.value).toMatchObject({ component: { type: "text", value: "moving-two" } });
      });

      it("holds the skip predicate to exact hash equality on a rooted surface", () => {
        expect(uiRefreshSectionUnchanged("abc", { root: 0, hash: "abc" })).toBe(true);
        expect(uiRefreshSectionUnchanged("abc", { root: 0, hash: "abd" })).toBe(false);
        expect(uiRefreshSectionUnchanged(undefined, { root: 0, hash: "abc" })).toBe(false);
        expect(uiRefreshSectionUnchanged("", { root: 0, hash: "" })).toBe(false);
        expect(uiRefreshSectionUnchanged("abc", { root: null, hash: "abc" })).toBe(false);
      });

      it("acknowledges only patches that identify the exact retained surface", () => {
        const receipt = { lifetime: { activationGeneration: 1n, instanceId: 4, guestLifetime: 1n }, patchSequence: 1n };
        const turn = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" }, uiPatchReceipt: encodeActorUiPatchReceipt(receipt) } as unknown as WireTurnResult;
        expect(
          patchAckEvents(turn, [
            { surface: pluginSurfaceRef(4, "workflow"), revision: 9n, baseRevision: 8n, ops: [] },
            { revision: 1n, baseRevision: 0n, ops: [] },
          ]),
        ).toEqual([{ kind: "patch-ack", payload: { receipt, surface: pluginSurfaceRef(4, "workflow"), revision: 9n } }]);
      });
  
      it("retains the first render patch so an unchanged surface-visible probe can reuse it", () => {
        const actorId = "initial-render-retention-test#1";
        expect(pluginSurfaceRef(1, "workflow")).toEqual({ instance: 1, surface: "workflow" });
        const root: UiNodeRecord = {
          id: 0,
          key: "root",
          component: { type: "text", value: "ready", emphasize: null, dataAttributes: null },
          layout: { kind: "leaf", width: "hug", height: "hug" },
          style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
          activity: "idle",
          disabled: false,
          transition: null,
          accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
          bindings: [],
          menu: null,
          children: [],
        };
        const surfaceId = retainedSurfaceId(1, "window");
        retainedWindowByActor.delete(actorId);
        try {
          retainTurnUiPatches(actorId, {
            uiPatches: [{ surface: pluginSurfaceRef(1, "window"), revision: 1n, baseRevision: 0n, ops: [{ tag: "upsert", val: { node: Array.from(encodePackValue(root)) } }, { tag: "set-root", val: 0n }] }],
          });
          const retained = retainedWindowByActor.get(actorId)?.get(surfaceId);
          expect(retained).toMatchObject({ surface: surfaceId, revision: 1, root: 0 });
          expect(() => retainedSurfaceHash(retainedSurfaceToSnapshot(retained!))).not.toThrow();
          expect(retainedSurfaceToBuiltNode(retained!)).toMatchObject({ key: "root", component: { type: "text", value: "ready" }, children: [] });
        } finally {
          retainedWindowByActor.delete(actorId);
        }
      });
    });
}
