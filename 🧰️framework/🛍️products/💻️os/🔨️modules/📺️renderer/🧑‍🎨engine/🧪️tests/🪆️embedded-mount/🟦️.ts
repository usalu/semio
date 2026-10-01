import bootLifecycle from "../../🧫️fixtures/⏳️boot-lifecycle/🔣️.json";
import bootLifecycleSchema from "../../🧬️schema/⏳️boot-lifecycle/🔣️.json";
// @vitest-environment jsdom

import reactShellSource from "../../🧱️elements/🐚️Shell/🟦️.tsx?raw";
import ts from "typescript";
import introductionFixture from "../../🧫️fixtures/🎓️host-introduction/🔣️.json";
import introductionSchema from "../../🧬️schema/🎓️host-introduction/🔣️.json";
import Ajv from "ajv/dist/2020";
import { createElement } from "react";
import graphlib from "graphlib";
import { createRoot } from "react-dom/client";
import { afterEach, describe, expect, it, vi } from "vitest";
import { PlaygroundBootPlanner, expandPluginRegistry, orderPluginRegistryEntries, type PluginCatalog } from "@semio-tech/framework";
import { admitWgpuPluginModules, prepareWgpuPluginModules, assertWgpuPluginPlan } from "../../🎯️targets/🧊️wgpu/🧩️plugin-modules/🟦️.ts";
import pluginModulesFixture from "../../🧫️fixtures/🧩️plugin-modules/🔣️.json";
import pluginModulesSchema from "../../🧬️schema/🧩️plugin-modules/🔣️.json";
import fixture from "../../🧫️fixtures/🪆️embedded-mount/🔣️.json";
import schema from "../../🧬️schema/🪆️embedded-mount/🔣️.json";
import { bootFrameworkOsWgpu } from "../../🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts";

vi.mock("@semio-tech/assets", () => ({ ICON_NAMES: [], ICONS: {} }));

class MountWorker {
  static owners: MountWorker[] = [];
  static holdClose = false;
  static holdBoot = false;
  static rejectClose = false;
  onmessage: ((event: MessageEvent) => void) | null = null;
  onmessageerror: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  messages: any[] = [];
  terminated = false;

  constructor(readonly url: string, readonly options: WorkerOptions) {
    MountWorker.owners.push(this);
  }

