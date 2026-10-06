import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, symlinkSync, rmSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { createRequire } from "node:module";
import { relativeSourceInputs, readSourceInputContract } from "../../../🕸️dependencies/🟦️typescript/🟨️.mjs";

/** 🧬️ Verifies one physical producer per semantic generator output in the native Nx task graph. */
export async function testGeneratorOwnership(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧬️generator-ownership/🔣️.json"), "utf8"));
  const root = mkdtempSync(join(output, "generator-ownership-"));
  const put = (path: string, value: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), value); };
  const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
  put(`${library}/🔣️taxonomy.json`, JSON.stringify({ generatorContracts: { fixture: fixture.contract } }));
  for (const [path, content] of Object.entries(fixture.files)) put(path, String(content));
  for (const [path, project] of Object.entries(fixture.projects)) {
    put(`${path}/📋️project.json`, JSON.stringify(project));
  }
  const plugin = (await import("../../../🟨️.mjs")).default;
  const files = Object.keys(fixture.projects).map(path => `${path}/📋️project.json`);
  const results = await plugin.createNodesV2[1](files, {}, { workspaceRoot: root });
  const projects: Record<string, any> = Object.assign({}, ...results.map(([, result]: any) => result.projects));
  for (const [id, expected] of Object.entries(fixture.expected)) {
    const [project, target] = id.split(":");
    assert.deepEqual(projects[project].targets[target].outputs, (expected as string[]).map(path => `{workspaceRoot}/${path}`), `Exactly one producer must own ${id}'s files`);
  }
  const graph = { nodes: Object.fromEntries(Object.entries(projects).map(([name, data]) => [name, { name, type: "lib", data }])), dependencies: Object.fromEntries(Object.keys(projects).map(name => [name, []])) };
  const tasks = require("nx/src/tasks-runner/create-task-graph").createTaskGraph(graph, {}, ["package"], ["generate"], undefined, {}, false);
  assert.deepEqual(tasks.dependencies["package:generate"].sort(), ["browser:generate-boot", "browser:generate-worker"]);
  assert.ok(projects.package.targets.generate.inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
  await plugin.createDependencies({}, { workspaceRoot: root, projects });
  assert.deepEqual(await plugin.createDependencies({}, { workspaceRoot: root, projects: {} }), []);
  const invalid = structuredClone(fixture.contract);
  invalid.outputRoots[0].producer.ownerPath = "missing";
  put(`${library}/🔣️taxonomy.json`, JSON.stringify({ generatorContracts: { fixture: invalid } }));
  await assert.rejects(async () => plugin.createDependencies({}, { workspaceRoot: root, projects }), /no Nx producer/);
  put(`${library}/🔣️taxonomy.json`, JSON.stringify({ generatorContracts: { fixture: fixture.contract } }));
  put("package.json", JSON.stringify({ private: true, name: "generator-ownership-fixture" }));
  put("nx.json", JSON.stringify({ useDaemonProcess: false, namedInputs: { sharedGlobals: [] }, cacheDirectory: ".nx/cache" }));
  put(".gitignore", "node_modules\n.nx\ndist\n.runs\n");
  for (const project of Object.values(projects)) put(`${project.root}/project.json`, JSON.stringify(project));
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js");
  const execute = async () => {
    const child = Bun.spawn(["node", cli, "run", "package:generate", "--outputStyle=static"], { cwd: root, env: { ...process.env, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache") }, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    assert.equal(code, 0, stdout + stderr);
  };
  const runs = () => readFileSync(join(root, ".runs"), "utf8").trim().split("\n");
  await execute();
  const cold = runs();
  assert.deepEqual([...cold].sort(), ["generate", "generate-boot", "generate-worker"]);
  const bytes = Object.values(fixture.expected).flat().map((path: any) => [path, readFileSync(join(root, path), "utf8")]);
  await execute(); assert.deepEqual(runs(), cold);
  rmSync(join(root, "dist"), { recursive: true });
  await execute(); assert.deepEqual(runs(), cold);
  for (const [path, content] of bytes) assert.equal(readFileSync(join(root, path), "utf8"), content);
  put("worker.ts", fixture.changedWorker);
  await execute();
  assert.deepEqual(runs().slice(cold.length), ["generate-worker", "generate"]);
  assert.equal(readFileSync(join(root, "dist/worker.js"), "utf8"), fixture.changedWorker);
  assert.equal(readFileSync(join(root, "dist/boot.js"), "utf8"), fixture.files["boot.ts"]);
}

/** 🧊️ Checks exclusive real WGPU producers against the canonical package projection and Bun's file reader. */
export async function testWgpuGeneratorOwnership(workspace: string): Promise<void> {
  const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
  const taxonomy = JSON.parse(readFileSync(join(workspace, library, "🔣️taxonomy.json"), "utf8"));
  const contract = taxonomy.generatorContracts["wgpu-frame-worker"], profile = contract.packageGeneration.browserProfile;
  const { renderWgpuPackageArtifacts } = await import(join(workspace, profile.ownerPath, "📽️projection/🟦️.ts"));
  const full = await renderWgpuPackageArtifacts(workspace);
  const partitioned: any[] = [];
  for (const producerTarget of [...new Set(contract.outputRoots.map((output: any) => output.producer?.target ?? contract.target))]) {
    const rendered = await renderWgpuPackageArtifacts(workspace, { producerTarget });
    assert.deepEqual(rendered.nodes.map((node: any) => node.path), contract.outputRoots.filter((output: any) => (output.producer?.target ?? contract.target) === producerTarget).map((output: any) => output.path));
    partitioned.push(...rendered.nodes);
  }
  assert.deepEqual(partitioned.sort((a, b) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path))), full.nodes);
  await assert.rejects(() => renderWgpuPackageArtifacts(workspace, { producerTarget: "missing:generate" }), /owns no outputs/);
  const observed = new Set<string>();
  for (const entry of profile.entries) {
    const result = await Bun.build({ entrypoints: [join(workspace, profile.ownerPath, entry.sourceRelativePath)], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "native-wgpu-input-oracle", setup(build) { build.onLoad({ filter: /.*/ }, args => { observed.add(args.path.slice(workspace.length + 1).replaceAll("\\", "/")); return undefined; }); } }] });
    assert.equal(result.success, true, result.logs.map(String).join("\n"));
  }
  assert.deepEqual([...observed].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b))), profile.sourceModulePaths);
  const project = JSON.parse(readFileSync(join(workspace, profile.ownerPath, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  const browserSources = new Set(relativeSourceInputs(readSourceInputContract(join(workspace, profile.ownerPath, "⚙️browser-build/🔣️.json")), workspace).files.map((path: string) => relative(workspace, path).replaceAll("\\", "/")));
  for (const source of profile.sourceModulePaths) {
    if (source.includes("/🤖️generated/")) assert.ok(project.targets["generate-frame-worker"].inputs.some((input: any) => input.dependentTasksOutputFiles === "**/" + source.split("/🤖️generated/")[1]));
    else assert.ok(project.namedInputs.frameWorkerSources.includes("{workspaceRoot}/" + source) || browserSources.has(source), source);
  }
}

/** 📦️ Exercises real WGPU bytes through native Nx cold publication, reuse, restoration and source invalidation. */
export async function testWgpuGeneratorPublication(workspace: string, output: string): Promise<void> {
  const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", taxonomyPath = library + "/🔣️taxonomy.json";
  const taxonomy = JSON.parse(readFileSync(join(workspace, taxonomyPath), "utf8")), contract = taxonomy.generatorContracts["wgpu-frame-worker"], owner = contract.packageGeneration.browserProfile.ownerPath;
  const projection = await import(join(workspace, owner, "📽️projection/🟦️.ts"));
  const root = mkdtempSync(join(output, "wgpu-publication-"));
  const put = (path: string, content: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); };
  const producers = { package: contract.target, worker: "@semio-tech/framework-renderer-wgpu:generate-frame-worker", boot: "@semio-tech/framework-renderer-wgpu:generate-browser-boot", library: "@semio-tech/framework-renderer-wgpu:generate-renderer-boot" }, targets: Record<string, any> = {};
  const expected = new Map<string, string>();
  for (const [name, producerTarget] of Object.entries(producers)) {
    const rendered = await projection.renderWgpuPackageArtifacts(workspace, { producerTarget });
    for (const path of rendered.inputs) put(path, readFileSync(join(workspace, path), "utf8"));
    for (const node of rendered.nodes) expected.set(node.path, node.content);
    targets[name] = { executor: "nx:run-commands", cache: true, inputs: [...rendered.inputs.map((path: string) => "{workspaceRoot}/" + path), "{workspaceRoot}/📜️script.ts", "{workspaceRoot}/📋️project.json", "{workspaceRoot}/" + taxonomyPath, ...(name === "package" ? [{ dependentTasksOutputFiles: "**/*" }] : [])], outputs: rendered.nodes.map((node: any) => "{workspaceRoot}/" + node.path), ...(name === "package" ? { dependsOn: ["worker", "boot", "library"] } : {}), options: { cwd: ".", command: "bun ./📜️script.ts " + name } };
  }
  put(taxonomyPath, JSON.stringify(taxonomy));
  put("📋️project.json", readFileSync(join(workspace, "📋️project.json"), "utf8"));
  put("package.json", JSON.stringify({ private: true, name: "wgpu-publication-fixture", packageManager: JSON.parse(readFileSync(join(workspace, "package.json"), "utf8")).packageManager }));
  put("project.json", JSON.stringify({ name: "publication", targets }));
  put("nx.json", JSON.stringify({ useDaemonProcess: false, cacheDirectory: ".nx/cache" }));
  put(".gitignore", ["node_modules", ".nx", ".runs", ...expected.keys()].join("\n") + "\n");
  put("📜️script.ts", `import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { generateFrameWorker } from ${JSON.stringify(join(workspace, owner, "🎞️frame-worker/🏗️builder/🟦️.ts"))};
import { renderBrowserBoot } from ${JSON.stringify(join(workspace, owner, "⚙️browser-build/🟦️.ts"))};
import { runWgpuPackageGenerator } from ${JSON.stringify(join(workspace, owner, "📦️publication/🟦️.ts"))};
const root = process.cwd(), bundle = join(root, ${JSON.stringify(owner)}, "📦️packages/🦀️rust"), name = process.argv[2];
if (name === "worker") await generateFrameWorker(bundle);
else if (name === "boot" || name === "library") { const artifact = await renderBrowserBoot(bundle, root, name === "library" ? "renderer-boot" : "browser-boot"); mkdirSync(dirname(artifact.path), { recursive: true }); writeFileSync(artifact.path, artifact.content); }
else if (name === "package") await runWgpuPackageGenerator(root, "generate");
else throw new Error("Unknown fixture producer");
appendFileSync(join(root, ".runs"), name + "\\n");
`);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const require = createRequire(import.meta.url), cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js");
  const run = async () => {
    const child = Bun.spawn(["node", cli, "run", "publication:package", "--outputStyle=static"], { cwd: root, env: { ...process.env, SEMIO_REPO_ROOT: root, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache") }, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    assert.equal(status, 0, stdout + stderr);
  };
  const runs = () => readFileSync(join(root, ".runs"), "utf8").trim().split("\n");
  await run(); const cold = runs(); assert.deepEqual([...cold].sort(), ["boot", "library", "package", "worker"]);
  for (const [path, bytes] of expected) assert.equal(readFileSync(join(root, path), "utf8"), bytes, path);
  await run(); assert.deepEqual(runs(), cold);
  for (const path of expected.keys()) rmSync(join(root, path));
  await run(); assert.deepEqual(runs(), cold);
  for (const [path, bytes] of expected) assert.equal(readFileSync(join(root, path), "utf8"), bytes, path);
  const entry = owner + "/🎞️frame-worker/🟦️.ts";
  put(entry, readFileSync(join(root, entry), "utf8") + '\nconsole.log("wgpu-publication-source-change");\n');
  await run(); assert.deepEqual(runs().slice(cold.length), ["worker", "package"]);
  for (const [path, bytes] of expected) if (!path.includes("/🎞️frame-worker/")) assert.equal(readFileSync(join(root, path), "utf8"), bytes, path);
}
