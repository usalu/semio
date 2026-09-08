#!/usr/bin/env bun
import assert from "node:assert/strict";
import { readFileSync, mkdirSync, writeFileSync, rmSync, existsSync, readdirSync, lstatSync, realpathSync, copyFileSync, chmodSync, symlinkSync, renameSync, mkdtempSync, utimesSync } from "node:fs";
import { join, resolve, relative, dirname, sep } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { spawn } from "node:child_process";
import { createInterface } from "node:readline";
import { createHash } from "node:crypto";
import { EventEmitter } from "node:events";
import { BundleScript, ScriptRouter, runBundleScriptMain, getWorkspaceRoot, runCmdStatus, wasmBuildEnvironment, wasmBindgenVersion, wasmBuildArguments, devToolingEnv } from "../📦️packages/🟦️typescript/🟦️.ts";
import plugin, { cacheInternals } from "../🟨️.mjs";
import { stageArtifacts } from "./📦️artifacts/🟦️.ts";

const SCRIPT_ROOT = dirname(fileURLToPath(import.meta.url));
const POLICY = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🔣️policy.json"), "utf8"));
const slash = (path: string): string => path.split(sep).join("/");
type Project = { name: string; root: string; targets: Record<string, any>; namedInputs?: Record<string, any>; [key: string]: any };
type Finding = { rule: string; path: string; line: number; entry_point: string; evidence: string; replacement: string; severity: string; status: string };

/** 🎫️ Keeps every diagnostic artifact within an explicitly selected repository ticket. */
function ticketOutput(root: string, args: string[]): string {
  const index = args.indexOf("--ticket");
  const ticket = args.find((arg) => arg.startsWith("--ticket="))?.slice(9) ?? (index >= 0 ? args[index + 1] : process.env.SEMIO_TICKET_DIR);
  if (!ticket) throw new Error("Select the active ticket with --ticket <directory> or SEMIO_TICKET_DIR");
  const directory = realpathSync(resolve(root, ticket));
  const tickets = realpathSync(join(root, ".🧬semio/🦑️repo/🎫️tickets"));
  if (!directory.startsWith(`${tickets}${sep}`) || !existsSync(join(directory, "🎫️ticket.json"))) throw new Error("Diagnostic output requires an existing repository ticket");
  const output = join(directory, "🗑️generated", "nx");
  mkdirSync(output, { recursive: true });
  return output;
}

/** 📂️ Enumerates source entries without traversing generated stores, symlinks or opaque trees. */
function sourceFiles(root: string): string[] {
  const files: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of readdirSync(join(root, directory), { withFileTypes: true })) {
      const path = slash(join(directory, entry.name));
      if (entry.isSymbolicLink() || path === "compose" || path === "temp/compose" || entry.name === ".🧬semio" || POLICY.generatedDirectories.includes(entry.name)) continue;
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile()) files.push(path);
    }
  };
  walk("");
  return files.sort();
}

