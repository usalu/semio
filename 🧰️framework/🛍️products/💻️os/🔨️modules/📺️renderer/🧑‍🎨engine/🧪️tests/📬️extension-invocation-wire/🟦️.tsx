/** 📬 Extension invocation WIT identity, icon export and host effect wire decoding, completion publication and command-ingress fault diagnostics. */
import { encodeAppFrame } from "@semio-tech/framework-os";
import type { ActivationRegistry } from "../../../../../../../🔨️modules/🎠️kernel/🎛️activation-registry/🟦️.ts";
import type { ShardClient, ShardEventEnvelope, ShardJobStep } from "../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import type { PluginWasmHandle, WireTurnResult, WireVariant } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import { cancelSpawnedJobV1, resetSpawnedJobsForTestsV1, spawnedJobsSnapshotV1 } from "../../🧱️elements/🔌️PluginRuntime/💼️job-ledger/🟦️.ts";
import { createPluginRuntimeSupport, type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerExtensionInvocationWireTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it } = vitest;
  const { testState, assertAddressedInvocation, commandIngressFaultDisplay, decodePackValue, decodePackWire, DEFAULT_SHARD_BUDGET, encodePackValue, loadPluginModule, retainedWindowByActor, SemioFaultError, serializeCommandIngressForActor, wireEffectToFriendly, wireExtensionInvocation, yieldPluginUiContinuation } = dependencies;
  const { encodeFaultBytes } = createPluginRuntimeSupport(dependencies);
    describe("extension invocation WIT request identity", () => {
      it("rejects narrowed, exhausted or malformed request identities", () => {
        for (const req of [0n, -1n, 0x10000000000000000n, 1, "1", undefined]) {
          expect(() => wireExtensionInvocation({ tag: "invoke-extension", val: { req, params: { extensionId: "text", capability: "evaluate", payload: [] } } })).toThrow("extension.request-id-invalid");
        }
      });
  
      it("preserves nested UTF-8 payloads and exact u64 ids in both renderer decoders", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        const { wireEffectToFriendly: sharedDecode } = await import("../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts");
        const requestJson = JSON.stringify({ ...fixture.request, label: "Hölzer 日本語" });
        for (const id of fixture.requestIds) {
          const req = BigInt(id);
          const wire = { tag: "invoke-extension", val: { req, params: { extensionId: fixture.extensionId, capability: fixture.capability, payload: new TextEncoder().encode(requestJson) } } };
          const expected = { invokeExtension: { req, extensionId: fixture.extensionId, capability: fixture.capability, requestJson } };
          expect(wireEffectToFriendly(wire)).toEqual(expected);
          expect(sharedDecode(wire, decodePackValue)).toEqual(expected);
        }
      });
    });

    describe("icon export wire decoding", () => {
      it("preserves every packed export item in both renderer doors and rejects malformed envelopes", async () => {
        const { default: fixture } = await import("../../🧱️elements/🖼️IconRenderHost/🧪️tests/🧫️fixtures/📡️export-wire/🔣️.json");
        const { default: schema } = await import("../../🧱️elements/🖼️IconRenderHost/🧬️schema/📡️export-wire/🔣️.json");
        const { default: Ajv2020 } = await import("ajv/dist/2020");
            const { wireEffectToFriendly: sharedDecode } = await import("../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts");
        const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
        for (const sample of fixture.cases) {
          expect(validate(sample.items), sample.name).toBe(sample.valid);
          const wire = { tag: "icon-render-export", val: { items: encodePackValue(sample.items) } };
          for (const decode of [wireEffectToFriendly, (effect: WireVariant) => sharedDecode(effect, decodePackWire)]) {
            if (sample.valid) expect(decode(wire), sample.name).toEqual({ iconRenderExport: { items: sample.items } });
            else expect(() => decode(wire), sample.name).toThrow("icon-export.items-invalid");
          }
        }
      });
    });

    describe("host effect address decoding", () => {
      it("reads every request-carrying effect out of its nested WIT params record", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🎯️host-effect-address.json");
        const nested = (tag: string, req: number, params: Record<string, unknown>) => ({ tag, val: { req, params } });
        const { req: dispatchReq, ...dispatchParams } = fixture.dispatchAction;
        expect(wireEffectToFriendly(nested("dispatch-action", dispatchReq, dispatchParams))).toEqual({ dispatchAction: { req: dispatchReq, action: fixture.dispatchAction.action, args: undefined, delayMs: fixture.dispatchAction.delayMs } });
        const { req: windowReq, ...windowParams } = fixture.openWindow;
        expect(wireEffectToFriendly(nested("open-window", windowReq, windowParams))).toEqual({ openWindow: { req: windowReq, kind: fixture.openWindow.kind, params: undefined } });
        const { req: dialogReq, ...dialogParams } = fixture.openDialog;
        expect(wireEffectToFriendly(nested("open-dialog", dialogReq, dialogParams))).toEqual({ openDialog: { req: dialogReq, dialogId: fixture.openDialog.dialogId, args: undefined } });
        const { req: spawnReq, ...spawnParams } = fixture.spawnPluginInstance;
        expect(wireEffectToFriendly(nested("spawn-plugin-instance", spawnReq, spawnParams))).toEqual({ spawnPluginInstance: { req: spawnReq, ...spawnParams } });
      });

      it("drops a dispatch-action whose action id never survived the boundary", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🎯️host-effect-address.json");
        expect(wireEffectToFriendly({ tag: "dispatch-action", val: { ...fixture.dispatchAction } })).toBeNull();
        expect(wireEffectToFriendly({ tag: "dispatch-action", val: { req: fixture.dispatchAction.req, params: { action: "", delayMs: 0 } } })).toBeNull();
      });

      it("refuses an unaddressed invocation with a typed renderer fault naming its window kind", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🎯️host-effect-address.json");
        const address = { pluginId: "procedural", appId: fixture.spawnPluginInstance.appId, windowKindId: fixture.windowKindId, actionId: "" };
        let thrown: unknown;
        try {
          assertAddressedInvocation({ address, arguments: {} }, "action", fixture.instanceId);
        } catch (error) {
          thrown = error;
        }
        expect(thrown).toBeInstanceOf(SemioFaultError);
        const fault = (thrown as InstanceType<typeof SemioFaultError>).fault;
        expect(fault.code).toBe(fixture.unaddressedFaultCode);
        expect(fault.origin).toBe(fixture.unaddressedFaultOrigin);
        expect(fault.message).toContain(fixture.windowKindId);
        expect(fault.scope.instanceId).toBe(String(fixture.instanceId));
        expect(() => assertAddressedInvocation({ address: { ...address, actionId: fixture.dispatchAction.action } }, "action", fixture.instanceId)).not.toThrow();
        expect(() => assertAddressedInvocation({ address: { commandId: "" } }, "command", fixture.instanceId)).toThrow(SemioFaultError);
        expect(() => assertAddressedInvocation({ address: { commandId: "open" } }, "command", fixture.instanceId)).not.toThrow();
      });
    });

    describe("extension invocation completion publication", () => {
      async function withRequester(turn: (actor: string, events: readonly ShardEventEnvelope[]) => Promise<WireTurnResult>, run: (handle: PluginWasmHandle, instance: number, activation: { replace(): void; captures(): number; guardedTurns(): number }) => Promise<void>, hooks: { activate?: (actor: string) => Promise<void>; open?: (actor: string) => Promise<void>; dispose?: (actor: string) => void; jobs?: { start?: (job: bigint, kind: string, input: Uint8Array) => void; step?: (job: bigint, budget: { fuel: bigint; deadlineMs: number }) => ShardJobStep; cancel?: (job: bigint) => void } } = {}): Promise<void> {
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts");
        const previous = { registry: testState.sharedActivationRegistry, shard: testState.sharedShardClient, fetch: globalThis.fetch };
        const idle: WireTurnResult = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        let generation = 1n;
        let captures = 0;
        let guardedTurns = 0;
        const disposed = new Set<string>();
        const dispose = (actor: string) => { if (!disposed.has(actor)) { disposed.add(actor); hooks.dispose?.(actor); } };
        const dispatch = async (actor: string, events: readonly ShardEventEnvelope[]) => {
          if (events.some(event => event.kind === "instance-open")) { await hooks.open?.(actor); return idle; }
          return turn(actor, events);
        };
        testState.sharedActivationRegistry = { registerManifest: () => {}, activate: async (_plugin: string, actor: string) => hooks.activate?.(actor), touch: () => {}, cancel: dispose } as unknown as ActivationRegistry;
        testState.sharedShardClient = {
          turn: dispatch,
          startJob: async (_actorId: string, job: bigint, kind: string, input: Uint8Array) => { hooks.jobs?.start?.(job, kind, input); },
          stepJob: async (_actorId: string, job: bigint, budget: { fuel: bigint; deadlineMs: number }) => hooks.jobs?.step?.(job, budget) ?? ({ status: "running" } as ShardJobStep),
          cancelJob: async (_actorId: string, job: bigint) => { hooks.jobs?.cancel?.(job); },
          captureInstanceLifecycle: (actorId: string, instanceId: number) => {
            const activationGeneration = generation;
            const activation = { actorId, activationGeneration, assertActive: () => { if (generation !== activationGeneration) throw new Error("actor-activation.revoked"); } };
            const lifetime = { activationGeneration, instanceId, guestLifetime: 1n };
            const openRequest = { kind: "open" as const, activationGeneration, instanceId, requestSequence: 1 };
            const captured = { kind: "captured" as const, lifetime, requestSequence: openRequest.requestSequence };
            let phase: "opening" | "captured" | "open" | "closing" | "accepted" | "retired" | "complete" = "opening";
            let pending: import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts").ActorInstanceLifecycleReceipt | null = null;
            const closeRequest = { kind: "close" as const, lifetime, requestSequence: 2 };
            const accepted = { kind: "accepted" as const, lifetime, requestSequence: closeRequest.requestSequence, closeGeneration: 1n };
            const retired = { ...accepted, kind: "retired" as const };
            return {
              activation,
              openRequest,
              get lifetime() { return phase === "opening" ? null : lifetime; },
              get pendingReceipt() { return pending; },
              interruptedTurn: null,
              open: async () => { await hooks.open?.(actorId); phase = "captured"; pending = captured; return { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle(captured) }; },
              poll: async () => idle,
              beginClose: () => { if (phase === "open") phase = "closing"; return closeRequest; },
              close: async () => { phase = "accepted"; pending = accepted; return { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) }; },
              acknowledge: async (receipt: typeof captured | typeof accepted | typeof retired) => {
                if (receipt.kind === "captured") { phase = "open"; pending = null; return idle; }
                if (receipt.kind === "accepted") { phase = "retired"; pending = retired; return { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle(retired) }; }
                phase = "complete"; pending = null; return idle;
              },
              bindHostRetirement: () => {},
              captureUiPatchAuthority: () => { throw new Error("fixture-native-ui-not-configured"); },
              submitUiAcknowledgement: async () => { throw new Error("fixture-native-ui-not-configured"); },
              dispose: () => dispose(actorId),
              progress: () => ({ kind: phase, failure: null }),
            };
          },
          captureActorActivation: (actorId: string) => {
            captures += 1;
            const activationGeneration = generation;
            return { actorId, activationGeneration, assertActive: () => { if (generation !== activationGeneration) throw new Error("actor-activation.revoked"); }, turn: (events: readonly ShardEventEnvelope[]) => { guardedTurns += 1; return dispatch(actorId, events); } };
          },
          dispose,
        } as unknown as ShardClient;
        globalThis.fetch = stubFetch(async () => new Response(JSON.stringify({ manifest: { pluginId: "extension-requester", apps: [] } }), { headers: { "content-type": "application/json" } }));
        let handle: PluginWasmHandle | undefined;
        try {
          handle = await loadPluginModule("extension-requester", "https://fixture.invalid/plugin.js");
          await run(handle, await handle.createApp("fixture"), { replace: () => { generation += 1n; }, captures: () => captures, guardedTurns: () => guardedTurns });
        } finally {
          await handle?.dispose();
          testState.sharedActivationRegistry = previous.registry;
          testState.sharedShardClient = previous.shard;
          globalThis.fetch = previous.fetch;
        }
      }
  
      it("settles the exact completion and returns frames and host effects", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        const bytes = (value: unknown) => Array.from(encodePackValue(value));
        let instance = 0;
        const submitted: ShardEventEnvelope[][] = [];
        await withRequester(async (_actor, events) => {
          submitted.push([...events]);
          if (submitted.length === 1) return { uiPatches: [], effects: [{ tag: "notify", val: { message: fixture.completion.notification } }], nextWake: null, status: { tag: "more-work" } };
          if (submitted.length === 2) return {
            uiPatches: [],
            effects: [{ tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } }],
            nextWake: null, status: { tag: "idle" },
          };
          return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        }, async (handle, opened) => {
          instance = opened;
          const req = BigInt(fixture.requestIds[2]!);
          const outcome = { ok: encodePackValue(fixture.response) };
          const response = await handle.captureExtensionCompletion!(instance, req).complete(outcome);
          expect(submitted).toEqual([[{ kind: "completed", payload: { req, outcome: { tag: "ok", val: Array.from(outcome.ok) } } }], []]);
          expect(response).toMatchObject({ output: fixture.response, requestedEffects: [{ notify: { message: fixture.completion.notification } }], uiScope: fixture.completion.uiScope, historyPatch: fixture.completion.historyPatch });
        });
      });
  
      // 💼️ Ticket 26/09/02/PUZZLE-3D-END-TO-END W-J. `Effect::SpawnJob` used to reach
      // `wireEffectToFriendly`'s `default:` arm and be dropped with a `[TRACE]` warning, and nothing
      // in this renderer ever called `ShardClient.startJob`/`stepJob` — so on this target EVERY
      // plugin-authored isolated job was started zero times and stepped zero times, silently. That is
      // what left puzzle 3d's fill planning frozen at zero in the browser.
      it("publishes isolated job completion without requiring another user command", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/🏁️job-completion-publication/🔣️.json");
                for (const row of fixture.cases) {
          let instance = 0;
          const submitted: ShardEventEnvelope[][] = [];
          const completions: unknown[] = [];
          const bytes = (value: unknown) => Array.from(encodePackValue(value));
          const history = { cursor: 2, upserts: [] };
          let terminal = false;
          let published = false;
          let remaining = row.continuations;
          await withRequester(async (_actor, events) => {
            submitted.push([...events]);
            terminal ||= events.some(event => event.kind === "job-completed");
            if (published) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
            if (terminal && remaining-- > 0) return { uiPatches: [], effects: [], nextWake: null, status: { tag: row.handoffStatus } };
            published = terminal;
            return {
              uiPatches: [], nextWake: null, status: { tag: "idle" },
              effects: terminal ? [
                { tag: "notify", val: { message: row.message } },
                { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ OperationCompleted: { operation: BigInt(row.operation), revision: BigInt(row.revision), ui_scope: bytes({ kind: "full" }), history_patch: bytes(history) } })) } },
              ] : [
                { tag: "spawn-job", val: { job: BigInt(row.job), kind: "semio.draw.mounted-vector", input: new Uint8Array([1]), placement: { tag: "isolated" } } },
                { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(null), diagnostics: bytes([]), ui_scope: bytes({ kind: "none" }), history_patch: [], messages: [], mutations: [], inverse_group: [] } })) } },
              ],
            };
          }, async (handle, opened) => {
            instance = opened;
            const unsubscribe = handle.subscribeOperationCompletions(instance, completion => completions.push(completion));
            try {
              await handle.captureExtensionCompletion!(instance, 1n).complete({ ok: encodePackValue(null) });
              for (let turn = 0; turn < 256 && completions.length === 0; turn += 1) await yieldPluginUiContinuation();
              if (row.continuations === 0) {
                await yieldPluginUiContinuation();
                expect(submitted, row.id).toHaveLength(2);
              }
              expect.soft(completions, row.id).toEqual([{ instanceId: instance, operation: BigInt(row.operation), revision: BigInt(row.revision), uiScope: { kind: "full" }, historyPatch: history, requestedEffects: [{ notify: { message: row.message } }], terminalOutput: undefined }]);
              expect.soft(submitted.length, row.id).toBeGreaterThanOrEqual(2 + row.continuations);
              expect(submitted.slice(1).flat().map(event => event.kind), row.id).toEqual(["job-completed"]);
            } finally { unsubscribe(); }
          }, { jobs: { step: () => ({ status: row.outcome as "done" | "failed", value: new Uint8Array([1]) }) } });
        }
        console.info("[DEBUG] Isolated job publication: four exact-authority fixtures complete without another input");
      });

      it("starts an isolated job, steps it to its terminal and feeds job-completed back into the actor", async () => {
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        const bytes = (value: unknown) => Array.from(encodePackValue(value));
        const started: Array<{ job: bigint; kind: string; input: readonly number[] }> = [];
        const stepped: bigint[] = [];
        const submitted: ShardEventEnvelope[][] = [];
        let instance = 0;
        await withRequester(async (_actor, events) => {
          submitted.push([...events]);
          if (submitted.length > 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
          return {
            uiPatches: [],
            effects: [
              { tag: "spawn-job", val: { job: 7n, kind: "semio.puzzle3d.fill", input: new Uint8Array([4, 2]), placement: { tag: "isolated" } } },
              { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } },
            ],
            nextWake: null,
            status: { tag: "idle" },
          };
        }, async (handle, opened) => {
          instance = opened;
          const response = await handle.captureExtensionCompletion!(instance, BigInt(fixture.requestIds[2]!)).complete({ ok: encodePackValue(fixture.response) });
          expect(response.requestedEffects).toEqual([]);
          for (let turn = 0; turn < 64 && !submitted.some(events => events.some(event => event.kind === "job-completed")); turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
          expect(started).toEqual([{ job: 7n, kind: "semio.puzzle3d.fill", input: [4, 2] }]);
          expect(stepped).toEqual([7n, 7n, 7n]);
          const completion = submitted.flat().find(event => event.kind === "job-completed");
          expect(completion?.payload).toEqual({ job: 7n, outcome: { tag: "ok", val: [1] } });
        }, {
          jobs: {
            start: (job, kind, input) => { started.push({ job, kind, input: Array.from(input) }); },
            step: (job) => { stepped.push(job); return stepped.length < 3 ? { status: "running" } : { status: "done", value: new Uint8Array([1]) }; },
          },
        });
      });

      // ⛽️ Ticket 26/09/02/PUZZLE-3D-END-TO-END W-F5. W-J's driver bounded a job's LIFETIME at a
      // constant `PLUGIN_JOB_STEP_LIMIT = 1 << 16` and cancelled whatever was still running there. A
      // `semio.puzzle3d.fill` plan of up to 1 000 placements reaches that in ~40 s of ticking, and the
      // cancel tore the guest's live envelope down mid-run (browser-measured 2026-09-10: `unreachable`
      // → `shard 0 lost` → `actor-activation.revoked`). The native host bounds the same jobs PER SLICE
      // and never over their lifetime — `ShardLoop::pump` grants one `job_budget_from_grant` per turn
      // and has no step counter at all — so this driver must agree: same budget every slice, terminal
      // only from the guest.
      it("bounds a job per slice and never ends one the guest is still running", async () => {
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        const bytes = (value: unknown) => Array.from(encodePackValue(value));
        const legacyHostLifetimeCap = 1 << 16;
        const slicesBeforeTerminal = legacyHostLifetimeCap + 1_024;
        const budgets = new Set<string>();
        const cancelled: bigint[] = [];
        const submitted: ShardEventEnvelope[][] = [];
        let stepped = 0;
        let instance = 0;
        await withRequester(async (_actor, events) => {
          submitted.push([...events]);
          if (submitted.length > 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
          return {
            uiPatches: [],
            effects: [
              { tag: "spawn-job", val: { job: 11n, kind: "semio.puzzle3d.fill", input: new Uint8Array([1]), placement: { tag: "isolated" } } },
              { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } },
            ],
            nextWake: null,
            status: { tag: "idle" },
          };
        }, async (handle, opened) => {
          instance = opened;
          await handle.captureExtensionCompletion!(instance, BigInt(fixture.requestIds[2]!)).complete({ ok: encodePackValue(fixture.response) });
          for (let turn = 0; turn < 4_096 && !submitted.some(events => events.some(event => event.kind === "job-completed")); turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
          expect(stepped).toBe(slicesBeforeTerminal);
          expect(cancelled).toEqual([]);
          expect([...budgets]).toEqual([`${DEFAULT_SHARD_BUDGET.fuel}/${DEFAULT_SHARD_BUDGET.wallMs}`]);
          const completion = submitted.flat().find(event => event.kind === "job-completed");
          expect(completion?.payload).toEqual({ job: 11n, outcome: { tag: "ok", val: [9] } });
        }, {
          jobs: {
            step: (_job, budget) => {
              stepped += 1;
              budgets.add(`${budget.fuel}/${budget.deadlineMs}`);
              return stepped < slicesBeforeTerminal ? { status: "running" } : { status: "done", value: new Uint8Array([9]) };
            },
            cancel: (job) => { cancelled.push(job); },
          },
        });
      });

      // 🧵️ Ticket 26/09/18 session 11 U5: the task manager lists what the runtime is stepping. A driven job is in
      // the ledger for exactly the lifetime of its drive, its row counts the admitted steps, and a human's cancel
      // reaches the guest's OWN `cancel-job` once — the guest then answers `job.unknown`, which the drive delivers
      // as the job's `job-completed` failure, and the row leaves with the drive.
      it("lists a driven job in the task-manager ledger and routes a cancel to the guest's own cancel-job", async () => {
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        const bytes = (value: unknown) => Array.from(encodePackValue(value));
        resetSpawnedJobsForTestsV1();
        const cancelled: bigint[] = [];
        const submitted: ShardEventEnvelope[][] = [];
        let instance = 0;
        let released = false;
        await withRequester(async (_actor, events) => {
          submitted.push([...events]);
          if (submitted.length > 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
          return {
            uiPatches: [],
            effects: [
              { tag: "spawn-job", val: { job: 13n, kind: "semio.puzzle3d.fill", input: new Uint8Array([2]), placement: { tag: "isolated" } } },
              { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } },
            ],
            nextWake: null,
            status: { tag: "idle" },
          };
        }, async (handle, opened) => {
          instance = opened;
          await handle.captureExtensionCompletion!(instance, BigInt(fixture.requestIds[2]!)).complete({ ok: encodePackValue(fixture.response) });
          for (let turn = 0; turn < 256 && (spawnedJobsSnapshotV1()[0]?.steps ?? 0) === 0; turn += 1) await new Promise(resolve => setTimeout(resolve, 5));
          const [row] = spawnedJobsSnapshotV1();
          expect(row).toMatchObject({ kind: "semio.puzzle3d.fill", job: 13n, cancelling: false });
          expect(row!.steps).toBeGreaterThan(0);
          expect(cancelSpawnedJobV1(row!.key)).toBe(true);
          expect(spawnedJobsSnapshotV1()[0]?.cancelling).toBe(true);
          for (let turn = 0; turn < 256 && cancelled.length === 0; turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
          expect(cancelled).toEqual([13n]);
          released = true;
          for (let turn = 0; turn < 1_024 && !submitted.some(events => events.some(event => event.kind === "job-completed")); turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
          const completion = submitted.flat().find(event => event.kind === "job-completed");
          expect(completion?.payload).toMatchObject({ job: 13n, outcome: { tag: "fault" } });
          for (let turn = 0; turn < 256 && spawnedJobsSnapshotV1().length > 0; turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
          expect(spawnedJobsSnapshotV1()).toEqual([]);
          expect(cancelSpawnedJobV1(row!.key)).toBe(false);
        }, {
          jobs: {
            step: () => (released ? { status: "failed", value: new TextEncoder().encode("job.unknown") } : { status: "running" }),
            cancel: (job) => { cancelled.push(job); },
          },
        });
      });

      it("reconciles failed actual actor bindings without giving a successor stale control", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧪️tests/🧫️fixtures/🔣️.json");
        const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts");
        for (const row of fixture.bindingRetirement) {
          const commands: string[] = [];
          let instance = 0, current = true;
          await withRequester(async (_actor, events) => {
            const effects: WireVariant[] = [];
            for (const event of events) {
              if (event.kind !== "message") continue;
              const value = event.payload as { source: { tag: string }; payload: number[] };
              if (value.source.tag !== "shell") continue;
              const command = decodeDocumentBackboneControlV1(Uint8Array.from(value.payload));
              commands.push(command.operation);
              const firstBind = command.bindingGeneration === 1n && command.operation === "bind";
              const refused = firstBind && row.result === "refused";
              if (firstBind && row.result === "transport") throw new Error("transport lost");
              if (firstBind && row.result === "replaced") current = false;
              const effect: WireVariant = { tag: "send-message", val: { target: { tag: "shell", val: instance }, payload: encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: refused ? "refused" : command.operation === "bind" ? "bound" : "retired", ...(refused ? { code: "binding-refused" } : {}) }) } };
              effects.push(effect);
              if (firstBind && row.result === "duplicate") effects.push(effect);
            }
            return { uiPatches: [], effects, nextWake: null, status: { tag: "idle" } };
          }, async (handle, opened) => {
            instance = opened;
            const source = { runtimeKey: "map", clientInstanceId: "client-a", scope: null, current: () => current, send: () => {} };
            await expect(handle.bindDocumentPort!(instance, source)).rejects.toThrow();
            expect(commands, row.result).toEqual(row.sequence);
            current = true;
            const successor = await handle.bindDocumentPort!(instance, { ...source, clientInstanceId: "client-b" });
            await successor.retire();
            expect(commands, row.result).toEqual([...row.sequence, "bind", "retire"]);
          });
        }
      });
  
      it("binds the actual actor document port and retires it before guest disposal", async () => {
        const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts");
        const { encodeBackboneMessage } = await import("@semio-tech/framework-os");
        const outgoing = encodeBackboneMessage({ kind: "ack", opIds: ["operation"] });
        let outgoingUri = "actor://map";
        const commands: string[] = [], delivered: number[][] = [], sent: number[][] = [];
        let instance = 0;
        await withRequester(async (_actor, events) => {
          const effects: WireVariant[] = [];
          for (const event of events) {
            if (event.kind !== "message") continue;
            const value = event.payload as { source: { tag: string; val: string }; payload: number[] };
            if (value.source.tag === "shell") {
              const command = decodeDocumentBackboneControlV1(Uint8Array.from(value.payload));
              commands.push(command.operation);
              effects.push({ tag: "send-message", val: { target: { tag: "shell", val: instance }, payload: encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" }) } });
            } else {
              delivered.push([...value.payload]);
              effects.push({ tag: "send-message", val: { target: { tag: "backbone", val: outgoingUri }, payload: outgoing } });
            }
          }
          return { uiPatches: [], effects, nextWake: null, status: { tag: "idle" } };
        }, async (handle, opened, activation) => {
          instance = opened;
          const source = { runtimeKey: "map", clientInstanceId: "client-a", scope: null };
          const port = await handle.bindDocumentPort!(instance, { ...source, current: () => true, send: payload => { sent.push([...payload]); } });
          expect(commands).toEqual(["bind"]);
          expect(await port.receive(source, outgoing)).toBe(true);
          expect(delivered).toEqual([[...outgoing]]);
          expect(sent).toEqual([[...outgoing]]);
          outgoingUri = "actor://foreign";
          await expect(port.receive(source, outgoing)).rejects.toThrow("actor-document-port.foreign-uri");
          expect(activation.guardedTurns()).toBeGreaterThanOrEqual(2);
          await handle.destroyApp(instance);
          expect(commands).toEqual(["bind", "retire"]);
          expect(await port.receive(source, outgoing)).toBe(false);
        });
      });
  
      it("awaits actual actor handle disposal until its exact document retirement settles", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧪️tests/🧫️fixtures/🔣️.json");
        const { default: deepEqual } = await import("fast-deep-equal");
        const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts");
        let release!: () => void, entered!: () => void;
        const gate = new Promise<void>(resolve => { release = resolve; });
        const retiring = new Promise<void>(resolve => { entered = resolve; });
        const events: string[] = [];
        await withRequester(async (_actor, input) => {
          const effects: WireVariant[] = [];
          for (const event of input) {
            if (event.kind !== "message") continue;
            const value = event.payload as { source: { tag: string }; payload: number[] };
            if (value.source.tag !== "shell") continue;
            const command = decodeDocumentBackboneControlV1(Uint8Array.from(value.payload));
            events.push(command.operation);
            if (command.operation === "retire") { entered(); await gate; events.push("retired"); }
            effects.push({ tag: "send-message", val: { target: { tag: "shell", val: command.instanceId }, payload: encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" }) } });
          }
          return { uiPatches: [], effects, nextWake: null, status: { tag: "idle" } };
        }, async (handle, instance) => {
          const port = await handle.bindDocumentPort!(instance, { runtimeKey: "map", clientInstanceId: "client", scope: null, current: () => true, send: () => {} });
          const closing = handle.dispose();
          try {
            expect(closing).toBeInstanceOf(Promise);
            expect(handle.dispose()).toBe(closing);
            let settled = false;
            void closing.then(() => { settled = true; });
            await retiring;
            expect(settled).toBe(false);
            expect(events).toEqual(fixture.disposal.bound.slice(0, 2));
            expect(await port.receive({ runtimeKey: "map", clientInstanceId: "client", scope: null }, Uint8Array.of(1))).toBe(false);
            await expect(handle.createApp("late")).rejects.toThrow("plugin-handle.closed");
          } finally { release(); await closing; }
          expect(deepEqual(events, fixture.disposal.bound)).toBe(true);
        }, { dispose: () => { events.push("dispose"); } });
      });

      it("awaits actual actor pending creation without opening an orphan after disposal", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧪️tests/🧫️fixtures/🔣️.json");
        const { default: deepEqual } = await import("fast-deep-equal");
        for (const row of fixture.disposal.opening) {
          let release!: () => void, entered!: () => void, target = "";
          const gate = new Promise<void>(resolve => { release = resolve; });
          const pending = new Promise<void>(resolve => { entered = resolve; });
          const events: string[] = [];
          await withRequester(async () => ({ uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }), async (handle, instance) => {
            const original = `extension-requester#${instance}`;
            target = "pending";
            const opening = handle.createApp("pending");
            const observed = opening.then(value => ({ value }), error => ({ error }));
            await pending;
            const closing = handle.dispose();
            try {
              expect(closing).toBeInstanceOf(Promise);
              let settled = false;
              void closing.then(() => { settled = true; }, () => {});
              await Promise.resolve();
              expect(settled).toBe(false);
              expect(target).not.toBe(original);
              expect(events).not.toContain("dispose");
            } finally { release(); await closing; }
            expect(await observed).toMatchObject({ error: { message: "plugin-handle.closed" } });
            expect(deepEqual(events, row.events), row.phase).toBe(true);
          }, {
            activate: async actor => {
              if (!target) return;
              target = actor; events.push("activate");
              if (row.phase === "activation") { entered(); await gate; }
              events.push("activated");
            },
            open: async actor => {
              if (actor !== target) return;
              events.push("open");
              if (row.phase === "open") { entered(); await gate; }
              events.push("opened");
            },
            dispose: actor => { if (actor === target) events.push("dispose"); },
          });
        }
      });

      it("rejects a framed completion fault without swallowing its structured fields", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        let instance = 0;
        await withRequester(async () => ({ uiPatches: [], effects: [{ tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Error: { in_reply_to: null, fault: Array.from(encodePackValue(fixture.fault)), report: [] } })) } }], nextWake: null, status: { tag: "idle" } }), async (handle, opened) => {
          instance = opened;
          await expect(handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) })).rejects.toMatchObject({ fault: fixture.fault });
        });
      });
  
      it("does not publish a late completion after its originating instance is destroyed", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        const entered = Promise.withResolvers<void>();
        const result = Promise.withResolvers<WireTurnResult>();
        let turns = 0;
        await withRequester(async () => { turns += 1; entered.resolve(); return result.promise; }, async (handle, instance) => {
          const completion = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
          const observed = expect(completion).rejects.toThrow("no actor");
          await entered.promise;
          const retiring = handle.destroyApp(instance);
          let retired = false;
          void retiring.then(() => { retired = true; });
          await Promise.resolve();
          expect(retired).toBe(false);
          result.resolve({ uiPatches: [], effects: [{ tag: "notify", val: { message: fixture.completion.notification } }], nextWake: null, status: { tag: "idle" } });
          await observed;
          await retiring;
          expect(turns).toBe(1);
          expect(retainedWindowByActor.has(`extension-requester#${instance}`)).toBe(false);
        });
      });
  
      it("captures activation before queued completion and rejects a same-name replacement before dispatch", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        let dispatched = 0;
        await withRequester(async () => { dispatched += 1; return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async (handle, instance, activation) => {
          const entered = Promise.withResolvers<void>();
          const release = Promise.withResolvers<void>();
          const held = serializeCommandIngressForActor(`extension-requester#${instance}`, async () => { entered.resolve(); await release.promise; });
          await entered.promise;
          const completing = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
          const observed = expect(completing).rejects.toThrow("actor-activation.revoked");
          const captures = activation.captures();
          activation.replace();
          release.resolve();
          await held;
          await observed;
          expect(captures).toBe(1);
          expect(activation.captures()).toBe(1);
          expect(dispatched).toBe(0);
        });
      });
  
      it("refuses publication after in-flight activation replacement without running a new continuation", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        const entered = Promise.withResolvers<void>();
        const release = Promise.withResolvers<WireTurnResult>();
        let dispatched = 0;
        await withRequester(async () => { dispatched += 1; entered.resolve(); return release.promise; }, async (handle, instance, activation) => {
          const completing = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
          const observed = expect(completing).rejects.toThrow("actor-activation.revoked");
          await entered.promise;
          activation.replace();
          release.resolve({ uiPatches: [], effects: [{ tag: "notify", val: { message: "stale activation" } }], nextWake: null, status: { tag: "idle" } });
          await observed;
          expect(activation.captures()).toBe(1);
          expect(activation.guardedTurns()).toBe(1);
          expect(dispatched).toBe(1);
          expect(retainedWindowByActor.get(`extension-requester#${instance}`)?.size ?? 0).toBe(0);
        });
      });
  
      it("keeps the original activation lease through every completion continuation", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        const entered = Promise.withResolvers<void>();
        const release = Promise.withResolvers<WireTurnResult>();
        let dispatched = 0;
        await withRequester(async () => {
          dispatched += 1;
          if (dispatched === 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
          entered.resolve();
          return release.promise;
        }, async (handle, instance, activation) => {
          const completing = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
          const observed = expect(completing).rejects.toThrow("actor-activation.revoked");
          await entered.promise;
          activation.replace();
          release.resolve({ uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } });
          await observed;
          expect(dispatched).toBe(2);
          expect(activation.captures()).toBe(1);
          expect(activation.guardedTurns()).toBe(2);
        });
      });
  
      it("rejects a completion captured before evaluation when that activation is later replaced", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        let dispatched = 0;
        await withRequester(async () => { dispatched += 1; return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async (handle, instance, activation) => {
          const completion = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId));
          activation.replace();
          await expect(completion.complete({ ok: encodePackValue(fixture.response) })).rejects.toThrow("actor-activation.revoked");
          expect(dispatched).toBe(0);
          expect(activation.captures()).toBe(1);
        });
      });
  
      it("claims one completion submission per captured request", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");
        let dispatched = 0;
        await withRequester(async () => { dispatched += 1; return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async (handle, instance) => {
          const completion = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId));
          expect(Object.isFrozen(completion)).toBe(true);
          expect([completion.instanceId, completion.req]).toEqual([instance, BigInt(fixture.requestId)]);
          await completion.complete({ ok: encodePackValue(fixture.response) });
          await expect(completion.complete({ ok: encodePackValue(fixture.response) })).rejects.toThrow("extension.completion-already-submitted");
          expect(dispatched).toBe(1);
        });
      });
    });

    describe("command-ingress fault diagnostics", () => {
      it("decodes the normalized scalar-wire fault payload instead of hiding the terminal cause", () => {
        const bytes = encodeFaultBytes("plugin.command-rejected");
        expect(commandIngressFaultDisplay({ tag: "fault", val: { cursor: {}, fault: { tag: "fault", val: Array.from(bytes) } } })).toBe("plugin.command-rejected: plugin.command-rejected");
      });
    });
}
