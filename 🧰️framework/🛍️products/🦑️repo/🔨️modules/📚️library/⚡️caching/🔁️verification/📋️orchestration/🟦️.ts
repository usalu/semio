import assert from "node:assert/strict";
import { existsSync, lstatSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { BundleScript } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { ticketOutput } from "../../🎫️output/🟦️.ts";

/** 🔁️ Verifies actual Nx execution, reuse, restoration and invalidation in an isolated ticket fixture. */
export class CacheVerifyScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const output = ticketOutput(this.repoRoot, args);
    const lease = join(output, "cache-verification.lease");
    writeFileSync(lease, JSON.stringify({ pid: process.pid }), { flag: "wx" });
    try { await this.verify(output); }
    finally {
      const marker = join(output, "cache-verification/.nx-verification.json");
      if (existsSync(marker) && JSON.parse(readFileSync(marker, "utf8")).pid === process.pid) writeFileSync(marker, JSON.stringify({ owner: "repo:cache-verify", active: false }) + "\n");
      rmSync(lease);
    }
  }

  private async verify(output: string): Promise<void> {
    const fixture = join(output, "cache-verification");
    const marker = join(fixture, ".nx-verification.json");
    if (existsSync(fixture)) {
      if (lstatSync(fixture).isSymbolicLink() || !existsSync(marker)) throw new Error(`Unowned verification fixture: ${fixture}`);
      const prior = JSON.parse(readFileSync(marker, "utf8"));
      if (prior.owner !== "repo:cache-verify" || prior.active) throw new Error(`Verification fixture is owned by an active or unknown run: ${fixture}`);
      rmSync(fixture, { recursive: true });
    }
    mkdirSync(fixture, { recursive: true });
    writeFileSync(marker, JSON.stringify({ owner: "repo:cache-verify", active: true, pid: process.pid }) + "\n");
    const put = (path: string, data: string | object): void => writeFileSync(join(fixture, path), typeof data === "string" ? data : JSON.stringify(data, null, 2));
    put("nx.json", { useDaemonProcess: false, cacheDirectory: ".nx/cache", maxCacheSize: "64MB", namedInputs: { production: ["{projectRoot}/source.json", "{projectRoot}/toolchain.json", "{projectRoot}/📜️script.ts", { env: "CACHE_PROBE_MODE" }], test: ["production", "{projectRoot}/spec.json"] } });
    put("package.json", { name: "cache-verification", private: true, nx: { includedScripts: [] } });
    put(".gitignore", "node_modules\n.nx\ndist\nstate\n*.log\n");
    put("source.json", { value: 42 });
    put("toolchain.json", { version: 1 });
    put("spec.json", { case: 1 });
    put("unrelated.json", { ignored: 1 });
    put("validation.json", { valid: true });
    put("project.json", { name: "probe", targets: {
      build: { executor: "nx:run-commands", cache: true, inputs: ["production"], outputs: ["{projectRoot}/dist"], options: { command: "bun ./📜️script.ts build" } },
      test: { executor: "nx:run-commands", cache: true, inputs: ["test"], outputs: [], options: { command: "bun ./📜️script.ts test" } },
      validate: { executor: "nx:run-commands", cache: false, outputs: [], options: { command: "bun ./📜️script.ts validate" } },
      guarded: { executor: "nx:run-commands", cache: true, dependsOn: ["validate"], inputs: ["production"], outputs: [], options: { command: "bun ./📜️script.ts guarded" } },
    } });
    put("📜️script.ts", `import { mkdirSync, existsSync, readFileSync, writeFileSync, chmodSync } from "node:fs";
const command = process.argv[2];
mkdirSync("state", {recursive:true});
const counter = "state/" + command;
const count = existsSync(counter) ? Number(readFileSync(counter,"utf8")) + 1 : 1;
writeFileSync(counter,String(count));
const source = JSON.parse(readFileSync("source.json","utf8"));
if (source.fail) throw new Error("intentional cache rejection");
if (command === "validate" && !JSON.parse(readFileSync("validation.json", "utf8")).valid) throw new Error("invalid dynamic input");
if (command === "build") {
  mkdirSync("dist", {recursive:true});
  writeFileSync("dist/consumer.cjs", "#!/usr/bin/env node\\nconsole.log(" + source.value + ");\\n");
  chmodSync("dist/consumer.cjs",0o755);
}
console.log("[cache-probe] " + command + " executed " + count);
`);
    symlinkSync(join(this.repoRoot, "node_modules"), join(fixture, "node_modules"), process.platform === "win32" ? "junction" : "dir");
    const observations: { scenario: string; buildExecutions: number; testExecutions: number; exitCode: number; durationMs: number }[] = [];
    const count = (task: string): number => existsSync(join(fixture, "state", task)) ? Number(readFileSync(join(fixture, "state", task), "utf8")) : 0;
    const run = async (scenario: string, task = "build", mode = "one", success = true): Promise<void> => {
      const env: Record<string, string | undefined> = { ...process.env, NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false", NX_TUI: "false", NX_NATIVE_COMMAND_RUNNER: "false", NX_WORKSPACE_DATA_DIRECTORY: join(fixture, ".nx", "workspace-data"), CACHE_PROBE_MODE: mode };
      for (const key of Object.keys(env)) if (key.startsWith("NX_TASK_") || ["NX_SKIP_NX_CACHE", "NX_SKIP_REMOTE_CACHE"].includes(key)) delete env[key];
      const started = performance.now();
      delete env.NX_FORCE_REUSE_CACHED_GRAPH;
      const child = (globalThis as any).Bun.spawn(["node", createRequire(join(this.repoRoot, "package.json")).resolve("nx/bin/nx.js"), "run", `probe:${task}`, "--output-style=static"], { cwd: fixture, env, stdout: "pipe", stderr: "pipe" });
      const cancel = (): void => { child.kill(); };
      process.once("SIGINT", cancel);
      process.once("SIGTERM", cancel);
      let status;
      try {
        const [stdout, stderr, exit] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
        writeFileSync(join(output, `${scenario}.log`), stdout + stderr);
        status = exit;
      } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
      observations.push({ scenario, buildExecutions: count("build"), testExecutions: count("test"), exitCode: status, durationMs: Math.round(performance.now() - started) });
      assert.equal(status === 0, success, `${scenario}: inspect ${join(output, `${scenario}.log`)}`);
      console.log(`[cache-verify] ${scenario}: builds=${count("build")} tests=${count("test")}`);
    };
    try {
      await run("cold"); assert.equal(count("build"), 1);
      await run("warm"); assert.equal(count("build"), 1);
      const expected = readFileSync(join(fixture, "dist/consumer.cjs"));
      rmSync(join(fixture, "dist"), { recursive: true });
      await run("restore"); assert.equal(count("build"), 1);
      assert.deepEqual(readFileSync(join(fixture, "dist/consumer.cjs")), expected);
      if (process.platform !== "win32") assert.equal(lstatSync(join(fixture, "dist/consumer.cjs")).mode & 0o111, 0o111);
      const consumer = (globalThis as any).Bun.spawnSync(process.platform === "win32" ? ["node", join(fixture, "dist/consumer.cjs")] : [join(fixture, "dist/consumer.cjs")], { stdout: "pipe", env: { ...process.env, FORCE_COLOR: "0" } });
      assert.equal(consumer.exitCode, 0); assert.equal(consumer.stdout.toString().trim(), "42");
      await run("test-cold", "test"); assert.equal(count("test"), 1);
      put("spec.json", { case: 2 });
      await run("test-only-build"); assert.equal(count("build"), 1);
      await run("test-only-test", "test"); assert.equal(count("test"), 2);
      put("unrelated.json", { ignored: 2 });
      await run("unrelated"); assert.equal(count("build"), 1);
      put("source.json", { value: 43 });
      await run("source-change"); assert.equal(count("build"), 2);
      await run("environment-change", "build", "two"); assert.equal(count("build"), 3);
      put("toolchain.json", { version: 2 });
      await run("toolchain-change", "build", "two"); assert.equal(count("build"), 4);
      await run("guarded-cold", "guarded", "two"); assert.equal(count("guarded"), 1); assert.equal(count("validate"), 1);
      await run("guarded-warm", "guarded", "two"); assert.equal(count("guarded"), 1); assert.equal(count("validate"), 2);
      put("validation.json", { valid: false });
      await run("guarded-rejected", "guarded", "two", false); assert.equal(count("guarded"), 1); assert.equal(count("validate"), 3);
      put("source.json", { fail: true });
      await run("failure-first", "build", "two", false);
      await run("failure-retry", "build", "two", false); assert.equal(count("build"), 6);
      writeFileSync(join(dirname(dirname(output)), "📓️cache-verification.md"), `# Nx Runtime Cache Verification\n\nPassed on ${process.platform}/${process.arch} with Nx ${createRequire(import.meta.url)("nx/package.json").version}. This isolated fixture verifies the Nx contract; individual product restoration remains separately tracked.\n\n| Scenario | Builds Executed | Tests Executed | Exit | Milliseconds |\n| --- | ---: | ---: | ---: | ---: |\n${observations.map((row) => `| ${row.scenario} | ${row.buildExecutions} | ${row.testExecutions} | ${row.exitCode} | ${row.durationMs} |`).join("\n")}\n\nRestored artifact bytes and executable bits matched; its consumer printed 42.\n`);
    } finally {
      writeFileSync(join(output, "cache-verification.json"), JSON.stringify(observations, null, 2) + "\n");
      writeFileSync(marker, JSON.stringify({ owner: "repo:cache-verify", active: false }) + "\n");
    }
  }
}

