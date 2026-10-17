/** 🚪️ Actual OS worker preserves source admission and activation lifetime through delayed disposal. */
import { test, expect } from "bun:test";
import { pathToFileURL } from "node:url";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { KernelOriginalBootstrapStock, originalBootstrapStockFromWire } from "../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🟦️.ts";
import { originalOperationWorkerSource, type OriginalOperationReceipt } from "../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts";
import { turnBootstrapCompositionFixture as stockFixture, retainedTurnBootstrapSchema as stockSchema, actorLifetimeFixtureUrl } from "../../../../../../../../🔨️modules/🎭️actor/🧪️tests/🧫️fixtures/🟦️.ts";

test("actor instance close worker activation preserves the new generation across a delayed dispose", async () => {
    const { readFileSync } = await import("node:fs");
    const materializeSpecifier = new URL("../../🟦️.ts", import.meta.url).href;
    const { shardWorkerSource } = await import(/* @vite-ignore */ materializeSpecifier);
    const fixture = JSON.parse(readFileSync(actorLifetimeFixtureUrl, "utf8"));
    const prior = BigInt(fixture.reopen.prior.activationGeneration);
    const current = prior + 1n;
    expect(new Ajv({ strict: true }).validate(stockSchema, stockFixture)).toBe(true);
    const pool = new KernelOriginalBootstrapStock(originalBootstrapStockFromWire(stockFixture));
    const original = new Map([[prior, pool.admit()], [current, pool.admit()]]);
    const { Worker } = await import("node:worker_threads");
    type Message = { readonly kind: string; readonly requestId?: string; readonly ok?: boolean; readonly value?: unknown; readonly error?: string; readonly activationGeneration?: bigint; readonly receipt?: OriginalOperationReceipt; readonly sourceClosed?: boolean };
    const pending = new Map<string, { resolve: (value: Message) => void; reject: (error: unknown) => void }>();
    const closed = new Map<bigint, () => void>();
    let receipts = 0;
    const emitted = shardWorkerSource();
    const clocks = "../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/clocks.js";
    expect(emitted.split(JSON.stringify(clocks)).length).toBe(2);
    const nodeWorker = emitted.replace(JSON.stringify(clocks), JSON.stringify(pathToFileURL(Bun.resolveSync("@bytecodealliance/preview2-shim/clocks", import.meta.dir)).href));
    const worker = new Worker(
      "const { parentPort } = require('node:worker_threads'); const self = { location: { href: 'https://lifecycle-test.invalid/worker.js' }, postMessage: value => parentPort.postMessage(value), addEventListener: (_, callback) => parentPort.on('message', data => callback({ data })) }; const WebAssembly = { Suspending: function(){}, promising: function(){} };\n" +
        nodeWorker,
      { eval: true },
    );
    worker.on("message", (message: Message) => {
      if (message.kind === "original-turn-receipt") {
        pool.receive(original.get(message.activationGeneration!)!, message.receipt!);
        receipts++;
      }
      if (message.kind === "original-source-close" && message.sourceClosed) {
        pool.returnClosed(original.get(message.activationGeneration!)!, true);
        closed.get(message.activationGeneration!)?.();
        closed.delete(message.activationGeneration!);
      }
      if (message.kind !== "result" || !message.requestId) return;
      const waiting = pending.get(message.requestId);
      if (waiting) {
        pending.delete(message.requestId);
        waiting.resolve(message);
      }
    });
    worker.on("error", (error) => {
      for (const waiting of pending.values()) waiting.reject(error);
      pending.clear();
    });
    const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifacts) throw Error("An original caller artifact directory is required");
    mkdirSync(artifacts, { recursive: true });
    const directory = mkdtempSync(join(artifacts, "lifetime-"));
    const module = join(directory, "📜️script.ts");
    writeFileSync(module, originalOperationWorkerSource() + `
export async function createActorApi(actorId, activationGeneration, admission, clock, returned) {
  const originalSource = new OriginalOperationSource(admission, clock);
  const receive = () => {
    const turn = originalSource.issue();
    const receipt = { input: turn.input, spent: { copiedItems: 1, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 }, workUnitsSpent: 1 };
    originalSource.receive(receipt);
    returned.receipt(receipt);
  };
  return {
    originalSource,
    poll: async () => { receive(); return { actorId, activationGeneration }; },
    closeOriginal: async () => { receive(); originalSource.close(); return { operation: originalSource.operation, generation: originalSource.generation, sourceClosed: true }; }
  };
}`);
    const moduleUrl = pathToFileURL(module).href;
    const send = (data: unknown, requestId: string): Promise<Message> =>
      new Promise((resolve, reject) => {
        pending.set(requestId, { resolve, reject });
        worker.postMessage(data);
      });
    const dispose = (activationGeneration: bigint) => new Promise<void>(resolve => {
      closed.set(activationGeneration, resolve);
      worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration });
    });
    try {
      const activation = await send({ kind: "activate", requestId: "a1", actorId: "same", activationGeneration: prior, moduleUrl, assets: [], originalAdmission: original.get(prior) }, "a1");
      expect(activation, activation.error).toMatchObject({ ok: true });
      expect((await send({ kind: "turn", requestId: "t1", actorId: "same", activationGeneration: prior, events: [], budget: {} }, "t1")).value).toEqual({ actorId: "same", activationGeneration: prior });
      await dispose(prior);
      expect(await send({ kind: "activate", requestId: "a2", actorId: "same", activationGeneration: current, moduleUrl, assets: [], originalAdmission: original.get(current) }, "a2")).toMatchObject({ ok: true });
      worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration: prior });
      expect((await send({ kind: "turn", requestId: "t2", actorId: "same", activationGeneration: current, events: [], budget: {} }, "t2")).value).toEqual({ actorId: "same", activationGeneration: current });
      expect((await send({ kind: "frame", requestId: "f-old", actorId: "same", activationGeneration: prior, frame: { kind: "Register", actor: "same" } }, "f-old")).error).toMatch(/actor-lifecycle\.activation-mismatch/);
      expect(await send({ kind: "frame", requestId: "f-current", actorId: "same", activationGeneration: current, frame: { kind: "Register", actor: "same" } }, "f-current")).toMatchObject({ ok: true });
      expect((await send({ kind: "activate", requestId: "old", actorId: "old", activationGeneration: prior, moduleUrl, assets: [], originalAdmission: original.get(prior) }, "old")).ok).toBe(false);
      await dispose(current);
      expect(pool.liveSources).toBe(0);
      expect(receipts).toBe(4);
      expect(closed.size).toBe(0);
    } finally {
      await worker.terminate();
      rmSync(directory, { recursive: true, force: true });
    }
  }, 15_000);


