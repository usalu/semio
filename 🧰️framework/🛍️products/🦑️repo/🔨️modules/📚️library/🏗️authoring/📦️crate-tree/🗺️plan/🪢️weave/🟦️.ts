import type { DeclCut, RootEdit, WovenExport } from "../🧾️model/🟦️.ts";
import type { RustModuleNode } from "../../🌳️module-tree/🟦️.ts";

/** 🔍️ Trait and self type names of one `impl` item. */
type WeaveNode = { crate: string | null; children: Map<string, WeaveNode> };

/** 🧵️ Rewrites the artifact root so every module whose mount was cut out of its parent is exported again at its original path: module-level `pub use` and, where a module has woven children, a local `pub mod` that glob-re-exports the module of the crate that owns it (a local module shadows the glob, so no path changes). */
export function weaveRootEdits(rootEdits: readonly RootEdit[], wrapperNodes: ReadonlyMap<string, RustModuleNode>, weave: readonly WovenExport[], placement: ReadonlyMap<string, string>, fileCuts: readonly DeclCut[] = [], gates: ReadonlyMap<string, readonly string[]> = new Map()): { edits: RootEdit[]; cuts: DeclCut[]; problems: string[] } {
  const edits: RootEdit[] = [...rootEdits.map((edit) => ({ ...edit })), ...fileCuts.map((cut) => ({ start: cut.start, end: cut.end, text: cut.text, anchor: cut.anchor }))];
  const cutCount = fileCuts.length;
  const problems: string[] = [];
  const prefixOf = (eff: readonly string[], candidate: readonly string[]): boolean => candidate.length < eff.length && candidate.every((segment, index) => segment === eff[index]);
  const groups = new Map<string, { anchorIndex: number | null; wrapper: RustModuleNode | null; anchorEff: readonly string[]; root: WeaveNode }>();
  for (const entry of [...weave].sort((left, right) => left.eff.join("::").localeCompare(right.eff.join("::")))) {
    let best: { length: number; anchorIndex: number | null; wrapper: RustModuleNode | null; eff: readonly string[] } | null = null;
    edits.forEach((edit, index) => { if (edit.anchor && prefixOf(entry.eff, edit.anchor.eff) && (!best || edit.anchor.eff.length > best.length)) best = { length: edit.anchor.eff.length, anchorIndex: index, wrapper: null, eff: edit.anchor.eff }; });
    for (const [key, node] of wrapperNodes) {
      const eff = key.split("::");
      if (prefixOf(entry.eff, eff) && (!best || eff.length > best.length)) best = { length: eff.length, anchorIndex: null, wrapper: node, eff };
    }
    if (!best) { problems.push(`cannot weave ${entry.eff.join("::")}: no root module or wrapper encloses it`); continue; }
    const key = `${best.anchorIndex ?? "w"}:${best.eff.join("::")}`;
    const group = groups.get(key) ?? { anchorIndex: best.anchorIndex, wrapper: best.wrapper, anchorEff: best.eff, root: { crate: null, children: new Map() } };
    groups.set(key, group);
    let cursor = group.root;
    entry.eff.slice(best.length).forEach((segment, index, all) => {
      let next = cursor.children.get(segment);
      if (!next) cursor.children.set(segment, (next = { crate: null, children: new Map() }));
      if (index === all.length - 1) next.crate = entry.crate;
      cursor = next;
    });
  }
  const render = (node: WeaveNode, path: readonly string[], indent: string): string[] => {
    const lines: string[] = [];
    for (const [name, child] of [...node.children].sort((left, right) => Buffer.compare(Buffer.from(left[0]), Buffer.from(right[0])))) {
      const here = [...path, name];
      const gate = (gates.get(here.join("::")) ?? []).map((text) => `${indent}${text}`);
      if (!child.children.size && child.crate) { lines.push(...gate, `${indent}pub use ${child.crate}::${here.join("::")};`); continue; }
      const owner = child.crate ?? placement.get(here.join("::")) ?? null;
      if (!owner) { problems.push(`cannot weave below ${here.join("::")}: no crate owns that module`); continue; }
      lines.push(...gate, `${indent}pub mod ${name} {`, `${indent}    pub use ${owner}::${here.join("::")}::*;`, ...render(child, here, `${indent}    `), `${indent}}`);
    }
    return lines;
  };
  const inserts: RootEdit[] = [];
  for (const group of groups.values()) {
    const lines = render(group.root, group.anchorEff, group.anchorIndex === null ? "    " : "");
    if (!lines.length) continue;
    if (group.anchorIndex !== null) {
      const edit = edits[group.anchorIndex]!;
      const anchor = edit.anchor!;
      const anchorGates = (gates.get(anchor.eff.join("::")) ?? []).map((gate) => `${gate}\n`).join("");
      const head = anchor.component ? edit.text : `${anchorGates}${anchor.visibility} mod ${anchor.eff.at(-1)} {\n    pub use ${anchor.crate}::${anchor.eff.join("::")}::*;\n${lines.map((line) => `    ${line}`).join("\n")}\n}`;
      edits[group.anchorIndex] = { ...edit, text: anchor.component ? `${head}\n${lines.join("\n")}` : head };
    } else {
      const node = group.wrapper!;
      inserts.push({ start: node.bodyEnd, end: node.bodyEnd, text: `${lines.join("\n")}\n` });
    }
  }
  const rootPart = edits.slice(0, rootEdits.length);
  const cutPart = edits.slice(rootEdits.length, rootEdits.length + cutCount).map((edit, index) => ({ ...fileCuts[index]!, text: edit.text }));
  return { edits: [...rootPart, ...inserts], cuts: cutPart, problems };
}