/** 🧭️ Produces a reviewable inventory from the same project normalizer used by Nx. */
function inventory(root: string): { projects: Project[]; commands: any[]; artifacts: any[]; violations: Finding[] } {
  const files = sourceFiles(root);
  const defaults = JSON.parse(readFileSync(join(root, "nx.json"), "utf8")).targetDefaults ?? {};
  const projects = plugin.createNodesV2[1](files.filter((file) => file.endsWith("📋️project.json") || file.endsWith("Cargo.toml")), {}, { workspaceRoot: root }).flatMap(([, result]: any) => Object.values(result.projects)) as Project[];
  const commands: any[] = [];
  const artifacts: any[] = [];
  const violations: Finding[] = [];
  const report = (rule: string, path: string, entry_point: string, evidence: string, replacement: string): void => {
    const text = readFileSync(join(root, path), "utf8");
    const at = text.indexOf(JSON.stringify(entry_point.split(":").at(-1)));
    violations.push({ rule, path, line: at < 0 ? 1 : text.slice(0, at).split("\n").length, entry_point, evidence, replacement, severity: "error", status: "open" });
  };
  for (const project of projects) {
    for (const [name, declared] of Object.entries(project.targets)) {
      const target = { ...defaults[name], ...declared };
      const path = slash(join(project.root, existsSync(join(root, project.root, "📋️project.json")) ? "📋️project.json" : "Cargo.toml"));
      const identity = `${project.name}:${name}`;
      commands.push({ project: project.name, target: name, file: path, cwd: target.options?.cwd ?? root, command: target.options?.command, cache: target.cache === true, continuous: target.continuous === true, inputs: target.inputs, outputs: target.outputs ?? [], dependsOn: target.dependsOn ?? [] });
      if (typeof target.options?.command === "string" && (!/^bun(?: --watch)? (?:"[^"\n]*📜️script\.ts"|[^\s]*📜️script\.ts) [^\n]+$/.test(target.options.command) || /(?:&&|\|\|)/.test(target.options.command))) report("ORCH-01", path, identity, target.options.command, "Invoke one script and declare prerequisite ordering in dependsOn");
      if ((target.cache || /^(build(?:-|$)|wasm$|native-build$|package$|extension-package$)/.test(name)) && !Object.hasOwn(target, "outputs")) report("CACHE-06", path, identity, "Target still needs an explicit output contract", "Declare complete owned deliverables, or outputs: [] for a verified read-only task");
      for (const output of target.outputs ?? []) {
        const ownedPath = output.replaceAll("{workspaceRoot}", root).replaceAll("{projectRoot}", join(root, project.root));
        artifacts.push({ owner: identity, path: slash(relative(root, resolve(root, ownedPath))), category: "deliverable", cacheability: Boolean(target.cache), retention: "replace-on-next-success", producer: identity });
        if (/(?:^|\/)(?:node_modules|target|\.venv|\.nx)(?:\/|$)/.test(ownedPath) || resolve(root, ownedPath) === root) report("CACHE-04", path, identity, `Mutable or broad output: ${output}`, "Stage only this target's deliverables");
      }
    }
  }
  for (const file of files.filter((file) => file.endsWith("package.json"))) {
    let json;
    try { json = JSON.parse(readFileSync(join(root, file), "utf8")); } catch { continue; }
    if (Object.keys(json.scripts ?? {}).length && projects.some((project) => project.root === dirname(file) || project.root === "." && file === "package.json") && JSON.stringify(json.nx?.includedScripts) !== "[]") report("ORCH-05", file, "nx.includedScripts", "Nx re-infers forwarding scripts and replaces their implementation targets", "Set nx.includedScripts to [] for forwarding package scripts");
    for (const [name, command] of Object.entries(json.scripts ?? {})) {
      commands.push({ file, script: name, command, cwd: dirname(file) });
      if (file === "package.json" && name === "nx" && command === "bun ./📜️script.ts nx") continue;
      if (!/^(?:bun )?nx\b/.test(String(command))) report("ORCH-01", file, name, String(command), "Forward the public command to an independently implemented Nx target");
    }
  }
  return { projects, commands, artifacts, violations };
}

/** 📏️ Counts retained files without double-counting hard links or following symbolic links. */
function directoryBytes(path: string): { apparent: number; allocated: number; files: number } {
  const seen = new Set<string>();
  const result = { apparent: 0, allocated: 0, files: 0 };
  const walk = (file: string): void => {
    let stat;
    try { stat = lstatSync(file); } catch { return; }
    if (stat.isSymbolicLink()) return;
    const key = `${stat.dev}:${stat.ino}`;
    if (seen.has(key)) return;
    seen.add(key);
    if (stat.isDirectory()) for (const child of readdirSync(file)) walk(join(file, child));
    else if (stat.isFile()) { result.apparent += stat.size; result.allocated += (stat.blocks ?? Math.ceil(stat.size / 512)) * 512; result.files++; }
  };
  walk(path);
  return result;
}

class AuditScript extends BundleScript {
  run(args: string[]): void {
    const output = ticketOutput(this.repoRoot, args);
    const result = inventory(this.repoRoot);
    for (const [name, value] of Object.entries(result)) writeFileSync(join(output, `${name}.json`), JSON.stringify(value, null, 2) + "\n");
    writeFileSync(join(dirname(dirname(output)), "📓️nx-inventory.md"), `# Nx Inventory\n\n${result.projects.length} projects; ${result.commands.length} commands; ${result.artifacts.length} declared artifacts; ${result.violations.length} unresolved contract findings.\n\nGenerated machine-readable inventories are in 🗑️generated/nx while the ticket is active.\n`);
    console.log(`[nx-audit] projects=${result.projects.length} commands=${result.commands.length} artifacts=${result.artifacts.length} violations=${result.violations.length}`);
  }
}

class PolicyScript extends BundleScript {
  run(): void {
    const result = inventory(this.repoRoot);
    for (const finding of result.violations) console.error(`${finding.rule} ${finding.path}:${finding.line} ${finding.entry_point}: ${finding.evidence}`);
    if (result.violations.length) throw new Error(`${result.violations.length} Nx contract violations`);
    console.log("[nx-policy] all target and command contracts passed");
  }
}

class GraphScript extends BundleScript {
  run(): void {
    const { readCachedProjectGraph } = createRequire(import.meta.url)("@nx/devkit");
    const graph = readCachedProjectGraph();
    const edges = Object.values(graph.dependencies).flat() as any[];
    assert.ok(edges.length > 0, "A monorepo graph must contain dependency edges");
    for (const edge of edges) assert.ok(graph.nodes[edge.target] || graph.externalNodes?.[edge.target], `Unknown dependency ${edge.target}`);
    for (const project of inventory(this.repoRoot).projects) {
      const resolved = graph.nodes[project.name]?.data;
      assert.ok(resolved, `Nx omitted ${project.name}`);
      for (const [name, target] of Object.entries(project.targets)) {
        const actual = resolved.targets?.[name];
        assert.ok(actual, `Nx omitted ${project.name}:${name}`);
        for (const property of ["cache", "outputs", "dependsOn"]) if (Object.hasOwn(target, property)) assert.deepEqual(actual[property], target[property], `Nx precedence changed ${project.name}:${name}.${property}`);
      }
    }
    console.log(`[nx-graph] projects=${Object.keys(graph.nodes).length} edges=${edges.length}`);
  }
}

class DiskScript extends BundleScript {
  run(args: string[]): void {
    const output = ticketOutput(this.repoRoot, args);
    const stores = [
      { owner: "nx", path: ".nx/cache", category: "task-results", budget: 8 * 1024 ** 3, retention: "Nx maxCacheSize", cleanup: "Nx-managed eviction" },
      { owner: "cargo-workspace", path: "target", category: "compiler-state", budget: 20 * 1024 ** 3, retention: "native incrementality; explicit maintenance only", cleanup: "exclusive native maintenance" },
      { owner: "repo", path: ".🧬semio/🦑️repo/⚡️cache", category: "derived-state", budget: 2 * 1024 ** 3, retention: "owner-specific leases and retention", cleanup: "owner-marked inactive outputs only" },
    ].map((store) => ({ ...store, ...directoryBytes(join(this.repoRoot, store.path)) }));
    writeFileSync(join(output, "disk.json"), JSON.stringify(stores, null, 2) + "\n");
    for (const store of stores) console.log(`[nx-disk] ${store.owner}: ${(store.allocated / 1024 ** 3).toFixed(2)} GiB allocated; ${(store.apparent / 1024 ** 3).toFixed(2)} GiB apparent`);
  }
}

class DoctorScript extends BundleScript {
  run(): void {
    for (const command of ["bun", "node", "cargo", "rustc", "go", "uv", "dotnet", "cmake"]) {
      try {
        const result = Bun.spawnSync([command, command === "go" ? "version" : "--version"], { stdout: "pipe", stderr: "pipe" });
        console.log(`[nx-doctor] ${command}: ${result.exitCode === 0 ? result.stdout.toString().split("\n")[0] : "unavailable"}`);
      } catch { console.log(`[nx-doctor] ${command}: unavailable`); }
    }
  }
}

/** 🔧️ Fingerprints optional native tools without downloading or installing during hashing. */
class ToolchainScript extends BundleScript {
  run(args: string[]): void {
    if (args[0] !== "wasm") throw new Error("toolchain wasm");
    const versions = Object.fromEntries(["wasm-pack", "wasm-bindgen", "wasm-opt", "trunk"].map((tool) => {
      const override = tool === "wasm-bindgen" ? process.env.SEMIO_WASM_BINDGEN_BIN : tool === "wasm-opt" ? process.env.SEMIO_WASM_OPT_BIN : undefined;
      const path = override ? resolve(this.repoRoot, override) : Bun.which(tool, { PATH: process.env.PATH });
      if (!path) return [tool, "unavailable"];
      const result = Bun.spawnSync([path, "--version"], { stdout: "pipe", stderr: "pipe", timeout: 10000 });
      if (result.exitCode !== 0) throw new Error(`Cannot fingerprint ${tool}`);
      return [tool, result.stdout.toString().trim() || result.stderr.toString().trim()];
    }));
    console.log(JSON.stringify(versions));
  }
}

/** 🔐️ Hashes catalog membership and exact source bytes, including ignored generator inputs. */
class GeneratorInputsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args[0] !== "registry-catalog") throw new Error("generator-inputs registry-catalog");
    const { loadCatalogTaxonomy, registryCatalogInputPaths, registryCatalogInputView } = await import("../🔍️discovery/🟦️.ts");
    const taxonomy = loadCatalogTaxonomy(), view = registryCatalogInputView(this.repoRoot, taxonomy), hash = createHash("sha256");
    for (const path of registryCatalogInputPaths(this.repoRoot, taxonomy, view)) {
      const kind = view.kind(path);
      if (kind === "symlink") throw new Error(`Generator input is a symlink: ${path}`);
      const content = kind === "file" ? readFileSync(join(this.repoRoot, path)) : Buffer.alloc(0);
      hash.update(JSON.stringify([path, kind, content.byteLength]) + "\n").update(content);
    }
    console.log(hash.digest("hex"));
  }
}

/** 🔁️ Verifies actual Nx execution, reuse, restoration and invalidation in an isolated ticket fixture. */
class CacheVerifyScript extends BundleScript {
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
      const env = { ...process.env, NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false", NX_TUI: "false", NX_NATIVE_COMMAND_RUNNER: "false", NX_WORKSPACE_DATA_DIRECTORY: join(fixture, ".nx", "workspace-data"), CACHE_PROBE_MODE: mode };
      for (const key of Object.keys(env)) if (key.startsWith("NX_TASK_") || ["NX_SKIP_NX_CACHE", "NX_SKIP_REMOTE_CACHE"].includes(key)) delete env[key];
      const started = performance.now();
      delete env.NX_FORCE_REUSE_CACHED_GRAPH;
      const child = Bun.spawn(["node", createRequire(join(this.repoRoot, "package.json")).resolve("nx/bin/nx.js"), "run", `probe:${task}`, "--output-style=static"], { cwd: fixture, env, stdout: "pipe", stderr: "pipe" });
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
      const consumer = Bun.spawnSync(process.platform === "win32" ? ["node", join(fixture, "dist/consumer.cjs")] : [join(fixture, "dist/consumer.cjs")], { stdout: "pipe", env: { ...process.env, FORCE_COLOR: "0" } });
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

/** 🧹️ Delegates test retention to its owner; Nx manages task-result eviction independently. */
class DiskPruneScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const output = ticketOutput(this.repoRoot, args);
    const { collectGarbage, loadOracleRegistry, formatGcReport } = await import("../../🧪️test/📦️packages/🟦️typescript/🟦️.ts");
    const report = collectGarbage(this.repoRoot, loadOracleRegistry(this.repoRoot), { dry: !args.includes("--apply"), olderThanMs: 7 * 86400000 });
    writeFileSync(join(output, "prune.json"), JSON.stringify({ tests: report, nx: "Nx maxCacheSize eviction", preserved: ["Cargo incremental state", "Renderer and plugin artifacts", "Active and pinned test evidence"] }, null, 2) + "\n");
    console.log(formatGcReport(report));
  }
}

