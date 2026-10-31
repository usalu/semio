/** 🧬️ The browser host lowers every reactor and jobs call exactly as the owned WIT declares it; the WIT itself, read through independent JCO types, is the oracle. */
import { test, expect } from "bun:test";
import { fileURLToPath, pathToFileURL } from "node:url";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { types } from "@bytecodealliance/jco";
import ts from "typescript";
import { APP_CHANNEL_VERSION } from "@semio-tech/framework-os";
import { originalOperationWorkerSource } from "../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts";
import { PLUGIN_HOST_SHIM_FILE } from "../../🧾️contract/🟦️.ts";
import { pluginComponentBridgeSource } from "../../🌉️component-bridge-source/🟦️.ts";
import { browserActorModuleSource } from "../../../🧩️actor-module/🟦️.ts";

type Member = { readonly name: string; readonly optional: boolean; readonly type: string };
type Oracle = { readonly records: Map<string, Member[]>; readonly functions: Map<string, string[]>; readonly files: Map<string, string> };

const retainedTurn = fileURLToPath(new URL("../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts", import.meta.url));

async function witOracle(): Promise<Oracle> {
  const generated = await types(fileURLToPath(new URL("../../../../🧬️schema", import.meta.url)), { name: "actor-contract", worldName: "actor", instantiation: "async", asyncMode: "jspi" });
  const files = new Map<string, string>(Object.entries(generated).map(([name, bytes]) => [name, new TextDecoder().decode(bytes)]));
  const records = new Map<string, Member[]>();
  const functions = new Map<string, string[]>();
  for (const name of ["reactor", "jobs"]) {
    const source = ts.createSourceFile(name + ".d.ts", files.get(`interfaces/semio-framework-${name}.d.ts`)!, ts.ScriptTarget.Latest, true);
    for (const node of source.statements) {
      if (ts.isInterfaceDeclaration(node)) records.set(node.name.text, node.members.filter(ts.isPropertySignature).map((member) => ({ name: member.name.getText(source), optional: member.questionToken !== undefined, type: member.type!.getText(source) })));
      if (ts.isFunctionDeclaration(node)) functions.set(`${name}.${node.name!.text}`, node.parameters.map((parameter) => parameter.type!.getText(source)));
    }
  }
  return { records, functions, files };
}

function conforms(oracle: Oracle, type: string, value: unknown, path: string): void {
  const base = type.replace(/\s*\|\s*undefined$/, "");
  if (base !== type && value === undefined) return;
  if (value === undefined) throw Error(`${path}: missing ${base}`);
  if (["bigint", "number", "boolean", "string"].includes(base)) {
    if (typeof value !== base) throw Error(`${path}: ${typeof value} is not ${base}`);
    return;
  }
  if (base === "Uint8Array") {
    if (!(value instanceof Uint8Array)) throw Error(`${path}: not Uint8Array`);
    return;
  }
  const array = /^Array<(.+)>$/.exec(base);
  if (array) {
    if (!Array.isArray(value)) throw Error(`${path}: not an array`);
    value.forEach((item, index) => conforms(oracle, array[1]!, item, `${path}[${index}]`));
    return;
  }
  const members = oracle.records.get(base);
  if (!members) return;
  if (typeof value !== "object" || value === null) throw Error(`${path}: not a ${base} record`);
  const names = new Set(members.map((member) => member.name));
  for (const key of Object.keys(value)) if (!names.has(key)) throw Error(`${path}: ${key} is not a field of ${base}`);
  for (const member of members) conforms(oracle, member.optional ? `${member.type} | undefined` : member.type, (value as Record<string, unknown>)[member.name], `${path}.${member.name}`);
}

function conformsCall(oracle: Oracle, call: readonly [string, string, readonly unknown[]]): void {
  const [owner, name, args] = call;
  const parameters = oracle.functions.get(`${owner}.${name}`);
  if (!parameters) throw Error(`${owner}.${name} is not a WIT export`);
  if (args.length !== parameters.length) throw Error(`${owner}.${name}: ${args.length} arguments for ${parameters.length} parameters`);
  parameters.forEach((type, index) => conforms(oracle, type, args[index], `${owner}.${name}#${index}`));
}

