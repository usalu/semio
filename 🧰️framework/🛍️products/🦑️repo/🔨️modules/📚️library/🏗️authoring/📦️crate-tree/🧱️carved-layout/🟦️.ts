/** ♻️ Carved layout: reads an already carved artifact back into the declared mount tree the carve planner starts from, so a re-carve replans the carved crates instead of mistaking their wiring for the artifact. */
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { collectDecls } from "../🧱️decl-tree/🟦️.ts";
import { parseRustSourceTree, type RustModuleNode, type RustSourceItem } from "../🌳️module-tree/🟦️.ts";
import { crateIdent } from "../🗺️plan/🏷️naming/🟦️.ts";
import { readCargoLib } from "../🗺️plan/📦️cargo-lib/🟦️.ts";
import { rewriteMountPaths } from "../📝️render/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";

/** 📦️ A crate of a carved artifact: the declarations it owns by effective path and the sibling crates it depends on. `owned` are the declarations the mirror root attaches, `recorded` every declaration the carve recorded in the manifest (absent for crates carved before the record existed, whose nested mounts then count as owned); `signed` crates carry the carve's doc line and keep their identity on a re-carve, the others (hand-written) only contribute their mounts. */
export type CarvedCrate = Readonly<{ name: string; ident: string; packageDir: string; description: string; owned: readonly string[]; recorded: readonly string[] | null; dependsOn: readonly string[]; signed: boolean }>;

/** 🧬️ The declared state of a carved artifact: its crates, the root source and the file texts of the mount tree the carve was cut from, and the package manifest without the carve's wiring. */
export type CarvedLayout = Readonly<{ crates: readonly CarvedCrate[]; rootSource: string; overlay: ReadonlyMap<string, string>; uncarvedManifest: string; notes: readonly string[] }>;

