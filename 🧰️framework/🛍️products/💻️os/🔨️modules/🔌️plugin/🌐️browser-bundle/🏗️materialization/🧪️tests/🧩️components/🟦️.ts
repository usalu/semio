/** 🧩️ The generated component bridge of a package of several components imports only the factory of the component an activation
 * names, the first one for an activation that names none, and refuses a component it does not stage, as a package of one component
 * refuses every name; a deployment that stages every declared component in one factory reaches that factory for each of them. The factories are stand-ins that name themselves, so the test reads which one the generated bridge reached.
 * @see ../../🌉️component-bridge-source/🟦️.ts */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { Worker } from "node:worker_threads";
import { shardWorkerSource } from "../../🧵️shard-worker-source/🟦️.ts";
import { monolithicComponentTable, pluginComponentBridgeSource } from "../../🌉️component-bridge-source/🟦️.ts";
import { hostShimSource } from "../../🪢️host-shim-source/🟦️.ts";
import { ShardCoreModules } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🧊️core-modules/🟦️.ts";
import cases from "../../../../../../../../🔨️modules/🎠️kernel/🧩️package-components/🧪️tests/🧫️fixtures/🔣️.json";

const shards = cases.registrations.find(row => Array.isArray(row.components))!;
const base = (id: string) => `${id.replaceAll("-", "_")}_component`;

/** 🏭️ A factory that answers its own base name instead of an instance, so the refusal names the factory the bridge imported. */
function factorySource(name: string): string {
  return `export const wasiImports = {};\nexport async function instantiate() { throw new Error("factory:${name}"); }\n`;
}

/** 🧊️ A factory that asks for two core modules of its own base, as a jco factory does for each core module it instantiates, and then names itself. */
function coreFactorySource(name: string): string {
  return `export const wasiImports = {};\nexport async function instantiate(getCoreModule) { await getCoreModule("${name}.core.wasm"); await getCoreModule("${name}.core2.wasm"); throw new Error("factory:${name}"); }\n`;
}

const EMPTY_MODULE = new Uint8Array([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0]);

/** 🎬️ The refusal one activation's `createActorApi` ends in: the factory it reached, or the bridge's own refusal. */
async function reached(bridge: { createActorApi: (...args: unknown[]) => Promise<unknown> }, component: string | null | undefined): Promise<string> {
  const args: unknown[] = ["actor#1", 1n, {}, () => 0, {}, async () => { throw new Error("core modules are never asked for"); }];
  if (component !== undefined) args.push(component);
  try { await bridge.createActorApi(...args); }
  catch (error) { return (error as Error).message; }
  return "instantiated";
}

