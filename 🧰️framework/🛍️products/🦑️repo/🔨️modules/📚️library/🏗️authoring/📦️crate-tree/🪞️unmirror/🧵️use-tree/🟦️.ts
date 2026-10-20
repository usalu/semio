/** 🧵️ Use trees of Rust `use` statements: scanning every statement of a source (module level and function local), flattening it to leaves and rendering leaves back. */
import { rustTokenPairs, rustTokens, type RustToken } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";

/** 🍃️ One leaf of a use tree: the full path, the optional alias and whether it imports a glob. */
export type UseLeaf = { path: string[]; alias: string | null; glob: boolean };

/** 🧾️ One `use` statement with its character range (attributes included), visibility, attribute texts and leaves. */
export type UseStatement = { start: number; end: number; keywordStart: number; visibility: string; attributes: string[]; leaves: UseLeaf[] };

/** 🧱️ A tokenised source with matched delimiters, shared by every scan of one file. */
export type ScannedSource = { text: string; tokens: RustToken[]; pairs: ReadonlyMap<number, number> };

/** 🔭️ Tokenises a source once for all scans. */
export function scanSource(text: string): ScannedSource {
  const tokens = rustTokens(text);
  return { text, tokens, pairs: rustTokenPairs(tokens) };
}

const identifier = (token: RustToken | undefined): boolean => token?.kind === "identifier";

/** 🌿️ Flattens the use tree between token `from` (inclusive) and `to` (exclusive) below `prefix`; `null` when the tree is malformed. */
function flatten(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, from: number, to: number, prefix: readonly string[]): UseLeaf[] | null {
  let cursor = from;
  const path = [...prefix];
  if (tokens[cursor]?.text === "::") cursor += 1;
  for (;;) {
    const token = tokens[cursor];
    if (!token || cursor >= to) return null;
    if (token.text === "{") {
      const close = pairs.get(cursor);
      if (close === undefined || close >= to) return null;
      const leaves: UseLeaf[] = [];
      let begin = cursor + 1;
      for (let index = cursor + 1; index <= close; index += 1) {
        const text = tokens[index]!.text;
        if (text === "{" || text === "(" || text === "[") { index = pairs.get(index) ?? index; continue; }
        if (text !== "," && index !== close) continue;
        if (index > begin) {
          const part = flatten(tokens, pairs, begin, index, path);
          if (!part) return null;
          leaves.push(...part);
        }
        begin = index + 1;
      }
      return leaves;
    }
    if (token.text === "*") return cursor + 1 === to ? [{ path, alias: null, glob: true }] : null;
    if (!identifier(token)) return null;
    path.push(token.text);
    cursor += 1;
    if (tokens[cursor]?.text === "::" && cursor < to) { cursor += 1; continue; }
    let alias: string | null = null;
    if (tokens[cursor]?.text === "as" && cursor + 1 < to) { alias = tokens[cursor + 1]!.text; cursor += 2; }
    if (cursor !== to) return null;
    if (path.at(-1) === "self") path.pop();
    return [{ path, alias, glob: false }];
  }
}

/** 🔎️ Finds every `use` statement of a source, including the ones inside function bodies; a statement that contains a macro variable or cannot be parsed is skipped. */
export function scanUseStatements(source: ScannedSource): UseStatement[] {
  const { tokens, pairs } = source;
  const found: UseStatement[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]!.text !== "use" || tokens[index]!.kind !== "identifier" || tokens[index - 1]?.text === "." || tokens[index - 1]?.text === "::" || tokens[index - 1]?.text === "$") continue;
    let end = index + 1;
    while (end < tokens.length && tokens[end]!.text !== ";") { const close = pairs.get(end); end = tokens[end]!.text === "{" && close !== undefined ? close + 1 : end + 1; }
    if (end >= tokens.length) continue;
    const leaves = flatten(tokens, pairs, index + 1, end, []);
    if (!leaves) continue;
    let first = index;
    let visibility = "";
    if (tokens[first - 1]?.text === "pub") { first -= 1; visibility = "pub"; }
    else if (tokens[first - 1]?.text === ")" && pairs.has(first - 1) && tokens[pairs.get(first - 1)! - 1]?.text === "pub") { const open = pairs.get(first - 1)!; visibility = source.text.slice(tokens[open - 1]!.start, tokens[first - 1]!.end); first = open - 1; }
    const keywordStart = tokens[first]!.start;
    const attributes: string[] = [];
    let cursor = first;
    while (tokens[cursor - 1]?.text === "]" && pairs.has(cursor - 1)) {
      const open = pairs.get(cursor - 1)!;
      if (tokens[open - 1]?.text !== "#") break;
      attributes.unshift(source.text.slice(tokens[open - 1]!.start, tokens[cursor - 1]!.end));
      cursor = open - 1;
    }
    found.push({ start: tokens[cursor]!.start, end: tokens[end]!.end, keywordStart, visibility, attributes, leaves });
    index = end;
  }
  return found;
}

/** 🏷️ The name a leaf binds, `null` for globs and underscore imports. */
export function leafBinding(leaf: UseLeaf): string | null {
  if (leaf.glob) return null;
  const name = leaf.alias ?? leaf.path.at(-1) ?? null;
  return name === "_" ? null : name;
}

/** 🖨️ Renders leaves as `use` statements with the given attributes and visibility, grouping leaves of one parent path into braces. */
export function renderUseLeaves(leaves: readonly UseLeaf[], visibility: string, attributes: readonly string[], indent: string): string {
  const groups = new Map<string, string[]>();
  for (const leaf of leaves) {
    const parent = (leaf.glob ? leaf.path : leaf.path.slice(0, -1)).join("::");
    const name = leaf.glob ? "*" : `${leaf.path.at(-1)}${leaf.alias ? ` as ${leaf.alias}` : ""}`;
    const bucket = groups.get(parent) ?? [];
    if (!bucket.includes(name)) bucket.push(name);
    groups.set(parent, bucket);
  }
  const head = `${visibility ? `${visibility} ` : ""}use `;
  const lines: string[] = [];
  for (const [parent, names] of groups) {
    const body = names.length === 1 ? (parent ? `${parent}::${names[0]}` : names[0]!) : `${parent ? `${parent}::` : ""}{${names.join(", ")}}`;
    lines.push(`${[...attributes.map((attribute) => `${attribute}\n${indent}`), head].join("")}${body};`);
  }
  return lines.join(`\n${indent}`);
}
