/** 🔍️ Module-level name resolution over the carved crates of a repository: builds the module tree of every crate from its `#[path]` mounts and resolves `use` leaves and paths through explicit imports, globs and re-exports, recording where a path crosses into another carved crate. */
import { existsSync, readFileSync } from "node:fs";
import { posix, resolve } from "node:path";
import { rustTokens } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { parseRustSourceTree, type RustModuleNode, type RustSourceItem } from "../../🌳️module-tree/🟦️.ts";
import { leafBinding, scanSource, scanUseStatements, type ScannedSource, type UseLeaf, type UseStatement } from "../🧵️use-tree/🟦️.ts";

/** 📦️ One Cargo package with a library target: its name, extern identifier, repository-relative directory and library root. */
export type CrateInfo = Readonly<{ name: string; ident: string; dir: string; root: string }>;

/** 🧭️ The foreign path a lookup ended up at: the extern identifier of the owning crate and the module path below it, as written in the re-export that crossed the crate boundary; `cut` tells that the lookup went through an edge that is about to be deleted, so the path has to be re-written. */
export type Rep = Readonly<{ ident: string; segs: readonly string[]; cut: boolean }>;

/** 🎯️ One resolution result: a module or an item together with the foreign path when the lookup crossed a re-export into another crate. */
export type Hit = Readonly<{ kind: "module"; node: ModNode; rep: Rep | null } | { kind: "item"; node: ModNode; name: string; rep: Rep | null }>;

/** 🍃️ A use leaf bound to the module that contains its statement. */
export type Bound = { readonly node: ModNode; readonly statement: UseStatement; readonly leaf: UseLeaf; readonly key: string };

/** 🌳️ One module of a crate: its children, declared item names, use leaves and the character region it occupies in its file. */
export type ModNode = {
  readonly id: number;
  readonly crate: CrateInfo;
  readonly name: string;
  readonly parent: ModNode | null;
  readonly file: string;
  readonly visibility: string;
  readonly test: boolean;
  readonly inline: boolean;
  readonly region: readonly [number, number];
  readonly children: Map<string, ModNode[]>;
  readonly items: Map<string, string>;
  readonly weak: Set<string>;
  readonly uses: Bound[];
};

/** 📄️ A parsed source file with the modules it hosts and its use statements. */
export type FileInfo = { readonly path: string; readonly source: ScannedSource; readonly statements: UseStatement[]; readonly nodes: ModNode[]; mounts: number; readonly mountedBy: string[] };

/** 🧬️ Combines the foreign path found inside a lookup with the path that led to it: a path found inside wins, the cut flag of the way in stays. */
export function mergeRep(inner: Rep | null, outer: Rep | null, segment: string): Rep | null {
  if (inner) return outer?.cut && !inner.cut ? { ...inner, cut: true } : inner;
  return outer ? { ident: outer.ident, segs: [...outer.segs, segment], cut: outer.cut } : null;
}

const KEYWORD_HEADS = new Set(["crate", "self", "super"]);

/** 🏗️ The repository view the resolver works on: crates by extern identifier, file texts through an overlay and lazily built module trees. */
export class Universe {
  readonly crates = new Map<string, CrateInfo>();
  readonly files = new Map<string, FileInfo>();
  private readonly roots = new Map<string, ModNode | null>();
  private readonly leafMemo = new Map<string, Hit[]>();
  private readonly memberMemo = new Map<string, Hit[]>();
  private nextId = 1;
  ignoreVisibility = false;
  private lowest = Number.POSITIVE_INFINITY;
  private readonly depths = new Map<string, number>();
  apiLeaf: (bound: Bound) => boolean = () => false;
  cutEdge: (bound: Bound) => boolean = () => false;
  constructor(readonly repoRoot: string, readonly overlay: ReadonlyMap<string, string> = new Map()) {}

  read(path: string): string | null {
    const overlaid = this.overlay.get(path);
    if (overlaid !== undefined) return overlaid;
    const absolute = resolve(this.repoRoot, path);
    return existsSync(absolute) ? readFileSync(absolute, "utf8") : null;
  }

