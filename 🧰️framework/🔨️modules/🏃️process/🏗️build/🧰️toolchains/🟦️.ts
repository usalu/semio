import {createHash} from "node:crypto";
import {existsSync, readFileSync} from "node:fs";
import {delimiter, dirname, join} from "node:path";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {resolveLanguageV1} from "../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type: "json"};
import document from "./🔣️.json" with {type: "json"};

if (validateJsonSchemaSubset(schema, document, schema).length || document.python.tool.version !== document.python.uv || Object.values(document.python.interpreters).some(interpreter => !(interpreter.series in document.python.cpython.series))) throw new Error("Closed native toolchain policy required / Geschlossene native Toolchain-Richtlinie erforderlich");

/** 🧾️ The authored policy: pins, layouts, projects, archives and texts of every native toolchain. */
export const NATIVE_TOOLCHAINS_V1 = document;
/** 🖥️ One operating system and architecture the pins cover. */
export type ToolchainHostV1 = (typeof document.hosts)[number];
/** 🧰️ The toolchain classes keyed by identity. */
export type ToolchainKindV1 = "python" | "dotnet" | "cmake" | "go";
/** 🔨️ The pinned archive tools provisioned below the tools partition. */
export type NativeToolV1 = "cmake" | "ninja" | "go" | "uv" | CargoToolV1;
/** 🦀️ Cargo archive tool names are owned by the closed pin document. */
export type CargoToolV1 = keyof typeof document.cargo;
/** 🧲️ The pinned Cargo tools in their authored provisioning order. */
export const CARGO_TOOL_IDS_V1: readonly CargoToolV1[] = Object.freeze(Object.keys(document.cargo) as CargoToolV1[]);
/** 🧲️ Every pinned archive tool, in the order the repository provisions them. */
export const NATIVE_TOOL_IDS_V1: readonly NativeToolV1[] = Object.freeze(["go", "uv", "cmake", "ninja", ...CARGO_TOOL_IDS_V1] as const);
/** 📦️ One pinned archive of one tool for one host. */
export type NativeDistributionV1 = (typeof document.cmake.cmake.distributions)[number];
/** 🐍️ One uv project of the policy. */
export type PythonProjectV1 = (typeof document.python.projects)[number];
/** 🗂️ The partition roots a plan lays its directories below; the caller owns their meaning and budget. */
export type ToolchainRootsV1 = Readonly<{dependencies: string; tools: string}>;
/** 🧮️ What distinguishes one Python environment: the project, the host and the digests of the manifest and the lock. */
export type PythonEnvironmentInputV1 = Readonly<{project: string; host: ToolchainHostV1; roots: ToolchainRootsV1; manifestSha256: string; lockSha256: string}>;
/** 📋️ A planned environment: where it lives, which interpreter it runs on, the variables and the arguments every uv call takes. */
export type PythonEnvironmentV1 = Readonly<{
  project: string;
  root: string;
  series: string;
  groups: readonly string[];
  key: string;
  directory: string;
  interpreters: string;
  cache: string;
  variables: Readonly<Record<string, string>>;
  syncArguments: readonly string[];
  runArguments: readonly string[];
}>;
/** 🔐️ A toolchain identity: the closed parts that were digested and the digest. */
export type ToolchainIdentityV1 = Readonly<{kind: ToolchainKindV1; host: ToolchainHostV1; pins: Readonly<Record<string, unknown>>; environment: Readonly<Record<string, string>>; digest: string}>;

const hash = (value: string | Uint8Array): string => createHash("sha256").update(value).digest("hex");
const hosts = new Set<string>(document.hosts);

/** 🔤️ Writes a value as canonical JSON: sorted keys, no whitespace, no escaping of non-ASCII text. */
export function canonicalToolchainJsonV1(value: unknown): string {
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  if (Array.isArray(value)) return "[" + value.map(canonicalToolchainJsonV1).join(",") + "]";
  return "{" + Object.keys(value).sort().map(key => JSON.stringify(key) + ":" + canonicalToolchainJsonV1((value as Record<string, unknown>)[key])).join(",") + "}";
}

