import { resolve } from "node:path";
import type { RustModuleNode } from "../../🌳️module-tree/🟦️.ts";
import { rustTokenPairs, rustTokens } from "@semio-tech/compiler-syntax-rust-ts";
import { flattenUse } from "../../🧬️mutation-families/🌳️use-tree/🟦️.ts";

/** 🧭️ Absolute directory that nested `#[path]` attributes of an inline module resolve against. */
export function inlineModuleBase(parentBase: string, node: RustModuleNode): string {
  return resolve(parentBase, node.pathAttribute ?? node.name);
}

/** 📂️ Absolute file of an external module declared in a file or inline scope. */
export function externalModuleFile(fileDir: string, inlineBase: string, inFileScope: boolean, node: RustModuleNode): string | null {
  return node.pathAttribute === null ? null : resolve(inFileScope ? fileDir : inlineBase, node.pathAttribute);
}

/** 🔗️ Chains of `crate::a::b` paths in one source text; group members yield one chain per first segment. */
export function crateRelativeChains(source: string): string[][] {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const chains: string[][] = [];
  for (let index = 0; index + 2 < tokens.length; index += 1) {
    if (tokens[index]!.text !== "crate" || tokens[index + 1]!.text !== "::" || (index > 0 && tokens[index - 1]!.text === "::" && tokens[index - 2]?.kind === "identifier" && tokens[index - 2]!.text !== "self")) continue;
    if (tokens[index - 1]?.text === "extern") continue;
    let cursor = index + 2;
    const chain: string[] = [];
    while (tokens[cursor]?.kind === "identifier") {
      chain.push(tokens[cursor]!.text);
      if (tokens[cursor + 1]?.text === "::" && tokens[cursor + 2]?.kind === "identifier") cursor += 2;
      else { cursor += 1; break; }
    }
    const grouped = tokens[cursor]?.text === "::" && tokens[cursor + 1]?.text === "{";
    if (chain.length && !grouped) chains.push(chain);
    if (tokens[cursor - 0]?.text === "::" && tokens[cursor + 1]?.text === "{") {
      const close = pairs.get(cursor + 1);
      if (close !== undefined) groupHeads(tokens, pairs, cursor + 2, close, chain, chains);
    } else if (!chain.length && tokens[index + 2]?.text === "{") {
      const close = pairs.get(index + 2);
      if (close !== undefined) groupHeads(tokens, pairs, index + 3, close, [], chains);
    }
  }
  return chains;
}

function groupHeads(tokens: ReturnType<typeof rustTokens>, pairs: ReadonlyMap<number, number>, from: number, to: number, prefix: readonly string[], chains: string[][]): void {
  for (let cursor = from; cursor < to;) {
    if (tokens[cursor]!.text === ",") { cursor += 1; continue; }
    const chain = [...prefix];
    let step = cursor;
    while (tokens[step]?.kind === "identifier" && step < to) {
      chain.push(tokens[step]!.text);
      if (tokens[step + 1]?.text === "::" && step + 2 < to) {
        if (tokens[step + 2]!.text === "{") {
          const close = pairs.get(step + 2);
          if (close !== undefined) groupHeads(tokens, pairs, step + 3, close, chain, chains);
          step = (close ?? step + 2) + 1;
          chain.length = 0;
          break;
        }
        step += 2;
      } else { step += 1; break; }
    }
    if (chain.length > prefix.length) chains.push(chain);
    while (step < to && tokens[step]!.text !== ",") step = (pairs.get(step) !== undefined && pairs.get(step)! > step) ? pairs.get(step)! + 1 : step + 1;
    cursor = step;
  }
}

/** 🔗️ Chains that go through a `use crate::..::name;` alias: `name::tail` is `crate::..::name::tail`. */
export function aliasChains(source: string): string[][] {
  const aliases = new Map<string, string[]>();
  const tokens = rustTokens(source);
  for (let at = 0; at < tokens.length; at += 1) {
    if (tokens[at]!.text !== "use" || ["::", ".", "$"].includes(tokens[at - 1]?.text ?? "")) continue;
    let end = at;
    while (end < tokens.length && tokens[end]!.text !== ";") end += 1;
    if (end >= tokens.length) break;
    for (const entry of flattenUse(source.slice(tokens[at]!.start, tokens[end]!.end))) if (entry.path[0] === "crate" && entry.path.length > 1 && entry.leaf !== "*") aliases.set(entry.leaf, entry.path.slice(1));
    at = end;
  }
  if (!aliases.size) return [];
  const out: string[][] = [];
  for (let i = 0; i + 2 < tokens.length; i += 1) {
    const base = aliases.get(tokens[i]!.text);
    if (!base || tokens[i]!.kind !== "identifier" || tokens[i + 1]!.text !== "::" || ["::", ".", "use"].includes(tokens[i - 1]?.text ?? "")) continue;
    const chain = [...base];
    let k = i + 2;
    while (tokens[k]?.kind === "identifier") { chain.push(tokens[k]!.text); if (tokens[k + 1]?.text !== "::") break; k += 2; }
    out.push(chain);
  }
  return out;
}

