import { dirname, posix, relative, resolve } from "node:path";
import { rustTokens } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import type { RustModuleNode } from "../🌳️module-tree/🟦️.ts";

/** 🧭️ Repo-relative path between two absolute paths, always with forward slashes. */
export function relativePosix(from: string, to: string): string {
  const value = relative(from, to).replaceAll("\\", "/");
  return value === "" ? "." : value;
}

/** 🔤️ Compares strings by UTF-8 byte order, the order Cargo and the registries keep. */
export function compareBytes(left: string, right: string): number {
  return Buffer.compare(Buffer.from(left, "utf8"), Buffer.from(right, "utf8"));
}

/** 🪞️ One module declaration a carved crate re-declares: its effective path, the declaring text and scope. */
export type MirrorUnit = Readonly<{ path: readonly string[]; node: RustModuleNode; sourceText: string; oldScope: string; component: boolean; fileDir?: string }>;

/** 📥️ One module path a carved crate imports from a dependency crate (a glob import for component items). */
export type MirrorImport = Readonly<{ path: readonly string[]; crateIdent: string; visibility: string; order: number; glob: boolean }>;

/** 🧾️ Everything needed to write the mirror root of one carved crate. */
export type MirrorSpec = Readonly<{
  doc: string;
  newRootDir: string;
  innerAttributes: readonly string[];
  externLines: readonly string[];
  movedItemsText: string;
  itemImports: readonly string[];
  owned: readonly MirrorUnit[];
  imported: readonly MirrorImport[];
  wrapperNodes: ReadonlyMap<string, RustModuleNode>;
  hoist?: boolean;
}>;

type TrieNode = { name: string; children: Map<string, TrieNode>; owned: MirrorUnit | null; component: MirrorUnit | null; imported: MirrorImport | null; globs: MirrorImport[]; order: number };

/** 🛤️ Rewrites every `#[path]` of a copied module declaration so it resolves from the new scope. */
export function rewriteMountPaths(source: string, node: RustModuleNode, oldScope: string, newScope: string): string {
  const edits: { start: number; end: number; text: string }[] = [];
  const visit = (current: RustModuleNode, oldBase: string, newBase: string): void => {
    if (current.pathAttribute !== null) {
      const region = source.slice(current.start, current.declStart);
      const match = /path\s*=\s*"((?:[^"\\]|\\.)*)"/u.exec(region);
      if (match) {
        const literalStart = current.start + match.index + match[0].indexOf('"') + 1;
        const literalEnd = literalStart + match[1]!.length;
        if (current.inline) {
          if (current.pathAttribute !== ".") throw new Error(`Inline module ${current.name} uses a non-trivial #[path]; mirror it by hand`);
        } else {
          const target = resolve(oldBase, current.pathAttribute);
          edits.push({ start: literalStart, end: literalEnd, text: relativePosix(newBase, target) });
        }
      }
    }
    if (current.inline) {
      const nextOld = resolve(oldBase, current.pathAttribute ?? current.name);
      const nextNew = resolve(newBase, current.pathAttribute ?? current.name);
      for (const child of current.children) visit(child, nextOld, nextNew);
    }
  };
  visit(node, oldScope, newScope);
  let text = source.slice(node.start, node.end);
  for (const edit of edits.sort((a, b) => b.start - a.start)) text = text.slice(0, edit.start - node.start) + edit.text + text.slice(edit.end - node.start);
  const lineStart = source.lastIndexOf("\n", node.start - 1) + 1;
  const column = /^[ \t]*$/u.test(source.slice(lineStart, node.start)) ? node.start - lineStart : 0;
  if (!column) return text;
  return text.split("\n").map((line, index) => (index === 0 ? line : line.replace(new RegExp(`^[ \\t]{0,${column}}`, "u"), ""))).join("\n");
}

/** 🪞️ Renders the lib root of a carved crate: header, moved root items, and the mirrored module nest. */
export function renderMirrorRoot(spec: MirrorSpec): string {
  return renderMirrorFiles(spec).root;
}

const mirrorText = (unit: MirrorUnit, nestDir: string): string => rebaseRustIncludes(rewriteMountPaths(unit.sourceText, unit.node, unit.oldScope, nestDir), unit.fileDir ?? unit.oldScope, nestDir);

const closesWithBrace = (text: string): boolean => text.trimEnd().endsWith("}");

