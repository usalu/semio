/** 💓️ The heartbeat, liveness and shared array buffer heartbeat tests. */
import { loadActorShardClientFixture, loadClientSegmentedDownloadSchema, loadClientSegmentedDownloadFixture } from "../../../../../../../🔨️modules/🎭️actor/🧪️tests/🧫️fixtures/🟦️.ts";
import { readModuleTreeSource } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { flushAdmission, shardClientInternals } from "../🧰️settlement-test-kit/🟦️.ts";
import type { ShardClientSettlementTestKit, ShardJspiScope } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🩺 Registers the postMessage heartbeat watchdog, schema owned liveness and shared array buffer heartbeat tests. */
export function registerHeartbeatAndLivenessTests(kit: ShardClientSettlementTestKit): void {
  const { BUDGET, SEGMENTED_DOWNLOAD_CONTRACT, SEGMENTED_DOWNLOAD_REFUSAL, SHARD_JSPI_FAULT_CODE, SHARD_LIVENESS_POLICY, ShardJspiUnavailableError, assertShardJspiAvailable, describe, describeShardMessageError, describeShardSilence, describeShardWorkerError, evaluateShardLiveness, expect, harness, isShardLostError, it, settleFailedInstanceOpen, shardJspiAvailable, testSource } = kit;
  describe("ShardClient heartbeat watchdog (postMessage path)", () => {
    it("does not miss while idle (no pending turn)", () => {
      const { client, advance } = harness(1);
      advance(100_000);
      client.checkHeartbeats(100_000);
      // no throw, no rebuild callback would have fired — nothing to assert but absence of an exception
      expect(true).toBe(true);
    });

    it("terminates + rebuilds after 3 consecutive missed-heartbeat windows on a stuck turn", async () => {
      const lost: Array<{ index: number; actorIds: readonly string[] }> = [];
      // 🫀️ `firstTurnTimeoutMs` collapsed onto the ordinary window: this test is about the ordinary
      // ladder, and a stuck FIRST turn is priced against its own ceiling (see the first-turn test).
      const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000, firstTurnTimeoutMs: 1000, onShardLost: (index, actorIds) => lost.push({ index, actorIds }) });
      setNow(0);
      const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
      await flushAdmission();
      const activateMsg = workers[0]!.sent[0] as { readonly requestId: string };
      workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;

      const originalWorker = workers[0]!;
      client.turn("stuck", [], BUDGET).catch(() => {}); // never replies — simulates a hung guest; terminate() rejects it

      advance(1001);
      client.checkHeartbeats();
      expect(lost).toEqual([]);
      expect(originalWorker.terminated).toBe(false);

      advance(1001);
      client.checkHeartbeats();
      advance(1001);
      client.checkHeartbeats();

      expect(originalWorker.terminated).toBe(true);
      expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
      expect(client.shardIndexFor("stuck")).toBeUndefined(); // rebuild cleared routing; caller must re-activate
    });

    it("a fresh heartbeat resets the miss count", async () => {
      const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000 });
      setNow(0);
      const activatePromise = client.activate("busy", "https://x/busy.js", [], BUDGET);
      await flushAdmission();
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      void client.turn("busy", [], BUDGET);

      advance(1001);
      client.checkHeartbeats();
      advance(1001);
      client.checkHeartbeats();
      // guest emits a heartbeat right before the 3rd window would elapse
      workers[0]!.deliver({ kind: "heartbeat", turnSeq: 7 });
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false);
    });
  });
  describe("ShardClient liveness watchdog (schema-owned timelines)", () => {
    type LivenessScenario = {
      readonly id: string;
      readonly heartbeatTimeoutMs: number;
      readonly firstTurnTimeoutMs: number;
      readonly timeline: readonly { readonly atMs: number; readonly event: string }[];
      readonly expected: { readonly missesAtCheck: readonly number[]; readonly terminatedAtMs: number | null };
    };
    type LivenessFixture = {
      readonly policy: Readonly<Record<string, number>>;
      readonly worker: { readonly activationPhases: readonly string[]; readonly progressPhase: string };
      readonly jspi: { readonly faultCode: string; readonly vectors: readonly { readonly id: string; readonly webAssembly: boolean; readonly suspending: boolean; readonly promising: boolean; readonly available: boolean }[] };
      readonly scenarios: readonly LivenessScenario[];
    };

    async function livenessFixture(): Promise<LivenessFixture> {
      const fixture = await loadActorShardClientFixture();
      const { actorShardLivenessSchema: schema } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🫀️liveness/🟦️.ts");
      const { default: Ajv } = await import("ajv");
      const validate = new Ajv({ strict: true }).compile(schema);
      expect(validate(SHARD_LIVENESS_POLICY), JSON.stringify(validate.errors)).toBe(true);
      return fixture as unknown as LivenessFixture;
    }

    /** 🔮️ Independent oracle — the watchdog rule re-derived from its prose statement ("a busy shard is
     * one that has said SOMETHING since `heartbeatTimeoutMs` ago; every further silent window costs one
     * miss; `missedLimit` in a row kills it") as a flat simulation over the timeline, sharing no code
     * with {@link evaluateShardLiveness} or `ShardClient`. Agreement between this, the fixture's own
     * recorded expectations and the real client is what makes the rule trustworthy. */
    function simulateLiveness(scenario: LivenessScenario, missedLimit: number): { readonly missesAtCheck: number[]; readonly terminatedAtMs: number | null } {
      let pendingSince: number | null = null;
      let pendingFirstTurn = false;
      let livenessAtMs = 0;
      let missed = 0;
      let lastMissAtMs = 0;
      const missesAtCheck: number[] = [];
      let terminatedAtMs: number | null = null;
      for (const step of scenario.timeline) {
        if (terminatedAtMs !== null) break;
        if (step.event === "pending" || step.event === "pending-first-turn") {
          if (pendingSince === null) { pendingSince = step.atMs; pendingFirstTurn = step.event === "pending-first-turn"; }
          continue;
        }
        if (step.event === "idle" || step.event === "liveness") {
          if (step.event === "idle") { pendingSince = null; pendingFirstTurn = false; }
          livenessAtMs = step.atMs;
          missed = 0;
          lastMissAtMs = step.atMs;
          continue;
        }
        const timeoutMs = pendingFirstTurn ? Math.max(scenario.heartbeatTimeoutMs, scenario.firstTurnTimeoutMs) : scenario.heartbeatTimeoutMs;
        const silent = pendingSince !== null && step.atMs - Math.max(livenessAtMs, pendingSince) > timeoutMs && step.atMs - lastMissAtMs >= timeoutMs;
        if (silent) { missed += 1; lastMissAtMs = step.atMs; }
        missesAtCheck.push(missed);
        if (missed >= missedLimit) terminatedAtMs = step.atMs;
      }
      return { missesAtCheck, terminatedAtMs };
    }

    it("mirrors the schema-owned policy record in every consumer of it", async () => {
      const fixture = await livenessFixture();
      expect({ ...SHARD_LIVENESS_POLICY }).toEqual(fixture.policy);
      const { SEGMENTED_DOWNLOAD_CHUNK_BYTES, SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS } = await import("../../../🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts");
      expect(SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS).toBe(fixture.policy.progressIntervalMs);
      // 📤️ The segmented-download chunk contract, same rule: one schema-owned record, four enforcing hops
      // (guest producer, generated worker, host transport, host drain) and no literal of anyone's own.
      const schema = await loadClientSegmentedDownloadSchema();
      const contractFixture = await loadClientSegmentedDownloadFixture();
      const { default: Ajv } = await import("ajv");
      expect(new Ajv({ strict: true }).validate(schema, { ...SEGMENTED_DOWNLOAD_CONTRACT, maximumOperationId: SEGMENTED_DOWNLOAD_CONTRACT.maximumOperationId.toString() })).toBe(true);
      expect({ ...SEGMENTED_DOWNLOAD_CONTRACT, maximumOperationId: SEGMENTED_DOWNLOAD_CONTRACT.maximumOperationId.toString() }).toEqual(contractFixture.contract);
      expect(SEGMENTED_DOWNLOAD_CHUNK_BYTES).toBe(contractFixture.contract.chunkBytes);
      expect(contractFixture.contract.chunkBytes * contractFixture.contract.maximumOutstandingChunks).toBe(contractFixture.contract.maximumTotalBytes);
      expect(Object.values(SEGMENTED_DOWNLOAD_REFUSAL).sort()).toEqual(contractFixture.refusals.map((refusal) => refusal.code).sort());
      const { shardWorkerSource } = await import("../../../🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts");
      expect(shardWorkerSource()).toContain(`const SEGMENTED_DOWNLOAD_CHUNK_BYTES = ${contractFixture.contract.chunkBytes};`);
    });

    it("serves the shard worker from the schema-owned distribution route, not a transliteration of it", async () => {
      const { SHARD_WORKER_URL } = await import("../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts");
      const { MODULE_PLUGIN_ROUTE, MODULE_SHARD_DIRECTORY } = await import("../../../📇️registry/📦️deployment/🟦️.ts");
      const { SHARD_WORKER_FILE } = await import("../../../🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts");
      expect(SHARD_WORKER_URL).toBe(`${MODULE_PLUGIN_ROUTE}/${MODULE_SHARD_DIRECTORY}/${SHARD_WORKER_FILE}`);
      // 🩺️ Every browser-side spawner must use THAT url. A private ASCII transliteration
      // (`/plugin-modules/_shard/…`) is answered by Vite's SPA fallback with `text/html`, which a
      // module Worker rejects as a message-less `error` event — the 2026-09-05 four-dead-shard boot.
      const { readModuleTreeSource } = await import("../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts");
      for (const spawner of ["../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx", "../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts"]) {
        const source = readModuleTreeSource(new URL(spawner, testSource.url));
        expect(source, spawner).not.toContain("/plugin-modules/_shard/");
        if (source.includes("new Worker(")) expect(source, spawner).toContain("SHARD_WORKER_URL");
      }
    });

    it("classifies JSPI availability exactly as the fixture's vectors say", async () => {
      const fixture = await livenessFixture();
      expect(SHARD_JSPI_FAULT_CODE).toBe(fixture.jspi.faultCode);
      const observed = fixture.jspi.vectors.map((vector) => {
        const scope: ShardJspiScope = vector.webAssembly ? { WebAssembly: { Suspending: vector.suspending ? class {} : undefined, promising: vector.promising ? () => undefined : undefined } } : {};
        const available = shardJspiAvailable(scope);
        // 🔮️ Independent oracle: "JSPI is present when BOTH lifting primitives are callable", stated
        // directly against the vector rather than through the implementation under test.
        expect(available, vector.id).toBe(vector.webAssembly && vector.suspending && vector.promising);
        let thrown: unknown = null;
        try { assertShardJspiAvailable(scope); } catch (error) { thrown = error; }
        expect(thrown === null, vector.id).toBe(vector.available);
        if (thrown !== null) {
          if (!(thrown instanceof ShardJspiUnavailableError)) throw new Error("JSPI refusal did not preserve its typed fault");
          expect(thrown.code).toBe(fixture.jspi.faultCode);
          expect(Object.keys(thrown.text)).toEqual(["en", "de"]);
        }
        return { id: vector.id, available };
      });
      expect(observed).toEqual(fixture.jspi.vectors.map((vector) => ({ id: vector.id, available: vector.available })));
    });

    it("names a worker onerror instead of logging a bare Event", async () => {
      expect(describeShardWorkerError({ type: "error", message: "Uncaught TypeError: WebAssembly.Suspending is not a constructor", filename: "http://localhost:6076/plugin-modules/_shard/🟨️shard-worker.js", lineno: 18, colno: 9 })).toBe(
        "Uncaught TypeError: WebAssembly.Suspending is not a constructor at http://localhost:6076/plugin-modules/_shard/🟨️shard-worker.js:18:9",
      );
      expect(describeShardWorkerError({ type: "error", message: "" })).toContain('redacted "error" event');
      expect(describeShardWorkerError({ error: { message: "boom" } })).toBe("boom");
      const { client, workers } = harness(1);
      void client.activate("crashy", "https://x/c.js", [], BUDGET).catch(() => {});
      await flushAdmission();
      workers[0]!.onerror?.({ type: "error", message: "top-level throw", filename: "worker.js", lineno: 18, colno: 9 });
      expect(isShardLostError(new Error("shard 0 worker crashed: top-level throw at worker.js:18:9"))).toBe(true);
      client.disposeAll();
    });

    it("surfaces a worker messageerror as a named shard loss instead of going quiet", async () => {
      expect(describeShardMessageError(0, { type: "messageerror", data: { bad: true } })).toContain("could not be deserialized");
      expect(isShardLostError(new Error(describeShardMessageError(0, { type: "messageerror" }).replace("shard 0 worker message error", "shard 0 worker crashed")))).toBe(true);
      const { client, workers } = harness(1);
      let rejection: unknown = null;
      const activation = client.activate("mangled", "https://x/c.js", [], BUDGET).catch((error: unknown) => { rejection = error; });
      await flushAdmission();
      workers[0]!.onmessageerror?.({ type: "messageerror" });
      await activation;
      expect(rejection).toBeInstanceOf(Error);
      expect((rejection as Error).message).toContain("could not be deserialized");
      client.disposeAll();
    });

    it("names the silence, the phase and every outstanding request when the watchdog kills a shard", async () => {
      const detail = describeShardSilence({
        shardIndex: 0,
        nowMs: 15_003,
        lastLivenessAtMs: 0,
        lastHeartbeatPhase: "progress",
        inFlight: [{ kind: "turn", actorId: "procedural#1", startedAtMs: 0, firstTurn: true }, { kind: "checkpoint", actorId: "flow#1", startedAtMs: 5_200, firstTurn: false }],
      });
      expect(isShardLostError(new Error(detail))).toBe(true);
      expect(detail).toContain("silent for 15003 ms");
      expect(detail).toContain('last reported phase "progress"');
      expect(detail).toContain("turn (first turn) procedural#1 started 15003 ms ago");
      expect(detail).toContain("checkpoint flow#1 started 9803 ms ago");
      expect(describeShardSilence({ shardIndex: 2, nowMs: 9, lastLivenessAtMs: Number.NEGATIVE_INFINITY, lastHeartbeatPhase: null, inFlight: [] })).toContain("never sent a single message");
      // 🩺️ The live boot's whole evidence was `shard 0 terminated`. Every rejection the watchdog
      // hands back must now carry the cause instead, or the console is back where it started.
      const { client, workers, setNow } = harness(1, { heartbeatTimeoutMs: 1000, firstTurnTimeoutMs: 1000 });
      setNow(0);
      const activate = client.activate("silent", "https://x/s.js", [], BUDGET);
      await flushAdmission();
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent.find((message) => (message as { kind: string }).kind === "activate") as { requestId: string }).requestId, ok: true, value: undefined });
      await activate;
      let turnFault: unknown = null;
      const turn = client.turn("silent", [], BUDGET).catch((error: unknown) => { turnFault = error; });
      for (const atMs of [1001, 2002, 3003, 4004]) { setNow(atMs); client.checkHeartbeats(); }
      await turn;
      expect((turnFault as Error).message).toContain("terminated by the host watchdog");
      expect((turnFault as Error).message).toContain("turn (first turn) silent started");
      client.disposeAll();
    });

    it("prices an actor's first turn against firstTurnTimeoutMs and every later turn against the ordinary window", async () => {
      const first = { nowMs: 20_000, oldestPendingStartedAtMs: 0, lastLivenessAtMs: 0, missedCount: 0, lastMissCountedAtMs: 0, heartbeatTimeoutMs: 5_000, firstTurnTimeoutMs: 30_000 };
      expect(evaluateShardLiveness({ ...first, oldestPendingIsFirstTurn: true })).toEqual({ missedCount: 0, lastMissCountedAtMs: 0, terminate: false });
      expect(evaluateShardLiveness({ ...first, oldestPendingIsFirstTurn: false }).missedCount).toBe(1);
      const { client, workers, setNow } = harness(1, { heartbeatTimeoutMs: 1000, firstTurnTimeoutMs: 10_000 });
      setNow(0);
      const activate = client.activate("slowboot", "https://x/s.js", [], BUDGET);
      await flushAdmission();
      const activateId = (workers[0]!.sent.find((message) => (message as { kind: string }).kind === "activate") as { requestId: string }).requestId;
      workers[0]!.deliver({ kind: "result", requestId: activateId, ok: true, value: undefined });
      await activate;
      // 🫀️ First turn: 4 s of total silence — three ordinary windows, zero first-turn windows.
      const firstTurn = client.turn("slowboot", [], BUDGET);
      const firstTurnId = (workers[0]!.sent.at(-1) as { requestId: string }).requestId;
      for (const atMs of [1001, 2002, 3003, 4004]) { setNow(atMs); client.checkHeartbeats(); }
      expect(workers[0]!.terminated).toBe(false);
      workers[0]!.deliver({ kind: "result", requestId: firstTurnId, ok: true, value: undefined });
      await firstTurn;
      // 🫀️ Second turn on the same activation: the one-off initialization is spent, so the same 4 s
      // of silence is now three misses and a dead shard.
      let lost = 0;
      const second = client.turn("slowboot", [], BUDGET).catch(() => { lost += 1; });
      for (const atMs of [5005, 6006, 7007, 8008]) { setNow(atMs); client.checkHeartbeats(); }
      await second;
      expect(workers[0]!.terminated).toBe(true);
      expect(lost).toBe(1);
      client.disposeAll();
    });

    it("settleFailedInstanceOpen rejects with the ORIGINAL open fault even when the cleanup throws the owner guard", async () => {
      const cleanups: string[] = [];
      const original = new Error("shard 0 terminated by the host watchdog: …");
      await expect(settleFailedInstanceOpen(original, async () => { cleanups.push("ran"); throw new Error("plugin-ui.native-owner-required"); })).rejects.toBe(original);
      expect(cleanups).toEqual(["ran"]);
      await expect(settleFailedInstanceOpen(original, async () => { cleanups.push("clean"); })).rejects.toBe(original);
      expect(cleanups).toEqual(["ran", "clean"]);
    });

    it("agrees with the independent oracle and the fixture on every watchdog timeline", async () => {
      const fixture = await livenessFixture();
      const observed: string[] = [];
      for (const scenario of fixture.scenarios) {
        const lost: number[] = [];
        const { client, workers, setNow } = harness(1, { heartbeatTimeoutMs: scenario.heartbeatTimeoutMs, firstTurnTimeoutMs: scenario.firstTurnTimeoutMs, onShardLost: (index) => lost.push(index) });
        setNow(0);
        const activatePromise = client.activate("live", "https://x/live.js", [], BUDGET);
        await flushAdmission();
        const activateId = (workers[0]!.sent.find((message) => (message as { kind: string }).kind === "activate") as { requestId: string }).requestId;
        workers[0]!.deliver({ kind: "result", requestId: activateId, ok: true, value: undefined });
        await activatePromise;
        const missesAtCheck: number[] = [];
        const outstanding: string[] = [];
        let terminatedAtMs: number | null = null;
        let beat = 0;
        for (const step of scenario.timeline) {
          if (terminatedAtMs !== null) break;
          setNow(step.atMs);
          if (step.event === "pending") {
            void client.checkpoint("live").catch(() => {});
            outstanding.push((workers[0]!.sent.at(-1) as { requestId: string }).requestId);
            continue;
          }
          // 🫀️ A REAL `turn` on a freshly activated actor — the only request the client classifies as
          // a first turn, and the one whose synchronous guest initialization provably cannot beat.
          if (step.event === "pending-first-turn") {
            void client.turn("live", [], BUDGET).catch(() => {});
            outstanding.push((workers[0]!.sent.at(-1) as { requestId: string }).requestId);
            continue;
          }
          if (step.event === "idle") {
            for (const requestId of outstanding.splice(0)) workers[0]!.deliver({ kind: "result", requestId, ok: true, value: new Uint8Array() });
            continue;
          }
          if (step.event === "liveness") {
            beat += 1;
            workers[0]!.deliver({ kind: "heartbeat", turnSeq: beat, phase: fixture.worker.progressPhase });
            continue;
          }
          // 🔎️ Captured BEFORE the tick: a terminating check replaces `shards[0]` with a rebuilt slot
          // whose count is legitimately zero, and the count under test is the retired slot's.
          const slot = (shardClientInternals(client).state.shards as readonly { readonly heartbeat: { readonly missedCount: number } }[])[0]!;
          client.checkHeartbeats();
          missesAtCheck.push(slot.heartbeat.missedCount);
          if (lost.length > 0) terminatedAtMs = step.atMs;
        }
        expect({ id: scenario.id, missesAtCheck, terminatedAtMs }).toEqual({ id: scenario.id, ...simulateLiveness(scenario, fixture.policy.missedLimit!) });
        expect(missesAtCheck, scenario.id).toEqual(scenario.expected.missesAtCheck);
        expect(terminatedAtMs, scenario.id).toBe(scenario.expected.terminatedAtMs);
        client.disposeAll();
        observed.push(scenario.id);
      }
      expect(observed).toEqual(fixture.scenarios.map((scenario) => scenario.id));
    });

    it("beats at every generated-worker activation boundary and tickers on through a stalled import", async () => {
      const fixture = await livenessFixture();
      const { default: ts } = await import("typescript");
      const { readFileSync } = await import("node:fs");
      const vm = await import("node:vm");
      const path = new URL("../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts", testSource.url);
      const parsed = ts.createSourceFile(path.pathname, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
      const declaration = parsed.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "shardWorkerSource");
      const returned = declaration && ts.isFunctionDeclaration(declaration) ? declaration.body?.statements.find(ts.isReturnStatement)?.expression : null;
      if (!returned || !ts.isTemplateExpression(returned)) throw new Error("Changed generated shard worker source");
      const { shardWorkerSource } = await import("../../../🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts");
      // 🧪️ `await import(url)` has no meaning inside a bare `vm` context; the ONE call is redirected to
      // an injected loader so the same generated bytes can be driven with that boundary held open.
      const { classicShardWorkerSource } = await import("../../✂️worker-vm-source/🟦️.ts");
      const source = classicShardWorkerSource(shardWorkerSource())
        .replace("await import(/* @vite-ignore */ moduleUrl)", "await __import(moduleUrl)")
        // 🩺️ The guest-diagnostics shim import is the SECOND dynamic import this worker carries
        // (`armGuestRuntimeDiagnostics`). It is already wrapped in its own try/catch, so a bare `vm`
        // context degrades it to "diagnostics disarmed" — it only has to stop being a bare
        // `await import(...)` this assertion would read as an unredirected boundary.
        .replace('await import(/* @vite-ignore */ "../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js")', "Promise.reject(new Error('no shim in a bare vm context'))")
        // 🩺️ …and the switch that reaches it reads `self.location.href`, which a bare `vm` context
        // does not have. Disarmed here rather than stubbed, because this law is about the beats.
        .replace("await armGuestRuntimeDiagnostics();", "await Promise.resolve(false);");
      expect(source).not.toContain("@vite-ignore");
      const beats: (string | undefined)[] = [];
      const results: string[] = [];
      const intervals: { readonly delayMs: number; readonly tick: () => void }[] = [];
      const cleared: number[] = [];
      let bridge: (value: { createActorApi: (actorId: string, generation: bigint) => Promise<unknown> }) => void = () => {};
      let api: (value: unknown) => void = () => {};
      const bridgeReady = new Promise<{ createActorApi: (actorId: string, generation: bigint) => Promise<unknown> }>((resolve) => { bridge = resolve; });
      const apiReady = new Promise<unknown>((resolve) => { api = resolve; });
      let dispatch: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
      const context = vm.createContext({
        WebAssembly: { Suspending: class {}, promising: (value: unknown) => value },
        __import: () => bridgeReady,
        setInterval: (tick: () => void, delayMs: number) => intervals.push({ delayMs, tick }),
        clearInterval: (handle: number) => cleared.push(handle),
        self: {
          postMessage: (message: { kind: string; phase?: string; requestId?: string }) => {
            if (message.kind === "heartbeat") beats.push(message.phase);
            if (message.kind === "result") results.push(message.requestId ?? "");
          },
          addEventListener: (_kind: string, handler: typeof dispatch) => { dispatch = handler; },
        },
      });
      new vm.Script(source).runInContext(context);
      if (!dispatch) throw new Error("Missing generated worker dispatcher");
      const send = dispatch as (event: { data: Record<string, unknown> }) => Promise<void>;
      const activating = send({ data: { kind: "activate", requestId: "a1", actorId: "a", activationGeneration: 1n, moduleUrl: "https://fixture.invalid/a.js", assets: [] } });
      await Promise.resolve();
      expect(beats).toEqual([fixture.worker.activationPhases[0]]);
      expect(intervals).toHaveLength(1);
      expect(intervals[0]!.delayMs).toBe(fixture.policy.progressIntervalMs);
      intervals[0]!.tick();
      intervals[0]!.tick();
      expect(results).toEqual([]);
      bridge({ createActorApi: () => apiReady as Promise<unknown> });
      await Promise.resolve();
      await Promise.resolve();
      api({ poll: async () => undefined });
      await activating;
      expect(beats).toEqual(["module-fetch", fixture.worker.progressPhase, fixture.worker.progressPhase, "module-ready", "actor-ready"]);
      expect(beats.slice(3)).toEqual(fixture.worker.activationPhases.slice(1));
      expect(results).toEqual(["a1"]);
      expect(cleared).toHaveLength(1);
    });
  });
  describe("ShardClient SAB heartbeat path", () => {
    it("pollHeartbeatSab reads Atomics-stored turnSeq and feeds the same miss-count state machine", async () => {
      const sab = new SharedArrayBuffer(4 * Int32Array.BYTES_PER_ELEMENT);
      const view = new Int32Array(sab);
      const { client, workers, advance, setNow } = harness(1, { heartbeatSab: sab, heartbeatTimeoutMs: 1000 });
      setNow(0);
      const activatePromise = client.activate("sab-actor", "https://x/s.js", [], BUDGET);
      await flushAdmission();
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent.find((m) => (m as { kind: string }).kind === "activate") as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      void client.turn("sab-actor", [], BUDGET);

      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false);

      Atomics.store(view, 0, 5);
      client.pollHeartbeatSab();
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false); // heartbeat seen via SAB reset the window
    });
  });
}
