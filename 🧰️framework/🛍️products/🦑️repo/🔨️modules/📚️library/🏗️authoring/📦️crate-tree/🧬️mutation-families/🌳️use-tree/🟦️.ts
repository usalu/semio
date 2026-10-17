/** 🌳️ Use tree: module exporting `pruneUseLeaves`, `flattenUse`. */
import { rustTokens, type RustToken } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

type UseTree = { prefix: string[]; glob: boolean; alias: string | null; children: UseTree[] | null };

function parseUseTree(tokens: readonly RustToken[], start: number): { tree: UseTree; next: number } | null {
  let k = start;
  const prefix: string[] = [];
  if (tokens[k]?.text === "::") k += 1;
  for (;;) {
    const token = tokens[k];
    if (!token) return null;
    if (token.text === "*") return { tree: { prefix, glob: true, alias: null, children: null }, next: k + 1 };
    if (token.text === "{") {
      const children: UseTree[] = [];
      k += 1;
      while (tokens[k] && tokens[k]!.text !== "}") {
        const child = parseUseTree(tokens, k);
        if (!child) return null;
        children.push(child.tree);
        k = child.next;
        if (tokens[k]?.text === ",") k += 1;
      }
      return { tree: { prefix, glob: false, alias: null, children }, next: k + 1 };
    }
    if (token.kind !== "identifier") return null;
    prefix.push(token.text);
    k += 1;
    if (tokens[k]?.text === "::") { k += 1; continue; }
    let alias: string | null = null;
    if (tokens[k]?.text === "as") { alias = tokens[k + 1]!.text; k += 2; }
    return { tree: { prefix, glob: false, alias, children: null }, next: k };
  }
}

function pruneUseTree(tree: UseTree, drop: ReadonlySet<string>): UseTree | null {
  if (tree.children) {
    const children = tree.children.map((child) => pruneUseTree(child, drop)).filter((child): child is UseTree => child !== null);
    if (!children.length) return null;
    if (children.length === 1 && !children[0]!.alias && children[0]!.prefix[0] !== "self") return { prefix: [...tree.prefix, ...children[0]!.prefix], glob: children[0]!.glob, alias: children[0]!.alias, children: children[0]!.children };
    return { ...tree, children };
  }
  if (tree.glob) return tree;
  const leaf = tree.alias ?? (tree.prefix.at(-1) === "self" ? tree.prefix.at(-2) : tree.prefix.at(-1));
  return leaf !== undefined && drop.has(leaf) ? null : tree;
}

function renderUseTree(tree: UseTree): string {
  const head = tree.prefix.join("::");
  if (tree.children) return `${head}${head ? "::" : ""}{${tree.children.map(renderUseTree).join(", ")}}`;
  if (tree.glob) return `${head}${head ? "::" : ""}*`;
  return `${head}${tree.alias ? ` as ${tree.alias}` : ""}`;
}

/** ✂️ Removes the named leaves from a `use` statement of any nesting; answers `null` when nothing is left to import. */
export function pruneUseLeaves(statement: string, drop: ReadonlySet<string>): string | null {
  const tokens = rustTokens(statement);
  const useIndex = tokens.findIndex((token) => token.text === "use");
  if (useIndex < 0) return statement;
  const parsed = parseUseTree(tokens, useIndex + 1);
  if (!parsed || tokens[parsed.next]?.text !== ";") return statement;
  const pruned = pruneUseTree(parsed.tree, drop);
  if (!pruned) return null;
  return `${statement.slice(0, tokens[useIndex + 1]!.start)}${renderUseTree(pruned)};`;
}

/** 🍃️ Every leaf a `use` statement binds with the full path it names; glob leaves are skipped and `self` leaves name their parent. */
export function flattenUse(statement: string): { path: string[]; leaf: string }[] {
  const tokens = rustTokens(statement);
  const useIndex = tokens.findIndex((token) => token.text === "use");
  if (useIndex < 0) return [];
  const parsed = parseUseTree(tokens, useIndex + 1);
  if (!parsed) return [];
  const out: { path: string[]; leaf: string }[] = [];
  const visit = (tree: UseTree, prefix: readonly string[]): void => {
    const path = [...prefix, ...tree.prefix];
    if (tree.children) { for (const child of tree.children) visit(child, path); return; }
    if (tree.glob) return;
    const selfLeaf = path.at(-1) === "self";
    const clean = selfLeaf ? path.slice(0, -1) : path;
    out.push({ path: clean, leaf: tree.alias ?? clean.at(-1)! });
  };
  visit(parsed.tree, []);
  return out;
}
