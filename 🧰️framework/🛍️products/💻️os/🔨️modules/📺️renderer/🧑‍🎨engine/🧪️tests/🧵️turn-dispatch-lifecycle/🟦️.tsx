/** 🧵 submitPluginTurn: captured lifecycle scheduling, UI owner composition, command pages and acknowledgement crossings. */
import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";
import type { UiNodeRecord } from "@semio-tech/framework";
import type { ActivationRegistry } from "@semio-tech/framework-kernel";
import type { ShardEventEnvelope } from "@semio-tech/framework-actor/shard-client";
import type { ShardInstanceLifecycleLease } from "@semio-tech/framework-actor/shard-client";
import type { ShardWorkerLike } from "@semio-tech/framework-actor/shard-client";
import type { PluginWasmHandle, WireTurnResult } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import { flushMicrotasks, createPluginRuntimeSupport, type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerTurnDispatchLifecycleTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it, vi } = vitest;
  const { testState, consumeTypedOperationEffects, DEFAULT_SHARD_BUDGET, encodeActorUiPatchReceipt, encodePackValue, invocationFromFrames, loadPluginModule, OwnedResidentLedger, PLUGIN_TURN_MAILBOX_CAPACITY, retainedWindowByActor, settleAcknowledgedPluginTurns, settlePluginTurn, ShardClient, submitPluginLifecycleTurn, submitPluginTurn, teardownPluginActor, typedOperationAcknowledgements } = dependencies;
  const { withFakeShardClient } = createPluginRuntimeSupport(dependencies);
    //#region 🧪️terra-web-plugin-runtime
    describe("submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue)", () => {
      it("schedules captured lifecycle work through the original owner after operation revocation", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/⏱️lifecycle-scheduler.json");
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts");
        const { OwnedUiInstance } = await import("@semio-tech/framework/modules/ui/contract/retained/instance");
        const sent: Array<{ kind: string; requestId: string; events?: readonly ShardEventEnvelope[] }> = [];
        const worker: ShardWorkerLike = { onmessage: null, onerror: null, postMessage(message) { sent.push(message as typeof sent[number]); }, terminate() {} };
        const client = new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(), residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
        async function answer<T>(pending: Promise<T>, value: unknown): Promise<T> { await flushMicrotasks(8); const message = sent.at(-1)!; worker.onmessage!({ data: { kind: "result", requestId: message.requestId, ok: true, value } }); return pending; }
        await answer(client.activate(fixture.actor, "/fixture.js", [], DEFAULT_SHARD_BUDGET), undefined);
        const owner = client.captureInstanceLifecycle(fixture.actor, fixture.instance);
        const lifetime = { activationGeneration: owner.activation.activationGeneration, instanceId: fixture.instance, guestLifetime: BigInt(fixture.guestLifetime) };
        const captured = { kind: "captured" as const, lifetime, requestSequence: owner.openRequest.requestSequence };
        const input = { appId: "fixture", actor: {}, config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() };
        const raw = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(captured) };
        const opened = await answer(submitPluginLifecycleTurn(owner, { kind: "open", input }, "Interactive"), raw);
        expect(opened.owner).toBe(owner); expect(opened.raw).toBe(raw); expect(opened.turn.original).toBe(raw); expect(opened.turn.lifecycleReceipt).toBe(raw.lifecycleReceipt);
        const ui = new OwnedUiInstance(owner.activation, lifetime, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 });
        owner.bindHostRetirement(ui);
        const phases = [owner.progress().kind]; const kinds = [sent.at(-1)!.events?.[0]?.kind ?? null];
        const plainidle = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: captured }, "Interactive"), plainidle); phases.push(owner.progress().kind); kinds.push(sent.at(-1)!.events?.[0]?.kind ?? null);
        await answer(submitPluginLifecycleTurn(owner, { kind: "poll" }, "UserVisible"), plainidle); phases.push(owner.progress().kind); kinds.push(sent.at(-1)!.events?.[0]?.kind ?? null);
        const request = owner.beginClose(); ui.beginClose();
        await expect(submitPluginTurn(fixture.actor, [{ kind: "app-command", payload: {} }], "Interactive", undefined, undefined, owner.activation)).rejects.toThrow(/revoked/);
        const accepted = { kind: "accepted" as const, lifetime, requestSequence: request.requestSequence, closeGeneration: BigInt(fixture.closeGeneration) };
        const retired = { ...accepted, kind: "retired" as const };
        await answer(submitPluginLifecycleTurn(owner, { kind: "close" }, "Interactive"), { ...plainidle, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) }); phases.push(owner.progress().kind); kinds.push(sent.at(-1)!.events?.[0]?.kind ?? null);
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: accepted }, "Interactive"), { ...plainidle, lifecycleReceipt: encodeActorInstanceLifecycle(retired) }); phases.push(owner.progress().kind); kinds.push(sent.at(-1)!.events?.[0]?.kind ?? null);
        while (ui.closeStep({ maxItems: 1, maxBytes: 4096 }).kind !== "complete") {}
        const retirement = ui.takeRetirementWitness()!;
        const failed = submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: retired, retirement }, "Interactive");
        const observed = expect(failed).rejects.toThrow("actor-lifecycle.ack-not-admitted");
        await answer(failed.catch(() => undefined), { ...plainidle, status: { tag: "faulted", val: new Uint8Array([1]) } }); await observed; expect(owner.pendingReceipt).toEqual(retired);
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: retired, retirement }, "Interactive"), plainidle); phases.push(owner.progress().kind); kinds.push(sent.at(-1)!.events?.[0]?.kind ?? null);
        expect(phases).toEqual(fixture.phases); expect(kinds).toEqual(fixture.events);
        const before = sent.length;
        for (const kind of fixture.refusedWork) await expect(submitPluginLifecycleTurn(owner, { kind, events: [{ kind: "app-command", payload: {} }], run: () => { throw new Error("Unowned callback"); } } as never, "Interactive")).rejects.toThrow("actor-lifecycle.work-kind");
        expect(sent).toHaveLength(before);
        teardownPluginActor(fixture.actor); client.disposeAll();
      });

      it("schedules captured lifecycle UI ACKs with their exact private source and successful submission receipt", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/⏱️lifecycle-scheduler.json");
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts");
        const { encodeActorUiPatchReceipt } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts");
        const { OwnedUiInstance } = await import("@semio-tech/framework/modules/ui/contract/retained/instance");
        const sent: Array<{ kind: string; requestId: string; events?: readonly ShardEventEnvelope[] }> = [];
        const worker: ShardWorkerLike = { onmessage: null, onerror: null, postMessage(message) { sent.push(message as typeof sent[number]); }, terminate() {} };
        const client = new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(), residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
        async function answer<T>(pending: Promise<T>, value: unknown): Promise<T> { await flushMicrotasks(8); worker.onmessage!({ data: { kind: "result", requestId: sent.at(-1)!.requestId, ok: true, value } }); return pending; }
        const plain = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        const actorId = `${fixture.actor}-ui-ack`;
        await answer(client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET), undefined);
        const owner = client.captureInstanceLifecycle(actorId, fixture.instance);
        const lifetime = { activationGeneration: owner.activation.activationGeneration, instanceId: fixture.instance, guestLifetime: BigInt(fixture.guestLifetime) };
        const captured = { kind: "captured" as const, lifetime, requestSequence: owner.openRequest.requestSequence };
        await answer(submitPluginLifecycleTurn(owner, { kind: "open", input: { appId: "fixture", actor: {}, config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() } }, "Interactive"), { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle(captured) });
        const ui = new OwnedUiInstance(owner.activation, lifetime, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 }); owner.bindHostRetirement(ui);
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: captured }, "Interactive"), plain);
        const value = fixture.uiAcknowledgement;
        const receipt = { lifetime, patchSequence: BigInt(value.patchSequence) };
        const original = { ...plain, uiPatchReceipt: encodeActorUiPatchReceipt(receipt), uiPatches: [{ surface: { instance: fixture.instance, surface: value.surface }, revision: BigInt(value.revision), baseRevision: BigInt(value.baseRevision), ops: [] }] };
        const polled = await answer(submitPluginLifecycleTurn(owner, { kind: "poll" }, "UserVisible"), original);
        expect(polled.turn.uiPatchReceipt).toBe(original.uiPatchReceipt);
        const source = owner.captureUiPatchAuthority(original, 0);
        expect(source.value.operationCount).toBe(value.operationCount);
        const grant = { maxItems: 1, maxBytes: 4096 };
        const lookup = ui.beginSurfaceLookup(owner.activation, lifetime, value.surface)!;
        for (let count = 0; lookup.advance(grant).kind !== "ready"; count++) if (count > 1024) throw new Error("Fixture lookup did not complete");
        const facade = lookup.takeResult()!; lookup.beginClose(); while (lookup.closeStep(grant).kind !== "complete") {}
        const patch = ui.beginPatch(source, facade); patch.finishInput();
        for (let count = 0; patch.advance(grant).kind !== "ready"; count++) if (count > 1024) throw new Error("Fixture publication did not complete");
        const token = patch.peekAcknowledgement()!;
        const close = owner.beginClose(); ui.beginClose();
        const post = worker.postMessage; worker.postMessage = () => { throw new Error("fixture post refusal"); };
        await expect(submitPluginLifecycleTurn(owner, { kind: "issued-ui-acks", entries: [{ source, token }] }, "Interactive")).rejects.toThrow("fixture post refusal");
        expect(patch.peekAcknowledgement()).toBe(token); worker.postMessage = post;
        const submitted = await answer(submitPluginLifecycleTurn(owner, { kind: "issued-ui-acks", entries: [{ source, token }] }, "Interactive"), plain);
        expect(submitted.owner).toBe(owner); expect(submitted.raw).toBe(plain); expect(sent.at(-1)!.events?.[0]?.kind).toBe("patch-ack");
        expect(sent.at(-1)!.events).toHaveLength(1);
        expect(sent.at(-1)!.events?.[0]?.payload).toEqual({ receipt, surface: { instance: fixture.instance, surface: value.surface }, revision: BigInt(value.revision) });
        expect(submitted.submissions).toHaveLength(1);
        expect(patch.acceptAcknowledgement(submitted.submissions![0]!)).toBe(true); expect(patch.acceptAcknowledgement(submitted.submissions![0]!)).toBe(false);
        const accepted = { kind: "accepted" as const, lifetime, requestSequence: close.requestSequence, closeGeneration: BigInt(fixture.closeGeneration) };
        const retired = { ...accepted, kind: "retired" as const };
        await answer(submitPluginLifecycleTurn(owner, { kind: "close" }, "Interactive"), { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) });
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: accepted }, "Interactive"), { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle(retired) });
        for (let count = 0; ui.closeStep(grant).kind !== "complete"; count++) if (count > 1024) throw new Error("Fixture UI close did not complete");
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: retired, retirement: ui.takeRetirementWitness()! }, "Interactive"), plain);
        expect(owner.progress().kind).toBe("complete"); teardownPluginActor(actorId); client.disposeAll();
      });

      it("composes the production plugin runtime with one real UI owner through exact patch ACK and retirement", async () => {
        const { default: equal } = await import("fast-deep-equal");
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/⏱️lifecycle-scheduler.json");
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts");
        const previous = { registry: testState.sharedActivationRegistry, shard: testState.sharedShardClient, fetch: globalThis.fetch };
        const sent: Array<{ readonly kind: string; readonly events: readonly string[] }> = [];
        const plain = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        let lifetime: { readonly activationGeneration: bigint; readonly instanceId: number; readonly guestLifetime: bigint } | null = null;
        const closeGeneration = BigInt(fixture.closeGeneration);
        const node: UiNodeRecord = { id: 0, key: "root", component: { type: "text", value: "owned", emphasize: null, dataAttributes: null }, layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false, transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [] };
        const worker: ShardWorkerLike = {
          onmessage: null,
          onerror: null,
          postMessage(raw) {
            const message = raw as { readonly kind: string; readonly requestId?: string; readonly activationGeneration?: bigint; readonly events?: readonly ShardEventEnvelope[] };
            const events = (message.events ?? []).map(event => event.kind);
            if (message.kind === "dispose") { sent.push({ kind: message.kind, events }); return; }
            const requestId = message.requestId;
            if (!requestId) return;
            let value: unknown = undefined;
            if (message.kind === "turn") {
              sent.push({ kind: message.kind, events });
              const first = message.events?.[0];
              if (first?.kind === "instance-open") {
                const payload = first.payload as { readonly instance: number; readonly activationGeneration: bigint; readonly requestSequence: number };
                lifetime = { activationGeneration: payload.activationGeneration, instanceId: payload.instance, guestLifetime: BigInt(fixture.guestLifetime) };
                const captured = { kind: "captured" as const, lifetime, requestSequence: payload.requestSequence };
                value = {
                  ...plain,
                  lifecycleReceipt: encodeActorInstanceLifecycle(captured),
                  uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime, patchSequence: BigInt(fixture.uiAcknowledgement.patchSequence) }),
                  uiPatches: [{ surface: { instance: payload.instance, surface: fixture.runtimeUiComposition.surface }, revision: 1n, baseRevision: 0n, ops: [{ tag: "upsert", val: { node: encodePackValue(node) } }, { tag: "set-root", val: 0n }] }],
                };
              } else if (first?.kind === "instance-close") {
                const close = first.payload as { readonly requestSequence: number };
                value = { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "accepted", lifetime: lifetime!, requestSequence: close.requestSequence, closeGeneration }) };
              } else if (first?.kind === "instance-lifecycle-ack") {
                const receipt = (first.payload as { readonly receipt: { readonly kind: string; readonly requestSequence: number } }).receipt;
                value = receipt.kind === "accepted" ? { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "retired", lifetime: lifetime!, requestSequence: receipt.requestSequence, closeGeneration }) } : plain;
              } else value = plain;
            }
            queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value } }));
          },
          terminate() {},
        };
        const client = new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(), residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
        testState.sharedShardClient = client;
        testState.sharedActivationRegistry = { registerManifest: () => {}, activate: async (_plugin: string, actorId: string) => client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET), touch: () => {}, cancel: (actorId: string) => client.dispose(actorId) } as unknown as ActivationRegistry;
        globalThis.fetch = stubFetch(async () => new Response(JSON.stringify({ manifest: { pluginId: "owned-ui", apps: [] } }), { headers: { "content-type": "application/json" } }));
        let handle: PluginWasmHandle | null = null;
        try {
          handle = await loadPluginModule("owned-ui", "https://fixture.invalid/plugin.js");
          const instance = await handle.createApp("fixture");
          const actorId = `owned-ui#${instance}`;
          expect(retainedWindowByActor.has(actorId)).toBe(false);
          const openEvents = sent.filter(entry => entry.kind === "turn").flatMap(entry => entry.events);
          expect(openEvents).toEqual(fixture.runtimeUiComposition.openEvents);
          expect(equal(openEvents, fixture.runtimeUiComposition.openEvents)).toBe(true);
          const response = await handle.refreshUi(instance, { viewState: { locale: "en", terminology: "native", windowInstances: [{ id: fixture.runtimeUiComposition.surface, windowKindId: "fixture" }] }, windows: [{ key: fixture.runtimeUiComposition.surface, bodyKey: "root" }] });
          expect(response.windows).toMatchObject([{ key: fixture.runtimeUiComposition.surface, value: { key: "root", component: { type: "text", value: "owned" }, children: [] } }]);
          expect(retainedWindowByActor.has(actorId)).toBe(false);
          const beforeClose = sent.flatMap(entry => entry.kind === "dispose" ? [entry.kind] : entry.events).length;
          await handle.destroyApp(instance);
          const allEvents = sent.flatMap(entry => entry.kind === "dispose" ? [entry.kind] : entry.events);
          const closeEvents = allEvents.slice(beforeClose);
          expect(closeEvents).toEqual(fixture.runtimeUiComposition.closeEvents);
          expect(equal(closeEvents, fixture.runtimeUiComposition.closeEvents)).toBe(true);
          expect(retainedWindowByActor.has(actorId)).toBe(false);
        } finally {
          await handle?.dispose();
          testState.sharedActivationRegistry = previous.registry;
          testState.sharedShardClient = previous.shard;
          globalThis.fetch = previous.fetch;
        }
      });

      /** ⚖️ LAW: one 8 KiB command costs the host EXACTLY its page count plus the continuations the
       * guest itself asked for — the host contributes no round trip of its own.
       *
       * 🥽️ Ticket 26/09/02 wave B22 §1.3 measured **84.2 worker round trips per `registerBrushMesh`**
       * (842 posts over 10 consecutive mesh-only settles) and wave B24 attributed every one of them to
       * the guest: `plugin_exchange` answered `Pending` after each ingress move, and
       * `plugin_continue_typed_operations` advanced exactly one publication unit per call. This law
       * pins the OTHER half — that `runQueuedTurn` never polls while the guest is busy, never re-sends
       * an accepted page, and never issues a refresh per continuation — so the census stays readable as
       * `pages + guestContinuations` and a host-side regression cannot hide inside the guest's number.
       * Its native twins are `a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page` and
       * the continuation bound in `retained_operation_continues_after_command_admission_until_
       * publication_and_retirement`. */
      it("spends one worker turn per command page plus the guest's own continuations and none of its own", async () => {
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts");
        const COMMAND_BYTES = 8_192;
        const invocation = { address: { pluginId: "ingress-census", appId: "fixture", windowKindId: "editor", actionId: "registerBrushMesh" }, arguments: { payload: "m".repeat(7_800) } };
        const census: Array<{ readonly guestContinuations: number; readonly bytes: number; readonly pages: number; readonly turns: number }> = [];
        for (const guestContinuations of [0, 1, 4]) {
          const previous = { registry: testState.sharedActivationRegistry, shard: testState.sharedShardClient, fetch: globalThis.fetch };
          const plain = { uiPatches: [], effects: [], nextWake: null };
          const more = { ...plain, status: { tag: "more-work" } };
          const idle = { ...plain, status: { tag: "idle" } };
          const kinds: string[] = [];
          let counting = false;
          let bytes = 0;
          let declared = 0;
          let remaining = guestContinuations;
          let lifetime: { readonly activationGeneration: bigint; readonly instanceId: number; readonly guestLifetime: bigint } | null = null;
          const worker: ShardWorkerLike = {
            onmessage: null, onerror: null, terminate() {},
            postMessage(raw) {
              const message = raw as { readonly kind: string; readonly requestId?: string; readonly events?: readonly ShardEventEnvelope[]; readonly commandPage?: { readonly cursor: { readonly pageIndex: number; readonly pageCount: number }; readonly bytes: Uint8Array } };
              if (!message.requestId) return;
              let value: unknown = idle;
              if (message.kind === "turn") {
                const first = message.events?.[0];
                if (first?.kind === "instance-open") {
                  const payload = first.payload as { readonly instance: number; readonly activationGeneration: bigint; readonly requestSequence: number };
                  lifetime = { activationGeneration: payload.activationGeneration, instanceId: payload.instance, guestLifetime: 1n };
                  value = { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "captured", lifetime, requestSequence: payload.requestSequence }) };
                } else if (first?.kind === "instance-close") {
                  value = { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "accepted", lifetime: lifetime!, requestSequence: (first.payload as { readonly requestSequence: number }).requestSequence, closeGeneration: 1n }) };
                } else if (first?.kind === "instance-lifecycle-ack") {
                  const receipt = (first.payload as { readonly receipt: { readonly kind: string; readonly requestSequence: number } }).receipt;
                  value = receipt.kind === "accepted" ? { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "retired", lifetime: lifetime!, requestSequence: receipt.requestSequence, closeGeneration: 1n }) } : idle;
                } else if (message.commandPage) {
                  const cursor = message.commandPage.cursor;
                  if (counting) {
                    kinds.push(`command-page#${cursor.pageIndex}`);
                    bytes += message.commandPage.bytes.length;
                    declared = cursor.pageCount;
                  }
                  value = cursor.pageIndex + 1 === cursor.pageCount
                    ? { ...(remaining > 0 ? more : idle), commandIngress: { tag: "command-complete", val: {} } }
                    : { ...more, commandIngress: { tag: "page-accepted", val: {} } };
                } else if (counting) {
                  kinds.push("settle-continuation");
                  remaining -= 1;
                  value = remaining > 0 ? more : idle;
                }
              }
              queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId: message.requestId, ok: true, value } }));
            },
          };
          const client = new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(), residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
          testState.sharedShardClient = client;
          testState.sharedActivationRegistry = { registerManifest: () => {}, activate: async (_plugin: string, actorId: string) => client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET), touch: () => {}, cancel: (actorId: string) => client.dispose(actorId) } as unknown as ActivationRegistry;
          globalThis.fetch = stubFetch(async () => new Response(JSON.stringify({ manifest: { pluginId: "ingress-census", apps: [] } }), { headers: { "content-type": "application/json" } }));
          let handle: PluginWasmHandle | null = null;
          try {
            handle = await loadPluginModule("ingress-census", "https://fixture.invalid/plugin.js");
            const instance = await handle.createApp("fixture");
            counting = true;
            await handle.handleAction(instance, JSON.stringify({ ...invocation, address: { ...invocation.address, instanceId: String(instance) } }), { locale: "en", terminology: "native", windowInstances: [] } as never);
            counting = false;
            expect(bytes).toBeGreaterThan(COMMAND_BYTES - 1_024);
            expect(bytes).toBeLessThanOrEqual(COMMAND_BYTES);
            expect(declared).toBe(2);
            expect(kinds).toEqual([...Array.from({ length: declared }, (_, page) => `command-page#${page}`), ...Array.from({ length: guestContinuations }, () => "settle-continuation")]);
            census.push({ guestContinuations, bytes, pages: declared, turns: kinds.length });
          } finally {
            try { await handle?.dispose(); } finally {
              client.disposeAll();
              testState.sharedActivationRegistry = previous.registry;
              testState.sharedShardClient = previous.shard;
              globalThis.fetch = previous.fetch;
            }
          }
        }
        for (const row of census) expect(row.turns).toBe(row.pages + row.guestContinuations);
      });

      it("retries actual actor retirement with the original witness after final acknowledgement failure", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧪️tests/🧫️fixtures/🔣️.json");
        const { default: equal } = await import("fast-deep-equal");
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts");
        const { OwnedUiInstance } = await import("@semio-tech/framework/modules/ui/contract/retained/instance");
        for (const row of fixture.disposal.retirementRetry) {
          const previous = { registry: testState.sharedActivationRegistry, shard: testState.sharedShardClient, fetch: globalThis.fetch };
          const plain = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
          const events: string[] = [];
          let lifetime: { activationGeneration: bigint; instanceId: number; guestLifetime: bigint } | null = null;
          let refused = false;
          const worker: ShardWorkerLike = {
            onmessage: null, onerror: null, terminate() {},
            postMessage(raw) {
              const message = raw as { kind: string; requestId?: string; events?: readonly ShardEventEnvelope[] };
              if (message.kind === "dispose") { events.push("dispose"); return; }
              if (!message.requestId) return;
              let value: unknown = undefined;
              if (message.kind === "turn") {
                const first = message.events?.[0];
                if (first?.kind === "instance-open") {
                  const payload = first.payload as { activationGeneration: bigint; instance: number; requestSequence: number };
                  lifetime = { activationGeneration: payload.activationGeneration, instanceId: payload.instance, guestLifetime: 1n };
                  events.push("open");
                  value = { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "captured", lifetime, requestSequence: payload.requestSequence }) };
                } else if (first?.kind === "instance-close") {
                  const payload = first.payload as { requestSequence: number };
                  events.push("close");
                  value = { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "accepted", lifetime: lifetime!, requestSequence: payload.requestSequence, closeGeneration: 1n }) };
                } else if (first?.kind === "instance-lifecycle-ack") {
                  const receipt = (first.payload as { receipt: { kind: string; requestSequence: number } }).receipt;
                  events.push(`ack:${receipt.kind}`);
                  if (receipt.kind === "retired" && !refused) {
                    refused = true;
                    if (row.failure === "transport") throw new Error("fixture-final-ack-transport");
                    value = { ...plain, status: { tag: "faulted", val: new Uint8Array([1]) } };
                  } else value = receipt.kind === "accepted" ? { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "retired", lifetime: lifetime!, requestSequence: receipt.requestSequence, closeGeneration: 1n }) } : plain;
                } else value = plain;
              }
              queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId: message.requestId, ok: true, value } }));
            },
          };
          const client = new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(), residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
          const captured = vi.spyOn(client, "captureInstanceLifecycle");
          const witnesses = vi.spyOn(OwnedUiInstance.prototype, "takeRetirementWitness");
          testState.sharedShardClient = client;
          testState.sharedActivationRegistry = { registerManifest: () => {}, activate: async (_plugin: string, actorId: string) => client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET), touch: () => {}, cancel: () => {} } as unknown as ActivationRegistry;
          globalThis.fetch = stubFetch(async () => new Response(JSON.stringify({ manifest: { pluginId: "retirement-retry", apps: [] } }), { headers: { "content-type": "application/json" } }));
          let handle: PluginWasmHandle | null = null;
          try {
            handle = await loadPluginModule("retirement-retry", "https://fixture.invalid/plugin.js");
            const instance = await handle.createApp("fixture");
            const lease = captured.mock.results[0]!.value as ShardInstanceLifecycleLease;
            await expect(handle.destroyApp(instance)).rejects.toThrow(row.failure === "transport" ? "fixture-final-ack-transport" : "actor-lifecycle.ack-not-admitted");
            expect(lease.pendingReceipt?.kind).toBe("retired");
            expect(lease.progress().kind).toBe("blocked");
            expect(events).toEqual(row.events.slice(0, -2));
            expect(witnesses).toHaveBeenCalledTimes(row.witnesses);
            const receipt = lease.pendingReceipt;
            await handle.destroyApp(instance);
            expect(lease.progress().kind).toBe("complete");
            expect(lease.pendingReceipt).toBeNull();
            expect(receipt?.kind).toBe("retired");
            expect(witnesses).toHaveBeenCalledTimes(row.witnesses);
            expect(equal(events, row.events)).toBe(true);
            await handle.destroyApp(instance);
            expect(events).toEqual(row.events);
          } finally {
            try { await handle?.dispose(); } finally {
              captured.mockRestore(); witnesses.mockRestore(); client.disposeAll();
              testState.sharedActivationRegistry = previous.registry; testState.sharedShardClient = previous.shard; globalThis.fetch = previous.fetch;
            }
          }
        }
      });

      it("schedules captured lifecycle without silent eviction when either ingress fills the actor queue", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/⏱️lifecycle-scheduler.json");
        expect(fixture.mailbox.capacity).toBe(PLUGIN_TURN_MAILBOX_CAPACITY);
        for (const incoming of fixture.mailbox.overflow) {
          const actorId = `${fixture.actor}-capacity-${incoming}`;
          let release!: () => void;
          const raw = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
          const gate = new Promise<typeof raw>(resolve => { release = () => resolve(raw); });
          let first = true; let lifecycleDispatched = 0;
          const owner = { activation: { actorId }, async poll() { lifecycleDispatched++; return raw; } } as unknown as ShardInstanceLifecycleLease;
          await withFakeShardClient(async () => { if (first) { first = false; return gate; } return raw; }, async () => {
            const running = submitPluginTurn(actorId, [], "Interactive"); await flushMicrotasks(8);
            const pending = submitPluginLifecycleTurn(owner, { kind: "poll" }, "UserVisible");
            let lifecycleSettled = false; void pending.then(() => { lifecycleSettled = true; }, () => {});
            const commands = Array.from({ length: fixture.mailbox.capacity - 1 }, () => submitPluginTurn(actorId, [], "Interactive"));
            const overflow = (incoming === "operation" ? submitPluginTurn(actorId, [], "Interactive") : submitPluginLifecycleTurn(owner, { kind: "poll" }, "Interactive")).then(() => "accepted", () => "refused");
            expect(lifecycleDispatched).toBe(0);
            release(); await running; await Promise.all(commands); const outcome = await overflow; await flushMicrotasks(8);
            expect(outcome).toBe(fixture.mailbox.outcome); expect(lifecycleDispatched).toBe(1); expect(lifecycleSettled).toBe(true); expect((await pending).owner).toBe(owner);
            teardownPluginActor(actorId);
          });
        }
      });

      it("continues admitted operations after surfaces are retained and ACKs each exact result", async () => {
        const { Buffer } = await import("node:buffer");
        const { default: Ajv } = await import("ajv");
        const { default: equal } = await import("fast-deep-equal");
        const { default: settlement } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/📬️typed-operation-settlement.json");
        const { default: rendererModule } = await import("../../../🧬️schema/🔣️.json");
        const schema = { $ref: `${rendererModule.$id}#/$defs/PluginRuntimeTypedOperationSettlementV1` };
        expect(new Ajv({ strict: true }).addSchema(rendererModule).compile(schema)(settlement)).toBe(true);
        const { default: fixture } = await import("@semio-tech/framework-plugin/reactor/testing/fixtures.json");
        const token = Buffer.alloc(25);
        token.writeUInt32LE(fixture.wire.receiver, 0);
        token.writeBigUInt64LE(BigInt(fixture.wire.operation), 4);
        token.writeBigUInt64LE(BigInt(fixture.wire.generation), 12);
        token.writeUInt32LE(fixture.wire.sequence, 20);
        token[24] = fixture.wire.attempt;
        const expectedAck = Buffer.concat([Buffer.from("semio.typed-operation-ack.v1\0"), token]);
        const payload = Buffer.from(fixture.wire.payload);
        const length = Buffer.alloc(4);
        length.writeUInt32LE(payload.length);
        let turns = 0;
        await withFakeShardClient(async (_actor, events) => {
          if (turns > 0) expect(events).toEqual([{ kind: "message", payload: { source: { tag: "shell", val: String(fixture.wire.receiver) }, payload: Array.from(expectedAck) } }]);
          const lane = fixture.wire.lanes[turns++];
          const effects = lane === undefined ? [] : [{ tag: "send-message", val: { target: { tag: "shell", val: String(fixture.wire.receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), token, Buffer.from([lane]), length, payload])) } }];
          return { uiPatches: [], effects, nextWake: null, status: { tag: lane === undefined ? "idle" : "more-work" } };
        }, async () => {
          const result = await settlePluginTurn("retained-operation#7", { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } }, "Interactive", new Set(), undefined, true);
          expect(turns).toBe(fixture.wire.lanes.length + 1);
          expect(result.effects).toEqual(settlement.retainedEffects);
          expect(equal(result.effects, settlement.retainedEffects)).toBe(true);
          expect(result.status).toEqual(settlement.status);
          const invocation = invocationFromFrames([], result.effects, "action");
          expect({ output: invocation.output, requestedEffects: invocation.requestedEffects }).toEqual(settlement.invocation);
        });
      });

      it("retires every result page one turn carried in a single acknowledgement crossing", async () => {
        const { Buffer } = await import("node:buffer");
        const { default: fixture } = await import("@semio-tech/framework-plugin/reactor/testing/fixtures.json");
        const operations = 4;
        const tokenBytes = (operation: number): Buffer => {
          const token = Buffer.alloc(25);
          token.writeUInt32LE(fixture.wire.receiver, 0);
          token.writeBigUInt64LE(BigInt(operation + 1), 4);
          token.writeBigUInt64LE(BigInt(fixture.wire.generation), 12);
          token.writeUInt32LE(fixture.wire.sequence, 20);
          token[24] = fixture.wire.attempt;
          return token;
        };
        const payload = Buffer.from(fixture.wire.payload);
        const length = Buffer.alloc(4);
        length.writeUInt32LE(payload.length);
        const pageLane=fixture.wire.lanes[0];if(pageLane===undefined)throw Error("Authored typed operation lane required");
        const pageEffect = (operation: number) => ({
          tag: "send-message",
          val: { target: { tag: "shell", val: String(fixture.wire.receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), tokenBytes(operation), Buffer.from([pageLane]), length, payload])) },
        });
        const crossings: string[][] = [];
        await withFakeShardClient(async (_actor, events) => {
          crossings.push(events.map((event) => event.kind));
          return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        }, async () => {
          const carried = { uiPatches: [], effects: Array.from({ length: operations }, (_unused, operation) => pageEffect(operation)), nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult;
          expect(typedOperationAcknowledgements(carried)).toHaveLength(operations);
          await settlePluginTurn("typed-operation-batch#7", carried, "Interactive", new Set(), undefined, true);
        });
        expect(crossings).toHaveLength(1);
        expect(crossings[0]).toEqual(Array.from({ length: operations }, () => "message"));
      });

      it("does not replay already acknowledged ingress publications during settlement", async () => {
        const { Buffer } = await import("node:buffer");
        const { default: equal } = await import("fast-deep-equal");
        const { default: settlement } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/📬️typed-operation-settlement.json");
        const { default: fixture } = await import("@semio-tech/framework-plugin/reactor/testing/fixtures.json");
        const results: WireTurnResult[] = fixture.wire.lanes.map((lane, sequence) => {
          const header = Buffer.alloc(30);
          header.writeUInt32LE(fixture.wire.receiver, 0);
          header.writeBigUInt64LE(BigInt(fixture.wire.operation), 4);
          header.writeBigUInt64LE(BigInt(fixture.wire.generation), 12);
          header.writeUInt32LE(sequence, 20);
          header[24] = fixture.wire.attempt;
          header[25] = lane;
          return { uiPatches: [], effects: [{ tag: "send-message", val: { target: { tag: "shell", val: String(fixture.wire.receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), header])) } }], nextWake: null, status: { tag: "more-work" } };
        });
        const pending = typedOperationAcknowledgements(results.at(-1)!);
        await withFakeShardClient(async (_actor, events) => {
          expect(events).toEqual(pending);
          return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        }, async () => {
          const result = await settleAcknowledgedPluginTurns("retained-ingress#7", results, pending);
          expect(result.effects).toEqual(settlement.retainedEffects);
          expect(equal(consumeTypedOperationEffects(result.effects), settlement.retainedEffects)).toBe(true);
          expect(result.status).toEqual(settlement.status);
          const invocation = invocationFromFrames([], result.effects, "action");
          expect({ output: invocation.output, requestedEffects: invocation.requestedEffects }).toEqual(settlement.invocation);
        });
      });
    });
  //#endregion 🧪️terra-web-plugin-runtime
}