  postMessage(message: any): void {
    this.messages.push(message);
    if (message.kind === "boot") queueMicrotask(() => {
      for (const event of bootLifecycle.progress) this.onmessage?.({ data: { ...event, kind: "boot-progress", lifecycle: message.lifecycle, worker: { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" } } } as MessageEvent);
      if (!MountWorker.holdBoot) this.acknowledgeBoot();
    });
    if (message.kind === "close" && MountWorker.rejectClose) throw new Error("closed worker channel");
    if (message.kind === "close" && !MountWorker.holdClose) queueMicrotask(() => this.onmessage?.({ data: { kind: "closed", lifecycle: message.lifecycle } } as MessageEvent));
    if (message.kind === "introspect") queueMicrotask(() => this.onmessage?.({ data: { ...message, kind: "introspection", json: JSON.stringify({ windows: [], owner: this.messages.find(message => message.kind === "boot")?.descriptor.pluginVariant }) } } as MessageEvent));
  }

  acknowledgeBoot(): void {
    const message = this.messages.find(message => message.kind === "boot");
    this.onmessage?.({ data: { kind: "booted", lifecycle: message.lifecycle } } as MessageEvent);
  }

  acknowledgeClose(): void {
    const message = this.messages.find(message => message.kind === "close");
    this.onmessage?.({ data: { kind: "closed", lifecycle: message.lifecycle } } as MessageEvent);
  }

  terminate(): void {
    this.terminated = true;
  }
}

const cleanups: (() => void | Promise<void>)[] = [];

afterEach(async () => {
  for (const cleanup of cleanups.splice(0)) await cleanup();
  document.body.replaceChildren();
  MountWorker.owners = [];
  MountWorker.holdClose = false;
  MountWorker.holdBoot = false;
  MountWorker.rejectClose = false;
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

function mountRoots() {
  return fixture.roots.map(row => {
    const root = document.createElement("main");
    root.id = row.id;
    Object.defineProperties(root, { clientWidth: { value: row.width }, clientHeight: { value: row.height } });
    document.body.append(root);
    return root;
  });
}

function installBrowserRuntime() {
  vi.stubGlobal("Worker", MountWorker);
  vi.stubGlobal("ResizeObserver", class { observe() {} disconnect() {} });
  vi.stubGlobal("Image", class { onload?: () => void; set src(_value: string) { queueMicrotask(() => this.onload?.()); } });
  vi.stubGlobal("ImageData", class { data: Uint8ClampedArray; constructor(readonly width: number, readonly height: number) { this.data = new Uint8ClampedArray(width * height * 4); } });
  Object.defineProperty(HTMLCanvasElement.prototype, "transferControlToOffscreen", { configurable: true, value: () => ({}) });
  vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue({ clearRect() {}, drawImage() {}, getImageData: () => ({ width: 24, height: 24, data: new Uint8ClampedArray(24 * 24 * 4) }) } as any);
}

describe("embedded WGPU mount lifecycle", () => {
  it("validates the neutral independent-owner lifecycle", () => {
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("keeps independent DOM roots and retirement equal to React's root oracle", async () => {
    const roots = mountRoots();
    const oracle = roots.map((root, index) => {
      const owner = createRoot(root);
      owner.render(createElement("canvas", { "data-owner": fixture.roots[index]!.id }));
      return owner;
    });
    await vi.waitFor(() => expect(document.querySelectorAll("canvas")).toHaveLength(fixture.expected.canvases));
    oracle[0]!.unmount();
    expect(roots[0]!.childElementCount).toBe(0);
    expect(roots[1]!.querySelector("canvas")?.dataset.owner).toBe(fixture.expected.remainingRoot);
    oracle[1]!.unmount();
  });

  it("boots through isolated frame Workers and disposes only its own owner", async () => {
    installBrowserRuntime();
    const roots = mountRoots();
    const disposers: (() => Promise<void>)[] = [];
    for (const row of fixture.roots) {
      const dispose = await bootFrameworkOsWgpu({
        rootId: row.id, plugin: row.variant, locks: { locale: row.locale },
        rendererModuleUrl: "data:text/javascript,export default async function(){};export function semioWgpuWorkerBootstrap(){}",
      });
      disposers.push(dispose);
      cleanups.push(dispose);
    }
    expect(MountWorker.owners).toHaveLength(fixture.expected.workers);
    expect(document.querySelectorAll("canvas")).toHaveLength(fixture.expected.canvases);
    expect(new Set([...document.querySelectorAll("[id]")].map(node => node.id)).size).toBe(document.querySelectorAll("[id]").length);
    for (const [index, owner] of MountWorker.owners.entries()) expect(owner.messages.find(message => message.kind === "boot").descriptor.pluginVariant).toBe(fixture.roots[index]!.variant);
    await disposers[0]!();
    expect(MountWorker.owners.map(owner => owner.terminated)).toEqual(fixture.expected.workerStoppedAfterFirstDispose);
    expect(roots[0]!.childElementCount).toBe(0);
    expect(roots[1]!.querySelector("canvas")).not.toBeNull();
    await disposers[0]!();
    expect(MountWorker.owners[1]!.terminated).toBe(false);
    await disposers[1]!();
    expect(MountWorker.owners.every(owner => owner.terminated)).toBe(fixture.expected.allStoppedAfterFinalDispose);
    expect(document.querySelectorAll("canvas")).toHaveLength(0);
  });

  it("retires descendant shards and refuses late shard creation after its owner closes", async () => {
    installBrowserRuntime();
    mountRoots();
    const dispose = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id });
    cleanups.push(dispose);
    const frame = MountWorker.owners[0]!;
    frame.onmessage?.({ data: { kind: "shard-spawn", shardIndex: 0, url: "https://example.test/shard.js" } } as MessageEvent);
    const shard = MountWorker.owners[1]!;
    await dispose();
    const stopped = shard.terminated;
    frame.onmessage?.({ data: { kind: "shard-spawn", shardIndex: 1, url: "https://example.test/shard.js" } } as MessageEvent);
    const lateIgnored = MountWorker.owners.length === 2;
    expect(stopped).toBe(fixture.expected.descendantWorkerRetired);
    expect(lateIgnored).toBe(fixture.expected.lateWorkerSpawnIgnored);
  });

  it("exposes accepted introspection through its own mount and refuses retired owners", async () => {
    installBrowserRuntime();
    mountRoots();
    const first = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id, plugin: fixture.roots[0]!.variant });
    const second = await bootFrameworkOsWgpu({ rootId: fixture.roots[1]!.id, plugin: fixture.roots[1]!.variant });
    cleanups.push(first, second);
    expect(JSON.parse(await first.introspection.dumpChrome()).owner).toBe(fixture.roots[0]!.variant);
    expect(JSON.parse(await second.introspection.dumpChrome()).owner).toBe(fixture.roots[1]!.variant);
    await first();
    await expect(first.introspection.dumpChrome()).rejects.toThrow("wgpu-mount-retired");
    expect(JSON.parse(await second.introspection.dumpChrome()).owner).toBe(fixture.roots[1]!.variant);
  });

  it("makes every disposal caller wait for the same Worker retirement", async () => {
    installBrowserRuntime();
    mountRoots();
    const dispose = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id });
    cleanups.push(dispose);
    MountWorker.holdClose = true;
    let firstDone = false;
    let secondDone = false;
    const first = dispose().then(() => { firstDone = true; });
    const second = dispose().then(() => { secondDone = true; });
    await Promise.resolve();
    await Promise.resolve();
    const waited = !firstDone && !secondDone;
    MountWorker.owners[0]!.acknowledgeClose();
    await Promise.all([first, second]);
    expect(waited).toBe(fixture.expected.retirementWaitsForWorker);
    expect(MountWorker.owners[0]!.terminated).toBe(true);
  });

  it("retires the previous root owner before installing its replacement", async () => {
    installBrowserRuntime();
    const roots = mountRoots();
    const first = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id });
    cleanups.push(first);
    const second = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id, plugin: fixture.roots[1]!.variant });
    cleanups.push(second);
    expect(MountWorker.owners[0]!.terminated).toBe(fixture.expected.replacementRetiresPrevious);
    expect(MountWorker.owners[1]!.terminated).toBe(false);
    await first();
    expect(roots[0]!.querySelector("canvas")).not.toBeNull();
    expect(MountWorker.owners[1]!.terminated).toBe(false);
  });

  it.each(["error", "unreachable-close"] as const)("settles retirement when the Worker is %s", async failure => {
    installBrowserRuntime();
    mountRoots();
    const dispose = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id });
    cleanups.push(dispose);
    const owner = MountWorker.owners[0]!;
    if (failure === "error") {
      MountWorker.holdClose = true;
      vi.spyOn(console, "error").mockImplementation(() => {});
      owner.onerror?.({ message: "Worker stopped" } as ErrorEvent);
    } else MountWorker.rejectClose = true;
    const retirement = dispose();
    await Promise.resolve();
    const stopped = owner.terminated;
    if (!stopped) owner.acknowledgeClose();
    await retirement;
    expect(stopped).toBe(fixture.expected.crashedWorkerRetires);
  });

});


