/** 🥾️ Cross-platform zero-touch laws for a fresh clone on devcontainer, native Windows, macOS and Linux: the stock
 * Windows path budget, script encodings Windows PowerShell 5.1 and bash read correctly, the distribution-neutral Linux
 * linker, repo-local git configuration, the native bootstraps' provisioning contract and clone-resolvable workspaces.
 * Every measurement is cross-checked against an independent implementation (iconv-lite, smol-toml, ignore, git). @see ../../🧫️fixtures/🥾️cross-platform-bootstrap/🔣️.json */
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { existsSync, lstatSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, extname, join, relative, resolve } from "node:path";
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
  unixBootstrap: Readonly<{ path: string; packageManagers: readonly string[]; requiredTools: readonly string[]; forbidden: readonly string[]; installerInvocations: number }>;
  windowsBootstrap: Readonly<{ path: string; required: readonly string[]; forbidden: readonly string[] }>;
  nativePins: Readonly<{ launcherRegistry: string; hosts: readonly string[]; tools: readonly string[]; platforms: readonly string[]; architectures: readonly string[]; rowShape: string; forbiddenInstallers: readonly string[]; playwrightCache: string }>;
  windowsLongPaths: Readonly<{ cases: readonly Readonly<{ output: string; expected: boolean | null }>[] }>;
  nodePin: Readonly<{ manifest: string; dockerfile: string; dockerfileArg: string }>;
  workspaceManifests: Readonly<{ rootManifest: string }>;
  freshClone: Readonly<{ bootstrapSources: string; bootstrapSourcesSchema: string; entries: readonly string[]; deferredImports: Readonly<{ source: string; imports: readonly Readonly<{ path: string; kind: string }>[] }> }>;
}>;
type BootstrapSources = Readonly<{ sources: readonly Readonly<{ id: string; module: string; export: string; outputs: readonly string[]; requiredBy: string }>[] }>;

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(libraryRoot, "../../../../..");
const fixture = JSON.parse(readFileSync(join(libraryRoot, "🧪️tests/🧫️fixtures/🥾️cross-platform-bootstrap/🔣️.json"), "utf8")) as Fixture;

const BOM = Buffer.from([0xef, 0xbb, 0xbf]);
const bootstrapSources = JSON.parse(readFileSync(join(repoRoot, fixture.freshClone.bootstrapSources), "utf8")) as BootstrapSources;
const bootstrapOutputs = new Set(bootstrapSources.sources.flatMap((source) => source.outputs));

