import { acquireCompilerCapacityLeaseV1, compilerAdmissionBudgetV1, scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { nextPublicationFiles } from "../../../../🖥️server/🎛️coordinator/🏗️builder/📦️publication/🟦️.ts";
import bunWorkerSchema from "../../📦️bundle/🟦️bun/🧬️schema/🔣️.json" with { type: "json" };
import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import bundleSchema from "../../📦️bundle/🧬️schema/🔣️.json" with { type: "json" };
import { bundleJavaScript, bundleVirtualJavaScript, readBunBundleFiles, withWholeHostCompilerCapacity } from "../../📦️bundle/🟦️.ts";
import { scriptInvocationEnvironment } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runInNewContext } from "node:vm";
import { availableParallelism } from "node:os";
import { coordinatorBuildPolicy } from "../../../../🖥️server/🎛️coordinator/🏗️builder/🎚️mode/🟦️.ts";
import { extensionBuildConfig } from "../../../../💻️client/🧩️vscode/🏗️builder/🟦️.ts";
import assert from "node:assert/strict";
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { mkdir, mkdtemp, rm, writeFile, symlink } from "node:fs/promises";
import { createRequire } from "node:module";
import { compilerStateDirectory, withCompilerState } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗃️state/🟦️.ts";
import { buildViteArtifact, serveVite } from "../../🌐️vite/🟦️.ts";
import { typecheckProject } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import { emitTypeScriptDeclarations } from "../../📦️artifacts/🟦️typescript/🧬️declarations/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { repoCacheDirectory } from "../../🟦️.ts";
import {viteStorageLayoutV1} from "../../🌐️vite/💾️storage/🟦️.ts";
import {repositoryCacheBudgetV1} from "../../💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import { join, relative } from "node:path";

