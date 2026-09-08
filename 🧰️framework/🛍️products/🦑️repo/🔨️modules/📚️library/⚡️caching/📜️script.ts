#!/usr/bin/env bun
import assert from "node:assert/strict";
import { readFileSync, mkdirSync, writeFileSync, rmSync, existsSync, readdirSync, lstatSync, realpathSync, copyFileSync, chmodSync, symlinkSync, renameSync, mkdtempSync } from "node:fs";
import { join, resolve, relative, dirname, sep } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { spawn } from "node:child_process";
import { createInterface } from "node:readline";
import { createHash } from "node:crypto";
import { BundleScript, ScriptRouter, runBundleScriptMain, getWorkspaceRoot, runCmdStatus, wasmBuildEnvironment, wasmBindgenVersion } from "../📦️packages/🟦️typescript/🟦️.ts";
import plugin, { cacheInternals } from "../🟨️.mjs";

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
  async run(): Promise<void> {
    const { createProjectGraphAsync } = createRequire(import.meta.url)("@nx/devkit");
    const graph = await createProjectGraphAsync();
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

/** 🧱️ Replaces only a previously owned deliverable tree and restores it if publication fails. */
export function stageArtifacts(staging: string, owner: string, files: ReadonlyMap<string, string>): void {
  const marker = ".nx-artifact.json";
  for (let parent = resolve(staging); dirname(parent) !== parent; parent = dirname(parent)) if (existsSync(parent) && lstatSync(parent).isSymbolicLink()) throw new Error(`Symlink artifact destination: ${parent}`);
  if (existsSync(staging) && (!existsSync(join(staging, marker)) || JSON.parse(readFileSync(join(staging, marker), "utf8")).owner !== owner)) throw new Error(`Unowned artifact directory: ${staging}`);
  mkdirSync(dirname(staging), { recursive: true });
  const lease = `${staging}.lease`;
  writeFileSync(lease, JSON.stringify({ owner, pid: process.pid }), { flag: "wx" });
  let temporary: string | undefined;
  try { temporary = mkdtempSync(`${staging}.stage-`); }
  catch (error) { rmSync(lease); throw error; }
  const previous = `${temporary}.previous`;
  try {
    for (const [name, source] of files) {
      if (name.startsWith("/") || name.split(/[\\/]/).includes("..")) throw new Error(`Invalid artifact path ${name}`);
      const destination = join(temporary, name);
      mkdirSync(dirname(destination), { recursive: true });
      copyFileSync(source, destination);
      chmodSync(destination, lstatSync(source).mode & 0o777);
    }
    writeFileSync(join(temporary, marker), JSON.stringify({ version: 1, owner, files: [...files.keys()].sort() }) + "\n");
    if (existsSync(staging)) renameSync(staging, previous);
    try { renameSync(temporary, staging); }
    catch (error) { if (existsSync(previous)) renameSync(previous, staging); throw error; }
    rmSync(previous, { recursive: true, force: true });
  } finally { rmSync(temporary, { recursive: true, force: true }); rmSync(lease); }
}

/** 📦️ Captures Cargo's declared deliverables, including link dependencies, without copying compiler state. */
export async function buildCargoArtifacts(manifest: string, args: string[] = [], repoRoot = getWorkspaceRoot()): Promise<void> {
  const path = resolve(repoRoot, manifest);
  const sourceRoot = dirname(path);
  const staging = join(sourceRoot, "dist", "build");
  const owner = slash(relative(repoRoot, path));
  const files = new Map<string, string>();
  const dependencies = new Map<string, string>();
  let hasLibrary = false;
  let cancelled = false;
  let forceKill: ReturnType<typeof setTimeout> | undefined;
  const child = spawn("cargo", ["build", "--locked", "--manifest-path", path, ...args, "--message-format=json-render-diagnostics"], { cwd: repoRoot, env: process.env, detached: process.platform !== "win32", stdio: ["inherit", "pipe", "inherit"] });
  const cancel = (): void => {
    cancelled = true;
    if (!child.pid) return;
    if (process.platform === "win32") Bun.spawnSync(["taskkill", "/pid", String(child.pid), "/t", "/f"], { stdout: "ignore", stderr: "ignore" });
    else {
      try { process.kill(-child.pid, "SIGTERM"); } catch {}
      forceKill = setTimeout(() => { try { process.kill(-child.pid!, "SIGKILL"); } catch {} }, 2000);
      forceKill.unref();
    }
  };
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const status = new Promise<number>((accept) => { child.once("error", (error) => { console.error(error.message); accept(1); }); child.once("close", (code) => accept(code ?? 1)); });
  try {
    for await (const line of createInterface({ input: child.stdout!, crlfDelay: Infinity })) {
      let message;
      try { message = JSON.parse(line); } catch { process.stdout.write(line + "\n"); continue; }
      if (message.reason !== "compiler-artifact" || message.target?.kind?.includes("custom-build")) continue;
      const packageUrl = message.package_id?.split("#")[0]?.replace(/^path\+/, "");
      const bin = args.indexOf("--bin"), example = args.indexOf("--example");
      const selected = bin >= 0 ? message.target?.kind?.includes("bin") && message.target.name === args[bin + 1] : example >= 0 ? message.target?.kind?.includes("example") && message.target.name === args[example + 1] : args.includes("--bins") ? message.target?.kind?.includes("bin") : true;
      const primary = selected && packageUrl?.startsWith("file:") && resolve(fileURLToPath(packageUrl)) === sourceRoot;
      for (const file of message.filenames ?? []) {
        if (file.endsWith(".d")) continue;
        const library = primary && file.endsWith(".rmeta") ? message.filenames.find((path: string) => path.endsWith(".rlib")) : undefined;
        const name = (library ? library.replace(/\.rlib$/, ".rmeta") : file).split(/[\\/]/).at(-1)!;
        if (primary) { files.set(name, file); hasLibrary ||= file.endsWith(".rlib"); }
        else if (/\.(rlib|rmeta|so|dylib|dll|lib)$/.test(file)) dependencies.set(`deps/${name}`, file);
      }
    }
    if (await status !== 0 || cancelled) throw new Error(`Cargo artifact build ${cancelled ? "cancelled" : "failed"}: ${owner}`);
  } finally { if (forceKill) clearTimeout(forceKill); process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  if (files.size === 0) throw new Error(`Cargo emitted no final artifacts for ${owner}`);
  if (hasLibrary) for (const [name, file] of dependencies) files.set(name, file);
  stageArtifacts(staging, owner, files);
  console.log(`[nx-native] staged ${files.size} deliverables in ${slash(relative(repoRoot, staging))}`);
}

class NativeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [tool, operation] = args;
    const index = args.indexOf("--manifest");
    const manifest = index >= 0 ? args[index + 1] : undefined;
    if (tool !== "cargo" || !["build", "check", "test"].includes(operation) || !manifest) throw new Error("native cargo build|check|test --manifest <Cargo.toml>");
    const extra = args.slice(index + 2);
    if (operation === "build") return buildCargoArtifacts(manifest, extra, this.repoRoot);
    const status = runCmdStatus("cargo", [operation, "--locked", "--manifest-path", resolve(this.repoRoot, manifest), ...extra], { cwd: this.repoRoot });
    if (status) throw new Error(`cargo ${operation} failed (${status})`);
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
      const child = Bun.spawn(["node", join(this.repoRoot, "node_modules/nx/bin/nx.js"), "run", `probe:${task}`, "--output-style=static"], { cwd: fixture, env, stdout: "pipe", stderr: "pipe" });
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
  for (const row of vectors.policies) {
    const target = cacheInternals.targetPolicy(row.target, { cache: true }, policy);
    assert.deepEqual({ cache: target.cache, continuous: target.continuous ?? false }, { cache: row.cache, continuous: row.continuous }, row.target);
  }
  const result = inventory(root), contracts = result.projects;
  assert.deepEqual(result.violations.filter((finding) => finding.rule === "ORCH-01"), [], "All public commands must use one script entrypoint");
  for (const entry of vectors.entrypoints) assert.ok(contracts.find((project) => project.name === entry.project)?.targets[entry.target].dependsOn?.includes(entry.prerequisite), `${entry.project}:${entry.target} needs ${entry.prerequisite} in Nx`);
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
  const source = ts.createSourceFile("📜️script.ts", readFileSync(join(root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
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
  const fixture = join(resolve(root, ticket), "🗑️generated", "policy-contract");
  const rustFixture = join(fixture, "rust-inputs");
  for (const [path, contents] of Object.entries(vectors.rust.files)) { mkdirSync(dirname(join(rustFixture, path)), { recursive: true }); writeFileSync(join(rustFixture, path), contents as string); }
  const rustInputs = cacheInternals.rustSourceFiles([join(rustFixture, vectors.rust.entry)], rustFixture).map((path: string) => slash(relative(rustFixture, path))).sort();
  assert.deepEqual(rustInputs, vectors.rust.inputs);
  const rustOracle = Bun.spawnSync(["rustc", "--crate-type=lib", "--emit=dep-info", "-o", join(rustFixture, "oracle.d"), vectors.rust.entry], { cwd: rustFixture, stdout: "pipe", stderr: "pipe" });
  assert.equal(rustOracle.exitCode, 0, rustOracle.stderr.toString());
  const rustDependencies = readFileSync(join(rustFixture, "oracle.d"), "utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/).map((path) => slash(relative(rustFixture, resolve(rustFixture, path)))).sort();
  assert.deepEqual(rustInputs, [...new Set(rustDependencies)]);
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
  assert.equal(root.length > 0, true);
  rmSync(fixture, { recursive: true });
  console.log("[cache-contract] schema, side effects, continuous tasks, duplicate identities, levels and native dependency cases passed");
}

class TestScript extends BundleScript {
  async run(): Promise<void> { await testCacheContracts(); }
}

const router = new ScriptRouter(SCRIPT_ROOT).register("test", TestScript).register("audit", AuditScript).register("policy-check", PolicyScript).register("artifact-check", PolicyScript).register("graph-check", GraphScript).register("doctor", DoctorScript).register("disk-report", DiskScript).register("disk-prune", DiskPruneScript).register("cache-verify", CacheVerifyScript).register("native", NativeScript).register("toolchain", ToolchainScript).register("generator-inputs", GeneratorInputsScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
