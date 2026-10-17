import { spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { parseRustSourceTree, type RustModuleNode, type RustSourceItem } from "../🌳️module-tree/🟦️.ts";
import { relativePosix } from "../📝️render/🟦️.ts";

/** 🌳️ One resolved module of a crate: its path, owning file and cfg. */
export type ResolvedModule = Readonly<{ path: string; file: string | null; inline: boolean; cfgTest: boolean; visibility: string }>;

/** 🔍️ Resolves every module of a crate by effective path (component hops collapsed), following `#[path]` mounts as the source counter does. */
export function resolveCrateModules(repoRoot: string, rootFile: string, readText: (absolute: string) => string | null = (path) => (existsSync(path) ? readFileSync(path, "utf8") : null)): ResolvedModule[] {
  const out: ResolvedModule[] = [];
  const visitFile = (file: string, trail: readonly string[], cfgTest: boolean): void => {
    const text = readText(file);
    if (text === null) throw new Error(`Missing module file ${relativePosix(repoRoot, file)}`);
    const tree = parseRustSourceTree(text);
    visitNodes(tree.modules, tree.items, text, file, dirname(file), true, trail, cfgTest);
  };
  const visitNodes = (nodes: readonly RustModuleNode[], items: readonly RustSourceItem[], text: string, file: string, base: string, inFileScope: boolean, trail: readonly string[], cfgTest: boolean): void => {
    const hasGlob = items.some((item) => item.kind === "use" && /^pub\s+use\s+component::\*\s*;/u.test(text.slice(item.declStart, item.end)));
    for (const node of nodes) {
      if (trail.length === 0 && node.name === "mirror" && !node.inline && node.pathAttribute === "🪞️mirror/🦀️.rs") { visitFile(resolve(inFileScope ? dirname(file) : base, node.pathAttribute), trail, cfgTest || node.conditionalTest); continue; }
      const component = node.name === "component" && !node.inline && hasGlob;
      const path = component ? trail : [...trail, node.name];
      const test = cfgTest || node.conditionalTest;
      if (node.inline) {
        out.push({ path: path.join("::"), file: null, inline: true, cfgTest: test, visibility: node.visibility });
        visitNodes(node.children, node.items, text, file, resolve(base, node.pathAttribute ?? node.name), false, path, test);
      } else if (node.pathAttribute !== null) {
        const child = resolve(inFileScope ? dirname(file) : base, node.pathAttribute);
        out.push({ path: path.join("::") + (component ? "::<component>" : ""), file: relativePosix(repoRoot, child), inline: false, cfgTest: test, visibility: node.visibility });
        visitFile(child, path, test);
      } else out.push({ path: path.join("::"), file: "unresolved", inline: false, cfgTest: test, visibility: node.visibility });
    }
  };
  visitFile(rootFile, [], false);
  return out;
}

/** 🧮️ Compares the module tree of an original crate with the union of the carved crates by effective path: every original module must exist with the same file, and no extra file module may appear. */
export function compareModuleTrees(original: readonly ResolvedModule[], carved: readonly (readonly ResolvedModule[])[]): { problems: string[]; originalCount: number; unionCount: number } {
  const files = new Map<string, ResolvedModule>();
  const inline = new Set<string>();
  const problems: string[] = [];
  for (const crate of carved) for (const module of crate) {
    if (module.inline) { inline.add(module.path); continue; }
    const existing = files.get(module.path);
    if (existing && (existing.file !== module.file || existing.cfgTest !== module.cfgTest)) problems.push(`module ${module.path} is declared twice: ${existing.file} and ${module.file}`);
    else files.set(module.path, module);
  }
  const expectedFiles = new Map(original.filter((module) => !module.inline).map((module) => [module.path, module] as const));
  for (const [path, module] of expectedFiles) {
    const actual = files.get(path);
    if (!actual) problems.push(`module ${path} (${module.file}) is missing from the carved crates`);
    else if (actual.file !== module.file || actual.cfgTest !== module.cfgTest) problems.push(`module ${path} changed: ${module.file} -> ${actual.file}`);
  }
  for (const path of files.keys()) if (!expectedFiles.has(path)) problems.push(`module ${path} appears only in the carved crates`);
  for (const module of original) if (module.inline && !inline.has(module.path) && !files.has(module.path)) problems.push(`inline module ${module.path} is missing`);
  return { problems, originalCount: expectedFiles.size, unionCount: files.size };
}

/** 🔭️ Unresolved external crate names that rustc reports for one crate root (analysis only, output discarded). */
export function rustcUnresolvedCrates(rootFile: string, crateName: string, flags: readonly string[] = []): { unresolved: string[]; otherErrors: string[]; exit: number | null } {
  const result = spawnSync("rustc", ["+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", crateName.replaceAll("-", "_"), "--emit=metadata", "-o", "/dev/null", ...flags, rootFile], { encoding: "utf8", maxBuffer: 256 * 1024 * 1024, timeout: 120_000 });
  const text = `${result.stderr ?? ""}`;
  const unresolved = new Set<string>();
  const others: string[] = [];
  for (const match of text.matchAll(/^error(?:\[(E\d+)\])?: ([^\n]+)\n\s+--> ([^\n]+)/gmu)) {
    const message = match[2]!;
    const crate = /(?:unresolved module or unlinked crate|can't find crate for|unresolved import|use of unresolved module or unlinked crate|failed to resolve: use of undeclared crate or module|failed to resolve: could not find `[^`]+` in) `([A-Za-z0-9_]+)`/u.exec(message);
    if (crate && /unlinked crate|can't find crate|undeclared crate/u.test(message)) unresolved.add(crate[1]!);
    else if (/could not find `[^`]+` in `([A-Za-z0-9_]+)`/u.test(message)) unresolved.add(/in `([A-Za-z0-9_]+)`/u.exec(message)![1]!);
    else others.push(`${match[1] ?? ""} ${message} @ ${match[3]}`);
  }
  return { unresolved: [...unresolved].sort(), otherErrors: others, exit: result.status };
}

/** 🧱️ Copies source directories into a sandbox that mirrors the repository layout (node_modules and targets excluded). */
export function buildSandbox(repoRoot: string, sandboxRoot: string, directories: readonly string[]): void {
  rmSync(sandboxRoot, { recursive: true, force: true });
  for (const directory of directories) {
    const source = resolve(repoRoot, directory);
    const destination = resolve(sandboxRoot, directory);
    mkdirSync(dirname(destination), { recursive: true });
    cpSync(source, destination, { recursive: true, filter: (path) => !/\/(node_modules|target|dist)(\/|$)/u.test(path) });
  }
}

/** 🩹️ Sandbox patches that keep the differential effective: no `extern crate`, no external glob imports. */
export function patchSandboxSources(sandboxRoot: string, externalPrefixes: readonly string[]): void {
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = resolve(directory, entry.name);
      if (entry.isDirectory()) walk(path);
      else if (entry.name.endsWith(".rs")) {
        const text = readFileSync(path, "utf8");
        const patched = text
          .replace(/^[ \t]*(?:#\[[^\]]*\]\s*)*extern\s+crate\s+[^;]+;[ \t]*$/gmu, "")
          .replace(new RegExp(`\\buse\\s+(?:${externalPrefixes.map((prefix) => prefix.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join("|")})[A-Za-z0-9_:]*::\\*;`, "gu"), "const _: () = ();");
        if (patched !== text) writeFileSync(path, patched);
      }
    }
  };
  walk(sandboxRoot);
}

/** 🔬️ One normalised rustc diagnostic. */
export type OracleDiagnostic = string;

/** 🪤️ One rustc error with the facts the cascade filter needs. */
export type OracleEntry = Readonly<{ code: string; message: string; label: string; file: string; line: number; sourceLine: string }>;

/** 🧭️ The module path without the crate prefix, the combined-layout mount segments and the empty segments: the form in which a diagnostic names a module and a module resolution lists it. */
export const plainModulePath = (segments: readonly string[]): string => segments.filter((segment) => segment !== "" && segment !== "crate" && !segment.startsWith("__c_")).join("::");

/** 🪤️ Drops the `unresolved import` errors that only repeat a re-export of an external crate the sandbox does not link: a failed external import leaves no binding, so every further `use` of its name reports an error that the compiled crate does not have. A module that glob-re-exports an unlinked crate (`pub use ext::*;`, `pub use ext::{self as graph, *};`) gives no binding for any name, so an import through that module (`globModules` lists them by plain module path) is such a cascade as well. Both sides of a differential go through the same filter. */
export function dropUnlinkedCascades(entries: readonly OracleEntry[], globModules: ReadonlySet<string> = new Set()): OracleEntry[] {
  const tainted = new Set<string>();
  for (const entry of entries) if (entry.message.includes("unlinked crate") || entry.label.includes("unlinked crate")) for (const match of entry.sourceLine.matchAll(/[A-Za-z_][A-Za-z0-9_]*/gu)) tainted.add(match[0]);
  return entries.filter((entry) => {
    if (entry.code !== "E0432" || !entry.message.startsWith("unresolved import")) return true;
    const paths = [...entry.message.matchAll(/`([^`]+)`/gu)].map((match) => match[1]!.split("::"));
    return paths.length === 0 || !paths.every((path) => tainted.has(path.at(-1)!) || globModules.has(plainModulePath(path.slice(0, -1))));
  });
}

/** 🌐️ The plain module paths of the modules whose file glob-re-exports an unlinked external crate, read from the oracle entries that name such a glob. */
export function unlinkedGlobModules(sandboxRoot: string, rootFile: string, entries: readonly OracleEntry[]): Set<string> {
  const files = new Set(entries.filter((entry) => (entry.message.includes("unlinked crate") || entry.label.includes("unlinked crate")) && /\buse\b[^;]*\*/u.test(entry.sourceLine)).map((entry) => entry.file));
  const modules = new Set<string>();
  if (!files.size) return modules;
  try {
    for (const module of resolveCrateModules(sandboxRoot, rootFile)) if (module.file !== null && files.has(module.file)) modules.add(plainModulePath(module.path.replace(/::<component>$/u, "").split("::")));
  } catch { return modules; }
  return modules;
}

/** 🔬️ Runs the full-resolution oracle on one crate root and returns normalised diagnostics. */
export function runResolutionOracle(sandboxRoot: string, rootFile: string, crateName: string, flags: readonly string[]): { diagnostics: OracleDiagnostic[]; locations: string[]; exit: number | null } {
  const result = spawnSync("rustc", ["+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", crateName.replaceAll("-", "_"), "--emit=metadata", "--error-format=json", "-o", resolve(sandboxRoot, ".oracle.rmeta"), ...flags, rootFile], { encoding: "utf8", maxBuffer: 512 * 1024 * 1024, timeout: 300_000, env: { ...process.env, CARGO_MANIFEST_DIR: sandboxRoot } });
  const entries: OracleEntry[] = [];
  for (const line of (result.stderr ?? "").split("\n")) {
    if (!line.startsWith("{")) continue;
    try {
      const message = JSON.parse(line) as { level?: string; code?: { code?: string } | null; message?: string; spans?: { file_name: string; is_primary: boolean; label?: string | null; line_start?: number; text?: { text: string }[] }[] };
      if (message.level !== "error") continue;
      const primary = message.spans?.find((span) => span.is_primary);
      const file = primary ? relativePosix(sandboxRoot, resolve(sandboxRoot, primary.file_name)) : "";
      entries.push({ code: message.code?.code ?? "", message: (message.message ?? "").replaceAll(sandboxRoot, "<sandbox>").replaceAll(/\bcomponent::/gu, ""), label: primary?.label ?? "", file, line: primary?.line_start ?? 0, sourceLine: primary?.text?.[0]?.text ?? "" });
    } catch { continue; }
  }
  const kept = dropUnlinkedCascades(entries, unlinkedGlobModules(sandboxRoot, rootFile, entries));
  return { diagnostics: kept.map((entry) => [entry.code, entry.message, entry.file, entry.label].join("\t")), locations: kept.map((entry) => (entry.file ? `${entry.file}:${entry.line}` : "")), exit: result.status };
}

/** 🧮️ Multiset difference of two diagnostic lists. */
export function diffDiagnostics(original: readonly string[], candidate: readonly string[]): { onlyOriginal: string[]; onlyCandidate: string[] } {
  const counts = new Map<string, number>();
  for (const entry of original) counts.set(entry, (counts.get(entry) ?? 0) + 1);
  const onlyCandidate: string[] = [];
  for (const entry of candidate) {
    const left = counts.get(entry) ?? 0;
    if (left > 0) counts.set(entry, left - 1);
    else onlyCandidate.push(entry);
  }
  const onlyOriginal: string[] = [];
  for (const [entry, count] of counts) for (let i = 0; i < count; i += 1) onlyOriginal.push(entry);
  return { onlyOriginal, onlyCandidate };
}

/** 🛡️ One cross-crate path whose target is not publicly reachable. */
export type VisibilityViolation = Readonly<{ from: string; file: string; chain: string; reason: string }>;

/** 🛡️ Checks every `crate::<unit>::…` path that crosses a crate boundary resolves through `pub` modules to a `pub` item. */
export function checkCrossCrateVisibility(repoRoot: string, crates: readonly { name: string; unitPaths: readonly string[][]; rootFile: string; rootText: string; sourceFiles: ReadonlyMap<string, string> }[], chainsOf: (text: string) => string[][]): VisibilityViolation[] {
  const violations: VisibilityViolation[] = [];
  type Info = { modules: Map<string, ResolvedModule>; items: Map<string, Map<string, { kind: string; visibility: string }>>; globs: Map<string, string[]> };
  const infos = new Map<string, Info>();
  for (const crate of crates) {
    const modules = new Map(resolveCrateModules(repoRoot, crate.rootFile, (path) => (path === crate.rootFile ? crate.rootText : existsSync(path) ? readFileSync(path, "utf8") : null)).map((entry) => [entry.path, entry] as const));
    const items = new Map<string, Map<string, { kind: string; visibility: string }>>();
    const globs = new Map<string, string[]>();
    const indexFile = (key: string, file: string): void => {
      const text = file === crate.rootFile ? crate.rootText : existsSync(file) ? readFileSync(file, "utf8") : null;
      if (text === null) return;
      const tree = parseRustSourceTree(text);
      const names = items.get(key) ?? new Map();
      for (const item of tree.items) if (item.name && item.kind !== "impl") names.set(item.name, { kind: item.kind, visibility: item.visibility });
      for (const item of tree.items) if (item.kind === "use" && /^pub\s+use\s+[A-Za-z0-9_]+::\*;/u.test(text.slice(item.declStart, item.end))) globs.set(key, [...(globs.get(key) ?? []), /use\s+([A-Za-z0-9_]+)::\*/u.exec(text.slice(item.declStart, item.end))![1]!]);
      items.set(key, names);
    };
    for (const [key, module] of modules) if (module.file && module.file !== "unresolved") indexFile(key, resolve(repoRoot, module.file));
    infos.set(crate.name, { modules, items, globs });
  }
  for (const caller of crates) for (const [file, text] of caller.sourceFiles) for (const chain of chainsOf(text)) {
    const target = crates.find((candidate) => candidate.name !== caller.name && candidate.unitPaths.some((unit) => unit.length <= chain.length && unit.every((segment, i) => segment === chain[i])));
    if (!target) continue;
    const info = infos.get(target.name)!;
    const unit = target.unitPaths.filter((entry) => entry.length <= chain.length && entry.every((segment, i) => segment === chain[i])).sort((a, b) => b.length - a.length)[0]!;
    let key = unit.join("::");
    for (let i = unit.length; i < chain.length; i += 1) {
      const segment = chain[i]!;
      const childKey = `${key}::${segment}`;
      const child = info.modules.get(childKey);
      if (child) {
        if (child.visibility !== "pub") { violations.push({ from: caller.name, file, chain: chain.join("::"), reason: `module ${childKey} is ${child.visibility}` }); break; }
        key = childKey;
        continue;
      }
      const sources = [key, ...((info.globs.get(key) ?? []).map((glob) => `${key}::${glob}`))];
      const found = sources.map((source) => info.items.get(source)?.get(segment)).find(Boolean);
      if (found && found.visibility !== "pub" && found.kind !== "use") violations.push({ from: caller.name, file, chain: chain.join("::"), reason: `item ${segment} in ${key} is ${found.visibility}` });
      break;
    }
  }
  return violations;
}

/** ⚖️ One impl that the orphan rules would reject after the carve. */
export type CoherenceViolation = Readonly<{ crate: string; file: string; trait: string | null; selfType: string; definedIn: readonly string[] }>;

/** ⚖️ Flags `impl <foreign trait> for <type defined in another carved crate>` and inherent impls of foreign types. */
export function checkOrphanRules(crates: readonly { name: string; sources: ReadonlyMap<string, string> }[], implHeaderOf: (text: string) => { trait: string | null; self: string | null; traitArgs: readonly string[] } | null, itemsOf: (text: string) => readonly RustSourceItem[]): CoherenceViolation[] {
  const typeDefs = new Map<string, Set<string>>();
  const traits = new Set<string>();
  for (const crate of crates) for (const [, text] of crate.sources) for (const item of itemsOf(text)) {
    if ((item.kind === "struct" || item.kind === "enum" || item.kind === "type") && item.name) typeDefs.set(item.name, new Set([...(typeDefs.get(item.name) ?? []), crate.name]));
    if (item.kind === "trait" && item.name) traits.add(item.name);
  }
  const violations: CoherenceViolation[] = [];
  for (const crate of crates) for (const [file, text] of crate.sources) for (const item of itemsOf(text)) {
    if (item.kind !== "impl") continue;
    const header = implHeaderOf(text.slice(item.declStart, item.end));
    if (!header?.self) continue;
    const owners = typeDefs.get(header.self);
    if (!owners || owners.has(crate.name)) continue;
    if (header.trait && traits.has(header.trait)) continue;
    if (header.traitArgs.some((argument) => typeDefs.get(argument)?.has(crate.name))) continue;
    violations.push({ crate: crate.name, file, trait: header.trait, selfType: header.self, definedIn: [...owners] });
  }
  return violations;
}

/** 🔁️ One source file that the module trees of more than one package reach, with the manifests of those packages. */
export type SharedMount = Readonly<{ file: string; manifests: readonly string[] }>;

/** 🔁️ Lists the source files reached from more than one package: every package contributes the closure of its lib, test and bin roots (explicit `#[path]` mounts followed, missing generated files tolerated). A file with a second owner is the same source compiled into two crates, so each owner must be the single crate that the others depend on. `within` keeps only the files that at least one matching manifest reaches. */
export function sharedMountFiles(repoRoot: string, manifests: readonly string[], within: (manifest: string) => boolean = () => true): SharedMount[] {
  const owners = new Map<string, Set<string>>();
  const read = (file: string): string => (existsSync(file) ? readFileSync(file, "utf8") : "");
  for (const manifest of manifests) {
    const absolute = resolve(repoRoot, manifest);
    if (!existsSync(absolute)) continue;
    let parsed: { package?: unknown; lib?: { path?: string }; test?: { path?: string; name?: string }[]; bin?: { path?: string; name?: string }[] };
    try { parsed = Bun.TOML.parse(readFileSync(absolute, "utf8")) as typeof parsed; } catch { continue; }
    if (!parsed.package) continue;
    const directory = dirname(absolute);
    const roots = [parsed.lib?.path ?? "src/lib.rs", ...(parsed.test ?? []).map((entry) => entry.path ?? `tests/${entry.name}.rs`), ...(parsed.bin ?? []).map((entry) => entry.path ?? `src/bin/${entry.name}.rs`)].map((path) => resolve(directory, path)).filter((path) => existsSync(path));
    for (const root of roots) for (const file of new Set([relativePosix(repoRoot, root), ...resolveCrateModules(repoRoot, root, read).flatMap((module) => (module.file !== null && module.file !== "unresolved" ? [module.file] : []))])) owners.set(file, new Set([...(owners.get(file) ?? []), manifest]));
  }
  return [...owners].filter(([, set]) => set.size > 1 && [...set].some(within)).map(([file, set]) => ({ file, manifests: [...set].sort() })).sort((left, right) => left.file.localeCompare(right.file));
}
