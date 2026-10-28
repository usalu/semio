import { spawnSync } from "node:child_process";
import { accessSync, constants, statSync } from "node:fs";
import { homedir } from "node:os";
import { posix, win32 } from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const protocol = schema.definitions.Protocol.const;
type PathRowV1 = Readonly<{ kind: "path"; programs: readonly string[] }>;
type RustRowV1 = Readonly<{ kind: "rust"; programs: readonly string[]; environment: Readonly<{ compiler: string }>; directories: readonly Readonly<{ environment: string; homeSegments: readonly string[]; segments: readonly string[] }>[] }>;
type DeveloperRowV1 = Readonly<{ kind: "developer-directory"; environment: string; program: string; arguments: readonly string[]; timeoutSeconds: number }>;
type VisualRowV1 = Readonly<{ kind: "visual-studio"; linker: string; installation: Readonly<{ environment: string }>; locator: Readonly<{ environments: readonly string[]; segments: readonly string[]; fallback: string }>; arguments: readonly string[]; components: Readonly<Record<string, string>>; toolsSegments: readonly string[]; timeoutSeconds: number }>;
type ToolRowV1 = PathRowV1 | RustRowV1 | DeveloperRowV1 | VisualRowV1;
const tools = protocol.tools as unknown as Readonly<Record<string, ToolRowV1>>;

/** 🖥️ The facts about the machine the probes read: the platform, the processor architecture, the environment and the home directory. */
export type NativeToolchainHostV1 = Readonly<{ platform: string; arch: string; env: Readonly<Record<string, string | undefined>>; home: string }>;
/** 📟️ What one probe program printed and how it ended; `undefined` when it could not be started. */
export type NativeToolchainRunV1 = Readonly<{ status: number | null; stdout: string }>;
/** 🔌️ Every side effect of the probes: file lookups and the one short program start; tests replace it by a table so every platform runs on any host. */
export type NativeToolchainPortsV1 = Readonly<{
  isExecutable(path: string): boolean;
  isDirectory(path: string): boolean;
  run(program: string, arguments_: readonly string[], timeoutMilliseconds: number): NativeToolchainRunV1 | undefined;
}>;
/** 🔬️ The verdict on one prerequisite of the native toolchain and where it was found. */
export type NativeToolProbeV1 = Readonly<{ id: string; found: boolean; location: string | null; how: "environment" | "path" | "cargo-home" | "developer-directory" | "installer" | "none" }>;

const found = (id: string, location: string, how: NativeToolProbeV1["how"]): NativeToolProbeV1 => Object.freeze({ id, found: true, location, how });
const absent = (id: string): NativeToolProbeV1 => Object.freeze({ id, found: false, location: null, how: "none" });
const flavor = (platform: string): typeof posix => (platform === "win32" ? win32 : posix) as typeof posix;
const variable = (host: NativeToolchainHostV1, name: string): string | undefined => {
  const key = host.platform === "win32" ? Object.keys(host.env).find(candidate => candidate.toLowerCase() === name.toLowerCase()) ?? name : name;
  const value = host.env[key];
  return value === undefined || value === "" ? undefined : value;
};
const programFile = (host: NativeToolchainHostV1, program: string): string => (host.platform === "win32" && !/\.[A-Za-z0-9]+$/u.test(program) ? `${program}${protocol.pathExtensions[0]}` : program);

function searchDirectories(host: NativeToolchainHostV1): string[] {
  const separator = host.platform === "win32" ? ";" : ":";
  return (variable(host, "PATH") ?? "").split(separator).filter(entry => entry !== "");
}

function firstProgram(host: NativeToolchainHostV1, ports: NativeToolchainPortsV1, directories: readonly string[], programs: readonly string[]): string | undefined {
  const path = flavor(host.platform);
  for (const directory of directories) for (const program of programs) { const candidate = path.join(directory, programFile(host, program)); if (ports.isExecutable(candidate)) return candidate; }
  return undefined;
}

function probeRust(id: string, row: RustRowV1, host: NativeToolchainHostV1, ports: NativeToolchainPortsV1): NativeToolProbeV1 {
  const named = variable(host, row.environment.compiler);
  if (named !== undefined) return ports.isExecutable(named) ? found(id, named, "environment") : absent(id);
  const path = flavor(host.platform), onPath = firstProgram(host, ports, searchDirectories(host), row.programs);
  if (onPath !== undefined) return found(id, onPath, "path");
  const directories = row.directories.map(entry => { const base = variable(host, entry.environment); return path.join(base ?? path.join(host.home, ...entry.homeSegments), ...entry.segments); });
  const home = firstProgram(host, ports, directories, row.programs);
  return home === undefined ? absent(id) : found(id, home, "cargo-home");
}

