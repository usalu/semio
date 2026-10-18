import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF } from "../../✍️handcrafted-mutation/🟦️.ts";
import { POLICY_TS_COMPONENT_LEAF } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyReadFileSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_STDIO_ARTIFACTS_REL, POLICY_STDIO_FACET_DECOMPOSER, POLICY_STDIO_OWNER_TABLE_REL, POLICY_STDIO_PLUGIN_REL, policyLoadStdioOwnerTable, policyRustFileHasRealTraitImpl, policyStdioArtifactFacets } from "../📋️owner-table/🟦️.ts";
import { loadTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { POLICY_STANDARDS_DIR, policyListArtifactDialectDirs } from "../../../../🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts";
import { policyListPluginArtifactDirs } from "../../../../🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts";

function policyStdioFacetRsTsBreaches(
  repoRoot: string,
  facetRel: string,
  scope: string,
  kind: string,
  traitName: string,
): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const rsRel = `${facetRel}/${POLICY_RS_COMPONENT_LEAF}`;
  const tsRel = `${facetRel}/${POLICY_TS_COMPONENT_LEAF}`;
  if (!existsSync(join(repoRoot, facetRel))) {
    breaches.push({
      id: `${kind}-missing-${facetRel}`,
      summary: `"${scope}" is missing ${facetRel.split("/").pop()}/`,
      kind,
      scope,
      priority: "high",
      reason: `Every artifact must expose ${facetRel.split("/").pop()} with Rust and TypeScript taxonomy leaves.`,
      solution: `Create ${facetRel}/ with ${POLICY_RS_COMPONENT_LEAF} and ${POLICY_TS_COMPONENT_LEAF}.`,
    });
    return breaches;
  }
  if (!existsSync(join(repoRoot, rsRel))) {
    breaches.push({
      id: `${kind}-rs-${facetRel}`,
      summary: `"${rsRel}" is missing`,
      kind,
      scope,
      priority: "high",
      reason: `Facet ${facetRel} must declare ${traitName} in ${POLICY_RS_COMPONENT_LEAF}.`,
      solution: `Add ${rsRel} implementing ${traitName}.`,
    });
  } else {
    const body = policyReadFileSafe(repoRoot, rsRel);
    if (!policyRustFileHasRealTraitImpl(body, traitName)) {
      breaches.push({
        id: `${kind}-trait-${facetRel}`,
        summary: `"${rsRel}" has no real "impl ${traitName} for ..." block (comment/string-stripped)`,
        kind,
        scope,
        priority: "high",
        reason: `${POLICY_RS_COMPONENT_LEAF} must implement the SDK ${traitName} trait with a real impl block, not just mention its name in a comment or string.`,
        solution: `Add a real "impl ${traitName} for <YourType>" block in ${rsRel}.`,
      });
    }
  }
  if (!existsSync(join(repoRoot, tsRel))) {
    breaches.push({
      id: `${kind}-ts-${facetRel}`,
      summary: `"${tsRel}" is missing`,
      kind,
      scope,
      priority: "high",
      reason: `Facet ${facetRel} must re-export ${traitName} from the TypeScript barrel leaf.`,
      solution: `Add ${tsRel} exporting ${traitName}.`,
    });
  }
  return breaches;
}

