import { loadActorCodecFixture, loadTurnBootstrapCompositionFixture } from "@semio-tech/framework-actor/testing/fixtures";
type TestSource = { readonly directory: string; readonly url: string };

/** 🧬️ LAW: one codec call reaches a component's `codec` interface through every browser hop with the
 * same answer — the zero-import arm (`actorCodecAnswer`), the GENERATED bridge (`pluginComponentBridgeSource`,
 * imported as a real ES module beside a stand-in jco component) and the GENERATED worker
 * (`shardWorkerSource`'s `codec` case, which also refuses an overlapping turn and a stale activation).
 * Driven from the language-agnostic `🧬️component-codec/🧫️fixtures/🔣️.json`, with independent generated bridge and worker executions so both halves cannot drift.
 * @see 🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts
 * @see 🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts
 * @see 🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs (the native twin) */
export async function registerComponentCodecReplyTests(vitest: NonNullable<ImportMeta["vitest"]>, _testSource: TestSource): Promise<void> {
  const { describe, it, expect } = vitest;
  const fixture = await loadActorCodecFixture();
  const { actorCodecAnswer } = await import("../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧬️codec/🟦️.ts");
  const { OriginalOperationSource } = await import("../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts");
  const { KernelOriginalBootstrapStock, originalBootstrapStockFromWire } = await import("@semio-tech/framework-actor/retained-turn/bootstrap");
  const stockFixture = await loadTurnBootstrapCompositionFixture();
  type CodecRequest = import("../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧬️codec/🟦️.ts").ActorCodecRequest;
  type CodecExports = import("../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧬️codec/🟦️.ts").ActorCodecExports;
  type Case = (typeof fixture.cases)[number];
  type Pair = { readonly packHex: string; readonly sprHex: string };

  const bytes = (hex: string): Uint8Array => Uint8Array.from(hex.match(/../g) ?? [], (pair) => Number.parseInt(pair, 16));
  const hex = (value: unknown): string => (value instanceof Uint8Array ? Array.from(value, (byte) => byte.toString(16).padStart(2, "0")).join("") : `<${Object.prototype.toString.call(value)}>`);
  const pair = (value: Pair) => ({ pack: bytes(value.packHex), spr: bytes(value.sprHex) });
  const request = (row: Case): Record<string, unknown> => ("pair" in row.request && row.request.pair ? { ...row.request, pair: pair(row.request.pair as Pair) } : { ...row.request });
  const componentFault = (value: unknown): Error => Object.defineProperty(new Error(`${String(value)} (see error.payload)`), "payload", { value, enumerable: true });
  const codecFor = (row: Case): CodecExports | undefined => {
    const behaviour = row.component as Record<string, unknown>;
    if (behaviour.exported === false) return undefined;
    const refuse = (): void => {
      if (typeof behaviour.throws === "string") throw new Error(behaviour.throws);
      if (typeof behaviour.faultHex === "string") throw componentFault({ tag: "fault", val: bytes(behaviour.faultHex) });
    };
    return {
      packSchemaHash: async () => { refuse(); return bytes(behaviour.bytesHex as string); },
      genesis: async () => { refuse(); return pair(behaviour.pair as Pair); },
      printMirror: async () => { refuse(); const mirror = behaviour.mirror as readonly [string, string]; return mirror; },
    };
  };
  const verdict = async (call: () => Promise<unknown>): Promise<Record<string, unknown>> => {
    try {
      const answer = (await call()) as Record<string, unknown>;
      if ("fault" in answer) return { faultHex: hex(answer.fault) };
      const ok = answer.ok as Record<string, unknown> | Uint8Array | readonly string[];
      if (ok instanceof Uint8Array) return { okHex: hex(ok) };
      if (Array.isArray(ok)) return { okMirror: [...ok] };
      const value = ok as Record<string, unknown>;
      return { okPair: { packHex: hex(value.pack), sprHex: hex(value.spr) } };
    } catch (error) {
      return { refusal: error instanceof Error ? error.message : String(error) };
    }
  };

  describe("ShardComponentCodecLane", () => {
    it("answers every General codec fixture through the generated product bridge", async () => {
      const { mkdtempSync, mkdirSync, writeFileSync, rmSync } = await import("node:fs");
      const { join } = await import("node:path");
      const { pathToFileURL } = await import("node:url");
      const { pluginComponentBridgeSource } = await import("../../🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts");
      const { hostShimSource } = await import("../../🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts");
      const { APP_CHANNEL_VERSION } = await import("@semio-tech/framework-os");
      const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR;
      if (!artifactBase) throw Error("SEMIO_TEST_ARTIFACT_DIR is required for generated bridge evidence");
      mkdirSync(artifactBase, { recursive: true });
      const directory = mkdtempSync(join(artifactBase, "codec-bridge-"));
      const stock = new KernelOriginalBootstrapStock(originalBootstrapStockFromWire(stockFixture));
      const clock = () => BigInt(Math.floor(performance.now() * 1000));
      try {
        const observed: Record<string, unknown>[] = [];
        for (const [index, row] of fixture.cases.entries()) {
          const codec = codecFor(row);
          (globalThis as { __semioCodecFixture?: unknown }).__semioCodecFixture = codec;
          const caseDirectory = join(directory, `case-${index}`);
          mkdirSync(caseDirectory, { recursive: true });
          writeFileSync(join(caseDirectory, "🌉️bridge.js"), pluginComponentBridgeSource("fixture_component", "fixture.wasm"));
          writeFileSync(join(caseDirectory, "🟨️.js"), hostShimSource());
          writeFileSync(join(caseDirectory, "fixture_component.js"), `export const reactor = { channelVersion: async () => ${APP_CHANNEL_VERSION}, admitActorInput: async (_wire, budget) => ({ input: budget.input, spent: { copiedItems: 0n, copiedBytes: 0n, retainedCapacityBytes: 0n, releasedBytes: 0n }, workUnitsSpent: 0n, ready: true, disposition: "admitted" }) }; export const jobs = {}; export const checkpoint = {}; export const describe = {}; export const codec = globalThis.__semioCodecFixture;\n`);
          const bridge = await import(pathToFileURL(join(caseDirectory, "🌉️bridge.js")).href);
          const original = stock.admit();
          let api: { readonly originalSource: Pick<InstanceType<typeof OriginalOperationSource>, "close" | "issue">; readonly codec: (request: Record<string, unknown>) => Promise<unknown> } | undefined;
          try {
            api = await bridge.createActorApi(`a${index}`, 1n, original, clock, { receipt: (receipt: import("../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts").OriginalOperationReceipt) => stock.receive(original, receipt), hold: (refusal: unknown) => stock.hold(original, refusal) });
            if (!api) throw Error("Missing generated codec bridge API");
            const mounted = api;
            const generated = await verdict(() => mounted.codec(request(row)));
            expect(generated, `${row.id} (generated bridge)`).toEqual(row.expected);
            observed.push({ id: row.id, ...generated });
          } catch (failure) {
            if (!api) stock.hold(original, failure);
            throw failure;
          } finally {
            if (api) {
              const closing=api;
              closing.originalSource.close();
              expect(() => closing.originalSource.issue()).toThrow("original-turn.source-not-ready");
              stock.returnClosed(original, true);
            }
          }
        }
        expect(stock.liveSources).toBe(0);
      } finally {
        delete (globalThis as { __semioCodecFixture?: unknown }).__semioCodecFixture;
        rmSync(directory, { recursive: true, force: true });
      }
    });

    it("preserves a held native poll refusal across the next browser action", async () => {
      const { mkdtempSync, mkdirSync, readFileSync, writeFileSync, rmSync } = await import("node:fs");
      const { join } = await import("node:path");
      const { pathToFileURL } = await import("node:url");
      const { pluginComponentBridgeSource } = await import("../../🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts");
      const { hostShimSource } = await import("../../🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts");
      const { APP_CHANNEL_VERSION } = await import("@semio-tech/framework-os");
      const law = JSON.parse(readFileSync(new URL("./🧫️fixtures/🚫️native-refusal.json", import.meta.url), "utf8"));
      const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR;
      if (!artifactBase) throw Error("SEMIO_TEST_ARTIFACT_DIR is required for generated bridge evidence");
      mkdirSync(artifactBase, { recursive: true });
      const directory = mkdtempSync(join(artifactBase, "native-refusal-"));
      const stock = new KernelOriginalBootstrapStock(originalBootstrapStockFromWire(stockFixture));
      const original = stock.admit();
      const held: unknown[] = [];
      const received: number[] = [];
      try {
        writeFileSync(join(directory, "🌉️bridge.js"), pluginComponentBridgeSource("fixture_component", "fixture.wasm"));
        writeFileSync(join(directory, "🟨️.js"), hostShimSource());
        writeFileSync(join(directory, "fixture_component.js"), `
          const receipt = (budget) => ({ input: budget.input, spent: { copiedItems: 0n, copiedBytes: 0n, retainedCapacityBytes: 0n, releasedBytes: 0n }, workUnitsSpent: 0n });
          export const reactor = {
            channelVersion: async () => ${APP_CHANNEL_VERSION},
            admitActorContext: async (budget) => ({ ...receipt(budget), ready: true }),
            admitActorInput: async (_wire, budget) => ({ ...receipt(budget), ready: true, disposition: "admitted" }),
            poll: async (budget) => { throw Object.assign(Error("[object Object] (see error.payload)"), {
              payload: { ...receipt(budget), failure: { tag: ${JSON.stringify(law.failure)} } },
            }); },
          };
        `);
        const bridge = await import(pathToFileURL(join(directory, "🌉️bridge.js")).href);
        const clock = () => BigInt(Math.floor(performance.now() * 1000));
        const api = await bridge.createActorApi("native-refusal", 1n, original, clock, {
          receipt(receipt: Parameters<typeof stock.receive>[1]) {
            received.push(Number(receipt.input.epoch));
            stock.receive(original, receipt);
          },
          hold(refusal: unknown) {
            held.push(refusal);
            stock.hold(original, refusal);
          },
        });
        const poll = () => api.poll([], undefined, undefined, { maxEffects: 1, maxPatchBytes: 1, maxFrames: 1 });
        await expect(poll()).rejects.toThrow(law.expectedMessage);
        await expect(poll()).rejects.toThrow(law.expectedMessage);
        await expect(api.closeOriginal()).rejects.toThrow(law.expectedCloseMessage);
        expect(received).toEqual(law.expectedReceiptEpochs);
        expect(held).toHaveLength(law.expectedHeldCount);
        expect(api.originalSource.nativeRefusal).toBe(held[0]);
      } finally {
        rmSync(directory, { recursive: true, force: true });
      }
    });

    it("answers codec requests with a beat, schedules guest crossings in order, and closes a suspended actor", async () => {
      const vm = await import("node:vm");
      const { readFileSync } = await import("node:fs");
      const { shardWorkerSource } = await import("../../🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts");
      const { classicShardWorkerSource } = await import("../✂️worker-vm-source/🟦️.ts");
      const scheduling = JSON.parse(readFileSync(new URL("./🧫️fixtures/🚦️actor-turn-order.json", import.meta.url), "utf8"));
      const posted: Record<string, unknown>[] = [];
      let dispatch: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
      let row = fixture.cases[0]!;
      const stock = new KernelOriginalBootstrapStock(originalBootstrapStockFromWire(stockFixture)), original = stock.admit();
      const originalSource = new OriginalOperationSource(original, () => BigInt(Math.floor(performance.now() * 1000)));
      try {
      const context = vm.createContext({
        WebAssembly: { Suspending: class {}, promising: (value: unknown) => value },
        TextDecoder,
        console: { log: () => {}, error: () => {}, warn: () => {} },
        setInterval: () => null,
        clearInterval: () => {},
        self: { postMessage: (message: Record<string, unknown>) => posted.push(message), addEventListener: (kind: string, callback: typeof dispatch) => { if (kind === "message") dispatch = callback; } },
        api: { originalSource, codec: (codecRequest: import("../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧬️codec/🟦️.ts").ActorCodecRequest) => actorCodecAnswer(codecFor(row), codecRequest) },
      });
      new vm.Script(classicShardWorkerSource(shardWorkerSource())).runInContext(context);
      new vm.Script('actors.set("a", { api, activationGeneration: 1n, pendingAssets: [] });').runInContext(context);
      const send = dispatch as unknown as ((event: { data: Record<string, unknown> }) => Promise<void>) | null;
      if (!send) throw new Error("Missing generated worker dispatcher");
      const answered = (requestId: string) => posted.find((message) => message.kind === "result" && message.requestId === requestId) as { ok?: boolean; value?: unknown; error?: string; beat?: { turnSeq?: number } } | undefined;
      for (const [index, current] of fixture.cases.entries()) {
        row = current;
        await send({ data: { kind: "codec", requestId: `c${index}`, actorId: "a", activationGeneration: 1n, request: request(current) } });
        const result = answered(`c${index}`);
        expect(result, `${current.id}: a codec request the worker never answers is an outstanding request the watchdog kills the shard over`).toBeTruthy();
        const worker = result?.ok ? await verdict(async () => result.value) : { refusal: result?.error?.replace(/^Error: /, "") };
        expect(worker, `${current.id} (generated worker)`).toEqual(current.expected);
        if (result?.ok) expect(typeof result.beat?.turnSeq, "a codec call is a guest step and proves liveness like a job step").toBe("number");
      }
      let refusedGuestCalls=0;
      context.api.codec=(codecRequest:CodecRequest)=>actorCodecAnswer({packSchemaHash:async()=>{refusedGuestCalls++;return new Uint8Array([1]);},genesis:async()=>{refusedGuestCalls++;return{pack:new Uint8Array([1]),spr:new Uint8Array([2])};},printMirror:async()=>{refusedGuestCalls++;return["one","two"];}},codecRequest);
      for(const [index,current]of fixture.requestCases.entries())if(!current.accepted){
        const value:unknown=current.request,projected=value!==null&&typeof value==="object"&&!Array.isArray(value)?Object.fromEntries(Object.entries(value).map(([name,item])=>[name,name==="pair"&&item!==null&&typeof item==="object"&&!Array.isArray(item)?Object.fromEntries(Object.entries(item).map(([key,value])=>[key==="packHex"?"pack":key==="sprHex"?"spr":key,(key==="packHex"||key==="sprHex")&&typeof value==="string"?bytes(value):value])):item])):value;
        await send({data:{kind:"codec",requestId:"request-refusal-"+index,actorId:"a",activationGeneration:1n,request:projected}});
        const result=answered("request-refusal-"+index);expect(result?.ok).toBe(false);expect(result?.error?.replace(/^Error: /,"")).toBe(current.refusal);
      }
      expect(refusedGuestCalls).toBe(0);
      await send({ data: { kind: "codec", requestId: "stale", actorId: "a", activationGeneration: 2n, request: request(row) } });
      expect(answered("stale")?.ok).toBe(false);
      expect(answered("stale")?.error).toContain("actor-lifecycle.activation-mismatch");
      const calls: string[] = [];
      const sendCodec = (id: string, actorId = "a", activationGeneration = 1n) =>
        send({ data: { kind: "codec", requestId: id, actorId, activationGeneration, request: id } });
      const firstEntered = Promise.withResolvers<void>();
      const finishFirst = Promise.withResolvers<void>();
      context.api.codec = async (id: string) => {
        calls.push(id);
        if (id === "first") { firstEntered.resolve(); await finishFirst.promise; }
      };
      context.api.takeSegmentedDownloadChunk = async () => { calls.push("download"); };
      new vm.Script('actors.set("other", { api, activationGeneration: 2n, pendingAssets: [] });').runInContext(context);
      const first = sendCodec("first");
      await firstEntered.promise;
      const download = send({ data: { kind: "takeSegmentedDownloadChunk", requestId: "download", actorId: "a", instanceId: 1, operationId: 1n } });
      const second = sendCodec("second");
      try {
        await sendCodec("other-actor", "other", 2n);
        expect(calls).toEqual(scheduling.whileFirstActorIsSuspended);
        expect(answered("download")).toBeUndefined();
        expect(answered("second")).toBeUndefined();
      } finally {
        finishFirst.resolve();
        await Promise.all([first, download, second]);
      }
      expect(calls).toEqual(scheduling.afterFirstActorResumes);
      for (const id of ["first", "download", "second", "other-actor"]) expect(answered(id)?.ok).toBe(true);

      calls.length = 0;
      const pollEntered = Promise.withResolvers<void>();
      const finishPoll = Promise.withResolvers<void>();
      context.api.poll = async () => {
        if (calls.includes("turn")) { calls.push("drive"); return { status: { tag: "idle" } }; }
        calls.push("turn");
        pollEntered.resolve();
        await finishPoll.promise;
        return {
          status: { tag: "more-work" }, effects: [], uiPatches: [],
          commandIngress: { tag: "idle" }, coldPairIngress: { tag: "idle" },
        };
      };
      const turn = send({ data: { kind: "turn", requestId: "turn", actorId: "a", activationGeneration: 1n, events: [], budget: {} } });
      await pollEntered.promise;
      const afterTurn = sendCodec("after-turn");
      finishPoll.resolve();
      await Promise.all([turn, afterTurn]);
      expect(calls).toEqual(scheduling.afterInputArrivesDuringPoll);
      expect(answered("turn")?.ok).toBe(true);
      expect(answered("after-turn")?.ok).toBe(true);

      calls.length = 0;
      const lastEntered = Promise.withResolvers<void>();
      const finishLast = Promise.withResolvers<void>();
      context.api.codec = async (id: string) => {
        calls.push(id);
        if (id === "finishing") { lastEntered.resolve(); await finishLast.promise; }
      };
      context.api.closeOriginal = async () => {
        calls.push("close");
        return { operation: original.operation, generation: original.generation, sourceClosed: true };
      };
      const finishing = sendCodec("finishing");
      await lastEntered.promise;
      const queued = sendCodec("queued");
      const dispose = send({ data: { kind: "dispose", actorId: "a", activationGeneration: 1n } });
      try {
        expect(posted.some(message => message.kind === "original-source-close")).toBe(false);
      } finally {
        finishLast.resolve();
        await Promise.all([finishing, queued, dispose]);
      }
      expect(calls).toEqual(scheduling.disposeWhileSuspended);
      expect(answered("queued")?.error).toContain(scheduling.queuedRefusal);
      expect(posted.filter(message => message.kind === "original-source-close")).toEqual([
        { kind: "original-source-close", actorId: "a", activationGeneration: 1n, operation: original.operation, generation: original.generation, sourceClosed: true },
      ]);
      } finally {
        originalSource.close();
        expect(() => originalSource.issue()).toThrow("original-turn.source-not-ready");
        stock.returnClosed(original, true);
      }
      expect(stock.liveSources).toBe(0);
    });
  });
}
