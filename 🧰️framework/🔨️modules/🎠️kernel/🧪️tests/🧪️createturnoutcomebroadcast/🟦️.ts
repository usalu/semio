type TestSource = { readonly directory: string; readonly url: string };
type KernelTestModule = typeof import("../../🟦️.ts");
type IoSchemaTestModule = typeof import("../../../🚪️io/🧬️schema/🟦️.ts");
type ActorTestModule = typeof import("../../../🎭️actor/📮️shard-client/🟦️.ts");
type ResidentTestModule = typeof import("../../../🌱️value/💾️resident/🟦️.ts");

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<KernelTestModule, "createTurnOutcomeBroadcast">, source: TestSource): Promise<void> {
  const { createTurnOutcomeBroadcast } = dependencies;
  type TurnOutcome = import("../../🟦️.ts").TurnOutcome;

  const { describe, expect, it } = vitest;

  describe("createTurnOutcomeBroadcast", () => {
    it("multicasts one pushed value to EVERY live subscriber, not a shared drain-once FIFO", async () => {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const iteratorA = broadcast.stream[Symbol.asyncIterator]();
      const iteratorB = broadcast.stream[Symbol.asyncIterator]();
      broadcast.push({ instanceId: 1, frames: [] });
      const [stepA, stepB] = await Promise.all([iteratorA.next(), iteratorB.next()]);
      expect(stepA).toEqual({ value: { instanceId: 1, frames: [] }, done: false });
      expect(stepB).toEqual({ value: { instanceId: 1, frames: [] }, done: false });
    });

    it("queues a value pushed before next() is called, and delivers queued values in push order", async () => {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const iterator = broadcast.stream[Symbol.asyncIterator]();
      broadcast.push({ instanceId: 2, frames: [] });
      broadcast.push({ instanceId: 3, frames: [] });
      expect(await iterator.next()).toEqual({ value: { instanceId: 2, frames: [] }, done: false });
      expect(await iterator.next()).toEqual({ value: { instanceId: 3, frames: [] }, done: false });
    });

    it("return() unsubscribes immediately — a later push never reaches a next() called after it", async () => {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const iterator = broadcast.stream[Symbol.asyncIterator]();
      expect(await iterator.return?.()).toEqual({ value: undefined, done: true });
      const pending = iterator.next();
      broadcast.push({ instanceId: 4, frames: [] });
      // 🎯️ an unsubscribed iterator's next() must NOT resolve from this push — racing it against an
      // already-resolved promise proves it is still pending, not that it settled "not yet" by luck.
      const raceResult = await Promise.race([pending.then(() => "resolved" as const), Promise.resolve("not-yet" as const)]);
      expect(raceResult).toBe("not-yet");
    });

    it("complete() force-closes every still-live subscriber at once", async () => {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const iteratorA = broadcast.stream[Symbol.asyncIterator]();
      const iteratorB = broadcast.stream[Symbol.asyncIterator]();
      const pendingA = iteratorA.next();
      const pendingB = iteratorB.next();
      broadcast.complete();
      expect(await pendingA).toEqual({ value: undefined, done: true });
      expect(await pendingB).toEqual({ value: undefined, done: true });
    });
  });

}

