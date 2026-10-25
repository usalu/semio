/** 🪞️ Unmirror: removes the path-preserving re-export layers a carve leaves behind. Every `pub use` that forwards another carved crate's module or item under its old path is deleted, and every consumer (private imports, globs, inline paths, other crates) is re-pointed at the crate that owns the item. */
import { parseRustSourceTree, type RustModuleNode } from "../🌳️module-tree/🟦️.ts";
import { Universe, isKeywordHead, repText, type Bound, type FileInfo, type Hit, type ModNode, type Rep } from "./🔍️resolver/🟦️.ts";
import { leafBinding, renderUseLeaves, type ScannedSource, type UseLeaf, type UseStatement } from "./🧵️use-tree/🟦️.ts";

/** ⚙️ Options of one unmirror run: the crates whose root-level explicit re-exports are the designed artifact API, the crates being unmirrored (their re-export edges are deleted; other files only re-point paths that start in one of them) and which files may be edited. */
export type UnmirrorOptions = Readonly<{ apiRoots: ReadonlySet<string>; targets: ReadonlySet<string>; editable: (path: string) => boolean }>;

/** 🧾️ What a run changed. */
export type UnmirrorReport = {
  edits: Map<string, string>;
  removedLeaves: number;
  repointedLeaves: number;
  repointedChains: number;
  addedImports: number;
  removedModules: number;
  unresolvedMirrors: string[];
  multiMounted: string[];
  invisible: string[];
};

type Edit = { start: number; end: number; text: string };
type Planned = { leaf: UseLeaf; source: Bound | null; visibility?: string };

const isPublic = (visibility: string): boolean => visibility === "pub";
const samePath = (left: readonly string[], right: readonly string[]): boolean => left.length === right.length && left.every((segment, index) => segment === right[index]);

/** 📏️ Membership test over disjoint-or-nested character ranges. */
function rangeIndex(ranges: readonly (readonly [number, number])[]): (offset: number) => boolean {
  const sorted = [...ranges].sort((left, right) => left[0] - right[0]);
  return (offset) => {
    let low = 0;
    let high = sorted.length - 1;
    while (low <= high) {
      const middle = (low + high) >> 1;
      const [start, end] = sorted[middle]!;
      if (offset < start) high = middle - 1;
      else if (offset >= end) low = middle + 1;
      else return true;
    }
    return sorted.some(([start, end]) => start <= offset && offset < end);
  };
}

/** 🧩️ The character ranges of the argument lists of macro invocations (`name!(…)`, `name![…]`, `name!{…}`); a `macro_rules!` definition is no invocation. */
function macroInvocationRanges(source: ScannedSource): (readonly [number, number])[] {
  const { tokens, pairs } = source;
  const ranges: (readonly [number, number])[] = [];
  for (let index = 1; index + 1 < tokens.length; index += 1) {
    const close = pairs.get(index + 1);
    if (tokens[index]!.text !== "!" || tokens[index - 1]!.kind !== "identifier" || tokens[index - 1]!.text === "macro_rules" || close === undefined) continue;
    ranges.push([tokens[index + 1]!.start, tokens[close]!.end]);
  }
  return ranges;
}

/** ▶️ Drives one unmirror run over the files of a universe. */
export class Unmirror {
  readonly report: UnmirrorReport = { edits: new Map(), removedLeaves: 0, repointedLeaves: 0, repointedChains: 0, addedImports: 0, removedModules: 0, unresolvedMirrors: [], multiMounted: [], invisible: [] };
  constructor(private readonly universe: Universe, private readonly options: UnmirrorOptions) {
    universe.apiLeaf = (bound) => this.isApi(bound);
    universe.cutEdge = (bound) => this.isMirrorEdge(bound);
  }

  private foreignHead(bound: Bound): boolean {
    const head = bound.leaf.path[0];
    return head !== undefined && !isKeywordHead(head) && this.universe.crates.has(head) && head !== bound.node.crate.ident && this.universe.member(bound.node, head, bound.node).length === 0;
  }

  private crossing(bound: Bound, hit: Hit | undefined): Rep | null {
    return hit?.rep?.cut && hit.rep.ident !== bound.node.crate.ident ? hit.rep : null;
  }

  private isApi(bound: Bound): boolean {
    const node = bound.node;
    const root = node.parent === null || (node.name === "mirror" && node.parent.parent === null);
    return root && !bound.leaf.glob && isPublic(bound.statement.visibility) && this.options.apiRoots.has(node.crate.ident);
  }

