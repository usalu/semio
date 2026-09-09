import assert from "node:assert/strict";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, relative, resolve } from "node:path";

/** 🚀️ Compares the public bootstrap with pinned native Nx while unrelated sources are invalid. */
export async function testNxBootstrap(workspace: string, output: string): Promise<void> {
  const require = createRequire(join(workspace, "package.json")), vectorsRoot = resolve(import.meta.dirname, "../../🧫️fixtures/nx-bootstrap");
  const fixture = JSON.parse(readFileSync(join(vectorsRoot, "🔣️.json"), "utf8"));
  assert.equal(require("jsonschema").validate(fixture, JSON.parse(readFileSync(join(vectorsRoot, "🛂️schema/🔣️.json"), "utf8"))).valid, true);
  const root = mkdtempSync(join(output, "nx-bootstrap-"));
  writeFileSync(join(root, "package.json"), JSON.stringify({ name: "workspace", private: true }));
  writeFileSync(join(root, "nx.json"), "{}");
  for (const source of fixture.unrelatedSources) { const path = join(root, source.path); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, source.content); }
  for (const source of fixture.eagerSources) { const path = join(root, source); mkdirSync(dirname(path), { recursive: true }); copyFileSync(join(workspace, source), path); }
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const entry = JSON.parse(readFileSync(join(workspace, "package.json"), "utf8")).scripts.nx.match(/^bun (\.\/[^\s]*📜️script\.ts) nx$/)?.[1];
  assert.ok(entry, "The public Nx alias must select one repository bootstrap script");
  const graph = await require("esbuild").build({ absWorkingDir: workspace, entryPoints: [entry], bundle: true, write: false, metafile: true, packages: "external", platform: "node", format: "esm", logLevel: "silent" });
  assert.deepEqual(Object.keys(graph.metafile.inputs).map(path => relative(workspace, resolve(workspace, path)).replaceAll("\\", "/")).sort(), [...fixture.eagerSources].sort());
  for (const output of Object.values(graph.metafile.outputs) as { imports: { path: string; external: boolean }[] }[]) for (const dependency of output.imports) assert.ok(dependency.external && dependency.path.startsWith("node:"), `Non-system bootstrap import: ${dependency.path}`);
  const env = { ...process.env, NX_WORKSPACE_ROOT_PATH: root, NX_CACHE_DIRECTORY: join(root, ".nx/cache"), NX_WORKSPACE_ROOT: root, REPO_ROOT: root, NX_DAEMON: "false", NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data"), NO_COLOR: "1", FORCE_COLOR: "0" };
  const run = async (name: string, argv: string[]) => {
    const child = Bun.spawn(argv, { cwd: root, env, stdout: "pipe", stderr: "pipe" });
    const timer = setTimeout(() => child.kill("SIGKILL"), 30000);
    try {
      const [stdout, stderr, exit] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
      writeFileSync(join(root, name + ".log"), stdout + stderr);
      assert.equal(exit, fixture.expectedExit, `${name}: ${stderr}`);
      return stdout.replace(/\u001b\[[0-9;]*m/g, "").trim();
    } finally { clearTimeout(timer); }
  };
  const native = await run("native", ["node", require.resolve("nx/bin/nx.js"), ...fixture.arguments]);
  const program = join(root, entry);
  assert.equal(await run("public", [process.execPath, program, "nx", ...fixture.arguments]), native);
  console.log("[DEBUG] Public Nx bootstrap matches pinned native Nx with poisoned application sources and taxonomy PASS");
}
