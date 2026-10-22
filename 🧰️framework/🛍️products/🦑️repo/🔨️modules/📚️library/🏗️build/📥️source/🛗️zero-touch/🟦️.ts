import { existsSync, readFileSync } from "node:fs";
import { isBuiltin } from "node:module";
import { join, posix, resolve } from "node:path";
import { isRuntimeBuiltinSpecifierV1 } from "../../../../../../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";
import { TranspilerV1 } from "../../../../../../../🔨️modules/⚙️runtime/🔬️source/🟦️.ts";

/** 🛗️ One entry whose relative import closure must run on a checkout without installed dependencies; `static` follows only static imports, `full` also dynamic imports and requires. */
export type ZeroTouchClosureV1 = Readonly<{ id: string; entry: string; mode: "full" | "static"; reason: string }>;

/** 🚧️ One module that needs installed dependencies and may only be reached through a dynamic import or a require call of a closure. */
export type ZeroTouchSeamV1 = Readonly<{ module: string; reason: string }>;

/** 🗺️ The authored roster: the closures and the post-install seams they must not load eagerly. */
export type ZeroTouchRosterV1 = Readonly<{ closures: readonly ZeroTouchClosureV1[]; seams: readonly ZeroTouchSeamV1[] }>;

/** 📦️ One workspace package as the specifier resolver sees it: its repository-relative directory and its `exports` table. */
export type ZeroTouchPackageV1 = Readonly<{ directory: string; exports: Readonly<Record<string, unknown>> }>;

/** 🌲️ The workspace as the zero-touch law sees it: text reads by repository-relative path and the workspace packages by name. */
export interface ZeroTouchTreeV1 {
  read(path: string): string | undefined;
  readonly packages: Readonly<Record<string, ZeroTouchPackageV1>>;
}

/** 🚫️ The finding kinds; `workspace-specifier` is a bare name of a workspace package, `external-package` any other bare name, `seam-static` a static import of a post-install seam, `missing-entry` a closure entry or seam that is gone. */
export type ZeroTouchFindingKindV1 = "workspace-specifier" | "external-package" | "seam-static" | "missing-entry";

/** 🔎️ One import that leaves the relative closure of an entry. */
export type ZeroTouchFindingV1 = Readonly<{ closure: string; kind: ZeroTouchFindingKindV1; file: string; specifier: string; detail: string }>;

/** 🧾️ The import kinds of a module: `static` import and re-export statements, `dynamic` import calls, `require` calls. */
export type ZeroTouchImportKindV1 = "static" | "dynamic" | "require";

const CODE = /\.(?:[cm]?[jt]s|tsx)$/u;
const EXTENSIONS = [".ts", ".tsx", ".mts", ".js", ".mjs"];
const KINDS: Readonly<Record<string, ZeroTouchImportKindV1>> = { "import-statement": "static", "dynamic-import": "dynamic", "require-call": "require" };
const compareBytes = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));
const decode = (specifier: string): string => {
  if (!/[\u0080-ÿ]/u.test(specifier) || /[Ā-￿]/u.test(specifier)) return specifier;
  const decoded = Buffer.from(specifier, "latin1").toString("utf8");
  return Buffer.from(decoded).toString("latin1") === specifier ? decoded : specifier;
};

