import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { POLICY_MUTATION_PLAN_DIR, POLICY_MUTATIONS_FACET } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyMutationLawInventory, policyMutationLawRustFiles } from "../🟦️.ts";
import { policyReadFileSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";

type PolicyDiffOnlyRuleId = "R8" | "R9" | "R10" | "R11" | "R12" | "R13" | "R14" | "R15" | "R16";

type PolicyDiffOnlyRule = Readonly<{ code: string; slug: string; reason: string; solution: string }>;

/** 📜️The diff-only mutation law's rules (`26/10/08/DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES` design L1–L5, violation codes V1–V4): one reason and one remedy per rule, keyed by rule id. */
const POLICY_DIFF_ONLY_RULES: Readonly<Record<PolicyDiffOnlyRuleId, PolicyDiffOnlyRule>> = {
  R8: { code: "V3-LEAF-APPLY", slug: "leaf-mutable-base", reason: "a scanned diff/inverse/helper fn takes or creates a `&mut`", solution: "Build the sparse diff or concrete inverse declaratively from the payload and reads of `base`; only the central applier writes into a snapshot." },
  R9: { code: "V3-LEAF-APPLY", slug: "leaf-applies-diff", reason: "a diff is applied outside the central applier (`apply_diff`, `ApplyCapability`, `.apply(`)", solution: "Remove the apply: leaves never turn a diff into a snapshot; callers use `protocol::apply_diff`, which alone mints the `ApplyCapability`." },
  R10: { code: "V1-SNAPSHOT-DIFF", slug: "leaf-between", reason: "a diff is built by differencing or simulating snapshots (`between`, `*_replacing`, `state_after`, `value_diff*`, …)", solution: "Name the changed entities and fields with their new values instead; `between` is for sync and import only." },
  R11: { code: "V2-DIFF-DERIVED-INVERSE", slug: "diff-derived-inverse", reason: "an inverse is derived from the forward diff or a simulation of it (`diff(`, `negative`, `state_after`, …)", solution: "Build the inverse mutations from the payload and reads of `base` inside the leaf's own inverse, as absolute setters of the base values." },
  R12: { code: "V1-SNAPSHOT-DIFF", slug: "base-clone-diff", reason: "a fn copies a reference parameter (`clone`/`to_vec`/`to_owned`/`iter().cloned().collect()`) into a mutable binding", solution: "Read `base` and emit the sparse diff rows directly; never edit a copy of the snapshot and difference it." },
  R13: { code: "V1-SNAPSHOT-DIFF", slug: "whole-state-diff", reason: "the diff type is the whole snapshot it applies to", solution: "Replace it with a sparse typed diff (keyed added/removed/modified rows) with a concrete `DiffAlgebra::inverse`." },
  R14: { code: "V2-RESTORE-INVERSE", slug: "restore-inverse", reason: "an inverse returns a whole-snapshot or whole-record restore (`Restore…`, `Replace…`, `*Snapshot` variants, `*_state`/`*_content` fields)", solution: "Return the concrete mutations that put each touched value back to its exact base value." },
  R15: { code: "V4-LAW-UNTESTED", slug: "inverse-sum-law-untested", reason: "no test under the leaf's own `🧪️tests` calls `assert_mutation_inverse_sum_law`", solution: "Add a test under the leaf's `🧪️tests` that calls `assert_mutation_inverse_sum_law(&mutation, &before)` on each applied fixture." },
  R16: { code: "V3-LEAF-APPLY", slug: "outcome-apply-to", reason: "`MutationOutcome::apply_to` is gone: a diff is applied only by the central applier", solution: "Call `protocol::apply_diff(outcome.diff(), &base)` instead." },
};

/** 🧰️`MutationDiff` methods that are the central applier's own machinery, never diff or inverse logic: the only fns of a diff type exempt from the diff-only rules. */
const POLICY_DIFF_ONLY_DIFF_TYPE_MACHINERY: ReadonlySet<string> = new Set(["apply", "absorb", "retire_cold", "retire_projection", "is_empty"]);

/** 🔎️Raw cue that a file may hold a hand-written `impl … MutationKind<` or `impl … Mutation<` block. */
const POLICY_DIFF_ONLY_IMPL_PROBE_RE = /\bimpl\b[^{;]*\bMutation(?:Kind)?\s*</;

/** 🔎️Raw cue that a file may hold an `impl … MutationDiff<` block. */
const POLICY_DIFF_ONLY_DIFF_IMPL_PROBE_RE = /\bimpl\b[^{;]*\bMutationDiff\s*</;

const POLICY_RUST_RAW_STRING_RE = /b?r(#*)"/y;

const POLICY_RUST_TEST_ITEM_RE = /#\[\s*(?:cfg\s*\(\s*test\s*\)|test)\s*\]/g;

const POLICY_RUST_FN_RE = /\bfn\s+([A-Za-z_]\w*)/g;

const POLICY_RUST_IMPL_RE = /\bimpl\b/g;

const POLICY_RUST_USE_RE = /\buse\s[^;]*;/g;

const POLICY_DIFF_ONLY_APPLY_RE = /(?:\.|::)apply\s*\(|(?<!\bfn\s+)\bapply_diff\s*\(|\bApplyCapability\b/g;

/** 🔎️Snapshot-differencing and simulation helpers under any spelling: `*between*`, `*_replacing`, `*_state_diff`, `state_after*`, `value_diff*`. */
const POLICY_DIFF_ONLY_BETWEEN_RE = /(?<!\bfn\s+)(?<!\w)(?:\w*between\w*|\w*_replacing|\w*_state_diff|state_after\w*|value_diff\w*)\s*(?:::\s*<[^>(]*>)?\s*\(/g;

/** 🔎️Fns that build diff-like or whole-state values from the forward payload: negative simulators and derived-inverse helpers. */
const POLICY_DIFF_ONLY_SIMULATOR_RE = /(?<!\bfn\s+)(?<!\w)(?:negative|state_after\w*|inverse_of|value_diff\w*)\s*\(|\.negat(?:e|ion)\w*\s*\(/g;

/** 🔎️Raw cue that a plugin file defines a fn named like a diff or inverse builder. */
const POLICY_DIFF_ONLY_DIFFISH_FN_PROBE_RE = /\bfn\s+\w*(?:diff|inverse|negative|negation|replacing|state_after)\w*/;

const POLICY_DIFF_ONLY_DIFFISH_NAME_RE = /(?:^|_)(?:diff|inverse|negative|negation|replacing)(?:_|$)|state_after/;

const POLICY_DIFF_ONLY_INVERSEISH_NAME_RE = /inverse|negative|negation/;

const POLICY_DIFF_ONLY_PAYLOAD_PARAM_RE = /^(?:payload|mutation|op|edit|self)$/;

const POLICY_DIFF_ONLY_MUT_REF_RE = /&\s*mut\b/g;

/** 🔎️A `&mut` parameter or value typed as a whole artifact state: the one `&mut` a non-builder helper must never take. */
const POLICY_DIFF_ONLY_MUT_STATE_RE = /&\s*mut\s+(?:\w+::)*\w*(?:Snapshot|Document|State)\b/g;

/** 🚪️Infrastructure directories (and any directory ending `-internals`): io codecs, retained prepare state machines, encoders and engine internals are not diff logic and may apply diffs. */
const POLICY_DIFF_ONLY_INFRASTRUCTURE_DIRS: ReadonlySet<string> = new Set(["🚪️io", "🎮️prepare", "📦️codec"]);

const POLICY_DIFF_ONLY_DIFF_CALL_RE = /(?<!\w)diff\s*\(/g;

const POLICY_DIFF_ONLY_INVERSE_HELPER_RE = /(?<![\w.])(\w*_inverse|\w*inverse_\w*)\s*\(/g;

const POLICY_DIFF_ONLY_DIFF_ARGUMENT_RE = /\b\w*[dD]iff\w*\b|\boutcome\b/;

const POLICY_DIFF_ONLY_RESTORE_RE = /\b(?:SetSnapshot|PatchSnapshot|ReplaceDocument|ReplaceSnapshot|ReplaceConfig|ReplaceState|SetDocument|Restore\w*)\b|\b(?:Self|[A-Z]\w*)::\w*Snapshot\b|(?<!\blet\s)(?<!\bmut\s)\b\w+_(?:state|content|snapshot|document)\s*:(?!:)/g;

const POLICY_DIFF_ONLY_APPLY_TO_RE = /(?:\.|::)apply_to\s*\(/g;

const POLICY_DIFF_ONLY_TYPE_DIFF_RE = /\btype\s+Diff\s*=\s*([^;]+);/;

/** 🩻️`content` with comments, string literals, raw strings and char literals blanked to spaces — same length, newlines kept — so every index and line number still addresses the original source. */
function policyMaskRustSource(content: string): string {
  const size = content.length;
  const pieces: string[] = [];
  let kept = 0;
  let index = 0;
  const blank = (from: number, to: number) => {
    pieces.push(content.slice(kept, from), content.slice(from, to).replace(/[^\n]/g, " "));
    kept = to;
    index = to;
  };
  while (index < size) {
    const ch = content[index]!;
    const next = content[index + 1];
    if (ch === "/" && next === "/") {
      const eol = content.indexOf("\n", index);
      blank(index, eol === -1 ? size : eol);
    } else if (ch === "/" && next === "*") {
      let depth = 1;
      let cursor = index + 2;
      while (cursor < size && depth > 0) {
        const pair = content.slice(cursor, cursor + 2);
        if (pair === "/*") {
          depth++;
          cursor += 2;
        } else if (pair === "*/") {
          depth--;
          cursor += 2;
        } else cursor++;
      }
      blank(index, cursor);
    } else if (ch === '"') {
      let cursor = index + 1;
      while (cursor < size && content[cursor] !== '"') cursor += content[cursor] === "\\" ? 2 : 1;
      blank(index, Math.min(cursor + 1, size));
    } else if ((ch === "r" || ch === "b") && !/\w/.test(content[index - 1] ?? " ")) {
      POLICY_RUST_RAW_STRING_RE.lastIndex = index;
      const raw = POLICY_RUST_RAW_STRING_RE.exec(content);
      if (raw) {
        const terminator = `"${raw[1]}`;
        const end = content.indexOf(terminator, index + raw[0].length);
        blank(index, end === -1 ? size : end + terminator.length);
      } else index++;
    } else if (ch === "'") {
      if (next === "\\") {
        const end = content.indexOf("'", index + 3);
        blank(index, end === -1 ? index + 1 : end + 1);
      } else {
        const width = (content.codePointAt(index + 1) ?? 0) > 0xffff ? 2 : 1;
        if (content[index + 1 + width] === "'") blank(index, index + 2 + width);
        else index++;
      }
    } else index++;
  }
  return pieces.join("") + content.slice(kept);
}

/** 🪢️For every `{` of `masked` the index of its matching `}` (`-1` when unmatched); braces inside masked comments and literals never count. */
function policyRustBraceClose(masked: string): Int32Array {
  const close = new Int32Array(masked.length).fill(-1);
  const stack: number[] = [];
  for (let index = 0; index < masked.length; index++) {
    const ch = masked.charCodeAt(index);
    if (ch === 123) stack.push(index);
    else if (ch === 125) {
      const open = stack.pop();
      if (open !== undefined) close[open] = index;
    }
  }
  return close;
}

/** 🧽️`masked` with every `#[cfg(test)]`/`#[test]` item blanked: unit-test modules and test functions are not mutation leaves. */
function policyRustBlankTestItems(masked: string, close: Int32Array): string {
  const pieces: string[] = [];
  let kept = 0;
  for (const match of masked.matchAll(POLICY_RUST_TEST_ITEM_RE)) {
    if (match.index < kept) continue;
    let cursor = match.index + match[0].length;
    while (cursor < masked.length && masked[cursor] !== "{" && masked[cursor] !== ";") cursor++;
    const end = masked[cursor] === "{" && close[cursor]! >= 0 ? close[cursor]! + 1 : Math.min(cursor + 1, masked.length);
    pieces.push(masked.slice(kept, match.index), masked.slice(match.index, end).replace(/[^\n]/g, " "));
    kept = end;
  }
  return pieces.join("") + masked.slice(kept);
}

/** ➰️The index just past the `>` closing the `<` at `open` (a `->` arrow never closes). */
function policyRustSkipAngles(text: string, open: number): number {
  let depth = 0;
  for (let index = open; index < text.length; index++) {
    const ch = text[index];
    if (ch === "<") depth++;
    else if (ch === ">" && text[index - 1] !== "-" && --depth === 0) return index + 1;
  }
  return text.length;
}

/** 🔗️The index of the `)` matching the `(` at `open`. */
function policyRustParenEnd(text: string, open: number): number {
  let depth = 0;
  for (let index = open; index < text.length; index++) {
    if (text[index] === "(") depth++;
    else if (text[index] === ")" && --depth === 0) return index;
  }
  return text.length;
}

/** ✂️Splits `text` at its top-level commas (angle, paren, bracket and brace depth zero). */
function policyRustSplitTop(text: string): string[] {
  const parts: string[] = [];
  let depth = 0;
  let from = 0;
  for (let index = 0; index < text.length; index++) {
    const ch = text[index]!;
    if (ch === "<" || ch === "(" || ch === "[" || ch === "{") depth++;
    else if (ch === ")" || ch === "]" || ch === "}" || (ch === ">" && text[index - 1] !== "-")) depth--;
    else if (ch === "," && depth === 0) {
      parts.push(text.slice(from, index));
      from = index + 1;
    }
  }
  parts.push(text.slice(from));
  return parts.map((part) => part.trim()).filter(Boolean);
}

type PolicyRustFn = Readonly<{ name: string; start: number; bodyStart: number; end: number; baseParam: string; refParams: readonly string[] }>;

type PolicyRustImplKind = "mutation" | "diff" | "algebra" | "outcome" | "other";

type PolicyRustImpl = Readonly<{ kind: PolicyRustImplKind; args: readonly string[]; self: string; traitImpl: boolean; start: number; bodyStart: number; end: number }>;

/** 🔩️Every `fn` of `masked` that has a body, with the name of its last parameter (the `base` of a `diff`/`inverse`). */
function policyRustFns(masked: string, close: Int32Array): PolicyRustFn[] {
  const fns: PolicyRustFn[] = [];
  const skipSpace = (from: number) => {
    let cursor = from;
    while (cursor < masked.length && /\s/.test(masked[cursor]!)) cursor++;
    return cursor;
  };
  for (const match of masked.matchAll(POLICY_RUST_FN_RE)) {
    let cursor = skipSpace(match.index + match[0].length);
    if (masked[cursor] === "<") cursor = skipSpace(policyRustSkipAngles(masked, cursor));
    if (masked[cursor] !== "(") continue;
    const paramsEnd = policyRustParenEnd(masked, cursor);
    let open = paramsEnd + 1;
    let depth = 0;
    for (; open < masked.length; open++) {
      const ch = masked[open];
      if (ch === "(") depth++;
      else if (ch === ")") depth--;
      else if (depth === 0 && (ch === "{" || ch === ";")) break;
    }
    const end = masked[open] === "{" ? close[open]! : -1;
    if (end < 0) continue;
    const params = policyRustSplitTop(masked.slice(cursor + 1, paramsEnd));
    const last = params.at(-1) ?? "";
    const refParams = params.flatMap((param) => {
      const named = /^(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*(&(?!\s*mut\b))?/.exec(param);
      return named?.[2] && !/\w*(?:Diff|Delta|Patch|Mutation)\w*/.test(param.slice(param.indexOf(":"))) ? [named[1]!] : [];
    });
    fns.push({ name: match[1]!, start: match.index, bodyStart: open, end, baseParam: /^(?:mut\s+)?([A-Za-z_]\w*)\s*:/.exec(last)?.[1] ?? "base", refParams });
  }
  return fns;
}

/** 🧩️Every item-level `impl` block of `masked` with the trait it implements, that trait's generic arguments and the implementing type. */
function policyRustImpls(masked: string, close: Int32Array): PolicyRustImpl[] {
  const impls: PolicyRustImpl[] = [];
  for (const match of masked.matchAll(POLICY_RUST_IMPL_RE)) {
    let before = match.index - 1;
    while (before >= 0 && /\s/.test(masked[before]!)) before--;
    if (before >= 0 && !";}{]".includes(masked[before]!)) continue;
    let open = match.index;
    while (open < masked.length && masked[open] !== "{" && masked[open] !== ";") open++;
    const end = masked[open] === "{" ? close[open]! : -1;
    if (end < 0) continue;
    const header = masked.slice(match.index + 4, open).replace(/\s+/g, " ").trim();
    let cursor = 0;
    if (header[0] === "<") cursor = policyRustSkipAngles(header, 0);
    const rest = header.slice(cursor).replace(/^\s+/, "").replace(/^!/, "");
    const trait = /^([\w:]+)\s*</.exec(rest);
    let kind: PolicyRustImplKind = "other";
    let args: string[] = [];
    let self = "";
    if (trait) {
      const name = trait[1]!.split("::").pop()!;
      const angleOpen = trait[0].length - 1;
      const angleEnd = policyRustSkipAngles(rest, angleOpen);
      args = policyRustSplitTop(rest.slice(angleOpen + 1, angleEnd - 1));
      self = /^\s*for\s+(.+?)(?:\s+where\b.*)?$/.exec(rest.slice(angleEnd))?.[1] ?? "";
      kind = name === "MutationKind" || name === "Mutation" ? "mutation" : name === "MutationDiff" ? "diff" : name === "DiffAlgebra" ? "algebra" : name === "MutationOutcome" && self === "" ? "outcome" : "other";
    }
    impls.push({ kind, args, self, traitImpl: /(?:^|[\s>])for\s/.test(rest), start: match.index, bodyStart: open, end });
  }
  return impls;
}

/** 🏷️The last path segment of a Rust type with its generics dropped (`protocol::Foo<Bar>` → `Foo`). */
function policyRustTypeName(type: string): string {
  return type.replace(/<.*$/s, "").replace(/^[&\s]+/, "").trim().split("::").pop()!.trim();
}

/** 📍️Maps a character index of `content` to its 1-based line by binary search over the line starts. */
function policyRustLineLookup(content: string): (index: number) => number {
  const starts = [0];
  for (let at = content.indexOf("\n"); at !== -1; at = content.indexOf("\n", at + 1)) starts.push(at + 1);
  return (index) => {
    let low = 0;
    let high = starts.length - 1;
    while (low < high) {
      const mid = (low + high + 1) >> 1;
      if (starts[mid]! <= index) low = mid;
      else high = mid - 1;
    }
    return low + 1;
  };
}

/** 🧾️One breach record of a diff-only rule: `file:line rule:code reason (token)` is the summary the gate prints. */
function policyDiffOnlyBreach(rule: PolicyDiffOnlyRuleId, relPath: string, line: number, token: string): BreachRecord {
  const spec = POLICY_DIFF_ONLY_RULES[rule];
  return {
    id: `diff-only-${rule.toLowerCase()}-${relPath}-${line}`,
    summary: `${relPath}:${line} ${rule}:${spec.code} ${spec.reason} (\`${token}\`)`,
    kind: `diff-only-mutation/${spec.slug}`,
    scope: relPath,
    line,
    priority: "high",
    reason: `${spec.code}: ${spec.reason}.`,
    solution: spec.solution,
  };
}

/**
 * 🔬️Rules R8–R14 and R16 over ONE Rust file, pure. Comments, literals and `#[cfg(test)]` items never count.
 * Scope: non-test files under `🧬️mutations`, under `🧬️mutation-support` and under any `🔺️diff`/`↩️inverse` directory of a
 * `🧬️schema` are scanned in EVERY fn (except under the infrastructure directories `🚪️io`, `🎮️prepare`, `📦️codec`, which are treated like any other file); other non-test files holding an `impl … Mutation<`/`MutationKind<`/`MutationDiff<`/
 * `DiffAlgebra<` block are scanned in the fns inside those impls and in fns named like a diff or inverse builder
 * (`*diff*`, `*inverse*`, `negative`, `*_replacing`, `state_after`), so artifact-root helpers are covered too. R8 flags any `&mut`
 * in a fn named like a builder (`diff`, `*inverse*`, …) and, in every other scanned fn, only a `&mut` typed `…Snapshot`/`…Document`/`…State`. Never scanned:
 * fns of non-mutation trait impls (codecs, `Default`, …), fns named `*between*` (sync machinery) and the diff type's own
 * `apply`/`absorb`/`retire_*`/`is_empty` — plus every fn outside `🧬️mutations` that receives an `ApplyCapability` and, in a file that implements a
 * diff type outside `🧬️mutations`, the `*apply*` helpers its `apply` calls. A diff type's own `inverse` legitimately reads
 * its rows: R11 flags only simulators (`negative`, `state_after`, …) there, and `diff(`/`*_inverse(…diff…)` only on the
 * mutation side. Every `DiffAlgebra::inverse` body IS scanned. R9 additionally covers the whole
 * file under any `🧬️schema`/`🧬️mutations`/`🧬️mutation-support` directory. R13's `impl MutationDiff<X> for X` is judged in
 * every non-test Rust file. R13 and R14 apply to every lane and mutation type, including presence and transient roots:
 * diffs remain sparse and inverses restore touched values through concrete mutations. R16 scans every Rust file.
 * The planted-violation law (`🧪️tests/🧪️diff-only-law-gate`) drives exactly this function.
 */
export function policyDiffOnlyFileBreaches(relPath: string, content: string): BreachRecord[] {
  if (!relPath.endsWith(".rs")) return [];
  const directories = relPath.split("/").slice(0, -1);
  const underMutations = directories.includes(POLICY_MUTATIONS_FACET);
  const underSchema = directories.includes("🧬️schema");
  const underSupport = directories.includes("🧬️mutation-support");
  const testPath = directories.some((segment) => segment === "🧪️tests" || segment === "🧫️fixtures");
  const infrastructure = directories.some((segment) => POLICY_DIFF_ONLY_INFRASTRUCTURE_DIRS.has(segment) || segment.endsWith("-internals"));
  const everyFn = !testPath && !infrastructure && (underMutations || underSupport || (underSchema && directories.some((segment) => segment === "🔺️diff" || segment === "↩️inverse")));
  const leafCandidate = !testPath && (everyFn || underSchema || POLICY_DIFF_ONLY_IMPL_PROBE_RE.test(content) || POLICY_DIFF_ONLY_DIFF_IMPL_PROBE_RE.test(content) || (relPath.startsWith("✏️s/") && relPath.includes("/🗿️artifacts/") && POLICY_DIFF_ONLY_DIFFISH_FN_PROBE_RE.test(content)));
  const applyToCandidate = content.includes("apply_to");
  if (!leafCandidate && !applyToCandidate) return [];
  const all = policyMaskRustSource(content);
  const close = policyRustBraceClose(all);
  const lineOf = policyRustLineLookup(content);
  const breaches: BreachRecord[] = [];
  const seen = new Set<string>();
  const add = (rule: PolicyDiffOnlyRuleId, index: number, token: string) => {
    const line = lineOf(index);
    const key = `${rule}|${line}`;
    if (seen.has(key)) return;
    seen.add(key);
    breaches.push(policyDiffOnlyBreach(rule, relPath, line, token.replace(/\s+/g, " ").trim()));
  };
  if (applyToCandidate) {
    for (const match of all.matchAll(POLICY_DIFF_ONLY_APPLY_TO_RE)) add("R16", match.index, match[0]);
    if (all.includes("MutationOutcome"))
      for (const outcome of policyRustImpls(all, close).filter((impl) => impl.kind === "outcome"))
        for (const match of all.slice(outcome.bodyStart, outcome.end).matchAll(/\bfn\s+apply_to\b/g)) add("R16", outcome.bodyStart + match.index, match[0]);
  }
  if (!leafCandidate) return breaches;
  const code = policyRustBlankTestItems(all, close);
  const impls = policyRustImpls(code, close);
  for (const impl of impls)
    if (impl.kind === "diff" && impl.args.length === 1 && policyRustTypeName(impl.args[0]!) === policyRustTypeName(impl.self)) add("R13", impl.start, `impl MutationDiff<${policyRustTypeName(impl.self)}> for ${policyRustTypeName(impl.self)}`);
  const mutationImpls = impls.filter((impl) => impl.kind === "mutation");
  const hasDiffImpl = impls.some((impl) => impl.kind === "diff");
  const within = (spans: readonly { bodyStart: number; end: number }[], index: number) => spans.some((span) => span.bodyStart <= index && index <= span.end);
  const innermost = (index: number) => impls.filter((impl) => impl.bodyStart <= index && index <= impl.end).sort((a, b) => b.bodyStart - a.bodyStart)[0];
  const classified = policyRustFns(code, close).map((fn) => {
    const impl = innermost(fn.start);
    const logicImpl = impl !== undefined && (impl.kind === "mutation" || impl.kind === "diff" || impl.kind === "algebra");
    const machinery = impl !== undefined && (impl.kind === "diff" || impl.kind === "algebra") && POLICY_DIFF_ONLY_DIFF_TYPE_MACHINERY.has(fn.name);
    const applier = (impl?.kind === "diff" && fn.name === "apply") || (hasDiffImpl && !underMutations && fn.name.includes("apply")) || (!underMutations && code.slice(fn.start, fn.bodyStart).includes("ApplyCapability"));
    const exempt = machinery || applier || fn.name.includes("between") || (impl !== undefined && impl.kind === "other" && impl.traitImpl);
    return { fn, applier, scanned: !exempt && (everyFn || logicImpl || (!infrastructure && POLICY_DIFF_ONLY_DIFFISH_NAME_RE.test(fn.name))) };
  });
  const scanned = classified.filter((row) => row.scanned).map((row) => row.fn);
  const appliers = classified.filter((row) => row.applier).map((row) => ({ bodyStart: row.fn.start, end: row.fn.end }));
  const inverseFns = scanned.filter((fn) => POLICY_DIFF_ONLY_INVERSEISH_NAME_RE.test(fn.name));
  const diffTypeSide = (fn: PolicyRustFn) => {
    const impl = innermost(fn.start);
    return (impl !== undefined && (impl.kind === "diff" || impl.kind === "algebra")) || (underSchema && !underMutations);
  };
  const scannedSpans = scanned.map((fn) => ({ bodyStart: fn.start, end: fn.end }));
  for (const fn of scanned) {
    const builder = fn.name === "diff" || POLICY_DIFF_ONLY_DIFFISH_NAME_RE.test(fn.name);
    for (const match of code.slice(fn.start, fn.end + 1).matchAll(builder ? POLICY_DIFF_ONLY_MUT_REF_RE : POLICY_DIFF_ONLY_MUT_STATE_RE)) add("R8", fn.start + match.index, "&mut");
  }
  const applyEverywhere = !infrastructure && (underMutations || underSchema || underSupport);
  const withoutUses = code.replace(POLICY_RUST_USE_RE, (statement) => statement.replace(/[^\n]/g, " "));
  for (const match of withoutUses.matchAll(POLICY_DIFF_ONLY_APPLY_RE))
    if (!within(appliers, match.index) && (applyEverywhere || within(scannedSpans, match.index) || within(mutationImpls, match.index))) add("R9", match.index, match[0]);
  for (const match of code.matchAll(POLICY_DIFF_ONLY_BETWEEN_RE)) if (within(scannedSpans, match.index)) add("R10", match.index, match[0]);
  for (const fn of inverseFns) {
    const body = code.slice(fn.bodyStart, fn.end + 1);
    for (const match of body.matchAll(POLICY_DIFF_ONLY_SIMULATOR_RE)) add("R11", fn.bodyStart + match.index, match[0]);
    if (!diffTypeSide(fn)) {
      for (const match of body.matchAll(POLICY_DIFF_ONLY_DIFF_CALL_RE)) add("R11", fn.bodyStart + match.index, match[0]);
      for (const match of body.matchAll(POLICY_DIFF_ONLY_INVERSE_HELPER_RE)) {
        const open = match.index + match[0].length - 1;
        const args = body.slice(open + 1, policyRustParenEnd(body, open));
        if (/diff/i.test(match[1]!) || POLICY_DIFF_ONLY_DIFF_ARGUMENT_RE.test(args)) add("R11", fn.bodyStart + match.index, `${match[1]}(`);
      }
    }
    for (const match of body.matchAll(POLICY_DIFF_ONLY_RESTORE_RE)) add("R14", fn.bodyStart + match.index, match[0]);
  }
  for (const fn of scanned) {
    const roots = fn.refParams.filter((name) => !POLICY_DIFF_ONLY_PAYLOAD_PARAM_RE.test(name));
    if (roots.length === 0) continue;
    const body = code.slice(fn.bodyStart, fn.end + 1);
    if (!body.includes("let mut")) continue;
    const copy = new RegExp(`\\blet\\s+mut\\s+\\w+\\s*(?::[^=;]*)?=\\s*(?:\\(\\s*\\*\\s*)?(?:${roots.join("|")})\\b[^;{}]*?(?:\\.(?:clone|to_owned|to_vec)\\s*\\(\\s*\\)|\\.iter\\s*\\(\\s*\\)\\s*\\.cloned\\s*\\(\\s*\\)\\s*\\.collect[^;]*)\\s*\\)?\\s*;`, "g");
    for (const match of body.matchAll(copy)) add("R12", fn.bodyStart + match.index, "let mut … = <ref param>….clone()");
  }
  for (const impl of mutationImpls) {
    const declared = POLICY_DIFF_ONLY_TYPE_DIFF_RE.exec(code.slice(impl.bodyStart, impl.end));
    if (!declared) continue;
    const name = policyRustTypeName(declared[1]!);
    if (name === "Self" || name === policyRustTypeName(impl.self) || (impl.args[0] !== undefined && name === policyRustTypeName(impl.args[0]))) add("R13", impl.bodyStart + declared.index, `type Diff = ${name}`);
  }
  return breaches;
}

/**
 * 🧪️Rule R15, pure: every direct mutation leaf (a directory below a `🧬️mutations` directory that owns a `🔺️diff` and no
 * `🧩️plan`) needs a Rust test under its OWN `🧪️tests` that calls the framework's `assert_mutation_inverse_sum_law`
 * (L3: the inverse diffs sum to the negative diff). `files` is the git-visible inventory, `read` returns a file's content.
 */
export function policyInverseSumLawUntestedLeafBreaches(files: readonly string[], read: (relPath: string) => string): BreachRecord[] {
  const leaves = new Map<string, string>();
  const composites = new Set<string>();
  const tested = new Set<string>();
  for (const relPath of files) {
    const segments = relPath.split("/");
    const facet = segments.indexOf(POLICY_MUTATIONS_FACET);
    if (facet === -1) continue;
    const planAt = segments.indexOf(POLICY_MUTATION_PLAN_DIR, facet + 1);
    if (planAt !== -1) composites.add(segments.slice(0, planAt).join("/"));
    const testsAt = segments.indexOf("🧪️tests", facet + 1);
    if (testsAt !== -1) {
      if (relPath.endsWith(".rs") && read(relPath).includes("assert_mutation_inverse_sum_law")) tested.add(segments.slice(0, testsAt).join("/"));
      continue;
    }
    const diffAt = segments.indexOf("🔺️diff", facet + 1);
    if (diffAt !== -1 && relPath.endsWith(".rs")) {
      const leaf = segments.slice(0, diffAt).join("/");
      if (!leaves.has(leaf)) leaves.set(leaf, relPath);
    }
  }
  return [...leaves]
    .filter(([leaf]) => !composites.has(leaf) && !tested.has(leaf))
    .map(([leaf, diffRel]) => policyDiffOnlyBreach("R15", diffRel, 1, `${leaf.split("/").at(-1)}/🧪️tests`));
}

const POLICY_DIFF_ONLY_SCANS = new Map<string, Map<string, BreachRecord[]>>();

/** 🗂️The per-file rules over the shared inventory's Rust sources, scanned ONCE per root and grouped by rule slug. */
function policyDiffOnlyScan(repoRoot: string): Map<string, BreachRecord[]> {
  const cached = POLICY_DIFF_ONLY_SCANS.get(repoRoot);
  if (cached) return cached;
  const byKind = new Map<string, BreachRecord[]>();
  for (const relPath of policyMutationLawRustFiles(repoRoot))
    for (const breach of policyDiffOnlyFileBreaches(relPath, policyReadFileSafe(repoRoot, relPath))) {
      const rows = byKind.get(breach.kind);
      if (rows) rows.push(breach);
      else byKind.set(breach.kind, [breach]);
    }
  POLICY_DIFF_ONLY_SCANS.set(repoRoot, byKind);
  return byKind;
}

/** 📏️Rule R8: no `&mut` in the signature or body of any scanned diff, inverse or helper fn (L4). */
export function policyLeafMutableBaseBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R8.slug}`) ?? [];
}

/** 📏️Rule R9: no `.apply(`, `apply_diff(` or `ApplyCapability` anywhere under `🧬️schema`/`🧬️mutations`/`🧬️mutation-support` or in scanned fns (L4/L5); only a diff type's own `apply` (and the `*apply*` helpers it calls) may forward its capability. */
export function policyLeafCentralApplyBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R9.slug}`) ?? [];
}

/** 📏️Rule R10: no `*between*`, `*_replacing`, `*_state_diff`, `state_after*` or `value_diff*` call in a scanned fn (L1: the diff is declarative, never a snapshot difference or simulation). */
export function policyLeafBetweenBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R10.slug}`) ?? [];
}

/** 📏️Rule R11: an inverse never calls `diff(`, `.diff()`, `negative`, `state_after`, `inverse_of`, `value_diff*` or a `*_inverse(…diff…)` helper (L2/AMB-3). */
export function policyDiffDerivedInverseBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R11.slug}`) ?? [];
}

/** 📏️Rule R12: no scanned fn copies a reference parameter into a mutable binding (L1). */
export function policyBaseCloneDiffBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R12.slug}`) ?? [];
}

/** 📏️Rule R13: the diff type is never the snapshot it applies to (`type Diff = <snapshot>`, `impl MutationDiff<X> for X`) (L1). */
export function policyWholeStateDiffBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R13.slug}`) ?? [];
}

/** 📏️Rule R14: an inverse never returns a `SetSnapshot`/`PatchSnapshot`/`Replace…`/`Restore…`/`*Snapshot` variant nor a whole `*_state`/`*_content` record (L2). */
export function policyRestoreInverseBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R14.slug}`) ?? [];
}

/** 📏️Rule R15: every direct mutation leaf has a test under its own `🧪️tests` calling `assert_mutation_inverse_sum_law` (L3). */
export function policyInverseSumLawUntestedBreaches(repoRoot: string): BreachRecord[] {
  return policyInverseSumLawUntestedLeafBreaches(policyMutationLawInventory(repoRoot).files, (relPath) => policyReadFileSafe(repoRoot, relPath));
}

/** 📏️Rule R16: `MutationOutcome::apply_to` and `.apply_to(` are gone from every Rust file (L4). */
export function policyOutcomeApplyToBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R16.slug}`) ?? [];
}
