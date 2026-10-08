import { parseGeneratorPreviewProgressPolicyV1, type GeneratorPreviewProgressPolicyV1 } from "../../🏭️generator/👁️preview/📈️progress/🟦️.ts";
import { requireRecord, requireStringArray, requireString, requireLiteral, requireExactKeys, type UnknownRecord } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
/** 🏗️ Canonical 🔣️taxonomy source service. */
import { validateTaxonomy, type Taxonomy as DiscoveryTaxonomy, createTaxonomyPathMatcher, parseNamedFixedDirectoryContractSetScope, parseFixedDirectoryContractSetScope, canonicalPrimaryFilenameForKind, parseSemanticOwnedDocumentCorrections, parseSemanticOwnedCurrentSourceRevisions, type SemanticPathProjectionReferenceConsumerForm, generatorPreviewScriptArguments, generatorPreviewResourceLimits, type RegistryCatalogInputDiscovery, type GeneratorProjectionActivation, type SemanticPackageGeneration, semanticOwnedInputFileSnapshot, type TaxonomyPathMatcher, type SemanticOwnedInputFileSnapshot, type SemanticFacetPrimaryFileProjectionContract } from "../../🔍️discovery/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { posix, relative, resolve } from "node:path";
import { normalizeRelative, splitLeadingEmoji, emojiFold } from "../🛣️path/🟦️.ts";
import { assertLexicalInputOutsideOpaque, noFollowDirectoryChain, verifyNoFollowDirectoryChain } from "../📁️input/🟦️.ts";

export interface TaxonomyLoadOptions { readonly repoRoot: string; readonly taxonomyPath?: string }

export interface FileKindSpec {
  readonly emoji: string;
  readonly extensionChains: readonly string[];
  readonly role: string;
}

interface SemanticDirectoryKindSpec {
  readonly emoji: string;
  readonly slugPattern: string;
  readonly allowEmojiOnly: boolean;
  readonly inferWithoutEmoji?: boolean;
  readonly projectionOnly?: boolean;
  readonly parentKindIds?: readonly string[];
}

interface SemanticLifecycleOwnedFileProjectionContract {
  readonly contractKind: "owner-sibling-manifest-file";
  readonly ownerFixedDirectoryContractId: string;
  readonly requiredSiblingFixedFilenameContractId: string;
  readonly manifestAdapter: "json";
  readonly manifestStatusLocation: "status";
  readonly allowedStatuses: readonly ["closed", "open"];
  readonly sourceFileKindId: string;
  readonly sourceFilename: string;
  readonly destinationDirectoryKindId: string;
  readonly destinationDirectoryName: string;
  readonly destinationFilename: string;
  readonly emptyContentRule: "zero-byte";
  readonly statusDispositions: Readonly<{ readonly open: "project"; readonly "closed-empty": "remove"; readonly "closed-nonempty": "problem"; readonly invalid: "problem" }>;
  readonly rationaleRule: "ticket-important-markdown-projection-v1";
}

interface SemanticHistoryOwnedFileProjectionContract {
  readonly contractKind: "owner-optional-sibling-manifest-file";
  readonly ownerFixedDirectoryContractId: string;
  readonly optionalSiblingFixedFilenameContractId: string;
  readonly manifestAdapter: "json";
  readonly manifestStatusLocation: "status";
  readonly sourceFileKindId: string;
  readonly sourceFilename: string;
  readonly destinationDirectoryKindId: string;
  readonly destinationDirectoryName: string;
  readonly destinationFilename: string;
  readonly admittedDispositions: readonly ["closed-nonzero", "invalid-manifest", "missing-manifest"];
  readonly rationaleRule: "ticket-important-history-markdown-v1";
}

interface SemanticExactOwnedFileProjectionContract {
  readonly contractKind: "exact-owner-path-catalog";
  readonly authorityCatalogPath: string;
  readonly authorityCatalogSha256: string;
  readonly sourceFileKindId: "markdown";
  readonly sourceBasenames: readonly ["LICENSE.md", "README.md"];
  readonly destinationDirectoryKinds: Readonly<{
    readonly license: Readonly<{ readonly directoryKindId: "owner-license"; readonly directoryName: "⚖️license"; readonly filename: "📝️.md" }>;
    readonly readme: Readonly<{ readonly directoryKindId: "owner-readme"; readonly directoryName: "📃️readme"; readonly filename: "📝️.md" }>;
  }>;
  readonly allowedDispositions: readonly ["attribution-relocate", "configurable-owner-license-relocate", "fixed", "generated-evidence-relocate", "owner-documentation-relocate"];
  readonly ownerEvidenceKinds: readonly ["configurable-owner-license", "ordinary-owner-doc", "package-publication", "third-party-attribution", "ticket-evidence", "ticket-scratch"];
  readonly referenceOwnerIds: readonly ["asset-distribution-owner", "bun-package-publisher", "commonmark-scratch-rust-reader", "markdown-relative-reference-adapter", "repo-cli-dev-docs-go", "vscode-package-ignore"];
  readonly generatorOwnerIds: readonly ["assets-build"];
  readonly expectedCounts: Readonly<{ readonly fixed: 4; readonly license: 8; readonly projected: 36; readonly readme: 32; readonly referenceBindings: 62; readonly total: 40 }>;
  readonly authoredDocumentCorrections: ReturnType<typeof parseSemanticOwnedDocumentCorrections>;
  readonly currentSourceRevisions?: ReturnType<typeof parseSemanticOwnedCurrentSourceRevisions>;
  readonly rationaleRule: "readme-license-owner-projection-v1";
}

interface SemanticPrimaryOwnedFileProjectionContract {
  readonly contractKind: "owner-primary-file";
  readonly ownerFixedDirectoryContractId: string;
  readonly sourceFileKindId: string;
  readonly sourceFilename: string;
  readonly destinationFilename: string;
  readonly rationaleRule: "ticket-document-primary-markdown-v1";
}

type SemanticOwnedFileProjectionContract = SemanticExactOwnedFileProjectionContract | SemanticFacetPrimaryFileProjectionContract | SemanticHistoryOwnedFileProjectionContract | SemanticLifecycleOwnedFileProjectionContract | SemanticPrimaryOwnedFileProjectionContract;

export type FixedContractScope = Readonly<
  | { kind: "exact-path"; path: string }
  | { kind: "repository-root" }
  | { kind: "package-root"; ecosystemId: string }
  | { kind: "directory-kind"; directoryKindId: string }
  | { kind: "fixed-directory-contract"; fixedDirectoryContractId: string }
  | { kind: "fixed-directory-contract-set"; fixedDirectoryContractIds: readonly string[] }
  | { kind: "sibling-fixed-filename-contract"; fixedFilenameContractId: string }
  | { kind: "path-pattern" }
>;

export interface FixedFilenameContract {
  readonly pathPattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly configurability: "unconfigurable";
  readonly scope: FixedContractScope;
  readonly verification: string;
  readonly expires: string | null;
}

export interface FixedDirectoryContract {
  readonly pathPattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly configurability: "unconfigurable";
  readonly scope: Exclude<FixedContractScope, { readonly kind: "package-root" }>;
  readonly verification: string;
  readonly expires: string | null;
}

interface FixedFilenameRejectionContract {
  readonly sourcePathIdentities: readonly string[];
  readonly disposition: "normalize" | "relocate";
  readonly reason: string;
}

export interface ConfigurableEntryContract {
  readonly filename: string;
  readonly fileKindId: string;
  readonly ecosystemId: string;
  readonly role: string;
  readonly configurationSources: readonly string[];
}

interface FileKindResolutionRuleSpec {
  readonly extensionChain: string;
  readonly fileKindId: string;
  readonly priority: number;
  readonly filenamePattern?: string;
  readonly pathPattern?: string;
  readonly parentKindIds?: readonly string[];
  readonly ancestorKindIds?: readonly string[];
}

interface ScopedFileKindSpec {
  readonly pathPattern: string;
  readonly parentDirectoryKindId?: string;
  readonly emoji: string;
  readonly extensionChains: readonly string[];
  readonly role: string;
  readonly sourceFilenamePattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly verification: string;
  readonly expires: string | null;
}

interface SemanticDirectoryMemberKindSpec {
  readonly ownerKindIds: readonly string[];
  readonly memberNames: readonly string[];
  readonly source: "registry";
}

interface SemanticProjectedMemberKindSpec {
  readonly ownerKindIds: readonly string[];
  readonly projectionContractId: string;
  readonly sourceMemberKindId: string;
  readonly identityField: "mutationDirectoryName" | "commandDirectoryName";
}

export type SemanticProjectionCaptureField = "standardVersion" | "subsetId" | "mutationId" | "scenarioId" | "commandDirectoryName";

type SemanticProjectionSourceSegment = Readonly<{ kindId: string; literal: string } | { kindId: string; capture: SemanticProjectionCaptureField } | { memberKindId: string; literal: string } | { projectedMemberKindId: string; capture: SemanticProjectionCaptureField }>;

type SemanticProjectionDestinationSegment = Readonly<{ kindId: string; literal: string } | { kindId: string; render: "profile" } | { kindId: string; copy: SemanticProjectionCaptureField } | { projectedMemberKindId: string; copy: SemanticProjectionCaptureField }>;

interface SemanticPathProjectionProfileRenderer {
  readonly direction: "forward-only";
  readonly captureFields: readonly ["standardVersion", "subsetId"];
  readonly directoryKindId: string;
  readonly template: "🪆️{standardVersion}-{subsetId}";
  readonly tupleCollisionFields: readonly ["artifactId", "standardVersion", "subsetId"];
}

export interface SemanticDescendantKindNode {
  readonly pathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly nodeType: "directory" | "file";
  readonly kindId: string;
  readonly sourceFilename?: string;
}

interface SemanticDescendantFixedFileNode {
  readonly pathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly nodeType: "file";
  readonly fixedFilenameContractId: string;
}

type SemanticDescendantNode = SemanticDescendantKindNode | SemanticDescendantFixedFileNode;

interface SemanticDescendantAlternative {
  readonly id: string;
  readonly mode: "exactly-one";
  readonly nodes: readonly SemanticDescendantNode[];
}

export interface SemanticExactDescendantContract {
  readonly rootDirectoryKindId: string;
  readonly requiredNodes: readonly SemanticDescendantNode[];
  readonly exclusiveAlternatives: readonly SemanticDescendantAlternative[];
  readonly realizedNodeCount: number;
  readonly pathBudgetReserve: Readonly<{ derivation: "longest-canonical-descendant-suffix"; bytes: number }>;
}

interface SemanticCatalogDescendantContract {
  readonly contractKind: "catalog";
  readonly rootDirectoryKindId: string;
  readonly catalogContractId: string;
  readonly leafFileKindId: string;
  readonly rendering: "semantic-member-directory-and-physical-kind-leaf";
  readonly pathBudgetReserve: Readonly<{ derivation: "longest-rendered-catalog-descendant-suffix"; bytes: number }>;
}

type SemanticDescendantContract = SemanticExactDescendantContract | SemanticCatalogDescendantContract;

interface SemanticMutationPathProjectionCatalogContract {
  readonly registryField: "vectors";
  readonly required: true;
  readonly allowEmpty: true;
  readonly runtimeKindsField: "kinds";
  readonly runtimeKindsRelation: "independent";
  readonly mutationIdField: "mutationId";
  readonly sourceMutationDirectoryNameField: "sourceMutationDirectoryName";
  readonly mutationDirectoryNameField: "mutationDirectoryName";
  readonly scenariosField: "scenarios";
  readonly scenarioIdField: "id";
  readonly scenarioDirectoryNameField: "directoryName";
  readonly sourceBundleUniquenessFields: readonly ["mutationId", "sourceMutationDirectoryName", "scenarioId"];
  readonly canonicalBundleUniquenessFields: readonly ["mutationId", "mutationDirectoryName", "scenarioId"];
  readonly coverage: "every-physical-bundle-exactly-once";
}

export interface SemanticDistributedJsonManifestCatalogContract {
  readonly contractKind: "distributed-json-manifest-catalog";
  readonly ownerArtifactMemberName: string;
  readonly profileVectors: readonly Readonly<{ artifactId: string; standardVersion: string; subsetId: string }>[];
  readonly modelManifestSchema: string;
  readonly modelManifestSourceFilename: string;
  readonly modelIdentityField: "id";
  readonly memberIdentityField: "id";
  readonly memberVersionField: "version";
  readonly requiredMemberVersion: string;
  readonly requiredModelManifest: true;
  readonly categoryRules: readonly Readonly<{ sourceDirectoryName: string; directoryKindId: string; sourceShape: "direct-semantic-json"; manifestSchema: string; memberDirectoryEmoji: string } | { sourceDirectoryName: string; directoryKindId: string; sourceShape: "nested-fixed-json"; manifestSchema: string; fixedSourceFilename: string }>[];
  readonly coverage: "every-source-file-and-destination-node-exactly-once";
  readonly unknownCategoryPolicy: "problem";
  readonly unownedModelPolicy: "problem";
}

export interface SemanticExactOwnerVectorsCatalogContract {
  readonly contractKind: "exact-owner-vectors";
  readonly required: true;
  readonly allowEmpty: false;
  readonly identityFields: readonly ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"];
  readonly coverage: "every-physical-command-bundle-exactly-once";
  readonly vectors: readonly Readonly<{ artifactId: string; standardVersion: string; subsetId: string; commandDirectoryName: string }>[];
}

type SemanticPathProjectionCatalogContract = SemanticMutationPathProjectionCatalogContract | SemanticDistributedJsonManifestCatalogContract | SemanticExactOwnerVectorsCatalogContract;

export interface SemanticPathProjectionContract {
  readonly sourceOwnerKindId: string;
  readonly sourceArtifactMemberName?: string;
  readonly sourceSegments: readonly SemanticProjectionSourceSegment[];
  readonly profileRendererId: string;
  readonly destinationOwnerKindId: string;
  readonly destinationSegments: readonly SemanticProjectionDestinationSegment[];
  readonly descendantContractId: string;
  readonly catalogContractId: string;
  readonly rationaleRule: "artifact-example-model-catalog-projection-v1" | "artifact-editor-command-projection-v1";
}

export interface SemanticPathProjectionReferenceConsumerContract {
  readonly projectionContractId: string;
  readonly consumerIdentity: string;
  readonly ownership: "external";
  readonly sourcePathPattern: string;
  readonly sourcePathIdentities: readonly string[];
  readonly adapters: readonly ("rust" | "typescript" | "json" | "toml")[];
  readonly supportedForms: readonly SemanticPathProjectionReferenceConsumerForm[];
  readonly staleMarkers: readonly string[];
}