/** ⚖️Thirty-six stdio codec artifacts exist under 🗄️stdio with required completeness facets. */
export function policyStdioCatalogBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) {
    breaches.push({
      id: "stdio-catalog-owner-table-missing",
      summary: `owner table missing at ${POLICY_STDIO_OWNER_TABLE_REL}`,
      kind: "stdio-artifacts/catalog",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Stdio roster and DAG are normative in the ticket owner table until taxonomy absorbs them.",
      solution: `Restore ${POLICY_STDIO_OWNER_TABLE_REL}.`,
    });
    return breaches;
  }
  const expectedCount = table.counts?.stdio_artifacts ?? 36;
  const rosterIds = Object.keys(table.artifacts);
  if (rosterIds.length !== expectedCount) {
    breaches.push({
      id: "stdio-catalog-roster-count",
      summary: `schema-derived artifact definitions have ${rosterIds.length} entries but normative count is ${expectedCount}`,
      kind: "stdio-artifacts/catalog",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "The closed stdio catalog must list exactly 36 format artifacts.",
      solution: `Fix schema-owned definitions under ${POLICY_STDIO_OWNER_TABLE_REL}.`,
    });
  }
  const pluginRoot = join(repoRoot, POLICY_STDIO_PLUGIN_REL);
  if (!existsSync(pluginRoot)) {
    breaches.push({
      id: "stdio-catalog-plugin-missing",
      summary: `${POLICY_STDIO_PLUGIN_REL} plugin root is missing`,
      kind: "stdio-artifacts/catalog",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Stdio codecs live in the dedicated 🗄️stdio plugin (zero apps).",
      solution: `Scaffold ${POLICY_STDIO_PLUGIN_REL} per ticket W2.`,
    });
    return breaches;
  }
  const taxonomy = loadTaxonomy();
  const requiredFacets = policyStdioArtifactFacets(taxonomy);
  for (const formatId of rosterIds) {
    const entry = table.artifacts[formatId]!;
    const artRel = `${POLICY_STDIO_ARTIFACTS_REL}/${entry.dir}`;
    const scope = `🗄️stdio/${entry.dir}`;
    if (!existsSync(join(repoRoot, artRel))) {
      breaches.push({
        id: `stdio-catalog-artifact-${formatId}`,
        summary: `stdio artifact "${formatId}" missing at ${artRel}`,
        kind: "stdio-artifacts/catalog",
        scope,
        priority: "high",
        reason: "Every stdio roster id must materialize as an artifact directory under 🗄️stdio.",
        solution: `Create ${artRel}/ with builder, decomposer, schema, engine, and io facets.`,
      });
      continue;
    }
    if (policyArtifactIsMigrated(repoRoot, artRel)) {
      // Migrated: the flat schema/engine/io/builder/decomposer facets are gone by design --
      // policyStandardsCoverageBreaches (+ the Builder/Analyzer/Composer migrated rules) own
      // the deep check under 🏅️standards/ now.
      continue;
    }
    for (const facet of requiredFacets) {
      const facetRel = `${artRel}/${facet}`;
      if (existsSync(join(repoRoot, facetRel))) continue;
      breaches.push({
        id: `stdio-catalog-facet-${formatId}-${facet}`,
        summary: `"${artRel}" is missing required facet ${facet}/`,
        kind: "stdio-artifacts/catalog",
        scope,
        priority: "high",
        reason: "Stdio codec artifacts carry the same completeness facets as domain artifacts.",
        solution: `Add ${facetRel}/ per normative spec §2.`,
      });
    }
  }
  return breaches;
}

/** 🏅 True once an artifact has grown a 🏅️standards/ child -- see 🏅️PolicyRuleStandardsSubsets below.
 * The seven original rules skip migrated artifacts entirely; the new standards/subsets rules own them. */
export function policyArtifactIsMigrated(repoRoot: string, artRel: string): boolean {
  const taxonomy = loadTaxonomy();
  const standardsDirName = (taxonomy as any).standardsDirName ?? POLICY_STANDARDS_DIR;
  return existsSync(join(repoRoot, `${artRel}/${standardsDirName}`));
}

/** ⚖️Legacy artifacts no longer own an explicit builder facet. */
export function policyArtifactBuilderBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    if (policyArtifactIsMigrated(repoRoot, artRel)) continue;
    const facetRel = `${artRel}/🏗️builder`;
    if (!existsSync(join(repoRoot, facetRel))) continue;
    breaches.push({
      id: `artifact-derived-builder-${artRel}`,
      summary: `"${artRel}" has a forbidden explicit builder facet`,
      kind: "stdio-artifacts/derived-facets",
      scope: artRel,
      priority: "high",
      reason: "Artifact lifecycle capabilities are derived from schema and IO hooks.",
      solution: `Remove ${facetRel}/ and derive its builder from the subset schema.`,
    });
  }
  return breaches;
}

