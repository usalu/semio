/** 🧭️ Wave 4 plugin consistency policy: DSL, pack, command envelope, diff, grammar file, protocol file and protocol migration completeness.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { POLICY_SKIP_DIRS, policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { missingArtifactDiffWireTypes } from "../../../🚪️io/🏛️architecture/🟦️.ts";
import { policyLineOfIndex } from "../../../🔍️discovery/📍️source-coordinate/🟦️.ts";
import { POLICY_TS_COMPONENT_LEAF } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { readdirSync, type Dirent } from "node:fs";
import { join } from "node:path";
import { POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST, POLICY_DIFF_COMPLETENESS_ALLOWLIST, POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST, POLICY_GRAMMAR_FILE_ALLOWLIST, POLICY_PACK_COMPLETENESS_ALLOWLIST, POLICY_PROTOCOL_FILE_ALLOWLIST, POLICY_TS_FACADE_CONSTITUTIONAL_FACETS } from "../📋️allowlists/🟦️.ts";
import { policyAllRustFiles, policyExtractFnBody, policyNormalizeRelPath } from "../🏛️foundation/🟦️.ts";

//#region 🔧️PolicyRuleDslCompleteness

type PolicyDocumentAppUsage = { scope: string; line: number; appType: string; snapshotType: string; mutationType: string };

const POLICY_DOCUMENT_APP_IMPL_RE = /impl\s+DocumentApp\s+for\s+(\w+)\s*\{/g;

/** 🔎️Extracts `type Snapshot = X;` / `type Mutation = Y;` (generic args stripped) from every `impl DocumentApp for … { … }` block in one file's content. */
export function policyDocumentAppUsages(scope: string, content: string): PolicyDocumentAppUsage[] {
  const usages: PolicyDocumentAppUsage[] = [];
  let m: RegExpExecArray | null;
  POLICY_DOCUMENT_APP_IMPL_RE.lastIndex = 0;
  while ((m = POLICY_DOCUMENT_APP_IMPL_RE.exec(content))) {
    const body = policyExtractFnBody(content, m.index);
    const snapshotMatch = body.match(/type\s+Snapshot\s*=\s*([\w:<>]+)\s*;/);
    const mutationMatch = body.match(/type\s+Mutation\s*=\s*([\w:<>]+)\s*;/);
    if (!snapshotMatch || !mutationMatch) continue;
    usages.push({
      scope,
      line: policyLineOfIndex(content, m.index),
      appType: m[1]!,
      snapshotType: snapshotMatch[1]!.split("::").pop()!.replace(/<.*$/, ""),
      mutationType: mutationMatch[1]!.split("::").pop()!.replace(/<.*$/, ""),
    });
  }
  return usages;
}

const POLICY_STRUCT_OR_ENUM_DECL_RE = /^\s*(?:pub\s+)?(?:struct|enum)\s+(\w+)\b/gm;
const POLICY_DSL_DERIVE_RE = /derive\([^)]*\bDsl(?:Document|Ops|Record|Enum|Scalar)\b[^)]*\)/;
// 🎞️ CW9: `OpText` moved from `vcs` to `protocol` (see protocol/command/rs) — CW7's import-path sweep
// rewrote every hand-rolled `impl vcs::OpText for X` to `impl protocol::OpText for X`, so this must
// recognize both prefixes (`DocumentDsl` stays vcs-owned, but allowing `protocol::` there too is
// harmless since no such impl exists).
const POLICY_HAND_ROLLED_IMPL_RE = /impl(?:<[^>]*>)?\s+(?:vcs::|protocol::)?(DocumentDsl|OpText)\s+for\s+(?:vcs::|protocol::)?(\w+)/g;

