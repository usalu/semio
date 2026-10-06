/** 🗿️ Actual third-party Nx execution proves semantic verification invalidation and reuse. */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, symlinkSync } from "node:fs";
import { join, dirname } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import plugin from "../../../🟨️.mjs";
import { getWorkspaceRoot } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { ticketOutput } from "../../🎫️output/🟦️.ts";

test("artifact verification reruns for semantic provider, fixture, DDL and shared control edits", async () => {
  const workspace = getWorkspaceRoot(); const output = ticketOutput(workspace, []);
  const fixture = await Bun.file(new URL("../../🧫️fixtures/🗿️artifact-source/🔣️.json", import.meta.url)).json();
  const root = mkdtempSync(join(output, "artifact-source-"));
  const put = (path: string, value: unknown): void => { const file = join(root, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, typeof value === "string" ? value : JSON.stringify(value)); };
  const require = createRequire(join(workspace, "package.json"));
  put("package.json", { name: "artifact-source-verification", private: true, nx: { includedScripts: [] }, devDependencies: { typescript: require("typescript/package.json").version } });
  put("bun.lock", { lockfileVersion: 1, workspaces: { "": { name: "artifact-source-verification", devDependencies: { typescript: require("typescript/package.json").version } } }, packages: { typescript: [`typescript@${require("typescript/package.json").version}`, "", {}] } });
  put("nx.json", { useDaemonProcess: false, cacheDirectory: ".nx/cache", maxCacheSize: "32MB", plugins: [{ plugin: fileURLToPath(new URL("../../../🟨️.mjs", import.meta.url)), options: { analyzeLockfile: true } }] });
  put(".gitignore", "node_modules\n.nx\ndist\n");
  put("artifact/🟦️.ts", 'export { default as definition } from "./📜️artifact-definition.json";');
  put("artifact/📜️artifact-definition.json", { kind: "neutral.artifact" });
  put("artifact/domain/provider.json", { value: 42 }); put("artifact/🧫️fixtures/fixture.json", { label: "first" }); put("shared/control.json", { limit: 1024 });
  put("artifact/domain/schema.sql", "CREATE TABLE semantic_entity(id INTEGER PRIMARY KEY, value INTEGER);");
  put(`${fixture.packageRoot}/package.json`, { name: "artifact-source-probe", private: true });
  put(`${fixture.packageRoot}/📜️script.ts`, `import { readFileSync,writeFileSync,mkdirSync,existsSync } from "node:fs";
const state="../../dist/executions";mkdirSync("../../dist",{recursive:true});const count=existsSync(state)?Number(readFileSync(state,"utf8"))+1:1;writeFileSync(state,String(count));
const provider=JSON.parse(readFileSync("../../domain/provider.json","utf8"));const fixture=JSON.parse(readFileSync("../../🧫️fixtures/fixture.json","utf8"));const control=JSON.parse(readFileSync("../../../shared/control.json","utf8"));
console.log(JSON.stringify({execution:count,value:provider.value,label:fixture.label,limit:control.limit,sql:readFileSync("../../domain/schema.sql","utf8")}));
`);
  const authored = { name: "artifact-source-probe", tags: ["role:artifact", "language:typescript"], targets: { test: { executor: "nx:run-commands", cache: true, inputs: fixture.declaredInputs, outputs: [], options: { command: "bun ./📜️script.ts test" } } } };
  const config = `${fixture.packageRoot}/📋️project.json`; put(config, authored);
  const nodes = await plugin.createNodesV2[1]([config], {}, { workspaceRoot: root });
  const project = nodes.flatMap(([, value]) => Object.values(value.projects)).find(value => value.name === authored.name)!;
  expect(project).toBeDefined(); put(`${fixture.packageRoot}/project.json`, project);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const nx = createRequire(join(workspace, "package.json")).resolve("nx/bin/nx.js");
  const run = async (name: string): Promise<string> => {
    const env: Record<string, string | undefined> = { ...process.env, NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false", NX_TUI: "false", NX_NATIVE_COMMAND_RUNNER: "false", NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data") };
    for (const key of Object.keys(env)) if (key.startsWith("NX_TASK_") || ["NX_SKIP_NX_CACHE", "NX_SKIP_REMOTE_CACHE", "NX_FORCE_REUSE_CACHED_GRAPH"].includes(key)) delete env[key];
    const child = Bun.spawn(["node", nx, "run", "artifact-source-probe:test", "--output-style=static"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
    const timer = setTimeout(() => child.kill(), 30_000);
    const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]); clearTimeout(timer);
    writeFileSync(join(root, `${name}.log`), stdout + stderr); expect(status, stdout + stderr).toBe(0); return stdout;
  };
  const count = (): number => Number(readFileSync(join(root, "artifact/dist/executions"), "utf8"));
  await run("first"); expect(count()).toBe(1); await run("reuse"); expect(count()).toBe(1);
  for (const [index, change] of fixture.changes.entries()) { put(change.path, change.content); const stdout = await run(`edit-${index}`); expect(count()).toBe(index + 2); expect(stdout).toContain(JSON.stringify({ [change.field]: change.expected }).slice(1, -1)); }
}, { timeout: 90_000 });
