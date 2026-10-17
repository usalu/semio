/** 🧪️ Public surface of the repository test platform package `@semio-tech/repo-test`: the exports of its domain modules, one explicit list per module. */

//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

export {
  CORE_COMPARISON_PROFILES, implementationOfAdapterKey, isExcludedTestPath, isTestOraclePath, TAXONOMY_REL_PATH, TEST_DOMAIN_REL_PATH,
  testAdapterFilenames, testFeatureFilename, testFilenameForKind, testLocationPath, testTaxonomy,
} from "../../🗂️taxonomy/🟦️.ts";
export type {
  ComparisonProfileSpec, TestInPackageImplementation, TestTaxonomy,
} from "../../🗂️taxonomy/🟦️.ts";
export {
  digest, fileDigest, setDigest,
} from "../../🔢️digests/🟦️.ts";
export {
  canonicalize, compareProjections, coreProfileTable, projectionHash,
} from "../../🟰️comparison/🟦️.ts";
export type {
  ComparisonDiff, ComparisonVerdict,
} from "../../🟰️comparison/🟦️.ts";
export {
  discoverTestCases, repoRootFromHere, testProjectName,
} from "../../🧭️case-discovery/🟦️.ts";
export type {
  DiscoveredCase,
} from "../../🧭️case-discovery/🟦️.ts";
export {
  agentCacheRoot, markOutputDir, readOutputMarker, testCacheDir, testCacheRoot,
} from "../../⚡️output-cache/🟦️.ts";
export type {
  OutputMarker,
} from "../../⚡️output-cache/🟦️.ts";
export {
  ADAPTER_ENTRY_POINTS, ADAPTER_SUBJECT_REGISTRATION, adapterRegistersSubject, isJustifiedMutationDecision, oracleHostModule, oracleImplementation,
  oracleLinkedPackages, oraclePackages,
} from "../../🗃️oracle-registry/🟦️.ts";
export type {
  NoOracleDecision, OracleEntry, OracleHostPackage, OracleLinkedPackage, ReferenceSurvey, SubjectHostPackage,
} from "../../🗃️oracle-registry/🟦️.ts";
export {
  mutationCatalogProblems,
} from "../../🧮️mutation-catalog/🟦️.ts";
export type {
  MutationCatalog, MutationVector, MutationVectorScenario,
} from "../../🧮️mutation-catalog/🟦️.ts";
export {
  currentPlatform, declaredSubsetPolicies, declaredSubsets, isWildcardSubset, isWildcardSubsetFor, subsetCoordinate, subsetCoordinatesOfOwner,
  subsetPolicyIsSingle, surfaceCoordinate, surfaceProblem, WILDCARD_SUBSET_IDS,
} from "../../🪆️subset-policy/🟦️.ts";
export type {
  SurfaceTarget,
} from "../../🪆️subset-policy/🟦️.ts";
export { clearSchemaContractCache, forbiddenSchemaPlacements, isFixtureOwnedPath, matchesTaxonomyPathPattern, normativeSchemaFormat, parseSchemaUri, readSchemaCatalog, resolveSchemaExport, SCHEMA_DIAGNOSTIC_CODE_TABLE, SCHEMA_DIAGNOSTIC_CODES, SCHEMA_DIAGNOSTIC_EMITTERS, schemaDiagnosticCodesEmittedBy, schemaExportFile, schemaExportResolution, schemaFormatKey, schemaJsonDialect, schemaModuleDirName, schemaScopeFile, schemaScopeOwnerLevels, submodulePaths } from "../../📐️schema-contracts/🟦️.ts";
export { isDeclaredDialectDefinition, isJsonSchemaDefinition, schemaScopeEligibility, schemaTreeFiles } from "../../📐️schema-contracts/🏗️scope-eligibility-and-tree/🟦️.ts";
export { schemaFixtureIsolationDiagnostics, schemaMeasurementDisagreementDiagnostics, schemaModuleDirectoriesOnDisk, schemaOwnerEligibilityDiagnostics, schemaPlacementDiagnostics } from "../../📐️schema-contracts/🩺️placement-diagnostics/🟦️.ts";
export { declaredSchemaFormatIds, declaresSchemaExport, declaresSchemaExportParser, GRAPHQL_EXPORT_KEYWORDS, moduleExportSchema, mutationLeafSchemaId, restrictedExportFormats, SCHEMA_EXPORT_FORMATS_KEYWORD, schemaExportCompletenessDiagnostics, scopeNormativeDocuments } from "../../📐️schema-contracts/📤️export-completeness/🟦️.ts";
export { schemaContractDiagnostics, schemaResolutionDiagnostics } from "../../📐️schema-contracts/🔗️reference-diagnostics/🟦️.ts";
export type {
  ResolvedSchemaExport, SchemaCatalog, SchemaCatalogExport, SchemaCatalogScope, SchemaDiagnostic, SchemaDiagnosticCode,
  SchemaDiagnosticCodeEmittedBy, SchemaDiagnosticEmitter, SchemaExportResolution, SchemaHarnessDiagnosticCode, SchemaScopeEligibility,
  SchemaScopeOwnerLevels,
} from "../../📐️schema-contracts/🟦️.ts";
export {
  discoverSchemaFixtures, runSchemaFixture, SCHEMA_FIXTURE_STAGES, validateAgainstJsonSchema,
} from "../../✅️schema-validation/🟦️.ts";
export type {
  SchemaFixtureReport, SchemaFixtureStage, SchemaFixtureTarget, SchemaStageExpectation, SchemaStageOutcome, SchemaTestExample, SchemaValidationError,
} from "../../✅️schema-validation/🟦️.ts";
export {
  compareInventories, engineFamilyId, isMutationOutcomeClass, isQualifyingOracleKind, isSemioNativeArtifact, manifestTarget, mutationManifestDigest,
  mutationManifestProblems, owningSubsetOf, QUALIFYING_ORACLE_KINDS, readRuntimeInventory, runtimeInventoryPath, SUPPLEMENTAL_ORACLE_KINDS,
  writeRuntimeInventory,
} from "../../📑️mutation-manifest/🟦️.ts";
export type {
  InventoryEquality, ManifestMutation, MutationManifest, NativeSecondImplementationEvidence, OracleKind, OracleRequirement, QualifyingOracleKind,
  RuntimeMutationInventory, SupplementalOracleKind,
} from "../../📑️mutation-manifest/🟦️.ts";
export {
  isQualifiedProbe, probeReportProblems,
} from "../../🔬️probes/🟦️.ts";
export type {
  ProbeEntry, ProbeQualification, ProbeReport, ResultArtifact,
} from "../../🔬️probes/🟦️.ts";
export {
  CORE_TOLERANCE_PROFILES, resolveTolerance, resolveToleranceProfile,
} from "../../📏️tolerances/🟦️.ts";
export type {
  ResolvedTolerances, ToleranceProfile,
} from "../../📏️tolerances/🟦️.ts";
export {
  readResults, validateResult,
} from "../../📤️results/🟦️.ts";
export type {
  TestResult,
} from "../../📤️results/🟦️.ts";
export {
  evaluatePipeline, evaluateStageAssertions, executePipeline, pipelineRoleArtifacts,
} from "../../🔀️comparison-pipelines/🟦️.ts";
export type {
  AssertionVerdict, ComparisonPipeline, ComparisonStage, PipelineVerdict,
} from "../../🔀️comparison-pipelines/🟦️.ts";
export {
  clearContributionCache, contributionSchemaProblems, discoverTestContributions, loadOracleRegistry, manifestsOwning, oracleHostPackagesFor,
  pipelineTable, probeTable, profileTable, subjectHostPackagesFor, toleranceProfileTable,
} from "../../🤝️contributions/🟦️.ts";
export type {
  OracleRegistry, TestContribution,
} from "../../🤝️contributions/🟦️.ts";
export {
  agentId, heartbeatLease, LEASE_STALE_MS, LEASE_STATES, leasePath, leaseReclaimable, processAlive, PROTECTED_RETENTION_CLASSES, readLease,
  RETENTION_CLASSES, withAtomicRunDir, writeLease,
} from "../../🔐️run-lease/🟦️.ts";
export type {
  LeaseState, RetentionClass, RunLease,
} from "../../🔐️run-lease/🟦️.ts";
export {
  computeRunKey, runKey,
} from "../../🗝️run-key/🟦️.ts";
export type {
  RunKeyComponents, RunManifest,
} from "../../🗝️run-key/🟦️.ts";
export {
  contentDigest, contentDigestOf, fixtureBundleDigest, fixtureFilePath, testEvidenceProblems, verifyFixture,
} from "../../🧷️fixture-evidence/🟦️.ts";
export type {
  FixtureVerification,
} from "../../🧷️fixture-evidence/🟦️.ts";
export {
  fixtureBlobPath, fixtureBlobRoot, installEvidenceFile, installFixtureBlob, materializeFixtureBlob, publishTestEvidence, testEvidenceRoot,
} from "../../🗄️blob-storage/🟦️.ts";
export type {
  MaterializeMode,
} from "../../🗄️blob-storage/🟦️.ts";
export {
  fixtureFilesUnder, resolveTestInputs, testInputUrisIn,
} from "../../📥️fixture-inputs/🟦️.ts";
export {
  artifactOfOwner, binaryProtocolDriftBreaches, isMutationLeafDirectory, leafDescriptorCoverage, manifestFromLeafDescriptors,
  mutationLeafDirectories, outcomeClassesOf, payloadSchemaRelativePath, readLeafDescriptors, resolvePayloadSchema, resolvePayloadSchemas,
  scaffoldLeafDescriptor, scaffoldOwnerDescriptors,
} from "../../🌿️mutation-leaves/🟦️.ts";
export type {
  DerivedField, LeafScaffold, MutationLeafDescriptor, PayloadSchemaResolution,
} from "../../🌿️mutation-leaves/🟦️.ts";
export {
  classifyLegacyKind, DEPENDENCY_CLASSES, DEPENDENCY_ECOSYSTEMS, dependencyEcosystemOf, dependencyEcosystemOfRegistryValue, dotnetPackageReferences,
  externalOracleHostPackages, goProductionClosure, importProbe, isProductionClass, pythonRuntimeImports, ratchetDependencies,
  scanDeclaredDependencies,
} from "../../🔒️oracle-dependencies/🟦️.ts";
export type {
  ClassifiedDependency, DependencyClass, DependencyEcosystem, DependencyRatchet,
} from "../../🔒️oracle-dependencies/🟦️.ts";
export {
  TESTING_BREACH_KINDS,
} from "../../🚨️breaches/🟦️.ts";
export type {
  TestingBreachKind,
} from "../../🚨️breaches/🟦️.ts";
export {
  assessTestImplementationPath, inPackageTestImplementation, inspectTestLayoutSources, scanTestLayout, TEST_LAYOUT_FINDING_CODES,
  testImplementationFilenames, testLayoutBreaches,
} from "../../🗺️test-layout/🟦️.ts";
export type {
  TestLayoutFinding, TestLayoutFindingCode, TestLayoutProgress, TestLayoutScanOptions, TestLayoutSource,
} from "../../🗺️test-layout/🟦️.ts";
export {
  BINARY_CARRIER_FORMATS, capabilityManifestBreaches, engineIndependenceBreaches, fixtureProvenanceBreaches, fixtureWriterProvenanceBreaches,
  isolationBreaches, mutationFixtureBreaches, mutationInventoryBreaches, nativeSecondImplementationBreaches, noOracleMisuseBreaches,
  oracleRequirementBreaches, registryRecordBreaches, reimplementationOracleBreaches, stubDeserializerBreaches, stubSerializerBreaches,
  vectorReplayBreaches,
} from "../../🚧️mutation-gates/🟦️.ts";
export {
  caseAboveSubsetBreaches, mutationCoverageBreaches, mutationVectorRegistryBreaches, validateCaseContract,
} from "../../📃️case-contract/🟦️.ts";
export {
  caseContractBreaches, MIGRATION_STATUSES, migrationStatusByOwner, mutationVocabularyRequiresCatalog, oracleImportsInProduction,
  repositoryContractBreaches, validateAllContracts,
} from "../../🏛️repository-contract/🟦️.ts";
export type {
  MigrationStatus,
} from "../../🏛️repository-contract/🟦️.ts";
export {
  buildCasePlan, evaluateCrossSubjectParity, evaluateParity, levelsUpTo, markRunComplete, planExecution, subjectRawInputsByScenario, testId,
} from "../../🎬️execution-plan/🟦️.ts";
export {
  committedArtifact, makeAdapterContext, registeredHandler, validateRegistration,
} from "../../🔌️adapter-context/🟦️.ts";
export {
  cleanTestOutputs, collectGarbage, formatCleanReport, formatGcReport, markReferencedBlobs,
} from "../../🗑️retention/🟦️.ts";
export type {
  GcCandidate, GcReport, TestCleanRemoval, TestCleanReport,
} from "../../🗑️retention/🟦️.ts";
export {
  buildCoverageMatrix, computeCoverageMetrics, COVERAGE_DIMENSIONS, enforceMetricGates, enforceReleaseGates, formatCoverageQuestions, formatMetrics,
  measureCoverage, RELEASE_GATED_DIMENSIONS, renderDiff, renderJUnit, summarizeRun,
} from "../../📈️reporting/🟦️.ts";
export type {
  CoverageDimension, CoverageMetrics, CoverageRow, DimensionMeasurement, ImplementationCoverage, RunSummary,
} from "../../📈️reporting/🟦️.ts";