/** 🪞️ Renders the mirror root and, when the nest is too long or carries authored items, moves the nest into `🪞️mirror/🦀️.rs` so the lib root stays pure wiring. */
export function renderMirrorFiles(spec: MirrorSpec): { root: string; body: string | null } {
  const nestDir = spec.hoist ? `${spec.newRootDir}/🪞️mirror` : spec.newRootDir;
  const root: TrieNode = { name: "", children: new Map(), owned: null, component: null, imported: null, globs: [], order: 0 };
  const place = (path: readonly string[], assign: (node: TrieNode) => void, order: number): void => {
    let cursor = root;
    path.forEach((segment, depth) => {
      let next = cursor.children.get(segment);
      if (!next) cursor.children.set(segment, (next = { name: segment, children: new Map(), owned: null, component: null, imported: null, globs: [], order: spec.wrapperNodes.get(path.slice(0, depth + 1).join("::"))?.start ?? order }));
      cursor = next;
    });
    assign(cursor);
  };
  for (const unit of spec.owned) place(unit.path, (node) => { if (unit.component) node.component = unit; else { node.owned = unit; node.order = unit.node.start + 0; } }, unit.node.start);
  for (const entry of spec.imported) place(entry.path, (node) => { if (entry.glob) node.globs.push(entry); else { node.imported = entry; node.order = entry.order; } }, entry.order);
  const lines: string[] = [];
  const emit = (node: TrieNode, trail: readonly string[], indent: string): void => {
    const children = [...node.children.values()].sort((a, b) => a.order - b.order);
    for (const child of children) {
      const childTrail = [...trail, child.name];
      if (child.owned && !child.component && !child.children.size) {
        const text = mirrorText(child.owned, nestDir);
        lines.push(...text.split("\n").map((line) => (line.length ? indent + line : line)));
      } else if (child.imported && !child.component && !child.children.size) {
        lines.push(`${indent}${child.imported.visibility === "private" ? "" : `${child.imported.visibility} `}use ${child.imported.crateIdent}::${childTrail.join("::")};`);
      } else if (child.owned && child.owned.node.inline && !child.component && child.children.size && closesWithBrace(mirrorText(child.owned, nestDir))) {
        const ownedLines = mirrorText(child.owned, nestDir).split("\n").map((line) => (line.length ? indent + line : line));
        let last = ownedLines.length - 1;
        while (last >= 0 && ownedLines[last]!.trim() === "") last -= 1;
        lines.push(...ownedLines.slice(0, last));
        const inner = indent + "    ";
        for (const glob of child.globs) lines.push(`${inner}pub use ${glob.crateIdent}::${childTrail.join("::")}::*;`);
        if (child.imported) lines.push(`${inner}pub use ${child.imported.crateIdent}::${childTrail.join("::")}::*;`);
        emit(child, childTrail, inner);
        lines.push(`${indent}}`);
      } else {
        const wrapper = spec.wrapperNodes.get(childTrail.join("::"));
        const attributes = wrapper?.attributeTexts.map((text) => `${indent}#[${/^path\s*=/u.test(text) ? 'path = "."' : text}]`) ?? [`${indent}#[path = "."]`];
        lines.push(...attributes, `${indent}${wrapper?.visibility && wrapper.visibility !== "private" ? `${wrapper.visibility} ` : "pub "}mod ${child.name} {`);
        const inner = indent + "    ";
        if (child.owned) {
          const text = mirrorText(child.owned, nestDir);
          lines.push(...text.split("\n").map((line) => (line.length ? inner + line : line)));
        }
        if (child.component) {
          const text = mirrorText(child.component, nestDir);
          lines.push(...text.split("\n").map((line) => (line.length ? inner + line : line)), `${inner}pub use component::*;`);
        }
        for (const glob of child.globs) lines.push(`${inner}pub use ${glob.crateIdent}::${childTrail.join("::")}::*;`);
        if (child.imported) lines.push(`${inner}pub use ${child.imported.crateIdent}::${childTrail.join("::")}::*;`);
        emit(child, childTrail, inner);
        lines.push(`${indent}}`);
      }
    }
  };
  emit(root, [], "");
  const header = [spec.doc, "", ...spec.innerAttributes, ...(spec.innerAttributes.length ? [""] : []), ...spec.externLines, ...(spec.externLines.length ? [""] : []), ...spec.itemImports, ...(spec.itemImports.length ? [""] : [])];
  const moved = spec.movedItemsText.trim() ? [spec.movedItemsText.trim(), ""] : [];
  const widened = lines.map((line) => line.replace(/^(\s*)pub\((?:crate|super|self|in [^)]*)\)(\s+(?:mod|use)\s)/u, "$1pub$2"));
  const tidy = (parts: readonly string[]): string => parts.join("\n").replace(/\n{3,}/gu, "\n\n").trimEnd() + "\n";
  if (!spec.hoist) return { root: tidy([...header, ...moved, ...widened]), body: null };
  return {
    root: tidy([...header, ...moved, '#[path = "🪞️mirror/🦀️.rs"]', "mod mirror;", "pub use mirror::*;"]),
    body: tidy(["//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.", "", ...widened]),
  };
}

