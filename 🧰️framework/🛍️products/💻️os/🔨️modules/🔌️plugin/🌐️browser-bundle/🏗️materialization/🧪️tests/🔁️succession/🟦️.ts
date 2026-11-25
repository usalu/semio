/** 🔁️ The generated worker, the generated component bridge, the real shard transport and the real kernel stock carry one actor past several exhausted stocks; only the component is a stand-in that enforces the guest's own accounting. */
import { test, expect } from "bun:test";
import { pathToFileURL } from "node:url";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { KernelOriginalBootstrapStock, RootOriginalBootstrapOwner } from "@semio-tech/framework-actor/retained-turn/bootstrap";
import { ShardRequestTransport } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/📬️request-transport/🟦️.ts";
import fixture from "../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🔁️succession/🧪️tests/🧫️fixtures/🔣️.json";
import { faultNotice } from "../../../../../../../../🔨️modules/🎠️kernel/🏷️notice-labels/🟦️.ts";

const operation = { grant: fixture.stock.grant, fuel: fixture.stock.fuel, maximumElapsedUs: 3_600_000_000n };
const packs = new URL("../../../../../../../../🔨️modules/🎁️packs/🟦️.ts", import.meta.url).href;

async function host(operations: number, command: typeof fixture.run.command) {
  const { shardWorkerSource } = await import("../../🧵️shard-worker-source/🟦️.ts");
  const { pluginComponentBridgeSource } = await import("../../🌉️component-bridge-source/🟦️.ts");
  const { hostShimSource } = await import("../../🪢️host-shim-source/🟦️.ts");
  const { APP_CHANNEL_VERSION } = await import("@semio-tech/framework-os");
  const { Worker } = await import("node:worker_threads");
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("An original caller artifact directory is required");
  mkdirSync(artifacts, { recursive: true });
  const directory = mkdtempSync(join(artifacts, "succession-"));
  writeFileSync(join(directory, "🌉️bridge.js"), pluginComponentBridgeSource("fixture_component"));
  writeFileSync(join(directory, "🟨️.js"), hostShimSource());
  writeFileSync(join(directory, "fixture_component.js"), `
    import { decodePackValue, packUIntSafeOrNull } from ${JSON.stringify(packs)};
    export async function instantiate(getCoreModule, imports) {
    const axes = [["maximumItems", "copiedItems"], ["maximumCopyBytes", "copiedBytes"], ["maximumCapacityBytes", "retainedCapacityBytes"], ["maximumReleaseBytes", "releasedBytes"]];
    const none = { copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 };
    const command = ${JSON.stringify(command)};
    const number = (value) => packUIntSafeOrNull(value);
    let staged, left, fuel, epoch = 0n, source, tenure, input, document = 0, successions = 0;
    const jobs = new Map();
    let registered = false;
    const begin = (budget) => {
      const turn = budget.input;
      if (left === undefined) { source = [turn.operation, turn.generation, turn.work.absoluteDeadlineUs, turn.work.clockBinding]; tenure = turn.operation; left = Object.fromEntries(axes.map(([axis]) => [axis, Number(turn.grant[axis])])); fuel = Number(turn.work.fuel); }
      if ([turn.operation, turn.generation, turn.work.absoluteDeadlineUs, turn.work.clockBinding].some((value, index) => value !== source[index]) || turn.epoch <= epoch) throw new Error("guest.identity");
      for (const [axis] of axes) if (Number(turn.grant[axis]) > left[axis]) throw new Error("guest.renewed-authority");
      if (Number(turn.work.fuel) > fuel) throw new Error("guest.renewed-work");
      epoch = turn.epoch;
    };
    const spend = (budget, spent, work) => {
      for (const [axis, key] of axes) { left[axis] -= spent[key]; if (left[axis] < 0) throw new Error("guest.overspent"); }
      fuel -= work;
      return { input: budget.input, spent: Object.fromEntries(Object.keys(none).map((key) => [key, BigInt(spent[key])])), workUnitsSpent: BigInt(work), refusal: { tag: "none" } };
    };
    const refusal = (val) => ({ tag: "some", val });
    const reactor = {
      channelVersion: async () => ${APP_CHANNEL_VERSION},
      stageActorContextBudget: (budget) => { staged = budget; },
      admitActorInput: async (wire) => { const budget = staged; begin(budget); input = decodePackValue(wire); if (input.operation === "start-job" && number(input.source?.job) === 13) { input = undefined; return { ...spend(budget, { ...none, copiedItems: 1 }, 0), ready: false, disposition: "returned" }; } return { ...spend(budget, { ...none, copiedItems: 1, copiedBytes: wire.length, retainedCapacityBytes: wire.length }, 0), ready: true, disposition: "retained" }; },
      admitActorContext: async () => {
        const budget = staged, succession = input?.operation === "succeed" ? input.succession : undefined;
        begin(budget);
        if (succession === undefined) return { ...spend(budget, none, 0), ready: true };
        input = undefined;
        if (jobs.size > 0) return { ...spend(budget, none, 0), ready: false, refusal: refusal("ownership-limit") };
        if (BigInt(number(succession.epoch)) !== budget.input.epoch || BigInt(number(succession.successor)) <= tenure) return { ...spend(budget, none, 0), ready: false, refusal: refusal("invariant-violated") };
        const reply = { ...spend(budget, { ...none, copiedItems: 1, copiedBytes: 96 }, 1), ready: true };
        tenure = BigInt(number(succession.successor)); successions += 1; fuel = number(succession.fuel);
        left = Object.fromEntries(axes.map(([axis]) => [axis, number(succession.grant[axis])]));
        return reply;
      },
      poll: async () => {
        const budget = staged; begin(budget);
        if (input?.operation !== "poll") throw new Error("guest.input");
        input = undefined; document += 1;
        const { work, ...spent } = command, { refusal: _, ...receipt } = spend(budget, spent, work);
        return { retainedReceipt: receipt, uiPatches: [], effects: [], presence: [], nextWake: { tag: "none" }, status: { tag: "idle" }, fuelUsed: BigInt(work), commandIngress: { kind: 0, cursor: {}, fault: new Uint8Array() }, coldPairIngress: { tag: "idle" }, lifecycleReceipt: { tag: "none" }, uiPatchReceipt: { tag: "none" }, probe: { document, successions, tenure, jobs: jobs.size } };
      },
      closeActorContext: async () => { const budget = staged; begin(budget); if (registered) return { ...spend(budget, none, 0), complete: false, blocked: false }; return { ...spend(budget, { ...none, copiedItems: 1 }, 1), complete: true, blocked: false }; },
    };
    let jobBudget;
    const work = (extra) => { const budget = jobBudget; begin(budget); const { work, ...spent } = command; return { ...spend(budget, spent, work), ...extra }; };
    const small = (extra) => { const budget = jobBudget; begin(budget); return { ...spend(budget, { ...none, copiedItems: 1 }, 1), ...extra }; };
    const jobsApi = {
      stageJobBudget: (budget) => { jobBudget = budget; },
      startJob: async (job) => { jobs.set(job, 3); registered = true; input = undefined; return work({ admitted: true }); },
      stepJob: async (job) => { const steps = jobs.get(job); jobs.set(job, steps - 1); return work({ outcome: steps > 1 ? { tag: "none" } : { tag: "some", val: { job, operation: 1n, generation: 1n, state: "done", byteLength: 0n } } }); },
      readJobOutput: async () => small({ chunk: { tag: "some", val: { offset: 0n, bytes: { tag: "none" }, outputClosed: true } } }),
      acknowledgeJobOutput: async (job) => { jobs.delete(job); return small({ acknowledged: true }); },
      cancelJob: async (job) => { jobs.delete(job); return small({ admitted: true }); },
      closeJobs: async () => { jobs.clear(); registered = false; return small({ admitted: true }); },
    };
    return { reactor, jobs: jobsApi, checkpoint: {}, describe: {} };
    }
    export const wasiImports = Object.freeze({});
  `);
  const clocks = "../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/clocks.js";
  const source = shardWorkerSource().replace(JSON.stringify(clocks), JSON.stringify(pathToFileURL(Bun.resolveSync("@bytecodealliance/preview2-shim/clocks", import.meta.dir)).href));
  const worker = new Worker("const { parentPort } = require('node:worker_threads'); const self = { location: { href: 'https://succession-test.invalid/worker.js' }, postMessage: value => parentPort.postMessage(value), addEventListener: (kind, callback) => { if (kind === 'message') parentPort.on('message', data => callback({ data })); } }; WebAssembly.Suspending = function(){}; WebAssembly.promising = function(){};\n" + source, { eval: true });
  const stock = new KernelOriginalBootstrapStock({ version: 1, operation, stock: { grant: { maximumItems: operation.grant.maximumItems * operations, maximumCopyBytes: operation.grant.maximumCopyBytes * operations, maximumCapacityBytes: operation.grant.maximumCapacityBytes * operations, maximumReleaseBytes: operation.grant.maximumReleaseBytes * operations, maximumDepth: operation.grant.maximumDepth }, fuel: operation.fuel * operations, maximumElapsedUs: operation.maximumElapsedUs } });
  const owner = new RootOriginalBootstrapOwner(stock), shards: any[] = [];
  const state: any = { shards, pending: new Map(), originalClosures: new Map(), originalReturns: new Map(), actorActivations: new Map(), actorsPastFirstTurn: new Set(), actorShard: new Map(), originalWorkStock: owner, now: () => performance.now(), heartbeatSabView: null, requestSeq: 0, onActorTrap: () => {}, createWorker: () => { const port: any = { postMessage: (message: unknown) => worker.postMessage(message) }; worker.on("message", (data) => port.onmessage({ data })); worker.on("error", (error) => port.onerror(error)); return port; } };
  const transport = new ShardRequestTransport({ state, watchdog: { noteLiveness() {}, recordHeartbeat() {} }, activations: { inboundActivation: () => undefined }, effects: { handleInboundFrame() {}, abortOutstandingEffects() {} } } as never);
  shards.push(transport.spawnShard(0));
  const originalAdmission = await owner.admit();
  state.actorActivations.set("a", { originalAdmission, slot: shards[0], actorId: "a", generation: 1n, available: true });
  const send = (message: Record<string, unknown>): Promise<any> => { const requestId = transport.nextRequestId(); return transport.send(shards[0], { requestId, actorId: "a", ...message } as never, requestId); };
  await send({ kind: "activate", activationGeneration: 1n, moduleUrl: pathToFileURL(join(directory, "🌉️bridge.js")).href, assets: [], originalAdmission });
  return { stock, state, originalAdmission, send, turn: () => send({ kind: "turn", activationGeneration: 1n, events: [], budget: { maxEffects: 1, maxPatchBytes: 1, maxFrames: 1 } }), start: (job: bigint) => send({ kind: "startJob", job, jobKind: "kind", input: new Uint8Array([1]) }), step: (job: bigint) => send({ kind: "stepJob", job, budget: { deadlineMs: 1000 } }), cancel: (job: bigint) => send({ kind: "cancelJob", job }), retire: async () => { state.originalClosures.set(originalAdmission.operation, state.actorActivations.get("a")); shards[0].worker.postMessage({ kind: "dispose", actorId: "a", activationGeneration: 1n }); for (let wait = 0; wait < 500 && stock.liveSources > 0; wait += 1) await new Promise((resolve) => setTimeout(resolve, 10)); }, close: async () => { await worker.terminate(); rmSync(directory, { recursive: true, force: true }); } };
}
const heavy = { ...fixture.run.command, copiedBytes: 1 << 18, retainedCapacityBytes: 1 << 17 };

