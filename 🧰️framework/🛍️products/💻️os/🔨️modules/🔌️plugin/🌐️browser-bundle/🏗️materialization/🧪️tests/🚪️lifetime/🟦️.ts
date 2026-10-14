/** 🚪️ Actual OS worker preserves source admission and activation lifetime through delayed disposal. */
import { test, expect } from "bun:test";
import { pathToFileURL } from "node:url";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { KernelOriginalBootstrapStock, originalBootstrapStockFromWire } from "../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🟦️.ts";
import { originalOperationWorkerSource, type OriginalOperationReceipt } from "../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts";
import stockFixture from "../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/📋️composition.json";
import stockSchema from "../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧬️schema/🔣️.json";

test("actor instance close worker activation preserves the new generation across a delayed dispose", async () => {
    const { readFileSync } = await import("node:fs");
    const materializeSpecifier = new URL("../../🟦️.ts", import.meta.url).href;
    const { shardWorkerSource } = await import(/* @vite-ignore */ materializeSpecifier);
    const fixture = JSON.parse(readFileSync(new URL("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️tests/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
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
      console.log("[DEBUG] Actual emitted OS worker retained two distinct genuine root admissions, four exact receipts, matching close acknowledgements and new activation after stale disposal; no native heap qualification");
    } finally {
      await worker.terminate();
      rmSync(directory, { recursive: true, force: true });
    }
  }, 15_000);