/** 📄️ Manifest rows copied from an original package. */
export type ManifestDependencyRows = Readonly<{ dependencies: readonly string[]; devDependencies: readonly string[]; targetSections: readonly string[]; unsupported: readonly string[] }>;

/** 🔍️ Extracts dependency rows from a Cargo manifest as authored text, flagging sections this carve cannot copy. */
export function readManifestDependencyRows(manifestText: string): ManifestDependencyRows {
  const dependencies: string[] = [];
  const devDependencies: string[] = [];
  const targetSections: string[] = [];
  const unsupported: string[] = [];
  let section = "";
  let target: string[] | null = null;
  for (const line of manifestText.split("\n")) {
    const header = /^\s*\[{1,2}([^\]]+)\]{1,2}\s*$/u.exec(line);
    if (header) {
      section = header[1]!.trim();
      if (target) targetSections.push(target.join("\n"));
      target = /^target\..*dependencies$/u.test(section) ? [line] : null;
      if (/^(features|bin|test|bench|example|build-dependencies|patch|replace)(\.|$)/u.test(section) || section === "package.metadata.semio.rust-source-boundary") unsupported.push(section);
      continue;
    }
    if (target) { target.push(line); continue; }
    if (!line.trim() || line.trim().startsWith("#")) continue;
    if (section === "dependencies") dependencies.push(line);
    else if (section === "dev-dependencies") devDependencies.push(line);
  }
  if (target) targetSections.push(target.join("\n"));
  return { dependencies, devDependencies, targetSections, unsupported };
}

/** 🛤️ Re-bases every relative `path = "…"` of manifest rows from one directory to another. */
export function rebaseManifestRow(row: string, fromDir: string, toDir: string): string {
  return row.replace(/path\s*=\s*"([^"]*)"/gu, (_, value: string) => `path = "${relativePosix(toDir, resolve(fromDir, value))}"`);
}

/** 🔑️ Dependency key of one manifest row. */
export function manifestRowKey(row: string): string {
  return /^\s*([A-Za-z0-9_\-]+)\s*=/u.exec(row)?.[1] ?? "";
}

/** 📦️ Everything needed to write one carved crate manifest. */
export type ManifestSpec = Readonly<{
  name: string;
  description: string;
  workspaceRel: string;
  libPath: string;
  dependencyRows: readonly string[];
  devDependencyRows: readonly string[];
  targetSections: readonly string[];
}>;

/** 📝️ Renders the manifest of a carved library crate. */
export function renderCrateManifest(spec: ManifestSpec): string {
  const lines = [
    "[package]",
    `workspace = "${spec.workspaceRel}"`,
    `name = "${spec.name}"`,
    "version.workspace = true",
    "edition.workspace = true",
    "rust-version.workspace = true",
    `description = ${JSON.stringify(spec.description)}`,
    "",
    "[package.metadata.semio]",
    'role = "library"',
    "",
    "[lints]",
    "workspace = true",
    "",
    "[lib]",
    `path = "${spec.libPath}"`,
    "",
    "[dependencies]",
    ...[...spec.dependencyRows].sort((a, b) => compareBytes(manifestRowKey(a), manifestRowKey(b))),
    "",
  ];
  if (spec.devDependencyRows.length) lines.push("[dev-dependencies]", ...[...spec.devDependencyRows].sort((a, b) => compareBytes(manifestRowKey(a), manifestRowKey(b))), "");
  for (const section of spec.targetSections) lines.push(section.trimEnd(), "");
  return lines.join("\n").replace(/\n{3,}/gu, "\n\n").trimEnd() + "\n";
}

