import type { BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { artifactIoArchitectureBreaches } from "../../../🚪️io/🏛️architecture/🟦️.ts";
import { policyArtifactAnalyzerBreaches, policyArtifactBuilderBreaches, policyArtifactBuilderMigratedBreaches, policyArtifactComposerBreaches, policyArtifactDecomposerBreaches, policyStdioCatalogBreaches } from "./🗂️stdio-catalog/🟦️.ts";
import { policySchemaRepresentationBreaches } from "./🧬️schema-representation/🟦️.ts";
import { policyCodecFidelityBreaches, policyIoMatrixMigratedBreaches, policyIoSerializerMatrixBreaches, policyIoTerminalityBreaches } from "./🔌️io-matrix/🟦️.ts";
import { policyStandardsCoverageBreaches, policyStandardSubsetVocabularyBreaches } from "./🧾️standards/🟦️.ts";
import { policyComposerDependencyBreaches, policyDialectLiteralPathBreaches, policyMutationVocabularyBreaches, policyRoundTripTestBreaches, policyStdioCodecIdUniquenessBreaches } from "./🔁️dialects-and-round-trip/🟦️.ts";

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