describe("embedded WGPU introduction policy", () => {
  it("matches the neutral first-frame policy against the authored React predicate", () => {
    const validate = new Ajv({ strict: true }).compile(introductionSchema);
    expect(validate(introductionFixture), JSON.stringify(validate.errors)).toBe(true);
    const source = reactShellSource;
    const tree = ts.createSourceFile("Shell.tsx", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
    const declaration = tree.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "shouldAutoStartIntroduction")!;
    const module = ts.transpileModule(declaration.getText(tree), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText;
    const oracle = new Function("exports", module + "; return exports.shouldAutoStartIntroduction;")({});
    for (const row of introductionFixture.cases) expect(oracle({ ...row, suppressed: row.iframe || row.suppressed, dismissedAppIds: new Set() }), row.id).toBe(row.expectedAutoStart);
  });

  it("carries initial and live suppression to only its owning frame Worker", async () => {
    installBrowserRuntime();
    mountRoots();
    const first = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id, suppressAutoIntroduction: true } as any);
    const second = await bootFrameworkOsWgpu({ rootId: fixture.roots[1]!.id, suppressAutoIntroduction: false } as any);
    cleanups.push(first, second);
    expect(MountWorker.owners.map(owner => owner.messages.find(message => message.kind === "boot").introductionSuppressed)).toEqual([true, false]);
    (first as any).setIntroductionSuppressed(false);
    expect(MountWorker.owners[0]!.messages.find(message => message.kind === "host-introduction-policy").suppressed).toBe(false);
    expect(MountWorker.owners[1]!.messages.some(message => message.kind === "host-introduction-policy")).toBe(false);
  });
});


