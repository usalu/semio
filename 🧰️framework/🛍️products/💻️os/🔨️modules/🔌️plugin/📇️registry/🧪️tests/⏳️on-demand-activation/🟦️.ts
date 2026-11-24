/** ⏳️ Lazy extension activation laws: the boot plan holds the primary plugin and its hard requirements only, a deferred plugin is activated on demand with progress and cancellation, a missing optional plugin never breaks anything, the React and WGPU hosts compute one plan, and the wire shapes agree with an independent schema validator. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { admitOnDemandRequest, expandPluginRegistry, onDemandPluginEntries, onDemandStatusAnnouncement, onDemandStatusLabel, resolveOnDemandOwners, resolvePlaygroundBoot, type OnDemandRequest, type OnDemandStatus, type PluginRegistryEntry } from "@semio-tech/framework-kernel";
import { dependsOnToPluginDependencies } from "@semio-tech/framework-kernel/plugin-registry";
import type { PluginPackageDescriptor } from "@semio-tech/framework-kernel/descriptor-admission";
import { assertWgpuPluginPlan, createWgpuOnDemandActivator, prepareWgpuPluginModules } from "@semio-tech/framework-renderer-wgpu/plugin-modules";
import schema from "../../../../📺️renderer/🧑‍🎨engine/🧬️schema/🧩️plugin-modules/🔣️.json";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { admitPluginCatalogV1, pluginCatalogV1 } from "../../🟦️.ts";
import { parseSemioActivatesOn } from "../../🔎️discovery/🟦️.ts";
import fixture from "../🧫️fixtures/⏳️on-demand-activation/🔣️.json";

const receiptSource = { async read() { throw Error("no package receipt is requested by the lazy activation oracle"); } };
const url = (id: string) => `https://example.test/${id}.js`;
const authority = { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: 30000, now: () => 0, cancelled: () => false, progress: () => {} };
const catalog = () => pluginCatalogV1(admitPluginCatalogV1({
  version: 1, hosts: [],
  targets: fixture.catalog.map(row => ({ ...row, moduleUrl: url(row.pluginId), wasmOut: `${row.pluginId}.wasm`, capabilities: [], ...(row.extends ? { extends: row.extends } : {}) })),
  playgrounds: [{ variant: fixture.variant, pluginId: fixture.primary, aliases: [] }],
}, authority));
const modules = fixture.catalog.map(row => ({ pluginId: row.pluginId, moduleUrl: url(row.pluginId) }));
const descriptor = (id: string, dependencies: readonly object[] = []) => ({ manifest: { pluginId: id, version: "1.0.0", dependencies } }) as unknown as PluginPackageDescriptor;
const hostEdges = (id: string) => id.startsWith("flow-extension-") || id === "forms-eager" ? [{ pluginId: "flow", version: "=1.0.0" }] : id === "playbook-module-procedural" ? [{ pluginId: "playbook", version: "=1.0.0" }] : [];

type Harness = { reads: string[]; mounts: string[]; statuses: OnDemandStatus[] };
async function prepare(options: { readonly unpublished?: readonly string[] } = {}) {
  const harness: Harness = { reads: [], mounts: [], statuses: [] };
  const readDescriptor = async (id: string) => {
    harness.reads.push(id);
    if (options.unpublished?.includes(id)) throw new Error(`descriptor unpublished: ${id}`);
    return descriptor(id, hostEdges(id));
  };
  const prepared = await prepareWgpuPluginModules(catalog(), modules, receiptSource, { bootVariant: fixture.variant, readDescriptor: readDescriptor as never });
  const boot = [...harness.reads];
  harness.reads.length = 0;
  return { prepared, harness, boot, readDescriptor };
}

describe("boot plan: primary plus hard requirements only", () => {
  test("reads exactly the primary, its eager contributors and its hard closure", async () => {
    const { prepared, boot } = await prepare({ unpublished: fixture.expected.deferred });
    expect(boot).toEqual(fixture.expected.bootReads);
    const plan = resolvePlaygroundBoot(prepared.catalog, fixture.variant);
    assertWgpuPluginPlan(plan, prepared, "variant");
    expect(plan.plugins.map(row => row.pluginId).sort()).toEqual(fixture.expected.plan);
    expect(prepared.deferred.map(row => row.pluginId).sort()).toEqual(fixture.expected.deferred);
  });
  test("the declared hard dependency joins the plan although no descriptor edge names it", async () => {
    const without = (id: string) => id !== "forms-eager";
    const supplied = modules.filter(row => without(row.pluginId));
    const trimmed = pluginCatalogV1(admitPluginCatalogV1({
      version: 1, hosts: [],
      targets: fixture.catalog.filter(row => without(row.pluginId)).map(row => ({ ...row, moduleUrl: url(row.pluginId), wasmOut: `${row.pluginId}.wasm`, capabilities: [], ...(row.extends ? { extends: row.extends } : {}) })),
      playgrounds: [{ variant: fixture.variant, pluginId: fixture.primary, aliases: [] }],
    }, authority));
    const reads: string[] = [];
    const prepared = await prepareWgpuPluginModules(trimmed, supplied, receiptSource, { bootVariant: fixture.variant, readDescriptor: (async (id: string) => { reads.push(id); return descriptor(id); }) as never });
    expect(reads).toEqual(["procedural", "flow"]);
    const plan = resolvePlaygroundBoot(prepared.catalog, fixture.variant);
    assertWgpuPluginPlan(plan, prepared, "variant");
    expect(plan.plugins.map(row => row.pluginId)).toEqual(["flow", "procedural"]);
  });
  test("a deferred descriptor that is unpublished or absent never fails the boot", async () => {
    const supplied = modules.filter(row => !fixture.expected.deferred.includes(row.pluginId));
    const prepared = await prepareWgpuPluginModules(catalog(), supplied, receiptSource, { bootVariant: fixture.variant, readDescriptor: (async (id: string) => descriptor(id, hostEdges(id))) as never });
    assertWgpuPluginPlan(resolvePlaygroundBoot(prepared.catalog, fixture.variant), prepared, "variant");
    expect(prepared.deferred).toEqual([]);
  });
  test("a required edge still refuses a missing dependency", async () => {
    const supplied = modules.filter(row => row.pluginId !== "flow");
    const prepared = await prepareWgpuPluginModules(catalog(), supplied, receiptSource, { bootVariant: fixture.variant, readDescriptor: (async (id: string) => descriptor(id, hostEdges(id))) as never });
    expect(() => assertWgpuPluginPlan(resolvePlaygroundBoot(prepared.catalog, fixture.variant), prepared, "variant")).toThrow();
  });
  test("an on-demand descriptor edge is not loaded at boot and is not a graph requirement", async () => {
    const reads: string[] = [];
    const prepared = await prepareWgpuPluginModules(catalog(), modules, receiptSource, { bootVariant: fixture.variant, readDescriptor: (async (id: string) => { reads.push(id); return descriptor(id, id === "procedural" ? [{ pluginId: "flow-extension-brep", activation: "on-demand" }, { pluginId: "flow", activation: "required" }] : hostEdges(id)); }) as never });
    expect(reads).not.toContain("flow-extension-brep");
    assertWgpuPluginPlan(resolvePlaygroundBoot(prepared.catalog, fixture.variant), prepared, "variant");
  });
  test("an on-demand edge to a plugin without activation events is refused", async () => {
    await expect(prepareWgpuPluginModules(catalog(), modules, receiptSource, { bootVariant: fixture.variant, readDescriptor: (async (id: string) => descriptor(id, id === "procedural" ? [{ pluginId: "flow", activation: "on-demand" }, { pluginId: "forms-eager", activation: "on-demand" }] : hostEdges(id))) as never })).rejects.toThrow("on-demand dependency without activation events");
  });
});

describe("React and WGPU hosts share one plan", () => {
  test("the React registry expansion, the kernel planner and the WGPU boot plan select the same plugins", async () => {
    const full = catalog();
    const rows: PluginRegistryEntry[] = [...full.plugins, ...full.extensions].map(target => ({ pluginId: target.pluginId, moduleUrl: url(target.pluginId), contributes: target.contributes, consumes: target.consumes, activationEvents: target.activationEvents, dependencies: dependsOnToPluginDependencies(target.dependsOn) }));
    const react = expandPluginRegistry(rows, fixture.primary).map(row => row.pluginId).sort();
    const kernel = resolvePlaygroundBoot(full, fixture.variant).plugins.map(row => row.pluginId).sort();
    const { prepared } = await prepare();
    const wgpu = resolvePlaygroundBoot(prepared.catalog, fixture.variant).plugins.map(row => row.pluginId).sort();
    expect(react).toEqual(fixture.expected.plan);
    expect(kernel).toEqual(react);
    expect(wgpu).toEqual(react);
    expect(onDemandPluginEntries(rows, expandPluginRegistry(rows, fixture.primary)).map(row => row.pluginId).sort()).toEqual(fixture.expected.deferred);
  });
  test("request resolution agrees on every language-agnostic case", () => {
    const rows = fixture.catalog.map(row => ({ pluginId: row.pluginId, moduleUrl: url(row.pluginId), contributes: row.contributes, consumes: row.consumes, activationEvents: row.activationEvents }));
    const deferred = rows.filter(row => fixture.expected.deferred.includes(row.pluginId));
    for (const row of fixture.owners) expect(resolveOnDemandOwners(deferred, row.request as OnDemandRequest)).toEqual(row.owners);
  });
});

describe("on-demand activation", () => {
  test("a node kind starts its extension: descriptor admitted, module mounted, progress and states reported", async () => {
    const { prepared, harness, readDescriptor } = await prepare();
    const activator = createWgpuOnDemandActivator(prepared, receiptSource, { readDescriptor: readDescriptor as never, mount: async (id, moduleUrl) => { harness.mounts.push(`${id}@${moduleUrl}`); }, onStatus: status => harness.statuses.push(status) });
    const outcome = await activator.request({ kind: "node-kind", nodeKind: "brep.bool.cut" });
    expect(outcome).toEqual({ requested: ["flow-extension-brep"], active: ["flow-extension-brep"], unavailable: [], cancelled: [] });
    expect(harness.reads).toEqual(["flow-extension-brep"]);
    expect(harness.mounts).toEqual([`flow-extension-brep@${url("flow-extension-brep")}`]);
    expect(harness.statuses.map(status => status.phase).filter((phase, index, phases) => phases[index - 1] !== phase)).toEqual(["resolving", "loading", "active"]);
    expect(harness.statuses.filter(status => status.phase === "loading").map(status => `${status.completed}/${status.total}`)).toEqual(["0/1", "1/2"]);
    expect(prepared.packages.has("flow-extension-brep")).toBe(true);
    expect(await activator.request({ kind: "node-kind", nodeKind: "brep.sphere" })).toMatchObject({ active: ["flow-extension-brep"] });
    expect(harness.mounts.length).toBe(1);
  });
  test("a request in flight is joined, never started twice", async () => {
    const { prepared, harness, readDescriptor } = await prepare();
    let release!: () => void;
    const gate = new Promise<void>(resolve => { release = resolve; });
    const activator = createWgpuOnDemandActivator(prepared, receiptSource, { readDescriptor: readDescriptor as never, mount: async id => { harness.mounts.push(id); await gate; } });
    const first = activator.request({ kind: "tag", tag: "flow.extension" });
    const second = activator.request({ kind: "node-kind", nodeKind: "math.vector" });
    await new Promise(resolve => setTimeout(resolve, 5));
    release();
    expect((await first).active.sort()).toEqual(["flow-extension-brep", "flow-extension-math", "flow-extension-primitive"]);
    expect((await second).active).toEqual(["flow-extension-math"]);
    expect(harness.mounts.filter(id => id === "flow-extension-math").length).toBe(1);
  });
  test("cancellation settles the caller at once and a later request starts again", async () => {
    const { prepared, harness, readDescriptor } = await prepare();
    let stall = true;
    const activator = createWgpuOnDemandActivator(prepared, receiptSource, { readDescriptor: readDescriptor as never, mount: async id => { harness.mounts.push(id); if (stall) await new Promise(() => {}); }, onStatus: status => harness.statuses.push(status) });
    const pending = activator.request({ kind: "plugin", pluginId: "flow-extension-math" });
    await new Promise(resolve => setTimeout(resolve, 5));
    expect(activator.cancel("flow-extension-math")).toBe(1);
    expect(await pending).toMatchObject({ active: [], cancelled: ["flow-extension-math"], unavailable: [] });
    expect(harness.statuses.at(-1)?.phase).toBe("cancelled");
    expect(activator.cancel("flow-extension-math")).toBe(0);
    stall = false;
    expect(await activator.request({ kind: "plugin", pluginId: "flow-extension-math" })).toMatchObject({ active: ["flow-extension-math"] });
  });
  test("a caller signal withdraws the request", async () => {
    const { prepared, readDescriptor } = await prepare();
    const activator = createWgpuOnDemandActivator(prepared, receiptSource, { readDescriptor: readDescriptor as never, mount: () => new Promise(() => {}) });
    const caller = new AbortController();
    const pending = activator.request({ kind: "plugin", pluginId: "flow-extension-brep" }, caller.signal);
    await new Promise(resolve => setTimeout(resolve, 5));
    caller.abort();
    expect((await pending).cancelled).toEqual(["flow-extension-brep"]);
  });
  test("a missing optional descriptor reports unavailable and settles instead of throwing", async () => {
    const { prepared, harness, readDescriptor } = await prepare();
    const activator = createWgpuOnDemandActivator(prepared, receiptSource, { readDescriptor: (async (id: string) => { if (id === "flow-extension-primitive") throw new Error("descriptor 404"); return readDescriptor(id); }) as never, mount: async id => { harness.mounts.push(id); }, onStatus: status => harness.statuses.push(status) });
    const outcome = await activator.request({ kind: "tag", tag: "flow.extension" });
    expect(outcome.active.sort()).toEqual(["flow-extension-brep", "flow-extension-math"]);
    expect(outcome.unavailable).toEqual([{ pluginId: "flow-extension-primitive", phase: "unavailable", completed: 0, total: 1, reason: "load-failed" }]);
    expect(harness.mounts).not.toContain("flow-extension-primitive");
    expect(await activator.request({ kind: "plugin", pluginId: "not-installed" })).toMatchObject({ requested: [], unavailable: [{ pluginId: "not-installed", reason: "not-installed" }] });
    expect(await activator.request({ kind: "node-kind", nodeKind: "zzz.unknown" })).toEqual({ requested: [], active: [], unavailable: [], cancelled: [] });
  });
  test("a required dependency is started first and a missing one makes the plugin unavailable", async () => {
    const { prepared, harness, readDescriptor } = await prepare();
    const activator = createWgpuOnDemandActivator(prepared, receiptSource, { readDescriptor: readDescriptor as never, mount: async id => { harness.mounts.push(id); } });
    expect(await activator.request({ kind: "event", event: "on-artifact-kind:s.playbook.procedural" })).toMatchObject({ requested: ["playbook-module-procedural"], active: ["playbook-module-procedural"] });
    expect(harness.mounts).toEqual(["playbook", "playbook-module-procedural"]);
    const lonely = await prepareWgpuPluginModules(catalog(), modules.filter(row => row.pluginId !== "playbook"), receiptSource, { bootVariant: fixture.variant, readDescriptor: (async (id: string) => descriptor(id, hostEdges(id))) as never });
    const isolated = createWgpuOnDemandActivator(lonely, receiptSource, { readDescriptor: (async (id: string) => descriptor(id, hostEdges(id))) as never, mount: async () => {} });
    expect((await isolated.request({ kind: "event", event: "on-artifact-kind:s.playbook.procedural" })).unavailable).toMatchObject([{ pluginId: "playbook-module-procedural", reason: "dependency-missing" }]);
  });
  test("a descriptor of another identity is refused", async () => {
    const { prepared } = await prepare();
    const activator = createWgpuOnDemandActivator(prepared, receiptSource, { readDescriptor: (async () => descriptor("something-else")) as never, mount: async () => {} });
    expect((await activator.request({ kind: "plugin", pluginId: "flow-extension-brep" })).unavailable).toMatchObject([{ reason: "load-failed" }]);
  });
});

describe("bilingual and accessible state", () => {
  test("every phase has an English and a German text, and a region tag follows its language", () => {
    for (const [phase, row] of Object.entries(fixture.labels)) {
      const status = { pluginId: "flow-extension-brep", phase: phase as OnDemandStatus["phase"] };
      expect(onDemandStatusLabel(status, "en")).toBe(row.en);
      expect(onDemandStatusLabel(status, "de")).toBe(row.de);
      expect(onDemandStatusLabel(status, "de-CH")).toBe(row.de);
      expect(onDemandStatusLabel(status, "fr")).toBe(row.en);
    }
    for (const phase of schema["x-semio-admission"].onDemandPhases) expect(onDemandStatusLabel({ pluginId: "x", phase: phase as OnDemandStatus["phase"] }, "de")).toContain("x");
  });
  test("a running load is a polite busy status, an unavailable plugin an assertive alert", () => {
    expect(onDemandStatusAnnouncement({ phase: "loading" })).toEqual({ role: "status", ariaLive: "polite", busy: true });
    expect(onDemandStatusAnnouncement({ phase: "active" })).toEqual({ role: "status", ariaLive: "polite", busy: false });
    expect(onDemandStatusAnnouncement({ phase: "unavailable" })).toEqual({ role: "alert", ariaLive: "assertive", busy: false });
  });
});

describe("schema against an independent validator", () => {
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(schema);
  const check = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`)!;
  test("requests", () => {
    const validate = check("onDemandRequest");
    for (const value of fixture.requests.valid) { expect(validate(value)).toBe(true); expect(admitOnDemandRequest(value)).toEqual(value as OnDemandRequest); }
    for (const value of fixture.requests.invalid) { expect(validate(value)).toBe(false); expect(() => admitOnDemandRequest(value)).toThrow("on-demand.request-invalid"); }
  });
  test("dependency edges", async () => {
    const validate = check("dependency");
    for (const edge of fixture.edges.valid) {
      expect(validate(edge)).toBe(true);
      await prepareWgpuPluginModules(catalog(), modules, receiptSource, { bootVariant: fixture.variant, readDescriptor: (async (id: string) => descriptor(id, id === "procedural" ? [{ pluginId: "flow", activation: "required" }, ...(edge.pluginId === "flow" ? [] : [edge])] : hostEdges(id))) as never });
    }
    for (const edge of fixture.edges.invalid) {
      expect(validate(edge)).toBe(false);
      await expect(prepareWgpuPluginModules(catalog(), modules, receiptSource, { bootVariant: fixture.variant, readDescriptor: (async (id: string) => descriptor(id, id === "procedural" ? [edge] : hostEdges(id))) as never })).rejects.toThrow("dependency activation");
    }
  });
  test("statuses", () => {
    const validate = check("onDemandStatus");
    expect(validate({ pluginId: "flow-extension-brep", phase: "unavailable", completed: 0, total: 1, reason: "load-failed" })).toBe(true);
    expect(validate({ pluginId: "flow-extension-brep", phase: "paused", completed: 0, total: 1 })).toBe(false);
    expect(validate({ pluginId: "flow-extension-brep", phase: "active", completed: -1, total: 1 })).toBe(false);
  });
});

describe("the committed catalog declares the classification", () => {
  const root = findWorkspaceRoot();
  const rows = JSON.parse(readFileSync(join(root, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json"), "utf8")) as { pluginId: string; dependsOn: string[]; activationEvents: string[]; contributes: string[] }[];
  const namespaces: Record<string, string> = { bim: "bim", brep: "brep", dictionary: "dictionary", draw: "draw", list: "list", logic: "logic", math: "math", primitive: "core", text: "text" };
  test("procedural requires the flow hub alone and defers its nine flow extensions", () => {
    expect(rows.find(row => row.pluginId === "procedural")?.dependsOn).toEqual(["flow"]);
    for (const [slug, namespace] of Object.entries(namespaces)) {
      const row = rows.find(candidate => candidate.pluginId === `flow-extension-${slug}`)!;
      expect(row.contributes).toContain("flow.extension");
      expect(row.activationEvents).toEqual([`on-node-kind:${namespace}`]);
    }
  });
  test("an authored activation event is a well-formed catalog event", () => {
    expect(parseSemioActivatesOn('activates-on = ["on-node-kind:brep"]', "Cargo.toml")).toEqual(["on-node-kind:brep"]);
    expect(parseSemioActivatesOn("role = \"extension\"", "Cargo.toml")).toEqual([]);
    expect(() => parseSemioActivatesOn('activates-on = ["brep"]', "Cargo.toml")).toThrow("malformed activation event");
    expect(() => parseSemioActivatesOn('activates-on = ["on-node-kind:a", "on-node-kind:a"]', "Cargo.toml")).toThrow("repeats");
  });
});