const guestSource = (version: number) => `
const calls = (globalThis.__witCalls ??= []);
const zero = { copiedItems: 0n, copiedBytes: 0n, retainedCapacityBytes: 0n, releasedBytes: 0n };
const record = (owner, name, args) => calls.push([owner, name, args]);
export const reactor = {
  channelVersion: async () => ${version},
  stageCommandPage: async (cursor, bytes) => record("reactor", "stageCommandPage", [cursor, bytes]),
  stageColdPairPage: async (page) => record("reactor", "stageColdPairPage", [page]),
  admitActorContext: async (input, policy) => { record("reactor", "admitActorContext", [input, policy]); return { input, spent: zero, workUnitsSpent: 0n, ready: true }; },
  closeActorContext: async (input, policy) => { record("reactor", "closeActorContext", [input, policy]); return { input, spent: { ...zero, copiedItems: 1n }, workUnitsSpent: 0n, complete: true, blocked: false }; },
  poll: async (events, budget) => {
    record("reactor", "poll", [events, budget]);
    const cursor = { owner: 0n, generation: 0n, commandIndex: 0, commandCount: 0, instance: 0, seq: 0n, kind: 0, pageIndex: 0, pageCount: 0, itemCount: 0, metadata: 0 };
    return { retainedReceipt: { input: budget.retained, spent: zero, workUnitsSpent: 0n, remaining: budget.retained.grant, remainingFuel: budget.retained.work.fuel }, uiPatches: [], effects: [], presence: [], nextWake: { tag: "none" }, status: { tag: "idle" }, fuelUsed: 0n, commandIngress: { kind: 0, cursor, fault: new Uint8Array() }, coldPairIngress: { tag: "idle" }, lifecycleReceipt: { tag: "none" }, uiPatchReceipt: { tag: "none" } };
  },
};
const call = (input, extra) => ({ input, spent: zero, workUnitsSpent: 0n, ...extra });
export const jobs = {
  startJob: async (source, budget) => { record("jobs", "startJob", [source, budget]); return call(budget.input, { admitted: true }); },
  stepJob: async (job, budget) => { record("jobs", "stepJob", [job, budget]); return call(budget.input, { sourceClosed: false, outcome: { tag: "some", val: { job, operation: budget.input.operation, generation: budget.input.generation, state: "done", byteLength: 3n } } }); },
  readJobOutput: async (job, budget) => { record("jobs", "readJobOutput", [job, budget]); return call(budget.input, { sourceClosed: false, chunk: { tag: "some", val: { offset: 0n, bytes: { tag: "some", val: new Uint8Array([1, 2, 3]) }, outputClosed: true } } }); },
  acknowledgeJobOutput: async (job, budget) => { record("jobs", "acknowledgeJobOutput", [job, budget]); return call(budget.input, { acknowledged: true, sourceClosed: false }); },
  cancelJob: async (job, budget) => { record("jobs", "cancelJob", [job, budget]); return call(budget.input, { admitted: true }); },
  takeSegmentedDownloadChunk: async () => ({ tag: "none" }),
};
export const checkpoint = {};
export const describe = {};
`;

const SHAPE = { maxEffects: 1, maxPatchBytes: 2, maxFrames: 3 };
const zeroProgress = { copiedItems: 0n, copiedBytes: 0n, retainedCapacityBytes: 0n, releasedBytes: 0n };
const wireReceipt = (budget: any) => ({ input: budget.retained, spent: zeroProgress, workUnitsSpent: 0n, remaining: budget.retained.grant, remainingFuel: budget.retained.work.fuel });

test("the generated component bridge calls the reactor and jobs exports with exactly the WIT parameters", async () => {
  const oracle = await witOracle();
  const directory = mkdtempSync(join(tmpdir(), "semio-wit-lowering-"));
  const calls: [string, string, unknown[]][] = [];
  Object.assign(globalThis, { __witCalls: calls });
  try {
    writeFileSync(join(directory, "guest.js"), guestSource(APP_CHANNEL_VERSION));
    writeFileSync(join(directory, PLUGIN_HOST_SHIM_FILE), "export function __resolveEffect() {}\nexport function __rejectEffect() {}\nexport function __bindOriginalClock() {}\nexport function __checkoutOriginal() {}\nexport function __returnOriginal() {}\nexport function operationClockUs() { return 5n; }\n");
    writeFileSync(join(directory, "bridge.mjs"), pluginComponentBridgeSource("guest", "guest.wasm"));
    const { createActorApi } = (await import(pathToFileURL(join(directory, "bridge.mjs")).href)) as { createActorApi: (...args: unknown[]) => Promise<Record<string, (...args: unknown[]) => Promise<any>>> };
    const admission = { operation: 3n, generation: 1n, grant: { maximumItems: 64, maximumCopyBytes: 4096, maximumCapacityBytes: 4096, maximumReleaseBytes: 4096, maximumDepth: 8 }, fuel: 1000, maximumElapsedUs: 60_000_000n };
    const api = await createActorApi("wit-actor", 1n, admission, () => 5n, { receipt() {}, hold() {} });
    await api.poll!([], undefined, undefined, SHAPE);
    await api.startJob!(7n, "semio.kind", new Uint8Array([9]));
    expect(await api.stepJob!(7n, { fuel: 1n, deadlineMs: 5 })).toEqual({ status: "done", value: new Uint8Array([1, 2, 3]) });
    await api.cancelJob!(7n);
    await api.closeOriginal!();
    expect(calls.map(([owner, name]) => `${owner}.${name}`)).toEqual(["reactor.admitActorContext", "reactor.poll", "jobs.startJob", "jobs.stepJob", "jobs.readJobOutput", "jobs.acknowledgeJobOutput", "jobs.cancelJob", "reactor.closeActorContext"]);
    for (const call of calls) conformsCall(oracle, call);
    const poll = calls.find(([, name]) => name === "poll")![2][1] as any;
    expect(poll.retained.work).toEqual({ clockBinding: 3n, absoluteDeadlineUs: 60_000_005n, fuel: 1000n });
    expect(poll.retainedPolicy).toEqual(poll.retained.grant);
    expect(poll.workFuel).toBe(1000n);
    const start = calls.find(([, name]) => name === "startJob")![2] as any[];
    expect(start[0]).toEqual({ job: 7n, kind: "semio.kind", input: new Uint8Array([9]) });
  } finally {
    delete (globalThis as Record<string, unknown>).__witCalls;
    rmSync(directory, { recursive: true, force: true });
  }
}, 30_000);