test("a bridge of several components imports only the factory of the activation's component", async () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned test outputs");
  mkdirSync(artifacts, { recursive: true });
  const directory = mkdtempSync(join(artifacts, "components-"));
  try {
    const components = shards.components!;
    for (const component of components) writeFileSync(join(directory, `${base(component.id)}.js`), factorySource(base(component.id)));
    writeFileSync(join(directory, "🟨️.js"), hostShimSource());
    writeFileSync(join(directory, "bridge.mjs"), pluginComponentBridgeSource(base(components[0]!.id), Object.fromEntries(components.map(component => [component.id, base(component.id)]))));
    writeFileSync(join(directory, "single.mjs"), pluginComponentBridgeSource(base(components[1]!.id)));
    writeFileSync(join(directory, "monolith.mjs"), pluginComponentBridgeSource(base(components[2]!.id), monolithicComponentTable({ components }, base(components[2]!.id))));
    const bridge = await import(pathToFileURL(join(directory, "bridge.mjs")).href);
    for (const activation of shards.activations!) expect(await reached(bridge, activation.component), String(activation.app)).toBe(`factory:${base(activation.component!)}`);
    expect(await reached(bridge, null)).toBe(`factory:${base(components[0]!.id)}`);
    expect(await reached(bridge, undefined)).toBe(`factory:${base(components[0]!.id)}`);
    expect(await reached(bridge, "semio-hub-stdio-pdf-2-0")).toStartWith("actor-component.unknown");
    const single = await import(pathToFileURL(join(directory, "single.mjs")).href);
    expect(await reached(single, null)).toBe(`factory:${base(components[1]!.id)}`);
    expect(await reached(single, components[0]!.id)).toStartWith("actor-component.unknown");
    const monolith = await import(pathToFileURL(join(directory, "monolith.mjs")).href);
    for (const component of [null, ...components.map(row => row.id)]) expect(await reached(monolith, component), String(component)).toBe(`factory:${base(components[2]!.id)}`);
    expect(monolithicComponentTable({}, "only")).toEqual({});
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("an activation compiles only the core modules of its component, and every instance of that component shares one compiled module", async () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned test outputs");
  mkdirSync(artifacts, { recursive: true });
  const directory = mkdtempSync(join(artifacts, "components-lazy-"));
  try {
    const components = shards.components!;
    for (const component of components) writeFileSync(join(directory, `${base(component.id)}.js`), coreFactorySource(base(component.id)));
    writeFileSync(join(directory, "🟨️.js"), hostShimSource());
    writeFileSync(join(directory, "bridge.mjs"), pluginComponentBridgeSource(base(components[0]!.id), Object.fromEntries(components.map(component => [component.id, base(component.id)]))));
    const bridge = await import(pathToFileURL(join(directory, "bridge.mjs")).href);
    const loader = new ShardCoreModules<WebAssembly.Module>(async () => new WebAssembly.Module(EMPTY_MODULE));
    const requested: string[] = [];
    const open = async (component: string | null, holder: string): Promise<string> => {
      const coreModule = async (url: string) => { requested.push(new URL(url).pathname.split("/").pop()!); return loader.acquire(holder, url); };
      try { await bridge.createActorApi(holder, 1n, {}, () => 0, {}, coreModule, component); } catch (error) { return (error as Error).message; }
      return "instantiated";
    };
    const editors = components[1]!.id;
    expect(requested).toEqual([]);
    expect(await open(editors, "first")).toBe(`factory:${base(editors)}`);
    expect(await open(editors, "second")).toBe(`factory:${base(editors)}`);
    expect(requested).toEqual([`${base(editors)}.core.wasm`, `${base(editors)}.core2.wasm`, `${base(editors)}.core.wasm`, `${base(editors)}.core2.wasm`]);
    expect(loader.compilations).toBe(2);
    expect(loader.seeds().length).toBe(2);
    expect(await open(null, "third")).toBe(`factory:${base(components[0]!.id)}`);
    expect(requested.slice(4)).toEqual([`${base(components[0]!.id)}.core.wasm`, `${base(components[0]!.id)}.core2.wasm`]);
    expect(loader.compilations).toBe(4);
    expect(requested.some(name => name.startsWith(base(components[2]!.id)))).toBe(false);
    expect(loader.release("first")).toEqual([]);
    expect(loader.release("second")).toHaveLength(2);
    expect(loader.seeds().length).toBe(2);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("a generated shard worker loads the component an activation names, fetches and compiles each of its core modules once, and adopts a seed instead of compiling it", async () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned test outputs");
  mkdirSync(artifacts, { recursive: true });
  const directory = mkdtempSync(join(artifacts, "components-worker-"));
  const components = shards.components!;
  const [first, editors, viewers] = components.map(component => component.id);
  const core = (id: string, index: number) => new URL(`./${base(id)}.core${index === 1 ? "" : index}.wasm`, pathToFileURL(join(directory, "bridge.mjs"))).href;
  writeFileSync(join(directory, "bridge.mjs"), `export async function createActorApi(actorId, generation, admission, clock, original, coreModule, component) {
  const base = component.replaceAll("-", "_") + "_component";
  await coreModule(new URL("./" + base + ".core.wasm", import.meta.url).href);
  await coreModule(new URL("./" + base + ".core2.wasm", import.meta.url).href);
  return { component, closeOriginal: async () => ({ operation: admission.operation, generation: admission.generation, sourceClosed: true }), originalSource: admission };
}
`);
  const clocks = "../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/clocks.js";
  const source = shardWorkerSource().replace(JSON.stringify(clocks), JSON.stringify(pathToFileURL(Bun.resolveSync("@bytecodealliance/preview2-shim/clocks", import.meta.dir)).href));
  const worker = new Worker(`const { parentPort } = require('node:worker_threads'); const self = { location: { href: 'https://components-test.invalid/worker.js' }, postMessage: value => parentPort.postMessage(value), addEventListener: (kind, callback) => { if (kind === 'message') parentPort.on('message', data => callback({ data })); } }; WebAssembly.Suspending = function(){}; WebAssembly.promising = function(){}; globalThis.fetch = async url => { parentPort.postMessage({ kind: 'probe-fetch', url }); return {}; }; WebAssembly.compileStreaming = async () => new WebAssembly.Module(new Uint8Array([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0]));\n${source}`, { eval: true });
  const fetched: string[] = [], held: string[] = [], replies = new Map<string, (message: any) => void>();
  worker.on("message", (message: any) => {
    if (message.kind === "probe-fetch") fetched.push(message.url);
    else if (message.kind === "core-module") held.push(message.url);
    else if (message.kind === "result") replies.get(message.requestId)?.(message);
  });
  let generation = 0n;
  const activate = (component: string, coreModules: readonly (readonly [string, WebAssembly.Module])[] = []) => new Promise<any>(resolve => {
    generation += 1n;
    const requestId = `activate-${generation}`;
    replies.set(requestId, resolve);
    worker.postMessage({ kind: "activate", requestId, actorId: `actor-${generation}`, activationGeneration: generation, originalAdmission: { operation: generation, generation: 1n }, moduleUrl: pathToFileURL(join(directory, "bridge.mjs")).href, component, caps: [], budget: {}, assets: [], coreModules });
  });
  try {
    expect((await activate(editors!)).ok).toBe(true);
    expect(fetched).toEqual([core(editors!, 1), core(editors!, 2)]);
    expect((await activate(editors!)).ok).toBe(true);
    expect(fetched).toHaveLength(2);
    expect(held).toEqual([core(editors!, 1), core(editors!, 2), core(editors!, 1), core(editors!, 2)]);
    expect((await activate(first!)).ok).toBe(true);
    expect(fetched.slice(2)).toEqual([core(first!, 1), core(first!, 2)]);
    expect((await activate(viewers!, [[core(viewers!, 1), new WebAssembly.Module(EMPTY_MODULE)]])).ok).toBe(true);
    expect(fetched.slice(4)).toEqual([core(viewers!, 2)]);
    expect(fetched.some(url => url.includes(base(viewers!)) && url.endsWith(`${base(viewers!)}.core.wasm`))).toBe(false);
  } finally {
    await worker.terminate();
    rmSync(directory, { recursive: true, force: true });
  }
}, 60_000);