/** 🧪️ Executes language-neutral policy examples against the Nx project plugin. */
export async function testCacheContracts(): Promise<void> {
  const { validate } = createRequire(import.meta.url)("jsonschema");
  const policy = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🔣️policy.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧬️policy.schema.json"), "utf8"));
  assert.equal(validate(policy, schema).valid, true);
  assert.ok(cacheInternals, "cache policy must be exposed for contract verification");
  for (const name of ["setup", "publish", "update", "deploy", "format", "clean-test", "bench"]) {
    const target = cacheInternals.targetPolicy(name, { cache: true, options: { command: `bun ./📜️script.ts ${name}` } }, policy);
    assert.equal(target.cache, false, name);
  }
  for (const name of ["dev", "dev-storybook", "serve", "watch", "test-watch"]) {
    const target = cacheInternals.targetPolicy(name, { cache: true }, policy);
    assert.equal(target.cache, false, name);
    assert.equal(target.continuous, true, name);
  }
  const root = getWorkspaceRoot();
  assert.equal(Object.keys(JSON.parse(readFileSync(join(root, "nx.json"), "utf8")).targetDefaults ?? {}).length, 0, "Native Nx defaults override custom project metadata; apply defaults inside the repository plugin");
  assert.equal(wasmBuildEnvironment(root, {}).CARGO_TARGET_DIR, join(root, ".🧬semio/🦑️repo/⚡️cache/cargo/browser"));
  assert.equal(wasmBuildEnvironment(root, { CARGO_TARGET_DIR: "chosen-cache" }).CARGO_TARGET_DIR, join(root, "chosen-cache"));
  const vectors = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/nx-contract/🔣️.json"), "utf8"));
  assert.equal(validate(vectors, JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/nx-contract/🧬️.schema.json"), "utf8"))).valid, true);
  const loggingKeys = Object.keys(vectors.daemonEnvironment), savedLogging = Object.fromEntries(loggingKeys.map((key) => [key, process.env[key]]));
  const quiet = devToolingEnv(Object.fromEntries(loggingKeys.map((key) => [key, undefined])));
  try {
    for (const [key, value] of Object.entries(vectors.daemonEnvironment)) { assert.equal(quiet[key], value); process.env[key] = quiet[key]; }
    const oracle = createRequire(import.meta.url)("nx/src/daemon/client/daemon-environment.js").getDaemonSpawnEnv();
    for (const [key, value] of Object.entries(vectors.daemonEnvironment)) assert.equal(oracle[key], value, key);
    assert.equal(devToolingEnv({ NX_NATIVE_LOGGING: "nx=debug" }).NX_NATIVE_LOGGING, "nx=debug");
  } finally { for (const [key, value] of Object.entries(savedLogging)) if (value === undefined) delete process.env[key]; else process.env[key] = value; }
  for (const row of vectors.wasmProfiles) {
    const args = wasmBuildArguments(row.profile);
    assert.deepEqual(args, { pack: row.pack, cargo: row.cargo });
    for (const [tool, flags] of [["cargo", args.cargo], ["wasm-pack", args.pack]] as const) {
      const help = Bun.spawnSync([tool, "build", ...flags, "--help"], { stdout: "pipe", stderr: "pipe" });
      assert.equal(help.exitCode, 0, `${tool} rejected ${flags.join(" ")}: ${help.stderr.toString()}`);
    }
  }
  const socketFrames = createRequire(import.meta.url)("nx/src/utils/consume-messages-from-socket.js");
  assert.equal(typeof socketFrames.writeMessage, "function", "Nx must preserve Unicode across socket frame boundaries");
  for (const size of vectors.socketFrames.chunkSizes) {
    const frames: Buffer[] = [], received: unknown[] = [];
    for (const message of vectors.socketFrames.messages) socketFrames.writeMessage({ write: (chunk: Buffer) => frames.push(chunk) }, Buffer.from(JSON.stringify(message)));
    const consume = socketFrames.consumeMessagesFromSocket((message: Buffer) => received.push(socketFrames.parseMessage(message)));
    const bytes = Buffer.concat(frames);
    for (let offset = 0; offset < bytes.length; offset += size) consume(bytes.subarray(offset, offset + size));
    assert.deepEqual(received, vectors.socketFrames.messages, `Unicode socket frame size ${size}`);
  }
  for (const row of vectors.policies) {
    const target = cacheInternals.targetPolicy(row.target, { cache: true }, policy);
    assert.deepEqual({ cache: target.cache, continuous: target.continuous ?? false }, { cache: row.cache, continuous: row.continuous }, row.target);
  }
  const selectionApi = await import("../🎮️playground/🟦️.ts");
  assert.equal(typeof selectionApi.loadFrameworkOsPlaygroundSelections, "function", "Development selection must read authored metadata before generation");
  const selectionFixture = mkdtempSync(join(ticketOutput(root, []), "playground-selection-"));
  try {
    const vector = vectors.playgroundSelections, manifest = join(selectionFixture, vector.manifest);
    mkdirSync(dirname(manifest), { recursive: true });
    writeFileSync(manifest, vector.text);
    const originalTime = lstatSync(manifest).mtime;
    for (const port of vector.ports) {
      const text = vector.text.replace(String(vector.ports[0]), String(port));
      writeFileSync(manifest, text);
      utimesSync(manifest, originalTime, originalTime);
      const authored = createRequire(import.meta.url)("@iarna/toml").parse(text).package.metadata;
      const rows = selectionApi.loadFrameworkOsPlaygroundSelections(selectionFixture, [vector.manifest]);
      assert.deepEqual(rows, [{ ...authored.semio.playground[0], pluginId: authored.component.package.slice(6), cratePath: slash(dirname(vector.manifest)) }]);
      assert.equal(rows[0].variant, vector.variant);
      assert.deepEqual(rows[0].aliases, [vector.alias]);
      assert.equal(rows[0].ports.react, port);
    }
    const stale = join(selectionFixture, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json");
    mkdirSync(dirname(stale), { recursive: true });
    writeFileSync(stale, "invalid generated catalog");
    assert.equal(selectionApi.loadFrameworkOsPlaygroundSelections(selectionFixture, [vector.manifest])[0].variant, vector.variant);
    assert.throws(() => selectionApi.loadFrameworkOsPlaygroundSelections(selectionFixture, [vector.manifest, vector.manifest]), /duplicate/i);
    writeFileSync(manifest, vector.text.replace(String(vector.ports[0]), "65536"));
    assert.throws(() => selectionApi.loadFrameworkOsPlaygroundSelections(selectionFixture, [vector.manifest]), /port/i);
    assert.throws(() => selectionApi.loadFrameworkOsPlaygroundSelections(selectionFixture, ["../Cargo.toml"]), /outside/i);
  } finally { rmSync(selectionFixture, { recursive: true }); }
  await (await import("./🧪️tests/📜️script.ts")).testCommandInputs(root, ticketOutput(root, []));
  console.log("[DEBUG] Native command contracts passed; collecting project inventory");
  const result = inventory(root), contracts = result.projects;
  console.log(`[DEBUG] Project inventory collected: ${contracts.length} projects`);
  const toml = createRequire(import.meta.url)("@iarna/toml");
  let componentPackages = 0;
  const componentLaunchers: { project: string; pluginId: string }[] = [];
  for (const project of contracts) {
    const path = join(root, project.root, "Cargo.toml");
    if (!existsSync(path)) continue;
    const manifest = toml.parse(readFileSync(path, "utf8"));
    if (!manifest.package?.metadata?.component?.package || !["plugin", "extension"].includes(manifest.package?.metadata?.semio?.role)) continue;
    componentPackages++;
    componentLaunchers.push({ project: project.name, pluginId: manifest.package.metadata.component.package.slice("semio:".length) });
    const moduleCatalog = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json"), "utf8"));
    const moduleDirectory = moduleCatalog.modules.find((row: any) => row.pluginId === manifest.package.metadata.component.package.slice("semio:".length))?.directoryName;
    assert.ok(moduleDirectory);
    for (const row of vectors.materialization.profiles) {
      const target = project.targets[row.target], shared = contracts.find((project) => project.name === vectors.materialization.project)?.targets[row.support];
      assert.equal(target?.cache, true, `${project.name}:${row.target} needs a materialization producer`);
      assert.deepEqual(target.dependsOn, [row.component, `${vectors.materialization.project}:${row.support}`]);
      assert.deepEqual(target.outputs, [`{workspaceRoot}/${vectors.materialization.root}/dist/${row.profile}/🔌️plugin-modules/${moduleDirectory}`]);
      assert.ok(target.inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
      assert.ok(target.options.command.includes(`materialize ${row.profile} --manifest`));
      assert.equal(shared?.cache, true);
      assert.equal(shared.outputs.length, 2);
    }
    for (const row of vectors.componentProfiles) {
      const target = project.targets[row.target];
      assert.equal(target?.cache, true, `${project.name}:${row.target} needs a component producer`);
      assert.deepEqual(target.outputs, [row.output]);
      assert.equal(target.parallelism, false);
      assert.ok(target.options.command.includes(`native component ${row.profile} --manifest`));
    }
  }
  assert.ok(componentPackages > 0, "Component metadata must discover plugin producers");
  const fontProject = contracts.find((project) => project.name === vectors.fontAssets.project)!;
  for (const [name, output] of [[vectors.fontAssets.tool, vectors.fontAssets.toolOutput], [vectors.fontAssets.producer, vectors.fontAssets.output]]) {
    assert.equal(fontProject.targets[name]?.cache, true, `${name} must be separately cacheable`);
    assert.deepEqual(fontProject.targets[name].outputs, [output]);
  }
  assert.deepEqual(fontProject.targets[vectors.fontAssets.producer].dependsOn, [vectors.fontAssets.tool]);
  assert.ok(fontProject.targets[vectors.fontAssets.producer].inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
  const activationRoot = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation");
  const activation = await import(join(activationRoot, "🟦️.ts"));
  const activationCases = JSON.parse(readFileSync(join(activationRoot, "🧫️cases.json"), "utf8"));
  const ActivationAjv = createRequire(import.meta.url)("ajv");
  const validateActivation = new ActivationAjv().compile(JSON.parse(readFileSync(join(activationRoot, "🧬️.schema.json"), "utf8")));
  const { keyBy, sortBy } = createRequire(import.meta.url)("lodash");
  let activationReceipt: any;
  for (const cycle of activationCases.cycles) {
    const completed = cycle.plugins.map(([pluginId, digest]: string[]) => ({ pluginId, artifactSha256: digest.repeat(64) }));
    const prior = keyBy(activationReceipt?.plugins ?? [], "pluginId");
    const nextTimestamp = Math.max(cycle.now, 1, ...Object.values(prior).map((row: any) => row.rebuiltAt + 1));
    const oracle = sortBy(completed, "pluginId").map((row: any) => ({ ...row, rebuiltAt: prior[row.pluginId]?.artifactSha256 === row.artifactSha256 ? prior[row.pluginId].rebuiltAt : nextTimestamp }));
    const next = activation.nextActivationReceipt(activationCases.variant, activationCases.profile, completed, activationReceipt, cycle.now);
    assert.equal(validateActivation(next), true, JSON.stringify(validateActivation.errors));
    assert.deepEqual(next.plugins, oracle);
    assert.deepEqual(next.plugins.map((row: any) => [row.pluginId, row.artifactSha256[0], row.rebuiltAt]), cycle.expected);
    assert.deepEqual(activation.parseActivationReceipt(JSON.parse(JSON.stringify(next))), next);
    activationReceipt = next;
  }
  assert.throws(() => activation.parseActivationReceipt({ ...activationReceipt, plugins: [...activationReceipt.plugins, ...activationReceipt.plugins] }), /Duplicate/);
  assert.throws(() => activation.parseActivationReceipt({ ...activationReceipt, unknown: true }), /Invalid/);
  assert.throws(() => activation.nextActivationReceipt("other", "dev", [], activationReceipt, 500), /identity/);
  assert.throws(() => activation.nextActivationReceipt("../note", "dev", [], undefined, 500), /Invalid/);
  const sessionProject = contracts.find((project) => project.name === vectors.playgroundSessions.project)!;
  for (const variant of vectors.playgroundSessions.variants) {
    const session = sessionProject.targets[`session-${variant}`];
    assert.equal(session?.cache, true, `${variant} needs its own cacheable playground session`);
    assert.deepEqual(session.dependsOn, [vectors.playgroundSessions.prerequisite]);
    assert.deepEqual(session.outputs, [`{projectRoot}/dist/sessions/${variant}`]);
    assert.ok(session.inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
  }
  console.log("[DEBUG] Component and activation contracts passed; checking editor and playground contracts");
  const registryRoot = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
  const registry = await import(join(registryRoot, "📜️script.ts"));
  const launch = await import(join(registryRoot, "🖥️launch.ts"));
  const configurations = Bun.JSONC.parse(launch.generateLaunchJson(root, registry.generatePlaygroundRegistry(root), componentLaunchers)).configurations;
  for (const project of contracts) for (const [name, target] of Object.entries(project.targets) as [string, any][]) if (target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts") && ["build", "check", "test"].includes(name)) assert.ok(configurations.some((row: any) => row.command === `bun nx run ${project.name}:${name}`), `Missing native editor command ${project.name}:${name}`);
  const preparationProject = contracts.find((project) => project.name === vectors.playgroundPreparation.project)!;
  for (const playground of registry.generatePlaygroundRegistry(root)) {
    for (const engine of playground.engines) assert.ok(contracts.some((project) => resolve(root, project.root) === resolve(root, engine) && project.targets.wasm), `Engine ${engine} must name an authored producer`);
    for (const profile of vectors.playgroundPreparation.profiles) {
      const targetName = `prepare-${playground.variant}-react-${profile}`, preparation = preparationProject.targets[targetName];
      assert.ok(preparation, `${targetName} needs declared prerequisites`);
      assert.equal(preparation.cache, false);
      assert.deepEqual(preparation.outputs, []);
      const activationName = `activate-${playground.variant}-react-${profile}`, activationTarget = preparationProject.targets[activationName];
      assert.ok(activationTarget, `${activationName} must follow completed preparation`);
      assert.equal(activationTarget.cache, false);
      assert.deepEqual(activationTarget.outputs, []);
      assert.deepEqual(activationTarget.dependsOn, [targetName]);
      assert.equal(configurations.filter((configuration: any) => configuration.command === `bun nx run ${preparationProject.name}:${activationName}`).length, 1);
      for (const command of ["serve", "dev"]) {
        const serverName = `${command}-${playground.variant}-react-${profile}`, server = preparationProject.targets[serverName];
        assert.ok(server, `${serverName} needs an Nx server owner`);
        assert.equal(server.continuous, true);
        assert.equal(server.cache, false);
        assert.deepEqual(server.outputs, []);
        assert.deepEqual(server.dependsOn, [activationName]);
        assert.equal(server.options.command, `bun ./📜️script.ts serve ${playground.variant} react ${profile}`);
        assert.equal(configurations.filter((configuration: any) => configuration.command === `bun nx run ${preparationProject.name}:${serverName}`).length, 1);
      }
      const components = registry.buildPlaygroundSession(playground.variant).plugins.map((row: any) => `${componentLaunchers.find((entry) => entry.pluginId === row.pluginId)!.project}:materialize-${profile}`);
      assert.deepEqual([...preparation.dependsOn].sort(), [...new Set([`@semio-tech/plugin-registry:session-${playground.variant}`, `@semio-tech/framework-plugin-web:support-${profile}`, vectors.playgroundPreparation.fonts, ...vectors.playgroundPreparation.engines, ...components])].sort());
      assert.equal(configurations.filter((configuration: any) => configuration.command === `bun nx run ${preparationProject.name}:${targetName}`).length, 1);
    }
    const target = `session-${playground.variant}`;
    assert.ok(sessionProject.targets[target]);
    assert.equal(configurations.filter((configuration: any) => configuration.command === `bun nx run ${sessionProject.name}:${target}`).length, 1);
  }
  for (const entry of componentLaunchers) for (const row of [...vectors.componentProfiles, ...vectors.materialization.profiles]) assert.equal(configurations.filter((configuration: any) => configuration.command === `bun nx run ${entry.project}:${row.target}`).length, 1, `${entry.project}:${row.target} must have one editor command`);
  assert.deepEqual(result.violations.filter((finding) => finding.rule === "ORCH-01"), [], "All public commands must use one script entrypoint");
  for (const entry of vectors.entrypoints) assert.ok(contracts.find((project) => project.name === entry.project)?.targets[entry.target].dependsOn?.includes(entry.prerequisite), `${entry.project}:${entry.target} needs ${entry.prerequisite} in Nx`);
  for (const entry of vectors.nativeBinaries) {
    const project = contracts.find((project) => project.name === entry.project)!;
    assert.equal(project.targets[entry.target].cache, true, `${entry.project} binary must be restorable`);
    assert.deepEqual(project.targets[entry.target].outputs, [entry.output]);
    assert.ok(project.targets[entry.consumer].dependsOn.includes(entry.target));
  }
  for (const entry of vectors.components) {
    const project = contracts.find((project) => project.name === entry.project)!;
    const target = project.targets[entry.target];
    assert.ok(project.namedInputs?.default?.includes(entry.schemaInput), `${entry.project} must hash its shared component schema`);
    assert.equal(target.cache, true, `${entry.project}:${entry.target} component must be restorable`);
    assert.deepEqual(target.outputs, [entry.output]);
    assert.ok(contracts.find((project) => project.name === entry.consumerProject)!.targets[entry.consumerTarget].dependsOn.includes(`${entry.project}:${entry.target}`));
  }
  console.log("[DEBUG] Editor and playground contracts passed; checking lifecycle and compiler contracts");
  const mcpRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
  const mcp = await import(join(root, mcpRoot, "🟦️.ts"));
  const binaryVectors = JSON.parse(readFileSync(join(root, mcpRoot, "🧫️fixtures/🧱️binary-gate.json"), "utf8"));
  const pathOracle = createRequire(import.meta.url)("node:path");
  for (const row of binaryVectors.pathCases) {
    assert.equal(mcp.resolveMcpBinaryPath(row.repoRoot, row.environment, row.platform), row.expected);
    const paths = row.platform === "win32" ? pathOracle.win32 : pathOracle.posix;
    assert.equal(paths.resolve(row.repoRoot, row.environment.SEMIO_OS_MCP_BIN ?? binaryVectors.artifactRoot, ...(row.environment.SEMIO_OS_MCP_BIN ? [] : [binaryVectors.cargoBinary + (row.platform === "win32" ? ".exe" : "")])), row.expected);
  }
  const generators = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).generatorContracts;
  for (const id of vectors.generators) {
    const authority = generators[id], split = authority.target.lastIndexOf(":"), project = contracts.find((project) => project.name === authority.target.slice(0, split))!;
    const target = project.targets[authority.target.slice(split + 1)];
    assert.equal(target.cache, true, `${id} must cache its verified deliverables`);
    assert.deepEqual(target.outputs, authority.outputRoots.map((output: any) => `{workspaceRoot}/${output.path}`), id);
    for (const path of authority.inputPatterns) assert.ok(target.inputs.includes(`{workspaceRoot}/${path}`), `${id} missing ${path}`);
    if (authority.inputDiscovery) assert.ok(target.inputs.some((input: any) => input.runtime?.includes("generator-inputs")), `${id} must hash discovered membership and ignored source bytes`);
    if (authority.checkTarget) assert.equal(project.targets[authority.checkTarget.slice(authority.checkTarget.lastIndexOf(":") + 1)].cache, false, `${id} freshness checks must inspect current bytes`);
  }
  const [validatedProject, validatedTarget] = vectors.validationPrerequisite.target.split(":");
  assert.ok(contracts.find((project) => project.name === validatedProject)?.targets[validatedTarget].dependsOn.includes(vectors.validationPrerequisite.prerequisite));
  assert.ok(!contracts.find((project) => project.name === validatedProject)?.targets[validatedTarget].inputs.includes("^default"), "Exact catalog discovery must not reintroduce generated outputs through broad dependency defaults");
  const workspace = contracts.find((project) => project.name === "workspace")!;
  assert.deepEqual(workspace.targets.setup.dependsOn, vectors.lifecycle.setupDependencies);
  assert.deepEqual(workspace.targets.prepare.dependsOn, vectors.lifecycle.prepareDependencies);
  for (const target of vectors.lifecycle.setupDependencies) assert.equal(workspace.targets[target]?.cache, false);
  const hostProject = contracts.find((project) => project.name === vectors.platformEnvironment.project)!;
  for (const target of Object.values(hostProject.targets) as any[]) for (const key of vectors.platformEnvironment.keys) assert.equal(target.options?.env?.[key], undefined, `Shared target metadata cannot force ${key}`);
  const bootstrap = vectors.bootstrap, dotnet = contracts.find((project) => project.name === bootstrap.dotnetProject);
  assert.ok(dotnet, "The current .NET support library needs an Nx owner");
  assert.deepEqual(dotnet.targets.build.outputs, ["{projectRoot}/dist/build"]);
  assert.ok(dotnet.targets.build.dependsOn.includes("deps"));
  assert.equal(dotnet.targets.deps.cache, false);
  assert.ok(workspace.targets["deps-dotnet"].dependsOn.includes(`${bootstrap.dotnetProject}:deps`));
  assert.ok(readFileSync(join(root, "Monorepo.sln"), "utf8").includes(bootstrap.dotnetPath.replaceAll("/", "\\")));
  assert.ok(readFileSync(join(root, bootstrap.dotnetPath, "🧪️Semio.Repo.Test.csproj"), "utf8").includes(`Include="${bootstrap.compile}"`));
  const python = createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(join(root, "pyproject.toml"), "utf8"));
  assert.deepEqual(python.tool.uv.workspace.members, bootstrap.pythonMembers);
  assert.deepEqual(workspace.targets["deps-python"].dependsOn, bootstrap.pythonDependencies);
  const styling = contracts.find((project) => project.name === bootstrap.stylingProject)!;
  assert.ok(styling.targets.generate.dependsOn.includes(bootstrap.stylingGenerator));
  assert.deepEqual(styling.targets.generate.outputs, []);
  for (const target of Object.values(styling.targets) as any[]) if (target.options.command.includes(" test")) assert.ok(target.dependsOn.includes(bootstrap.stylingGenerator));
  assert.ok(!/compose|topologic|vcpkg/i.test(readFileSync(join(root, "CMakeLists.txt"), "utf8") + readFileSync(join(root, "CMakePresets.json"), "utf8")));
  const ts = createRequire(import.meta.url)("typescript");
  for (const row of vectors.engineOutputs) {
    const project = contracts.find((project) => project.name === row.project)!;
    assert.deepEqual(project.targets.wasm.outputs, row.outputs);
    const engineSource = ts.createSourceFile("engine.ts", readFileSync(join(root, project.root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const outputs: string[] = [];
    const inspect = (node: any): void => {
      if (ts.isCallExpression(node) && node.expression.getText(engineSource) === "runWasmPackWebBuild") {
        const output = node.arguments[0].properties.find((property: any) => property.name?.getText(engineSource) === "outputDirectory");
        outputs.push(output?.initializer.text ?? "pkg");
      }
      ts.forEachChild(node, inspect);
    };
    inspect(engineSource);
    assert.deepEqual(outputs, [row.directory]);
  }
  const source = ts.createSourceFile("📜️script.ts", readFileSync(join(root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const coordinator = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "NxScript");
  const coordinatorCode = ts.transpileModule(coordinator.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  for (const vector of JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/🛑️cancellation.json"), "utf8")).cases) {
    const killed: number[] = [], child = Object.assign(new EventEmitter(), { pid: 1234 });
    const runtime = Object.assign(new EventEmitter(), { platform: vector.platform, exitCode: 0, kill: (pid: number, signal: number | string) => { if (!signal) throw new Error("No process"); killed.push(pid); } });
    const Coordinator = new Function("Script", "process", "createRequire", "join", "resolveNxInvocation", "devToolingEnv", "orchestratorBudgetOpts", "spawnNxProcess", "stopNxProcessTree", coordinatorCode + "; return NxScript;")(
      class { root = root; }, runtime, () => ({ resolve: (name: string) => name }), join, (args: string[]) => ({ args, env: {} }), (env: unknown) => env, () => ({}), () => child,
      (command: string) => { if (command === "taskkill") { killed.push(child.pid); return { status: 0 }; } if (vector.throws) throw new Error("Snapshot unavailable"); return { status: 0, stdout: vector.stdout }; });
    const done = new Coordinator().run(["run", "fixture:build"]);
    assert.doesNotThrow(() => runtime.emit("SIGTERM"), vector.name);
    child.emit("close", null);
    await done;
    assert.equal(runtime.exitCode, vector.expectedExit, vector.name);
    assert.ok(killed.length > 0, `${vector.name}: owned launch process must still be stopped`);
    assert.equal(runtime.listenerCount("SIGTERM"), 0);
  }
  console.log("[DEBUG] Nx cancellation stops owned launch processes after malformed or unavailable process snapshots PASS");
  const invocation = source.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "resolveNxInvocation");
  const route = ts.transpileModule(invocation.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const resolveInvocation = new Function("process", `${route}; return resolveNxInvocation;`)({ env: {} });
  const resolvePrint = new Function("process", "readFileSync", "join", "WORKSPACE_ROOT", `${route}; return resolveNxInvocation;`)({ env: {} }, readFileSync, join, root);
  for (const path of ["🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🧫️invocations.json", "♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧫️invocations.json", "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️invocations.json"]) {
    const invocations = JSON.parse(readFileSync(join(root, path), "utf8"));
    for (const vector of invocations.valid) {
      const result = resolvePrint(vector.input);
      assert.deepEqual(result.args, vector.args);
      assert.equal(result.watch, vector.watch);
      if (vector.env) assert.deepEqual(result.env, vector.env);
    }
    for (const vector of invocations.invalid) assert.throws(() => resolvePrint(vector));
  }
  for (const command of ["prepare", "activate", "serve", "dev"]) {
    const selected = `@semio-tech/framework-os-dev:${command}-note-react-dev`;
    const invocation = resolveInvocation(["run", selected]);
    assert.equal(invocation.watch, command === "dev" ? "@semio-tech/framework-os-dev:activate-note-react-dev" : undefined);
    assert.equal(invocation.env.SEMIO_BUILD_MODE, "dev");
    assert.equal(resolveInvocation(["run", selected, "--graph=stdout"]).watch, undefined);
  }
  assert.deepEqual(resolveInvocation(["run", "workspace:dev", "--", "mcp", "stdio", "os", "--folder", "chosen"]).args, ["run", "@semio-tech/framework-os-mcp-rs:dev", "--", "stdio", "--folder", "chosen"]);
  assert.deepEqual(resolveInvocation(["run", "workspace:dev", "--", "mcp", "http", "os"]).args, ["run", "@semio-tech/framework-os-mcp-rs:dev", "--", "http", "--port", "6300"]);
  for (const renderer of vectors.benchmarkRenderers) {
    const target = `bench-plugins-${renderer}`;
    const args = resolveInvocation(["run", "workspace:bench", "--", "plugins", `--renderer=${renderer}`, "--count", "3"]).args;
    assert.deepEqual(args, ["run", `@semio-tech/framework-os-dev:${target}`, "--", "--count", "3"]);
    assert.equal(hostProject.targets[target].dependsOn.includes("@semio-tech/framework-os-scale-fixture:build-wasm"), renderer === "native");
    assert.equal(hostProject.targets[target].dependsOn.includes("@semio-tech/framework-renderer-wgpu:native-build"), renderer === "native");
  }
  assert.deepEqual(resolveInvocation(["run", "@semio-tech/framework-renderer-wgpu:native", "--", "s", "--release"]).args, ["run", "@semio-tech/framework-renderer-wgpu:native-release", "--", "s"]);
  const hostSource = ts.createSourceFile("host.ts", readFileSync(join(root, hostProject.root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const apple = hostSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "ensureAppleDeveloperDir");
  const selectApple = ts.transpileModule(apple.getText(hostSource), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  for (const platform of vectors.platformEnvironment.platforms) {
    const host = { platform, env: {} as Record<string, string> };
    new Function("process", "existsSync", `${selectApple}; ensureAppleDeveloperDir();`)(host, () => true);
    assert.deepEqual(Object.keys(host.env).sort(), platform === "darwin" ? [...vectors.platformEnvironment.keys].sort() : []);
  }
  const setup = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "SetupScript").getText(source);
  assert.ok(!/runWorkspaceCodegen|buildRepoMcpClient|runNx|\["nx"|\["build"/.test(setup), "Setup must only provision dependency environments");
  const lock = readFileSync(join(root, "Cargo.lock"), "utf8");
  const lockOracle = createRequire(import.meta.url)("@iarna/toml").parse(lock);
  assert.equal(wasmBindgenVersion(lock), lockOracle.package.find((pkg: any) => pkg.name === "wasm-bindgen").version);
  for (const row of vectors.wasm) {
    const project = contracts.find((project) => project.name === row.project);
    assert.ok(project?.targets.wasm.outputs?.includes(row.output), `${row.project}:wasm must own ${row.output}`);
    assert.notEqual(project?.targets.wasm.cache, false, `${row.project}:wasm must be cacheable`);
    assert.ok(project?.namedInputs?.default.some((input: any) => input.env === "RUSTFLAGS"), `${row.project} must preserve toolchain inputs`);
    if (row.output.startsWith("{projectRoot}/") && !row.output.includes("../")) assert.ok(project?.namedInputs?.default.includes(`!${row.output}/**/*`), `${row.project} must exclude its output in the project fileset`);
  }
  const ticket = process.env.SEMIO_TICKET_DIR;
  assert.ok(ticket, "SEMIO_TICKET_DIR must point to the active ticket for generated test files");
  console.log("[DEBUG] Lifecycle and compiler contracts passed; checking source discovery and cancellation");
  const fixture = join(resolve(root, ticket), "🗑️generated", "policy-contract");
  const rustFixture = join(fixture, "rust-inputs");
  for (const [path, contents] of Object.entries(vectors.rust.files)) { mkdirSync(dirname(join(rustFixture, path)), { recursive: true }); writeFileSync(join(rustFixture, path), contents as string); }
  const rustInputs = cacheInternals.rustSourceFiles([join(rustFixture, vectors.rust.entry)], rustFixture).map((path: string) => slash(relative(rustFixture, path))).sort();
  assert.deepEqual(rustInputs, vectors.rust.inputs);
  const rustOracle = Bun.spawnSync(["rustc", "--crate-type=lib", "--emit=dep-info", "-o", join(rustFixture, "oracle.d"), vectors.rust.entry], { cwd: rustFixture, stdout: "pipe", stderr: "pipe" });
  assert.equal(rustOracle.exitCode, 0, rustOracle.stderr.toString());
  const rustDependencies = readFileSync(join(rustFixture, "oracle.d"), "utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/).map((path) => slash(relative(rustFixture, resolve(rustFixture, path)))).sort();
  assert.deepEqual(rustInputs, [...new Set(rustDependencies)]);
  const discovery = vectors.rustDiscoveryCache, sourceCache = cacheInternals.createRustSourceCache(discovery.limitBytes);
  const revisionRoot = join(fixture, "rust-revisions");
  for (const [path, contents] of Object.entries(discovery.files)) { mkdirSync(dirname(join(revisionRoot, path)), { recursive: true }); writeFileSync(join(revisionRoot, path), contents as string); }
  mkdirSync(dirname(join(revisionRoot, discovery.entry)), { recursive: true });
  for (const revision of discovery.revisions) {
    const entry = join(revisionRoot, discovery.entry);
    writeFileSync(entry, revision.source);
    utimesSync(entry, 1_700_000_000, 1_700_000_000);
    const inputs = cacheInternals.rustSourceFiles([entry], revisionRoot, sourceCache).map((path: string) => slash(relative(revisionRoot, path))).sort();
    assert.deepEqual(inputs, revision.inputs);
    const oracle = Bun.spawnSync(["rustc", "--crate-type=lib", "--emit=dep-info", "-o", "oracle.d", discovery.entry], { cwd: revisionRoot, stdout: "pipe", stderr: "pipe" });
    assert.equal(oracle.exitCode, 0, oracle.stderr.toString());
    const dependencies = readFileSync(join(revisionRoot, "oracle.d"), "utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/).map((path) => slash(relative(revisionRoot, resolve(revisionRoot, path)))).sort();
    assert.deepEqual(inputs, [...new Set(dependencies)]);
    const hits = sourceCache.hits;
    assert.deepEqual(cacheInternals.rustSourceFiles([entry], revisionRoot, sourceCache).map((path: string) => slash(relative(revisionRoot, path))).sort(), inputs);
    assert.ok(sourceCache.hits > hits);
    assert.ok(sourceCache.bytes <= discovery.limitBytes);
  }
  for (let index = 0; index < 100; index++) {
    const entry = join(revisionRoot, discovery.entry);
    writeFileSync(entry, `pub const VALUE_${index}: usize = ${index};`);
    cacheInternals.rustSourceFiles([entry], revisionRoot, sourceCache);
    assert.ok(sourceCache.bytes <= discovery.limitBytes);
  }
  assert.ok(sourceCache.entries.size < 100);
  const materializerSource = ts.createSourceFile("materializer.ts", readFileSync(join(root, vectors.materialization.root, "🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const spawnDeclaration = materializerSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "spawnAsync");
  const spawnCode = ts.transpileModule(spawnDeclaration.getText(materializerSource), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const materializerSpawn = new Function("spawn", `${spawnCode}; return spawnAsync;`)(spawn);
  const materializerFixture = join(fixture, "materializer-cancel");
  mkdirSync(materializerFixture, { recursive: true });
  const childScript = join(materializerFixture, "📜️script.ts"), pidPath = join(materializerFixture, "pid");
  writeFileSync(childScript, 'import { writeFileSync } from "node:fs"; if (process.argv[2] === "fail") { process.stderr.write("x".repeat(200000)); process.exit(1); } process.on("SIGTERM", () => {}); setInterval(() => {}, 1000); writeFileSync(process.argv[2], String(process.pid));');
  rmSync(pidPath, { force: true });
  const controller = new AbortController();
  const execution = materializerSpawn("bun", [childScript, pidPath], materializerFixture, controller.signal).then(() => undefined, (error: Error) => error);
  let childPid = 0;
  try {
    const startupDeadline = Date.now() + 10000;
    while (!existsSync(pidPath)) { assert.ok(Date.now() < startupDeadline, "Materializer fixture must become ready"); await new Promise((accept) => setTimeout(accept, 25)); }
    childPid = Number(readFileSync(pidPath, "utf8"));
    const started = Date.now();
    controller.abort(new Error("materializer cancelled"));
    assert.match(String(await execution), /materializer cancelled/);
    assert.ok(Date.now() - started < vectors.materialization.cancellation.shutdownMilliseconds);
    assert.throws(() => process.kill(childPid, 0), "Cancelled code generation must terminate the subprocess");
  } finally {
    controller.abort();
    if (childPid && process.platform !== "win32") try { process.kill(-childPid, "SIGKILL"); } catch {}
  }
  const diagnostic = await materializerSpawn("bun", [childScript, "fail"], materializerFixture).then(() => "", (error: Error) => error.message);
  assert.match(diagnostic, /exited with status 1/);
  assert.ok(diagnostic.length < vectors.materialization.cancellation.maximumDiagnosticCharacters);
  const kernelInputs = contracts.find((project) => project.name === "@semio-tech/framework-os-kernel")?.namedInputs?.default;
  assert.ok(kernelInputs?.includes("{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"));
  assert.ok(!kernelInputs?.includes("{workspaceRoot}/🧰️framework/🛍️products/💻️os/**/*.{rs,toml,json,semio,wit,wgsl,glsl,h,c,cpp}"));
  mkdirSync(join(fixture, "leaf"), { recursive: true });
  mkdirSync(join(fixture, "other"), { recursive: true });
  writeFileSync(join(fixture, "leaf", "📋️project.json"), JSON.stringify({ name: "leaf", targets: { test: { options: { command: "bun ./📜️script.ts test" } } } }));
  writeFileSync(join(fixture, "leaf", "📜️script.ts"), "export {};\n");
  writeFileSync(join(fixture, "other", "📋️project.json"), JSON.stringify({ name: "leaf" }));
  const goManifest = vectors.go.manifest;
  writeFileSync(join(fixture, "go.mod"), goManifest);
  const goOracle = Bun.spawnSync(["go", "mod", "edit", "-json", `-modfile=${join(fixture, "go.mod")}`], { cwd: fixture, env: { ...process.env, GOWORK: "off" }, stdout: "pipe", stderr: "pipe" });
  assert.equal(goOracle.exitCode, 0, goOracle.stderr.toString());
  const go = JSON.parse(goOracle.stdout.toString());
  const { manifest: _manifest, ...expectedGo } = vectors.go;
  assert.deepEqual(cacheInternals.goDependencies(goManifest), expectedGo);
  assert.deepEqual(cacheInternals.goDependencies(goManifest), { module: go.Module.Path, requires: go.Require.map((row: any) => row.Path), replacements: go.Replace.map((row: any) => ({ module: row.Old.Path, path: row.New.Path })) });
  assert.throws(() => plugin.createNodesV2[1](["leaf/📋️project.json", "other/📋️project.json"], {}, { workspaceRoot: fixture }), /duplicate.*leaf/i);
  const projects = plugin.createNodesV2[1](["leaf/📋️project.json"], {}, { workspaceRoot: fixture });
  assert.equal(projects[0][1].projects.leaf.targets["test-exhaustive"].cache, false);
  assert.equal(projects[0][1].projects.leaf.targets.test.executor, "nx:run-commands");
  assert.deepEqual(cacheInternals.nativeDependencies({ dependencies: { core: { path: "../core" }, serde: "1" } }, {}), [{ name: "core", path: "../core", workspace: false, kind: "dependencies" }]);
  const artifact = join(fixture, "consumer");
  writeFileSync(artifact, "native fixture\n");
  chmodSync(artifact, 0o755);
  const staged = join(fixture, "dist/build");
  stageArtifacts(staged, "leaf/Cargo.toml", new Map([["consumer", artifact], ["obsolete", artifact]]));
  stageArtifacts(staged, "leaf/Cargo.toml", new Map([["consumer", artifact]]));
  assert.deepEqual(readFileSync(join(staged, "consumer")), readFileSync(artifact));
  assert.equal(existsSync(join(staged, "obsolete")), false);
  if (process.platform !== "win32") assert.equal(lstatSync(join(staged, "consumer")).mode & 0o111, 0o111);
  assert.throws(() => stageArtifacts(staged, "another/Cargo.toml", new Map()), /Unowned/);
  assert.throws(() => stageArtifacts(staged, "leaf/Cargo.toml", new Map([["../escape", artifact]])), /Invalid artifact/);
  assert.ok(existsSync(join(staged, "consumer")));
  const testApi = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts"));
  const taxonomy = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
  mkdirSync(dirname(join(fixture, taxonomy)), { recursive: true });
  copyFileSync(join(root, taxonomy), join(fixture, taxonomy));
  const results = join(testApi.testCacheRoot(fixture), "tasks", "probe", "test", "results", "run");
  mkdirSync(results, { recursive: true });
  const retainedHash = "sha256:" + "a".repeat(64);
  writeFileSync(join(results, "🗝️run.json"), JSON.stringify({ artifacts: [{ sha256: retainedHash }] }));
  assert.ok(testApi.markReferencedBlobs(fixture, { contributions: [] } as any).has(retainedHash), "Task-scoped restored reports must retain their referenced blobs");
  const past = new Date(Date.now() - 30 * 86400000).toISOString();
  testApi.writeLease(results, { runId: "complete", agentId: "cache-test", pid: process.pid, state: "complete", createdAt: past, heartbeatAt: past, retention: "ephemeral-success" });
  const active = join(dirname(results), "active");
  testApi.writeLease(active, { runId: "active", agentId: "cache-test", pid: process.pid, state: "active", createdAt: past, heartbeatAt: past, retention: "ephemeral-success" });
  const pendingBlob = testApi.installFixtureBlob(fixture, new TextEncoder().encode("active publication"));
  const gc = testApi.collectGarbage(fixture, { contributions: [] } as any, { dry: false, olderThanMs: 7 * 86400000 });
  assert.ok(gc.removed.some((path: string) => path.endsWith("/results/run")), "Completed scoped runs must be collectible");
  assert.ok(existsSync(active), "Active scoped runs must survive collection");
  assert.ok(existsSync(testApi.fixtureBlobPath(fixture, pendingBlob)), "An active run may not yet have published its blob references");
  writeFileSync(join(active, "🗝️run.json"), "incomplete");
  assert.throws(() => testApi.markReferencedBlobs(fixture, { contributions: [] } as any), /Cannot determine/);
  const { resolveNxInvocation } = await import(join(root, "📜️script.ts"));
  assert.deepEqual(resolveNxInvocation(["run", "workspace:build", "--graph=stdout", "--", "assets"]).args, ["run", "@semio-tech/assets:build", "--graph=stdout"]);
  assert.deepEqual(resolveNxInvocation(["run", "workspace:dev", "--", "storybook", "ui"]).args, ["run", "workspace:dev-storybook", "--", "ui"]);
  assert.deepEqual(resolveNxInvocation(["show", "projects"]).args, ["show", "projects"]);
  assert.deepEqual(resolveNxInvocation(["run", "workspace:cpp", "--", "build", "macos-release"]).args, ["run", "workspace:cpp-build", "--", "macos-release"]);
  const cpp = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "CppScript");
  const configure = cpp.members.find((node: any) => node.name?.getText(source) === "runConfigure").getText(source);
  assert.ok(!configure.includes("runSetup"), "CMake configuration must not reinstall its prerequisites");
  assert.ok(!cpp.getText(source).includes("ensureVcpkg"), "No current project consumes a vcpkg installation");
  assert.equal(resolveNxInvocation(["run", "workspace:test", "--", "quick"]).args[1], "workspace:test-quick");
  assert.equal(resolveNxInvocation(["run", "workspace:test", "--", "fundamental"]).args[1], "workspace:test-fundamental");
  assert.equal(resolveNxInvocation(["run", "workspace:setup", "--", "deps", "wasm"]).args[1], "workspace:deps-wasm");
  const rootTest = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "TestScript").getText(source);
  assert.ok(!rootTest.includes("run-many"), "The root test target cannot schedule a second Nx graph");
  for (const level of ["fundamental", "quick", "long", "exhaustive"]) assert.ok(workspace.targets[`test-${level}`].dependsOn.length > 0);
  assert.equal(resolveNxInvocation(["run", "workspace:dev", "--", "s"]).args[1], "@semio-tech/framework-os-dev:dev");
  assert.ok(invocation.getText(source).includes("loadFrameworkOsPlaygroundSelections()"));
  assert.ok(!invocation.getText(source).includes("loadFrameworkOsPlaygroundCatalog()"));
  assert.equal(root.length > 0, true);
  rmSync(fixture, { recursive: true });
  console.log("[cache-contract] schema, graph ownership, source-byte discovery, native dependency oracles and materializer cancellation passed");
}

class TestScript extends BundleScript {
  async run(): Promise<void> { await testCacheContracts(); }
}

const router = new ScriptRouter(SCRIPT_ROOT).register("test", TestScript).register("audit", AuditScript).register("policy-check", PolicyScript).register("artifact-check", PolicyScript).register("graph-check", GraphScript).register("doctor", DoctorScript).register("disk-report", DiskScript).register("disk-prune", DiskPruneScript).register("cache-verify", CacheVerifyScript).register("toolchain", ToolchainScript).register("generator-inputs", GeneratorInputsScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