test("actual generated component bridge embeds its defining operation admission without imports or owner reconstruction",async()=>{
 const {pluginComponentBridgeSource}=await import("../../🟦️.ts"),{readFileSync}=await import("node:fs"),ts=await import("typescript"),{runInNewContext}=await import("node:vm"),source=pluginComponentBridgeSource("component","component.wasm"),syntax=ts.createSourceFile("generated-bridge.js",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),names=["admitActorCodecRequestV1","admitActorCodecAnswerV1","codecAnswer"],arms=syntax.statements.filter(node=>ts.isVariableStatement(node)&&node.declarationList.declarations.some(declaration=>ts.isObjectBindingPattern(declaration.name)&&declaration.name.elements.some(element=>ts.isIdentifier(element.name)&&names.includes(element.name.text)))).map(node=>node.getText(syntax));expect(arms).toHaveLength(1);expect(ts.preProcessFile(arms.join("\n"),true,true).importedFiles).toHaveLength(0);
 const runtime=runInNewContext(arms.join("\n")+";({codecAnswer,admitActorCodecRequestV1,admitActorCodecAnswerV1})") as {admitActorCodecRequestV1:(value:unknown)=>unknown;codecAnswer:(codec:unknown,request:unknown)=>Promise<any>;admitActorCodecAnswerV1:(operation:string,value:unknown)=>unknown},foreign=runInNewContext('({hash:new Uint8Array([1,2]),pair:{pack:new Uint8Array([3]),spr:new Uint8Array([4])},mirror:["body","history"]})') as {hash:Uint8Array;pair:{pack:Uint8Array;spr:Uint8Array};mirror:readonly[string,string]};expect(foreign.hash instanceof Uint8Array).toBe(false);
 const codec={packSchemaHash:async()=>foreign.hash,genesis:async()=>foreign.pair,printMirror:async(_kind:string,value:unknown)=>{expect(value).toBe(foreign.pair);return foreign.mirror;}};for(const[request,owner]of[[{operation:"pack-schema-hash",artifactKind:"probe"},foreign.hash],[{operation:"genesis",artifactKind:"probe",documentId:"document"},foreign.pair],[{operation:"print-mirror",artifactKind:"probe",pair:foreign.pair},foreign.mirror]]as const){expect(runtime.admitActorCodecRequestV1(request)).toBe(request);const answer=await runtime.codecAnswer(codec,request);expect(answer.ok).toBe(owner);expect(runtime.admitActorCodecAnswerV1(request.operation,answer)).toBe(answer);}let reads=0,calls=0;const hostile={operation:"pack-schema-hash",get artifactKind(){reads++;return"probe";}};await expect(runtime.codecAnswer({packSchemaHash:async()=>{calls++;return foreign.hash;}},hostile)).rejects.toThrow("actor-codec.request");await expect(runtime.codecAnswer({packSchemaHash:async()=>{calls++;return foreign.hash;}},{operation:"pack-schema-hash",artifactKind:"probe",pair:foreign.pair})).rejects.toThrow("actor-codec.request");expect(reads).toBe(0);expect(calls).toBe(0);
 await expect(runtime.codecAnswer({genesis:async()=>foreign.hash},{operation:"genesis",artifactKind:"probe",documentId:"document"})).rejects.toThrow("actor-codec.answer:genesis");const fault=runInNewContext("new Uint8Array([7])")as Uint8Array;const answer=await runtime.codecAnswer({genesis:async()=>{throw{payload:{tag:"fault",val:fault}};}},{operation:"genesis",artifactKind:"probe",documentId:"document"});expect(answer.fault).toBe(fault);
 expect(source).toContain("codec: (request) => codecAnswer(codec, request)");const {shardWorkerSource}=await import("../../🟦️.ts");expect(shardWorkerSource()).toContain("reply(requestId, await actor.api.codec(msg.request)");const renderer=readFileSync(new URL("../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts",import.meta.url),"utf8");expect(renderer).not.toContain("function codecBytes(");expect(renderer).not.toContain("function codecMirror(");expect(renderer).toContain('codecPackSchemaHash: (artifactKind) => handle.codec({ operation: "pack-schema-hash", artifactKind })');
});