/** 🔬️ Lists the runtime module specifiers of one source text through the system compiler: type-only imports carry no runtime and stay out. */
export function scanModuleSpecifiersV1(path: string, text: string): readonly Readonly<{ specifier: string; kind: ZeroTouchImportKindV1 }>[] {
  const source = text.replace(/^#![^\r\n]*/u, ""), transpiler = new TranspilerV1({ loader: /\.[cm]?tsx?$/u.test(path) ? "ts" : "js", target: "runtime", define: { "import.meta.vitest": "undefined" } });
  const rows = new Map<string, Readonly<{ specifier: string; kind: ZeroTouchImportKindV1 }>>();
  for (const row of [...transpiler.scan(source).imports, ...transpiler.scanImports(source)]) {
    const kind = KINDS[row.kind];
    if (kind === undefined) continue;
    const specifier = decode(row.path);
    rows.set(`${kind}\0${specifier}`, { specifier, kind });
  }
  return [...rows.values()].sort((left, right) => compareBytes(left.specifier, right.specifier) || compareBytes(left.kind, right.kind));
}

/** 🎯️ Resolves a package export target to a repository-relative file, honouring condition maps and one `*` wildcard. */
export function workspaceSpecifierTargetV1(packages: Readonly<Record<string, ZeroTouchPackageV1>>, specifier: string): string | undefined {
  const pick = (value: unknown): string | undefined => (typeof value === "string" ? value : value !== null && typeof value === "object" ? pick((value as Record<string, unknown>).bun ?? (value as Record<string, unknown>).import ?? (value as Record<string, unknown>).default) : undefined);
  for (const name of Object.keys(packages).filter((candidate) => specifier === candidate || specifier.startsWith(`${candidate}/`)).sort((left, right) => right.length - left.length)) {
    const owner = packages[name]!, key = `.${specifier.slice(name.length)}`, direct = pick(owner.exports[key]);
    if (direct !== undefined) return posix.normalize(posix.join(owner.directory, direct));
    for (const [pattern, value] of Object.entries(owner.exports)) {
      const star = pattern.indexOf("*");
      if (star < 0) continue;
      const head = pattern.slice(0, star), tail = pattern.slice(star + 1), target = pick(value);
      if (target !== undefined && key.startsWith(head) && key.endsWith(tail) && key.length >= head.length + tail.length) return posix.normalize(posix.join(owner.directory, target.replace("*", key.slice(head.length, key.length - tail.length))));
    }
  }
  return undefined;
}

function relativeTarget(tree: ZeroTouchTreeV1, from: string, specifier: string): string | undefined {
  const base = posix.normalize(posix.join(posix.dirname(from), specifier.split("?")[0]!));
  for (const candidate of [base, ...EXTENSIONS.map((extension) => base + extension)]) if (CODE.test(candidate) && tree.read(candidate) !== undefined) return candidate;
  return undefined;
}

/** 🪜️ Walks the relative import closure of every entry and lists each import that is neither relative nor a runtime builtin: workspace bare names must be relative, external packages are refused (the capture of `🔏️closure` refuses the same set on disk). */
export function zeroTouchClosureFindingsV1(roster: ZeroTouchRosterV1, tree: ZeroTouchTreeV1): Readonly<{ files: Readonly<Record<string, number>>; findings: readonly ZeroTouchFindingV1[] }> {
  const findings: ZeroTouchFindingV1[] = [], files: Record<string, number> = {}, seams = new Set(roster.seams.map((seam) => seam.module));
  for (const seam of roster.seams) if (tree.read(seam.module) === undefined) findings.push({ closure: "seams", kind: "missing-entry", file: seam.module, specifier: seam.module, detail: "The post-install seam module does not exist." });
  for (const closure of roster.closures) {
    const seen = new Set<string>(), pending = [closure.entry];
    if (tree.read(closure.entry) === undefined) findings.push({ closure: closure.id, kind: "missing-entry", file: closure.entry, specifier: closure.entry, detail: "The closure entry does not exist." });
    while (pending.length) {
      const file = pending.pop()!;
      if (seen.has(file)) continue;
      seen.add(file);
      const text = tree.read(file);
      if (text === undefined) continue;
      for (const { specifier, kind } of scanModuleSpecifiersV1(file, text)) {
        if (closure.mode === "static" && kind !== "static") continue;
        if (specifier.startsWith(".")) {
          const target = relativeTarget(tree, file, specifier);
          if (target === undefined) continue;
          if (!seams.has(target)) pending.push(target);
          else if (kind === "static") findings.push({ closure: closure.id, kind: "seam-static", file, specifier, detail: `The post-install module ${target} may only be loaded by a dynamic import.` });
        } else if (specifier.startsWith("node:") || isRuntimeBuiltinSpecifierV1(specifier) || isBuiltin(specifier)) continue;
        else if (Object.keys(tree.packages).some((name) => specifier === name || specifier.startsWith(`${name}/`))) {
          const target = workspaceSpecifierTargetV1(tree.packages, specifier);
          findings.push({ closure: closure.id, kind: "workspace-specifier", file, specifier, detail: target === undefined ? "Import the owner module by a relative path; no export of the package resolves this specifier." : `Import the owner module by the relative path to ${target}.` });
        } else findings.push({ closure: closure.id, kind: "external-package", file, specifier, detail: `External implementation runtime dependency refused: ${specifier}` });
      }
    }
    files[closure.id] = seen.size;
  }
  findings.sort((left, right) => compareBytes(left.closure, right.closure) || compareBytes(left.file, right.file) || compareBytes(left.specifier, right.specifier));
  return { files, findings };
}

/** 📋️ Reads the roster authored in `🗂️closures/🔣️.json` and refuses any shape its schema does not describe. */
export function readZeroTouchRosterV1(text: string): ZeroTouchRosterV1 {
  const value = JSON.parse(text) as { version?: unknown; closures?: unknown; seams?: unknown };
  if (value.version !== 1 || !Array.isArray(value.closures) || value.closures.length === 0 || !Array.isArray(value.seams) || Object.keys(value).length !== 3) throw Error("Closed zero-touch closure roster required");
  const ids = new Set<string>(), modules = new Set<string>();
  const closures = value.closures.map((row: Record<string, unknown>) => {
    if (typeof row.id !== "string" || typeof row.entry !== "string" || typeof row.reason !== "string" || (row.mode !== "full" && row.mode !== "static") || Object.keys(row).length !== 4 || ids.has(row.id)) throw Error("Closed zero-touch closure row required");
    ids.add(row.id);
    return { id: row.id, entry: row.entry, mode: row.mode, reason: row.reason } as ZeroTouchClosureV1;
  });
  const seams = value.seams.map((row: Record<string, unknown>) => {
    if (typeof row.module !== "string" || typeof row.reason !== "string" || Object.keys(row).length !== 2 || modules.has(row.module)) throw Error("Closed zero-touch seam row required");
    modules.add(row.module);
    return { module: row.module, reason: row.reason } as ZeroTouchSeamV1;
  });
  return { closures, seams };
}

/** 🌱️ The checkout on disk: files by repository-relative path and the packages named by the root `workspaces` list. */
export function diskZeroTouchTreeV1(root: string): ZeroTouchTreeV1 {
  const base = resolve(root), packages: Record<string, ZeroTouchPackageV1> = {};
  const manifest = JSON.parse(readFileSync(join(base, "package.json"), "utf8")) as { workspaces?: string[] };
  for (const directory of manifest.workspaces ?? []) {
    const path = join(base, directory, "package.json");
    if (!existsSync(path)) continue;
    const json = JSON.parse(readFileSync(path, "utf8")) as { name?: string; exports?: string | Record<string, unknown> };
    if (typeof json.name === "string" && json.exports !== undefined) packages[json.name] = { directory, exports: typeof json.exports === "string" ? { ".": json.exports } : json.exports };
  }
  return { packages, read: (path) => { try { return readFileSync(join(base, path), "utf8"); } catch { return undefined; } } };
}

export const ZERO_TOUCH_CLOSURES_FILE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🗂️closures/🔣️.json";

/** ✅️ Checks the authored closure roster against the checkout on disk. */
export function checkZeroTouchClosuresV1(root: string): ReturnType<typeof zeroTouchClosureFindingsV1> {
  const tree = diskZeroTouchTreeV1(root), roster = tree.read(ZERO_TOUCH_CLOSURES_FILE);
  if (roster === undefined) throw Error("Closed zero-touch closure roster required");
  return zeroTouchClosureFindingsV1(readZeroTouchRosterV1(roster), tree);
}

/** 🧭️ One root command whose module closure the law holds to the registry's declaration. */
export type ZeroTouchCommandV1 = Readonly<{ id: string; module: string; requires: "none" | "dependencies" }>;

/** 🪜️ Walks the full closure of every command module: a workspace package specifier is refused for every command; an external package is refused for `none` and allowed for `dependencies`, whose module is only loaded after the router has provisioned the dependencies; a `none` command reaches a seam by a dynamic import only. */
export function zeroTouchCommandFindingsV1(commands: readonly ZeroTouchCommandV1[], seams: readonly string[], tree: ZeroTouchTreeV1): Readonly<{ files: Readonly<Record<string, number>>; findings: readonly ZeroTouchFindingV1[] }> {
  const findings: ZeroTouchFindingV1[] = [], files: Record<string, number> = {}, seamModules = new Set(seams);
  for (const command of commands) {
    const closure = `command:${command.id}`, seen = new Set<string>(), pending = [command.module];
    if (tree.read(command.module) === undefined) findings.push({ closure, kind: "missing-entry", file: command.module, specifier: command.module, detail: "The command module does not exist." });
    while (pending.length) {
      const file = pending.pop()!;
      if (seen.has(file)) continue;
      seen.add(file);
      const text = tree.read(file);
      if (text === undefined) continue;
      for (const { specifier, kind } of scanModuleSpecifiersV1(file, text)) {
        if (specifier.startsWith(".")) {
          const target = relativeTarget(tree, file, specifier);
          if (target === undefined) continue;
          if (command.requires === "dependencies" || !seamModules.has(target)) pending.push(target);
          else if (kind === "static") findings.push({ closure, kind: "seam-static", file, specifier, detail: `The post-install module ${target} may only be loaded by a dynamic import.` });
        } else if (specifier.startsWith("node:") || isRuntimeBuiltinSpecifierV1(specifier) || isBuiltin(specifier)) continue;
        else if (Object.keys(tree.packages).some((name) => specifier === name || specifier.startsWith(`${name}/`))) {
          const target = workspaceSpecifierTargetV1(tree.packages, specifier);
          findings.push({ closure, kind: "workspace-specifier", file, specifier, detail: target === undefined ? "Import the owner module by a relative path; no export of the package resolves this specifier." : `Import the owner module by the relative path to ${target}.` });
        } else if (command.requires === "none") findings.push({ closure, kind: "external-package", file, specifier, detail: `The command declares requires none but reaches the external package ${specifier}; declare requires dependencies or remove the import.` });
      }
    }
    files[closure] = seen.size;
  }
  findings.sort((left, right) => compareBytes(left.closure, right.closure) || compareBytes(left.file, right.file) || compareBytes(left.specifier, right.specifier));
  return { files, findings };
}

/** 📋️ Reads the command rows the law needs from the registry text; the registry schema owns the complete shape. */
export function readZeroTouchCommandsV1(text: string): readonly ZeroTouchCommandV1[] {
  const value = JSON.parse(text) as { commands?: unknown };
  if (!Array.isArray(value.commands) || value.commands.length === 0) throw Error("Root command registry required");
  const ids = new Set<string>();
  return value.commands.map((row: Record<string, unknown>) => {
    if (typeof row.id !== "string" || typeof row.module !== "string" || (row.requires !== "none" && row.requires !== "dependencies") || ids.has(row.id)) throw Error("Root command registry row required");
    ids.add(row.id);
    return { id: row.id, module: row.module, requires: row.requires };
  });
}

export const ZERO_TOUCH_COMMANDS_FILE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🔣️.json";

/** ✅️ Checks every command of the root registry against the checkout on disk; the seams of the closure roster are the post-install modules. */
export function checkZeroTouchCommandsV1(root: string): ReturnType<typeof zeroTouchCommandFindingsV1> {
  const tree = diskZeroTouchTreeV1(root), registry = tree.read(ZERO_TOUCH_COMMANDS_FILE), roster = tree.read(ZERO_TOUCH_CLOSURES_FILE);
  if (registry === undefined || roster === undefined) throw Error("Root command registry and closure roster required");
  return zeroTouchCommandFindingsV1(readZeroTouchCommandsV1(registry), readZeroTouchRosterV1(roster).seams.map((seam) => seam.module), tree);
}