export async function registerTests2(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<KernelTestModule, "AppRouter"> & Pick<IoSchemaTestModule, "dialectCoordinate">, source: TestSource): Promise<void> {
  const { AppRouter, dialectCoordinate } = dependencies;
  type AppRef = import("../../../🛂️manifest/🧬️schema/🟦️.ts").AppRef;
  type AppRole = import("../../../🛂️manifest/🧬️schema/🟦️.ts").AppRole;
  type AppRouterManifest = import("../../🟦️.ts").AppRouterManifest;
  type ArtifactDialect = import("../../../🚪️io/🧬️schema/🟦️.ts").ArtifactDialect;

  const { describe, expect, it } = vitest;

  describe("AppRouter", () => {
    it("orders a loaded aggregate plugin after the foreign surface owner it depends on", () => {
      const cadDialect = { artifactKind: "s.cad.cad", standard: "1", subset: "*" };
      const aggregate: AppRouterManifest = {
        pluginId: "demonstrator",
        apps: [{ id: "s.cad.cad@1/*#editor", role: "editor", dialect: cadDialect }],
        dependencies: [{ pluginId: "cad", version: "*" }],
      };
      const owner: AppRouterManifest = {
        pluginId: "cad",
        apps: [{ id: "s.cad.cad@1/*#editor", role: "editor", dialect: cadDialect }],
      };
      const router = AppRouter.build([aggregate, owner]);
      expect(router.ownerPluginId("s.cad.cad")).toBe("cad");
      expect(router.entriesFor(cadDialect, "editor")).toEqual([
        { pluginId: "cad", appId: "s.cad.cad@1/*#editor" },
        { pluginId: "demonstrator", appId: "s.cad.cad@1/*#editor" },
      ]);
      expect(router.pluginFaults()).toEqual([]);
    });

    it("isolates a breaching plugin per the shared fixture instead of failing every route", async () => {
      type Fixture = Readonly<{
        manifests: readonly AppRouterManifest[];
        expectedOwners: readonly { readonly artifactKind: string; readonly pluginId: string }[];
        expectedRoutes: readonly { readonly dialect: ArtifactDialect; readonly role: AppRole; readonly entries: readonly AppRef[] }[];
        expectedFaults: readonly { readonly pluginId: string; readonly code: string; readonly origin: string; readonly severity: string }[];
      }>;
      const { readFile } = await import("node:fs/promises");
      const { dirname, join } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const fixture = JSON.parse(await readFile(join(dirname(fileURLToPath(source.url)), "🧫️fixtures/🧫️app-router-plugin-faults/🔣️.json"), "utf8")) as Fixture;
      const router = AppRouter.build(fixture.manifests);
      for (const owner of fixture.expectedOwners) expect(router.ownerPluginId(owner.artifactKind), owner.artifactKind).toBe(owner.pluginId);
      for (const route of fixture.expectedRoutes) expect(router.entriesFor(route.dialect, route.role), `${dialectCoordinate(route.dialect)}#${route.role}`).toEqual(route.entries);
      expect(router.pluginFaults().map((fault) => ({ pluginId: fault.scope.pluginId, code: fault.code, origin: fault.origin, severity: fault.severity }))).toEqual(
        fixture.expectedFaults.map((fault) => ({ pluginId: fault.pluginId, code: fault.code, origin: fault.origin, severity: fault.severity })),
      );
      for (const fault of fixture.expectedFaults) expect(router.faultFor(fault.pluginId)?.code, fault.pluginId).toBe(fault.code);
      expect(router.faultFor("cad")).toBeUndefined();
    });
  });

}

