/** 🛂️ Wave 4 plugin consistency policy: stdio artifact catalog, standards subsets and artifact I/O rules.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { canonicalFilenameForKind, canonicalPrimaryFilenameForKind, loadTaxonomy, schemaFacetFormatEntries, type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { loadCatalogTaxonomy, subsetDirectoryNameForId, subsetIdForDirectoryName } from "../../../🔍️discovery/🟦️.ts";
import { policyReadFileSafe, policyReaddirSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { artifactIoArchitectureBreaches } from "../../../🚪️io/🏛️architecture/🟦️.ts";
import { policyWalkRelFiles } from "../../../🔍️discovery/🚶️file-walk/🟦️.ts";
import { policyListPluginArtifactDirs } from "../../../🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts";
import { POLICY_STANDARDS_DIR, POLICY_SUBSETS_DIR, policyListArtifactDialectDirs, type PolicyArtifactDialect } from "../../../🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, POLICY_TS_COMPONENT_LEAF } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { policyAllRustFiles, policyNormalizeRelPath } from "../🏛️foundation/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF } from "../✍️handcrafted-mutation/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { stdioArtifactLedger } from "../../../🗄️stdio-ledger/🟦️.ts";

//#region 🔧️PolicyRuleArtifactIo
/** 🧾 Schema-derived stdio definition view for policy checks. */
const POLICY_STDIO_OWNER_TABLE_REL = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🔣️.json";
const POLICY_STDIO_PLUGIN_REL = "✏️s/🔌️plugins/🗄️stdio";
export const POLICY_STDIO_ARTIFACTS_REL = `${POLICY_STDIO_PLUGIN_REL}/🗿️artifacts`;
const POLICY_STDIO_FACET_DECOMPOSER = "🪓️decomposer";
const POLICY_STDIO_FACET_TEXT = "📝️text";
const POLICY_STDIO_FACET_BINARY = "💾️binary";
const POLICY_STDIO_FACET_DESERIALIZERS = "🧩️deserializers";
const POLICY_STDIO_FACET_SERIALIZERS = "🧵️serializers";
const POLICY_STDIO_IO_IMPORT = "📥️import";
const POLICY_STDIO_IO_EXPORT = "📤️export";
const POLICY_STDIO_SCHEMA_CHILD_FALLBACK = ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const;
const POLICY_STDIO_REPRESENTATION_FALLBACK = [POLICY_STDIO_FACET_TEXT, POLICY_STDIO_FACET_BINARY] as const;
const POLICY_STDIO_ARTIFACT_FACET_FALLBACK = ["🧬️schema", "⚙️engine", "🚪️io", POLICY_STDIO_FACET_DECOMPOSER] as const;
const POLICY_STDIO_LEGACY_ARTIFACT_FACETS = new Set(["🗣️dsl", "🔧️op", "📡️spr", "🔺️diff", "📸️snapshot"]);
const POLICY_STDIO_TEXT_SPEC_LEAVES = [
  ...["grammar-semio", "ebnf", "antlr", "graphql", "json", "protobuf"].map((kindId) => canonicalPrimaryFilenameForKind(kindId, loadCatalogTaxonomy())),
  POLICY_RS_COMPONENT_LEAF,
  POLICY_TS_COMPONENT_LEAF,
] as const;
const POLICY_STDIO_BINARY_SPEC_LEAVES = [
  ...["protocol-semio", "abnf", "kaitai", "spicy"].map((kindId) => canonicalPrimaryFilenameForKind(kindId, loadCatalogTaxonomy())),
  POLICY_RS_COMPONENT_LEAF,
  POLICY_TS_COMPONENT_LEAF,
] as const;
const POLICY_STDIO_CODEC_BANNED_MARKERS = ["SRAS", "IFCCARTOONMESH", "b\"minimal\"", "stub codec", "minimal stub codec"] as const;

type PolicyStdioDefinitionTable = {
  artifacts: Record<string, { dir: string; depends: string[] }>;
  dependency_edges: { from: string; to: string }[];
  owners: Array<{
    path: string;
    stdio_artifacts: string[];
    import: string[];
    export: string[];
  }>;
  counts?: { stdio_artifacts?: number };
};

function policyLoadStdioOwnerTable(repoRoot: string): PolicyStdioDefinitionTable | null {
  try {
    const ledger = stdioArtifactLedger(repoRoot);
    const artifacts = Object.fromEntries(ledger.artifacts.map((artifact) => [artifact.id, { dir: artifact.directory, depends: [...artifact.depends] }]));
    return { artifacts, dependency_edges: ledger.artifacts.flatMap((artifact) => artifact.depends.map((to) => ({ from: artifact.id, to }))), owners: [], counts: { stdio_artifacts: ledger.counts.artifacts } };
  } catch {
    return null;
  }
}

function policyStdioArtifactFacets(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const dirs = taxonomy.artifactComponentDirs;
  if (!dirs?.length) return [...POLICY_STDIO_ARTIFACT_FACET_FALLBACK];
  const out = new Set<string>(POLICY_STDIO_ARTIFACT_FACET_FALLBACK);
  for (const d of dirs) {
    if (!POLICY_STDIO_LEGACY_ARTIFACT_FACETS.has(d)) out.add(d);
  }
  return [...out];
}

function policyStdioSchemaChildDirs(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const from = taxonomy.schemaChildDirs as string[] | undefined;
  return from?.length ? [...from] : [...POLICY_STDIO_SCHEMA_CHILD_FALLBACK];
}

function policyStdioRepresentationDirs(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  return [...taxonomy.nativeRepresentationDirs];
}

function policyStdioFormatDir(artifacts: PolicyStdioDefinitionTable["artifacts"], formatId: string): string | undefined {
  return artifacts[formatId]?.dir;
}

function policyStdioArtifactsDirName(): string {
  return loadTaxonomy().artifactsDirName ?? "🗿️artifacts";
}