/** 🧪️ Proves browser compiler policy and state ownership against real Vite and TypeScript. */
export async function proveWebBuild(workspace: string, output: string, invocation: ScriptInvocation): Promise<void> {
 const vitePhase=scriptCompilerAdmissionControlV1(invocation,"test:vite-entry",0);
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️cases.json"), "utf8"));
  const { semioViteBuild } = await import("../../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🎚️mode/🟦️.ts");
  for (const row of fixture.modes) {
    const policy = semioViteBuild(undefined, row.mode), extension = extensionBuildConfig(workspace, "entry.ts", "out", "extension.cjs", false, row.mode), coordinator = coordinatorBuildPolicy(row.mode, 2);
    assert.equal(extension.build.minify, row.minify); assert.equal(extension.build.sourcemap, row.sourcemap);
    assert.equal(policy.rollupOptions.treeshake, row.mode === "release"); assert.equal(extension.build.rollupOptions.treeshake, row.mode === "release");
    assert.equal(coordinator.output, `.cache/b/p/m/next/${row.mode}/out`); assert.equal(coordinator.optimization.minimize, row.mode === "release"); assert.equal(coordinator.workers, 2);
    for (const key of ["minify", "cssMinify", "sourcemap"] as const) assert.equal(policy[key], row[key]);
  }
  for (const mode of fixture.invalidModes) assert.throws(() => semioViteBuild(undefined, mode));
  const stateSchema = JSON.parse(readFileSync(join(workspace, "🧰️framework/🔨️modules/🏃️process/🏗️build/🗃️state/🧬️schema/🔣️.json"), "utf8"));
  const require = createRequire(join(workspace, "package.json")), { resolveConfig } = await import("vite"), typescript = require("typescript");
  assert.equal(require("jsonschema").validate(fixture.bundle, bundleSchema).valid, true);
  const activationRoot = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation"), activation = await import(join(activationRoot, "🟦️.ts")), activationSchema = JSON.parse(readFileSync(join(activationRoot, "🧬️schema/🔣️.json"), "utf8"));
  const activationOracle = new (require("ajv").default)().addSchema(activationSchema).getSchema(`${activationSchema.$id}#/$defs/DevActivationV1`);
  for (const row of fixture.modes) {
    const receipt = { schema: "semio.dev.activation/v1", variant: "fixture", profile: row.mode, plugins: [] };
    assert.deepEqual(activation.parseActivationReceipt(receipt), receipt);
    assert.equal(activationOracle(receipt), true);
    assert.equal(activation.developmentRuntimeRoot(output, "fixture", row.mode, "react"), join(output, "dist/runtime/react", row.mode, "fixture"));
  }
  for (const profile of fixture.invalidModes) assert.throws(() => activation.parseActivationReceipt({ schema: "semio.dev.activation/v1", variant: "fixture", profile, plugins: [] }), /Invalid activation receipt/);
  const boundedRuntime = activation.developmentRuntimeRoot(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"), fixture.activation.boundedVariant, "prototype", "react");
  assert.ok(join(activation.activationSourceFreshnessDirectory(boundedRuntime, fixture.activation.boundedPluginId), fixture.activation.stageFile).length <= 256, "Every mutable source-index staging path must fit the repository path limit");


  if (availableParallelism() > 1) await assert.rejects(withWholeHostCompilerCapacity(join(workspace, ".cache/b/l/m/data"), scriptCompilerAdmissionControlV1(invocation, "web:capacity", 10_000), async () => { throw Error("Unadmitted Bun compiler started"); }, { SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: "1" }), /complete host worker reservation/);
  for (const row of fixture.modes) {
    const oracle = await resolveConfig({ configFile: false, build: semioViteBuild(undefined, row.mode) }, "build", row.mode);
    for (const key of ["minify", "cssMinify", "sourcemap"] as const) assert.equal(oracle.build[key], row[key]);
    assert.equal(oracle.build.rollupOptions.treeshake, row.mode === "release");
  }
  for (const row of fixture.modes) {
    const extension = extensionBuildConfig(workspace, "entry.ts", "out", "extension.cjs", false, row.mode), transformed = await extension.plugins[0].transform(fixture.extensionSource, "fixture.ts");
    assert.ok(transformed); assert.ok(transformed.code.includes("retainedPrototypeDeclaration")); assert.equal(transformed.code.includes("test-only"), false);
    const oracle = await require("esbuild").transform(transformed.code, { loader: "ts", format: "cjs", minify: row.mode === "release" });
    assert.ok(oracle.code.includes("retained"));
  }
  await mkdir(output, { recursive: true });
  const temporary = await mkdtemp(join(output, "web-")), app = join(temporary, "app"), stateRoot = repoCacheDirectory(workspace);
  await mkdir(app);
  await writeFile(join(app, "package.json"), '{"type":"module"}');
  await symlink(join(workspace, "node_modules"), join(app, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  await writeFile(join(app, "index.html"), '<!doctype html><script type="module" src="/entry.js"></script>');
  const immutableActivation = join(temporary, "activation-compiled"), activationRuntime = temporary, activationMarker = activation.activationSourceFreshnessDirectory(activationRuntime, fixture.activation.pluginId);
  await mkdir(immutableActivation); await writeFile(join(immutableActivation, "entry.ts"), fixture.activation.source);
  const cryptoOracle = require("next/dist/compiled/crypto-browserify").createHash("sha256").update(fixture.activation.pluginId).digest("hex").slice(0, fixture.activation.keyLength);
  assert.equal(activationMarker, join(activationRuntime, fixture.activation.freshnessDirectory, cryptoOracle));
  const { writeStagedSourceFreshness, readStagedSourceStatIndex } = await import(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🔍️freshness/🟦️.ts"));
  const activationSourceHash = await writeStagedSourceFreshness(activationMarker, immutableActivation, invocation.control.signal);
  assert.deepEqual(readdirSync(immutableActivation), ["entry.ts"]);
  assert.equal(readStagedSourceStatIndex(activationMarker)?.contentSha256, activationSourceHash);

  const nextSource = join(temporary, "next-source"), nextCompiled = join(temporary, "next-compiled"); await mkdir(nextSource); await mkdir(nextCompiled);
  for (const [name, bytes] of Object.entries(fixture.nextPublication.sources)) await writeFile(join(nextSource, name), String(bytes));
  const { nodeFileTrace } = require("next/dist/compiled/@vercel/nft"), oracleTrace = await nodeFileTrace([join(nextSource, "app.js")], { base: workspace });
  const nextTrace = { version: 1, files: [...oracleTrace.fileList].map((name: string) => relative(nextCompiled, join(workspace, name))) };
  await writeFile(join(nextCompiled, "app.js.nft.json"), JSON.stringify(nextTrace)); await writeFile(join(nextCompiled, "required-server-files.json"), JSON.stringify({ config: { typescript: {} } }));
  for (const row of fixture.modes) {
    const publication = await nextPublicationFiles(workspace, nextSource, nextCompiled, join(workspace,".cache/b/p/m/next",row.mode,"out"), row.mode, temporary, invocation.control.signal);
    for (const name of fixture.nextPublication.expected) assert.ok(publication.has(name), name);
    assert.equal(JSON.parse(readFileSync(publication.get("standalone/payload.json")!, "utf8")).answer, 42);
  }
  console.log("[DEBUG] Next standalone publication contains the actual installed SDK trace closure in both modes");
  const bunPackage = join(temporary, "bun-package"); await mkdir(bunPackage); await writeFile(join(bunPackage, "entry.ts"), fixture.bunPackage.source); await writeFile(join(bunPackage, "payload.txt"), fixture.bunPackage.payload);
  for (const row of fixture.modes) {
    const result = await bundleVirtualJavaScript({ version: 1, kind: "bun-package", mode: row.mode, entrypoint: join(bunPackage, "entry.ts") }, { workspace, invocation, control:scriptCompilerAdmissionControlV1(invocation,"test:bun-carried-phase",120000) });
    const emitted = join(bunPackage, `${row.mode}.js`); await writeFile(emitted, result.source); assert.equal((await import(emitted)).answer, fixture.bunPackage.answer);
    const oracle = await require("esbuild").build({ entryPoints: [join(bunPackage, "entry.ts")], bundle: true, write: false, format: "esm", loader: { ".txt": "text" }, plugins: [{ name: "text-oracle", setup(build: any) { build.onLoad({ filter: /entry\.ts$/ }, () => ({ contents: fixture.bunPackage.source.replace(' with { type: "text" }', ""), loader: "ts" })); } }] });
    const oracleFile = join(bunPackage, `${row.mode}-oracle.js`); await writeFile(oracleFile, oracle.outputFiles[0].text); assert.equal((await import(oracleFile)).answer, fixture.bunPackage.answer);
    assert.equal(!!result.sourceMap, row.sourcemap);
  }
  console.log("[DEBUG] Essential Bun text-import artifact matches independent esbuild loader oracle in both modes");
  const largeEntry = join(bunPackage, "large.ts"), largeOutput = join(bunPackage, "large-prototype.js"), largeOracle = join(bunPackage, "large-oracle.js");
  await writeFile(largeEntry, 'export const payload = "' + "q".repeat(fixture.largePrototype.characters) + '"; export const payloadLength = payload.length;');
  await withWholeHostCompilerCapacity(join(workspace, ".cache/b/l/m/data"), scriptCompilerAdmissionControlV1(invocation, "web:large-prototype", 600_000), async environment => {
    await bundleJavaScript({ workspace, source: largeEntry, output: largeOutput, mode: "prototype", platform: "node", runtime: "bun", environment, invocation, signal: invocation.control.signal });
    await require("esbuild").build({ entryPoints: [largeEntry], outfile: largeOracle, bundle: true, format: "esm", treeShaking: false, minify: false });
    assert.ok(readFileSync(largeOutput).byteLength > fixture.largePrototype.minimumOutputBytes);
    assert.equal((await import(largeOutput)).payloadLength, (await import(largeOracle)).payloadLength); assert.equal((await import(largeOutput)).payloadLength, fixture.largePrototype.characters);
    const request = { ...fixture.bunWorker, mode: "prototype" }, requestPath = join(bunPackage, "manifest-request.json"), responsePath = join(bunPackage, "manifest-response.json"); await writeFile(requestPath, JSON.stringify(request));
    await runOwnedCommand(process.execPath, [join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/📜️script.ts"), "debug", requestPath, responsePath], workspace, "bun:manifest-proof", 30_000, { env: scriptInvocationEnvironment(invocation, { ...environment, SEMIO_BUILD_MODE: "prototype" }), signal: invocation.control.signal });
    const manifestBytes = readFileSync(responsePath), manifest = JSON.parse(manifestBytes.toString("utf8")); assert.ok(manifestBytes.byteLength <= fixture.largePrototype.maximumManifestBytes); assert.equal(require("jsonschema").validate(manifest, bunWorkerSchema.definitions.Response).valid, true);
    const files = await readBunBundleFiles(request, responsePath, invocation.control.signal); await writeFile(files.source, "tampered"); await assert.rejects(readBunBundleFiles(request, responsePath, invocation.control.signal), /custody|digest/);
  });
  console.log(`[DEBUG] Prototype Bun artifact exceeds16MiB (${fixture.largePrototype.characters} payload characters), matches esbuild; bounded manifest schema and actual digest tampering refusal verified`);
  const wasmOwner = join(temporary, "wasm-owner"), packageRoot = join(temporary, "node_modules/@semio-tech/fixture-wasm");
  await mkdir(packageRoot, { recursive: true });
  await writeFile(join(packageRoot, "package.json"), JSON.stringify({ name: "@semio-tech/fixture-wasm", type: "module", module: "fixture.js", semioWasmOwner: { version: 1, sourceOwner: relative(workspace, wasmOwner).replaceAll("\\", "/"), mode: "release", output: "dist/wasm/release" } }));
  await writeFile(join(packageRoot, "fixture.js"), 'export const wasmIntent = "active-projection";');
  for (const row of fixture.modes) { const directory = join(wasmOwner, "dist/wasm", row.mode); await mkdir(directory, { recursive: true }); await writeFile(join(directory, "fixture.js"), `export const wasmIntent = ${JSON.stringify(row.mode)};`); }
  await writeFile(join(app, "entry.js"), 'import { wasmIntent } from "@semio-tech/fixture-wasm"; document.body.dataset.intent = wasmIntent;' + fixture.source);
  const config = join(app, "🟦️.ts"), policyPath = join(workspace, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🎚️mode/🟦️.ts");
  const wasmPluginPath = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🕸️wasm/🟦️.ts");
  await writeFile(config, `import { semioViteBuild } from ${JSON.stringify(policyPath)}; import { canonicalWasmBuildVitePlugin } from ${JSON.stringify(wasmPluginPath)}; export default ({ mode }) => ({ plugins: [canonicalWasmBuildVitePlugin(${JSON.stringify(workspace)}, mode)], publicDir: false, logLevel: "silent", cacheDir: process.env.SEMIO_VITE_CACHE_DIRECTORY, build: semioViteBuild(undefined, mode) });`);
  const options = { invocation, control:scriptCompilerAdmissionControlV1(invocation,"test:vite-entry",0), root: app, workspace, config, owner: fixture.owner };
  const sizes = new Map<string, number>();
  try {
    assert.equal(require("jsonschema").validate(fixture.bunWorker, bunWorkerSchema).valid, true);
    for (const row of fixture.modes) {
      const observed = await bundleVirtualJavaScript({ ...fixture.bunWorker, mode: row.mode }, { workspace, invocation, control:scriptCompilerAdmissionControlV1(invocation,"test:bun-carried-phase",120000), signal: invocation.control.signal });
      const load = async (source: string) => import("data:text/javascript;base64," + Buffer.from(source).toString("base64"));
      const oracle = await require("esbuild").build({ stdin: { contents: fixture.bunWorker.contents, loader: "ts" }, bundle: true, format: "esm", write: false, plugins: [{ name: "fixture-runtime", setup(build: any) { build.onResolve({ filter: /^semio:actor-/ }, (args: any) => ({ path: args.path.slice("semio:actor-".length), namespace: "fixture" })); build.onLoad({ filter: /.*/, namespace: "fixture" }, (args: any) => ({ contents: fixture.bunWorker.runtime[args.path], loader: "ts" })); } }] });
      assert.equal((await load(observed.source)).answer, (await load(oracle.outputFiles[0].text)).answer); assert.equal(observed.loadedRuntime, 2);
    }
    const cancellation = new AbortController(); let compiling = false;
    const request = { version: 1 as const, mode: "prototype" as const, kind: "actor" as const, contents: Array.from({ length: 50_000 }, (_, index) => `export function function${index}(value: number): number { return value + ${index}; }`).join("\n"), runtime: { host: "export const host = 1;", wasi: "export const wasi = 2;" } };
    await assert.rejects(bundleVirtualJavaScript(request, { workspace, invocation, control:scriptCompilerAdmissionControlV1(invocation,"test:bun-carried-phase",120000), signal: cancellation.signal, onLine: line => { if (line.includes("[bun-worker] compiling")) { compiling = true; setTimeout(() => cancellation.abort(), 20); } } }));
    assert.equal(compiling, true);
    await withWholeHostCompilerCapacity(join(workspace, ".cache/b/l/m/data"), scriptCompilerAdmissionControlV1(invocation, "web:capacity", 10_000), async () => {});
    console.log("[DEBUG] Actual Bun virtual-source compilation cancelled after worker start; process tree exited and host capacity released");
    const loadNextConfig = require("next/dist/server/config").default;
    for (const row of fixture.modes) {
      const coordinator = coordinatorBuildPolicy(row.mode, 2), nextRoot = join(temporary, `next-${row.mode}`);
      await mkdir(nextRoot);
      const oracle = await loadNextConfig("phase-production-build", nextRoot, { customConfig: { distDir: coordinator.output, productionBrowserSourceMaps: coordinator.sourceMap, experimental: { cpus: coordinator.workers, serverMinification: coordinator.optimize } } });
      assert.equal(oracle.distDir, coordinator.output); assert.equal(oracle.experimental.cpus, 2); assert.equal(oracle.experimental.serverMinification, coordinator.optimize);
    }
    for (const row of fixture.modes) {
      const started = Date.now(),destination = await buildViteArtifact({ ...options, mode: row.mode });
      const assets = readdirSync(join(destination, "assets")), js = assets.find(name => name.endsWith(".js"))!;
      assert.ok(js); assert.equal(assets.some(name => name.endsWith(".js.map")), row.sourcemap);
      const emitted = readFileSync(join(destination, "assets", js), "utf8");
      assert.ok(emitted.includes(row.mode)); assert.equal(emitted.includes("active-projection"), false);
      sizes.set(row.mode, Buffer.byteLength(emitted));
      console.log(`[DEBUG] Vite ${row.mode}: bytes=${sizes.get(row.mode)} elapsedMs=${Date.now() - started}`);
    }
    assert.ok(sizes.get("release")! < sizes.get("prototype")!);
    for (const row of fixture.modes) {
      const target = join(temporary, `esbuild-${row.mode}.js`);
      await bundleJavaScript({ workspace, source: join(app, "entry.js"), output: target, mode: row.mode, platform: "browser", external: ["@semio-tech/fixture-wasm"], signal: invocation.control.signal, invocation, environment: { ...process.env, GOMAXPROCS: "1" } });
      assert.equal(existsSync(target + ".map"), row.sourcemap);
    }
    await Promise.all([0, 1].map(() => buildViteArtifact({ ...options, mode: "prototype" })));
    assert.ok(existsSync(join(viteStorageLayoutV1(join(repositoryCacheBudgetV1(workspace).partitions.products.directory,"m"),"prototype",fixture.owner).output,".semio-artifact.json")));
    const serving = [new AbortController(), new AbortController()], addresses: string[] = [], listeners: Promise<void>[] = [];
    try {
      await Promise.all(serving.map((controller, index) => new Promise<void>((accept, reject) => {
        const running = serveVite({control:vitePhase, invocation, root: app, config, mode: "prototype", host: "127.0.0.1", port: 0, signal: controller.signal, ready: url => { addresses[index] = url; accept(); } });
        running.catch(reject); listeners.push(running);
      })));
      assert.notEqual(addresses[0], addresses[1]);
      for (const address of addresses) { const response = await fetch(address); assert.equal(response.status, 200); await response.text(); const entry = await fetch(address + "entry.js"); assert.equal(entry.status, 200); await entry.text(); }
      let warmRequest!: Promise<Response>, warmCompleted = false;
      await withWholeHostCompilerCapacity(join(workspace, ".cache/b/l/m/data"), scriptCompilerAdmissionControlV1(invocation, "web:vite-idle", 600_000), async () => {
        const cachedResponse = await fetch(addresses[0] + "entry.js", { signal: AbortSignal.any([invocation.control.signal, AbortSignal.timeout(5_000)]) }); assert.equal(cachedResponse.status, 200); await cachedResponse.text();
        await writeFile(join(app, "entry.js"), readFileSync(join(app, "entry.js"), "utf8") + '\nexport const warmedCompilerValue = "warm-update";');
        warmRequest = fetch(addresses[0] + "entry.js?semioWarmUpdate=2", { signal: invocation.control.signal }).then(response => { warmCompleted = true; return response; });
        await new Promise(accept => setTimeout(accept, 100)); assert.equal(warmCompleted, false);
      });
      const warmResponse = await warmRequest; assert.equal(warmResponse.status, 200); assert.ok((await warmResponse.text()).includes("warm-update"));
      console.log("[DEBUG] Concurrent Vite listeners served module transforms; idle listeners released host admission, cached responses stayed available, and warm transforms queued behind another graph");
    } finally { serving.forEach(controller => controller.abort()); await Promise.all(listeners); }
    const warmService = new AbortController();
    let warmed!: Promise<void>;
    await new Promise<void>((accept, reject) => {
      warmed = serveVite({control:vitePhase, invocation, root: app, config, mode: "prototype", host: "127.0.0.1", port: 0, signal: warmService.signal, ready: async url => { const response = await fetch(url); assert.equal(response.status, 200); await response.text(); accept(); } });
      warmed.catch(reject);
    });
    warmService.abort(); await warmed;
    console.log("[DEBUG] Concurrent isolated Vite listeners and warm optimizer snapshot publication verified");
    const state = { root: stateRoot, owner: fixture.owner, compiler: "test:1", configuration: config, mode: "prototype" as const };
    assert.equal(require("jsonschema").validate(state, stateSchema).valid, true);
    assert.notEqual(compilerStateDirectory(state), compilerStateDirectory({ ...state, mode: "release" }));
    assert.notEqual(compilerStateDirectory(state), compilerStateDirectory({ ...state, owner: "fixture:other" }));
    assert.notEqual(compilerStateDirectory(state), compilerStateDirectory({ ...state, compiler: "test:2" }));
    const order: string[] = [], arrivalSignal = invocation.control.signal, arrivalOwner = { ...state, owner: "fixture:opposite-state-arrival", configuration: temporary, control: scriptCompilerAdmissionControlV1(invocation, "web:arrival", 600_000), leaseDirectory: join(workspace, ".cache/b/l/m/data"), files: ["probe"] };
    const host = await acquireCompilerCapacityLeaseV1({ directory: join(workspace, ".cache/b/l/m/data"), control: scriptCompilerAdmissionControlV1(invocation, "web:capacity", 600_000) });
    const independent = withCompilerState(arrivalOwner, async () => { order.push("independent"); });
    try {
      await new Promise(accept => setTimeout(accept, 100));
      await withCompilerState({ ...arrivalOwner, environment: { ...process.env, SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: String(host.parallelism) } }, async () => { order.push("inherited"); });
    } finally { host.release(); }
    await independent; assert.deepEqual(order, fixture.lockOrder);
    console.log("[DEBUG] Opposite-arrival inherited and independent compiler graphs completed in host-before-state order");
    let active = 0, maximum = 0;
    await Promise.all([0, 1].map(() => withCompilerState({ ...state, control: scriptCompilerAdmissionControlV1(invocation, "web:state", 10_000), leaseDirectory: join(workspace, ".cache/b/l/m/data"), files: ["probe"] }, async directory => {
      active++; maximum = Math.max(maximum, active);
      await writeFile(join(directory, "probe"), String(active));
      await new Promise(resolve => setTimeout(resolve, 25)); active--;
    })));
    assert.equal(maximum, 1);
    const cancelled = new AbortController(); cancelled.abort();
    assert.equal(invocation.control.signal.aborted, false);
    const cancelledControl = scriptCompilerAdmissionControlV1(invocation, "web:local-cancel", 30_000, cancelled.signal);
    await assert.rejects(acquireCompilerCapacityLeaseV1({ directory: join(workspace, ".cache/b/l/m/data"), control: cancelledControl }), /abort/i);
    assert.equal(invocation.control.signal.aborted, false);
    await assert.rejects(async () => withCompilerState({ ...state, control: scriptCompilerAdmissionControlV1(invocation, "web:state", 10_000, cancelled.signal), leaseDirectory: join(workspace, ".cache/b/l/m/data"), files: ["probe"] }, async () => assert.fail("Cancelled compiler admitted")));
    const watchedSource = join(temporary, "watch.ts"), watchedOutput = join(temporary, "watch-output"), watchStop = new AbortController();
    await writeFile(watchedSource, fixture.watch.source);
    let published = 0, requested = 0, initialReady!: () => void, updatedReady!: () => void, queuedReady!: () => void, failureReady!: () => void, recoveryReady!: () => void;
    const initial = new Promise<void>(accept => { initialReady = accept; }), updated = new Promise<void>(accept => { updatedReady = accept; }), queued = new Promise<void>(accept => { queuedReady = accept; }), failed = new Promise<void>(accept => { failureReady = accept; }), recovered = new Promise<void>(accept => { recoveryReady = accept; });
    const watchWorker = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts");
    const watching = runOwnedCommand(process.execPath, [watchWorker, "watch-extension", watchedSource, watchedOutput], workspace, "extension:watch-regression", 600_000, { env: scriptInvocationEnvironment(invocation, { ...process.env, SEMIO_BUILD_MODE: "prototype", SEMIO_TEST_ARTIFACT_DIR: output, GOMAXPROCS: String(availableParallelism()) }), signal: AbortSignal.any([invocation.control.signal, watchStop.signal]), onLine: line => { if (line.includes("[extension-watch] failed")) failureReady(); if (line.includes("[extension-watch] waiting") && ++requested === 2) queuedReady(); if (line.includes("[extension-watch] published")) { published++; if (published === 1) initialReady(); else if (published === 2) updatedReady(); else recoveryReady(); } } });
    watching.catch(() => {});
    try {
      await Promise.race([initial, watching]);
      await withWholeHostCompilerCapacity(join(workspace, ".cache/b/l/m/data"), scriptCompilerAdmissionControlV1(invocation, "web:watch-idle", 600_000), async () => {
        await writeFile(watchedSource, fixture.watch.updatedSource); await Promise.race([queued, watching]); assert.equal(published, 1);
      });
      await Promise.race([updated, watching]);
      const observed = { exports: {} }, oracle = { exports: {} }, compiled = await require("esbuild").transform(fixture.watch.updatedSource, { loader: "ts", format: "cjs" });
      runInNewContext(readFileSync(join(watchedOutput, "extension.js"), "utf8"), { module: observed, exports: observed.exports }); runInNewContext(compiled.code, { module: oracle, exports: oracle.exports });
      assert.equal((observed.exports as { watchedValue: number }).watchedValue, (oracle.exports as { watchedValue: number }).watchedValue); assert.equal((observed.exports as { watchedValue: number }).watchedValue, fixture.watch.expected);
      await writeFile(watchedSource, fixture.watch.invalidSource); await Promise.race([failed, watching]);
      await withWholeHostCompilerCapacity(join(workspace, ".cache/b/l/m/data"), scriptCompilerAdmissionControlV1(invocation, "web:watch-failure", 600_000), async () => {});
      assert.equal(published, 2); await assert.rejects(require("esbuild").transform(fixture.watch.invalidSource, { loader: "ts" }));
      await writeFile(watchedSource, fixture.watch.updatedSource); await Promise.race([recovered, watching]);
    } finally { watchStop.abort(); await assert.rejects(watching); }
    await withWholeHostCompilerCapacity(join(workspace, ".cache/b/l/m/data"), scriptCompilerAdmissionControlV1(invocation, "web:watch-release", 600_000), async () => {});
    console.log("[DEBUG] Actual Rollup watch published warm update matching esbuild, waited behind another graph, released failed rebuild capacity, recovered, held no idle capacity, and cancelled its child tree");
    const project = join(app, "tsconfig.json");
    await writeFile(project, JSON.stringify({ compilerOptions: { target: "ES2022", module: "ESNext", types: [], noEmit: true, strict: true, skipLibCheck: true }, files: ["value.ts"] }));
    await writeFile(join(app, "value.ts"), 'export const value: number = 42;');
    const parsed = typescript.getParsedCommandLineOfConfigFile(project, {}, { ...typescript.sys, onUnRecoverableConfigFileDiagnostic: (diagnostic: unknown) => assert.fail(JSON.stringify(diagnostic)) });
    assert.deepEqual(parsed.fileNames, [join(app, "value.ts")]);
    const diagnostics: string[][] = [];
    for (const label of ["cold", "warm"]) {
      const lines: string[] = [], started = Date.now();
      await typecheckProject({ workspace, stateRoot, leaseDirectory: join(workspace, ".cache/b/l/m/data"), invocation, cwd: app, project, owner: "fixture:web-typecheck", mode: "prototype", timeoutMs: 30_000, args: ["--extendedDiagnostics"], execution: { onLine: line => lines.push(line) } });
      diagnostics.push(lines);
      console.log(`[DEBUG] TypeScript ${label}: elapsedMs=${Date.now() - started} ${lines.filter(line => /Check time|Total time/.test(line)).join("; ")}`);
    }
    assert.ok(diagnostics[0].some(line => /^Check time:/.test(line)));
    assert.equal(diagnostics[1].some(line => /^Check time:/.test(line)), false);
    await Promise.all([0, 1].map(() => typecheckProject({ workspace, stateRoot, leaseDirectory: join(workspace, ".cache/b/l/m/data"), invocation, cwd: app, project, owner: "fixture:web-typecheck", mode: "prototype", timeoutMs: 30_000 })));
    await typecheckProject({ workspace, stateRoot, leaseDirectory: join(workspace, ".cache/b/l/m/data"), invocation, cwd: app, project, owner: "fixture:web-typecheck", mode: "release", timeoutMs: 30_000 });
    const declarationsSource = join(temporary, "types-source");
    await mkdir(declarationsSource);
    for (const [name, bytes] of Object.entries(fixture.declarations.sources)) await writeFile(join(declarationsSource, name), String(bytes));
    for (const row of fixture.modes) {
      const declarationRoot = join(temporary, `types-${row.mode}`), typeRoot = join(declarationRoot, "🧬️types");
      const phase = scriptCompilerAdmissionControlV1(invocation, "web:declaration-oracle", 30_000), lease = await acquireCompilerCapacityLeaseV1({ directory: join(workspace, ".cache/b/l/m/data"), control: phase, environment: process.env, requestedParallelism: 1 });
      try {
        const count = emitTypeScriptDeclarations(join(declarationsSource, "entry.ts"), typeRoot, workspace);
        assert.equal(count, 3);
        const consumer = join(declarationRoot, "consumer.ts");
        await writeFile(consumer, fixture.declarations.consumer);
        const configuration = join(declarationRoot, "tsconfig.json");
        await writeFile(configuration, JSON.stringify({ compilerOptions: { strict: true, noEmit: true, target: "ES2022", module: "ESNext", moduleResolution: "Bundler", types: [], resolveJsonModule: true, allowSyntheticDefaultImports: true, skipLibCheck: true }, files: ["consumer.ts"] }));
        await runOwnedCommand(process.execPath, [require.resolve("typescript/bin/tsc"), "-p", configuration], declarationRoot, `declaration-consumer:${row.mode}`, compilerAdmissionBudgetV1(phase), { signal: phase.signal, env: { ...process.env, SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: "1" } });
        for (const file of readdirSync(typeRoot)) assert.ok(join(typeRoot, file).length <= 256);
        console.log(`[DEBUG] Flattened ${row.mode} declarations: ${count} modules, local type/ambient/JSON imports consumed by independent tsc`);
      } finally { lease.release(); }
    }
    await writeFile(join(app, "value.ts"), 'export const value: number = "bad";');
    await assert.rejects(() => typecheckProject({ workspace, stateRoot, leaseDirectory: join(workspace, ".cache/b/l/m/data"), invocation, cwd: app, project, owner: "fixture:web-typecheck", mode: "prototype", timeoutMs: 30_000 }));
    console.log("[DEBUG] Browser prototype/release artifacts, concurrent publication, cancellation, TypeScript warm reuse and error invalidation verified");
  } finally { await rm(temporary, { recursive: true, force: true }); }

}