  addCrate(name: string, dir: string, root: string): CrateInfo {
    const info = { name, ident: name.replaceAll("-", "_"), dir, root };
    this.crates.set(info.ident, info);
    return info;
  }

  loadedRoots(): string[] {
    return [...this.roots].filter(([, node]) => node !== null).map(([ident]) => ident);
  }

  root(ident: string): ModNode | null {
    if (this.roots.has(ident)) return this.roots.get(ident)!;
    const info = this.crates.get(ident);
    this.roots.set(ident, null);
    if (!info) return null;
    const text = this.read(info.root);
    if (text === null) return null;
    const node = this.makeNode(info, "", null, info.root, "pub", false, [0, text.length]);
    this.roots.set(ident, node);
    this.loadFile(info.root, node, text, posix.dirname(info.root));
    return node;
  }

  private makeNode(crate: CrateInfo, name: string, parent: ModNode | null, file: string, visibility: string, test: boolean, region: readonly [number, number], inline = false): ModNode {
    return { id: this.nextId++, crate, name, parent, file, visibility, test, inline, region, children: new Map(), items: new Map(), weak: new Set(), uses: [] };
  }

  private loadFile(path: string, fileNode: ModNode, text: string, base: string, key = path): void {
    let info = this.files.get(key);
    if (info) { info.mounts += 1; info.mountedBy.push(`${fileNode.crate.ident}:${fileNode.parent?.name ?? "<root>"}`); return; }
    const source = scanSource(text);
    const statements = scanUseStatements(source);
    info = { path, source, statements, nodes: [fileNode], mounts: 1, mountedBy: [`${fileNode.crate.ident}:${fileNode.parent?.name ?? "<root>"}`] };
    this.files.set(key, info);
    const tree = parseRustSourceTree(text);
    this.fill(info, fileNode, tree.items, tree.modules, posix.dirname(path), true, base);
    for (const statement of statements) {
      const owner = info.nodes.filter((node) => node.region[0] <= statement.start && statement.end <= node.region[1]).sort((left, right) => (right.region[1] - right.region[0]) - (left.region[1] - left.region[0])).at(-1)!;
      statement.leaves.forEach((leaf, index) => owner.uses.push({ node: owner, statement, leaf, key: `${key}:${statement.start}:${index}` }));
    }
  }

  private fill(info: FileInfo, node: ModNode, items: readonly RustSourceItem[], modules: readonly RustModuleNode[], fileDir: string, inFileScope: boolean, base: string): void {
    for (const item of items) {
      if (item.kind === "use" || item.kind === "impl" || item.kind === "extern-crate") continue;
      const name = item.name ?? this.customItemName(info, item);
      if (name) node.items.set(name, item.visibility === "private" ? "" : item.visibility);
      else if (item.kind === "other") for (const word of this.macroWords(info, item)) node.weak.add(word);
    }
    for (const module of modules) {
      const visibility = module.visibility === "private" ? "" : module.visibility;
      const test = node.test || module.conditionalTest;
      const siblings = node.children.get(module.name) ?? [];
      node.children.set(module.name, siblings);
      if (module.inline) {
        const child = this.makeNode(node.crate, module.name, node, info.path, visibility, test, [module.bodyStart, module.bodyEnd], true);
        siblings.push(child);
        info.nodes.push(child);
        this.fill(info, child, module.items, module.children, fileDir, false, posix.normalize(posix.join(base, module.pathAttribute ?? module.name)));
        continue;
      }
      if (module.pathAttribute === null) { siblings.push(this.makeNode(node.crate, module.name, node, info.path, visibility, test, [0, 0])); continue; }
      const relativePath = posix.normalize(posix.join(inFileScope ? fileDir : base, module.pathAttribute));
      const first = this.files.get(relativePath);
      const key = first && first.nodes[0]!.crate.ident !== node.crate.ident ? `${relativePath}\u0000${node.crate.ident}` : relativePath;
      const known = this.files.get(key);
      if (known) {
        known.mounts += 1;
        known.mountedBy.push(`${node.crate.ident}:${node.name}/${module.name}`);
        siblings.push({ ...known.nodes[0]!, id: this.nextId++, name: module.name, parent: node, visibility, test });
        continue;
      }
      if (first) { first.mounts += 1; first.mountedBy.push(`${node.crate.ident}:${node.name}/${module.name}`); }
      const text = this.read(relativePath);
      if (text === null) { siblings.push(this.makeNode(node.crate, module.name, node, relativePath, visibility, test, [0, 0])); continue; }
      const child = this.makeNode(node.crate, module.name, node, relativePath, visibility, test, [0, text.length]);
      siblings.push(child);
      this.loadFile(relativePath, child, text, posix.dirname(relativePath), key);
    }
  }