export async function registerTests3(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<KernelTestModule, "ActivationRegistry" | "DEFAULT_MAX_RESIDENT_ACTORS" | "RUNTIME_METRICS_PUBLISH_INTERVAL_MS" | "intersectCapabilityGrants" | "residentActorCapFromMemory" | "runtimeMetricsDue"> & Pick<ActorTestModule, "ShardClient"> & Pick<ResidentTestModule, "OwnedResidentLedger">, source: TestSource): Promise<void> {
  const { ActivationRegistry, DEFAULT_MAX_RESIDENT_ACTORS, OwnedResidentLedger, RUNTIME_METRICS_PUBLISH_INTERVAL_MS, ShardClient, intersectCapabilityGrants, residentActorCapFromMemory, runtimeMetricsDue } = dependencies;
  type MemoryProbe = import("../../🟦️.ts").MemoryProbe;
  type PluginCatalog = import("../../🟦️.ts").PluginCatalog;
  type RuntimeMetricsSnapshot = import("../../🟦️.ts").RuntimeMetricsSnapshot;
  type ActivationRegistry = import("../../🟦️.ts").ActivationRegistry;
  type ShardBudget = import("../../../🎭️actor/📮️shard-client/🟦️.ts").ShardBudget;
  type ShardCapabilityGrant = import("../../../🎭️actor/📮️shard-client/🟦️.ts").ShardCapabilityGrant;
  type ShardClient = import("../../../🎭️actor/📮️shard-client/🟦️.ts").ShardClient;
  type OwnedResidentLedger = import("../../../🌱️value/💾️resident/🟦️.ts").OwnedResidentLedger;

  const { describe, expect, it, vi } = vitest;

  const BUDGET_FIXTURE: ShardBudget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };

  /** 🧪️ A `ShardWorkerLike` that immediately auto-replies success to every request-bearing message —
   * enough for `ActivationRegistry.activate`/`suspend` to resolve without hand-delivering replies
   * (unlike `shard-client.ts`'s own `FakeShardWorker`, which is deliberately manual for THAT file's
   * out-of-order-reply tests; this region only needs "the round trip completes"). Factored out (web-
   * activation) so tests that need their own `ShardClient` construction (custom `onShardLost`/
   * `exclusiveShardCount`, not just the one-liner {@link fakeShardClient} covers) can reuse it too. */
  function createAutoReplyWorker(): { postMessage: (message: unknown) => void; terminate: () => void; onmessage: ((event: { readonly data: unknown }) => void) | null; onerror: ((event: unknown) => void) | null } {
    const worker: { postMessage: (message: unknown) => void; terminate: () => void; onmessage: ((event: { readonly data: unknown }) => void) | null; onerror: ((event: unknown) => void) | null } = {
      postMessage: (message) => {
        const requestId = (message as { readonly requestId?: string }).requestId;
        if (requestId) queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value: undefined } }));
      },
      terminate: () => {},
      onmessage: null,
      onerror: null,
    };
    return worker;
  }

  function fakeShardClient(shardCount = 1): ShardClient {
    return new ShardClient({ residentLedger: fixtureResidentLedger(), shardCount, createWorker: () => createAutoReplyWorker() });
  }

  function fixtureResidentLedger(): OwnedResidentLedger { return new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }); }

  /** 🧪️ Advances `n` real microtask ticks with no real timer/sleep involved — enough hops for a
   * `TurnScheduler` pump + a fake-worker's `queueMicrotask` reply + this registry's own
   * `runQueuedTurn` await chain to settle deterministically. */
  async function flushMicrotasks(n = 10): Promise<void> {
    for (let i = 0; i < n; i += 1) await Promise.resolve();
  }

  describe("ActivationRegistry.runtimeMetricsActorRows / runtimeMetricsSnapshot", () => {
    it("rows cover both resident and suspended actors, never activated-and-forgotten ones", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, now: () => 500, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");

      const rows = registry.runtimeMetricsActorRows();
      expect(rows).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 }]);

      await registry.suspend("actor-1");
      const afterSuspend = registry.runtimeMetricsActorRows();
      expect(afterSuspend).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: false, shard: null }]);
    });

    it("snapshot combines actor rows with ShardClient.shardMetricsSamples at the given clock reading", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, now: () => 999, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");

      const snapshot = registry.runtimeMetricsSnapshot(1_000);
      expect(snapshot.sampledAtMs).toBe(1_000);
      expect(snapshot.actors).toHaveLength(1);
      expect(snapshot.shards).toEqual(shardClient.shardMetricsSamples(1_000));
    });
  });

  describe("runtimeMetricsDue", () => {
    it("gates at the 500ms / 2Hz interval, always due on the first call", () => {
      expect(runtimeMetricsDue(null, 0)).toBe(true);
      expect(runtimeMetricsDue(1_000, 1_200)).toBe(false);
      expect(runtimeMetricsDue(1_000, 1_500)).toBe(true);
    });
  });

  describe("ActivationRegistry.startRuntimeMetricsPublisher", () => {
    it("calls the sink with the os.runtime.metrics topic at the 2Hz interval, and stop() cancels it", () => {
      vi.useFakeTimers();
      try {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        const calls: Array<{ readonly topic: string; readonly snapshot: RuntimeMetricsSnapshot }> = [];
        const stop = registry.startRuntimeMetricsPublisher((topic, snapshot) => calls.push({ topic, snapshot }));

        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(calls).toHaveLength(1);
        expect(calls[0]!.topic).toBe("os.runtime.metrics");

        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(calls).toHaveLength(2);

        stop();
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 3);
        expect(calls).toHaveLength(2);
      } finally {
        vi.useRealTimers();
      }
    });
  });

  describe("ActivationRegistry.cancel", () => {
    it("disposes the worker-side instance and forgets the actor entirely — resume() afterward throws unknown actor", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      expect(registry.isResident("actor-1")).toBe(true);

      registry.cancel("actor-1");

      expect(registry.isResident("actor-1")).toBe(false);
      expect(registry.runtimeMetricsActorRows()).toEqual([]);
      expect(shardClient.shardIndexFor("actor-1")).toBeUndefined(); // dispose() cleared the routing entry
      await expect(registry.resume("actor-1")).rejects.toThrow(/unknown actor/);
    });

    it("is a no-op for an actor this registry never activated", () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      expect(() => registry.cancel("ghost")).not.toThrow();
    });

    it("cancelling a suspended (non-resident but still tracked) actor still forgets it", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      await registry.suspend("actor-1");
      expect(registry.runtimeMetricsActorRows()).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: false, shard: null }]);

      registry.cancel("actor-1");
      expect(registry.runtimeMetricsActorRows()).toEqual([]);
    });
  });

  //#region 🧪️ExtensionCascadeTests
  /** 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): a minimal `PluginCatalog`
   * with ONE plugin (`p1`) and ONE extension (`p1-ext`) whose `dependsOn: ["p1"]` names it as the
   * parent — exactly the shape `registerCatalog`'s own `extensionsByParent` index groups by. */
  function catalogWithOneExtension(): PluginCatalog {
    return {
      plugins: [{ pluginId: "p1", wasmOut: "p1.wasm", role: "plugin", contributes: [], consumes: [] }],
      extensions: [{ pluginId: "p1-ext", wasmOut: "p1-ext.wasm", role: "extension", contributes: [], consumes: [], dependsOn: ["p1"] }],
      hosts: [],
      playgrounds: [],
      moduleUrl: (pluginId) => `https://x/${pluginId}/🌉️bridge.js`,
      extensionModuleUrl: (pluginId) => `https://x/ext/${pluginId}/🌉️bridge.js`,
    };
  }

  describe("intersectCapabilityGrants", () => {
    it("keeps only requested grants the parent's own granted set also carries, matched by id", () => {
      const grant = (id: string): ShardCapabilityGrant => ({ id, token: "t", scope: "s", expiresMs: null });
      const granted = [grant("fs.read"), grant("net.fetch")];
      const requested = [grant("fs.read"), grant("fs.admin")];
      expect(intersectCapabilityGrants(granted, requested).map((g) => g.id)).toEqual(["fs.read"]);
    });

    it("is empty when the parent holds nothing, never escalates an ungranted request", () => {
      const grant = (id: string): ShardCapabilityGrant => ({ id, token: "t", scope: "s", expiresMs: null });
      expect(intersectCapabilityGrants([], [grant("fs.admin")])).toEqual([]);
    });
  });

  describe("ActivationRegistry extension cascade (registerCatalog)", () => {
    it("activate() cascades to every registered extension of the plugin, under a deterministic child actorId", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());

      await registry.activate("p1", "actor-1", "manual");

      expect(registry.isResident("actor-1")).toBe(true);
      expect(registry.isResident("actor-1::p1-ext")).toBe(true);
      const rows = registry.runtimeMetricsActorRows();
      expect(rows).toContainEqual({ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 });
      expect(rows).toContainEqual({ actorId: "actor-1::p1-ext", pluginId: "p1-ext", resident: true, shard: 0 });
    });

    it("a plugin with no registered extensions activates with no cascade side effects", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });

      await registry.activate("p1", "actor-1", "manual");

      expect(registry.runtimeMetricsActorRows()).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 }]);
    });

    it("suspend() cascades leaves-first, resume() cascades parent-first — zero orphans either way", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());
      await registry.activate("p1", "actor-1", "manual");

      await registry.suspend("actor-1");
      expect(registry.isResident("actor-1")).toBe(false);
      expect(registry.isResident("actor-1::p1-ext")).toBe(false);
      // still tracked (suspended, not cancelled) — resume must find both again.
      expect(registry.runtimeMetricsActorRows().map((r) => r.actorId).sort()).toEqual(["actor-1", "actor-1::p1-ext"]);

      await registry.resume("actor-1");
      expect(registry.isResident("actor-1")).toBe(true);
      expect(registry.isResident("actor-1::p1-ext")).toBe(true);
    });

    it("cancel() on the parent takes its extension down too — permanently, zero orphans", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());
      await registry.activate("p1", "actor-1", "manual");

      registry.cancel("actor-1");

      expect(registry.runtimeMetricsActorRows()).toEqual([]);
      await expect(registry.resume("actor-1")).rejects.toThrow(/unknown actor/);
      await expect(registry.resume("actor-1::p1-ext")).rejects.toThrow(/unknown actor/);
    });

    it("scopes an extension's activated caps to the intersection with its parent's own granted set", async () => {
      const shardClient = fakeShardClient();
      const sentCaps = new Map<string, readonly ShardCapabilityGrant[]>();
      const worker = createAutoReplyWorker();
      const originalPostMessage = worker.postMessage;
      worker.postMessage = (message) => {
        const msg = message as { readonly kind?: string; readonly actorId?: string; readonly caps?: readonly ShardCapabilityGrant[] };
        if (msg.kind === "activate" && msg.actorId) sentCaps.set(msg.actorId, msg.caps ?? []);
        originalPostMessage(message);
      };
      const client = new ShardClient({ residentLedger: fixtureResidentLedger(), shardCount: 1, createWorker: () => worker });
      const registry = new ActivationRegistry({ shardClient: client, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());
      const grant = (id: string): ShardCapabilityGrant => ({ id, token: "t", scope: "s", expiresMs: null });
      // Override the parent's manifest (registerCatalog seeds `caps: []`) so there is something real
      // to intersect against, and the extension's own manifest with a request that only PARTIALLY
      // overlaps — `fs.read` must survive, `fs.admin` must not, the parent never held it.
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1/p1.wasm", caps: [grant("fs.read")] });
      registry.registerManifest({ pluginId: "p1-ext", moduleUrl: "https://x/ext/p1-ext/p1-ext.wasm", caps: [grant("fs.read"), grant("fs.admin")] });

      await registry.activate("p1", "actor-1", "manual");

      expect(sentCaps.get("actor-1::p1-ext")?.map((g) => g.id)).toEqual(["fs.read"]);
    });
  });
  //#endregion 🧪️ExtensionCascadeTests

  //#region 🧪️TurnDispatchTests
  describe("ActivationRegistry.enqueueTurn lane priority", () => {
    it("dispatches turns by lane priority end-to-end through the registry, not enqueue order", async () => {
      const shardClient = fakeShardClient();
      const order: string[] = [];
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: (actorId) => order.push(actorId) });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "low", "manual");
      await registry.activate("p1", "high", "manual");
      await registry.activate("p1", "mid", "manual");

      registry.enqueueTurn("low", "Background", []);
      registry.enqueueTurn("high", "Interactive", []);
      registry.enqueueTurn("mid", "UserVisible", []);

      await flushMicrotasks();
      expect(order).toEqual(["high", "mid", "low"]);
    });
  });

  describe("ActivationRegistry.suspend cancels queued turns", () => {
    it("a suspended actor's queued turns are cancelled, never delivered", async () => {
      const shardClient = fakeShardClient();
      const delivered: string[] = [];
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: () => delivered.push("delivered") });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");

      registry.enqueueTurn("actor-1", "Interactive", []); // queued but not yet dispatched
      await registry.suspend("actor-1"); // cancels it synchronously, before checkpoint/dispose even starts

      await flushMicrotasks();
      expect(delivered).toEqual([]);

      await registry.resume("actor-1");
      await flushMicrotasks();
      expect(delivered).toEqual([]); // still nothing — the cancelled turn never resurfaces after resume
    });
  });
  //#endregion 🧪️TurnDispatchTests

  //#region 🧪️ShardLossRestoreTests
  describe("ActivationRegistry.handleShardLost / restoreActors", () => {
    it("is a valid ShardClientOptions.onShardLost value", () => {
      let registry!: ActivationRegistry;
      const shardClient = new ShardClient({
        residentLedger: fixtureResidentLedger(),
        shardCount: 1,
        createWorker: () => createAutoReplyWorker(),
        onShardLost: (shardIndex, actorIds) => registry.handleShardLost(shardIndex, actorIds),
      });
      registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      expect(typeof registry.handleShardLost).toBe("function");
    });

    it("restores exactly the actors that were on the lost shard, leaving an actor on a different shard untouched", async () => {
      const shardClient = new ShardClient({ residentLedger: fixtureResidentLedger(), shardCount: 2, exclusiveShardCount: 0, createWorker: () => createAutoReplyWorker() });
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });

      await registry.activate("p1", "on-shard-0", "manual");
      await registry.activate("p1", "on-shard-1", "manual");
      expect(shardClient.shardIndexFor("on-shard-0")).toBe(0);
      expect(shardClient.shardIndexFor("on-shard-1")).toBe(1);

      // simulate what checkHeartbeats' own 3-strike ladder does — terminate + rebuild shard 0 only,
      // then hand its actorIds to the registry exactly as ShardClient's own onShardLost callback would.
      const lostActorIds = shardClient.terminate(0);
      shardClient.rebuild(0);
      expect(lostActorIds).toEqual(["on-shard-0"]);

      await registry.restoreActors(lostActorIds);

      expect(registry.isResident("on-shard-0")).toBe(true); // restored
      expect(registry.isResident("on-shard-1")).toBe(true); // never touched — different shard
      expect(shardClient.shardIndexFor("on-shard-0")).toBe(0); // re-activated on the rebuilt shard
    });
  });

  describe("ActivationRegistry restore ordering", () => {
    it("a restored actor does not receive turns that were queued before the restore, but does receive turns queued after", async () => {
      const shardClient = new ShardClient({ residentLedger: fixtureResidentLedger(), shardCount: 1, createWorker: () => createAutoReplyWorker() });
      const delivered: string[] = [];
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: () => delivered.push("delivered") });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");

      // Enqueue, then lose the shard, all synchronously — nothing yields to the scheduler's own
      // microtask pump until after `restoreActors` has already cancelled the queue below.
      registry.enqueueTurn("actor-1", "Interactive", []);
      const lostActorIds = shardClient.terminate(0);
      shardClient.rebuild(0);

      await registry.restoreActors(lostActorIds);
      await flushMicrotasks();
      expect(delivered).toEqual([]); // the pre-restart turn never ran

      registry.enqueueTurn("actor-1", "Interactive", []); // enqueued AFTER the restore completed
      await flushMicrotasks();
      expect(delivered).toEqual(["delivered"]); // proves the actor is alive again, not permanently dropped
    });
  });
  //#endregion 🧪️ShardLossRestoreTests

  //#region 🧪️MemoryPressureCapTests
  describe("residentActorCapFromMemory", () => {
    it("derives the cap from deviceMemoryGiB when present, clamped to [4, 96]", () => {
      expect(residentActorCapFromMemory({ deviceMemoryGiB: 1 })).toBe(6);
      expect(residentActorCapFromMemory({ deviceMemoryGiB: 16 })).toBe(96);
    });

    it("falls back to jsHeapSizeLimitBytes when deviceMemoryGiB is absent", () => {
      expect(residentActorCapFromMemory({ jsHeapSizeLimitBytes: 256 * 1024 * 1024 })).toBe(4);
    });

    it("falls back to the hardcoded constant when neither signal is present", () => {
      expect(residentActorCapFromMemory({})).toBe(DEFAULT_MAX_RESIDENT_ACTORS);
    });
  });

  describe("ActivationRegistry.maxResidentActors derived from an injected memory probe", () => {
    async function activateAndCountResident(memoryProbe: MemoryProbe, activationCount: number): Promise<number> {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], memoryProbe });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      for (let i = 0; i < activationCount; i += 1) await registry.activate("p1", `actor-${i}`, "manual");
      let resident = 0;
      for (let i = 0; i < activationCount; i += 1) if (registry.isResident(`actor-${i}`)) resident += 1;
      return resident;
    }

    it("a small deviceMemoryGiB reading evicts down to its (small) derived cap", async () => {
      const resident = await activateAndCountResident(() => ({ deviceMemoryGiB: 1 }), 10);
      expect(resident).toBe(residentActorCapFromMemory({ deviceMemoryGiB: 1 })); // 6
      expect(resident).toBeLessThan(10);
    });

    it("a large deviceMemoryGiB reading keeps every one of the same 10 activations resident", async () => {
      const resident = await activateAndCountResident(() => ({ deviceMemoryGiB: 16 }), 10);
      expect(resident).toBe(10); // well under the derived 96 cap — nothing evicted
    });
  });
  //#endregion 🧪️MemoryPressureCapTests

  //#region 🧪️MetricsBusTests
  describe("ActivationRegistry.metricsBus (autoStartMetricsPublisher)", () => {
    it("publishes os.runtime.metrics as a CustomEvent on metricsBus at the 2Hz interval, driven by the injected clock, and dispose() stops it", () => {
      vi.useFakeTimers();
      try {
        const shardClient = fakeShardClient();
        let simulatedNowMs = 0;
        const registry = new ActivationRegistry({
          shardClient,
          defaultBudget: BUDGET_FIXTURE,
          fetchAssets: async () => [],
          now: () => simulatedNowMs,
          autoStartMetricsPublisher: true,
        });
        const received: RuntimeMetricsSnapshot[] = [];
        registry.metricsBus.addEventListener("os.runtime.metrics", (event) => received.push((event as CustomEvent<RuntimeMetricsSnapshot>).detail));

        simulatedNowMs = RUNTIME_METRICS_PUBLISH_INTERVAL_MS;
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(received).toHaveLength(1);
        expect(received[0]!.sampledAtMs).toBe(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);

        simulatedNowMs = RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 2;
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(received).toHaveLength(2);

        registry.dispose();
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 5);
        expect(received).toHaveLength(2); // dispose() stopped the loop
      } finally {
        vi.useRealTimers();
      }
    });

    it("stays empty (no live interval, no bus traffic) when autoStartMetricsPublisher is left at its default", () => {
      vi.useFakeTimers();
      try {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        const received: RuntimeMetricsSnapshot[] = [];
        registry.metricsBus.addEventListener("os.runtime.metrics", (event) => received.push((event as CustomEvent<RuntimeMetricsSnapshot>).detail));
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 10);
        expect(received).toEqual([]);
        registry.dispose(); // no-op, nothing was started — must not throw
      } finally {
        vi.useRealTimers();
      }
    });
  });
  //#endregion 🧪️MetricsBusTests

}