/** 🚪️ The `exclude-patterns` entry that hides a member directory from the repository's native member discovery, which skips any directory whose own path or whose ancestor matches a pattern; `undefined` when the member is visible. Cargo itself keeps an explicitly listed member even when `exclude` names it, so a hidden member builds with raw Cargo and is unknown to the launcher. */
export function hidingExcludePattern(member: string, patterns: readonly string[]): string | undefined {
  const segments = member.split("/");
  for (let depth = 1; depth <= segments.length; depth++) {
    const directory = segments.slice(0, depth).join("/");
    const hit = patterns.find((pattern) => new Bun.Glob(pattern).match(directory) || new Bun.Glob(pattern).match(`${directory}/`) || (pattern.endsWith("/**") && new Bun.Glob(pattern.slice(0, -3)).match(directory)));
    if (hit !== undefined) return hit;
  }
  return undefined;
}

/** 🚪️ Reads the exclusion list the repository's native discovery applies to a workspace manifest: `exclude-patterns` of `[workspace.metadata.semio.repository]`, else Cargo's `exclude`. */
function admittedExclusions(workspace: { exclude?: string[]; metadata?: { semio?: { repository?: { "exclude-patterns"?: string[] } } } }): string[] {
  const admission = workspace.metadata?.semio?.repository;
  return admission === undefined ? workspace.exclude ?? [] : admission["exclude-patterns"] ?? [];
}

/** 🚪️ Lists workspace members that the repository's native discovery cannot see because an `exclude-patterns` entry hides them; an empty list means Cargo and the launcher agree on the member set. */
export function workspaceAdmissionProblems(manifestText: string): string[] {
  const workspace = (Bun.TOML.parse(manifestText) as { workspace?: { members?: string[]; exclude?: string[]; metadata?: { semio?: { repository?: { "exclude-patterns"?: string[] } } } } }).workspace;
  if (!workspace) return ["manifest has no [workspace] table"];
  const patterns = admittedExclusions(workspace);
  return (workspace.members ?? []).flatMap((member) => { const hit = hidingExcludePattern(member, patterns); return hit === undefined ? [] : [`member ${member} is hidden from repository discovery by exclude pattern ${hit}`]; });
}

/** 🚪️ Removes from an array literal under `[workspace]` or its repository metadata every entry the predicate rejects, keeping the rest of the text untouched. */
function dropArrayEntries(manifestText: string, key: string, hidden: (entry: string) => boolean): string {
  const origin = Math.max(manifestText.indexOf("[workspace]"), 0);
  const found = new RegExp(`(\\n[ \\t]*${key}\\s*=\\s*)\\[([\\s\\S]*?)\\]`, "u").exec(manifestText.slice(origin));
  if (!found) return manifestText;
  const start = origin + found.index;
  const entries = [...found[2]!.matchAll(/"((?:[^"\\]|\\.)*)"/gu)].map((entry) => entry[1]!);
  const kept = entries.filter((entry) => !hidden(entry));
  if (kept.length === entries.length) return manifestText;
  const indent = /^\n(\s*)"/u.exec(found[2]!)?.[1] ?? "  ";
  const rendered = kept.length ? `[\n${kept.map((entry) => `${indent}"${entry}",`).join("\n")}\n]` : "[]";
  return manifestText.slice(0, start) + found[1] + rendered + manifestText.slice(start + found[0].length);
}