interface MutationCatalogProjectionContractIds {
  readonly contractKind: "canonical-mutation-case-pair";
  readonly contractId: "canonical-mutation-case-pair-v1";
  readonly sourceOwnerKindId: string;
  readonly projectedMemberKindId: string;
  readonly implementationSegments: readonly SemanticProjectionSourceSegment[];
  readonly fixtureSegments: readonly SemanticProjectionSourceSegment[];
  readonly implementationDescendantContractId: string;
  readonly fixtureDescendantContractId: string;
  readonly catalogContractId: string;
  readonly coverage: "every-catalog-vector-has-one-implementation-and-one-fixture-bundle";
}

type GeneratorOwnership = "owned" | "external";

interface GeneratorOutputRootSpec {
  readonly path: string;
  readonly inclusion: "tracked" | "ignored";
}

export interface GeneratorContractSpec {
  readonly ownership: GeneratorOwnership;
  readonly ownerPath: string | null;
  readonly target: string | null;
  readonly previewTarget?: string;
  readonly previewArguments?: readonly string[];
  readonly previewProgress?: GeneratorPreviewProgressPolicyV1;
  readonly previewLimits?: { readonly maxOutputBytes: number; readonly timeoutMs: number };
  readonly compilerInputManifest?: { readonly kind: "compiler-input-manifest-v1"; readonly manifestOutputPath: string; readonly manifestSchemaPath: string; readonly staticAuthorityPath: string; readonly maxFiles: number };
  readonly checkTarget?: string;
  readonly inputPatterns: readonly string[];
  readonly inputDiscovery?: RegistryCatalogInputDiscovery;
  readonly packageGeneration?: SemanticPackageGeneration;
  readonly projectionActivation?: GeneratorProjectionActivation;
  readonly outputRoots: readonly GeneratorOutputRootSpec[];
  readonly reason: string;
}

export interface PackageBoundaryRule {
  readonly manifestContractId: string | null;
  readonly entryContractIds: readonly string[];
  readonly allowedFixedContractIds: readonly string[];
  readonly allowedFileKindIds: readonly string[];
  readonly allowedDirectoryKindIds: readonly string[];
  readonly glueGrammarId: string;
  readonly recursive: true;
  readonly uncertainRole: "problem";
  readonly implementationRole: "problem";
}

interface PackageBoundaryProfile {
  readonly admission: "blocked-until-language-directory-registered";
  readonly allowedFileKindIds: readonly string[];
  readonly allowedDirectoryKindIds: readonly string[];
  readonly allowedFixedContractIds: readonly string[];
  readonly glueGrammarId: string;
  readonly recursive: true;
  readonly uncertainRole: "problem";
  readonly implementationRole: "problem";
  readonly reason: string;
}

interface PackageSourceDisposition {
  readonly contractKind: "fixed" | "configurable";
  readonly disposition: "adapter-source" | "tool-metadata";
  readonly validator: "package-glue" | "command-router" | "vitest-configuration" | "tool-config-vitest" | "tool-config-tailwind" | "tool-config-postcss" | "tool-config-eslint" | "tool-config-dependency-cruiser" | "pytest-configuration" | "eslint-configuration" | "vscode-test-configuration";
  readonly grammarId?: string;
  readonly authority: string;
  readonly verification: string;
}

interface EcosystemSpec {
  readonly packageIdentity: "manifest" | "boundary-only";
  readonly manifestContractId: string | null;
}

export interface PackageGlueGrammar {
  readonly analyzer: "rust" | "typescript" | "javascript" | "go" | "python" | "dotnet" | "c-cpp" | "tex";
  readonly allowedRoles: readonly ("declaration" | "registration" | "bootstrap" | "thin-delegation")[];
  readonly maxDelegationStatements: number;
}

interface TaxonomyV7 {
  readonly schemaVersion: 7;
  readonly windowEmptyFacetFileKindId: string;
  readonly fileKinds: Readonly<Record<string, FileKindSpec>>;
  readonly semanticDirectoryKinds: Readonly<Record<string, SemanticDirectoryKindSpec>>;
  readonly fixedFilenameContracts: Readonly<Record<string, FixedFilenameContract>>;
  readonly fixedFilenameRejectionContracts: Readonly<Record<string, FixedFilenameRejectionContract>>;
  readonly fixedDirectoryContracts: Readonly<Record<string, FixedDirectoryContract>>;
  readonly configurableEntryContracts: Readonly<Record<string, ConfigurableEntryContract>>;
  readonly fileKindResolutionRules: Readonly<Record<string, FileKindResolutionRuleSpec>>;
  readonly scopedFileKinds: Readonly<Record<string, ScopedFileKindSpec>>;
  readonly semanticDirectoryMemberKinds: Readonly<Record<string, SemanticDirectoryMemberKindSpec>>;
  readonly semanticProjectedMemberKinds: Readonly<Record<string, SemanticProjectedMemberKindSpec>>;
  readonly semanticPathProjectionProfileRenderers: Readonly<Record<string, SemanticPathProjectionProfileRenderer>>;
  readonly semanticDescendantContracts: Readonly<Record<string, SemanticDescendantContract>>;
  readonly semanticPathProjectionCatalogContracts: Readonly<Record<string, SemanticPathProjectionCatalogContract>>;
  readonly semanticPathProjectionContracts: Readonly<Record<string, SemanticPathProjectionContract>>;
  readonly semanticOwnedFileProjectionContracts: Readonly<Record<string, SemanticOwnedFileProjectionContract>>;
  readonly semanticPackageProjectionContracts: DiscoveryTaxonomy["semanticPackageProjectionContracts"];
  readonly semanticPathProjectionReferenceConsumerContracts: Readonly<Record<string, SemanticPathProjectionReferenceConsumerContract>>;
  readonly mutationCatalogProjection: MutationCatalogProjectionContractIds;
  readonly generatorContracts: Readonly<Record<string, GeneratorContractSpec>>;
  readonly ecosystems: Readonly<Record<string, EcosystemSpec>>;
  readonly packageBoundaryRules: Readonly<Record<string, PackageBoundaryRule>>;
  readonly packageBoundaryProfiles: Readonly<Record<string, PackageBoundaryProfile>>;
  readonly packageGlueGrammar: Readonly<Record<string, PackageGlueGrammar>>;
  readonly packageSourceDispositions: Readonly<Record<string, PackageSourceDisposition>>;
  readonly pathExclusions: Readonly<Record<string, { readonly path: string; readonly mode: "opaque"; readonly reason: string }>>;
  readonly unicodeNormalization: { readonly form: "NFC"; readonly caseFold: "lower"; readonly locale: "und" };
  readonly variationSelectorPolicy: { readonly selector: "\uFE0F"; readonly requiredAfterEmoji: true; readonly comparison: "ignore-selector" };
  readonly collisionPolicy: {
    readonly comparisons: readonly ("byte" | "nfc" | "case-fold" | "vs16-fold" | "same-kind")[];
    readonly maxPathBytes: number;
    readonly rejectWindowsReservedNames: boolean;
    readonly rejectTrailingDotsAndSpaces: boolean;
  };
  readonly areaEnforcement: {
    readonly requiredState: "clean";
    readonly undeclaredAreas: "enforce";
    readonly opaquePathExclusionIds: readonly string[];
  };
}

interface TaxonomyContentFacts {
  readonly schema: TaxonomyV7;
  readonly discoverySchema: DiscoveryTaxonomy;
  readonly exclusions: readonly { readonly id: string; readonly path: string }[];
  readonly fileKinds: readonly (FileKindSpec & { readonly id: string })[];
  readonly directoryKinds: readonly (SemanticDirectoryKindSpec & { readonly id: string; readonly slugRegex: RegExp })[];
}

export interface LoadedTaxonomy extends TaxonomyContentFacts {
  readonly path: string;
  readonly pathMatcher: TaxonomyPathMatcher;
  readonly input: SemanticOwnedInputFileSnapshot;
}

export const TAXONOMY_RELATIVE_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";

function fixedExpiry(value: unknown, name: string): string | null {
  if (value === null) return null;
  const expires = requireString(value, name);
  if (!/^\d{4}-\d{2}-\d{2}$/u.test(expires)) throw new Error(`Taxonomy v7 ${name} must be null or YYYY-MM-DD`);
  return expires;
}

/** 🚧️ The exact opaque subtrees `🔣️taxonomy.json` may declare, in order: two user-owned scratch
 * trees plus the one tracked nested-repository gitlink (`git ls-files -s` mode `160000`), which must be
 * filtered lexically here or `inventoryTaxonomyWithSourceParentPruning` refuses to classify anything at
 * all. Mirrors `🔍️discovery/🟦️.ts`'s `OPAQUE_PATH_EXCLUSIONS`; the two files are separate bundles with
 * no shared import path for a three-row table. The gitlink is named, never its parent `♻️mit-bestand/`,
 * whose `📋️bericht`/`🖼️asset` subtrees are generator-contract outputs and may not sit inside an opaque
 * subtree. */
const TAXONOMY_OPAQUE_PATH_EXCLUSIONS: readonly (readonly [string, string])[] = [
  ["compose", "compose/"],
  ["temp-compose", "temp/compose/"],
  ["mit-bestand-recherche", "♻️mit-bestand/🔎️recherche/"],
];

