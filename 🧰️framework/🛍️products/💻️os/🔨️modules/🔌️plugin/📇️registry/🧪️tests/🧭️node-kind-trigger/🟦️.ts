/** 🧭️ Node-kind trigger laws: the scan finds exactly the kinds a document names, the mapping activates exactly the extensions the 3D examples use, activation settles before the guest gets the document, a cancellation hands nothing off, an unavailable extension still opens the document with its nodes reported unavailable in English and German, and the React-shaped and WGPU-shaped activators agree. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import {
  admitOnDemandRequest, createOnDemandActivator, documentActivationLabel, documentExtensionOwners, exampleDocumentText, guestActivationRequest, nodeKindRequests, ON_DEMAND_REQUEST_TOPIC, openDocumentAfterActivation, openExampleAfterActivation, scanNodeKinds,
  type DocumentActivationReport, type OnDemandActivator, type PluginRegistryEntry,
} from "@semio-tech/framework-kernel";
import { createWgpuOnDemandActivator, prepareWgpuPluginModules } from "@semio-tech/framework-renderer-wgpu/plugin-modules";
import type { PluginPackageDescriptor } from "@semio-tech/framework-kernel/descriptor-admission";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { applyOnDemandRequestEffect } from "../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/⚡️host-effects/🧭️on-demand-request/🟦️.ts";
import { admitPluginCatalogV1, pluginCatalogV1 } from "../../🟦️.ts";
import schema from "../../../../📺️renderer/🧑‍🎨engine/🧬️schema/🧩️plugin-modules/🔣️.json";
import activationFixture from "../🧫️fixtures/⏳️on-demand-activation/🔣️.json";
import fixture from "../🧫️fixtures/🧭️node-kind-trigger/🔣️.json";

const root = findWorkspaceRoot();
const examplesDirectory = join(root, "✏️s", "🔌️plugins", "🌀️procedural", "🗿️artifacts", "🧊️generation3d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "📚️examples");
const committed = JSON.parse(readFileSync(join(root, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json"), "utf8")) as { pluginId: string; activationEvents: string[]; contributes: string[]; consumes: string[] }[];
const entries: PluginRegistryEntry[] = committed.map((row) => ({ pluginId: row.pluginId, moduleUrl: `https://example.test/${row.pluginId}.js`, contributes: row.contributes, consumes: row.consumes, activationEvents: row.activationEvents }));
const deferred = entries.filter((entry) => (entry.activationEvents ?? []).length > 0);

/** 🔎️ The independent oracle: whitespace tokens that start with `neuron-kind=`, no regular expression shared with the implementation. */
const oracleKinds = (text: string): string[] => [...new Set(text.split(/\s+/u).filter((token) => token.startsWith("neuron-kind=")).map((token) => token.slice("neuron-kind=".length).replaceAll('"', "")))].sort();

type ExampleFile = { readonly slug: string; readonly text: string };
const exampleFiles = (): ExampleFile[] => readdirSync(examplesDirectory).flatMap((directory) => {
  const assets = join(examplesDirectory, directory, "🖼️assets");
  try {
    return readdirSync(assets, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => ({ slug: /[a-z][a-z0-9-]*$/u.exec(directory)![0], text: readFileSync(join(assets, entry.name, "🗣️.dsl.semio"), "utf8") }));
  } catch { return []; }
});

const activatorOver = (activate: (entry: PluginRegistryEntry, signal: AbortSignal) => Promise<void>, active = new Set<string>()): OnDemandActivator =>
  createOnDemandActivator({ candidates: () => deferred, installed: () => entries, active: () => active, activate: (entry, context) => activate(entry, context.signal) });