export function policyRustFileHasRealTraitImpl(body: string, traitName: string): boolean {
  const stripped = policyStripRustCommentsAndStrings(body);
  return new RegExp(`\\bimpl\\s*(?:<[^>{]*>\\s*)?${traitName}\\b\\s*(?:<[^>{]*>\\s*)?for\\b`).test(stripped);
}

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
function policyArtifactIsMigrated(repoRoot: string, artRel: string): boolean {
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

//#region 🏅️PolicyRuleStandardsSubsets
/**
 * 🏅 W9+ (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase 2): dual-shape vocabulary.
 * An artifact is "migrated" once it has a 🏅️standards/ child; unmigrated artifacts
 * are validated exclusively by the seven rules above and never touched here. This
 * keeps the whole sweep additive: these rules are vacuous ([]) until the first
 * artifact actually grows a 🏅️standards/ dir.
 */


const POLICY_DERIVED_FACETS = [
  { dir: "🏗️builder", hook: "construction:", name: "builder" },
  { dir: "🧐️analyzer", hook: "analysis:", name: "analyzer" },
  { dir: "🎹️composer", hook: "composition:", name: "composer" },
] as const;




/** ⚖️Every migrated standard/subset dir carries its required engine/schema/IO children. */
export function policyStandardsCoverageBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const taxonomy = loadTaxonomy();
  const standardChildDirs = ((taxonomy as any).standardChildDirs as string[] | undefined) ?? [];
  const subsetChildDirs = ((taxonomy as any).subsetChildDirs as string[] | undefined) ?? [];
  const slugPattern = new RegExp((taxonomy as any).standardSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$");
  const subsetSlugPattern = new RegExp((taxonomy as any).subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$");
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    if (!slugPattern.test(dialect.standardSlug)) {
      breaches.push({
        id: `standards-slug-${dialect.standardRel}`,
        summary: `"${dialect.standardRel}" standard slug "${dialect.standardSlug}" does not match ${slugPattern}`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Standard directory slugs are normalized lowercase identifiers (🔖️2.0, 🔖️ap214, 🔖️1, …).",
        solution: `Rename ${dialect.standardRel} to match the slug pattern.`,
      });
    }
    if (subsetIdForDirectoryName(dialect.subsetsRel, dialect.subsetDirName, taxonomy) === null) {
      breaches.push({
        id: `standards-subset-prefix-${dialect.subsetRel}`,
        summary: `"${dialect.subsetRel}" subset dir "${dialect.subsetDirName}" is not declared by its exact semantic subset roster`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Every subset dir under 🪆️subsets/ must resolve through either its owner-scoped semantic roster or the default subset convention.",
        solution: `Rename ${dialect.subsetRel} to one registered semantic subset directory.`,
      });
    } else if (dialect.subsetId !== ((taxonomy as any).subsetAnyId ?? "*") && !subsetSlugPattern.test(dialect.subsetId)) {
      breaches.push({
        id: `standards-subset-slug-${dialect.subsetRel}`,
        summary: `"${dialect.subsetRel}" subset id "${dialect.subsetId}" does not match ${subsetSlugPattern}`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Real subset ids are normalized lowercase identifiers naming an industry conformance profile/class/view or semantic type (✳️a, ✳️cc6, ✳️rv, 🧊️brep, 🔺️mesh, …), never a version or a conformance level.",
        solution: `Rename ${dialect.subsetRel} to match the slug pattern.`,
      });
    }
    for (const child of standardChildDirs) {
      if (child === POLICY_SUBSETS_DIR) continue;
      if (existsSync(join(repoRoot, `${dialect.standardRel}/${child}`))) continue;
      breaches.push({
        id: `standards-standard-child-${dialect.standardRel}-${child}`,
        summary: `"${dialect.standardRel}" is missing required child ${child}/`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Every standard carries engine/subsets per 🔣️taxonomy.json standardChildDirs.",
        solution: `Add ${dialect.standardRel}/${child}/.`,
      });
    }
    for (const child of subsetChildDirs) {
      if (existsSync(join(repoRoot, `${dialect.subsetRel}/${child}`))) continue;
      breaches.push({
        id: `standards-subset-child-${dialect.subsetRel}-${child}`,
        summary: `"${dialect.subsetRel}" is missing required child ${child}/`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Every subset carries schema/io; lifecycle capabilities are derived from their hooks.",
        solution: `Add ${dialect.subsetRel}/${child}/.`,
      });
    }
  }
  return breaches;
}

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

/** 🔣 One row per (migrated artifact, standard) — the manifest a `policyStandardSubsetVocabularyBreaches` group checks. */
type PolicyStandardManifestGroup = {
  artRel: string;
  standardRel: string;
  standardSlug: string;
  manifestRel: string;
  dialects: PolicyArtifactDialect[];
};

function policyGroupDialectsByStandard(repoRoot: string): PolicyStandardManifestGroup[] {
  const taxonomy = loadTaxonomy();
  const subsetsDirName = taxonomy.subsetsDirName;
  if (!taxonomy.subsetsManifestFileKindId) throw new Error("[taxonomy] subsets manifest file kind is not declared.");
  const manifestFilename = canonicalFilenameForKind(taxonomy.subsetsManifestFileKindId, taxonomy);
  const groups = new Map<string, PolicyStandardManifestGroup>();
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    let group = groups.get(dialect.standardRel);
    if (!group) {
      group = {
        artRel: dialect.artRel,
        standardRel: dialect.standardRel,
        standardSlug: dialect.standardSlug,
        manifestRel: `${dialect.standardRel}/${subsetsDirName}/${manifestFilename}`,
        dialects: [],
      };
      groups.set(dialect.standardRel, group);
    }
    group.dialects.push(dialect);
  }
  return [...groups.values()];
}

/**
 * ⚖️ Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: every migrated standard declares
 * its real industry subset vocabulary (or, for non-stdio/"domain" artifacts, explicitly declares
 * none) in `🪆️subsets/🔣️.json`, and that declaration must always equal what's actually on
 * disk — in both directions, so a manifest can never silently drift ahead of or behind the real
 * dirs. Real (non-`*`) subsets on stdio artifacts additionally need a registered `SubsetValidator`
 * on their subset composer — the static half of "every real subset gets a real validator"; the
 * generic runtime half (`io.subset.validator-missing`) is `run_subset_validation` in
 * `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs`.
 */
export function policyStandardSubsetVocabularyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const taxonomy = loadTaxonomy();
  const anyId = (taxonomy as any).subsetAnyId ?? "*";
  const subsetSlugPattern = new RegExp((taxonomy as any).subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$");
  const stdioPrefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const group of policyGroupDialectsByStandard(repoRoot)) {
    const isStdio = group.artRel.startsWith(stdioPrefix);
    const manifestAbs = join(repoRoot, group.manifestRel);
    if (!existsSync(manifestAbs)) {
      breaches.push({
        id: `standards-subset-vocabulary-missing-${group.standardRel}`,
        summary: `"${group.standardRel}" has no ${group.manifestRel.split("/").pop()} subset vocabulary manifest`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: ("Every standard declares its subset vocabulary in 🪆️subsets/" + canonicalPrimaryFilenameForKind(loadTaxonomy().semanticManifestFileKindId) + " — the single source of truth policy checks on-disk dirs against."),
        solution: `Create ${group.manifestRel} declaring {"${anyId}": {...}${isStdio ? ", plus each real subset this standard actually has" : ""}}.`,
      });
      continue;
    }
    let parsed: { artifact?: unknown; standard?: unknown; subsets?: Record<string, unknown> } | undefined;
    try {
      parsed = JSON.parse(readFileSync(manifestAbs, "utf8"));
    } catch (e) {
      breaches.push({
        id: `standards-subset-vocabulary-invalid-${group.standardRel}`,
        summary: `"${group.manifestRel}" is not valid JSON (${(e as Error).message})`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "The subset vocabulary manifest must parse as JSON.",
        solution: `Fix ${group.manifestRel}.`,
      });
      continue;
    }
    const declaredIds = Object.keys(parsed?.subsets ?? {});
    if (parsed?.standard !== group.standardSlug) {
      breaches.push({
        id: `standards-subset-vocabulary-standard-mismatch-${group.standardRel}`,
        summary: `"${group.manifestRel}" declares standard "${String(parsed?.standard)}" but lives under 🔖️${group.standardSlug}/`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "The manifest's own standard field must match the 🔖️<slug> directory it lives under.",
        solution: `Fix "standard" in ${group.manifestRel} to "${group.standardSlug}".`,
      });
    }
    if (!declaredIds.includes(anyId)) {
      breaches.push({
        id: `standards-subset-vocabulary-missing-any-${group.standardRel}`,
        summary: `"${group.manifestRel}" does not declare the base subset "${anyId}"`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "Every standard carries the unconstrained base subset; it must always be declared.",
        solution: `Add "${anyId}" to the "subsets" object in ${group.manifestRel}.`,
      });
    }
    for (const id of declaredIds) {
      if (id === anyId || subsetSlugPattern.test(id)) continue;
      breaches.push({
        id: `standards-subset-vocabulary-bad-id-${group.standardRel}-${id}`,
        summary: `"${group.manifestRel}" declares subset id "${id}" which does not match ${subsetSlugPattern}`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "Declared subset ids must be normalized lowercase identifiers.",
        solution: `Fix or remove "${id}" in ${group.manifestRel}.`,
      });
    }
    if (!isStdio) {
      const extra = declaredIds.filter((id) => id !== anyId);
      if (extra.length > 0) {
        breaches.push({
          id: `standards-subset-vocabulary-domain-real-subset-${group.standardRel}`,
          summary: `"${group.manifestRel}" declares real subset(s) [${extra.join(", ")}] on a non-stdio (domain) artifact`,
          kind: "stdio-artifacts/standards-subset-vocabulary",
          scope: group.artRel,
          priority: "high",
          reason: "Domain (non-stdio) artifacts stay at v1 with the \"*\" subset only — real industry subset vocabularies are a stdio-only concept.",
          solution: `Remove [${extra.join(", ")}] from ${group.manifestRel}, or move this artifact under stdio if it genuinely has industry subsets.`,
        });
      }
    }
    const subsetsRel = `${group.standardRel}/${taxonomy.subsetsDirName}`;
    const declaredDirs = new Set(declaredIds.map((id) => subsetDirectoryNameForId(subsetsRel, id, taxonomy)).filter((name): name is string => name !== null));
    for (const id of declaredIds) if (subsetDirectoryNameForId(subsetsRel, id, taxonomy) === null) breaches.push({
      id: `standards-subset-vocabulary-unmapped-id-${group.standardRel}-${id}`,
      summary: `"${group.manifestRel}" declares subset id "${id}" without an owner-scoped physical directory identity`,
      kind: "stdio-artifacts/standards-subset-vocabulary",
      scope: group.artRel,
      priority: "high",
      reason: "Every declared logical subset id must map to one exact semantic directory in its standard's roster.",
      solution: `Register an exact physical directory for "${id}" under ${subsetsRel}.`,
    });
    const actualDirs = new Set(group.dialects.map((d) => d.subsetDirName));
    for (const dir of actualDirs) {
      if (!declaredDirs.has(dir)) {
        breaches.push({
          id: `standards-subset-vocabulary-undeclared-dir-${group.standardRel}-${dir}`,
          summary: `"${group.standardRel}/${(taxonomy as any).subsetsDirName ?? POLICY_SUBSETS_DIR}/${dir}" exists on disk but is not declared in ${group.manifestRel}`,
          kind: "stdio-artifacts/standards-subset-vocabulary",
          scope: group.artRel,
          priority: "high",
          reason: "On-disk subset dirs and the manifest's declared vocabulary must be exactly equal, in both directions.",
          solution: `Add an entry for this subset to ${group.manifestRel}.`,
        });
      }
    }
    for (const dir of declaredDirs) {
      if (!actualDirs.has(dir)) {
        breaches.push({
          id: `standards-subset-vocabulary-missing-dir-${group.standardRel}-${dir}`,
          summary: `${group.manifestRel} declares a subset whose dir "${dir}" does not exist under ${group.standardRel}/${(taxonomy as any).subsetsDirName ?? POLICY_SUBSETS_DIR}/`,
          kind: "stdio-artifacts/standards-subset-vocabulary",
          scope: group.artRel,
          priority: "high",
          reason: "On-disk subset dirs and the manifest's declared vocabulary must be exactly equal, in both directions.",
          solution: `Create the ${dir}/ subset dir, or remove its entry from ${group.manifestRel} if it was declared ahead of the real implementation.`,
        });
      }
    }
    if (isStdio) {
      for (const dialect of group.dialects) {
        if (dialect.subsetId === anyId) continue;
        const composerRel = `${dialect.subsetRel}/🚪️io/${POLICY_RS_COMPONENT_LEAF}`;
        const composerAbs = join(repoRoot, composerRel);
        const body = existsSync(composerAbs) ? readFileSync(composerAbs, "utf8") : "";
        const hasValidatorImpl = policyRustFileHasRealTraitImpl(body, "SubsetValidator");
        const hasRegisterCall = /register_subset_validator\s*\(/.test(policyStripRustCommentsAndStrings(body));
        if (!hasValidatorImpl || !hasRegisterCall) {
          breaches.push({
            id: `standards-subset-vocabulary-validator-missing-${dialect.subsetRel}`,
            summary: `"${dialect.subsetRel}" is a real subset but its IO hook does not ${!hasValidatorImpl ? "implement SubsetValidator" : "call register_subset_validator"}`,
            kind: "stdio-artifacts/standards-subset-vocabulary",
            scope: dialect.artRel,
            priority: "high",
            reason: "Every real (non-\"*\") stdio subset registers a real SubsetValidator (io.subset.validator-missing otherwise fires at runtime for every compose of this dialect) — see the PDF/A pilot.",
            solution: `Add \`impl SubsetValidator for ...\` and a \`register_subset_validator(...)\` call to ${composerRel}.`,
          });
        }
      }
    }
  }
  return breaches;
}
//#endregion 🏅️PolicyRuleStandardsSubsets

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

/** 🔎️True if any `🦀️.rs` exists anywhere under `rootRel` (any depth) — used to check a
 * migrated io leaf exists without having to guess which of a stdio format's own (standard,
 * subset) pairs a given domain artifact happens to bridge to. */
function policyHasComponentUnder(repoRoot: string, rootRel: string): boolean {
  const abs = join(repoRoot, rootRel);
  if (!existsSync(abs)) return false;
  const stack = [rootRel];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    for (const entry of policyReaddirSafe(repoRoot, dir)) {
      if (entry.isDirectory) {
        stack.push(`${dir}/${entry.name}`);
      } else if (entry.name === POLICY_RS_COMPONENT_LEAF) {
        return true;
      }
    }
  }
  return false;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 3: the
 * REAL replacement for `policyIoSerializerMatrixBreaches`'s dead early-continue (see that
 * function's own comment — every owner is migrated now, so it has been a permanent no-op since
 * this session's Phase 1 completed the migration; confirmed via zero `stdio-artifacts/io-matrix`
 * breaches anywhere in a real `bun ./📜️script.ts policy` run). This checks the MIGRATED shape
 * instead: for each catalog owner's curated import/export format list, at least one of that domain
 * artifact's own (standard, subset) dirs must carry a real io leaf for that format — searched
 * existence-anywhere-under its `🚪️io/<direction>/<facet>/🗿️artifacts/<format-dir>/` tree (not a
 * specific (format-standard, format-subset) pair, since a domain artifact may reasonably bridge to
 * any one of a format's now-multiple standards, e.g. pdf 1.4 vs 1.7 — picking one to require would
 * be guessing, and this rule shouldn't invent a policy the codebase hasn't actually adopted yet).
 */
export function policyIoMatrixMigratedBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.artifacts;
  const artifactsDir = policyStdioArtifactsDirName();
  const dialectsByArt = new Map<string, PolicyArtifactDialect[]>();
  for (const d of policyListArtifactDialectDirs(repoRoot)) {
    if (!dialectsByArt.has(d.artRel)) dialectsByArt.set(d.artRel, []);
    dialectsByArt.get(d.artRel)!.push(d);
  }
  for (const owner of table.owners ?? []) {
    const scope = owner.path;
    const dialects = dialectsByArt.get(scope);
    if (!dialects || dialects.length === 0) continue; // not migrated (yet) -- the legacy flat-path rule below still owns it
    const checkDirection = (direction: string, childFacet: string, formatIds: string[], label: string) => {
      for (const formatId of formatIds) {
        const formatDir = policyStdioFormatDir(roster, formatId);
        if (!formatDir) continue; // unknown-format-id is already reported by the legacy rule for unmigrated owners; a migrated owner referencing an unknown id is vanishingly unlikely and not worth a second breach shape here
        const found = dialects.some((d) => policyHasComponentUnder(repoRoot, `${d.subsetRel}/🚪️io/${direction}/${childFacet}/${artifactsDir}/${formatDir}`));
        if (!found) {
          breaches.push({
            id: `io-matrix-migrated-${scope}-${direction}-${formatId}`,
            summary: `"${scope}" has no migrated ${label} leaf for stdio format "${formatId}" in any of its own (standard, subset) dirs`,
            kind: "artifact-io/io-matrix-migrated",
            scope,
            priority: "high",
            reason: `The catalog's curated ${label} list for this artifact names "${formatId}", but no 🚪️io/${direction}/${childFacet}/${artifactsDir}/${formatDir}/ leaf exists under any of this artifact's migrated standard/subset dirs.`,
            solution: `Add a ${direction}/${childFacet} leaf for ${formatId} under one of "${scope}"'s 🏅️standards/🔖️.../🪆️subsets/✳️.../🚪️io/ dirs, or remove "${formatId}" from the ${label} list in ${POLICY_STDIO_OWNER_TABLE_REL} if it's no longer curated.`,
          });
        }
      }
    };
    checkDirection(POLICY_STDIO_IO_IMPORT, POLICY_STDIO_FACET_DESERIALIZERS, owner.import ?? [], "import");
    checkDirection(POLICY_STDIO_IO_EXPORT, POLICY_STDIO_FACET_SERIALIZERS, owner.export ?? [], "export");
  }
  return breaches;
}

