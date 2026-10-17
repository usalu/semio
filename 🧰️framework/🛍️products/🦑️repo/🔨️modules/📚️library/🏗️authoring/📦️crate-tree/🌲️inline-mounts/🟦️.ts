/** 📦️ Inline mounts: module exporting `InlinedMountTrees`, `inlineMountTreeFiles`. */
import { dirname, resolve } from "node:path";
import { parseRustSourceTree, type RustModuleNode } from "../🌳️module-tree/🟦️.ts";
import { relativePosix } from "../📝️render/🟦️.ts";

/** 🌲️ The artifact root with its pure mount-tree files folded back in, the files that became unreferenced and a note per folded file. */
export type InlinedMountTrees = { readonly rootSource: string; readonly deletes: Set<string>; readonly notes: string[] };

const PATH_ATTRIBUTE = /#\[\s*path\s*=\s*"([^"]*)"\s*\]/gu;

function onlyDotInline(nodes: readonly RustModuleNode[]): boolean {
  return nodes.every((node) => (node.inline ? node.pathAttribute === "." && onlyDotInline(node.children) : true));
}

/** 🌲️ Folds every root-level file module that consists only of `#[path = "."]` inline mount modules back into the root as one inline `#[path = "."]` wrapper; relative `#[path]` attributes are re-based to the root directory. The carve planner slices a mount tree only while its wrapper modules are inline in the lib root, so a root whose trees were moved into files is flattened for planning and the emptied files are deleted by the carve. */
export function inlineMountTreeFiles(rootFile: string, rootSource: string, readText: (absolute: string) => string | null): InlinedMountTrees {
  const rootDir = dirname(rootFile);
  const tree = parseRustSourceTree(rootSource);
  const edits: { start: number; end: number; text: string }[] = [];
  const deletes = new Set<string>();
  const notes: string[] = [];
  for (const node of tree.modules) {
    if (node.inline || node.conditionalTest || node.pathAttribute === null || node.pathAttribute === ".") continue;
    const file = resolve(rootDir, node.pathAttribute);
    const text = readText(file);
    if (text === null) continue;
    const parsed = parseRustSourceTree(text);
    if (parsed.items.length || !parsed.modules.some((child) => child.inline) || !onlyDotInline(parsed.modules)) continue;
    const fileDir = dirname(file);
    let body = text;
    for (const inner of [...parsed.innerAttributes].sort((a, b) => b.start - a.start)) body = body.slice(0, inner.start) + body.slice(inner.end);
    body = body.replace(/^(?:\s*\/\/![^\n]*\n)+/u, "").trim();
    body = body.replace(PATH_ATTRIBUTE, (whole: string, path: string) => (path === "." ? whole : `#[path = "${relativePosix(rootDir, resolve(fileDir, path))}"]`));
    const indented = body.split("\n").map((line) => (line.length ? `    ${line}` : line)).join("\n");
    const head = rootSource.slice(node.start, node.declStart).replace(PATH_ATTRIBUTE, '#[path = "."]');
    edits.push({ start: node.start, end: node.end, text: `${head}${node.visibility ? `${node.visibility} ` : ""}mod ${node.name} {\n${indented}\n}` });
    deletes.add(file);
    notes.push(`folded the mount tree file ${relativePosix(rootDir, file)} into the root module ${node.name}`);
  }
  let out = rootSource;
  for (const edit of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return { rootSource: out, deletes, notes };
}
