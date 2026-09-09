#!/usr/bin/env bun
import { createCachePolicyTests } from "./🧪️tests/⚡️cache-contracts/🟦️.ts";
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
import { createArtifactRegistry, measureArtifactRegistry, artifactBudgets, type ArtifactRegistry } from "./📦️artifacts/📇️registry/🟦️.ts";
import { readInventoryGraph, type InventoryProject } from "./📇️inventory/🟦️.ts";

const SCRIPT_ROOT = dirname(fileURLToPath(import.meta.url));
const POLICY = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🔣️policy.json"), "utf8"));
const slash = (path: string): string => path.split(sep).join("/");
type Project = InventoryProject;
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

/** 🧭️ Audits the complete resolved Nx graph and its source entry points. */
export function inventory(root: string): { projects: Project[]; commands: any[]; artifacts: any[]; artifactRegistry: ArtifactRegistry; violations: Finding[] } {
  const files = sourceFiles(root);
  const { projects, sources } = readInventoryGraph(root);
  const commands: any[] = [];
  const artifacts: any[] = [];
  const violations: Finding[] = [];
  const report = (rule: string, path: string, entry_point: string, evidence: string, replacement: string): void => {
    const text = readFileSync(join(root, path), "utf8");
    const at = text.indexOf(JSON.stringify(entry_point.split(":").at(-1)));
    violations.push({ rule, path, line: at < 0 ? 1 : text.slice(0, at).split("\n").length, entry_point, evidence, replacement, severity: "error", status: "open" });
  };
  for (const project of projects) {
    for (const [name, target] of Object.entries(project.targets)) {
      const path = sources[project.name][name];
      const identity = `${project.name}:${name}`;
      commands.push({ project: project.name, target: name, file: path, executor: target.executor, configurations: target.configurations, cwd: target.options?.cwd ?? root, command: target.options?.command, cache: target.cache === true, continuous: target.continuous === true, inputs: target.inputs, outputs: target.outputs ?? [], dependsOn: target.dependsOn ?? [] });
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
      if (file === "package.json" && name === "nx" && command === `bun ./${slash(relative(root, join(SCRIPT_ROOT, "🚀️bootstrap/📜️script.ts")))} nx`) continue;
      if (!/^(?:bun )?nx\b/.test(String(command))) report("ORCH-01", file, name, String(command), "Forward the public command to an independently implemented Nx target");
    }
  }
  const consumers = new Map<string, Set<string>>();
  for (const command of commands.filter(command => command.target)) for (const dependency of command.dependsOn) {
    const target = typeof dependency === "string" ? dependency : dependency.target;
    const selectors = typeof dependency === "object" ? dependency.projects : undefined;
    const selected = selectors ? (Array.isArray(selectors) ? selectors : [selectors]) : [command.project];
    if (typeof target !== "string" || target.startsWith("^") || /[*?{}]/.test(target)) continue;
    for (const project of selected) {
      if (!projects.some(candidate => candidate.name === project)) continue;
      const producer = target.includes(":") ? target : `${project}:${target}`;
      if (!consumers.has(producer)) consumers.set(producer, new Set());
      consumers.get(producer)!.add(`${command.project}:${command.target}`);
    }
  }
  const artifactRegistry = createArtifactRegistry(artifacts.map(artifact => ({ ...artifact, consumers: [...(consumers.get(artifact.owner) ?? [])] })));
  for (const finding of artifactRegistry.findings) {
    const command = commands.find(command => `${command.project}:${command.target}` === finding.owner);
    if (command) report(finding.rule, command.file, finding.owner, finding.evidence, "Declare one exclusive deliverable owner and use explicit target prerequisites for consumers");
  }
  return { projects, commands, artifacts: artifactRegistry.entries, artifactRegistry, violations };
}

