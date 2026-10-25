import { TaxonomyCancellationError } from "../🏃️operation/🟦️.ts";
import { normalizeRelative } from "../🛣️path/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, createLexicalInputSession, lstatOrNull } from "../📁️input/🟦️.ts";
import { lstatSync, readFileSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { inspectRustAssertionMessageSpans, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences } from "../../🔍️discovery/🧷️rust-path-references/🟦️.ts";
import { inspectRustCargoManifest, inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof, type RustModuleGraph } from "../../🔍️discovery/🌐️rust-module-graph/🟦️.ts";
import { inspectRustNonRepoJoinBaseSpans } from "../../🔍️discovery/🔀️rust-non-repo-join-bases/🟦️.ts";
import { rustTokens as rustSyntaxTokens, rustTokenPairs } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import { basename, dirname, posix } from "node:path";
import { requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { checkCancellation } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { type ReferenceToken, lineLocation, regexTokens } from "../🔤️reference-tokenizers/🟦️.ts";
import { unsupportedReferenceTokens } from "../✂️reference-edits/🟦️.ts";
import { ancestorReferenceCoordinateRoot } from "../🔗️reference-inventory/🟦️.ts";
import { markdownSourceCoordinateSpans } from "../❄️frozen-evidence/🟦️.ts";

/** 🪡️ A `rust-comment-path` backtick span that embeds a real path inside a longer illustrative string
 * (a shell command example, e.g. `` `grep -n "pattern" 🧰️framework/…/component.rs` ``) still names one
 * real, rewritable target — the LAST whitespace-delimited word, which is what the generic
 * `unsupportedReferenceTokens` fallback already extracts and resolves independently, flagging it
 * `reference-syntax-unsupported` because this scanner previously only ever offered the WHOLE span as
 * a token. Extracting the same trailing word here, with its true narrower start/end, lets that
 * fallback candidate's span/value match and be properly rewritten in place instead. */
export function rustTokens(path: string, content: string, index?: ReferencePathIndex): readonly ReferenceToken[] {
  const standalone = rustStandaloneTokens(content);
  return index ? indexedReferenceTokens(path, content, index, standalone) : standalone.tokens;
}

/** 🦀️ The index-free rust reference tokens: the literal path forms, then (spliced at `rustManifestAt` once the path index is known) the
 * manifest-relative joins, then assertion messages, catalog joins, `mod` declarations and comment paths. The literal forms include
 * `owner_file: "<repo-relative path>"`, `bounded_first_step_tool_proofs!`'s self-declared authority field
 * (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`): editor `🦀️.rs` files name their own path there as a plain literal, never
 * through `.join()`, so no join-chain detector sees it; it mirrors the `#[path=…]`/`include!` bare-literal style. */
export function rustStandaloneTokens(content: string): StandaloneReferenceTokens {
  const rows = regexTokens(content, "rust", "rust-string-path", [/#\s*\[\s*path\s*=\s*"([^"]+)"/gu, /\b(?:include|include_str|include_bytes)!\s*\(\s*"([^"]+)"/gu, /\bowner_file\s*:\s*"([^"]+)"/gu]);
  const rustManifestAt = rows.length, inspection = rustManifestInspection(content), rustManifest = inspection.references.length > 0 || inspection.arguments.length > 0 || inspection.candidates.length > 0;
  for (const message of inspectRustAssertionMessageSpans(content)) for (const token of unsupportedReferenceTokens(message.value, "rust")) {
    const start = message.start + token.start;
    rows.push({ adapter: "rust", structuredLocation: lineLocation(content, start, `rust-assertion-message:${message.macroName}`), start, end: message.start + token.end, value: token.value });
  }
  for (const match of content.matchAll(/\.join\(\s*"([^"]*🖼️assets\/🏗️modelDefinitions)"\s*\)/gu)) {
    if (match.index === undefined) continue;
    const start = match.index + match[0].indexOf(requireTaxonomyValue(match[1],"match[1]"));
    rows.push({ adapter: "rust", structuredLocation: lineLocation(content, start, "artifact-catalog-root-join"), start, end: start + requireTaxonomyValue(match[1],"match[1]").length, value: requireTaxonomyValue(match[1],"required reference value"), rewriteKind: "artifact-catalog-prose", rewriteData: { form: "relative-root" } });
  }
  for (const match of content.matchAll(/^([ \t]*)((?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;)/gmu)) {
    if (match.index === undefined) continue;
    const statement = requireTaxonomyValue(match[2],"match[2]");
    const start = match.index + requireTaxonomyValue(match[1],"match[1]").length;
    const name = match[3];
    rows.push({
      adapter: "rust",
      structuredLocation: lineLocation(content, start, "rust-mod"),
      start,
      end: start + statement.length,
      value: statement,
      targetValues: [`./${name}.rs`, `./${name}/mod.rs`],
      rewriteKind: "rust-mod",
      rewriteData: { indentation: requireTaxonomyValue(match[1],"required indentation"), declaration: statement },
    });
  }
  for (const match of content.matchAll(/(?:^|\n)[ \t]*(?:(?:\/\/\/)|(?:\/\/!)|(?:\/\/))([^\r\n]*)/gu)) {
    if (match.index === undefined) continue;
    const fragment = requireTaxonomyValue(match[1],"match[1]");
    const start = match.index + match[0].indexOf(fragment);
    for (const quoted of fragment.matchAll(/`([^`]+)`/gu)) {
      if (quoted.index === undefined) continue;
      const raw = requireTaxonomyValue(quoted[1],"quoted[1]"), rawStart = start + quoted.index + quoted[0].indexOf(raw);
      const segment = /\s/u.test(raw) ? raw.match(/(?:^|\s)([^\s]*[/.][^\s]*)$/u) : null;
      const value = requireTaxonomyValue(segment ? segment[1] : raw,"segment ? segment[1] : raw");
      const tokenStart = segment ? rawStart + raw.lastIndexOf(requireTaxonomyValue(segment[1],"segment[1]")) : rawStart;
      if (!/[/.]/u.test(value)) continue;
      rows.push({ adapter: "rust", structuredLocation: lineLocation(content, tokenStart, "rust-comment-path"), start: tokenStart, end: tokenStart + value.length, value });
    }
    const catalog = fragment.match(/((?:📚️examples\/)?🖼️assets\/🏗️modelDefinitions\/\*\/([^/\s`*]+)\/(\*\.json)?)/u);
    if (catalog) {
      const tokenStart = start + fragment.indexOf(requireTaxonomyValue(catalog[1],"catalog[1]"));
      rows.push({ adapter: "rust", structuredLocation: lineLocation(content, tokenStart, "artifact-catalog-comment"), start: tokenStart, end: tokenStart + requireTaxonomyValue(catalog[1],"catalog[1]").length, value: requireTaxonomyValue(catalog[1],"required reference value"), rewriteKind: "artifact-catalog-prose", rewriteData: { form: "category-glob", category: catalog[2]!, members: catalog[3] ? "json" : "directory" } });
    }
  }
  return { tokens: rows, rustManifestAt, rustManifest, ...(rustManifest ? { rustInspection: inspection } : {}) };
}

/** 🗒️ Resolves real repo-relative paths named as plain single-backtick inline code or a bare
 * path-only list item, outside every opaque Markdown block (fenced/indented code, blockquotes,
 * HTML) — reusing {@link markdownSourceCoordinateSpans}'s already-proven span discipline (built for
 * `frozenMarkdownCoordinateEvidenceCoordinates`) so a design-rationale doc gets the same live-
 * reference treatment `rustTokens`/`typescriptCommentPathReferenceAuthority` already give source
 * comments. A span registered as frozen evidence stays protected regardless — the shared
 * `frozenEvidence`/`isFrozenSourceCoordinateToken` checks in the caller run over every adapter's
 * tokens before any rewrite, and a value that never resolves to a real repo path is simply ignored. */
export function markdownCommentPathReferenceAuthority(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const span of markdownSourceCoordinateSpans(content)) {
    const value = content.slice(span.start, span.end);
    if (!value || /[\s[\](){}<>"'`\\]/u.test(value) || !/[/.]/u.test(value)) continue;
    rows.push({ adapter: "markdown", structuredLocation: lineLocation(content, span.start, `markdown-${span.form}`), start: span.start, end: span.end, value });
  }
  return rows;
}

/** 🧩️ One file's reference tokens that need no path index, where rust manifest-relative tokens splice in, and whether any can exist. */
export interface StandaloneReferenceTokens { readonly tokens: readonly ReferenceToken[]; readonly rustManifestAt: number; readonly rustManifest: boolean; readonly rustInspection?: RustManifestInspection }

/** 🔗️ {@link referenceTokens} with a path index, assembled from its standalone part plus the index-dependent rust manifest tokens. */
export function indexedReferenceTokens(path: string, content: string, index: ReferencePathIndex, standalone: StandaloneReferenceTokens): readonly ReferenceToken[] {
  return standalone.rustManifest ? [...standalone.tokens.slice(0, standalone.rustManifestAt), ...rustManifestReferenceTokens(path, content, index, standalone.rustInspection), ...standalone.tokens.slice(standalone.rustManifestAt)] : standalone.tokens;
}

export function splitTokenSuffix(value: string): { readonly path: string; readonly suffix: string } {
  const index = value.search(/[?#]/);
  return index < 0 ? { path: value, suffix: "" } : { path: value.slice(0, index), suffix: value.slice(index) };
}

export interface ReferencePathIndex {
  readonly exact: ReadonlySet<string>;
  readonly nfc: ReadonlyMap<string, string | null>;
  readonly extensionless: ReadonlyMap<string, string | null>;
  readonly pythonModule: ReadonlyMap<string, string | null>;
  readonly repoRoot?: string;
  readonly coordinateRoots: readonly string[];
  readonly coordinateRootSet: ReadonlySet<string>;
  readonly coordinateRootByReference: Map<string, string | undefined>;
  readonly contextPaths: readonly string[];
  readonly contextPathSet: ReadonlySet<string>;
  readonly affectedPaths: ReadonlySet<string>;
  readonly cancelFile?: string;
}

interface RustReferenceGraphView {
  readonly graph: RustModuleGraph;
  readonly hashes: ReadonlyMap<string, string>;
  readonly unreadableInputs: ReadonlyMap<string, Error>;
}

const rustReferenceGraphs = new WeakMap<ReferencePathIndex, Map<string, RustReferenceGraphView>>();

const rustUnprovenReferenceTargets = new WeakMap<ReferencePathIndex, Map<string, readonly string[]>>();

const rustReferenceContextFiles = new WeakMap<ReferencePathIndex, Map<string, readonly string[]>>();

function rustContextFiles(path: string, index: ReferencePathIndex): readonly string[] {
  const coordinateRoot = ancestorReferenceCoordinateRoot(path, index.coordinateRootSet) ?? "";
  const views = rustReferenceContextFiles.get(index) ?? new Map<string, readonly string[]>();
  rustReferenceContextFiles.set(index, views);
  const cached = views.get(coordinateRoot);
  if (cached) return cached;
  const files = index.contextPaths.filter((candidate) => (candidate.endsWith(".rs") || basename(candidate) === "Cargo.toml") && (ancestorReferenceCoordinateRoot(candidate, index.coordinateRootSet) ?? "") === coordinateRoot);
  views.set(coordinateRoot, files);
  return files;
}

const exactPathsByLeafs = new WeakMap<ReferencePathIndex, ReadonlyMap<string, readonly string[]>>();

/** 🌿️ The index's exact paths grouped by final segment in iteration order: a path equal to or ending in `/<suffix>` shares the suffix's final segment, so a suffix query reads one group instead of every path. */
function exactPathsByLeaf(index: ReferencePathIndex): ReadonlyMap<string, readonly string[]> {
  let groups = exactPathsByLeafs.get(index);
  if (!groups) {
    const byLeaf = new Map<string, string[]>();
    for (const path of index.exact) {
      const leaf = path.slice(path.lastIndexOf("/") + 1), rows = byLeaf.get(leaf);
      if (rows) rows.push(path);
      else byLeaf.set(leaf, [path]);
    }
    exactPathsByLeafs.set(index, groups = byLeaf);
  }
  return groups;
}

function unprovenRustReferenceTargets(referencePath: string, value: string, index: ReferencePathIndex): readonly string[] {
  const cache = rustUnprovenReferenceTargets.get(index) ?? new Map<string, readonly string[]>();
  rustUnprovenReferenceTargets.set(index, cache);
  const coordinateRoot = ancestorReferenceCoordinateRoot(referencePath, index.coordinateRootSet), key = `${coordinateRoot ?? ""}\0${value}`;
  const cached = cache.get(key);
  if (cached) return cached;
  const decoded = unsupportedReferenceTokens(`"${value}"`, "rust")[0]?.targetValues?.[0] ?? value, suffix = decoded.replace(/^(?:\.\.?\/)+/u, "");
  const explicitEscape = /^(?:\.\.\/|\/|[A-Za-z]:[\\/])/u.test(decoded);
  const leaf = suffix.slice(suffix.lastIndexOf("/") + 1);
  const targets = (leaf ? exactPathsByLeaf(index).get(leaf) ?? [] : [...index.exact]).filter((candidate) => (!coordinateRoot || candidate.startsWith(`${coordinateRoot}/`) || explicitEscape) && suffix !== "" && (candidate === suffix || candidate.endsWith(`/${suffix}`)));
  cache.set(key, targets);
  return targets;
}

function rustReferenceNeedsOwnership(path: string, references: ReturnType<typeof inspectRustManifestPathReferences>, index: ReferencePathIndex, candidates: ReturnType<typeof inspectRustManifestPathCandidates> = []): boolean {
  if (index.affectedPaths.has(path)) return true;
  const affected = (candidate: string): boolean => index.affectedPaths.has(candidate) || index.affectedPaths.has(index.nfc.get(candidate.normalize("NFC")) ?? "");
  const manifests = rustContextFiles(path, index).filter((candidate) => basename(candidate) === "Cargo.toml");
  for (const reference of references) {
    if (unprovenRustReferenceTargets(path, reference.value, index).some(affected)) return true;
    for (const manifest of manifests) {
      try { if (affected(normalizeRelative(posix.join(posix.dirname(manifest), ...reference.base, reference.value)))) return true; } catch {}
    }
  }
  for (const candidate of candidates) {
    if (unprovenRustReferenceTargets(path, candidate.value, index).some(affected)) return true;
    for (const manifest of manifests) for (const parts of candidate.targets) {
      try { if (affected(normalizeRelative(posix.join(posix.dirname(manifest), ...parts)))) return true; } catch {}
    }
  }
  return false;
}

function rustReferenceGraph(path: string, index: ReferencePathIndex): RustReferenceGraphView | null {
  if (!index.repoRoot) return null;
  const coordinateRoot = ancestorReferenceCoordinateRoot(path, index.coordinateRootSet) ?? "";
  const views = rustReferenceGraphs.get(index) ?? new Map<string, RustReferenceGraphView>();
  rustReferenceGraphs.set(index, views);
  const cached = views.get(coordinateRoot);
  if (cached) return cached;
  const files = rustContextFiles(path, index);
  const contents = new Map<string, string | undefined>(), hashes = new Map<string, string>(), unreadableInputs = new Map<string, Error>(), admit = createLexicalInputSession();
  const read = (candidate: string): string | undefined => {
    checkCancellation(index.repoRoot!, index.cancelFile);
    if (contents.has(candidate)) return contents.get(candidate);
    let absolute: string;
    try { absolute = admit(index.repoRoot!, candidate, "Rust module ownership", true); }
    catch (error) { unreadableInputs.set(candidate, error instanceof Error ? error : new Error(String(error))); contents.set(candidate, undefined); return undefined; }
    const stat = lstatOrNull(absolute);
    if (!stat) { contents.set(candidate, undefined); return undefined; }
    if (!stat.isFile()) { unreadableInputs.set(candidate, new Error(`Rust module ownership is not a regular file: ${candidate}`)); contents.set(candidate, undefined); return undefined; }
    const bytes = readFileSync(absolute), after = lstatSync(absolute);
    if (after.mode !== stat.mode || after.size !== stat.size || after.mtimeMs !== stat.mtimeMs || bytes.byteLength !== stat.size) throw new Error(`Rust module ownership changed during its snapshot: ${candidate}`);
    hashes.set(candidate, sha256(bytes));
    contents.set(candidate, bytes.toString("utf8"));
    return contents.get(candidate);
  };
  const graph = inspectRustModuleGraph(files, read, { strictManifests: true, checkCancellation: () => checkCancellation(index.repoRoot!, index.cancelFile) });
  const view = { graph, hashes, unreadableInputs };
  views.set(coordinateRoot, view);
  return view;
}

/** 🧮️ Admits a complete finite interpretation only through one unchanged physical Cargo source chain. An ancestor's glob import (`use x::*`) is never disqualifying here — file participation comes only from `mod`/`#[path]` declarations, which `inspectRustModuleGraphFacts` already tracks completely regardless of glob re-exports; a glob only affects NAME resolution, never which physical files exist in the graph. A non-literal `.join(...)` argument is separately, structurally unrepresentable by every extractor this function consumes (`inspectRustManifestPathReferences`/`inspectRustManifestPathCandidates`/`inspectRustJoinArgumentSpans` all require a string literal, or a loop bound to string literals, to record anything at all) — so it can never reach this proof as a false `finite` positive regardless of glob imports. */
/** 🛡️ Exact, fully-qualified invocation heads of framework-owned macros verified (by reading their `macro_rules!` bodies in full) to expand to zero `mod` items, so calling them can never hide additional module structure from the static crate-graph prover below. Every plugin's crate-root `glue.rs` calls exactly the first of these once; matched only against the comment-free, single-space-joined token stream `rustCodeOnlyTextForMacroTrust` builds, never raw text, so a decoy in a string literal or comment cannot match. Adding an entry here is a safety-relevant claim about a macro's expansion, not a formatting nicety — verify the full `macro_rules!` body contains no `mod` token before adding one. `derive_artifact_facets!` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, both its primary arm and its `@children_ty` dispatch arms) expands only to `struct`/`impl` items built from `$crate`-qualified type paths and method delegations — zero `mod` tokens anywhere in its body — and is invoked once per artifact subset's `🧬️schema/🦀️component.rs` across the plugin tree, so leaving it untrusted was blocking the crate-graph proof for every ancestor that calls it, not just one. */
const RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS: readonly string[] = [
  "semio_framework_plugin :: plugin_exports !",
  "semio_framework_plugin :: derive_artifact_facets !",
];

/** 📦️ Bare names of crate-LOCAL `macro_rules!` macros — declared directly inside an ancestor file this guard scans, not merely called from one — verified (by reading the full `macro_rules! name { … }` body) to expand to zero `mod` items. `stdio`'s crate-root `🦀️.rs` declares `impl_serde_op_codec!` this way: its body is two trait impls (`protocol::OpText`/`protocol::OpBinary`) built entirely from method calls (`serde_json::to_string`/`to_vec`/`from_str`/`from_slice`, `.expect`/`.map_err`) — zero `mod` tokens anywhere in the expansion template. Unlike `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS` (an external macro's exact call-site text), a name registered here needs BOTH its definition head (`macro_rules ! name`) and every local invocation (`name !`) scrubbed, since both live in the same ancestor file being trusted. */
const RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS: readonly string[] = [
  "impl_serde_op_codec",
];

/** 📚️ Bare names of Rust standard-library macros that the language itself guarantees are expression/statement-position only — none can ever expand to, or contain, an item-position `mod`, so their mere presence can never hide module structure. This is a closed, deliberately narrow allowlist (not "trust all of std"): only macros actually encountered in a repository ancestor file get registered, one at a time, exactly like every other entry on this page. `stdio`'s `🦀️.rs` uses `format!` (`semantic_fingerprint`'s error path) and `unreachable!` (`hash_hex_bytes`'s exhaustiveness arm) — both ordinary value-producing expressions, no different in kind from a plain function call as far as module structure is concerned. The `assert*!` family is the same kind of claim: `assert!`/`assert_eq!`/`assert_ne!`/`debug_assert!`/`debug_assert_eq!`/`debug_assert_ne!` expand only to a boolean check plus a `panic!` arm (itself already covered by this same guarantee) — never an item — and appear in essentially every `#[cfg(test)] mod tests` block repo-wide, so leaving them untrusted was disqualifying nearly any ancestor file with tests, independent of whichever other construct also happened to be in it. `vec!` is the same guarantee again: it expands only to a sequence of `Vec::new()`/`.push(...)` calls (or `<[_]>::into_vec(box [...])`), always in expression position — `stdio.pdf`'s `✳️a` schema ancestor (`🧬️schema/🦀️component.rs`) builds a `PdfSnapshot { pages: vec![...] }` test fixture with it, which was disqualifying the pdf `✳️a`/`✳️x` mutation files' `CARGO_MANIFEST_DIR` proof (confirmed by direct instrumentation of `rustFiniteManifestTargets`, not by re-reading the regex). */
const RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS: readonly string[] = [
  "format",
  "unreachable",
  "panic",
  "assert",
  "assert_eq",
  "assert_ne",
  "debug_assert",
  "debug_assert_eq",
  "debug_assert_ne",
  "vec",
];

/** 🧼️ Comment-free, single-space-joined token text, further scrubbed of every call to a macro registered in `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS`, every crate-local macro registered in `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS` (both its `macro_rules!` head and its local invocations), every std expression macro registered in `RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS`, every bare attribute named in `RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_NAMES`, and every argument-free attribute path registered in `RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS` — used only to decide whether an ancestor's attribute/macro usage should make the module-graph prover distrust it, never to decide what the prover treats as physically present. `cfg`/`cfg_attr` are safe to scrub for a structural reason: `inspectRustModuleGraphFacts` already records a `mod` under either attribute as an ordinary module fact (flagged `conditional: true`, never dropped), so the actual proof below already treats a cfg-gated `mod` — and the real, on-disk `#[path]` file it names — as a complete participant; cfg governs whether the module COMPILES, not whether the path reference exists in source text. `allow`/`derive` are safe for a different, simpler reason: the Rust language guarantees both are inert with respect to compilation and module structure for any argument, so their mere presence can never hide a `mod`. Attribute PATHS (registered by their exact fully-qualified, argument-free spelling, e.g. a `#[proc_macro_attribute]` invoked as `#[crate::name]`) are a third, per-macro claim exactly like `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS` — verify the proc-macro's own body emits zero `mod` tokens and cannot structurally emit one before adding an entry. Excluding any of these from the *trust* scan does not exclude what they attach to from the *proof*. */
const RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_NAMES: ReadonlySet<string> = new Set(["cfg", "cfg_attr", "allow", "derive"]);

/** 📂️ Exact, argument-free, fully-qualified attribute-macro paths verified (by reading the proc-macro's own implementation in full) to expand to zero `mod` items and to structurally be unable to emit one — e.g. `semio_framework_async_macros::async_test` (`🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️.rs`, `expand_async_test`) parses its input as a `syn::ItemFn`, rejects anything that isn't a bare `async fn`, and re-emits only `#[test]`/`fn`/`struct`/`impl` items — no code path produces a `mod`. Matched only against the comment-free, single-space-joined token stream between one attribute's brackets, so a decoy in a string literal or comment cannot match. */
const RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS: ReadonlySet<string> = new Set([
  "semio_framework_async_macros :: async_test",
]);

/** 🔑️ Reserved Rust keywords (2021 edition, plus the unused-but-reserved future set) — never valid
 * macro names, so an identifier-KIND token spelling one of these can precede a bare `!` only as an
 * expression-starting keyword (`if !x`, `while !x`, `return !x`, `match !x { … }`, …), never as a
 * `name!` macro invocation. Distinguishes `rustCodeOnlyTextForMacroTrust`'s bare-`!` scrub from a
 * false positive on `if !(…)`/`while !(…)` — the tokenizer gives keywords `kind: "identifier"` too,
 * same as any real identifier, so "preceded by an identifier" alone is not a precise enough test. */
const RUST_RESERVED_KEYWORDS: ReadonlySet<string> = new Set([
  "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "try", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
]);

/** 🧹️ The code-only token text the macro-trust checks scan: the module-structure-transparent attributes and macros are dropped and every other token is kept.
 *
 * A bare `!` immediately after a real identifier is a macro-invocation bang (`name!`/`path::name!`), the only Rust construct a lone `!` token can start that is capable of expanding to an item, and stays in the text for the macro-trust checks. Every other bare `!` is prefix boolean negation (`!expr`), which the grammar restricts to expression position: it can never expand into, or hide, a `mod`. A RESERVED KEYWORD is never a valid macro name, so `if !(…)`/`while !(…)`/`return !x` are negation too, even though the tokenizer gives keywords `kind: "identifier"` the same as real identifiers; `RUST_RESERVED_KEYWORDS` disambiguates. Confirmed by direct instrumentation of `rustFiniteManifestTargets`: the `if !(page.width > 0.0 && …)` of the `stdio.pdf` `✳️x` schema ancestor was tripping the downstream `/[#!]/` scan and disqualifying an otherwise-trusted ancestor.
 *
 * `!=` is one punctuation token (see the `punctuation` list of `rustTokens`), never a macro bang, yet it literally CONTAINS a `!` character, which the downstream `/[#!]/` scan matches as a textual substring regardless of token boundaries. It is the same false-positive family as the bare-`!` negation (the `d.code.0 != CODE_TEXT_EMPTY` of the `stdio.pdf` `✳️a` schema ancestor was tripping it after the bare-`!` fix) and is neutralized the same way: comparison operators are expression-position only and can never expand into, or hide, a `mod`. */
function rustCodeOnlyTextForMacroTrust(source: string): string {
  const tokens = rustSyntaxTokens(source), pairs = rustTokenPairs(tokens), kept: string[] = [];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]!.text === "#") {
      let bracket = index + 1;
      if (tokens[bracket]?.text === "!") bracket += 1;
      if (tokens[bracket]?.text === "[") {
        const close = pairs.get(bracket), name = tokens[bracket + 1]?.text;
        if (close !== undefined && name !== undefined && RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_NAMES.has(name) && tokens[bracket + 2]?.text === "(") { index = close; continue; }
        if (close !== undefined && RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS.has(tokens.slice(bracket + 1, close).map((token) => token.text).join(" "))) { index = close; continue; }
      }
    }
    if (tokens[index]!.text === "!" && (tokens[index - 1]?.kind !== "identifier" || RUST_RESERVED_KEYWORDS.has(tokens[index - 1]!.text))) continue;
    if (tokens[index]!.text === "!=") { kept.push("<>"); continue; }
    kept.push(tokens[index]!.text);
  }
  let text = kept.join(" ");
  for (const invocation of RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS) text = text.split(invocation).join(" ");
  for (const name of RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS) text = text.split(`macro_rules ! ${name}`).join(" ").split(`${name} !`).join(" ");
  for (const name of RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS) text = text.split(`${name} !`).join(" ");
  return text;
}

function rustFiniteManifestTargets(path: string, content: string, candidates: ReturnType<typeof inspectRustManifestPathCandidates>, index: ReferencePathIndex, view: RustReferenceGraphView | null): ReadonlyMap<number, readonly string[]> {
  const result = new Map<number, readonly string[]>(), contexts = view?.graph.contexts.get(path) ?? [];
  if (!index.repoRoot || !view || !contexts.length || contexts.some((context) => context.manifestPath === null)) return result;
  const manifests = [...new Set(contexts.map((context) => context.manifestPath!))];
  if (manifests.length !== 1 || view.hashes.get(path) !== sha256(content)) return result;
  const coordinateRoot = ancestorReferenceCoordinateRoot(path, index.coordinateRootSet) ?? "";
  const sameRoot = (target: string): boolean => (index.coordinateRootSet.has(target) ? target : ancestorReferenceCoordinateRoot(target, index.coordinateRootSet) ?? "") === coordinateRoot;
  const proofPaths = [...new Set(contexts.flatMap((context) => [context.manifestPath!, ...context.sourceChain]))];
  if (!proofPaths.includes(path)) return result;
  const physicalPath = (base: string, parts: readonly string[], requireDirectory = false): string => {
    if (parts.length === 0 || parts.some((part) => posix.isAbsolute(part) || /^[A-Za-z]:/u.test(part) || part.includes("\\") || part.includes("\u0000"))) throw new Error("Rust finite path has no local physical identity");
    let current = normalizeRelative(base);
    if (!sameRoot(current) || !lstatOrNull(assertLexicalInputOutsideOpaque(index.repoRoot!, current, "Rust finite physical base", true))?.isDirectory()) throw new Error("Rust finite path has no coordinate-local physical base");
    const segments = parts.join("/").split("/");
    for (let step = 0; step < segments.length; step++) {
      checkCancellation(index.repoRoot!, index.cancelFile);
      current = normalizeRelative(posix.join(current, segments[step]!));
      if (!sameRoot(current)) throw new Error("Rust finite path step escapes its coordinate root");
      const absolute = assertLexicalInputOutsideOpaque(index.repoRoot!, current, "Rust finite path step", true), stat = lstatOrNull(absolute);
      const directory = requireDirectory || step + 1 < segments.length || ["", ".", ".."].includes(segments[step]!);
      if (!stat || (directory ? !stat.isDirectory() : !stat.isFile() && !stat.isDirectory())) throw new Error("Rust finite path step is not a physical file or directory");
    }
    return current;
  };
  const contents = new Map<string, string>();
  try {
    for (const source of proofPaths) {
      checkCancellation(index.repoRoot, index.cancelFile);
      if (!index.contextPathSet.has(source) || !sameRoot(source) || !view.hashes.has(source)) return result;
      const absolute = assertLexicalInputOutsideOpaque(index.repoRoot, source, "Rust finite source authority", true), before = lstatOrNull(absolute);
      if (!before?.isFile()) return result;
      const bytes = readFileSync(absolute), after = lstatSync(absolute);
      if (after.mode !== before.mode || after.size !== before.size || after.mtimeMs !== before.mtimeMs || bytes.byteLength !== before.size || sha256(bytes) !== view.hashes.get(source)) return result;
      contents.set(source, bytes.toString("utf8"));
    }
    const manifest = inspectRustCargoManifest(contents.get(manifests[0]!)!, true);
    if (!manifest.valid || manifest.dependencies.includes("std")) return result;
    const facts = new Map(proofPaths.filter((source) => source.endsWith(".rs")).map((source) => [source, inspectRustModuleGraphFacts(contents.get(source)!)]));
    const parentImports = facts.get(path)?.uses.some((use) => /^(?:super::)+\*$/u.test(use.specifier)) ?? false;
    for (const source of proofPaths.filter((source) => source.endsWith(".rs") && source !== path)) {
      const text = rustCodeOnlyTextForMacroTrust(contents.get(source)!);
      const withoutPathAttributes = text.replace(/#\s*\[\s*path\s*=\s*"[^"\\]*"\s*\]/gu, "");
      if (/[#!]/u.test(withoutPathAttributes) || /\bmacro\b/u.test(text) || parentImports && /\b(?:std|env)\b/u.test(text)) return result;
    }
    for (const context of contexts) {
      if (physicalPath(posix.dirname(manifests[0]!), [manifest.libPath ?? "src/lib.rs"]) !== context.crateRoot) return result;
      for (let chain = 0; chain + 1 < context.sourceChain.length; chain++) {
        const source = context.sourceChain[chain]!, next = context.sourceChain[chain + 1]!;
        const owners = (view.graph.contexts.get(source) ?? []).filter((owner) => owner.manifestPath === context.manifestPath && owner.crateRoot === context.crateRoot && owner.sourceChain.length === chain + 1 && owner.sourceChain.every((item, index) => item === context.sourceChain[index]) && owner.modulePath.every((item, index) => item === context.modulePath[index]));
        let proven = 0;
        for (const owner of owners) {
          const sourceFacts = facts.get(source);
          if (!sourceFacts || rustModuleScopeProof(sourceFacts, owner.sourceScope).state === "unresolved") return result;
          for (const module of sourceFacts.modules) {
          if (module.modulePath.length !== owner.sourceScope.length + 1 || !owner.sourceScope.every((item, index) => item === module.modulePath[index])) continue;
          const modulePath = [...owner.modulePath, module.name];
          if (!modulePath.every((item, index) => item === context.modulePath[index])) continue;
          if (module.unresolved) return result;
          if (module.inline) { if (module.pathTarget !== null) physicalPath(owner.moduleBase, [module.pathTarget], true); continue; }
          if (view.graph.targets.get(`${context.crateRoot}\0${modulePath.join("::")}`) !== next) continue;
          const base = module.pathTarget !== null && owner.sourceScope.length === 0 ? posix.dirname(source) : owner.moduleBase;
          const raw = module.pathTarget ?? (next === posix.join(base, `${module.name}.rs`) ? `${module.name}.rs` : `${module.name}/mod.rs`);
          if (physicalPath(base, [raw]) !== next) return result;
          proven++;
        }
        }
        if (proven !== 1) return result;
      }
    }
  } catch (error) {
    if (error instanceof TaxonomyCancellationError) throw error;
    return result;
  }
  for (const candidate of candidates) {
    if (!Number.isInteger(candidate.start) || !Number.isInteger(candidate.end) || candidate.start < 0 || candidate.end <= candidate.start || content.slice(candidate.start, candidate.end) !== candidate.value || candidate.targets.length === 0 || candidate.targets.length > 256) continue;
    try {
      const targets = new Set<string>();
      for (const parts of candidate.targets) {
        checkCancellation(index.repoRoot, index.cancelFile);
        const target = normalizeRelative(posix.join(posix.dirname(manifests[0]!), ...parts));
        if (!index.contextPathSet.has(target) || !sameRoot(target)) throw new Error("Rust finite target lacks coordinate-local admission");
        if (physicalPath(posix.dirname(manifests[0]!), parts) !== target) throw new Error("Rust finite target identity changed");
        targets.add(target);
      }
      result.set(candidate.start, [...targets].sort(generatorPathCompare));
    } catch (error) {
      if (error instanceof TaxonomyCancellationError) throw error;
    }
  }
  return result;
}

/** 🔬️ The content-pure syntax facts rust manifest-relative tokens are built from: join references, join argument spans, finite loop
 * candidates, and (only when there are argument spans) the spans whose base is proven not to be the repository. */
export interface RustManifestInspection {
  readonly references: readonly ReturnType<typeof inspectRustManifestPathReferences>[number][];
  readonly arguments: readonly ReturnType<typeof inspectRustJoinArgumentSpans>[number][];
  readonly candidates: readonly ReturnType<typeof inspectRustManifestPathCandidates>[number][];
  readonly nonRepoBases: readonly number[];
}

/** 📋️ Computes one rust source's {@link RustManifestInspection}. */
function rustManifestInspection(content: string): RustManifestInspection {
  const references = inspectRustManifestPathReferences(content), arguments_ = inspectRustJoinArgumentSpans(content), candidates = inspectRustManifestPathCandidates(content);
  return { references, arguments: arguments_, candidates, nonRepoBases: arguments_.length > 0 ? [...inspectRustNonRepoJoinBaseSpans(content)] : [] };
}

/** 🧷️ Builds the manifest-relative reference tokens of one Rust source.
 *
 * Two structurally distinct failures are kept apart in the unsupported reason: `targets` is `undefined` when a reference never reached the per-candidate map of `rustFiniteManifestTargets` at all (an early whole-file guard bailed, or its own try/catch threw for this candidate specifically), so no proof was ever attempted; `targets.length !== 1` with `targets` DEFINED means the proof ran to completion and found zero or several distinct physical targets, a genuine ambiguity and not a missing proof. Three earlier diagnoses mistook the first for the second (see the `rust-path-join` ticket history).
 *
 * `env::temp_dir()`, `env::var[_os]()`, `env::args()` or a bare `fn` parameter with no local `CARGO_MANIFEST_DIR` anchor PROVES the join's root can never name a repository file at plan time: that is a proven non-reference, so it is never recorded at all (not merely marked resolved). An unrecognized base is untouched and still falls through to the `rust-path-join-unproven` rows unchanged. */
export function rustManifestReferenceTokens(path: string, content: string, index: ReferencePathIndex, inspection: RustManifestInspection = rustManifestInspection(content)): readonly ReferenceToken[] {
  const { references, arguments: arguments_, candidates } = inspection;
  if (references.length === 0 && arguments_.length === 0 && candidates.length === 0) return [];
  const view = !rustReferenceNeedsOwnership(path, references, index, candidates) ? null : rustReferenceGraph(path, index), contexts = view?.graph.contexts.get(path) ?? [];
  for (const [candidate, error] of view?.unreadableInputs ?? []) if (candidate === path || basename(candidate) === "Cargo.toml" && (posix.dirname(candidate) === "." || path.startsWith(`${posix.dirname(candidate)}/`))) throw error;
  if (view?.hashes.has(path) && view.hashes.get(path) !== sha256(content)) throw new Error(`Rust reference source changed during ownership resolution: ${path}`);
  const manifests = [...new Set(contexts.map((context) => context.manifestPath).filter((manifest): manifest is string => manifest !== null))];
  const proofPaths = [...new Set(contexts.flatMap((context) => [context.manifestPath!, ...context.sourceChain]))].sort(generatorPathCompare);
  const digest = sha256(canonicalJson(proofPaths.map((source) => ({ path: source, sha256: view?.hashes.get(source) }))));
  const conflicts = (left: Pick<ReferenceToken, "start" | "end">, right: Pick<ReferenceToken, "start" | "end">): boolean => left.start === right.start || left.start < right.end && right.start < left.end;
  const writableInputs = new Set(references.filter((reference, index) => !references.some((other, otherIndex) => index !== otherIndex && conflicts(reference, other))));
  const finiteInputs = new Set(candidates.filter((candidate, index) => !references.some((reference) => conflicts(candidate, reference)) && !candidates.some((other, otherIndex) => index !== otherIndex && conflicts(candidate, other))));
  const finite = rustFiniteManifestTargets(path, content, [...Array.from(writableInputs, (reference) => ({ start: reference.start, end: reference.end, value: reference.value, targets: [[...reference.base, reference.value]] })), ...finiteInputs], index, view);
  const rows: ReferenceToken[] = references.map((reference) => {
    let sourceBase: string | undefined, physicalTargets: string[] = [], unsupportedReason: string | undefined;
    const targets = writableInputs.has(reference) ? finite.get(reference.start) : undefined;
    try {
      if (manifests.length !== 1) unsupportedReason = `Rust manifest-relative path requires one proven Cargo owner, found ${manifests.length}`;
      else if (targets === undefined) unsupportedReason = "Rust manifest-relative path was never admitted into a proven physical source chain";
      else if (targets.length !== 1) unsupportedReason = `Rust manifest-relative path resolved to ${targets.length} distinct physical targets, not exactly one`;
      else { physicalTargets = [...targets]; sourceBase = normalizeRelative(posix.join(dirname(manifests[0]!), ...reference.base)); }
    } catch (error) { unsupportedReason = error instanceof Error ? error.message : String(error); }
    if (unsupportedReason) physicalTargets = [...unprovenRustReferenceTargets(path, reference.value, index)];
    return { adapter: "rust" as const, structuredLocation: lineLocation(content, reference.start, `rust-path-join:${digest}`), start: reference.start, end: reference.end, value: reference.value, physicalTargets, ...(unsupportedReason ? { unsupportedReason } : { rewriteKind: "rust-path-join" as const, rewriteData: { sourceBase: sourceBase! } }) };
  });
  for (const candidate of candidates) if (!rows.some((row) => row.start === candidate.start && row.end === candidate.end)) {
    const physicalTargets = finiteInputs.has(candidate) ? finite.get(candidate.start) : undefined;
    rows.push({ adapter: "rust", structuredLocation: lineLocation(content, candidate.start, physicalTargets ? `rust-finite-manifest-targets:${digest}` : "rust-path-join-unproven"), start: candidate.start, end: candidate.end, value: candidate.value, physicalTargets: physicalTargets ?? unprovenRustReferenceTargets(path, candidate.value, index), ...(physicalTargets ? { physicalInterpretation: "rust-finite-manifest-targets" as const } : {}), unsupportedReason: "Rust finite candidate has no writable literal authority" });
  }
  const nonRepoBases = new Set(inspection.nonRepoBases);
  for (const argument of arguments_) if (!nonRepoBases.has(argument.start) && !rows.some((row) => row.start === argument.start && row.end === argument.end)) rows.push({ adapter: "rust", structuredLocation: lineLocation(content, argument.start, "rust-path-join-unproven"), ...argument, physicalTargets: unprovenRustReferenceTargets(path, argument.value, index), unsupportedReason: "Rust join argument has no proven immutable manifest-relative base" });
  return rows;
}

export function rustReferenceInterpretationCovers(token: ReferenceToken, candidate: ReferenceToken): boolean {
  if (token.adapter !== "rust" || candidate.adapter !== "rust") return false;
  if (token.rewriteKind === "rust-path-join") return token.start <= candidate.start && token.end >= candidate.end;
  return token.physicalInterpretation === "rust-finite-manifest-targets" && token.rewriteKind === undefined && Boolean(token.unsupportedReason) && (token.physicalTargets?.length ?? 0) > 0 && token.start === candidate.start && token.end === candidate.end && token.value === candidate.value;
}
