import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, rmSync, unlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";

/** 📦️ Compares dependency synchronization with Bun and runs its uncached target through native Nx. */
export async function testDependencyBootstrap(workspace: string, output: string): Promise<void> {
  const require = createRequire(join(workspace, "package.json")), vectors = resolve(import.meta.dirname, "../../🧫️fixtures/dependency-bootstrap");
  const fixture = JSON.parse(readFileSync(join(vectors, "🔣️.json"), "utf8"));
  assert.equal(require("jsonschema").validate(fixture, JSON.parse(readFileSync(join(vectors, "🛂️schema/🔣️.json"), "utf8"))).valid, true);
  const target = JSON.parse(readFileSync(join(workspace, "📋️project.json"), "utf8")).targets[fixture.target];
  assert.equal(target.cache, false);
  assert.deepEqual(target.outputs, []);
  assert.equal(target.options.command, `bun "${fixture.entry}" ${fixture.command}`);
  const graph = await require("esbuild").build({ absWorkingDir: workspace, entryPoints: [fixture.entry], bundle: true, write: false, metafile: true, platform: "node", packages: "external", format: "esm", logLevel: "silent" });
  for (const source of Object.keys(graph.metafile.inputs)) assert.notEqual(resolve(workspace, source), join(workspace, "📜️script.ts"));
  for (const value of Object.values(graph.metafile.outputs) as { imports: { path: string }[] }[]) for (const dependency of value.imports) assert.ok(dependency.path.startsWith("node:"), dependency.path);
  const root = mkdtempSync(join(output, "dependency-bootstrap-")), env = { ...process.env, NX_WORKSPACE_ROOT_PATH: root, NX_CACHE_DIRECTORY: join(root, ".nx/cache"), NX_WORKSPACE_ROOT: root, REPO_ROOT: root, NX_DAEMON: "false", NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data"), NODE_PATH: join(workspace, "node_modules"), NO_COLOR: "1", FORCE_COLOR: "0" };
  mkdirSync(join(root, "dependency"));
  writeFileSync(join(root, "dependency/package.json"), JSON.stringify({ name: fixture.dependency.name, version: fixture.dependency.version, type: "module", main: "index.js" }));
  writeFileSync(join(root, "dependency/index.js"), fixture.dependency.content);
  writeFileSync(join(root, "package.json"), JSON.stringify({ name: "bootstrap-fixture", private: true, dependencies: { [fixture.dependency.name]: "file:./dependency" } }));
  writeFileSync(join(root, "nx.json"), JSON.stringify({ useDaemonProcess: false }));
  writeFileSync(join(root, "project.json"), JSON.stringify({ name: "workspace", targets: { [fixture.target]: target } }));
  writeFileSync(join(root, "📜️script.ts"), "throw new Error('Application code must not run during dependency setup');");
  for (const source of Object.keys(graph.metafile.inputs)) { const destination = join(root, source); mkdirSync(dirname(destination), { recursive: true }); copyFileSync(join(workspace, source), destination); }
  const run = async (name: string, argv: string[], expected = 0) => {
    const child = Bun.spawn(argv, { cwd: root, env, stdout: "pipe", stderr: "pipe" }), timer = setTimeout(() => child.kill("SIGKILL"), 90000);
    try {
      const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
      writeFileSync(join(root, name + ".log"), stdout + stderr);
      assert.equal(status, expected, `${name}: ${stdout}\n${stderr}`);
      return stdout + stderr;
    } finally { clearTimeout(timer); }
  };
  await run("native-bun", [process.execPath, "install"]);
  await run("native-frozen-bun", [process.execPath, ...fixture.arguments]);
  const expected = readFileSync(join(root, "node_modules", fixture.dependency.name, "index.js"), "utf8"), lock = readFileSync(join(root, "bun.lock"), "utf8");
  for (let attempt = 0; attempt < 2; attempt++) {
    rmSync(join(root, "node_modules"), { recursive: true });
    const log = await run(`nx-sync-${attempt}`, ["node", require.resolve("nx/bin/nx.js"), "run", `workspace:${fixture.target}`, "--output-style=stream"]);
    assert.ok(!log.includes("[local cache]"));
    assert.equal(readFileSync(join(root, "node_modules", fixture.dependency.name, "index.js"), "utf8"), expected);
    assert.equal(readFileSync(join(root, "bun.lock"), "utf8"), lock);
  }
  console.log("[DEBUG] Native Nx runs uncached frozen Bun synchronization without application imports and restores removed dependencies twice PASS");
}

/** 🛠️ Acquires only pinned Nx tooling into an empty checkout and compares its CLI to native Nx. */
export async function testNxTooling(workspace: string, output: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../🧫️fixtures/dependency-bootstrap/🔣️.json"), "utf8")), require = createRequire(join(workspace, "package.json"));
  const source = join(workspace, fixture.tooling.path), manifest = JSON.parse(readFileSync(join(source, "package.json"), "utf8"));
  assert.deepEqual(Object.keys(manifest.dependencies).sort(), fixture.tooling.packages);
  for (const version of Object.values(manifest.dependencies)) assert.match(version as string, /^\d+\.\d+\.\d+$/);
  assert.equal(manifest.packageManager, JSON.parse(readFileSync(join(workspace, "package.json"), "utf8")).packageManager);
  const { provisionNxTools, activateNxTools } = await import(join(source, "📜️script.ts"));
  const root = mkdtempSync(join(output, "nx-tooling-"));
  mkdirSync(join(root, fixture.tooling.path), { recursive: true });
  for (const file of ["package.json", "bun.lock"]) copyFileSync(join(source, file), join(root, fixture.tooling.path, file));
  for (const path of Object.values(manifest.patchedDependencies) as string[]) { mkdirSync(dirname(join(root, path)), { recursive: true }); copyFileSync(join(workspace, path), join(root, path)); }
  writeFileSync(join(root, "package.json"), JSON.stringify({ name: "empty-checkout", private: true, packageManager: manifest.packageManager, dependencies: { "not-an-installable-application-package": "0.0.0" } }));
  writeFileSync(join(root, "nx.json"), "{}");
  const controller = new AbortController(), installation = await provisionNxTools(root, controller.signal);
  await activateNxTools(root, installation, controller.signal);
  assert.equal(realpathSync(join(root, ".nx/installation")), dirname(installation.modulePath));
  assert.equal(existsSync(join(root, "node_modules")), false);
  const native = Bun.spawnSync(["node", require.resolve("nx/bin/nx.js"), "--version"], { cwd: workspace, env: { ...process.env, NX_DAEMON: "false" }, stdout: "pipe", stderr: "pipe" });
  const foreign = mkdtempSync(join(output, "nx-tooling-foreign-"));
  for (const folder of ["data", "cache"]) { mkdirSync(join(foreign, folder)); writeFileSync(join(foreign, folder, "sentinel.txt"), fixture.tooling.storage.sentinel); }
  writeFileSync(join(foreign, "data/file-map.json"), fixture.tooling.storage.sentinel);
  const inherited = { ...process.env, NX_WORKSPACE_DATA_DIRECTORY: join(foreign, "data"), NX_CACHE_DIRECTORY: join(foreign, "cache") };
  const env = { ...inherited, NX_WORKSPACE_DATA_DIRECTORY: join(root, fixture.tooling.storage.data), NX_CACHE_DIRECTORY: join(root, fixture.tooling.storage.cache), NODE_PATH: installation.modulePath, NX_DAEMON: "false", NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_ROOT: root, REPO_ROOT: root };
  const actual = Bun.spawnSync(["node", installation.cli, "--version"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
  assert.equal(actual.exitCode, 0, actual.stderr.toString());
  assert.ok(actual.stdout.toString().includes(manifest.dependencies.nx));
  assert.ok(native.stdout.toString().includes(manifest.dependencies.nx));
  const bootstrap = fixture.tooling.path.replace(/\/🛠️tools$/, "/📜️script.ts"), library = fixture.tooling.path.split("/⚡️caching/")[0];
  const eager = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../🧫️fixtures/nx-bootstrap/🔣️.json"), "utf8")).eagerSources;
  for (const path of [...eager, fixture.entry, fixture.tooling.path + "/📜️script.ts", library + "/⚡️caching/🔒️leases/🟦️.ts"]) { mkdirSync(dirname(join(root, path)), { recursive: true }); copyFileSync(join(workspace, path), join(root, path)); }
  writeFileSync(join(root, "📜️script.ts"), "throw new Error('Application code must not run during Nx acquisition');");
  const publicRun = Bun.spawn([process.execPath, join(root, bootstrap), "nx", "--version"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
  const [stdout, stderr, status] = await Promise.all([new Response(publicRun.stdout).text(), new Response(publicRun.stderr).text(), publicRun.exited]);
  writeFileSync(join(root, "public-bootstrap.log"), stdout + stderr);
  assert.equal(status, 0, stderr);
  assert.equal(stdout.trim(), actual.stdout.toString().trim());
  mkdirSync(join(root, "dependency"));
  writeFileSync(join(root, "dependency/package.json"), JSON.stringify({ name: fixture.dependency.name, version: fixture.dependency.version, type: "module", main: "index.js" }));
  writeFileSync(join(root, "dependency/index.js"), fixture.dependency.content);
  writeFileSync(join(root, "package.json"), JSON.stringify({ name: "empty-checkout", private: true, packageManager: manifest.packageManager, workspaces: ["dependency"], dependencies: { [fixture.dependency.name]: "workspace:*" } }));
  const target = JSON.parse(readFileSync(join(workspace, "📋️project.json"), "utf8")).targets[fixture.target];
  writeFileSync(join(root, "project.json"), JSON.stringify({ name: "workspace", targets: { [fixture.target]: target } }));
  const oracle = Bun.spawnSync([process.execPath, "install", "--lockfile-only"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
  assert.equal(oracle.exitCode, 0, oracle.stderr.toString());
  const lock = readFileSync(join(root, "bun.lock"), "utf8");
  for (const overrides of [[], ["--", "--force"]]) {
    const command = Bun.spawn([process.execPath, join(root, bootstrap), "nx", "run", `workspace:${fixture.target}`, "--output-style=stream", ...overrides], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
    const [out, err, code] = await Promise.all([new Response(command.stdout).text(), new Response(command.stderr).text(), command.exited]);
    writeFileSync(join(root, overrides.length ? "public-rejected-override.log" : "public-dependency-sync.log"), out + err);
    assert.equal(code, overrides.length ? 1 : 0, out + err);
    assert.equal(readFileSync(join(root, "node_modules", fixture.dependency.name, "index.js"), "utf8"), fixture.dependency.content);
    assert.equal(readFileSync(join(root, "bun.lock"), "utf8"), lock);
  }
  for (const path of fixture.tooling.graphSources) { mkdirSync(dirname(join(root, path)), { recursive: true }); copyFileSync(join(workspace, path), join(root, path)); }
  copyFileSync(join(root, "project.json"), join(root, "📋️project.json"));
  unlinkSync(join(root, "project.json"));
  const graphRun = Bun.spawn([process.execPath, join(root, bootstrap), "nx", "run", `workspace:${fixture.target}`, "--output-style=stream"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
  const [graphOut, graphErr, graphStatus] = await Promise.all([new Response(graphRun.stdout).text(), new Response(graphRun.stderr).text(), graphRun.exited]);
  writeFileSync(join(root, "public-repository-plugins.log"), graphOut + graphErr);
  assert.equal(graphStatus, 0, graphOut + graphErr);
  assert.equal(readFileSync(join(foreign, "data/file-map.json"), "utf8"), fixture.tooling.storage.sentinel, "A fixture must not inherit another workspace's graph storage");
  assert.deepEqual(readdirSync(join(foreign, "cache")), ["sentinel.txt"]);
  assert.ok(existsSync(join(root, fixture.tooling.storage.data, "project-graph.json")));
  assert.equal(readFileSync(join(root, "bun.lock"), "utf8"), lock);
  const before = readdirSync(dirname(installation.modulePath)).sort();
  assert.deepEqual(await provisionNxTools(root, controller.signal), installation);
  assert.deepEqual(readdirSync(dirname(installation.modulePath)).sort(), before);
  controller.abort();
  await assert.rejects(provisionNxTools(root, controller.signal));
  assert.equal(existsSync(join(root, "node_modules/nx")), false);
  unlinkSync(join(root, ".nx/installation"));
  mkdirSync(join(root, ".nx/installation"));
  writeFileSync(join(root, ".nx/installation/source.txt"), "foreign installation");
  await assert.rejects(activateNxTools(root, installation, new AbortController().signal), /another owner/);
  assert.equal(readFileSync(join(root, ".nx/installation/source.txt"), "utf8"), "foreign installation");
  console.log("[DEBUG] Empty checkout keeps native graph/cache storage private, preserves foreign storage, acquires only frozen Nx tooling; public Nx uses repository plugins to synchronize application dependencies, preserves the lock, rejects overrides, reuses tooling and protects foreign directories PASS");
}

/** 🛑️ Cancels a real installer tree whose two processes deliberately ignore graceful termination. */
export async function testDependencyCancellation(workspace: string, output: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../🧫️fixtures/dependency-bootstrap/🔣️.json"), "utf8"));
  const { runBun } = await import(join(workspace, fixture.entry));
  const root = mkdtempSync(join(output, "dependency-cancellation-")), script = join(root, "📜️script.ts"), ready = join(root, "ready.json");
  writeFileSync(script, `import { existsSync, writeFileSync, renameSync } from "node:fs"; import { join } from "node:path";
process.on("SIGTERM", () => {});
const root = process.argv[3], publish = (name, value) => { const path = join(root, name); writeFileSync(path + ".stage", value); renameSync(path + ".stage", path); };
if (process.argv[2] === "child") publish("child.ready", String(process.pid));
else { const child = Bun.spawn([process.execPath, import.meta.path, "child", root], { stdout: "ignore", stderr: "ignore" }); while (!existsSync(join(root, "child.ready"))) await Bun.sleep(10); publish("ready.json", JSON.stringify({ installer: process.pid, child: child.pid })); }
setInterval(() => {}, 1000);
`);
  const controller = new AbortController(), result = runBun([script, "installer", root], root, controller.signal).then(() => undefined, (error: unknown) => error);
  try {
    const deadline = Date.now() + fixture.cancellation.timeoutMs;
    while (!existsSync(ready) && Date.now() < deadline) await Bun.sleep(20);
    assert.ok(existsSync(ready), "Installer tree readiness timed out");
    const pids = JSON.parse(readFileSync(ready, "utf8"));
    for (const actor of fixture.cancellation.actors) assert.ok(Number.isSafeInteger(pids[actor]) && pids[actor] > 0);
    controller.abort();
    assert.ok(await result instanceof Error);
    const alive = (pid: number) => { try { process.kill(pid, 0); return true; } catch { return false; } };
    const stopped = Date.now() + fixture.cancellation.timeoutMs;
    while (Object.values(pids).some(pid => alive(pid as number)) && Date.now() < stopped) await Bun.sleep(20);
    for (const actor of fixture.cancellation.actors) assert.equal(alive(pids[actor]), false, `${actor} survived cancellation`);
    if (process.platform !== "win32") for (const actor of fixture.cancellation.actors) {
      const oracle = Bun.spawnSync(["ps", "-p", String(pids[actor]), "-o", "pid="], { stdout: "pipe", stderr: "pipe" });
      assert.equal(oracle.stdout.toString().trim(), "", `${actor} remains in the native process table`);
    }
    console.log("[DEBUG] Real installer and child ignore SIGTERM, then owned process-tree cancellation removes both from the native process table PASS");
  } finally { controller.abort(); await result; }
}
