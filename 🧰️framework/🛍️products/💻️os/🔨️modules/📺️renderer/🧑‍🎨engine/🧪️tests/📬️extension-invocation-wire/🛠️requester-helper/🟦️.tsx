/** 🛠️ The completion publication requester harness shared by the completion publication parts. */
import type { ActivationRegistry } from "@semio-tech/framework-kernel";
import type { ShardClient } from "@semio-tech/framework-actor/shard-client";
import type { ShardEventEnvelope, ShardJobStep } from "@semio-tech/framework-actor/shard-client";
import type { PluginWasmHandle, WireTurnResult } from "../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { stubFetch } from "../../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import type { InvocationWireTestKit } from "../🧰️invocation-wire-test-kit/🟦️.tsx";

/** 🔧 Creates the requester harness shared by the completion publication parts. */
export function createCompletionPublicationHelpers(kit: InvocationWireTestKit) {
  const { loadPluginModule, testState } = kit;
    async function withRequester(turn: (actor: string, events: readonly ShardEventEnvelope[]) => Promise<WireTurnResult>, run: (handle: PluginWasmHandle, instance: number, activation: { replace(): void; captures(): number; guardedTurns(): number }) => Promise<void>, hooks: { activate?: (actor: string) => Promise<void>; open?: (actor: string) => Promise<void>; dispose?: (actor: string) => void; jobs?: { start?: (job: bigint, kind: string, input: Uint8Array) => void; step?: (job: bigint, budget: { fuel: bigint; deadlineMs: number }) => ShardJobStep; cancel?: (job: bigint) => void } } = {}): Promise<void> {
      const { encodeActorInstanceLifecycle } = await import("../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts");
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
          let pending: import("../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts").ActorInstanceLifecycleReceipt | null = null;
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
  return { withRequester };
}
export type CompletionPublicationHelpers = ReturnType<typeof createCompletionPublicationHelpers>;
