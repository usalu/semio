import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🛖️ Audits the semantic directory registrations of every declared directory value and the required child directory subsets. */
export function auditRepresentationAndChildDirectories(audit: TaxonomyAudit): void {
  const { taxonomy, problems, semanticDirectoryKindId } = audit;
  const directoryValues = [
    taxonomy.packagesDirName, taxonomy.targetsDirName, taxonomy.elementsDirName, taxonomy.artifactsDirName, taxonomy.modesDirName,
    taxonomy.windowsDirName, taxonomy.standardsDirName, taxonomy.subsetsDirName, taxonomy.viewerDirName, taxonomy.editorDirName,
    taxonomy.exampleAssetsDirName, taxonomy.exampleTestsDirName, taxonomy.testExamplesDirName, taxonomy.testOraclesDirName, ...taxonomy.artifactChildDirs, ...taxonomy.newArtifactChildDirs,
    ...taxonomy.standardChildDirs, ...taxonomy.subsetChildDirs, ...taxonomy.surfaceChildDirs, ...taxonomy.modeChildDirs,
    ...taxonomy.windowChildDirs, ...taxonomy.taxonomyLeafParentDirs, ...taxonomy.pluginChildDirs, ...taxonomy.osChildDirs,
    ...taxonomy.rootDataDirNames, ...taxonomy.schemaChildDirs, ...taxonomy.representationDirs, ...taxonomy.ioDirectionDirs,
    ...taxonomy.ioSemanticCollectionDirNames, ...Object.values(taxonomy.ioDirectionChildDirs), ...taxonomy.mutationBehaviorFacetDirs, ...taxonomy.mutationOrganizationalFacetDirs,
  ];
  if (!Array.isArray(taxonomy.nativeRepresentationDirs) || taxonomy.nativeRepresentationDirs.length === 0 || new Set(taxonomy.nativeRepresentationDirs).size !== taxonomy.nativeRepresentationDirs.length || taxonomy.nativeRepresentationDirs.some(directory => !taxonomy.representationDirs.includes(directory))) problems.push("nativeRepresentationDirs must be a non-empty unique subset of representationDirs.");
  for (const directory of new Set(directoryValues)) if (!semanticDirectoryKindId(directory, taxonomy)) problems.push(`Semantic directory ${JSON.stringify(directory)} is not uniquely registered.`);
  for (const dir of taxonomy.artifactComponentDirs) if (!taxonomy.artifactChildDirs.includes(dir)) problems.push(`artifactChildDirs must include ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.windowRequiredChildDirs) if (!taxonomy.windowChildDirs.includes(dir)) problems.push(`windowChildDirs must include ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.surfaceRequiredChildDirs) if (!taxonomy.surfaceChildDirs.includes(dir)) problems.push(`surfaceChildDirs must include ${JSON.stringify(dir)}.`);
}