/** 🔧️ Names the host a plan or an identity is for: `darwin-arm64`, `linux-x64`, `win32-x64`. */
export function toolchainHostV1(platform: string = process.platform, architecture: string = process.arch): ToolchainHostV1 {
  const host = platform + "-" + architecture;
  if (!hosts.has(host)) throw new Error(toolchainMessageV1("host", {host}, {}));
  return host as ToolchainHostV1;
}

/** 🗣️ Renders one policy text in the resolved language; without a resolvable language it renders every language, so no language is ever assumed. */
export function toolchainMessageV1(key: keyof typeof document.messages.texts, values: Readonly<Record<string, string | number>>, environment: Readonly<Record<string, string | undefined>> = process.env, args: readonly string[] = []): string {
  const text = document.messages.texts[key] as Readonly<Record<string, string>>, fill = (template: string) => template.replace(/\{([a-z]+)\}/gu, (_all, name: string) => String(values[name] ?? "{" + name + "}"));
  try {
    return fill(text[resolveLanguageV1(document.messages.languages, args, environment, () => []).language]!);
  } catch {
    return document.messages.languages.map(language => fill(text[language]!)).join(" / ");
  }
}

/** 🏠️ The dependencies and tools partition roots of one checkout. */
export function toolchainRootsV1(workspace: string): ToolchainRootsV1 {
  return Object.freeze({dependencies: join(workspace, ...document.roots.dependencies.split("/")), tools: join(workspace, ...document.roots.tools.split("/"))});
}

/** 📁️ The directory the managed interpreters of the repository live in, below a tools partition root. */
export function pythonInterpreterDirectoryV1(toolsRoot: string): string {
  return join(toolsRoot, ...document.python.layout.interpreters.split("/"));
}

/** 🐍️ One pinned CPython build: its full version, the release it comes from and the archive of one host. */
export type PythonInterpreterV1 = Readonly<{series: string; version: string; release: string; versionArguments: readonly string[]; distribution: (typeof document.python.cpython.series)["3.12"]["distributions"][number]}>;

/** 🐍️ Finds the pinned CPython build of one interpreter series for one host. */
export function pythonInterpreterV1(series: string, host: ToolchainHostV1): PythonInterpreterV1 {
  const entry = (document.python.cpython.series as Readonly<Record<string, (typeof document.python.cpython.series)["3.12"]>>)[series], distribution = entry?.distributions.find(row => row.host === host);
  if (entry === undefined || distribution === undefined) throw new Error(toolchainMessageV1("python", {series}));
  return Object.freeze({series, version: entry.version, release: document.python.cpython.release, versionArguments: document.python.cpython.versionArguments, distribution});
}

/** 🔗️ The download address of the pinned CPython archive; the `+` of the release build is percent-encoded. */
export function pythonInterpreterUrlV1(series: string, host: ToolchainHostV1): string {
  return pythonInterpreterV1(series, host).release + "/" + pythonInterpreterV1(series, host).distribution.archive.replaceAll("+", "%2B");
}

/** 🏠️ Where one pinned CPython lives: the directory uv recognises as a managed installation, below the interpreter directory of the tools partition. */
export function pythonInterpreterHomeV1(series: string, host: ToolchainHostV1, toolsRoot: string): string {
  return join(pythonInterpreterDirectoryV1(toolsRoot), pythonInterpreterV1(series, host).distribution.directory);
}

/** ▶️ The executable of one pinned CPython. */
export function pythonInterpreterBinaryV1(series: string, host: ToolchainHostV1, toolsRoot: string): string {
  const row = pythonInterpreterV1(series, host).distribution, home = pythonInterpreterHomeV1(series, host, toolsRoot);
  return join(row.binaryDirectory === "." ? home : join(home, ...row.binaryDirectory.split("/")), row.binaries[0]!);
}

/** 🪪️ Names the intended installation independently of any installation state: `cpython:3.12.13:darwin-arm64:<sha256>`. */
export function pythonInterpreterIdentityV1(series: string, host: ToolchainHostV1): string {
  const pin = pythonInterpreterV1(series, host);
  return "cpython:" + pin.version + ":" + host + ":" + pin.distribution.sha256;
}