test("the document browser actor driver issues its own turns and job crossings with exactly the WIT parameters", async () => {
  const oracle = await witOracle();
  const calls: [string, string, unknown[]][] = [];
  const directory = mkdtempSync(join(tmpdir(), "semio-wit-lowering-"));
  Object.assign(globalThis, { __witCalls: calls });
  try {
    writeFileSync(join(directory, "guest.mjs"), guestSource(APP_CHANNEL_VERSION));
    const guest = await import(pathToFileURL(join(directory, "guest.mjs")).href);
    const { OriginalSelfIssuedTurns } = new Function(originalOperationWorkerSource() + ";return {OriginalSelfIssuedTurns}")() as { OriginalSelfIssuedTurns: new (guest: unknown, clock: () => bigint) => { poll(events: unknown[], shape: unknown): Promise<unknown>; startJob(...args: unknown[]): Promise<unknown>; stepJob(...args: unknown[]): Promise<unknown>; cancelJob(...args: unknown[]): Promise<unknown> } };
    let now = 10n;
    const turns = new OriginalSelfIssuedTurns(guest, () => now++);
    await turns.poll([], SHAPE);
    await turns.poll([{ tag: "wake" }], SHAPE);
    await expect(turns.poll([{ tag: "wake" }, { tag: "wake" }], SHAPE)).rejects.toThrow("a reactor turn admits at most one event");
    await turns.startJob(1n, "kind", new Uint8Array());
    expect(await turns.stepJob(1n, { deadlineMs: 2 })).toEqual({ status: "done", value: new Uint8Array([1, 2, 3]) });
    await turns.cancelJob(1n);
    expect(calls.map(([owner, name]) => `${owner}.${name}`)).toEqual(["reactor.admitActorContext", "reactor.poll", "reactor.poll", "jobs.startJob", "jobs.stepJob", "jobs.readJobOutput", "jobs.acknowledgeJobOutput", "jobs.cancelJob"]);
    for (const call of calls) conformsCall(oracle, call);
    const epochs = calls.filter(([, name]) => name === "poll").map(([, , args]) => (args[1] as any).retained.epoch);
    expect(epochs).toEqual([2n, 3n]);
  } finally {
    delete (globalThis as Record<string, unknown>).__witCalls;
    rmSync(directory, { recursive: true, force: true });
  }
});

test("the closed document actor module lowers every reactor.poll and jobs invocation through the self-issued driver", async () => {
  const oracle = await witOracle();
  const calls: [string, string, unknown[]][] = [];
  const directory = mkdtempSync(join(tmpdir(), "semio-wit-lowering-"));
  Object.assign(globalThis, { __witCalls: calls });
  try {
    writeFileSync(join(directory, "guest.mjs"), guestSource(APP_CHANNEL_VERSION));
    Object.assign(globalThis, { __witGuest: await import(pathToFileURL(join(directory, "guest.mjs")).href) });
    const stubs = "const createBrowserHostActivation = () => ({ close() {}, invocationFailure: (error) => error, resolveEffect() {}, rejectEffect() {} });\nconst createBrowserWasiActivation = () => ({ close() {} });\n";
    writeFileSync(join(directory, "actor.mjs"), stubs + browserActorModuleSource("async function instantiateFreshComponent() { return globalThis.__witGuest; }", "", false).replace(/^import .*$/gm, ""));
    const { activate } = (await import(pathToFileURL(join(directory, "actor.mjs")).href)) as { activate: (identity: unknown, port: unknown) => Promise<{ invoke(path: string[], args: unknown[]): Promise<any>; close(): Promise<void> }> };
    let now = 100n;
    const actor = await activate({ actorId: "document", activationGeneration: 1n }, { operationClockUs: () => now++ });
    await actor.invoke(["reactor", "poll"], [[], SHAPE]);
    await actor.invoke(["jobs", "startJob"], [3n, "kind", new Uint8Array([4])]);
    expect(await actor.invoke(["jobs", "stepJob"], [3n, { deadlineMs: 4 }])).toEqual({ status: "done", value: new Uint8Array([1, 2, 3]) });
    await actor.invoke(["jobs", "cancelJob"], [3n]);
    await expect(actor.invoke(["reactor", "poll"], [[{ tag: "wake" }, { tag: "wake" }], SHAPE])).rejects.toThrow("a reactor turn admits at most one event");
    await actor.close();
    expect(calls.map(([owner, name]) => `${owner}.${name}`)).toEqual(["reactor.admitActorContext", "reactor.poll", "jobs.startJob", "jobs.stepJob", "jobs.readJobOutput", "jobs.acknowledgeJobOutput", "jobs.cancelJob"]);
    for (const call of calls) conformsCall(oracle, call);
  } finally {
    delete (globalThis as Record<string, unknown>).__witCalls;
    delete (globalThis as Record<string, unknown>).__witGuest;
    rmSync(directory, { recursive: true, force: true });
  }
});