describe("explicit embedded plugin registry", () => {
  it("validates neutral bounded module inputs through the independent schema oracle", () => {
    const validate = new Ajv({ strict: true }).addKeyword("x-semio-admission").compile(pluginModulesSchema);
    expect(validate(pluginModulesFixture.valid), JSON.stringify(validate.errors)).toBe(true);
    for (const invalid of pluginModulesFixture.invalid) expect(validate(invalid)).toBe(false);
    expect(validate(Array.from({ length: 257 }, () => pluginModulesFixture.valid[0]))).toBe(false);
  });

  it("carries explicit module URLs to its owned Worker without catalogue substitution", async () => {
    installBrowserRuntime();
    mountRoots();
    const plugins = pluginModulesFixture.valid;
    const dispose = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id, plugin: "custom.host", plugins });
    cleanups.push(dispose);
    const boot = MountWorker.owners[0]!.messages.find(message => message.kind === "boot");
    expect(boot.plugins).toEqual(plugins.map(row => ({ ...row, moduleUrl: new URL(row.moduleUrl, window.location.href).href })));
  });
});


const emptyPluginCatalog: PluginCatalog = { plugins: [], extensions: [], hosts: [], playgrounds: [], moduleUrl: () => { throw new Error("outside supplied registry"); }, extensionModuleUrl: () => { throw new Error("outside supplied registry"); } };

function describedModule(id: string, dependencies: readonly { readonly pluginId: string; readonly version?: string }[] = [], version = "1.0.0") {
  return { packageId: id, componentSha256: "a".repeat(64), manifest: { pluginId: id, label: id, version, apps: [], workflows: [], examples: [], dependencies } };
}

