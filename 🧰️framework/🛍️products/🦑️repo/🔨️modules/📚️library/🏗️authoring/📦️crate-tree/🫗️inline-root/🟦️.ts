/** 📦️ Inline root: module exporting `inlineRootFileModules`. */
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { rustTokens } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { parseRustSourceTree } from "../🌳️module-tree/🟦️.ts";
import type { TestTransformResult } from "../🧪️test-closure/🟦️.ts";

/** 🫗️ Brings the file modules that the artifact root mounts and glob-re-exports back into the root as regions, so the planner can host their items like any other root item: a root file module is kept by the facade and no lower crate can reach it. Only plain item files qualify: a `use super::*;` as the only parent reference, no inner attributes, no mounts or path-relative macros. */
export function inlineRootFileModules(previous: TestTransformResult | null, rootFile: string, originalRootSource: string): TestTransformResult {
  const result: TestTransformResult = previous ?? { rootSource: originalRootSource, edits: new Map(), creates: new Map(), deletes: new Set(), notes: [] };
  const readText = (path: string): string | null => (result.deletes.has(path) ? null : result.creates.get(path) ?? result.edits.get(path) ?? (existsSync(path) ? readFileSync(path, "utf8") : null));
  const source = result.rootSource;
  const tree = parseRustSourceTree(source);
  const edits: { start: number; end: number; text: string }[] = [];
  for (const node of tree.modules) {
    if (node.inline || node.pathAttribute === null || node.conditionalTest) continue;
    const file = resolve(dirname(rootFile), node.pathAttribute);
    const text = readText(file);
    if (text === null || /^#!\[/mu.test(text) || /#\[path|include_(?:str|bytes)!|\bfile!|module_path!|^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s/mu.test(text)) continue;
    const glob = tree.items.find((item) => item.kind === "use" && new RegExp(`^(?:pub(?:\\([^)]*\\))?\\s+)?use\\s+${node.name}::\\*\\s*;$`, "u").test(source.slice(item.declStart, item.end).trim()));
    if (!glob) continue;
    const body = text.replace(/^use super::\*;[ \t]*\n/mu, "");
    if (rustTokens(body).some((token) => token.text === "super")) continue;
    const header = body.split("\n").map((line) => (line.startsWith("//!") ? `//${line.slice(3)}` : line)).join("\n").trim();
    const regions = /^\/\/#region /mu.test(header);
    edits.push({ start: node.start, end: node.end, text: regions ? header : `//#region 🧩️${node.name}\n${header}\n//#endregion 🧩️${node.name}` });
    edits.push({ start: glob.start, end: glob.end, text: "" });
    result.deletes.add(file);
    result.notes.push(`inlined the root file module ${node.name} into the artifact root`);
  }
  let out = source;
  for (const edit of edits.sort((left, right) => right.start - left.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  result.rootSource = out;
  return result;
}