function probeDeveloperDirectory(id: string, row: DeveloperRowV1, host: NativeToolchainHostV1, ports: NativeToolchainPortsV1): NativeToolProbeV1 {
  const named = variable(host, row.environment);
  if (named !== undefined && ports.isDirectory(named)) return found(id, named, "environment");
  const run = ports.run(row.program, row.arguments, row.timeoutSeconds * 1000), printed = run?.stdout.split(/\r?\n/u)[0]?.trim() ?? "";
  return run?.status === 0 && printed !== "" && ports.isDirectory(printed) ? found(id, printed, "developer-directory") : absent(id);
}

function probeVisualStudio(id: string, row: VisualRowV1, host: NativeToolchainHostV1, ports: NativeToolchainPortsV1): NativeToolProbeV1 {
  const path = flavor(host.platform), toolsIn = (installation: string): boolean => ports.isDirectory(path.join(installation, ...row.toolsSegments));
  const linker = firstProgram(host, ports, searchDirectories(host), [row.linker]);
  if (linker !== undefined) return found(id, linker, "path");
  const named = variable(host, row.installation.environment);
  if (named !== undefined && toolsIn(named)) return found(id, named, "environment");
  const component = row.components[host.arch];
  if (component === undefined) return absent(id);
  const directories = row.locator.environments.map(name => variable(host, name)).filter((value): value is string => value !== undefined).map(base => path.join(base, ...row.locator.segments.slice(0, -1)));
  const locator = firstProgram(host, ports, directories, [row.locator.segments.at(-1)!]) ?? firstProgram(host, ports, searchDirectories(host), [row.locator.fallback]);
  if (locator === undefined) return absent(id);
  const run = ports.run(locator, row.arguments.map(argument => argument.replace("{component}", component)), row.timeoutSeconds * 1000), installation = run?.stdout.split(/\r?\n/u).map(line => line.trim()).find(line => line !== "");
  return run?.status === 0 && installation !== undefined && toolsIn(installation) ? found(id, installation, "installer") : absent(id);
}

/** 🆔️ Lists the prerequisite ids a probe exists for. */
export function nativeToolchainToolIdsV1(): readonly string[] {
  return Object.freeze(Object.keys(tools));
}

/** 📋️ Lists what the native toolchain needs on one platform (a Rust compiler and the platform's C linker), in the order the remedies name them; an unknown platform needs nothing declared. */
export function nativeToolchainRequirementsV1(platform: string): readonly string[] {
  return Object.freeze([...((protocol.requirements as Readonly<Record<string, readonly string[]>>)[platform] ?? [])]);
}

/** 🔭️ Probes one prerequisite of the native toolchain without starting more than one short program; `undefined` for an id without a probe, so the caller may fall back to a plain `PATH` search. See https://github.com/microsoft/vswhere */
export function probeNativeToolV1(id: string, host: NativeToolchainHostV1, ports: NativeToolchainPortsV1): NativeToolProbeV1 | undefined {
  const row = tools[id];
  if (row === undefined) return undefined;
  if (row.kind === "path") { const location = firstProgram(host, ports, searchDirectories(host), row.programs); return location === undefined ? absent(id) : found(id, location, "path"); }
  if (row.kind === "rust") return probeRust(id, row, host, ports);
  if (row.kind === "developer-directory") return probeDeveloperDirectory(id, row, host, ports);
  return probeVisualStudio(id, row, host, ports);
}

/** 🧪️ Probes every prerequisite the platform declares, so a caller sees all that is missing at once. */
export function probeNativeToolchainV1(host: NativeToolchainHostV1, ports: NativeToolchainPortsV1): readonly NativeToolProbeV1[] {
  return Object.freeze(nativeToolchainRequirementsV1(host.platform).map(id => probeNativeToolV1(id, host, ports)!));
}

/** 🌍️ Describes the running machine. */
export function nativeToolchainHostV1(environment: NodeJS.ProcessEnv = process.env): NativeToolchainHostV1 {
  return Object.freeze({ platform: process.platform, arch: process.arch, env: environment, home: homedir() });
}

/** 🏭️ Binds the probes to the real file system and one bounded child process (no window, no input, standard output only, a timeout that ends it). */
export function defaultNativeToolchainPortsV1(): NativeToolchainPortsV1 {
  return Object.freeze({
    isExecutable: path => { try { if (!statSync(path).isFile()) return false; if (process.platform !== "win32") accessSync(path, constants.X_OK); return true; } catch { return false; } },
    isDirectory: path => { try { return statSync(path).isDirectory(); } catch { return false; } },
    run: (program, arguments_, timeoutMilliseconds) => {
      const run = spawnSync(program, [...arguments_], { encoding: "utf8", timeout: timeoutMilliseconds, windowsHide: true, stdio: ["ignore", "pipe", "ignore"] });
      return run.error === undefined ? { status: run.status, stdout: run.stdout ?? "" } : undefined;
    },
  });
}
