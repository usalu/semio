/** ☁️ Transpile, probe and describe stages produced on another machine equal a local run byte for byte, through the real worker harness of the command offload; jco is the third-party oracle of the component and of the transpiled tree. */
import { afterAll, beforeAll, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { componentEmbed, componentNew } from "@bytecodealliance/jco";
import cases from "../../../🧪️tests/🦀️native-transpile/🧫️cases.json" with { type: "json" };
import { eventuallyV1, startClusterV1, type ClusterV1 } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🧰️cluster/🟦️.ts";
import { remoteExecutionHostPropertiesV1 } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🟦️.ts";
import { offloadCommandV1 } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts";
import { commandToolDigestV1, detectWorkerCommandsV1 } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🧰️command/🛠️worker-tools/🟦️.ts";
import { fileDigest } from "../../../../📦️distribution/📋️inventory/🟦️.ts";
import { stageCacheV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import { nativeTranspileBinaryV1 } from "../../../🦀️native-transpile/🟦️.ts";
import { transpilePluginComponentModulesAsync } from "../../../🟦️.ts";
import { probeStageRemoteV1, transpileStageRemoteV1, type ComponentModuleWorkV1 } from "../../🟦️.ts";
import { describeStageRemoteV1 } from "../../../../../🖨️describe/🧩️component-stage/🟦️.ts";

const repoRoot = fileURLToPath(new URL("../../../../../../../../../../", import.meta.url));
const scratch = realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "stage-remote-")));
const binary = (() => { try { return nativeTranspileBinaryV1(repoRoot); } catch { return undefined; } })();
const agentStop = new AbortController();
let cluster: ClusterV1, reporter: Promise<void> | undefined, detected: Awaited<ReturnType<typeof detectWorkerCommandsV1>>, environment: Record<string, string>;

function tree(root: string, directory = root): Map<string, string> {
  const files = new Map<string, string>();
  for (const entry of readdirSync(directory)) {
    const path = join(directory, entry);
    if (statSync(path).isDirectory()) for (const [name, hash] of tree(root, path)) files.set(name, hash);
    else files.set(relative(root, path), new Bun.CryptoHasher("sha256").update(readFileSync(path)).digest("hex"));
  }
  return files;
}

const workOf = (name: string, wasm: string, packageName: string): ComponentModuleWorkV1 => {
  const work = join(scratch, name);
  mkdirSync(work, { recursive: true });
  return { repoRoot, work, environment, cache: stageCacheV1(join(scratch, `${name}-stages`)), deliverable: { wasm, profile: "release" }, target: { packageName } } as unknown as ComponentModuleWorkV1;
};

beforeAll(async () => {
  detected = await detectWorkerCommandsV1(repoRoot, process.env);
  cluster = await startClusterV1({ workers: 0, root: scratch, scheduler: { leaseSeconds: 5, lingerSeconds: 1, unmatchedSeconds: 6 } });
  cluster.startWorker({ name: "wA", actions: "rustc", unconfinedForTests: false, properties: [...remoteExecutionHostPropertiesV1(), ...detected.properties], commands: detected.tools });
  const host = { version: 1, atMs: Date.now() + 86_400_000, platform: "darwin", cores: 8, totalMegabytes: 65536, availableMegabytes: 60000, pressure: "normal", swapUsedMegabytes: 0, swapOutMegabytesPerSecond: 0, load: 0.5, openFiles: 1000, openFilesMaximum: 200000, attachedImages: 0 };
  const { buildRemoteExecutionLoadReportV1, readRemoteExecutionCountersV1, remoteExecutionPlatformIdentityV1, runRemoteExecutionReporterV1 } = await import("../../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🌡️load/🟦️.ts");
  const report = async () => buildRemoteExecutionLoadReportV1({ origin: "wA", identity: remoteExecutionPlatformIdentityV1("darwin", "arm64", "29a34002290ca2fc", "aarch64-apple-darwin"), view: { host, grants: [], waiters: [] } as never, worker: { state: "idle", concurrency: 2, running: 0, sandbox: "darwin-sandbox", confinement: "full", network: "denied" }, counters: readRemoteExecutionCountersV1(scratch), nowMs: Date.now() });
  reporter = runRemoteExecutionReporterV1({ scheduler: cluster.server.workerUrl, token: undefined, fetch, intervalSeconds: 1, signal: agentStop.signal, report });
  await eventuallyV1(async () => ((await (await fetch(cluster.server.workerUrl + "/status")).json()) as { workers?: unknown[] }).workers?.length! >= 1, 60000, "the worker joined");
  await eventuallyV1(async () => ((await (await fetch(cluster.server.workerUrl + "/v1/fleet/load")).json()) as { machines?: { origin: string; headroom?: boolean }[] }).machines?.some((row) => row.origin === "wA" && row.headroom === true) === true, 60000, "the worker's machine reports its load");
  environment = { PATH: process.env.PATH ?? "", HOME: scratch, SEMIO_UNIT_CACHE_ROOT: join(scratch, "offload"), SEMIO_REMOTE_EXECUTION_ENDPOINT: cluster.server.grpcUrl, SEMIO_REMOTE_EXECUTION_SCHEDULER: cluster.server.workerUrl, SEMIO_REMOTE_CACHE_ENDPOINT: cluster.cache.url.replace("//0.0.0.0", "//127.0.0.1"), SEMIO_REMOTE_CACHE_INSTANCE: "main", SEMIO_REMOTE_CACHE_ACCESS: "read-write", SEMIO_REMOTE_CACHE_TOKEN: "test-token-not-a-secret", ...(binary ? { SEMIO_NATIVE_TRANSPILE_BIN: binary } : {}) };
}, 120000);
afterAll(async () => { agentStop.abort(); await reporter?.catch(() => {}); await cluster?.stop().catch(() => {}); rmSync(scratch, { recursive: true, force: true }); });