/** 🗂️ Adds members to the `[workspace] members` array in byte order and lifts the exclusion entries that would hide an added member from the repository's native discovery, keeping the rest of the manifest untouched. */
export function addWorkspaceMembers(manifestText: string, additions: readonly string[]): string {
  const match = /(\[workspace\][\s\S]*?\nmembers\s*=\s*)\[([\s\S]*?)\]/u.exec(manifestText);
  if (!match) throw new Error("Workspace manifest has no members array");
  const existing = [...match[2]!.matchAll(/"((?:[^"\\]|\\.)*)"/gu)].map((entry) => entry[1]!);
  const all = [...new Set([...existing, ...additions])].sort(compareBytes);
  const rendered = `[\n${all.map((entry) => `  "${entry}",`).join("\n")}\n]`;
  const widened = manifestText.slice(0, match.index) + match[1] + rendered + manifestText.slice(match.index + match[0].length);
  const workspace = (Bun.TOML.parse(widened) as { workspace: { exclude?: string[]; metadata?: { semio?: { repository?: { "exclude-patterns"?: string[] } } } } }).workspace;
  const hides = (pattern: string): boolean => additions.some((member) => hidingExcludePattern(member, [pattern]) !== undefined);
  const lifted = [...new Set([...(workspace.exclude ?? []), ...admittedExclusions(workspace)])].filter(hides);
  if (!lifted.length) return widened;
  return dropArrayEntries(dropArrayEntries(widened, "exclude", (entry) => lifted.includes(entry)), "exclude-patterns", (entry) => lifted.includes(entry));
}

/** 🔒️ Adds carved crates to the artifact lockfile and extends the artifact package's own dependency list. */
export function patchCargoLock(lockText: string, artifactCrate: string, carved: readonly { name: string; dependsOn: readonly string[] }[]): string {
  const blocks = lockText.split(/\n(?=\[\[package\]\]\n)/u);
  const header = blocks[0]!.startsWith("[[package]]") ? "" : blocks.shift()!;
  const parse = (block: string): { name: string; version: string; rest: string; dependencies: string[] } => {
    const name = /name = "([^"]+)"/u.exec(block)![1]!;
    const version = /version = "([^"]+)"/u.exec(block)?.[1] ?? "0.1.0";
    const dependencies = /dependencies = \[([\s\S]*?)\n\]/u.exec(block);
    const list = dependencies ? [...dependencies[1]!.matchAll(/"((?:[^"\\]|\\.)*)"/gu)].map((entry) => entry[1]!) : [];
    return { name, version, rest: block, dependencies: list };
  };
  const render = (name: string, version: string, dependencies: readonly string[]): string => `[[package]]\nname = "${name}"\nversion = "${version}"\n${dependencies.length ? `dependencies = [\n${[...dependencies].sort(compareBytes).map((entry) => ` "${entry}",`).join("\n")}\n]\n` : ""}`;
  const packages = blocks.map(parse);
  const artifact = packages.find((entry) => entry.name === artifactCrate);
  if (!artifact) throw new Error(`Lockfile has no package ${artifactCrate}`);
  const baseDependencies = artifact.dependencies;
  const out = new Map<string, string>();
  for (const entry of packages) out.set(entry.name, entry.rest.trimEnd() + "\n");
  for (const crate of carved) out.set(crate.name, render(crate.name, artifact.version, [...new Set([...baseDependencies.filter((name) => !carved.some((other) => other.name === name)), ...crate.dependsOn])]));
  out.set(artifactCrate, render(artifactCrate, artifact.version, [...new Set([...baseDependencies, ...carved.map((crate) => crate.name)])]));
  const ordered = [...out.entries()].sort((a, b) => compareBytes(a[0], b[0])).map((entry) => entry[1].trimEnd());
  return `${header}${ordered.join("\n\n")}\n`;
}

