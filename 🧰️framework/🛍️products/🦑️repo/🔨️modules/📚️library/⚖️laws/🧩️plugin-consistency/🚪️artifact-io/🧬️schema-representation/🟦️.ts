import { existsSync } from "node:fs";
import { join } from "node:path";
import { POLICY_STDIO_BINARY_SPEC_LEAVES, POLICY_STDIO_FACET_BINARY, POLICY_STDIO_FACET_TEXT, POLICY_STDIO_TEXT_SPEC_LEAVES, policyStdioRepresentationDirs, policyStdioSchemaChildDirs } from "../📋️owner-table/🟦️.ts";
import { canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { schemaFacetFormatEntries } from "../../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts";
import { policyArtifactIsMigrated } from "../🗂️stdio-catalog/🟦️.ts";
import { policyListArtifactDialectDirs } from "../../../../🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts";
import { policyReaddirSafe, policyReadFileSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF } from "../../✍️handcrafted-mutation/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, POLICY_TS_COMPONENT_LEAF } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { policyListPluginArtifactDirs } from "../../../../🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts";

function policySchemaRepresentationLeavesFor(repDir: string): readonly string[] {
  if (repDir === POLICY_STDIO_FACET_TEXT) return POLICY_STDIO_TEXT_SPEC_LEAVES;
  if (repDir === POLICY_STDIO_FACET_BINARY) return POLICY_STDIO_BINARY_SPEC_LEAVES;
  return [];
}

function policySchemaFormatLeafBreaches(
  repoRoot: string,
  facetAbs: string,
  artRel: string,
  taxonomy: ReturnType<typeof loadTaxonomy>,
): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const [formatId, format] of schemaFacetFormatEntries(facetAbs, taxonomy)) {
    const leafFilename = canonicalPrimaryFilenameForKind(format.fileKindId, taxonomy);
    const leafRel = `${facetAbs}/${leafFilename}`;
    if (existsSync(join(repoRoot, leafRel))) continue;
    breaches.push({
      id: `stdio-schema-format-${leafRel}`,
      summary: `"${facetAbs}" is missing schemaFormats leaf ${leafFilename} (${formatId})`,
      kind: "stdio-artifacts/schema-representation",
      scope: artRel,
      priority: "high",
      reason: "Each schema node carries all schemaFormats leaves for its facet kind from 🔣️taxonomy.json.",
      solution: `Add ${leafRel}.`,
    });
  }
  return breaches;
}

/** 🏅 One (scope, schema-root) pair per artifact: the flat artifact-root root for an unmigrated
 * artifact, or one subset-relative root per (standard, subset) dialect for a migrated one. */
function policyArtifactSchemaRoots(repoRoot: string, artRel: string, schemaFacet: string): { scope: string; schemaRoot: string }[] {
  if (!policyArtifactIsMigrated(repoRoot, artRel)) {
    return [{ scope: artRel, schemaRoot: `${artRel}/${schemaFacet}` }];
  }
  return policyListArtifactDialectDirs(repoRoot)
    .filter((d) => d.artRel === artRel)
    .map((d) => ({ scope: artRel, schemaRoot: `${d.subsetRel}/${schemaFacet}` }));
}

/** 🔎️True when `schemaRoot` is exactly the delegating re-export pair — `🦀️.rs` (`pub use
 * …::any::schema::*;`) + `🟦️.ts` (either a literal `export * from ".../✳️any/.../🟦️component"`
 * re-export, or a `meta` stamp with no own `interface`/`type`/`enum` — both shapes are attested on
 * disk, ifc ✳️cv20/✳️sav/✳️cobie use the former, pdf/step/zip's conformance subsets use the latter)
 * and NOTHING else. A subset in this shape is a validation-gated conformance STAMP on top of its
 * standard's schema-owning subset — it never duplicates the schema, so it never carries facet
 * mirrors/grammar leaves/diff/mutations trees. Structural, not name-based: a delegating subset
 * never needs an allowlist/filter edit. */
function policySchemaIsDelegatingPair(repoRoot: string, schemaRoot: string): boolean {
  const entries = policyReaddirSafe(repoRoot, schemaRoot);
  if (entries.length !== 2) return false;
  if (entries.some((e) => e.isDirectory)) return false;
  const names = new Set(entries.map((e) => e.name));
  if (!names.has(POLICY_RS_COMPONENT_LEAF) || !names.has(POLICY_TS_COMPONENT_LEAF)) return false;
  const rs = policyReadFileSafe(repoRoot, schemaRoot, POLICY_RS_COMPONENT_LEAF);
  const ts = policyReadFileSafe(repoRoot, schemaRoot, POLICY_TS_COMPONENT_LEAF);
  const rsReexports = /pub\s+use\s+[\w:]+::any::schema::\*\s*;/.test(policyStripRustCommentsAndStrings(rs));
  const tsHasOwnType = /export\s+(interface|type|enum)\s+\w+/.test(ts);
  if (tsHasOwnType) return false;
  const tsLiteralReexport = /export\s+\*\s+from\s+["'][^"']*✳️any\/🧬️schema\/🟦️component["']\s*;/.test(ts);
  const tsHasMetaStamp = /export\s+const\s+meta\s*=\s*\{[^}]*artifactKind\s*:[^}]*\}/.test(ts);
  return rsReexports && (tsLiteralReexport || tsHasMetaStamp);
}