const workspacePackages: Plugin = {
  name: "workspace-packages",
  setup(builder) {
    builder.onResolve({ filter: /.*/ }, (args) => args.kind === "dynamic-import" ? { path: args.path, external: true } : undefined);
    builder.onResolve({ filter: /^[^./]/ }, (args) => {
      if (args.path.startsWith("node:")) return { path: args.path, external: true };
      try {
        const real = realpathSync(Bun.resolveSync(args.path, args.resolveDir));
        return real.split(/[\\/]/u).includes("node_modules") ? { path: args.path, external: true } : { path: real };
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
  test("Nx entrypoints resolve the declared package layout through independent loaders", () => {
    const contract = JSON.parse(readFileSync(join(libraryRoot, "🧫️fixtures/🥾️nx-entrypoints/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(libraryRoot, "🧬️schema/🥾️nx-entrypoints/🔣️.json"), "utf8"));
    expect(new Ajv({ allErrors: true, strict: true }).compile(schema)(contract)).toBe(true);
    const nodeOracle = spawnSync("node", ["--input-type=module", "-e", 'import {createRequire} from "node:module";import {readFileSync,realpathSync} from "node:fs";import {resolve} from "node:path";const rows=JSON.parse(readFileSync(0,"utf8")),load=createRequire(resolve(".nx/installation/package.json"));process.stdout.write(JSON.stringify(rows.map(row=>realpathSync(load.resolve(row.specifier)))));'], { cwd: repoRoot, input: JSON.stringify(contract.cases), encoding: "utf8", windowsHide: true, timeout: 10_000 });
    expect(nodeOracle.status).toBe(0);
    const nodeTargets = JSON.parse(nodeOracle.stdout) as string[];
    const load = createRequire(join(repoRoot, ".nx/installation/package.json"));
    const metadata = load("nx/package.json") as { bin: { nx: string } };
    for (const row of contract.cases as { id: string; source: string; pattern: string; specifier: string; target: string }[]) {
      const original = readFileSync(join(libraryRoot, row.source), "utf8").match(new RegExp(row.pattern))?.[1];
      expect(original).toBe(row.specifier);
      const independent = realpathSync(load.resolve(row.specifier));
      expect(realpathSync(Bun.resolveSync(original!, join(repoRoot, ".nx/installation")))).toBe(independent);
      expect(nodeTargets[contract.cases.indexOf(row)]).toBe(independent);
      expect(independent).toBe(realpathSync(join(dirname(load.resolve("nx/package.json")), row.target)));
      if (row.id === "cli") expect(independent).toBe(realpathSync(join(dirname(load.resolve("nx/package.json")), metadata.bin.nx)));
    }
  });

  test("keeps deferred command imports distinct from initial router imports", async () => {
    const result = await build({ stdin: { contents: fixture.freshClone.deferredImports.source, resolveDir: repoRoot, loader: "ts" }, bundle: true, write: false, metafile: true, platform: "node", format: "esm", plugins: [workspacePackages] });
    const imports = Object.values(result.metafile.outputs).flatMap((output) => output.imports.map(({ path, kind }) => ({ path, kind })));
    expect(imports).toEqual(fixture.freshClone.deferredImports.imports as typeof imports);
    expect(Object.keys(result.metafile.inputs)).toEqual(["<stdin>"]);
  });
  test("validates the portable law fixture", () => {
    
    expect(fixture["pathBudget"]["unit"]).toEqual("utf16");
    
  });

  test("keeps every tracked path inside the stock Windows MAX_PATH budget below a declared clone root", () => {
    const { windowsMaxPath, cloneRootMax, componentMax } = fixture.pathBudget;
    const fileBudget = windowsMaxPath - 1 - cloneRootMax;
    // CreateDirectoryW rejects a directory at MAX_PATH - 12, so the directory string itself stops one unit earlier.
    const directoryBudget = windowsMaxPath - 12 - 1 - cloneRootMax;
    const directories = [...new Set(tracked.flatMap((path) => path.split("/").slice(0, -1).map((_, index, parts) => parts.slice(0, index + 1).join("/"))))];
    const utf16 = (value: string) => iconv.encode(value, "utf16le").length / 2;
    const files = measurePaths(tracked, (value) => value.length, fileBudget, componentMax);
    const fileOracle = measurePaths(tracked, utf16, fileBudget, componentMax);
    const dirs = measurePaths(directories, (value) => value.length, directoryBudget, componentMax);
    const dirOracle = measurePaths(directories, utf16, directoryBudget, componentMax);
    expect(tracked.length).toBeGreaterThan(1000);
    expect(files).toEqual(fileOracle);
    expect(dirs).toEqual(dirOracle);
    expect(files.overComponent).toEqual([]);
    expect(dirs.overComponent).toEqual([]);
    expect(files.overBudget, `tracked relative paths above ${fileBudget} UTF-16 units (max ${files.max})`).toEqual([]);
    expect(dirs.overBudget, `tracked directories above ${directoryBudget} UTF-16 units (max ${dirs.max})`).toEqual([]);
  }, 60_000);

  test("keeps current product paths inside the stock Windows MAX_PATH budget", () => {
    const paths = [...new Set(git(["-c", "core.quotepath=off", "ls-files", "-z", "--cached", "--others", "--exclude-standard"]).split("\0").filter(path => path && !path.includes("/🎫️tickets/") && existsSync(join(repoRoot, path))))];
    const { windowsMaxPath, cloneRootMax, componentMax } = fixture.pathBudget;
    const directories = [...new Set(paths.flatMap(path => path.split("/").slice(0, -1).map((_, index, parts) => parts.slice(0, index + 1).join("/"))))];
    const utf16 = (value: string) => iconv.encode(value, "utf16le").length / 2;
    const files = measurePaths(paths, value => value.length, windowsMaxPath - 1 - cloneRootMax, componentMax);
    const dirs = measurePaths(directories, value => value.length, windowsMaxPath - 13 - cloneRootMax, componentMax);
    expect(paths.length).toBeGreaterThan(1000);
    expect(files).toEqual(measurePaths(paths, utf16, windowsMaxPath - 1 - cloneRootMax, componentMax));
    expect(dirs).toEqual(measurePaths(directories, utf16, windowsMaxPath - 13 - cloneRootMax, componentMax));
    expect(files.overComponent).toEqual([]);
    expect(dirs.overComponent).toEqual([]);
    expect(files.overBudget).toEqual([]);
    expect(dirs.overBudget).toEqual([]);
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
    const attributes = git(["check-attr", "-z", "eol", "--", ...scripts]).split("\0");
    for (const [index, script] of scripts.entries()) {
      expect({ script, cr: readFileSync(join(repoRoot, script)).includes(0x0d) }).toEqual({ script, cr: false });
      expect(attributes.slice(index * 3, index * 3 + 3)).toEqual([script, "eol", "lf"]);
    }
  });

  test.skipIf(process.platform === "win32")("parses every shell script with bash -n (POSIX hosts)", () => {
    const scripts = productScripts(fixture.scriptEncoding.lfExtensions);
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
      const { SetupScript, REPO_LOCAL_GIT_CONFIG } = await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts"));
      expect(REPO_LOCAL_GIT_CONFIG).toEqual(fixture.repoLocalGitConfig);
      const { createScriptProcessEnvelope, withScriptProcessEnvelope } = await import(join(repoRoot, "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts"));
      await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "cross-platform-bootstrap-test", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async (invocation: unknown) => { await new SetupScript(clone, clone, invocation).run(["git"]); });
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
    const calls = shellFunction(source, "ensure_native_toolchain").replace(/\\\n\s*/gu, " ").split("\n").filter((line) => /^\s*install_linux_packages\s/u.test(line));
    expect(calls.length).toBe(fixture.unixBootstrap.installerInvocations);
    expect(shellFunction(source, "ensure_native_toolchain")).toContain('done < <(linux_toolchain_packages "$manager")');
    for (const call of calls) expect({ call: call.trim(), guarded: call.includes("|| log ") }).toEqual({ call: call.trim(), guarded: true });
  });

  test.skipIf(process.platform === "win32")("lists the Linux toolchain packages of every supported package manager through bash (POSIX hosts)", () => {
    const source = readFileSync(join(repoRoot, fixture.unixBootstrap.path), "utf8"), packages = shellFunction(source, "linux_toolchain_packages");
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
    const unix = readFileSync(join(repoRoot, fixture.unixBootstrap.path), "utf8");
    expect(unix).toContain('"$REPO_ROOT/package.json"');expect(unix).toContain('/"engines"/ { engines = 1 }');
    const windows = readFileSync(join(repoRoot, fixture.windowsBootstrap.path), "utf8");
    expect(windows).toContain("$manifest.engines.node");
    expect(windows).not.toContain("OpenJS.NodeJS");
    expect(nativePins().assets.filter((row) => row.tool === "node").map((row) => row.version)).toEqual(Array(6).fill(pinned));
  });

  const nativePinsPath = (source: string, pattern: RegExp, separator: string) => join(repoRoot, ...source.match(pattern)![1]!.split(separator));
  const nativePins = () => {
    const path = nativePinsPath(readFileSync(join(repoRoot, fixture.unixBootstrap.path), "utf8"), /^PINS_FILE="\$REPO_ROOT\/(.*)"$/mu, "/");
    return { path, schema: JSON.parse(readFileSync(path, "utf8")), ...(JSON.parse(readFileSync(path, "utf8")).definitions.Pins.const as { assets: { tool: string; version: string; platform: string; arch: string; url: string; sha256: string }[] }) };
  };

  test("pins and checksums every installer the native bootstraps download, agreeing with the image, the launcher registry and rustup schema", () => {
    const pins = nativePins(), rules = fixture.nativePins;
    const validate = new Ajv({ allErrors: true, strict: true }).compile(pins.schema);
    expect(validate({ assets: pins.assets }), JSON.stringify(validate.errors)).toBe(true);
    const shell = readFileSync(join(repoRoot, fixture.unixBootstrap.path), "utf8"), windows = readFileSync(join(repoRoot, fixture.windowsBootstrap.path), "utf8");
    const registryPath = join(repoRoot, rules.launcherRegistry);
    expect(nativePinsPath(shell, /^LAUNCHER_REGISTRY="\$REPO_ROOT\/(.*)"$/mu, "/")).toBe(registryPath);
    expect(nativePinsPath(windows, /^\$script:PinsFile = Join-Path \$repoRoot "(.*)"$/mu, "\\")).toBe(pins.path);
    expect(shell).toContain('bash "$LAUNCHER_INSTALL" "$LAUNCHER_BIN"');
    expect(windows).not.toContain("BunPinsFile");
    const rowLines = readFileSync(pins.path, "utf8").split("\n").filter((line) => line.trim().startsWith('{"tool":'));
    expect(rowLines.length).toBe(pins.assets.length);
    for (const [index, line] of rowLines.entries()) {
      expect(line.trim(), line).toMatch(new RegExp(rules.rowShape, "u"));
      expect(JSON.parse(line.trim().replace(/,$/u, ""))).toEqual(pins.assets[index]!);
    }
    for (const row of pins.assets) expect({ url: row.url, host: rules.hosts.includes(new URL(row.url).hostname) }).toEqual({ url: row.url, host: true });
    for (const tool of rules.tools) {
      const rows = pins.assets.filter((row) => row.tool === tool);
      expect(rows.map((row) => `${row.platform}-${row.arch}`).sort()).toEqual(rules.platforms.flatMap((platform: string) => rules.architectures.map((arch: string) => `${platform}-${arch}`)).sort());
      expect(new Set(rows.map((row) => row.version)).size).toBe(1);
      for (const row of rows) expect(row.url).toContain(row.version);
    }
    expect(pins.assets.filter((row) => row.tool === "dotnet-install")).toHaveLength(1);
    const bun = JSON.parse(readFileSync(registryPath, "utf8")).runtimes.bun as { version: string; download: { base: string }; distributions: { platform: string; arch: string; libc: string; archive: string; sha256: string }[] };
    expect(bun.version).toBe((JSON.parse(readFileSync(join(repoRoot, "package.json"), "utf8")) as { packageManager: string }).packageManager.replace("bun@", ""));
    const image = readFileSync(join(repoRoot, fixture.nodePin.dockerfile), "utf8");
    const imageNode = [...image.matchAll(/node_archive="([^"]+)"; node_sha="([a-f0-9]{64})"/gu)].map((match) => [match[1], match[2]]);
    expect(imageNode).toEqual(["x64", "arm64"].map((arch) => [arch, pins.assets.find((row) => row.tool === "node" && row.platform === "linux" && row.arch === arch)!.sha256]));
    const imageBun = [...image.matchAll(/bun_archive="([^"]+)"; bun_sha="([a-f0-9]{64})"/gu)].map((match) => [match[1], match[2]]);
    expect(imageBun).toEqual([]);
    expect(bun.download.base).toContain(bun.version);
    for (const row of bun.distributions) {
      expect(row.sha256).toMatch(/^[a-f0-9]{64}$/u);
      expect(row.archive).toMatch(/^bun-.+\.zip$/u);
    }
    const rustup = JSON.parse(readFileSync(join(libraryRoot, "⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/🧬️schema/🔣️.json"), "utf8")).definitions.Protocol.const.bootstrapVersion as string;
    expect(pins.assets.find((row) => row.tool === "rustup")!.version).toBe(rustup);
    const devcontainer = Bun.JSONC.parse(readFileSync(join(repoRoot, ".devcontainer/devcontainer.json"), "utf8")) as { features: Record<string, { version?: string }> };
    expect(pins.assets.find((row) => row.tool === "uv")!.version).toBe(devcontainer.features["ghcr.io/jsburckhardt/devcontainer-features/uv:1"]!.version!);
    const goVersion = pins.assets.find((row) => row.tool === "go")!.version, goFeature = devcontainer.features["ghcr.io/devcontainers/features/go:1"]!.version!;
    expect(goVersion.startsWith(`${goFeature}.`)).toBe(true);
    expect(Bun.semver.order(goVersion, readFileSync(join(repoRoot, "go.work"), "utf8").match(/^go (\d+\.\d+(?:\.\d+)?)$/mu)![1]! + (/^go \d+\.\d+$/mu.test(readFileSync(join(repoRoot, "go.work"), "utf8")) ? ".0" : ""))).toBeGreaterThanOrEqual(0);
    for (const fragment of rules.forbiddenInstallers) {
      expect({ fragment, shell: shell.includes(fragment) }).toEqual({ fragment, shell: false });
      expect({ fragment, windows: windows.includes(fragment) }).toEqual({ fragment, windows: false });
    }
    const curlLines = shell.split("\n").filter((line) => /^\s*curl\s/u.test(line));
    expect(curlLines).toHaveLength(1);
    expect(shellFunction(shell, "download_verified")).toContain(curlLines[0]!.trim());
    expect(shellFunction(shell, "download_verified")).toContain("curl --proto '=https' --tlsv1.2");
    expect(shellFunction(shell, "download_verified")).toContain("verify_sha256");
    expect(windows.match(/Invoke-WebRequest/gu)).toHaveLength(1);
    expect(windows.match(/function Save-VerifiedDownload \{[\s\S]*?Invoke-WebRequest[\s\S]*?Get-FileHash[\s\S]*?Checksum mismatch[\s\S]*?\n\}/u)).not.toBeNull();
  });

  test.skipIf(process.platform==="win32")("selects pinned distributions in the shell exactly as the schemas declare them", () => {
    const pins = nativePins(), source = readFileSync(join(repoRoot, fixture.unixBootstrap.path), "utf8");
    const functions = ["pin_row", "pin_field", "version_at_least"].map((name) => shellFunction(source, name)).join("\n");
    const script = (body: string) => spawnSync("bash", ["-c", `set -euo pipefail\nPINS_FILE="$1"\n${functions}\n${body}`, "law", pins.path], { encoding: "utf8" });
    for (const row of pins.assets.filter((candidate) => candidate.platform !== "any")) {
      const result = script(`row="$(pin_row ${row.tool} ${row.platform} ${row.arch})"; printf '%s|%s|%s' "$(pin_field "$row" version)" "$(pin_field "$row" url)" "$(pin_field "$row" sha256)"`);
      expect({ tool: row.tool, status: result.status, stdout: result.stdout }).toEqual({ tool: row.tool, status: 0, stdout: `${row.version}|${row.url}|${row.sha256}` });
    }
    expect(script("pin_row node freebsd x64").stdout).toBe("");
    for (const [installed, pinned, expected] of [["1.26.9", "1.26.9", true], ["1.26.10", "1.26.9", true], ["1.25.7", "1.26.9", false], ["2.0", "1.26.9", true]] as const) {
      expect({ installed, status: script(`version_at_least ${installed} ${pinned}`).status === 0 }).toEqual({ installed, status: expected });
      expect(Bun.semver.order(installed.split(".").length === 2 ? `${installed}.0` : installed, pinned) >= 0).toBe(expected);
    }
  });

  test("applies the long-path, Playwright cache and process-safety contracts in the Windows bootstrap", async () => {
    const windows = readFileSync(join(repoRoot, fixture.windowsBootstrap.path), "utf8");
    for (const [key, value] of fixture.repoLocalGitConfig) expect(windows).toContain(`@("${key}", "${value}")`);
    expect(windows).toContain("config --local $setting[0] $setting[1]");
    expect(windows).not.toContain("this script does not self-elevate");
    expect(windows).not.toContain("Mount-DiskImage");
    expect(windows).not.toContain("SeManageVolumePrivilege");
    expect(windows).toContain(fixture.nativePins.playwrightCache);
    expect(windows).not.toContain("node_modules\\.cache\\ms-playwright");
    expect(windows.split("\n").filter((line) => /Stop-Process/u.test(line) && !line.trim().startsWith("#"))).toEqual(["            Stop-Process -Id $process.ProcessId -ErrorAction Stop"]);
    expect(windows).toContain("-Consent:$StopRepoPython");
    expect(windows).toContain('Write-Step ("[readiness] LongPathsEnabled=');
    const { parseWindowsLongPathsEnabled } = await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts"));
    for (const row of fixture.windowsLongPaths.cases as { output: string; expected: boolean | null }[]) {
      const value = row.output.split(/\r?\n/u).find((line) => line.includes("LongPathsEnabled"))?.trim().split(/\s+/u).at(-1);
      const oracle = value === undefined ? null : Number.parseInt(value, 16) === 1;
      expect(parseWindowsLongPathsEnabled(row.output) ?? null).toBe(oracle);
      expect(oracle).toBe(row.expected);
    }
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