describe("scan and mapping against the 3D examples", () => {
  const files = exampleFiles();
  test("every bundled generation3d example is covered by the fixture", () => {
    expect(files.map((file) => file.slug).sort()).toEqual(Object.keys(fixture.examples).sort());
  });
  for (const file of files) {
    test(`${file.slug}: the scan equals the independent oracle and activates exactly its extensions`, () => {
      const expected = fixture.examples[file.slug as keyof typeof fixture.examples];
      expect([...scanNodeKinds(file.text)].sort()).toEqual(oracleKinds(file.text));
      expect(oracleKinds(file.text)).toEqual(expected.nodeKinds);
      expect(documentExtensionOwners(scanNodeKinds(file.text), deferred)).toEqual(expected.extensions);
      const namespaces = [...new Set(oracleKinds(file.text).map((kind) => kind.split(".")[0]))].sort();
      expect(documentExtensionOwners(scanNodeKinds(file.text), deferred)).toEqual(namespaces.map((namespace) => `flow-extension-${namespace}`));
    });
  }
  for (const name of ["dsl", "json", "none"] as const) {
    test(`document form ${name}`, () => {
      const text = fixture.documents[name];
      const expected = fixture.expected[name];
      expect([...scanNodeKinds(text)].sort()).toEqual(expected.nodeKinds);
      expect(nodeKindRequests(scanNodeKinds(text)).map((request) => request.nodeKind)).toEqual(expected.requests);
      expect(documentExtensionOwners(scanNodeKinds(text), deferred)).toEqual(expected.extensions);
    });
  }
  test("the JSON form scans like a structural walk of the parsed document", () => {
    const walk = (value: unknown): string[] => Array.isArray(value) ? value.flatMap(walk) : value !== null && typeof value === "object" ? Object.entries(value).flatMap(([key, member]) => key === "neuronKind" && typeof member === "string" ? [member] : walk(member)) : [];
    expect([...scanNodeKinds(fixture.documents.json)].sort()).toEqual(walk(JSON.parse(fixture.documents.json)).sort());
  });
  test("the requests it emits are admitted by the wire schema and the independent validator", () => {
    const ajv = new Ajv({ strict: false });
    ajv.addSchema(schema);
    const check = ajv.getSchema(`${schema.$id}#/$defs/onDemandRequest`)!;
    for (const file of files) for (const request of nodeKindRequests(scanNodeKinds(file.text))) {
      expect(check(request)).toBe(true);
      expect(admitOnDemandRequest(request)).toEqual(request);
    }
  });
});

describe("activation settles before the guest gets the document", () => {
  test("the handoff runs once, after every extension of the document is active", async () => {
    const events: string[] = [];
    const activator = activatorOver(async (entry) => {
      events.push(`start:${entry.pluginId}`);
      await new Promise((resolve) => setTimeout(resolve, entry.pluginId === "flow-extension-brep" ? 15 : 1));
      events.push(`active:${entry.pluginId}`);
    });
    const progress: string[] = [];
    const opened = await openDocumentAfterActivation({ text: fixture.documents.dsl, activator, locale: "en", handoff: ({ report }) => { events.push("handoff"); return report.state; }, progress: (completed, total, label) => progress.push(`${completed}/${total} ${label}`) });
    expect(opened).toMatchObject({ state: "opened", result: "ready", report: { active: ["flow-extension-brep", "flow-extension-math"], unavailable: [] } });
    expect(events.at(-1)).toBe("handoff");
    expect(events.filter((event) => event === "handoff")).toHaveLength(1);
    expect(events.indexOf("handoff")).toBeGreaterThan(events.indexOf("active:flow-extension-brep"));
    expect(events.indexOf("handoff")).toBeGreaterThan(events.indexOf("active:flow-extension-math"));
    expect(progress[0]).toBe(`0/2 ${fixture.labels.preparing.en.replace("1 of 2", "0 of 2")}`);
    expect(progress.at(-1)).toContain("2/2");
  });
  test("an already active extension is not started again and a document without kinds is handed off at once", async () => {
    const started: string[] = [];
    const activator = activatorOver(async (entry) => { started.push(entry.pluginId); }, new Set(["flow-extension-brep"]));
    await openDocumentAfterActivation({ text: fixture.documents.dsl, activator, locale: "de", handoff: () => 0 });
    expect(started).toEqual(["flow-extension-math"]);
    let calls = 0;
    expect(await openDocumentAfterActivation({ text: fixture.documents.none, activator, locale: "en", handoff: () => ++calls })).toMatchObject({ state: "opened", result: 1 });
  });
  test("a bundled example is resolved by id from the manifest rows; one with no host-visible body is handed off at once", async () => {
    const examples = [{ id: "sphere-box-fuse", documentJson: fixture.documents.dsl }, { id: "opaque" }];
    expect(exampleDocumentText(examples, "sphere-box-fuse")).toBe(fixture.documents.dsl);
    expect(exampleDocumentText(examples, "missing")).toBeUndefined();
    const started: string[] = [];
    const activator = activatorOver(async (entry) => { started.push(entry.pluginId); });
    await openExampleAfterActivation({ examples, exampleId: "opaque", activator, locale: "en", handoff: () => 0 });
    expect(started).toEqual([]);
    await openExampleAfterActivation({ examples, exampleId: "sphere-box-fuse", activator, locale: "en", handoff: () => 0 });
    expect(started.sort()).toEqual(["flow-extension-brep", "flow-extension-math"]);
  });
});