export async function registerTests4(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<KernelTestModule, "expandPluginRegistry">, source: TestSource): Promise<void> {
  const { expandPluginRegistry } = dependencies;

  const { describe, expect, it } = vitest;
  describe("expandPluginRegistry", () => {
    it("includes transitive dependsOn of primary and consume-matched contributors", () => {
      const plugins = [
        { pluginId: "host-plugin", moduleUrl: "a", consumes: ["ext.tag"], dependencies: [{ pluginId: "core", version: "*" }] },
        { pluginId: "core", moduleUrl: "b", dependencies: [{ pluginId: "stdio", version: "*" }] },
        { pluginId: "stdio", moduleUrl: "c" },
        { pluginId: "ext", moduleUrl: "d", contributes: ["ext.tag"], dependencies: [{ pluginId: "flow", version: "*" }] },
        { pluginId: "flow", moduleUrl: "e", dependencies: [{ pluginId: "stdio", version: "*" }] },
        { pluginId: "unrelated", moduleUrl: "f" },
      ] as const;
      const expanded = expandPluginRegistry(plugins, "host-plugin", false);
      const ids = new Set(expanded.map((entry) => entry.pluginId));
      expect(ids.has("host-plugin")).toBe(true);
      expect(ids.has("core")).toBe(true);
      expect(ids.has("stdio")).toBe(true);
      expect(ids.has("ext")).toBe(true);
      expect(ids.has("flow")).toBe(true);
      expect(ids.has("unrelated")).toBe(false);
    });
  });

}

