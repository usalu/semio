import { accessSync, constants, existsSync, lstatSync, readFileSync, readdirSync, readlinkSync, realpathSync, statSync } from "node:fs";
import { createHash } from "node:crypto";
import { homedir } from "node:os";
import { delimiter, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const policy = Object.freeze({ ...schema.definitions.Protocol.const, architectures: Object.freeze({ ...schema.definitions.Protocol.const.architectures }), executedTools: Object.freeze([...schema.definitions.Protocol.const.executedTools]), explicitEnvironment: Object.freeze([...schema.definitions.Protocol.const.explicitEnvironment]) });

/** 🔌️ Installed compiler discovery reads only bounded SDK branches and executable identities. */
export interface NativeCompilerHostV1 {
  readonly platform: string;
  readonly architecture: string;
  readonly home: string;
  readonly environment: Readonly<Record<string, string | undefined>>;
  isExecutable(path: string): boolean;
  list(path: string): readonly string[];
  canonical(path: string): string;
  exists?(path: string): boolean;
}

/** 🛠️ An installed SDK compiler; selection alone conveys no compiler-store authority. */
export interface NativeCompilerSelectionV1 { readonly version: 1; readonly command: string; readonly root: string; readonly machine: "x86_64" | "aarch64"; readonly libraryDirectory?: string }

/** 🎯️ Effective target, host and explicit Cargo environment choices before default projection. */
export interface NativeCompilerContextV1 { readonly target: string; readonly host: string; readonly configuredLinker: boolean; readonly configuredHostLinker: boolean; readonly configuredEnvironment: readonly string[] }

/** 🔌️ Native source closure exposes only filesystem facts and bytes behind this testable interface. */
export interface NativeCompilerSourcePortV1 {
  inspect(path: string): Readonly<{ kind: "file" | "directory" | "symlink"; bytes: number }>;
  list(path: string): readonly string[];
  read(path: string): Uint8Array;
  target(path: string): string;
  canonical(path: string): string;
  dependencies?(contents: Uint8Array, path: string): readonly string[];
}

export const SYSTEM_NATIVE_COMPILER_SOURCE_PORT_V1 = Object.freeze<NativeCompilerSourcePortV1>({
  inspect: path => { const value = lstatSync(path); if (!value.isFile() && !value.isDirectory() && !value.isSymbolicLink()) throw Error("Regular native toolchain source required"); return { kind: value.isSymbolicLink() ? "symlink" : value.isDirectory() ? "directory" : "file", bytes: value.size }; },
  list: path => readdirSync(path), read: path => readFileSync(path), target: path => readlinkSync(path), canonical: path => realpathSync(path),
});

/** 🔏️ Hashes a bounded complete toolchain closure, including directory membership and symlink targets. */
export async function nativeCompilerSourceClosureV1(roots: readonly string[], context: Readonly<{ check(): void; yield(): Promise<void> }>, port: NativeCompilerSourcePortV1 = SYSTEM_NATIVE_COMPILER_SOURCE_PORT_V1): Promise<Readonly<{ version: 1; digest: string; files: number; bytes: number; roots: readonly string[] }>> {
  const selected = [...new Set(roots)].sort(), pending = [...selected].reverse(), seen = new Set<string>(), hash = createHash("sha256");
  let files = 0, directories = 0, bytes = 0, steps = 0, yieldedBytes = 0;
  while (pending.length) {
    context.check();
    if (++steps % 64 === 0 || bytes - yieldedBytes >= 8388608) { await context.yield(); yieldedBytes = bytes; }
    const path = pending.pop()!;
    if (seen.has(path)) continue;
    seen.add(path);
    const status = port.inspect(path);
    if (status.kind === "directory") {
      if (++directories > policy.maximumSourceDirectories) throw Error("Native compiler source directory bound exceeded");
      const names = [...port.list(path)].sort();
      if (names.length + pending.length > policy.maximumSourceFiles + policy.maximumSourceDirectories) throw Error("Native compiler source frontier bound exceeded");
      hash.update(JSON.stringify({ path, kind: status.kind, names }) + "\n");
      for (const name of names.reverse()) { if (name === "." || name === ".." || /[\\/]/u.test(name)) throw Error("Original native source member required"); pending.push(join(path, name)); }
    } else if (status.kind === "symlink") {
      if (++files > policy.maximumSourceFiles) throw Error("Native compiler source file bound exceeded");
      const target = port.target(path);
      let canonical: string | null;
      try { canonical = port.canonical(path); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; canonical = null; }
      hash.update(JSON.stringify({ path, kind: status.kind, target, canonical }) + "\n");
      if (canonical !== null) pending.push(canonical);
    } else {
      if (++files > policy.maximumSourceFiles || status.bytes > policy.maximumFileBytes || bytes + status.bytes > policy.maximumSourceBytes) throw Error("Native compiler source byte or file bound exceeded");
      const contents = port.read(path);
      if (contents.byteLength !== status.bytes) throw Error("Native compiler source extent changed");
      bytes += contents.byteLength;
      hash.update(JSON.stringify({ path, kind: status.kind, bytes: contents.byteLength, digest: createHash("sha256").update(contents).digest("hex") }) + "\n");
      pending.push(...port.dependencies?.(contents, path) ?? []);
    }
  }
  context.check();
  return Object.freeze({ version: 1, digest: hash.digest("hex"), files, bytes, roots: Object.freeze(selected) });
}

/** 🔗️ Resolves an ELF64 compiler's loader and declared shared libraries from bounded native search paths. */
export function nativeCompilerElfSourcesV1(contents: Uint8Array, path: string, search: readonly string[], isFile: (path: string) => boolean, defaults: readonly string[] = ["/usr/lib", "/lib"]): readonly string[] {
  const bytes = Buffer.from(contents.buffer, contents.byteOffset, contents.byteLength);
  if (bytes.length < 64 || bytes.readUInt32BE(0) !== 0x7f454c46 || bytes[4] !== 2 || bytes[5] !== 1 || ![2, 3].includes(bytes.readUInt16LE(16))) return [];
  const offset = Number(bytes.readBigUInt64LE(32)), width = bytes.readUInt16LE(54), count = bytes.readUInt16LE(56);
  if (width < 56 || count > 1024 || !Number.isSafeInteger(offset) || offset + width * count > bytes.length) throw Error("Bounded native ELF program headers required");
  const segments = Array.from({ length: count }, (_, index) => { const start = offset + index * width; return { type: bytes.readUInt32LE(start), offset: Number(bytes.readBigUInt64LE(start + 8)), address: Number(bytes.readBigUInt64LE(start + 16)), size: Number(bytes.readBigUInt64LE(start + 32)) }; });
  const text = (start: number, maximum: number): string => { if (start < 0 || maximum < 1 || start + maximum > bytes.length) throw Error("Bounded native ELF string required"); const end = bytes.indexOf(0, start); if (end < start || end >= start + maximum) throw Error("Terminated native ELF string required"); return bytes.toString("utf8", start, end); };
  const result: string[] = [], interpreter = segments.find(row => row.type === 3), dynamic = segments.find(row => row.type === 2);
  if (interpreter) result.push(text(interpreter.offset, interpreter.size));
  if (!dynamic) return result;
  if (dynamic.offset + dynamic.size > bytes.length || dynamic.size / 16 > 4096) throw Error("Bounded native ELF dynamic table required");
  const entries: { tag: number; value: number }[] = [];
  for (let position = dynamic.offset; position + 16 <= dynamic.offset + dynamic.size; position += 16) { const tag = Number(bytes.readBigInt64LE(position)); if (tag === 0) break; entries.push({ tag, value: Number(bytes.readBigUInt64LE(position + 8)) }); }
  const address = entries.find(row => row.tag === 5)?.value;
  if (address === undefined) return result;
  const load = segments.find(row => row.type === 1 && address >= row.address && address < row.address + row.size);
  if (!load) throw Error("Original native ELF string table required");
  const start = load.offset + address - load.address, size = entries.find(row => row.tag === 10)?.value ?? 0, string = (position: number) => text(start + position, Math.min(512, size - position));
  const runpath = entries.find(row => row.tag === 29), rpath = entries.find(row => row.tag === 15), expand = (value: string) => value.split(":").filter(Boolean).map(directory => directory.replace(/\$\{?ORIGIN\}?/gu, dirname(path)));
  const paths = [...(!runpath && rpath ? expand(string(rpath.value)) : []), ...search, ...(runpath ? expand(string(runpath.value)) : []), ...defaults];
  for (const row of entries.filter(row => row.tag === 1)) {
    const name = string(row.value);
    if (/[\\/]/u.test(name)) throw Error("Closed native ELF library name required");
    const dependency = paths.map(directory => join(directory, name)).find(isFile);
    if (!dependency) throw Error("Native compiler dependency unavailable: " + name);
    result.push(dependency);
  }
  return Object.freeze(result);
}

/** 📦️ Declares the complete SDK C toolchain, native startup/runtime and recursively resolved ELF dependencies. */
export function nativeCompilerSourcePlanV1(selection: NativeCompilerSelectionV1, environment: Readonly<Record<string, string | undefined>>, additional: readonly string[] = [], filesystem: NativeCompilerSourcePortV1 = SYSTEM_NATIVE_COMPILER_SOURCE_PORT_V1): Readonly<{ roots: readonly string[]; port: NativeCompilerSourcePortV1 }> {
  const library = selection.libraryDirectory ?? policy.libraryDirectory;
  const runtime = filesystem.list(library).filter(name => /^(?:[Srg]?crt[^/]*\.o|ld-linux[^/]*\.so(?:\.[0-9]+)*|lib(?:c|m|mvec|pthread|rt|dl|util|gcc_s)(?:_nonshared)?\.(?:a|so(?:\.[0-9]+)*))$/u.test(name)).map(name => join(library, name));
  const roots = [selection.command, join(selection.root, "include"), join(selection.root, "lib/gcc"), join(selection.root, "libexec/gcc"), ...policy.executedTools, ...runtime, fileURLToPath(import.meta.url), fileURLToPath(new URL("./🧬️schema/🔣️.json", import.meta.url)), ...additional];
  const defaults = [library, "/lib", join("/usr/lib", selection.machine + "-linux-gnu"), join("/lib", selection.machine + "-linux-gnu")];
  const port = { ...filesystem, dependencies: (contents: Uint8Array, path: string) => nativeCompilerElfSourcesV1(contents, path, (environment.LD_LIBRARY_PATH ?? "").split(":").filter(Boolean), candidate => existsSync(candidate) && !lstatSync(candidate).isDirectory(), defaults) };
  return Object.freeze({ roots: Object.freeze(roots), port });
}

/** 🖥️ Supplies the native compiler discovery interface on the current host. */
export function systemNativeCompilerHostV1(environment: Readonly<Record<string, string | undefined>> = process.env): NativeCompilerHostV1 {
  return { platform: process.platform, architecture: process.arch, home: homedir(), environment,
    isExecutable: path => { try { if (!statSync(path).isFile()) return false; accessSync(path, constants.X_OK); return true; } catch { return false; } },
    list: path => { try { return readdirSync(path); } catch { return []; } }, canonical: path => realpathSync(path), exists: path => { try { return statSync(path).isFile(); } catch { return false; } } };
}

/** 🔎️ Uses a locally installed SDK only when Linux has no native compiler on its declared search path. */
export function selectNativeCompilerV1(host: NativeCompilerHostV1 = systemNativeCompilerHostV1()): NativeCompilerSelectionV1 | null {
  if (host.platform !== "linux" || policy.explicitEnvironment.some(name => host.environment[name] !== undefined)) return null;
  const machine = policy.architectures[host.architecture as keyof typeof policy.architectures] as NativeCompilerSelectionV1["machine"] | undefined;
  if (!machine) return null;
  for (const directory of (host.environment.PATH ?? "").split(delimiter).filter(Boolean)) for (const name of ["cc", "gcc", "clang"]) if (host.isExecutable(join(directory, name))) return null;
  for (const base of [join(host.home, ".local/share/flatpak/runtime", policy.sdk, machine), join("/var/lib/flatpak/runtime", policy.sdk, machine)]) {
    const branches = host.list(base);
    if (branches.length > policy.maximumBranches) continue;
    for (const branch of branches.filter(name => /^[0-9]+\.[0-9]+$/u.test(name)).sort((a, b) => b.localeCompare(a, "en", { numeric: true }))) {
      const command = join(base, branch, "active/files/bin", policy.compiler);
      if (!host.isExecutable(command)) continue;
      const canonical = host.canonical(command);
      const libraryDirectory = [policy.libraryDirectory, join(policy.libraryDirectory, machine + "-linux-gnu")].find(directory => host.exists?.(join(directory, "libc.so"))) ?? policy.libraryDirectory;
      return Object.freeze({ version: 1, command: canonical, root: dirname(dirname(canonical)), machine, libraryDirectory });
    }
  }
  return null;
}

/** 🧰️ Gives GCC host binutils/runtime libraries and its SDK headers as separate, space-preserving arguments. */
export function nativeCompilerArgumentsV1(selection: NativeCompilerSelectionV1): readonly string[] {
  return Object.freeze(["-B" + policy.linkerDirectory, "-L" + (selection.libraryDirectory ?? policy.libraryDirectory), "-isystem" + join(selection.root, "include"), "-isystem" + join(selection.root, "include", selection.machine + "-linux-gnu")]);
}

/** 📜️ Renders the tracked POSIX driver alias without interpreting compiler arguments as shell input. */
export function nativeCompilerShimSourceV1(selection: NativeCompilerSelectionV1): string {
  const quote = (value: string): string => "'" + value.replaceAll("'", "'\\''") + "'";
  return "#!/bin/sh\nexec " + quote(selection.command) + ' "$@" ' + nativeCompilerArgumentsV1(selection).map(quote).join(" ") + "\n";
}

/** 🌿️ Projects a selected owned compiler alias without changing explicit compiler choices or Rust flags. */
export function nativeCompilerEnvironmentV1(environment: Readonly<Record<string, string | undefined>>, selection: NativeCompilerSelectionV1 | null, context: NativeCompilerContextV1): Readonly<Record<string, string | undefined>> {
  if (!selection || context.host !== selection.machine + "-unknown-linux-gnu") return environment;
  if (policy.explicitEnvironment.some(name => environment[name] !== undefined || context.configuredEnvironment.includes(name))) return environment;
  const cross = context.target !== context.host, key = "CARGO_TARGET_" + context.host.toUpperCase().replace(/-/gu, "_") + "_LINKER";
  const explicitFlags = [environment.RUSTFLAGS, environment.CARGO_ENCODED_RUSTFLAGS].some(value => value !== undefined && /(?:^|[\s\x1f])(?:-C\s*|--codegen[=\s]*)?linker=/u.test(value));
  const explicitLinker = context.configuredHostLinker || !cross && context.configuredLinker || environment[key] !== undefined || !cross && explicitFlags;
  const explicitCompiler = ["CC", "HOST_CC", ...(!cross ? ["TARGET_CC"] : []), "CC_" + context.host, "CC_" + context.host.replace(/-/gu, "_")].some(name => environment[name] !== undefined || context.configuredEnvironment.includes(name));
  return Object.freeze({ ...environment, ...(!explicitCompiler ? { [cross ? "HOST_CC" : "CC"]: selection.command } : {}), ...(!explicitLinker ? { PATH: [dirname(selection.command), environment.PATH].filter(Boolean).join(delimiter) } : {}) });
}