/** 🔎️ Finds one Python project by its id. */
export function pythonProjectV1(id: string): PythonProjectV1 {
  const row = document.python.projects.find(project => project.id === id);
  if (!row) throw new Error(toolchainMessageV1("project", {project: id}));
  return row;
}

/** 🌳️ Finds the Python project whose root is a repository-relative path; `.` and the empty path name the workspace. */
export function pythonProjectForRootV1(root: string): PythonProjectV1 | undefined {
  const normal = root.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/+$/u, "") || ".";
  return document.python.projects.find(project => project.root === normal);
}

/** ▶️ The interpreter series a project runs on. */
export function pythonSeriesV1(project: string): string {
  const row = pythonProjectV1(project), interpreter = (document.python.interpreters as Readonly<Record<string, {series: string}>>)[row.interpreter];
  if (!interpreter) throw new Error(toolchainMessageV1("project", {project: row.interpreter}));
  return interpreter.series;
}

/** ➡️ Plans the isolated environment of one project: below the dependencies root, named by everything that distinguishes it, so two agents with the same lock share one environment and a changed lock gets a new one instead of mutating a live one. */
export function pythonEnvironmentV1(input: PythonEnvironmentInputV1): PythonEnvironmentV1 {
  const row = pythonProjectV1(input.project), series = pythonSeriesV1(input.project), layout = document.python.layout;
  if (!/^[a-f0-9]{64}$/u.test(input.manifestSha256) || !/^[a-f0-9]{64}$/u.test(input.lockSha256)) throw new Error(toolchainMessageV1("digests", {}));
  const key = row.id + "-" + hash(canonicalToolchainJsonV1({project: row.id, uv: document.python.uv, series, host: input.host, groups: row.groups, manifest: input.manifestSha256, lock: input.lockSha256})).slice(0, 16);
  const directory = join(input.roots.dependencies, ...layout.environments.split("/"), key), interpreters = join(input.roots.tools, ...layout.interpreters.split("/")), cache = join(input.roots.dependencies, ...layout.cache.split("/"));
  const variables = Object.freeze({...document.python.variables, UV_PROJECT_ENVIRONMENT: directory, UV_PYTHON_INSTALL_DIR: interpreters, UV_PYTHON: series, UV_CACHE_DIR: cache});
  const groups = row.groups.flatMap(group => ["--group", group]);
  return Object.freeze({
    project: row.id,
    root: row.root,
    series,
    groups: Object.freeze([...row.groups]),
    key,
    directory,
    interpreters,
    cache,
    variables,
    syncArguments: Object.freeze(["sync", "--locked", "--project", row.root, ...groups]),
    runArguments: Object.freeze(["run", "--locked", "--no-sync", "--project", row.root]),
  });
}

/** 🦑️ Plans the environment of one Python project of the repository from the digests of its manifest and lock as they are on disk; a project without a lock has no environment. */
export function pythonEnvironmentOfProjectV1(workspace: string, project: string, roots: ToolchainRootsV1 = toolchainRootsV1(workspace), host: ToolchainHostV1 = toolchainHostV1()): PythonEnvironmentV1 {
  const row = pythonProjectV1(project), read = (name: string): string => {
    try {
      return hash(readFileSync(join(workspace, row.root, name)));
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code === "ENOENT") throw new Error(toolchainMessageV1("file", {file: join(row.root, name), project}));
      throw error;
    }
  };
  return pythonEnvironmentV1({project, host, roots, manifestSha256: read("pyproject.toml"), lockSha256: read("uv.lock")});
}

/** 🐍️ The interpreter inside a planned Python environment, `Scripts/python.exe` on Windows and `bin/python` elsewhere; the environment's digest key makes it follow every manifest or lock change, so a stale environment is never reused. */
export function pythonEnvironmentInterpreterV1(plan: Pick<PythonEnvironmentV1, "directory">, platform: NodeJS.Platform = process.platform): string {
  return platform === "win32" ? join(plan.directory, "Scripts", "python.exe") : join(plan.directory, "bin", "python");
}

