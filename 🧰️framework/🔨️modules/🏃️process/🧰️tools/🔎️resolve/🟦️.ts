import { accessSync, constants, existsSync, readdirSync, realpathSync, statSync } from "node:fs";
import { homedir } from "node:os";
import { delimiter, dirname, isAbsolute, join, basename } from "node:path";
import { findToolV1, TOOL_REGISTRY_V1, type ToolEntryV1, type ToolLocatorV1, type ToolRegistryV1 } from "../📖️registry/🟦️.ts";
import { runtimeCommandNameV1 } from "../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import { readToolPinV1, systemToolPinPortV1, toolDigestForHostV1, type ToolPinPortV1, type ToolPinV1 } from "../📌️pin/🟦️.ts";
import { nativeCompilerArgumentsV1, selectNativeCompilerV1 } from "../🛠️native-compiler/🟦️.ts";

/** 🔌️ What resolving a tool touches on the host; tests replace it so every platform is testable on any host. */
export type ToolHostV1 = Readonly<{
  platform: NodeJS.Platform; architecture: string; environment: Readonly<Record<string, string | undefined>>; root: string; home: string; execPath: string; runtimeCommand: string;
  isExecutable(path: string): boolean; exists(path: string): boolean; list(directory: string): readonly string[]; pins: ToolPinPortV1;
  canonical?(path: string): string;
}>;

/** 🔎️ Where a tool was found and how to start it: the program, the arguments that precede the caller's, and the locator that found it. */
export type ToolResolutionV1 = Readonly<{ id: string; command: string | null; prefix: readonly string[]; suffix?: readonly string[]; source: ToolLocatorV1["kind"] | "missing"; tried: readonly string[]; pin: ToolPinV1 }>;

/** 🖥️ The real host of this process rooted at a repository checkout. */
export function systemToolHostV1(root: string, environment: Readonly<Record<string, string | undefined>> = process.env): ToolHostV1 {
  return Object.freeze({
    platform: process.platform, architecture: process.arch, environment, root, home: homedir(), execPath: process.execPath, runtimeCommand: runtimeCommandNameV1(),
    isExecutable: path => { try { if (!statSync(path).isFile()) return false; if (process.platform !== "win32") accessSync(path, constants.X_OK); return true; } catch { return false; } },
    exists: path => existsSync(path), list: directory => { try { return readdirSync(directory); } catch { return []; } }, canonical: path => realpathSync(path), pins: systemToolPinPortV1,
  });
}

const suffixes = (host: ToolHostV1): readonly string[] => host.platform === "win32" ? ["", ...(host.environment.PATHEXT ?? ".EXE;.CMD;.BAT").split(";").map(item => item.toLowerCase()).filter(Boolean)] : [""];
const expand = (directory: string, host: ToolHostV1): string => directory === "~" ? host.home : directory.startsWith("~/") ? join(host.home, directory.slice(2)) : directory;
const withSuffix = (path: string, host: ToolHostV1): string | undefined => suffixes(host).map(suffix => path + suffix).find(candidate => host.isExecutable(candidate));

const pathHit = (names: readonly string[], directories: readonly string[], host: ToolHostV1): string | undefined => {
  for (const directory of directories) for (const name of names) { const hit = withSuffix(join(directory, name), host); if (hit !== undefined) return hit; }
  return undefined;
};

const expandWildcard = (pattern: string, host: ToolHostV1): readonly string[] => {
  const star = pattern.indexOf("*");
  if (star < 0) return [pattern];
  const head = pattern.slice(0, star).replace(/[\\/][^\\/]*$/u, ""), tail = pattern.slice(head.length + 1).split(/[\\/]/u).slice(1).join("/");
  return host.list(head).map(name => join(head, name, tail));
};

const packageRoots = (registry: ToolRegistryV1, host: ToolHostV1): readonly string[] => registry.packageRoots.flatMap(root => expandWildcard(isAbsolute(root) ? root : join(host.root, root), host));

