import {createScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, existsSync, symlinkSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { setTimeout as delay } from "node:timers/promises";

/** 🔐️ Exercises the installed Nx cache against a real source write while its child is running. */
export async function testNxSourceCacheCustody(workspace: string, output: string): Promise<void> {
  output=resolve(output);
  const row = JSON.parse(readFileSync(new URL("../../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).cacheCustody;
  const root = mkdtempSync(join(output, "s-")), cli = resolve(workspace, ".nx/installation/node_modules/nx/dist/bin/nx.js");
  try {
    mkdirSync(join(root, ".nx"), { recursive: true });
    symlinkSync(join(workspace, ".nx/installation"), join(root, ".nx/installation"), process.platform === "win32" ? "junction" : "dir");
    symlinkSync(join(workspace, "🧰️framework"), join(root, "🧰️framework"), process.platform === "win32" ? "junction" : "dir");
    const contract = JSON.stringify({ version: 1, files: ["source.json", "📜️script.ts"] }), key = createHash("sha256").update(contract).digest("hex"), inventory = join(root, ".🧬semio/🦑️repo/⚡️cache/build-input-closures");
    mkdirSync(inventory, { recursive: true }); writeFileSync(join(inventory, key + ".json"), contract);
    writeFileSync(join(root, "nx.json"), JSON.stringify({ useDaemonProcess: false, plugins: [], tasksRunnerOptions: { default: { runner: "nx/tasks-runners/default", options: {} } } }));
    writeFileSync(join(root, "package.json"), JSON.stringify({ name: "source-custody-fixture", private: true, scripts: { build: "nx run fixture:build" } }));
    writeFileSync(join(root, "project.json"), JSON.stringify({ name: "fixture", root: ".", targets: { build: { executor: "nx:run-commands", cache: true, inputs: [{ env: "SEMIO_SOURCE_CLOSURE_" + key }], outputs: ["{projectRoot}/out"], options: { command: "bun ./📜️script.ts build", cwd: "." } } } }));
    writeFileSync(join(root, "source.json"), JSON.stringify({ value: row.initial }));
    writeFileSync(join(root, "📜️script.ts"), `#!/usr/bin/env bun
import {existsSync,writeFileSync,readFileSync,mkdirSync} from "node:fs";
if(process.argv[2]!=="build")throw Error("Expected build");
writeFileSync("ready","ready");
const deadline=Date.now()+30000;while(!existsSync("continue")){if(Date.now()>deadline)throw Error("Fixture deadline");await Bun.sleep(20);}
const count=existsSync("count")?Number(readFileSync("count","utf8")):0;writeFileSync("count",String(count+1));
mkdirSync("out",{recursive:true});writeFileSync("out/result.json",readFileSync("source.json"));console.log("[DEBUG] fixture compiler executed");
`);
    const launch = (suffix: string) => {
      const env = { ...process.env, NX_SKIP_NX_CACHE: "false", NX_DISABLE_NX_CACHE: "false", NX_SKIP_REMOTE_CACHE: "true", NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false", NX_WORKSPACE_DATA_DIRECTORY: join(root, "d"), NX_CACHE_DIRECTORY: join(root, "c"), NX_FORCE_REUSE_CACHED_GRAPH: "false" };
      env.SEMIO_SCRIPT_PROCESS_INVOCATION=JSON.stringify(createScriptProcessEnvelope({version:1,owner:"nx-source-custody-fixture",maximumElapsedMilliseconds:0},{},Date.now()));delete env.SEMIO_SCRIPT_CAPABILITIES;delete env.SEMIO_BUILD_GRAPH_PATH;delete env.SEMIO_BUILD_GRAPH_DIGEST;delete env.SEMIO_BUILD_REUSE_GRAPH;
      const child = spawn("node", [cli, "run", "fixture:build", "--output-style=stream", "--verbose"], { cwd: root, env, stdio: ["ignore", "pipe", "pipe"] });
      let log = "";
      for (const stream of [child.stdout, child.stderr]) stream.on("data", bytes => { log += String(bytes); if (log.length > 8 * 1024 * 1024) child.kill(); });
      const timer = setTimeout(() => child.kill(), 60000);
      const done = new Promise<void>((accept, reject) => { child.once("error", reject); child.once("close", code => { clearTimeout(timer); writeFileSync(join(output, `source-custody-${suffix}.log`), log); code === 0 ? accept() : reject(Error(log)); }); });
      void done.catch(() => {});
      return { child, done, output: () => log };
    };
    const first = launch("edited"), deadline = Date.now() + 45000;
    while (!existsSync(join(root, "ready"))) { if (Date.now() > deadline || first.child.exitCode !== null) { await first.done; throw Error("Fixture child did not start"); } await delay(20); }
    writeFileSync(join(root, "source.json"), JSON.stringify({ value: row.during })); writeFileSync(join(root, "continue"), "continue");
    await first.done;
    assert.match(first.output(), /cache promotion withheld/);
    await launch("fresh").done;
    await launch("warm").done;
    assert.equal(Number(readFileSync(join(root, "count"), "utf8")), row.expectedExecutions, "Edited execution must not become a stale cache hit; the unchanged third execution must hit");
    assert.equal(JSON.parse(readFileSync(join(root, "out/result.json"), "utf8")).value, row.during);
    console.log("[DEBUG] Installed Nx withheld mid-task source promotion, then executed fresh and restored warm cache");
  } finally { rmSync(root, { recursive: true, force: true }); }
}
