import assert from "node:assert/strict";
import { cp, mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, join, relative } from "node:path";
import { pathToFileURL } from "node:url";

/** 🧪️ Compares role-aware production copies with native Vite and JavaScript consumers. */
export async function testProductionBrowserArtifacts(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), directory = dirname(dirname(import.meta.dir));
  const { productionBrowserSources, productionBrowserArtifactsVitePlugin, selectProductionBrowserComponents } = await import("../../🟦️.ts");
  const cases = JSON.parse(await readFile(join(import.meta.dir, "../../🧫️fixtures/🌐️production-browser-artifacts/🔣️.json"), "utf8"));
  const schema = JSON.parse(await readFile(join(directory, "🧬️schema/🔣️.json"), "utf8"));
  const packageRoot = join(directory, "../../📦️packages/🟦️typescript");
  const project = JSON.parse(await readFile(join(packageRoot, "📋️project.json"), "utf8"));
  assert.equal(project.name, cases.completion.project);
  assert.deepEqual(project.targets.build.dependsOn, [cases.completion.prerequisite], "The generic build must delegate its complete production graph to Nx");
  assert.equal(project.targets.build.options.command, cases.completion.command);
  assert.deepEqual(project.targets.build.outputs, []);
  assert.equal(project.targets.build.cache, true);
  assert.equal(JSON.parse(await readFile(join(packageRoot, "package.json"), "utf8")).scripts.build, `bun nx run ${cases.completion.project}:build`);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launch = require("jsonc-parser").parse(await readFile(join(workspace, path), "utf8"));
    for (const target of cases.completion.launches) assert.ok(launch.configurations.some((row: any) => row.command === `bun nx run ${cases.completion.project}:${target}`), `${path}: ${target}`);
    assert.ok(!launch.configurations.some((row: any) => row.command?.startsWith(`bun nx run ${cases.completion.project}:build --`)));
  }
  assert.equal(require("jsonschema").validate(cases.plan, schema).valid, true);
  assert.deepEqual(selectProductionBrowserComponents(cases.session, "cad", "cad", cases.plan.components), cases.plan);
  for (const invalid of cases.invalidSessions) assert.throws(() => selectProductionBrowserComponents(invalid, "cad", "cad", cases.plan.components), /production/i);
  for (const invalid of cases.invalid) {
    assert.equal(require("jsonschema").validate(invalid, schema).valid, false);
    assert.throws(() => productionBrowserSources(workspace, invalid));
  }
  assert.throws(() => productionBrowserSources(workspace, { ...cases.plan, components: [cases.plan.components[0], cases.plan.components[0]] }), /Duplicate/);
  await mkdir(output, { recursive: true });
  const root = await mkdtemp(join(output, "production-browser-"));
  try {
    const { productionBrowserInputHash } = await import("../../../⚙️inputs/🟦️.ts");
    const policy = JSON.parse(await readFile(join(directory, "../⚙️inputs/🔣️.json"), "utf8"));
    assert.ok(require("jsonschema").validate(policy, JSON.parse(await readFile(join(directory, "../⚙️inputs/🧬️schema/🔣️.json"), "utf8"))).valid);
    const hash = await productionBrowserInputHash(root, cases.environment.base);
    const oracle = new Bun.CryptoHasher("sha256").update(JSON.stringify({ environment: [["VITE_SEMIO_APP_ROLE", "editor"]], files: policy.files.map((file: string) => [file, null]) })).digest("hex");
    assert.equal(hash, oracle);
    assert.notEqual(await productionBrowserInputHash(root, cases.environment.changed), hash);
    assert.equal(await productionBrowserInputHash(root, cases.environment.ignored), hash);
    assert.equal(await productionBrowserInputHash(root, { SEMIO_LOCKED_LOCALE: " de " }), await productionBrowserInputHash(root, { VITE_SEMIO_LOCKED_LOCALE: "de" }));
    assert.notEqual(await productionBrowserInputHash(root, { SEMIO_LOCKED_LOCALE: "en" }), await productionBrowserInputHash(root, { SEMIO_LOCKED_LOCALE: "de" }));
    await writeFile(join(root, cases.environment.dotenv.path), cases.environment.dotenv.bytes);
    assert.notEqual(await productionBrowserInputHash(root, cases.environment.base), hash);
    await rm(join(root, cases.environment.dotenv.path));
    const sources = productionBrowserSources(root, cases.plan);
    assert.deepEqual(sources.map(source => source.destination), cases.destinations);
    assert.ok(sources.every(source => !source.root.includes("runtime") && !source.root.includes("activation")));
    for (let index = 0; index < sources.length; index++) {
      const source = sources[index], name = index === 0 ? "io.js" : index === 1 ? "worker.js" : index === 2 ? "font.bin" : "🌉️bridge.js";
      await mkdir(source.root, { recursive: true });
      const bytes = index === 0 ? 'export const identity = "owned";' : index > 2 ? 'export { identity } from "@bytecodealliance/preview2-shim/io";' : "owned";
      await writeFile(join(source.root, name), bytes);
      await writeFile(join(source.root, ".nx-artifact.json"), JSON.stringify({ version: 1, owner: source.owner, files: [name] }));
      await writeFile(join(source.root, "📥️install.json"), JSON.stringify({ installedAt: Date.now() }));
    }
    await writeFile(join(root, "package.json"), '{"name":"browser-production","private":true,"type":"module"}');
    await writeFile(join(root, "index.html"), '<script type="module" src="/entry.js"></script>');
    await writeFile(join(root, "entry.js"), 'document.documentElement.dataset.built = "owned";');
    const { build } = await import("vite");
    const outDir = join(root, "dist"), options = { root, configFile: false as const, logLevel: "silent" as const, plugins: [productionBrowserArtifactsVitePlugin(root, cases.plan)], build: { outDir, emptyOutDir: true } };
    await build(options);
    const files = await require("fast-glob")("**/*", { cwd: outDir, dot: true, onlyFiles: true });
    assert.ok(!files.some((path: string) => path.endsWith("📥️install.json") || path.endsWith(".nx-artifact.json")));
    for (const source of sources.slice(3)) {
      const artifact = join(outDir, source.destination, "🌉️bridge.js");
      assert.equal((await import(pathToFileURL(artifact).href)).identity, "owned");
    }
    const before = await Promise.all(files.sort().map(async (file: string) => [file, (await readFile(join(outDir, file))).toString("base64")]));
    await build({ ...options, plugins: [productionBrowserArtifactsVitePlugin(root, cases.plan)], build: { ...options.build, write: false } });
    assert.deepEqual(await Promise.all(files.sort().map(async (file: string) => [file, (await readFile(join(outDir, file))).toString("base64")])), before);
    const forbidden = join(root, "unwritten");
    await build({ ...options, plugins: [productionBrowserArtifactsVitePlugin(root, cases.plan)], build: { outDir: forbidden, write: false } });
    await assert.rejects(readFile(join(forbidden, sources[0].destination, "io.js")), /ENOENT/);
    const caching = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"), published = join(root, "published");
    for (let index = 0; index < sources.length; index++) await cp(sources[index].root, join(root, "fixture-sources", String(index)), { recursive: true });
    await writeFile(join(root, "plan.json"), JSON.stringify(cases.plan));
    await writeFile(join(root, "⚙️vite.config.ts"), `import { productionBrowserArtifactsVitePlugin } from ${JSON.stringify(join(directory, "🟦️.ts"))}; import plan from "./plan.json"; import { dirname } from "node:path"; import { fileURLToPath } from "node:url"; const root = dirname(fileURLToPath(import.meta.url)); export default { root, publicDir: false, logLevel: "silent", plugins: [productionBrowserArtifactsVitePlugin(root, plan)] };`);
    await writeFile(join(root, "📜️script.ts"), `import { appendFileSync } from "node:fs"; import { cp } from "node:fs/promises"; import { buildViteArtifact } from ${JSON.stringify(join(caching, "🌐️vite/🟦️.ts"))}; if (process.argv[2] === "prepare") { const sources = ${JSON.stringify(sources)}; for (let index = 0; index < sources.length; index++) await cp(import.meta.dir + "/fixture-sources/" + index, sources[index].root, { recursive: true }); } else { appendFileSync(import.meta.dir + "/.compiler-invocations", "build\\n"); await buildViteArtifact({ root: import.meta.dir, workspace: ${JSON.stringify(workspace)}, config: import.meta.dir + "/⚙️vite.config.ts", output: import.meta.dir + "/published", owner: "fixture:production" }); }`);
    await writeFile(join(root, "nx.json"), '{"plugins":[]}');
    await writeFile(join(root, "project.json"), JSON.stringify({ name: "browser-production", targets: { prepare: { executor: "nx:run-commands", cache: true, outputs: sources.map(source => `{projectRoot}/${relative(root, source.root).replaceAll("\\", "/")}`), inputs: ["{projectRoot}/fixture-sources/**/*", "{projectRoot}/📜️script.ts"], options: { command: "bun ./📜️script.ts prepare" } }, build: { executor: "nx:run-commands", cache: true, dependsOn: ["prepare"], outputs: ["{projectRoot}/published"], inputs: [...["entry.js", "index.html", "📜️script.ts", "⚙️vite.config.ts", "plan.json"].map(file => `{projectRoot}/${file}`), { dependentTasksOutputFiles: "**/*", transitive: true }], options: { command: "bun ./📜️script.ts build" } } } }));
    const configured = JSON.parse(await readFile(join(root, "project.json"), "utf8"));
    configured.targets[cases.completion.prerequisite] = configured.targets.build;
    configured.targets.build = { ...project.targets.build, options: { command: `bun ${JSON.stringify(join(directory, "../🏁completion/📜️script.ts"))} complete` } };
    await writeFile(join(root, "project.json"), JSON.stringify(configured));
    const nx = join(dirname(require.resolve("nx/package.json")), require("nx/package.json").bin.nx), env = { ...process.env, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, REPO_ROOT: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache"), NODE_PATH: join(workspace, "node_modules"), NX_SKIP_NX_CACHE: "false", NX_SKIP_REMOTE_CACHE: "true" };
    const runNx = async () => {
      const child = Bun.spawn(["node", nx, "run", "browser-production:build", "--output-style=static"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
      const timer = setTimeout(() => child.kill("SIGKILL"), 90_000);
      try { const [status, stdout, stderr] = await Promise.all([child.exited, new Response(child.stdout).text(), new Response(child.stderr).text()]); assert.equal(status, 0, stdout + stderr); return stdout + stderr; }
      finally { clearTimeout(timer); }
    };
    await runNx();
    const inventory = async () => {
      const paths = (await require("fast-glob")("**/*", { cwd: published, dot: true, onlyFiles: true })).sort();
      return Promise.all(paths.map(async (file: string) => [file, (await readFile(join(published, file))).toString("base64")]));
    };
    const cold = await inventory();
    assert.match(await runNx(), /cache/i);
    await rm(published, { recursive: true });
    assert.match(await runNx(), /cache/i);
    assert.deepEqual(await inventory(), cold);
    assert.equal(await readFile(join(root, ".compiler-invocations"), "utf8"), "build\n");
    for (const source of sources.slice(3)) assert.equal((await import(pathToFileURL(join(published, source.destination, "🌉️bridge.js")).href)).identity, "owned");
    await writeFile(join(root, "fixture-sources/0/io.js"), 'export const identity = "updated";');
    await runNx();
    assert.equal(await readFile(join(root, ".compiler-invocations"), "utf8"), "build\nbuild\n");
    assert.notDeepEqual(await inventory(), cold);
    console.log(`[DEBUG] Production Vite copies ${sources.length} declared artifact owners, runs relocated plugin and extension imports, excludes installation metadata and respects write:false PASS`);
    console.log("[DEBUG] Generic production completion delegates to native Nx; warm and deleted-output restoration preserve every published byte without rerunning Vite, and a dependency byte change rebuilds PASS");
  } finally { await rm(root, { recursive: true, force: true }); }
}