describe("cancellation", () => {
  test("a caller abort during activation hands nothing off and says so in both languages", async () => {
    const caller = new AbortController();
    const activator = activatorOver((_, signal) => new Promise((_resolve, reject) => signal.addEventListener("abort", () => reject(new Error("aborted")), { once: true })));
    let handedOff = 0;
    const notices: { text: string; role: string }[] = [];
    const pending = openDocumentAfterActivation({ text: fixture.documents.dsl, activator, locale: "de", signal: caller.signal, handoff: () => ++handedOff, notify: (notice) => notices.push({ text: notice.text, role: notice.announcement.role }) });
    await new Promise((resolve) => setTimeout(resolve, 5));
    caller.abort();
    const settled = await pending;
    expect(settled.state).toBe("cancelled");
    expect(handedOff).toBe(0);
    expect(notices).toHaveLength(1);
    expect(notices[0]!.text.startsWith("Das Öffnen des Dokuments wurde")).toBe(true);
    expect(notices[0]!.text).toContain("flow-extension-");
  });
  test("an abort before the first request starts nothing; activator.cancel settles the document as cancelled", async () => {
    const started: string[] = [];
    const activator = activatorOver(async (entry) => { started.push(entry.pluginId); });
    const early = new AbortController();
    early.abort();
    expect(await openDocumentAfterActivation({ text: fixture.documents.dsl, activator, locale: "en", signal: early.signal, handoff: () => 1 })).toMatchObject({ state: "cancelled" });
    expect(started).toEqual([]);
    const hanging = activatorOver(() => new Promise(() => {}));
    const pending = openDocumentAfterActivation({ text: fixture.documents.dsl, activator: hanging, locale: "en", handoff: () => 1 });
    await new Promise((resolve) => setTimeout(resolve, 5));
    expect(hanging.cancel()).toBe(2);
    expect((await pending).state).toBe("cancelled");
  });
  test("the cancelled and unavailable texts equal the language-agnostic fixture", () => {
    const cancelled = { nodeKinds: [], active: [], unavailable: [], cancelled: ["flow-extension-brep"], unowned: [], state: "cancelled" } as DocumentActivationReport;
    expect(documentActivationLabel("cancelled", "en", { plugins: "flow-extension-brep" })).toBe(fixture.labels.cancelled.en);
    expect(documentActivationLabel("cancelled", "de-CH", { plugins: "flow-extension-brep" })).toBe(fixture.labels.cancelled.de);
    expect(documentActivationLabel("cancelled", "fr", { plugins: "x" })).toStartWith("Opening the document");
    expect(cancelled.state).toBe("cancelled");
  });
});

describe("an unavailable extension", () => {
  const failing = () => activatorOver(async (entry) => { if (entry.pluginId === "flow-extension-brep") throw new Error("module 404"); });
  test("still opens the document, reports each of its nodes unavailable and announces it accessibly in en and de", async () => {
    const text = "neuron id=a neuron-kind=brep.bool.cut x\nneuron id=b neuron-kind=math.add y\n";
    const seen: { report: DocumentActivationReport }[] = [];
    for (const locale of ["en", "de"] as const) {
      const notices: { text: string; role: string; live: string }[] = [];
      const opened = await openDocumentAfterActivation({ text, activator: failing(), locale, handoff: (context) => { seen.push(context); return "opened"; }, notify: (notice) => notices.push({ text: notice.text, role: notice.announcement.role, live: notice.announcement.ariaLive }) });
      expect(opened).toMatchObject({ state: "opened", result: "opened", report: { state: "partial", active: ["flow-extension-math"], unavailable: [{ nodeKind: "brep.bool.cut", pluginId: "flow-extension-brep", reason: "load-failed" }] } });
      expect(notices).toEqual([{ text: fixture.labels.unavailable[locale], role: "alert", live: "assertive" }]);
    }
    expect(seen).toHaveLength(2);
  });
  test("a kind no deferred extension claims is reported unowned and never blocks the document", async () => {
    const opened = await openDocumentAfterActivation({ text: fixture.documents.json, activator: failing(), locale: "en", handoff: () => "ok" });
    expect(opened).toMatchObject({ state: "opened", report: { state: "ready", unowned: ["zzz.unknown"], active: ["flow-extension-math"] } });
  });
  test("an extension whose descriptor is missing reports the same through the WGPU activator", async () => {
    const { prepared, readDescriptor, receiptSource } = await wgpuHarness({ unpublished: ["flow-extension-brep"] });
    const activator = createWgpuOnDemandActivator(prepared, receiptSource, { readDescriptor: readDescriptor as never, mount: async () => {} });
    const opened = await openDocumentAfterActivation({ text: fixture.documents.dsl, activator, locale: "de", handoff: () => "ok" });
    expect(opened).toMatchObject({ state: "opened", report: { state: "partial", unavailable: [{ nodeKind: "brep.bool.cut" }, { nodeKind: "brep.prim3d.box" }] } });
  });
});

