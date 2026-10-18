import { canonicalPrimaryFilenameForKind, loadCatalogTaxonomy, loadTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF } from "../../✍️handcrafted-mutation/🟦️.ts";
import { POLICY_TS_COMPONENT_LEAF } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { stdioArtifactLedger } from "../../../../🗄️stdio-ledger/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

/** 🧾 Schema-derived stdio definition view for policy checks. */
export const POLICY_STDIO_OWNER_TABLE_REL = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🔣️.json";

export const POLICY_STDIO_PLUGIN_REL = "✏️s/🔌️plugins/🗄️stdio";

export const POLICY_STDIO_ARTIFACTS_REL = `${POLICY_STDIO_PLUGIN_REL}/🗿️artifacts`;

export const POLICY_STDIO_FACET_DECOMPOSER = "🪓️decomposer";

export const POLICY_STDIO_FACET_TEXT = "📝️text";

export const POLICY_STDIO_FACET_BINARY = "💾️binary";

export const POLICY_STDIO_FACET_DESERIALIZERS = "🧩️deserializers";

export const POLICY_STDIO_FACET_SERIALIZERS = "🧵️serializers";

export const POLICY_STDIO_IO_IMPORT = "📥️import";

export const POLICY_STDIO_IO_EXPORT = "📤️export";

const POLICY_STDIO_SCHEMA_CHILD_FALLBACK = ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const;

const POLICY_STDIO_REPRESENTATION_FALLBACK = [POLICY_STDIO_FACET_TEXT, POLICY_STDIO_FACET_BINARY] as const;

const POLICY_STDIO_ARTIFACT_FACET_FALLBACK = ["🧬️schema", "⚙️engine", "🚪️io", POLICY_STDIO_FACET_DECOMPOSER] as const;

const POLICY_STDIO_LEGACY_ARTIFACT_FACETS = new Set(["🗣️dsl", "🔧️op", "📡️spr", "🔺️diff", "📸️snapshot"]);

export const POLICY_STDIO_TEXT_SPEC_LEAVES = [
  ...["grammar-semio", "ebnf", "antlr", "graphql", "json", "protobuf"].map((kindId) => canonicalPrimaryFilenameForKind(kindId, loadCatalogTaxonomy())),
  POLICY_RS_COMPONENT_LEAF,
  POLICY_TS_COMPONENT_LEAF,
] as const;

export const POLICY_STDIO_BINARY_SPEC_LEAVES = [
  ...["protocol-semio", "abnf", "kaitai", "spicy"].map((kindId) => canonicalPrimaryFilenameForKind(kindId, loadCatalogTaxonomy())),
  POLICY_RS_COMPONENT_LEAF,
  POLICY_TS_COMPONENT_LEAF,
] as const;

export const POLICY_STDIO_CODEC_BANNED_MARKERS = ["SRAS", "IFCCARTOONMESH", "b\"minimal\"", "stub codec", "minimal stub codec"] as const;

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

export function policyLoadStdioOwnerTable(repoRoot: string): PolicyStdioDefinitionTable | null {
  try {
    const ledger = stdioArtifactLedger(repoRoot);
    const artifacts = Object.fromEntries(ledger.artifacts.map((artifact) => [artifact.id, { dir: artifact.directory, depends: [...artifact.depends] }]));
    return { artifacts, dependency_edges: ledger.artifacts.flatMap((artifact) => artifact.depends.map((to) => ({ from: artifact.id, to }))), owners: [], counts: { stdio_artifacts: ledger.counts.artifacts } };
  } catch {
    return null;
  }
}

export function policyStdioArtifactFacets(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const dirs = taxonomy.artifactComponentDirs;
  if (!dirs?.length) return [...POLICY_STDIO_ARTIFACT_FACET_FALLBACK];
  const out = new Set<string>(POLICY_STDIO_ARTIFACT_FACET_FALLBACK);
  for (const d of dirs) {
    if (!POLICY_STDIO_LEGACY_ARTIFACT_FACETS.has(d)) out.add(d);
  }
  return [...out];
}

export function policyStdioSchemaChildDirs(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const from = taxonomy.schemaChildDirs as string[] | undefined;
  return from?.length ? [...from] : [...POLICY_STDIO_SCHEMA_CHILD_FALLBACK];
}

export function policyStdioRepresentationDirs(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  return [...taxonomy.nativeRepresentationDirs];
}

export function policyStdioFormatDir(artifacts: PolicyStdioDefinitionTable["artifacts"], formatId: string): string | undefined {
  return artifacts[formatId]?.dir;
}

export function policyStdioArtifactsDirName(): string {
  return loadTaxonomy().artifactsDirName ?? "🗿️artifacts";
}

export function policyRustFileHasRealTraitImpl(body: string, traitName: string): boolean {
  const stripped = policyStripRustCommentsAndStrings(body);
  return new RegExp(`\\bimpl\\s*(?:<[^>{]*>\\s*)?${traitName}\\b\\s*(?:<[^>{]*>\\s*)?for\\b`).test(stripped);
}