export async function registerTests5(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<KernelTestModule, "IoEntryGraph" | "ioIdentify" | "ioRun"> & Pick<IoSchemaTestModule, "dialectCoordinate">, source: TestSource): Promise<void> {
  const { IoEntryGraph, dialectCoordinate, ioIdentify, ioRun } = dependencies;
  type ArtifactDialect = import("../../../🚪️io/🧬️schema/🟦️.ts").ArtifactDialect;
  type IoEntryGraphPlugin = import("../../🟦️.ts").IoEntryGraphPlugin;

  const { describe, expect, it } = vitest;
  describe("IoEntryGraph", () => {
    // 🧭️ SAME fixture as the Rust twin (`💻️os/🔌️plugin/🖥️host/🦀️.rs`,
    // `io_router_w1d_fixture_entries`) and `🧪️w1d-io-router-parity.ts` — `stdio` owns one Exact
    // hop, `gif` owns a Canonical migration hop AND a competing Lossy direct shortcut.
    const binaryRaw: ArtifactDialect = { artifactKind: "s.stdio.binary", standard: "raw", subset: "*" };
    const gif87a: ArtifactDialect = { artifactKind: "s.stdio.gif", standard: "87a", subset: "*" };
    const gif89a: ArtifactDialect = { artifactKind: "s.stdio.gif", standard: "89a", subset: "*" };
    const fixturePlugins: IoEntryGraphPlugin[] = [
      { pluginId: "stdio", entries: [{ from: binaryRaw, into: gif87a, fidelity: "Exact", sniffs: true }] },
      {
        pluginId: "gif",
        entries: [
          { from: gif87a, into: gif89a, fidelity: "Canonical", sniffs: false },
          { from: binaryRaw, into: gif89a, fidelity: "Lossy", sniffs: true },
        ],
      },
    ];

    it("resolves the highest-minimum-fidelity route regardless of registration order", () => {
      const forward = IoEntryGraph.build(fixturePlugins).route(binaryRaw, gif89a);
      const reversed = IoEntryGraph.build([...fixturePlugins].reverse()).route(binaryRaw, gif89a);
      expect(forward).toEqual(reversed);
      expect(forward).toEqual({
        hops: [
          { from: binaryRaw, into: gif87a, fidelity: "Exact", sniffs: true },
          { from: gif87a, into: gif89a, fidelity: "Canonical", sniffs: false },
        ],
        fidelity: "Canonical",
      });
    });

    it("respects maxHops, picking the direct (weaker) shortcut when bounded to 1", () => {
      const route = IoEntryGraph.build(fixturePlugins).route(binaryRaw, gif89a, 1);
      expect(route).toEqual({ hops: [{ from: binaryRaw, into: gif89a, fidelity: "Lossy", sniffs: true }], fidelity: "Lossy" });
    });

    it("rejects a different plugin claiming an already-owned (from,into) key", () => {
      expect(() => IoEntryGraph.build([...fixturePlugins, { pluginId: "intruder", entries: [{ from: binaryRaw, into: gif87a, fidelity: "Lossy", sniffs: false }] }])).toThrow(/conflict/);
    });

    it("ownerOf reports the registering plugin", () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      expect(graph.ownerOf(binaryRaw, gif87a)).toBe("stdio");
      expect(graph.ownerOf(gif87a, gif89a)).toBe("gif");
      expect(graph.ownerOf(gif89a, binaryRaw)).toBeUndefined();
    });

    it("carrierEntries returns only the sniff-declaring hops whose from is the given carrier", () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const entries = graph.carrierEntries(binaryRaw);
      expect(entries.map((entry) => ({ into: entry.into, pluginId: entry.pluginId }))).toEqual([
        { into: gif87a, pluginId: "stdio" },
        { into: gif89a, pluginId: "gif" },
      ]);
    });

    it("ioRun executes the whole route hop by hop, feeding each hop's output to the next", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const calls: string[] = [];
      const result = await ioRun(graph, "norm", binaryRaw, gif89a, new Uint8Array([1]), (pluginId, from, into, payload) => {
        calls.push(`${pluginId}:${dialectCoordinate(from)}->${dialectCoordinate(into)}`);
        return new Uint8Array([...payload, payload.length]);
      });
      expect(calls).toEqual(["stdio:s.stdio.binary@raw/*->s.stdio.gif@87a/*", "gif:s.stdio.gif@87a/*->s.stdio.gif@89a/*"]);
      expect(Array.from(result)).toEqual([1, 1, 2]);
    });

    it("ioRun refuses the WHOLE route (no partial execution) when the calling plugin owns any hop", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      let ran = false;
      await expect(
        ioRun(graph, "gif", binaryRaw, gif89a, new Uint8Array(), () => {
          ran = true;
          return new Uint8Array();
        }),
      ).rejects.toThrow(/refused/);
      expect(ran).toBe(false);
    });

    it("ioIdentify fans sniffHop out across carrier entries, skipping the calling plugin's own, sorted by confidence then coordinate", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const results = await ioIdentify(graph, "norm", binaryRaw, new Uint8Array(), (pluginId) => (pluginId === "stdio" ? 3 : 1));
      expect(results).toEqual([
        [gif87a, "High"],
        [gif89a, "Low"],
      ]);
    });

    it("ioIdentify skips the calling plugin's own carrier entries", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const results = await ioIdentify(graph, "stdio", binaryRaw, new Uint8Array(), () => 3);
      expect(results).toEqual([[gif89a, "High"]]);
    });
  });

}