class AuditScript extends BundleScript {
  run(args: string[]): void {
    const output = ticketOutput(this.repoRoot, args);
    const result = inventory(this.repoRoot);
    for (const [name, value] of Object.entries(result)) writeFileSync(join(output, `${name}.json`), JSON.stringify(value, null, 2) + "\n");
    writeFileSync(join(dirname(dirname(output)), "📓️nx-inventory.md"), `# Nx Inventory\n\n${result.projects.length} projects from every native Nx provider; ${result.commands.length} commands; ${result.artifacts.length} declared artifacts; ${result.violations.length} unresolved contract findings.\n\nResolved target settings and configuration provenance come from the graph constructed by the outer Nx invocation. Generated machine-readable inventories are in 🗑️generated/nx while the ticket is active.\n`);
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
    const files = sourceFiles(this.repoRoot);
    const declared = plugin.createNodesV2[1](files.filter(file => file.endsWith("📋️project.json") || file.endsWith("Cargo.toml")), {}, { workspaceRoot: this.repoRoot }).flatMap(([, result]: any) => Object.values(result.projects)) as Project[];
    for (const project of declared) {
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
  async run(args: string[]): Promise<void> {
    const output = ticketOutput(this.repoRoot, args);
    const controller = new AbortController(), cancel = (): void => controller.abort(new Error("Storage accounting cancelled"));
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      const registry = inventory(this.repoRoot).artifactRegistry;
      const result = await measureArtifactRegistry(this.repoRoot, registry, { signal: controller.signal, onProgress: progress => console.log(`[nx-disk] measured ${progress.files} files; ${progress.path}`) });
      const budgets = Object.entries(artifactBudgets).map(([group, budget]) => {
        const allocated = result.entries.filter(entry => entry.retention.budgetGroup === group).reduce<number | null>((total, entry) => total === null || entry.allocated === null ? null : total + entry.allocated, 0);
        return { group, budget, allocated, overBudget: allocated === null ? null : allocated > budget };
      });
      writeFileSync(join(output, "disk.json"), JSON.stringify({ ...result, budgets, findings: registry.findings }, null, 2) + "\n");
      const allocation = (value: number | null): string => value === null ? "allocation unavailable" : `${(value / 1024 ** 3).toFixed(2)} GiB allocated`;
      for (const entry of result.entries.filter(entry => entry.present && entry.files > 0)) console.log(`[nx-disk] ${entry.owner}: ${allocation(entry.allocated)}; ${entry.path}`);
      console.log(`[nx-disk] ${result.complete ? "Complete" : "Incomplete"}: ${result.totals.files} unique files; ${allocation(result.totals.allocated)}`);
      if (!result.complete) throw new Error(`Storage accounting is incomplete: ${result.errors.length} read errors; ${registry.findings.length} ownership violations`);
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
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

class TestScript extends BundleScript {
  async run(): Promise<void> { await testCacheContracts(); }
}

type ArtifactPackageRecord = {
  root: string;
  source: string;
  rust: { cargoName: string; nxName: string; manifest: string };
  typescript?: { name: string; nxName: string; manifest: string; entry: { types: string; import: string } };
};

/** 🧭️ Discovers artifact owners from taxonomy roots, independently of package naming. */
function artifactPackageInventory(root: string): { schemaVersion: 1; packages: ArtifactPackageRecord[] } {
  const owners: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of readdirSync(join(root, directory), { withFileTypes: true })) {
      if (!entry.isDirectory() || entry.isSymbolicLink() || entry.name === "node_modules" || entry.name === ".git" || entry.name === ".🧬semio" || POLICY.generatedDirectories.includes(entry.name)) continue;
      const path = slash(join(directory, entry.name));
      if (entry.name !== "🗿️artifacts") { walk(path); continue; }
      for (const artifact of readdirSync(join(root, path), { withFileTypes: true })) {
        const owner = slash(join(path, artifact.name));
        if (artifact.isDirectory() && existsSync(join(root, owner, "🦀️.rs"))) owners.push(owner);
      }
    }
  };
  for (const base of ["✏️s", "🧰️framework"]) if (existsSync(join(root, base))) walk(base);
  const projectFiles = sourceFiles(root).filter((path) => path.endsWith("📋️project.json") || path.endsWith("Cargo.toml"));
  const discovered = plugin.createNodesV2[1](projectFiles, {}, { workspaceRoot: root }).flatMap(([, result]: any) => Object.values(result.projects)) as Project[];
  const projects = new Map(discovered.map((project) => [slash(project.root), project]));
  const packages = owners.map((owner): ArtifactPackageRecord => {
    const source = `${owner}/🦀️.rs`;
    const rustRoot = `${owner}/📦️packages/🦀️rust`;
    const rustManifest = `${rustRoot}/Cargo.toml`;
    assert.ok(existsSync(join(root, rustManifest)), `Artifact source has no Rust package declaration: ${source}`);
    const cargo = createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(join(root, rustManifest), "utf8"));
    assert.equal(cargo.lib?.path, "../../🦀️.rs", `${rustManifest} must compile its taxonomy source directly`);
    const rustProject = projects.get(rustRoot);
    assert.ok(rustProject, `Nx omitted ${rustRoot}`);
    assert.ok(rustProject.tags?.includes("role:artifact"), `${rustProject.name} must declare role:artifact`);
    const nativeSources = rustProject.namedInputs?.nativeSources ?? [];
    const ownsSource = nativeSources.some((input: unknown) => typeof input === "string" && input.startsWith("{workspaceRoot}/") && !input.startsWith("!") && new Bun.Glob(input.slice("{workspaceRoot}/".length)).match(source));
    assert.ok(ownsSource, `${rustProject.name} does not hash its taxonomy source`);
    assert.deepEqual(rustProject.targets?.build?.outputs, ["{projectRoot}/dist/build"], `${rustProject.name} has an invalid build output contract`);
    const record: ArtifactPackageRecord = { root: owner, source, rust: { cargoName: cargo.package.name, nxName: rustProject.name, manifest: rustManifest } };
    const typescriptSource = `${owner}/🟦️.ts`;
    if (existsSync(join(root, typescriptSource))) {
      const typescriptRoot = `${owner}/📦️packages/🟦️typescript`;
      const manifest = `${typescriptRoot}/package.json`;
      assert.ok(existsSync(join(root, manifest)), `TypeScript artifact source has no package declaration: ${typescriptSource}`);
      const declaration = JSON.parse(readFileSync(join(root, manifest), "utf8"));
      const project = projects.get(typescriptRoot);
      assert.ok(project, `Nx omitted ${typescriptRoot}`);
      assert.equal(project.name, declaration.name, `${manifest} and Nx names differ`);
      assert.ok(project.namedInputs?.default?.some((input: unknown) => typeof input === "string" && input.startsWith(`{workspaceRoot}/${owner}/`)), `${project.name} does not hash its taxonomy owner`);
      assert.deepEqual(project.targets?.build?.outputs, ["{projectRoot}/dist"], `${project.name} has an invalid build output contract`);
      record.typescript = { name: declaration.name, nxName: project.name, manifest, entry: declaration.exports?.["."] };
    }
    return record;
  }).sort((left, right) => left.root.localeCompare(right.root));
  assert.ok(packages.length > 0, "Artifact taxonomy has no package roots");
  assert.equal(new Set(packages.map((entry) => entry.root)).size, packages.length, "Artifact roots must be unique");
  return { schemaVersion: 1, packages };
}

/** 🏃️ Captures a bounded subprocess while retaining progress and cancellation. */
async function captureArtifactContract(command: string, args: string[], cwd: string, timeoutMs: number): Promise<string> {
  const child = spawn(command, args, { cwd, env: process.env, detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"], windowsHide: true });
  let stdout = "", stderr = "", stopped = "", forceKill: ReturnType<typeof setTimeout> | undefined;
  const terminate = (reason: string): void => {
    stopped ||= reason;
    if (!child.pid) return;
    if (process.platform === "win32") child.kill("SIGTERM");
    else {
      try { process.kill(-child.pid, "SIGTERM"); } catch {}
      forceKill ??= setTimeout(() => { try { process.kill(-child.pid!, "SIGKILL"); } catch {} }, 2000);
      forceKill.unref();
    }
  };
  child.stdout.on("data", (chunk) => { stdout += chunk; if (stdout.length > 64 * 1024 * 1024) terminate("output limit"); });
  child.stderr.on("data", (chunk) => { stderr += chunk; if (stderr.length > 64 * 1024 * 1024) terminate("output limit"); });
  const interrupt = (): void => terminate("SIGINT");
  const stop = (): void => terminate("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", stop);
  const started = Date.now();
  const progress = setInterval(() => console.log(`[artifact-package-contract] ${command} running elapsedMs=${Date.now() - started}`), 10_000);
  const timeout = setTimeout(() => terminate(`timeout ${timeoutMs}ms`), timeoutMs);
  try {
    const status = await new Promise<number>((accept, reject) => { child.once("error", reject); child.once("close", (code) => accept(code ?? 1)); });
    assert.equal(stopped, "", `${command} stopped: ${stopped}\n${stderr}`);
    assert.equal(status, 0, stderr || `${command} exited with status ${status}`);
    return stdout;
  } finally {
    clearInterval(progress);
    clearTimeout(timeout);
    if (forceKill) clearTimeout(forceKill);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", stop);
  }
}

/** 🧬️ Validates every taxonomy artifact's language-neutral package boundary. */
class ArtifactPackageContractScript extends BundleScript {
  async run(): Promise<void> {
    const fixtureRoot = join(SCRIPT_ROOT, "🧫️fixtures/artifact-packages");
    const schema = JSON.parse(readFileSync(join(fixtureRoot, "🛂️schema/🔣️.json"), "utf8"));
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const { default: Ajv2020 } = await import("ajv/dist/2020.js");
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    for (const accepted of fixture.accepted) assert.ok(validate(accepted), JSON.stringify(validate.errors));
    for (const rejected of fixture.rejected) assert.equal(validate(rejected.value), false, `Negative fixture accepted: ${rejected.id}`);
    const contract = artifactPackageInventory(this.repoRoot);
    assert.ok(validate(contract), JSON.stringify(validate.errors));
    const cargoNames = new Set<string>(), nxNames = new Set<string>();
    for (const entry of contract.packages) {
      assert.ok(!cargoNames.has(entry.rust.cargoName), `Duplicate Cargo package ${entry.rust.cargoName}`);
      assert.ok(!nxNames.has(entry.rust.nxName), `Duplicate Nx project ${entry.rust.nxName}`);
      cargoNames.add(entry.rust.cargoName);
      nxNames.add(entry.rust.nxName);
      const packageRoot = join(this.repoRoot, dirname(entry.rust.manifest));
      const implementations = sourceFiles(packageRoot).filter((path) => /(?:^|\/)(?!📜️script\.ts$).+\.(?:rs|ts|tsx)$/.test(path) && !path.startsWith("dist/"));
      assert.deepEqual(implementations, [], `Rust package declarations contain implementation: ${implementations.join(", ")}`);
      const artifactMarker = "/🗿️artifacts/", marker = entry.root.indexOf(artifactMarker), parentSource = marker < 0 ? "" : `${entry.root.slice(0, marker)}/📦️packages/🦀️rust/🦀️.rs`;
      if (parentSource && existsSync(join(this.repoRoot, parentSource))) {
        const parent = readFileSync(join(this.repoRoot, parentSource), "utf8");
        assert.equal(/#\[path\s*=\s*"\.\.\/\.\.\/🗿️artifacts\//.test(parent), false, `${parentSource} recompiles taxonomy artifact implementation`);
        assert.equal(/^(?:pub\s+mod\s+(?:artifacts|editor|viewer)\b|pub\s+use\s+semio_s_artifact_)/m.test(parent), false, `${parentSource} publicly re-exports an artifact implementation`);
      }
      if (entry.typescript) {
        assert.ok(!nxNames.has(entry.typescript.nxName), `Duplicate Nx project ${entry.typescript.nxName}`);
        nxNames.add(entry.typescript.nxName);
        const declaration = JSON.parse(readFileSync(join(this.repoRoot, entry.typescript.manifest), "utf8"));
        assert.equal(declaration.type, "module");
        assert.equal(declaration.private, true);
        assert.equal(declaration.types, "./dist/🟦️.d.ts");
        assert.deepEqual(declaration.exports?.["."], entry.typescript.entry);
      }
    }
    const metadata = JSON.parse(await captureArtifactContract("cargo", ["metadata", "--locked", "--offline", "--format-version", "1"], this.repoRoot, 180_000));
    const cargoPackages = new Map<string, any>(metadata.packages.map((entry: any) => [entry.id, entry]));
    const byName = new Map<string, any>(metadata.packages.map((entry: any) => [entry.name, entry]));
    const nodes = new Map<string, any>((metadata.resolve?.nodes ?? []).map((entry: any) => [entry.id, entry]));
    for (const entry of contract.packages) {
      const cargo = byName.get(entry.rust.cargoName);
      assert.ok(cargo, `Cargo metadata omitted ${entry.rust.cargoName}`);
      assert.equal(slash(relative(this.repoRoot, cargo.manifest_path)), entry.rust.manifest);
      const visit = (id: string, requested: string[], defaults: boolean, route: string[], visited: Set<string>): void => {
        const dependency = cargoPackages.get(id);
        const enabled = new Set<string>(requested);
        if (defaults && dependency?.features?.default) enabled.add("default");
        const activated = new Set<string>();
        const forwarded = new Map<string, Set<string>>();
        const queue = [...enabled];
        while (queue.length) {
          const feature = queue.pop()!;
          for (const value of dependency?.features?.[feature] ?? []) {
            if (value.startsWith("dep:")) activated.add(value.slice(4));
            else if (value.includes("/")) {
              const [name, selected] = value.replace("?/", "/").split("/", 2);
              if (!forwarded.has(name)) forwarded.set(name, new Set());
              forwarded.get(name)!.add(selected);
            } else if (!enabled.has(value)) { enabled.add(value); queue.push(value); }
          }
        }
        const state = `${id}\0${[...enabled].sort().join(",")}\0${defaults}`;
        if (visited.has(state)) return;
        visited.add(state);
        const role = dependency?.metadata?.semio?.role;
        assert.ok(role !== "plugin" && role !== "extension", `${entry.rust.cargoName} reaches composition package ${[...route, dependency?.name].join(" -> ")}`);
        for (const edge of nodes.get(id)?.deps ?? []) {
          if (!edge.dep_kinds?.some((kind: any) => kind.kind === null)) continue;
          const target = cargoPackages.get(edge.pkg);
          const declarations = (dependency?.dependencies ?? []).filter((row: any) => row.kind === null && row.name === target?.name && (row.rename ?? row.name) === edge.name);
          const active = declarations.filter((row: any) => !row.optional || activated.has(edge.name) || enabled.has(edge.name));
          if (!active.length) continue;
          const features = new Set<string>(forwarded.get(edge.name) ?? []);
          for (const declaration of active) for (const feature of declaration.features ?? []) features.add(feature);
          visit(edge.pkg, [...features], active.some((row: any) => row.uses_default_features), [...route, dependency?.name], visited);
        }
      };
      visit(cargo.id, [], true, [], new Set());
    }
    const legacy = sourceFiles(this.repoRoot).filter((path) => {
      if (!path.endsWith(".rs")) return false;
      try { return /semio_s_plugin_(?![a-z0-9_]*_test_oracle::)[a-z0-9_]+::artifacts::/.test(readFileSync(join(this.repoRoot, path), "utf8")); }
      catch (error) { if ((error as { code?: string }).code === "ENOENT") return false; throw error; }
    });
    assert.deepEqual(legacy, [], `Rust consumers retain composition artifact namespaces: ${legacy.join(", ")}`);
    console.log(`[artifact-package-contract] AJV=${fixture.accepted.length + 1}/${fixture.rejected.length} packages=${contract.packages.length} rust=${cargoNames.size} typescript=${contract.packages.filter((entry) => entry.typescript).length} dag=clean`);
  }
}

const router = new ScriptRouter(SCRIPT_ROOT).register("test", TestScript).register("audit", AuditScript).register("policy-check", PolicyScript).register("artifact-check", PolicyScript).register("artifact-package-contract", ArtifactPackageContractScript).register("graph-check", GraphScript).register("doctor", DoctorScript).register("disk-report", DiskScript).register("disk-prune", DiskPruneScript).register("cache-verify", CacheVerifyScript);
const createCachePolicyTestsInstance = createCachePolicyTests({ assert, cacheInternals, chmodSync, copyFileSync, createRequire, devToolingEnv, dirname, EventEmitter, existsSync, getWorkspaceRoot, inventory, join, lstatSync, mkdirSync, mkdtempSync, plugin, readFileSync, relative, resolve, rmSync, SCRIPT_ROOT, slash, spawn, stageArtifacts, ticketOutput, utimesSync, wasmBindgenVersion, wasmBuildArguments, wasmBuildEnvironment, writeFileSync }, { directory: import.meta.dir, url: import.meta.url });
export const testCacheContracts = createCachePolicyTestsInstance.testCacheContracts;


if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
