/** 🧬️ Wave 4 plugin consistency policy: sniff reality and the schema overhaul S2 rules.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { canonicalPrimaryFilenameForKind, type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { loadCatalogTaxonomy } from "../../../🔍️discovery/🟦️.ts";
import { policyReadFileSafe, policyReaddirSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyListArtifactDialectDirs } from "../../../🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, policyStripEmoji } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policySnakeToCamel } from "../../../🧬️schema/🔍️field-discovery/🧱️contract/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { POLICY_STDIO_ARTIFACTS_REL, policyRustFileHasRealTraitImpl } from "../🚪️artifact-io/🟦️.ts";
import { policyStripTsCommentsAndStrings } from "../🚧️authority-bans/🟦️.ts";
import { policyAllRustFiles, policyNormalizeRelPath } from "../🏛️foundation/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

//#region 🔧️PolicyRuleSniffReality
/**
 * 🎫️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 4: a
 * `fn sniff(...)` whose parameter is underscore-prefixed (Rust's own "deliberately unused"
 * convention) cannot possibly inspect the bytes/text it was handed — it can only ever return a
 * constant confidence, which defeats the entire point of format sniffing. Seeded at V0 with every
 * file that fails this today (shrink-only allowlist, same pattern as
 * `POLICY_PACK_COMPLETENESS_ALLOWLIST`): as each artifact's sniff is made real (see the codec
 * uplift waves), remove its entry here — an allowlisted entry whose sniff has ALREADY been fixed
 * is flagged separately below as a stale-acceptance breach, so cleanup isn't silently forgotten.
 */
const POLICY_SNIFF_REALITY_ALLOWLIST = new Set<string>([
  "writer/standards#1-subsets-any-analyzer-component",
  "mathematical/equation/standards#1-subsets-any-analyzer-component",
  "procedural/generation2d/standards#1-subsets-any-analyzer-component",
  "procedural/generation3d/standards#1-subsets-any-analyzer-component",
  "flow/standards#1-subsets-any-analyzer-component",
  "gis/gisterrain/standards#1-subsets-any-analyzer-component",
  "gis/gismap/standards#1-subsets-any-analyzer-component",
  "vcs/standards#1-subsets-any-analyzer-component",
  "animate/presentation/standards#1-subsets-any-analyzer-component",
  "shooting/standards#1-subsets-any-analyzer-component",
  "demonstrator/playground/standards#1-subsets-any-analyzer-component",
  "sequence/standards#1-subsets-any-analyzer-component",
  "fem/2d/standards#1-subsets-any-analyzer-component",
  "fem/3d/standards#1-subsets-any-analyzer-component",
  "architect/program/standards#1-subsets-any-analyzer-component",
  "process/process3d/standards#1-subsets-any-analyzer-component",
  "lowpoly/standards#1-subsets-any-analyzer-component",
  "reasoning/wires/standards#1-subsets-any-analyzer-component",
  "forms/standards#1-subsets-any-analyzer-component",
  "layout/standards#1-subsets-any-analyzer-component",
  "cad/standards#1-subsets-any-analyzer-component",
  "norm/iso16757/standards#1-subsets-any-analyzer-component",
  "norm/vdi3805/standards#1-subsets-any-analyzer-component",
  "norm/din4108/standards#1-subsets-any-analyzer-component",
  "norm/din16798/standards#1-subsets-any-analyzer-component",
  "norm/en1990/standards#1-subsets-any-analyzer-component",
  "norm/en1991/standards#1-subsets-any-analyzer-component",
  "norm/en1992/standards#1-subsets-any-analyzer-component",
  "norm/en1993/standards#1-subsets-any-analyzer-component",
  "norm/en1994/standards#1-subsets-any-analyzer-component",
  "norm/en1995/standards#1-subsets-any-analyzer-component",
  "norm/en1996/standards#1-subsets-any-analyzer-component",
  "norm/en1997/standards#1-subsets-any-analyzer-component",
  "norm/en1998/standards#1-subsets-any-analyzer-component",
  "norm/en1999/standards#1-subsets-any-analyzer-component",
  "norm/din18599/standards#1-subsets-any-analyzer-component",
  "playbook/standards#1-subsets-any-analyzer-component",
  "imperative/standards#1-subsets-any-analyzer-component",
  "remodel/standards#1-subsets-any-analyzer-component",
  "energy/model/standards#1-subsets-any-analyzer-component",
  "trinity/rewriting/standards#1-subsets-any-analyzer-component",
  "trinity/jack/standards#1-subsets-any-analyzer-component",
  "dag/standards#1-subsets-any-analyzer-component",
  "draw/standards#1-subsets-any-analyzer-component",
  "raster/standards#1-subsets-any-analyzer-component",
  "stdio/ifc/standards#4-subsets-any-analyzer-component",
  "stdio/binary/standards#raw-subsets-any-analyzer-component",
  "stdio/pdf/standards#1.4-subsets-any-analyzer-component",
  "stdio/step/standards#ap214-subsets-any-analyzer-component",
  "stdio/xml/standards#1.0-subsets-any-analyzer-component",
  "stdio/dwg/standards#ac1018-subsets-any-analyzer-component",
  "stdio/dwg/standards#ac1024-subsets-any-analyzer-component",
  "stdio/deflate/standards#rfc1950-subsets-any-analyzer-component",
  "note/standards#1-subsets-any-analyzer-component",
  "puzzle/2d/standards#1-subsets-any-analyzer-component",
  "puzzle/5d/standards#1-subsets-any-analyzer-component",
  "puzzle/3d/standards#1-subsets-any-analyzer-component",
  "block/2d/standards#1-subsets-any-analyzer-component",
  "block/5d/standards#1-subsets-any-analyzer-component",
  "block/3d/standards#1-subsets-any-analyzer-component",
  "space/home/standards#1-subsets-any-analyzer-component",
  "sourcing/curate/standards#1-subsets-any-analyzer-component",
]);

