import { validateFrozenCoordinateEvidenceContracts, validateFrozenMarkdownCoordinateEvidenceContracts, validateHistoricalDocumentEvidencePopulations } from "../📑️taxonomy-document/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 📆️ Audits the removed vocabulary keys, the exact schema version and the frozen coordinate, frozen markdown coordinate and historical document evidence populations. */
export function auditSchemaVersionAndEvidence(audit: TaxonomyAudit): void {
  const { taxonomy, problems, document } = audit;
  const removedKeys = [
    "semanticManifestFilename", "subsetsManifestFilename", "packagingFileNames", "packagingFileSuffixes", "packagingDirNames",
    "surfaceSchemaSpecFilenames", "textSpecFilenames", "binarySpecFilenames", "artifactSchemaSpecFilenames", "exampleLeafFilenames",
    "exampleTestLeafFilenames", "semioDataLeafPrefix", "semioFileExtension", "artifactSpecFilenames", "windowEmptyFacetFilename",
    "taxonomyLeafFilenames", "entryFilenames", "storyLeafFilename", "requireEmojiPrefixWithVs16", "rootDataFileNames",
    "rootDocFileNames", "areaStates", "pluginTaxonomyStates", "repoWideFiles", "testFeatureFilename", "testAdapterFilenames",
    "testContributionFilename", "testOutputMarkerFilename", "testExcludedPathPrefixes", "testOracleRegistryPath", "testSchemaPath",
    "layeringGeneratedInventories", "semanticProjectionContracts", "projectedMemberKinds", "projectionContracts", "profileRenderers",
    "descendantContracts", "mutationCatalogProjectionContractId", "mutationCatalogKindBijection", "mutationChildDirs", "compositeMutationChildDirs",
  ];
  for (const key of removedKeys) if (key in document) problems.push(`${key} was removed by schema version 7; use kind IDs or exact contracts.`);
  if (taxonomy.schemaVersion !== 7) problems.push(`schemaVersion must be exactly 7, got ${JSON.stringify(taxonomy.schemaVersion)}.`);
  problems.push(...validateFrozenCoordinateEvidenceContracts(taxonomy.frozenCoordinateEvidenceContracts));
  problems.push(...validateFrozenMarkdownCoordinateEvidenceContracts(taxonomy.frozenMarkdownCoordinateEvidenceContracts));
  problems.push(...validateHistoricalDocumentEvidencePopulations(taxonomy.historicalDocumentEvidencePopulations));
}