const url = (id: string) => `https://example.test/${id}.js`;
const authority = { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: 30000, now: () => 0, cancelled: () => false, progress: () => {} };
const descriptor = (id: string, dependencies: readonly object[] = []) => ({ manifest: { pluginId: id, version: "1.0.0", dependencies } }) as unknown as PluginPackageDescriptor;
const hostEdges = (id: string) => id.startsWith("flow-extension-") ? [{ pluginId: "flow", version: "=1.0.0" }] : [];
async function wgpuHarness(options: { readonly unpublished?: readonly string[] } = {}) {
  const receiptSource = { async read() { throw Error("no package receipt is requested by the node-kind oracle"); } };
  const catalog = pluginCatalogV1(admitPluginCatalogV1({
    version: 1, hosts: [],
    targets: activationFixture.catalog.map((row) => ({ ...row, moduleUrl: url(row.pluginId), wasmOut: `${row.pluginId}.wasm`, capabilities: [], ...((row as { extends?: string }).extends ? { extends: (row as { extends?: string }).extends } : {}) })),
    playgrounds: [{ variant: activationFixture.variant, pluginId: activationFixture.primary, aliases: [] }],
  }, authority));
  const readDescriptor = async (id: string) => {
    if (options.unpublished?.includes(id)) throw new Error(`descriptor unpublished: ${id}`);
    return descriptor(id, hostEdges(id));
  };
  const prepared = await prepareWgpuPluginModules(catalog, activationFixture.catalog.map((row) => ({ pluginId: row.pluginId, moduleUrl: url(row.pluginId) })), receiptSource, { bootVariant: activationFixture.variant, readDescriptor: readDescriptor as never });
  return { prepared, readDescriptor, receiptSource };
}

describe("React and WGPU parity", () => {
  test("both activator shapes give the same report and the same mounts for every 3D example, handoff last", async () => {
    for (const file of exampleFiles()) {
      const reactMounts: string[] = [];
      const react = activatorOver(async (entry) => { reactMounts.push(entry.pluginId); });
      const { prepared, readDescriptor, receiptSource } = await wgpuHarness();
      const wgpuMounts: string[] = [];
      const wgpu = createWgpuOnDemandActivator(prepared, receiptSource, { readDescriptor: readDescriptor as never, mount: async (id) => { wgpuMounts.push(id); } });
      const order: string[] = [];
      const reports = await Promise.all([[react, reactMounts, "react"], [wgpu, wgpuMounts, "wgpu"]].map(async ([activator, mounts, name]) => {
        const opened = await openDocumentAfterActivation({ text: file.text, activator: activator as OnDemandActivator, locale: "en", handoff: () => { order.push(`${name}:handoff:${(mounts as string[]).length}`); } });
        return opened.report;
      }));
      expect(reports[0]).toEqual(reports[1]!);
      expect([...reactMounts].sort()).toEqual([...wgpuMounts].sort());
      expect([...reactMounts].sort()).toEqual(fixture.examples[file.slug as keyof typeof fixture.examples].extensions);
      const expectedCount = fixture.examples[file.slug as keyof typeof fixture.examples].extensions.length;
      expect(order.sort()).toEqual([`react:handoff:${expectedCount}`, `wgpu:handoff:${expectedCount}`]);
    }
  });
});