/** 🧬️One O(total content) pass building every type name that's DSL-complete — via `#[derive(dsl::Dsl…)]` a few lines above its own `struct`/`enum` declaration, or a hand-rolled `impl (vcs::|protocol::)?DocumentDsl`/`impl (vcs::|protocol::)?OpText` (an explicit generic impl also counts, e.g. `impl<D> OpText for SetDocumentMutation<D>`) — split by trait since a type may satisfy one but not the other. */
function policyDslCompleteTypeNames(files: readonly { content: string }[]): { documentDsl: Set<string>; opText: Set<string> } {
  const documentDsl = new Set<string>();
  const opText = new Set<string>();
  for (const { content } of files) {
    const lines = content.split("\n");
    let m: RegExpExecArray | null;
    POLICY_STRUCT_OR_ENUM_DECL_RE.lastIndex = 0;
    while ((m = POLICY_STRUCT_OR_ENUM_DECL_RE.exec(content))) {
      const lineNo = policyLineOfIndex(content, m.index);
      const precedingLines = lines.slice(Math.max(0, lineNo - 6), lineNo - 1).join("\n");
      if (POLICY_DSL_DERIVE_RE.test(precedingLines)) {
        documentDsl.add(m[1]!);
        opText.add(m[1]!);
      }
    }
    POLICY_HAND_ROLLED_IMPL_RE.lastIndex = 0;
    while ((m = POLICY_HAND_ROLLED_IMPL_RE.exec(content))) {
      (m[1] === "DocumentDsl" ? documentDsl : opText).add(m[2]!);
    }
  }
  return { documentDsl, opText };
}

const POLICY_USE_ALIAS_RE = /\b(\w+)\s+as\s+(\w+)\b/g;

/**
 * 🔎️One O(total content) pass building every `RealName as AliasName` rename seen in any `use` item
 * (e.g. `use raster_core::{RasterSnapshot as RasterDocument};`) — a technology's block-kind wrapper
 * commonly re-exports another crate's already-DSL-complete type under a locally-meaningful alias
 * (`forms`'s `PlaybookSpec as FormSpec`, `raster/plugin`'s `RasterSnapshot as RasterDocument`), so a
 * plain per-file struct/impl scan alone would report a false gap on the alias name.
 */
function policyTypeAliasMap(files: readonly { content: string }[]): Map<string, string> {
  const aliasOf = new Map<string, string>();
  for (const { content } of files) {
    let m: RegExpExecArray | null;
    POLICY_USE_ALIAS_RE.lastIndex = 0;
    while ((m = POLICY_USE_ALIAS_RE.exec(content))) aliasOf.set(m[2]!, m[1]!);
  }
  return aliasOf;
}

/** 🔎️Resolves `typeName` through `aliasOf` (`RealName as AliasName` renames) up to a handful of hops, so an alias chain still bottoms out at its real declaration. */
function policyResolveAlias(aliasOf: ReadonlyMap<string, string>, typeName: string): string {
  let resolved = typeName;
  for (let hop = 0; hop < 5; hop++) {
    const next = aliasOf.get(resolved);
    if (!next || next === resolved) break;
    resolved = next;
  }
  return resolved;
}

/**
 * 📏️dsl/ migration lock-step rule: every `impl DocumentApp for X` app's `Snapshot`/`Mutation` type
 * must be DSL-complete — `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslOps)]` on the type itself (or
 * on the real type behind a `RealName as AliasName` import rename), a hand-rolled `impl DocumentDsl`/
 * `impl OpText`, or a documented generic bridge (see `POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST`).
 * Advisory/textual — the real compile-time gate is `DocumentApp`'s `Snapshot: vcs::DocumentDsl` /
 * `Mutation: vcs::OpText` bounds in `framework/plugin/rs/lib.rs`; this catches the same gap without
 * needing a full `cargo build`. A single pass builds the DSL-complete type-name sets once
 * (`policyDslCompleteTypeNames`) so checking every app's usage stays O(1) instead of re-scanning the
 * whole corpus per usage.
 */
