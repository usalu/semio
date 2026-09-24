/** 🥾️ Cross-platform zero-touch laws for a fresh clone on devcontainer, native Windows, macOS and Linux: the stock
 * Windows path budget, script encodings Windows PowerShell 5.1 and bash read correctly, the distribution-neutral Linux
 * linker, repo-local git configuration, the native bootstraps' provisioning contract and clone-resolvable workspaces.
 * Every measurement is cross-checked against an independent implementation (iconv-lite, smol-toml, ignore, git). @see ../../🧫️fixtures/🥾️cross-platform-bootstrap/🔣️.json */
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, lstatSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { extname, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import { build, type Plugin } from "esbuild";
import iconv from "iconv-lite";
import ignore from "ignore";
import * as SmolToml from "smol-toml";

type Fixture = Readonly<{
  pathBudget: Readonly<{ unit: "utf16"; windowsMaxPath: number; cloneRootMax: number; componentMax: number }>;
  scriptEncoding: Readonly<{ bomExtensions: readonly string[]; lfExtensions: readonly string[]; excludedPrefixes: readonly string[] }>;
  cargoLinker: Readonly<{ config: string; linuxTable: string; linuxRustflags: readonly string[]; forbiddenRustflagFragments: readonly string[] }>;
  repoLocalGitConfig: readonly (readonly [string, string])[];
  unixBootstrap: Readonly<{ path: string; packageManagers: readonly string[]; requiredTools: readonly string[]; forbidden: readonly string[] }>;
  windowsBootstrap: Readonly<{ path: string; required: readonly string[]; forbidden: readonly string[] }>;
  nodePin: Readonly<{ manifest: string; dockerfile: string; dockerfileArg: string }>;
  workspaceManifests: Readonly<{ rootManifest: string }>;
  freshClone: Readonly<{ bootstrapSources: string; bootstrapSourcesSchema: string; entries: readonly string[] }>;
}>;
type BootstrapSources = Readonly<{ sources: readonly Readonly<{ id: string; module: string; export: string; outputs: readonly string[]; requiredBy: string }>[] }>;

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(libraryRoot, "../../../../..");
const fixture = JSON.parse(readFileSync(join(libraryRoot, "🧫️fixtures/🥾️cross-platform-bootstrap/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(join(libraryRoot, "🧬️schema/🥾️cross-platform-bootstrap/🔣️.json"), "utf8"));
const BOM = Buffer.from([0xef, 0xbb, 0xbf]);
const bootstrapSources = JSON.parse(readFileSync(join(repoRoot, fixture.freshClone.bootstrapSources), "utf8")) as BootstrapSources;
const bootstrapOutputs = new Set(bootstrapSources.sources.flatMap((source) => source.outputs));

const workspacePackages: Plugin = {
  name: "workspace-packages",
  setup(builder) {
    builder.onResolve({ filter: /^[^./]/ }, (args) => {
      if (args.path.startsWith("node:")) return { path: args.path, external: true };
      try {
        const real = realpathSync(Bun.resolveSync(args.path, args.resolveDir));
        return real.includes("/node_modules/") ? { path: args.path, external: true } : { path: real };
      } catch {
        return { path: args.path, external: true };
      }
    });
  },
};

async function staticClosure(entry: string): Promise<Readonly<{ inputs: readonly string[]; externals: readonly string[] }>> {
  const result = await build({ absWorkingDir: repoRoot, entryPoints: [entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", plugins: [workspacePackages], loader: { ".node": "empty", ".wasm": "empty", ".css": "empty", ".svg": "empty", ".png": "empty" } });
  const inputs = Object.keys(result.metafile.inputs).map((path) => relative(repoRoot, resolve(repoRoot, path)).replaceAll("\\", "/"));
  const externals = Object.values(result.metafile.outputs).flatMap((output) => output.imports.filter((dependency) => dependency.external).map((dependency) => dependency.path));
  return { inputs, externals };
}

function ignoredPaths(paths: readonly string[], patternsOnly = false): Set<string> {
  const result = spawnSync("git", ["-c", "core.quotepath=off", "check-ignore", ...(patternsOnly ? ["--no-index"] : []), "--stdin", "-z"], { cwd: repoRoot, input: paths.join("\0") + "\0", encoding: "utf8" });
  return new Set(result.stdout.split("\0").filter(Boolean));
}

function git(args: readonly string[], cwd = repoRoot, env: NodeJS.ProcessEnv = process.env): string {
  const result = spawnSync("git", [...args], { cwd, env, encoding: "utf8", maxBuffer: 256 * 1024 * 1024, windowsHide: true });
  if (result.status !== 0) throw new Error(`git ${args.join(" ")} failed: ${result.stderr}`);
  return result.stdout;
}

const tracked = git(["-c", "core.quotepath=off", "ls-files", "-z"]).split("\0").filter(Boolean);

type PathBudgetReport = Readonly<{ max: number; overBudget: readonly string[]; overComponent: readonly string[] }>;

function measurePaths(paths: readonly string[], units: (value: string) => number, budget: number, componentMax: number): PathBudgetReport {
  let max = 0;
  const overBudget: string[] = [], overComponent: string[] = [];
  for (const path of paths) {
    const length = units(path);
    if (length > max) max = length;
    if (length > budget) overBudget.push(path);
    if (path.split("/").some((component) => units(component) > componentMax)) overComponent.push(path);
  }
  return { max, overBudget, overComponent };
}

function productScripts(extensions: readonly string[]): string[] {
  return tracked.filter((path) => extensions.includes(extname(path)) && !fixture.scriptEncoding.excludedPrefixes.some((prefix) => path.startsWith(prefix)));
}

function shellFunction(source: string, name: string): string {
  const match = source.match(new RegExp(`^${name}\\(\\) \\{[\\s\\S]*?^\\}`, "mu"));
  if (!match) throw new Error(`${name} is not defined as a top-level shell function`);
  return match[0];
}

describe("cross-platform bootstrap", () => {
  test("validates the portable law fixture", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
  });

  test("keeps every tracked path inside the stock Windows MAX_PATH budget below a declared clone root", () => {
    const { windowsMaxPath, cloneRootMax, componentMax } = fixture.pathBudget;
    const budget = windowsMaxPath - 1 - cloneRootMax;
    const ours = measurePaths(tracked, (value) => value.length, budget, componentMax);
    const oracle = measurePaths(tracked, (value) => iconv.encode(value, "utf16le").length / 2, budget, componentMax);
    expect(tracked.length).toBeGreaterThan(1000);
    expect(ours).toEqual(oracle);
    expect(ours.overComponent).toEqual([]);
    expect(ours.overBudget, `tracked relative paths above ${budget} UTF-16 units (max ${ours.max})`).toEqual([]);
  }, 60_000);

  test("saves every non-ASCII PowerShell script with a UTF-8 BOM so Windows PowerShell 5.1 decodes it as UTF-8", () => {
    const scripts = productScripts(fixture.scriptEncoding.bomExtensions);
    expect(scripts.length).toBeGreaterThan(0);
    for (const script of scripts) {
      const bytes = readFileSync(join(repoRoot, script));
      if (!bytes.some((byte) => byte >= 0x80)) continue;
      const ours = bytes.subarray(0, 3).equals(BOM);
      const oracle = iconv.decode(bytes, "utf8") !== bytes.toString("utf8");
      expect({ script, ours }).toEqual({ script, ours: oracle });
      expect({ script, bom: ours }).toEqual({ script, bom: true });
    }
  });

  test("checks shell scripts out with LF endings and keeps them parseable by bash", () => {
    const scripts = productScripts(fixture.scriptEncoding.lfExtensions);
    expect(scripts.length).toBeGreaterThan(0);
    const attributes = git(["check-attr", "eol", "--", ...scripts]).trim().split("\n");
    for (const [index, script] of scripts.entries()) {
      expect({ script, cr: readFileSync(join(repoRoot, script)).includes(0x0d) }).toEqual({ script, cr: false });
      expect(attributes[index]).toBe(`${script}: eol: lf`);
    }
    if (process.platform === "win32") return;
    for (const script of scripts) {
      const parsed = spawnSync("bash", ["-n", join(repoRoot, script)], { encoding: "utf8" });
      expect({ script, status: parsed.status, stderr: parsed.stderr }).toEqual({ script, status: 0, stderr: "" });
    }
  });

  test("links every glibc Linux host with the toolchain's rust-lld and no distribution linker", () => {
    const source = readFileSync(join(repoRoot, fixture.cargoLinker.config), "utf8");
    const ours = Bun.TOML.parse(source) as { target: Record<string, { rustflags?: string[] }> };
    const oracle = SmolToml.parse(source);
    expect(JSON.parse(JSON.stringify(ours))).toEqual(JSON.parse(JSON.stringify(oracle)));
    expect(ours.target[fixture.cargoLinker.linuxTable]?.rustflags).toEqual([...fixture.cargoLinker.linuxRustflags]);
    expect(Object.keys(ours.target).filter((table) => /linux/u.test(table))).toEqual([fixture.cargoLinker.linuxTable]);
    const flags = Object.values(ours.target).flatMap((table) => table.rustflags ?? []);
    for (const fragment of fixture.cargoLinker.forbiddenRustflagFragments) expect(flags.filter((flag) => flag.includes(fragment))).toEqual([]);
  });

  test("writes only repo-local git configuration and leaves the user's global git config untouched", async () => {
    const root = mkdtempSync(join(tmpdir(), "semio-setup-git-"));
    const globalConfig = join(root, "user.gitconfig");
    const sentinel = "[user]\n\tname = Sentinel\n";
    const previous = process.env.GIT_CONFIG_GLOBAL;
    try {
      writeFileSync(globalConfig, sentinel);
      process.env.GIT_CONFIG_GLOBAL = globalConfig;
      const clone = join(root, "clone");
      git(["init", "-q", clone], root);
      writeFileSync(join(clone, "AGENTS.md"), "# agents\n");
      const { SetupScript, REPO_LOCAL_GIT_CONFIG } = await import(join(repoRoot, "📜️script.ts"));
      expect(REPO_LOCAL_GIT_CONFIG).toEqual(fixture.repoLocalGitConfig);
      await new SetupScript(clone, clone).run(["git"]);
      for (const [key, value] of fixture.repoLocalGitConfig) expect(git(["config", "--local", "--get", key], clone).trim()).toBe(value);
      expect(readFileSync(globalConfig, "utf8")).toBe(sentinel);
      expect(lstatSync(join(clone, "CLAUDE.md")).isSymbolicLink() || process.platform === "win32").toBe(true);
      expect(readFileSync(join(clone, "CLAUDE.md"), "utf8")).toBe("# agents\n");
    } finally {
      if (previous === undefined) delete process.env.GIT_CONFIG_GLOBAL;
      else process.env.GIT_CONFIG_GLOBAL = previous;
      rmSync(root, { recursive: true, force: true });
    }
  }, 120_000);

  test("provisions the Unix toolchain through every supported package manager without user-global configuration", () => {
    const source = readFileSync(join(repoRoot, fixture.unixBootstrap.path), "utf8");
    for (const fragment of fixture.unixBootstrap.forbidden) expect({ fragment, present: source.includes(fragment) }).toEqual({ fragment, present: false });
    const packages = shellFunction(source, "linux_toolchain_packages");
    expect(shellFunction(source, "linux_package_manager")).toContain(`for manager in ${fixture.unixBootstrap.packageManagers.join(" ")}; do`);
    expect(source).toContain(`report_missing_tools ${fixture.unixBootstrap.requiredTools.join(" ")}`);
    expect(source.match(/\bapt-get (update|install)\b/gu)?.length ?? 0).toBe(source.match(/run_privileged env DEBIAN_FRONTEND=noninteractive apt-get (update|install)\b/gu)?.length ?? 0);
    const calls = source.replace(/\\\n\s*/gu, " ").split("\n").filter((line) => /^\s*install_linux_packages\s/u.test(line));
    expect(calls.length).toBeGreaterThan(1);
    for (const call of calls) expect({ call: call.trim(), guarded: call.includes("|| log ") }).toEqual({ call: call.trim(), guarded: true });
    if (process.platform === "win32") return;
    for (const manager of fixture.unixBootstrap.packageManagers) {
      const listed = spawnSync("bash", ["-c", `${packages}\nlinux_toolchain_packages "$1"`, "law", manager], { encoding: "utf8" });
      expect({ manager, status: listed.status }).toEqual({ manager, status: 0 });
      expect(listed.stdout.trim().split("\n").length, manager).toBeGreaterThan(8);
    }
  });

  test("pins one Node.js for Nx across the devcontainer image and the native bootstraps", () => {
    const pinned = (JSON.parse(readFileSync(join(repoRoot, fixture.nodePin.manifest), "utf8")) as { engines: { node: string } }).engines.node;
    const image = readFileSync(join(repoRoot, fixture.nodePin.dockerfile), "utf8").match(new RegExp(`^ARG ${fixture.nodePin.dockerfileArg}=(.+)$`, "mu"))?.[1];
    expect(pinned).toMatch(/^\d+\.\d+\.\d+$/u);
    expect(image).toBe(pinned);
    expect(readFileSync(join(repoRoot, fixture.unixBootstrap.path), "utf8")).toContain('require("./package.json").engines.node');
    expect(readFileSync(join(repoRoot, fixture.windowsBootstrap.path), "utf8")).toContain('"OpenJS.NodeJS.LTS"');
  });

  test("guides Windows long paths in every supported language and never writes user-global tool configuration", () => {
    const source = readFileSync(join(repoRoot, fixture.windowsBootstrap.path), "utf8");
    for (const fragment of fixture.windowsBootstrap.required) expect({ fragment, present: source.includes(fragment) }).toEqual({ fragment, present: true });
    for (const fragment of fixture.windowsBootstrap.forbidden) expect({ fragment, present: source.includes(fragment) }).toEqual({ fragment, present: false });
  });

  test("lists only workspace members a fresh clone checks out", () => {
    const manifest = JSON.parse(readFileSync(join(repoRoot, fixture.workspaceManifests.rootManifest), "utf8")) as { workspaces: string[] | { packages: string[] } };
    const members = Array.isArray(manifest.workspaces) ? manifest.workspaces : manifest.workspaces.packages;
    const manifests = members.map((member) => `${member}/package.json`);
    const ignored = new Set(spawnSync("git", ["-c", "core.quotepath=off", "check-ignore", "--stdin", "-z"], { cwd: repoRoot, input: manifests.join("\0") + "\0", encoding: "utf8" }).stdout.split("\0").filter(Boolean));
    const trackedSet = new Set(tracked);
    const rootRules = ignore().add(readFileSync(join(repoRoot, ".gitignore"), "utf8"));
    expect(manifests.length).toBeGreaterThan(10);
    for (const path of manifests) {
      const nested = path.split("/").slice(0, -1).some((_, index, parts) => existsSync(join(repoRoot, ...parts.slice(0, index + 1), ".gitignore")));
      if (!nested && !trackedSet.has(path)) expect({ path, ignored: rootRules.ignores(path) }).toEqual({ path, ignored: ignored.has(path) });
      expect({ path, cloneResolvable: !ignored.has(path) || bootstrapOutputs.has(path) }).toEqual({ path, cloneResolvable: true });
      expect({ path, exists: existsSync(join(repoRoot, path)) }).toEqual({ path, exists: true });
    }
  });
  test("publishes bootstrap sources from system-only modules a fresh clone already has", async () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(repoRoot, fixture.freshClone.bootstrapSourcesSchema), "utf8")));
    expect(validate(bootstrapSources), JSON.stringify(validate.errors)).toBe(true);
    for (const source of bootstrapSources.sources) {
      const closure = await staticClosure(source.module);
      expect({ id: source.id, ignored: [...ignoredPaths(closure.inputs)] }).toEqual({ id: source.id, ignored: [] });
      expect({ id: source.id, externals: closure.externals.filter((specifier) => !specifier.startsWith("node:")) }).toEqual({ id: source.id, externals: [] });
      expect({ id: source.id, export: typeof (await import(pathToFileURL(join(repoRoot, source.module)).href))[source.export] }).toEqual({ id: source.id, export: "function" });
    }
    const { publishBootstrapSources } = await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"));
    publishBootstrapSources(repoRoot);
    const outputs = [...bootstrapOutputs];
    expect(outputs.filter((output) => !existsSync(join(repoRoot, output)))).toEqual([]);
    expect([...ignoredPaths(outputs, true)].sort()).toEqual([...outputs].sort());
  }, 120_000);

  test("loads every setup-path script on a fresh clone once the bootstrap sources exist", async () => {
    for (const entry of fixture.freshClone.entries) {
      const closure = await staticClosure(entry);
      expect(closure.inputs.length, entry).toBeGreaterThan(0);
      const missingOnClone = [...ignoredPaths(closure.inputs)].filter((path) => !bootstrapOutputs.has(path));
      expect({ entry, missingOnClone }).toEqual({ entry, missingOnClone: [] });
    }
  }, 300_000);
});