/** 🧭️ The provisioned interpreter of the repository's own Python environment, found from `workspace` or its nearest ancestor holding `pyproject.toml` and `uv.lock` (a Cargo sub-workspace resolves to the checkout), or `null` while that environment is not synchronized yet (`repo run workspace:deps-python`). */
export function provisionedPythonV1(workspace: string, project = "workspace"): string | null {
  for (let root = workspace; ; root = dirname(root)) {
    if (existsSync(join(root, "pyproject.toml")) && existsSync(join(root, "uv.lock"))) {
      try {
        const interpreter = pythonEnvironmentInterpreterV1(pythonEnvironmentOfProjectV1(root, project));
        return existsSync(interpreter) ? interpreter : null;
      } catch {
        return null;
      }
    }
    if (dirname(root) === root) return null;
  }
}

/** 🧷️ Adds `SEMIO_PYTHON` to a child environment: the explicit value wins, then the provisioned environment interpreter; with neither, the environment is returned unchanged. The one resolver every test and command spawn goes through. */
export function withProvisionedPythonV1<T extends Readonly<Record<string, string | undefined>>>(environment: T, workspace: string): T | (T & {SEMIO_PYTHON: string}) {
  if (environment.SEMIO_PYTHON) return environment;
  const interpreter = provisionedPythonV1(workspace);
  return interpreter ? {...environment, SEMIO_PYTHON: interpreter} : environment;
}

/** 🔣️ Renders the repository `global.json` that pins the .NET SDK. */
export function dotnetGlobalJsonV1(): string {
  return JSON.stringify({sdk: {version: document.dotnet.sdk, rollForward: document.dotnet.rollForward}}, null, 2) + "\n";
}

/** 📂️ Renders the repository `Directory.Build.props`: every MSBuild run, also a raw `dotnet build`, writes `obj/` and `bin/` below one artifacts path outside the source packages; the repository scripts pass their own session path as `--artifacts-path`, which wins. */
export function dotnetDirectoryBuildPropsV1(): string {
  return [
    "<Project>",
    "  <PropertyGroup>",
    "    <UseArtifactsOutput>true</UseArtifactsOutput>",
    "    <ArtifactsPath Condition=\"'$(ArtifactsPath)' == ''\">$(MSBuildThisFileDirectory)" + document.dotnet.artifacts + "/$(MSBuildProjectName)</ArtifactsPath>",
    "  </PropertyGroup>",
    "</Project>",
    "",
  ].join("\n");
}

/** 📕️ The pinned release of one tool: version, release address, version arguments and archives. */
export function nativeToolSpecV1(tool: NativeToolV1): typeof document.cmake.cmake {
  return tool === "go" ? document.go.tool : tool === "uv" ? document.python.tool : tool === "cargo-nextest" || tool === "cargo-llvm-cov" ? document.cargo[tool] : document.cmake[tool];
}

/** 💻️ Selects the pinned archive of one tool for one host. */
export function nativeDistributionV1(tool: NativeToolV1, host: ToolchainHostV1): NativeDistributionV1 {
  const row = nativeToolSpecV1(tool).distributions.find(candidate => candidate.host === host);
  if (!row) throw new Error(toolchainMessageV1("host", {host}, {}));
  return row;
}

/** 🔗️ The download address of the pinned archive. */
export function nativeToolUrlV1(tool: NativeToolV1, host: ToolchainHostV1): string {
  return nativeToolSpecV1(tool).release + "/" + nativeDistributionV1(tool, host).archive;
}

/** 🛣️ Where one pinned tool lives below the tools partition: named by version, host and the first 16 digits of the archive digest, so a different pin never reuses a directory. */
export function nativeToolDirectoryV1(tool: NativeToolV1, host: ToolchainHostV1, toolsRoot: string): string {
  return join(toolsRoot, "native", tool, nativeToolSpecV1(tool).version, host + "-" + nativeDistributionV1(tool, host).sha256.slice(0, 16));
}

/** 🛠️ The directory that holds the executables of one installed pinned tool. */
export function nativeToolBinaryDirectoryV1(tool: NativeToolV1, host: ToolchainHostV1, toolsRoot: string): string {
  const row = nativeDistributionV1(tool, host), base = nativeToolDirectoryV1(tool, host, toolsRoot);
  return row.binaryDirectory === "." ? base : join(base, ...row.binaryDirectory.split("/"));
}