describe("the guest asks for an extension it met in a document", () => {
  const bytes = (text: string) => [...new TextEncoder().encode(text)];
  test("the Rust wire text of the flow hub is admitted, one request per namespace", () => {
    const rustCases = ['{"kind":"node-kind","nodeKind":"brep.bool.cut"}', '{"kind":"node-kind","nodeKind":"math.add"}'];
    expect(rustCases.map((text) => guestActivationRequest({ topic: ON_DEMAND_REQUEST_TOPIC, payload: bytes(text) }))).toEqual([...nodeKindRequests(["math.add", "brep.bool.cut", "brep.prim3d.box"])]);
  });
  test("the topic equals the schema constant and another topic is no request", () => {
    expect((schema as { $defs: { onDemandRequestTopic: { const: string } } }).$defs.onDemandRequestTopic.const).toBe(ON_DEMAND_REQUEST_TOPIC);
    expect(guestActivationRequest({ topic: "os.other", payload: bytes("{}") })).toBeUndefined();
  });
  test("a payload that is no admitted request is refused with a typed fault", () => {
    for (const payload of ["not json", '{"kind":"node-kind"}', '{"kind":"node-kind","nodeKind":".x"}', '{"kind":"plugin","pluginId":"x","extra":1}']) expect(() => guestActivationRequest({ topic: ON_DEMAND_REQUEST_TOPIC, payload: bytes(payload) })).toThrow("on-demand.request-invalid");
    expect(() => guestActivationRequest({ topic: ON_DEMAND_REQUEST_TOPIC, payload: [0xff, 0xfe] })).toThrow("on-demand.request-invalid");
  });
  test("the React effect pass starts the activation and leaves other events alone; an invalid payload never throws into it", async () => {
    const sent: string[] = [];
    const door = { semioReactActivateOnDemand: async (json: string) => { sent.push(json); return "{}"; } };
    expect(applyOnDemandRequestEffect({ publishEvent: { topic: ON_DEMAND_REQUEST_TOPIC, payload: bytes('{"kind":"node-kind","nodeKind":"brep.bool.cut"}') } }, door)).toBe(true);
    expect(sent).toEqual(['{"kind":"node-kind","nodeKind":"brep.bool.cut"}']);
    expect(applyOnDemandRequestEffect({ publishEvent: { topic: "os.other", payload: [] } }, door)).toBe(false);
    expect(applyOnDemandRequestEffect({ publishEvent: { topic: ON_DEMAND_REQUEST_TOPIC, payload: bytes("nope") } }, door)).toBe(true);
    expect(sent).toHaveLength(1);
    expect(applyOnDemandRequestEffect({ publishEvent: { topic: ON_DEMAND_REQUEST_TOPIC, payload: bytes('{"kind":"node-kind","nodeKind":"math.add"}') } }, {})).toBe(true);
  });
});

describe("both hosts hand the document over only after activation", () => {
  const engine = join(root, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📺️renderer", "🧑‍🎨engine");
  const read = (...segments: string[]) => readFileSync(join(engine, ...segments), "utf8");
  test("the WGPU frame Worker activates before it boots the renderer and the shell", () => {
    const boot = read("🎯️targets", "🧊️wgpu", "🎞️frame-worker", "🔌️boot-and-plugins", "🟦️.ts");
    expect(boot.indexOf("openExampleAfterActivation({")).toBeGreaterThan(boot.indexOf("mountPluginHandles(bootPlan.plugins"));
    expect(boot.indexOf("openExampleAfterActivation({")).toBeLessThan(boot.indexOf("semioWgpuWorkerBootstrap!("));
    expect(boot).toContain("bootAbort.signal");
    expect(boot).toContain("example-activation.cancelled");
  });
  test("the React picker and boot example hand the action to the guest inside the activation handoff", () => {
    const scope = read("🧱️elements", "🏛️ShellHost", "🔎️command-bar", "🔶️scope", "🟦️.tsx");
    const call = scope.slice(scope.indexOf("return openExampleAfterActivation({"));
    expect(call.indexOf("handoff: () => onAction(buildActiveExampleAction(")).toBeGreaterThan(call.indexOf("openExampleAfterActivation"));
    expect(scope.match(/onAction\(buildActiveExampleAction/g)).toHaveLength(1);
    expect(scope).toContain("exampleActivationRef.current?.abort()");
  });
  test("the WGPU shell forwards the guest request to the same activator door the frame Worker exposes", () => {
    const worker = read("🎯️targets", "🧊️wgpu", "🎞️frame-worker", "🟦️.ts");
    const shell = readFileSync(join(engine, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "📥️staging", "🔀️actions", "🎬️shell-actions", "🎯️dispatch-action", "🦀️.rs"), "utf8");
    const bridge = readFileSync(join(engine, "🧱️elements", "🌉️ProgramBridge", "🎯️targets", "🧊️wgpu", "🧰️document-render-plugin-install-and-native-loading", "🦀️.rs"), "utf8");
    expect(worker).toContain("semioWgpuActivateOnDemand");
    expect(bridge).toContain('js_plugin_install_door("semioWgpuActivateOnDemand")');
    expect(shell).toContain(`topic == "${ON_DEMAND_REQUEST_TOPIC}"`);
    expect(readFileSync(join(root, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🌊️flow", "🖥️host", "🚦️eval-status", "🦀️.rs"), "utf8")).toContain(`ON_DEMAND_REQUEST_TOPIC: &str = "${ON_DEMAND_REQUEST_TOPIC}"`);
  });
});