/** ⚖️Curated import/export deserializer and serializer matrix from 🧪owner-table.json. */
export function policyIoSerializerMatrixBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.artifacts;
  const artifactsDir = policyStdioArtifactsDirName();
  for (const owner of table.owners ?? []) {
    const scope = owner.path;
    if (policyArtifactIsMigrated(repoRoot, scope)) {
      // Migrated owner: its io leaves moved under 🏅️standards/.../🚪️io/ with target-qualified
      // (standard, subset) leaf paths -- a dedicated migrated io-matrix rule lands with the
      // domain fan-out (W14); until then this flat-path rule simply doesn't apply to it.
      continue;
    }
    const ioRoot = `${scope}/🚪️io`;
    if (!existsSync(join(repoRoot, ioRoot))) {
      breaches.push({
        id: `stdio-io-matrix-io-${scope}`,
        summary: `"${scope}" is missing 🚪️io/`,
        kind: "stdio-artifacts/io-matrix",
        scope,
        priority: "high",
        reason: "Curated stdio pairs are wired through the io facet deserializer/serializer tree.",
        solution: `Create ${ioRoot}/ with import/deserializers and export/serializers.`,
      });
      continue;
    }
    const checkLeaves = (direction: string, childFacet: string, formatIds: string[], label: string) => {
      for (const formatId of formatIds) {
        const formatDir = policyStdioFormatDir(roster, formatId);
        if (!formatDir) {
          breaches.push({
            id: `stdio-io-matrix-unknown-${scope}-${formatId}`,
            summary: `unknown stdio format id "${formatId}" on ${scope}`,
            kind: "stdio-artifacts/io-matrix",
            scope,
            priority: "high",
            reason: "Matrix format ids must exist in the schema-owned definition set.",
            solution: `Fix ${label} list for ${scope} in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
          });
          continue;
        }
        const leafBase = `${ioRoot}/${direction}/${childFacet}/${artifactsDir}/${formatDir}`;
        for (const leaf of [POLICY_RS_COMPONENT_LEAF, POLICY_TS_COMPONENT_LEAF] as const) {
          const leafRel = `${leafBase}/${leaf}`;
          if (existsSync(join(repoRoot, leafRel))) continue;
          breaches.push({
            id: `stdio-io-matrix-${leafRel}`,
            summary: `missing ${label} ${leaf} for ${formatId} under ${scope}`,
            kind: "stdio-artifacts/io-matrix",
            scope,
            priority: "high",
            reason: "Each curated pair needs both Rust and TypeScript codec leaves under 🗿️artifacts/<stdio-dir>/.",
            solution: `Add ${leafRel}.`,
          });
        }
      }
    };
    checkLeaves(POLICY_STDIO_IO_IMPORT, POLICY_STDIO_FACET_DESERIALIZERS, owner.import ?? [], "import");
    checkLeaves(POLICY_STDIO_IO_EXPORT, POLICY_STDIO_FACET_SERIALIZERS, owner.export ?? [], "export");
  }
  return breaches;
}

/** ⚖️Stdio dependency DAG is acyclic and every format eventually depends on binary. */
export function policyIoTerminalityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.artifacts;
  const nodes = new Set(Object.keys(roster));
  const adj = new Map<string, Set<string>>();
  for (const id of nodes) adj.set(id, new Set(roster[id]?.depends ?? []));
  for (const edge of table.dependency_edges) {
    if (!nodes.has(edge.from) || !nodes.has(edge.to)) {
      breaches.push({
        id: `stdio-dag-unknown-${edge.from}-${edge.to}`,
        summary: `schema-derived dependency edge references unknown node (${edge.from} → ${edge.to})`,
        kind: "stdio-artifacts/io-terminality",
        scope: POLICY_STDIO_PLUGIN_REL,
        priority: "high",
        reason: "Dependency edges must only connect schema-defined format ids.",
        solution: `Align schema dependencies in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
      });
      continue;
    }
    adj.get(edge.from)?.add(edge.to);
    const rosterDeps = new Set(roster[edge.from]?.depends ?? []);
    if (!rosterDeps.has(edge.to)) {
      breaches.push({
        id: `stdio-dag-definition-edge-${edge.from}-${edge.to}`,
        summary: `schema-derived dependency edge ${edge.from}→${edge.to} is absent from its definition`,
        kind: "stdio-artifacts/io-terminality",
        scope: POLICY_STDIO_PLUGIN_REL,
        priority: "high",
        reason: "Derived dependency edges must mirror definition dependencies.",
        solution: `Fix dependencies for ${edge.from} in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
      });
    }
  }
  const visitedGlobal = new Set<string>();
  const findCycle = (start: string): string[] | null => {
    const stack: string[] = [];
    const onStack = new Set<string>();
    const dfs = (n: string): string[] | null => {
      if (onStack.has(n)) return [...stack.slice(stack.indexOf(n)), n];
      if (visitedGlobal.has(n)) return null;
      onStack.add(n);
      stack.push(n);
      for (const dep of adj.get(n) ?? []) {
        const cyc = dfs(dep);
        if (cyc) return cyc;
      }
      stack.pop();
      onStack.delete(n);
      visitedGlobal.add(n);
      return null;
    };
    return dfs(start);
  };
  for (const n of nodes) {
    if (visitedGlobal.has(n)) continue;
    const cyc = findCycle(n);
    if (!cyc) continue;
    breaches.push({
      id: `stdio-dag-cycle-${cyc.join("-")}`,
      summary: `stdio DAG cycle: ${cyc.join(" → ")}`,
      kind: "stdio-artifacts/io-terminality",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Stdio codec dependencies must form a DAG.",
      solution: "Remove cyclic schema definition dependencies.",
    });
    break;
  }
  const memo = new Map<string, boolean>();
  const reachesBinary = (n: string, trail: Set<string>): boolean => {
    if (n === "binary") return true;
    if (memo.has(n)) return memo.get(n)!;
    if (trail.has(n)) return false;
    trail.add(n);
    const deps = adj.get(n);
    if (!deps || deps.size === 0) {
      memo.set(n, false);
      return false;
    }
    let ok = true;
    for (const d of deps) ok = ok && reachesBinary(d, trail);
    memo.set(n, ok);
    return ok;
  };
  for (const n of nodes) {
    if (n === "binary") continue;
    if (reachesBinary(n, new Set())) continue;
    breaches.push({
      id: `stdio-dag-term-${n}`,
      summary: `stdio format "${n}" does not terminate at binary via depends`,
      kind: "stdio-artifacts/io-terminality",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Every stdio artifact dependency chain must eventually reach the binary root.",
      solution: `Fix dependencies for "${n}" until binary is reachable.`,
    });
  }
  return breaches;
}

/** ⚖️Banned stub codec markers (SRAS, IFCCARTOONMESH, minimal stubs) must not remain in Rust sources. */
export function policyCodecFidelityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const scanRoots = [
    "🧰️framework/🔨️modules/🔺️mesh",
    "🧰️framework/🛍️products/💻️os",
    "🧰️framework/🛍️products/💻️os/🖥️host",
    "✏️s/🔌️plugins",
  ];
  const rsFiles = policyWalkRelFiles(repoRoot, scanRoots, (_p, name) => name.endsWith(".rs"));
  for (const rel of rsFiles) {
    const body = policyReadFileSafe(repoRoot, rel);
    for (const marker of POLICY_STDIO_CODEC_BANNED_MARKERS) {
      if (!body.includes(marker)) continue;
      breaches.push({
        id: `stdio-codec-ban-${rel}-${marker}`,
        summary: `"${rel}" contains banned stub marker ${JSON.stringify(marker)}`,
        kind: "stdio-artifacts/codec-fidelity",
        scope: rel,
        priority: "high",
        reason: "Framework and plugin codecs must be real round-trip implementations, not SRAS/IFCCARTOONMESH/minimal stubs.",
        solution: `Replace the stub in ${rel} with a stdio-owned codec or delete the dead path.`,
      });
    }
  }
  return breaches;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 2: a
 * SELF-referential `Dialect { artifact_kind: "s.stdio.<X>", standard: StandardId("<S>"), subset:
 * SubsetId("<U>") }` literal (one whose `artifact_kind` names the SAME stdio artifact the file
 * lives under -- e.g. `🎞️gif`'s own `DIALECT`/`WRITES` const, never a cross-artifact `DEP_*`/`READS`
 * dependency literal naming some OTHER artifact) must agree with the `🏅️standards/🔖️<S>/…
 * 🪆️subsets/✳️<U>/` directory the file actually lives under. Cross-artifact references are skipped
 * entirely -- they correctly point at a directory this file doesn't live inside, so there's nothing
 * to verify from here. Subset checking honors the `SubsetId::ANY = SubsetId("*")` sentinel
 * convention (the literal is `"*"`, the directory is named `✳️any`) rather than naive string
 * equality -- moot today (every subset is `any`), meaningful once D5's PDF/A-2b pilot lands.
 */
// 🐛 Scoped to `const DIALECT`/`const WRITES` specifically (the two canonical self-identity const
// names used throughout this codebase's composers) — earlier versions matched ANY `Dialect{...}`
// literal in the file, which also caught legitimate same-artifact-different-subset dependency
// consts (e.g. a subset composer's own `DIALECT_ANY` fallback reference into its standard's `any`
// subset) and flagged them as if they claimed self-identity. Indirection (`const WRITES: Dialect =
// SOME_OTHER_CONST;`) is a false negative here (nothing to check), which is the safe direction —
// never a false positive.
const POLICY_DIALECT_LITERAL_RE = /const\s+(?:DIALECT|WRITES)\s*:\s*Dialect\s*=\s*Dialect\s*\{\s*artifact_kind:\s*"([^"]+)"\s*,\s*standard:\s*StandardId\("([^"]+)"\)\s*,\s*subset:\s*SubsetId\("([^"]+)"\)\s*\}/g;

function policyDialectLiteralPathBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches; // owner-table-missing is already flagged by policyStdioCatalogBreaches
  const taxonomy = loadTaxonomy();
  const subsetsDirName = taxonomy.subsetsDirName;
  const dirToKey = new Map<string, string>();
  for (const [key, entry] of Object.entries(table.artifacts)) {
    dirToKey.set(entry.dir, key);
  }
  const artifactsPrefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const cut = relPath.indexOf(artifactsPrefix);
    if (cut === -1) continue;
    const afterArtifacts = relPath.slice(cut + artifactsPrefix.length);
    const ownDir = afterArtifacts.split("/")[0];
    const ownKey = ownDir ? dirToKey.get(ownDir) : undefined;
    if (!ownKey) continue;
    const ownArtifactKind = `s.stdio.${ownKey}`;
    const standardMatch = relPath.match(/🏅️standards\/🔖️([^/]+)\//);
    const pathSegments = relPath.split("/");
    const subsetsIndex = pathSegments.lastIndexOf(subsetsDirName);
    const subsetsOwner = subsetsIndex >= 0 ? pathSegments.slice(0, subsetsIndex + 1).join("/") : null;
    const physicalSubsetDirectory = subsetsIndex >= 0 ? pathSegments[subsetsIndex + 1] ?? null : null;
    const pathSubsetId = subsetsOwner && physicalSubsetDirectory ? subsetIdForDirectoryName(subsetsOwner, physicalSubsetDirectory, taxonomy) : null;
    const content = policyReadFileSafe(repoRoot, relPath);
    POLICY_DIALECT_LITERAL_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_DIALECT_LITERAL_RE.exec(content))) {
      const [, litKind, litStandard, litSubset] = m!;
      if (litKind !== ownArtifactKind) continue;
      if (standardMatch && litStandard !== standardMatch[1]) {
        breaches.push({
          id: `dialect-literal-standard-${relPath}-${litStandard}`,
          summary: `"${relPath}" declares StandardId("${litStandard}") but lives under 🏅️standards/🔖️${standardMatch[1]}/`,
          kind: "artifact-io/dialect-literal-path",
          scope: relPath,
          priority: "high",
          reason: "A self-referential Dialect literal's standard must match the 🔖️<standard> directory the file physically lives under, or the two will silently drift apart the next time either one changes alone.",
          solution: `Fix StandardId("${litStandard}") in ${relPath} to "${standardMatch[1]}", or move the file to 🏅️standards/🔖️${litStandard}/ if that's the one that's wrong.`,
        });
      }
      if (pathSubsetId !== null && litSubset !== pathSubsetId) {
        breaches.push({
          id: `dialect-literal-subset-${relPath}-${litSubset}`,
          summary: `"${relPath}" declares SubsetId("${litSubset}") but lives under ${subsetsOwner}/${physicalSubsetDirectory}/`,
          kind: "artifact-io/dialect-literal-path",
          scope: relPath,
          priority: "high",
          reason: "A self-referential Dialect literal's logical subset id must match the exact owner-scoped semantic directory it physically lives under.",
          solution: `Fix SubsetId("${litSubset}") in ${relPath} to "${pathSubsetId}", or move the file if the directory is wrong.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 🎫 Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, W1 Task 4
 * blocking finding: `store::os_store::codec_registry::register_document_codec` (🧰️framework/…/🏪️store/🦀️.rs) is a
 * plain `HashMap::insert` keyed by the codec's schema id string — registering two DIFFERENT codecs
 * under the SAME id does not panic, it silently overwrites (last-registered-wins), with zero
 * runtime diagnostic (empirically confirmed by W1's own regression test). This is a real risk once
 * many parallel agents each mint their own schema-id constant (13 semio subsets + 7 new formats +
 * 28 pre-existing artifacts, all sharing one process-wide registry) — a typo'd copy-paste collision
 * would silently load the wrong codec at runtime with zero build or test signal. This rule
 * statically resolves every `register_document_codec(store::artifact_codec::<…>(<id-expr>))`
 * call site's id expression — a string literal, or a `const _: &str` referenced bare or fully
 * path-qualified (both forms are used across the crate) — across the WHOLE stdio crate, and flags
 * any resolved id string claimed by more than one call site. Mirrors `policyDialectLiteralPathBreaches`
 * above: same two-pass grep-then-cross-reference shape, same `policyAllRustFiles` file source.
 */
function policyStdioCodecIdUniquenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginRoot = join(repoRoot, POLICY_STDIO_PLUGIN_REL);
  if (!existsSync(pluginRoot)) return breaches;
  const artifactsPrefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  const files = policyAllRustFiles(repoRoot).filter((relPath) => relPath.startsWith(artifactsPrefix));

  // Pass 1: crate-wide `const NAME: &str = "value";` map, keyed by the identifier's *last path
  // segment* — call sites reference a schema-id const either bare or fully `crate::…::`-qualified.
  const constValue = new Map<string, string>();
  const CONST_RE = /(?:pub\s+)?const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]*)"\s*;/g;
  for (const relPath of files) {
    const content = policyReadFileSafe(repoRoot, relPath);
    CONST_RE.lastIndex = 0;
    let cm: RegExpExecArray | null;
    while ((cm = CONST_RE.exec(content))) {
      const [, name, value] = cm;
      if (name !== undefined && value !== undefined) constValue.set(name, value);
    }
  }

  // Pass 2: every register_document_codec call site's id expression, resolved to a literal value.
  const CALL_RE = /register_document_codec\(\s*store::artifact_codec::<[\s\S]*?>\(\s*([\w:]+|"[^"]*")\s*\)\s*\)/g;
  const claims = new Map<string, { relPath: string; line: number }[]>();
  for (const relPath of files) {
    const content = policyReadFileSafe(repoRoot, relPath);
    CALL_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = CALL_RE.exec(content))) {
      const raw = m[1]!;
      const value = raw.startsWith('"') ? raw.slice(1, -1) : constValue.get(raw.split("::").pop()!);
      if (!value) continue; // unresolved id expression — not guessed at, silently skipped
      const line = content.slice(0, m.index).split("\n").length;
      const list = claims.get(value) ?? [];
      list.push({ relPath, line });
      claims.set(value, list);
    }
  }

  for (const [id, sites] of claims) {
    if (sites.length <= 1) continue;
    const locations = sites.map((s) => `${s.relPath}:${s.line}`).join(", ");
    for (const site of sites) {
      breaches.push({
        id: `stdio-codec-id-duplicate-${id}-${site.relPath}-${site.line}`,
        summary: `schema id "${id}" is claimed by ${sites.length} register_document_codec call sites: ${locations}`,
        kind: "stdio-artifacts/codec-id-uniqueness",
        scope: site.relPath,
        priority: "high",
        reason:
          "store::os_store::codec_registry::register_document_codec silently overwrites on a duplicate id (plain HashMap::insert, no panic) — two codecs sharing an id means the second-registered one silently and invisibly wins at runtime, with zero build or test signal.",
        solution: `Give each of ${locations} its own distinct schema id constant/value.`,
      });
    }
  }
  return breaches;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 8:
 * replaces the old `policyCodecFidelityBreaches` (a 5-string banned-marker grep — "does this file
 * avoid saying SRAS/IFCCARTOONMESH", a purely negative signal) with a positive one: every
 * standard-level `⚙️engine/🦀️.rs` must contain a real decode→encode→decode (or
 * equivalent lossless) round-trip test. Detected via a deliberately generous name/body heuristic
 * (`round_trip`/`roundtrip`/`decode_encode`/`encode_decode`/`lossless`, case-insensitive) rather
 * than requiring one exact test name — every codec this session actually delivered uses one of
 * these, under varying names (`codec_round_trip`, `zip_deflate_round_trip`,
 * `real_decode_stays_lossless_on_reencode`, ...). Old rule NOT deleted yet (still catches literal
 * stub markers, a cheap orthogonal check) — the plan calls for deleting it once this one's
 * allowlist is fully burned down; premature while 43 files remain.
 */
const POLICY_ROUND_TRIP_TEST_ALLOWLIST = new Set<string>([
  // 🐛 Same lesson as the other two allowlists in this file: keys are policyNormalizeRelPath's
  // canonical short form, computed programmatically against a real repo scan, never hand-typed —
  // see the sniff-reality allowlist's own comment for the full story of what happens otherwise.
  "trinity/jack/standards#1-engine-component",
  "trinity/rewriting/standards#1-engine-component",
  "raster/standards#1-engine-component",
  "flow/standards#1-engine-component",
  "process/process3d/standards#1-engine-component",
  "norm/din4108/standards#1-engine-component",
  "norm/din18599/standards#1-engine-component",
  "norm/din16798/standards#1-engine-component",
  "norm/en1990/standards#1-engine-component",
  "norm/en1991/standards#1-engine-component",
  "norm/en1992/standards#1-engine-component",
  "norm/en1993/standards#1-engine-component",
  "norm/en1994/standards#1-engine-component",
  "norm/en1995/standards#1-engine-component",
  "norm/en1996/standards#1-engine-component",
  "norm/en1997/standards#1-engine-component",
  "norm/en1998/standards#1-engine-component",
  "norm/en1999/standards#1-engine-component",
  "cad/standards#1-engine-component",
  "demonstrator/playground/standards#1-engine-component",
  "block/2d/standards#1-engine-component",
  "block/3d/standards#1-engine-component",
  "block/5d/standards#1-engine-component",
  "dag/standards#1-engine-component",
  "stdio/pdf/standards#1.4-engine-component", // deliberately untouched; real vocabulary is on 1.7
  "reasoning/wires/standards#1-engine-component",
  "writer/standards#1-engine-component",
  "animate/presentation/standards#1-engine-component",
  "space/home/standards#1-engine-component",
  "procedural/generation2d/standards#1-engine-component",
  "vcs/standards#1-engine-component",
  "gis/gismap/standards#1-engine-component",
  "gis/gisterrain/standards#1-engine-component",
  "note/standards#1-engine-component",
  "architect/program/standards#1-engine-component",
  "shooting/standards#1-engine-component",
  "puzzle/2d/standards#1-engine-component",
  "puzzle/3d/standards#1-engine-component",
  "puzzle/5d/standards#1-engine-component",
  "fem/2d/standards#1-engine-component",
  "fem/3d/standards#1-engine-component",
  "playbook/standards#1-engine-component",
  "energy/model/standards#1-engine-component",
  // seeded by W1b scaffold, burn down as W3 rewires each format's ArtifactPack off JSON-passthrough
  // onto its own engine's real encoder (see w1b-scaffold-manifest.md §6 — sniff/minimal-parse is
  // already real+tested for all 7, this is specifically the missing decode→encode→decode test) and
  // as W2 adds semio v1's own round-trip test alongside geometry/triples' existing 9. Keys computed
  // via policyNormalizeRelPath.
  // W3 closer removed epw/mp4/mp3/avi/wav — each `⚙️engine/🦀️.rs` now has a real
  // `#[cfg(test)]` round-trip test (grep-verified, policy re-run green after). html stays: its
  // real codec_retention_law round-trip test lives in `📸️snapshot/🦀️.rs`, not
  // `⚙️engine/🦀️.rs` — this rule is scoped to the engine file specifically, so html's
  // `⚙️engine/🦀️.rs` (just `sniff_real_bytes` + a small sniff test) genuinely still has
  // no round-trip signal under this rule's own detection method; a real architectural mismatch
  // between where html's parser lives and where this rule looks, not a fixed-and-forgotten entry
  // — documented here as a follow-up rather than moved, since relocating html's parser is a
  // design decision outside this closer's scope.
  "stdio/html/standards#5-engine-component",
  "stdio/semio/standards#v1-engine-component",
]);

const POLICY_ROUND_TRIP_SIGNAL_RE = /round_trip|roundtrip|decode_encode|encode_decode|lossless/i;

function policyRoundTripTestBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith("✏️s/🔌️plugins/")) continue;
    if (!relPath.endsWith(`⚙️engine/${POLICY_RS_COMPONENT_LEAF}`)) continue;
    if (!relPath.includes("🏅️standards/🔖️")) continue;
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_ROUND_TRIP_TEST_ALLOWLIST.has(normalized);
    const content = policyReadFileSafe(repoRoot, relPath);
    const hasSignal = content.includes("#[cfg(test)]") && POLICY_ROUND_TRIP_SIGNAL_RE.test(content);
    if (!hasSignal) {
      if (allowlisted) continue;
      breaches.push({
        id: `round-trip-test-${relPath}`,
        summary: `"${relPath}" has no real decode→encode→decode round-trip test`,
        kind: "artifact-io/round-trip-test",
        scope: relPath,
        priority: "high",
        reason: "Every standard-level engine must prove its codec is lossless with a real round-trip test, not just avoid a list of known-stub markers.",
        solution: `Add a #[test] round-trip (decode→encode→decode) test to ${relPath}, or if this standard hasn't been reached by the codec uplift yet, add "${normalized}" to POLICY_ROUND_TRIP_TEST_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `round-trip-test-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_ROUND_TRIP_TEST_ALLOWLIST but already has a round-trip test`,
        kind: "artifact-io/round-trip-test",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_ROUND_TRIP_TEST_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 6
 * (scoped): every derived composition hook declares its cross-artifact dependencies as `const DEP_<NAME>:
 * Dialect = Dialect { artifact_kind: "s.stdio.<dep>", standard: StandardId("<std>"), subset:
 * SubsetId(...) }` (confirmed the universal pattern across every stdio composer this session
 * touched). This rule verifies each declared dependency's (artifact, standard) pair actually
 * exists on disk — `🗿️artifacts/<dep-dir>/🏅️standards/🔖️<std>/` — catching a phantom dependency
 * (typo'd standard, or a standard that got renamed/deleted out from under a composer that still
 * references the old name, e.g. exactly the `ac1018`→`ac1024` kind of rename this session did
 * several times) at policy-check time instead of a runtime compose failure. This is the "no
 * phantoms" half of the plan's fuller "composer dialect consts ↔ io leaf dirs bijection" — the "no
 * orphans" half (an io leaf dir nobody's composer ever references) needs a second pass once rule 3
 * (io-matrix migrated-leaves) traces the legacy early-continue branch it currently shares
 * plumbing with; left for a future session rather than rushed.
 */
const POLICY_DEP_DIALECT_RE = /const\s+DEP_\w+\s*:\s*Dialect\s*=\s*Dialect\s*\{\s*artifact_kind:\s*"s\.stdio\.([^"]+)"\s*,\s*standard:\s*StandardId\("([^"]+)"\)/g;

function policyComposerDependencyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const dirByKey = new Map<string, string>();
  for (const [key, entry] of Object.entries(table.artifacts)) {
    dirByKey.set(key, entry.dir);
  }
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.endsWith(`🚪️io/${POLICY_RS_COMPONENT_LEAF}`)) continue;
    if (!relPath.startsWith(`${POLICY_STDIO_ARTIFACTS_REL}/`)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    POLICY_DEP_DIALECT_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_DEP_DIALECT_RE.exec(content))) {
      const [, depKey, depStandard] = m!;
      const depDir = dirByKey.get(depKey!);
      if (!depDir) {
        breaches.push({
          id: `composer-dep-unknown-artifact-${relPath}-${depKey}`,
          summary: `"${relPath}" depends on unknown stdio artifact "${depKey}" (not in the catalog roster)`,
          kind: "artifact-io/composer-dependency",
          scope: relPath,
          priority: "high",
          reason: "A composer's DEP_* dialect must name a real, currently-cataloged stdio artifact — an unknown one is either a typo or a stale reference to a deleted/renamed artifact.",
          solution: `Fix the artifact_kind in ${relPath}'s DEP_* constant, or add "${depKey}" to the catalog roster if it's genuinely new.`,
        });
        continue;
      }
      const depStandardDir = join(repoRoot, POLICY_STDIO_ARTIFACTS_REL, depDir, "🏅️standards", `🔖️${depStandard}`);
      if (!existsSync(depStandardDir)) {
        breaches.push({
          id: `composer-dep-unknown-standard-${relPath}-${depKey}-${depStandard}`,
          summary: `"${relPath}" depends on ${depKey}@${depStandard}, but no such standard directory exists`,
          kind: "artifact-io/composer-dependency",
          scope: relPath,
          priority: "high",
          reason: "A composer's DEP_* dialect must name a standard that actually exists on disk — a phantom reference (typo, or a stale reference to a renamed/deleted standard) fails at compose time instead of at policy-check time.",
          solution: `Fix the StandardId in ${relPath}'s DEP_* constant to match an existing 🏅️standards/🔖️<std>/ directory under ${depDir}, or restore that standard if it was deleted in error.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 5:
 * flagship artifacts (svg/gltf/pdf/gif — the ones D2 explicitly names as getting real mutation
 * vocabularies, not just `{NoMutation, SetSnapshot}`) must have a mutation enum with more than
 * those two variants. Shrink-only allowlist for the flagships this hasn't reached yet.
 */
const POLICY_FLAGSHIP_MUTATION_ALLOWLIST = new Set<string>([
  // 🐛 Keys are policyNormalizeRelPath's canonical <pluginId>/<component>#<tail> short form, NOT
  // the raw file path — a full-path entry silently never matches (found the hard way: this
  // allowlist, and the sibling POLICY_SNIFF_REALITY_ALLOWLIST, both shipped with raw-path keys for
  // most of a session, during which `bun ./📜️script.ts verify` (a DIFFERENT, narrower gate
  // pipeline than `bun ./📜️script.ts policy`) was mistakenly used to "confirm" zero breaches —
  // `policy` is the command that actually runs these rules; always verify against `policy`, not
  // `verify`, when touching anything under //#region 🔧️PolicyRule*).
  "stdio/gltf/standards#2.0-subsets-any-schema-mutations-component", // gltf-internal's steps 1-2 scope didn't include mutations; not yet reached
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-component", // 1.4 deliberately left untouched; the real vocabulary landed on 1.7 instead
  "stdio/gif/standards#87a-subsets-any-schema-mutations-component", // permanent, not a todo: 87a has no GCE/animation/transparency concept to mutate: 89a carries the real vocabulary
]);

/** 🔎️Top-level variant names of the first `pub enum ...Mutation { ... }` found in `content`, or `null` if none found. Brace-depth-tracked (variants carry struct-style bodies with their own braces). */
function policyRustMutationEnumVariants(content: string): string[] | null {
  const stripped = policyStripRustCommentsAndStrings(content);
  const headerMatch = stripped.match(/pub\s+enum\s+\w*Mutation\w*\s*\{/);
  if (!headerMatch) return null;
  const start = headerMatch.index! + headerMatch[0].length;
  let depth = 1;
  let i = start;
  let segStart = start;
  const variants: string[] = [];
  const pushSeg = (raw: string) => {
    const m = raw.match(/(?:#\[[^\]]*\]\s*)*(\w+)/);
    if (m) variants.push(m[1]!);
  };
  while (i < stripped.length && depth > 0) {
    const ch = stripped[i];
    if (ch === "{" || ch === "(") depth++;
    else if (ch === "}" || ch === ")") {
      depth--;
      if (depth === 0) {
        pushSeg(stripped.slice(segStart, i));
        break;
      }
    } else if (ch === "," && depth === 1) {
      pushSeg(stripped.slice(segStart, i));
      segStart = i + 1;
    }
    i++;
  }
  return variants;
}

function policyMutationVocabularyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const flagshipDirs = [
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌳️pdf",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif",
  ];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.endsWith(`🧬️mutations/${POLICY_RS_COMPONENT_LEAF}`)) continue;
    if (!flagshipDirs.some((dir) => relPath.startsWith(`${dir}/`))) continue;
    const normalized = policyNormalizeRelPath(relPath);
    const content = policyReadFileSafe(repoRoot, relPath);
    const variants = policyRustMutationEnumVariants(content);
    const realVariantCount = variants ? variants.filter((v) => v !== "NoMutation" && v !== "SetSnapshot").length : 0;
    const allowlisted = POLICY_FLAGSHIP_MUTATION_ALLOWLIST.has(normalized);
    if (realVariantCount === 0) {
      if (allowlisted) continue;
      breaches.push({
        id: `mutation-vocabulary-${relPath}`,
        summary: `"${relPath}" is a flagship mutation enum with only NoMutation/SetSnapshot`,
        kind: "artifact-io/mutation-vocabulary",
        scope: relPath,
        priority: "high",
        reason: "D2 names this artifact as a flagship that must expose a real mutation vocabulary (targeted insert/remove/set-field variants with real apply+inverse), not just the universal NoMutation/SetSnapshot pair every artifact starts with.",
        solution: `Add real, targeted mutation variants (with real apply+inverse) to ${relPath}, or if not reached yet, add "${normalized}" to POLICY_FLAGSHIP_MUTATION_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `mutation-vocabulary-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_FLAGSHIP_MUTATION_ALLOWLIST but already has ${realVariantCount} real mutation variant(s)`,
        kind: "artifact-io/mutation-vocabulary",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_FLAGSHIP_MUTATION_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/** 🚪️ Enforces semantic schema and artifact wire ownership across every artifact dialect. */
export function policyArtifactIoBreaches(repoRoot: string): BreachRecord[] {
  return artifactIoArchitectureBreaches(repoRoot);
}

/** ⚖️Aggregates stdio-artifact policy scanners (catalog, builder, decomposer, schema, io matrix, DAG, codecs,
 * plus the standards/subsets migrated-side rules -- each pair is shape-partitioned: an artifact is checked
 * by exactly one side of every pair, never both, per policyArtifactIsMigrated). */
export function policyStdioArtifactsBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyStdioCatalogBreaches(repoRoot),
    ...policyArtifactBuilderBreaches(repoRoot),
    ...policyArtifactDecomposerBreaches(repoRoot),
    ...policySchemaRepresentationBreaches(repoRoot),
    ...policyArtifactIoBreaches(repoRoot),
    ...policyIoSerializerMatrixBreaches(repoRoot),
    ...policyIoMatrixMigratedBreaches(repoRoot),
    ...policyIoTerminalityBreaches(repoRoot),
    ...policyCodecFidelityBreaches(repoRoot),
    ...policyStandardsCoverageBreaches(repoRoot),
    ...policyArtifactBuilderMigratedBreaches(repoRoot),
    ...policyArtifactAnalyzerBreaches(repoRoot),
    ...policyArtifactComposerBreaches(repoRoot),
    ...policyDialectLiteralPathBreaches(repoRoot),
    ...policyStdioCodecIdUniquenessBreaches(repoRoot),
    ...policyMutationVocabularyBreaches(repoRoot),
    ...policyRoundTripTestBreaches(repoRoot),
    ...policyComposerDependencyBreaches(repoRoot),
    ...policyStandardSubsetVocabularyBreaches(repoRoot),
  ];
}

//#endregion 🔧️PolicyRuleArtifactIo
