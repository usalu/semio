/** 🧬️ Support hoist: module exporting `SupportHoist`, `inherentSelf`, `hoistAggregateSupport`. */
import { existsSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { rustTokens } from "@semio-tech/compiler-syntax-rust-ts";
import { parseRustSourceTree, type RustSourceItem } from "../../🌳️module-tree/🟦️.ts";

/** 🔭️ One name an aggregate module puts in scope: an import, a public child module or an item of its own. */
export type AggregateName = { readonly kind: "import" | "module"; readonly statement: string } | { readonly kind: "item" };

/** 🔭️ What the namespace of an aggregate module offers to a leaf below it: its imports (as absolute statements), its public child modules and its own items. */
export type AggregateScope = { readonly names: ReadonlyMap<string, AggregateName>; readonly glob: boolean };

/** 🧰️ A support module cut out of an aggregate file: the new aggregate text, the new file and the names it now hosts. */
export type SupportHoist = { readonly aggregateText: string; readonly supportFile: string; readonly supportText: string; readonly moved: readonly string[] };

const MOVABLE: ReadonlySet<RustSourceItem["kind"]> = new Set(["fn", "const", "static", "struct", "enum", "type", "trait"]);
const FOLDER_EMOJIS = ["\u{1F9F0}\u{FE0F}", "\u{1F6E0}\u{FE0F}", "\u{1F527}\u{FE0F}", "\u{1F9F2}\u{FE0F}"];

function leadStart(text: string, start: number): number {
  let cursor = start;
  for (;;) {
    const lineStart = text.lastIndexOf("\n", cursor - 2) + 1;
    const previous = text.slice(lineStart, cursor - 1);
    if (cursor <= 0 || !/^\s*\/\/(?!!)/u.test(previous) || /^\s*\/\/\s*#(?:end)?region\b/u.test(previous)) return cursor;
    cursor = lineStart;
  }
}

/** 🏷️ The type an inherent `impl` header is written for (`impl Name {`, `impl<T> Name<T> {`), or null for trait impls. */
export function inherentSelf(head: string): string | null {
  const flat = head.replace(/\s+/gu, " ").trim();
  if (/\sfor\s/u.test(flat)) return null;
  return /^(?:unsafe )?impl ?(?:<[^{}]*?>)? ?([A-Za-z_][A-Za-z0-9_]*)(?: ?<.*>)?(?: where .*)?$/u.exec(flat)?.[1] ?? null;
}

function usedNames(text: string): Set<string> {
  const tokens = rustTokens(text);
  const out = new Set<string>();
  for (const [index, token] of tokens.entries()) if (token.kind === "identifier" && tokens[index - 1]?.text !== ".") out.add(token.text);
  return out;
}

function declaredIn(text: string): Set<string> {
  const tree = parseRustSourceTree(text);
  const out = new Set<string>();
  for (const item of tree.items) if (item.name) out.add(item.name);
  for (const node of tree.modules) out.add(node.name);
  return out;
}

function pickFolder(directory: string): string | null {
  const taken = existsSync(directory) ? readdirSync(directory).map((name) => Array.from(name)[0] ?? "") : [];
  const free = FOLDER_EMOJIS.find((emoji) => !taken.includes(Array.from(emoji)[0]!));
  return free === undefined ? null : `${free}support`;
}

function dropEmptyRegions(text: string): string {
  return text.replace(/^[ \t]*\/\/#region[ \t]+([^\n]*?)[ \t]*\n(?:[ \t]*\n)*[ \t]*\/\/#endregion[ \t]+\1[ \t]*\n?/gmu, "");
}

/** 🧰️ Moves the helper functions and constants of an aggregate module that its mutation leaves call into a `support` child module, so the leaves can import them from below the aggregate instead of reaching up into it. Items that mention the aggregate, one of its child modules or a test-only item stay where they are and keep their leaves coupled. */
export function hoistAggregateSupport(input: { readonly aggregateFile: string; readonly aggregateText: string; readonly aggregateName: string; readonly leafTexts: readonly string[]; readonly scope: AggregateScope }): SupportHoist | null {
  const { aggregateFile, aggregateText: text, aggregateName, scope } = input;
  if (scope.glob || parseRustSourceTree(text).modules.some((node) => node.name === "support")) return null;
  const tree = parseRustSourceTree(text);
  const testGated = (item: RustSourceItem): boolean => item.attributeTexts.some((attribute) => /cfg\(\s*test\s*\)/u.test(attribute));
  const candidates = new Map<string, RustSourceItem>();
  for (const item of tree.items) if (item.name && MOVABLE.has(item.kind) && !testGated(item) && item.name !== aggregateName && !candidates.has(item.name)) candidates.set(item.name, item);
  const moduleNames = new Set(tree.modules.map((node) => node.name));
  const otherNames = new Set<string>([aggregateName, ...moduleNames]);
  for (const item of tree.items) if (item.name && !candidates.has(item.name) && item.kind !== "use") otherNames.add(item.name);
  const headOf = (item: RustSourceItem): string => text.slice(item.declStart, Math.min(item.end, item.declStart + 400)).split("{")[0]!;
  const inherentImpls = new Map<string, RustSourceItem[]>();
  const implHeads: string[] = [];
  for (const item of tree.items) {
    if (item.kind !== "impl") continue;
    const head = headOf(item);
    const self = inherentSelf(head);
    if (self !== null && candidates.has(self) && !testGated(item)) inherentImpls.set(self, [...(inherentImpls.get(self) ?? []), item]);
    else implHeads.push(head);
  }
  const bodyOf = (item: RustSourceItem): string => text.slice(item.start, item.end);
  const references = new Map<string, Set<string>>();
  const immovable = new Set<string>();
  for (const [name, item] of candidates) {
    const names = new Set<string>([...usedNames(bodyOf(item)), ...(inherentImpls.get(name) ?? []).flatMap((impl) => [...usedNames(bodyOf(impl))])]);
    const refs = new Set<string>();
    for (const used of names) {
      if (used === name) continue;
      if (candidates.has(used)) refs.add(used);
      else if (otherNames.has(used)) immovable.add(name);
    }
    if (implHeads.some((head) => new RegExp(`\\b${name}\\b`, "u").test(head))) immovable.add(name);
    references.set(name, refs);
  }
  for (let changed = true; changed;) {
    changed = false;
    for (const [name, refs] of references) if (!immovable.has(name) && [...refs].some((ref) => immovable.has(ref))) { immovable.add(name); changed = true; }
  }
  const needed = new Set<string>();
  for (const leaf of input.leafTexts) {
    const own = declaredIn(leaf);
    for (const used of usedNames(leaf)) if (candidates.has(used) && !immovable.has(used) && !own.has(used)) needed.add(used);
  }
  const moved = new Set<string>();
  const visit = (name: string): void => {
    if (moved.has(name)) return;
    moved.add(name);
    for (const ref of references.get(name) ?? []) visit(ref);
  };
  for (const name of needed) visit(name);
  if (!moved.size) return null;
  const folder = pickFolder(dirname(aggregateFile));
  if (folder === null) return null;
  const ordered = [...moved].map((name) => candidates.get(name)!).sort((a, b) => a.start - b.start);
  const bodies: string[] = [];
  const cuts: { start: number; end: number }[] = [];
  const reexports = new Map<string, string[]>();
  const wanted = new Set<string>();
  for (const item of ordered) {
    const lead = leadStart(text, item.start);
    let end = item.end;
    if (text[end] === "\n") end += 1;
    cuts.push({ start: lead, end });
    const declaration = text.slice(item.declStart, item.end).replace(/^pub(?:\([^)]*\))?\s*/u, "");
    bodies.push(`${text.slice(lead, item.declStart)}pub ${declaration}`);
    reexports.set(item.visibility, [...(reexports.get(item.visibility) ?? []), item.name!]);
    for (const used of usedNames(bodyOf(item))) wanted.add(used);
    for (const impl of inherentImpls.get(item.name!) ?? []) {
      const implLead = leadStart(text, impl.start);
      let implEnd = impl.end;
      if (text[implEnd] === "\n") implEnd += 1;
      cuts.push({ start: implLead, end: implEnd });
      bodies.push(text.slice(implLead, impl.end));
      for (const used of usedNames(bodyOf(impl))) wanted.add(used);
    }
  }
  const imports: string[] = [];
  for (const name of [...wanted].sort()) {
    const entry = scope.names.get(name);
    if (!entry || moved.has(name)) continue;
    if (entry.kind === "import") imports.push(entry.statement);
  }
  const supportText = [`//! 🧰️ \`${aggregateName}\` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.`, "", ...(imports.length ? [...new Set(imports), ""] : []), bodies.join("\n\n"), ""].join("\n");
  let out = text;
  for (const cut of cuts.sort((a, b) => b.start - a.start)) out = out.slice(0, cut.start) + out.slice(cut.end);
  const firstMount = tree.modules.find((node) => !node.conditionalTest);
  const mount = [
    `#[path = "${folder}/🦀️.rs"]`,
    "pub mod support;",
    ...[...reexports].flatMap(([visibility, names]) => [...(visibility === "private" ? ["#[allow(unused_imports)]"] : []), `${visibility === "private" ? "" : `${visibility} `}use support::{${names.sort().join(", ")}};`]),
    "",
  ].join("\n");
  const cutBefore = (offset: number): number => cuts.filter((cut) => cut.end <= offset).reduce((sum, cut) => sum + (cut.end - cut.start), 0);
  const useEnd = tree.items.filter((item) => item.kind === "use").map((item) => item.end).at(-1) ?? 0;
  const anchor = firstMount ? firstMount.start : useEnd;
  const at = anchor - cutBefore(anchor);
  out = firstMount ? `${out.slice(0, at)}${mount}${out.slice(at)}` : `${out.slice(0, at)}\n${mount}${out.slice(at)}`;
  return { aggregateText: dropEmptyRegions(out), supportFile: join(dirname(aggregateFile), folder, "🦀️.rs"), supportText, moved: [...moved].sort() };
}