describe("explicit embedded plugin registry admission", () => {
  it("agrees with Ajv and the independent DOM URL parser on bounded input vectors", () => {
    const validate = new Ajv({ strict: true }).addKeyword("x-semio-admission").compile(pluginModulesSchema);
    const admitted = admitWgpuPluginModules(pluginModulesFixture.valid, pluginModulesFixture.baseUrl);
    expect(validate(admitted)).toBe(true);
    for (let index = 0; index < admitted.length; index++) {
      const document = window.document.implementation.createHTMLDocument();
      const base = document.createElement("base");
      base.href = pluginModulesFixture.baseUrl;
      document.head.append(base);
      const anchor = document.createElement("a");
      anchor.href = pluginModulesFixture.valid[index]!.moduleUrl;
      expect(admitted[index]!.moduleUrl).toBe(anchor.href);
    }
    for (const rows of [...pluginModulesFixture.invalid, ...pluginModulesFixture.semanticInvalid]) expect(() => admitWgpuPluginModules(rows, pluginModulesFixture.baseUrl)).toThrow();
    expect(admitWgpuPluginModules([], pluginModulesFixture.baseUrl)).toEqual([]);
    expect(() => admitWgpuPluginModules(Array.from({ length: 257 }, (_, index) => ({ pluginId: `plugin${index}`, moduleUrl: `https://example.test/${index}.js` })), pluginModulesFixture.baseUrl)).toThrow();
    const bytes = Array.from({ length: 32 }, (_, index) => ({ pluginId: `plugin${index}`, moduleUrl: `https://example.test/${index}/${"a".repeat(4060)}.js` }));
    expect(() => admitWgpuPluginModules(bytes, pluginModulesFixture.baseUrl)).toThrow("module bytes");
  });

  it("selects only supplied custom modules with the React registry and graphlib dependency oracles", async () => {
    for (const row of pluginModulesFixture.selection) {
      const modules = admitWgpuPluginModules(row.rows.map(({ pluginId, moduleUrl }) => ({ pluginId, moduleUrl })), pluginModulesFixture.baseUrl);
      const prepared = await prepareWgpuPluginModules(emptyPluginCatalog, modules, { readDescriptor: async id => describedModule(id, row.rows.find(module => module.pluginId === id)?.dependencies ?? []) });
      const planner = new PlaygroundBootPlanner(prepared.catalog, row.variant, undefined, "selection" in row ? row.selection as "all" : "variant");
      while (planner.step()) {}
      const plan = planner.finish();
      const react = expandPluginRegistry(row.rows, "selection" in row && row.selection === "all" ? undefined : row.variant);
      const graph = new graphlib.Graph({ directed: true });
      for (const module of react) graph.setNode(module.pluginId);
      for (const module of react) for (const edge of module.dependencies ?? []) graph.setEdge(edge.pluginId, module.pluginId);
      const missing = react.some(module => (module.dependencies ?? []).some(edge => !react.some(other => other.pluginId === edge.pluginId)));
      const oracleRefused = react.length === 0 || missing || !graphlib.alg.isAcyclic(graph);
      expect(oracleRefused).toBe("refused" in row && row.refused);
      if (oracleRefused) expect(() => assertWgpuPluginPlan(plan, prepared)).toThrow();
      else {
        assertWgpuPluginPlan(plan, prepared);
        expect(plan.plugins.map(module => module.pluginId)).toEqual(row.expected);
        expect(graphlib.alg.topsort(graph)).toEqual(row.expected);
        expect(orderPluginRegistryEntries(react).order.map(module => module.pluginId)).toEqual(row.expected);
      }
    }
  });

  it("refuses foreign descriptors and exact dependency version mismatches before actors", async () => {
    const modules = admitWgpuPluginModules(pluginModulesFixture.valid, pluginModulesFixture.baseUrl);
    const ajv = new Ajv({ strict: true }).addKeyword("x-semio-admission");
    ajv.addSchema(pluginModulesSchema);
    const validateAuthority = ajv.compile({ $ref: `${pluginModulesSchema.$id}#/$defs/descriptorAuthority` });
    for (const row of pluginModulesFixture.authority) {
      const descriptor = describedModule(row.received, row.dependencies);
      const structural = validateAuthority(descriptor.manifest);
      const duplicate = new Set(row.dependencies.map(edge => edge.pluginId)).size !== row.dependencies.length;
      expect(!structural || duplicate || row.received !== row.requested).toBe(row.refused);
      const admission = prepareWgpuPluginModules(emptyPluginCatalog, [{ pluginId: row.requested, moduleUrl: modules[0]!.moduleUrl }], { readDescriptor: async () => descriptor });
      if (row.refused) await expect(admission).rejects.toThrow();
      else expect((await admission).packages.get(row.requested)?.manifest.pluginId).toBe(row.requested);
    }
    const prepared = await prepareWgpuPluginModules(emptyPluginCatalog, modules, { readDescriptor: async id => describedModule(id, id === "custom.host" ? [{ pluginId: "custom.core", version: "=2.0.0" }] : []) });
    expect(() => assertWgpuPluginPlan(new PlaygroundBootPlanner(prepared.catalog, "custom.host").finish(), prepared)).toThrow("descriptor dependency graph");
    await expect(prepareWgpuPluginModules(emptyPluginCatalog, modules, { readDescriptor: async id => describedModule(id, [{ pluginId: "custom.core", version: "^1" }]) })).rejects.toThrow("dependency version");
    await expect(prepareWgpuPluginModules(emptyPluginCatalog, modules, { readDescriptor: async id => describedModule(id, [{ pluginId: "custom.core" }, { pluginId: "custom.core" }]) })).rejects.toThrow("dependency identity");
  });

  it("withdraws descriptor admission at a turn boundary before reading or starting actors", async () => {
    const abort = new AbortController();
    let reads = 0;
    await expect(prepareWgpuPluginModules(emptyPluginCatalog, admitWgpuPluginModules(pluginModulesFixture.valid, pluginModulesFixture.baseUrl), { signal: abort.signal, yieldTurn: async () => abort.abort(), readDescriptor: async id => { reads++; return describedModule(id); } })).rejects.toThrow();
    expect(reads).toBe(0);
  });

  it("refuses an invalid replacement registry before retiring its existing owner", async () => {
    installBrowserRuntime();
    mountRoots();
    const dispose = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id });
    cleanups.push(dispose);
    const worker = MountWorker.owners[0]!;
    await expect(bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id, plugins: pluginModulesFixture.semanticInvalid[0] })).rejects.toThrow();
    expect(MountWorker.owners).toHaveLength(1);
    expect(worker.messages.some(message => message.kind === "close")).toBe(false);
  });
});