test("the WIT oracle refuses the pre-WIT shapes the browser host used to send", async () => {
  const oracle = await witOracle();
  const grant = { maximumItems: 1n, maximumCopyBytes: 1n, maximumCapacityBytes: 1n, maximumReleaseBytes: 1n, maximumDepth: 1n };
  const legacyInput = { operation: 1n, generation: 1n, epoch: 1n, grant };
  const work = { clockBinding: 1n, absoluteDeadlineUs: 9n, fuel: 1n };
  const input = { ...legacyInput, work };
  const budget = { retained: input, retainedPolicy: grant, workFuel: 1n, fuel: 1n, deadlineUs: 9n, maxEffects: 1, maxPatchBytes: 1, maxFrames: 1 };
  expect(() => conformsCall(oracle, ["reactor", "poll", [[], budget]])).not.toThrow();
  expect(() => conformsCall(oracle, ["reactor", "poll", [[], { ...budget, retained: legacyInput }]])).toThrow("missing");
  const { retainedPolicy, workFuel, ...withoutLoan } = budget;
  expect(() => conformsCall(oracle, ["reactor", "poll", [[], withoutLoan]])).toThrow("missing");
  expect(() => conformsCall(oracle, ["reactor", "poll", [[], { ...budget, deadlineMs: 5 }]])).toThrow("deadlineMs is not a field");
  expect(() => conformsCall(oracle, ["reactor", "admitActorContext", [input]])).toThrow("1 arguments for 2 parameters");
  expect(() => conformsCall(oracle, ["jobs", "startJob", [7n, "kind", new Uint8Array()]])).toThrow("3 arguments for 2 parameters");
  expect(() => conformsCall(oracle, ["jobs", "cancelJob", [7n]])).toThrow("1 arguments for 2 parameters");
  expect(() => conformsCall(oracle, ["jobs", "stepJob", [7n, { fuel: 1n, deadlineMs: 5 }]])).toThrow("is not a field");
  void retainedPolicy; void workFuel;
});

test("the lowering helpers type-check against the declarations JCO derives from the WIT", async () => {
  const oracle = await witOracle();
  const directory = mkdtempSync(join(tmpdir(), "semio-wit-types-"));
  try {
    for (const [name, text] of oracle.files) {
      mkdirSync(dirname(join(directory, name)), { recursive: true });
      writeFileSync(join(directory, name), text);
    }
    const probe = join(directory, "probe.ts");
    const module = JSON.stringify(retainedTurn);
    writeFileSync(probe, `import type { Budget, RetainedGrant, RetainedTurnInput } from "./interfaces/semio-framework-reactor.js";
import type { JobBudget } from "./interfaces/semio-framework-jobs.js";
import { OriginalJobFunds, originalBudgetWit, originalGrantWit, originalTurnWit, type OriginalOperationTurn, type OriginalTurnShape } from ${module};
declare const turn: OriginalOperationTurn;
declare const shape: OriginalTurnShape;
export const input: RetainedTurnInput = originalTurnWit(turn);
export const budget: Budget = originalBudgetWit(turn, shape);
export const grant: RetainedGrant = originalGrantWit(turn.input.grant);
export const job: JobBudget = new OriginalJobFunds(1n, 0n, 1n).crossing();
`);
    const program = ts.createProgram([probe], { strict: true, noEmit: true, skipLibCheck: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, types: [] });
    const diagnostics = ts.getPreEmitDiagnostics(program).filter((diagnostic) => diagnostic.file?.fileName === probe || diagnostic.file?.fileName.startsWith(directory));
    expect(diagnostics.map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"))).toEqual([]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}, 30_000);