  private primary(hits: readonly Hit[], value = false): Hit | undefined {
    const chosen = hits.find((hit) => hit.rep?.cut && (hit.kind === "item") === value) ?? hits.find((hit) => hit.rep?.cut) ?? hits[0];
    if (chosen?.kind !== "module") return chosen;
    const forward = this.forwardOf(chosen.node);
    return forward ? ({ ...chosen, rep: forward } as Hit) : chosen;
  }

  private isMirrorEdge(bound: Bound): boolean {
    return this.universe.cycles.compute(`edge:${bound.key}`, () => false, () => this.computeMirrorEdge(bound));
  }

  private forwardOf(node: ModNode): Rep | null {
    return this.universe.cycles.compute(`forward:${node.id}`, () => null, () => this.computeForward(node));
  }

  private computeForward(node: ModNode): Rep | null {
    if (!node.inline || node.uses.length === 0 || this.ownNamespace(node)) return null;
    const reps = new Set<string>();
    let found: Rep | null = null;
    for (const bound of node.uses) {
      if (!this.isMirrorEdge(bound)) continue;
      const hit = this.primary(this.universe.leafHits(bound));
      if (hit?.kind !== "module" || !hit.rep) return null;
      reps.add(repText(hit.rep));
      found = { ...hit.rep, cut: true };
    }
    return reps.size === 1 ? found : null;
  }

  private privateShim(node: ModNode): boolean {
    for (let cursor: ModNode | null = node; cursor?.parent; cursor = cursor.parent) if (cursor.visibility !== "") return false;
    return true;
  }

  private computeMirrorEdge(bound: Bound): boolean {
    if (!isPublic(bound.statement.visibility) || this.isApi(bound)) return false;
    if (!this.options.targets.has(bound.node.crate.ident)) {
      const through = this.primary(this.universe.leafHits(bound));
      if (!through || !this.privateShim(bound.node)) return false;
      return through.rep?.cut === true || (through.kind === "module" && through.node.inline && this.options.targets.has(through.node.crate.ident) && !this.ownNamespace(through.node));
    }
    const hit = this.primary(this.universe.leafHits(bound));
    if (hit?.kind === "module" && hit.node.inline && (hit.node.uses.length > 0 || hit.node.children.size > 0) && !this.ownNamespace(hit.node)) return true;
    return hit !== undefined && (this.crossing(bound, hit) !== null || this.foreignHead(bound));
  }

  private ownNamespace(node: ModNode): boolean {
    return this.universe.cycles.compute(`own:${node.id}`, () => false, () => {
      if (node.items.size > 0) return true;
      for (const children of node.children.values()) for (const child of children) if ((!child.test || node.test) && (!child.inline || this.ownNamespace(child))) return true;
      return node.uses.some((bound) => !this.isMirrorEdge(bound));
    });
  }