it("matches neutral public React registry omission and unfiltered primary selection", async () => {
  installBrowserRuntime();
  mountRoots();
  for (const row of pluginModulesFixture.libraryDefaults) {
    const dispose = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id, plugin: "plugin" in row ? row.plugin : undefined, plugins: row.supplied ? pluginModulesFixture.valid : undefined });
    cleanups.push(dispose);
    const boot = MountWorker.owners.at(-1)!.messages.find(message => message.kind === "boot");
    expect(boot.plugins).toHaveLength(row.expectedCount);
    expect(boot.descriptor.pluginVariant).toBe(row.expectedPrimary);
    expect(boot.pluginRegistrySelection).toBe(row.expectedSelection);
    await dispose();
  }
});


const browserTurn = () => new Promise<void>(resolve => setTimeout(resolve, 0));

describe("public WGPU boot progress and cancellation", () => {
  it("validates neutral lifecycle and bounded progress with strict Ajv", () => {
    const validate = new Ajv({ strict: true }).addKeyword("x-semio-lifecycle").compile(bootLifecycleSchema);
    expect(validate(bootLifecycle), JSON.stringify(validate.errors)).toBe(true);
    for (const progress of [-1, 2, NaN, Infinity]) expect(validate({ ...bootLifecycle, progress: [{ stage: "boot", progress }] })).toBe(false);
  });

  it("refuses pre-aborted ownership before touching DOM or allocating a Worker", async () => {
    installBrowserRuntime();
    const root = mountRoots()[0]!;
    root.textContent = "existing owned DOM";
    const abort = new AbortController();
    abort.abort();
    const outcome = await bootFrameworkOsWgpu({ rootId: root.id, signal: abort.signal }).then(dispose => { cleanups.push(dispose); return undefined; }, error => error);
    expect(outcome?.name).toBe("AbortError");
    expect(MountWorker.owners).toHaveLength(bootLifecycle.cancellation[0]!.expectedWorkers);
    expect(root.textContent).toBe("existing owned DOM");
  });

  it("publishes only bounded owned progress and isolates callback faults", async () => {
    installBrowserRuntime();
    mountRoots();
    const events: { stage: string; progress: number }[] = [];
    const dispose = await bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id, onProgress: event => { events.push({ stage: event.stage, progress: event.progress }); if (event.stage === "plugin-graph") throw new Error("observer fault"); } });
    cleanups.push(dispose);
    expect(events).toEqual(bootLifecycle.progress);
    await dispose();
    const worker = MountWorker.owners[0]!;
    worker.onmessage?.({ data: { kind: "boot-progress", lifecycle: worker.messages[0]!.lifecycle, stage: "retired", progress: 1, worker: {} } } as MessageEvent);
    expect(events).toEqual(bootLifecycle.progress);
  });

  it("refuses out-of-contract Worker progress before publishing to the embedder", async () => {
    installBrowserRuntime();
    mountRoots();
    MountWorker.holdBoot = true;
    const events: { stage: string; progress: number }[] = [];
    const pending = bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id, onProgress: event => events.push(event) });
    await browserTurn();
    const worker = MountWorker.owners[0]!;
    try {
      for (const event of [{ stage: "", progress: 0 }, { stage: "x".repeat(513), progress: 0 }, { stage: "boot", progress: -1 }, { stage: "boot", progress: 2 }, { stage: "boot", progress: NaN }, { stage: "boot", progress: Infinity }]) {
        worker.onmessage?.({ data: { kind: "boot-progress", lifecycle: worker.messages[0]!.lifecycle, ...event, worker: { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" } } } as MessageEvent);
      }
      expect(events.map(({ stage, progress }) => ({ stage, progress }))).toEqual(bootLifecycle.progress);
    } finally {
      MountWorker.holdBoot = false;
      worker.acknowledgeBoot();
      cleanups.push(await pending);
    }
  });

  it("rejects cancelled pending boot only after its owned Worker acknowledges retirement", async () => {
    installBrowserRuntime();
    mountRoots();
    MountWorker.holdBoot = true;
    MountWorker.holdClose = true;
    const abort = new AbortController();
    let settled = false;
    const pending = bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id, signal: abort.signal }).then(dispose => { cleanups.push(dispose); settled = true; return undefined; }, error => { settled = true; return error; });
    await browserTurn();
    const worker = MountWorker.owners[0]!;
    try {
      abort.abort();
      await browserTurn();
      expect(worker.messages.some(message => message.kind === "close")).toBe(true);
      expect(settled).toBe(false);
      worker.acknowledgeClose();
      expect((await pending)?.name).toBe("AbortError");
      expect(worker.terminated).toBe(true);
    } finally {
      MountWorker.holdBoot = false;
      MountWorker.holdClose = false;
      worker.acknowledgeBoot();
      if (worker.messages.some(message => message.kind === "close")) worker.acknowledgeClose();
      await pending;
    }
  });

  it("settles pagehide cancellation after retiring a pending owned Worker", async () => {
    installBrowserRuntime();
    mountRoots();
    MountWorker.holdBoot = true;
    MountWorker.holdClose = true;
    let settled = false;
    let outcome: unknown;
    const pending = bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id }).then(dispose => { cleanups.push(dispose); settled = true; }, error => { outcome = error; settled = true; });
    await browserTurn();
    const worker = MountWorker.owners[0]!;
    try {
      window.dispatchEvent(new Event("pagehide"));
      await browserTurn();
      expect(worker.messages.some(message => message.kind === "close")).toBe(true);
      expect(settled).toBe(false);
      worker.acknowledgeClose();
      await browserTurn();
      expect(settled).toBe(true);
      expect(outcome).toMatchObject({ name: "AbortError" });
      await pending;
    } finally {
      MountWorker.holdBoot = false;
      MountWorker.holdClose = false;
      worker.acknowledgeBoot();
      worker.acknowledgeClose();
    }
  });

  it("withdraws a pending prior owner and cancels queued replacement without creating another Worker", async () => {
    installBrowserRuntime();
    mountRoots();
    MountWorker.holdBoot = true;
    MountWorker.holdClose = true;
    const first = bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id }).then(dispose => { cleanups.push(dispose); return undefined; }, error => error);
    await browserTurn();
    const worker = MountWorker.owners[0]!;
    const abort = new AbortController();
    const second = bootFrameworkOsWgpu({ rootId: fixture.roots[0]!.id, signal: abort.signal }).then(dispose => { cleanups.push(dispose); return undefined; }, error => error);
    try {
      await browserTurn();
      expect(worker.messages.some(message => message.kind === "close")).toBe(true);
      abort.abort();
      expect(MountWorker.owners).toHaveLength(bootLifecycle.cancellation[2]!.expectedWorkers);
      worker.acknowledgeClose();
      expect((await first)?.name).toBe("AbortError");
      expect((await second)?.name).toBe("AbortError");
      expect(MountWorker.owners).toHaveLength(1);
    } finally {
      MountWorker.holdBoot = false;
      MountWorker.holdClose = false;
      worker.acknowledgeBoot();
      if (worker.messages.some(message => message.kind === "close")) worker.acknowledgeClose();
      await first;
      await second;
    }
  });

  it("retires a ready owned root on abort with the React unmount oracle", async () => {
    installBrowserRuntime();
    const root = mountRoots()[0]!;
    const oracle = createRoot(root);
    oracle.render(createElement("canvas"));
    await vi.waitFor(() => expect(root.querySelector("canvas")).not.toBeNull());
    oracle.unmount();
    expect(root.childElementCount).toBe(0);
    const abort = new AbortController();
    const dispose = await bootFrameworkOsWgpu({ rootId: root.id, signal: abort.signal });
    cleanups.push(dispose);
    abort.abort();
    await browserTurn();
    expect(MountWorker.owners[0]!.terminated).toBe(true);
    expect(root.childElementCount).toBe(0);
  });
});