/** 🧱️ Finds one tool on a host by trying its locators in order; a missing tool answers `source: "missing"` with what was tried. A `command` that is not the first executable name of the tool (`rustc` of the Rust toolchain) is only looked up on the path. */
export function resolveToolV1(entry: ToolEntryV1, host: ToolHostV1, registry: ToolRegistryV1 = TOOL_REGISTRY_V1, depth = 0, command?: string): ToolResolutionV1 {
  const pin = readToolPinV1(entry, host.root, host.pins), tried: string[] = [];
  const found = (command: string, source: ToolLocatorV1["kind"], prefix: readonly string[] = [], suffix: readonly string[] = []): ToolResolutionV1 => Object.freeze({ id: entry.id, command, prefix, ...(suffix.length ? { suffix } : {}), source, tried, pin });
  const through = (via: string): ToolResolutionV1 | undefined => {
    const tool = findToolV1(via, registry);
    if (tool === undefined || depth > 4) return undefined;
    const inner = resolveToolV1(tool, host, registry, depth + 1);
    return inner.command === null ? undefined : inner;
  };
  if (!entry.platforms.includes(host.platform as never)) return Object.freeze({ id: entry.id, command: null, prefix: [], source: "missing", tried: [`platform ${host.platform}`], pin });
  const secondary = command !== undefined && command !== entry.commands[0];
  for (const locator of entry.locate) {
    if (secondary && locator.kind !== "path") continue;
    if (locator.kind === "environment") {
      const value = host.environment[locator.name] ?? host.environment[`SEMIO_TOOL_${entry.id.toUpperCase().replace(/-/gu, "_")}`];
      tried.push(locator.name);
      if (value !== undefined && value !== "" && host.exists(value)) return found(value, "environment");
    } else if (locator.kind === "native-compiler") {
      tried.push("installed native compiler SDK");
      const selected = selectNativeCompilerV1({ ...host, canonical: path => host.canonical?.(path) ?? path });
      if (selected) return found(selected.command, "native-compiler", [], nativeCompilerArgumentsV1(selected));
    } else if (locator.kind === "runtime") {
      tried.push("runtime");
      if (entry.commands.includes(basename(host.execPath).replace(/\.exe$/iu, "")) || entry.commands.includes(host.runtimeCommand)) return found(host.execPath, "runtime");
    } else if (locator.kind === "provisioned") {
      const partition = host.root + "/" + (registry.partitions[locator.partition] ?? locator.partition);
      const digest = toolDigestForHostV1(pin, host.platform, host.architecture, entry.pin.digests?.hostExtras ?? {});
      const segments = locator.segments.map(segment => segment.replace(/\{sha256(?::(\d+))?\}/gu, (_, length?: string) => digest === undefined ? "" : length ? digest.sha256.slice(0, Number(length)) : digest.sha256).replace("{version}", pin.version ?? "").replace("{host}", `${host.platform}-${host.architecture}`));
      tried.push(join(partition, ...segments, locator.executable));
      if (digest !== undefined) { const hit = withSuffix(join(partition, ...segments, locator.executable), host); if (hit !== undefined) return found(hit, "provisioned"); }
    } else if (locator.kind === "package-script") {
      const via = through(locator.via);
      for (const root of packageRoots(registry, host)) {
        const script = join(root, locator.package, locator.script);
        tried.push(script);
        if (via !== undefined && host.exists(script)) return found(via.command!, "package-script", [...via.prefix, script, ...locator.prefix ?? []]);
      }
    } else if (locator.kind === "script") {
      const via = through(locator.via), script = join(host.root, locator.path);
      tried.push(script);
      if (via !== undefined && host.exists(script)) return found(via.command!, "script", [...via.prefix, script, ...locator.prefix ?? []]);
    } else {
      const directories = [...(host.environment.PATH ?? host.environment.Path ?? "").split(delimiter).filter(Boolean), ...locator.directories.map(directory => expand(directory, host))];
      tried.push(`PATH ${entry.commands[0]}`);
      const hit = pathHit(secondary ? [command] : locator.names ?? [entry.commands[0]!], directories, host);
      if (hit !== undefined) return found(hit, "path");
    }
  }
  return Object.freeze({ id: entry.id, command: null, prefix: [], source: "missing", tried, pin });
}

/** 🏠️ Finds the repository root: the explicit root, the workspace variables, or the nearest ancestor that holds `MODULE.bazel`. */
export function toolRootV1(cwd: string = process.cwd(), environment: Readonly<Record<string, string | undefined>> = process.env): string {
  const named = environment.SEMIO_WORKSPACE_ROOT ?? environment.REPO_ROOT;
  if (named !== undefined && named !== "") return named;
  for (let directory = cwd; ; directory = dirname(directory)) {
    if (existsSync(join(directory, "MODULE.bazel"))) return directory;
    if (dirname(directory) === directory) return cwd;
  }
}
