import { rustTokenPairs, rustTokenSegments, rustTokens, type RustToken } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import { collectDecls, superReach, type ModuleDecl } from "../🧱️decl-tree/🟦️.ts";
import { absolutizeSuperPaths, declaredNames } from "../🧪️test-closure/🟢️harness/🟦️.ts";
import { identifierSet, parseRustSourceTree, useLeafNames, type RustSourceItem } from "../🌳️module-tree/🟦️.ts";
import { flattenUse, pruneUseLeaves } from "./🌳️use-tree/🟦️.ts";
import { hoistAggregateSupport, type AggregateName, type AggregateScope } from "./🧰️support-hoist/🟦️.ts";
import { absolutizeBeyond } from "../🪜️lift-kids/🟦️.ts";

/** 🎁️ One variant of a transparent mutation aggregate: the single-field tuple wrapping a leaf payload. */
export type MutationVariant = { readonly variant: string; readonly payload: string; readonly path: string };

/** 🧬️ A `#[derive(dsl::Mutations)]` enum and the leaf payloads it wraps. */
export type MutationAggregate = { readonly name: string; readonly file: string; readonly effPath: readonly string[]; readonly snapshot: string | null; readonly diff: string | null; readonly variants: readonly MutationVariant[] };

/** 🏗️ A leaf builder that returned the concrete aggregate and now stays generic over it. */
export type LeafBuilder = { readonly name: string; readonly params: string; readonly args: string; readonly chain: string };

/** 💼️ What one conversion run rewrote and what it left for hand work. */
export type MutationFamilyResult = {
  readonly edits: Map<string, string>;
  readonly creates: Map<string, string>;
  readonly notes: string[];
  readonly aggregates: readonly MutationAggregate[];
  readonly converted: readonly string[];
  readonly hand: readonly { readonly payload: string; readonly reason: string }[];
};

type Edit = { start: number; end: number; text: string };
type Fail = { readonly error: string };
type Rewritten = { readonly text: string; readonly bounds: Set<string>; readonly builders: LeafBuilder[] };
type LeafContext = {
  readonly agg: string;
  readonly variantChain: ReadonlyMap<string, string>;
  readonly builderChain: ReadonlyMap<string, string>;
  readonly leafChain: string;
  readonly payload: string;
  readonly snapshot: string | null;
  readonly diff: string | null;
  readonly diffResolved: string | null;
};

const isFail = (value: unknown): value is Fail => typeof value === "object" && value !== null && "error" in value;

function chainStart(tokens: readonly RustToken[], index: number): number {
  let cursor = index;
  while (tokens[cursor - 1]?.text === "::" && tokens[cursor - 2]?.kind === "identifier") cursor -= 2;
  return cursor;
}

/** 🧭️ The module path of a declaration without the private `component` mounts, so a path written into another file names only modules other crates can reach. */
function plainPath(eff: readonly string[]): string {
  return eff.filter((segment) => segment !== "component").join("::");
}

function lineOf(text: string, offset: number): number {
  return text.slice(0, offset).split("\n").length;
}

function lineText(text: string, offset: number): string {
  const start = text.lastIndexOf("\n", offset - 1) + 1;
  const end = text.indexOf("\n", offset);
  return text.slice(start, end < 0 ? text.length : end).trim().slice(0, 120);
}