export function policyDslCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const paths = policyAllRustFiles(repoRoot);
  const files = paths.map((relPath) => ({ relPath, content: policyReadFileSafe(repoRoot, relPath) }));
  const complete = policyDslCompleteTypeNames(files);
  const aliasOf = policyTypeAliasMap(files);

  for (const { relPath, content } of files) {
    for (const usage of policyDocumentAppUsages(relPath, content)) {
      const checks: readonly { label: "Snapshot" | "Mutation"; typeName: string; trait: "DocumentDsl" | "OpText"; completeNames: Set<string> }[] = [
        { label: "Snapshot", typeName: usage.snapshotType, trait: "DocumentDsl", completeNames: complete.documentDsl },
        { label: "Mutation", typeName: usage.mutationType, trait: "OpText", completeNames: complete.opText },
      ];
      for (const { label, typeName, trait, completeNames } of checks) {
        if (POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST.has(typeName)) continue;
        if (completeNames.has(typeName)) continue;
        if (completeNames.has(policyResolveAlias(aliasOf, typeName))) continue;
        breaches.push({
          id: `dsl-completeness-${relPath}-${usage.line}-${label}`,
          summary: `"${usage.scope}"'s DocumentApp::${label} = ${typeName} has neither a #[derive(dsl::Dsl...)] nor a hand-rolled impl ${trait}`,
          kind: "dsl-migration/completeness",
          scope: relPath,
          line: usage.line,
          priority: "high",
          reason: "Every DocumentApp app's Snapshot must implement vcs::DocumentDsl and Mutation must implement vcs::OpText (compiler-enforced since the Lock step) — via #[derive(dsl::DslDocument)]/#[derive(dsl::DslOps)], a hand-rolled impl, or a documented generic bridge.",
          solution: `Add #[derive(dsl::DslDocument)] (Snapshot) / #[derive(dsl::DslOps)] (Mutation) to ${typeName}, write a hand-rolled impl ${trait} for ${typeName}, or if it's a genuine generic bridge add it to POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST citing why.`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleDslCompleteness

//#region 🔧️PolicyRulePackCompleteness
/**
 * 📏️pack/ rollout lock-step rule: every `*.rs` file that calls `assert_dsl_round_trip(`/
 * `assert_document_text_round_trip(` must ALSO call `assert_dsl_pack_equivalence(`/
 * `assert_document_pack_round_trip(` somewhere in the same file — dsl and pack are two projections of
 * the same value model (see `vcs::DocumentPack`'s LAW doc comment), so a technology proving its DSL
 * round trip without also proving its pack round trip is an incomplete migration. A documented
 * allowlist (`POLICY_PACK_COMPLETENESS_ALLOWLIST`) tracks not-yet-converted files exactly like
 * `POLICY_JSON_FIXTURE_ALLOWLIST` does for the dsl/ migration — remove an entry once that file adds the
 * pack-equivalence call (see `vcs/rs/lib.rs`'s own `demo_dsl_pack_equivalence`/
 * `document_text_round_trips_after_apply_and_checkpoint` tests for the pattern).
 */
export function policyPackCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (POLICY_PACK_COMPLETENESS_ALLOWLIST.has(policyNormalizeRelPath(relPath))) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const hasDslCheck = content.includes("assert_dsl_round_trip(") || content.includes("assert_document_text_round_trip(");
    if (!hasDslCheck) continue;
    const hasPackCheck = content.includes("assert_dsl_pack_equivalence(") || content.includes("assert_document_pack_round_trip(");
    if (hasPackCheck) continue;
    breaches.push({
      id: `pack-completeness-${relPath}`,
      summary: `"${relPath}" calls assert_dsl_round_trip/assert_document_text_round_trip but never assert_dsl_pack_equivalence/assert_document_pack_round_trip`,
      kind: "pack-migration/completeness",
      scope: relPath,
      priority: "high",
      reason: "Every DSL round-trip test must have a pack-round-trip sibling on the same fixture(s), at the same test level — dsl and pack are two projections of the same (RecordSpec, RecordValue) value model, never two independently-maintained sources of truth.",
      solution: `Add an assert_dsl_pack_equivalence(...)/assert_document_pack_round_trip(...) call beside ${relPath}'s existing DSL round-trip test(s), or if this technology hasn't adopted pack yet, add "${relPath}" to POLICY_PACK_COMPLETENESS_ALLOWLIST citing the follow-up ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRulePackCompleteness

//#region 🔧️PolicyRuleCommandEnvelopeCompleteness
/**
 * 📏️CW7 command-envelope law rule — mirrors `policyPackCompletenessBreaches`'s shrinking-allowlist
 * pattern one step further: every file that already proves the dsl/pack round-trip laws
 * (`assert_dsl_pack_equivalence(`/`assert_document_pack_round_trip(`) must ALSO call
 * `vcs::test_support::assert_command_envelope_round_trip` beside them — a technology's `Edit<Mutation>`
 * round-tripping through `protocol::MutationEnvelope`s is a third projection of the same value model,
 * not an optional extra. `POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST` tracks not-yet-converted
 * files exactly like `POLICY_PACK_COMPLETENESS_ALLOWLIST` does for the pack lock step.
 */
export function policyCommandEnvelopeCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST.has(policyNormalizeRelPath(relPath))) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const hasPackCheck = content.includes("assert_dsl_pack_equivalence(") || content.includes("assert_document_pack_round_trip(");
    if (!hasPackCheck) continue;
    if (content.includes("assert_command_envelope_round_trip")) continue;
    breaches.push({
      id: `command-envelope-completeness-${relPath}`,
      summary: `"${relPath}" proves the pack round-trip law but never calls assert_command_envelope_round_trip`,
      kind: "protocol-migration/command-envelope-completeness",
      scope: relPath,
      priority: "high",
      reason: "CW7 added vcs::test_support::assert_command_envelope_round_trip as the command-envelope law every technology's Edit/Mutation pair must also prove, beside its existing dsl/pack round-trip laws.",
      solution: `Add a vcs::test_support::assert_command_envelope_round_trip::<Snapshot, Mutation>(...) call beside ${relPath}'s existing pack round-trip test(s), or if this technology hasn't wired the command-envelope law yet, add "${relPath}" to POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST citing the follow-up ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleCommandEnvelopeCompleteness

//#region 🔧️PolicyRuleDiffCompleteness
/** 🔺️ Every authored artifact diff owns both native representations outside its semantic schema. */
export function policyDiffCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [], files = policyAllRustFiles(repoRoot);
  const codecs = new Map<string, {text: string; binary: string}>();
  for (const relPath of files) {
    if (POLICY_DIFF_COMPLETENESS_ALLOWLIST.has(policyNormalizeRelPath(relPath))) continue;
    const owner = relPath.match(/^(.*?\/🗿️artifacts\/[^/]+(?:\/🏅️standards\/[^/]+\/🪆️subsets\/[^/]+)?)(?:\/|$)/u)?.[1];
    if (!owner || /\/(?:🚪️io|🧪️tests|🧫️fixtures)\//u.test(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!/\bMutationDiff\s*</u.test(content)) continue;
    let representations = codecs.get(owner);
    if (!representations) {
      representations = {text: "", binary: ""};
      for (const source of files.filter(path => path.startsWith(owner + "/🚪️io/"))) {
        if (source.includes("/📝️text/")) representations.text += policyReadFileSafe(repoRoot, source) + "\n";
        if (source.includes("/💾️binary/")) representations.binary += policyReadFileSafe(repoRoot, source) + "\n";
      }
      codecs.set(owner, representations);
    }
    const missing = missingArtifactDiffWireTypes(content, representations.text, representations.binary);
    if (!missing.length) continue;
    breaches.push({
      id: `diff-completeness-${relPath}`,
      summary: `"${relPath}" has diff types without paired I/O codecs: ${missing.join(", ")}`,
      kind: "artifact-io/diff-completeness",
      scope: relPath,
      priority: "high",
      reason: "Semantic diffs require independently owned DiffText and DiffBinary implementations.",
      solution: `Implement ${missing.join(", ")} in ${owner}/🚪️io/📝️text/🔺️diff and 🚪️io/💾️binary/🔺️diff.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleDiffCompleteness

//#region 🔧️PolicyRuleGrammarFileCompleteness
/**
 * 📏️W1 grammar-engine wave rule (design ruling B-R2) — the mirror-image of every other completeness
 * rule in this file: instead of scanning for real files that are missing something, this one simply
 * asserts `POLICY_GRAMMAR_FILE_ALLOWLIST` shrinks to empty over time (see that constant's doc comment
 * for the current, deliberately-empty W1 state — no app has committed a `.grammar` file yet, so
 * there is nothing to enumerate as a breach today). Kept as its own real (if currently vacuous)
 * policy function — not a bare constant check inline elsewhere — so a later wave that starts seeding
 * this allowlist with committed-but-stale `.grammar` files gets real breach reporting for free.
 */
export function policyGrammarFileBreaches(_repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of POLICY_GRAMMAR_FILE_ALLOWLIST) {
    breaches.push({
      id: `grammar-file-completeness-${relPath}`,
      summary: `"${relPath}" is tracked in POLICY_GRAMMAR_FILE_ALLOWLIST as missing/stale its handcrafted .grammar.semio spec`,
      kind: "dsl-migration/grammar-file-completeness",
      scope: relPath,
      priority: "low",
      reason: "Handcrafted grammar program: each artifact facet must commit a normative .grammar.semio checked by dsl_grammar::Recognizer.",
      solution: `Handcraft and commit ${relPath}, run the conformance sweep, then remove it from POLICY_GRAMMAR_FILE_ALLOWLIST.`,
    });
  }
  return breaches;
}

export function policyProtocolFileBreaches(_repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of POLICY_PROTOCOL_FILE_ALLOWLIST) {
    breaches.push({
      id: `protocol-file-completeness-${relPath}`,
      summary: `"${relPath}" is tracked in POLICY_PROTOCOL_FILE_ALLOWLIST as missing/stale its .protocol.semio spec`,
      kind: "dsl-migration/protocol-file-completeness",
      scope: relPath,
      priority: "low",
      reason: "Handcrafted protocol program: pack/spr facets must commit .protocol.semio verified by verify_protocol_bytes.",
      solution: `Handcraft and commit ${relPath}, prove recognizer agreement, then remove it from POLICY_PROTOCOL_FILE_ALLOWLIST.`,
    });
  }
  return breaches;
}

/**
 * 🏷️True when `relPath` is a constitutional non-mutation artifact TS facade.
 * Stubs under these paths are accepted — no per-file allowlist (Wave 2b).
 */
function policyIsConstitutionalTsFacadePath(relPath: string): boolean {
  const parts = relPath.replaceAll("\\", "/").split("/");
  if (parts[parts.length - 1] !== POLICY_TS_COMPONENT_LEAF) return false;
  if (!parts.includes("🗿️artifacts")) return false;
  const parent = parts[parts.length - 2] ?? "";
  if (POLICY_TS_FACADE_CONSTITUTIONAL_FACETS.has(parent)) return true;
  return false;
}

/** 🧪True when a TS facade still throws the WASM scaffold placeholder. */
function policyTsFacadeIsScaffoldStub(content: string): boolean {
  return /wire\s+.+\s+to plugin WASM/i.test(content) || /throw new Error\(\s*["'`][^"'`]*WASM[^"'`]*["'`]\s*\)/.test(content);
}

/**
 * 📏️Structural TS-facade rule (replaces POLICY_TS_FACADE_ALLOWLIST): scaffold stubs are accepted under
 * constitutional non-mutation facets. Mutation representations must be real descriptor-backed mirrors.
 */
export function policyTsFacadeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const files: string[] = [];
  const walk = (relDir: string): void => {
    let entries: Dirent[];
    try {
      entries = readdirSync(join(repoRoot, relDir), { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name) || ent.name.startsWith(".")) continue;
        walk(childRel);
        continue;
      }
      if (ent.name === POLICY_TS_COMPONENT_LEAF) files.push(childRel);
    }
  };
  walk("✏️s");
  for (const relPath of files.sort()) {
    if (!relPath.replaceAll("\\", "/").includes("/🗿️artifacts/")) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!policyTsFacadeIsScaffoldStub(content)) continue;
    if (policyIsConstitutionalTsFacadePath(relPath)) continue;
    breaches.push({
      id: `ts-facade-misplaced-stub-${relPath}`,
      summary: `"${relPath}" is a WASM scaffold stub outside constitutional artifact facets`,
      kind: "dsl-migration/ts-facade-completeness",
      scope: relPath,
      priority: "medium",
      reason: "Scaffold stubs are only accepted under constitutional facets (including 🧬️mutations and 🔧️op) or mutation triad leaves — never elsewhere.",
      solution: `Move ${relPath} under a constitutional facet / mutation triad, or wire it to plugin WASM.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleGrammarFileCompleteness

//#region 🔧️PolicyRuleProtocolMigration
/**
 * 🔒️Names that must never again be reached through a re-created `vcs::` shim: the temporary CW3
 * `pub use protocol::{...}` re-export deleted in CW8 (`Mutation`/`MutationDiff`/`MutationMeta`/
 * `OpText`/`Edit`/`merge_concurrent_diffs`/`ReconcileReport`/`ReconcileSeverity`), the CW3-extracted
 * `semio_framework_core` types (`HybridLogicalTimestamp`/`MutationEnvelope`/`MutationDag`/`UndoPolicy`/
 * `MergeStrategyKind`), and the hub wire-frame enums (`HubClientFrame`/`HubServerFrame`).
 */
const POLICY_PROTOCOL_MIGRATION_NAMES = [
  "Mutation",
  "MutationDiff",
  "MutationMeta",
  "OpText",
  "Edit",
  "merge_concurrent_diffs",
  "ReconcileReport",
  "ReconcileSeverity",
  "HybridLogicalTimestamp",
  "MutationEnvelope",
  "MutationDag",
  "UndoPolicy",
  "MergeStrategyKind",
  "HubClientFrame",
  "HubServerFrame",
] as const;
const POLICY_PROTOCOL_MIGRATION_QUALIFIED_RE = new RegExp(`\\bvcs::(${POLICY_PROTOCOL_MIGRATION_NAMES.join("|")})\\b`, "g");
const POLICY_PROTOCOL_MIGRATION_USE_BLOCK_RE = /use\s+(?:::)?vcs::\{([^}]*)\}/gs;

/**
 * 📏️CW8 regression-prevention rule: `vcs/rs/lib.rs`'s temporary CW3 `pub use protocol::{Mutation,
 * MutationDiff, OpText, MutationMeta, Edit, merge_concurrent_diffs, ReconcileReport,
 * ReconcileSeverity}` shim is deleted — every dependent crate now imports `protocol::` directly (see
 * `policyCommandEnvelopeCompletenessBreaches` above and CW7's import sweep). This rule fires the
 * moment `vcs::` is asked to re-supply any of those names again, or any of the other CW3-extracted
 * `semio_framework_core` types or the hub wire-frame enums — none of which `vcs` has ever re-exported,
 * so today this is pure regression prevention, not an open migration.
 */
export function policyProtocolMigrationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (relPath === "./🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/⚡️implementations/🦀️rust/📦️lib.rs") continue; // the crate that used to own the shim; never reaches itself via "vcs::"
    const content = policyReadFileSafe(repoRoot, relPath);
    const seenLines = new Set<number>();
    const lineOf = (index: number): number => content.slice(0, index).split(/\r?\n/).length;
    const isCommentLine = (line: number): boolean => /^\s*(\/\/|\*)/.test(content.split(/\r?\n/)[line - 1] ?? "");

    let m: RegExpExecArray | null;
    POLICY_PROTOCOL_MIGRATION_QUALIFIED_RE.lastIndex = 0;
    while ((m = POLICY_PROTOCOL_MIGRATION_QUALIFIED_RE.exec(content))) {
      // Skip "crate::vcs::X" — a crate's own nested module named "vcs" is not the external crate root.
      if (content.slice(Math.max(0, m.index - 7), m.index) === "crate::") continue;
      const line = lineOf(m.index);
      if (isCommentLine(line) || seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `protocol-migration-${relPath}-${line}`,
        summary: `"${relPath}:${line}" still references vcs::${m[1]} — the CW8 shim is gone, import protocol::${m[1]} directly`,
        kind: "protocol-migration/vcs-shim-regression",
        scope: relPath,
        line,
        priority: "high",
        reason: "CW8 deleted vcs/rs/lib.rs's temporary pub-use shim for protocol-owned Mutation/OpText/MutationDiff/MutationMeta/Edit/ReconcileReport/ReconcileSeverity and never re-exported the CW3-extracted framework_core types or hub wire frames — vcs:: must never resolve any of them again.",
        solution: `Import the name from "protocol" directly (e.g. "use protocol::${m[1]};") instead of "vcs::${m[1]}".`,
      });
    }

    POLICY_PROTOCOL_MIGRATION_USE_BLOCK_RE.lastIndex = 0;
    while ((m = POLICY_PROTOCOL_MIGRATION_USE_BLOCK_RE.exec(content))) {
      const tokens = new Set((m[1] ?? "").split(/[^\w]+/).filter(Boolean));
      const hit = POLICY_PROTOCOL_MIGRATION_NAMES.find((name) => tokens.has(name));
      if (!hit) continue;
      const line = lineOf(m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `protocol-migration-use-${relPath}-${line}`,
        summary: `"${relPath}:${line}" imports ${hit} from vcs:: — the CW8 shim is gone, import protocol::${hit} directly`,
        kind: "protocol-migration/vcs-shim-regression",
        scope: relPath,
        line,
        priority: "high",
        reason: "CW8 deleted vcs/rs/lib.rs's temporary pub-use shim; a multi-name \"use vcs::{...}\" block must not re-import a name that only protocol:: still owns.",
        solution: `Split the "use vcs::{...}" block at ${relPath}:${line}: keep vcs's own names there and add a separate "use protocol::{${hit}, ...};" for ${hit}.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleProtocolMigration
