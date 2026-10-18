import { dirname } from "node:path";
import { parseRustSourceTree, type RustModuleNode, rustOuterDocStart, type RustSourceItem } from "../../🌳️module-tree/🟦️.ts";

export function lcaDirectory(files: readonly string[]): string {
  const parts = files.map((file) => dirname(file).split("/"));
  const first = parts[0] ?? [];
  let length = first.length;
  for (const other of parts) { let i = 0; while (i < length && i < other.length && other[i] === first[i]) i += 1; length = i; }
  return first.slice(0, length).join("/");
}

export function allItems(text: string): RustSourceItem[] {
  const tree = parseRustSourceTree(text);
  const out: RustSourceItem[] = [...tree.items];
  const visit = (nodes: readonly RustModuleNode[]): void => { for (const node of nodes) if (node.inline) { out.push(...node.items); visit(node.children); } };
  visit(tree.modules);
  return out;
}

export function removeRange(text: string, start: number, end: number): { start: number; end: number } {
  let right = end;
  while (text[right] === " " || text[right] === "\t") right += 1;
  if (text[right] === "\n") right += 1;
  return { start: rustOuterDocStart(text, start), end: right };
}