function applyEdits(text: string, edits: readonly Edit[]): string {
  let out = text;
  for (const edit of [...edits].sort((left, right) => right.start - left.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return out;
}

function angleClose(tokens: readonly RustToken[], open: number): number {
  let depth = 0;
  for (let index = open; index < tokens.length; index += 1) {
    const text = tokens[index]!.text;
    if (text === "<") depth += 1;
    else if (text === ">") depth -= 1;
    else if (text === ">>") depth -= 2;
    if (depth <= 0) return index;
  }
  return -1;
}

/** 🔎️ Finds every `#[derive(.. Mutations ..)]` enum of single-field tuple variants among the given declarations. */
export function findMutationAggregates(decls: Iterable<ModuleDecl>, readText: (absolute: string) => string | null): MutationAggregate[] {
  const out: MutationAggregate[] = [];
  const seen = new Set<string>();
  for (const decl of decls) {
    if (decl.testOnly || decl.file === null || seen.has(decl.file)) continue;
    seen.add(decl.file);
    const text = readText(decl.file);
    if (text === null || !text.includes("Mutations")) continue;
    const tokens = rustTokens(text);
    const pairs = rustTokenPairs(tokens);
    for (let i = 2; i < tokens.length; i += 1) {
      if (tokens[i]!.text !== "Mutations" || tokens[i - 1]!.text !== "::" || tokens[i + 1]?.text === "::") continue;
      let open = i;
      while (open > 0 && tokens[open]!.text !== "(") open -= 1;
      if (tokens[open - 1]?.text !== "derive") continue;
      const close = pairs.get(open);
      if (close === undefined || close < i) continue;
      let snapshot: string | null = null;
      let diff: string | null = null;
      const enumAt = tokens.findIndex((token, index) => index > close && token.text === "enum");
      let scanFrom = open;
      while (scanFrom > 0 && open - scanFrom < 400 && tokens[scanFrom - 1]!.text !== "}" && tokens[scanFrom - 1]!.text !== ";") scanFrom -= 1;
      for (let k = scanFrom; k < (enumAt < 0 ? tokens.length : enumAt); k += 1) {
        if (tokens[k]!.text !== "mutations" || tokens[k + 1]?.text !== "(" || tokens[k - 1]?.text !== "[") continue;
        const end = pairs.get(k + 1)!;
        for (const [a, b] of rustTokenSegments(tokens, pairs, k + 2, end, ",")) {
          const key = tokens[a]?.text;
          if (tokens[a + 1]?.text !== "=") continue;
          const value = text.slice(tokens[a + 2]!.start, tokens[b - 1]!.end);
          if (key === "snapshot") snapshot = value;
          if (key === "diff") diff = value;
        }
      }
      const enumIndex = tokens.findIndex((token, index) => index > close && token.text === "enum");
      if (enumIndex < 0 || tokens[enumIndex + 1]?.kind !== "identifier" || tokens[enumIndex + 2]?.text !== "{") continue;
      const bodyClose = pairs.get(enumIndex + 2)!;
      const variants: MutationVariant[] = [];
      let supported = true;
      for (const [a, b] of rustTokenSegments(tokens, pairs, enumIndex + 3, bodyClose, ",")) {
        let k = a;
        while (tokens[k]?.text === "#") k = (pairs.get(k + 1) ?? k + 1) + 1;
        if (k >= b) continue;
        if (tokens[k]?.kind !== "identifier" || tokens[k + 1]?.text !== "(" || pairs.get(k + 1) !== b - 1) { supported = false; break; }
        let last = b - 2;
        while (tokens[last]?.kind !== "identifier" && last > k + 1) last -= 1;
        variants.push({ variant: tokens[k]!.text, payload: tokens[last]!.text, path: text.slice(tokens[k + 2]!.start, tokens[last]!.end).replace(/\s+/gu, "") });
      }
      if (supported && variants.length) out.push({ name: tokens[enumIndex + 1]!.text, file: decl.file, effPath: decl.eff, snapshot, diff, variants });
    }
  }
  return out;
}

function topLevelItemAt(items: readonly RustSourceItem[], offset: number): RustSourceItem | null {
  return items.find((item) => item.start <= offset && offset < item.end) ?? null;
}

type Classified = { readonly kind: "constructor"; readonly start: number; readonly end: number; readonly variant: string } | { readonly kind: "type"; readonly start: number; readonly end: number };

function classifyOccurrence(text: string, tokens: readonly RustToken[], index: number): Classified | Fail {
  const first = chainStart(tokens, index);
  const previous = tokens[first - 1]?.text;
  const next = tokens[index + 1]?.text;
  if (next === "::") {
    if (tokens[index + 2]?.kind === "identifier" && tokens[index + 3]?.text === "(") return { kind: "constructor", start: tokens[first]!.start, end: tokens[index + 3]!.end, variant: tokens[index + 2]!.text };
    return { error: `unsupported use of the aggregate path at line ${lineOf(text, tokens[index]!.start)}: ${lineText(text, tokens[index]!.start)}` };
  }
  if ((previous === "<" || previous === "<<") && next === "as") return { kind: "type", start: tokens[first]!.start, end: tokens[index]!.end };
  if (previous !== undefined && ["<", ",", "->", "&", "mut", ":"].includes(previous) && next !== undefined && [">", ">>", ",", "{", ";", ")", "where"].includes(next)) return { kind: "type", start: tokens[first]!.start, end: tokens[index]!.end };
  return { error: `unsupported use of the aggregate type at line ${lineOf(text, tokens[index]!.start)}: ${lineText(text, tokens[index]!.start)}` };
}

function boundOf(chain: string): string {
  return `From<${chain}>`;
}

function collectBuilderBounds(tokens: readonly RustToken[], from: number, to: number, ctx: LeafContext, bounds: Set<string>, ownName: string | null): void {
  for (let k = from; k < to; k += 1) {
    const token = tokens[k]!;
    if (token.kind !== "identifier" || tokens[k + 1]?.text !== "(" || tokens[k - 1]?.text === "." || tokens[k - 1]?.text === "fn" || token.text === ownName) continue;
    const chain = ctx.builderChain.get(token.text);
    if (chain) bounds.add(boundOf(chain));
  }
}

function rewriteFunction(text: string, tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, item: RustSourceItem, ctx: LeafContext, edits: Edit[], bounds: Set<string>): { builder: LeafBuilder | null } | Fail {
  const first = tokens.findIndex((token) => token.start >= item.declStart);
  let f = first;
  while (f < tokens.length && tokens[f]!.text !== "fn") f += 1;
  const last = tokens.findIndex((token) => token.start >= item.end);
  const end = last < 0 ? tokens.length : last;
  const name = tokens[f + 1];
  if (!name || name.kind !== "identifier") return { error: `unreadable fn at line ${lineOf(text, item.declStart)}` };
  if (tokens[f + 2]?.text === "<") return { error: `fn ${name.text} already has generics` };
  if (tokens[f + 2]?.text !== "(") return { error: `unreadable fn ${name.text}` };
  const close = pairs.get(f + 2)!;
  let body = close + 1;
  while (body < end && tokens[body]!.text !== "{") { if (tokens[body]!.text === "where") return { error: `fn ${name.text} already has a where clause` }; if (tokens[body]!.text === ";") return { error: `fn ${name.text} has no body` }; body += 1; }
  if (body >= end) return { error: `fn ${name.text} has no body` };
  const occurrences: number[] = [];
  for (let k = f; k < end; k += 1) if (tokens[k]!.kind === "identifier" && tokens[k]!.text === ctx.agg) occurrences.push(k);
  if (!occurrences.length) return { builder: null };
  const local = new Set<string>();
  let returnsAggregate = false;
  let globbed = false;
  for (const k of occurrences) {
    const useAt = chainStart(tokens, k) - 1;
    if (tokens[k + 1]?.text === "::" && tokens[k + 2]?.text === "*" && tokens[useAt]?.text === "use") {
      let semicolon = k + 3;
      while (tokens[semicolon] && tokens[semicolon]!.text !== ";") semicolon += 1;
      edits.push({ start: tokens[useAt]!.start, end: tokens[semicolon]?.end ?? tokens[k + 2]!.end, text: "" });
      globbed = true;
      continue;
    }
    const classified = classifyOccurrence(text, tokens, k);
    if (isFail(classified)) return classified;
    if (classified.kind === "constructor") {
      const chain = ctx.variantChain.get(classified.variant);
      if (!chain) return { error: `variant ${classified.variant} of the aggregate has no converted leaf` };
      local.add(boundOf(chain));
      edits.push({ start: classified.start, end: classified.end, text: "Op::from(" });
    } else {
      edits.push({ start: classified.start, end: classified.end, text: "Op" });
      if (k > close && k < body) returnsAggregate = tokens[close + 1]?.text === "->" && chainStart(tokens, k) === close + 2 && k + 1 === body;
    }
  }
  if (globbed) {
    for (let t = body; t < end; t += 1) {
      const chain = tokens[t]!.kind === "identifier" ? ctx.variantChain.get(tokens[t]!.text) : undefined;
      if (!chain || tokens[t + 1]?.text !== "(" || ["::", ".", "fn", "struct", "enum", "mod", "use", "type", "trait", "impl"].includes(tokens[t - 1]?.text ?? "")) continue;
      local.add(boundOf(chain));
      edits.push({ start: tokens[t]!.start, end: tokens[t]!.end, text: "Op::from" });
    }
  }
  collectBuilderBounds(tokens, body, end, ctx, local, name.text);
  for (const bound of local) bounds.add(bound);
  edits.push({ start: name.end, end: name.end, text: "<Op>" });
  const sorted = [...local].sort();
  if (sorted.length) {
    const bodyStart = tokens[body]!.start;
    const trimmed = text[bodyStart - 1] === " " ? bodyStart - 1 : bodyStart;
    edits.push({ start: trimmed, end: bodyStart, text: `\nwhere\n    Op: ${sorted.join(" + ")},\n` });
  }
  if (!returnsAggregate || tokens[f - 1]?.text !== "pub") return { builder: null };
  const params: string[] = [];
  for (const [a, b] of rustTokenSegments(tokens, pairs, f + 3, close, ",")) {
    let k = a;
    if (tokens[k]?.text === "mut") k += 1;
    if (tokens[k]?.kind !== "identifier" || tokens[k]!.text === "self" || tokens[k + 1]?.text !== ":") return { error: `builder ${name.text} has a pattern parameter` };
    params.push(tokens[k]!.text);
    void b;
  }
  const paramsText = text.slice(tokens[f + 2]!.end, tokens[close]!.start).trim();
  return { builder: { name: name.text, params: paramsText, args: params.join(", "), chain: "" } };
}

/** 🧷️ The `Op: A + B` bounds of the top-level functions a companion already wrote generic over its aggregate (`fn x<Op>(..) where Op: A + B`); the leaf impl must carry them. */
function genericBounds(text: string): Set<string> {
  const bounds = new Set<string>();
  if (!/\bOp\b/u.test(text)) return bounds;
  const tokens = rustTokens(text);
  const pairs = rustTokenPairs(tokens);
  for (let k = 0; k + 6 < tokens.length; k += 1) {
    if (tokens[k]!.text !== "fn" || tokens[k + 2]?.text !== "<" || tokens[k + 3]?.text !== "Op" || tokens[k + 4]?.text !== ">" || tokens[k + 5]?.text !== "(") continue;
    let clause = (pairs.get(k + 5) ?? k + 5) + 1;
    while (clause < tokens.length && tokens[clause]!.text !== "where" && tokens[clause]!.text !== "{" && tokens[clause]!.text !== ";") clause += 1;
    if (tokens[clause]?.text !== "where" || tokens[clause + 1]?.text !== "Op" || tokens[clause + 2]?.text !== ":") continue;
    let bodyAt = clause + 3;
    while (bodyAt < tokens.length && tokens[bodyAt]!.text !== "{") bodyAt += 1;
    for (const [a, b] of rustTokenSegments(tokens, pairs, clause + 3, bodyAt, "+")) bounds.add(text.slice(tokens[a]!.start, tokens[b - 1]!.end).replace(/,\s*$/u, "").trim());
  }
  return bounds;
}

function rewriteSource(text: string, ctx: LeafContext, role: "component" | "companion", extraBounds: ReadonlySet<string>): Rewritten | Fail {
  if (!text.includes(ctx.agg)) return { text, bounds: role === "companion" ? genericBounds(text) : new Set(), builders: [] };
  const tree = parseRustSourceTree(text);
  const tokens = rustTokens(text);
  const pairs = rustTokenPairs(tokens);
  const edits: Edit[] = [];
  const bounds = new Set<string>();
  const builders: LeafBuilder[] = [];
  const claimed = new Set<RustSourceItem>();
  for (let k = 0; k < tokens.length; k += 1) {
    if (tokens[k]!.kind !== "identifier" || tokens[k]!.text !== ctx.agg) continue;
    const item = topLevelItemAt(tree.items, tokens[k]!.start);
    if (!item) return { error: `the aggregate is mentioned outside a top-level item at line ${lineOf(text, tokens[k]!.start)} (inline test module?)` };
    claimed.add(item);
  }
  for (const item of claimed) {
    if (item.kind === "use") {
      const statement = text.slice(item.declStart, item.end);
      if (new RegExp(`\\b${ctx.agg}\\s+as\\b`, "u").test(statement)) return { error: "the aggregate is imported under an alias" };
      const throughAggregate = /^(?:pub(?:\([^)]*\))?\s+)?use\s+crate::(?:schema::)?mutations::/u.test(statement);
      const moved = throughAggregate ? useLeafNames(statement).filter((leaf) => leaf !== ctx.agg && ctx.builderChain.has(leaf)) : [];
      const pruned = pruneUseLeaves(statement, new Set([ctx.agg, ...moved]));
      const own = moved.map((name) => {
        const module = ctx.builderChain.get(name)!.split("::").slice(0, -1).join("::");
        const reachesModule = tokens.some((token, index) => token.text === name && tokens[index + 1]?.text === "::" && tokens[index - 1]?.text !== "::");
        return reachesModule ? `use ${module}::{self, ${name}};` : `use ${module}::${name};`;
      });
      const appended = own.length ? `${pruned === null ? "" : "\n"}${own.join("\n")}` : "";
      if (pruned === null) {
        const after = text[item.end] === "\n" ? item.end + 1 : item.end;
        edits.push({ start: item.start, end: after, text: appended ? `${appended}\n` : "" });
      } else edits.push({ start: item.declStart, end: item.end, text: `${pruned}${appended}` });
      continue;
    }
    if (item.kind === "fn") {
      const result = rewriteFunction(text, tokens, pairs, item, ctx, edits, bounds);
      if (isFail(result)) return result;
      if (result.builder) builders.push({ ...result.builder, chain: ctx.leafChain });
      continue;
    }
    if (item.kind === "impl" && role === "component") {
      const result = rewriteImpl(text, tokens, pairs, item, ctx, edits, extraBounds);
      if (isFail(result)) return result;
      continue;
    }
    return { error: `the aggregate is used by a ${item.kind} item at line ${lineOf(text, item.declStart)}` };
  }
  let rewritten = applyEdits(text, edits);
  const prefix = role === "companion" ? "super::super::" : "super::";
  for (const chain of ctx.variantChain.values()) {
    const [leaf, payload] = chain.split("::").slice(-2);
    if (leaf && payload) rewritten = rewritten.replaceAll(`${prefix}${leaf}::${payload}`, chain);
  }
  return { text: rewritten, bounds, builders };
}

function kernelPathOf(traitPath: string, text: string): string | null {
  const qualified = /^(.*)::MutationKind$/u.exec(traitPath);
  if (qualified) return qualified[1]!;
  const tree = parseRustSourceTree(text);
  for (const item of tree.items) {
    if (item.kind !== "use") continue;
    const statement = text.slice(item.declStart, item.end);
    if (!useLeafNames(statement).includes("MutationKind")) continue;
    const match = /use\s+((?:\w+::)*\w+)::(?:\{[^}]*\bMutationKind\b[^}]*\}|MutationKind\b)/u.exec(statement);
    if (match) return match[1]!;
  }
  return null;
}