  private regionIdentifiers(info: FileInfo, node: ModNode): Set<string> {
    const names = new Set<string>();
    const inner = info.nodes.filter((other) => other !== node && other.inline && other.region[0] >= node.region[0] && other.region[1] <= node.region[1]);
    const nested = rangeIndex(inner.map((other) => other.region));
    const tokens = info.source.tokens;
    const inMacroArguments = rangeIndex(macroInvocationRanges(info.source));
    for (let index = 0; index < tokens.length; index += 1) {
      const token = tokens[index]!;
      if (token.kind === "string" && token.start >= node.region[0] && token.end <= node.region[1] && !nested(token.start) && inMacroArguments(token.start)) {
        for (const capture of token.text.matchAll(/(?<!\{)\{([A-Za-z_]\w*)(?:[:!][^}]*)?\}(?!\})/gu)) names.add(capture[1]!);
        continue;
      }
      if (token.start < node.region[0] || token.end > node.region[1] || token.kind !== "identifier" || nested(token.start)) continue;
      const previous = tokens[index - 1]?.text;
      if (previous === "." || previous === "::" || previous === "$" || tokens[index + 1]?.text === ":") continue;
      names.add(token.text);
    }
    return names;
  }

  private boundNames(node: ModNode): Set<string> {
    const names = new Set<string>([...node.children.keys(), ...node.items.keys()]);
    for (const bound of node.uses) { const name = leafBinding(bound.leaf); if (name) names.add(name); }
    return names;
  }

  private opaqueGlobTarget(node: ModNode): Rep | null {
    const found = new Map<string, Rep>();
    for (const bound of node.uses) {
      if (!bound.leaf.glob || !this.isMirrorEdge(bound)) continue;
      for (const hit of this.universe.leafHits(bound)) if (hit.kind === "module" && hit.rep && this.universe.opaqueModule(hit.node)) found.set(repText(hit.rep), { ...hit.rep, cut: true });
    }
    return found.size === 1 ? [...found.values()][0]! : null;
  }

  private decide(info: FileInfo, bound: Bound, added: Set<string>): Planned[] {
    const { leaf, node } = bound;
    const hits = this.universe.leafHits(bound);
    const hit = this.primary(hits);
    const keep: Planned[] = [{ leaf, source: bound }];
    if (!hit) {
      if (isPublic(bound.statement.visibility) && this.foreignHead(bound)) this.report.unresolvedMirrors.push(`${info.path}: ${leaf.path.join("::")}`);
      else {
        const partial = this.universe.resolvePath(node, leaf.path);
        const prefix = partial && partial.consumed < leaf.path.length && !leaf.glob ? this.primary(partial.hits) : undefined;
        const owner = prefix?.kind === "module" ? (prefix.rep?.cut ? prefix.rep : this.opaqueGlobTarget(prefix.node)) : null;
        if (partial && owner && owner.ident !== node.crate.ident) {
          this.report.repointedLeaves += 1;
          return [{ leaf: { path: [owner.ident, ...owner.segs, ...leaf.path.slice(partial.consumed)], alias: leaf.alias, glob: false }, source: bound }];
        }
        const last = partial?.hits[0];
        if (last?.kind === "module" && last.node.inline && !this.ownNamespace(last.node)) this.report.unresolvedMirrors.push(`${info.path}: ${leaf.path.join("::")} (path runs into a mirror module that is deleted; crate ${node.crate.ident}, mounted by ${info.mountedBy.join(", ")})`);
      }
      return keep;
    }
    if (this.isMirrorEdge(bound)) { this.report.removedLeaves += 1; return leaf.glob ? this.decideGlob(info, bound, hits, added, true) : this.demoted(info, bound, hit); }
    if (leaf.glob) return this.decideGlob(info, bound, hits, added, false);
    const rep = this.crossing(bound, hit);
    if (!rep) {
      if (hit.kind === "module" && hit.node.inline && (hit.node.uses.length > 0 || hit.node.children.size > 0) && !this.ownNamespace(hit.node)) { this.report.removedLeaves += 1; return []; }
      return keep;
    }
    const path = [rep.ident, ...rep.segs];
    if (samePath(path, leaf.path)) return keep;
    this.report.repointedLeaves += 1;
    return [{ leaf: { path, alias: this.aliasFor(info, bound, path), glob: false }, source: bound }];
  }

  private renames = new Map<string, Map<string, string>>();

  private aliasFor(info: FileInfo, bound: Bound, path: readonly string[]): string | null {
    const last = bound.leaf.path.at(-1) ?? null;
    if (bound.leaf.alias !== null) return bound.leaf.alias;
    const next = path.at(-1) ?? null;
    if (last === null || next === last) return null;
    const known = this.renames.get(info.path) ?? new Map<string, string>();
    const taken = this.boundNames(bound.node);
    taken.delete(last);
    if (next === null || taken.has(next) || (known.has(last) && known.get(last) !== next)) return last;
    known.set(last, next);
    this.renames.set(info.path, known);
    return null;
  }

  private demoted(info: FileInfo, bound: Bound, hit: Hit): Planned[] {
    const name = leafBinding(bound.leaf);
    const rep = hit.rep;
    if (!name || !rep || !this.usedLocally(info, bound.node, name, hit.kind === "item")) return [];
    const path = [rep.ident, ...rep.segs];
    return [{ leaf: { path, alias: this.aliasFor(info, bound, path), glob: false }, source: bound, visibility: "" }];
  }

  private usedLocally(info: FileInfo, node: ModNode, name: string, items: boolean): boolean {
    const tokens = info.source.tokens;
    const inner = rangeIndex(info.nodes.filter((other) => other !== node && other.inline && other.region[0] >= node.region[0] && other.region[1] <= node.region[1]).map((other) => other.region));
    const statements = rangeIndex(info.statements.map((statement) => [statement.start, statement.end] as const));
    for (let index = 0; index < tokens.length; index += 1) {
      const token = tokens[index]!;
      if (token.text !== name || token.kind !== "identifier" || token.start < node.region[0] || token.end > node.region[1] || inner(token.start) || statements(token.start)) continue;
      const previous = tokens[index - 1]?.text;
      if (previous === "." || previous === "::" || previous === "$" || tokens[index + 1]?.text === ":" && tokens[index + 2]?.text !== ":") continue;
      if (!items && tokens[index + 1]?.text === "::") continue;
      return true;
    }
    return false;
  }

  private decideGlob(info: FileInfo, bound: Bound, hits: Hit[], added: Set<string>, drop: boolean): Planned[] {
    const { leaf, node } = bound;
    const target = hits.find((hit) => hit.kind === "module");
    if (!target) {
      const rep = this.crossing(bound, hits.find((hit) => hit.kind === "item"));
      if (!rep || drop) return [{ leaf, source: bound }];
      const path = [rep.ident, ...rep.segs];
      if (samePath(path, leaf.path)) return [{ leaf, source: bound }];
      this.report.repointedLeaves += 1;
      return [{ leaf: { path, alias: null, glob: true }, source: bound }];
    }
    if (target.kind !== "module") return [{ leaf, source: bound }];
    const publicKept = isPublic(bound.statement.visibility) && !drop;
    const mrep = target.rep?.cut && target.rep.ident !== node.crate.ident ? target.rep : null;
    const imports: Planned[] = [];
    const local = this.boundNames(node);
    const keep = !drop && this.ownNamespace(target.node);
    for (const name of this.regionIdentifiers(info, node)) {
      if (local.has(name) || added.has(name)) continue;
      const sub = this.universe.member(target.node, name, node)[0];
      if (!sub) {
        const hidden = this.universe.memberIgnoringVisibility(target.node, name, node)[0];
        if (hidden && hidden.node.crate.ident !== node.crate.ident) this.report.invisible.push(`${info.path}: ${name} is not visible from ${node.crate.ident} (defined in ${hidden.node.crate.ident} ${hidden.node.file})`);
      }
      const through = sub?.rep?.cut && sub.rep.ident !== target.node.crate.ident ? sub.rep : null;
      const found = through ?? (sub && mrep && !keep && !publicKept ? { ...mrep, segs: [...mrep.segs, name] } : null);
      if (!found) continue;
      added.add(name);
      const owned = [found.ident, ...found.segs];
      let alias: string | null = null;
      if (owned.at(-1) !== name) {
        const known = this.renames.get(info.path) ?? new Map<string, string>();
        if (local.has(owned.at(-1)!) || (known.has(name) && known.get(name) !== owned.at(-1))) alias = name;
        else { known.set(name, owned.at(-1)!); this.renames.set(info.path, known); }
      }
      imports.push({ leaf: { path: owned, alias, glob: false }, source: null, ...(drop || publicKept ? { visibility: "" } : {}) });
    }
    this.report.addedImports += imports.length;
    if (publicKept) return [{ leaf, source: bound }, ...imports];
    const path = mrep ? [mrep.ident, ...mrep.segs] : leaf.path;
    if (!keep) { if (!drop) this.report.removedLeaves += 1; }
    else if (!samePath(path, leaf.path)) this.report.repointedLeaves += 1;
    return keep ? [{ leaf: { path, alias: null, glob: true }, source: bound }, ...imports] : imports;
  }

  private headSurvives(node: ModNode, names: readonly string[]): boolean {
    const head = names[0]!;
    if (isKeywordHead(head) || this.universe.crates.has(head)) return false;
    const resolved = this.universe.resolvePath(node, [head]);
    const hit = resolved ? this.primary(resolved.hits) : undefined;
    if (hit?.rep?.cut !== true || hit.rep.ident === node.crate.ident) return false;
    const full = this.universe.resolvePath(node, names);
    const last = full ? this.primary(full.hits) : undefined;
    return last?.rep === undefined || last.rep === null || last.rep.ident === hit.rep.ident;
  }

  private chains(info: FileInfo, inStatement: (offset: number) => boolean): { edits: Edit[]; touched: Set<string> } {
    const edits: Edit[] = [];
    const touched = new Set<string>();
    const tokens = info.source.tokens;
    const inMacroArguments = rangeIndex(macroInvocationRanges(info.source));
    for (let index = 0; index < tokens.length; index += 1) {
      const head = tokens[index]!;
      const previous = tokens[index - 1]?.text;
      if (head.kind !== "identifier" || previous === "::" || previous === "." || previous === "$" || inStatement(head.start)) continue;
      let cursor = index;
      const last = [head];
      while (tokens[cursor + 1]?.text === "::" && tokens[cursor + 2]?.kind === "identifier") { last.push(tokens[cursor + 2]!); cursor += 2; }
      const first = index;
      index = cursor;
      if (last.length < 2) continue;
      const node = this.universe.nodeAt(info.path, tokens[first]!.start);
      if (!node) continue;
      const names = last.map((token) => token.text);
      if (inMacroArguments(tokens[first]!.start) && this.headSurvives(node, names)) continue;
      const result = this.universe.resolvePath(node, names);
      const chosen = result ? this.primary(result.hits, tokens[cursor + 1]?.text === "(" || tokens[cursor + 1]?.text === "!") : undefined;
      const rep = chosen?.rep;
      if (!result || !chosen || !rep?.cut || rep.ident === node.crate.ident) continue;
      if (!isKeywordHead(names[0]!) && !this.universe.crates.has(names[0]!) && result.consumed === 1) continue;
      const text = repText(rep);
      if (text === names.slice(0, result.consumed).join("::")) continue;
      edits.push({ start: tokens[first]!.start, end: last[result.consumed - 1]!.end, text });
      touched.add(names[0]!);
      this.report.repointedChains += 1;
    }
    return { edits, touched };
  }

  private emptiedModules(text: string, deletions: readonly Edit[]): Edit[] {
    const tree = parseRustSourceTree(text);
    const blank = (module: RustModuleNode): boolean => {
      if (!module.inline) return false;
      const inside = deletions.filter((edit) => edit.start >= module.bodyStart && edit.end <= module.bodyEnd);
      const emptied = module.children.filter(blank);
      if (!inside.length && !emptied.length) return false;
      const cuts = [...inside, ...emptied.map((child) => ({ start: child.start, end: child.end }))].sort((left, right) => left.start - right.start);
      let rest = "";
      let cursor = module.bodyStart;
      for (const cut of cuts) { if (cut.start > cursor) rest += text.slice(cursor, cut.start); cursor = Math.max(cursor, cut.end); }
      rest += text.slice(cursor, module.bodyEnd);
      return rest.replace(/\/\*[\s\S]*?\*\//gu, "").replace(/\/\/[^\n]*/gu, "").trim() === "";
    };
    const edits: Edit[] = [];
    const visit = (modules: readonly RustModuleNode[]): void => {
      for (const module of modules) {
        if (blank(module)) edits.push({ start: module.start, end: module.end, text: "" });
        else visit(module.children);
      }
    };
    visit(tree.modules);
    return edits;
  }

  audit(path: string): string[] {
    const info = this.universe.files.get(path);
    if (!info) return [];
    const failures: string[] = [];
    for (const node of info.nodes) for (const bound of node.uses) {
      const head = bound.leaf.path[0];
      if (!head || this.universe.leafHits(bound).length) continue;
      const partial = this.universe.resolvePath(node, bound.leaf.path);
      if (partial && partial.hits.some((hit) => hit.kind === "module" && this.universe.opaqueModule(hit.node))) continue;
      if (isKeywordHead(head) || this.universe.crates.has(head) || node.children.has(head)) failures.push(`use ${bound.leaf.path.join("::")}${bound.leaf.glob ? "::*" : ""}`);
    }
    const covered = rangeIndex(info.statements.map((statement) => [statement.start, statement.end] as const));
    const tokens = info.source.tokens;
    for (let index = 0; index < tokens.length; index += 1) {
      const head = tokens[index]!;
      const previous = tokens[index - 1]?.text;
      if (head.kind !== "identifier" || previous === "::" || previous === "." || previous === "$" || covered(head.start)) continue;
      const names = [head.text];
      while (tokens[index + 1]?.text === "::" && tokens[index + 2]?.kind === "identifier") { names.push(tokens[index + 2]!.text); index += 2; }
      if (names.length < 2 || !(isKeywordHead(names[0]!) || this.universe.crates.has(names[0]!))) continue;
      const node = this.universe.nodeAt(path, head.start);
      const result = node ? this.universe.resolvePath(node, names) : null;
      if (!result || (result.hits[0]!.kind === "module" && result.consumed < names.length && !result.hits.some((hit) => hit.kind === "module" && this.universe.opaqueModule(hit.node)))) failures.push(`path ${names.join("::")}`);
    }
    return failures;
  }

  processFile(path: string): void {
    const info = this.universe.files.get(path);
    if (!info || !this.options.editable(path)) return;
    if (info.mounts !== 1) { this.report.multiMounted.push(`${path} <- ${info.mountedBy.join(", ")}`); return; }
    const text = info.source.text;
    const byStatement = new Map<UseStatement, Bound[]>();
    for (const node of info.nodes) for (const bound of node.uses) byStatement.set(bound.statement, [...(byStatement.get(bound.statement) ?? []), bound]);
    const inStatement = rangeIndex([...byStatement.keys()].map((statement) => [statement.start, statement.end] as const));
    const planned = new Map<UseStatement, Planned[]>();
    const addedByNode = new Map<number, Set<string>>();
    for (const [statement, bounds] of byStatement) {
      const rows: Planned[] = [];
      for (const bound of bounds) {
        const added = addedByNode.get(bound.node.id) ?? new Set<string>();
        addedByNode.set(bound.node.id, added);
        rows.push(...this.decide(info, bound, added));
      }
      planned.set(statement, rows);
    }
    const chain = this.chains(info, inStatement);
    const chainIndex = rangeIndex(chain.edits.map((edit) => [edit.start, edit.end] as const));
    const remaining = new Map<string, number>();
    if (chain.touched.size) for (const token of info.source.tokens) {
      if (token.kind !== "identifier" || !chain.touched.has(token.text) || inStatement(token.start) || chainIndex(token.start)) continue;
      remaining.set(token.text, (remaining.get(token.text) ?? 0) + 1);
    }
    const statementEdits: Edit[] = [];
    for (const [statement, bounds] of byStatement) {
      const rows = planned.get(statement)!.filter((row) => {
        const name = leafBinding(row.leaf);
        if (!name || !row.source || !chain.touched.has(name)) return true;
        const hit = this.primary(this.universe.leafHits(row.source));
        return hit?.kind !== "module" || (remaining.get(name) ?? 0) > 0;
      });
      const next = rows.map((row) => row.leaf);
      const original = bounds.map((bound) => bound.leaf);
      if (next.length === original.length && next.every((leaf, index) => leaf === original[index]) && rows.every((row) => row.visibility === undefined)) continue;
      const lineStart = text.lastIndexOf("\n", statement.start - 1) + 1;
      const indent = /^[ \t]*$/u.test(text.slice(lineStart, statement.start)) ? text.slice(lineStart, statement.start) : "";
      if (!next.length) {
        let end = statement.end;
        while (text[end] === " " || text[end] === "\t") end += 1;
        if (text[end] === "\n") end += 1;
        const previousBlank = lineStart === 0 || text.slice(text.lastIndexOf("\n", lineStart - 2) + 1, lineStart).trim() === "";
        if (previousBlank && indent) while (/^[ \t]*\n/u.test(text.slice(end, end + 200))) end += text.slice(end).indexOf("\n") + 1;
        statementEdits.push({ start: indent ? lineStart : statement.start, end, text: "" });
      } else {
        const groups = new Map<string, UseLeaf[]>();
        for (const row of rows) groups.set(row.visibility ?? statement.visibility, [...(groups.get(row.visibility ?? statement.visibility) ?? []), row.leaf]);
        const text = [...groups].map(([visibility, leaves]) => renderUseLeaves(leaves, visibility, statement.attributes, indent)).join(`\n${indent}`);
        statementEdits.push({ start: statement.start, end: statement.end, text });
      }
    }
    const renameEdits: Edit[] = [];
    const renames = this.renames.get(path);
    if (renames) for (const [index, token] of info.source.tokens.entries()) {
      const next = renames.get(token.text);
      const before = info.source.tokens[index - 1]?.text;
      if (!next || token.kind !== "identifier" || before === "." || before === "::" || before === "$" || inStatement(token.start) || chainIndex(token.start)) continue;
      renameEdits.push({ start: token.start, end: token.end, text: next });
    }
    chain.edits.push(...renameEdits);
    const emptied = this.emptiedModules(text, statementEdits.filter((edit) => edit.text === ""));
    this.report.removedModules += emptied.length;
    const inside = rangeIndex(emptied.map((edit) => [edit.start, edit.end] as const));
    const all = [...statementEdits, ...chain.edits].filter((edit) => !inside(edit.start));
    all.push(...emptied);
    if (!all.length) return;
    all.sort((left, right) => right.start - left.start);
    for (let index = 1; index < all.length; index += 1) if (all[index]!.end > all[index - 1]!.start) throw new Error(`unmirror: overlapping edits in ${path} at ${all[index]!.start}`);
    let out = text;
    for (const edit of all) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
    if (out !== text) this.report.edits.set(path, out);
  }
}
