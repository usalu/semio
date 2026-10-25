import { dirname, resolve } from "node:path";
import { rustTokens } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import { identifierSet, parseRustSourceTree, type RustModuleNode, type RustSourceItem } from "../🌳️module-tree/🟦️.ts";
import { declaredNames, hostPrivate } from "../🧪️test-closure/🟢️harness/🟦️.ts";

/** 🧩️ One module declaration of a crate, reached through the root file and every mounted file. */
export type ModuleDecl = {
  readonly id: string;
  readonly raw: readonly string[];
  readonly eff: readonly string[];
  readonly node: RustModuleNode;
  readonly declFile: string;
  readonly declText: string;
  readonly parentId: string | null;
  readonly file: string | null;
  readonly scope: string;
  readonly inlineRoot: boolean;
  readonly testOnly: boolean;
  readonly isComponent: boolean;
  readonly globItem: RustSourceItem | null;
  readonly depthInFile: number;
  readonly children: string[];
};

/** 🌳️ All declarations of one crate keyed by raw module path. */
export type DeclTree = { readonly decls: Map<string, ModuleDecl>; readonly roots: string[] };

/** 📦️ Walks every module declaration of a crate; text of mounted files comes from `readText`. */
export function collectDecls(rootFile: string, rootText: string, readText: (absolute: string) => string | null): DeclTree {
  const decls = new Map<string, ModuleDecl>();
  const roots: string[] = [];
  const walk = (nodes: readonly RustModuleNode[], items: readonly RustSourceItem[], declFile: string, declText: string, scope: string, inFileScope: boolean, raw: readonly string[], eff: readonly string[], parentId: string | null, inlineRoot: boolean, testOnly: boolean, depth: number): void => {
    const globComponent = items.find((item) => item.kind === "use" && /^pub\s+use\s+component::\*\s*;/u.test(declText.slice(item.declStart, item.end))) ?? null;
    for (const node of nodes) {
      const rawPath = [...raw, node.name];
      const id = rawPath.join("::");
      const isComponent = node.name === "component" && !node.inline && globComponent !== null;
      const effPath = isComponent ? eff : [...eff, node.name];
      const file = !node.inline && node.pathAttribute !== null ? resolve(inFileScope ? dirname(declFile) : scope, node.pathAttribute) : null;
      const test = testOnly || node.conditionalTest;
      const decl: ModuleDecl = { id, raw: rawPath, eff: effPath, node, declFile, declText, parentId, file, scope: inFileScope ? dirname(declFile) : scope, inlineRoot, testOnly: test, isComponent, globItem: isComponent ? globComponent : null, depthInFile: depth, children: [] };
      decls.set(id, decl);
      if (parentId) decls.get(parentId)!.children.push(id); else roots.push(id);
      if (node.inline) {
        walk(node.children, node.items, declFile, declText, resolve(scope, node.pathAttribute ?? node.name), false, rawPath, effPath, id, inlineRoot, test, depth + 1);
      } else if (file !== null) {
        const text = readText(file);
        if (text === null) continue;
        const tree = parseRustSourceTree(text);
        walk(tree.modules, tree.items, file, text, dirname(file), true, rawPath, effPath, id, false, test, 0);
      }
    }
  };
  const tree = parseRustSourceTree(rootText);
  walk(tree.modules, tree.items, rootFile, rootText, dirname(rootFile), true, [], [], null, true, false, 0);
  return { decls, roots };
}

/** 🔗️ Longest `super::` chain at each module nesting level of a file text. */
export function superReach(text: string): number {
  const tree = parseRustSourceTree(text);
  const spans: [number, number][] = [];
  const collect = (nodes: readonly RustModuleNode[]): void => {
    for (const node of nodes) if (node.inline) { spans.push([node.bodyStart, node.bodyEnd]); collect(node.children); }
  };
  collect(tree.modules);
  const tokens = rustTokens(text);
  let reach = 0;
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i]!.text !== "super" || (tokens[i - 1]?.text === "::" && tokens[i - 2]?.text === "super")) continue;
    let length = 1;
    let cursor = i;
    while (tokens[cursor + 1]?.text === "::" && tokens[cursor + 2]?.text === "super") { length += 1; cursor += 2; }
    const level = spans.filter(([a, b]) => tokens[i]!.start >= a && tokens[i]!.start < b).length;
    reach = Math.max(reach, length - level);
  }
  return reach;
}

/** 🧮️ A union-find over strings. */
export class UnionFind {
  private readonly parent = new Map<string, string>();
  find(value: string): string {
    if (!this.parent.has(value)) this.parent.set(value, value);
    let root = value;
    while (this.parent.get(root) !== root) root = this.parent.get(root)!;
    this.parent.set(value, root);
    return root;
  }
  union(a: string, b: string): void {
    const left = this.find(a);
    const right = this.find(b);
    if (left !== right) this.parent.set(right, left);
  }
}

/** 🍰️ A set of declarations that must live in one crate, with the attach points a mirror root re-declares. */
export type CarveSlice = {
  readonly id: string;
  readonly decls: readonly ModuleDecl[];
  readonly attach: readonly ModuleDecl[];
};

