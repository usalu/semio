import assert from "node:assert/strict";
import { appendFileSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

/** 🖥️ Compares native Nx continuous ownership, default sharing and cancellation in independent invocations. */
export async function testContinuousServices(workspace: string, generated: string, candidate?: string, selection?: string): Promise<void> {
  for (const mode of selection ? [selection] : ["isolated", "cancelled", "default-sharing", "vite-cancelled"]) await testContinuousServiceScenario(workspace, generated, mode, candidate);
}

/** 🧪️ Runs one schema-defined service lifetime against the installed native Nx task runner. */
async function testContinuousServiceScenario(workspace: string, generated: string, mode: string, candidate?: string): Promise<void> {
  const contractRoot = join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/continuous-services");
  const contract = JSON.parse(readFileSync(join(contractRoot, "🔣️.json"), "utf8"));
  assert.ok(new (createRequire(import.meta.url)("ajv").default)().validate(JSON.parse(readFileSync(join(contractRoot, "🛂️schema/🔣️.json"), "utf8")), contract));
  assert.ok(contract.timeouts.gate >= 2 * contract.timeouts.wait && contract.timeouts.service >= contract.timeouts.gate + contract.timeouts.wait, "Service fixture owners must outlive their parent orchestration deadlines");
  const scenario = contract.cases.find(row => row.name === mode);
  assert.ok(scenario, "Unknown continuous service scenario");
  const root = mkdtempSync(join(generated, "continuous-isolation-")), nxRoot = join(root, "node_modules/nx");
  mkdirSync(dirname(nxRoot), { recursive: true });
  cpSync(dirname(createRequire(join(workspace, "package.json")).resolve("nx/package.json")), nxRoot, { recursive: true });
  if (candidate) writeFileSync(join(nxRoot, "dist/src/tasks-runner/task-orchestrator.js"), readFileSync(candidate));
  const put = (name: string, value: unknown) => { const path = join(root, name); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, typeof value === "string" ? value : JSON.stringify(value)); };
  put("nx.json", { useDaemonProcess: false, neverConnectToCloud: true });
  put("package.json", { name: "continuous-isolation-fixture", private: true });
  put(".gitignore", "node_modules\n.nx\nstate\n");
  put("project.json", { name: "fixture", targets: {
    serve: { executor: "nx:run-commands", continuous: true, cache: false, metadata: scenario.sharing ? {} : contract.metadata, options: { command: "bun ./📜️script.ts serve" } },
    test: { executor: "nx:run-commands", cache: false, dependsOn: ["serve"], options: { command: "bun ./📜️script.ts test" } },
  } });
  put("state/.keep", "");
  put("📜️script.ts", String.raw`import { appendFileSync, existsSync, writeFileSync, readFileSync, renameSync } from "node:fs";
  const id = process.env.FIXTURE_RUN_ID, shared = process.env.FIXTURE_SHARED === "1", path = name => "state/" + name + "-" + (shared && name === "ready" ? "first" : id);
  const event = phase => appendFileSync("state/events.jsonl", JSON.stringify({ phase, id, pid: process.pid }) + "\n");
  const until = async test => { const deadline = Date.now() + ${contract.timeouts.gate}; while (!test()) { if (Date.now() > deadline) throw new Error("Fixture gate timeout: " + id); await Bun.sleep(20); } };
  if (process.argv[2] === "serve") {
    const server = Bun.serve({ port: 0, hostname: "127.0.0.1", fetch: () => new Response(id) });
    const ready = path("ready"); writeFileSync(ready + ".stage", String(server.port)); renameSync(ready + ".stage", ready); event("serve");
    const stop = () => { server.stop(true); event("closed"); process.exit(0); };
    process.once("SIGTERM", stop); process.once("SIGINT", stop); setTimeout(stop, ${contract.timeouts.service});
  } else {
    await until(() => existsSync(path("ready")));
    const url = "http://127.0.0.1:" + readFileSync(path("ready"), "utf8");
    if (await (await fetch(url)).text() !== (shared ? "first" : id)) throw new Error("Wrong server identity");
    event("consumer"); await until(() => existsSync(path("finish")));
    if (await (await fetch(url)).text() !== (shared ? "first" : id)) throw new Error("Server disappeared before its consumer");
    event("done");
  }
  `);
  if (scenario.vite) {
    const implementation = join(dirname(fileURLToPath(import.meta.url)), "../../🌐️vite");
    put("app/index.html", "<!doctype html><title>Nx Vite fixture</title>");
    put("app/⚙️vite.config.ts", 'export default { logLevel: "silent" };');
    const project = JSON.parse(readFileSync(join(root, "project.json"), "utf8"));
    project.targets.prepare = { executor: "nx:run-commands", cache: false, options: { command: "bun ./📜️script.ts prepare" } };
    project.targets.serve.dependsOn = ["prepare"]; put("project.json", project);
    put("📜️script.ts", String.raw`import { appendFileSync, existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { serveVite } from ${JSON.stringify(join(implementation, "🟦️.ts"))};
import { openServiceSession, readServiceSession, publishServiceReady, waitForServiceReady, closeServiceSession } from ${JSON.stringify(join(implementation, "🧾️session/🟦️.ts"))};
const id = process.env.FIXTURE_RUN_ID, pid = Number(process.env.NX_INVOCATION_ROOT_PID), owner = "fixture:e2e", sessions = resolve("state/sessions");
const event = phase => appendFileSync("state/events.jsonl", JSON.stringify({ phase, id, pid: process.pid }) + "\n");
if (process.argv[2] === "prepare") await openServiceSession(sessions, owner, pid);
else {
  const session = readServiceSession(sessions, owner, pid), controller = new AbortController();
  const cancel = () => controller.abort(); process.once("SIGTERM", cancel); process.once("SIGINT", cancel);
  const timer = setTimeout(cancel, ${contract.timeouts.service});
  try {
    if (process.argv[2] === "serve") {
      await serveVite({ root: resolve("app"), config: resolve("app/⚙️vite.config.ts"), host: "127.0.0.1", port: 0, signal: controller.signal, session, ready: async url => {
        await publishServiceReady(sessions, session, url, controller.signal); writeFileSync("state/ready-" + id, new URL(url).port); writeFileSync("state/session-" + id, JSON.stringify(session)); event("serve");
      } });
    } else {
      await waitForServiceReady(sessions, session, controller.signal, ${contract.timeouts.wait}); event("consumer");
      const deadline = Date.now() + ${contract.timeouts.gate};
      while (!existsSync("state/finish-" + id)) { controller.signal.throwIfAborted(); if (Date.now() > deadline) throw new Error("Consumer gate timeout"); await Bun.sleep(20); }
      await waitForServiceReady(sessions, session, controller.signal, ${contract.timeouts.health}); event("done");
    }
  } finally {
    clearTimeout(timer); process.removeListener("SIGTERM", cancel); process.removeListener("SIGINT", cancel);
    if (process.argv[2] === "serve") { await closeServiceSession(sessions, session); event("closed"); }
  }
}
`);
  }
  const env = { ...process.env, NX_WORKSPACE_ROOT: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/workspace-data"), NX_DAEMON: "false", NX_TUI: "false", NX_NATIVE_COMMAND_RUNNER: "false", NX_NO_CLOUD: "true", NODE_OPTIONS: "" };
  for (const key of Object.keys(env)) if (key.startsWith("NX_TASK_") || ["NX_INVOCATION_ROOT_PID", "NX_SOCKET_DIR", "NX_DAEMON_SOCKET_DIR", "npm_lifecycle_event", "npm_lifecycle_script"].includes(key)) delete env[key];
  const children: ReturnType<typeof Bun.spawn>[] = [], runs: Promise<number>[] = [], outcomes = new Map<string, number>();
  const events = () => existsSync(join(root, "state/events.jsonl")) ? readFileSync(join(root, "state/events.jsonl"), "utf8").trim().split("\n").filter(Boolean).map(line => JSON.parse(line)) : [];
  const run = (id: string) => {
    const child = Bun.spawn(["node", join(nxRoot, "dist/bin/nx.js"), "run", "fixture:test", "--output-style=stream"], { cwd: root, env: { ...env, FIXTURE_RUN_ID: id, FIXTURE_SHARED: scenario.sharing ? "1" : "0" }, stdout: "pipe", stderr: "pipe" });
    children.push(child);
    const drain = async stream => { for await (const bytes of stream) appendFileSync(join(root, "state/" + id + ".log"), bytes); };
    const completion = Promise.all([drain(child.stdout), drain(child.stderr), child.exited]).then(results => { outcomes.set(id, results[2]); return results[2]; });
    runs.push(completion); return completion;
  };
  const until = async (test: () => boolean, active: string[] = []) => {
    const deadline = Date.now() + contract.timeouts.wait;
    while (!test()) {
      for (const id of active) if (outcomes.has(id)) throw new Error(`Continuous fixture ${id} exited ${outcomes.get(id)} before its expected event; see ${join(root, "state/" + id + ".log")}`);
      assert.ok(Date.now() < deadline, "Continuous fixture did not reach its expected event within the deadline");
      await Bun.sleep(50);
    }
  };
  let completed = false;
  console.log("[DEBUG] Native continuous task ownership fixture: " + root);
  try {
    const first = run("first"); await until(() => events().some(row => row.phase === "consumer" && row.id === "first"), ["first"]);
    const second = run("second"); await until(() => events().some(row => row.phase === "consumer" && row.id === "second"), ["first", "second"]);
    assert.equal(events().filter(row => row.phase === "serve").length, scenario.services);
    if (scenario.sharing) {
      put("state/finish-second", "done"); assert.equal(await second, 0);
      const port = readFileSync(join(root, "state/ready-first"), "utf8");
      assert.equal(await (await fetch("http://127.0.0.1:" + port)).text(), "first");
      put("state/finish-first", "done"); assert.equal(await first, 0);
    } else {
      if (scenario.cancelFirst) { children[0].kill("SIGTERM"); assert.equal(await first, contract.cancellationExitCode); }
      else { put("state/finish-first", "done"); assert.equal(await first, 0); }
      const port = readFileSync(join(root, "state/ready-second"), "utf8");
      if (scenario.vite) assert.deepEqual(await (await fetch("http://127.0.0.1:" + port + "/__semio/nx-service")).json(), JSON.parse(readFileSync(join(root, "state/session-second"), "utf8")));
      else assert.equal(await (await fetch("http://127.0.0.1:" + port)).text(), "second");
      put("state/finish-second", "done"); assert.equal(await second, 0);
    }
    await until(() => events().filter(row => row.phase === "closed").length === scenario.services);
    const { createServer } = await import("node:net");
    for (const id of scenario.sharing ? ["first"] : contract.runs) {
      const port = Number(readFileSync(join(root, "state/ready-" + id), "utf8")), server = createServer();
      await new Promise<void>((accept, reject) => { server.once("error", reject); server.listen(port, "127.0.0.1", accept); });
      await new Promise<void>((accept, reject) => server.close(error => error ? reject(error) : accept()));
    }
    completed = true;
    console.log(`[DEBUG] Native Nx ${scenario.name}: ${scenario.services} service owners, independent completion/cancellation and released ports PASS`);
  } finally {
    put("state/finish-first", "done"); put("state/finish-second", "done");
    for (const child of children) if (child.exitCode === null) child.kill("SIGTERM");
    await Promise.all(runs);
    if (completed) rmSync(root, { recursive: true, force: true });
  }
}