/** 🧭️ The `PATH` entries that make the pinned tools the only ones a build sees first; CMake and Ninja unless others are named. */
export function nativeToolPathV1(host: ToolchainHostV1, toolsRoot: string, tools: readonly NativeToolV1[] = ["cmake", "ninja"]): readonly string[] {
  return Object.freeze(tools.map(tool => nativeToolBinaryDirectoryV1(tool, host, toolsRoot)));
}

/** 📌️ Puts the `bin` directory of every pinned tool that is provisioned in this checkout first on the `PATH` of an environment, under the spelling of the variable that environment already uses; a tool that is not provisioned adds nothing, so the call never installs and never fails. */
export function provisionedToolPathV1(environment: Readonly<Record<string, string | undefined>>, workspace: string, tools: readonly NativeToolV1[] = ["go", "uv", "cmake", "ninja"], host: ToolchainHostV1 = toolchainHostV1()): Readonly<Record<string, string | undefined>> {
  const key = Object.keys(environment).find(name => name.toLowerCase() === "path") ?? "PATH", root = toolchainRootsV1(workspace).tools;
  const directories = tools.map(tool => nativeToolBinaryDirectoryV1(tool, host, root)).filter(directory => existsSync(directory));
  return Object.freeze({...environment, [key]: [...directories, ...(environment[key] ?? "").split(delimiter).filter(Boolean)].join(delimiter)});
}

/** 🪪️ Names the intended installation independently of any installation state: `cmake:4.4.4:darwin-arm64:<sha256>`. */
export function nativeToolIdentityV1(tool: NativeToolV1, host: ToolchainHostV1): string {
  return tool + ":" + nativeToolSpecV1(tool).version + ":" + host + ":" + nativeDistributionV1(tool, host).sha256;
}

/** 🔑️ Digests the identity of one toolchain class on one host: the pins and the declared variables that are set, never an installation path. */
export function toolchainIdentityV1(kind: ToolchainKindV1, input: Readonly<{host?: ToolchainHostV1; project?: string; environment?: Readonly<Record<string, string | undefined>>}> = {}): ToolchainIdentityV1 {
  const host = input.host ?? toolchainHostV1(), source = input.environment ?? {};
  const names: readonly string[] = document[kind].environment;
  const pins: Record<string, unknown> = kind === "python"
    ? {uv: document.python.uv, series: input.project === undefined ? [...new Set(document.python.projects.map(project => pythonSeriesV1(project.id)))].sort() : [pythonSeriesV1(input.project)]}
    : kind === "dotnet" ? {sdk: document.dotnet.sdk, rollForward: document.dotnet.rollForward}
    : kind === "cmake" ? Object.fromEntries((["cmake", "ninja"] as const).map(tool => [tool, {version: document.cmake[tool].version, sha256: nativeDistributionV1(tool, host).sha256}]))
    : {version: document.go.version, sha256: nativeDistributionV1("go", host).sha256};
  const environment = Object.fromEntries(names.flatMap(name => source[name] === undefined ? [] : [[name, source[name]!]]));
  const digest = hash(canonicalToolchainJsonV1({kind, host, pins, environment}));
  return Object.freeze({kind, host, pins: Object.freeze(pins), environment: Object.freeze(environment), digest});
}

/** 📖️ Reads the pinned Go version a `go.work` text demands, or undefined when it names none. */
export function goWorkVersionV1(text: string): string | undefined {
  return /^go (\d+\.\d+\.\d+)$/mu.exec(text)?.[1];
}

/** 🧪️ Decides whether a link of an extracted tree points outside the tree: an absolute target, a drive letter or a path that climbs above the root from the directory of the link. */
export function extractedLinkEscapesV1(path: string, target: string): boolean {
  if (/^(?:[A-Za-z]:|[\\/])/u.test(target)) return true;
  const stack = path.split(/[\\/]/u).slice(0, -1).filter(part => part !== "" && part !== ".");
  for (const part of target.split(/[\\/]/u)) {
    if (part === "" || part === ".") continue;
    if (part === "..") {
      if (stack.length === 0) return true;
      stack.pop();
    } else stack.push(part);
  }
  return false;
}