const CARVE_SIGNATURE = "path-preserving carve of the artifact root mounts";
const USE_HEAD = /^(?:\s*#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?([A-Za-z_][A-Za-z0-9_]*)/u;
const USE_PATH = /^(?:\s*#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?([A-Za-z_][A-Za-z0-9_]*)((?:::[A-Za-z_][A-Za-z0-9_]*)*)(::\*|::\{[^}]*\})?\s*;\s*$/u;
const COMPONENT_GLOB = /^(?:\s*#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+component::\*\s*;\s*$/u;
const REGION = /\n?\/\/#region (🧩️ArtifactItems\d*)\n([\s\S]*?)\n\/\/#endregion \1\n?/gu;

type Decl = { source: string; node: RustModuleNode; scope: string };
type Nest = { name: string; header: string | null; scope: string; items: string[]; leaf: Decl | null; component: Decl | null; file: string | null; componentFile: string | null; extended: boolean; children: Map<string, Nest> };

const nestOf = (name: string, scope: string): Nest => ({ name, header: null, scope, items: [], leaf: null, component: null, file: null, componentFile: null, extended: false, children: new Map() });
const fileOf = (nest: Nest | null | undefined): string | null => nest?.componentFile ?? nest?.file ?? null;
const mount = (decl: Decl, scope: string): string => rewriteMountPaths(decl.source, decl.node, decl.scope, scope, false);
const indented = (text: string, indent: string): string => text.split("\n").map((line, index) => (index === 0 || !line.length ? line : indent + line)).join("\n");
const headOf = (statement: string): string | null => USE_HEAD.exec(statement)?.[1] ?? null;
const lineStartOf = (text: string, at: number): number => { const start = text.lastIndexOf("\n", at - 1) + 1; return /^[ \t]*$/u.test(text.slice(start, at)) ? start : at; };
const lineEndOf = (text: string, at: number): number => { let end = at; while (text[end] === " " || text[end] === "\t") end += 1; return text[end] === "\n" ? end + 1 : at; };

function applyEdits(text: string, edits: readonly { start: number; end: number; text: string }[]): string {
  let out = text;
  for (const edit of [...edits].sort((a, b) => b.start - a.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return out;
}

class Collector {
  readonly root = nestOf("", "");
  readonly owned = new Map<string, Set<string>>();
  readonly roots = new Map<string, Set<string>>();
  private current = new Set<string>();
  constructor(private readonly idents: ReadonlySet<string>) {}

  derived(statement: string): boolean {
    const head = headOf(statement);
    return head !== null && (this.idents.has(head) || (head === "crate" && /::artifact_items(?:_\d+)?::/u.test(statement)));
  }

  collect(crate: string, source: string, scope: string): void {
    const owned = this.owned.get(crate) ?? new Set<string>();
    this.owned.set(crate, owned);
    this.current = this.roots.get(crate) ?? new Set<string>();
    this.roots.set(crate, this.current);
    const tree = parseRustSourceTree(source);
    this.walk(source, tree.modules, tree.items, scope, this.root, [], owned, true);
  }

  private walk(source: string, nodes: readonly RustModuleNode[], items: readonly RustSourceItem[], scope: string, nest: Nest, path: readonly string[], owned: Set<string>, top: boolean): void {
    const hasGlob = items.some((item) => item.kind === "use" && COMPONENT_GLOB.test(source.slice(item.start, item.end)));
    let authored = false;
    for (const item of items) {
      const statement = source.slice(item.start, item.end);
      if (item.kind === "use" && this.derived(statement)) { const parsed = parseUse(statement); if (parsed?.glob && parsed.path.length && parsed.path.join("::") === path.join("::")) nest.extended = true; }
      if (item.kind === "use" && (this.derived(statement) || COMPONENT_GLOB.test(statement))) continue;
      if (!nest.items.includes(statement)) nest.items.push(statement);
      if (!top) authored = true;
    }
    if (authored) owned.add(path.join("::"));
    for (const node of nodes) {
      if (node.inline) {
        const inner = resolve(scope, node.pathAttribute ?? node.name);
        const child = nest.children.get(node.name) ?? nestOf(node.name, inner);
        nest.children.set(node.name, child);
        child.header ??= source.slice(node.start, node.bodyStart).split("\n").map((line) => line.trim()).join("\n");
        this.walk(source, node.children, node.items, inner, child, [...path, node.name], owned, false);
      } else if (node.pathAttribute !== null) {
        const file = resolve(scope, node.pathAttribute);
        if (node.name === "component" && hasGlob) {
          if (nest.component === null) { nest.component = { source, node, scope }; nest.componentFile = file; }
          this.current.add(file);
          owned.add(path.join("::"));
          continue;
        }
        const child = nest.children.get(node.name) ?? nestOf(node.name, scope);
        nest.children.set(node.name, child);
        if (child.leaf === null) { child.leaf = { source, node, scope }; child.file = file; }
        this.current.add(file);
        owned.add([...path, node.name].join("::"));
      }
    }
  }
}

type Parsed = { ident: string; path: string[]; glob: boolean; names: string[] };

function parseUse(statement: string): Parsed | null {
  const match = USE_PATH.exec(statement);
  if (!match) return null;
  const path = match[2]!.split("::").filter(Boolean);
  return { ident: match[1]!, path, glob: match[3] === "::*", names: match[3]?.startsWith("::{") ? match[3].slice(3, -1).split(",").map((name) => name.trim()).filter(Boolean) : [] };
}

class Rebuilder {
  readonly appends = new Map<string, string[]>();
  readonly placed = new Set<Nest>();
  notes: string[] = [];
  constructor(private readonly root: Nest, private readonly collector: Collector) {}

  nodeAt(path: readonly string[]): Nest | null {
    let cursor: Nest | null = this.root;
    for (const segment of path) cursor = cursor?.children.get(segment) ?? null;
    return cursor;
  }

  parentFile(path: readonly string[]): string | null { return fileOf(this.nodeAt(path.slice(0, -1))); }

  wrapper(node: Nest, scope: string): string {
    this.placed.add(node);
    const own = /path\s*=\s*"([^"]*)"/u.exec(node.header ?? "")?.[1] ?? node.name;
    const inner = resolve(scope, own);
    const lines = [...(node.header ?? `#[path = "."]\npub mod ${node.name} {`).split("\n")];
    const body: string[] = [];
    if (node.component) body.push(mount(node.component, inner), "pub use component::*;");
    body.push(...node.items);
    for (const child of node.children.values()) {
      this.placed.add(child);
      if ((child.leaf && fileOf(node) !== null) || (child.extended && !child.leaf && !child.component)) { this.cover(child); continue; }
      body.push(child.leaf ? mount(child.leaf, inner) : this.wrapper(child, inner));
    }
    return [...lines, ...body.map((line) => indented(line, "    ")).map((line) => `    ${line}`), "}"].join("\n");
  }

  decl(path: readonly string[], scope: string): string {
    const node = this.nodeAt(path);
    if (!node) return "";
    this.placed.add(node);
    if (node.leaf) return this.parentFile(path) !== null || this.nodeAt(path.slice(0, -1))?.extended ? "" : mount(node.leaf, scope);
    if (node.extended && !node.component) { node.children.forEach((child) => this.cover(child)); return ""; }
    if (node.header !== null) return this.wrapper(node, scope);
    this.cover(node);
    return "";
  }

  cover(node: Nest): void {
    this.placed.add(node);
    node.children.forEach((child) => this.cover(child));
  }

  statement(parsed: Parsed, scope: string): string {
    if (parsed.glob) {
      const node = this.nodeAt(parsed.path);
      if (node) this.placed.add(node);
      return node?.component ? `${mount(node.component, scope)}\npub use component::*;` : "";
    }
    if (parsed.names.length) return parsed.names.map((name) => this.decl([...parsed.path, name], scope)).filter(Boolean).join("\n");
    return parsed.path.length ? this.decl(parsed.path, scope) : "";
  }

  appendFile(file: string, text: string): void { this.appends.set(file, [...(this.appends.get(file) ?? []), text]); }

  readonly deferred: { path: string[]; leaf: Decl; node: Nest }[] = [];

  filePass(node: Nest, path: readonly string[]): void {
    const file = fileOf(node);
    for (const child of node.children.values()) {
      if (file !== null && child.leaf) { this.placed.add(child); this.appendFile(file, mount(child.leaf, dirname(file))); }
      else if (node.extended && child.leaf) this.deferred.push({ path: [...path], leaf: child.leaf, node: child });
      this.filePass(child, [...path, child.name]);
    }
  }

  fullyDerived(source: string, node: RustModuleNode): boolean {
    return node.items.every((item) => item.kind === "use" && this.collector.derived(source.slice(item.start, item.end))) && node.children.every((child) => child.inline && this.fullyDerived(source, child));
  }

  facade(source: string, rootDir: string): string {
    const edits: { start: number; end: number; text: string }[] = [];
    const visit = (nodes: readonly RustModuleNode[], items: readonly RustSourceItem[], scope: string, here: readonly string[]): void => {
      for (const item of items) {
        const text = source.slice(item.start, item.end);
        if (item.kind !== "use" || !this.collector.derived(text)) continue;
        const parsed = parseUse(text);
        const start = lineStartOf(source, item.start);
        const column = source.slice(start, item.start);
        const made = parsed ? this.statement(parsed, scope) : "";
        edits.push({ start, end: lineEndOf(source, item.end), text: made ? `${column}${indented(made, column)}\n` : "" });
      }
      for (const node of nodes) {
        if (!node.inline) continue;
        const path = [...here, node.name];
        const inner = resolve(scope, node.pathAttribute ?? node.name);
        const target = this.nodeAt(path);
        if (target !== null) this.placed.add(target);
        const own = (item: RustSourceItem): Parsed | null => { const text = source.slice(item.start, item.end); return item.kind === "use" && this.collector.derived(text) ? parseUse(text) : null; };
        if (target !== null && target.header !== null && node.items.some((item) => own(item)?.glob === true && own(item)!.path.join("::") === path.join("::")) && this.fullyDerived(source, node)) {
          const start = lineStartOf(source, node.start);
          const column = source.slice(start, node.start);
          const text = this.decl(path, scope);
          edits.push({ start, end: lineEndOf(source, node.end), text: text ? `${column}${indented(text, column)}\n` : "" });
          continue;
        }
        visit(node.children, node.items, inner, path);
      }
    };
    const tree = parseRustSourceTree(source);
    visit(tree.modules, tree.items, rootDir, []);
    return applyEdits(source, edits);
  }
}

function dependencyRows(manifestText: string): Map<string, { path?: string }> {
  const parsed = runtimeParseTomlV1(manifestText) as { dependencies?: Record<string, { path?: string } | string> };
  return new Map(Object.entries(parsed.dependencies ?? {}).filter((entry): entry is [string, { path?: string }] => typeof entry[1] === "object" && typeof entry[1].path === "string"));
}

/** 🧹️ The package manifest with the carve's wiring taken out again: the dependency and dev-dependency rows of the carved crates and the feature entries that forward to them, so the planner sees the manifest the carve started from. */
export function uncarvedManifestText(manifestText: string, carved: ReadonlySet<string>): string {
  const keep = (line: string): boolean => { const key = /^\s*([A-Za-z0-9_-]+)\s*=/u.exec(line)?.[1]; return key === undefined || !carved.has(key); };
  let section = "";
  const lines = manifestText.split("\n").filter((line) => {
    const header = /^\s*\[{1,2}([^\]]+)\]{1,2}\s*$/u.exec(line);
    if (header) section = header[1]!.trim();
    return header !== null || !(section === "dependencies" || section === "dev-dependencies") || keep(line);
  });
  const forwarded = new RegExp(`"(?:${[...carved].map((name) => name.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join("|")})\\??/[^"]*"`, "u");
  return lines.map((line) => {
    if (!carved.size || !/^\s*[A-Za-z0-9_-]+\s*=\s*\[.*\]\s*$/u.test(line) || !forwarded.test(line)) return line;
    const name = /^\s*([A-Za-z0-9_-]+)/u.exec(line)![1]!;
    const entries = [...line.matchAll(/"((?:[^"\\]|\\.)*)"/gu)].map((match) => match[1]!).filter((entry) => !forwarded.test(`"${entry}"`));
    return `${name} = [${entries.map((entry) => JSON.stringify(entry)).join(", ")}]`;
  }).join("\n");
}

/** ♻️ Reads a carved artifact back: the mount tree every carved crate declares is merged, the carve's re-exports are replaced by the declarations they stand for (children whose parent file lost their mount get it back in that file), moved root items return to the root, and the package manifest loses the carve's wiring. Returns `null` when no crate of the artifact is carved. */
export function readCarvedLayout(repoRoot: string, packageDirRel: string): CarvedLayout | null {
  const manifestPath = resolve(repoRoot, packageDirRel, "Cargo.toml");
  const manifestText = readFileSync(manifestPath, "utf8");
  const lib = readCargoLib(manifestPath);
  const workspace = (runtimeParseTomlV1(manifestText) as { package?: { workspace?: string } }).package?.workspace;
  const artifactDir = resolve(dirname(manifestPath), workspace ?? ".");
  const providers: { name: string; dir: string; manifest: string; entry: string; text: string }[] = [...dependencyRows(manifestText)].flatMap(([name, spec]) => {
    const dir = resolve(dirname(manifestPath), spec.path!);
    const manifest = resolve(dir, "Cargo.toml");
    if (!dir.startsWith(`${artifactDir}/`) || dir === dirname(manifestPath) || !existsSync(manifest)) return [];
    const entry = readCargoLib(manifest).entry;
    return existsSync(entry) ? [{ name, dir, manifest, entry, text: readFileSync(entry, "utf8") }] : [];
  });
  const signed = new Set(providers.filter((provider) => provider.text.includes(CARVE_SIGNATURE)).map((provider) => provider.name));
  if (!signed.size) return null;
  const idents = new Set(providers.map((provider) => crateIdent(provider.name)));
  const collector = new Collector(idents);
  providers.splice(0, providers.length, ...providers.filter((provider) => signed.has(provider.name)));
  const ordered = [...providers.filter((provider) => signed.has(provider.name)), ...providers.filter((provider) => !signed.has(provider.name))];
  const bodyOf = (provider: { entry: string; text: string }): { text: string; scope: string } => {
    const hoisted = /#\[path\s*=\s*"([^"]*)"\]\s*mod\s+mirror\s*;/u.exec(provider.text);
    const body = hoisted ? resolve(dirname(provider.entry), hoisted[1]!) : null;
    return body !== null && existsSync(body) ? { text: readFileSync(body, "utf8"), scope: dirname(body) } : { text: provider.text, scope: dirname(provider.entry) };
  };
  for (const provider of ordered) { const body = bodyOf(provider); collector.collect(provider.name, body.text, body.scope); }
  const rootDir = dirname(lib.entry);
  const rebuilder = new Rebuilder(collector.root, collector);
  rebuilder.filePass(collector.root, []);
  const overlay = new Map<string, string>();
  const notes: string[] = [];
  const regions: { text: string; uses: string[] }[] = [];
  const files = new Set<string>();
  const gather = (node: Nest): void => { for (const file of [node.file, node.componentFile]) if (file) files.add(file); node.children.forEach(gather); };
  gather(collector.root);
  for (const file of files) {
    if (!existsSync(file)) continue;
    let text = readFileSync(file, "utf8");
    let changed = false;
    text = text.replace(REGION, (_, _label: string, body: string) => {
      changed = true;
      const inner = /^pub mod artifact_items(?:_\d+)? \{\n([\s\S]*)\n\}$/u.exec(body)?.[1] ?? body;
      const tree = parseRustSourceTree(inner);
      const uses = tree.items.filter((item) => item.kind === "use").map((item) => inner.slice(item.start, item.end));
      const crossed = uses.filter((statement) => /^use\s+crate::\{[^}]*\};$/u.test(statement.trim()));
      const kept = applyEdits(inner, tree.items.filter((item) => item.kind === "use").map((item) => ({ start: lineStartOf(inner, item.start), end: lineEndOf(inner, item.end), text: "" })));
      regions.push({ text: kept.trim(), uses: uses.filter((statement) => !crossed.includes(statement)) });
      return "";
    });
    if (changed) overlay.set(file, text);
  }
  let rootSource = rebuilder.facade(readFileSync(lib.entry, "utf8"), rootDir);
  const appendTo = (file: string, text: string): void => {
    const base = overlay.get(file) ?? readFileSync(file, "utf8");
    overlay.set(file, `${base}${base.endsWith("\n") ? "" : "\n"}${text}\n`);
  };
  for (const [file, texts] of rebuilder.appends) appendTo(file, texts.join("\n"));
  const read = (absolute: string): string | null => overlay.get(absolute) ?? (existsSync(absolute) ? readFileSync(absolute, "utf8") : null);
  for (let pending = [...rebuilder.deferred]; pending.length;) {
    const decls = [...collectDecls(lib.entry, rootSource, read).decls.values()];
    const ready = pending.filter((entry) => decls.some((decl) => decl.file !== null && decl.eff.join("::") === entry.path.join("::")));
    if (!ready.length) break;
    for (const entry of ready) {
      const owner = decls.find((decl) => decl.file !== null && decl.eff.join("::") === entry.path.join("::"))!;
      rebuilder.placed.add(entry.node);
      appendTo(owner.file!, mount(entry.leaf, dirname(owner.file!)));
    }
    pending = pending.filter((entry) => !ready.includes(entry));
  }
  const heads = new RegExp(`(?<![A-Za-z0-9_:])(?:${[...idents].map((ident) => ident.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join("|")})::`, "gu");
  const stripDerived = (text: string): string => {
    const tree = parseRustSourceTree(text);
    const edits: { start: number; end: number; text: string }[] = [];
    const isDerived = (item: RustSourceItem): boolean => item.kind === "use" && collector.derived(text.slice(item.start, item.end));
    const fully = (node: RustModuleNode): boolean => (node.items.length > 0 || node.children.length > 0) && node.items.every(isDerived) && node.children.every((child) => child.inline && fully(child));
    const visit = (nodes: readonly RustModuleNode[], items: readonly RustSourceItem[]): void => {
      for (const item of items) if (isDerived(item)) edits.push({ start: lineStartOf(text, item.start), end: lineEndOf(text, item.end), text: "" });
      for (const node of nodes) if (node.inline && fully(node)) edits.push({ start: lineStartOf(text, node.start), end: lineEndOf(text, node.end), text: "" }); else if (node.inline) visit(node.children, node.items);
    };
    visit(tree.modules, tree.items);
    return applyEdits(text, edits);
  };
  const repoint = (text: string): string => stripDerived(text).replace(heads, "crate::");
  if (idents.size) {
    for (const decl of collectDecls(lib.entry, rootSource, read).decls.values()) {
      if (decl.file === null) continue;
      const text = read(decl.file);
      if (text !== null && repoint(text) !== text) overlay.set(decl.file, repoint(text));
    }
    rootSource = repoint(rootSource);
  }
  const rootTree = parseRustSourceTree(rootSource);
  const present = new Set(rootTree.items.map((item) => rootSource.slice(item.start, item.end).trim()));
  const additions = [...collector.root.items, ...regions.flatMap((region) => region.uses)].filter((statement) => !present.has(statement.trim()) && !/^(?:\s*#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+crate::\{[^}]*\};$/u.test(statement.trim()));
  const insertAt = Math.min(...[...rootTree.items, ...rootTree.modules].map((node) => node.start), rootSource.length);
  const unique = [...new Set(additions.map((statement) => statement.trim()))];
  const leftovers: Nest[] = [];
  const touched = (node: Nest): boolean => rebuilder.placed.has(node) || [...node.children.values()].some(touched);
  const scan = (node: Nest, parent: Nest): void => { if (!touched(node) && !(node.leaf && (fileOf(parent) !== null || node.leaf.node.visibility === "private"))) { leftovers.push(node); return; } node.children.forEach((child) => scan(child, node)); };
  collector.root.children.forEach((child) => scan(child, collector.root));
  const tail = [...regions.map((region) => region.text), ...leftovers.map((node) => rebuilder.wrapper(node, rootDir))];
  if (leftovers.length) notes.push(`${leftovers.length} carved module(s) had no re-export in the root and were mounted at its end`);
  rootSource = `${rootSource.slice(0, insertAt)}${unique.length ? `${unique.join("\n")}\n\n` : ""}${rootSource.slice(insertAt)}`;
  rootSource = `${rootSource.trimEnd()}\n${tail.length ? `\n${tail.join("\n\n")}\n` : ""}`;
  const mounted = (file: string, seen: Set<string>): Set<string> => {
    if (seen.has(file) || !existsSync(file)) return seen;
    seen.add(file);
    const visit = (nodes: readonly RustModuleNode[], scope: string): void => {
      for (const node of nodes) {
        if (node.inline) visit(node.children, resolve(scope, node.pathAttribute ?? node.name));
        else if (node.pathAttribute !== null) mounted(resolve(scope, node.pathAttribute), seen);
      }
    };
    visit(parseRustSourceTree(readFileSync(file, "utf8")).modules, dirname(file));
    return seen;
  };
  const decls = [...collectDecls(lib.entry, rootSource, read).decls.values()];
  const closureOf = (name: string): Set<string> => [...(collector.roots.get(name) ?? [])].reduce((seen, file) => mounted(file, seen), new Set<string>());
  const closures = new Map(providers.filter((provider) => signed.has(provider.name)).map((provider) => [provider.name, closureOf(provider.name)] as const));
  const manifestDeps = new Map(providers.map((provider) => [provider.name, Object.keys((runtimeParseTomlV1(readFileSync(provider.manifest, "utf8")) as { dependencies?: Record<string, unknown> }).dependencies ?? {}).filter((key) => providers.some((other) => other.name === key))] as const));
  const reaches = (from: string, to: string, seen = new Set<string>()): boolean => from !== to && !seen.has(from) && (seen.add(from), (manifestDeps.get(from) ?? []).some((next) => next === to || reaches(next, to, seen)));
  const ownerOfFile = (file: string): string | undefined => [...closures].filter(([, closure]) => closure.has(file)).sort(([a, left], [b, right]) => Number(reaches(a, b)) - Number(reaches(b, a)) || Number(collector.roots.get(b)?.has(file) ?? false) - Number(collector.roots.get(a)?.has(file) ?? false) || left.size - right.size)[0]?.[0];
  const declarers = new Map<string, string[]>();
  for (const [name, effs] of collector.owned) if (signed.has(name)) for (const eff of effs) declarers.set(eff, [...(declarers.get(eff) ?? []), name]);
  const ownerOfDecl = (decl: { eff: readonly string[]; file: string | null; declFile: string }): string | undefined => { const only = declarers.get(decl.eff.join("::")); return only?.length === 1 ? only[0] : ownerOfFile(decl.file ?? decl.declFile); };
  const declaredElsewhere = new Set(decls.filter((decl) => ownerOfFile(decl.file ?? decl.declFile) !== undefined).map((decl) => decl.eff.join("::")));
  const assigned = new Map<string, string[]>();
  for (const decl of decls) { const owner = ownerOfDecl(decl); if (owner) assigned.set(owner, [...(assigned.get(owner) ?? []), decl.eff.join("::")]); }
  const crates = providers.map((provider): CarvedCrate => {
    const derived = [...new Set(assigned.get(provider.name) ?? [])].sort();
    const parsed = runtimeParseTomlV1(readFileSync(provider.manifest, "utf8")) as { package?: { description?: string; metadata?: { semio?: { carve?: { owned?: string[] } } } }; dependencies?: Record<string, unknown> };
    return { recorded: parsed.package?.metadata?.semio?.carve?.owned?.slice().sort() ?? (signed.has(provider.name) ? [...new Set([...derived, ...[...(collector.owned.get(provider.name) ?? [])].filter((eff) => !declaredElsewhere.has(eff))])].sort() : null), name: provider.name, ident: crateIdent(provider.name), packageDir: relativeTo(repoRoot, provider.dir), description: parsed.package?.description ?? "", owned: [...(collector.owned.get(provider.name) ?? [])].sort(), dependsOn: Object.keys(parsed.dependencies ?? {}).filter((key) => providers.some((other) => other.name === key)).sort(), signed: signed.has(provider.name) };
  });
  return { crates, rootSource, overlay, uncarvedManifest: uncarvedManifestText(manifestText, new Set(providers.map((provider) => provider.name))), notes };
}

function relativeTo(root: string, path: string): string {
  return path.slice(root.length).replace(/^[\\/]+/u, "").replaceAll("\\", "/");
}