function rewriteImpl(text: string, tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, item: RustSourceItem, ctx: LeafContext, edits: Edit[], extraBounds: ReadonlySet<string>): { ok: true } | Fail {
  let impl = tokens.findIndex((token) => token.start >= item.declStart);
  while (impl < tokens.length && tokens[impl]!.text !== "impl") impl += 1;
  const last = tokens.findIndex((token) => token.start >= item.end);
  const end = last < 0 ? tokens.length : last;
  if (tokens[impl + 1]?.text === "<") return { error: "the MutationKind impl already has generics" };
  let open = impl;
  while (open < end && tokens[open]!.text !== "{") open += 1;
  let forIndex = impl;
  while (forIndex < open && tokens[forIndex]!.text !== "for") forIndex += 1;
  if (forIndex >= open) return { error: "the aggregate is used by an inherent impl" };
  if (tokens.slice(impl, open).some((token) => token.text === "where")) return { error: "the MutationKind impl already has a where clause" };
  let lt = impl + 1;
  while (lt < forIndex && tokens[lt]!.text !== "<") lt += 1;
  if (lt >= forIndex || tokens[lt - 1]?.text !== "MutationKind") return { error: "the aggregate is used by an impl of another trait" };
  const gt = angleClose(tokens, lt);
  if (gt < 0) return { error: "unreadable MutationKind impl header" };
  const args = rustTokenSegments(tokens, pairs, lt + 1, gt, ",");
  if (args.length !== 2) return { error: "unexpected MutationKind arguments" };
  const [[sa, sb], [aa, ab]] = args as [readonly [number, number], readonly [number, number]];
  const aggregateTokens = tokens.slice(aa, ab).map((token) => token.text).join("");
  if (!new RegExp(`^(?:[A-Za-z0-9_]+::)*${ctx.agg}$`, "u").test(aggregateTokens)) return { error: "the second MutationKind argument is not the aggregate" };
  const snapshotText = text.slice(tokens[sa]!.start, tokens[sb - 1]!.end);
  const traitStart = tokens[impl + 1]!.start;
  const traitPath = text.slice(traitStart, tokens[lt]!.start).trim();
  const payloadText = text.slice(tokens[forIndex + 1]!.start, tokens[open - 1]!.end).trim();
  const kernel = kernelPathOf(traitPath, text);
  if (!kernel) return { error: "the kernel path of MutationKind could not be resolved" };
  const bodyText = text.slice(tokens[open]!.start, item.end);
  const diffMatch = /fn\s+diff\b[^{]*?->\s*(?:[A-Za-z0-9_]+::)*MutationOutcome\s*<\s*([^{}]+?)\s*>\s*\{/u.exec(bodyText);
  const written = diffMatch?.[1] ?? null;
  const diffType = written !== null && !new RegExp(`\\b${ctx.agg}\\b`, "u").test(written) ? written : ctx.diff;
  if (!diffType) return { error: "the diff type of the MutationKind impl could not be resolved" };
  const diffViaAggregate = written !== null && new RegExp(`\\b${ctx.agg}\\b`, "u").test(written);
  if (diffViaAggregate && !ctx.diffResolved) return { error: `the diff type ${diffType} cannot be named by an absolute path from the aggregate` };
  const bodyEdits: Edit[] = [];
  const local = new Set<string>();
  for (let k = open; k < end; k += 1) {
    if (tokens[k]!.kind !== "identifier" || tokens[k]!.text !== ctx.agg) continue;
    const classified = classifyOccurrence(text, tokens, k);
    if (isFail(classified)) return classified;
    if (classified.kind === "constructor") {
      const chain = ctx.variantChain.get(classified.variant);
      if (!chain) return { error: `variant ${classified.variant} of the aggregate has no converted leaf` };
      local.add(boundOf(chain));
      bodyEdits.push({ start: classified.start, end: classified.end, text: "Op::from(" });
    } else bodyEdits.push({ start: classified.start, end: classified.end, text: "Op" });
  }
  collectBuilderBounds(tokens, open, end, ctx, local, null);
  const own = boundOf(`${ctx.leafChain}::${ctx.payload}`);
  const all = new Set<string>([boundOf(payloadText), ...[...local, ...extraBounds].filter((bound) => bound !== own)]);
  const header = `impl<Op> ${traitPath}<${snapshotText}, Op> for ${payloadText}\nwhere\n    Op: ${kernel}::Mutation<${snapshotText}, Diff = ${diffViaAggregate ? ctx.diffResolved : diffType}> + ${[...all].sort().join(" + ")},\n`;
  edits.push({ start: tokens[impl]!.start, end: tokens[open]!.start, text: header });
  edits.push(...bodyEdits);
  return { ok: true };
}


const PRIMITIVES = new Set(["bool", "char", "str", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "String", "Vec", "Option", "Box", "Result", "Self", "HashMap", "BTreeMap", "mut", "dyn", "impl", "crate", "std", "core", "alloc", "self", "super"]);

function aggregateNames(text: string): Set<string> {
  const tree = parseRustSourceTree(text);
  const names = new Set<string>();
  for (const item of tree.items) {
    if (item.kind === "use") for (const leaf of useLeafNames(text.slice(item.declStart, item.end))) names.add(leaf);
    else if (item.name) names.add(item.name);
  }
  return names;
}

function paramsResolveIn(params: string, known: ReadonlySet<string>): boolean {
  const tokens = rustTokens(params);
  const pairs = rustTokenPairs(tokens);
  for (const [a, b] of rustTokenSegments(tokens, pairs, 0, tokens.length, ",")) {
    const colon = tokens.findIndex((token, index) => index >= a && index < b && token.text === ":");
    if (colon < 0) return false;
    for (let k = colon + 1; k < b; k += 1) {
      const token = tokens[k]!;
      if (token.kind !== "identifier" || tokens[k - 1]?.text === "::" || tokens[k - 1]?.text === "'") continue;
      if (!PRIMITIVES.has(token.text) && !known.has(token.text)) return false;
    }
  }
  return true;
}

function resolveDiff(diff: string | null, fullPaths: ReadonlyMap<string, string>, aggregateEff: readonly string[]): string | null {
  if (!diff) return null;
  if (/^(?:crate|::)/u.test(diff)) return diff;
  if (/^[A-Za-z_][A-Za-z0-9_]*$/u.test(diff)) return fullPaths.get(diff) ?? `crate::${[...aggregateEff].join("::")}::${diff}`;
  return null;
}

function payloadDefinition(decls: readonly ModuleDecl[], readText: (absolute: string) => string | null, name: string, hint: readonly string[] | null = null): { decl: ModuleDecl; file: string }[] {
  const found: { decl: ModuleDecl; file: string }[] = [];
  for (const decl of decls) {
    if (decl.testOnly || decl.file === null) continue;
    const text = readText(decl.file);
    if (text === null) continue;
    const match = new RegExp(`\\bpub\\s+(?:struct|enum)\\s+${name}\\b`, "u").exec(text);
    if (!match) continue;
    const window = text.slice(Math.max(0, match.index - 1800), match.index);
    const attribute = window.lastIndexOf("MutationLeaf");
    if (attribute < 0 || /\n\}\n/u.test(window.slice(attribute))) continue;
    found.push({ decl, file: decl.file });
  }
  if (found.length > 1 && hint) {
    const exact = found.filter((entry) => entry.decl.eff.join("::") === hint.join("::"));
    if (exact.length === 1) return exact;
  }
  return found;
}

/** 🔭️ Reads the names an aggregate module file puts in scope for the leaves mounted below it. */
export function aggregateScope(text: string, eff: readonly string[], isComponent: boolean): AggregateScope {
  const tree = parseRustSourceTree(text);
  const names = new Map<string, AggregateName>();
  const children = new Set(tree.modules.map((node) => node.name));
  const above = isComponent ? eff : eff.slice(0, -1);
  let glob = false;
  for (const item of tree.items) {
    if (item.attributeTexts.some((attribute) => /cfg\(\s*test\s*\)/u.test(attribute))) continue;
    if (item.kind === "use") {
      const statement = text.slice(item.declStart, item.end);
      if (/\*\s*;\s*$/u.test(statement)) { glob = true; continue; }
      for (const entry of flattenUse(statement)) {
        const head = entry.path[0];
        let absolute: string[] | null = null;
        if (head === "crate") absolute = entry.path;
        else if (head === "self") absolute = ["crate", ...eff, ...entry.path.slice(1)];
        else if (head === "super") absolute = entry.path[1] === "super" ? null : ["crate", ...above, ...entry.path.slice(1)];
        else if (head !== undefined && children.has(head)) absolute = ["crate", ...eff, ...entry.path];
        else absolute = entry.path;
        if (absolute === null) { names.set(entry.leaf, { kind: "item" }); continue; }
        names.set(entry.leaf, { kind: "import", statement: `use ${absolute.join("::")}${entry.leaf === absolute.at(-1) ? "" : ` as ${entry.leaf}`};` });
      }
    } else if (item.name && item.kind !== "impl" && item.kind !== "extern-crate" && item.kind !== "macro") names.set(item.name, { kind: "item" });
  }
  for (const node of tree.modules) {
    if (node.conditionalTest || node.name === "component") continue;
    names.set(node.name, node.visibility === "pub" ? { kind: "module", statement: `use ${["crate", ...eff, node.name].join("::")};` } : { kind: "item" });
  }
  return { names, glob };
}

/** 🔗️ Replaces the `use super::..::*;` glob of a leaf file that reaches the aggregate by explicit absolute imports of the names the file uses; answers the names that cannot be imported (the leaf then stays coupled to the aggregate). */
export function decoupleFromAggregate(text: string, depth: number, scope: AggregateScope): { readonly text: string; readonly coupled: readonly string[] } | null {
  const tree = parseRustSourceTree(text);
  const pattern = new RegExp(`^use\\s+(?:super\\s*::\\s*){${depth}}\\*\\s*;`, "u");
  const glob = tree.items.find((item) => item.kind === "use" && item.visibility === "private" && pattern.test(text.slice(item.declStart, item.end)));
  if (!glob) return null;
  if (scope.glob) return { text, coupled: ["the aggregate imports through a glob"] };
  const own = new Set<string>([...declaredNames(text), ...tree.modules.map((node) => node.name)]);
  const statements: string[] = [];
  const coupled: string[] = [];
  const used = identifierSet(`${text.slice(0, glob.declStart)}${text.slice(glob.end)}`);
  for (const name of [...used].sort()) {
    const entry = scope.names.get(name);
    if (!entry || own.has(name)) continue;
    if (entry.kind === "item") coupled.push(name);
    else statements.push(`#[allow(unused_imports)]\n${entry.statement}`);
  }
  if (coupled.length) return { text, coupled };
  const after = text[glob.end] === "\n" ? glob.end + 1 : glob.end;
  const replacement = statements.length ? `${statements.join("\n")}\n` : "";
  return { text: `${text.slice(0, glob.declStart)}${replacement}${text.slice(after)}`, coupled: [] };
}

/** 🏘️ Converts every concrete mutation leaf to a leaf generic over its aggregate, so leaves and aggregate no longer form one dependency cycle and the planner can place the leaves in family crates. */
export function convertMutationLeaves(rootFile: string, rootSource: string, readText: (absolute: string) => string | null): MutationFamilyResult {
  const decls = collectDecls(rootFile, rootSource, readText);
  const list = [...decls.decls.values()];
  const aggregates = findMutationAggregates(list, readText);
  const edits = new Map<string, string>();
  const creates = new Map<string, string>();
  const notes: string[] = [];
  const converted: string[] = [];
  const hand: { payload: string; reason: string }[] = [];
  const read = (file: string): string => edits.get(file) ?? readText(file) ?? "";
  for (const aggregate of aggregates) {
    const located = new Map<string, { decl: ModuleDecl; file: string }>();
    const home = list.find((decl) => decl.file === aggregate.file && !decl.testOnly) ?? null;
    if (home && !home.isComponent && home.node.visibility === "private") {
      notes.push(`mutation aggregate ${aggregate.name} stays concrete: its module is private, so no absolute path can name its leaves`);
      continue;
    }
    if (home && superReach(read(aggregate.file)) > 0) {
      const absolute = absolutizeSuperPaths(read(aggregate.file), home.isComponent ? [...aggregate.effPath, "component"] : aggregate.effPath);
      if (absolute !== null) edits.set(aggregate.file, absolute);
    }
    const aggregateText0 = read(aggregate.file);
    const reexports = new Map<string, string[]>();
    const fullPaths = new Map<string, string>();
    for (const item of parseRustSourceTree(aggregateText0).items) {
      if (item.kind !== "use") continue;
      for (const entry of flattenUse(aggregateText0.slice(item.declStart, item.end))) {
        const resolved = entry.path[0] === "crate" ? entry.path.slice(1) : entry.path[0] === "super" ? [...aggregate.effPath.slice(0, -1), ...entry.path.slice(1)] : entry.path[0] === "self" ? [...aggregate.effPath, ...entry.path.slice(1)] : [...aggregate.effPath, ...entry.path];
        reexports.set(entry.leaf, resolved.slice(0, -1));
        fullPaths.set(entry.leaf, ["crate", ...resolved].join("::"));
      }
    }
    for (const variant of aggregate.variants) {
      const written = variant.path.split("::");
      const hintFromPath = written[0] === "crate" ? written.slice(1, -1) : written[0] === "super" ? [...aggregate.effPath.slice(0, -1), ...written.slice(1, -1)] : written[0] === "self" ? [...aggregate.effPath, ...written.slice(1, -1)] : [...aggregate.effPath, ...written.slice(0, -1)];
      const found = payloadDefinition(list, readText, variant.payload, reexports.get(variant.payload) ?? hintFromPath);
      if (found.length === 1) located.set(variant.variant, found[0]!);
      else hand.push({ payload: variant.payload, reason: found.length ? `payload defined in ${found.length} files` : "payload struct with MutationLeaf not found" });
    }
    const variantChain = new Map<string, string>();
    for (const [variant, place] of located) variantChain.set(variant, `crate::${plainPath(place.decl.eff)}::${aggregate.variants.find((entry) => entry.variant === variant)!.payload}`);
    const builderChain = new Map<string, string>();
    const prepared: { variant: MutationVariant; place: { decl: ModuleDecl; file: string }; companions: ModuleDecl[] }[] = [];
    const holderOf = (place: { decl: ModuleDecl; file: string }): ModuleDecl => {
      const parent = place.decl.parentId ? decls.decls.get(place.decl.parentId) ?? null : null;
      if (!parent) return place.decl;
      if (place.decl.isComponent) return parent;
      const alone = parent.node.inline && !place.decl.children.length && [...located.values()].every((other) => other.decl === place.decl || other.decl.parentId !== parent.id);
      return alone ? parent : place.decl;
    };
    for (const variant of aggregate.variants) {
      const place = located.get(variant.variant);
      if (!place) continue;
      const holder = holderOf(place);
      const companions = holder.children.map((id) => decls.decls.get(id)!).filter((child) => child.id !== place.decl.id && !child.testOnly && child.file !== null);
      prepared.push({ variant, place, companions });
      const source = readText(place.file) ?? "";
      for (const match of source.matchAll(/\bpub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*(?:[A-Za-z0-9_]+::)*(\w+)\s*\{/gu)) if (match[2] === aggregate.name) builderChain.set(match[1]!, variantChain.get(variant.variant)!);
    }
    const builders = new Map<string, LeafBuilder>();
    const aggregateDecl = list.find((decl) => decl.file === aggregate.file && !decl.testOnly) ?? null;
    const scope0 = aggregateDecl ? aggregateScope(aggregateText0, aggregate.effPath, aggregateDecl.isComponent) : null;
    if (aggregateDecl && scope0 && prepared.length) {
      const holder = aggregateDecl.isComponent && aggregateDecl.parentId ? decls.decls.get(aggregateDecl.parentId)! : aggregateDecl;
      const leafTexts = list.filter((decl) => decl !== aggregateDecl && decl.file !== null && !decl.testOnly && decl.raw.length > holder.raw.length && holder.raw.every((segment, index) => decl.raw[index] === segment)).map((decl) => read(decl.file!));
      const hoisted = hoistAggregateSupport({ aggregateFile: aggregate.file, aggregateText: aggregateText0, aggregateName: aggregate.name, leafTexts, scope: scope0 });
      if (hoisted) {
        edits.set(aggregate.file, hoisted.aggregateText);
        creates.set(hoisted.supportFile, hoisted.supportText);
        notes.push(`mutation aggregate ${aggregate.name}: ${hoisted.moved.length} helper(s) hosted in a support module (${hoisted.moved.join(", ")})`);
      }
    }
    const aggregateText1 = read(aggregate.file);
    const scope = aggregateDecl ? aggregateScope(aggregateText1, aggregate.effPath, aggregateDecl.isComponent) : null;
    let count = 0;
    for (const entry of prepared) {
      const payload = entry.variant.payload;
      const ctx: LeafContext = { agg: aggregate.name, variantChain, builderChain, leafChain: `crate::${plainPath(entry.place.decl.eff)}`, payload, snapshot: aggregate.snapshot, diff: aggregate.diff, diffResolved: resolveDiff(aggregate.diff, fullPaths, aggregate.effPath) };
      const staged = new Map<string, string>();
      let failure: string | null = null;
      const union = new Set<string>();
      for (const companion of entry.companions) {
        const result = rewriteSource(read(companion.file!), ctx, "companion", new Set());
        if (isFail(result)) { failure = `${companion.raw.at(-1)}: ${result.error}`; break; }
        staged.set(companion.file!, result.text);
        for (const bound of result.bounds) union.add(bound);
      }
      if (failure === null) {
        const own = rewriteSource(read(entry.place.file), ctx, "component", union);
        if (isFail(own)) failure = own.error;
        else {
          staged.set(entry.place.file, own.text);
          for (const builder of own.builders) builders.set(builder.name, builder);
        }
      }
      if (failure === null) {
        const holder = holderOf(entry.place);
        for (const childId of holder.children) {
          const test = decls.decls.get(childId)!;
          if (!test.testOnly || test.node.inline || test.file === null || test.children.length) continue;
          const body = read(test.file);
          if (!new RegExp(`\\b${aggregate.name}\\b`, "u").test(body) || superReach(body) === 0) continue;
          const absolute = absolutizeSuperPaths(body, test.eff);
          if (absolute === null) { failure = `test module ${test.raw.at(-1)} uses super:: inside nested modules`; break; }
          staged.set(test.file, absolute);
        }
      }
      if (failure !== null) { hand.push({ payload, reason: failure }); continue; }
      if (scope && aggregateDecl && !entry.place.decl.isComponent) {
        const files = new Map<string, ModuleDecl>([[entry.place.file, entry.place.decl], ...entry.companions.map((companion): [string, ModuleDecl] => [companion.file!, companion])]);
        for (const [file, decl] of files) {
          const decoupled = decoupleFromAggregate(staged.get(file) ?? read(file), decl.raw.length - aggregateDecl.raw.length, scope);
          if (!decoupled) continue;
          if (decoupled.coupled.length) notes.push(`mutation leaf ${payload} stays coupled to its aggregate in ${decl.raw.at(-1)}: uses ${decoupled.coupled.join(", ")}`);
          else staged.set(file, decoupled.text);
        }
        for (const [file, decl] of files) {
          const text = staged.get(file) ?? read(file);
          const keep = decl.raw.length - entry.place.decl.raw.length;
          if (superReach(text) <= keep) continue;
          const absolute = absolutizeBeyond(text, decl.raw, keep, false);
          if (absolute !== null) staged.set(file, absolute);
        }
      }
      for (const [file, text] of staged) if (text !== read(file)) edits.set(file, text);
      converted.push(payload);
      count += 1;
    }
    if (!count) continue;
    const aggregateText = read(aggregate.file);
    const tree = parseRustSourceTree(aggregateText);
    const useEdits: Edit[] = [];
    const reexported = new Set<string>();
    const known = aggregateNames(aggregateText);
    const wrappable = new Set([...builders.values()].filter((builder) => paramsResolveIn(builder.params, known)).map((builder) => builder.name));
    for (const item of tree.items) {
      if (item.kind !== "use") continue;
      const statement = aggregateText.slice(item.declStart, item.end);
      if (!/^pub\b/u.test(statement)) continue;
      const leaves = useLeafNames(statement);
      const dropped = leaves.filter((leaf) => wrappable.has(leaf));
      if (!dropped.length) continue;
      for (const leaf of dropped) reexported.add(leaf);
      const pruned = pruneUseLeaves(statement, new Set(dropped));
      if (pruned === null) useEdits.push({ start: item.start, end: aggregateText[item.end] === "\n" ? item.end + 1 : item.end, text: "" });
      else useEdits.push({ start: item.declStart, end: item.end, text: pruned });
    }
    if (reexported.size) {
      const wrappers = [...reexported].sort().map((name) => {
        const builder = builders.get(name)!;
        return `/// 🏗️ Builder of the \`${name}\` leaf for this aggregate; the leaf builder stays generic over its aggregate.\npub fn ${name}(${builder.params}) -> ${aggregate.name} {\n    ${builder.chain}::${name}::<${aggregate.name}>(${builder.args})\n}`;
      });
      edits.set(aggregate.file, applyEdits(aggregateText, useEdits).trimEnd() + `\n\n//#region 🔖️ConcreteBuilders\n${wrappers.join("\n\n")}\n//#endregion 🔖️ConcreteBuilders\n`);
    } else if (useEdits.length) edits.set(aggregate.file, applyEdits(aggregateText, useEdits));
    notes.push(`mutation aggregate ${aggregate.name}: ${count}/${aggregate.variants.length} leaves converted to generic leaves, ${reexported.size} concrete builder(s) kept on the aggregate`);
  }
  for (const entry of hand) notes.push(`mutation leaf ${entry.payload} stays concrete: ${entry.reason}`);
  return { edits, creates, notes, aggregates, converted, hand };
}
