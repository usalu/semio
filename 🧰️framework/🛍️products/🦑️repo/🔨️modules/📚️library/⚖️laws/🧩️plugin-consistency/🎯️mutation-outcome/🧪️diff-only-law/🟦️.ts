import { createRequire } from "node:module";
import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { POLICY_MUTATION_PLAN_DIR, POLICY_MUTATIONS_FACET, policyArtifactRootOfMutationsDir, POLICY_RS_COMPONENT_LEAF_NAME } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyMutationLawInventory, policyMutationLawRustFiles, policyMutationLawSourceFiles } from "../🟦️.ts";
import { policyReadFileSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";

type PolicyDiffOnlyRuleId = "R8" | "R9" | "R10" | "R11" | "R12" | "R13" | "R14" | "R15" | "R16" | "R17" | "R18";

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
  R17: { code: "V3-LEAF-APPLY", slug: "host-apply", reason: "a host/editor/apply/preparation lane changes a snapshot outside `protocol::apply_diff` (`&mut <X>Snapshot` field writes, `snapshot_mut`, before/after differencing, TypeScript local apply into a snapshot ref)", solution: "Turn the gesture into a mutation whose sparse diff names the changed rows and apply it only with `protocol::apply_diff(&mutation.diff(&base).diff, &base)` (directly or through the artifact's `🌉️apply` wrapper); the shell dispatches the envelope and renders the store's projection." },
  R18: { code: "V1-SNAPSHOT-DIFF", slug: "row-emitter", reason: "a computational gesture commits an assembled mesh or compares rows with the base to sparsify (`commit_built`, `equals_base`, a kernel `*_job(` builder, `sparse_rows(`, `::produced(`)", solution: "Make the gesture a row emitter: read the base through the write-recording overlay, decide which faces and vertices it replaces, removes or appends, and state exactly those rows through the face editor; the journal is the diff." },
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
/** 📐️Pure geometry named `…_between` (the angle, distance or gap between two vectors or points) is not snapshot differencing and never counts for R10. */
const POLICY_DIFF_ONLY_GEOMETRY_BETWEEN_RE = /^(?:\w+_)?(?:angle|distance|gap)_between\b/;
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

/** 🫧️The ephemeral-root setter kinds: the presence/window-transient mutation TYPES that travel by whole-root transfer. Judged by the mutation's own type name, never by a directory. */
const POLICY_DIFF_ONLY_EPHEMERAL_MUTATION_RE = /^\w*(?:Transient|Presence)Mutation$/;
/** 🫧️The one whole-root setter an ephemeral mutation may invert to: `Self::Snapshot` / `Self::<Name>Snapshot` carrying the base root. */
const POLICY_DIFF_ONLY_EPHEMERAL_SETTER_RE = /^Self::\w*Snapshot$/;
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

const POLICY_HOST_APPLY_LANE_DIRS: ReadonlySet<string> = new Set(["✏️editor", "🌉️apply", "📬️preparation"]);
/** 🚪️Directories R17 never reads: snapshot decoders, test oracles and fixtures, and the trees R8–R16 already gate leaf by leaf. */
const POLICY_HOST_APPLY_EXCLUDED_DIRS: ReadonlySet<string> = new Set(["🚪️io", "🔮️oracles", "🧪️tests", "🧫️fixtures", "🧬️schema", "🧬️mutations", "🧬️mutation-support"]);
/** 🚪️Directories whose code is an exempt CONTEXT for the call sites of a snapshot-writing helper: decoders, oracles, tests. */
const POLICY_HOST_APPLY_SITE_EXEMPT_DIRS: ReadonlySet<string> = new Set(["🚪️io", "🔮️oracles", "🧪️tests", "🧫️fixtures"]);
const POLICY_HOST_APPLY_FRAMEWORK_PREFIX = "🧰️framework/🛍️products/💻️os/";
const POLICY_HOST_APPLY_PLUGIN_ROOT_RE = new RegExp(`^✏️s/🔌️plugins/[^/]+/🗿️artifacts/[^/]+/${POLICY_RS_COMPONENT_LEAF_NAME.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`);
/** 🔎️A `&mut <X>Snapshot` / `&mut <X>Document` parameter: the only parameter shape R17 follows (framework code has its own `*Document` containers, so there only `Snapshot` counts). */
const POLICY_HOST_APPLY_MUT_PARAM_RE = /^(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*&\s*(?:'\w+\s+)?mut\s+(?:\w+::)*\w*(?:Snapshot|Document)\b/;
const POLICY_HOST_APPLY_MUT_SNAPSHOT_PARAM_RE = /^(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*&\s*(?:'\w+\s+)?mut\s+(?:\w+::)*\w*Snapshot\b/;
/** 🔎️A fn that builds a load effect: everything it calls fills a freshly decoded document (genesis), not an apply lane. */
const POLICY_HOST_APPLY_LOAD_BUILDER_RE = /\breset_document_effect\b|\bLoadDocument\b/;
const POLICY_HOST_APPLY_PLACE = String.raw`(?:\s*\.\s*\w+|\s*\[[^\]\n]*\])*`;
const POLICY_HOST_APPLY_WRITE_METHODS = String.raw`push|push_str|push_back|push_front|pop|remove|insert|extend|extend_from_slice|truncate|clear|retain|swap|swap_remove|drain|append|resize|entry|take|replace|sort\w*|dedup\w*|set_\w+|\w+_mut`;
/** 🔎️A before/after differencing call: `object_patch_diff(&before, &after)`. */
const POLICY_HOST_APPLY_BEFORE_AFTER_CALL_RE = /\b(\w*(?:diff|delta|patch)\w*)\s*\(\s*&\s*\w*(?:before|old|prev|previous|original)\w*\s*,\s*&\s*\w*(?:after|new|next|updated|edited)\w*\s*[,)]/gi;
const POLICY_HOST_APPLY_BEFORE_NAME_RE = /^(?:\w*_)?(?:before|old|prev|previous|original)\w*$/i;
const POLICY_HOST_APPLY_AFTER_NAME_RE = /^(?:\w*_)?(?:after|new|next|updated|edited)\w*$/i;
const POLICY_HOST_APPLY_TS_LOCAL_APPLY_RE = /\bapply(?:Local|Optimistic)\w*\s*\(|\bapply\w*Mutation\w*Local\w*\s*\(/;
const POLICY_HOST_APPLY_TS_REF_ASSIGN_RE = /\b(\w*(?:Catalog|Snapshot|Document|Archive)\w*Ref)\.current\s*=\s*([^;\n]+)/g;
/** 🔎️TypeScript probe: only files that mention an archive put/restore or a snapshot ref are parsed. */
const POLICY_HOST_APPLY_TS_PROBE_RE = /Ref\.current|localDocumentArchive|encodeDocumentArchiveBytes|restore\w*Archive/;
/** 🗃️The archive read back from the store/plugin after its central apply: persisting THAT is the persistence lane, not a second apply. */
const POLICY_HOST_APPLY_TS_READ_BACK_RE = /\bread\w*DocumentArchive\w*\s*\(/;
/** 🚪️An archive that enters from outside (admission, folder read-back, decode, parse, load): an import/admission path. */
const POLICY_HOST_APPLY_TS_IMPORT_RE = /[Aa]dmission|ReadBack|\b(?:decode|parse|load|admit|verify)\w*Archive\w*\s*\(|\bloadAppDocumentArchive\b/;
const POLICY_HOST_APPLY_TS_SEND_RE = /\bpostMessage\s*\(|\.send\s*\(/;
const POLICY_HOST_APPLY_TS_RESTORE_RE = /^restore\w*Archive\w*$/;

/** 🏠️Which language lane R17 reads `relPath` in: `rust` host/editor/apply/preparation/crate-root/framework-os files, `ts` for non-test shell sources, `null` otherwise. */
function policyHostApplyScope(relPath: string): "rust" | "ts" | null {
  const directories = relPath.split("/").slice(0, -1);
  if (directories.some((segment) => POLICY_HOST_APPLY_EXCLUDED_DIRS.has(segment) || segment === "node_modules")) return null;
  if (relPath.endsWith(".rs")) {
    const host = directories.some((segment, index) => segment === "🔨️modules" && directories[index + 1] === "🏠️host");
    return host || directories.some((segment) => POLICY_HOST_APPLY_LANE_DIRS.has(segment)) || relPath.startsWith(POLICY_HOST_APPLY_FRAMEWORK_PREFIX) || POLICY_HOST_APPLY_PLUGIN_ROOT_RE.test(relPath) ? "rust" : null;
  }
  if (/\.tsx?$/.test(relPath) && !/\.(?:test|spec)\.tsx?$/.test(relPath) && !relPath.endsWith(".d.ts")) return "ts";
  return null;
}

/** 🧭️The files whose text resolves a helper's call sites: a plugin artifact's tree, a framework os module's tree, else the file's own directory. */
export function policyHostApplyGroupKey(relPath: string): string {
  const segments = relPath.split("/");
  if (segments[0] === "✏️s" && segments[1] === "🔌️plugins" && segments[3] === "🗿️artifacts" && segments.length > 5) return segments.slice(0, 5).join("/");
  const module = segments.indexOf("🔨️modules");
  if (relPath.startsWith(POLICY_HOST_APPLY_FRAMEWORK_PREFIX) && module !== -1 && segments.length > module + 2) return segments.slice(0, module + 2).join("/");
  return segments.slice(0, -1).join("/");
}

/** 🔬️One parsed Rust file for R17: comments, literals and `#[cfg(test)]` items blanked, plus its fns, impls and diff machinery. */
type PolicyHostApplyRustFile = Readonly<{ relPath: string; allCode: string; code: string; fns: readonly PolicyRustFn[]; impls: readonly PolicyRustImpl[]; machinery: ReadonlySet<string>; exemptDirectory: boolean }>;

function policyHostApplyParseRust(relPath: string, content: string): PolicyHostApplyRustFile {
  const all = policyMaskRustSource(content);
  const close = policyRustBraceClose(all);
  const code = policyRustBlankTestItems(all, close);
  const impls = policyRustImpls(code, close);
  const fns = policyRustFns(code, close);
  const machinery = new Set<string>();
  const reach = (from: number, to: number) => {
    for (const call of code.slice(from, to).matchAll(/\b([A-Za-z_]\w*)\s*\(/g)) {
      if (machinery.has(call[1]!)) continue;
      const callees = fns.filter((fn) => fn.name === call[1]);
      if (callees.length === 0) continue;
      machinery.add(call[1]!);
      for (const callee of callees) reach(callee.bodyStart, callee.end);
    }
  };
  for (const impl of impls) if (impl.kind === "diff") reach(impl.bodyStart, impl.end);
  return { relPath, allCode: all, code, fns, impls, machinery, exemptDirectory: relPath.split("/").slice(0, -1).some((segment) => POLICY_HOST_APPLY_SITE_EXEMPT_DIRS.has(segment)) };
}

/**
 * 🧮️Is a snapshot-writing helper exempt because EVERY call site of it, resolved textually within its group, sits in an exempt context
 * — an `🚪️io`/`🔮️oracles`/`🧪️tests` file, a fn that builds a `LoadDocument`/`reset_document_effect`, a `MutationDiff` impl or the helpers it calls —
 * or in a helper that is itself exempt by the same rule? A helper with no resolvable call site is NOT exempt (nothing proves where it runs).
 */
function policyHostApplyHelperExempt(name: string, files: ReadonlyMap<string, PolicyHostApplyRustFile>, depth: number, visiting: Set<string>): boolean {
  const key = name;
  if (depth > 6 || visiting.has(key)) return false;
  visiting.add(key);
  try {
    const callPattern = new RegExp(String.raw`(?<!\bfn\s+)(?<![\w])${name}\s*(?:::<[^>]*>)?\s*\(`, "g");
    let sites = 0;
    for (const file of files.values()) {
      for (const match of file.allCode.matchAll(callPattern)) {
        sites++;
        if (file.exemptDirectory || !/\S/.test(file.code.slice(match.index, match.index + match[0].length))) continue;
        const enclosing = file.fns.filter((fn) => fn.start <= match.index && match.index <= fn.end).sort((a, b) => b.start - a.start)[0];
        if (enclosing === undefined) return false;
        if (enclosing.name === name) continue;
        if (file.impls.some((impl) => impl.kind === "diff" && impl.bodyStart <= match.index && match.index <= impl.end) || file.machinery.has(enclosing.name)) continue;
        if (POLICY_HOST_APPLY_LOAD_BUILDER_RE.test(file.code.slice(enclosing.bodyStart, enclosing.end + 1))) continue;
        if (!policyHostApplyHelperExempt(enclosing.name, files, depth + 1, visiting)) return false;
      }
    }
    return sites > 0;
  } finally {
    visiting.delete(key);
  }
}

let policyHostApplyTypescript: typeof import("typescript") | undefined;

/** 🔎️TypeScript R17: whole-archive puts and archive restores from handlers that are not import/admission/persistence paths, and snapshot refs written from a local apply. */
function policyHostApplyTypescriptBreaches(relPath: string, content: string, add: (index: number, token: string, detail: string) => void): void {
  if (!POLICY_HOST_APPLY_TS_PROBE_RE.test(content)) return;
  const code = content.replace(/\/\*[\s\S]*?\*\/|\/\/[^\n]*/g, (comment) => comment.replace(/[^\n]/g, " "));
  const locals = new Set<string>();
  for (const match of code.matchAll(/\b(?:const|let)\s+(\w+)\s*(?::[^=;\n]+)?=\s*(?:await\s+)?(?:\w+\.)*apply(?:Local|Optimistic)\w*\s*\(/g)) locals.add(match[1]!);
  for (const match of code.matchAll(POLICY_HOST_APPLY_TS_REF_ASSIGN_RE)) {
    const rhs = match[2]!.trim();
    if (POLICY_HOST_APPLY_TS_LOCAL_APPLY_RE.test(rhs) || locals.has(rhs.replace(/\W.*$/s, ""))) add(match.index, `${match[1]}.current = ${rhs.slice(0, 40)}`, "The shell writes the snapshot ref from a local apply instead of replacing it with the store's central-apply projection.");
  }
  if (!/localDocumentArchive|encodeDocumentArchiveBytes|restore\w*Archive/.test(code)) return;
  policyHostApplyTypescript ??= createRequire(import.meta.url)("typescript") as typeof import("typescript");
  const ts = policyHostApplyTypescript;
  const source = ts.createSourceFile(relPath, content, ts.ScriptTarget.Latest, true, relPath.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const isFunction = (node: import("typescript").Node): node is import("typescript").FunctionLikeDeclaration => ts.isArrowFunction(node) || ts.isFunctionExpression(node) || ts.isFunctionDeclaration(node) || ts.isMethodDeclaration(node);
  const lastSegment = (text: string): string => text.replace(/\??\.current$/, "").split(/[.?]+/).at(-1) ?? "";
  const declaredName = (fn: import("typescript").Node): string | null => {
    let parent: import("typescript").Node | undefined = fn.parent;
    while (parent !== undefined && (ts.isParenthesizedExpression(parent) || ts.isAsExpression(parent) || (ts.isCallExpression(parent) && /^use(?:Callback|Memo)$/.test(parent.expression.getText(source))))) parent = parent.parent;
    if (parent === undefined) return null;
    if (ts.isFunctionDeclaration(fn) || ts.isMethodDeclaration(fn)) return fn.name?.getText(source) ?? null;
    if (ts.isVariableDeclaration(parent)) return parent.name.getText(source);
    if (ts.isPropertyAssignment(parent)) return parent.name.getText(source);
    if (ts.isBinaryExpression(parent) && parent.operatorToken.kind === ts.SyntaxKind.EqualsToken) return lastSegment(parent.left.getText(source));
    return null;
  };
  const contextOf = (node: import("typescript").Node): import("typescript").Node => {
    const hookCallback = (at: import("typescript").Node): boolean => ts.isCallExpression(at.parent) && /^use[A-Z]\w*$/.test(at.parent.expression.getText(source));
    for (let at: import("typescript").Node | undefined = node.parent; at !== undefined; at = at.parent) if (isFunction(at) && (declaredName(at) !== null || hookCallback(at))) return at;
    return source;
  };
  const isBranch = (node: import("typescript").Node): boolean => (ts.isIfStatement(node.parent) && (node.parent.thenStatement === node || node.parent.elseStatement === node)) || ts.isCaseClause(node) || ts.isDefaultClause(node);
  const branchOf = (site: import("typescript").Node): import("typescript").Node | null => {
    for (let at: import("typescript").Node | undefined = site.parent; at !== undefined && at !== source; at = at.parent) {
      if (isFunction(at) && (declaredName(at) !== null)) return null;
      if (isBranch(at)) return at;
    }
    return null;
  };
  const callSites = new Map<string, import("typescript").CallExpression[]>();
  const walkCalls = (node: import("typescript").Node): void => {
    if (ts.isCallExpression(node)) {
      const name = lastSegment(node.expression.getText(source));
      callSites.set(name, [...(callSites.get(name) ?? []), node]);
    }
    ts.forEachChild(node, walkCalls);
  };
  walkCalls(source);
  const exempt = (context: import("typescript").Node, depth: number, visiting: Set<import("typescript").Node>): boolean => {
    if (context === source) return false;
    const text = context.getText(source);
    if (POLICY_HOST_APPLY_TS_READ_BACK_RE.test(text) || POLICY_HOST_APPLY_TS_IMPORT_RE.test(text)) return true;
    const name = isFunction(context) ? declaredName(context) : null;
    if (name === null || depth > 6 || visiting.has(context)) return false;
    const sites = callSites.get(name) ?? [];
    visiting.add(context);
    const all = sites.length > 0 && sites.every((site) => {
      const branch = branchOf(site);
      return branch === null ? exempt(contextOf(site), depth + 1, visiting) : POLICY_HOST_APPLY_TS_READ_BACK_RE.test(branch.getText(source)) || POLICY_HOST_APPLY_TS_IMPORT_RE.test(branch.getText(source));
    });
    visiting.delete(context);
    return all;
  };
  const flag = (node: import("typescript").Node, token: string, detail: string): void => add(node.getStart(source), token, detail);
  const putContexts = new Set<import("typescript").Node>();
  const encodes: { node: import("typescript").Node; context: import("typescript").Node }[] = [];
  const walk = (node: import("typescript").Node): void => {
    if (ts.isPropertyAssignment(node) && node.name.getText(source) === "kind" && ts.isStringLiteralLike(node.initializer) && node.initializer.text === "localDocumentArchive") {
      const context = contextOf(node);
      if (!exempt(context, 0, new Set())) {
        putContexts.add(context);
        flag(node, 'kind: "localDocumentArchive"', "A whole-archive put from a handler that is not an import, admission or persistence path replaces the document without a mutation envelope, history or inverse; dispatch a mutation instead.");
      }
    }
    if (ts.isCallExpression(node)) {
      const callee = lastSegment(node.expression.getText(source));
      const context = contextOf(node);
      if (callee === "encodeDocumentArchiveBytes" && POLICY_HOST_APPLY_TS_SEND_RE.test(context.getText(source))) encodes.push({ node, context });
      if (POLICY_HOST_APPLY_TS_RESTORE_RE.test(callee) && !(isFunction(context) && POLICY_HOST_APPLY_TS_RESTORE_RE.test(declaredName(context) ?? "")) && !exempt(context, 0, new Set()))
        flag(node, `${callee}(…)`, "A revert path restores a captured whole archive over the user's edits; replay the recorded mutations' concrete inverses in reverse instead.");
    }
    ts.forEachChild(node, walk);
  };
  walk(source);
  for (const { node, context } of encodes)
    if (!putContexts.has(context) && !exempt(context, 0, new Set())) {
      putContexts.add(context);
      flag(node, "encodeDocumentArchiveBytes(…) sent", "A handler encodes a whole archive and sends it outside any import, admission or persistence path; dispatch a mutation instead.");
    }
}

/**
 * 🏠️Rule R17 over ONE source file, pure (L4/L5 beyond `🧬️schema`/`🧬️mutations`): a host, editor, apply or preparation lane turns a mutation into a
 * snapshot only through `protocol::apply_diff`. Rust (`policyHostApplyScope`): a fn with a `&mut <X>Snapshot`/`&mut <X>Document` parameter that WRITES
 * through it (assignment, `push`/`remove`/`insert`/… or `*_mut(` on a path rooted at it, `mem::replace|take|swap(&mut it…)`) — except `*it = …apply_diff(…)`, which is
 * the central application itself —, any `snapshot_mut`, and a before/after differencing helper (`object_patch_diff(&before, &after)`, or a diff/delta/patch fn
 * taking a `before` and an `after` reference). A writing helper is exempt only when it is `MutationDiff` machinery or EVERY call site of it, resolved textually within
 * `group` (the files of its artifact/module tree; the file itself by default), sits in an exempt context — see `policyHostApplyHelperExempt`; there are no name-based exemptions.
 * TypeScript: `<catalog|snapshot|document|archive>…Ref.current = applyLocal…(…)` (directly or through a local bound to it); a `kind: "localDocumentArchive"` put or an
 * `encodeDocumentArchiveBytes` send, and a `restore…Archive(…)` revert call, in a declared function that does not read the archive back from the store, does not take it from an
 * admission/decode/load, and whose every call site is not such a function either. `#[cfg(test)]` items, comments and literals never count; decoders live in `🚪️io` and are out of scope.
 */
export function policyHostApplyFileBreaches(relPath: string, content: string, group?: () => ReadonlyMap<string, string>): BreachRecord[] {
  const scope = policyHostApplyScope(relPath);
  if (scope === null) return [];
  const breaches: BreachRecord[] = [];
  const seen = new Set<number>();
  const lineOf = policyRustLineLookup(content);
  const add = (index: number, token: string, detail: string) => {
    const line = lineOf(index);
    if (seen.has(line)) return;
    seen.add(line);
    const breach = policyDiffOnlyBreach("R17", relPath, line, token.replace(/\s+/g, " ").trim());
    breaches.push({ ...breach, reason: `${breach.reason} ${detail}` });
  };
  if (scope === "ts") {
    policyHostApplyTypescriptBreaches(relPath, content, add);
    return breaches;
  }
  const self = policyHostApplyParseRust(relPath, content);
  const { code, fns, machinery } = self;
  const framework = relPath.startsWith(POLICY_HOST_APPLY_FRAMEWORK_PREFIX);
  const pending: { index: number; token: string; detail: string; fn: string }[] = [];
  for (const match of code.matchAll(/\bsnapshot_mut\b/g)) add(match.index, "snapshot_mut", "A mutable snapshot escape hatch lets callers write fields outside the central applier.");
  if (!framework) for (const match of code.matchAll(POLICY_HOST_APPLY_BEFORE_AFTER_CALL_RE)) add(match.index, `${match[1]}(&before, &after)`, "A before/after differencing helper builds the diff from two snapshots (V1-SNAPSHOT-DIFF).");
  for (const fn of fns) {
    const open = code.indexOf("(", fn.start);
    if (open < 0 || open > fn.bodyStart) continue;
    const params = policyRustSplitTop(code.slice(open + 1, policyRustParenEnd(code, open)));
    if (/diff|delta|patch/.test(fn.name)) {
      const refs = params.filter((param) => !framework || /Snapshot|Document/.test(param)).map((param) => /^(?:mut\s+)?(\w+)\s*:\s*&(?!\s*mut\b)/.exec(param)?.[1]).filter((name): name is string => name !== undefined);
      if (refs.some((name) => POLICY_HOST_APPLY_BEFORE_NAME_RE.test(name)) && refs.some((name) => POLICY_HOST_APPLY_AFTER_NAME_RE.test(name))) add(fn.start, `fn ${fn.name}(before, after)`, "A before/after differencing helper builds the diff from two snapshots (V1-SNAPSHOT-DIFF).");
    }
    if (machinery.has(fn.name)) continue;
    const body = code.slice(fn.bodyStart, fn.end + 1);
    for (const param of params) {
      const name = (framework ? POLICY_HOST_APPLY_MUT_SNAPSHOT_PARAM_RE : POLICY_HOST_APPLY_MUT_PARAM_RE).exec(param)?.[1];
      if (name === undefined) continue;
      const writes: { index: number; token: string }[] = [];
      for (const match of body.matchAll(new RegExp(String.raw`(?<![\w.])(\*\s*)?${name}${POLICY_HOST_APPLY_PLACE}\s*(?:[-+*/|&^%]|<<|>>)?=(?![=>])`, "g"))) {
        if (match[1] !== undefined && match[0].replace(/\s+/g, "") === `*${name}=`) {
          const end = body.indexOf(";", match.index);
          const statement = body.slice(match.index + match[0].length, end === -1 ? body.length : end);
          if (/\bapply_diff\s*\(/.test(statement)) continue;
          const bound = /^\s*\(?\s*(\w+)\s*\)?\s*$/.exec(statement)?.[1];
          if (bound !== undefined && new RegExp(String.raw`\b(?:let|Ok\()[^;{}]*\b${bound}\b[^;{}]*?=[^;]*?\bapply_(?:diff|mutation|\w+_mutation\w*)\s*\(`).test(body.slice(0, match.index))) continue;
        }
        writes.push({ index: match.index, token: match[0] });
      }
      for (const match of body.matchAll(new RegExp(String.raw`(?<![\w.])${name}${POLICY_HOST_APPLY_PLACE}\s*\.\s*(?:${POLICY_HOST_APPLY_WRITE_METHODS})\s*\(`, "g"))) writes.push({ index: match.index, token: match[0] });
      for (const match of body.matchAll(new RegExp(String.raw`\b(?:std::)?mem::(?:replace|take|swap)\s*\(\s*&\s*mut\s+\*?\s*${name}\b`, "g"))) writes.push({ index: match.index, token: match[0] });
      for (const write of writes) pending.push({ index: fn.bodyStart + write.index, token: write.token, detail: `\`${fn.name}\` writes fields of its \`&mut\` snapshot parameter \`${name}\` instead of applying a diff through \`protocol::apply_diff\`.`, fn: fn.name });
    }
  }
  if (pending.length > 0) {
    const files = new Map<string, PolicyHostApplyRustFile>();
    for (const [path, text] of group?.() ?? []) if (path !== relPath && path.endsWith(".rs")) files.set(path, policyHostApplyParseRust(path, text));
    files.set(relPath, self);
    const verdicts = new Map<string, boolean>();
    for (const finding of pending) {
      if (!verdicts.has(finding.fn)) verdicts.set(finding.fn, policyHostApplyHelperExempt(finding.fn, files, 0, new Set()));
      if (!verdicts.get(finding.fn)) add(finding.index, finding.token, finding.detail);
    }
  }
  return breaches;
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
 * every non-test Rust file. R13 and R14 apply to every lane and mutation type; the single exemption is R14's
 * `Self::…Snapshot` setter inside an `impl Mutation<…> for *TransientMutation|*PresenceMutation` (the ephemeral
 * whole-root setter, inverted to itself carrying the base root — design.md "Ephemeral roots"): decided by the mutation's
 * TYPE name, never by a path, and R13 stays strict there because the diff must remain sparse. R16 scans every Rust file.
 * The planted-violation law (`🧪️tests/🧪️diff-only-law-gate`) drives exactly this function.
 */
function policyDiffOnlyLeafFileBreaches(relPath: string, content: string): BreachRecord[] {
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
  for (const match of code.matchAll(POLICY_DIFF_ONLY_BETWEEN_RE)) if (within(scannedSpans, match.index) && !POLICY_DIFF_ONLY_GEOMETRY_BETWEEN_RE.test(match[0])) add("R10", match.index, match[0]);
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
    const owner = innermost(fn.start);
    const ephemeralSetter = owner?.kind === "mutation" && POLICY_DIFF_ONLY_EPHEMERAL_MUTATION_RE.test(policyRustTypeName(owner.self));
    for (const match of body.matchAll(POLICY_DIFF_ONLY_RESTORE_RE))
      if (!(ephemeralSetter && POLICY_DIFF_ONLY_EPHEMERAL_SETTER_RE.test(match[0]))) add("R14", fn.bodyStart + match.index, match[0]);
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

/** 🔎️R18 tokens: the commit of an assembled mesh, the base-equality filter that sparsifies it, a kernel topology builder (`extrude_faces_job`, `bevel_job`, …) and the whole-arena difference oracles. */
const POLICY_ROW_EMITTER_BANNED_RE = /\b(?:commit_built|equals_base|sparse_rows)\b|::produced\s*\(|\.(?:extrude_faces|inset_faces|bevel|loop_cut|subdivide_faces|triangulate|mirror|decimate|flip_faces|merge_vertices|dissolve_edges|snap_vertices)_job(?:_owned)?\s*\(/g;
/** 🔎️Cheap probe: only files naming a banned token are parsed. */
const POLICY_ROW_EMITTER_PROBE_RE = /commit_built|equals_base|sparse_rows|::produced|_job(?:_owned)?\s*\(/;

/**
 * 🧱️Rule R18 over ONE source file, pure: a computational mesh gesture is a row emitter. In the non-test code of a `🧬️mutations` tree and
 * of the mesh overlay (`🥽️mesh/📝️journal`) nothing may commit an assembled mesh, filter rows against the base to sparsify them, call a kernel topology
 * builder or difference whole arenas: that is a before/after comparison (V1-SNAPSHOT-DIFF) however it is named.
 */
export function policyRowEmitterFileBreaches(relPath: string, content: string): BreachRecord[] {
  if (!relPath.endsWith(".rs") || !POLICY_ROW_EMITTER_PROBE_RE.test(content)) return [];
  const directories = relPath.split("/").slice(0, -1);
  if (directories.some((segment) => segment === "🧪️tests" || segment === "🧫️fixtures")) return [];
  const emitter = directories.includes(POLICY_MUTATIONS_FACET) || (directories.includes("🥽️mesh") && directories.includes("📝️journal"));
  if (!emitter) return [];
  const masked = policyMaskRustSource(content);
  const code = policyRustBlankTestItems(masked, policyRustBraceClose(masked));
  const lineOf = policyRustLineLookup(content);
  const seen = new Set<number>();
  const breaches: BreachRecord[] = [];
  for (const match of code.matchAll(POLICY_ROW_EMITTER_BANNED_RE)) {
    const line = lineOf(match.index);
    if (seen.has(line)) continue;
    seen.add(line);
    breaches.push(policyDiffOnlyBreach("R18", relPath, line, match[0].replace(/\s+/g, " ").trim()));
  }
  return breaches;
}

/** 🔬️Rules R8–R14, R16 (leaf trees), R17 (host/editor apply lanes) and R18 (row emitters) over ONE source file, pure. The planted-violation law (`🧪️tests/🧪️diff-only-law-gate`) drives exactly this function. */
export function policyDiffOnlyFileBreaches(relPath: string, content: string, group?: () => ReadonlyMap<string, string>): BreachRecord[] {
  return [...policyDiffOnlyLeafFileBreaches(relPath, content), ...policyHostApplyFileBreaches(relPath, content, group), ...policyRowEmitterFileBreaches(relPath, content)];
}

const POLICY_INVERSE_SUM_LAW_CALL = "assert_mutation_inverse_sum_law";
const POLICY_LAW_PAYLOAD_IMPL_RE = /\bMutationKind\s*<[^;{]*?>\s*for\s+(\w+)/g;

/** 🔎️The named fns of a Rust source as `{name, start, end}` spans over the braces of their bodies; comments, strings and chars are blanked first so a brace inside them never unbalances the scan. */
function policyRustFnSpans(content: string): { name: string; start: number; end: number }[] {
  const blanked = content.replace(/\/\/[^\n]*|\/\*[\s\S]*?\*\/|r#*"[\s\S]*?"#*|"(?:\\[\s\S]|[^"\\])*"|'(?:\\.|[^'\\])'/g, (hit) => " ".repeat(hit.length));
  const spans: { name: string; start: number; end: number }[] = [];
  for (const head of blanked.matchAll(/\bfn\s+(\w+)\b[^{;]*\{/g)) {
    const start = head.index + head[0].length - 1;
    let depth = 0;
    let end = start;
    for (; end < blanked.length; end++) {
      const ch = blanked[end];
      if (ch === "{") depth++;
      else if (ch === "}" && --depth === 0) break;
    }
    spans.push({ name: head[1]!, start, end });
  }
  return spans;
}

/** 🔎️Everything a test source's sum-law fns can reach by name: the fns calling `assert_mutation_inverse_sum_law`, the same-file fns they call and the same-file fns calling them (two hops each way) — the text a family test names its variants in. Empty when the file never calls the law inside a fn. */
function policyInverseSumLawReach(content: string): string {
  if (!content.includes(POLICY_INVERSE_SUM_LAW_CALL)) return "";
  const spans = policyRustFnSpans(content);
  const bodyOf = (span: { start: number; end: number }): string => content.slice(span.start, span.end + 1);
  const seeds = new Set(spans.filter((span) => bodyOf(span).includes(POLICY_INVERSE_SUM_LAW_CALL)).map((span) => span.name));
  if (seeds.size === 0) return "";
  const reached = new Set(seeds);
  for (let hop = 0; hop < 2; hop++)
    for (const span of spans) {
      const body = bodyOf(span);
      if (reached.has(span.name)) {
        for (const call of body.matchAll(/\b(\w+)\s*\(/g)) if (spans.some((other) => other.name === call[1])) reached.add(call[1]!);
      } else if ([...reached].some((name) => new RegExp(`\\b${name}\\s*\\(`).test(body))) reached.add(span.name);
    }
  return spans.filter((span) => reached.has(span.name)).map(bodyOf).join("\n");
}

/** 🗺️The tree a leaf's family tests live in: its `🪆️subsets/<subset>` directory when it has one (stdio standards), else the artifact root above its `🧬️mutations` directory. */
function policyInverseSumLawFamilyScope(leaf: string): string {
  const segments = leaf.split("/");
  const facet = segments.indexOf(POLICY_MUTATIONS_FACET);
  const subsets = segments.indexOf("🪆️subsets");
  if (subsets !== -1 && subsets + 1 < facet) return segments.slice(0, subsets + 2).join("/");
  return policyArtifactRootOfMutationsDir(segments.slice(0, facet + 1).join("/"));
}

/**
 * 🧪️Rule R15, pure: every direct mutation leaf needs the framework's `assert_mutation_inverse_sum_law` (L3: the inverse
 * diffs sum to the negative diff). A leaf is a directory below a `🧬️mutations` directory that owns a `🔺️diff`, or a
 * `🦀️.rs` that implements `MutationKind<..>` (stdio's leaf layout carries no `🔺️diff` directory); a leaf with a `🧩️plan`
 * is composite and exempt. It is tested when a test under its OWN `🧪️tests` calls the law, or when a family test of its
 * subset/artifact (any `🧪️tests` file) calls the law in a fn that NAMES the leaf — its `MutationKind` payload type, its enum
 * variant (the Pascal-cased leaf slug) or its quoted kebab slug — directly or through same-file helper fns. A family test
 * that calls the law without naming the leaf — a shared loop over cases defined elsewhere — does not count.
 * `files` is the git-visible inventory, `read` returns a file's content.
 */
export function policyInverseSumLawUntestedLeafBreaches(files: readonly string[], read: (relPath: string) => string): BreachRecord[] {
  const leaves = new Map<string, string>();
  const payloads = new Map<string, string[]>();
  const composites = new Set<string>();
  const tested = new Set<string>();
  const familyReach = new Map<string, string>();
  for (const relPath of files) {
    const segments = relPath.split("/");
    const facet = segments.indexOf(POLICY_MUTATIONS_FACET);
    const inTests = segments.includes("🧪️tests");
    if (inTests && relPath.endsWith(".rs")) {
      const reach = policyInverseSumLawReach(read(relPath));
      if (reach) familyReach.set(relPath, reach);
    }
    if (facet === -1) continue;
    const planAt = segments.indexOf(POLICY_MUTATION_PLAN_DIR, facet + 1);
    if (planAt !== -1) composites.add(segments.slice(0, planAt).join("/"));
    const testsAt = segments.indexOf("🧪️tests", facet + 1);
    if (testsAt !== -1) {
      if (relPath.endsWith(".rs") && read(relPath).includes(POLICY_INVERSE_SUM_LAW_CALL)) tested.add(segments.slice(0, testsAt).join("/"));
      continue;
    }
    const diffAt = segments.indexOf("🔺️diff", facet + 1);
    if (diffAt !== -1 && relPath.endsWith(".rs")) {
      const leaf = segments.slice(0, diffAt).join("/");
      if (!leaves.has(leaf)) leaves.set(leaf, relPath);
    } else if (segments.length === facet + 3 && segments[facet + 2] === POLICY_RS_COMPONENT_LEAF_NAME) {
      const kinds = [...read(relPath).matchAll(POLICY_LAW_PAYLOAD_IMPL_RE)].map((hit) => hit[1]!);
      if (kinds.length === 0) continue;
      const leaf = segments.slice(0, facet + 2).join("/");
      leaves.set(leaf, relPath);
      payloads.set(leaf, kinds);
    }
  }
  const named = (leaf: string): boolean => {
    const slug = leaf.split("/").at(-1)!.replace(/^[^a-z0-9]+/u, "");
    const variant = slug.split("-").map((part) => part.charAt(0).toUpperCase() + part.slice(1)).join("");
    const names = [...(payloads.get(leaf) ?? []), variant];
    const scope = `${policyInverseSumLawFamilyScope(leaf)}/`;
    const texts = [...familyReach].filter(([testRel]) => testRel.startsWith(scope)).map(([, reach]) => reach);
    return texts.some((text) => text.includes(`"${slug}"`) || names.some((name) => new RegExp(`\\b${name}\\b`).test(text)));
  };
  return [...leaves]
    .filter(([leaf]) => !composites.has(leaf) && !tested.has(leaf) && !named(leaf))
    .map(([leaf, anchorRel]) => policyDiffOnlyBreach("R15", anchorRel, 1, `${leaf.split("/").at(-1)}/🧪️tests`));
}

const POLICY_DIFF_ONLY_SCANS = new Map<string, Map<string, BreachRecord[]>>();

/** 🗂️The per-file rules over the shared inventory's Rust sources, scanned ONCE per root and grouped by rule slug. */
function policyDiffOnlyScan(repoRoot: string): Map<string, BreachRecord[]> {
  const cached = POLICY_DIFF_ONLY_SCANS.get(repoRoot);
  if (cached) return cached;
  const byKind = new Map<string, BreachRecord[]>();
  const sources = policyMutationLawSourceFiles(repoRoot);
  const groupFiles = new Map<string, string[]>();
  for (const relPath of sources) if (relPath.endsWith(".rs")) groupFiles.set(policyHostApplyGroupKey(relPath), [...(groupFiles.get(policyHostApplyGroupKey(relPath)) ?? []), relPath]);
  const groups = new Map<string, ReadonlyMap<string, string>>();
  const groupOf = (relPath: string) => () => {
    const key = policyHostApplyGroupKey(relPath);
    let group = groups.get(key);
    if (!group) groups.set(key, (group = new Map((groupFiles.get(key) ?? []).map((path) => [path, policyReadFileSafe(repoRoot, path)] as const))));
    return group;
  };
  for (const relPath of sources)
    for (const breach of policyDiffOnlyFileBreaches(relPath, policyReadFileSafe(repoRoot, relPath), groupOf(relPath))) {
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

/** 📏️Rule R17: host/editor/apply/preparation lanes and the shell change a snapshot only through `protocol::apply_diff` — no `&mut <X>Snapshot` field writes, `snapshot_mut`, before/after differencing or TypeScript local apply into a snapshot ref (L4/L5 beyond `🧬️schema`/`🧬️mutations`). */
export function policyHostApplyBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R17.slug}`) ?? [];
}

/** 📏️Rule R18: computational mesh gestures are row emitters — no assembled-mesh commit, base-equality sparsifier, kernel topology builder or whole-arena difference in the non-test code of a `🧬️mutations` tree or the mesh overlay. */
export function policyRowEmitterBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R18.slug}`) ?? [];
}

/** 📏️Rule R16: `MutationOutcome::apply_to` and `.apply_to(` are gone from every Rust file (L4). */
export function policyOutcomeApplyToBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R16.slug}`) ?? [];
}