test("one generated actor runs past several stocks and keeps every turn and every job", async () => {
  const actor = await host(fixture.run.stocks, heavy);
  try {
    let turns = 0, jobs = 0, last: any;
    while ((last?.probe.successions ?? 0) < fixture.run.stocks - 1) {
      if (turns > 4000) throw new Error("the stocks never ran out");
      last = await actor.turn(); turns += 1;
      if (turns % fixture.run.jobEvery === 0) {
        const job = BigInt(++jobs);
        await actor.start(job);
        for (;;) { const step = await actor.step(job); if (step.status === "done") break; last = await actor.turn(); turns += 1; }
      }
    }
    expect(last.probe).toEqual({ document: turns, successions: fixture.run.stocks - 1, tenure: BigInt(fixture.run.stocks), jobs: 0 });
    expect(jobs).toBeGreaterThan(1);
    await Promise.all(actor.state.originalReturns.values());
    expect([actor.stock.liveSources, actor.stock.fuelRemaining, actor.stock.retainedRemaining.maximumCopyBytes]).toEqual([1, 0, 0]);
  } finally { await actor.close(); }
}, 60_000);

test("an actor whose job never quiesces answers the typed fault through the worker, and cancelling the job renews it", async () => {
  const actor = await host(2, heavy);
  try {
    await actor.start(1n);
    let turns = 0;
    const fault = await (async () => { for (; turns < 4000; turns += 1) try { await actor.turn(); } catch (error) { return error as Error & { fault?: { code: string; params: Record<string, string> } }; } throw new Error("never exhausted"); })();
    expect(fault.fault).toMatchObject({ code: "plugin.original-stock-exhausted", params: { axis: "maximumCopyBytes" } });
    expect(faultNotice(fault.fault as never, [], "native", "en")?.code).toBe("plugin.original-stock-exhausted");
    expect(faultNotice(fault.fault as never, [], "native", "de")?.text).toContain("Arbeitsbudget");
    const stepped = await actor.step(1n).then(() => undefined, (error: Error) => error);
    expect(stepped?.message).toContain("plugin.original-stock-exhausted");
    await actor.cancel(1n);
    const renewed = await actor.turn();
    expect(renewed.probe).toEqual({ document: turns + 1, successions: 1, tenure: 2n, jobs: 0 });
  } finally { await actor.close(); }
}, 60_000);

test("a job the guest refuses to start does not hold back the renewal", async () => {
  const actor = await host(2, heavy);
  try {
    const refused = await actor.start(13n).then(() => undefined, (error: Error) => error);
    expect(refused?.message).toContain("original-job.input-refused");
    let turns = 0, last: any;
    for (; (last?.probe.successions ?? 0) < 1; turns += 1) { if (turns > 4000) throw new Error("never renewed"); last = await actor.turn(); }
    expect(last.probe).toEqual({ document: turns, successions: 1, tenure: 2n, jobs: 0 });
  } finally { await actor.close(); }
}, 60_000);

test("an actor that ran a job closes its context on retirement and returns its whole source to the stock", async () => {
  const actor = await host(fixture.run.stocks, heavy);
  try {
    await actor.turn();
    for (let job = 1n; job <= BigInt(fixture.retirement.jobs); job += 1n) {
      await actor.start(job);
      for (;;) if ((await actor.step(job)).status === "done") break;
    }
    expect(actor.stock.liveSources).toBe(1);
    await actor.retire();
    expect(actor.stock.liveSources).toBe(fixture.retirement.liveSourcesAfterClose);
    expect(actor.state.originalClosures.size).toBe(0);
  } finally { await actor.close(); }
}, 60_000);
