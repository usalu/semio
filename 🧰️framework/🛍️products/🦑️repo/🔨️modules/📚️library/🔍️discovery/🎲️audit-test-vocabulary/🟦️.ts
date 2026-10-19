import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🎲️ Audits the test vocabulary: implementation kind mappings, directory names, obsolete stems, modules, patterns and in-package implementations. */
export function auditTestVocabulary(audit: TaxonomyAudit): void {
  const { taxonomy, problems, ids } = audit;
  const mappings: [string, Readonly<Record<string, string>>][] = [
    ["componentFileKinds", taxonomy.componentFileKinds], ["exampleFileKinds", taxonomy.exampleFileKinds],
    ["exampleTestFileKinds", taxonomy.exampleTestFileKinds], ["testAdapterFileKinds", taxonomy.testAdapterFileKinds],
    ["artifactSpecFileKinds", taxonomy.artifactSpecFileKinds], ["artifactSchemaSpecFileKinds", taxonomy.artifactSchemaSpecFileKinds],
    ["surfaceSchemaSpecFileKinds", taxonomy.surfaceSchemaSpecFileKinds],
  ];
  for (const [key, mapping] of mappings) for (const [owner, kindId] of Object.entries(mapping ?? {})) if (!taxonomy.fileKinds[kindId]) problems.push(`${key}[${JSON.stringify(owner)}] references missing kind ${JSON.stringify(kindId)}.`);
  ids(taxonomy.testImplementationFileKindIds, taxonomy.fileKinds, "testImplementationFileKindIds");
  if (!Array.isArray(taxonomy.testLegacyDirectoryNames) || taxonomy.testLegacyDirectoryNames.some((name) => typeof name !== "string" || !name || /[\\/]/u.test(name))) problems.push("testLegacyDirectoryNames must contain non-empty directory names.");
  if (!Array.isArray(taxonomy.testFixtureLegacyDirectoryNames) || taxonomy.testFixtureLegacyDirectoryNames.some((name) => typeof name !== "string" || !name || /[\\/]/u.test(name))) problems.push("testFixtureLegacyDirectoryNames must contain non-empty directory names.");
  if (taxonomy.testExamplesDirName !== "📚️examples") problems.push('testExamplesDirName must select the canonical "📚️examples" collection.');
  if (taxonomy.testOraclesDirName !== "🔮️oracles") problems.push('testOraclesDirName must select the canonical "🔮️oracles" collection.');
  for (const [key, values] of [["testObsoleteCategoryStems", taxonomy.testObsoleteCategoryStems], ["testObsoleteTestEmojiCategoryStems", taxonomy.testObsoleteTestEmojiCategoryStems]] as const) {
    if (!Array.isArray(values) || values.some((name) => typeof name !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(name) || name !== name.normalize("NFC")) || new Set(values).size !== values.length) problems.push(`${key} must contain unique normalized lowercase stems.`);
  }
  if (typeof taxonomy.testRunnerConfigurationCaseName !== "string" || !taxonomy.testRunnerConfigurationCaseName || /[\\/]/u.test(taxonomy.testRunnerConfigurationCaseName)) problems.push("testRunnerConfigurationCaseName must be one directory name.");
  if (!Array.isArray(taxonomy.testSubjectBoundaryOwnerKinds) || taxonomy.testSubjectBoundaryOwnerKinds.some((name) => typeof name !== "string" || !taxonomy.testOwnerKinds.includes(name))) problems.push("testSubjectBoundaryOwnerKinds must name owner collections from testOwnerKinds.");
  if (!Array.isArray(taxonomy.testDeliveryScopeDirectoryNames) || taxonomy.testDeliveryScopeDirectoryNames.some((name) => typeof name !== "string" || !name || /[\\/]/u.test(name))) problems.push("testDeliveryScopeDirectoryNames must contain non-empty directory names.");
  if (!Array.isArray(taxonomy.testJavaScriptFrameworkModules) || taxonomy.testJavaScriptFrameworkModules.some((name) => typeof name !== "string" || !name)) problems.push("testJavaScriptFrameworkModules must contain non-empty module specifiers.");
  if (!Array.isArray(taxonomy.testAssertionModules) || taxonomy.testAssertionModules.some((name) => typeof name !== "string" || !name)) problems.push("testAssertionModules must contain non-empty module specifiers.");
  try { new RegExp(taxonomy.testSelfTestDeclarationPattern, "u"); } catch { problems.push("testSelfTestDeclarationPattern must be a valid Unicode regular expression."); }
  if (!Array.isArray(taxonomy.testLegacyFilenamePatterns)) problems.push("testLegacyFilenamePatterns must be an array.");
  else for (const [index, entry] of taxonomy.testLegacyFilenamePatterns.entries()) {
    if (!entry || typeof entry.id !== "string" || !entry.id || typeof entry.pattern !== "string" || !entry.pattern) problems.push(`testLegacyFilenamePatterns[${index}] must declare an id and pattern.`);
    else try { new RegExp(entry.pattern, "u"); } catch { problems.push(`testLegacyFilenamePatterns[${index}].pattern is not a valid Unicode regular expression.`); }
  }
  for (const [key, kindId] of [
    ["semanticManifestFileKindId", taxonomy.semanticManifestFileKindId], ["subsetsManifestFileKindId", taxonomy.subsetsManifestFileKindId],
    ["storyFileKindId", taxonomy.storyFileKindId], ["testFeatureFileKindId", taxonomy.testFeatureFileKindId],
    ["testContributionFileKindId", taxonomy.testContributionFileKindId], ["testOutputMarkerFileKindId", taxonomy.testOutputMarkerFileKindId],
    ["windowEmptyFacetFileKindId", taxonomy.windowEmptyFacetFileKindId], ["mutationComponentFileKindId", taxonomy.mutationComponentFileKindId], ["mutationDescriptorFileKindId", taxonomy.mutationDescriptorFileKindId], ["testOracleRegistryLocation.fileKindId", taxonomy.testOracleRegistryLocation?.fileKindId],
    ["testSchemaLocation.fileKindId", taxonomy.testSchemaLocation?.fileKindId],
  ] as const) if (!kindId || !taxonomy.fileKinds[kindId]) problems.push(`${key} references a missing file kind.`);
}