/** 🪞️ Root inline modules that only re-export `crate::`/`super::` paths (`pub mod schema { pub use super::a::b::*; }`): the paths each one aliases and the text to copy into every crate that goes through it. */
export function rootAliasModules(source: string, modules: readonly RustModuleNode[]): Map<string, { targets: string[][]; text: string; nested: ReadonlySet<string> }> {
  const out = new Map<string, { targets: string[][]; text: string; nested: ReadonlySet<string> }>();
  const targetsOf = (node: RustModuleNode): string[][] | null => {
    if (!node.inline || node.pathAttribute !== null || !node.items.length || node.items.some((item) => item.kind !== "use")) return null;
    const targets: string[][] = [];
    for (const item of node.items) {
      const match = /^(?:pub(?:\([^)]*\))?\s+)?use\s+(?:crate|super)::([A-Za-z0-9_]+(?:::[A-Za-z0-9_]+)*)(?:::\*)?\s*;$/u.exec(source.slice(item.declStart, item.end).trim());
      if (match) targets.push(match[1]!.split("::")); else return null;
    }
    return targets.length ? targets : null;
  };
  for (const node of modules) {
    const targets = targetsOf(node);
    if (!targets) continue;
    const key = (rows: string[][]) => rows.map((row) => row.join("::")).sort().join("|");
    const nested = node.children.map((child) => ({ child, targets: targetsOf(child) }));
    if (nested.some((entry) => entry.targets === null || key(entry.targets) !== key(targets))) continue;
    out.set(node.name, { targets, text: source.slice(node.start, node.end), nested: new Set(nested.map((entry) => entry.child.name)) });
  }
  return out;
}

/** 🔗️ Strongly connected components of a graph where an edge means "depends on"; dependencies come before their dependents. */
export function stronglyConnected(nodes: readonly string[], edges: ReadonlyMap<string, ReadonlySet<string>>): string[][] {
  const index = new Map<string, number>();
  const low = new Map<string, number>();
  const onStack = new Set<string>();
  const stack: string[] = [];
  const out: string[][] = [];
  let counter = 0;
  const visit = (node: string): void => {
    index.set(node, counter);
    low.set(node, counter);
    counter += 1;
    stack.push(node);
    onStack.add(node);
    for (const next of edges.get(node) ?? []) {
      if (!index.has(next)) { visit(next); low.set(node, Math.min(low.get(node)!, low.get(next)!)); }
      else if (onStack.has(next)) low.set(node, Math.min(low.get(node)!, index.get(next)!));
    }
    if (low.get(node) !== index.get(node)) return;
    const members: string[] = [];
    for (let member = stack.pop()!; ; member = stack.pop()!) {
      onStack.delete(member);
      members.push(member);
      if (member === node) break;
    }
    out.push(members);
  };
  for (const node of nodes) if (!index.has(node)) visit(node);
  return out;
}

/** 🧷️ Prefixes `crate::` to every path that starts with one of the given root module names, which is how root items keep resolving after they move into a module of their own; the entries of a `use` group that follows a path prefix continue that path and are never heads. */
export function qualifyModuleHeads(text: string, modules: ReadonlySet<string>, exclude: ReadonlySet<string> = new Set()): string {
  const tokens = rustTokens(text);
  const at: number[] = [];
  let inUse = false;
  const groups: boolean[] = [];
  for (let k = 0; k + 1 < tokens.length; k += 1) {
    const token = tokens[k]!;
    const previous = tokens[k - 1]?.text;
    if (token.text === "use" && previous !== "::" && previous !== ".") { inUse = true; groups.length = 0; }
    else if (inUse && token.text === ";") inUse = false;
    else if (inUse && token.text === "{") groups.push(previous === "::");
    else if (inUse && token.text === "}") groups.pop();
    const insideGroupOfPath = inUse && (previous === "{" || previous === ",") && groups.at(-1) === true;
    if (token.kind === "identifier" && tokens[k + 1]!.text === "::" && previous !== "::" && previous !== "." && previous !== "$" && !insideGroupOfPath && modules.has(token.text) && !exclude.has(token.text)) at.push(token.start);
  }
  let out = text;
  for (const position of at.sort((a, b) => b - a)) out = `${out.slice(0, position)}crate::${out.slice(position)}`;
  return out;
}