function parseTaxonomy(raw: unknown, path: string): TaxonomyContentFacts {
  const root = requireRecord(raw, "root");
  if (root.schemaVersion !== 7) throw new Error(`Taxonomy schemaVersion must be 7 at ${path}`);
  const discoveryProblems = validateTaxonomy(root as unknown as DiscoveryTaxonomy);
  if (discoveryProblems.length > 0) throw new Error(`Taxonomy v7 discovery contract validation failed at ${path}: ${discoveryProblems.join(" | ")}`);
  const pathMatcher = createTaxonomyPathMatcher();
  function validatedContractPattern(value: unknown, name: string, exactBasename: boolean): string {
    const pattern = requireString(value, name);
    if (pattern !== pattern.normalize("NFC") || pattern.startsWith("/") || pattern.endsWith("/") || pattern.includes("\\") || pattern.includes("//") || pattern.includes("\u0000")) throw new Error(`Taxonomy v7 ${name} must be one NFC workspace-relative POSIX pattern`);
    if (/[{}]/u.test(pattern) || /^!/u.test(pattern) || /[!@+?*]\(/u.test(pattern)) throw new Error(`Taxonomy v7 ${name} uses unsupported glob syntax`);
    for (const segment of pattern.split("/")) {
      if (segment.includes("**") && segment !== "**") throw new Error(`Taxonomy v7 ${name} may use ** only as a whole segment`);
      for (const match of segment.matchAll(/\[([^\]]*)\]/gu)) if (!/^[A-Za-z0-9-]+$/u.test(match[1]) || /^[!^]/u.test(match[1])) throw new Error(`Taxonomy v7 ${name} has an invalid character class`);
      if ((segment.match(/\[/gu)?.length ?? 0) !== (segment.match(/\]/gu)?.length ?? 0)) throw new Error(`Taxonomy v7 ${name} has an unclosed character class`);
    }
    const filename = pattern.slice(pattern.lastIndexOf("/") + 1);
    if (exactBasename && /[*?\[\]{}]/u.test(filename)) throw new Error(`Taxonomy v7 ${name} must end in one exact literal basename`);
    pathMatcher.matches("", pattern);
    return pattern;
  }
  const fileKindRows = requireRecord(root.fileKinds, "fileKinds");
  const directoryKindRows = requireRecord(root.semanticDirectoryKinds, "semanticDirectoryKinds");
  const fixedRows = requireRecord(root.fixedFilenameContracts, "fixedFilenameContracts");
  const fixedRejectionRows = requireRecord(root.fixedFilenameRejectionContracts, "fixedFilenameRejectionContracts");
  const fixedDirectoryRows = requireRecord(root.fixedDirectoryContracts, "fixedDirectoryContracts");
  const configurableRows = requireRecord(root.configurableEntryContracts, "configurableEntryContracts");
  const fileResolutionRows = requireRecord(root.fileKindResolutionRules, "fileKindResolutionRules");
  const scopedFileRows = requireRecord(root.scopedFileKinds, "scopedFileKinds");
  const directoryMemberRows = requireRecord(root.semanticDirectoryMemberKinds, "semanticDirectoryMemberKinds");
  const projectedMemberRows = requireRecord(root.semanticProjectedMemberKinds, "semanticProjectedMemberKinds");
  const projectionRendererRows = requireRecord(root.semanticPathProjectionProfileRenderers, "semanticPathProjectionProfileRenderers");
  const descendantContractRows = requireRecord(root.semanticDescendantContracts, "semanticDescendantContracts");
  const projectionCatalogRows = requireRecord(root.semanticPathProjectionCatalogContracts, "semanticPathProjectionCatalogContracts");
  const projectionRows = requireRecord(root.semanticPathProjectionContracts, "semanticPathProjectionContracts");
  const ownedFileProjectionRows = requireRecord(root.semanticOwnedFileProjectionContracts, "semanticOwnedFileProjectionContracts");
  const projectionConsumerRows = requireRecord(root.semanticPathProjectionReferenceConsumerContracts, "semanticPathProjectionReferenceConsumerContracts");
  const mutationCatalogProjectionRow = requireRecord(root.mutationCatalogProjection, "mutationCatalogProjection");
  const generatorRows = requireRecord(root.generatorContracts, "generatorContracts");
  const ecosystemRows = requireRecord(root.ecosystems, "ecosystems");
  const boundaryRows = requireRecord(root.packageBoundaryRules, "packageBoundaryRules");
  const boundaryProfileRows = requireRecord(root.packageBoundaryProfiles, "packageBoundaryProfiles");
  const grammarRows = requireRecord(root.packageGlueGrammar, "packageGlueGrammar");
  const sourceDispositionRows = requireRecord(root.packageSourceDispositions, "packageSourceDispositions");
  const exclusionRows = requireRecord(root.pathExclusions, "pathExclusions");
  const unicode = requireRecord(root.unicodeNormalization, "unicodeNormalization");
  const selector = requireRecord(root.variationSelectorPolicy, "variationSelectorPolicy");
  const collision = requireRecord(root.collisionPolicy, "collisionPolicy");
  const enforcement = requireRecord(root.areaEnforcement, "areaEnforcement");
  if (unicode.form !== "NFC" || unicode.caseFold !== "lower" || unicode.locale !== "und") throw new Error("Taxonomy v7 unicodeNormalization must select NFC/lower/und");
  if (selector.selector !== "\uFE0F" || selector.requiredAfterEmoji !== true || selector.comparison !== "ignore-selector") throw new Error("Taxonomy v7 variationSelectorPolicy is not canonical");
  const requiredComparisons = ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"];
  if (canonicalJson(collision.comparisons) !== canonicalJson(requiredComparisons) || !Number.isSafeInteger(collision.maxPathBytes) || (collision.maxPathBytes as number) < 1 || collision.rejectWindowsReservedNames !== true || collision.rejectTrailingDotsAndSpaces !== true) throw new Error("Taxonomy v7 collisionPolicy is incomplete");
  if (enforcement.requiredState !== "clean" || enforcement.undeclaredAreas !== "enforce") throw new Error("Taxonomy v7 areaEnforcement must enforce clean undeclared areas");

  const fileKinds: Record<string, FileKindSpec> = {};
  for (const [id, value] of Object.entries(fileKindRows)) {
    const spec = requireRecord(value, `fileKinds.${id}`);
    const emoji = requireString(spec.emoji, `fileKinds.${id}.emoji`).normalize("NFC");
    const extensionChains = requireStringArray(spec.extensionChains, `fileKinds.${id}.extensionChains`);
    if (extensionChains.length === 0 || extensionChains.some((chain) => !chain.startsWith("."))) throw new Error(`Taxonomy v7 fileKinds.${id}.extensionChains must contain dotted chains`);
    fileKinds[id] = { emoji, extensionChains: [...new Set(extensionChains)].sort((a, b) => b.length - a.length || a.localeCompare(b)), role: requireString(spec.role, `fileKinds.${id}.role`) };
  }
  if (Object.keys(fileKinds).length === 0) throw new Error("Taxonomy v7 fileKinds must not be empty");

  const semanticDirectoryKinds: Record<string, SemanticDirectoryKindSpec> = {};
  for (const [id, value] of Object.entries(directoryKindRows)) {
    const spec = requireRecord(value, `semanticDirectoryKinds.${id}`);
    const emoji = requireString(spec.emoji, `semanticDirectoryKinds.${id}.emoji`).normalize("NFC");
    const slugPattern = requireString(spec.slugPattern, `semanticDirectoryKinds.${id}.slugPattern`);
    new RegExp(slugPattern, "u");
    if (typeof spec.allowEmojiOnly !== "boolean") throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.allowEmojiOnly must be boolean`);
    if (spec.inferWithoutEmoji !== undefined && typeof spec.inferWithoutEmoji !== "boolean") throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.inferWithoutEmoji must be boolean when present`);
    if (spec.projectionOnly !== undefined && typeof spec.projectionOnly !== "boolean") throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.projectionOnly must be boolean when present`);
    semanticDirectoryKinds[id] = { emoji, slugPattern, allowEmojiOnly: spec.allowEmojiOnly, ...(spec.inferWithoutEmoji === undefined ? {} : { inferWithoutEmoji: spec.inferWithoutEmoji }), ...(spec.projectionOnly === undefined ? {} : { projectionOnly: spec.projectionOnly }), ...(spec.parentKindIds === undefined ? {} : { parentKindIds: requireStringArray(spec.parentKindIds, `semanticDirectoryKinds.${id}.parentKindIds`) }) };
  }
  if (Object.keys(semanticDirectoryKinds).length === 0) throw new Error("Taxonomy v7 semanticDirectoryKinds must not be empty");

  const fixedFilenameContracts: Record<string, FixedFilenameContract> = {};
  for (const [id, value] of Object.entries(fixedRows)) {
    const spec = requireRecord(value, `fixedFilenameContracts.${id}`);
    if (spec.configurability !== "unconfigurable") throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.configurability must be unconfigurable`);
    const inputScopeRow = requireRecord(spec.scope, `fixedFilenameContracts.${id}.scope`);
    const namedScope = inputScopeRow.kind === "named-fixed-directory-contract-set" ? parseNamedFixedDirectoryContractSetScope(inputScopeRow, root.fixedDirectoryContracts as DiscoveryTaxonomy["fixedDirectoryContracts"], (root.fixedDirectoryContractSets ?? {}) as NonNullable<DiscoveryTaxonomy["fixedDirectoryContractSets"]>) : undefined;
    const scopeRow: UnknownRecord = namedScope ? { kind: namedScope.kind, fixedDirectoryContractIds: [...namedScope.fixedDirectoryContractIds] } : inputScopeRow;
    const scopeKind = requireString(scopeRow.kind, `fixedFilenameContracts.${id}.scope.kind`) as FixedContractScope["kind"];
    if (!["exact-path", "repository-root", "package-root", "directory-kind", "fixed-directory-contract", "fixed-directory-contract-set", "sibling-fixed-filename-contract", "path-pattern"].includes(scopeKind)) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.kind is invalid`);
    const scope: FixedContractScope = scopeKind === "exact-path"
      ? (requireExactKeys(scopeRow, ["kind", "path"], `fixedFilenameContracts.${id}.scope`), { kind: "exact-path", path: normalizeRelative(requireString(scopeRow.path, `fixedFilenameContracts.${id}.scope.path`)) })
      : scopeKind === "package-root"
        ? (requireExactKeys(scopeRow, ["kind", "ecosystemId"], `fixedFilenameContracts.${id}.scope`), { kind: "package-root", ecosystemId: requireString(scopeRow.ecosystemId, `fixedFilenameContracts.${id}.scope.ecosystemId`) })
        : scopeKind === "directory-kind"
          ? (requireExactKeys(scopeRow, ["kind", "directoryKindId"], `fixedFilenameContracts.${id}.scope`), { kind: "directory-kind", directoryKindId: requireString(scopeRow.directoryKindId, `fixedFilenameContracts.${id}.scope.directoryKindId`) })
          : scopeKind === "fixed-directory-contract"
            ? (requireExactKeys(scopeRow, ["kind", "fixedDirectoryContractId"], `fixedFilenameContracts.${id}.scope`), { kind: "fixed-directory-contract", fixedDirectoryContractId: requireString(scopeRow.fixedDirectoryContractId, `fixedFilenameContracts.${id}.scope.fixedDirectoryContractId`) })
            : scopeKind === "fixed-directory-contract-set"
              ? parseFixedDirectoryContractSetScope(scopeRow, root.fixedDirectoryContracts as DiscoveryTaxonomy["fixedDirectoryContracts"])
            : scopeKind === "sibling-fixed-filename-contract"
              ? (requireExactKeys(scopeRow, ["kind", "fixedFilenameContractId"], `fixedFilenameContracts.${id}.scope`), { kind: "sibling-fixed-filename-contract", fixedFilenameContractId: requireString(scopeRow.fixedFilenameContractId, `fixedFilenameContracts.${id}.scope.fixedFilenameContractId`) })
          : (requireExactKeys(scopeRow, ["kind"], `fixedFilenameContracts.${id}.scope`), { kind: scopeKind });
    if (scope.kind === "directory-kind" && !semanticDirectoryKinds[scope.directoryKindId]) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.directoryKindId is invalid`);
    fixedFilenameContracts[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `fixedFilenameContracts.${id}.pathPattern`, true),
      authority: requireString(spec.authority, `fixedFilenameContracts.${id}.authority`),
      reason: requireString(spec.reason, `fixedFilenameContracts.${id}.reason`),
      configurability: "unconfigurable",
      scope,
      verification: requireString(spec.verification, `fixedFilenameContracts.${id}.verification`),
      expires: fixedExpiry(spec.expires, `fixedFilenameContracts.${id}.expires`),
    };
  }

  const fixedDirectoryContracts: Record<string, FixedDirectoryContract> = {};
  for (const [id, value] of Object.entries(fixedDirectoryRows)) {
    const spec = requireRecord(value, `fixedDirectoryContracts.${id}`);
    if (spec.configurability !== "unconfigurable") throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.configurability must be unconfigurable`);
    const scopeRow = requireRecord(spec.scope, `fixedDirectoryContracts.${id}.scope`);
    const scopeKind = requireString(scopeRow.kind, `fixedDirectoryContracts.${id}.scope.kind`);
    if (!["exact-path", "repository-root", "directory-kind", "path-pattern"].includes(scopeKind)) throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.scope.kind is invalid`);
    const scope: FixedDirectoryContract["scope"] = scopeKind === "exact-path"
      ? (requireExactKeys(scopeRow, ["kind", "path"], `fixedDirectoryContracts.${id}.scope`), { kind: "exact-path", path: normalizeRelative(requireString(scopeRow.path, `fixedDirectoryContracts.${id}.scope.path`)) })
      : scopeKind === "directory-kind"
        ? (requireExactKeys(scopeRow, ["kind", "directoryKindId"], `fixedDirectoryContracts.${id}.scope`), { kind: "directory-kind", directoryKindId: requireString(scopeRow.directoryKindId, `fixedDirectoryContracts.${id}.scope.directoryKindId`) })
        : (requireExactKeys(scopeRow, ["kind"], `fixedDirectoryContracts.${id}.scope`), { kind: scopeKind as "repository-root" | "path-pattern" });
    if (scope.kind === "directory-kind" && !semanticDirectoryKinds[scope.directoryKindId]) throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.scope.directoryKindId is invalid`);
    fixedDirectoryContracts[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `fixedDirectoryContracts.${id}.pathPattern`, false),
      authority: requireString(spec.authority, `fixedDirectoryContracts.${id}.authority`),
      reason: requireString(spec.reason, `fixedDirectoryContracts.${id}.reason`),
      configurability: "unconfigurable",
      scope,
      verification: requireString(spec.verification, `fixedDirectoryContracts.${id}.verification`),
      expires: fixedExpiry(spec.expires, `fixedDirectoryContracts.${id}.expires`),
    };
  }
  if (Object.keys(fixedDirectoryContracts).length === 0) throw new Error("Taxonomy v7 fixedDirectoryContracts must not be empty");
  for (const [id, contract] of Object.entries(fixedFilenameContracts)) {
    if (contract.scope.kind === "fixed-directory-contract" && !fixedDirectoryContracts[contract.scope.fixedDirectoryContractId]) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.fixedDirectoryContractId is invalid`);
    if (contract.scope.kind === "sibling-fixed-filename-contract" && !fixedFilenameContracts[contract.scope.fixedFilenameContractId]) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.fixedFilenameContractId is invalid`);
  }

  const fixedFilenameRejectionContracts: Record<string, FixedFilenameRejectionContract> = {};
  const rejectedFixedPaths = new Set<string>();
  for (const [id, value] of Object.entries(fixedRejectionRows)) {
    const spec = requireRecord(value, `fixedFilenameRejectionContracts.${id}`);
    requireExactKeys(spec, ["sourcePathIdentities", "disposition", "reason"], `fixedFilenameRejectionContracts.${id}`);
    if (spec.disposition !== "normalize" && spec.disposition !== "relocate") throw new Error(`Taxonomy v7 fixedFilenameRejectionContracts.${id}.disposition is invalid`);
    const sourcePathIdentities = requireStringArray(spec.sourcePathIdentities, `fixedFilenameRejectionContracts.${id}.sourcePathIdentities`).map(normalizeRelative);
    if (sourcePathIdentities.length === 0 || sourcePathIdentities.some((path) => rejectedFixedPaths.has(path))) throw new Error(`Taxonomy v7 fixedFilenameRejectionContracts.${id}.sourcePathIdentities are empty or duplicated`);
    for (const path of sourcePathIdentities) rejectedFixedPaths.add(path);
    fixedFilenameRejectionContracts[id] = { sourcePathIdentities, disposition: spec.disposition, reason: requireString(spec.reason, `fixedFilenameRejectionContracts.${id}.reason`) };
  }
  if (Object.keys(fixedFilenameRejectionContracts).length === 0) throw new Error("Taxonomy v7 fixedFilenameRejectionContracts must not be empty");

  const configurableEntryContracts: Record<string, ConfigurableEntryContract> = {};
  for (const [id, value] of Object.entries(configurableRows)) {
    const spec = requireRecord(value, `configurableEntryContracts.${id}`);
    const fileKindId = requireString(spec.fileKindId, `configurableEntryContracts.${id}.fileKindId`);
    if (!fileKinds[fileKindId]) throw new Error(`Taxonomy v7 configurableEntryContracts.${id} references unknown file kind ${fileKindId}`);
    configurableEntryContracts[id] = {
      filename: requireString(spec.filename, `configurableEntryContracts.${id}.filename`),
      fileKindId,
      ecosystemId: requireString(spec.ecosystemId, `configurableEntryContracts.${id}.ecosystemId`),
      role: requireString(spec.role, `configurableEntryContracts.${id}.role`),
      configurationSources: requireStringArray(spec.configurationSources, `configurableEntryContracts.${id}.configurationSources`),
    };
  }

  const fileKindResolutionRules: Record<string, FileKindResolutionRuleSpec> = {};
  for (const [id, value] of Object.entries(fileResolutionRows)) {
    const spec = requireRecord(value, `fileKindResolutionRules.${id}`);
    const extensionChain = requireString(spec.extensionChain, `fileKindResolutionRules.${id}.extensionChain`);
    const fileKindId = requireString(spec.fileKindId, `fileKindResolutionRules.${id}.fileKindId`);
    if (!fileKinds[fileKindId]?.extensionChains.includes(extensionChain)) throw new Error(`Taxonomy v7 fileKindResolutionRules.${id} does not reference an owned extension chain`);
    if (!Number.isSafeInteger(spec.priority)) throw new Error(`Taxonomy v7 fileKindResolutionRules.${id}.priority must be an integer`);
    const filenamePattern = typeof spec.filenamePattern === "string" ? spec.filenamePattern : undefined;
    const pathPattern = typeof spec.pathPattern === "string" ? validatedContractPattern(spec.pathPattern, `fileKindResolutionRules.${id}.pathPattern`, false) : undefined;
    if (filenamePattern) new RegExp(filenamePattern, "u");
    const parentKindIds = spec.parentKindIds === undefined ? undefined : requireStringArray(spec.parentKindIds, `fileKindResolutionRules.${id}.parentKindIds`);
    const ancestorKindIds = spec.ancestorKindIds === undefined ? undefined : requireStringArray(spec.ancestorKindIds, `fileKindResolutionRules.${id}.ancestorKindIds`);
    for (const kindId of [...(parentKindIds ?? []), ...(ancestorKindIds ?? [])]) if (!semanticDirectoryKinds[kindId]) throw new Error(`Taxonomy v7 fileKindResolutionRules.${id} references unknown directory kind ${kindId}`);
    fileKindResolutionRules[id] = { extensionChain, fileKindId, priority: spec.priority as number, filenamePattern, pathPattern, parentKindIds, ancestorKindIds };
  }
  if (Object.keys(fileKindResolutionRules).length === 0) throw new Error("Taxonomy v7 fileKindResolutionRules must not be empty");

  const scopedFileKinds: Record<string, ScopedFileKindSpec> = {};
  for (const [id, value] of Object.entries(scopedFileRows)) {
    const spec = requireRecord(value, `scopedFileKinds.${id}`);
    const extensionChains = requireStringArray(spec.extensionChains, `scopedFileKinds.${id}.extensionChains`);
    if (extensionChains.length === 0 || extensionChains.some((chain) => !chain.startsWith("."))) throw new Error(`Taxonomy v7 scopedFileKinds.${id}.extensionChains must contain dotted chains`);
    const sourceFilenamePattern = requireString(spec.sourceFilenamePattern, `scopedFileKinds.${id}.sourceFilenamePattern`);
    new RegExp(sourceFilenamePattern, "u");
    const role = requireString(spec.role, `scopedFileKinds.${id}.role`);
    if (!["source", "schema", "specification", "configuration", "documentation", "test", "asset", "generated", "marker", "evidence"].includes(role)) throw new Error(`Taxonomy v7 scopedFileKinds.${id}.role is invalid`);
    const parentDirectoryKindId = spec.parentDirectoryKindId === undefined ? undefined : requireString(spec.parentDirectoryKindId, `scopedFileKinds.${id}.parentDirectoryKindId`);
    if (parentDirectoryKindId && !semanticDirectoryKinds[parentDirectoryKindId]) throw new Error(`Taxonomy v7 scopedFileKinds.${id} references unknown parent directory kind ${parentDirectoryKindId}`);
    scopedFileKinds[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `scopedFileKinds.${id}.pathPattern`, false),
      parentDirectoryKindId,
      emoji: requireString(spec.emoji, `scopedFileKinds.${id}.emoji`).normalize("NFC"),
      extensionChains: [...new Set(extensionChains)].sort((left, right) => right.length - left.length || left.localeCompare(right)),
      role,
      sourceFilenamePattern,
      authority: requireString(spec.authority, `scopedFileKinds.${id}.authority`),
      reason: requireString(spec.reason, `scopedFileKinds.${id}.reason`),
      verification: requireString(spec.verification, `scopedFileKinds.${id}.verification`),
      expires: fixedExpiry(spec.expires, `scopedFileKinds.${id}.expires`),
    };
  }

  const semanticDirectoryMemberKinds: Record<string, SemanticDirectoryMemberKindSpec> = {};
  for (const [id, value] of Object.entries(directoryMemberRows)) {
    const spec = requireRecord(value, `semanticDirectoryMemberKinds.${id}`);
    if (spec.source !== "registry") throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id}.source must be registry`);
    const ownerKindIds = requireStringArray(spec.ownerKindIds, `semanticDirectoryMemberKinds.${id}.ownerKindIds`);
    const memberNames = requireStringArray(spec.memberNames, `semanticDirectoryMemberKinds.${id}.memberNames`);
    if (ownerKindIds.length === 0 || memberNames.length === 0) throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} must declare owners and members`);
    if (memberNames.some((name) => name !== name.normalize("NFC") || !splitLeadingEmoji(name).emoji)) throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} member names must be NFC emoji-leading evidence`);
    semanticDirectoryMemberKinds[id] = { ownerKindIds: [...new Set(ownerKindIds)].sort(), memberNames: [...new Set(memberNames)].sort(), source: "registry" };
  }
  const directoryContextIds = new Set([...Object.keys(semanticDirectoryKinds), ...Object.keys(semanticDirectoryMemberKinds)]);
  for (const [id, spec] of Object.entries(semanticDirectoryMemberKinds)) for (const ownerId of spec.ownerKindIds) if (!directoryContextIds.has(ownerId)) throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} references unknown owner kind ${ownerId}`);

  const semanticProjectedMemberKinds: Record<string, SemanticProjectedMemberKindSpec> = {};
  for (const [id, value] of Object.entries(projectedMemberRows)) {
    const spec = requireRecord(value, `semanticProjectedMemberKinds.${id}`);
    if (spec.identityField !== "mutationDirectoryName" && spec.identityField !== "commandDirectoryName") throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id}.identityField is invalid`);
    const ownerKindIds = requireStringArray(spec.ownerKindIds, `semanticProjectedMemberKinds.${id}.ownerKindIds`);
    if (ownerKindIds.length === 0) throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id}.ownerKindIds must not be empty`);
    semanticProjectedMemberKinds[id] = { ownerKindIds: [...new Set(ownerKindIds)].sort(), projectionContractId: requireString(spec.projectionContractId, `semanticProjectedMemberKinds.${id}.projectionContractId`), sourceMemberKindId: requireString(spec.sourceMemberKindId, `semanticProjectedMemberKinds.${id}.sourceMemberKindId`), identityField: spec.identityField };
  }
  if (Object.keys(semanticProjectedMemberKinds).length === 0) throw new Error("Taxonomy v7 semanticProjectedMemberKinds must not be empty");
  const allDirectoryContextIds = new Set([...directoryContextIds, ...Object.keys(semanticProjectedMemberKinds), ...Object.keys(fixedDirectoryContracts)]);
  for (const [id, spec] of Object.entries(semanticDirectoryKinds)) for (const parentId of spec.parentKindIds ?? []) if (!allDirectoryContextIds.has(parentId)) throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id} references unknown parent kind ${parentId}`);
  for (const [id, spec] of Object.entries(semanticProjectedMemberKinds)) {
    if (!semanticDirectoryMemberKinds[spec.sourceMemberKindId]) throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown source member kind ${spec.sourceMemberKindId}`);
    for (const ownerId of spec.ownerKindIds) if (!allDirectoryContextIds.has(ownerId)) throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown owner kind ${ownerId}`);
  }

  const semanticPathProjectionProfileRenderers: Record<string, SemanticPathProjectionProfileRenderer> = {};
  for (const [id, value] of Object.entries(projectionRendererRows)) {
    const spec = requireRecord(value, `semanticPathProjectionProfileRenderers.${id}`);
    if (spec.direction !== "forward-only" || canonicalJson(spec.captureFields) !== canonicalJson(["standardVersion", "subsetId"]) || spec.template !== "🪆️{standardVersion}-{subsetId}" || canonicalJson(spec.tupleCollisionFields) !== canonicalJson(["artifactId", "standardVersion", "subsetId"])) throw new Error(`Taxonomy v7 semanticPathProjectionProfileRenderers.${id} is not the forward-only standard/subset contract`);
    const directoryKindId = requireString(spec.directoryKindId, `semanticPathProjectionProfileRenderers.${id}.directoryKindId`);
    if (!semanticDirectoryKinds[directoryKindId]) throw new Error(`Taxonomy v7 semanticPathProjectionProfileRenderers.${id} references unknown directory kind ${directoryKindId}`);
    semanticPathProjectionProfileRenderers[id] = { direction: "forward-only", captureFields: ["standardVersion", "subsetId"], directoryKindId, template: "🪆️{standardVersion}-{subsetId}", tupleCollisionFields: ["artifactId", "standardVersion", "subsetId"] };
  }
  if (Object.keys(semanticPathProjectionProfileRenderers).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionProfileRenderers must not be empty");

  const parseDescendantNode = (value: unknown, name: string): SemanticDescendantNode => {
    const spec = requireRecord(value, name);
    if (spec.nodeType !== "directory" && spec.nodeType !== "file") throw new Error(`Taxonomy v7 ${name}.nodeType is invalid`);
    const parseSegments = (value: unknown, key: string): readonly Readonly<{ kindId: string; literal: string }>[] => {
      if (!Array.isArray(value)) throw new Error(`Taxonomy v7 ${key} must be an array`);
      return value.map((value, index) => {
        const segment = requireRecord(value, `${key}[${index}]`);
        const kindId = requireString(segment.kindId, `${key}[${index}].kindId`);
        const literal = requireString(segment.literal, `${key}[${index}].literal`).normalize("NFC");
        const kind = semanticDirectoryKinds[kindId];
        const leading = splitLeadingEmoji(literal);
        if (!kind || emojiFold(leading.emoji) !== emojiFold(kind.emoji) || !new RegExp(kind.slugPattern, "u").test(leading.rest)) throw new Error(`Taxonomy v7 ${key} has an invalid semantic path segment ${literal}`);
        return { kindId, literal };
      });
    };
    const pathSegments = parseSegments(spec.pathSegments, `${name}.pathSegments`);
    if (spec.nodeType === "directory") {
      const kindId = requireString(spec.kindId, `${name}.kindId`);
      if (!allDirectoryContextIds.has(kindId) || spec.sourceFilename !== undefined || spec.fixedFilenameContractId !== undefined || spec.packageGlue !== undefined) throw new Error(`Taxonomy v7 ${name} references an invalid directory kind ${kindId}`);
      return { pathSegments, nodeType: "directory", kindId };
    }
    const authorities = [spec.kindId !== undefined, spec.fixedFilenameContractId !== undefined].filter(Boolean).length;
    if (authorities !== 1) throw new Error(`Taxonomy v7 ${name} must declare exactly one file authority`);
    if (spec.kindId !== undefined) {
      const kindId = requireString(spec.kindId, `${name}.kindId`);
      if (!fileKinds[kindId]) throw new Error(`Taxonomy v7 ${name} references unknown file kind ${kindId}`);
      const sourceFilename = spec.sourceFilename === undefined ? undefined : requireString(spec.sourceFilename, `${name}.sourceFilename`).normalize("NFC");
      if (sourceFilename !== undefined && (kindId !== "rust-source" || sourceFilename !== "🦀️.rs")) throw new Error(`Taxonomy v7 ${name}.sourceFilename is not the frozen Draw Rust source leaf`);
      return { pathSegments, nodeType: "file", kindId, ...(sourceFilename ? { sourceFilename } : {}) };
    }
    if (spec.fixedFilenameContractId !== undefined) {
      const fixedFilenameContractId = requireString(spec.fixedFilenameContractId, `${name}.fixedFilenameContractId`);
      if (!fixedFilenameContracts[fixedFilenameContractId]) throw new Error(`Taxonomy v7 ${name} references unknown fixed filename contract ${fixedFilenameContractId}`);
      return { pathSegments, nodeType: "file", fixedFilenameContractId };
    }
    throw new Error(`Taxonomy v7 ${name} has no file authority`);
  };
  const semanticDescendantContracts: Record<string, SemanticDescendantContract> = {};
  for (const [id, value] of Object.entries(descendantContractRows)) {
    const spec = requireRecord(value, `semanticDescendantContracts.${id}`);
    const rootDirectoryKindId = requireString(spec.rootDirectoryKindId, `semanticDescendantContracts.${id}.rootDirectoryKindId`);
    if (!allDirectoryContextIds.has(rootDirectoryKindId)) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} references unknown root directory kind ${rootDirectoryKindId}`);
    if (spec.contractKind === "catalog") {
      const catalogContractId = requireString(spec.catalogContractId, `semanticDescendantContracts.${id}.catalogContractId`);
      const leafFileKindId = requireString(spec.leafFileKindId, `semanticDescendantContracts.${id}.leafFileKindId`);
      const reserve = requireRecord(spec.pathBudgetReserve, `semanticDescendantContracts.${id}.pathBudgetReserve`);
      if (!fileKinds[leafFileKindId] || spec.rendering !== "semantic-member-directory-and-physical-kind-leaf" || reserve.derivation !== "longest-rendered-catalog-descendant-suffix" || !Number.isSafeInteger(reserve.bytes) || (reserve.bytes as number) <= 0) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} is not a valid catalog descendant contract`);
      semanticDescendantContracts[id] = { contractKind: "catalog", rootDirectoryKindId, catalogContractId, leafFileKindId, rendering: "semantic-member-directory-and-physical-kind-leaf", pathBudgetReserve: { derivation: "longest-rendered-catalog-descendant-suffix", bytes: reserve.bytes as number } };
      continue;
    }
    if (!Array.isArray(spec.requiredNodes) || !Array.isArray(spec.exclusiveAlternatives)) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} node lists must be arrays`);
    const requiredNodes = spec.requiredNodes.map((node, index) => parseDescendantNode(node, `semanticDescendantContracts.${id}.requiredNodes[${index}]`));
    const exclusiveAlternatives = spec.exclusiveAlternatives.map((value, index) => {
      const alternative = requireRecord(value, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}]`);
      if (alternative.mode !== "exactly-one" || !Array.isArray(alternative.nodes) || alternative.nodes.length < 2) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} alternative must contain exactly-one candidates`);
      return { id: requireString(alternative.id, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}].id`), mode: "exactly-one" as const, nodes: alternative.nodes.map((node, nodeIndex) => parseDescendantNode(node, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}].nodes[${nodeIndex}]`)) };
    });
    if (!Number.isSafeInteger(spec.realizedNodeCount) || spec.realizedNodeCount !== requiredNodes.length + exclusiveAlternatives.length) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id}.realizedNodeCount is invalid`);
    const reserve = requireRecord(spec.pathBudgetReserve, `semanticDescendantContracts.${id}.pathBudgetReserve`);
    const suffix = (node: SemanticDescendantNode): string => {
      const segments = node.pathSegments.map((segment) => segment.literal);
      if (node.nodeType === "file") {
        if ("kindId" in node) {
          segments.push(canonicalPrimaryFilenameForKind(node.kindId, root as unknown as DiscoveryTaxonomy));
        } else if ("fixedFilenameContractId" in node) segments.push(posix.basename(fixedFilenameContracts[node.fixedFilenameContractId].pathPattern));
        else throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} file authority is invalid`);
      }
      return segments.length === 0 ? "" : `/${segments.join("/")}`;
    };
    const reserveBytes = Math.max(...[...requiredNodes, ...exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].map((node) => Buffer.byteLength(suffix(node), "utf8")));
    if (reserve.derivation !== "longest-canonical-descendant-suffix" || reserve.bytes !== reserveBytes) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id}.pathBudgetReserve is not derived from its longest suffix`);
    semanticDescendantContracts[id] = { rootDirectoryKindId, requiredNodes, exclusiveAlternatives, realizedNodeCount: spec.realizedNodeCount as number, pathBudgetReserve: { derivation: "longest-canonical-descendant-suffix", bytes: reserveBytes } };
  }
  if (Object.keys(semanticDescendantContracts).length === 0) throw new Error("Taxonomy v7 semanticDescendantContracts must not be empty");

  const semanticPathProjectionCatalogContracts: Record<string, SemanticPathProjectionCatalogContract> = {};
  const expectedCatalogContract: SemanticMutationPathProjectionCatalogContract = { registryField: "vectors", required: true, allowEmpty: true, runtimeKindsField: "kinds", runtimeKindsRelation: "independent", mutationIdField: "mutationId", sourceMutationDirectoryNameField: "sourceMutationDirectoryName", mutationDirectoryNameField: "mutationDirectoryName", scenariosField: "scenarios", scenarioIdField: "id", scenarioDirectoryNameField: "directoryName", sourceBundleUniquenessFields: ["mutationId", "sourceMutationDirectoryName", "scenarioId"], canonicalBundleUniquenessFields: ["mutationId", "mutationDirectoryName", "scenarioId"], coverage: "every-physical-bundle-exactly-once" };
  for (const [id, value] of Object.entries(projectionCatalogRows)) {
    const spec = requireRecord(value, `semanticPathProjectionCatalogContracts.${id}`);
    if (spec.contractKind === undefined) {
      if (canonicalJson(value) !== canonicalJson(expectedCatalogContract)) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not the independent required vector registry contract`);
      semanticPathProjectionCatalogContracts[id] = expectedCatalogContract;
      continue;
    }
    if (spec.contractKind === "distributed-json-manifest-catalog") {
      if (spec.modelIdentityField !== "id" || spec.memberIdentityField !== "id" || spec.memberVersionField !== "version" || spec.requiredModelManifest !== true || spec.coverage !== "every-source-file-and-destination-node-exactly-once" || spec.unknownCategoryPolicy !== "problem" || spec.unownedModelPolicy !== "problem" || !Array.isArray(spec.categoryRules) || spec.categoryRules.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not a strict distributed manifest catalog`);
      if (!Array.isArray(spec.profileVectors) || spec.profileVectors.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.profileVectors must be non-empty`);
      const profileVectors = spec.profileVectors.map((value, index) => {
        const vector = requireRecord(value, `semanticPathProjectionCatalogContracts.${id}.profileVectors[${index}]`);
        const profile = { artifactId: requireString(vector.artifactId, "profile artifactId"), standardVersion: requireString(vector.standardVersion, "profile standardVersion"), subsetId: requireString(vector.subsetId, "profile subsetId") };
        if (canonicalJson(Object.keys(vector).sort()) !== canonicalJson(["artifactId", "standardVersion", "subsetId"]) || profile.artifactId !== spec.ownerArtifactMemberName || Object.values(profile).some((field) => field !== field.normalize("NFC") || /[\\/]/u.test(field))) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.profileVectors[${index}] is not an exact NFC owner tuple`);
        return profile;
      });
      if (new Set(profileVectors.map((vector) => canonicalJson(vector))).size !== profileVectors.length) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats a profile vector`);
      const categoryRules = spec.categoryRules.map((value, index) => {
        const rule = requireRecord(value, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}]`);
        const sourceDirectoryName = requireString(rule.sourceDirectoryName, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].sourceDirectoryName`).normalize("NFC");
        const directoryKindId = requireString(rule.directoryKindId, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].directoryKindId`);
        const manifestSchema = requireString(rule.manifestSchema, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].manifestSchema`);
        if (!semanticDirectoryKinds[directoryKindId]) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}] references an unknown directory kind`);
        if (rule.sourceShape === "direct-semantic-json") return { sourceDirectoryName, directoryKindId, sourceShape: "direct-semantic-json" as const, manifestSchema, memberDirectoryEmoji: requireString(rule.memberDirectoryEmoji, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].memberDirectoryEmoji`).normalize("NFC") };
        if (rule.sourceShape === "nested-fixed-json") return { sourceDirectoryName, directoryKindId, sourceShape: "nested-fixed-json" as const, manifestSchema, fixedSourceFilename: requireString(rule.fixedSourceFilename, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].fixedSourceFilename`).normalize("NFC") };
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].sourceShape is invalid`);
      });
      if (new Set(categoryRules.map((rule) => rule.sourceDirectoryName)).size !== categoryRules.length) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats a catalog category`);
      semanticPathProjectionCatalogContracts[id] = { contractKind: "distributed-json-manifest-catalog", ownerArtifactMemberName: requireString(spec.ownerArtifactMemberName, `semanticPathProjectionCatalogContracts.${id}.ownerArtifactMemberName`).normalize("NFC"), profileVectors, modelManifestSchema: requireString(spec.modelManifestSchema, `semanticPathProjectionCatalogContracts.${id}.modelManifestSchema`), modelManifestSourceFilename: requireString(spec.modelManifestSourceFilename, `semanticPathProjectionCatalogContracts.${id}.modelManifestSourceFilename`).normalize("NFC"), modelIdentityField: "id", memberIdentityField: "id", memberVersionField: "version", requiredMemberVersion: requireString(spec.requiredMemberVersion, `semanticPathProjectionCatalogContracts.${id}.requiredMemberVersion`), requiredModelManifest: true, categoryRules, coverage: "every-source-file-and-destination-node-exactly-once", unknownCategoryPolicy: "problem", unownedModelPolicy: "problem" };
      continue;
    }
    if (spec.contractKind === "exact-owner-vectors") {
      if (spec.required !== true || spec.allowEmpty !== false || canonicalJson(spec.identityFields) !== canonicalJson(["artifactId", "standardVersion", "subsetId", "commandDirectoryName"]) || spec.coverage !== "every-physical-command-bundle-exactly-once" || !Array.isArray(spec.vectors) || spec.vectors.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not a strict exact-owner vector registry`);
      const vectors = spec.vectors.map((value, index) => {
        const vector = requireRecord(value, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}]`);
        return { artifactId: requireString(vector.artifactId, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].artifactId`).normalize("NFC"), standardVersion: requireString(vector.standardVersion, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].standardVersion`), subsetId: requireString(vector.subsetId, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].subsetId`), commandDirectoryName: requireString(vector.commandDirectoryName, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].commandDirectoryName`).normalize("NFC") };
      });
      if (new Set(vectors.map((vector) => canonicalJson(vector))).size !== vectors.length) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats an owner vector`);
      semanticPathProjectionCatalogContracts[id] = { contractKind: "exact-owner-vectors", required: true, allowEmpty: false, identityFields: ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"], coverage: "every-physical-command-bundle-exactly-once", vectors };
      continue;
    }
    throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.contractKind is invalid`);
  }
  if (Object.keys(semanticPathProjectionCatalogContracts).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionCatalogContracts must not be empty");
  for (const [id, contract] of Object.entries(semanticDescendantContracts)) if ("contractKind" in contract && !semanticPathProjectionCatalogContracts[contract.catalogContractId]) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} references an unknown catalog contract`);

  const captureFields = new Set<SemanticProjectionCaptureField>(["standardVersion", "subsetId", "mutationId", "scenarioId", "commandDirectoryName"]);
  const parseProjectionSegment = (value: unknown, name: string, destination: boolean): SemanticProjectionSourceSegment | SemanticProjectionDestinationSegment => {
    const spec = requireRecord(value, name);
    const kindId = typeof spec.kindId === "string" ? spec.kindId : undefined;
    const memberKindId = typeof spec.memberKindId === "string" ? spec.memberKindId : undefined;
    const projectedMemberKindId = typeof spec.projectedMemberKindId === "string" ? spec.projectedMemberKindId : undefined;
    if ((kindId ? 1 : 0) + (memberKindId ? 1 : 0) + (projectedMemberKindId ? 1 : 0) !== 1) throw new Error(`Taxonomy v7 ${name} must identify exactly one kind`);
    if (kindId && !allDirectoryContextIds.has(kindId)) throw new Error(`Taxonomy v7 ${name} references unknown directory kind ${kindId}`);
    if (memberKindId && !semanticDirectoryMemberKinds[memberKindId]) throw new Error(`Taxonomy v7 ${name} references unknown semantic member kind ${memberKindId}`);
    if (projectedMemberKindId && !semanticProjectedMemberKinds[projectedMemberKindId]) throw new Error(`Taxonomy v7 ${name} references unknown projected member kind ${projectedMemberKindId}`);
    if (destination) {
      if (memberKindId) throw new Error(`Taxonomy v7 ${name} cannot render a source member kind`);
      if (spec.literal !== undefined && kindId) return { kindId, literal: requireString(spec.literal, `${name}.literal`) };
      if (spec.render === "profile" && kindId) return { kindId, render: "profile" };
      if (typeof spec.copy === "string" && captureFields.has(spec.copy as SemanticProjectionCaptureField)) return projectedMemberKindId ? { projectedMemberKindId, copy: spec.copy as SemanticProjectionCaptureField } : { kindId: kindId!, copy: spec.copy as SemanticProjectionCaptureField };
    } else {
      if (spec.literal !== undefined && kindId) return { kindId, literal: requireString(spec.literal, `${name}.literal`) };
      if (spec.literal !== undefined && memberKindId) {
        const literal = requireString(spec.literal, `${name}.literal`).normalize("NFC");
        if (!semanticDirectoryMemberKinds[memberKindId].memberNames.includes(literal)) throw new Error(`Taxonomy v7 ${name}.literal is not registered by ${memberKindId}`);
        return { memberKindId, literal };
      }
      if (typeof spec.capture === "string" && captureFields.has(spec.capture as SemanticProjectionCaptureField)) return projectedMemberKindId ? { projectedMemberKindId, capture: spec.capture as SemanticProjectionCaptureField } : { kindId: kindId!, capture: spec.capture as SemanticProjectionCaptureField };
    }
    throw new Error(`Taxonomy v7 ${name} has an invalid ${destination ? "destination" : "source"} operation`);
  };
  const semanticPathProjectionContracts: Record<string, SemanticPathProjectionContract> = {};
  for (const [id, value] of Object.entries(projectionRows)) {
    const spec = requireRecord(value, `semanticPathProjectionContracts.${id}`);
    if (!Array.isArray(spec.sourceSegments) || !Array.isArray(spec.destinationSegments) || !["artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(String(spec.rationaleRule))) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} is invalid`);
    const sourceOwnerKindId = requireString(spec.sourceOwnerKindId, `semanticPathProjectionContracts.${id}.sourceOwnerKindId`);
    const destinationOwnerKindId = requireString(spec.destinationOwnerKindId, `semanticPathProjectionContracts.${id}.destinationOwnerKindId`);
    if (!semanticDirectoryMemberKinds[sourceOwnerKindId] || !semanticDirectoryMemberKinds[destinationOwnerKindId]) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} owner kind is invalid`);
    const profileRendererId = requireString(spec.profileRendererId, `semanticPathProjectionContracts.${id}.profileRendererId`);
    const descendantContractId = requireString(spec.descendantContractId, `semanticPathProjectionContracts.${id}.descendantContractId`);
    const catalogContractId = requireString(spec.catalogContractId, `semanticPathProjectionContracts.${id}.catalogContractId`);
    if (!semanticPathProjectionProfileRenderers[profileRendererId] || !semanticDescendantContracts[descendantContractId] || !semanticPathProjectionCatalogContracts[catalogContractId]) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} references an unknown registry`);
    const rationaleRule = spec.rationaleRule as SemanticPathProjectionContract["rationaleRule"];
    const sourceArtifactMemberName = spec.sourceArtifactMemberName === undefined ? undefined : requireString(spec.sourceArtifactMemberName, `semanticPathProjectionContracts.${id}.sourceArtifactMemberName`).normalize("NFC");
    const expectedArtifact = rationaleRule === "artifact-example-model-catalog-projection-v1" ? "📐️cad" : rationaleRule === "artifact-editor-command-projection-v1" ? "🖍️drawing" : undefined;
    if (sourceArtifactMemberName !== expectedArtifact) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id}.sourceArtifactMemberName does not match its rationale`);
    const sourceSegments = spec.sourceSegments.map((segment, index) => parseProjectionSegment(segment, `semanticPathProjectionContracts.${id}.sourceSegments[${index}]`, false) as SemanticProjectionSourceSegment);
    const destinationSegments = spec.destinationSegments.map((segment, index) => parseProjectionSegment(segment, `semanticPathProjectionContracts.${id}.destinationSegments[${index}]`, true) as SemanticProjectionDestinationSegment);
    const captures = sourceSegments.flatMap((segment) => "capture" in segment ? [segment.capture] : []);
    const expectedCaptures: readonly SemanticProjectionCaptureField[] = rationaleRule === "artifact-editor-command-projection-v1" ? ["standardVersion", "subsetId", "commandDirectoryName"] : ["standardVersion", "subsetId"];
    if (canonicalJson(captures) !== canonicalJson(expectedCaptures)) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} has invalid captures for ${rationaleRule}`);
    const descendant = semanticDescendantContracts[descendantContractId];
    const catalog = semanticPathProjectionCatalogContracts[catalogContractId];
    if (rationaleRule === "artifact-example-model-catalog-projection-v1" ? !("contractKind" in descendant && descendant.contractKind === "catalog" && "contractKind" in catalog && catalog.contractKind === "distributed-json-manifest-catalog") : "contractKind" in descendant || !("contractKind" in catalog && catalog.contractKind === "exact-owner-vectors")) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} references incompatible descendant/catalog authorities`);
    const descendantNodes = "contractKind" in descendant ? [] : [...descendant.requiredNodes, ...descendant.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)];
    const sourceNamedNodes = descendantNodes.filter((node): node is SemanticDescendantKindNode & { readonly sourceFilename: string } => "kindId" in node && node.sourceFilename !== undefined);
    if (rationaleRule === "artifact-editor-command-projection-v1" ? sourceNamedNodes.length !== descendantNodes.filter((node) => "kindId" in node && node.nodeType === "file" && node.kindId === "rust-source").length || sourceNamedNodes.filter((node) => node.pathSegments.length === 0).length !== 1 : sourceNamedNodes.length !== 0) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} has invalid source-filename descendant authority`);
    semanticPathProjectionContracts[id] = { sourceOwnerKindId, ...(sourceArtifactMemberName ? { sourceArtifactMemberName } : {}), sourceSegments, profileRendererId, destinationOwnerKindId, destinationSegments, descendantContractId, catalogContractId, rationaleRule };
  }
  if (Object.keys(semanticPathProjectionContracts).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionContracts must not be empty");
  for (const [id, spec] of Object.entries(semanticProjectedMemberKinds)) if (!semanticPathProjectionContracts[spec.projectionContractId] && spec.projectionContractId !== "canonical-mutation-case-pair-v1") throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown projection contract ${spec.projectionContractId}`);
  const semanticOwnedFileProjectionContracts: Record<string, SemanticOwnedFileProjectionContract> = {};
  for (const [id, value] of Object.entries(ownedFileProjectionRows)) {
    const name = `semanticOwnedFileProjectionContracts.${id}`;
    const spec = requireRecord(value, name);
    if (spec.contractKind === "exact-owner-path-catalog") {
      requireExactKeys(spec, ["contractKind", "authorityCatalogPath", "authorityCatalogSha256", "sourceFileKindId", "sourceBasenames", "destinationDirectoryKinds", "allowedDispositions", "ownerEvidenceKinds", "referenceOwnerIds", "generatorOwnerIds", "expectedCounts", "authoredDocumentCorrections", "rationaleRule", ...(Object.hasOwn(spec, "currentSourceRevisions") ? ["currentSourceRevisions"] : [])], name);
      const authorityCatalogPath = normalizeRelative(requireString(spec.authorityCatalogPath, name + ".authorityCatalogPath"));
      const authorityCatalogSha256 = requireString(spec.authorityCatalogSha256, name + ".authorityCatalogSha256");
      const sourceBasenames = requireStringArray(spec.sourceBasenames, name + ".sourceBasenames");
      const allowedDispositions = requireStringArray(spec.allowedDispositions, name + ".allowedDispositions");
      const ownerEvidenceKinds = requireStringArray(spec.ownerEvidenceKinds, name + ".ownerEvidenceKinds");
      const referenceOwnerIds = requireStringArray(spec.referenceOwnerIds, name + ".referenceOwnerIds");
      const generatorOwnerIds = requireStringArray(spec.generatorOwnerIds, name + ".generatorOwnerIds");
      const destinations = requireRecord(spec.destinationDirectoryKinds, name + ".destinationDirectoryKinds");
      requireExactKeys(destinations, ["license", "readme"], name + ".destinationDirectoryKinds");
      const parseDestination = (kind: "license" | "readme"): { readonly directoryKindId: "owner-license" | "owner-readme"; readonly directoryName: "⚖️license" | "📃️readme"; readonly filename: "📝️.md" } => {
        const destination = requireRecord(destinations[kind], name + ".destinationDirectoryKinds." + kind);
        requireExactKeys(destination, ["directoryKindId", "directoryName", "filename"], name + ".destinationDirectoryKinds." + kind);
        return { directoryKindId: requireString(destination.directoryKindId, name + ".destinationDirectoryKinds." + kind + ".directoryKindId") as "owner-license" | "owner-readme", directoryName: requireString(destination.directoryName, name + ".destinationDirectoryKinds." + kind + ".directoryName") as "⚖️license" | "📃️readme", filename: requireString(destination.filename, name + ".destinationDirectoryKinds." + kind + ".filename") as "📝️.md" };
      };
      const destinationDirectoryKinds = { license: parseDestination("license"), readme: parseDestination("readme") };
      const expectedCounts = requireRecord(spec.expectedCounts, name + ".expectedCounts");
      requireExactKeys(expectedCounts, ["fixed", "license", "projected", "readme", "referenceBindings", "total"], name + ".expectedCounts");
      const counts = { fixed: expectedCounts.fixed, license: expectedCounts.license, projected: expectedCounts.projected, readme: expectedCounts.readme, referenceBindings: expectedCounts.referenceBindings, total: expectedCounts.total };
      if (id !== "readme-license-owner-leaves-v1"
        || !/^[a-f0-9]{64}$/u.test(authorityCatalogSha256)
        || spec.sourceFileKindId !== "markdown"
        || canonicalJson(sourceBasenames) !== canonicalJson(["LICENSE.md", "README.md"])
        || canonicalJson(destinationDirectoryKinds) !== canonicalJson({ license: { directoryKindId: "owner-license", directoryName: "⚖️license", filename: "📝️.md" }, readme: { directoryKindId: "owner-readme", directoryName: "📃️readme", filename: "📝️.md" } })
        || canonicalJson(allowedDispositions) !== canonicalJson(["attribution-relocate", "configurable-owner-license-relocate", "fixed", "generated-evidence-relocate", "owner-documentation-relocate"])
        || canonicalJson(ownerEvidenceKinds) !== canonicalJson(["configurable-owner-license", "ordinary-owner-doc", "package-publication", "third-party-attribution", "ticket-evidence", "ticket-scratch"])
        || canonicalJson(referenceOwnerIds) !== canonicalJson(["asset-distribution-owner", "bun-package-publisher", "commonmark-scratch-rust-reader", "markdown-relative-reference-adapter", "repo-cli-dev-docs-go", "vscode-package-ignore"])
        || canonicalJson(generatorOwnerIds) !== canonicalJson(["assets-build"])
        || canonicalJson(counts) !== canonicalJson({ fixed: 4, license: 8, projected: 36, readme: 32, referenceBindings: 62, total: 40 })
        || spec.rationaleRule !== "readme-license-owner-projection-v1"
        || !fileKinds.markdown
        || semanticDirectoryKinds["owner-license"]?.projectionOnly !== true
        || semanticDirectoryKinds["owner-readme"]?.projectionOnly !== true
        || !generatorRows["assets-build"]) throw new Error("Taxonomy v7 " + name + " does not use the exact README/LICENSE owner catalog grammar");
      semanticOwnedFileProjectionContracts[id] = {
        contractKind: "exact-owner-path-catalog",
        authorityCatalogPath,
        authorityCatalogSha256,
        sourceFileKindId: "markdown",
        sourceBasenames: ["LICENSE.md", "README.md"],
        destinationDirectoryKinds: { license: { directoryKindId: "owner-license", directoryName: "⚖️license", filename: "📝️.md" }, readme: { directoryKindId: "owner-readme", directoryName: "📃️readme", filename: "📝️.md" } },
        allowedDispositions: ["attribution-relocate", "configurable-owner-license-relocate", "fixed", "generated-evidence-relocate", "owner-documentation-relocate"],
        ownerEvidenceKinds: ["configurable-owner-license", "ordinary-owner-doc", "package-publication", "third-party-attribution", "ticket-evidence", "ticket-scratch"],
        referenceOwnerIds: ["asset-distribution-owner", "bun-package-publisher", "commonmark-scratch-rust-reader", "markdown-relative-reference-adapter", "repo-cli-dev-docs-go", "vscode-package-ignore"],
        generatorOwnerIds: ["assets-build"],
        expectedCounts: { fixed: 4, license: 8, projected: 36, readme: 32, referenceBindings: 62, total: 40 },
        authoredDocumentCorrections: parseSemanticOwnedDocumentCorrections(spec.authoredDocumentCorrections),
        ...(Object.hasOwn(spec, "currentSourceRevisions") ? { currentSourceRevisions: parseSemanticOwnedCurrentSourceRevisions(spec.currentSourceRevisions) } : {}),
        rationaleRule: "readme-license-owner-projection-v1",
      };
      continue;
    }
    if (spec.contractKind === "semantic-facet-primary-file") {
      requireExactKeys(spec, ["contractKind", "sourceRoot", "sourceFilename", "fileKindAuthority", "sourceDisposition", "directoryCaptures", "ownerPathPatterns", "authoringCommand", "referenceConsumer", "rationaleRule"], name);
      const directoryCaptures: Record<string, { kindIds: readonly string[]; names?: readonly string[] }> = {};
      for (const [capture, row] of Object.entries(requireRecord(spec.directoryCaptures, `${name}.directoryCaptures`))) {
        const captureSpec = requireRecord(row, `${name}.directoryCaptures.${capture}`);
        directoryCaptures[capture] = { kindIds: requireStringArray(captureSpec.kindIds, `${name}.directoryCaptures.${capture}.kindIds`), ...(captureSpec.names ? { names: requireStringArray(captureSpec.names, `${name}.directoryCaptures.${capture}.names`) } : {}) };
      }
      const ownerPathPatterns = Object.fromEntries(Object.entries(requireRecord(spec.ownerPathPatterns, `${name}.ownerPathPatterns`)).map(([form, pattern]) => [form, requireString(pattern, `${name}.ownerPathPatterns.${form}`)]));
      const authoring = requireRecord(spec.authoringCommand, `${name}.authoringCommand`), consumer = requireRecord(spec.referenceConsumer, `${name}.referenceConsumer`);
      semanticOwnedFileProjectionContracts[id] = { contractKind: "semantic-facet-primary-file", sourceRoot: requireString(spec.sourceRoot, `${name}.sourceRoot`), sourceFilename: requireString(spec.sourceFilename, `${name}.sourceFilename`), fileKindAuthority: "windowEmptyFacetFileKindId", sourceDisposition: "authored", directoryCaptures, ownerPathPatterns, authoringCommand: { scriptPath: requireString(authoring.scriptPath, `${name}.authoringCommand.scriptPath`), command: ["new", "surface"], writeDisposition: "create-if-absent" }, referenceConsumer: { path: requireString(consumer.path, `${name}.referenceConsumer.path`), ownerRoot: requireString(consumer.ownerRoot, `${name}.referenceConsumer.ownerRoot`), adapter: "rust", region: "✏️👁️Surfaces", lineTemplate: requireString(consumer.lineTemplate, `${name}.referenceConsumer.lineTemplate`) }, rationaleRule: "artifact-empty-facet-primary-markdown-v1" };
      continue;
    }
    if (spec.contractKind === "owner-primary-file") {
      requireExactKeys(spec, ["contractKind", "ownerFixedDirectoryContractId", "sourceFileKindId", "sourceFilename", "destinationFilename", "rationaleRule"], name);
      const contract = { contractKind: "owner-primary-file" as const, ownerFixedDirectoryContractId: requireString(spec.ownerFixedDirectoryContractId, `${name}.ownerFixedDirectoryContractId`), sourceFileKindId: requireString(spec.sourceFileKindId, `${name}.sourceFileKindId`), sourceFilename: requireString(spec.sourceFilename, `${name}.sourceFilename`), destinationFilename: requireString(spec.destinationFilename, `${name}.destinationFilename`), rationaleRule: "ticket-document-primary-markdown-v1" as const };
      if (id !== contract.rationaleRule || spec.rationaleRule !== contract.rationaleRule || contract.ownerFixedDirectoryContractId !== "ticket-slug" || !fixedDirectoryRows[contract.ownerFixedDirectoryContractId] || contract.sourceFileKindId !== "markdown" || !fileKinds.markdown || fileKinds.markdown.extensionChains.length !== 1 || contract.sourceFilename !== "ticket.md" || contract.destinationFilename !== `${fileKinds.markdown.emoji}${fileKinds.markdown.extensionChains[0]}`) throw new Error(`Taxonomy v7 ${name} does not use the exact ticket document primary-leaf grammar`);
      semanticOwnedFileProjectionContracts[id] = contract;
      continue;
    }
    const ownerFixedDirectoryContractId = requireString(spec.ownerFixedDirectoryContractId, `${name}.ownerFixedDirectoryContractId`);
    const sourceFileKindId = requireString(spec.sourceFileKindId, `${name}.sourceFileKindId`);
    const destinationDirectoryKindId = requireString(spec.destinationDirectoryKindId, `${name}.destinationDirectoryKindId`);
    if (!fixedDirectoryRows[ownerFixedDirectoryContractId] || !fileKinds[sourceFileKindId] || semanticDirectoryKinds[destinationDirectoryKindId]?.projectionOnly !== true) throw new Error(`Taxonomy v7 ${name} references unknown or non-projection authority`);
    const common = { ownerFixedDirectoryContractId, sourceFileKindId, sourceFilename: requireString(spec.sourceFilename, `${name}.sourceFilename`), destinationDirectoryKindId, destinationDirectoryName: requireString(spec.destinationDirectoryName, `${name}.destinationDirectoryName`), destinationFilename: requireString(spec.destinationFilename, `${name}.destinationFilename`) };
    if (spec.contractKind === "owner-sibling-manifest-file") {
      requireExactKeys(spec, ["contractKind", "ownerFixedDirectoryContractId", "requiredSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "allowedStatuses", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "emptyContentRule", "statusDispositions", "rationaleRule"], name);
      const allowedStatuses = requireStringArray(spec.allowedStatuses, `${name}.allowedStatuses`);
      const statusDispositions = requireRecord(spec.statusDispositions, `${name}.statusDispositions`);
      requireExactKeys(statusDispositions, ["open", "closed-empty", "closed-nonempty", "invalid"], `${name}.statusDispositions`);
      const requiredSiblingFixedFilenameContractId = requireString(spec.requiredSiblingFixedFilenameContractId, `${name}.requiredSiblingFixedFilenameContractId`);
      if (spec.manifestAdapter !== "json" || spec.manifestStatusLocation !== "status" || canonicalJson(allowedStatuses) !== canonicalJson(["closed", "open"]) || spec.emptyContentRule !== "zero-byte" || spec.rationaleRule !== "ticket-important-markdown-projection-v1" || canonicalJson(statusDispositions) !== canonicalJson({ open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" }) || !fixedFilenameContracts[requiredSiblingFixedFilenameContractId]) throw new Error(`Taxonomy v7 ${name} does not use the exact active owner-file projection grammar`);
      semanticOwnedFileProjectionContracts[id] = { contractKind: "owner-sibling-manifest-file", ...common, requiredSiblingFixedFilenameContractId, manifestAdapter: "json", manifestStatusLocation: "status", allowedStatuses: ["closed", "open"], emptyContentRule: "zero-byte", statusDispositions: { open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" }, rationaleRule: "ticket-important-markdown-projection-v1" };
    } else if (spec.contractKind === "owner-optional-sibling-manifest-file") {
      requireExactKeys(spec, ["contractKind", "ownerFixedDirectoryContractId", "optionalSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "admittedDispositions", "rationaleRule"], name);
      const optionalSiblingFixedFilenameContractId = requireString(spec.optionalSiblingFixedFilenameContractId, `${name}.optionalSiblingFixedFilenameContractId`);
      const admittedDispositions = requireStringArray(spec.admittedDispositions, `${name}.admittedDispositions`);
      if (spec.manifestAdapter !== "json" || spec.manifestStatusLocation !== "status" || spec.rationaleRule !== "ticket-important-history-markdown-v1" || canonicalJson(admittedDispositions) !== canonicalJson(["closed-nonzero", "invalid-manifest", "missing-manifest"]) || !fixedFilenameContracts[optionalSiblingFixedFilenameContractId]) throw new Error(`Taxonomy v7 ${name} does not use the exact historical owner-file projection grammar`);
      semanticOwnedFileProjectionContracts[id] = { contractKind: "owner-optional-sibling-manifest-file", ...common, optionalSiblingFixedFilenameContractId, manifestAdapter: "json", manifestStatusLocation: "status", admittedDispositions: ["closed-nonzero", "invalid-manifest", "missing-manifest"], rationaleRule: "ticket-important-history-markdown-v1" };
    } else throw new Error(`Taxonomy v7 ${name}.contractKind is invalid`);
  }
  if (canonicalJson(Object.keys(semanticOwnedFileProjectionContracts)) !== canonicalJson(["artifact-empty-facet-primary-markdown-v1", "readme-license-owner-leaves-v1", "ticket-document-primary-markdown-v1", "ticket-important-history-markdown-v1", "ticket-important-markdown-v1"])) throw new Error("Taxonomy v7 semanticOwnedFileProjectionContracts must contain the exact artifact-facet, README/LICENSE, ticket-document, active, and history contracts");
  const semanticPathProjectionReferenceConsumerContracts: Record<string, SemanticPathProjectionReferenceConsumerContract> = {};
  const referenceConsumerForms = new Set<SemanticPathProjectionReferenceConsumerForm>(["path-reference", "artifact-catalog-glob", "artifact-catalog-prose:root-marker", "artifact-catalog-prose:relative-root", "artifact-catalog-prose:category-glob", "artifact-catalog-prose:catalog-grammar"]);
  const referenceConsumerAdapters = new Set<SemanticPathProjectionReferenceConsumerContract["adapters"][number]>(["rust", "typescript", "json", "toml"]);
  const referenceConsumerIdentities = new Set<string>();
  for (const [id, value] of Object.entries(projectionConsumerRows)) {
    const spec = requireRecord(value, `semanticPathProjectionReferenceConsumerContracts.${id}`);
    requireExactKeys(spec, ["projectionContractId", "consumerIdentity", "ownership", "sourcePathPattern", "sourcePathIdentities", "adapters", "supportedForms", "staleMarkers"], `semanticPathProjectionReferenceConsumerContracts.${id}`);
    const projectionContractId = requireString(spec.projectionContractId, `semanticPathProjectionReferenceConsumerContracts.${id}.projectionContractId`);
    if (!semanticPathProjectionContracts[projectionContractId]) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} references an unknown projection contract`);
    if (spec.ownership !== "external") throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.ownership must be external`);
    const consumerIdentity = requireString(spec.consumerIdentity, `semanticPathProjectionReferenceConsumerContracts.${id}.consumerIdentity`);
    if (referenceConsumerIdentities.has(consumerIdentity)) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts repeats consumer identity ${consumerIdentity}`);
    referenceConsumerIdentities.add(consumerIdentity);
    const sourcePathPattern = requireString(spec.sourcePathPattern, `semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathPattern`);
    if (!sourcePathPattern.startsWith("^") || !sourcePathPattern.endsWith("$")) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathPattern must be a full-match expression`);
    const sourcePathRegex = new RegExp(sourcePathPattern, "u");
    const sourcePathIdentities = requireStringArray(spec.sourcePathIdentities, `semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathIdentities`);
    const adapters = requireStringArray(spec.adapters, `semanticPathProjectionReferenceConsumerContracts.${id}.adapters`) as readonly SemanticPathProjectionReferenceConsumerContract["adapters"][number][];
    const supportedForms = requireStringArray(spec.supportedForms, `semanticPathProjectionReferenceConsumerContracts.${id}.supportedForms`) as readonly SemanticPathProjectionReferenceConsumerForm[];
    const staleMarkers = requireStringArray(spec.staleMarkers, `semanticPathProjectionReferenceConsumerContracts.${id}.staleMarkers`);
    if (sourcePathIdentities.length === 0 || adapters.length === 0 || supportedForms.length === 0 || staleMarkers.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} must be nonempty`);
    if (new Set(sourcePathIdentities).size !== sourcePathIdentities.length || sourcePathIdentities.some((path) => path !== normalizeRelative(path) || !sourcePathRegex.test(path))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathIdentities are invalid`);
    if (new Set(adapters).size !== adapters.length || adapters.some((adapter) => !referenceConsumerAdapters.has(adapter))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.adapters are invalid`);
    if (new Set(supportedForms).size !== supportedForms.length || supportedForms.some((form) => !referenceConsumerForms.has(form))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.supportedForms are invalid`);
    if (new Set(staleMarkers).size !== staleMarkers.length || staleMarkers.some((marker) => !marker || marker !== marker.normalize("NFC"))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.staleMarkers are invalid`);
    semanticPathProjectionReferenceConsumerContracts[id] = { projectionContractId, consumerIdentity, ownership: "external", sourcePathPattern, sourcePathIdentities: [...sourcePathIdentities], adapters: [...adapters], supportedForms: [...supportedForms], staleMarkers: [...staleMarkers] };
  }
  if (Object.keys(semanticPathProjectionReferenceConsumerContracts).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionReferenceConsumerContracts must not be empty");
  if (mutationCatalogProjectionRow.contractKind !== "canonical-mutation-case-pair" || mutationCatalogProjectionRow.contractId !== "canonical-mutation-case-pair-v1" || mutationCatalogProjectionRow.coverage !== "every-catalog-vector-has-one-implementation-and-one-fixture-bundle" || !Array.isArray(mutationCatalogProjectionRow.implementationSegments) || !Array.isArray(mutationCatalogProjectionRow.fixtureSegments)) throw new Error("Taxonomy v7 mutationCatalogProjection must be the canonical mutation case pair contract");
  const mutationCatalogProjection: MutationCatalogProjectionContractIds = {
    contractKind: "canonical-mutation-case-pair",
    contractId: "canonical-mutation-case-pair-v1",
    sourceOwnerKindId: requireString(mutationCatalogProjectionRow.sourceOwnerKindId, "mutationCatalogProjection.sourceOwnerKindId"),
    projectedMemberKindId: requireString(mutationCatalogProjectionRow.projectedMemberKindId, "mutationCatalogProjection.projectedMemberKindId"),
    implementationSegments: mutationCatalogProjectionRow.implementationSegments.map((value, index) => parseProjectionSegment(value, `mutationCatalogProjection.implementationSegments[${index}]`, false) as SemanticProjectionSourceSegment),
    fixtureSegments: mutationCatalogProjectionRow.fixtureSegments.map((value, index) => parseProjectionSegment(value, `mutationCatalogProjection.fixtureSegments[${index}]`, false) as SemanticProjectionSourceSegment),
    implementationDescendantContractId: requireString(mutationCatalogProjectionRow.implementationDescendantContractId, "mutationCatalogProjection.implementationDescendantContractId"),
    fixtureDescendantContractId: requireString(mutationCatalogProjectionRow.fixtureDescendantContractId, "mutationCatalogProjection.fixtureDescendantContractId"),
    catalogContractId: requireString(mutationCatalogProjectionRow.catalogContractId, "mutationCatalogProjection.catalogContractId"),
    coverage: "every-catalog-vector-has-one-implementation-and-one-fixture-bundle",
  };
  if (!semanticDirectoryMemberKinds[mutationCatalogProjection.sourceOwnerKindId] || !semanticProjectedMemberKinds[mutationCatalogProjection.projectedMemberKindId] || !semanticDescendantContracts[mutationCatalogProjection.implementationDescendantContractId] || !semanticDescendantContracts[mutationCatalogProjection.fixtureDescendantContractId] || !semanticPathProjectionCatalogContracts[mutationCatalogProjection.catalogContractId]) throw new Error("Taxonomy v7 mutationCatalogProjection references unknown canonical pair registries");

  const generatorContracts: Record<string, GeneratorContractSpec> = {};
  const generatorRoots: { readonly id: string; readonly path: string }[] = [];
  for (const [id, value] of Object.entries(generatorRows)) {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id)) throw new Error(`Taxonomy v7 generatorContracts.${id} has an invalid identifier`);
    const spec = requireRecord(value, `generatorContracts.${id}`);
    if (spec.ownership !== "owned" && spec.ownership !== "external") throw new Error(`Taxonomy v7 generatorContracts.${id}.ownership is invalid`);
    const ownership = spec.ownership as GeneratorOwnership;
    const ownerPath = spec.ownerPath === null ? null : normalizeRelative(requireString(spec.ownerPath, `generatorContracts.${id}.ownerPath`));
    const target = spec.target === null ? null : requireString(spec.target, `generatorContracts.${id}.target`);
    const previewTarget = spec.previewTarget === undefined ? undefined : requireString(spec.previewTarget, `generatorContracts.${id}.previewTarget`);
    const checkTarget = spec.checkTarget === undefined ? undefined : requireString(spec.checkTarget, `generatorContracts.${id}.checkTarget`);
    if ((ownership === "owned") !== (ownerPath !== null && target !== null)) throw new Error(`Taxonomy v7 generatorContracts.${id} owner and target do not match ownership`);
    if (target && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(target)) throw new Error(`Taxonomy v7 generatorContracts.${id}.target must be one exact Nx target`);
    if (ownership === "owned" ? !previewTarget : previewTarget !== undefined) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget does not match ownership`);
    if (previewTarget && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(previewTarget)) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget must be one exact Nx target`);
    const previewArguments = spec.previewArguments === undefined ? undefined : requireStringArray(spec.previewArguments, `generatorContracts.${id}.previewArguments`);
    if (target) generatorPreviewScriptArguments({ ownership, target, previewTarget, previewArguments });
    else if (previewArguments !== undefined) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewArguments requires owned output authority`);
    const previewProgress = spec.previewProgress === undefined ? undefined : parseGeneratorPreviewProgressPolicyV1(spec.previewProgress);
    if (previewProgress && (!target || !previewTarget || ownership !== "owned")) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewProgress requires owned preview authority`);
    const previewLimits = spec.previewLimits === undefined ? undefined : generatorPreviewResourceLimits({ ownership, previewTarget, previewLimits: spec.previewLimits as GeneratorContractSpec["previewLimits"] });
    const compilerInputManifest = spec.compilerInputManifest === undefined ? undefined : requireRecord(spec.compilerInputManifest, `generatorContracts.${id}.compilerInputManifest`) as unknown as GeneratorContractSpec["compilerInputManifest"];
    if (checkTarget && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(checkTarget)) throw new Error(`Taxonomy v7 generatorContracts.${id}.checkTarget must be one exact Nx target`);
    const inputPatterns = requireStringArray(spec.inputPatterns, `generatorContracts.${id}.inputPatterns`).map((pattern, index) => validatedContractPattern(pattern, `generatorContracts.${id}.inputPatterns[${index}]`, false));
    if (ownership === "owned" ? inputPatterns.length === 0 : inputPatterns.length !== 0) throw new Error(`Taxonomy v7 generatorContracts.${id}.inputPatterns do not match ownership`);
    const outputRows = spec.outputRoots;
    if (!Array.isArray(outputRows) || outputRows.length === 0) throw new Error(`Taxonomy v7 generatorContracts.${id}.outputRoots must not be empty`);
    const outputRoots = outputRows.map((value, index) => {
      const output = requireRecord(value, `generatorContracts.${id}.outputRoots[${index}]`);
      const outputPath = requireString(output.path, `generatorContracts.${id}.outputRoots[${index}].path`);
      if (outputPath !== normalizeRelative(outputPath) || /[*?\[\]]/u.test(outputPath)) throw new Error(`Taxonomy v7 generatorContracts.${id} output path must be one literal NFC repository path`);
      if (output.inclusion !== "tracked" && output.inclusion !== "ignored") throw new Error(`Taxonomy v7 generatorContracts.${id} output inclusion is invalid`);
      generatorRoots.push({ id, path: outputPath });
      return { path: outputPath, inclusion: output.inclusion } as GeneratorOutputRootSpec;
    }).sort((left, right) => left.path.localeCompare(right.path));
    if (new Set(outputRoots.map((output) => output.path)).size !== outputRoots.length) throw new Error(`Taxonomy v7 generatorContracts.${id} repeats an output root`);
    const inputDiscovery = spec.inputDiscovery === undefined ? undefined : requireRecord(spec.inputDiscovery, `generatorContracts.${id}.inputDiscovery`) as unknown as RegistryCatalogInputDiscovery;
    if (inputDiscovery && (id !== "plugin-registry" || ownership !== "owned" || inputDiscovery.kind !== "registry-catalog")) throw new Error(`Taxonomy v7 generatorContracts.${id}.inputDiscovery has no exact catalog authority`);
    const projectionActivation = spec.projectionActivation === undefined ? undefined : requireRecord(spec.projectionActivation, `generatorContracts.${id}.projectionActivation`) as unknown as GeneratorProjectionActivation;
    const packageGeneration = spec.packageGeneration === undefined ? undefined : requireRecord(spec.packageGeneration, `generatorContracts.${id}.packageGeneration`) as unknown as SemanticPackageGeneration;
    generatorContracts[id] = { ownership, ownerPath, target, previewTarget, previewArguments, previewProgress, previewLimits, checkTarget, inputPatterns: [...new Set(inputPatterns)].sort(), inputDiscovery, compilerInputManifest, packageGeneration, projectionActivation, outputRoots, reason: requireString(spec.reason, `generatorContracts.${id}.reason`) };
  }
  if (Object.keys(generatorContracts).length === 0) throw new Error("Taxonomy v7 generatorContracts must not be empty");
  for (let left = 0; left < generatorRoots.length; left++) for (let right = left + 1; right < generatorRoots.length; right++) {
    const a = generatorRoots[left];
    const b = generatorRoots[right];
    if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`)) throw new Error(`Taxonomy v7 generator output roots overlap: ${a.id}:${a.path} and ${b.id}:${b.path}`);
  }

  const ecosystems: Record<string, EcosystemSpec> = {};
  for (const [id, value] of Object.entries(ecosystemRows)) {
    const spec = requireRecord(value, `ecosystems.${id}`);
    if (spec.packageIdentity !== "manifest" && spec.packageIdentity !== "boundary-only") throw new Error(`Taxonomy v7 ecosystems.${id}.packageIdentity is invalid`);
    const manifestContractId = spec.manifestContractId === null ? null : requireString(spec.manifestContractId, `ecosystems.${id}.manifestContractId`);
    if ((spec.packageIdentity === "manifest") !== (manifestContractId !== null)) throw new Error(`Taxonomy v7 ecosystems.${id} manifest identity is incomplete`);
    ecosystems[id] = { packageIdentity: spec.packageIdentity, manifestContractId };
  }
  if (Object.keys(ecosystems).length === 0) throw new Error("Taxonomy v7 ecosystems must not be empty");

  const packageGlueGrammar: Record<string, PackageGlueGrammar> = {};
  for (const [id, value] of Object.entries(grammarRows)) {
    const spec = requireRecord(value, `packageGlueGrammar.${id}`);
    if (!["rust", "typescript", "javascript", "go", "python", "dotnet", "c-cpp", "tex"].includes(String(spec.analyzer))) throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.analyzer is invalid`);
    const allowedRoles = requireStringArray(spec.allowedRoles, `packageGlueGrammar.${id}.allowedRoles`) as PackageGlueGrammar["allowedRoles"];
    if (allowedRoles.some((role) => !["declaration", "registration", "bootstrap", "thin-delegation"].includes(role)) || new Set(allowedRoles).size !== allowedRoles.length) throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.allowedRoles is invalid`);
    if (!Number.isSafeInteger(spec.maxDelegationStatements) || (spec.maxDelegationStatements as number) < 0) throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.maxDelegationStatements is invalid`);
    packageGlueGrammar[id] = { analyzer: spec.analyzer as PackageGlueGrammar["analyzer"], allowedRoles, maxDelegationStatements: spec.maxDelegationStatements as number };
  }

  const packageBoundaryRules: Record<string, PackageBoundaryRule> = {};
  for (const [id, value] of Object.entries(boundaryRows)) {
    const spec = requireRecord(value, `packageBoundaryRules.${id}`);
    const glueGrammarId = requireString(spec.glueGrammarId, `packageBoundaryRules.${id}.glueGrammarId`);
    if (!packageGlueGrammar[glueGrammarId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown grammar ${glueGrammarId}`);
    if (spec.recursive !== true || spec.uncertainRole !== "problem" || spec.implementationRole !== "problem") throw new Error(`Taxonomy v7 packageBoundaryRules.${id} must be recursive and fail closed`);
    packageBoundaryRules[id] = {
      manifestContractId: spec.manifestContractId === null ? null : requireString(spec.manifestContractId, `packageBoundaryRules.${id}.manifestContractId`),
      entryContractIds: requireStringArray(spec.entryContractIds, `packageBoundaryRules.${id}.entryContractIds`),
      allowedFixedContractIds: requireStringArray(spec.allowedFixedContractIds, `packageBoundaryRules.${id}.allowedFixedContractIds`),
      allowedFileKindIds: requireStringArray(spec.allowedFileKindIds, `packageBoundaryRules.${id}.allowedFileKindIds`),
      allowedDirectoryKindIds: requireStringArray(spec.allowedDirectoryKindIds, `packageBoundaryRules.${id}.allowedDirectoryKindIds`),
      glueGrammarId,
      recursive: true,
      uncertainRole: "problem",
      implementationRole: "problem",
    };
    const rule = packageBoundaryRules[id];
    if (rule.manifestContractId && !fixedFilenameContracts[rule.manifestContractId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown manifest contract ${rule.manifestContractId}`);
    for (const contractId of rule.entryContractIds) if (!configurableEntryContracts[contractId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown entry contract ${contractId}`);
    for (const contractId of rule.allowedFixedContractIds) if (!fixedFilenameContracts[contractId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown fixed contract ${contractId}`);
    for (const kindId of rule.allowedFileKindIds) if (!fileKinds[kindId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown file kind ${kindId}`);
    for (const kindId of rule.allowedDirectoryKindIds) if (!semanticDirectoryKinds[kindId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown directory kind ${kindId}`);
  }
  const packageBoundaryProfiles: Record<string, PackageBoundaryProfile> = {};
  for (const [id, value] of Object.entries(boundaryProfileRows)) {
    const spec = requireRecord(value, `packageBoundaryProfiles.${id}`);
    requireExactKeys(spec, ["admission", "allowedFileKindIds", "allowedDirectoryKindIds", "allowedFixedContractIds", "glueGrammarId", "recursive", "uncertainRole", "implementationRole", "reason"], `packageBoundaryProfiles.${id}`);
    if (spec.admission !== "blocked-until-language-directory-registered" || spec.recursive !== true || spec.uncertainRole !== "problem" || spec.implementationRole !== "problem") throw new Error(`Taxonomy v7 packageBoundaryProfiles.${id} must remain fail-closed`);
    const glueGrammarId = requireString(spec.glueGrammarId, `packageBoundaryProfiles.${id}.glueGrammarId`);
    if (!packageGlueGrammar[glueGrammarId]) throw new Error(`Taxonomy v7 packageBoundaryProfiles.${id} references unknown grammar`);
    packageBoundaryProfiles[id] = { admission: "blocked-until-language-directory-registered", allowedFileKindIds: requireStringArray(spec.allowedFileKindIds, `packageBoundaryProfiles.${id}.allowedFileKindIds`), allowedDirectoryKindIds: requireStringArray(spec.allowedDirectoryKindIds, `packageBoundaryProfiles.${id}.allowedDirectoryKindIds`), allowedFixedContractIds: requireStringArray(spec.allowedFixedContractIds, `packageBoundaryProfiles.${id}.allowedFixedContractIds`), glueGrammarId, recursive: true, uncertainRole: "problem", implementationRole: "problem", reason: requireString(spec.reason, `packageBoundaryProfiles.${id}.reason`) };
  }
  if (Object.keys(packageBoundaryProfiles).length === 0) throw new Error("Taxonomy v7 packageBoundaryProfiles must not be empty");
  /** 🔖️ Each externally-mandated tool-config validator token is reserved for exactly one contract id, mirroring the pre-existing vitest-configuration/vitest-config-entry pinning. */
  const TOOL_CONFIG_VALIDATORS: Readonly<Record<string, string>> = { "vitest-configuration": "vitest-config-entry", "tool-config-tailwind": "tailwind-config", "tool-config-postcss": "postcss-config", "tool-config-eslint": "eslint-config", "tool-config-dependency-cruiser": "dependency-cruiser-config", "pytest-configuration": "root-pytest-config", "eslint-configuration": "root-eslint-config", "vscode-test-configuration": "vscode-test-cli-config" };
  const packageSourceDispositions: Record<string, PackageSourceDisposition> = {};
  for (const [id, value] of Object.entries(sourceDispositionRows)) {
    const spec = requireRecord(value, `packageSourceDispositions.${id}`);
    requireExactKeys(spec, ["contractKind", "disposition", "validator", ...(spec.grammarId === undefined ? [] : ["grammarId"]), "authority", "verification"], `packageSourceDispositions.${id}`);
    const configValidatorOwner = TOOL_CONFIG_VALIDATORS[spec.validator as string];
    if (spec.contractKind !== "fixed" && spec.contractKind !== "configurable" || spec.disposition !== "adapter-source" && spec.disposition !== "tool-metadata" || spec.validator !== "package-glue" && spec.validator !== "command-router" && configValidatorOwner === undefined || (configValidatorOwner !== undefined && id !== configValidatorOwner)) throw new Error(`Taxonomy v7 packageSourceDispositions.${id} is invalid`);
    const grammarId = spec.grammarId === undefined ? undefined : requireString(spec.grammarId, `packageSourceDispositions.${id}.grammarId`);
    if (grammarId !== undefined && !packageGlueGrammar[grammarId]) throw new Error(`Taxonomy v7 packageSourceDispositions.${id} references unknown grammar ${grammarId}`);
    packageSourceDispositions[id] = { contractKind: spec.contractKind, disposition: spec.disposition, validator: requireLiteral(spec.validator, `packageSourceDispositions.${id}.validator`, ["package-glue", "command-router", "vitest-configuration", "tool-config-vitest", "tool-config-tailwind", "tool-config-postcss", "tool-config-eslint", "tool-config-dependency-cruiser", "pytest-configuration", "eslint-configuration", "vscode-test-configuration"] as const), ...(grammarId === undefined ? {} : { grammarId }), authority: requireString(spec.authority, `packageSourceDispositions.${id}.authority`), verification: requireString(spec.verification, `packageSourceDispositions.${id}.verification`) };
  }
  if (Object.keys(packageSourceDispositions).length === 0) throw new Error("Taxonomy v7 packageSourceDispositions must not be empty");
  for (const [id, contract] of Object.entries(fixedFilenameContracts)) if (contract.scope.kind === "package-root" && !packageBoundaryRules[contract.scope.ecosystemId]) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id} references unknown ecosystem ${contract.scope.ecosystemId}`);

  const pathExclusions: Record<string, { path: string; mode: "opaque"; reason: string }> = {};
  const exclusions: { id: string; path: string }[] = [];
  for (const [id, value] of Object.entries(exclusionRows)) {
    const spec = requireRecord(value, `pathExclusions.${id}`);
    if (spec.mode !== "opaque") throw new Error(`Taxonomy v7 pathExclusions.${id}.mode must be opaque`);
    const excludedPath = normalizeRelative(requireString(spec.path, `pathExclusions.${id}.path`));
    pathExclusions[id] = { path: excludedPath, mode: "opaque", reason: requireString(spec.reason, `pathExclusions.${id}.reason`) };
    exclusions.push({ id, path: excludedPath });
  }
  if (canonicalJson(Object.entries(pathExclusions).map(([id, spec]) => [id, spec.path])) !== canonicalJson(TAXONOMY_OPAQUE_PATH_EXCLUSIONS.map(([id, path]) => [id, normalizeRelative(path)]))) throw new Error(`Taxonomy v7 pathExclusions must contain exactly opaque ${TAXONOMY_OPAQUE_PATH_EXCLUSIONS.map(([, path]) => path).join(", ")}`);
  for (const id of requireStringArray(enforcement.opaquePathExclusionIds, "areaEnforcement.opaquePathExclusionIds")) {
    if (!pathExclusions[id]) throw new Error(`Taxonomy v7 areaEnforcement references unknown opaque exclusion ${id}`);
  }
  if (canonicalJson(enforcement.opaquePathExclusionIds) !== canonicalJson(TAXONOMY_OPAQUE_PATH_EXCLUSIONS.map(([id]) => id))) throw new Error(`Taxonomy v7 areaEnforcement must require ${TAXONOMY_OPAQUE_PATH_EXCLUSIONS.map(([id]) => id).join(", ")} in order`);
  const opaquePaths = Object.values(pathExclusions).map((entry) => entry.path);
  const crossesOpaque = (value: string): boolean => opaquePaths.some((opaque) => value === opaque || value.startsWith(`${opaque}/`) || opaque.startsWith(`${value}/`));
  for (const [id, contract] of Object.entries(semanticPathProjectionReferenceConsumerContracts)) {
    if (contract.sourcePathIdentities.some(crossesOpaque)) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} crosses an opaque path`);
    const pattern = new RegExp(contract.sourcePathPattern, "u");
    if (opaquePaths.some((opaque) => pattern.test(opaque) || pattern.test(`${opaque}/probe`))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} admits an opaque path`);
  }
  for (const [id, contract] of Object.entries(generatorContracts)) {
    if (contract.ownerPath && crossesOpaque(contract.ownerPath)) throw new Error(`Taxonomy v7 generatorContracts.${id}.ownerPath crosses an opaque path`);
    for (const pattern of contract.inputPatterns) if (opaquePaths.some((opaque) => pathMatcher.matches(opaque, pattern) || pathMatcher.matches(`${opaque}/probe`, pattern))) throw new Error(`Taxonomy v7 generatorContracts.${id} input pattern admits an opaque path`);
    for (const output of contract.outputRoots) if (crossesOpaque(output.path)) throw new Error(`Taxonomy v7 generatorContracts.${id} output root crosses an opaque path`);
  }

  const schema: TaxonomyV7 = {
    schemaVersion: 7,
    windowEmptyFacetFileKindId: requireString(root.windowEmptyFacetFileKindId, "windowEmptyFacetFileKindId"),
    fileKinds,
    semanticDirectoryKinds,
    fixedFilenameContracts,
    fixedFilenameRejectionContracts,
    fixedDirectoryContracts,
    configurableEntryContracts,
    fileKindResolutionRules,
    scopedFileKinds,
    semanticDirectoryMemberKinds,
    semanticProjectedMemberKinds,
    semanticPathProjectionProfileRenderers,
    semanticDescendantContracts,
    semanticPathProjectionCatalogContracts,
    semanticPathProjectionContracts,
    semanticOwnedFileProjectionContracts,
    semanticPackageProjectionContracts: root.semanticPackageProjectionContracts as DiscoveryTaxonomy["semanticPackageProjectionContracts"],
    semanticPathProjectionReferenceConsumerContracts,
    mutationCatalogProjection,
    generatorContracts,
    ecosystems,
    packageBoundaryRules,
    packageBoundaryProfiles,
    packageGlueGrammar,
    packageSourceDispositions,
    pathExclusions,
    unicodeNormalization: { form: "NFC", caseFold: "lower", locale: "und" },
    variationSelectorPolicy: { selector: "\uFE0F", requiredAfterEmoji: true, comparison: "ignore-selector" },
    collisionPolicy: {
      comparisons: collision.comparisons as TaxonomyV7["collisionPolicy"]["comparisons"],
      maxPathBytes: collision.maxPathBytes as number,
      rejectWindowsReservedNames: collision.rejectWindowsReservedNames === true,
      rejectTrailingDotsAndSpaces: collision.rejectTrailingDotsAndSpaces === true,
    },
    areaEnforcement: { requiredState: "clean", undeclaredAreas: "enforce", opaquePathExclusionIds: [...(enforcement.opaquePathExclusionIds as string[])] },
  };
  return {
    schema,
    discoverySchema: root as unknown as DiscoveryTaxonomy,
    exclusions: exclusions.sort((a, b) => a.path.localeCompare(b.path)),
    fileKinds: Object.entries(fileKinds).map(([id, spec]) => ({ id, ...spec })).sort((a, b) => a.id.localeCompare(b.id)),
    directoryKinds: Object.entries(semanticDirectoryKinds).map(([id, spec]) => ({ id, ...spec, slugRegex: new RegExp(`^(?:${spec.slugPattern})$`, "u") })).sort((a, b) => a.id.localeCompare(b.id)),
  };
}

/** 🧭️ What a generator-input walk reads from a loaded taxonomy: the discovery vocabulary it classifies with and
 * the matcher it excludes with. Narrower than [[LoadedTaxonomy]] so a caller can build one without the walk's
 * private loading state. */
export type GeneratorInputTaxonomy = Pick<LoadedTaxonomy, "discoverySchema" | "pathMatcher" | "exclusions">;

/** 🧠️ Keeps private content-derived facts while every load captures its own physical input and session. */
const PARSED_TAXONOMIES = new Map<string, TaxonomyContentFacts>();

const PARSED_TAXONOMY_CAPACITY = 8;

export function loadTaxonomy(options: TaxonomyLoadOptions): LoadedTaxonomy {
  const path = assertLexicalInputOutsideOpaque(options.repoRoot, options.taxonomyPath ?? TAXONOMY_RELATIVE_PATH, "taxonomyPath", true);
  const ancestors = noFollowDirectoryChain(resolve(options.repoRoot));
  const input = semanticOwnedInputFileSnapshot(options.repoRoot, relative(resolve(options.repoRoot), path).replaceAll("\\", "/"));
  verifyNoFollowDirectoryChain(ancestors);
  if (!input) throw new Error("Taxonomy schema is absent: " + path);
  let parsed = PARSED_TAXONOMIES.get(input.contentHash);
  if (!parsed) {
    const bytes = Buffer.from(input.bytes), text = bytes.toString("utf8");
    if (!Buffer.from(text).equals(bytes)) throw new Error("Taxonomy schema has lossy UTF-8: " + path);
    const values = parseTaxonomy(JSON.parse(text) as unknown, path);
    if (PARSED_TAXONOMIES.size >= PARSED_TAXONOMY_CAPACITY) PARSED_TAXONOMIES.delete(PARSED_TAXONOMIES.keys().next().value!);
    PARSED_TAXONOMIES.set(input.contentHash, values);
    parsed = values;
  }
  return { ...structuredClone(parsed), path, input, pathMatcher: createTaxonomyPathMatcher() };
}
