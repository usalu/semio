/** 🌋 Shard-loss wiring, descriptor fetch abort and level-parallel dependency boot. */
import type { PluginRegistryEntry } from "@semio-tech/framework";
import type { ActivationRegistry } from "@semio-tech/framework-kernel";
import type { ShardWorkerLike } from "@semio-tech/framework-actor/shard-client";
import type { PluginWasmHandle } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import { flushMicrotasks, createPluginRuntimeSupport, type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerShardLossAndBootTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it } = vitest;
  const { testState, buildShardClientOptions, fetchDescriptorManifest, handlePluginShardLost, forgetInstanceForRecovery, onPluginInstancesLost, PLUGIN_ACTOR_INSTANCE_LOST_FAULT, rememberInstanceForRecovery, loadPluginModulesInDependencyOrder, poolConcurrency } = dependencies;
  const { fakeHandle } = createPluginRuntimeSupport(dependencies);
    describe("PluginRuntime shard-loss wiring (real restore, not just a console.error)", () => {
      it("handlePluginShardLost delegates to ActivationRegistry.handleShardLost for EXACTLY the affected actorIds", () => {
        const restoreCalls: Array<{ readonly shardIndex: number; readonly actorIds: readonly string[] }> = [];
        const fakeRegistry = { handleShardLost: (shardIndex: number, actorIds: readonly string[]) => restoreCalls.push({ shardIndex, actorIds }) } as unknown as ActivationRegistry;
        const previous = testState.sharedActivationRegistry;
        testState.sharedActivationRegistry = fakeRegistry;
        try {
          handlePluginShardLost(2, ["plugin-a#1", "plugin-b#7"]);
          expect(restoreCalls).toEqual([{ shardIndex: 2, actorIds: ["plugin-a#1", "plugin-b#7"] }]);
        } finally {
          testState.sharedActivationRegistry = previous;
        }
      });
  
      /** ⚖️ LAW: restoring the ACTOR is not restoring the APP. A watchdog kill takes no checkpoint, so
       * the guest comes back with no `createApp` instance at all and every later command page fails
       * the guest's own liveness check (`plugin.command-page-invalid`) — the session is dead until the
       * user reloads. The runtime must therefore retire each lost instance's bookkeeping AND announce
       * it, so whoever created the instance can create it again
       * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`). */
      it("handlePluginShardLost retires the app instances those actors hosted and announces exactly them", () => {
        const released: string[] = [];
        const announced: Array<{ readonly shardIndex: number; readonly lost: readonly { readonly pluginId: string; readonly instanceId: number; readonly actorId: string }[] }> = [];
        const previous = testState.sharedActivationRegistry;
        testState.sharedActivationRegistry = { handleShardLost: () => {} } as unknown as ActivationRegistry;
        const unsubscribe = onPluginInstancesLost((shardIndex, lost) => announced.push({ shardIndex, lost }));
        rememberInstanceForRecovery({ pluginId: "procedural", instanceId: 1, actorId: "procedural#1", release: () => released.push("procedural#1") });
        rememberInstanceForRecovery({ pluginId: "other", instanceId: 4, actorId: "other#4", release: () => released.push("other#4") });
        try {
          handlePluginShardLost(0, ["procedural#1", "flow-extension-brep#request"]);
          expect(released).toEqual(["procedural#1"]);
          expect(announced).toHaveLength(1);
          expect(announced[0]?.shardIndex).toBe(0);
          expect(announced[0]?.lost).toEqual([{ pluginId: "procedural", instanceId: 1, actorId: "procedural#1" }]);
          handlePluginShardLost(0, ["procedural#1"]);
          expect(released).toEqual(["procedural#1"]);
          expect(announced).toHaveLength(1);
          forgetInstanceForRecovery("other#4");
          handlePluginShardLost(1, ["other#4"]);
          expect(released).toEqual(["procedural#1"]);
          expect(announced).toHaveLength(1);
        } finally {
          unsubscribe();
          forgetInstanceForRecovery("procedural#1");
          forgetInstanceForRecovery("other#4");
          testState.sharedActivationRegistry = previous;
        }
      });

      /** ⚖️ LAW: a shard loss with NO live app instance on it announces nothing at all — an extension
       * request actor dying is a retryable request failure, not a lost session. */
      it("a lost shard that hosted no app instance announces nothing", () => {
        const announced: unknown[] = [];
        const previous = testState.sharedActivationRegistry;
        testState.sharedActivationRegistry = { handleShardLost: () => {} } as unknown as ActivationRegistry;
        const unsubscribe = onPluginInstancesLost((_shardIndex, lost) => announced.push(lost));
        try {
          handlePluginShardLost(3, ["flow-extension-brep#request"]);
          expect(announced).toEqual([]);
          expect(PLUGIN_ACTOR_INSTANCE_LOST_FAULT).toBe("plugin.actor-instance-lost");
        } finally {
          unsubscribe();
          testState.sharedActivationRegistry = previous;
        }
      });

      it("buildShardClientOptions wires onShardLost to handlePluginShardLost (not a bare console.error) and sizes shardCount via poolConcurrency", () => {
        const fakeCreateWorker = () => ({ postMessage: () => {}, terminate: () => {}, onmessage: null, onerror: null }) as unknown as ShardWorkerLike;
        const options = buildShardClientOptions(fakeCreateWorker);
        expect(options.onShardLost).toBe(handlePluginShardLost);
        expect(options.shardCount).toBe(poolConcurrency());
        expect(options.createWorker).toBe(fakeCreateWorker);
      });
    });

    describe("fetchDescriptorManifest AbortSignal", () => {
      it("propagates an aborted signal's fetch rejection instead of silently falling back to an empty manifest", async () => {
        const controller = new AbortController();
        controller.abort();
        const originalFetch = globalThis.fetch;
        globalThis.fetch = stubFetch(async () => {
          throw new DOMException("aborted", "AbortError");
        });
        try {
          await expect(fetchDescriptorManifest("p", "https://x/p.js", controller.signal)).rejects.toThrow();
        } finally {
          globalThis.fetch = originalFetch;
        }
      });
  
      it("propagates a network failure without manufacturing an empty descriptor", async () => {
        const originalFetch = globalThis.fetch;
        const failure = new Error("network down");
        globalThis.fetch = stubFetch(async () => {
          throw failure;
        });
        try {
          await expect(fetchDescriptorManifest("p", "https://x/p.js")).rejects.toBe(failure);
        } finally {
          globalThis.fetch = originalFetch;
        }
      });
  
      it("rejects the dev server's HTML SPA fallback without a parse warning", async () => {
        const originalFetch = globalThis.fetch;
        const originalWarn = console.warn;
        let warningCount = 0;
        console.warn = () => {
          warningCount += 1;
        };
        globalThis.fetch = stubFetch(async () => new Response("<!doctype html>", { status: 200, headers: { "content-type": "text/html" } }));
        try {
          await expect(fetchDescriptorManifest("p", "https://x/p.js")).rejects.toThrow("plugin.descriptor-invalid");
          expect(warningCount).toBe(0);
        } finally {
          console.warn = originalWarn;
          globalThis.fetch = originalFetch;
        }
      });
    });

    describe("loadPluginModulesInDependencyOrder — level-parallel boot", () => {
      it("runs independent siblings in parallel within a level, holding a dependent until its dependency's WHOLE level finishes", async () => {
        const started: string[] = [];
        const releaseFns = new Map<string, () => void>();
        const loadModule = (pluginId: string): Promise<PluginWasmHandle> =>
          new Promise<PluginWasmHandle>((resolve) => {
            started.push(pluginId);
            releaseFns.set(pluginId, () => resolve(fakeHandle(pluginId, [], [])));
          });
        const entries: PluginRegistryEntry[] = [
          { pluginId: "a", moduleUrl: "https://x/a.js", dependencies: [] },
          { pluginId: "b", moduleUrl: "https://x/b.js", dependencies: [] },
          { pluginId: "c", moduleUrl: "https://x/c.js", dependencies: [{ pluginId: "a" }] },
        ];
        const resultPromise = loadPluginModulesInDependencyOrder(entries, { loadModule, concurrency: 4 });
        await flushMicrotasks();
        expect(new Set(started)).toEqual(new Set(["a", "b"]));
        expect(started).not.toContain("c");
        releaseFns.get("a")!();
        releaseFns.get("b")!();
        await flushMicrotasks();
        expect(started).toContain("c");
        releaseFns.get("c")!();
        const result = await resultPromise;
        expect(result.handles.map((handle) => handle.pluginId)).toEqual(["a", "b", "c"]);
        expect(result.errors).toEqual([]);
        expect(result.loadFailures).toEqual([]);
      });
  
      it("bounds within-level concurrency to the given limit — a third independent sibling waits for a free slot", async () => {
        const started: string[] = [];
        const releaseFns: Array<() => void> = [];
        const loadModule = (pluginId: string): Promise<PluginWasmHandle> =>
          new Promise<PluginWasmHandle>((resolve) => {
            started.push(pluginId);
            releaseFns.push(() => resolve(fakeHandle(pluginId, [], [])));
          });
        const entries: PluginRegistryEntry[] = ["a", "b", "c"].map((id) => ({ pluginId: id, moduleUrl: `https://x/${id}.js`, dependencies: [] }));
        const resultPromise = loadPluginModulesInDependencyOrder(entries, { loadModule, concurrency: 2 });
        await flushMicrotasks();
        expect(started).toHaveLength(2);
        releaseFns[0]!();
        await flushMicrotasks();
        expect(started).toHaveLength(3);
        releaseFns[1]!();
        releaseFns[2]!();
        await resultPromise;
      });
  
      it("cascades a runtime load failure to skip dependents, while unrelated siblings still load — a distinct PluginLoadFailure, not a PluginGraphError", async () => {
        const loadModule = (pluginId: string): Promise<PluginWasmHandle> => (pluginId === "a" ? Promise.reject(new Error("boom")) : Promise.resolve(fakeHandle(pluginId, [], [])));
        const entries: PluginRegistryEntry[] = [
          { pluginId: "a", moduleUrl: "https://x/a.js", dependencies: [] },
          { pluginId: "b", moduleUrl: "https://x/b.js", dependencies: [] },
          { pluginId: "c", moduleUrl: "https://x/c.js", dependencies: [{ pluginId: "a" }] },
        ];
        const result = await loadPluginModulesInDependencyOrder(entries, { loadModule, concurrency: 4 });
        expect(result.handles.map((handle) => handle.pluginId)).toEqual(["b"]);
        expect(result.errors).toEqual([]);
        expect(result.loadFailures.map((failure) => failure.pluginId).sort()).toEqual(["a", "c"]);
      });
  
      it("defaults its concurrency bound to poolConcurrency() when the caller doesn't override it", async () => {
        const cap = poolConcurrency();
        let inFlight = 0;
        let maxInFlight = 0;
        const loadModule = (pluginId: string): Promise<PluginWasmHandle> =>
          new Promise<PluginWasmHandle>((resolve) => {
            inFlight += 1;
            maxInFlight = Math.max(maxInFlight, inFlight);
            queueMicrotask(() => {
              inFlight -= 1;
              resolve(fakeHandle(pluginId, [], []));
            });
          });
        const entries: PluginRegistryEntry[] = Array.from({ length: cap + 6 }, (_, index) => ({ pluginId: `p${index}`, moduleUrl: `https://x/p${index}.js`, dependencies: [] }));
        await loadPluginModulesInDependencyOrder(entries, { loadModule });
        expect(maxInFlight).toBeLessThanOrEqual(cap);
        expect(maxInFlight).toBeGreaterThan(0);
      });
  
      it("aborts cleanly: aborting mid-boot stops starting new loads without throwing, while an already-started load still settles normally", async () => {
        const controller = new AbortController();
        const started: string[] = [];
        const loadModule = (pluginId: string): Promise<PluginWasmHandle> => {
          started.push(pluginId);
          if (pluginId === "a") controller.abort();
          return Promise.resolve(fakeHandle(pluginId, [], []));
        };
        const entries: PluginRegistryEntry[] = [
          { pluginId: "a", moduleUrl: "https://x/a.js", dependencies: [] },
          { pluginId: "b", moduleUrl: "https://x/b.js", dependencies: [{ pluginId: "a" }] },
        ];
        const result = await loadPluginModulesInDependencyOrder(entries, { loadModule, signal: controller.signal });
        expect(started).toEqual(["a"]);
        expect(result.handles.map((handle) => handle.pluginId)).toEqual(["a"]);
        expect(result.loadFailures.some((failure) => failure.pluginId === "b")).toBe(true);
      });
    });
}