/** 📜️ Extends the package router's `packages` option so check and test cover every carved crate. */
export function patchRouterPackages(scriptText: string, artifactCrate: string, additions: readonly string[]): string {
  const call = new RegExp(`(runArtifactRustPackageMain\\([^,]+,\\s*"${artifactCrate.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}")\\s*(?:,\\s*(\\{))?`, "u").exec(scriptText);
  if (!call) throw new Error(`Router call for ${artifactCrate} not found`);
  const existing = /packages\s*:\s*\[([^\]]*)\]/u.exec(scriptText);
  const list = [...new Set([artifactCrate, ...(existing ? [...existing[1]!.matchAll(/"([^"]+)"/gu)].map((entry) => entry[1]!) : []), ...additions])].sort(compareBytes);
  const rendered = `packages: [${list.map((name) => `"${name}"`).join(", ")}]`;
  if (existing) return scriptText.replace(/packages\s*:\s*\[[^\]]*\]/u, rendered);
  if (call[2]) return scriptText.slice(0, call.index + call[0].length) + ` ${rendered},` + scriptText.slice(call.index + call[0].length);
  return scriptText.slice(0, call.index + call[1]!.length) + `, { ${rendered} }` + scriptText.slice(call.index + call[1]!.length);
}

/** 🗂️ One build-owner row of a carved crate. */
export type BuildOwnerRow = { readonly name: string; readonly root: string };

/** 🗂️ Inserts the owner rows of new crates into `📋️build-owners.json` text at their byte-order position; rows whose name or root is already registered are skipped. */
export function insertBuildOwnerRows(raw: string, rows: readonly BuildOwnerRow[]): { text: string; added: string[] } {
  const document = JSON.parse(raw) as { owners: { name: string; root: string; manifest: string; packages: string[]; [key: string]: unknown }[] };
  const known = new Set(document.owners.flatMap((owner) => [owner.name, owner.root]));
  const added: string[] = [];
  for (const row of [...rows].sort((left, right) => compareBytes(left.name, right.name))) {
    if (known.has(row.name) || known.has(row.root)) continue;
    const entry = { name: row.name, root: row.root, manifest: `${row.root}/Cargo.toml`, packages: [] as string[] };
    const at = document.owners.findIndex((owner) => compareBytes(owner.name, row.name) > 0);
    document.owners.splice(at < 0 ? document.owners.length : at, 0, entry);
    known.add(row.name);
    known.add(row.root);
    added.push(row.name);
  }
  return { text: JSON.stringify(document, null, 2) + "\n", added };
}

/** 🧲️ Rebases the file arguments of `include_str!`, `include_bytes!` and `include!` of moved text from the directory it came from to the directory it now lives in. */
export function rebaseRustIncludes(text: string, fromDirectory: string, toDirectory: string): string {
  if (fromDirectory === toDirectory) return text;
  const tokens = rustTokens(text);
  const edits: { start: number; end: number; text: string }[] = [];
  for (let k = 0; k + 3 < tokens.length; k += 1) {
    if (!["include_str", "include_bytes", "include"].includes(tokens[k]!.text) || tokens[k + 1]!.text !== "!" || tokens[k + 2]!.text !== "(" || tokens[k + 3]!.kind !== "string") continue;
    const literal = tokens[k + 3]!.text;
    if (!literal.startsWith('"')) continue;
    const relative = JSON.parse(literal) as string;
    edits.push({ start: tokens[k + 3]!.start, end: tokens[k + 3]!.end, text: JSON.stringify(relativePosix(toDirectory, resolve(fromDirectory, relative))) });
  }
  let out = text;
  for (const edit of edits.sort((left, right) => right.start - left.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return out;
}

/** 🧲️ Rebases the string literals that are joined onto `env!("CARGO_MANIFEST_DIR")` (in `concat!` or in `Path::join`) from the manifest directory a file was written for to the one it now compiles under. */
export function rebaseManifestDirPaths(text: string, fromManifestDirectory: string, toManifestDirectory: string): string {
  if (fromManifestDirectory === toManifestDirectory || !text.includes("CARGO_MANIFEST_DIR")) return text;
  const tokens = rustTokens(text);
  const edits: { start: number; end: number; text: string }[] = [];
  for (let k = 0; k + 5 < tokens.length; k += 1) {
    if (tokens[k]!.text !== "env" || tokens[k + 1]!.text !== "!" || tokens[k + 2]!.text !== "(" || tokens[k + 3]!.text !== '"CARGO_MANIFEST_DIR"' || tokens[k + 4]!.text !== ")") continue;
    let literal = -1;
    let leading = false;
    if (tokens[k + 5]!.text === "," && tokens[k + 6]?.kind === "string") { literal = k + 6; leading = true; }
    else if (tokens[k + 5]!.text === ")" && tokens[k + 6]?.text === "." && tokens[k + 7]?.text === "join" && tokens[k + 8]?.text === "(" && tokens[k + 9]?.kind === "string") literal = k + 9;
    if (literal < 0 || !tokens[literal]!.text.startsWith('"')) continue;
    const value = JSON.parse(tokens[literal]!.text) as string;
    const target = resolve(fromManifestDirectory, leading ? `.${value}` : value);
    const next = relativePosix(toManifestDirectory, target);
    edits.push({ start: tokens[literal]!.start, end: tokens[literal]!.end, text: JSON.stringify(leading ? `/${next}` : next) });
  }
  let out = text;
  for (const edit of edits.sort((left, right) => right.start - left.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return out;
}

export { dirname, posix };