/** ⚖️Every not-yet-migrated plugin artifact exposes 🪓️decomposer with rs+ts implementing ArtifactDecomposer. */
export function policyArtifactDecomposerBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    if (policyArtifactIsMigrated(repoRoot, artRel)) continue;
    breaches.push(
      ...policyStdioFacetRsTsBreaches(
        repoRoot,
        `${artRel}/${POLICY_STDIO_FACET_DECOMPOSER}`,
        artRel,
        "stdio-artifacts/decomposer",
        "ArtifactDecomposer",
      ),
    );
  }
  return breaches;
}

const POLICY_DERIVED_FACETS = [
  { dir: "🏗️builder", hook: "construction:", name: "builder" },
  { dir: "🧐️analyzer", hook: "analysis:", name: "analyzer" },
  { dir: "🎹️composer", hook: "composition:", name: "composer" },
] as const;

function policyDerivedArtifactFacetBreaches(repoRoot: string, facet: (typeof POLICY_DERIVED_FACETS)[number]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const checkedOwners = new Set<string>();
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    for (const owner of [dialect.artRel, dialect.standardRel, dialect.subsetRel]) {
      if (checkedOwners.has(owner)) continue;
      checkedOwners.add(owner);
      const explicitRel = `${owner}/${facet.dir}`;
      if (!existsSync(join(repoRoot, explicitRel))) continue;
      breaches.push({
        id: `artifact-derived-${facet.name}-explicit-${owner}`,
        summary: `"${owner}" has forbidden explicit ${facet.name} facet ${facet.dir}/`,
        kind: "stdio-artifacts/derived-facets",
        scope: dialect.artRel,
        priority: "high",
        reason: "Builder, analyzer, and composer are derived types, not artifact taxonomy nodes.",
        solution: `Remove ${explicitRel}/ and keep the ${facet.name} hook in schema or IO.`,
      });
    }
    const schemaRel = `${dialect.subsetRel}/🧬️schema/${POLICY_RS_COMPONENT_LEAF}`;
    const schemaAbs = join(repoRoot, schemaRel);
    const body = existsSync(schemaAbs) ? readFileSync(schemaAbs, "utf8") : "";
    if (body.includes("derive_artifact_facets!") && body.includes(facet.hook)) continue;
    breaches.push({
      id: `artifact-derived-${facet.name}-missing-${dialect.subsetRel}`,
      summary: `"${schemaRel}" does not derive its ${facet.name}`,
      kind: "stdio-artifacts/derived-facets",
      scope: dialect.artRel,
      priority: "high",
      reason: "Every subset schema derives the uniform lifecycle types from construction, analysis, and composition hooks.",
      solution: `Invoke derive_artifact_facets! in ${schemaRel} with a ${facet.hook} hook.`,
    });
  }
  return breaches;
}

/** ⚖️Every migrated subset derives its builder from schema-owned construction. */
export function policyArtifactBuilderMigratedBreaches(repoRoot: string): BreachRecord[] {
  return policyDerivedArtifactFacetBreaches(repoRoot, POLICY_DERIVED_FACETS[0]);
}

/** ⚖️Every migrated subset derives its analyzer from schema-owned analysis. */
export function policyArtifactAnalyzerBreaches(repoRoot: string): BreachRecord[] {
  return policyDerivedArtifactFacetBreaches(repoRoot, POLICY_DERIVED_FACETS[1]);
}

/** ⚖️Every migrated subset derives its composer from IO-owned composition. */
export function policyArtifactComposerBreaches(repoRoot: string): BreachRecord[] {
  return policyDerivedArtifactFacetBreaches(repoRoot, POLICY_DERIVED_FACETS[2]);
}
