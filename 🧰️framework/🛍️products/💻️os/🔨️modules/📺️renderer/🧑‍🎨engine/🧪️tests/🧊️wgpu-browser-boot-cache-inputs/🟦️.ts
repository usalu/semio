import assert from "node:assert/strict";
import { readFileSync, writeFileSync, mkdtempSync, mkdirSync, existsSync, statSync, utimesSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve, sep } from "node:path";
import { spawnSync } from "node:child_process";

/** 🧊️ Verifies runtime selection and native compiler independence from the canonical default session. */
export async function testWgpuBootInputs(workspace: string, generated: string): Promise<void> {
  const require = createRequire(import.meta.url), ts = require("typescript"), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧊️wgpu-browser-boot-cache-inputs/🔣️.json"), "utf8"));
  const entry = resolve(import.meta.dir, "../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), text = readFileSync(entry, "utf8"), source = ts.createSourceFile(entry, text, ts.ScriptTarget.Latest, true);
  const packageRoot = resolve(entry, "../../📦️packages/🟦️typescript"), project = JSON.parse(readFileSync(join(packageRoot, "📋️project.json"), "utf8"));
  assert.equal(project.targets["check-browser-worker"].cache, true, "Generated-file freshness checks must stay cached — a repeat check without a source change is otherwise a wasted 🧵️Trunk-less bundle rebuild every single run");
  const { cacheInternals } = await import(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"));
  const bootOutput = resolve(packageRoot, "../../🚀️browser-boot/🤖️generated/🟨️.js"), frameWorkerOutput = resolve(packageRoot, "../../🎞️frame-worker/🤖️generated/🟨️.js");
  const projectRoot = packageRoot.slice(workspace.length + 1).split(sep).join("/");
  const couplingInputs = (name: string) => cacheInternals.generatorOutputCouplingInputs(name, project.targets[name], project.targets, projectRoot, workspace);
  const hashesOutput = (inputs: unknown[], path: string) => inputs.some((input: any) => typeof input === "object" && typeof input?.runtime === "string" && input.runtime.includes(JSON.stringify(path)));
  assert.ok(hashesOutput(couplingInputs("check-browser-worker"), bootOutput), "check-browser-worker must hash generate-browser-boot's declared output directly — `default` excludes every declared output project-wide, so caching this target soundly requires an explicit digest of the current bytes, not a rebuild-and-compare that a stale cache hit would skip");
  assert.ok(hashesOutput(couplingInputs("check-browser-worker"), frameWorkerOutput), "check-browser-worker must also hash generate-frame-worker's declared output directly, for the same reason");
  assert.ok(hashesOutput(couplingInputs("check-frame-worker"), frameWorkerOutput), "check-frame-worker must hash generate-frame-worker's declared output directly");
  const generator = project.targets[fixture.generator.target]; assert.ok(generator, "Browser boot needs one Nx producer");
  assert.deepEqual(generator.inputs.filter((input: unknown) => typeof input === "string" && !input.startsWith("{")), fixture.generator.inputs);
  assert.deepEqual(generator.inputs.filter((input: any) => input.dependentTasksOutputFiles), [{ dependentTasksOutputFiles: fixture.generator.dependencyOutput }]);
  assert.equal(generator.cache, true); assert.deepEqual(generator.outputs, [fixture.generator.output]);
  assert.deepEqual(generator.dependsOn.toSorted(), fixture.generator.prerequisites.toSorted());
  for (const target of fixture.generator.consumers) assert.ok(project.targets[target].dependsOn.includes(fixture.generator.target), target);
  for (const target of ["wasm", "wasm-release"]) assert.ok(!project.targets[target].dependsOn.includes(fixture.generator.target), "Rust compilation must not regenerate the separate browser entry");
  const sourceInputs = cacheInternals.declaredSourceInputs(project, workspace).browserBootSources;
  const sourceFiles = sourceInputs.filter((input: unknown) => typeof input === "string").map((input: string) => input.replace("{workspaceRoot}/", ""));
  assert.ok(sourceFiles.includes(entry.slice(workspace.length + 1)));
  assert.ok(!sourceFiles.some((path: string) => path.includes(".vscode/") || path.includes("⚡️caching/📦️artifacts/🦀️rust/") || path.endsWith("📦️packages/🟦️typescript/📜️script.ts") || path.endsWith("🤖️generated/🎮️playgrounds/🟦️.ts")));
  assert.deepEqual(sourceInputs.filter((input: any) => input.externalDependencies), [{ externalDependencies: ["typescript"] }]);
  const catalog = await import(resolve(entry, "../../../../../../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts"));
  assert.equal(catalog.DEFAULT_HOST_VARIANT, fixture.defaultVariant);
  const names = new Set(["BOOT_FIELD_CAPACITY", "LOCATION_SEARCH_CAPACITY", "bounded", "bootDescriptor"]);
  const statements = source.statements.filter((node: any) => ts.isFunctionDeclaration(node) ? names.has(node.name?.text) : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration: any) => names.has(declaration.name?.text)));
  assert.equal(statements.length, names.size);
  const runtime = ts.transpileModule(statements.map((node: any) => node.getText(source)).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText;
  const output = mkdtempSync(join(generated, "wgpu-boot-inputs-"));
  const consumer = `const vm = require("node:vm"); const fixture = ${JSON.stringify(fixture)}; const runtime = ${JSON.stringify(runtime + "\nJSON.stringify(bootDescriptor())")}; const rows = fixture.cases.map(row => vm.runInNewContext(runtime, {URLSearchParams, DEFAULT_HOST_VARIANT: fixture.defaultVariant, PLAYGROUND_SESSION: {variant: fixture.ambientVariants[0]}, window: {location: {search: row.search}}, document: {querySelector: () => row.serverVariant ? {content: row.serverVariant} : null}})).map(JSON.parse); console.log(JSON.stringify(rows));`;
  const result = spawnSync("node", ["-e", consumer], { cwd: workspace, encoding: "utf8", timeout: 10000 });
  writeFileSync(join(output, "selection.log"), result.stdout + result.stderr); assert.equal(result.status, 0, result.stderr);
  assert.deepEqual(JSON.parse(result.stdout), fixture.cases.map((row: any) => row.expected));
  const session = resolve(entry, "../../../../../../🧑‍💻dev/🤖️generated/🎮️playground-session/🟦️.ts"), filter = /\.ts$/, bundles: Record<string, string[]> = { bun: [], esbuild: [] };
  const controlContents = "export const PLAYGROUND_SESSION = { variant: 'predicate-control' };";
  let bunControlReads = 0, esbuildControlReads = 0;
  const bunControl = await Bun.build({ entrypoints: [session], target: "browser", format: "esm", plugins: [{ name: "session-predicate-control", setup(build) { build.onLoad({ filter }, (args) => { if (resolve(args.path) !== session) return; bunControlReads++; return { contents: controlContents, loader: "ts" }; }); } }] });
  assert.equal(bunControl.success, true, bunControl.logs.map(String).join("\n"));
  assert.equal(bunControlReads, 1, "Bun's anonymous-leaf predicate must intercept the canonical session control");
  await require("esbuild").build({ entryPoints: [session], absWorkingDir: workspace, bundle: true, write: false, platform: "browser", format: "esm", logLevel: "silent", plugins: [{ name: "session-predicate-control", setup(build: any) { build.onLoad({ filter }, (args: any) => { if (resolve(args.path) !== session) return; esbuildControlReads++; return { contents: controlContents, loader: "ts" }; }); } }] });
  assert.equal(esbuildControlReads, 1, "esbuild's anonymous-leaf predicate must intercept the canonical session control");
  for (const variant of fixture.ambientVariants) {
    const contents = `export const PLAYGROUND_SESSION = { variant: ${JSON.stringify(variant)} };`;
    let bunReads = 0, esbuildReads = 0;
    const bun = await Bun.build({ entrypoints: [entry], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "session-input-oracle", setup(build) { build.onLoad({ filter }, (args) => { if (resolve(args.path) !== session) return; bunReads++; return { contents, loader: "ts" }; }); } }] });
    assert.equal(bun.success, true, bun.logs.map(String).join("\n")); assert.equal(bunReads, 0, "Bun must not read the canonical default session for a WGPU boot");
    assert.equal(bun.outputs.length, 1); bundles.bun.push(await bun.outputs[0].text());
    const esbuild = await require("esbuild").build({ entryPoints: [entry], absWorkingDir: workspace, bundle: true, write: false, platform: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, metafile: true, logLevel: "silent", plugins: [{ name: "session-input-oracle", setup(build: any) { build.onLoad({ filter }, (args: any) => { if (resolve(args.path) !== session) return; esbuildReads++; return { contents, loader: "ts" }; }); } }] });
    assert.equal(esbuildReads, 0, "esbuild must not read the canonical default session for a WGPU boot");
    for (const input of Object.keys(esbuild.metafile.inputs)) if (!input.endsWith("🤖️generated/🎮️playgrounds/🟦️.ts")) assert.ok(sourceFiles.includes(input), `Untracked browser input ${input}`);
    assert.ok(Object.keys(esbuild.metafile.inputs).some(path => path.endsWith("🤖️generated/🎮️playgrounds/🟦️.ts")), "The deterministic catalog must own the fallback");
    assert.equal(esbuild.outputFiles.length, 1); bundles.esbuild.push(esbuild.outputFiles[0].text);
  }
  for (const [compiler, builds] of Object.entries(bundles)) {
    assert.equal(builds[0], builds[1], compiler);
    const syntax = spawnSync("node", ["--input-type=module", "--check"], { input: builds[0], encoding: "utf8", timeout: 10000 }); assert.equal(syntax.status, 0, syntax.stderr);
    writeFileSync(join(output, `${compiler}.log`), `Independent session overlays emitted identical parseable bundles: ${Buffer.byteLength(builds[0])} bytes\n`);
  }
  const api = await import(resolve(packageRoot, "../../⚙️browser-build/🟦️.ts"));
  const fixtureWorkspace = join(output, "workspace"), owner = project.sourceRoot;
  const fixtureRoot = join(fixtureWorkspace, owner, "📦️packages/🦀️rust"), bootSource = resolve(fixtureRoot, "../../🚀️browser-boot/🟦️.ts");
  const taxonomyPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
  for (const path of ["nx.json", "📋️project.json", "package.json", taxonomyPath]) {
    mkdirSync(resolve(fixtureWorkspace, path, ".."), { recursive: true });
    writeFileSync(join(fixtureWorkspace, path), readFileSync(join(workspace, path)));
  }
  mkdirSync(resolve(bootSource, ".."), { recursive: true });
  writeFileSync(bootSource, "export const answer = 42;\n");
  const artifact = await api.renderBrowserBoot(fixtureRoot, fixtureWorkspace); mkdirSync(resolve(artifact.path, ".."), { recursive: true }); assert.equal(existsSync(artifact.path), false);
  await assert.rejects(() => api.checkBrowserBoot(fixtureRoot, fixtureWorkspace), /missing|stale/); assert.equal(existsSync(artifact.path), false);
  writeFileSync(artifact.path, artifact.content); utimesSync(artifact.path, 1700000000, 1700000000);
  await api.checkBrowserBoot(fixtureRoot, fixtureWorkspace); assert.equal(statSync(artifact.path).mtimeMs, 1700000000000);
  writeFileSync(bootSource, "export const answer = 43;\n");
  await assert.rejects(() => api.checkBrowserBoot(fixtureRoot, fixtureWorkspace), /stale/); assert.equal(statSync(artifact.path).mtimeMs, 1700000000000);
  assert.equal(readFileSync(artifact.path, "utf8"), artifact.content);
  const scriptText = readFileSync(join(packageRoot, "📜️script.ts"), "utf8"), scriptSource = ts.createSourceFile("script.ts", scriptText, ts.ScriptTarget.Latest, true);
  const check = scriptSource.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "BrowserWorkerCheckScript").getText(scriptSource);
  assert.ok(check.includes("checkBrowserBoot(")); assert.ok(!check.includes("buildBootScript("));
  console.log("[DEBUG] Missing and stale browser boot checks preserve artifact bytes and mtime; Nx owns generation PASS");
  console.log("[DEBUG] Bun/esbuild predicate controls read the canonical default session while WGPU bundles remain independent PASS");
}