/** 🔝️ Absolute module chains of the `super::` paths written directly in the body of one inline module (nested inline modules are walked as modules of their own). */
export function superChains(text: string, from: number, to: number, excluded: readonly (readonly [number, number])[], base: readonly string[]): string[][] {
  const tokens = rustTokens(text);
  const pairs = rustTokenPairs(tokens);
  const out: string[][] = [];
  for (let i = 0; i < tokens.length; i += 1) {
    const token = tokens[i]!;
    if (token.text !== "super" || token.start < from || token.start >= to || tokens[i - 1]?.text === "::" || excluded.some(([a, b]) => token.start >= a && token.start < b)) continue;
    let up = 1;
    let cursor = i;
    while (tokens[cursor + 1]?.text === "::" && tokens[cursor + 2]?.text === "super") { up += 1; cursor += 2; }
    if (up > base.length) continue;
    const chain = base.slice(0, base.length - up);
    let step = cursor + 1;
    while (tokens[step]?.text === "::" && tokens[step + 1]?.kind === "identifier") { chain.push(tokens[step + 1]!.text); step += 2; }
    if (tokens[step]?.text === "::" && tokens[step + 1]?.text === "{") {
      const close = pairs.get(step + 1);
      if (close !== undefined) groupHeads(tokens, pairs, step + 2, close, chain, out);
    } else out.push(chain);
  }
  return out;
}

/** 🪜️ Relative paths of two or more segments (`child::grandchild::item`) that start at a name written without `crate::`, `self::` or `super::`; they reach the grandchildren of the module the text lives in. */
export function relativeHeadChains(text: string): string[][] {
  const tokens = rustTokens(text);
  const out: string[][] = [];
  for (let i = 0; i + 2 < tokens.length; i += 1) {
    const token = tokens[i]!;
    const previous = tokens[i - 1]?.text;
    if (token.kind !== "identifier" || ["crate", "self", "super"].includes(token.text) || tokens[i + 1]!.text !== "::" || previous === "::" || previous === "." || previous === "$") continue;
    const chain = [token.text];
    let cursor = i;
    while (tokens[cursor + 1]?.text === "::" && tokens[cursor + 2]?.kind === "identifier") { chain.push(tokens[cursor + 2]!.text); cursor += 2; }
    if (chain.length >= 2) out.push(chain);
  }
  return out;
}

/** 🧭️ Names a source uses as the head of a path (`name::..`, `self::name`, `super::name`), which is how a module reaches a module it mounts. */
export function pathHeadNames(text: string): Set<string> {
  const tokens = rustTokens(text);
  const out = new Set<string>();
  let inUse = false;
  for (let i = 0; i + 1 < tokens.length; i += 1) {
    const token = tokens[i]!;
    if (token.text === "use") inUse = true;
    else if (token.text === ";") inUse = false;
    if (token.kind !== "identifier") continue;
    const previous = tokens[i - 1]?.text;
    const next = tokens[i + 1]!.text;
    if (next === "::" && previous !== "." && (previous !== "::" || ["self", "super"].includes(tokens[i - 2]?.text ?? ""))) out.add(token.text);
    else if (previous === "use" && (next === ";" || next === "as")) out.add(token.text);
    else if (inUse && (previous === "{" || previous === ",") && (next === "," || next === "}")) out.add(token.text);
  }
  return out;
}

/** 🧬️ The parsed head of one `impl`: its trait, its self type, and the identifiers inside the generic arguments of the trait path (a local type there satisfies the orphan rule). */
export function implHeader(text: string): { trait: string | null; self: string | null; traitArgs: string[] } | null {
  const tokens = rustTokens(text);
  const start = tokens.findIndex((token) => token.text === "impl");
  if (start < 0) return null;
  let cursor = start + 1;
  if (tokens[cursor]?.text === "<") {
    let depth = 0;
    for (; cursor < tokens.length; cursor += 1) {
      if (tokens[cursor]!.text === "<") depth += 1;
      else if (tokens[cursor]!.text === ">" || tokens[cursor]!.text === ">>") { depth -= tokens[cursor]!.text.length; if (depth <= 0) { cursor += 1; break; } }
    }
  }
  let end = cursor;
  let depth = 0;
  let forAt = -1;
  for (; end < tokens.length; end += 1) {
    const token = tokens[end]!.text;
    if (token === "<") depth += 1;
    else if (token === ">" || token === ">>") depth -= token.length;
    else if (depth === 0 && token === "for" && forAt < 0) forAt = end;
    else if (depth === 0 && (token === "{" || token === "where")) break;
  }
  const lastIdent = (from: number, to: number): string | null => {
    let name: string | null = null;
    let nested = 0;
    for (let i = from; i < to; i += 1) {
      const token = tokens[i]!;
      if (token.text === "<") nested += 1;
      else if (token.text === ">" || token.text === ">>") nested -= token.text.length;
      else if (nested === 0 && token.kind === "identifier" && !["dyn", "mut", "impl", "for"].includes(token.text)) name = token.text;
    }
    return name;
  };
  const argsOf = (from: number, to: number): string[] => {
    const out: string[] = [];
    let nested = 0;
    for (let i = from; i < to; i += 1) {
      const token = tokens[i]!;
      if (token.text === "<") nested += 1;
      else if (token.text === ">" || token.text === ">>") nested -= token.text.length;
      else if (nested > 0 && token.kind === "identifier") out.push(token.text);
    }
    return out;
  };
  if (forAt >= 0) return { trait: lastIdent(cursor, forAt), self: lastIdent(forAt + 1, end), traitArgs: argsOf(cursor, forAt) };
  return { trait: null, self: lastIdent(cursor, end), traitArgs: [] };
}