export function policySniffRealityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content.includes("fn sniff(")) continue;
    const hasUnusedParam = /fn\s+sniff\(\s*_[A-Za-z0-9_]*\s*:/.test(content);
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_SNIFF_REALITY_ALLOWLIST.has(normalized);
    if (hasUnusedParam) {
      if (allowlisted) continue;
      breaches.push({
        id: `sniff-reality-${relPath}`,
        summary: `"${relPath}" declares fn sniff(...) with an underscore-prefixed (unused) parameter`,
        kind: "artifact-io/sniff-reality",
        scope: relPath,
        priority: "high",
        reason: "sniff() must inspect the bytes/text it's handed to produce a real confidence signal — an unused parameter means it can only ever return a hardcoded constant, defeating format detection.",
        solution: `Make ${relPath}'s sniff() branch on its argument (magic bytes / structural check), or if this artifact hasn't been reached by the codec uplift yet, add "${normalized}" to POLICY_SNIFF_REALITY_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `sniff-reality-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_SNIFF_REALITY_ALLOWLIST but its sniff() already uses its argument`,
        kind: "artifact-io/sniff-reality",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed, or the allowlist silently drifts into meaninglessness.",
        solution: `Remove "${normalized}" from POLICY_SNIFF_REALITY_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleSniffReality


//#region 🔧️PolicyRuleSchemaOverhaulS2
/**
 * 🎫️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, S2 (glue.rs /
 * script.ts / schema-descriptor spine wave): S-7 (vcs collection-machinery ban) + S-8 (four new
 * drift/honesty/coverage rules). All five allowlists below are shrink-only per the ticket's own
 * convention (`POLICY_SNIFF_REALITY_ALLOWLIST`'s stale-entry sub-check pattern): a "new breach"
 * fires when a file/standard fails the check and isn't allowlisted; a "stale" low-priority breach
 * fires when an allowlisted entry has ALREADY been fixed, so cleanup is never silently forgotten.
 * Every allowlist here was seeded by running the exact detection logic below against the real tree
 * at S2 time (see `s2-artifacts/gen_s8_seeds.ts` in this ticket folder for the generator) — none of
 * these are hand-guessed.
 */

/**
 * 📏️S-7: the vcs plugin's generic `CollectionDiff`/`CollectionMutation`/`Patchable`/`Identified`
 * machinery has real users (🌊️flow's `FlowMutation`, 🌿️vcs itself, 🏪️store's re-export — verified by
 * S1) but ZERO stdio artifact users today (confirmed by grep at S1 and re-confirmed here). This
 * ticket's design has every stdio artifact-standard handcraft its own snapshot/diff/mutation triple
 * per `🧬️schema-design.md` instead of reaching for the generic collection machinery — this rule
 * keeps that true going forward. Seeded EMPTY: any hit is a real regression, not a backlog item.
 */
const POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST = new Set<string>([]);

const POLICY_STDIO_VCS_MACHINERY_BAN_MARKERS = ["CollectionDiff", "CollectionMutation", "Patchable", "Identified"] as const;
const POLICY_STDIO_VCS_MACHINERY_BAN_RE = new RegExp(`\\b(${POLICY_STDIO_VCS_MACHINERY_BAN_MARKERS.join("|")})\\b`);

function policyStdioVcsMachineryBanBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const prefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith(prefix)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const stripped = policyStripRustCommentsAndStrings(content);
    const hit = stripped.match(POLICY_STDIO_VCS_MACHINERY_BAN_RE)?.[1];
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST.has(normalized);
    if (hit) {
      if (allowlisted) continue;
      breaches.push({
        id: `stdio-vcs-machinery-ban-${relPath}`,
        summary: `"${relPath}" references vcs collection machinery (${hit}) — banned under stdio artifacts`,
        kind: "stdio-artifacts/vcs-machinery-ban",
        scope: relPath,
        priority: "high",
        reason: "Stdio artifacts handcraft their own snapshot/diff/mutation triples per artifact (see 🧬️schema-design.md); the generic vcs CollectionDiff/CollectionMutation/Patchable/Identified machinery is reserved for flow/vcs/store and must never leak into stdio.",
        solution: `Remove the ${hit} reference from ${relPath} and handcraft this artifact's own diff/mutation type instead, or add "${normalized}" to POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST citing a real, reviewed justification (expected to stay empty forever).`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `stdio-vcs-machinery-ban-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST but no longer references vcs collection machinery`,
        kind: "stdio-artifacts/vcs-machinery-ban",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/** 🗂️One (artifact, standard, subset) triple under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`, restricted to real on-disk dirs (dynamic — NOT a hardcoded "31/32" count, so a future new standard/subset is picked up automatically). */
type PolicyStdioStandardEntry = { artRel: string; artifactId: string; standardSlug: string; subsetRel: string; subsetId: string };

function policyListStdioStandardEntries(repoRoot: string): PolicyStdioStandardEntry[] {
  const out: PolicyStdioStandardEntry[] = [];
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    if (!dialect.artRel.startsWith(`${POLICY_STDIO_ARTIFACTS_REL}/`)) continue;
    out.push({
      artRel: dialect.artRel,
      artifactId: policyStripEmoji(dialect.artRel.slice(POLICY_STDIO_ARTIFACTS_REL.length + 1)),
      standardSlug: dialect.standardSlug,
      subsetRel: dialect.subsetRel,
      subsetId: dialect.subsetId,
    });
  }
  return out;
}

/**
 * 🔎️Only the SCHEMA-OWNING subset entries (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES,
 * replaces the old exact-string `subsetId === "✳️any"` filter). A subset is schema-owning iff its
 * `🧬️schema/📸️snapshot/` dir exists on disk — a real conformance subset (pdf 1.7's `✳️a`, step's
 * `✳️cc6`, …) is a validation-gated DELEGATING stamp on top of its standard's owning subset (pure
 * `pub use …::subsets::any::schema::*`, per the pilot), never a new snapshot type, so it never has
 * its own `📸️snapshot/` dir. `✳️any` is always schema-owning by construction. This is structural
 * rather than name-based so a future new subset never needs an allowlist/filter edit here — it is
 * automatically excluded from schema-internal rules (DiffAlgebra, field-sweep, grammar-honesty,
 * facet-mirror-drift) the moment it's created as a delegating subset, and automatically INCLUDED in
 * builder/analyzer/composer/io rules (those already iterate the full `policyListStdioStandardEntries`
 * result, unfiltered).
 */
export function policyListStdioSchemaOwningEntries(repoRoot: string): PolicyStdioStandardEntry[] {
  return policyListStdioStandardEntries(repoRoot).filter((e) => existsSync(join(repoRoot, e.subsetRel, "🧬️schema", "📸️snapshot")));
}

/**
 * 📏️S-8 rule 3 (`POLICY_DIFF_ALGEBRA`): every stdio artifact-standard's `🔺️diff/🦀️.rs` must
 * carry a real `impl DiffAlgebra<...> for ...` block (S1 added the trait, deliberately implemented it
 * for nothing — see `s1-spine-report.md`). Seeded with all 31 current standards; F-wave agents shrink
 * this to zero as they land real diffs with handcrafted `inverse`/`is_empty`.
 */
const POLICY_DIFF_ALGEBRA_ALLOWLIST = new Set<string>([]);

function policyDiffAlgebraBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    const rustRel = `${entry.subsetRel}/🧬️schema/🔺️diff/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    if (!existsSync(join(repoRoot, rustRel))) continue;
    const content = policyReadFileSafe(repoRoot, rustRel);
    const hasImpl = policyRustFileHasRealTraitImpl(content, "DiffAlgebra");
    const normalized = policyNormalizeRelPath(rustRel);
    const allowlisted = POLICY_DIFF_ALGEBRA_ALLOWLIST.has(normalized);
    if (!hasImpl) {
      if (allowlisted) continue;
      breaches.push({
        id: `diff-algebra-missing-${rustRel}`,
        summary: `"${rustRel}" has no impl DiffAlgebra<...> for ... block`,
        kind: "stdio-artifacts/diff-algebra",
        scope: entry.artRel,
        priority: "medium",
        reason: "Every stdio diff type must implement DiffAlgebra (inverse/is_empty) alongside MutationDiff — see 🧬️schema-design.md's Verb set per artifact.",
        solution: `Implement DiffAlgebra<${entry.artifactId}Snapshot> for the diff type in ${rustRel}, or if this standard hasn't been reached yet, add "${normalized}" to POLICY_DIFF_ALGEBRA_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `diff-algebra-stale-${rustRel}`,
        summary: `"${rustRel}" is allowlisted in POLICY_DIFF_ALGEBRA_ALLOWLIST but already implements DiffAlgebra`,
        kind: "stdio-artifacts/diff-algebra",
        scope: entry.artRel,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_DIFF_ALGEBRA_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

export function policyStdioStandardKey(artifactId: string, standardSlug: string): string {
  return `stdio/${artifactId}/standards#${standardSlug}`;
}

/**
 * 📏️S-8 rule 2 (`POLICY_GRAMMAR_HONESTY`): flags a grammar leaf (`.g4`/`.ebnf`/`.grammar.semio` under
 * `📝️text/`, `.ksy`/`.spicy`/`.abnf`/`.protocol.semio` under `💾️binary/`) whose content is still the
 * placeholder skeleton every stdio facet was scaffolded with (`payload = *OCTET` / `size-eos: true` /
 * `payload: bytes &eod;` / the fixed `DOCUMENT: 'schema' [ ]+`/`header = 'schema', space,` literal
 * templates) — literal-marker textual heuristic, matching this repo's other grammar/spec policies
 * (`policySpecDistinctnessBreaches` et al.), not a real parser. Scoped to stdio only (this ticket's
 * scope); the repo-wide `.grammar`-file completeness program is `POLICY_GRAMMAR_FILE_ALLOWLIST`
 * (jolly-spindle W1), a different rule for a different file convention. Seeded with the current
 * census — 645 of 651 stdio grammar leaves (nearly all, as expected: this program hasn't rewritten
 * any grammar leaf yet, only json's `.grammar.semio`/`.protocol.semio` got real content from an
 * earlier wave and are correctly NOT in this seed).
 */
const POLICY_GRAMMAR_HONESTY_LEAF_MARKERS: Record<string, string> = {
  [canonicalPrimaryFilenameForKind("antlr", loadCatalogTaxonomy())]: "DOCUMENT: 'schema' [ ]+",
  [canonicalPrimaryFilenameForKind("ebnf", loadCatalogTaxonomy())]: "header = 'schema', space,",
  [canonicalPrimaryFilenameForKind("grammar-semio", loadCatalogTaxonomy())]: "payload = *OCTET",
  [canonicalPrimaryFilenameForKind("abnf", loadCatalogTaxonomy())]: "payload = *OCTET",
  [canonicalPrimaryFilenameForKind("protocol-semio", loadCatalogTaxonomy())]: "payload = *OCTET",
  [canonicalPrimaryFilenameForKind("kaitai", loadCatalogTaxonomy())]: "size-eos: true",
  [canonicalPrimaryFilenameForKind("spicy", loadCatalogTaxonomy())]: "payload: bytes &eod;",
};

const POLICY_GRAMMAR_HONESTY_ALLOWLIST = new Set<string>([
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-binary-component.abnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-binary-component.ksy",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-binary-component.spicy",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-text-component.ebnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-text-component.g4",
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-text-component.g4",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-text-component.g4",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-binary-component.abnf",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-binary-component.ksy",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-binary-component.spicy",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/bmp/standards#v3-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-binary-component.abnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-binary-component.ksy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-binary-component.spicy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-text-component.ebnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-text-component.g4",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-text-component.g4",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-text-component.g4",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-binary-component.abnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-binary-component.ksy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-binary-component.spicy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-text-component.ebnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-text-component.g4",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-text-component.g4",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-text-component.g4",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-binary-component.abnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-binary-component.ksy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-binary-component.spicy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-text-component.ebnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-text-component.g4",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-text-component.g4",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-text-component.g4",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/dxf/standards#r12-subsets-any-schema-diff-binary-component.ksy",
  "stdio/dxf/standards#r12-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/dxf/standards#r12-subsets-any-schema-diff-binary-component.spicy",
  "stdio/dxf/standards#r12-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/dxf/standards#r12-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/dxf/standards#r12-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/dxf/standards#r12-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/dxf/standards#r12-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/dxf/standards#r12-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/gif/standards#87a-subsets-any-schema-diff-binary-component.abnf",
  "stdio/gif/standards#87a-subsets-any-schema-diff-binary-component.ksy",
  "stdio/gif/standards#87a-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/gif/standards#87a-subsets-any-schema-diff-binary-component.spicy",
  "stdio/gif/standards#87a-subsets-any-schema-diff-text-component.ebnf",
  "stdio/gif/standards#87a-subsets-any-schema-diff-text-component.g4",
  "stdio/gif/standards#87a-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-text-component.g4",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-text-component.g4",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/gif/standards#89a-subsets-any-schema-diff-binary-component.abnf",
  "stdio/gif/standards#89a-subsets-any-schema-diff-binary-component.ksy",
  "stdio/gif/standards#89a-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/gif/standards#89a-subsets-any-schema-diff-binary-component.spicy",
  "stdio/gif/standards#89a-subsets-any-schema-diff-text-component.ebnf",
  "stdio/gif/standards#89a-subsets-any-schema-diff-text-component.g4",
  "stdio/gif/standards#89a-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-text-component.g4",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-text-component.g4",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/gltf/standards#2.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/gltf/standards#2.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/ifc/standards#4-subsets-any-schema-diff-binary-component.abnf",
  "stdio/ifc/standards#4-subsets-any-schema-diff-binary-component.ksy",
  "stdio/ifc/standards#4-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/ifc/standards#4-subsets-any-schema-diff-binary-component.spicy",
  "stdio/ifc/standards#4-subsets-any-schema-diff-text-component.ebnf",
  "stdio/ifc/standards#4-subsets-any-schema-diff-text-component.g4",
  "stdio/ifc/standards#4-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-text-component.g4",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/ifc/standards#4-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-diff-binary-component.abnf",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-diff-binary-component.ksy",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-diff-binary-component.spicy",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/las/standards#1.0-subsets-any-schema-diff-binary-component.abnf",
  "stdio/las/standards#1.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/las/standards#1.0-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/las/standards#1.0-subsets-any-schema-diff-binary-component.spicy",
  "stdio/las/standards#1.0-subsets-any-schema-diff-text-component.ebnf",
  "stdio/las/standards#1.0-subsets-any-schema-diff-text-component.g4",
  "stdio/las/standards#1.0-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-text-component.g4",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/obj/standards#3.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/obj/standards#3.0-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/obj/standards#3.0-subsets-any-schema-diff-binary-component.spicy",
  "stdio/obj/standards#3.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/obj/standards#3.0-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/obj/standards#3.0-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/obj/standards#3.0-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/obj/standards#3.0-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/obj/standards#3.0-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-binary-component.abnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-binary-component.ksy",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-binary-component.spicy",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-text-component.ebnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-text-component.g4",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-text-component.g4",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-text-component.g4",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/pdf/standards#1.7-subsets-any-schema-diff-binary-component.ksy",
  "stdio/pdf/standards#1.7-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/pdf/standards#1.7-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/pdf/standards#1.7-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-binary-component.abnf",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-binary-component.spicy",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-text-component.ebnf",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-text-component.g4",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-text-component.g4",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-binary-component.abnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-binary-component.ksy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-binary-component.spicy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-text-component.ebnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-text-component.g4",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-text-component.g4",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-text-component.g4",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/step/standards#ap214-subsets-any-schema-diff-binary-component.abnf",
  "stdio/step/standards#ap214-subsets-any-schema-diff-binary-component.ksy",
  "stdio/step/standards#ap214-subsets-any-schema-diff-binary-component.spicy",
  "stdio/step/standards#ap214-subsets-any-schema-diff-text-component.ebnf",
  "stdio/step/standards#ap214-subsets-any-schema-diff-text-component.g4",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-text-component.g4",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-text-component.g4",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-binary-component.abnf",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-binary-component.ksy",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-binary-component.spicy",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-text-component.ebnf",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-text-component.g4",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-text-component.g4",
  "stdio/svg/standards#1.1-subsets-any-schema-diff-binary-component.abnf",
  "stdio/svg/standards#1.1-subsets-any-schema-diff-binary-component.ksy",
  "stdio/svg/standards#1.1-subsets-any-schema-diff-binary-component.spicy",
  "stdio/svg/standards#1.1-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/svg/standards#1.1-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/svg/standards#1.1-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-text-component.g4",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-binary-component.abnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-binary-component.ksy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-binary-component.spicy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-text-component.ebnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-text-component.g4",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-text-component.grammar.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-text-component.g4",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-text-component.grammar.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-text-component.g4",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-text-component.grammar.semio",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-binary-component.abnf",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-binary-component.ksy",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-binary-component.protocol.semio",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-binary-component.spicy",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-binary-component.protocol.semio",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/csv/standards#rfc4180-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/csv/standards#rfc4180-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/csv/standards#rfc4180-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-diff-binary-component.ksy",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/json/standards#rfc8259-subsets-any-schema-diff-binary-component.ksy",
  "stdio/json/standards#rfc8259-subsets-any-schema-diff-text-component.ebnf",
  "stdio/json/standards#rfc8259-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/json/standards#rfc8259-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/json/standards#rfc8259-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/json/standards#rfc8259-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/xml/standards#1.0-subsets-any-schema-diff-binary-component.abnf",
  "stdio/xml/standards#1.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/xml/standards#1.0-subsets-any-schema-diff-binary-component.spicy",
  "stdio/xml/standards#1.0-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/xml/standards#1.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/xml/standards#1.0-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/xml/standards#1.0-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/xml/standards#1.0-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/xml/standards#1.0-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-binary-component.abnf",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-binary-component.ksy",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-binary-component.spicy",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-text-component.ebnf",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-text-component.g4",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-binary-component.abnf",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-binary-component.ksy",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-binary-component.spicy",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-text-component.ebnf",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-text-component.g4",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-binary-component.abnf",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-binary-component.ksy",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-binary-component.protocol.semio",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-binary-component.spicy",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-text-component.ebnf",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-text-component.g4",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-text-component.grammar.semio",
]);

function policyGrammarHonestyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const walk = (relDir: string): void => {
    for (const ent of policyReaddirSafe(repoRoot, relDir)) {
      const childRel = `${relDir}/${ent.name}`;
      if (ent.isDirectory) {
        walk(childRel);
        continue;
      }
      const marker = POLICY_GRAMMAR_HONESTY_LEAF_MARKERS[ent.name];
      if (!marker) continue;
      const content = policyReadFileSafe(repoRoot, childRel);
      const isPlaceholder = content.includes(marker);
      const normalized = policyNormalizeRelPath(childRel);
      const allowlisted = POLICY_GRAMMAR_HONESTY_ALLOWLIST.has(normalized);
      if (isPlaceholder) {
        if (allowlisted) continue;
        breaches.push({
          id: `grammar-honesty-${childRel}`,
          summary: `"${childRel}" is still the scaffolded placeholder grammar, not a handcrafted spec`,
          kind: "stdio-artifacts/grammar-honesty",
          scope: childRel,
          priority: "low",
          reason: "Every grammar/protocol leaf must be handcrafted honestly per the format's real spec (user decision, see the master plan's Context section) — no *OCTET/size-eos passthrough placeholders may survive an artifact's own wave.",
          solution: `Handcraft ${childRel} to reflect this facet's real structure, or if this artifact/facet hasn't been reached yet, add "${normalized}" to POLICY_GRAMMAR_HONESTY_ALLOWLIST citing this ticket.`,
        });
      } else if (allowlisted) {
        breaches.push({
          id: `grammar-honesty-stale-${childRel}`,
          summary: `"${childRel}" is allowlisted in POLICY_GRAMMAR_HONESTY_ALLOWLIST but is no longer a placeholder`,
          kind: "stdio-artifacts/grammar-honesty",
          scope: childRel,
          priority: "low",
          reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
          solution: `Remove "${normalized}" from POLICY_GRAMMAR_HONESTY_ALLOWLIST.`,
        });
      }
    }
  };
  walk(POLICY_STDIO_ARTIFACTS_REL);
  return breaches;
}

/**
 * 📏️S-8 rule 1 (`POLICY_FACET_MIRROR_DRIFT`): for every stdio standard's snapshot/diff/mutations
 * facet, the camelCased public field identifiers found in the Rust `.rs` leaf should also appear
 * (textually) in the sibling `.ts`/`.graphql`/`.json`/`.proto` leaves — textual heuristic (substring
 * presence), not a real cross-language type check, matching this repo's other facet-parity policies.
 * Bidirectional: this also harvests each sibling leaf's OWN declared field names (interface members,
 * GraphQL type/input fields, JSON Schema `properties` keys, proto message fields) and flags any that
 * have no corresponding Rust field — this is what catches a sibling still shaped like the generic W1b
 * scaffold (`entries: [{key, value}]`) sitting where real fields belong; the forward direction alone
 * would pass that scaffold as long as it also happened to textually contain the real field names.
 * Seeded with the current census: ALL 93 checked (standard,facet) pairs drift today (31 standards × 3
 * facets — none of this program's facet mirrors have been rewritten yet; e.g. gif's TS mirror is
 * still literally zip's, per the master plan's own opening finding). F-wave agents shrink this to
 * zero as they rewrite each artifact's facets for real; this wave does NOT fix any facet itself.
 */
const POLICY_FACET_MIRROR_DRIFT_FACETS = ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const;
const POLICY_FACET_MIRROR_DRIFT_SIBLINGS = ["typescript-source", "graphql", "json", "protobuf"].map((kindId) => canonicalPrimaryFilenameForKind(kindId, loadCatalogTaxonomy()));
const POLICY_FACET_MIRROR_DRIFT_FIELD_RE = /(?:^|[\s{,(])(?:pub\s+)?([a-z][a-z0-9_]*)\s*:\s*[A-Za-z_&\[<('"]/gm;
const POLICY_FACET_MIRROR_DRIFT_KEYWORDS = new Set(["self", "where", "if", "else", "match", "for", "while", "let", "fn", "return", "in", "as", "dyn", "mut", "ref", "impl", "type"]);



/** 🔎️CamelCased public field/variant identifiers referenced in a schema facet's Rust leaf (textual heuristic, not a parser — matches struct field decls AND enum struct-variant field decls). */
function policyFacetRustFieldNames(content: string): string[] {
  const stripped = policyStripRustCommentsAndStrings(content);
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_FIELD_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_FIELD_RE.exec(stripped))) {
    const n = m[1]!;
    if (POLICY_FACET_MIRROR_DRIFT_KEYWORDS.has(n) || n.startsWith("r#")) continue;
    names.add(n);
  }
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

//#region 🔧️PolicyFacetMirrorDriftReverse
const POLICY_FACET_MIRROR_DRIFT_TS_RE = /(?:^|[\s{;(])([A-Za-z_][A-Za-z0-9_]*)\??\s*:\s*[^;\n=]+[;\n]/gm;
const POLICY_FACET_MIRROR_DRIFT_TS_KEYWORDS = new Set([
  "interface", "type", "export", "import", "from", "extends", "implements", "class", "function", "const", "let", "var",
  "namespace", "module", "declare", "readonly", "public", "private", "protected", "static", "abstract", "new", "this",
  "super", "typeof", "keyof", "case", "default", "switch", "try", "catch", "finally", "throw", "async", "await", "yield",
  "get", "set", "constructor", "void", "never", "unknown", "any", "undefined", "null", "true", "false", "satisfies",
  "asserts", "is", "infer", "of", "in", "as", "if", "else", "for", "while", "return",
]);
const POLICY_FACET_MIRROR_DRIFT_GRAPHQL_RE = /^\s*([a-z][A-Za-z0-9_]*)\s*:/gm;
const POLICY_FACET_MIRROR_DRIFT_PROTO_RE = /^\s*(?:optional\s+|repeated\s+)?(?:map\s*<[^>]+>|[\w.]+)\s+([a-z][a-z0-9_]*)\s*=\s*\d+\s*;/gm;
const POLICY_FACET_MIRROR_DRIFT_PROTO_SKIP_RE = /^\s*(enum|oneof|message|package|syntax|import)\b/;
const POLICY_FACET_MIRROR_DRIFT_SERDE_TAG_RE = /#\[serde\([^\]]*\btag\s*=\s*"([a-zA-Z_][a-zA-Z0-9_]*)"/g;
/** ⚖️Below-which a sibling field name is too short to trust as a substring match against a real Rust field (avoids "id"/"at"-style accidental hits) — see `policyFacetMirrorDriftBreaches`'s extraFields step. */
const POLICY_FACET_MIRROR_DRIFT_SUBSTRING_MIN_LEN = 4;

/**
 * 🔎️Serde internally-tagged-enum discriminant values (`#[serde(tag = "kind")]`) found in a schema
 * facet's Rust leaf. These are real fields every non-Rust mirror must carry (GraphQL/JSON-Schema/
 * proto have no native tagged-union sugar, so they always spell the discriminant out as an actual
 * field) even though `policyFacetRustFieldNames` never sees them — there is no `kind: SomeType`
 * struct-field declaration in the Rust source, just this attribute. Read raw (not comment/string
 * stripped) since the tag value IS a string literal; the regex is narrow enough not to misfire
 * inside a line or block comment in practice.
 */
function policyFacetRustTagFieldNames(content: string): string[] {
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_SERDE_TAG_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_SERDE_TAG_RE.exec(content))) names.add(m[1]!);
  return [...names];
}

const POLICY_FACET_MIRROR_DRIFT_VARIANT_RE = /^\s*([A-Z][A-Za-z0-9]*)\s*[{(]/gm;

/**
 * 🔎️PascalCase enum variant names declared in a schema facet's Rust leaf (e.g. `enum BmpMutation {
 * SetHeaderFields { .. }, SetPixelData { .. }, .. }`, or a plain `Rgb { r, g, b }` color variant).
 * `policyFacetRustFieldNames` only sees the FIELDS a struct/struct-variant declares, never the
 * variant name itself — but externally-tagged (the serde default, no `#[serde(tag = "…")]`) or
 * `oneof`-shaped mirrors spell the variant name out as a real field/message/oneof-arm identifier
 * (proto's `SetHeaderFields set_header_fields = 3;`, a GraphQL union member, a JSON discriminant
 * value, …), same problem `policyFacetRustTagFieldNames` solves for the internally-tagged case.
 * Line-start heuristic (`PascalCaseIdent` immediately followed by `{`/`(`) — true parsing would
 * need brace-tracked enum-body awareness this file's other policy rules don't attempt either; a
 * stray match (an `Ok(`/`Some(`/struct-literal-construction hit) only ever ADDS a permitted name,
 * never suppresses a real breach, so the cost of over-matching here is small.
 */
function policyFacetRustVariantFieldNames(content: string): string[] {
  const stripped = policyStripRustCommentsAndStrings(content);
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_VARIANT_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_VARIANT_RE.exec(stripped))) {
    const n = m[1]!;
    names.add(n.charAt(0).toLowerCase() + n.slice(1));
  }
  return [...names];
}

/** 🔎️Field names a `.ts` sibling mirror leaf itself declares (interface/type member lines), for the reverse extra-field drift check. */
function policyFacetTsFieldNames(content: string): string[] {
  const stripped = policyStripTsCommentsAndStrings(content);
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_TS_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_TS_RE.exec(stripped))) {
    const n = m[1]!;
    if (POLICY_FACET_MIRROR_DRIFT_TS_KEYWORDS.has(n)) continue;
    names.add(n);
  }
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Field names a `.graphql` sibling mirror leaf itself declares (type/input field lines), for the reverse extra-field drift check. */
function policyFacetGraphqlFieldNames(content: string): string[] {
  const noComments = content.replace(/#.*$/gm, "");
  const noArgs = noComments.replace(/\([^)]*\)/g, "");
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_GRAPHQL_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_GRAPHQL_RE.exec(noArgs))) names.add(m[1]!);
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Field names a `.json` sibling mirror leaf itself declares (every key under any `properties` object, including nested under `definitions`/`$defs`), for the reverse extra-field drift check. Throws if the file isn't valid JSON — callers must treat that as its own breach, not a crash. */
function policyFacetJsonFieldNames(content: string): string[] {
  const root: unknown = JSON.parse(content);
  const names = new Set<string>();
  const visit = (node: unknown): void => {
    if (!node || typeof node !== "object") return;
    if (Array.isArray(node)) {
      for (const item of node) visit(item);
      return;
    }
    const obj = node as Record<string, unknown>;
    if (obj.properties && typeof obj.properties === "object" && !Array.isArray(obj.properties)) {
      for (const key of Object.keys(obj.properties as Record<string, unknown>)) names.add(key);
    }
    for (const val of Object.values(obj)) visit(val);
  };
  visit(root);
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Field names a `.proto` sibling mirror leaf itself declares (numbered message fields, skipping enum/oneof/message/package/syntax/import lines), for the reverse extra-field drift check. */
function policyFacetProtoFieldNames(content: string): string[] {
  const bodyLines = content.split(/\r?\n/).filter((line) => !POLICY_FACET_MIRROR_DRIFT_PROTO_SKIP_RE.test(line));
  const body = bodyLines.join("\n");
  const names = new Set<string>();
  POLICY_FACET_MIRROR_DRIFT_PROTO_RE.lastIndex = 0;
  let m: RegExpExecArray | null;
  while ((m = POLICY_FACET_MIRROR_DRIFT_PROTO_RE.exec(body))) names.add(m[1]!);
  return [...names].map(policySnakeToCamel).filter(Boolean);
}

/** 🔎️Dispatches to the right per-language harvester by the sibling's own filename suffix. Returns `[]` for a suffix none of the harvesters own (defensive; every current entry in `POLICY_FACET_MIRROR_DRIFT_SIBLINGS` is covered). */
function policyFacetSiblingFieldNames(sib: string, content: string): string[] {
  if (sib.endsWith(".ts")) return policyFacetTsFieldNames(content);
  if (sib.endsWith(".graphql")) return policyFacetGraphqlFieldNames(content);
  if (sib.endsWith(".json")) return policyFacetJsonFieldNames(content);
  if (sib.endsWith(".proto")) return policyFacetProtoFieldNames(content);
  return [];
}
//#endregion 🔧️PolicyFacetMirrorDriftReverse

const POLICY_FACET_MIRROR_DRIFT_ALLOWLIST = new Set<string>([
  "stdio/bcf/standards#2.1-subsets-any-schema-diff-component",
  "stdio/bcf/standards#2.1-subsets-any-schema-mutations-component",
  "stdio/bcf/standards#2.1-subsets-any-schema-snapshot-component",
  "stdio/bmp/standards#v3-subsets-any-schema-diff-component",
  "stdio/bmp/standards#v3-subsets-any-schema-mutations-component",
  "stdio/bmp/standards#v3-subsets-any-schema-snapshot-component",
  "stdio/docx/standards#ecma-376-subsets-any-schema-diff-component",
  "stdio/docx/standards#ecma-376-subsets-any-schema-mutations-component",
  "stdio/docx/standards#ecma-376-subsets-any-schema-snapshot-component",
  "stdio/dwg/standards#ac1018-subsets-any-schema-diff-component",
  "stdio/dwg/standards#ac1018-subsets-any-schema-mutations-component",
  "stdio/dwg/standards#ac1018-subsets-any-schema-snapshot-component",
  "stdio/dwg/standards#ac1024-subsets-any-schema-diff-component",
  "stdio/dwg/standards#ac1024-subsets-any-schema-mutations-component",
  "stdio/dwg/standards#ac1024-subsets-any-schema-snapshot-component",
  "stdio/dxf/standards#r12-subsets-any-schema-diff-component",
  "stdio/dxf/standards#r12-subsets-any-schema-mutations-component",
  "stdio/dxf/standards#r12-subsets-any-schema-snapshot-component",
  "stdio/gif/standards#87a-subsets-any-schema-diff-component",
  "stdio/gif/standards#87a-subsets-any-schema-mutations-component",
  "stdio/gif/standards#87a-subsets-any-schema-snapshot-component",
  "stdio/gif/standards#89a-subsets-any-schema-diff-component",
  "stdio/gif/standards#89a-subsets-any-schema-mutations-component",
  "stdio/gif/standards#89a-subsets-any-schema-snapshot-component",
  "stdio/gltf/standards#2.0-subsets-any-schema-diff-component",
  "stdio/gltf/standards#2.0-subsets-any-schema-mutations-component",
  "stdio/gltf/standards#2.0-subsets-any-schema-snapshot-component",
  "stdio/ifc/standards#4-subsets-any-schema-diff-component",
  "stdio/ifc/standards#4-subsets-any-schema-mutations-component",
  "stdio/ifc/standards#4-subsets-any-schema-snapshot-component",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-diff-component",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-mutations-component",
  "stdio/jpg/standards#jfif-1.01-subsets-any-schema-snapshot-component",
  "stdio/las/standards#1.0-subsets-any-schema-diff-component",
  "stdio/las/standards#1.0-subsets-any-schema-mutations-component",
  "stdio/las/standards#1.0-subsets-any-schema-snapshot-component",
  "stdio/md/standards#commonmark-subsets-any-schema-diff-component",
  "stdio/md/standards#commonmark-subsets-any-schema-mutations-component",
  "stdio/md/standards#commonmark-subsets-any-schema-snapshot-component",
  "stdio/obj/standards#3.0-subsets-any-schema-diff-component",
  "stdio/obj/standards#3.0-subsets-any-schema-mutations-component",
  "stdio/obj/standards#3.0-subsets-any-schema-snapshot-component",
  "stdio/pdf/standards#1.4-subsets-any-schema-diff-component",
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-component",
  "stdio/pdf/standards#1.4-subsets-any-schema-snapshot-component",
  "stdio/pdf/standards#1.7-subsets-any-schema-diff-component",
  "stdio/pdf/standards#1.7-subsets-any-schema-mutations-component",
  "stdio/pdf/standards#1.7-subsets-any-schema-snapshot-component",
  "stdio/ply/standards#1.0-subsets-any-schema-diff-component",
  "stdio/ply/standards#1.0-subsets-any-schema-mutations-component",
  "stdio/ply/standards#1.0-subsets-any-schema-snapshot-component",
  "stdio/png/standards#1.2-subsets-any-schema-diff-component",
  "stdio/png/standards#1.2-subsets-any-schema-mutations-component",
  "stdio/png/standards#1.2-subsets-any-schema-snapshot-component",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-diff-component",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-mutations-component",
  "stdio/pptx/standards#ecma-376-subsets-any-schema-snapshot-component",
  "stdio/step/standards#ap214-subsets-any-schema-diff-component",
  "stdio/step/standards#ap214-subsets-any-schema-mutations-component",
  "stdio/step/standards#ap214-subsets-any-schema-snapshot-component",
  "stdio/stl/standards#ascii-subsets-any-schema-diff-component",
  "stdio/stl/standards#ascii-subsets-any-schema-mutations-component",
  "stdio/stl/standards#ascii-subsets-any-schema-snapshot-component",
  "stdio/svg/standards#1.1-subsets-any-schema-diff-component",
  "stdio/svg/standards#1.1-subsets-any-schema-mutations-component",
  "stdio/svg/standards#1.1-subsets-any-schema-snapshot-component",
  "stdio/tiff/standards#6.0-subsets-any-schema-diff-component",
  "stdio/tiff/standards#6.0-subsets-any-schema-mutations-component",
  "stdio/tiff/standards#6.0-subsets-any-schema-snapshot-component",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-diff-component",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-mutations-component",
  "stdio/xlsx/standards#ecma-376-subsets-any-schema-snapshot-component",
  "stdio/binary/standards#raw-subsets-any-schema-diff-component",
  "stdio/binary/standards#raw-subsets-any-schema-mutations-component",
  "stdio/binary/standards#raw-subsets-any-schema-snapshot-component",
  "stdio/csv/standards#rfc4180-subsets-any-schema-diff-component",
  "stdio/csv/standards#rfc4180-subsets-any-schema-mutations-component",
  "stdio/csv/standards#rfc4180-subsets-any-schema-snapshot-component",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-diff-component",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-mutations-component",
  "stdio/deflate/standards#rfc1950-subsets-any-schema-snapshot-component",
  "stdio/json/standards#rfc8259-subsets-any-schema-diff-component",
  "stdio/json/standards#rfc8259-subsets-any-schema-mutations-component",
  "stdio/json/standards#rfc8259-subsets-any-schema-snapshot-component",
  "stdio/txt/standards#utf-8-subsets-any-schema-diff-component",
  "stdio/txt/standards#utf-8-subsets-any-schema-mutations-component",
  "stdio/txt/standards#utf-8-subsets-any-schema-snapshot-component",
  "stdio/xml/standards#1.0-subsets-any-schema-diff-component",
  "stdio/xml/standards#1.0-subsets-any-schema-mutations-component",
  "stdio/xml/standards#1.0-subsets-any-schema-snapshot-component",
  "stdio/zip/standards#2.0-subsets-any-schema-diff-component",
  "stdio/zip/standards#2.0-subsets-any-schema-mutations-component",
  "stdio/zip/standards#2.0-subsets-any-schema-snapshot-component",
]);

function policyFacetMirrorDriftBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    for (const facet of POLICY_FACET_MIRROR_DRIFT_FACETS) {
      const facetRel = `${entry.subsetRel}/🧬️schema/${facet}`;
      const rustRel = `${facetRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      if (!existsSync(join(repoRoot, rustRel))) continue;
      const rustContent = policyReadFileSafe(repoRoot, rustRel);
      const camelFields = policyFacetRustFieldNames(rustContent);
      // 🩹 compareFields also folds in serde tag discriminants (`#[serde(tag = "kind")]`) and
      // camelCased enum variant names — real fields every non-Rust mirror must spell out even
      // though no Rust struct literally declares them as a named field (see
      // policyFacetRustTagFieldNames's and policyFacetRustVariantFieldNames's docstrings;
      // confirmed against stdio.json/png/bmp's real, handcrafted mirrors during this rule's own
      // verification).
      const compareFields = [...camelFields, ...policyFacetRustTagFieldNames(rustContent), ...policyFacetRustVariantFieldNames(rustContent)];
      const missingBySibling: string[] = [];
      for (const sib of POLICY_FACET_MIRROR_DRIFT_SIBLINGS) {
        const sibAbs = join(repoRoot, facetRel, sib);
        if (!existsSync(sibAbs)) {
          missingBySibling.push(`${sib}:MISSING_FILE`);
          continue;
        }
        const sibContent = readFileSync(sibAbs, "utf8");
        const missingFields = camelFields.filter((f) => !sibContent.includes(f));
        if (missingFields.length > 0) missingBySibling.push(`${sib}:${missingFields.length}`);
        let siblingFields: string[];
        try {
          siblingFields = policyFacetSiblingFieldNames(sib, sibContent);
        } catch {
          missingBySibling.push(`${sib}:PARSE_ERROR`);
          continue;
        }
        // 🩹 A sibling field counts as "real" if it exactly matches a Rust/tag field, OR visibly
        // CONTAINS one as a substring (e.g. GraphQL/proto disambiguate same-named enum-variant
        // fields with a type prefix — stdio.json's real mirror spells Rust's plain `value` field
        // as `boolValue`/`stringValue` per variant — mirroring the forward check's own substring
        // leniency, just in the other direction). Case-INSENSITIVE on purpose: the disambiguating
        // prefix capitalizes the embedded real field's first letter (`arrayItems`, not
        // `arrayitems`), so a case-sensitive substring test would miss it entirely. The length
        // floor keeps this from rubber-stamping every sibling field against a trivially short real
        // field name.
        const extraFields = siblingFields.filter((f) => {
          if (f === "schema" || compareFields.includes(f)) return false;
          const fLower = f.toLowerCase();
          return !compareFields.some((cf) => cf.length >= POLICY_FACET_MIRROR_DRIFT_SUBSTRING_MIN_LEN && fLower.includes(cf.toLowerCase()));
        });
        if (extraFields.length > 0) missingBySibling.push(`${sib}:extra:${extraFields.length}`);
      }
      const normalized = policyNormalizeRelPath(rustRel);
      const allowlisted = POLICY_FACET_MIRROR_DRIFT_ALLOWLIST.has(normalized);
      if (missingBySibling.length > 0) {
        if (allowlisted) continue;
        breaches.push({
          id: `facet-mirror-drift-${rustRel}`,
          summary: `"${facetRel}" siblings drift from the Rust leaf's real fields — missing and/or extra (${missingBySibling.join(", ")})`,
          kind: "stdio-artifacts/facet-mirror-drift",
          scope: entry.artRel,
          priority: "low",
          reason: "Every camelCased field the Rust leaf declares should also appear in its sibling .ts/.graphql/.json/.proto leaves, AND every field a sibling declares should correspond to a real Rust field — a stale/copy-pasted mirror (missing real fields) or a leftover generic scaffold (extra fields with no Rust counterpart, e.g. entries/key/value) both silently drift from the real shape otherwise (see the master plan's gif-TS-mirror-is-literally-zip's finding).",
          solution: `Rewrite ${facetRel}'s sibling leaves to mirror the Rust leaf's real fields exactly (no missing, no extra), or if this artifact/facet hasn't been reached yet, add "${normalized}" to POLICY_FACET_MIRROR_DRIFT_ALLOWLIST citing this ticket.`,
        });
      } else if (allowlisted) {
        breaches.push({
          id: `facet-mirror-drift-stale-${rustRel}`,
          summary: `"${facetRel}" is allowlisted in POLICY_FACET_MIRROR_DRIFT_ALLOWLIST but its siblings no longer drift`,
          kind: "stdio-artifacts/facet-mirror-drift",
          scope: entry.artRel,
          priority: "low",
          reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
          solution: `Remove "${normalized}" from POLICY_FACET_MIRROR_DRIFT_ALLOWLIST.`,
        });
      }
    }
  }
  return breaches;
}

/** ⚖️Aggregates S2's S-7 ban + all four S-8 drift/honesty/coverage rules. */
export function policySchemaOverhaulS2Breaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyStdioVcsMachineryBanBreaches(repoRoot),
    ...policyFacetMirrorDriftBreaches(repoRoot),
    ...policyGrammarHonestyBreaches(repoRoot),
    ...policyDiffAlgebraBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleSchemaOverhaulS2