/** 🏅 True when `schemaRoot` owns its schema (has a real `📸️snapshot/` on disk) — the generalized
 * form of `policyListStdioSchemaOwningEntries`'s filter, inlined here because this rule also covers
 * unmigrated (pre-dialect) artifacts, which are always schema-owning by construction (no subset
 * concept to delegate from). See `policySchemaIsDelegatingPair` for the sibling shape. */
function policySchemaRootIsOwning(repoRoot: string, schemaRoot: string, migrated: boolean): boolean {
  if (!migrated) return true;
  return existsSync(join(repoRoot, schemaRoot, "📸️snapshot"));
}

/** ⚖️ Validates pure semantic schema declarations and native representation specifications owned by I/O. */
export function policySchemaRepresentationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const taxonomy = loadTaxonomy();
  const schemaChildDirs = policyStdioSchemaChildDirs(taxonomy);
  const representationDirs = policyStdioRepresentationDirs(taxonomy);
  const schemaFacet = "🧬️schema";
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
   const migrated = policyArtifactIsMigrated(repoRoot, artRel);
   for (const { schemaRoot } of policyArtifactSchemaRoots(repoRoot, artRel, schemaFacet)) {
    if (!existsSync(join(repoRoot, schemaRoot))) {
      breaches.push({
        id: `stdio-schema-root-${artRel}`,
        summary: `"${artRel}" is missing ${schemaFacet}/`,
        kind: "stdio-artifacts/schema-representation",
        scope: artRel,
        priority: "high",
        reason: "Artifact-level schema facet is required; snapshot/diff/mutations nest beneath it.",
        solution: `Create ${schemaRoot}/ per normative spec §1.`,
      });
      continue;
    }
    if (!policySchemaRootIsOwning(repoRoot, schemaRoot, migrated)) {
      if (policySchemaIsDelegatingPair(repoRoot, schemaRoot)) continue;
      breaches.push({
        id: `stdio-schema-delegating-${schemaRoot}`,
        summary: `"${schemaRoot}" is neither schema-owning (no 📸️snapshot/) nor a valid delegating re-export pair`,
        kind: "stdio-artifacts/schema-representation",
        scope: artRel,
        priority: "high",
        reason: ("A delegating subset (validation-gated conformance stamp on an existing schema-owning subset, e.g. step ✳️cc1, pdf ✳️a, zip ✳️iso21320) carries ONLY " + POLICY_RS_COMPONENT_LEAF_NAME + " (`pub use …::any::schema::*;`) + " + POLICY_TS_COMPONENT_LEAF + " (a meta stamp) — no facet mirrors, grammar leaves, or diff/mutations trees."),
        solution: `Replace ${schemaRoot}'s contents with exactly ${POLICY_RS_COMPONENT_LEAF} (re-exporting the standard's schema-owning subset) + ${POLICY_TS_COMPONENT_LEAF} (a meta stamp), or add its own 📸️snapshot/ to make it schema-owning and satisfy the full tree below instead.`,
      });
      continue;
    }
    breaches.push(...policySchemaFormatLeafBreaches(repoRoot, schemaRoot, artRel, taxonomy));
    for (const child of schemaChildDirs) {
      const childAbs = `${schemaRoot}/${child}`;
      if (!existsSync(join(repoRoot, childAbs))) {
        breaches.push({
          id: `stdio-schema-child-${childAbs}`,
          summary: `"${schemaRoot}" is missing child ${child}/`,
          kind: "stdio-artifacts/schema-representation",
          scope: artRel,
          priority: "high",
          reason: `taxonomy.schemaChildDirs requires ${child} under every 🧬️schema facet.`,
          solution: `Add ${childAbs}/ with pure semantic declarations and schemaFormats leaves.`,
        });
        continue;
      }
      breaches.push(...policySchemaFormatLeafBreaches(repoRoot, childAbs, artRel, taxonomy));
      for (const rep of representationDirs) {
        const repAbs = `${schemaRoot.slice(0, -schemaFacet.length)}🚪️io/${rep}/${child}`;
        if (!existsSync(join(repoRoot, repAbs))) {
          if (child === "🧬️mutations" || child === "💡️inferences") continue;
          breaches.push({ id: `artifact-io-rep-${repAbs}`, summary: `"${repAbs}" is missing a native codec facet`, kind: "artifact-io/representation", scope: artRel, priority: "high", reason: "Snapshot and diff native wire codecs belong below an I/O representation.", solution: `Add ${repAbs}/ with normative spec leaves.` });
          continue;
        }
        if ((child === "🧬️mutations" || child === "💡️inferences") && policyReaddirSafe(repoRoot, repAbs).some(entry => entry.isDirectory || entry.name === "📝️.md")) continue;
        for (const leaf of policySchemaRepresentationLeavesFor(rep)) {
          const leafRel = `${repAbs}/${leaf}`;
          if (existsSync(join(repoRoot, leafRel))) continue;
          breaches.push({ id: `artifact-io-leaf-${leafRel}`, summary: `"${repAbs}" is missing spec leaf ${leaf}`, kind: "artifact-io/representation", scope: artRel, priority: "high", reason: "Wire codec specifications belong to their I/O representation and semantic facet.", solution: `Add ${leafRel}.` });
        }
      }
    }
   }
  }
  return breaches;
}