test.skipIf(binary === undefined)("a transpile stage produced on another machine holds the exact files of a local run, and every file but the rewritten module equals jco's", async () => {
  const row = cases.cases[0]!, base = row.name_option, wit = join(scratch, "wit");
  mkdirSync(wit, { recursive: true });
  writeFileSync(join(wit, "probe.wit"), row.wit);
  const wasm = join(scratch, `${base}.wasm`);
  writeFileSync(wasm, await componentNew(await componentEmbed({ dummy: true, witPath: wit }), []));
  const work = workOf("transpile", wasm, base), local = join(scratch, "transpile-local"), remote = join(scratch, "transpile-remote"), reference = join(scratch, "transpile-jco");
  await transpilePluginComponentModulesAsync(wasm, local, base, { repoRoot, mode: "release", preview2VendorDir: "", environment, control: undefined } as never);
  mkdirSync(remote, { recursive: true });
  const answered = await transpileStageRemoteV1(work, binary!, await fileDigest(binary!))(remote, new AbortController().signal);
  expect(answered, "the fleet took the step").toBe(true);
  expect([...tree(remote)]).toEqual([...tree(local)]);
  const jco = spawnSync(process.execPath, [join(repoRoot, "node_modules", ".bin", "jco"), "transpile", wasm.replace(/\.wasm$/u, ".wasm"), "-o", reference, "-q", "--name", base, ...row.maps.flatMap((map) => ["--map", map])], { encoding: "utf8" });
  expect(jco.status).toBe(0);
  const expected = tree(reference), actual = tree(remote);
  for (const [name, hash] of expected) if (!name.endsWith(".js")) expect(actual.get(name), name).toBe(hash);
}, 120000);

test.skipIf(binary === undefined)("a transpile whose digest no SHA-256 names, or that the fleet refuses, leaves the stage directory empty and answers false so the stage runs here", async () => {
  const work = workOf("transpile-fallback", join(scratch, "comp.wasm"), "comp"), directory = join(scratch, "transpile-fallback-out");
  mkdirSync(directory, { recursive: true });
  writeFileSync(join(directory, "stale.txt"), "left over");
  expect(await transpileStageRemoteV1(work, binary!, "not-a-digest")(directory, new AbortController().signal)).toBe(false);
  expect(readdirSync(directory)).toEqual([]);
  expect(await transpileStageRemoteV1({ ...work, environment: { ...environment, SEMIO_OFFLOAD: "never" } }, binary!, await fileDigest(binary!))(directory, new AbortController().signal)).toBe(false);
}, 60000);

const PROBE = `import { pathToFileURL } from "node:url";
const component = await import(pathToFileURL(process.argv[1]).href);
process.stdout.write(Buffer.from(component.answer()).toString("base64"));
`;