  private macroWords(info: FileInfo, item: RustSourceItem): string[] {
    const slice = info.source.text.slice(item.declStart, item.end);
    if (!/^[A-Za-z_][\w:]*!\s*[({[]/u.test(slice)) return [];
    return rustTokens(slice).filter((token) => token.kind === "identifier" && /^[A-Z][A-Za-z0-9_]*$/u.test(token.text)).map((token) => token.text);
  }

  private customItemName(info: FileInfo, item: RustSourceItem): string | null {
    if (item.kind !== "other") return null;
    const head = /^(?:pub(?:\([^)]*\))?\s+)?([a-z_]+)\s+([A-Za-z_][A-Za-z0-9_]*)\s*[{(<]/u.exec(info.source.text.slice(item.declStart, item.declStart + 160));
    return head && !["mod", "use", "let", "if", "match", "for", "while", "loop", "return", "where"].includes(head[1]!) ? head[2]! : null;
  }

  private visible(visibility: string, owner: ModNode, from: ModNode): boolean {
    if (visibility === "pub" || this.ignoreVisibility) return true;
    return from.crate.ident === owner.crate.ident;
  }

  member(node: ModNode, name: string, from: ModNode, stack: Set<string> = new Set()): Hit[] {
    const memo = `${node.id}\u0000${name}\u0000${from.crate.ident === node.crate.ident ? "s" : "x"}${this.ignoreVisibility ? "a" : ""}`;
    const known = this.memberMemo.get(memo);
    if (known) return known;
    if (stack.has(memo)) { this.closeCycle(memo); return []; }
    const frame = this.enter(stack, memo);
    const out: Hit[] = [];
    for (const child of node.children.get(name) ?? []) if (this.visible(child.visibility, node, from)) out.push({ kind: "module", node: child, rep: null });
    const declared = node.items.get(name);
    if (declared !== undefined && this.visible(declared, node, from)) out.push({ kind: "item", node, name, rep: null });
    for (const bound of node.uses) {
      if (bound.leaf.glob || leafBinding(bound.leaf) !== name || !this.visible(bound.statement.visibility, node, from)) continue;
      const hits = this.leafHits(bound, stack);
      const cut = this.cutEdge(bound);
      out.push(...(this.apiLeaf(bound) ? hits.map((hit) => ({ ...hit, rep: null }) as Hit) : hits.map((hit) => (cut && hit.rep ? { ...hit, rep: { ...hit.rep, cut: true } } : hit) as Hit)));
    }
    const shadowed = new Set(out.map((hit) => hit.kind));
    for (const bound of node.uses) {
      if (!bound.leaf.glob || !this.visible(bound.statement.visibility, node, from)) continue;
      const cut = this.cutEdge(bound);
      for (const raw of this.leafHits(bound, stack)) {
        if (raw.kind !== "module") continue;
        const target = cut && raw.rep ? { ...raw, rep: { ...raw.rep, cut: true } } : raw;
        for (const sub of this.member(target.node, name, from, stack)) if (!shadowed.has(sub.kind)) out.push({ ...sub, rep: mergeRep(sub.rep, target.rep, name) } as Hit);
      }
    }
    if (!out.length && node.weak.has(name)) out.push({ kind: "item", node, name, rep: null });
    if (this.leave(stack, memo, frame)) this.memberMemo.set(memo, out);
    return out;
  }

  leafHits(bound: Bound, stack: Set<string> = new Set()): Hit[] {
    const known = this.leafMemo.get(bound.key);
    if (known) return known;
    const marker = `leaf:${bound.key}`;
    if (stack.has(marker)) { this.closeCycle(marker); return []; }
    const frame = this.enter(stack, marker);
    const result = this.resolvePath(bound.node, bound.leaf.path, stack);
    const hits = result ? this.completeLeaf(result, bound.leaf.path) : [];
    if (this.leave(stack, marker, frame)) this.leafMemo.set(bound.key, hits);
    return hits;
  }

  private closeCycle(marker: string): void {
    this.lowest = Math.min(this.lowest, this.depths.get(marker) ?? 0);
  }

  private enter(stack: Set<string>, marker: string): { depth: number; outer: number } {
    const frame = { depth: stack.size, outer: this.lowest };
    this.lowest = Number.POSITIVE_INFINITY;
    stack.add(marker);
    this.depths.set(marker, frame.depth);
    return frame;
  }

  private leave(stack: Set<string>, marker: string, frame: { depth: number; outer: number }): boolean {
    stack.delete(marker);
    this.depths.delete(marker);
    const inner = this.lowest;
    this.lowest = inner < frame.depth ? Math.min(frame.outer, inner) : frame.outer;
    return inner >= frame.depth;
  }

  private completeLeaf(result: { hits: Hit[]; consumed: number }, path: readonly string[]): Hit[] {
    if (result.consumed === path.length) return result.hits;
    const rest = path.slice(result.consumed);
    return result.hits.filter((hit) => hit.kind === "item").map((hit) => ({ ...hit, rep: hit.rep ? { ...hit.rep, segs: [...hit.rep.segs, ...rest] } : null }) as Hit);
  }

  resolvePath(node: ModNode, path: readonly string[], stack: Set<string> = new Set()): { hits: Hit[]; consumed: number } | null {
    if (!path.length) return null;
    const head = path[0]!;
    let hits: Hit[] = [];
    let consumed = 1;
    if (head === "crate") { const root = this.root(node.crate.ident); hits = root ? [{ kind: "module", node: root, rep: null }] : []; }
    else if (head === "self") hits = [{ kind: "module", node, rep: null }];
    else if (head === "super") {
      let cursor: ModNode | null = node;
      let index = 0;
      while (path[index] === "super" && cursor) { cursor = cursor.parent; index += 1; }
      consumed = index;
      hits = cursor ? [{ kind: "module", node: cursor, rep: null }] : [];
    } else {
      hits = this.member(node, head, node, stack);
      if (!hits.length && this.crates.has(head)) {
        const root = this.root(head);
        hits = root ? [{ kind: "module", node: root, rep: head === node.crate.ident ? null : { ident: head, segs: [], cut: false } }] : [];
      }
    }
    if (!hits.length) return null;
    while (consumed < path.length) {
      const segment = path[consumed]!;
      const next: Hit[] = [];
      for (const hit of hits) {
        if (hit.kind !== "module") continue;
        for (const sub of this.member(hit.node, segment, node, stack)) next.push({ ...sub, rep: mergeRep(sub.rep, hit.rep, segment) } as Hit);
      }
      if (!next.length) break;
      hits = next;
      consumed += 1;
    }
    return { hits, consumed };
  }

  /** 👁️ Looks a name up like `member` but ignores visibility: finds items that exist but cannot be named from another crate. */
  memberIgnoringVisibility(node: ModNode, name: string, from: ModNode): Hit[] {
    this.ignoreVisibility = true;
    try { return this.member(node, name, from); } finally { this.ignoreVisibility = false; }
  }

  nodeAt(file: string, offset: number): ModNode | null {
    const info = this.files.get(file);
    if (!info) return null;
    let best: ModNode | null = null;
    for (const node of info.nodes) if (node.region[0] <= offset && offset <= node.region[1] && (!best || node.region[1] - node.region[0] <= best.region[1] - best.region[0])) best = node;
    return best;
  }
}

/** 🧾️ The foreign path as source text. */
export const repText = (rep: Rep): string => [rep.ident, ...rep.segs].join("::");

/** 🔑️ Whether a path head is resolved without the module scope (`crate`, `self` or `super`). */
export const isKeywordHead = (head: string): boolean => KEYWORD_HEADS.has(head);
