import assert from "node:assert/strict";
import { EventEmitter } from "node:events";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { nxChildEnvironment } from "../../🚀️bootstrap/📜️script.ts";

/** 🛑️ Exercises the actual coordinator after malformed process snapshots with controlled child processes. */
export async function testNxCoordinator(root: string): Promise<void> {
  const require = createRequire(import.meta.url), ts = require("typescript"), cache = join(import.meta.dir, "../..");
  const fixture = JSON.parse(readFileSync(join(cache, "🧫️fixtures/🛑️cancellation.json"), "utf8")), schema = JSON.parse(readFileSync(join(cache, "🧬️schema/🔣️.json"), "utf8"));
  assert.equal(require("jsonschema").validate(fixture, schema.$defs.NxCoordinatorFixture).valid, true);
  const source = ts.createSourceFile("nx.ts", readFileSync(join(cache, "🚀️bootstrap/📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const coordinator = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "NxScript");
  const code = ts.transpileModule(coordinator.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  for (const vector of fixture.cases) {
    const killed: number[] = [], child = Object.assign(new EventEmitter(), { pid: 1234 });
    const runtime = Object.assign(new EventEmitter(), { env: vector.environment, platform: vector.platform, exitCode: 0, kill: (pid: number, signal: number | string) => { if (!signal) throw new Error("No process"); killed.push(pid); } });
    let launchEnvironment: Record<string, string | undefined> = {};
    const Coordinator = new Function("Script", "process", "createRequire", "join", "existsSync", "resolveNxInvocation", "devToolingEnv", "orchestratorBudgetOpts", "spawnNxProcess", "stopNxProcessTree", "nxChildEnvironment", code + "; return NxScript;")(
      class { root = root; }, runtime, () => ({ resolve: (name: string) => name }), join, (path: string) => path.endsWith("node_modules/nx/package.json"), (args: string[]) => ({ args, env: {} }), (env: unknown) => env, () => ({}), (_command: string, _args: string[], options: { env: Record<string, string | undefined> }) => { launchEnvironment = options.env; return child; },
      (command: string) => { if (command === "taskkill") { killed.push(child.pid); return { status: 0 }; } if (vector.throws) throw new Error("Snapshot unavailable"); return { status: 0, stdout: vector.stdout }; }, nxChildEnvironment);
    const done = new Coordinator().run(["run", "fixture:build"]);
    assert.doesNotThrow(() => runtime.emit("SIGTERM"), vector.name);
    child.emit("close", null);
    await done;
    assert.equal(launchEnvironment.NX_WORKSPACE_DATA_DIRECTORY, vector.environment.NX_WORKSPACE_DATA_DIRECTORY ?? join(root, ".nx", "workspace-data"), vector.name);
    assert.equal(runtime.exitCode, vector.expectedExit, vector.name);
    assert.ok(killed.length > 0, `${vector.name}: owned launch process must still be stopped`);
    assert.equal(runtime.listenerCount("SIGTERM"), 0);
  }
  console.log("[DEBUG] Nx coordinator preserves explicit workspace data paths and stops owned launch processes after malformed or unavailable snapshots PASS");
}