test("a probe stage produced on another machine answers the bytes a local node run of the same probe answers, over the module and vendor layout its imports expect", async () => {
  const base = "probe_comp", root = join(scratch, "probe-tree"), module = join(root, "module"), vendor = join(root, "vendor", "preview2");
  mkdirSync(module, { recursive: true });
  mkdirSync(vendor, { recursive: true });
  writeFileSync(join(vendor, "shim.js"), `export const shim = "vendor-bytes";\n`);
  writeFileSync(join(module, "host.js"), `export const host = "host-bytes";\n`);
  writeFileSync(join(module, `${base}.js`), `import { shim } from "../vendor/preview2/shim.js";\nimport { host } from "./host.js";\nexport const answer = () => new TextEncoder().encode(shim + "|" + host);\n`);
  const work = workOf("probe", join(scratch, "unused.wasm"), "probe-comp");
  const local = spawnSync("node", ["--input-type=module", "--eval", PROBE, join(module, `${base}.js`)], { encoding: "utf8" });
  expect(local.status).toBe(0);
  const directory = join(scratch, "probe-out");
  mkdirSync(directory, { recursive: true });
  expect(await probeStageRemoteV1(work, module, vendor, PROBE)(directory, new AbortController().signal), "the fleet took the step").toBe(true);
  expect(Buffer.from(readFileSync(join(directory, "semantic.bin"))).toString("base64")).toBe(local.stdout);
  expect(Buffer.from(readFileSync(join(directory, "semantic.bin"))).toString("utf8")).toBe("vendor-bytes|host-bytes");
}, 120000);

test("a probe that fails on the worker, or whose node no machine owns, leaves the stage directory empty and answers false", async () => {
  const base = "probe_bad", root = join(scratch, "probe-bad"), module = join(root, "module"), vendor = join(root, "vendor");
  mkdirSync(module, { recursive: true });
  mkdirSync(vendor, { recursive: true });
  writeFileSync(join(module, `${base}.js`), `export const answer = () => { throw new Error("boom"); };\n`);
  const directory = join(scratch, "probe-bad-out");
  mkdirSync(directory, { recursive: true });
  expect(await probeStageRemoteV1(workOf("probe-bad", join(scratch, "unused.wasm"), "probe-bad"), module, vendor, PROBE)(directory, new AbortController().signal)).toBe(false);
  expect(readdirSync(directory)).toEqual([]);
}, 120000);

test("an output directory of a shipped tool lands below its destination exactly as the tool wrote it, and a tool that writes nothing there leaves the step to the local run", async () => {
  const tool = join(scratch, "tree-tool.sh"), text = `#!/bin/sh\nif [ "$2" = "empty" ]; then exit 0; fi\nmkdir -p "$1/sub"\nprintf alpha > "$1/a.txt"\nprintf beta > "$1/sub/b.txt"\n`;
  writeFileSync(tool, text);
  chmodSync(tool, 0o755);
  const digest = commandToolDigestV1(tool), run = (mode: string, destination: string) => offloadCommandV1({ tool: { name: "build-script", digest, file: tool }, arguments: ["@root/out/d", mode], inputs: new Map(), outputs: new Map(), directories: new Map([["out/d", destination]]), root: join(scratch, "tree-root"), environment, label: "tree", key: null });
  const destination = join(scratch, "tree-out");
  mkdirSync(destination, { recursive: true });
  writeFileSync(join(destination, "keep.txt"), "kept");
  const done = await run("full", destination);
  expect(done, JSON.stringify(done)).toMatchObject({ kind: "done", status: 0 });
  expect(readFileSync(join(destination, "a.txt"), "utf8")).toBe("alpha");
  expect(readFileSync(join(destination, "sub", "b.txt"), "utf8")).toBe("beta");
  expect(readFileSync(join(destination, "keep.txt"), "utf8")).toBe("kept");
  const empty = join(scratch, "tree-empty");
  const none = await run("empty", empty);
  expect(none).toMatchObject({ kind: "local" });
  expect(existsSync(join(empty, "a.txt"))).toBe(false);
}, 120000);

test("a describe whose emitter ran on another machine but answered a pair that does not verify leaves the stage directory empty and answers false", async () => {
  const emitter = join(scratch, "emitter.sh"), text = `#!/bin/sh\nmkdir -p "$4"\nprintf '{}' > "$4/🔣️.json"\nprintf x > "$4/🛂️.descriptor.semio"\n`;
  writeFileSync(emitter, text);
  chmodSync(emitter, 0o755);
  const wasm = join(scratch, "describe-artifacts", "component.wasm"), directory = join(scratch, "describe-out");
  mkdirSync(join(scratch, "describe-artifacts", "release"), { recursive: true });
  mkdirSync(directory, { recursive: true });
  writeFileSync(wasm, "\0asm");
  const answered = await describeStageRemoteV1(stageCacheV1(join(scratch, "describe-stages")), { wasm, directory: join(scratch, "describe-artifacts", "release") } as never, { environment } as never, emitter, commandToolDigestV1(emitter))(directory, new AbortController().signal);
  expect(answered).toBe(false);
  expect(readdirSync(directory)).toEqual([]);
  expect(readFileSync(join(scratch, "offload", "x", "offload.jsonl"), "utf8")).toContain('"crate":"describe component.wasm","key":null,"origin":"wA"');
}, 120000);
