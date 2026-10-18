import { loadActorCodecFixture, loadTurnBootstrapCompositionFixture } from "../../../../../../🔨️modules/🎭️actor/🧪️tests/🧫️fixtures/🟦️.ts";
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
  const { KernelOriginalBootstrapStock, originalBootstrapStockFromWire } = await import("../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🟦️.ts");
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
      const { APP_CHANNEL_VERSION } = await import("../../../../🟦️.ts");
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
          writeFileSync(join(caseDirectory, "fixture_component.js"), `export const reactor = { channelVersion: async () => ${APP_CHANNEL_VERSION} }; export const jobs = {}; export const checkpoint = {}; export const describe = {}; export const codec = globalThis.__semioCodecFixture;\n`);
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

    it("answers a codec request from the generated worker with a result and a beat, and refuses an overlapping turn or a stale activation", async () => {
      const vm = await import("node:vm");
      const { shardWorkerSource } = await import("../../🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts");
      const { classicShardWorkerSource } = await import("../✂️worker-vm-source/🟦️.ts");
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
      context.api.codec=(codecRequest:CodecRequest)=>actorCodecAnswer(codecFor(row),codecRequest);
      row = fixture.cases[0]!;
      new vm.Script('inFlightTurnActors.add("a");').runInContext(context);
      await send({ data: { kind: "codec", requestId: "overlap", actorId: "a", activationGeneration: 1n, request: request(row) } });
      expect(answered("overlap")?.ok, "a codec call never re-enters a component a turn of the same actor is suspended in").toBe(false);
      expect(answered("overlap")?.error).toContain("already has a turn in flight");
      new vm.Script('inFlightTurnActors.delete("a");').runInContext(context);
      await send({ data: { kind: "codec", requestId: "stale", actorId: "a", activationGeneration: 2n, request: request(row) } });
      expect(answered("stale")?.ok).toBe(false);
      expect(answered("stale")?.error).toContain("actor-lifecycle.activation-mismatch");
      } finally {
        originalSource.close();
        expect(() => originalSource.issue()).toThrow("original-turn.source-not-ready");
        stock.returnClosed(original, true);
      }
      expect(stock.liveSources).toBe(0);
    });
  });
}