/** 🔨️ Groups declarations into slices: coupled children (via `super::`), tests, and component siblings stay together; a test module that needs no `super::` item and names one of the detaching types becomes a slice of its own, so it can live in the crate of that type instead of cycling back to the module it tests. */
export function buildSlices(tree: DeclTree, readonlyFileTexts: (absolute: string) => string | null, trace: (message: string) => void = () => undefined, options: { readonly detachTestsMentioning?: ReadonlySet<string>; readonly detachAllTests?: boolean } = {}): { slices: CarveSlice[]; namespaces: Set<string> } {
  const union = new UnionFind();
  const reachOf = new Map<string, number>();
  for (const decl of tree.decls.values()) {
    const text = decl.file ? readonlyFileTexts(decl.file) : null;
    reachOf.set(decl.id, text === null ? 0 : superReach(text));
  }
  const subtreeCoupled = (decl: ModuleDecl): boolean => {
    const visit = (current: ModuleDecl, depth: number): boolean => {
      if ((reachOf.get(current.id) ?? 0) > depth) return true;
      return current.children.map((id) => tree.decls.get(id)!).some((child) => visit(child, depth + 1));
    };
    return visit(decl, 0);
  };
  const detaching = options.detachTestsMentioning ?? new Set<string>();
  const detachedCache = new Map<string, boolean>();
  const detached = (decl: ModuleDecl): boolean => {
    const known = detachedCache.get(decl.id);
    if (known !== undefined) return known;
    const answer = ((): boolean => {
      if (!decl.testOnly || decl.file === null || decl.node.inline || subtreeCoupled(decl) || !decl.parentId) return false;
      const parent = tree.decls.get(decl.parentId)!;
      if (parent.testOnly) return false;
      if (parent.parentId === null && !parent.node.inline) return false;
      const text = readonlyFileTexts(decl.file);
      if (text === null) return false;
      if (detaching.size && parent.node.inline && [...detaching].some((name) => new RegExp(`\\b${name}\\b`, "u").test(text))) return true;
      if (!options.detachAllTests) return false;
      const hostText = readonlyFileTexts(decl.declFile) ?? "";
      const privateUse = hostPrivate(hostText, identifierSet(text), declaredNames(text), text, parent.eff);
      if (privateUse.items.length || privateUse.relative.length || privateUse.carry.length) return false;
      return true;
    })();
    detachedCache.set(decl.id, answer);
    return answer;
  };
  for (const decl of tree.decls.values()) union.find(decl.id);
  for (const parent of tree.decls.values()) {
    const all = parent.children.map((id) => tree.decls.get(id)!);
    const kids = all.filter((kid) => !detached(kid));
    for (const kid of all) if (detached(kid)) trace(`detach ${kid.id} from ${parent.id}`);
    if (!kids.length) continue;
    if (parent.node.inline) {
      const anyCoupled = kids.some((kid) => kid.testOnly || subtreeCoupled(kid));
      if (anyCoupled) trace(`union kids of ${parent.id}: ${kids.filter((kid) => kid.testOnly || subtreeCoupled(kid)).map((kid) => kid.id + (kid.testOnly ? "[test]" : "[super]")).join(", ")}`);
      if (anyCoupled) for (const kid of kids) union.union(kids[0]!.id, kid.id);
      const component = kids.find((kid) => kid.isComponent);
      if (component) for (const kid of kids) if (kid.testOnly) union.union(component.id, kid.id);
    } else {
      for (const kid of kids) if (kid.testOnly || subtreeCoupled(kid) || kid.node.inline || kid.node.attributeTexts.some((text) => /^macro_use\b/u.test(text.trim()))) { trace(`union ${kid.id} into ${parent.id} (${kid.testOnly ? "test" : kid.node.inline ? "inline" : kid.node.attributeTexts.some((text) => /^macro_use\b/u.test(text.trim())) ? "macro_use" : "super"})`); union.union(parent.id, kid.id); }
    }
  }
  const groups = new Map<string, ModuleDecl[]>();
  for (const decl of tree.decls.values()) {
    const key = union.find(decl.id);
    groups.set(key, [...(groups.get(key) ?? []), decl]);
  }
  const namespaces = new Set<string>();
  const slices: CarveSlice[] = [];
  for (const [key, members] of groups) {
    const ids = new Set(members.map((member) => member.id));
    const attach = members.filter((member) => !member.parentId || !ids.has(member.parentId));
    slices.push({ id: key, decls: members, attach });
  }
  for (const decl of [...tree.decls.values()].reverse()) {
    if (!decl.node.inline) continue;
    if (!decl.inlineRoot || decl.node.pathAttribute === null) { for (const id of decl.children) union.union(decl.id, id); continue; }
    if (!decl.children.length) continue;
    if (decl.children.some((id) => namespaces.has(id))) { namespaces.add(decl.id); continue; }
    const owner = union.find(decl.id);
    const kidSlices = new Set(decl.children.map((id) => union.find(id)));
    if (kidSlices.size === 1 && kidSlices.has(owner)) continue;
    if (kidSlices.size === 1 && decl.children.length) { union.union(decl.id, [...kidSlices][0]!); continue; }
    namespaces.add(decl.id);
  }
  const final = new Map<string, ModuleDecl[]>();
  for (const decl of tree.decls.values()) {
    if (namespaces.has(decl.id)) continue;
    const key = union.find(decl.id);
    final.set(key, [...(final.get(key) ?? []), decl]);
  }
  const out: CarveSlice[] = [];
  for (const [key, members] of final) {
    const ids = new Set(members.map((member) => member.id));
    out.push({ id: key, decls: members, attach: members.filter((member) => !member.parentId || !ids.has(member.parentId) || namespaces.has(member.parentId)) });
  }
  void slices;
  return { slices: out, namespaces };
}
