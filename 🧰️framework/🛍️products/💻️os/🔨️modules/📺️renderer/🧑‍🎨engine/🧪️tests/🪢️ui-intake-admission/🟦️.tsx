/** 🪢 Owned ui intake admission rounds and the react host activation reason. */
import { fixtureOriginalWorkStock } from "../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/🟦️.ts";
import type { UiNodeRecord } from "@semio-tech/framework";
import type { ActivationRegistry } from "../../../../../../../🔨️modules/🎠️kernel/🎛️activation-registry/🟦️.ts";
import type { ShardClient, ShardEventEnvelope, ShardWorkerLike } from "../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import type { PluginWasmHandle } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import { flushMicrotasks, type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerUiIntakeAdmissionTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it } = vitest;
  const { testState, DEFAULT_SHARD_BUDGET, encodeActorUiPatchReceipt, encodePackValue, loadPluginModule, OwnedResidentLedger, ShardClient, submitPluginLifecycleTurn, teardownPluginActor } = dependencies;
  /** 🧺️ ONE OWNING INTAKE PER INSTANCE (ticket 26/09/09, lane `concurrent-patch-intake`).
   *
   * 🐛️ A turn page carries every patch that was ready when the turn assembled its result, and the
   * reactor's pending slots are minted per reconcile pass rather than per surface — so one
   * `toolRunStart` put TWO patches for `framework.panel.toolRun` in one page (the action's own panel
   * reconcile and the run's first progress reconcile). The host admitted every patch of a page to its
   * acknowledgement token BEFORE acknowledging any of them, so the second met the first's still-open
   * wire and receipt outbox in `OwnedUiInstance.beginPatch` and threw
   * `plugin-ui.intake-rejected:intake:Foreign or busy instance surface owner`; the throw skipped the
   * page's whole close, and that surface's cell stayed busy for the rest of the instance's life —
   * every later refresh and every `toolRunAbort`/`toolRunFinalize` rejected, and the Tool runs panel
   * never left `Running` (peer ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS,
   * `abort-finalize-midrun-*.txt`, :6013 2026-09-15 17:58). */
  describe("owned ui intake admission rounds", () => {
    it("opens the next round at a repeated surface, so one round never names a surface twice", () => {
      const { uiPatchAdmissionRoundsV1 } = dependencies;
      expect(uiPatchAdmissionRoundsV1(["1:a", "1:b", "1:a"])).toEqual([[0, 1], [2]]);
      expect(uiPatchAdmissionRoundsV1(["1:a", "1:b", "1:c"])).toEqual([[0, 1, 2]]);
      expect(uiPatchAdmissionRoundsV1(["1:a", "1:a", "1:a"])).toEqual([[0], [1], [2]]);
      expect(uiPatchAdmissionRoundsV1(["1:a", "1:b", "1:a", "1:b", "1:c"])).toEqual([[0, 1], [2, 3, 4]]);
      expect(uiPatchAdmissionRoundsV1([])).toEqual([]);
      expect(uiPatchAdmissionRoundsV1([null, null])).toEqual([[0, 1]]);
      for (const surfaces of [["1:a", "1:b", "1:a"], ["1:a", "1:a", "1:a"], ["1:a", "1:b", "1:a", "1:b", "1:c"]]) {
        const rounds = uiPatchAdmissionRoundsV1(surfaces);
        expect(rounds.flat()).toEqual(surfaces.map((_, index) => index));
        for (const round of rounds) expect(new Set(round.map((index) => surfaces[index]!)).size).toBe(round.length);
      }
    });

    it("names a foreign surface owner and a busy one apart, and answers whether a patch is still open", async () => {
      const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/⏱️lifecycle-scheduler.json");
      const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts");
      const { encodeActorUiPatchReceipt } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts");
      const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts");
      const sent: Array<{ kind: string; requestId: string; events?: readonly ShardEventEnvelope[] }> = [];
      const worker: ShardWorkerLike = { onmessage: null, onerror: null, postMessage(message) { sent.push(message as typeof sent[number]); }, terminate() {} };
      const client = new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(), residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
      async function answer<T>(pending: Promise<T>, value: unknown): Promise<T> { await flushMicrotasks(8); worker.onmessage!({ data: { kind: "result", requestId: sent.at(-1)!.requestId, ok: true, value } }); return pending; }
      const plain = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
      const actorId = `${fixture.actor}-intake-rounds`;
      await answer(client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET), undefined);
      const owner = client.captureInstanceLifecycle(actorId, fixture.instance);
      const lifetime = { activationGeneration: owner.activation.activationGeneration, instanceId: fixture.instance, guestLifetime: BigInt(fixture.guestLifetime) };
      const captured = { kind: "captured" as const, lifetime, requestSequence: owner.openRequest.requestSequence };
      await answer(submitPluginLifecycleTurn(owner, { kind: "open", input: { appId: "fixture", actor: {}, config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() } }, "Interactive"), { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle(captured) });
      const ui = new OwnedUiInstance(owner.activation, lifetime, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 });
      owner.bindHostRetirement(ui);
      await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: captured }, "Interactive"), plain);
      const value = fixture.uiAcknowledgement;
      const grant = { maxItems: 1, maxBytes: 4096 };
      const page = (revision: number) => ({ ...plain, uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime, patchSequence: BigInt(value.patchSequence + revision) }), uiPatches: [{ surface: { instance: fixture.instance, surface: value.surface }, revision: BigInt(revision), baseRevision: BigInt(revision - 1), ops: [] }] });
      const openSurface = async (revision: number) => {
        const original = page(revision);
        await answer(submitPluginLifecycleTurn(owner, { kind: "poll" }, "UserVisible"), original);
        const source = owner.captureUiPatchAuthority(original, 0);
        const lookup = ui.beginSurfaceLookup(owner.activation, lifetime, value.surface)!;
        for (let count = 0; lookup.advance(grant).kind !== "ready"; count += 1) if (count > 1024) throw new Error("Fixture lookup did not complete");
        const facade = lookup.takeResult()!;
        lookup.beginClose();
        while (lookup.closeStep(grant).kind !== "complete") {}
        return { source, facade };
      };
      const first = await openSurface(1);
      expect(ui.surfacePatchIsOpen(first.facade)).toBe(false);
      const patch = ui.beginPatch(first.source, first.facade);
      expect(ui.surfacePatchIsOpen(first.facade)).toBe(true);
      const second = await openSurface(2);
      expect(() => ui.beginPatch(second.source, second.facade)).toThrow("Busy instance surface owner");
      const foreign = new OwnedUiInstance(owner.activation, lifetime, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 });
      expect(() => foreign.beginPatch(second.source, second.facade)).toThrow("Foreign native instance patch owner");
      patch.finishInput();
      for (let count = 0; patch.advance(grant).kind !== "ready"; count += 1) if (count > 1024) throw new Error("Fixture publication did not complete");
      const token = patch.peekAcknowledgement()!;
      expect(ui.surfacePatchIsOpen(first.facade)).toBe(true);
      const submitted = await answer(submitPluginLifecycleTurn(owner, { kind: "issued-ui-acks", entries: [{ source: first.source, token }] }, "Interactive"), plain);
      expect(patch.acceptAcknowledgement(submitted.submissions![0]!)).toBe(true);
      patch.beginClose();
      for (let count = 0; !patch.terminalIsEmpty(); count += 1) { patch.closeStep(grant); if (count > 4096) throw new Error("Fixture patch close did not complete"); }
      expect(ui.surfacePatchIsOpen(first.facade)).toBe(false);
      expect(() => ui.beginPatch(second.source, second.facade)).not.toThrow();
      ui.beginClose();
      for (let count = 0; ui.closeStep(grant).kind !== "complete"; count += 1) if (count > 65536) break;
      foreign.beginClose();
      teardownPluginActor(actorId);
      client.disposeAll();
    });

    it("installs a turn page that names one surface twice, in order, and leaves no surface wedged", async () => {
      const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/⏱️lifecycle-scheduler.json");
      const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts");
      const previous = { registry: testState.sharedActivationRegistry, shard: testState.sharedShardClient, fetch: globalThis.fetch };
      const plain = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
      const closeGeneration = BigInt(fixture.closeGeneration);
      const panel = fixture.runtimeUiComposition.surface;
      const other = `${fixture.runtimeUiComposition.surface}-other`;
      const leaf = (text: string): UiNodeRecord => ({ id: 0, key: "root", component: { type: "text", value: text, emphasize: null, dataAttributes: null }, layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false, transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [] } as UiNodeRecord);
      const ops = (text: string) => [{ tag: "upsert", val: { node: encodePackValue(leaf(text)) } }, { tag: "set-root", val: 0n }];
      let lifetime: { readonly activationGeneration: bigint; readonly instanceId: number; readonly guestLifetime: bigint } | null = null;
      let patchSequence = 0n;
      let revision = 0n;
      const installs: string[] = [];
      const refusals: string[] = [];
      const worker: ShardWorkerLike = {
        onmessage: null,
        onerror: null,
        postMessage(raw) {
          const message = raw as { readonly kind: string; readonly requestId?: string; readonly events?: readonly ShardEventEnvelope[] };
          const requestId = message.requestId;
          if (!requestId) return;
          let value: unknown = undefined;
          if (message.kind === "turn") {
            const first = message.events?.[0];
            if (first?.kind === "instance-open") {
              const payload = first.payload as { readonly instance: number; readonly activationGeneration: bigint; readonly requestSequence: number };
              lifetime = { activationGeneration: payload.activationGeneration, instanceId: payload.instance, guestLifetime: BigInt(fixture.guestLifetime) };
              patchSequence += 1n;
              // 🧺️ THE page the defect needs: `panel` twice, with a sibling between them, `A'.base = A.revision`.
              value = {
                ...plain,
                lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "captured" as const, lifetime, requestSequence: payload.requestSequence }),
                uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime, patchSequence }),
                uiPatches: [
                  { surface: { instance: payload.instance, surface: panel }, revision: 1n, baseRevision: 0n, ops: ops("running") },
                  { surface: { instance: payload.instance, surface: other }, revision: 1n, baseRevision: 0n, ops: ops("sibling") },
                  { surface: { instance: payload.instance, surface: panel }, revision: 2n, baseRevision: 1n, ops: ops("aborting") },
                ],
              };
              revision = 2n;
            } else if (first?.kind === "instance-close") {
              const close = first.payload as { readonly requestSequence: number };
              value = { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "accepted", lifetime: lifetime!, requestSequence: close.requestSequence, closeGeneration }) };
            } else if (first?.kind === "instance-lifecycle-ack") {
              const receipt = (first.payload as { readonly receipt: { readonly kind: string; readonly requestSequence: number } }).receipt;
              value = receipt.kind === "accepted" ? { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "retired", lifetime: lifetime!, requestSequence: receipt.requestSequence, closeGeneration }) } : plain;
            } else if (first?.kind === "surface-visible") {
              // 🔁️ Every later refresh publishes the panel again — the crossing the wedged cell used to reject.
              patchSequence += 1n;
              revision += 1n;
              value = { ...plain, uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: lifetime!, patchSequence }), uiPatches: [{ surface: { instance: lifetime!.instanceId, surface: panel }, revision, baseRevision: revision - 1n, ops: ops(`refresh-${revision}`) }] };
            } else value = plain;
          }
          queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value } }));
        },
        terminate() {},
      };
      const client = new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(), residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
      testState.sharedShardClient = client;
      testState.sharedActivationRegistry = { registerManifest: () => {}, activate: async (_plugin: string, actorId: string) => client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET), touch: () => {}, cancel: (actorId: string) => client.dispose(actorId) } as unknown as ActivationRegistry;
      globalThis.fetch = stubFetch(async () => new Response(JSON.stringify({ manifest: { pluginId: "intake-rounds", apps: [] } }), { headers: { "content-type": "application/json" } }));
      const consoleError = console.error;
      console.error = (...args: unknown[]) => { refusals.push(args.map(String).join(" ")); };
      let handle: PluginWasmHandle | null = null;
      try {
        handle = await loadPluginModule("intake-rounds", "https://fixture.invalid/plugin.js");
        const instance = await handle.createApp("fixture");
        const viewState = { locale: "en" as const, terminology: "native" as const, windowInstances: [{ id: panel, windowKindId: "fixture" }, { id: other, windowKindId: "fixture" }] };
        const projected = await handle.refreshUi(instance, { viewState, windows: [{ key: panel, bodyKey: "root" }, { key: other, bodyKey: "root" }] });
        for (const window of projected.windows ?? []) installs.push(`${window.key}=${(window.value as { component?: { value?: string } } | undefined)?.component?.value ?? "none"}`);
        // 🔁️ A second refresh proves the panel's cell is not wedged: it publishes again and projects again.
        const again = await handle.refreshUi(instance, { viewState, windows: [{ key: panel, bodyKey: "root" }] });
        const panelAgain = (again.windows ?? []).find((window) => window.key === panel);
        expect(installs).toContain(`${other}=sibling`);
        expect(installs.some((entry) => entry.startsWith(`${panel}=`))).toBe(true);
        expect((panelAgain?.value as { component?: { value?: string } } | undefined)?.component?.value).toMatch(/^refresh-/);
        expect(refusals.filter((line) => line.includes("intake-rejected") || line.includes("Busy instance surface owner"))).toEqual([]);
        await handle.destroyApp(instance);
      } finally {
        console.error = consoleError;
        await handle?.dispose();
        testState.sharedActivationRegistry = previous.registry;
        testState.sharedShardClient = previous.shard;
        globalThis.fetch = previous.fetch;
      }
    });
  });

  /** 🎬️ The React DOM host's OWN activation reason, at the one call site that has it: `createApp`.
   *
   * `🎬️activation-owner` already proves `activationReasonForAppId` → `activationEventEnvelope` → the
   * guest's WIT `activate`, but it drives `ActivationRegistry` directly. Nothing held the REACT host to
   * passing the real reason into it, which is why `🔌️PluginRuntime`'s `createApp` sat on a literal
   * `"manual"` until 2026-09-19 (`📓️o2-activation-follow-ups.md` §5 gap 1) with every suite green. The
   * law is the host's: opening a foreign artifact kind activates its owner `on-artifact-kind:<kind>`,
   * and a bare landing app id stays `manual` — which marshals to NO guest event at all, the honest
   * delivery for a trigger the WIT variant has no case for. */
  describe("react host activation reason", () => {
    it("activates a foreign-kind open as on-artifact-kind:<kind> and a landing app as manual", async () => {
      const { activationEventEnvelope } = await import("@semio-tech/framework");
      const previous = { registry: testState.sharedActivationRegistry, shard: testState.sharedShardClient, fetch: globalThis.fetch };
      const activations: { readonly pluginId: string; readonly actorId: string; readonly reason: string }[] = [];
      testState.sharedActivationRegistry = {
        registerManifest: () => {},
        activate: async (pluginId: string, actorId: string, reason: string) => { activations.push({ pluginId, actorId, reason }); },
        touch: () => {},
        cancel: () => {},
      } as unknown as ActivationRegistry;
      testState.sharedShardClient = {
        captureInstanceLifecycle: () => { throw new Error("fixture-open-not-configured"); },
        dispose: () => {},
      } as unknown as ShardClient;
      globalThis.fetch = stubFetch(async () => new Response(JSON.stringify({ manifest: { pluginId: "beta", apps: [] } }), { headers: { "content-type": "application/json" } }));
      let handle: PluginWasmHandle | undefined;
      try {
        handle = await loadPluginModule("beta", "https://fixture.invalid/plugin.js");
        await expect(handle.createApp("s.beta.sheet@1/*#editor")).rejects.toThrow("fixture-open-not-configured");
        await expect(handle.createApp("home")).rejects.toThrow("fixture-open-not-configured");
      } finally {
        await handle?.dispose().catch(() => {});
        testState.sharedActivationRegistry = previous.registry;
        testState.sharedShardClient = previous.shard;
        globalThis.fetch = previous.fetch;
      }
      expect(activations.map((entry) => entry.reason)).toEqual(["on-artifact-kind:s.beta.sheet", "manual"]);
      expect(activations.map((entry) => entry.pluginId)).toEqual(["beta", "beta"]);
      expect(activationEventEnvelope(activations[0]!.reason as "on-artifact-kind:s.beta.sheet")).toEqual({ kind: "activate", payload: { instance: 0, reason: { tag: "on-artifact-kind", val: "s.beta.sheet" } } });
      expect(activationEventEnvelope(activations[1]!.reason as "manual")).toBeUndefined();
    });
  });
}
