//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js: shared taxonomy vocabulary + repo-wide package discovery contract.
//#endregion 🧲️Header

//#region 🔌️Adapters
import { ephemeralMap, ephemeralBox } from "@semio-tech/framework";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { closeSync, constants, existsSync, fstatSync, lstatSync, openSync, readFileSync, readdirSync, realpathSync, statSync } from "node:fs";
import { basename, dirname, extname, isAbsolute, join, posix, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
//#endregion 🔌️Adapters

const __dirname = dirname(fileURLToPath(import.meta.url));

//#region 🔣️Taxonomy
/** 🗺️ Repository-wide enforcement state; version 7 has no quiet or transitional area mode. */
export type AreaState = "clean";

/** 📈️ Derived (never declared) state of one already-migrated owner: `clean` once nothing but the taxonomy shape is left, `mixed` while residuals survive — see `discoverOwners`. */
export type PackageMaturity = "clean" | "mixed";

/** 🧩️ Semantic responsibility owned by one collection member. */
export type SemanticKind = "inference" | "mutation" | "io" | "module" | "artifact" | "standard" | "subset" | "plugin" | "product" | "extension" | "capability" | "ui" | "action" | "app" | "command";

/** 🚪️ Exact boundary direction of one I/O semantic collection. */
export type SemanticIoDirection = "import" | "export" | "transport";

/** 🧭️ Lowest legal owner of a reusable module. */
export type SemanticOwnerLevel = "subset" | "standard" | "artifact" | "app" | "plugin" | "product" | "s" | "framework";

/** 🗂️ Schema entry describing a recognized semantic collection directory. */
export interface SemanticCollectionSpec {
  readonly kind: SemanticKind;
  readonly direction?: SemanticIoDirection;
}

/** 🏷️ One exact child declared by a collection root's canonical manifest file kind. */
export interface SemanticMember {
  readonly directory: string;
  readonly id: string;
  readonly kind: SemanticKind;
  readonly responsibility: string;
  readonly generator?: string;
  readonly inference?: { readonly inputs: readonly string[]; readonly target: string };
  readonly mutation?: { readonly command: string; readonly event: string };
  readonly io?: { readonly format: string; readonly direction: SemanticIoDirection };
  readonly module?: { readonly productionConsumers: readonly string[] };
}

/** 🏷️ Where an ecosystem's semio role marker lives: a `table` inside the package manifest, addressed dotted (`package.metadata.semio` for TOML, `metadata.semio` for JSON). */
export interface EcosystemMarkerSpec {
  readonly in: "manifest";
  readonly format: "toml" | "json";
  readonly table: string;
  readonly roleKey: string;
  readonly idKey: string;
}

/** 🌐️ Per-language packaging contract expressed only through kind and exact-contract identifiers. */
export interface Ecosystem {
  readonly packageIdentity: "manifest" | "boundary-only";
  readonly manifestContractId: string | null;
  readonly moduleRootContractId: string | null;
  readonly marker: EcosystemMarkerSpec | null;
  readonly componentFileKindId: string;
  readonly sourceFileKindIds: readonly string[];
  readonly entryContractIds: readonly string[];
  readonly packagingDirectoryKindIds?: readonly string[];
}

/** 🎯️ One render/build target expressed through file-kind and entry-contract identifiers. */
export interface TargetSpec {
  readonly lang: string;
  readonly componentFileKindId: string;
  readonly entryContractIds: readonly string[];
}

/** 📄️ Canonical kind-only file identity; every extension chain includes its leading dot. */
export interface FileKindSpec {
  readonly emoji: string;
  readonly extensionChains: readonly string[];
  readonly role: "source" | "schema" | "specification" | "configuration" | "documentation" | "test" | "asset" | "generated" | "marker";
}

/** 📁️ Registered semantic directory identity and its permitted slug shape. */
export interface SemanticDirectoryKindSpec {
  readonly emoji: string;
  readonly slugPattern: string;
  readonly allowEmojiOnly: boolean;
  readonly inferWithoutEmoji?: boolean;
  readonly projectionOnly?: boolean;
  readonly parentKindIds?: readonly string[];
}

/** 🧭️ Contextual source-name rule selecting one global file kind without extension guessing. */
export interface FileKindResolutionRuleSpec {
  readonly extensionChain: string;
  readonly fileKindId: string;
  readonly priority: 0;
}

/** 🎟️ Owner-scoped evidence kind whose suffix is never admitted globally. */
export interface ScopedFileKindSpec {
  readonly pathPattern: string;
  readonly parentDirectoryKindId?: string;
  readonly emoji: string;
  readonly extensionChains: readonly string[];
  readonly role: "evidence";
  readonly sourceFilenamePattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly verification: string;
  readonly expires: string | null;
}

/** 🧩️ Exact owner-local semantic member identity loaded after structural directory kinds. */
export interface SemanticDirectoryMemberKindSpec {
  readonly ownerKindIds: readonly string[];
  readonly memberNames: readonly string[];
  readonly source: "registry";
}

/** 🪞️ Context-owned semantic member projected from one exact source registry. */
export interface SemanticProjectedMemberKindSpec {
  readonly ownerKindIds: readonly string[];
  readonly projectionContractId: string;
  readonly sourceMemberKindId: string;
  readonly identityField: "mutationDirectoryName" | "commandDirectoryName";
}

export type SemanticProjectionCaptureField = "standardVersion" | "subsetId" | "mutationId" | "scenarioId" | "commandDirectoryName";

/** 🧩️ One literal or captured source segment in a projection grammar. */
export type SemanticProjectionSourceSegment =
  | Readonly<{ kindId: string; literal: string }>
  | Readonly<{ kindId: string; capture: SemanticProjectionCaptureField }>
  | Readonly<{ memberKindId: string; literal: string }>
  | Readonly<{ projectedMemberKindId: string; capture: SemanticProjectionCaptureField }>;

/** 🎯️ One literal, forward-rendered, or identity-copied destination segment. */
export type SemanticProjectionDestinationSegment =
  | Readonly<{ kindId: string; literal: string }>
  | Readonly<{ kindId: string; render: "profile" }>
  | Readonly<{ kindId: string; copy: SemanticProjectionCaptureField }>
  | Readonly<{ projectedMemberKindId: string; copy: SemanticProjectionCaptureField }>;

/** 🪆️ Forward-only profile renderer; profile strings are never a reverse identity source. */
export interface SemanticPathProjectionProfileRenderer {
  readonly direction: "forward-only";
  readonly captureFields: readonly ["standardVersion", "subsetId"];
  readonly directoryKindId: string;
  readonly template: "🪆️{standardVersion}-{subsetId}";
  readonly tupleCollisionFields: readonly ["artifactId", "standardVersion", "subsetId"];
}

/** 🧱️ Exact directory ancestry plus one canonical file or directory kind. */
export interface SemanticDescendantDirectoryNode {
  readonly pathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly nodeType: "directory";
  readonly kindId: string;
}

/** 📄️ One physical-format leaf inside an exact projected bundle. */
export interface SemanticDescendantKindFileNode {
  readonly pathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly nodeType: "file";
  readonly kindId: string;
  readonly sourceFilename?: string;
}

/** 🔒️ One externally fixed filename inside an exact projected bundle. */
export interface SemanticDescendantFixedFileNode {
  readonly pathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly nodeType: "file";
  readonly fixedFilenameContractId: string;
}

/** 🚪️ One source-only package leaf projected to a configurable canonical entry. */
export interface SemanticDescendantConfigurableEntryFileNode {
  readonly sourcePathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly destinationPathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly nodeType: "file";
  readonly configurableEntry: Readonly<{
    contractId: string;
    sourceFilename: string;
    configurationReferences: readonly Readonly<{ fixedFilenameContractId: string; adapter: "json" | "toml"; structuredLocation: string }>[];
  }>;
}

export type SemanticDescendantNode = SemanticDescendantDirectoryNode | SemanticDescendantKindFileNode | SemanticDescendantFixedFileNode | SemanticDescendantConfigurableEntryFileNode;

/** ↔️ Exactly-one group within an otherwise required descendant bundle. */
export interface SemanticDescendantAlternative {
  readonly id: string;
  readonly mode: "exactly-one";
  readonly nodes: readonly SemanticDescendantNode[];
}

/** 📦️ Exact physical descendant shape and its derived deepest suffix reserve. */
export interface SemanticExactDescendantContract {
  readonly rootDirectoryKindId: string;
  readonly requiredNodes: readonly SemanticDescendantNode[];
  readonly exclusiveAlternatives: readonly SemanticDescendantAlternative[];
  readonly realizedNodeCount: number;
  readonly pathBudgetReserve: Readonly<{ derivation: "longest-canonical-descendant-suffix"; bytes: number }>;
}

/** 📚️ A manifest-owned recursive catalog whose realized descendants are data-driven. */
export interface SemanticCatalogDescendantContract {
  readonly contractKind: "catalog";
  readonly rootDirectoryKindId: string;
  readonly catalogContractId: string;
  readonly leafFileKindId: string;
  readonly rendering: "semantic-member-directory-and-physical-kind-leaf";
  readonly pathBudgetReserve: Readonly<{ derivation: "longest-rendered-catalog-descendant-suffix"; bytes: number }>;
}

export type SemanticDescendantContract = SemanticExactDescendantContract | SemanticCatalogDescendantContract;

/** 📚️ Catalog-facing physical vector registry; runtime kinds remain independent. */
export interface SemanticMutationPathProjectionCatalogContract {
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

/** 🗃️ One exact category grammar in a distributed JSON manifest catalog. */
export type SemanticCatalogCategoryRule =
  | Readonly<{ sourceDirectoryName: string; directoryKindId: string; sourceShape: "direct-semantic-json"; manifestSchema: string; memberDirectoryEmoji: string }>
  | Readonly<{ sourceDirectoryName: string; directoryKindId: string; sourceShape: "nested-fixed-json"; manifestSchema: string; fixedSourceFilename: string }>;

/** 🏗️ Strict manifest authority for one recursively projected CAD model catalog. */
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
  readonly categoryRules: readonly SemanticCatalogCategoryRule[];
  readonly coverage: "every-source-file-and-destination-node-exactly-once";
  readonly unknownCategoryPolicy: "problem";
  readonly unownedModelPolicy: "problem";
}

/** 🎮️ One exact owner vector set for projected command bundles. */
export interface SemanticExactOwnerVectorsCatalogContract {
  readonly contractKind: "exact-owner-vectors";
  readonly required: true;
  readonly allowEmpty: false;
  readonly identityFields: readonly ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"];
  readonly coverage: "every-physical-command-bundle-exactly-once";
  readonly vectors: readonly Readonly<{ artifactId: string; standardVersion: string; subsetId: string; commandDirectoryName: string }>[];
}

export type SemanticPathProjectionCatalogContract = SemanticMutationPathProjectionCatalogContract | SemanticDistributedJsonManifestCatalogContract | SemanticExactOwnerVectorsCatalogContract;

/** 🛤️ Exact source capture and forward-only destination grammar. */
export interface SemanticPathProjectionContract {
  readonly sourceOwnerKindId: string;
  readonly sourceArtifactMemberName?: string;
  readonly sourceSegments: readonly SemanticProjectionSourceSegment[];
  readonly profileRendererId: string;
  readonly destinationOwnerKindId: string;
  readonly destinationSegments: readonly SemanticProjectionDestinationSegment[];
  readonly descendantContractId: string;
  readonly catalogContractId: string;
  readonly rationaleRule: "artifact-mutation-test-projection-v1" | "artifact-example-model-catalog-projection-v1" | "artifact-editor-command-projection-v1";
}

/** 📌️ One active owner-and-sibling-manifest governed semantic leaf projection. */
export interface SemanticLifecycleOwnedFileProjectionContract {
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

/** 📓️ One ticket-owner governed historical semantic leaf projection. */
export interface SemanticHistoryOwnedFileProjectionContract {
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

/** 🖋️ Exact authored documentation splices activated only by a frozen owner-leaf move. */
export interface SemanticOwnedDocumentCorrection {
  readonly contractKind: "exact-owner-content-splices";
  readonly activation: "owner-leaf-move";
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly preimage: Readonly<{ sha256: string; mode: "0644"; size: number }>;
  readonly postimage: Readonly<{ sha256: string; size: number }>;
  readonly replacementFixedFilenameContractId: "root-script";
  readonly splices: readonly Readonly<{ line: number; startByte: number; endByte: number; oldValue: string; newValue: string; linePreimage: string }>[];
  readonly rationaleRule: "owner-script-filename-documentation-v1";
}

/** 🪪️ One reviewed current preimage with immutable catalog and baseline lineage. */
export interface SemanticOwnedCurrentSourceRevision {
  readonly catalogCaseIndex: 31;
  readonly sourcePath: string;
  readonly baselineCommit: string;
  readonly baselineBlob: string;
  readonly baselinePreimage: Readonly<{ sha256: string; size: number; mode: "0644" }>;
  readonly currentPreimage: Readonly<{ sha256: string; size: number; mode: "0644" }>;
  readonly expectationsPath: string;
  readonly expectationsSha256: string;
}

/** 🧾️ Exact observed expectation bytes and no-follow node witnesses supplied by the caller. */
export interface SemanticOwnedCurrentSourceExpectation {
  readonly path: string;
  readonly nodeKind: string;
  readonly mode: number;
  readonly ancestorNodeKinds: readonly string[];
  readonly bytes: Uint8Array;
}

/** 📸️ Exact bytes and file tuple captured through one no-follow descriptor. */
export interface SemanticOwnedInputFileSnapshot extends SemanticOwnedCurrentSourceExpectation {
  readonly nodeKind: "file";
  readonly contentHash: string;
  readonly size: number;
}

/** 🔏️ A selected source tuple and its revision digest without altering historical catalog rows. */
export interface SemanticOwnedCurrentSourcePreimageResult {
  readonly disposition: "none" | "catalog" | "revised" | "problem";
  readonly catalogCaseIndex: number | null;
  readonly preimage: Readonly<{ sha256: string; size: number; mode: "0644" }> | null;
  readonly revisionId: string | null;
  readonly revisionDigest: string | null;
  readonly problems: readonly string[];
}

/** 📚️ One digest-locked exact owner-file catalog with no basename-wide inference. */
export interface SemanticExactOwnedFileProjectionContract {
  readonly contractKind: "exact-owner-path-catalog";
  readonly authorityCatalogPath: string;
  readonly authorityCatalogSha256: string;
  readonly sourceFileKindId: string;
  readonly sourceBasenames: readonly ["LICENSE.md", "README.md"];
  readonly destinationDirectoryKinds: Readonly<{
    readonly license: Readonly<{ readonly directoryKindId: string; readonly directoryName: "⚖️license"; readonly filename: "📝️.md" }>;
    readonly readme: Readonly<{ readonly directoryKindId: string; readonly directoryName: "📃️readme"; readonly filename: "📝️.md" }>;
  }>;
  readonly allowedDispositions: readonly ["attribution-relocate", "configurable-owner-license-relocate", "fixed", "generated-evidence-relocate", "owner-documentation-relocate"];
  readonly ownerEvidenceKinds: readonly ["configurable-owner-license", "ordinary-owner-doc", "package-publication", "third-party-attribution", "ticket-evidence", "ticket-scratch"];
  readonly referenceOwnerIds: readonly ["asset-distribution-owner", "bun-package-publisher", "commonmark-scratch-rust-reader", "markdown-relative-reference-adapter", "repo-cli-dev-docs-go", "vscode-package-ignore"];
  readonly generatorOwnerIds: readonly ["assets-build"];
  readonly expectedCounts: Readonly<{ readonly fixed: 4; readonly license: 8; readonly projected: 36; readonly readme: 32; readonly referenceBindings: 62; readonly total: 40 }>;
  readonly authoredDocumentCorrections: Readonly<Record<string, SemanticOwnedDocumentCorrection>>;
  readonly currentSourceRevisions?: Readonly<Record<string, SemanticOwnedCurrentSourceRevision>>;
  readonly rationaleRule: "readme-license-owner-projection-v1";
}

/** 📄️ One primary physical leaf governed directly by its semantic owner. */
export interface SemanticPrimaryOwnedFileProjectionContract {
  readonly contractKind: "owner-primary-file";
  readonly ownerFixedDirectoryContractId: string;
  readonly sourceFileKindId: string;
  readonly sourceFilename: string;
  readonly destinationFilename: string;
  readonly rationaleRule: "ticket-document-primary-markdown-v1";
}

/** 🫙️ A primary facet leaf whose complete semantic owner path is registered. */
export interface SemanticFacetPrimaryFileProjectionContract {
  readonly contractKind: "semantic-facet-primary-file";
  readonly sourceRoot: string;
  readonly sourceFilename: string;
  readonly fileKindAuthority: "windowEmptyFacetFileKindId";
  readonly sourceDisposition: "authored";
  readonly directoryCaptures: Readonly<Record<string, Readonly<{ kindIds: readonly string[]; names?: readonly string[] }>>>;
  readonly ownerPathPatterns: Readonly<Record<string, string>>;
  readonly authoringCommand: Readonly<{ scriptPath: string; command: readonly ["new", "surface"]; writeDisposition: "create-if-absent" }>;
  readonly referenceConsumer: Readonly<{ path: string; ownerRoot: string; adapter: "rust"; region: "✏️👁️Surfaces"; lineTemplate: string }>;
  readonly rationaleRule: "artifact-empty-facet-primary-markdown-v1";
}

export type SemanticOwnedFileProjectionContract = SemanticExactOwnedFileProjectionContract | SemanticFacetPrimaryFileProjectionContract | SemanticHistoryOwnedFileProjectionContract | SemanticLifecycleOwnedFileProjectionContract | SemanticPrimaryOwnedFileProjectionContract;

/** 📦️ Exact nested Cargo source authority, composed with semantic package purity. */
export interface SemanticPackageProjectionContract {
  readonly contractKind: "exact-nested-cargo-package-catalog";
  readonly authorityCatalogPath: string;
  readonly authorityCatalogSha256: string;
  readonly packageIds: readonly ["wgpu-renderer", "jcoprobe-guest"];
  readonly sourceLeafCounts: readonly [32, 4];
  readonly purityCount: 27;
  readonly adapterCount: 5;
  readonly derivedLeafCount: 1;
  readonly joinedPathBindingCounts: readonly [1, 0];
  readonly generatedSourceRetirementCounts: readonly [1, 0];
  readonly authoredFragmentCounts: readonly [31, 0];
  readonly rationaleRule: "nested-cargo-package-projection-v1";
}

/** 🔒️ Exact manifest-user coordinates in authoritative dependency approval state. */
export interface SemanticPolicyStateCoordinateContract {
  readonly contractKind: "dependency-freeze-user-coordinates";
  readonly statePath: "🔒️dependencies.json";
  readonly stateSchemaVersion: 2;
  readonly sourceDisposition: "authored-policy-state";
  readonly ownerProjectionContractId: "nested-cargo-packages-v1";
  readonly packageIds: readonly ["jcoprobe-guest"];
  readonly manifestFilename: "Cargo.toml";
  readonly dependencyEvidenceField: "witDependency";
  readonly coordinatePointer: "/entries/*/users/*";
  readonly preserveNonCoordinateBytes: true;
}

export type SemanticPathProjectionReferenceConsumerForm = "path-reference" | "artifact-catalog-glob" | "artifact-catalog-prose:root-marker" | "artifact-catalog-prose:relative-root" | "artifact-catalog-prose:interaction-glob" | "artifact-catalog-prose:catalog-grammar";

/** 🔭️ One external projection consumer admitted by exact path, adapter, form, and stale marker. */
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

/** 🔗️ Exact schema IDs consumed by MutationCatalog validation without aliases. */
export interface MutationCatalogProjectionContractIds {
  readonly projectionContractId: string;
  readonly projectedMemberKindId: string;
  readonly descendantContractId: string;
  readonly catalogContractId: string;
}

/** 🧭️ Tagged scope predicate for an exact externally-authoritative path contract. */
export type FixedContractScope =
  | { readonly kind: "exact-path"; readonly path: string }
  | { readonly kind: "repository-root" }
  | { readonly kind: "package-root"; readonly ecosystemId: string }
  | { readonly kind: "directory-kind"; readonly directoryKindId: string }
  | { readonly kind: "fixed-directory-contract"; readonly fixedDirectoryContractId: string }
  | { readonly kind: "fixed-directory-contract-set"; readonly fixedDirectoryContractIds: readonly string[] }
  | { readonly kind: "named-fixed-directory-contract-set"; readonly fixedDirectoryContractSetId: string }
  | { readonly kind: "sibling-fixed-filename-contract"; readonly fixedFilenameContractId: string }
  | { readonly kind: "path-pattern" };

/** 🔒️ Exact externally-authoritative filename at one semantic scope. */
export interface FixedFilenameContract {
  readonly pathPattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly configurability: "unconfigurable";
  readonly scope: FixedContractScope;
  readonly verification: string;
  readonly expires: string | null;
}

/** 📁️ Exact externally-authoritative directory path at one semantic scope. */
export interface FixedDirectoryContract {
  readonly pathPattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly configurability: "unconfigurable";
  readonly descendants?: "reserved";
  readonly scope: Exclude<FixedContractScope, { readonly kind: "package-root" }>;
  readonly verification: string;
  readonly expires: string | null;
}

/** 🚫️ Exact fixed-looking paths that must normalize or relocate instead of gaining an exception. */
export interface FixedFilenameRejectionContract {
  readonly sourcePathIdentities: readonly string[];
  readonly disposition: "normalize" | "relocate";
  readonly reason: string;
}

/** 🔧️ Exact configurable package entry and the metadata locations that own its path. */
export interface ConfigurableEntryContract {
  readonly filename: string;
  readonly fileKindId: string;
  readonly ecosystemId: string;
  readonly role: "library" | "binary" | "bootstrap" | "setup" | "worker";
  readonly configurationSources: readonly string[];
}

/** 🧱️ Recursive package boundary contract; uncertain content is always a problem. */
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

/** 🧠️ Conservative source-role grammar selected by ecosystem. */
export interface PackageGlueGrammarSpec {
  readonly analyzer: "rust" | "typescript" | "javascript" | "go" | "python" | "dotnet" | "c-cpp";
  readonly allowedRoles: readonly ("declaration" | "registration" | "bootstrap" | "thin-delegation")[];
  readonly maxDelegationStatements: number;
}

/** 🧾️ Explicit authority and validator for a source-format fixed or configurable package entry. */
export interface PackageSourceDisposition {
  readonly contractKind: "fixed" | "configurable";
  readonly disposition: "adapter-source" | "tool-metadata";
  readonly validator: "package-glue" | "command-router" | "vitest-configuration" | "tool-config-vitest" | "tool-config-tailwind" | "tool-config-postcss" | "tool-config-eslint" | "tool-config-dependency-cruiser" | "pytest-configuration" | "eslint-configuration" | "vscode-test-configuration";
  readonly authority: string;
  readonly verification: string;
}

/** 🧱️ Native package boundary declared before an authoritative language-directory identity exists. */
export interface PackageBoundaryProfile {
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

/** 🗃️ Whether one generator is runnable by normalization or retained only as a fail-closed ownership finding. */
export type GeneratorOwnership = "owned" | "external";

/** 🎯 Exact generated output root plus its Git inventory channel. */
export interface GeneratorOutputRoot {
  readonly path: string;
  readonly inclusion: "tracked" | "ignored";
}

/** 📇️ Catalog content, membership and implementation dependency authority. */
export interface RegistryCatalogInputDiscovery {
  readonly kind: "registry-catalog";
  readonly previewInput: { readonly protocol: "registry-projected-inputs-v1"; readonly maxBytes: 67108864; readonly maxOperations: 200000 };
  readonly descriptorRelativePath: string;
  readonly exampleDirectoryName: string;
  readonly exampleFileKindId: string;
  readonly implementationEntryPaths: readonly string[];
  readonly workspaceImports: Readonly<Record<string, { readonly manifestPath: string; readonly entryPath: string }>>;
}

/** 🚦️ First generation requires exact package projection or complete canonical package authority. */
export interface GeneratorProjectionActivation {
  readonly kind: "canonical-or-planned-package";
  readonly projectionContractId: "nested-cargo-packages-v1";
  readonly packageId: "jcoprobe-guest" | "wgpu-renderer";
  readonly sourceManifestPath: string;
  readonly destinationManifestPath: string;
}

/** 🏗️ Exact package artifacts and bounded projected inputs share one producer authority. */
export interface SemanticPackageGeneration {
  readonly kind: "wgpu-package-artifacts";
  readonly previewInput: { readonly protocol: "package-projected-inputs-v1"; readonly maxBytes: 67108864; readonly maxOperations: 200000 };
  readonly catalogPath: string;
  readonly catalogSha256: string;
  readonly browserProfile: SemanticPackageBrowserProfile;
}

/** 🧩️ Explicit current JCO coordinates, separate from digest-locked historical projection evidence. */
export interface CurrentJcoPackageDestination {
  readonly kind: "jco-canonical-package-v1";
  readonly packageId: "jcoprobe-guest";
  readonly semanticOwnerRoot: string;
  readonly packageRoot: string;
  readonly cargoManifestPath: string;
  readonly cargoLockPath: string;
  readonly componentPath: string;
  readonly witPath: string;
  readonly adapterPath: string;
}

/** ⚙️ Schema-owned generator identity; runnable commands derive only from exact Nx targets. */
export interface GeneratorContract {
  readonly ownership: GeneratorOwnership;
  readonly ownerPath: string | null;
  readonly target: string | null;
  readonly previewTarget?: string;
  readonly previewArguments?: readonly string[];
  readonly previewLimits?: { readonly maxOutputBytes: number; readonly timeoutMs: number };
  readonly compilerInputManifest?: { readonly kind: "compiler-input-manifest-v1"; readonly manifestOutputPath: string; readonly manifestSchemaPath: string; readonly staticAuthorityPath: string; readonly maxFiles: number };
  readonly checkTarget?: string;
  readonly inputPatterns: readonly string[];
  readonly inputDiscovery?: RegistryCatalogInputDiscovery;
  readonly packageGeneration?: SemanticPackageGeneration;
  readonly currentPackageDestination?: CurrentJcoPackageDestination;
  readonly projectionActivation?: GeneratorProjectionActivation;
  readonly outputRoots: readonly GeneratorOutputRoot[];
  readonly reason: string;
}

/** 🚫️ A path prefix that must be filtered lexically before filesystem access. */
export interface PathExclusion {
  readonly path: string;
  readonly mode: "opaque";
  readonly reason: string;
}

/** 🔏️Repository path-identity rules for every non-reserved file and directory. */
export interface PathEmojiPolicy {
  readonly inventory: "git-visible";
  readonly identity: "single-emoji-grapheme";
  readonly siblingNamespace: "files-and-directories";
  readonly genericEmojiIdentities: readonly string[];
  readonly reservedSubtreeDirectoryNames: readonly string[];
}

/** 📂️One Git-visible path presented to the language-neutral path-emoji statutes. */
export interface PathEmojiEntry {
  readonly path: string;
  readonly nodeKind: "directory" | "file";
  readonly reserved?: boolean;
}

export type PathEmojiFindingKind = "missing" | "generic" | "presentation" | "spacing" | "duplicate" | "multiple" | "reserved-emoji";

/** ⚠️One deterministic path-emoji statute finding. */
export interface PathEmojiFinding {
  readonly kind: PathEmojiFindingKind;
  readonly path: string;
  readonly sibling?: string;
  readonly emoji?: string;
}

const PATH_EMOJI_SEGMENTER = new Intl.Segmenter("und", { granularity: "grapheme" });

/** 😀️Splits the complete leading emoji sequence from a path segment. */
export function leadingEmojiIdentity(value: string): Readonly<{ emoji: string; rest: string; first: string }> {
  let emoji = "", first = "";
  for (const { segment } of PATH_EMOJI_SEGMENTER.segment(value.normalize("NFC"))) {
    if (!/[\p{Extended_Pictographic}\p{Emoji_Presentation}\uFE0F\u20E3]/u.test(segment)) break;
    if (!first) first = segment;
    emoji += segment;
  }
  return { emoji, rest: value.slice(emoji.length), first };
}

/** 🪞️Folds presentation selectors for logical path-identity comparisons. */
export function foldPathEmojiIdentity(value: string): string {
  return value.normalize("NFC").replaceAll("\uFE0E", "").replaceAll("\uFE0F", "");
}

/** 📖️Identifies reserved documentation names independently of their optional format extension. */
export function reservedDocumentationBasename(name: string): string | null {
  const identity = leadingEmojiIdentity(name);
  return /^(?:README(?:\.[^/]+)?|LICENSE(?:\.[^/]+)?|AGENTS\.md)$/u.test(identity.rest) ? identity.rest : null;
}

const RGI_EMOJI_SEQUENCE = new RegExp("^(?:\\p{RGI_Emoji})$", "v");

/** 🔠️Recognizes one RGI emoji or one explicitly presented pictograph. */
function canonicalTaxonomyEmoji(value: unknown): value is string {
  return typeof value === "string" && value === value.normalize("NFC") && (RGI_EMOJI_SEQUENCE.test(value) || /^\p{Extended_Pictographic}\uFE0F$/u.test(value));
}

/** ⚖️Evaluates path emoji rules in one namespace shared by file and directory siblings. */
export function pathEmojiStatuteFindings(entries: readonly PathEmojiEntry[], genericEmojiIdentities: readonly string[]): PathEmojiFinding[] {
  const generic = new Set(genericEmojiIdentities.map(foldPathEmojiIdentity));
  const seen = new Map<string, PathEmojiEntry>();
  const findings: PathEmojiFinding[] = [];
  const sorted = [...entries].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
  for (const entry of sorted) {
    const name = entry.path.split("/").at(-1) ?? "";
    const identity = leadingEmojiIdentity(name);
    if (entry.nodeKind === "file" && reservedDocumentationBasename(name)) {
      if (identity.emoji) findings.push({ kind: "reserved-emoji", path: entry.path, emoji: identity.emoji });
      continue;
    }
    if (entry.reserved) continue;
    if (!identity.emoji) {
      findings.push({ kind: "missing", path: entry.path });
      continue;
    }
    if (identity.emoji !== identity.first || /[\p{Extended_Pictographic}\p{Emoji_Presentation}\u20E3]/u.test(identity.rest)) findings.push({ kind: "multiple", path: entry.path, emoji: identity.emoji });
    if (generic.has(foldPathEmojiIdentity(identity.first))) findings.push({ kind: "generic", path: entry.path, emoji: identity.first });
    const firstCodePoint = [...identity.first][0] ?? "";
    if (firstCodePoint && !/\p{Emoji_Presentation}/u.test(firstCodePoint) && !identity.first.includes("\uFE0F")) findings.push({ kind: "presentation", path: entry.path, emoji: identity.first });
    if (/^\s/u.test(identity.rest)) findings.push({ kind: "spacing", path: entry.path, emoji: identity.emoji });
    const parent = entry.path.includes("/") ? entry.path.slice(0, entry.path.lastIndexOf("/")) : "";
    const key = `${parent}\0${foldPathEmojiIdentity(identity.first)}`;
    const previous = seen.get(key);
    if (previous) findings.push({ kind: "duplicate", path: entry.path, sibling: previous.path, emoji: foldPathEmojiIdentity(identity.first) });
    else seen.set(key, entry);
  }
  return findings;
}

/** 🦀️ One valid way of writing the entry file's `#[path]` strings — see `Taxonomy.rustEntryPathRules`. */
export interface RustEntryPathConvention {
  readonly id: string;
  readonly outerReset: string | null;
  readonly groupingReset: string;
  readonly leafPrefix: string;
}

/** 🦀️ Cumulative-`#[path]` base rules for the relocated (Shape V2) rust entry file. */
export interface RustEntryPathRules {
  readonly _comment?: string;
  readonly entryDirFromOwner: string;
  readonly resolution: "cumulative";
  readonly groupingResetPath: string;
  readonly leafPathPrefix: string;
  readonly conventions: readonly RustEntryPathConvention[];
}

//#region 🔒️Frozen Coordinate Evidence
/** 📜️ Exact immutable Markdown source coordinates with bounded block and inline syntax. */
export interface FrozenMarkdownCoordinateEvidenceContract {
  readonly path: string;
  readonly grammar: "frozen-markdown-source-coordinates-v1";
  readonly sha256: string;
  readonly coordinates: readonly Readonly<{ start: number; end: number; kind: "source"; form: "inline-code" | "path-list-item"; valueSha256: string }>[];
}

/** 🧷️ Validates exact Markdown declarations without reading or discovering evidence. */
export function validateFrozenMarkdownCoordinateEvidenceContracts(value: unknown): string[] {
  const problems: string[] = [], paths = new Set<string>();
  const object = (entry: unknown): entry is Record<string, unknown> => entry !== null && typeof entry === "object" && !Array.isArray(entry);
  if (!object(value)) return ["frozenMarkdownCoordinateEvidenceContracts must be an object."];
  for (const [id, row] of Object.entries(value)) {
    const label = `frozenMarkdownCoordinateEvidenceContracts.${id}`;
    if (!/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(id)) problems.push(`${label} has an invalid contract id.`);
    if (!object(row)) { problems.push(`${label} must be an object.`); continue; }
    if (Object.keys(row).sort().join("\0") !== "coordinates\0grammar\0path\0sha256" || row.grammar !== "frozen-markdown-source-coordinates-v1") problems.push(`${label} requires the exact Markdown source-coordinate grammar and fields.`);
    if (typeof row.path !== "string" || !row.path.endsWith(".md") || /[\\:*?"<>|\u0000-\u001f]/u.test(row.path) || row.path.split("/").some((part) => !part || part === "." || part === "..") || /^(?:compose|temp\/compose)(?:\/|$)/u.test(row.path)) problems.push(`${label}.path must be one exact non-opaque repository-relative Markdown document.`);
    else { if (paths.has(row.path)) problems.push(`${label}.path duplicates another evidence owner.`); paths.add(row.path); }
    if (typeof row.sha256 !== "string" || !/^[a-f0-9]{64}$/u.test(row.sha256)) problems.push(`${label}.sha256 must bind exact document bytes.`);
    if (!Array.isArray(row.coordinates) || row.coordinates.length === 0) { problems.push(`${label}.coordinates must be a nonempty array.`); continue; }
    const spans: { start: number; end: number }[] = [];
    for (const [index, coordinate] of row.coordinates.entries()) {
      if (!object(coordinate)) { problems.push(`${label}.coordinates[${index}] must be an object.`); continue; }
      if (Object.keys(coordinate).sort().join("\0") !== "end\0form\0kind\0start\0valueSha256" || coordinate.kind !== "source" || !["inline-code", "path-list-item"].includes(String(coordinate.form))) problems.push(`${label}.coordinates[${index}] requires one exact source-span form.`);
      if (!Number.isSafeInteger(coordinate.start) || Number(coordinate.start) < 0 || !Number.isSafeInteger(coordinate.end) || Number(coordinate.end) <= Number(coordinate.start)) problems.push(`${label}.coordinates[${index}] requires a nonempty exact UTF-16 span.`);
      else spans.push({ start: Number(coordinate.start), end: Number(coordinate.end) });
      if (typeof coordinate.valueSha256 !== "string" || !/^[a-f0-9]{64}$/u.test(coordinate.valueSha256)) problems.push(`${label}.coordinates[${index}].valueSha256 must bind exact UTF-8 path bytes.`);
    }
    spans.sort((left, right) => left.start - right.start || left.end - right.end);
    if (spans.some((span, index) => index > 0 && span.start < spans[index - 1].end)) problems.push(`${label}.coordinates must not overlap or duplicate a span.`);
  }
  return problems;
}

/** 🔐️ Exact immutable JSON evidence with explicitly typed physical-coordinate value locations. */
export interface FrozenCoordinateEvidenceContract {
  readonly path: string;
  readonly sha256: string;
  readonly schemaVersion: number | null;
  readonly rootKind?: "array";
  readonly coordinates: readonly (Readonly<{ pointer: string; kind: "source" | "destination" }> | Readonly<{ pointer: string; kind: "source" | "destination"; representation: "recorded-repository-absolute"; recordedRepositoryRoot: string }> | Readonly<{ pointer: string; kind: "source"; representation: "recorded-package-owner-identity"; identityPrefix: "unmarked:" }> | Readonly<{ pointer: string; kind: "source"; representation: "json-escaped-source-path" }>)[];
}

/** 🧾️ Validates explicit evidence authority without discovering or reading any document. */
export function validateFrozenCoordinateEvidenceContracts(value: unknown): string[] {
  const problems: string[] = [], paths = new Set<string>();
  const object = (candidate: unknown): candidate is Record<string, unknown> => candidate !== null && typeof candidate === "object" && !Array.isArray(candidate);
  if (!object(value)) return ["frozenCoordinateEvidenceContracts must be an object."];
  for (const [id, row] of Object.entries(value)) {
    const label = `frozenCoordinateEvidenceContracts.${id}`;
    if (!/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(id)) problems.push(`${label} has an invalid contract id.`);
    if (!object(row)) { problems.push(`${label} must be an object.`); continue; }
    const arrayRoot = Object.hasOwn(row, "rootKind");
    if (Object.keys(row).sort().join("\0") !== (arrayRoot ? "coordinates\0path\0rootKind\0schemaVersion\0sha256" : "coordinates\0path\0schemaVersion\0sha256")) problems.push(`${label} must contain only its exact document-root fields.`);
    if (arrayRoot && (row.rootKind !== "array" || row.schemaVersion !== null)) problems.push(`${label}.rootKind requires explicit array authority with absent schemaVersion.`);
    const path = row.path;
    if (typeof path !== "string" || !path.endsWith(".json") || /[\\:*?"<>|\u0000-\u001f]/u.test(path) || path.split("/").some((part) => !part || part === "." || part === "..") || /^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) problems.push(`${label}.path must be one exact non-opaque repository-relative JSON document.`);
    else { if (paths.has(path)) problems.push(`${label}.path duplicates another evidence owner.`); paths.add(path); }
    if (typeof row.sha256 !== "string" || !/^[a-f0-9]{64}$/u.test(row.sha256)) problems.push(`${label}.sha256 must bind exact document bytes.`);
    if (row.schemaVersion !== null && (!Number.isSafeInteger(row.schemaVersion) || Number(row.schemaVersion) < 1)) problems.push(`${label}.schemaVersion must be a positive integer or explicit absent-property null authority.`);
    if (!Array.isArray(row.coordinates) || row.coordinates.length === 0) { problems.push(`${label}.coordinates must be a nonempty array.`); continue; }
    const pointers = new Set<string>();
    for (const [index, coordinate] of row.coordinates.entries()) {
      if (!object(coordinate)) { problems.push(`${label}.coordinates[${index}] must be an object.`); continue; }
      const recorded = Object.hasOwn(coordinate, "representation");
      const ownerIdentity = coordinate.representation === "recorded-package-owner-identity";
      const escapedSource = coordinate.representation === "json-escaped-source-path";
      if (Object.keys(coordinate).sort().join("\0") !== (ownerIdentity ? "identityPrefix\0kind\0pointer\0representation" : escapedSource ? "kind\0pointer\0representation" : recorded ? "kind\0pointer\0recordedRepositoryRoot\0representation" : "kind\0pointer")) { problems.push(`${label}.coordinates[${index}] must contain only the fields of its exact coordinate representation.`); continue; }
      if (escapedSource) {
        if (coordinate.kind !== "source") problems.push(`${label}.coordinates[${index}] requires source-only JSON encoding authority.`);
      } else if (ownerIdentity) {
        if (coordinate.identityPrefix !== "unmarked:" || coordinate.kind !== "source") problems.push(`${label}.coordinates[${index}] requires the exact unmarked: source-owner identity prefix.`);
      } else if (recorded) {
        const root = coordinate.recordedRepositoryRoot;
        if (coordinate.representation !== "recorded-repository-absolute" || typeof root !== "string" || !/^(?:\/(?!\/)|[A-Za-z]:\/).+/u.test(root) || /[\\*?"<>|\u0000-\u001f]/u.test(root) || root.replace(/^[A-Za-z]:/u, "").slice(1).split("/").some((part) => !part || part === "." || part === ".." || part.includes(":"))) problems.push(`${label}.coordinates[${index}] requires one exact lexical POSIX or drive-qualified recorded repository root.`);
      }
      if (typeof coordinate.pointer !== "string" || !/^(?:\/(?:\*|(?:[^/~*]|~[01])+))+$/u.test(coordinate.pointer) || /[\u0000-\u001f]/u.test(coordinate.pointer)) problems.push(`${label}.coordinates[${index}].pointer must use exact JSON-pointer segments or array-index wildcards.`);
      else { if (pointers.has(coordinate.pointer)) problems.push(`${label}.coordinates[${index}].pointer duplicates another declaration.`); pointers.add(coordinate.pointer); }
      if (coordinate.kind !== "source" && coordinate.kind !== "destination") problems.push(`${label}.coordinates[${index}].kind must be source or destination.`);
    }
  }
  return problems;
}
//#endregion 🔒️Frozen Coordinate Evidence

//#region 🗂️Historical Document Evidence
/** 🗂️ A whole-document historical-evidence population — ticket narrative reports, ticket workspace (evidence snapshots, scratch scripts, working notes), Cursor plan snapshots and per-developer prompt-log transcripts are never live references, so the engine excludes them from reference-candidate scanning entirely instead of freezing them coordinate-by-coordinate. Document KIND is the discriminator, not ticket lifecycle status. Membership never overrides a real machine-read contract: the engine additionally refuses to exempt any path matching a `fixedFilenameContracts` pattern, or sitting inside a directory that owns a package-root manifest (Cargo.toml/package.json/go.mod/…, derived from `fixedFilenameContracts[*].scope.kind === "package-root"`), so a future contract addition is protected automatically. */
export interface HistoricalDocumentEvidencePopulation {
  readonly grammar: "historical-document-evidence-v1";
  readonly directoryPattern: string;
  readonly leafPattern: string;
  readonly reason: string;
}

const HISTORICAL_DOCUMENT_EVIDENCE_POPULATION_IDS = ["ticket-report", "ticket-workspace", "cursor-plan-snapshot", "dev-prompt-log"] as const;

/** 🧷️ Validates the exact, closed, four-population historical-document-evidence grammar without touching the filesystem. */
export function validateHistoricalDocumentEvidencePopulations(value: unknown): string[] {
  const problems: string[] = [];
  const object = (entry: unknown): entry is Record<string, unknown> => entry !== null && typeof entry === "object" && !Array.isArray(entry);
  if (!object(value)) return ["historicalDocumentEvidencePopulations must be an object."];
  if (Object.keys(value).join("\0") !== HISTORICAL_DOCUMENT_EVIDENCE_POPULATION_IDS.join("\0")) problems.push('historicalDocumentEvidencePopulations must contain exactly ordered "ticket-report", "ticket-workspace", "cursor-plan-snapshot" and "dev-prompt-log" populations.');
  for (const id of HISTORICAL_DOCUMENT_EVIDENCE_POPULATION_IDS) {
    const row = (value as Record<string, unknown>)[id];
    const label = `historicalDocumentEvidencePopulations.${id}`;
    if (!object(row)) { problems.push(`${label} must be an object.`); continue; }
    if (Object.keys(row).sort().join("\0") !== "directoryPattern\0grammar\0leafPattern\0reason" || row.grammar !== "historical-document-evidence-v1") problems.push(`${label} requires the exact historical-document-evidence grammar and fields.`);
    if (typeof row.directoryPattern !== "string" || !row.directoryPattern) problems.push(`${label}.directoryPattern must be a nonempty glob.`);
    if (typeof row.leafPattern !== "string") problems.push(`${label}.leafPattern must be a regular-expression source string.`);
    else try { void new RegExp(row.leafPattern, "u"); } catch { problems.push(`${label}.leafPattern must be a valid regular expression.`); }
    if (typeof row.reason !== "string" || !row.reason) problems.push(`${label}.reason must be a nonempty justification.`);
  }
  const ticketReport = (value as Record<string, Record<string, unknown> | undefined>)["ticket-report"];
  if (object(ticketReport) && (ticketReport.directoryPattern !== "**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**" || ticketReport.leafPattern !== "^📓️.+\\.md$")) problems.push("historicalDocumentEvidencePopulations.ticket-report must use the exact ticket-document scope and leaf grammar.");
  const ticketWorkspace = (value as Record<string, Record<string, unknown> | undefined>)["ticket-workspace"];
  if (object(ticketWorkspace) && (ticketWorkspace.directoryPattern !== "**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**" || ticketWorkspace.leafPattern !== "^.+$")) problems.push("historicalDocumentEvidencePopulations.ticket-workspace must use the exact ticket-root nested-descendant scope and leaf grammar.");
  const cursorPlanSnapshot = (value as Record<string, Record<string, unknown> | undefined>)["cursor-plan-snapshot"];
  if (object(cursorPlanSnapshot) && (cursorPlanSnapshot.directoryPattern !== ".cursor/plans/*" || cursorPlanSnapshot.leafPattern !== "^.+\\.plan\\.md$")) problems.push("historicalDocumentEvidencePopulations.cursor-plan-snapshot must use the exact Cursor plan-snapshot scope.");
  const devPromptLog = (value as Record<string, Record<string, unknown> | undefined>)["dev-prompt-log"];
  if (object(devPromptLog) && (devPromptLog.directoryPattern !== "**/.🧬semio/🦑️repo/💬️prompts/**" || devPromptLog.leafPattern !== "^.+\\.md$")) problems.push("historicalDocumentEvidencePopulations.dev-prompt-log must use the exact prompt-log scope and leaf grammar.");
  return problems;
}
//#endregion 🗂️Historical Document Evidence

/**
 * 🔣️ Shape of `🔣️taxonomy.json` — the single source of truth for taxonomy directory-name/role/lang
 * vocabulary and the package-discovery contract, replacing the two independently hand-maintained copies in
 * framework/os/plugin/registry script.ts (`TAXONOMY_ARTIFACT_COMPONENTS`/`TAXONOMY_WINDOW_CHILDREN`) and
 * root script.ts (`POLICY_ARTIFACT_COMPONENT_DIRS`/`POLICY_WINDOW_CHILD_DIRS`) — see master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`.
 */
export interface Taxonomy {
  readonly _comment?: string;
  readonly schemaVersion: number;
  readonly fileKinds: Readonly<Record<string, FileKindSpec>>;
  readonly fileKindResolutionRules: Readonly<Record<string, FileKindResolutionRuleSpec>>;
  readonly scopedFileKinds: Readonly<Record<string, ScopedFileKindSpec>>;
  readonly semanticDirectoryKinds: Readonly<Record<string, SemanticDirectoryKindSpec>>;
  readonly semanticDirectoryMemberKinds: Readonly<Record<string, SemanticDirectoryMemberKindSpec>>;
  readonly semanticProjectedMemberKinds: Readonly<Record<string, SemanticProjectedMemberKindSpec>>;
  readonly semanticPathProjectionProfileRenderers: Readonly<Record<string, SemanticPathProjectionProfileRenderer>>;
  readonly semanticDescendantContracts: Readonly<Record<string, SemanticDescendantContract>>;
  readonly semanticPathProjectionCatalogContracts: Readonly<Record<string, SemanticPathProjectionCatalogContract>>;
  readonly semanticPathProjectionContracts: Readonly<Record<string, SemanticPathProjectionContract>>;
  readonly semanticOwnedFileProjectionContracts: Readonly<Record<string, SemanticOwnedFileProjectionContract>>;
  readonly semanticPackageProjectionContracts: Readonly<Record<string, SemanticPackageProjectionContract>>;
  readonly semanticPolicyStateCoordinateContracts: Readonly<Record<string, SemanticPolicyStateCoordinateContract>>;
  readonly semanticPathProjectionReferenceConsumerContracts: Readonly<Record<string, SemanticPathProjectionReferenceConsumerContract>>;
  readonly mutationCatalogProjection: MutationCatalogProjectionContractIds;
  readonly fixedFilenameContracts: Readonly<Record<string, FixedFilenameContract>>;
  readonly fixedFilenameRejectionContracts: Readonly<Record<string, FixedFilenameRejectionContract>>;
  readonly fixedDirectoryContracts: Readonly<Record<string, FixedDirectoryContract>>;
  readonly fixedDirectoryContractSets?: Readonly<Record<string, readonly string[]>>;
  readonly configurableEntryContracts: Readonly<Record<string, ConfigurableEntryContract>>;
  readonly packageBoundaryRules: Readonly<Record<string, PackageBoundaryRule>>;
  readonly packageBoundaryProfiles: Readonly<Record<string, PackageBoundaryProfile>>;
  readonly packageGlueGrammar: Readonly<Record<string, PackageGlueGrammarSpec>>;
  readonly packageSourceDispositions: Readonly<Record<string, PackageSourceDisposition>>;
  readonly generatorContracts: Readonly<Record<string, GeneratorContract>>;
  readonly pathExclusions: Readonly<Record<string, PathExclusion>>;
  readonly pathEmojiPolicy: PathEmojiPolicy;
  readonly unicodeNormalization: { readonly form: "NFC"; readonly caseFold: "lower"; readonly locale: "und" };
  readonly variationSelectorPolicy: { readonly selector: "\uFE0F"; readonly requiredAfterEmoji: true; readonly comparison: "ignore-selector" };
  readonly collisionPolicy: {
    readonly comparisons: readonly ("byte" | "nfc" | "case-fold" | "vs16-fold" | "same-kind")[];
    readonly maxPathBytes: number;
    readonly rejectWindowsReservedNames: true;
    readonly rejectTrailingDotsAndSpaces: true;
  };
  readonly areaEnforcement: { readonly requiredState: "clean"; readonly undeclaredAreas: "enforce"; readonly opaquePathExclusionIds: readonly string[] };
  readonly semanticManifestFileKindId: string;
  /** 📇️ Exact collection-owner manifest filenames whose semantic direction requires a stemmed identity. */
  readonly semanticManifestFilenameOverrides?: Readonly<Record<string, string>>;
  readonly semanticExtensionKey: string;
  readonly semanticConsumerMinimum: number;
  readonly semanticAllowedOwnerLevels: readonly SemanticOwnerLevel[];
  readonly semanticCollections: Readonly<Record<string, SemanticCollectionSpec>>;
  readonly roles: readonly string[];
  readonly langs: readonly string[];
  readonly ecosystems: Readonly<Record<string, Ecosystem>>;
  readonly targets: Readonly<Record<string, TargetSpec>>;
  readonly rustEntryPathRules: RustEntryPathRules;
  readonly packagesDirName: string;
  /** 🎯️ Optional per-lang render-target axis: `<owner>/📦️packages/<lang>/🎯️targets/<target>/<manifest>`. */
  readonly targetsDirName: string;
  /** 🧱️ Flat co-location dir holding one subdir per logical element. */
  readonly elementsDirName: string;
  /** 🚪️ Shape V2: the entry file lives inside `📦️packages/<lang>/`, no longer at the owner root. */
  readonly entryLocation: "packages" | "owner-root";
  /** 📁 Global semantic directory kinds legal directly below a language package. */
  readonly packagingDirectoryKindIds?: readonly string[];
  readonly artifactsDirName: string;
  readonly modesDirName: string;
  readonly windowsDirName: string;
  readonly standardsDirName: string;
  readonly subsetsDirName: string;
  readonly standardDirPrefix?: string;
  readonly standardSlugPattern?: string;
  /** 🪆️ Dir-name prefix for a subset slug under `🪆️subsets/` (mirrors `standardDirPrefix` one level down). */
  readonly subsetDirPrefix?: string;
  /** 🪆️ Legal shape for a subset id (the logical id, `*`/`subsetAnyId` excepted — that one never matches this pattern by design). */
  readonly subsetSlugPattern?: string;
  /** 🪆️ The logical id every standard's unconstrained base subset carries. */
  readonly subsetAnyId?: string;
  /** 🪆️ The on-disk dir name `subsetAnyId` maps to — canonical single source for the `"*"` ⇔ `✳️any` mapping. */
  readonly subsetAnyDirName?: string;
  /** 🪆️ Exact per-`🪆️subsets` physical identities for standards whose sibling semantics require distinct emojis. */
  readonly subsetDirectoryOverrides?: Readonly<Record<string, Readonly<Record<string, string>>>>;
  /** 🔣️ File kind of each per-standard subset vocabulary manifest. */
  readonly subsetsManifestFileKindId?: string;
  /** ✅️ COMPLETENESS set: every legacy artifact carries schema, engine, and IO. Lifecycle capabilities are schema-derived. */
  readonly artifactComponentDirs: readonly string[];
  /** 🌳️ STRUCTURAL set: every dir allowed as a child of an artifact (superset of `artifactComponentDirs`; the structural-only extra is `📚️examples`). */
  readonly artifactChildDirs: readonly string[];
  /** 🧬️ Required children of a standards-based artifact. */
  readonly newArtifactComponentDirs: readonly string[];
  readonly newArtifactChildDirs: readonly string[];
  /** 🏅️ Required and allowed children of a standard. */
  readonly standardComponentDirs: readonly string[];
  readonly standardChildDirs: readonly string[];
  /** 🪆️ Required and allowed children of a subset. */
  readonly subsetComponentDirs: readonly string[];
  readonly subsetChildDirs: readonly string[];
  /** 👁️ Surface dir name for the read-only role. */
  readonly viewerDirName: string;
  /** ✏️ Surface dir name for the mutating role. */
  readonly editorDirName: string;
  /** 👁️✏️ The closed role vocabulary a surface may carry — also the `AppRole` wire strings. */
  readonly surfaceRoles: readonly string[];
  /** 👁️✏️ Role → on-disk surface dir name; the canonical single source for the mapping. */
  readonly surfaceDirNames: Readonly<Record<string, string>>;
  /** 👁️✏️ STRUCTURAL set: surface dirs a subset may carry. */
  readonly subsetSurfaceDirs: readonly string[];
  /** 👁️✏️ COMPLETENESS set: surface dirs every OWNED subset must carry. */
  readonly subsetRequiredSurfaceDirs: readonly string[];
  /** 👁️✏️ The only dirs a CONTRIBUTED (foreign-kind) subset mirror may carry — never schema or io. */
  readonly contributedSubsetChildDirs: readonly string[];
  /** 👁️✏️ STRUCTURAL set: every directory allowed directly below a surface. */
  readonly surfaceChildDirs: readonly string[];
  /** 👁️✏️ COMPLETENESS set: surface children that must exist even when empty. */
  readonly surfaceRequiredChildDirs: readonly string[];
  /** 👁️✏️ IMPLEMENTATION set: language leaves every surface root must carry. */
  readonly surfaceComponentLangs: readonly string[];
  /** 🔣️ Normative schema file kind per surface schema facet path. */
  readonly surfaceSchemaSpecFileKinds: Readonly<Record<string, string>>;
  /** 🪆️ Allowed subset archetypes: owning owns types; derived reuses types + conformance gate. */
  readonly subsetArchetypes?: readonly string[];
  /** ⚖️ Allowed IO fidelity class names a subset may declare. */
  readonly ioFidelityClasses?: readonly string[];
  /** 🧬️ Full-match pattern for one direct `<emoji><verb>-<noun>` mutation owner directory. */
  readonly mutationDirectoryPattern: string;
  /** 🏘️ Exact domain and operation owners; semantic IDs remain independent of compact operation basenames. */
  readonly mutationDomainOwners: Readonly<Record<string, Readonly<Record<string, Readonly<Record<string, string>>>>>>;
  /** 🧭️ Exact catalog subset owners whose physical mutations belong to another registered subset of the same artifact and standard. */
  readonly mutationCatalogSourceOwners: Readonly<Record<string, string>>;
  /** 🦀️ File kind of the mandatory direct `<mutation>/🦀️.rs` owner leaf. */
  readonly mutationComponentFileKindId: string;
  /** 🪪️ Language-neutral descriptor file kind owned beside every direct mutation component. */
  readonly mutationDescriptorFileKindId: string;
  readonly mutationPayloadSchemaLocation: Readonly<{ directoryKindId: "schema"; directoryName: "🧬️schema"; fileKindId: "json" }>;
  readonly mutationPayloadSchemaAuthority: Readonly<{ contractKind: "descriptor-linked-mutation-payload-schema"; ownerAuthority: "mutationOwnerIdentity"; descriptorFileKindId: "json"; descriptorField: "payloadSchema"; descriptorSchemaVersion: 1; descriptorCardinality: "one-canonical-no-competing-descriptor"; descriptorOwnerField: "owner"; descriptorIdentityField: "semanticKind"; jsonSchemaDialect: string; targetAuthority: "owner-relative-regular-json-schema" }>;
  /** 🧬️ PLACEMENT-bound facets below a direct mutation owner: `🦠️mutation`/`🔺️diff`/`↩️inverse` behavior, required-when-present, never inlinable into the direct leaf. */
  readonly mutationBehaviorFacetDirs: readonly string[];
  /** 🔖️ Region marker whose presence in a direct mutation leaf proves that facet's behavior was inlined instead of split into its own directory. */
  readonly mutationDirectLeafForbiddenRegionMarkers: Readonly<Record<string, string>>;
  /** 🧩️ Optional organizational facets below a direct mutation owner; none is a completeness requirement. */
  readonly mutationOrganizationalFacetDirs: readonly string[];
  /** 🧬️ Required children of each `🧬️schema/` facet: snapshot, diff, mutations. */
  readonly schemaChildDirs: readonly string[];
  /** 📝️ Representation nodes under schema snapshot/diff/mutations. */
  readonly representationDirs: readonly string[];
  /** 🚪️ Top-level dirs under `🚪️io/`: import and export. */
  readonly ioDirectionDirs: readonly string[];
  /**
   * 🚪️ The NATIVE-codec facet dirs legal directly below `🚪️io/` (unsplit, both directions at once —
   * see the ⚠️ CORRECTION in design.md §1: `import`/`export` express direction, which exists only for
   * FOREIGN dialects, never for the single bidirectional native codec). Each member carries
   * `representationDirs` (`📝️text`/`💾️binary`) leaves and is itself declared in `semanticCollections` as
   * `"🚪️io/<member>"` with an io direction (`transport` for the bidirectional ones, `export` for
   * `💡️inferences` since inferences are derived-only, never imported).
   */
  readonly ioSemanticCollectionDirNames: readonly string[];
  /** 🚪️ Direction to codec folder (import→deserializers, export→serializers). */
  readonly ioDirectionChildDirs: Readonly<Record<string, string>>;
  /** 📖️ File kinds required under every text representation node. */
  readonly textSpecFileKinds: readonly string[];
  /** 📡️ File kinds required under every binary representation node. */
  readonly binarySpecFileKinds: readonly string[];
  /**
   * 🧬️ Schema serialisation formats a `🧬️schema` facet must carry, one handcrafted leaf each. `fieldCasing`
   * is the canonical casing a field name takes in that format, which the parity scanners normalise through.
   */
  readonly schemaFormats: Readonly<Record<string, { readonly fileKindId: string; readonly fieldCasing: string }>>;
  /** 🧬️ Per-facet-kind schema format subsets — normative leaf on disk selects the kind. */
  readonly schemaFacetKinds?: Readonly<Record<string, { readonly normativeFormat: string; readonly formats: readonly string[] }>>;
  /** 🔣️ Normative schema file kind per `🧬️schema` facet path. */
  readonly artifactSchemaSpecFileKinds: Readonly<Record<string, string>>;
  /** 🎛 Required children of each `🎚️config/` facet: its schema. */
  readonly configChildDirs: readonly string[];
  /** 👥️ Required children of each `👥️presence/` facet: its schema. */
  readonly presenceChildDirs: readonly string[];
  /** 🫧️ Required children of each `🫧️transient/` facet: its schema. The ephemeral local-only UI lane, fourth and last of the state mechanisms. */
  readonly transientChildDirs: readonly string[];
  readonly exampleAssetsDirName: string;
  readonly exampleTestsDirName: string;
  readonly exampleSlugPattern: string;
  readonly exampleAssetKindPrefixes: Readonly<Record<string, string>>;
  readonly exampleMediaKindPrefixes: Readonly<Record<string, string>>;
  readonly exampleFileKinds: Readonly<Record<string, string>>;
  readonly exampleTestFileKinds: Readonly<Record<string, string>>;
  readonly forbiddenExampleSlugs: readonly string[];
  readonly forbiddenExamplePluralDirs: readonly string[];
  /** 🎭️ STRUCTURAL set: every directory allowed directly below a mode. */
  readonly modeChildDirs: readonly string[];
  /** 🎭️ COMPLETENESS set: mode children that must exist even when empty. */
  readonly modeRequiredChildDirs: readonly string[];
  /** 📖️ Normative specification file kind per constitutional artifact facet. */
  readonly artifactSpecFileKinds: Readonly<Record<string, string>>;
  /** 🪟️ STRUCTURAL set: every directory allowed directly below a window. */
  readonly windowChildDirs: readonly string[];
  /** 🪟️ COMPLETENESS set: capability directories every window must carry, empty modules allowed. */
  readonly windowRequiredChildDirs: readonly string[];
  /** 🌐️ IMPLEMENTATION set: language component leaves every concrete capability member must carry. */
  readonly windowComponentLangs: readonly string[];
  /** 🪟️ IMPLEMENTATION set: language leaves the window ROOT itself must carry (facet items use `windowComponentLangs`). */
  readonly windowLeafLangs: readonly string[];
  /** 📌️ Tracked marker kind used when a required window facet has no specific items. */
  readonly windowEmptyFacetFileKindId: string;
  readonly taxonomyLeafParentDirs: readonly string[];
  /** 🍃️ Component file-kind identity, keyed by target or language. */
  readonly componentFileKinds: Readonly<Record<string, string>>;
  readonly physicalLeafRendering: Readonly<{
    direction: "forward-only";
    filename: "file-kind-emoji-and-extension-chain";
    sourceExtension: "longest-registered-chain";
    authoringExtension: "schema-ordered-primary";
    runtimeLookup: "canonical-only";
  }>;
  readonly referenceClosure: Readonly<{
    scope: "repository-incoming-and-moved-outgoing";
    candidateSource: "git-tracked-and-untracked-plus-explicit-ticket";
    candidateAdmission: "opaque-first-no-follow";
    coordinateRoots: "verified-repository-ownership";
    unsupportedPathBearingForms: "error";
    frozenSourceCoordinates: "exact-digest-and-token-authority";
    frozenPlanCoordinates: "canonical-schema-v2-digest-and-typed-token-authority";
    preimageDrift: "reject";
    newIncomingReferences: "reject-or-rollback";
    ordering: "utf8-byte";
    historicalDocumentEvidence: "ticket-report-workspace-cursor-plan-snapshot-and-dev-prompt-log-whole-document-excluded";
  }>;
  readonly frozenCoordinateEvidenceContracts: Readonly<Record<string, FrozenCoordinateEvidenceContract>>;
  readonly frozenMarkdownCoordinateEvidenceContracts: Readonly<Record<string, FrozenMarkdownCoordinateEvidenceContract>>;
  readonly historicalDocumentEvidencePopulations: Readonly<Record<string, HistoricalDocumentEvidencePopulation>>;
  readonly storyFileKindId: string;
  readonly testFeatureFileKindId: string;
  readonly testAdapterFileKinds: Readonly<Record<string, string>>;
  readonly testContributionFileKindId: string;
  readonly testContributionDirectoryOverrides: Readonly<Record<string, string>>;
  readonly testOutputMarkerFileKindId: string;
  readonly testOracleRegistryLocation: { readonly directoryPath: string; readonly fileKindId: string };
  readonly testSchemaLocation: { readonly directoryPath: string; readonly fileKindId: string };
  readonly libWiringLineBudget: number;
  readonly forbiddenPathSegments: readonly string[];
  /** 🔌️ Structural facet folders allowed directly under each plugin root. */
  readonly pluginChildDirs: readonly string[];
  readonly pluginRequiredChildDirs: readonly string[];
  /** 💻️ Structural facet folders owned directly by the OS product. */
  readonly osChildDirs: readonly string[];
  readonly osRequiredChildDirs: readonly string[];
  /** 🚫️ Emoji-stripped directory/file stems banned repo-wide (e.g. `core`, `shared`). */
  readonly bannedNameStems: readonly string[];
  readonly rootDataDirNames: readonly string[];
  readonly rootDataContractIds: readonly string[];
  readonly rootDocumentContractIds: readonly string[];
  readonly repoWideContractIds: readonly string[];
  readonly layeringGeneratedContractIds: readonly string[];
  readonly layeringGeneratedBanners: readonly string[];
  readonly areaLayers: Readonly<Record<string, "framework" | "implementation" | "repo-wide">>;
  readonly packageMaturityStates: readonly PackageMaturity[];
  /** 🧭️ How migration is detected — structurally, never from a hand-maintained package list. */
  readonly migratedMarker: "packages-dir-exists";
  /** 🔌️ Area roots whose package owners contribute plugins. */
  readonly pluginAreas: readonly string[];
  readonly areas: Readonly<Record<string, AreaState>>;
}

const cachedTaxonomy = ephemeralBox<Taxonomy | undefined>("framework.products.repo.modules.lib.discovery.component.ts.cachedTaxonomy", undefined);
const cachedCatalogTaxonomy = ephemeralBox<Taxonomy | undefined>("framework.products.repo.modules.lib.discovery.component.ts.cachedCatalogTaxonomy", undefined);

//#region 🧭️WorkspaceTaxonomyAuthority
/** 🧭️ No-follow workspace authority for the taxonomy named by root project metadata. */
export interface WorkspaceTaxonomyAuthority {
  readonly workspaceRoot: string;
  readonly manifestPath: string;
  readonly relativePath: string;
  readonly taxonomyPath: string;
}

function noFollowStat(path: string, expectation: "directory" | "file", subject: string): void {
  let stat: ReturnType<typeof lstatSync>;
  try {
    stat = lstatSync(path);
  } catch {
    throw new Error(`Workspace taxonomy ${subject} is missing at ${JSON.stringify(path)}.`);
  }
  if (stat.isSymbolicLink()) throw new Error(`Workspace taxonomy ${subject} must not be a symlink at ${JSON.stringify(path)}.`);
  if (expectation === "directory" ? !stat.isDirectory() : !stat.isFile()) throw new Error(`Workspace taxonomy ${subject} must be a ${expectation} at ${JSON.stringify(path)}.`);
}

function noFollowNodeKind(path: string): "missing" | "file" | "directory" | "symlink" | "other" {
  try {
    const stat = lstatSync(path);
    if (stat.isSymbolicLink()) return "symlink";
    if (stat.isFile()) return "file";
    if (stat.isDirectory()) return "directory";
    return "other";
  } catch {
    return "missing";
  }
}

/** 🧭️ Lexically verifies one workspace authority path before resolution. */
export function workspaceAuthorityPath(path: string, subject: string): string {
  const segments = path.split(/[\\/]/u);
  if (path.includes("\0")) throw new Error(`Workspace taxonomy ${subject} contains a NUL path character at ${JSON.stringify(path)}.`);
  if (/^[A-Za-z]:(?:$|[^\\/])/u.test(path)) throw new Error(`Workspace taxonomy ${subject} contains a drive-relative path at ${JSON.stringify(path)}.`);
  if (segments.some((segment) => segment.toLowerCase() === "compose")) throw new Error(`Workspace taxonomy ${subject} enters excluded compose content at ${JSON.stringify(path)}.`);
  if (segments.includes(".") || segments.includes("..")) throw new Error(`Workspace taxonomy ${subject} contains a dot or parent path segment at ${JSON.stringify(path)}.`);
  return resolve(path);
}

/** 🛡️ Requires each directory from the filesystem root to be real and no-follow. */
export function noFollowDirectoryAncestry(path: string, subject: string): void {
  const ancestry: string[] = [];
  for (let candidate = path; ; candidate = dirname(candidate)) {
    ancestry.push(candidate);
    if (dirname(candidate) === candidate) break;
  }
  for (const candidate of ancestry.reverse()) noFollowStat(candidate, "directory", subject);
}

function workspaceTaxonomyLocator(manifest: unknown): string {
  const metadata = typeof manifest === "object" && manifest !== null && !Array.isArray(manifest) ? (manifest as { metadata?: unknown }).metadata : undefined;
  const semio = typeof metadata === "object" && metadata !== null && !Array.isArray(metadata) ? (metadata as { semio?: unknown }).semio : undefined;
  const locator = typeof semio === "object" && semio !== null && !Array.isArray(semio) ? (semio as { taxonomy?: unknown }).taxonomy : undefined;
  if (typeof locator !== "string") throw new Error("Workspace taxonomy locator metadata.semio.taxonomy must be a string.");
  if (locator.length === 0 || locator.includes("\0") || locator.includes("\\") || locator.startsWith("/") || /^[A-Za-z]:/u.test(locator)) throw new Error(`Workspace taxonomy locator ${JSON.stringify(locator)} must be a safe repository-relative slash path.`);
  const segments = locator.split("/");
  if (segments.some((segment) => segment.length === 0 || segment === "." || segment === "..")) throw new Error(`Workspace taxonomy locator ${JSON.stringify(locator)} contains an unsafe path segment.`);
  if (segments.some((segment) => segment.toLowerCase() === "compose")) throw new Error(`Workspace taxonomy locator ${JSON.stringify(locator)} enters excluded compose content.`);
  return locator;
}

/** 🧭️ Resolves root metadata.semio.taxonomy only after lexical and no-follow checks. */
export function resolveWorkspaceTaxonomyAuthority(workspaceRoot: string): WorkspaceTaxonomyAuthority {
  const root = workspaceAuthorityPath(workspaceRoot, "root");
  noFollowDirectoryAncestry(root, "workspace root ancestry");
  noFollowStat(join(root, "nx.json"), "file", "nx manifest");
  const manifestPath = join(root, "📋️project.json");
  noFollowStat(manifestPath, "file", "project manifest");
  let manifest: unknown;
  try {
    manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
  } catch (error) {
    throw new Error(`Workspace taxonomy project manifest is not valid JSON: ${error instanceof Error ? error.message : String(error)}`);
  }
  const relativePath = workspaceTaxonomyLocator(manifest);
  const segments = relativePath.split("/");
  let current = root;
  for (const [index, segment] of segments.entries()) {
    current = join(current, segment);
    noFollowStat(current, index === segments.length - 1 ? "file" : "directory", index === segments.length - 1 ? "taxonomy file" : "taxonomy path component");
  }
  return { workspaceRoot: root, manifestPath, relativePath, taxonomyPath: current };
}

function readTaxonomyUnchecked(): Taxonomy {
  const authority = resolveWorkspaceTaxonomyAuthorityFromDirectory(__dirname);
  return JSON.parse(readFileSync(authority.taxonomyPath, "utf8")) as Taxonomy;
}

function taxonomyWorkspaceRoot(): string | null {
  return taxonomyWorkspaceRootFromDirectory(__dirname);
}

function taxonomyWorkspaceRootFromDirectory(startDirectory: string): string | null {
  let start: string;
  try {
    start = workspaceAuthorityPath(startDirectory, "start directory");
    noFollowDirectoryAncestry(start, "workspace start directory ancestry");
  } catch {
    return null;
  }
  for (let candidate = start; dirname(candidate) !== candidate; candidate = dirname(candidate)) {
    const nxPath = join(candidate, "nx.json");
    const nxKind = noFollowNodeKind(nxPath);
    if (nxKind === "missing") continue;
    if (nxKind === "symlink") throw new Error(`Workspace taxonomy nx manifest must not be a symlink at ${JSON.stringify(nxPath)}.`);
    if (nxKind !== "file") throw new Error(`Workspace taxonomy nx manifest must be a file at ${JSON.stringify(nxPath)}.`);
    const projectPath = join(candidate, "📋️project.json");
    const projectKind = noFollowNodeKind(projectPath);
    if (projectKind === "missing") throw new Error(`Workspace taxonomy project manifest is missing at ${JSON.stringify(projectPath)}.`);
    if (projectKind === "symlink") throw new Error(`Workspace taxonomy project manifest must not be a symlink at ${JSON.stringify(projectPath)}.`);
    if (projectKind !== "file") throw new Error(`Workspace taxonomy project manifest must be a file at ${JSON.stringify(projectPath)}.`);
    return candidate;
  }
  return null;
}

/** 🧭️ Finds the marker-paired workspace from a no-follow, non-compose discovery directory. */
export function resolveWorkspaceTaxonomyAuthorityFromDirectory(startDirectory: string): WorkspaceTaxonomyAuthority {
  const start = workspaceAuthorityPath(startDirectory, "start directory");
  noFollowDirectoryAncestry(start, "workspace start directory ancestry");
  const workspaceRoot = taxonomyWorkspaceRootFromDirectory(start);
  if (!workspaceRoot) throw new Error("Workspace taxonomy root containing exact nx.json and 📋️project.json markers could not be resolved.");
  return resolveWorkspaceTaxonomyAuthority(workspaceRoot);
}
//#endregion 🧭️WorkspaceTaxonomyAuthority

/** 📇️ Validates catalog vocabulary without reading unrelated generator outputs or workspace diagnostics. */
export function loadCatalogTaxonomy(): Taxonomy {
  if (cachedCatalogTaxonomy.current) return cachedCatalogTaxonomy.current;
  const taxonomy = readTaxonomyUnchecked();
  const problems = validateTaxonomy(taxonomy);
  if (problems.length > 0) throw new Error(`Invalid taxonomy schema:\n${problems.map((problem) => `- ${problem}`).join("\n")}`);
  cachedCatalogTaxonomy.current = taxonomy;
  return taxonomy;
}

/** 📖️ Reads, strictly validates, and caches the incompatible version-7 taxonomy and workspace contracts. */
export function loadTaxonomy(): Taxonomy {
  if (cachedTaxonomy.current) return cachedTaxonomy.current;
  const taxonomy = loadCatalogTaxonomy();
  const workspaceRoot = taxonomyWorkspaceRoot();
  const problems = workspaceRoot ? validateGeneratorContractsAgainstWorkspace(workspaceRoot, taxonomy) : ["generatorContracts workspace root could not be resolved."];
  if (problems.length > 0) throw new Error(`Invalid taxonomy schema:\n${problems.map((problem) => `- ${problem}`).join("\n")}`);
  cachedTaxonomy.current = taxonomy;
  return cachedTaxonomy.current;
}

/** 🪆️ Resolves one logical subset id to its exact owner-scoped physical directory identity. */
export function subsetDirectoryNameForId(subsetsOwnerPath: string, subsetId: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const owner = subsetsOwnerPath.replaceAll("\\", "/").replace(/\/$/u, "").normalize("NFC");
  const overrides = taxonomy.subsetDirectoryOverrides?.[owner];
  if (overrides) return overrides[subsetId] ?? null;
  const anyId = taxonomy.subsetAnyId ?? "*";
  if (subsetId === anyId) return taxonomy.subsetAnyDirName ?? "✳️any";
  const slug = new RegExp(taxonomy.subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$", "u");
  return slug.test(subsetId) ? `${taxonomy.subsetDirPrefix ?? "✳️"}${subsetId}` : null;
}

/** 🪆️ Resolves one physical subset directory identity back to its logical owner-scoped id. */
export function subsetIdForDirectoryName(subsetsOwnerPath: string, directoryName: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const owner = subsetsOwnerPath.replaceAll("\\", "/").replace(/\/$/u, "").normalize("NFC");
  const overrides = taxonomy.subsetDirectoryOverrides?.[owner];
  if (overrides) return Object.entries(overrides).find(([, name]) => name === directoryName)?.[0] ?? null;
  const anyId = taxonomy.subsetAnyId ?? "*";
  if (directoryName === (taxonomy.subsetAnyDirName ?? "✳️any")) return anyId;
  const prefix = taxonomy.subsetDirPrefix ?? "✳️";
  if (!directoryName.startsWith(prefix)) return null;
  const id = directoryName.slice(prefix.length);
  const slug = new RegExp(taxonomy.subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$", "u");
  return slug.test(id) ? id : null;
}

type SchemaFormatSpec = { readonly fileKindId: string; readonly fieldCasing: string };

/** 📄️ Returns every canonical kind-only filename for one file kind. */
export function canonicalFilenamesForKind(kindId: string, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const kind = taxonomy.fileKinds[kindId];
  return kind ? kind.extensionChains.map((extension) => `${kind.emoji}${extension}`) : [];
}

/** 📄️ Returns the single canonical filename for a one-extension file kind. */
export function canonicalFilenameForKind(kindId: string, taxonomy: Taxonomy = loadTaxonomy()): string {
  const names = canonicalFilenamesForKind(kindId, taxonomy);
  if (names.length !== 1) throw new Error(`File kind ${JSON.stringify(kindId)} must have exactly one extension chain, got ${names.length}.`);
  return names[0]!;
}

/** 🍃️ Renders the schema-ordered primary physical leaf for new authored content. */
export function canonicalPrimaryFilenameForKind(kindId: string, taxonomy: Taxonomy = loadTaxonomy()): string {
  const filename = canonicalFilenamesForKind(kindId, taxonomy)[0];
  if (!filename) throw new Error(`File kind ${JSON.stringify(kindId)} must declare a primary extension chain.`);
  return filename;
}

/** 📇️ Resolves a semantic collection's manifest by exact owner, never by ancestor or suffix fallback. */
export function semanticManifestFilenameForCollection(collectionPath: string, taxonomy: Taxonomy = loadTaxonomy()): string {
  const owner = collectionPath.replaceAll("\\", "/").replace(/\/$/u, "").normalize("NFC");
  return taxonomy.semanticManifestFilenameOverrides?.[owner] ?? canonicalPrimaryFilenameForKind(taxonomy.semanticManifestFileKindId, taxonomy);
}

/** 🍃️ Renders a source's exact longest-chain format; this does not authorize its owner or move. */
export function canonicalLeafFilenameForSourcePath(path: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const kindId = fileKindIdForSourcePath(path, taxonomy);
  if (!kindId) return null;
  const kind = taxonomy.fileKinds[kindId]!;
  const filename = path.replaceAll("\\", "/").split("/").pop()!.toLowerCase();
  const extension = [...kind.extensionChains].sort((left, right) => right.length - left.length).find((chain) => filename.endsWith(chain));
  return extension ? `${kind.emoji}${extension}` : null;
}

/** 🧬️ Renders the default schema location for a newly authored mutation, not existing descriptor authority. */
export function mutationPayloadSchemaRelativePath(taxonomy: Taxonomy = loadTaxonomy()): string {
  const location = taxonomy.mutationPayloadSchemaLocation;
  return `${location.directoryName}/${canonicalPrimaryFilenameForKind(location.fileKindId, taxonomy)}`;
}

/** 📐️ Validates Draft-07 keyword shapes without imposing names on a schema's physical file. */
export function mutationPayloadSchemaDocumentProblems(value: unknown, dialect = "http://json-schema.org/draft-07/schema#"): string[] {
  const problems: string[] = [];
  const object = (item: unknown): item is Record<string, unknown> => item !== null && typeof item === "object" && !Array.isArray(item);
  const canonical = (item: unknown): string => Array.isArray(item) ? `[${item.map(canonical).join(",")}]` : object(item) ? `{${Object.keys(item).sort().map((key) => `${JSON.stringify(key)}:${canonical(item[key])}`).join(",")}}` : JSON.stringify(item) ?? String(item);
  const unique = (items: readonly unknown[]): boolean => new Set(items.map(canonical)).size === items.length;
  const strings = (item: unknown): boolean => Array.isArray(item) && item.every((part) => typeof part === "string") && unique(item);
  const regex = (pattern: string): boolean => { try { new RegExp(pattern, "u"); return true; } catch { return false; } };
  const visit = (item: unknown, path: string): void => {
    if (typeof item === "boolean") return;
    if (!object(item)) { problems.push(`${path} must be an object or boolean schema`); return; }
    for (const [key, child] of Object.entries(item)) {
      const at = `${path}/${key}`;
      if (["$id", "$schema", "$ref", "$comment", "title", "description", "format", "contentMediaType", "contentEncoding", "pattern"].includes(key) && typeof child !== "string") problems.push(`${at} must be a string`);
      if (["readOnly", "uniqueItems"].includes(key) && typeof child !== "boolean") problems.push(`${at} must be a boolean`);
      if (["maximum", "exclusiveMaximum", "minimum", "exclusiveMinimum", "multipleOf"].includes(key) && (typeof child !== "number" || !Number.isFinite(child) || key === "multipleOf" && child <= 0)) problems.push(`${at} must be a valid numeric bound`);
      if (["maxLength", "minLength", "maxItems", "minItems", "maxProperties", "minProperties"].includes(key) && (typeof child !== "number" || !Number.isInteger(child) || child < 0)) problems.push(`${at} must be a nonnegative integer`);
      if (key === "pattern" && typeof child === "string" && !regex(child)) problems.push(`${at} must be a regular expression`);
      if (key === "required" && !strings(child)) problems.push(`${at} must contain unique strings`);
      if (key === "examples" && !Array.isArray(child)) problems.push(`${at} must be an array`);
      if (key === "enum" && (!Array.isArray(child) || child.length === 0 || !unique(child))) problems.push(`${at} must contain unique values`);
      if (key === "type") {
        const types = Array.isArray(child) ? child : [child];
        if (types.length === 0 || !unique(types) || types.some((type) => typeof type !== "string" || !["array", "boolean", "integer", "null", "number", "object", "string"].includes(type))) problems.push(`${at} must name unique JSON types`);
      }
      if (["additionalItems", "contains", "additionalProperties", "propertyNames", "if", "then", "else", "not"].includes(key)) visit(child, at);
      if (["allOf", "anyOf", "oneOf"].includes(key) || key === "items" && Array.isArray(child)) {
        if (!Array.isArray(child) || child.length === 0) problems.push(`${at} must contain schemas`);
        else child.forEach((schema, index) => visit(schema, `${at}/${index}`));
      } else if (key === "items") visit(child, at);
      if (["definitions", "properties", "patternProperties", "dependencies"].includes(key)) {
        if (!object(child)) problems.push(`${at} must be an object`);
        else for (const [name, schema] of Object.entries(child)) {
          if (key === "patternProperties" && !regex(name)) problems.push(`${at}/${name} must be a regular expression`);
          if (key === "dependencies" && Array.isArray(schema)) { if (!strings(schema)) problems.push(`${at}/${name} must contain unique strings`); }
          else visit(schema, `${at}/${name}`);
        }
      }
    }
  };
  if (!object(value) || value.$schema !== dialect) problems.push("Payload must declare its registered JSON Schema dialect");
  visit(value, "");
  return problems;
}

/** 🛡️ Validates descriptor authority against admitted, non-linked nodes within one exact operation owner. */
export function mutationPayloadSchemaProblems(owner: string, pointer: unknown, node: (path: string) => Readonly<{ kind: string; content?: string; repositoryBoundary?: boolean }>, dialect = "http://json-schema.org/draft-07/schema#"): string[] {
  const local = (path: string): boolean => path.length > 0 && path === path.normalize("NFC") && !/^[\/]|[\\:#\u0000-\u001F\u007F\u2028\u2029]/u.test(path) && path.split("/").every((part) => part !== "" && part !== "." && part !== "..");
  if (!local(owner) || typeof pointer !== "string" || !local(pointer) || !pointer.endsWith(".json")) return ["Payload schema must be an exact owner-relative JSON path without traversal, fragments, or absolute coordinates"];
  const target = `${owner}/${pointer}`, parts = target.split("/");
  for (let index = 1; index < parts.length; index++) {
    const path = parts.slice(0, index).join("/"), observed = node(path);
    if (observed.kind !== "directory" || observed.repositoryBoundary) return [`Payload schema ancestor ${path} is not an admitted regular owner directory`];
  }
  const observed = node(target);
  if (observed.kind !== "file" || observed.repositoryBoundary || typeof observed.content !== "string") return [`Payload schema ${target} is not an admitted regular file`];
  try { const document = JSON.parse(observed.content); return [...jsonDocumentDuplicateKeys(observed.content), ...mutationPayloadSchemaDocumentProblems(document, dialect)]; }
  catch { return [`Payload schema ${target} is not valid JSON`]; }
}

/** 🗝️ Rejects duplicate decoded members in already parsed JSON authority documents. */
export function jsonDocumentDuplicateKeys(source: string): string[] {
  const stack: (Set<string> | null)[] = [], problems: string[] = [];
  for (const token of source.matchAll(/"(?:\\.|[^"\\])*"\s*:?|[{}\[\]]/gu)) {
    const value = token[0];
    if (value === "{") stack.push(new Set());
    else if (value === "[") stack.push(null);
    else if (value === "}" || value === "]") stack.pop();
    else if (value.endsWith(":")) {
      const key = JSON.parse(value.slice(0, -1).trim()) as string, keys = stack.at(-1);
      if (keys?.has(key)) problems.push(`Duplicate JSON member ${JSON.stringify(key)} at offset ${token.index}`);
      keys?.add(key);
    }
  }
  return problems;
}

/** 🧷️ Returns a semantic leaf filename using the file kind's schema-ordered primary extension. */
export function canonicalStemmedFilenameForKind(kindId: string, stem: string, taxonomy: Taxonomy = loadTaxonomy()): string {
  const kind = taxonomy.fileKinds[kindId];
  const extension = kind?.extensionChains[0];
  if (!kind || !extension) throw new Error(`File kind ${JSON.stringify(kindId)} must declare a primary extension chain.`);
  if (!stem || /[\\/]/u.test(stem)) throw new Error(`Filename stem ${JSON.stringify(stem)} must be one non-empty path segment.`);
  return `${kind.emoji}${stem}${extension}`;
}

/** 🧭️ Resolves an exact canonical kind-only filename to its registered file-kind identifier. */
export function fileKindIdForFilename(filename: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const normalized = filename.normalize(taxonomy.unicodeNormalization.form);
  const matches = Object.entries(taxonomy.fileKinds).filter(([kindId]) => canonicalFilenamesForKind(kindId, taxonomy).includes(normalized));
  return matches.length === 1 ? matches[0]![0] : null;
}

/** 🧩️ Resolves a source path to one physical file kind through the longest registered extension chain. */
export function fileKindIdForSourcePath(path: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  const filename = normalized.slice(normalized.lastIndexOf("/") + 1).toLowerCase();
  const terminalCandidates = Object.entries(taxonomy.fileKinds).flatMap(([kindId, kind]) => kind.extensionChains.filter((extension) => filename.endsWith(extension)).map((extension) => ({ kindId, extension })));
  const longest = Math.max(0, ...terminalCandidates.map((candidate) => candidate.extension.length));
  const longestKindIds = [...new Set(terminalCandidates.filter((candidate) => candidate.extension.length === longest).map((candidate) => candidate.kindId))];
  return longestKindIds.length === 1 ? longestKindIds[0]! : null;
}

/** 🎟️ Resolves an owner-scoped evidence suffix without admitting it as a global file kind. */
export function scopedFileKindIdForSourcePath(path: string, taxonomy: Taxonomy = loadTaxonomy(), context: Readonly<{ parentDirectoryKindId?: string }> = {}): string | null {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  const filename = normalized.slice(normalized.lastIndexOf("/") + 1);
  const matches = Object.entries(taxonomy.scopedFileKinds).filter(([, spec]) => taxonomyPathPatternMatches(normalized, spec.pathPattern)
    && (spec.parentDirectoryKindId === undefined || spec.parentDirectoryKindId === context.parentDirectoryKindId)
    && new RegExp(spec.sourceFilenamePattern, "u").test(filename)
    && spec.extensionChains.some((extension) => filename.endsWith(extension)));
  return matches.length === 1 ? matches[0]![0] : null;
}

export interface TaxonomyCliWritePreparationFacts {
  readonly directoryName: string;
  readonly leafNames: readonly string[];
}

export interface TaxonomyCliAttemptPreparationChildFacts {
  readonly name: string;
  readonly nodeKind: "directory" | "file";
}

export interface TaxonomyCliAttemptPreparationFacts {
  readonly parentKindId: string;
  readonly directoryName: string;
  readonly children: readonly TaxonomyCliAttemptPreparationChildFacts[];
}

/** 🧪️ Validates every unpublished attempt preparation before any recovery mutation. */
export function taxonomyCliAttemptPreparationsProblems(facts: readonly Readonly<TaxonomyCliAttemptPreparationFacts>[], taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const names = new Set<string>();
  const ordinals = new Set<string>();
  for (const preparation of [...facts].sort((left, right) => projectionByteCompare(left.directoryName, right.directoryName))) {
    const kindId = semanticDirectoryKindId(preparation.directoryName, taxonomy, { parentKindId: preparation.parentKindId });
    if (kindId !== "transaction-attempt-preparation") problems.push(`Attempt preparation ${JSON.stringify(preparation.directoryName)} must resolve below transaction-attempts.`);
    if (names.has(preparation.directoryName)) problems.push(`Attempt preparation ${JSON.stringify(preparation.directoryName)} is duplicated.`);
    names.add(preparation.directoryName);
    const ordinal = preparation.directoryName.match(/^🚧️prepare-([0-9]{6})-/u)?.[1];
    if (!ordinal) problems.push(`Attempt preparation ${JSON.stringify(preparation.directoryName)} must carry one six-digit ordinal.`);
    else if (ordinals.has(ordinal)) problems.push(`Attempt preparation ordinal ${ordinal} is duplicated.`);
    else ordinals.add(ordinal);
    const childNames = new Set<string>();
    for (const child of [...preparation.children].sort((left, right) => projectionByteCompare(`${left.name}\0${left.nodeKind}`, `${right.name}\0${right.nodeKind}`))) {
      if (childNames.has(child.name)) problems.push(`Attempt preparation child ${JSON.stringify(child.name)} is duplicated.`);
      childNames.add(child.name);
      const expectedKind = child.name === "🚧️stage" ? "transaction-stage" : child.name === "💾️backup" ? "transaction-backup" : child.name === "🔒️lease" ? "transaction-lease" : null;
      if (expectedKind) {
        if (child.nodeKind !== "directory" || semanticDirectoryKindId(child.name, taxonomy, { parentKindId: kindId ?? undefined }) !== expectedKind) problems.push(`Attempt preparation child ${JSON.stringify(child.name)} must be its exact no-follow directory kind.`);
      } else if (child.name === canonicalFilenameForKind("json", taxonomy)) {
        if (child.nodeKind !== "file") problems.push("Attempt preparation journal must be an exact regular file.");
      } else problems.push(`Attempt preparation child ${JSON.stringify(child.name)} is not admitted.`);
    }
  }
  return problems;
}

function taxonomyCliWritePreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[] }>, directoryKindId: "transaction-edit-write-preparation" | "transaction-backup-write-preparation", leafName: "🚧️.edit" | "🚧️.backup", scopedKindId: "transaction-edit-write-candidate" | "transaction-backup-write-candidate", pathPrefix: string, taxonomy: Taxonomy): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== directoryKindId) problems.push(`${directoryKindId} must resolve below its exact preparation owner.`);
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push(`${directoryKindId} leaves must be unique.`);
  for (const leaf of facts.leafNames) {
    if (leaf !== leafName) problems.push(`${directoryKindId} leaf ${JSON.stringify(leaf)} is not the unpublished kind-only candidate.`);
    else if (scopedFileKindIdForSourcePath(`${pathPrefix}/${facts.directoryName}/${leaf}`, taxonomy, { parentDirectoryKindId: kindId ?? undefined }) !== scopedKindId) problems.push(`${directoryKindId} leaf ${JSON.stringify(leaf)} has no scoped unpublished authority.`);
  }
  return problems;
}

/** 📝️ Validates one unpublished rendered-edit writer state. */
export function taxonomyCliEditWritePreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  return taxonomyCliWritePreparationProblems(facts, "transaction-edit-write-preparation", "🚧️.edit", "transaction-edit-write-candidate", "🚧️stage/🚧️edit-owner", taxonomy);
}

/** 💾️ Validates one unpublished backup writer state. */
export function taxonomyCliBackupWritePreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  return taxonomyCliWritePreparationProblems(facts, "transaction-backup-write-preparation", "🚧️.backup", "transaction-backup-write-candidate", "💾️backup/🚧️backup-owner", taxonomy);
}

/** 🚧️ Validates one transaction edit-preparation directory's exact Windows-safe exchange state. */
export function taxonomyCliEditPreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[]; writePreparations: readonly Readonly<TaxonomyCliWritePreparationFacts>[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-edit-preparation") problems.push("Edit preparation directory must resolve below transaction-stage.");
  const operationHash = facts.directoryName.match(/^🚧️edit-([0-9a-f]{24})-/u)?.[1];
  if (!operationHash) problems.push("Edit preparation directory must carry one lowercase 24-hex operation hash.");
  const allowed = operationHash ? new Map([[`${operationHash}.edit`, "transaction-edit-candidate"], [`${operationHash}.pre`, "transaction-edit-preimage"]]) : new Map<string, string>();
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push("Edit preparation leaves must be unique.");
  for (const leaf of facts.leafNames) {
    const expectedKindId = allowed.get(leaf);
    if (!expectedKindId) problems.push(`Edit preparation leaf ${JSON.stringify(leaf)} is not hash-bound to its directory.`);
    else if (scopedFileKindIdForSourcePath(`🚧️stage/${facts.directoryName}/${leaf}`, taxonomy, { parentDirectoryKindId: kindId ?? undefined }) !== expectedKindId) problems.push(`Edit preparation leaf ${JSON.stringify(leaf)} has no scoped exchange authority.`);
  }
  if (facts.writePreparations.length > 1) problems.push("Edit preparation may contain at most one unpublished writer.");
  for (const writer of facts.writePreparations) problems.push(...taxonomyCliEditWritePreparationProblems({ parentKindId: kindId ?? "", directoryName: writer.directoryName, leafNames: writer.leafNames }, taxonomy));
  return problems;
}

/** 📦️ Validates one backup preparation's authoritative candidate and unpublished writer union. */
export function taxonomyCliBackupPreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[]; writePreparations: readonly Readonly<TaxonomyCliWritePreparationFacts>[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-backup-preparation") problems.push("Backup preparation directory must resolve below transaction-backup.");
  const operationHash = facts.directoryName.match(/^🚧️backup-([0-9a-f]{24})-/u)?.[1];
  if (!operationHash) problems.push("Backup preparation directory must carry one lowercase 24-hex operation hash.");
  const allowedLeaf = operationHash ? `${operationHash}.backup` : "";
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push("Backup preparation leaves must be unique.");
  for (const leaf of facts.leafNames) if (leaf !== allowedLeaf) problems.push(`Backup preparation leaf ${JSON.stringify(leaf)} is not hash-bound to its directory.`);
  if (facts.writePreparations.length > 1) problems.push("Backup preparation may contain at most one unpublished writer.");
  for (const writer of facts.writePreparations) problems.push(...taxonomyCliBackupWritePreparationProblems({ parentKindId: kindId ?? "", directoryName: writer.directoryName, leafNames: writer.leafNames }, taxonomy));
  return problems;
}

/** ♻️ Validates one restore preparation's exact empty, preimage, exchange, or postimage state. */
export function taxonomyCliRestorePreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-restore-preparation") problems.push("Restore preparation directory must resolve below transaction-backup.");
  const operationHash = facts.directoryName.match(/^🚧️restore-([0-9a-f]{24})-/u)?.[1];
  if (!operationHash) problems.push("Restore preparation directory must carry one lowercase 24-hex operation hash.");
  const allowed = operationHash ? new Set([`${operationHash}.backup`, `${operationHash}.post`]) : new Set<string>();
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push("Restore preparation leaves must be unique.");
  for (const leaf of facts.leafNames) {
    if (!allowed.has(leaf)) problems.push(`Restore preparation leaf ${JSON.stringify(leaf)} is not hash-bound to its directory.`);
    const scopedId = scopedFileKindIdForSourcePath(`💾️backup/${facts.directoryName}/${leaf}`, taxonomy, { parentDirectoryKindId: kindId ?? undefined });
    if (scopedId !== "transaction-backup-candidate" && scopedId !== "transaction-postimage-candidate") problems.push(`Restore preparation leaf ${JSON.stringify(leaf)} has no scoped candidate authority.`);
  }
  return problems;
}

/** 📝️ Validates a Windows-safe JSON-write exchange beneath the exact journal or lease-preparation owner. */
export function taxonomyCliJsonWritePreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-json-write-preparation") problems.push("JSON write preparation must resolve below transaction-journal-write, transaction-lease-preparation, or transaction-lease.");
  const allowed = new Set(["🔣️.json", "⏮️.json"]);
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push("JSON write preparation leaves must be unique.");
  for (const leaf of facts.leafNames) {
    if (!allowed.has(leaf)) problems.push(`JSON write preparation leaf ${JSON.stringify(leaf)} is not admitted.`);
    if (leaf === "⏮️.json" && scopedFileKindIdForSourcePath(`🚧️journal/${facts.directoryName}/${leaf}`, taxonomy, { parentDirectoryKindId: kindId ?? undefined }) !== "transaction-json-previous") problems.push("JSON write displaced previous leaf has no scoped exchange authority.");
  }
  return problems;
}

/** 🔒️ Validates canonical, preparing, and stale-quarantined lease publication states. */
export function taxonomyCliLeaseDirectoryProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[]; writePreparations: readonly Readonly<TaxonomyCliWritePreparationFacts>[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-lease" && kindId !== "transaction-lease-preparation") problems.push("Lease directory must resolve as a canonical lease or token-bound preparation below its exact parent.");
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push("Lease leaves must be unique.");
  for (const leaf of facts.leafNames) if (leaf !== canonicalFilenameForKind("json", taxonomy)) problems.push(`Lease leaf ${JSON.stringify(leaf)} is not the canonical JSON record.`);
  if (facts.writePreparations.length > 1) problems.push("Lease directory may contain at most one JSON-write preparation.");
  for (const writer of facts.writePreparations) problems.push(...taxonomyCliJsonWritePreparationProblems({ parentKindId: kindId ?? "", directoryName: writer.directoryName, leafNames: writer.leafNames }, taxonomy));
  const publishedState = kindId === "transaction-lease" || facts.directoryName.endsWith("-stale");
  const hasCanonical = facts.leafNames.includes(canonicalFilenameForKind("json", taxonomy));
  const exchangeWithoutCanonical = facts.writePreparations.length === 1 && [...facts.writePreparations[0]!.leafNames].sort(projectionByteCompare).join("\0") === ["⏮️.json", "🔣️.json"].sort(projectionByteCompare).join("\0");
  if (publishedState && !hasCanonical && !exchangeWithoutCanonical) problems.push("Published or stale lease must retain canonical JSON or the exact displaced-previous exchange state.");
  return problems;
}

/** 🎯 Resolves an output path to its one exact or ancestor-root generator classification. */
export function generatorContractIdsForOutputPath(path: string, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  return Object.entries(taxonomy.generatorContracts).filter(([, contract]) => contract.outputRoots.some((root) => normalized === root.path || normalized.startsWith(`${root.path}/`))).map(([id]) => id);
}

/** ⚙️ Derives the only runnable generator command shape admitted by v7. */
export function generatorNxCommand(contract: GeneratorContract): readonly ["bun", "nx", "run", string] {
  if (contract.ownership !== "owned" || !contract.target) throw new Error("Only owned generator contracts are runnable.");
  return ["bun", "nx", "run", contract.target];
}

/** 👁️ Derives an owned generator's mandatory read-only preview command. */
export function generatorNxPreviewCommand(contract: GeneratorContract): readonly ["bun", "nx", "run", string] {
  if (contract.ownership !== "owned" || !contract.previewTarget) throw new Error("Only owned generator contracts have preview targets.");
  return ["bun", "nx", "run", contract.previewTarget];
}

/** 👓️ Admits an exact read-only script invocation without sharing another generator's preview. */
export function generatorPreviewScriptArguments(contract: Pick<GeneratorContract, "ownership" | "target" | "previewTarget" | "previewArguments">): readonly string[] {
  if (contract.ownership !== "owned" || !contract.target || !contract.previewTarget) throw new Error("Only owned generator contracts have preview invocations");
  const project = contract.target.slice(0, contract.target.lastIndexOf(":"));
  if (contract.previewArguments === undefined) {
    if (contract.previewTarget !== `${project}:preview-generated`) throw new Error("previewTarget must be the exact owner preview-generated target or an explicit same-project preview-* invocation");
    return ["preview-generated"];
  }
  const args = contract.previewArguments, separator = contract.previewTarget.lastIndexOf(":");
  if (contract.previewTarget.slice(0, separator) !== project || !/^preview-[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(contract.previewTarget.slice(separator + 1)) || !Array.isArray(args) || args.length < 1 || args.length > 8 || args.some(arg => typeof arg !== "string" || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(arg)) || args.at(-1) !== "preview") throw new Error("Invalid explicit same-project preview invocation");
  return args;
}

/** ⏱️ Bounds a declared compiler preview without changing established generator defaults. */
export function generatorPreviewResourceLimits(contract: Pick<GeneratorContract, "ownership" | "previewTarget" | "previewLimits">): { readonly maxOutputBytes: number; readonly timeoutMs: number } {
  const limits = contract.previewLimits;
  if (limits === undefined) return { maxOutputBytes: 134217728, timeoutMs: 60000 };
  if (contract.ownership !== "owned" || !contract.previewTarget || !limits || typeof limits !== "object" || Array.isArray(limits) || Object.keys(limits).sort().join("|") !== "maxOutputBytes|timeoutMs" || !Number.isSafeInteger(limits.maxOutputBytes) || limits.maxOutputBytes < 1024 || limits.maxOutputBytes > 536870912 || !Number.isSafeInteger(limits.timeoutMs) || limits.timeoutMs < 1000 || limits.timeoutMs > 600000) throw new Error("Invalid bounded generator preview limits");
  return limits;
}

/** ✅️ Derives an owned generator's exact optional freshness command. */
export function generatorNxCheckCommand(contract: GeneratorContract): readonly ["bun", "nx", "run", string] | null {
  if (contract.ownership !== "owned" || !contract.checkTarget) return null;
  return ["bun", "nx", "run", contract.checkTarget];
}

/** 🧭️ Structural ancestry supplied to owner-local exact semantic member resolution. */
export interface SemanticDirectoryMatchContext {
  readonly parentKindId?: string;
  readonly ancestorKindIds?: readonly string[];
}

function canonicalSemanticDirectoryName(name: string, taxonomy: Taxonomy): string {
  const normalized = name.normalize(taxonomy.unicodeNormalization.form);
  const leading = leadingEmojiIdentity(normalized).first;
  if (!leading) return normalized;
  if (/^\p{Extended_Pictographic}$/u.test(leading)) return `${leading}\uFE0F${normalized.slice(leading.length)}`;
  return normalized;
}

/** 📁️ Resolves one canonical semantic directory name to its unique global or owner-local registry identifier. */
export function semanticDirectoryKindId(name: string, taxonomy: Taxonomy = loadTaxonomy(), context: SemanticDirectoryMatchContext = {}): string | null {
  const normalized = canonicalSemanticDirectoryName(name, taxonomy);
  const identity = leadingEmojiIdentity(normalized);
  const roleName = identity.emoji ? `${identity.first}${identity.rest}` : normalized;
  const parentKindId = context.parentKindId;
  const matches = Object.entries(taxonomy.semanticDirectoryKinds).filter(([, spec]) => {
    if (spec.parentKindIds && !parentKindId) return false;
    if (spec.parentKindIds && !spec.parentKindIds.includes(parentKindId!)) return false;
    if (!roleName.startsWith(spec.emoji)) return false;
    const slug = roleName.slice(spec.emoji.length);
    return (slug.length === 0 && spec.allowEmojiOnly) || (slug.length > 0 && new RegExp(spec.slugPattern, "u").test(slug));
  });
  const contextual = parentKindId ? matches.filter(([, spec]) => spec.parentKindIds?.includes(parentKindId)) : [];
  const resolved = contextual.length > 0 ? contextual : matches.filter(([, spec]) => !spec.parentKindIds);
  if (resolved.length === 1) return resolved[0]![0];
  if (resolved.length > 1) return null;
  const owners = [context.parentKindId, ...(context.ancestorKindIds ?? [])].filter((id): id is string => Boolean(id));
  for (const ownerKindId of owners) {
    const memberMatches = Object.entries(taxonomy.semanticDirectoryMemberKinds).filter(([, spec]) => spec.ownerKindIds.includes(ownerKindId) && spec.memberNames.includes(roleName));
    if (memberMatches.length > 0) return memberMatches.length === 1 ? memberMatches[0]![0] : null;
  }
  return null;
}

/** 📌️ Filesystem-independent admitted facts for one owner-governed semantic leaf. */
export interface SemanticOwnedFileProjectionFacts {
  readonly ownerPath: string;
  readonly ownerFixedDirectoryContractIds: readonly string[];
  readonly manifestPath: string;
  readonly manifestFixedFilenameContractIds: readonly string[];
  readonly manifestContent: string;
  readonly sourcePath: string;
  readonly sourceFileKindId: string;
  readonly sourceByteLength: number;
}

/** 📌️ Exact authority decision for one owner-governed semantic leaf. */
export interface SemanticOwnedFileProjectionAuthority {
  readonly contractId: string;
  readonly ownerPath: string;
  readonly manifestPath: string;
  readonly manifestContentHash: string;
  readonly sourcePath: string;
  readonly destinationPath?: string;
  readonly status?: "closed" | "open";
  readonly contentState: "nonzero" | "zero-byte";
  readonly disposition: "problem" | "project" | "remove" | "unclaimed";
  readonly problems: readonly string[];
}

/** 🧭️ Resolves one semantic leaf only through its exact owner and sibling-manifest contract. */
export function semanticOwnedFileProjectionAuthority(facts: SemanticOwnedFileProjectionFacts, taxonomy: Taxonomy = loadTaxonomy()): SemanticOwnedFileProjectionAuthority {
  if (!Number.isSafeInteger(facts.sourceByteLength) || facts.sourceByteLength < 0) throw new Error("sourceByteLength must be a non-negative safe integer.");
  const contractId = "ticket-important-markdown-v1";
  const contract = taxonomy.semanticOwnedFileProjectionContracts[contractId];
  if (!contract || contract.contractKind !== "owner-sibling-manifest-file") throw new Error("The active ticket important projection contract is required.");
  const contentState = facts.sourceByteLength === 0 ? "zero-byte" : "nonzero";
  const core = { contractId, ownerPath: facts.ownerPath, manifestPath: facts.manifestPath, manifestContentHash: createHash("sha256").update(facts.manifestContent).digest("hex"), sourcePath: facts.sourcePath, contentState } as const;
  const manifestContract = taxonomy.fixedFilenameContracts[contract.requiredSiblingFixedFilenameContractId];
  const claimed = facts.ownerFixedDirectoryContractIds.includes(contract.ownerFixedDirectoryContractId)
    && facts.manifestFixedFilenameContractIds.includes(contract.requiredSiblingFixedFilenameContractId)
    && dirname(facts.sourcePath) === facts.ownerPath
    && basename(facts.sourcePath) === contract.sourceFilename
    && facts.sourceFileKindId === contract.sourceFileKindId
    && dirname(facts.manifestPath) === facts.ownerPath
    && manifestContract !== undefined
    && basename(facts.manifestPath) === fixedContractFilename(manifestContract);
  if (!claimed) return { ...core, disposition: "unclaimed", problems: [] };
  let status: "closed" | "open" | undefined;
  try {
    const manifest = JSON.parse(facts.manifestContent) as unknown;
    if (typeof manifest === "object" && manifest !== null && !Array.isArray(manifest) && Object.hasOwn(manifest, contract.manifestStatusLocation)) {
      const candidate = (manifest as Record<string, unknown>)[contract.manifestStatusLocation];
      if (candidate === "closed" || candidate === "open") status = candidate;
    }
  } catch {}
  if (!status) return { ...core, disposition: "problem", problems: ["Ticket manifest must own an explicit status equal to closed or open."] };
  const destinationPath = `${facts.ownerPath}/${contract.destinationDirectoryName}/${contract.destinationFilename}`;
  if (status === "open") return { ...core, destinationPath, status, disposition: "project", problems: [] };
  if (contentState === "zero-byte") return { ...core, status, disposition: "remove", problems: [] };
  return { ...core, destinationPath, status, disposition: "problem", problems: ["Closed ticket important document must be exactly zero bytes."] };
}

/** 📓️ Filesystem-independent admitted facts for one historical ticket note. */
export interface SemanticOwnedFileHistoryProjectionFacts {
  readonly ownerPath: string;
  readonly ownerFixedDirectoryContractIds: readonly string[];
  readonly manifestPath?: string;
  readonly manifestFixedFilenameContractIds: readonly string[];
  readonly manifestContent?: string;
  readonly sourcePath: string;
  readonly sourceFileKindId: string;
  readonly sourceByteLength: number;
}

/** 📓️ Exact authority decision for one historical ticket note. */
export interface SemanticOwnedFileHistoryProjectionAuthority {
  readonly contractId: "ticket-important-history-markdown-v1";
  readonly ownerPath: string;
  readonly manifestPath?: string;
  readonly manifestContentHash?: string;
  readonly manifestState: "closed" | "invalid" | "missing" | "open";
  readonly sourcePath: string;
  readonly destinationPath?: string;
  readonly contentState: "nonzero" | "zero-byte";
  readonly disposition: "project" | "unclaimed";
  readonly problems: readonly string[];
}

/** 🧭️ Resolves one historical note only through its exact ticket owner and residual lifecycle state. */
export function semanticOwnedFileHistoryProjectionAuthority(facts: SemanticOwnedFileHistoryProjectionFacts, taxonomy: Taxonomy = loadTaxonomy()): SemanticOwnedFileHistoryProjectionAuthority {
  if (!Number.isSafeInteger(facts.sourceByteLength) || facts.sourceByteLength < 0) throw new Error("sourceByteLength must be a non-negative safe integer.");
  const contractId = "ticket-important-history-markdown-v1" as const;
  const contract = taxonomy.semanticOwnedFileProjectionContracts[contractId];
  if (!contract || contract.contractKind !== "owner-optional-sibling-manifest-file") throw new Error("The ticket important history projection contract is required.");
  const contentState = facts.sourceByteLength === 0 ? "zero-byte" : "nonzero";
  const manifestContract = taxonomy.fixedFilenameContracts[contract.optionalSiblingFixedFilenameContractId];
  const manifestClaimed = facts.manifestPath !== undefined
    && facts.manifestContent !== undefined
    && facts.manifestFixedFilenameContractIds.includes(contract.optionalSiblingFixedFilenameContractId)
    && dirname(facts.manifestPath) === facts.ownerPath
    && manifestContract !== undefined
    && basename(facts.manifestPath) === fixedContractFilename(manifestContract);
  let manifestState: SemanticOwnedFileHistoryProjectionAuthority["manifestState"] = "missing";
  if (facts.manifestPath !== undefined || facts.manifestContent !== undefined || facts.manifestFixedFilenameContractIds.length > 0) manifestState = "invalid";
  if (manifestClaimed) {
    try {
      const manifest = JSON.parse(facts.manifestContent!) as unknown;
      const candidate = typeof manifest === "object" && manifest !== null && !Array.isArray(manifest) ? (manifest as Record<string, unknown>)[contract.manifestStatusLocation] : undefined;
      manifestState = candidate === "closed" || candidate === "open" ? candidate : "invalid";
    } catch { manifestState = "invalid"; }
  }
  const core = {
    contractId,
    ownerPath: facts.ownerPath,
    ...(facts.manifestPath === undefined ? {} : { manifestPath: facts.manifestPath }),
    ...(facts.manifestContent === undefined ? {} : { manifestContentHash: createHash("sha256").update(facts.manifestContent).digest("hex") }),
    manifestState,
    sourcePath: facts.sourcePath,
    contentState,
  } as const;
  const claimed = facts.ownerFixedDirectoryContractIds.includes(contract.ownerFixedDirectoryContractId)
    && dirname(facts.sourcePath) === facts.ownerPath
    && basename(facts.sourcePath) === contract.sourceFilename
    && facts.sourceFileKindId === contract.sourceFileKindId;
  const admitted = manifestState === "missing" || manifestState === "invalid" || manifestState === "closed" && contentState === "nonzero";
  if (!claimed || !admitted) return { ...core, disposition: "unclaimed", problems: [] };
  return { ...core, destinationPath: `${facts.ownerPath}/${contract.destinationDirectoryName}/${contract.destinationFilename}`, disposition: "project", problems: [] };
}

/** 📄️ Filesystem-independent facts for a primary owner leaf. */
export interface SemanticOwnedPrimaryFileProjectionFacts {
  readonly ownerPath: string;
  readonly ownerFixedDirectoryContractIds: readonly string[];
  readonly sourcePath: string;
  readonly sourceFileKindId: string;
}

/** 📑️ A direct physical-leaf projection that does not depend on ticket lifecycle. */
export interface SemanticOwnedPrimaryFileProjectionAuthority {
  readonly contractId: "ticket-document-primary-markdown-v1";
  readonly ownerPath: string;
  readonly sourcePath: string;
  readonly destinationPath?: string;
  readonly disposition: "project" | "unclaimed";
  readonly problems: readonly string[];
}

/** 🧭️ Drops the exact ticket document convention only under its registered semantic owner. */
export function semanticOwnedPrimaryFileProjectionAuthority(facts: SemanticOwnedPrimaryFileProjectionFacts, taxonomy: Taxonomy = loadTaxonomy()): SemanticOwnedPrimaryFileProjectionAuthority {
  const contractId = "ticket-document-primary-markdown-v1" as const;
  const contract = taxonomy.semanticOwnedFileProjectionContracts[contractId];
  if (!contract || contract.contractKind !== "owner-primary-file") throw new Error("The ticket document primary leaf projection contract is required.");
  const core = { contractId, ownerPath: facts.ownerPath, sourcePath: facts.sourcePath };
  const claimed = facts.ownerFixedDirectoryContractIds.includes(contract.ownerFixedDirectoryContractId)
    && dirname(facts.sourcePath) === facts.ownerPath
    && basename(facts.sourcePath) === contract.sourceFilename
    && facts.sourceFileKindId === contract.sourceFileKindId;
  if (!claimed) return { ...core, disposition: "unclaimed", problems: [] };
  return { ...core, destinationPath: `${facts.ownerPath}/${contract.destinationFilename}`, disposition: "project", problems: [] };
}

/** 🧭️ Resolves an empty-facet marker only through a complete registered semantic owner chain. */
export function semanticArtifactEmptyFacetProjectionAuthority(facts: Readonly<{ sourcePath: string; sourceFileKindId: string }>, taxonomy: Taxonomy = loadTaxonomy()): Readonly<{ contractId: "artifact-empty-facet-primary-markdown-v1"; sourcePath: string; ownerForm?: string; destinationPath?: string; disposition: "project" | "unclaimed" }> {
  const contractId = "artifact-empty-facet-primary-markdown-v1" as const;
  const contract = taxonomy.semanticOwnedFileProjectionContracts[contractId];
  if (!contract || contract.contractKind !== "semantic-facet-primary-file") throw new Error("The artifact empty-facet primary-leaf contract is required.");
  const core = { contractId, sourcePath: facts.sourcePath };
  const kindId = taxonomy[contract.fileKindAuthority];
  const segments = facts.sourcePath.split("/");
  if (segments.some((segment) => !segment || segment === "." || segment === ".." || /[\\\0]/u.test(segment)) || !facts.sourcePath.startsWith(`${contract.sourceRoot}/`) || basename(facts.sourcePath) !== contract.sourceFilename || facts.sourceFileKindId !== kindId) return { ...core, disposition: "unclaimed" };
  const owners = dirname(facts.sourcePath).slice(contract.sourceRoot.length + 1).split("/");
  const matches = Object.entries(contract.ownerPathPatterns).filter(([, pattern]) => {
    const expected = pattern.split("/");
    if (expected.length !== owners.length) return false;
    let parentKindId = "plugins";
    return expected.every((segment, index) => {
      const name = owners[index]!;
      const kind = semanticDirectoryKindId(name, taxonomy, { parentKindId });
      if (!kind) return false;
      parentKindId = kind;
      const capture = /^\{([a-zA-Z]+)\}$/u.exec(segment)?.[1];
      if (!capture) return canonicalSemanticDirectoryName(name, taxonomy) === segment;
      const rule = contract.directoryCaptures[capture];
      return rule !== undefined && rule.kindIds.includes(kind) && (!rule.names || rule.names.includes(canonicalSemanticDirectoryName(name, taxonomy)));
    });
  });
  if (matches.length !== 1) return { ...core, disposition: "unclaimed" };
  return { ...core, ownerForm: matches[0]![0], destinationPath: `${dirname(facts.sourcePath)}/${canonicalFilenameForKind(kindId, taxonomy)}`, disposition: "project" };
}

/** 📚️ One exact physical scenario registration owned by a MutationCatalog vector. */
export interface SemanticProjectionScenarioRegistration {
  readonly id: string;
  readonly directoryName: string;
}

/** 🦠️ One independent physical vector registration; it is not a runtime-kind declaration. */
export interface SemanticProjectionVectorRegistration {
  readonly mutationId: string;
  readonly sourceMutationDirectoryName: string;
  readonly mutationDirectoryName: string;
  readonly scenarios: readonly SemanticProjectionScenarioRegistration[];
}

/** 📖️ One owner-local catalog whose physical and canonical vector identities are validated together. */
export interface SemanticProjectionCatalogRegistration {
  readonly ownerPath: string;
  readonly catalogId: string;
  readonly vectors: readonly SemanticProjectionVectorRegistration[];
}

/** 🧾️ Validates strict source/canonical identities, ownership, collisions, and path budgets. */
export function semanticProjectionCatalogProblems(registrations: readonly SemanticProjectionCatalogRegistration[], taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const projected = taxonomy.semanticProjectedMemberKinds[taxonomy.mutationCatalogProjection.projectedMemberKindId];
  const members = projected && taxonomy.semanticDirectoryMemberKinds[projected.sourceMemberKindId];
  const destinationOwners = new Set<string>();
  for (const [catalogIndex, catalog] of registrations.entries()) {
    const scope = `catalogs[${catalogIndex}]`;
    if (!catalog.ownerPath || !catalog.catalogId) problems.push(`${scope} must declare ownerPath and catalogId.`);
    const profile = catalog.ownerPath.match(/^(.*)\/🏅️standards\/🔖️([^/]+)\/🪆️subsets\/✳️([^/]+)$/u);
    if (!profile) problems.push(`${scope}.ownerPath is not an exact artifact standard/subset owner.`);
    const sourceTuples = new Set<string>();
    const canonicalTuples = new Set<string>();
    for (const [vectorIndex, vector] of catalog.vectors.entries()) {
      const vectorScope = `${scope}.vectors[${vectorIndex}]`;
      const keys = Object.keys(vector).sort().join("\0");
      if (keys !== ["mutationDirectoryName", "mutationId", "scenarios", "sourceMutationDirectoryName"].sort().join("\0")) problems.push(`${vectorScope} must contain exactly mutationId, sourceMutationDirectoryName, mutationDirectoryName, and scenarios.`);
      const mutationId = typeof vector.mutationId === "string" ? vector.mutationId : "";
      const sourceMutationDirectoryName = typeof vector.sourceMutationDirectoryName === "string" ? vector.sourceMutationDirectoryName : "";
      const mutationDirectoryName = typeof vector.mutationDirectoryName === "string" ? vector.mutationDirectoryName : "";
      for (const [field, value] of Object.entries({ mutationId, sourceMutationDirectoryName, mutationDirectoryName })) if (!value || value !== value.normalize("NFC") || /[\\/]/u.test(value)) problems.push(`${vectorScope}.${field} must be one non-empty NFC basename.`);
      if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(mutationId)) problems.push(`${vectorScope}.mutationId must be kebab-case.`);
      const sourceOwner = mutationCatalogSourceOwner(catalog.ownerPath, taxonomy);
      if (sourceOwner === null) problems.push(`${vectorScope} has invalid explicit catalog source ownership.`);
      const mutationRoot = `${sourceOwner ?? catalog.ownerPath}/🧬️schema/🧬️mutations`;
      const grouped = Object.hasOwn(taxonomy.mutationDomainOwners, mutationRoot);
      const registeredOwner = grouped ? mutationOwnerRelativePath(mutationRoot, mutationId, taxonomy) : null;
      const canonical = grouped ? registeredOwner ?? mutationDirectoryName : members ? canonicalSemanticDirectoryName(mutationDirectoryName, taxonomy) : mutationDirectoryName;
      if (grouped) {
        if (!registeredOwner || registeredOwner.split("/").at(-1) !== sourceMutationDirectoryName || registeredOwner.split("/").at(-1) !== mutationDirectoryName) problems.push(`${vectorScope} has no exact registered domain-operation owner for mutationId.`);
      } else {
        if ((sourceMutationDirectoryName.match(/[a-z0-9][a-z0-9-]*$/u)?.[0] ?? "") !== mutationId) problems.push(`${vectorScope}.sourceMutationDirectoryName must render mutationId.`);
        if ((mutationDirectoryName.match(/[a-z0-9][a-z0-9-]*$/u)?.[0] ?? "") !== mutationId) problems.push(`${vectorScope}.mutationDirectoryName must render mutationId.`);
        if (!members?.memberNames.includes(canonical)) problems.push(`${vectorScope}.mutationDirectoryName has no exact canonical schema membership.`);
      }
      if (!Array.isArray(vector.scenarios) || vector.scenarios.length === 0) problems.push(`${vectorScope}.scenarios must be non-empty.`);
      for (const [scenarioIndex, scenario] of (vector.scenarios ?? []).entries()) {
        const scenarioScope = `${vectorScope}.scenarios[${scenarioIndex}]`;
        if (Object.keys(scenario).sort().join("\0") !== ["directoryName", "id"].join("\0")) problems.push(`${scenarioScope} must contain exactly id and directoryName.`);
        if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(scenario.id) || leadingEmojiIdentity(scenario.directoryName).rest !== scenario.id || scenario.directoryName !== scenario.directoryName.normalize("NFC") || pathEmojiStatuteFindings([{ path: scenario.directoryName, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities).length > 0) problems.push(`${scenarioScope} must be one canonical NFC test-case identity.`);
        const sourceTuple = `${mutationId}\0${sourceMutationDirectoryName}\0${scenario.id}`;
        const canonicalTuple = `${mutationId}\0${canonical}\0${scenario.id}`;
        if (sourceTuples.has(sourceTuple)) problems.push(`${scenarioScope} duplicates a source bundle tuple.`);
        if (canonicalTuples.has(canonicalTuple)) problems.push(`${scenarioScope} duplicates a canonical bundle tuple.`);
        sourceTuples.add(sourceTuple);
        canonicalTuples.add(canonicalTuple);
        if (profile) {
          const destination = `${profile[1]}/🧪️tests/🪆️${profile[2]}-${profile[3]}/${canonical}/${scenario.directoryName}`.normalize("NFC");
          const destinationKey = destination.replaceAll("\uFE0F", "").toLocaleLowerCase("und");
          if (destinationOwners.has(destinationKey)) problems.push(`${scenarioScope} collides at projected destination ${JSON.stringify(destination)}.`);
          destinationOwners.add(destinationKey);
          const reserve = taxonomy.semanticDescendantContracts[taxonomy.mutationCatalogProjection.descendantContractId]?.pathBudgetReserve.bytes ?? taxonomy.collisionPolicy.maxPathBytes;
          if (new TextEncoder().encode(destination).length + reserve > taxonomy.collisionPolicy.maxPathBytes) problems.push(`${scenarioScope} exceeds maxPathBytes after the canonical descendant reserve.`);
        }
      }
    }
  }
  return problems;
}

/** 🧭️ Full source-owner identity required before resolving a projected mutation member. */
export interface SemanticProjectedMemberContext {
  readonly projectionContractId: string;
  readonly artifactId: string;
  readonly artifactDirectoryName: string;
  readonly standardVersion: string;
  readonly standardDirectoryName: string;
  readonly subsetId: string;
  readonly subsetDirectoryName: string;
  readonly mutationId: string;
  readonly mutationDirectoryName: string;
  readonly scenarioId: string;
  readonly scenarioDirectoryName: string;
  readonly vectors: readonly SemanticProjectionVectorRegistration[];
}

function exactSemanticKindName(name: string, kindId: string, parentKindId: string, taxonomy: Taxonomy): boolean {
  return semanticDirectoryKindId(name, taxonomy, { parentKindId }) === kindId;
}

/** 🪞️ Resolves one projected mutation member only from a complete artifact/profile/vector identity. */
export function semanticProjectedMemberKindId(name: string, context: SemanticProjectedMemberContext, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const contract = taxonomy.semanticPathProjectionContracts[context.projectionContractId];
  if (!contract || !context.artifactId || !context.mutationId || !context.scenarioId) return null;
  const sourceOwner = taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId];
  if (!sourceOwner?.memberNames.includes(canonicalSemanticDirectoryName(context.artifactDirectoryName, taxonomy))) return null;
  if (context.standardDirectoryName !== `🔖️${context.standardVersion}` || !exactSemanticKindName(context.standardDirectoryName, "standard", "standards", taxonomy)) return null;
  if (context.subsetDirectoryName !== `✳️${context.subsetId}` || !exactSemanticKindName(context.subsetDirectoryName, "subset", "subsets", taxonomy)) return null;
  if (context.scenarioDirectoryName !== `🧪️${context.scenarioId}` || semanticDirectoryKindId(context.scenarioDirectoryName, taxonomy, { parentKindId: "mutation-test-subject" }) !== "test-case") return null;
  const memberName = canonicalSemanticDirectoryName(name, taxonomy);
  if (memberName !== canonicalSemanticDirectoryName(context.mutationDirectoryName, taxonomy)) return null;
  const vectorMatches = context.vectors.filter((vector) => vector.mutationId === context.mutationId && canonicalSemanticDirectoryName(vector.mutationDirectoryName, taxonomy) === memberName);
  if (vectorMatches.length !== 1) return null;
  const scenarioMatches = vectorMatches[0]!.scenarios.filter((scenario) => scenario.id === context.scenarioId && scenario.directoryName === context.scenarioDirectoryName);
  if (scenarioMatches.length !== 1) return null;
  const matches = Object.entries(taxonomy.semanticProjectedMemberKinds).filter(([id, spec]) => {
    if (spec.projectionContractId !== context.projectionContractId || !contract.destinationSegments.some((segment) => "projectedMemberKindId" in segment && segment.projectedMemberKindId === id)) return false;
    const source = taxonomy.semanticDirectoryMemberKinds[spec.sourceMemberKindId];
    return spec.identityField === "mutationDirectoryName" && source?.memberNames.includes(memberName);
  });
  return matches.length === 1 ? matches[0]![0] : null;
}

/** 🪆️ Stable profile identity rendered only from captured standard/subset values. */
export interface SemanticProjectionProfileIdentity {
  readonly artifactId: string;
  readonly standardVersion: string;
  readonly subsetId: string;
}

/** 🎨️ Renders one profile without exposing a reverse string parser. */
export function renderSemanticProjectionProfile(contractId: string, identity: SemanticProjectionProfileIdentity, taxonomy: Taxonomy = loadTaxonomy()): string {
  const contract = taxonomy.semanticPathProjectionContracts[contractId];
  const renderer = contract && taxonomy.semanticPathProjectionProfileRenderers[contract.profileRendererId];
  if (!contract || !renderer || !identity.artifactId) throw new Error(`Unknown or incomplete semantic projection ${JSON.stringify(contractId)}.`);
  if (!new RegExp(taxonomy.semanticDirectoryKinds.standard.slugPattern, "u").test(identity.standardVersion) || !new RegExp(taxonomy.semanticDirectoryKinds.subset.slugPattern, "u").test(identity.subsetId)) throw new Error("Projection profile captures do not satisfy standard/subset slug contracts.");
  const rendered = renderer.template.replace("{standardVersion}", identity.standardVersion).replace("{subsetId}", identity.subsetId);
  const profileIndex = contract.destinationSegments.findIndex((segment) => "render" in segment && segment.render === "profile");
  const parent = profileIndex > 0 ? contract.destinationSegments[profileIndex - 1] : undefined;
  const parentKindId = parent && "kindId" in parent ? parent.kindId : undefined;
  if (semanticDirectoryKindId(rendered, taxonomy, { parentKindId }) !== renderer.directoryKindId) throw new Error(`Projection profile renderer produced invalid directory ${JSON.stringify(rendered)}.`);
  return rendered;
}

/** ⚖️ Renders profile tuples and rejects distinct tuples colliding within one artifact owner. */
export function renderSemanticProjectionProfiles(contractId: string, identities: readonly SemanticProjectionProfileIdentity[], taxonomy: Taxonomy = loadTaxonomy()): readonly Readonly<SemanticProjectionProfileIdentity & { directoryName: string }>[] {
  const seen = new Map<string, string>();
  return identities.map((identity) => {
    const directoryName = renderSemanticProjectionProfile(contractId, identity, taxonomy);
    const key = `${identity.artifactId}\0${directoryName}`;
    const tuple = `${identity.standardVersion}\0${identity.subsetId}`;
    const prior = seen.get(key);
    if (prior && prior !== tuple) throw new Error(`Projection profile collision for artifact ${JSON.stringify(identity.artifactId)} at ${JSON.stringify(directoryName)}.`);
    seen.set(key, tuple);
    return { ...identity, directoryName };
  });
}

/** 🔭️ Resolves exact schema-owned external projection consumers without owner inference. */
export function semanticPathProjectionReferenceConsumers(
  projectionContractId: string,
  sourcePath: string,
  adapter: SemanticPathProjectionReferenceConsumerContract["adapters"][number],
  form: SemanticPathProjectionReferenceConsumerForm,
  taxonomy: Taxonomy = loadTaxonomy(),
): readonly Readonly<{ id: string; contract: SemanticPathProjectionReferenceConsumerContract }>[] {
  return Object.entries(taxonomy.semanticPathProjectionReferenceConsumerContracts)
    .filter(([, contract]) => contract.projectionContractId === projectionContractId && contract.sourcePathIdentities.includes(sourcePath) && contract.adapters.includes(adapter) && contract.supportedForms.includes(form) && new RegExp(contract.sourcePathPattern, "u").test(sourcePath))
    .map(([id, contract]) => ({ id, contract }))
    .sort((left, right) => projectionByteCompare(left.id, right.id));
}

/** 🧱️ One exact source node supplied to the read-only projection authority. */
export interface SemanticProjectionAuthorityNode {
  readonly path: string;
  readonly nodeKind: "directory" | "file" | "symlink";
  readonly content?: string;
}

/** 🧭️ Inputs shared by the two artifact projection authorities. */
export interface SemanticPathProjectionAuthorityOptions {
  readonly artifactRoot: string;
  readonly contractId: string;
  readonly sourceRoot: string;
  readonly nodes: readonly SemanticProjectionAuthorityNode[];
  readonly occupiedPaths?: readonly string[];
  readonly layout?: "source" | "destination";
}

/** 🛤️ One deterministic existing-file projection pair. */
export interface SemanticPathProjectionMapping {
  readonly sourcePath: string;
  readonly destinationPath: string;
}

/** 🔗️ One structured configuration reference required by a configurable projected entry. */
export interface SemanticPathProjectionReferenceEdit {
  readonly path: string;
  readonly adapter: "json" | "toml";
  readonly structuredLocation: string;
  readonly oldValue: string;
  readonly newValue: string;
  readonly preimageHash: string;
}

/** 📊️ Fail-closed projection result; mappings are exposed only when problems is empty. */
export interface SemanticPathProjectionAuthority {
  readonly contractId: string;
  readonly sourceRoot: string;
  readonly destinationRoot: string;
  readonly mappings: readonly SemanticPathProjectionMapping[];
  readonly referenceEdits: readonly SemanticPathProjectionReferenceEdit[];
  readonly destinationDirectoryCount: number;
  readonly destinationNodeCount: number;
  readonly mappingDigest: string;
  readonly maxPathBytes: number;
  readonly problems: readonly string[];
}

/** 🎨️ Shared forward-only root rendering result for artifact profile projections. */
export interface ArtifactPathProjectionRoot {
  readonly captures: Readonly<Record<string, string>>;
  readonly destinationRoot: string;
  readonly problems: readonly string[];
}

function projectionByteCompare(left: string, right: string): number {
  return Buffer.from(left).compare(Buffer.from(right));
}

function projectionDirectories(root: string, paths: readonly string[]): string[] {
  const directories = new Set<string>([root]);
  for (const path of paths) {
    const segments = path.slice(root.length + 1).split("/");
    segments.pop();
    let current = root;
    for (const segment of segments) {
      current = `${current}/${segment}`;
      directories.add(current);
    }
  }
  return [...directories].sort(projectionByteCompare);
}

function projectionCanonicalKey(path: string, comparison: "nfc" | "case-fold" | "vs16-fold"): string {
  const normalized = path.normalize("NFC");
  if (comparison === "nfc") return normalized;
  if (comparison === "case-fold") return normalized.toLocaleLowerCase("und");
  return normalized.replaceAll("\uFE0F", "");
}

function artifactProjectionPathProblems(paths: readonly Readonly<{ path: string; nodeKind: "directory" | "file" }>[], occupiedPaths: readonly string[], taxonomy: Taxonomy): string[] {
  const problems: string[] = [];
  for (const comparison of ["nfc", "case-fold", "vs16-fold"] as const) {
    const seen = new Map<string, string>();
    for (const entry of paths) {
      const key = `${entry.nodeKind}\0${projectionCanonicalKey(entry.path, comparison)}`;
      const prior = seen.get(key);
      if (prior && prior !== entry.path) problems.push(`${comparison} collision between ${JSON.stringify(prior)} and ${JSON.stringify(entry.path)}.`);
      seen.set(key, entry.path);
    }
  }
  const occupied = new Set(occupiedPaths.flatMap((path) => [path, projectionCanonicalKey(path, "nfc"), projectionCanonicalKey(path, "case-fold"), projectionCanonicalKey(path, "vs16-fold")]));
  for (const { path } of paths) {
    if ([path, projectionCanonicalKey(path, "nfc"), projectionCanonicalKey(path, "case-fold"), projectionCanonicalKey(path, "vs16-fold")].some((key) => occupied.has(key))) problems.push(`Projected destination ${JSON.stringify(path)} is occupied.`);
    if (new TextEncoder().encode(path).length > taxonomy.collisionPolicy.maxPathBytes) problems.push(`Projected destination ${JSON.stringify(path)} exceeds maxPathBytes ${taxonomy.collisionPolicy.maxPathBytes}.`);
  }
  return problems;
}

/** 🪆️ Parses exact source segments and renders a destination profile without reverse-splitting it. */
export function renderArtifactPathProjectionRoot(options: Pick<SemanticPathProjectionAuthorityOptions, "artifactRoot" | "contractId" | "sourceRoot">, taxonomy: Taxonomy = loadTaxonomy()): ArtifactPathProjectionRoot {
  const problems: string[] = [];
  const contract = taxonomy.semanticPathProjectionContracts[options.contractId];
  if (!contract || !contract.sourceArtifactMemberName) return { captures: {}, destinationRoot: "", problems: [`Unknown artifact projection contract ${JSON.stringify(options.contractId)}.`] };
  const sourceOwner = taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId];
  const artifactDirectoryName = basename(options.artifactRoot);
  if (artifactDirectoryName !== contract.sourceArtifactMemberName || !sourceOwner?.memberNames.includes(artifactDirectoryName)) problems.push(`Artifact root does not match sourceArtifactMemberName ${JSON.stringify(contract.sourceArtifactMemberName)}.`);
  if (options.sourceRoot !== options.sourceRoot.normalize("NFC") || options.artifactRoot !== options.artifactRoot.normalize("NFC") || /\uFE0E/u.test(`${options.artifactRoot}\0${options.sourceRoot}`)) problems.push("Projection roots must be NFC and must not contain VS15.");
  const prefix = `${options.artifactRoot}/`;
  const sourceNames = options.sourceRoot.startsWith(prefix) ? options.sourceRoot.slice(prefix.length).split("/") : [];
  if (sourceNames.length !== contract.sourceSegments.length) problems.push("Source root does not have the exact projection grammar length.");
  const captures: Record<string, string> = {};
  let parentKindId: string | undefined;
  for (const [index, segment] of contract.sourceSegments.entries()) {
    const name = sourceNames[index] ?? "";
    if ("memberKindId" in segment) {
      const member = taxonomy.semanticDirectoryMemberKinds[segment.memberKindId];
      if (name !== segment.literal || !member?.ownerKindIds.includes(parentKindId ?? "") || !member.memberNames.includes(canonicalSemanticDirectoryName(name, taxonomy))) problems.push(`Source segment ${index} does not match exact member registry ${JSON.stringify(segment.memberKindId)}.`);
      parentKindId = segment.memberKindId;
      continue;
    }
    if ("projectedMemberKindId" in segment) {
      const projected = taxonomy.semanticProjectedMemberKinds[segment.projectedMemberKindId];
      const source = projected && taxonomy.semanticDirectoryMemberKinds[projected.sourceMemberKindId];
      const canonical = canonicalSemanticDirectoryName(name, taxonomy);
      if (!projected || projected.projectionContractId !== options.contractId || !projected.ownerKindIds.includes(parentKindId ?? "") || !source?.memberNames.includes(canonical) || canonical !== name) problems.push(`Source segment ${index} does not match projected member ${JSON.stringify(segment.projectedMemberKindId)}.`);
      captures[segment.capture] = name;
      parentKindId = segment.projectedMemberKindId;
      continue;
    }
    if ("literal" in segment) {
      if (name !== segment.literal || semanticDirectoryKindId(name, taxonomy, { parentKindId }) !== segment.kindId) problems.push(`Source segment ${index} does not match kind ${JSON.stringify(segment.kindId)}.`);
    } else {
      const kind = taxonomy.semanticDirectoryKinds[segment.kindId];
      if (!kind || semanticDirectoryKindId(name, taxonomy, { parentKindId }) !== segment.kindId || !name.startsWith(kind.emoji)) problems.push(`Source capture ${JSON.stringify(segment.capture)} does not match kind ${JSON.stringify(segment.kindId)}.`);
      else captures[segment.capture] = name.slice(kind.emoji.length);
    }
    parentKindId = segment.kindId;
  }
  const standardVersion = captures.standardVersion ?? "";
  const subsetId = captures.subsetId ?? "";
  let destinationParentKindId: string | undefined;
  const destinationNames: string[] = [];
  for (const segment of contract.destinationSegments) {
    if ("projectedMemberKindId" in segment) {
      const value = captures[segment.copy];
      if (!value) problems.push(`Destination copy ${JSON.stringify(segment.copy)} is missing.`);
      else destinationNames.push(value);
      destinationParentKindId = segment.projectedMemberKindId;
      continue;
    }
    if ("literal" in segment) destinationNames.push(segment.literal);
    else if ("render" in segment) {
      try {
        destinationNames.push(renderSemanticProjectionProfile(options.contractId, { artifactId: artifactDirectoryName, standardVersion, subsetId }, taxonomy));
      } catch (error) {
        problems.push(error instanceof Error ? error.message : String(error));
      }
    } else {
      const value = captures[segment.copy];
      if (!value) problems.push(`Destination copy ${JSON.stringify(segment.copy)} is missing.`);
      else destinationNames.push(value);
    }
    destinationParentKindId = segment.kindId;
  }
  return { captures, destinationRoot: problems.length === 0 ? `${options.artifactRoot}/${destinationNames.join("/")}` : "", problems };
}

/** 🗺️ Enumerates exact artifact roots by forward-rendering registered profile tuples. */
export function artifactPathProjectionCatalogRoots(artifactRoot: string, contractId: string, taxonomy: Taxonomy = loadTaxonomy()): readonly Readonly<{ sourceRoot: string; destinationRoot: string }>[] {
  const contract = taxonomy.semanticPathProjectionContracts[contractId];
  const catalog = contract && taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId];
  if (!contract?.sourceArtifactMemberName || !catalog || !("contractKind" in catalog)) return [];
  const vectors = catalog.contractKind === "distributed-json-manifest-catalog" ? catalog.profileVectors : catalog.vectors;
  const roots = vectors.filter((vector) => vector.artifactId === basename(artifactRoot)).map((vector) => {
    const captures = vector as Readonly<Record<string, string>>;
    const sourceRoot = `${artifactRoot}/${contract.sourceSegments.map((segment) => "literal" in segment ? segment.literal : "projectedMemberKindId" in segment ? captures[segment.capture] : `${taxonomy.semanticDirectoryKinds[segment.kindId]?.emoji ?? ""}${captures[segment.capture] ?? ""}`).join("/")}`;
    const rendered = renderArtifactPathProjectionRoot({ artifactRoot, contractId, sourceRoot }, taxonomy);
    if (rendered.problems.length > 0) throw new Error(`Invalid forward profile vector for ${contractId}: ${rendered.problems.join(" | ")}`);
    return { sourceRoot, destinationRoot: rendered.destinationRoot };
  }).sort((left, right) => projectionByteCompare(left.destinationRoot, right.destinationRoot));
  for (const comparison of ["nfc", "case-fold", "vs16-fold"] as const) if (new Set(roots.map(({ destinationRoot }) => projectionCanonicalKey(destinationRoot, comparison))).size !== roots.length) throw new Error(`Forward profile vectors collide for ${contractId} under ${comparison}`);
  return roots;
}

function projectionJsonManifest(node: SemanticProjectionAuthorityNode, scope: string, problems: string[]): Record<string, unknown> | null {
  if (node.nodeKind !== "file" || typeof node.content !== "string") {
    problems.push(`${scope} must be a readable JSON file.`);
    return null;
  }
  try {
    const value = JSON.parse(node.content) as unknown;
    if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("not an object");
    return value as Record<string, unknown>;
  } catch {
    problems.push(`${scope} must contain one JSON object.`);
    return null;
  }
}

function projectionFileMappingDigest(mappings: readonly SemanticPathProjectionMapping[]): string {
  return createHash("sha256").update(mappings.map(({ sourcePath, destinationPath }) => `${sourcePath}\0${destinationPath}`).join("\n")).digest("hex");
}

/** 🧭️ Renders one repository-relative path without platform separators. */
function projectionRelativePath(fromDirectory: string, toPath: string): string {
  const from = fromDirectory.split("/");
  const to = toPath.split("/");
  let shared = 0;
  while (shared < from.length && shared < to.length && from[shared] === to[shared]) shared++;
  return [...from.slice(shared).map(() => ".."), ...to.slice(shared)].join("/");
}

/** 🔎️ Reads one exact structured configuration value without accepting aliases. */
function projectionStructuredValue(content: string, adapter: "json" | "toml", structuredLocation: string): unknown {
  if (adapter === "json") {
    let value: unknown = JSON.parse(content);
    for (const segment of structuredLocation.split(".")) value = typeof value === "object" && value !== null ? (value as Record<string, unknown>)[segment] : undefined;
    return value;
  }
  const separator = structuredLocation.lastIndexOf(".");
  if (separator < 1 || separator === structuredLocation.length - 1) return undefined;
  const body = tomlTableBody(content, structuredLocation.slice(0, separator));
  return body === undefined ? undefined : tomlTableValues(body)[structuredLocation.slice(separator + 1)];
}

/** 🧾️ Validates and renders either the CAD manifest catalog or Draw exact command bundle without writes. */
export function semanticPathProjectionAuthority(options: SemanticPathProjectionAuthorityOptions, taxonomy: Taxonomy = loadTaxonomy()): SemanticPathProjectionAuthority {
  const root = renderArtifactPathProjectionRoot(options, taxonomy);
  const problems = [...root.problems];
  const contract = taxonomy.semanticPathProjectionContracts[options.contractId];
  const destinationLayout = options.layout === "destination";
  const physicalRoot = destinationLayout ? root.destinationRoot : options.sourceRoot;
  const pathOwners = new Map<string, SemanticProjectionAuthorityNode>();
  for (const node of options.nodes) {
    if (node.path !== node.path.normalize("NFC") || /\uFE0E/u.test(node.path)) problems.push(`Projection node ${JSON.stringify(node.path)} must be NFC and must not contain VS15.`);
    if (!(node.path === physicalRoot || node.path.startsWith(`${physicalRoot}/`))) problems.push(`Projection node ${JSON.stringify(node.path)} is outside its exact ${destinationLayout ? "destination" : "source"} root.`);
    if (pathOwners.has(node.path)) problems.push(`Projection node ${JSON.stringify(node.path)} is duplicated.`);
    pathOwners.set(node.path, node);
    if (node.nodeKind === "symlink") problems.push(`Projection source contains forbidden symlink ${JSON.stringify(node.path)}.`);
  }
  if (pathOwners.get(physicalRoot)?.nodeKind !== "directory") problems.push("Projection physical root must be present as a directory node.");
  const actualFiles = options.nodes.filter((node) => node.nodeKind === "file").map((node) => node.path).sort(projectionByteCompare);
  const expectedSourceDirectories = projectionDirectories(physicalRoot, actualFiles);
  const actualSourceDirectories = options.nodes.filter((node) => node.nodeKind === "directory").map((node) => node.path).sort(projectionByteCompare);
  if (expectedSourceDirectories.join("\0") !== actualSourceDirectories.join("\0")) problems.push("Projection source directories must be exactly those owned by source files.");
  const candidateMappings: SemanticPathProjectionMapping[] = [];
  const configurableEntries: { sourcePath: string; destinationPath: string; configurationReferences: SemanticDescendantConfigurableEntryFileNode["configurableEntry"]["configurationReferences"] }[] = [];
  if (contract && root.destinationRoot) {
    const catalog = taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId];
    if (catalog && "contractKind" in catalog && catalog.contractKind === "distributed-json-manifest-catalog") {
      if (catalog.profileVectors.filter((vector) => vector.artifactId === basename(options.artifactRoot) && vector.standardVersion === root.captures.standardVersion && vector.subsetId === root.captures.subsetId).length !== 1) problems.push("CAD catalog must match exactly one forward owner profile vector.");
      const modelIds = new Set<string>();
      const memberIds = new Set<string>();
      const modelDirectories = actualSourceDirectories.filter((path) => dirname(path) === physicalRoot).map((path) => basename(path));
      if (modelDirectories.length === 0) problems.push("CAD catalog must contain at least one manifest-owned model.");
      for (const modelDirectoryName of modelDirectories) {
        const manifestPath = `${physicalRoot}/${modelDirectoryName}/${destinationLayout ? canonicalFilenameForKind("json", taxonomy) : catalog.modelManifestSourceFilename}`;
        const manifestNode = pathOwners.get(manifestPath);
        const manifest = manifestNode ? projectionJsonManifest(manifestNode, `Model manifest ${JSON.stringify(manifestPath)}`, problems) : null;
        if (!manifestNode) problems.push(`Model manifest is missing for ${JSON.stringify(modelDirectoryName)}.`);
        const leading = modelDirectoryName.match(/^(\p{Extended_Pictographic}\uFE0F?(?:\u200D\p{Extended_Pictographic}\uFE0F?)*)/u)?.[1] ?? "";
        const semanticStem = modelDirectoryName.slice(leading.length);
        if (!leading || canonicalSemanticDirectoryName(modelDirectoryName, taxonomy) !== modelDirectoryName || manifest?.schema !== catalog.modelManifestSchema || manifest?.[catalog.memberVersionField] !== catalog.requiredMemberVersion || manifest?.[catalog.modelIdentityField] !== semanticStem) problems.push(`Model manifest ${JSON.stringify(manifestPath)} must declare the canonical directory id, schema, and version.`);
        if (typeof manifest?.[catalog.modelIdentityField] === "string") {
          const id = manifest[catalog.modelIdentityField] as string;
          if (modelIds.has(id)) problems.push(`Model manifest identity ${JSON.stringify(id)} is duplicated.`);
          modelIds.add(id);
        }
      }
      for (const physicalPath of actualFiles) {
        const relativePath = physicalPath.slice(physicalRoot.length + 1);
        let segments = relativePath.split("/");
        if (destinationLayout) {
          if (segments.length === 2 && segments[1] === canonicalFilenameForKind("json", taxonomy)) segments = [segments[0]!, catalog.modelManifestSourceFilename];
          else {
            const rule = catalog.categoryRules.find((candidate) => candidate.sourceDirectoryName === segments[1]);
            if (segments.length !== 4 || segments[3] !== canonicalFilenameForKind("json", taxonomy) || !rule) problems.push(`Canonical CAD category member ${JSON.stringify(physicalPath)} does not match its exact category rule.`);
            else if (rule.sourceShape === "direct-semantic-json" && segments[2]!.startsWith(rule.memberDirectoryEmoji) && segments[2]!.length > rule.memberDirectoryEmoji.length) segments = [segments[0]!, segments[1]!, `🔣️${segments[2]!.slice(rule.memberDirectoryEmoji.length)}.json`];
            else if (rule.sourceShape === "nested-fixed-json") segments = [segments[0]!, segments[1]!, segments[2]!, rule.fixedSourceFilename];
            else problems.push(`Canonical CAD member ${JSON.stringify(physicalPath)} has no exact semantic directory identity.`);
          }
        }
        const sourcePath = `${options.sourceRoot}/${segments.join("/")}`;
        const modelDirectoryName = segments[0] ?? "";
        let destinationRelativePath = "";
        let expectedSchema = catalog.modelManifestSchema;
        if (segments.length === 2 && segments[1] === catalog.modelManifestSourceFilename) destinationRelativePath = `${modelDirectoryName}/${canonicalFilenameForKind("json", taxonomy)}`;
        else {
          const rule = catalog.categoryRules.find((candidate) => candidate.sourceDirectoryName === segments[1]);
          if (!rule) problems.push(`Unknown CAD catalog category in ${JSON.stringify(sourcePath)}.`);
          else if (rule.sourceShape === "direct-semantic-json" && segments.length === 3) {
            const match = segments[2]!.match(/^🔣️(.+)\.json$/u);
            if (!match) problems.push(`CAD direct category file ${JSON.stringify(sourcePath)} does not have the exact semantic JSON shape.`);
            else destinationRelativePath = `${modelDirectoryName}/${rule.sourceDirectoryName}/${rule.memberDirectoryEmoji}${match[1]}/${canonicalFilenameForKind("json", taxonomy)}`;
            expectedSchema = rule.manifestSchema;
          } else if (rule.sourceShape === "nested-fixed-json" && segments.length === 4 && segments[3] === rule.fixedSourceFilename && canonicalSemanticDirectoryName(segments[2]!, taxonomy) === segments[2]) {
            destinationRelativePath = `${modelDirectoryName}/${rule.sourceDirectoryName}/${segments[2]}/${canonicalFilenameForKind("json", taxonomy)}`;
            expectedSchema = rule.manifestSchema;
          } else problems.push(`CAD category member ${JSON.stringify(sourcePath)} does not match its exact category rule.`);
        }
        const node = pathOwners.get(physicalPath)!;
        const manifest = projectionJsonManifest(node, `Catalog manifest ${JSON.stringify(sourcePath)}`, problems);
        if (manifest && (manifest.schema !== expectedSchema || manifest[catalog.memberVersionField] !== catalog.requiredMemberVersion || typeof manifest[catalog.memberIdentityField] !== "string" || manifest[catalog.memberIdentityField] === "")) problems.push(`Catalog manifest ${JSON.stringify(sourcePath)} has an invalid manifest schema, identity, or version.`);
        if (manifest && typeof manifest[catalog.memberIdentityField] === "string") {
          const key = `${modelDirectoryName}\0${segments[1] ?? ""}\0${manifest[catalog.memberIdentityField]}`;
          if (memberIds.has(key)) problems.push(`Distributed catalog member identity ${JSON.stringify(key)} is duplicated.`);
          memberIds.add(key);
        }
        if (destinationRelativePath) {
          const destinationPath = `${root.destinationRoot}/${destinationRelativePath}`;
          if (destinationLayout && destinationPath !== physicalPath) problems.push(`Canonical CAD member ${JSON.stringify(physicalPath)} does not equal its forward-rendered catalog path.`);
          candidateMappings.push({ sourcePath, destinationPath });
        }
      }
    } else if (catalog && "contractKind" in catalog && catalog.contractKind === "exact-owner-vectors") {
      const capture = root.captures.commandDirectoryName;
      const vectors = catalog.vectors.filter((vector) => vector.artifactId === basename(options.artifactRoot) && vector.standardVersion === root.captures.standardVersion && vector.subsetId === root.captures.subsetId && vector.commandDirectoryName === capture);
      if (vectors.length !== 1) problems.push("Draw command source must match exactly one owner vector.");
      const descendant = taxonomy.semanticDescendantContracts[contract.descendantContractId];
      if (!descendant || "contractKind" in descendant) problems.push("Draw projection must reference one exact descendant bundle.");
      else {
        const expectedSourceNodes = new Set<string>();
        for (const node of descendant.requiredNodes) {
          const sourceParent = ("configurableEntry" in node ? node.sourcePathSegments : node.pathSegments).map((segment) => segment.literal);
          const destinationRelativePath = semanticDescendantNodeRelativePath(node, taxonomy);
          let sourceRelativePath = sourceParent.join("/");
          if (node.nodeType === "file") {
            if ("kindId" in node) {
              const kind = taxonomy.fileKinds[node.kindId];
              sourceRelativePath = [...sourceParent, node.sourceFilename ?? canonicalFilenameForKind(node.kindId, taxonomy)].join("/");
            } else if ("fixedFilenameContractId" in node) sourceRelativePath = [...sourceParent, fixedContractFilename(taxonomy.fixedFilenameContracts[node.fixedFilenameContractId]!)].join("/");
            else sourceRelativePath = [...sourceParent, node.configurableEntry.sourceFilename].join("/");
            const mapping = { sourcePath: `${options.sourceRoot}/${sourceRelativePath}`, destinationPath: `${root.destinationRoot}/${destinationRelativePath}` };
            candidateMappings.push(mapping);
            if ("configurableEntry" in node) configurableEntries.push({ ...mapping, configurationReferences: node.configurableEntry.configurationReferences });
          }
          expectedSourceNodes.add(destinationLayout ? destinationRelativePath ? `${root.destinationRoot}/${destinationRelativePath}` : root.destinationRoot : sourceRelativePath ? `${options.sourceRoot}/${sourceRelativePath}` : options.sourceRoot);
        }
        if (destinationLayout) for (const directory of projectionDirectories(root.destinationRoot, candidateMappings.map(({ destinationPath }) => destinationPath))) expectedSourceNodes.add(directory);
        const actualSourceNodes = new Set(options.nodes.filter((node) => node.nodeKind !== "symlink").map((node) => node.path));
        if (expectedSourceNodes.size !== actualSourceNodes.size || [...expectedSourceNodes].some((path) => !actualSourceNodes.has(path))) problems.push("Draw source does not contain the exact command bundle.");
      }
    } else problems.push(`Projection catalog ${JSON.stringify(contract.catalogContractId)} has the wrong authority kind.`);
  }
  candidateMappings.sort((left, right) => projectionByteCompare(left.sourcePath, right.sourcePath));
  const candidateReferenceEdits: SemanticPathProjectionReferenceEdit[] = [];
  for (const entry of configurableEntries) for (const reference of entry.configurationReferences) {
    const fixedContract = taxonomy.fixedFilenameContracts[reference.fixedFilenameContractId];
    const manifestFilename = fixedContract && fixedContractFilename(fixedContract);
    const sourceManifestPath = manifestFilename ? `${entry.sourcePath.slice(0, entry.sourcePath.lastIndexOf("/"))}/${manifestFilename}` : "";
    const manifestMapping = candidateMappings.find(({ sourcePath }) => sourcePath === sourceManifestPath);
    const manifestNode = pathOwners.get(destinationLayout ? manifestMapping?.destinationPath ?? "" : sourceManifestPath);
    if (!fixedContract || !manifestMapping || manifestNode?.nodeKind !== "file" || manifestNode.content === undefined) {
      problems.push(`Configurable entry ${JSON.stringify(entry.sourcePath)} is missing its exact configuration manifest mapping.`);
      continue;
    }
    const oldValue = projectionRelativePath(sourceManifestPath.slice(0, sourceManifestPath.lastIndexOf("/")), entry.sourcePath);
    const newValue = projectionRelativePath(manifestMapping.destinationPath.slice(0, manifestMapping.destinationPath.lastIndexOf("/")), entry.destinationPath);
    let actualValue: unknown;
    try {
      actualValue = projectionStructuredValue(manifestNode.content, reference.adapter, reference.structuredLocation);
    } catch {
      actualValue = undefined;
    }
    if (actualValue !== (destinationLayout ? newValue : oldValue)) {
      problems.push(`Configuration reference ${JSON.stringify(`${destinationLayout ? manifestMapping.destinationPath : sourceManifestPath}:${reference.structuredLocation}`)} must resolve exactly to ${JSON.stringify(destinationLayout ? newValue : oldValue)}.`);
      continue;
    }
    if (destinationLayout) continue;
    candidateReferenceEdits.push({
      path: manifestMapping.destinationPath,
      adapter: reference.adapter,
      structuredLocation: reference.structuredLocation,
      oldValue,
      newValue,
      preimageHash: createHash("sha256").update(manifestNode.content).digest("hex"),
    });
  }
  candidateReferenceEdits.sort((left, right) => projectionByteCompare(`${left.path}\0${left.structuredLocation}`, `${right.path}\0${right.structuredLocation}`));
  const destinationDirectories = root.destinationRoot ? projectionDirectories(root.destinationRoot, candidateMappings.map(({ destinationPath }) => destinationPath)) : [];
  const destinationNodes = [...destinationDirectories.map((path) => ({ path, nodeKind: "directory" as const })), ...candidateMappings.map(({ destinationPath: path }) => ({ path, nodeKind: "file" as const }))];
  problems.push(...artifactProjectionPathProblems(destinationNodes, options.occupiedPaths ?? [], taxonomy));
  const accepted = problems.length === 0;
  return {
    contractId: options.contractId,
    sourceRoot: options.sourceRoot,
    destinationRoot: root.destinationRoot,
    mappings: accepted ? candidateMappings : [],
    referenceEdits: accepted ? candidateReferenceEdits : [],
    destinationDirectoryCount: accepted ? destinationDirectories.length : 0,
    destinationNodeCount: accepted ? destinationNodes.length : 0,
    mappingDigest: accepted ? projectionFileMappingDigest(candidateMappings) : "",
    maxPathBytes: accepted ? Math.max(0, ...destinationNodes.map(({ path }) => new TextEncoder().encode(path).length)) : 0,
    problems,
  };
}

/** 📏️ Derives a descendant node's canonical relative suffix from directory and physical file kinds. */
export function semanticDescendantNodeRelativePath(node: SemanticDescendantNode, taxonomy: Taxonomy = loadTaxonomy()): string {
  const parent = ("configurableEntry" in node ? node.destinationPathSegments : node.pathSegments).map((segment) => segment.literal);
  if (node.nodeType === "directory") return parent.join("/");
  if ("kindId" in node) return [...parent, canonicalFilenameForKind(node.kindId, taxonomy)].join("/");
  if ("fixedFilenameContractId" in node) {
    const contract = taxonomy.fixedFilenameContracts[node.fixedFilenameContractId];
    if (!contract) throw new Error(`Unknown fixed filename contract ${JSON.stringify(node.fixedFilenameContractId)}.`);
    return [...parent, fixedContractFilename(contract)].join("/");
  }
  const entry = taxonomy.configurableEntryContracts[node.configurableEntry.contractId];
  if (!entry) throw new Error(`Unknown configurable entry contract ${JSON.stringify(node.configurableEntry.contractId)}.`);
  return [...parent, entry.filename].join("/");
}

/** 🧭️ Context required to enforce a fixed contract's scope beyond its path pattern. */
export interface FixedContractMatchContext {
  readonly packageRoot?: boolean;
  readonly ecosystemId?: string;
  readonly parentDirectoryKindId?: string;
  readonly parentFixedDirectoryContractIds?: readonly string[];
  readonly siblingFixedFilenameContractIds?: readonly string[];
  readonly pathMatcher?: TaxonomyPathMatcher;
}

function taxonomyPatternExpression(pattern: string): RegExp {
  let expression = "^";
  for (let index = 0; index < pattern.length;) {
    if (pattern.slice(index, index + 3) === "**/") {
      expression += "(?:[^/]+/)*";
      index += 3;
      continue;
    }
    const character = pattern[index]!;
    if (character === "*" && pattern[index + 1] === "*") {
      expression += ".*";
      index += 2;
      continue;
    }
    if (character === "*") expression += "[^/]*";
    else if (character === "?") expression += "[^/]";
    else if (character === "[") {
      const end = pattern.indexOf("]", index + 1);
      if (end < 0) throw new Error(`Invalid taxonomy path pattern ${JSON.stringify(pattern)}.`);
      expression += pattern.slice(index, end + 1);
      index = end;
    } else expression += character.replace(/[\\^$.*+?()[\]{}|]/gu, "\\$&");
    index += 1;
  }
  return new RegExp(`${expression}$`, "u");
}

/** 🪢 Matches one NFC workspace-relative POSIX path against the v7 contract glob grammar. */
export function taxonomyPathPatternMatches(path: string, pattern: string): boolean {
  const normalizedPath = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize("NFC");
  return taxonomyPatternExpression(pattern.normalize("NFC")).test(normalizedPath);
}

/** 🧮️ An invocation-owned matcher with private compiled patterns and no retained path results. */
export interface TaxonomyPathMatcher {
  readonly matches: (path: string, pattern: string) => boolean;
}

/** 🧵️ Reuses successful pure compilations only for this validation or query invocation. */
export function createTaxonomyPathMatcher(): TaxonomyPathMatcher {
  const expressions = new Map<string, RegExp>();
  return Object.freeze({
    matches(path: string, pattern: string): boolean {
      const normalizedPath = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize("NFC");
      const normalizedPattern = pattern.normalize("NFC");
      let expression = expressions.get(normalizedPattern);
      if (!expression) {
        expression = taxonomyPatternExpression(normalizedPattern);
        expressions.set(normalizedPattern, expression);
      }
      return expression.test(normalizedPath);
    },
  });
}

/** 🔒️Parses a finite union of literal, separately declared parent-directory authorities. */
export function parseFixedDirectoryContractSetScope(input: unknown, parents: Readonly<Record<string, FixedDirectoryContract>>): Extract<FixedContractScope, { kind: "fixed-directory-contract-set" }> {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Fixed parent scope must be an object");
  const row = input as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "fixedDirectoryContractIds,kind" || row.kind !== "fixed-directory-contract-set" || !Array.isArray(row.fixedDirectoryContractIds) || row.fixedDirectoryContractIds.length < 1 || row.fixedDirectoryContractIds.length > 256) throw new Error("Fixed parent scope requires one nonempty finite parent ID set");
  const ids = new Set<string>(), paths = new Set<string>();
  for (const id of row.fixedDirectoryContractIds) {
    if (typeof id !== "string" || !id || ids.has(id) || !parents[id]) throw new Error("Fixed parent scope contains a duplicate or unknown directory contract ID");
    const parent = parents[id];
    if (/[*?\[\]{}]/u.test(parent.pathPattern) || !["exact-path", "path-pattern", "repository-root"].includes(parent.scope.kind) || paths.has(parent.pathPattern)) throw new Error("Fixed parent scope requires distinct literal parent authorities");
    ids.add(id);
    paths.add(parent.pathPattern);
  }
  return { kind: "fixed-directory-contract-set", fixedDirectoryContractIds: [...ids] };
}

/** 🏷️Resolves only a declared finite parent set, with no recursive aliases or inferred parents. */
export function parseNamedFixedDirectoryContractSetScope(input: unknown, parents: Readonly<Record<string, FixedDirectoryContract>>, sets: Readonly<Record<string, readonly string[]>>): Extract<FixedContractScope, { kind: "fixed-directory-contract-set" }> {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Named fixed parent scope must be an object");
  const row = input as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "fixedDirectoryContractSetId,kind" || row.kind !== "named-fixed-directory-contract-set" || typeof row.fixedDirectoryContractSetId !== "string" || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(row.fixedDirectoryContractSetId) || !Object.hasOwn(sets, row.fixedDirectoryContractSetId)) throw new Error("Named fixed parent scope requires one declared set ID");
  return parseFixedDirectoryContractSetScope({ kind: "fixed-directory-contract-set", fixedDirectoryContractIds: sets[row.fixedDirectoryContractSetId] }, parents);
}

function fixedScopeMatches(contract: FixedFilenameContract | FixedDirectoryContract, path: string, context: FixedContractMatchContext, sets: Readonly<Record<string, readonly string[]>> = {}): boolean {
  if (contract.scope.kind === "exact-path") return path === contract.scope.path;
  if (contract.scope.kind === "repository-root") return !path.includes("/");
  if (contract.scope.kind === "package-root") return context.packageRoot === true && context.ecosystemId === contract.scope.ecosystemId;
  if (contract.scope.kind === "directory-kind") return context.parentDirectoryKindId === contract.scope.directoryKindId;
  if (contract.scope.kind === "fixed-directory-contract") return context.parentFixedDirectoryContractIds?.includes(contract.scope.fixedDirectoryContractId) === true;
  if (contract.scope.kind === "fixed-directory-contract-set") return contract.scope.fixedDirectoryContractIds.some((id) => context.parentFixedDirectoryContractIds?.includes(id));
  if (contract.scope.kind === "named-fixed-directory-contract-set") return sets[contract.scope.fixedDirectoryContractSetId]?.some((id) => context.parentFixedDirectoryContractIds?.includes(id)) === true;
  if (contract.scope.kind === "sibling-fixed-filename-contract") return context.siblingFixedFilenameContractIds?.includes(contract.scope.fixedFilenameContractId) === true;
  return true;
}

/** 📄️ Extracts the exact literal basename owned by a fixed filename contract. */
export function fixedContractFilename(contract: FixedFilenameContract): string {
  return contract.pathPattern.slice(contract.pathPattern.lastIndexOf("/") + 1);
}

/** 📏️ Comparable specificity tuple for deterministic fixed-contract precedence. */
export function fixedContractSpecificity(contract: FixedFilenameContract | FixedDirectoryContract): readonly [number, number, number, number] {
  const wildcardTokens = contract.pathPattern.match(/\*\*|\*|\?|\[[^\]]+\]/gu) ?? [];
  const literalSegments = contract.pathPattern.split("/").filter((segment) => !/\*|\?|\[/u.test(segment)).length;
  const literalCodePoints = [...contract.pathPattern.replace(/\*\*|\*|\?|\[[^\]]+\]|\//gu, "")].length;
  const scopeStrength = { "path-pattern": 0, "directory-kind": 1, "package-root": 2, "fixed-directory-contract": 3, "fixed-directory-contract-set": 3, "named-fixed-directory-contract-set": 3, "sibling-fixed-filename-contract": 3, "repository-root": 4, "exact-path": 5 }[contract.scope.kind];
  return [literalSegments, literalCodePoints, -wildcardTokens.length, scopeStrength];
}

function compareFixedContracts(left: readonly [string, FixedFilenameContract | FixedDirectoryContract], right: readonly [string, FixedFilenameContract | FixedDirectoryContract]): number {
  const leftScore = fixedContractSpecificity(left[1]);
  const rightScore = fixedContractSpecificity(right[1]);
  for (let index = 0; index < leftScore.length; index += 1) if (leftScore[index] !== rightScore[index]) return rightScore[index]! - leftScore[index]!;
  return left[0].localeCompare(right[0]);
}

function fixedContractWinner<T extends FixedFilenameContract | FixedDirectoryContract>(kind: "filename" | "directory", matches: readonly (readonly [string, T])[]): string[] {
  if (matches.length === 0) return [];
  const ordered = [...matches].sort(compareFixedContracts);
  if (ordered.length > 1 && fixedContractSpecificity(ordered[0]![1]).every((score, index) => score === fixedContractSpecificity(ordered[1]![1])[index])) {
    throw new Error(`Path resolves to equal-specificity fixed ${kind} contracts ${JSON.stringify(ordered[0]![0])} and ${JSON.stringify(ordered[1]![0])}.`);
  }
  return [ordered[0]![0]];
}

/** 🔒️ Resolves the single deterministic fixed filename winner. */
export function fixedFilenameContractIdsForPath(path: string, taxonomy: Taxonomy = loadTaxonomy(), context: FixedContractMatchContext = {}): string[] {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  return fixedContractWinner("filename", Object.entries(taxonomy.fixedFilenameContracts)
    .filter(([, contract]) => (context.pathMatcher?.matches(normalized, contract.pathPattern) ?? taxonomyPathPatternMatches(normalized, contract.pathPattern)) && fixedScopeMatches(contract, normalized, context, taxonomy.fixedDirectoryContractSets)));
}

/** 📁️ Resolves the single deterministic fixed directory winner. */
export function fixedDirectoryContractIdsForPath(path: string, taxonomy: Taxonomy = loadTaxonomy(), context: FixedContractMatchContext = {}): string[] {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/$/u, "").normalize(taxonomy.unicodeNormalization.form);
  return fixedContractWinner("directory", Object.entries(taxonomy.fixedDirectoryContracts)
    .filter(([, contract]) => (context.pathMatcher?.matches(normalized, contract.pathPattern) ?? taxonomyPathPatternMatches(normalized, contract.pathPattern)) && fixedScopeMatches(contract, normalized, context, taxonomy.fixedDirectoryContractSets)));
}

/** 🧵️Invocation-owned fixed-contract resolver that indexes literal basenames and compiles each glob once. */
export function createFixedContractResolver(taxonomy: Taxonomy = loadTaxonomy()): Readonly<{
  filenameIdsForPath: (path: string, context?: FixedContractMatchContext) => string[];
  directoryIdsForPath: (path: string, context?: FixedContractMatchContext) => string[];
}> {
  const matcher = createTaxonomyPathMatcher();
  const index = <T extends FixedFilenameContract | FixedDirectoryContract>(contracts: Readonly<Record<string, T>>) => {
    const literal = new Map<string, (readonly [string, T])[]>(), wildcard: (readonly [string, T])[] = [];
    for (const entry of Object.entries(contracts)) {
      const name = entry[1].pathPattern.slice(entry[1].pathPattern.lastIndexOf("/") + 1);
      if (/[?*\[]/u.test(name)) wildcard.push(entry);
      else literal.set(name, [...(literal.get(name) ?? []), entry]);
    }
    return (path: string, context: FixedContractMatchContext, kind: "filename" | "directory"): string[] => {
      const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(kind === "directory" ? /\/$/u : /$^/u, "").normalize(taxonomy.unicodeNormalization.form);
      const name = normalized.slice(normalized.lastIndexOf("/") + 1);
      const candidates = [...(literal.get(name) ?? []), ...wildcard];
      return fixedContractWinner(kind, candidates.filter(([, contract]) => matcher.matches(normalized, contract.pathPattern) && fixedScopeMatches(contract, normalized, context, taxonomy.fixedDirectoryContractSets)));
    };
  };
  const filenames = index(taxonomy.fixedFilenameContracts), directories = index(taxonomy.fixedDirectoryContracts);
  return Object.freeze({
    filenameIdsForPath: (path: string, context: FixedContractMatchContext = {}) => filenames(path, context, "filename"),
    directoryIdsForPath: (path: string, context: FixedContractMatchContext = {}) => directories(path, context, "directory"),
  });
}

/** 🚫️ Resolves an exact fixed-looking path to its schema-owned normalize/relocate decision. */
export function fixedFilenameRejectionContractIdForPath(path: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  const matches = Object.entries(taxonomy.fixedFilenameRejectionContracts).filter(([, contract]) => contract.sourcePathIdentities.includes(normalized));
  if (matches.length > 1) throw new Error(`Path resolves to multiple fixed filename rejection contracts: ${matches.map(([id]) => id).join(", ")}.`);
  return matches[0]?.[0] ?? null;
}

function exactContractFilename(contractId: string | null, taxonomy: Taxonomy): string | null {
  if (!contractId) return null;
  const fixed = taxonomy.fixedFilenameContracts[contractId];
  return fixed ? fixedContractFilename(fixed) : taxonomy.configurableEntryContracts[contractId]?.filename ?? null;
}

function componentFilenames(taxonomy: Taxonomy): string[] {
  return [...new Set(Object.values(taxonomy.componentFileKinds).flatMap((kindId) => canonicalFilenamesForKind(kindId, taxonomy)))];
}

/** 🧬️ Resolves a schema facet's kind from its normative leaf on disk (`schemaFacetKinds`). */
export function resolveSchemaFacetKind(repoRoot: string, facetRel: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const facetAbs = join(repoRoot, facetRel);
  if (pathIsExcluded(repoRoot, facetAbs, taxonomy)) return null;
  if (!existsSync(facetAbs)) return null;
  for (const [kindId, kind] of Object.entries(taxonomy.schemaFacetKinds ?? {})) {
    const normative = taxonomy.schemaFormats[kind.normativeFormat];
    if (normative && canonicalFilenamesForKind(normative.fileKindId, taxonomy).some((filename) => existsSync(join(facetAbs, filename)))) return kindId;
  }
  return null;
}

/** 📜️ Returns the schemaFormats entries required for one facet path (kind-selected subset). */
export function schemaFacetFormatEntries(repoRoot: string, facetRel: string, taxonomy: Taxonomy = loadTaxonomy()): [string, SchemaFormatSpec][] {
  const kindId = resolveSchemaFacetKind(repoRoot, facetRel, taxonomy);
  const kind = kindId ? taxonomy.schemaFacetKinds?.[kindId] : taxonomy.schemaFacetKinds?.["🧬️data"];
  if (!kind) return Object.entries(taxonomy.schemaFormats ?? {}) as [string, SchemaFormatSpec][];
  return kind.formats
    .map((formatId) => [formatId, taxonomy.schemaFormats[formatId]] as [string, SchemaFormatSpec | undefined])
    .filter((entry): entry is [string, SchemaFormatSpec] => entry[1] !== undefined);
}

/** 📦️ Allowed semantic directory kinds inside one language package. */
export function packagingDirectoryKindIdsForLang(lang: string, taxonomy: Taxonomy = loadTaxonomy()): readonly string[] {
  const global = taxonomy.packagingDirectoryKindIds ?? [];
  const ecosystem = taxonomy.ecosystems[lang]?.packagingDirectoryKindIds ?? [];
  return [...new Set([...global, ...ecosystem])];
}

/** 🌳️ Level descriptor: fixed allowlist or wildcard (`*` = any emoji-prefixed slug dir). */
type ArtifactFacetLevel =
  | { readonly kind: "fixed"; readonly dirs: readonly string[] }
  | { readonly kind: "wildcard" }
  | { readonly kind: "none" };

/**
 * 🂡 Whether a dir name is an emoji-prefixed slug — an Extended_Pictographic codepoint at the
 * start with the canonical selector and resolve to exactly one semantic-directory registry entry.
 */
function isEmojiPrefixedSlugDir(name: string, taxonomy: Taxonomy): boolean {
  return semanticDirectoryKindId(name, taxonomy) !== null;
}

/** 🏘️ Validates explicitly authored two-tier mutation owners without assigning names or traversing files. */
export function mutationDomainOwnersProblems(owners: Taxonomy["mutationDomainOwners"], genericEmojiIdentities: readonly string[] = []): string[] {
  const problems: string[] = [];
  if (!owners || typeof owners !== "object" || Array.isArray(owners)) return ["mutationDomainOwners must be an exact-owner object."];
  for (const [root, domains] of Object.entries(owners)) {
    if (!root.endsWith("/🧬️mutations") || root !== root.normalize("NFC") || /[*?{}]/u.test(root) || root.includes(String.fromCharCode(92)) || [...root].some((character) => character.charCodeAt(0) < 32) || root.split("/").some((part) => !part || part === "." || part === "..")) problems.push(`mutationDomainOwners[${JSON.stringify(root)}] must name one exact repository mutation root.`);
    if (!domains || typeof domains !== "object" || Array.isArray(domains) || Object.keys(domains).length === 0) {
      problems.push(`mutationDomainOwners[${JSON.stringify(root)}] must declare non-empty domains.`);
      continue;
    }
    const identities = new Set<string>();
    const entries: { path: string; nodeKind: "directory" }[] = [];
    for (const [domain, operations] of Object.entries(domains)) {
      const domainId = leadingEmojiIdentity(domain).rest;
      entries.push({ path: `${root}/${domain}`, nodeKind: "directory" });
      if (domain.includes("/") || domain.includes(String.fromCharCode(92)) || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(domainId)) problems.push(`${root}/${domain} is not one domain basename.`);
      if (!operations || typeof operations !== "object" || Array.isArray(operations) || Object.keys(operations).length === 0) {
        problems.push(`${root}/${domain} must declare non-empty operations.`);
        continue;
      }
      for (const [operation, identity] of Object.entries(operations)) {
        const operationId = leadingEmojiIdentity(operation).rest;
        entries.push({ path: `${root}/${domain}/${operation}`, nodeKind: "directory" });
        if (operation.includes("/") || operation.includes(String.fromCharCode(92)) || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(operationId) || typeof identity !== "string" || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/u.test(identity)) problems.push(`${root}/${domain}/${operation} must declare one operation basename and its explicit full semantic identity.`);
        if (identities.has(identity)) problems.push(`${root} has duplicate semantic identity ${JSON.stringify(identity)}.`);
        identities.add(identity);
      }
    }
    for (const finding of pathEmojiStatuteFindings(entries, genericEmojiIdentities)) problems.push(`${finding.path} violates mutation-domain ${finding.kind} emoji identity.`);
  }
  return problems;
}

/** 🛂️ Validates direct source ownership without cross-artifact, cross-standard, chained, or inferred ownership. */
export function mutationCatalogSourceOwnersProblems(taxonomy: Pick<Taxonomy, "mutationCatalogSourceOwners" | "mutationDomainOwners">): string[] {
  const owners = taxonomy.mutationCatalogSourceOwners, problems: string[] = [];
  if (!owners || typeof owners !== "object" || Array.isArray(owners)) return ["mutationCatalogSourceOwners must be an exact-owner object."];
  const profile = (path: unknown): RegExpMatchArray | null => typeof path === "string" && path === path.normalize("NFC") && !/[\\\\:*?{}\u0000-\u001F]/u.test(path) && path.split("/").every((part) => part && part !== "." && part !== "..") ? path.match(/^(.+)\/🏅️standards\/(🔖️[^/]+)\/🪆️subsets\/([^/]+)$/u) : null;
  for (const [owner, source] of Object.entries(owners)) {
    const catalogProfile = profile(owner), sourceProfile = profile(source);
    if (!catalogProfile || !sourceProfile || catalogProfile[1] !== sourceProfile[1] || catalogProfile[2] !== sourceProfile[2] || owner === source || Object.hasOwn(owners, source) || !Object.hasOwn(taxonomy.mutationDomainOwners, `${source}/🧬️schema/🧬️mutations`)) problems.push(`mutationCatalogSourceOwners[${JSON.stringify(owner)}] must reference one distinct registered source subset in the same artifact and standard without ownership chains.`);
  }
  return problems;
}

/** 🧭️ Resolves only an exact declared catalog source; unmapped owners retain their own current source tree. */
export function mutationCatalogSourceOwner(owner: string, taxonomy: Pick<Taxonomy, "mutationCatalogSourceOwners" | "mutationDomainOwners"> = loadTaxonomy()): string | null {
  if (!Object.hasOwn(taxonomy.mutationCatalogSourceOwners, owner)) return owner;
  return mutationCatalogSourceOwnersProblems(taxonomy).length === 0 ? taxonomy.mutationCatalogSourceOwners[owner]! : null;
}

/** 🪪️ Resolves a physical mutation owner only through its declared exact layout. */
export function mutationOwnerIdentity(root: string, ownerPath: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  if (ownerPath !== ownerPath.normalize("NFC") || ownerPath.includes(String.fromCharCode(92))) return null;
  const domains = taxonomy.mutationDomainOwners[root];
  if (domains) {
    const [domain, operation, extra] = ownerPath.split("/");
    return extra === undefined && domain && operation && Object.hasOwn(domains, domain) && Object.hasOwn(domains[domain]!, operation) ? domains[domain]![operation]! : null;
  }
  return !ownerPath.includes("/") && mutationDirectoryNameIsValid(ownerPath, taxonomy) ? ownerPath.match(/[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/u)?.[0] ?? null : null;
}

/** 🗺️ Resolves one semantic ID to its explicitly registered domain and operation path. */
export function mutationOwnerRelativePath(root: string, identity: string, taxonomy: Pick<Taxonomy, "mutationDomainOwners"> = loadTaxonomy()): string | null {
  const paths = Object.entries(taxonomy.mutationDomainOwners[root] ?? {}).flatMap(([domain, operations]) => Object.entries(operations).filter(([, value]) => value === identity).map(([operation]) => `${domain}/${operation}`));
  return paths.length === 1 ? paths[0]! : null;
}

/** 🧬️ Whether one direct mutation owner has the configured emoji + semantic verb-noun identity. */
export function mutationDirectoryNameIsValid(name: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  return name === name.normalize("NFC") && new RegExp(taxonomy.mutationDirectoryPattern, "u").test(name);
}

/** 🏗️ Free top-level `fn <name>(` declarations (Rust's structural shape for an extracted diff/inverse body) keyed by the behavior facet they belong to. A trait-impl method (`async fn diff(&self, ...) { diff(self, base) }`, indented inside `impl … {}`) never matches — only a column-zero free function does, which is exactly the shape the collapsed leaves reduce to once split-out logic gets folded back in. */
const MUTATION_DIRECT_LEAF_STRUCTURAL_PATTERNS: Readonly<Record<string, RegExp>> = {
  "🔺️diff": /^pub (?:async )?fn diff\(/mu,
  "↩️inverse": /^pub (?:async )?fn inverse\(/mu,
};

/**
 * 🔖️🏗️ Behavior facets (`🔺️diff`/`↩️inverse`) that a mutation's direct leaf re-inlines instead of placing
 * in their own facet directory — the exact regression the SEMANTIC-MUTATIONS-OVERHAUL contract forbids.
 *
 * STRUCTURAL detection is the actual contract: a free `pub (async)? fn diff(`/`fn inverse(` in the direct
 * leaf with no sibling facet directory present is a violation regardless of comments — an author who
 * simply omits a `//#region` marker cannot evade this (confirmed at 266-mutation scale in `🏛️architect`,
 * where every leaf was marker-free). `siblingFacetDirs` must be the actual child directory names observed
 * beside the direct leaf on disk; pass an empty set only when no sibling facets exist (e.g. an inline fixture).
 *
 * The `mutationDirectLeafForbiddenRegionMarkers` marker scan still runs, unioned in: it is the only signal
 * for a non-Rust direct leaf (e.g. a `🟦️.ts` mirror, where "free `pub fn diff(`" has no meaning) and for any
 * Rust shape that carries the marker without matching the bare free-function form. It adds coverage the
 * structural check cannot reach; it is not a substitute for it.
 *
 * Empty means the direct leaf is clean.
 */
export function mutationDirectLeafInlinedBehaviorFacets(directLeafSource: string, siblingFacetDirs: ReadonlySet<string> = new Set(), taxonomy: Taxonomy = loadTaxonomy()): readonly string[] {
  const found = new Set<string>();
  const markers = taxonomy.mutationDirectLeafForbiddenRegionMarkers ?? {};
  for (const [facet, marker] of Object.entries(markers)) {
    if (new RegExp(`//#region\\s+${marker.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}\\b`, "u").test(directLeafSource)) found.add(facet);
  }
  for (const [facet, pattern] of Object.entries(MUTATION_DIRECT_LEAF_STRUCTURAL_PATTERNS)) {
    if (pattern.test(directLeafSource) && !siblingFacetDirs.has(facet)) found.add(facet);
  }
  return [...found].sort();
}

/** 🌳️ Declared child level of a path under an artifact root (parents are `/`-segments already accepted). */
function artifactFacetChildLevel(parents: readonly string[], taxonomy: Taxonomy): ArtifactFacetLevel {
  if (parents.length === 0) return { kind: "fixed", dirs: taxonomy.artifactComponentDirs };
  const root = parents[0]!;
  const a = parents[1];
  const b = parents[2];
  const c = parents[3];
  if (parents.length === 1) {
    if (root === "🧬️schema") return { kind: "fixed", dirs: taxonomy.schemaChildDirs ?? [] };
    if (root === "🚪️io") return { kind: "fixed", dirs: [...(taxonomy.ioDirectionDirs ?? []), ...(taxonomy.ioSemanticCollectionDirNames ?? [])] };
    return { kind: "none" };
  }
  if (root === "🧬️schema") {
    if (parents.length === 2 && (taxonomy.schemaChildDirs ?? []).includes(a!)) {
      if (a === "🧬️mutations") return { kind: "fixed", dirs: ["*"] };
      if (a === "💡️inferences") return { kind: "fixed", dirs: ["*"] };
      return { kind: "fixed", dirs: taxonomy.representationDirs ?? [] };
    }
    if (parents.length === 3 && a === "🧬️mutations") {
      if ((taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
      return { kind: "fixed", dirs: [...(taxonomy.mutationBehaviorFacetDirs ?? []), ...(taxonomy.mutationOrganizationalFacetDirs ?? [])] };
    }
    if (parents.length === 3 && a === "💡️inferences") return { kind: "none" };
    if (parents.length === 3 && (taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
    if (parents.length === 4 && a === "🧬️mutations") return { kind: "none" };
    return { kind: "none" };
  }
  if (root === "🚪️io") {
    const directions = taxonomy.ioDirectionDirs ?? [];
    const childMap = taxonomy.ioDirectionChildDirs ?? {};
    if (parents.length === 2 && (taxonomy.ioSemanticCollectionDirNames ?? []).includes(a!)) return { kind: "fixed", dirs: taxonomy.representationDirs ?? [] };
    if (parents.length === 3 && (taxonomy.ioSemanticCollectionDirNames ?? []).includes(a!) && (taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
    if (parents.length === 2 && directions.includes(a!)) {
      const child = childMap[a!];
      return child ? { kind: "fixed", dirs: [child] } : { kind: "none" };
    }
    if (parents.length === 3 && directions.includes(a!) && childMap[a!] === b) {
      return { kind: "fixed", dirs: [taxonomy.artifactsDirName] };
    }
    if (parents.length === 4 && b === childMap[a!] && c === taxonomy.artifactsDirName) return { kind: "wildcard" };
    if (parents.length === 5) return { kind: "none" };
    return { kind: "none" };
  }
  return { kind: "none" };
}

/** 🌳️ Declared children of a nesting artifact facet path (`/`-joined parents), empty when leaves-only. */
function artifactFacetChildDirs(facetPath: string, taxonomy: Taxonomy): readonly string[] {
  const parents = facetPath ? facetPath.split("/") : [];
  const level = artifactFacetChildLevel(parents, taxonomy);
  if (level.kind !== "fixed") return [];
  return level.dirs.filter((d) => d !== "*");
}

/** 🧭️ Whether a `/`-joined facet path walks only declared dirs from an artifact root (supports `*` wildcard levels). */
export function artifactFacetPathIsDeclared(facetPath: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const [root, ...rest] = facetPath.split("/");
  if (!root || !taxonomy.artifactComponentDirs.includes(root)) return false;
  const parents: string[] = [root];
  for (const segment of rest) {
    if (parents.length === 2 && parents[0] === "🧬️schema" && (parents[1] === "💡️inferences" || parents[1] === "🧬️mutations") && (taxonomy.representationDirs ?? []).includes(segment)) return false;
    const level = artifactFacetChildLevel(parents, taxonomy);
    if (level.kind === "none") return false;
    const directMutationOwner = parents.length === 2 && parents[0] === "🧬️schema" && parents[1] === "🧬️mutations";
    const wildcardAccepted = directMutationOwner ? mutationDirectoryNameIsValid(segment, taxonomy) : isEmojiPrefixedSlugDir(segment, taxonomy);
    if (level.kind === "wildcard") {
      if (!wildcardAccepted) return false;
    } else {
      const dirs = level.dirs;
      const allowWildcard = dirs.includes("*");
      const fixed = dirs.filter((d) => d !== "*");
      if (!(fixed.includes(segment) || (allowWildcard && wildcardAccepted))) return false;
    }
    parents.push(segment);
  }
  return true;
}

/**
 * 🚦️ Internal-consistency audit of the vocabulary itself: the completeness/structural artifact lists must
 * stay in their superset relation, direct mutation ownership must name one Rust component kind and only optional facets,
 * every registry, kind mapping, exact contract, package grammar, and clean area must agree. Returns human
 * readable problems (empty = healthy) so a vocabulary edit can never silently blind or flood the rules that
 * consume it.
 */
export function validateTaxonomy(taxonomy: Taxonomy = readTaxonomyUnchecked()): string[] {
  const problems: string[] = [];
  const pathMatcher = createTaxonomyPathMatcher();
  const document = taxonomy as unknown as Record<string, unknown>;
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

  const record = (value: unknown, key: string): value is Record<string, unknown> => {
    const valid = typeof value === "object" && value !== null && !Array.isArray(value);
    if (!valid || Object.keys(value as Record<string, unknown>).length === 0) problems.push(`${key} must be a non-empty object.`);
    return valid;
  };
  if (record(taxonomy.physicalLeafRendering, "physicalLeafRendering")) {
    const expected = { direction: "forward-only", filename: "file-kind-emoji-and-extension-chain", sourceExtension: "longest-registered-chain", authoringExtension: "schema-ordered-primary", runtimeLookup: "canonical-only" };
    if (Object.keys(taxonomy.physicalLeafRendering).sort().join("\0") !== Object.keys(expected).sort().join("\0") || Object.entries(expected).some(([key, value]) => (taxonomy.physicalLeafRendering as unknown as Record<string, unknown>)[key] !== value)) problems.push("physicalLeafRendering must declare exact forward-only kind-only rendering, longest source extension, primary authoring extension, and canonical-only runtime lookup.");
  }
  if (record(taxonomy.referenceClosure, "referenceClosure")) {
    const expected = { scope: "repository-incoming-and-moved-outgoing", candidateSource: "git-tracked-and-untracked-plus-explicit-ticket", candidateAdmission: "opaque-first-no-follow", coordinateRoots: "verified-repository-ownership", unsupportedPathBearingForms: "error", frozenSourceCoordinates: "exact-digest-and-token-authority", frozenPlanCoordinates: "canonical-schema-v2-digest-and-typed-token-authority", preimageDrift: "reject", newIncomingReferences: "reject-or-rollback", ordering: "utf8-byte", historicalDocumentEvidence: "ticket-report-workspace-cursor-plan-snapshot-and-dev-prompt-log-whole-document-excluded" };
    if (Object.keys(taxonomy.referenceClosure).sort().join("\0") !== Object.keys(expected).sort().join("\0") || Object.entries(expected).some(([key, value]) => (taxonomy.referenceClosure as unknown as Record<string, unknown>)[key] !== value)) problems.push("referenceClosure must declare repository-wide incoming and moved outgoing closure with opaque-first no-follow candidates, exact frozen coordinate authority, drift rejection, and byte ordering.");
  }
  const ids = (values: readonly string[] | undefined, registry: Readonly<Record<string, unknown>>, key: string): void => {
    if (!Array.isArray(values)) {
      problems.push(`${key} must be an array.`);
      return;
    }
    const seen = new Set<string>();
    for (const id of values) {
      if (seen.has(id)) problems.push(`${key} contains duplicate id ${JSON.stringify(id)}.`);
      seen.add(id);
      if (!(id in registry)) problems.push(`${key} references missing id ${JSON.stringify(id)}.`);
    }
  };
  const pattern = (value: string, key: string): void => {
    try {
      void new RegExp(value, "u");
    } catch {
      problems.push(`${key} is not a valid Unicode regular expression.`);
    }
  };
  const fullPattern = (value: unknown, key: string): void => {
    if (typeof value !== "string") {
      problems.push(`${key} must be a string.`);
      return;
    }
    pattern(value, key);
    if (!value.startsWith("^") || !value.endsWith("$")) problems.push(`${key} must be an anchored full-match pattern.`);
  };
  const pathPattern = (value: unknown, key: string): void => {
    if (typeof value !== "string" || !value || value !== value.normalize("NFC") || value.startsWith("/") || value.includes("\\") || /[{}!]/u.test(value) || /(^|\/)\*\*[^/]|[^/]\*\*(\/|$)/u.test(value)) {
      problems.push(`${key} must be one NFC workspace-relative v7 path pattern.`);
      return;
    }
    try {
      void pathMatcher.matches("", value);
    } catch {
      problems.push(`${key} is not a valid v7 path pattern.`);
    }
  };
  const workspacePath = (value: unknown, key: string): value is string => {
    const valid = typeof value === "string" && value.length > 0 && value === value.normalize("NFC") && !value.startsWith("/") && !value.endsWith("/") && !value.includes("\\")
      && !/[*?\[\]{}!]/u.test(value) && value.split("/").every((segment) => segment.length > 0 && segment !== "." && segment !== "..");
    if (!valid) problems.push(`${key} must be one exact NFC workspace-relative path.`);
    return valid;
  };
  const extensionChain = (value: unknown, key: string): void => {
    if (typeof value !== "string" || !/^\.[a-z0-9]+(?:[.-][a-z0-9]+)*$/u.test(value)) problems.push(`${key} must be one lowercase dot-prefixed extension chain.`);
  };
  const exactKeys = (value: object, allowed: readonly string[], key: string): void => {
    const actual = Object.keys(value).sort();
    const expected = [...allowed].sort();
    if (actual.join("\0") !== expected.join("\0")) problems.push(`${key} must contain exactly ${expected.join(", ")}.`);
  };
  const kebabId = (value: string, key: string): void => {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(value)) problems.push(`${key} must be kebab-case.`);
  };

  if (record(taxonomy.fileKinds, "fileKinds")) {
    const canonical = new Map<string, string>();
    const extensionOwners = new Map<string, string>();
    for (const [id, spec] of Object.entries(taxonomy.fileKinds)) {
      if (!canonicalTaxonomyEmoji(spec.emoji)) problems.push(`fileKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence.`);
      if (!Array.isArray(spec.extensionChains) || spec.extensionChains.length === 0) problems.push(`fileKinds[${JSON.stringify(id)}].extensionChains must be non-empty.`);
      for (const extension of spec.extensionChains ?? []) {
        if (!/^\.[a-z0-9]+(?:[.-][a-z0-9]+)*$/u.test(extension)) problems.push(`fileKinds[${JSON.stringify(id)}] has invalid extension chain ${JSON.stringify(extension)}.`);
        const extensionOwner = extensionOwners.get(extension);
        if (extensionOwner) problems.push(`fileKinds ${JSON.stringify(extensionOwner)} and ${JSON.stringify(id)} both own physical extension ${JSON.stringify(extension)}.`);
        extensionOwners.set(extension, id);
        const filename = `${spec.emoji}${extension}`;
        const prior = canonical.get(filename);
        if (prior) problems.push(`fileKinds ${JSON.stringify(prior)} and ${JSON.stringify(id)} collide at ${JSON.stringify(filename)}.`);
        canonical.set(filename, id);
      }
    }
    for (const [id, extension] of [["png", ".png"], ["bmp", ".bmp"]] as const) {
      const kind = taxonomy.fileKinds[id];
      if (!kind || kind.role !== "asset" || kind.emoji !== "🖼️" || kind.extensionChains.join("\0") !== extension) problems.push(`fileKinds.${id} must be the canonical 🖼️${extension} asset kind.`);
    }
  }

  if (record(taxonomy.fileKindResolutionRules, "fileKindResolutionRules")) for (const [id, rule] of Object.entries(taxonomy.fileKindResolutionRules)) {
    extensionChain(rule.extensionChain, `fileKindResolutionRules[${JSON.stringify(id)}].extensionChain`);
    const kind = taxonomy.fileKinds[rule.fileKindId];
    if (!kind || !kind.extensionChains.includes(rule.extensionChain)) problems.push(`fileKindResolutionRules[${JSON.stringify(id)}] must reference a kind owning its extension chain.`);
    if (rule.priority !== 0) problems.push(`fileKindResolutionRules[${JSON.stringify(id)}].priority must be zero for physical resolution.`);
    for (const removed of ["filenamePattern", "pathPattern", "parentKindIds", "ancestorKindIds"]) if (removed in rule) problems.push(`fileKindResolutionRules[${JSON.stringify(id)}].${removed} is forbidden; directories own semantics.`);
  }
  for (const [fileKindId, kind] of Object.entries(taxonomy.fileKinds)) for (const extension of kind.extensionChains) {
    const rules = Object.values(taxonomy.fileKindResolutionRules).filter((rule) => rule.extensionChain === extension && rule.fileKindId === fileKindId);
    if (rules.length !== 1) problems.push(`fileKindResolutionRules must own ${JSON.stringify(extension)} exactly once for ${JSON.stringify(fileKindId)}.`);
  }

  if (!(typeof taxonomy.scopedFileKinds === "object" && taxonomy.scopedFileKinds !== null && !Array.isArray(taxonomy.scopedFileKinds))) problems.push("scopedFileKinds must be an object.");
  else for (const [id, spec] of Object.entries(taxonomy.scopedFileKinds)) {
    pathPattern(spec.pathPattern, `scopedFileKinds[${JSON.stringify(id)}].pathPattern`);
    if (spec.parentDirectoryKindId !== undefined && !taxonomy.semanticDirectoryKinds[spec.parentDirectoryKindId]) problems.push(`scopedFileKinds[${JSON.stringify(id)}].parentDirectoryKindId must reference a semantic directory kind.`);
    if (!canonicalTaxonomyEmoji(spec.emoji)) problems.push(`scopedFileKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence.`);
    if (!Array.isArray(spec.extensionChains) || spec.extensionChains.length === 0) problems.push(`scopedFileKinds[${JSON.stringify(id)}].extensionChains must be non-empty.`);
    for (const extension of spec.extensionChains ?? []) extensionChain(extension, `scopedFileKinds[${JSON.stringify(id)}].extensionChains`);
    if (spec.role !== "evidence") problems.push(`scopedFileKinds[${JSON.stringify(id)}].role must be evidence.`);
    fullPattern(spec.sourceFilenamePattern, `scopedFileKinds[${JSON.stringify(id)}].sourceFilenamePattern`);
    if (!spec.authority || !spec.reason || !spec.verification) problems.push(`scopedFileKinds[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
    if (!(spec.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(spec.expires))) problems.push(`scopedFileKinds[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
  }

  if (record(taxonomy.semanticDirectoryKinds, "semanticDirectoryKinds")) for (const [id, spec] of Object.entries(taxonomy.semanticDirectoryKinds)) {
    if (!canonicalTaxonomyEmoji(spec.emoji)) problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence.`);
    if (typeof spec.slugPattern !== "string") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].slugPattern must be a string.`);
    else pattern(spec.slugPattern, `semanticDirectoryKinds[${JSON.stringify(id)}].slugPattern`);
    if (typeof spec.allowEmojiOnly !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].allowEmojiOnly must be boolean.`);
    if (spec.inferWithoutEmoji !== undefined && typeof spec.inferWithoutEmoji !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].inferWithoutEmoji must be boolean when present.`);
    if (spec.projectionOnly !== undefined && typeof spec.projectionOnly !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].projectionOnly must be boolean when present.`);
    ids(spec.parentKindIds ?? [], { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds, ...taxonomy.fixedDirectoryContracts }, `semanticDirectoryKinds[${JSON.stringify(id)}].parentKindIds`);
  }
  const taxonomyCliArtifactDirectoryKinds: Readonly<Record<string, Readonly<{ name: string; emoji: string; slugPattern: string; parentKindIds?: readonly string[] }>>> = {
    "taxonomy-transaction": { name: "🧾️taxonomy-transaction", emoji: "🧾️", slugPattern: "^taxonomy-transaction$" },
    "transaction-digest": { name: `🔖️${"0".repeat(64)}`, emoji: "🔖️", slugPattern: "^[a-f0-9]{64}$", parentKindIds: ["taxonomy-transaction"] },
    "transaction-attempts": { name: "🔂️attempts", emoji: "🔂️", slugPattern: "^attempts$", parentKindIds: ["transaction-digest"] },
    "transaction-attempt": { name: "🔢️000001", emoji: "🔢️", slugPattern: "^[0-9]{6}$", parentKindIds: ["transaction-attempts"] },
    "transaction-stage": { name: "🚧️stage", emoji: "🚧️", slugPattern: "^stage$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-backup": { name: "💾️backup", emoji: "💾️", slugPattern: "^backup$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-lease": { name: "🔒️lease", emoji: "🔒️", slugPattern: "^lease$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-attempt-preparation": { name: "🚧️prepare-000001-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^prepare-[0-9]{6}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-attempts"] },
    "transaction-edit-preparation": { name: "🚧️edit-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^edit-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-stage"] },
    "transaction-edit-write-preparation": { name: "🚧️write-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-edit-preparation"] },
    "transaction-journal-write": { name: "🚧️journal", emoji: "🚧️", slugPattern: "^journal$", parentKindIds: ["transaction-stage"] },
    "transaction-json-write-preparation": { name: "🚧️write-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-journal-write", "transaction-lease-preparation", "transaction-lease"] },
    "transaction-lease-preparation": { name: "🚧️lease-42-123e4567-e89b-42d3-a456-426614174000-preparing", emoji: "🚧️", slugPattern: "^lease-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}-(preparing|stale)$", parentKindIds: ["transaction-backup"] },
    "transaction-backup-preparation": { name: "🚧️backup-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^backup-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup"] },
    "transaction-backup-write-preparation": { name: "🚧️write-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup-preparation"] },
    "transaction-restore-preparation": { name: "🚧️restore-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^restore-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup"] },
    "taxonomy-inventory-data": { name: "📊️taxonomy-inventory", emoji: "📊️", slugPattern: "^taxonomy-inventory$" },
    "taxonomy-plan-data": { name: "📊️taxonomy-plan", emoji: "📊️", slugPattern: "^taxonomy-plan$" },
    "taxonomy-apply-data": { name: "📊️taxonomy-apply", emoji: "📊️", slugPattern: "^taxonomy-apply$" },
    "taxonomy-verification-data": { name: "📊️taxonomy-verification", emoji: "📊️", slugPattern: "^taxonomy-verification$" },
    "taxonomy-inventory-summary": { name: "📓️taxonomy-inventory", emoji: "📓️", slugPattern: "^taxonomy-inventory$" },
    "taxonomy-plan-summary": { name: "📓️taxonomy-plan", emoji: "📓️", slugPattern: "^taxonomy-plan$" },
    "taxonomy-apply-summary": { name: "📓️taxonomy-apply", emoji: "📓️", slugPattern: "^taxonomy-apply$" },
    "taxonomy-verification-summary": { name: "📓️taxonomy-verification", emoji: "📓️", slugPattern: "^taxonomy-verification$" },
    "taxonomy-inventory-shards": { name: "📊️shards", emoji: "📊️", slugPattern: "^shards$", parentKindIds: ["taxonomy-inventory-data"] },
    "taxonomy-inventory-shard-digest": { name: `🔖️${"0".repeat(64)}`, emoji: "🔖️", slugPattern: "^[a-f0-9]{64}$", parentKindIds: ["taxonomy-inventory-shards"] },
  };
  for (const [id, expected] of Object.entries(taxonomyCliArtifactDirectoryKinds)) {
    const spec = taxonomy.semanticDirectoryKinds[id];
    if (!spec) {
      problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] must declare the permanent taxonomy CLI artifact directory.`);
      continue;
    }
    exactKeys(spec, expected.parentKindIds ? ["emoji", "slugPattern", "allowEmojiOnly", "parentKindIds"] : ["emoji", "slugPattern", "allowEmojiOnly"], `semanticDirectoryKinds[${JSON.stringify(id)}]`);
    if (spec.emoji !== expected.emoji || spec.slugPattern !== expected.slugPattern || spec.allowEmojiOnly || JSON.stringify(spec.parentKindIds) !== JSON.stringify(expected.parentKindIds)) problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] must remain the exact taxonomy CLI artifact contract.`);
    if (semanticDirectoryKindId(expected.name, taxonomy, { parentKindId: expected.parentKindIds?.[0] }) !== id) problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] does not resolve its canonical taxonomy CLI artifact directory uniquely.`);
  }
  const editCandidate = taxonomy.scopedFileKinds["transaction-edit-candidate"];
  if (!editCandidate || editCandidate.pathPattern !== "**/🚧️stage/🚧️edit-*/*.edit" || editCandidate.parentDirectoryKindId !== "transaction-edit-preparation" || editCandidate.extensionChains.join("\0") !== ".edit" || editCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.edit$") problems.push("scopedFileKinds.transaction-edit-candidate must remain the exact transaction edit candidate authority.");
  const editPreimage = taxonomy.scopedFileKinds["transaction-edit-preimage"];
  if (!editPreimage || editPreimage.pathPattern !== "**/🚧️stage/🚧️edit-*/*.pre" || editPreimage.parentDirectoryKindId !== "transaction-edit-preparation" || editPreimage.extensionChains.join("\0") !== ".pre" || editPreimage.sourceFilenamePattern !== "^[0-9a-f]{24}\\.pre$") problems.push("scopedFileKinds.transaction-edit-preimage must remain the exact transaction edit displaced-preimage authority.");
  const editWriteCandidate = taxonomy.scopedFileKinds["transaction-edit-write-candidate"];
  if (!editWriteCandidate || editWriteCandidate.pathPattern !== "**/🚧️stage/🚧️edit-*/🚧️write-*/🚧️.edit" || editWriteCandidate.parentDirectoryKindId !== "transaction-edit-write-preparation" || editWriteCandidate.extensionChains.join("\0") !== ".edit" || editWriteCandidate.sourceFilenamePattern !== "^🚧️\\.edit$") problems.push("scopedFileKinds.transaction-edit-write-candidate must remain the exact unpublished edit-writer authority.");
  const backupWriteCandidate = taxonomy.scopedFileKinds["transaction-backup-write-candidate"];
  if (!backupWriteCandidate || backupWriteCandidate.pathPattern !== "**/💾️backup/🚧️backup-*/🚧️write-*/🚧️.backup" || backupWriteCandidate.parentDirectoryKindId !== "transaction-backup-write-preparation" || backupWriteCandidate.extensionChains.join("\0") !== ".backup" || backupWriteCandidate.sourceFilenamePattern !== "^🚧️\\.backup$") problems.push("scopedFileKinds.transaction-backup-write-candidate must remain the exact unpublished backup-writer authority.");
  const jsonPrevious = taxonomy.scopedFileKinds["transaction-json-previous"];
  if (!jsonPrevious || jsonPrevious.pathPattern !== "**/🚧️write-*/⏮️.json" || jsonPrevious.parentDirectoryKindId !== "transaction-json-write-preparation" || jsonPrevious.extensionChains.join("\0") !== ".json" || jsonPrevious.sourceFilenamePattern !== "^⏮️\\.json$") problems.push("scopedFileKinds.transaction-json-previous must remain the exact displaced canonical JSON authority.");
  const backupCandidate = taxonomy.scopedFileKinds["transaction-backup-candidate"];
  if (!backupCandidate || backupCandidate.pathPattern !== "**/💾️backup/🚧️restore-*/*.backup" || backupCandidate.parentDirectoryKindId !== "transaction-restore-preparation" || backupCandidate.extensionChains.join("\0") !== ".backup" || backupCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.backup$") problems.push("scopedFileKinds.transaction-backup-candidate must remain the exact transaction restore preimage authority.");
  const postimageCandidate = taxonomy.scopedFileKinds["transaction-postimage-candidate"];
  if (!postimageCandidate || postimageCandidate.pathPattern !== "**/💾️backup/🚧️restore-*/*.post" || postimageCandidate.parentDirectoryKindId !== "transaction-restore-preparation" || postimageCandidate.extensionChains.join("\0") !== ".post" || postimageCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.post$") problems.push("scopedFileKinds.transaction-postimage-candidate must remain the exact transaction restore postimage authority.");
  if (semanticDirectoryKindId("🚧️write-42-123e4567-e89b-42d3-a456-426614174000", taxonomy, { parentKindId: "transaction-lease-preparation" }) !== "transaction-json-write-preparation" || semanticDirectoryKindId("🚧️write-42-123e4567-e89b-42d3-a456-426614174000", taxonomy, { parentKindId: "transaction-lease" }) !== "transaction-json-write-preparation") problems.push("semanticDirectoryKinds.transaction-json-write-preparation must resolve below all three exact parent kinds.");
  for (const [name, expectedKindId] of [["🚧️stage", "transaction-stage"], ["💾️backup", "transaction-backup"], ["🔒️lease", "transaction-lease"]] as const) if (semanticDirectoryKindId(name, taxonomy, { parentKindId: "transaction-attempt-preparation" }) !== expectedKindId) problems.push(`semanticDirectoryKinds.${expectedKindId} must resolve below transaction-attempt-preparation.`);
  const attemptPreparation = { parentKindId: "transaction-attempts", directoryName: "🚧️prepare-000001-42-123e4567-e89b-42d3-a456-426614174000", children: [{ name: "🚧️stage", nodeKind: "directory" }, { name: "💾️backup", nodeKind: "directory" }, { name: "🔒️lease", nodeKind: "directory" }, { name: "🔣️.json", nodeKind: "file" }] } as const;
  if (taxonomyCliAttemptPreparationsProblems([attemptPreparation], taxonomy).length > 0 || taxonomyCliAttemptPreparationsProblems([attemptPreparation, attemptPreparation], taxonomy).length === 0) problems.push("transaction-attempt-preparation must retain its validate-all exact child and duplicate-sibling authority.");
  const leasePreparation = { parentKindId: "transaction-backup", directoryName: "🚧️lease-42-123e4567-e89b-42d3-a456-426614174000-preparing", leafNames: ["🔣️.json"], writePreparations: [] } as const;
  if (taxonomyCliLeaseDirectoryProblems(leasePreparation, taxonomy).length > 0) problems.push("transaction-lease-preparation must admit one complete canonical JSON lease before publication.");

  if (record(taxonomy.semanticDirectoryMemberKinds, "semanticDirectoryMemberKinds")) {
    const allDirectoryKinds = { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds };
    const ownerMembers = new Set<string>();
    for (const [id, spec] of Object.entries(taxonomy.semanticDirectoryMemberKinds)) {
      ids(spec.ownerKindIds, allDirectoryKinds, `semanticDirectoryMemberKinds[${JSON.stringify(id)}].ownerKindIds`);
      if (!Array.isArray(spec.memberNames) || spec.memberNames.length === 0) problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}].memberNames must be non-empty.`);
      for (const name of spec.memberNames ?? []) {
        if (typeof name !== "string" || name !== name.normalize("NFC") || /[\\/]/u.test(name) || !canonicalTaxonomyEmoji(leadingEmojiIdentity(name).first)) problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}] has invalid exact member ${JSON.stringify(name)}.`);
        for (const owner of spec.ownerKindIds ?? []) {
          const key = `${owner}\0${name}`;
          if (ownerMembers.has(key)) problems.push(`semanticDirectoryMemberKinds collide for owner ${JSON.stringify(owner)} and member ${JSON.stringify(name)}.`);
          ownerMembers.add(key);
        }
      }
      if (spec.source !== "registry") problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}].source must be registry.`);
    }
  }

  //#region 🪞️SemanticPathProjection
  const projectionDirectoryKinds = { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds };
  if (record(taxonomy.semanticProjectedMemberKinds, "semanticProjectedMemberKinds")) {
    for (const [id, spec] of Object.entries(taxonomy.semanticProjectedMemberKinds)) {
      kebabId(id, `semanticProjectedMemberKinds id ${JSON.stringify(id)}`);
      exactKeys(spec, ["ownerKindIds", "projectionContractId", "sourceMemberKindId", "identityField"], `semanticProjectedMemberKinds[${JSON.stringify(id)}]`);
      ids(spec.ownerKindIds, projectionDirectoryKinds, `semanticProjectedMemberKinds[${JSON.stringify(id)}].ownerKindIds`);
      if (spec.ownerKindIds.length === 0) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].ownerKindIds must be non-empty.`);
      if (!taxonomy.semanticPathProjectionContracts[spec.projectionContractId]) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].projectionContractId is missing.`);
      if (!taxonomy.semanticDirectoryMemberKinds[spec.sourceMemberKindId]) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].sourceMemberKindId is missing.`);
      const expectedIdentityField = spec.projectionContractId === "artifact-mutation-tests-v1" ? "mutationDirectoryName" : spec.projectionContractId === "artifact-editor-command-bundle-v1" ? "commandDirectoryName" : null;
      if (spec.identityField !== expectedIdentityField) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].identityField does not match its projection contract.`);
    }
    const visiting = new Set<string>();
    const visited = new Set<string>();
    const visit = (id: string): void => {
      if (visiting.has(id)) {
        problems.push(`semanticProjectedMemberKinds contains an owner cycle at ${JSON.stringify(id)}.`);
        return;
      }
      if (visited.has(id)) return;
      visiting.add(id);
      for (const owner of taxonomy.semanticProjectedMemberKinds[id]?.ownerKindIds ?? []) if (taxonomy.semanticProjectedMemberKinds[owner]) visit(owner);
      visiting.delete(id);
      visited.add(id);
    };
    for (const id of Object.keys(taxonomy.semanticProjectedMemberKinds)) visit(id);
  }

  if (record(taxonomy.semanticPathProjectionProfileRenderers, "semanticPathProjectionProfileRenderers")) for (const [id, renderer] of Object.entries(taxonomy.semanticPathProjectionProfileRenderers)) {
    kebabId(id, `semanticPathProjectionProfileRenderers id ${JSON.stringify(id)}`);
    exactKeys(renderer, ["direction", "captureFields", "directoryKindId", "template", "tupleCollisionFields"], `semanticPathProjectionProfileRenderers[${JSON.stringify(id)}]`);
    if (renderer.direction !== "forward-only") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].direction must be forward-only.`);
    if (renderer.captureFields.join("\0") !== "standardVersion\0subsetId") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].captureFields must be exactly standardVersion, subsetId.`);
    if (!taxonomy.semanticDirectoryKinds[renderer.directoryKindId]) problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].directoryKindId is missing.`);
    if (renderer.template !== "🪆️{standardVersion}-{subsetId}") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].template must be the exact forward profile renderer.`);
    if (renderer.tupleCollisionFields.join("\0") !== "artifactId\0standardVersion\0subsetId") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].tupleCollisionFields must be exactly artifactId, standardVersion, subsetId.`);
  }

  const validateDescendantNode = (node: SemanticDescendantNode, key: string, rootKindId: string): string | null => {
    const configurable = "configurableEntry" in node;
    const authorityKey = node.nodeType === "file" ? "kindId" in node ? "kindId" : "fixedFilenameContractId" in node ? "fixedFilenameContractId" : "configurableEntry" : "kindId";
    exactKeys(node, configurable ? ["sourcePathSegments", "destinationPathSegments", "nodeType", authorityKey] : ["pathSegments", "nodeType", authorityKey, ...("sourceFilename" in node ? ["sourceFilename"] : [])], key);
    const validateSegments = (segments: readonly Readonly<{ kindId: string; literal: string }>[] | unknown, field: string): void => {
      if (!Array.isArray(segments)) {
        problems.push(`${key}.${field} must be an array.`);
        return;
      }
      let parentKindId = rootKindId;
      for (const [index, segment] of segments.entries()) {
        exactKeys(segment, ["kindId", "literal"], `${key}.${field}[${index}]`);
        if (!taxonomy.semanticDirectoryKinds[segment.kindId]) problems.push(`${key}.${field}[${index}].kindId is missing.`);
        else if (semanticDirectoryKindId(segment.literal, taxonomy, { parentKindId }) !== segment.kindId) problems.push(`${key}.${field}[${index}].literal does not resolve uniquely to its kind.`);
        parentKindId = segment.kindId;
      }
    };
    if (configurable) {
      validateSegments(node.sourcePathSegments, "sourcePathSegments");
      validateSegments(node.destinationPathSegments, "destinationPathSegments");
    } else validateSegments(node.pathSegments, "pathSegments");
    if (node.nodeType === "directory") {
      if (!projectionDirectoryKinds[node.kindId]) problems.push(`${key}.kindId is not a directory kind.`);
      const expected = node.pathSegments.at(-1)?.kindId ?? rootKindId;
      if (node.kindId !== expected) problems.push(`${key}.kindId must equal its realized directory kind ${JSON.stringify(expected)}.`);
    } else if (node.nodeType === "file") {
      if ("kindId" in node && !taxonomy.fileKinds[node.kindId]) problems.push(`${key}.kindId is not a file kind.`);
      else if ("kindId" in node && node.sourceFilename !== undefined && (node.sourceFilename !== node.sourceFilename.normalize("NFC") || /[\\/]/u.test(node.sourceFilename) || !node.sourceFilename.startsWith(taxonomy.fileKinds[node.kindId]!.emoji) || fileKindIdForSourcePath(node.sourceFilename, taxonomy) !== node.kindId)) problems.push(`${key}.sourceFilename must be one NFC basename resolving to kindId.`);
      else if ("fixedFilenameContractId" in node && !taxonomy.fixedFilenameContracts[node.fixedFilenameContractId]) problems.push(`${key}.fixedFilenameContractId is missing.`);
      else if (configurable) {
        exactKeys(node.configurableEntry, ["contractId", "sourceFilename", "configurationReferences"], `${key}.configurableEntry`);
        const entry = taxonomy.configurableEntryContracts[node.configurableEntry.contractId];
        if (!entry) problems.push(`${key}.configurableEntry.contractId is missing.`);
        else if (node.configurableEntry.sourceFilename !== node.configurableEntry.sourceFilename.normalize("NFC") || /[\\/]/u.test(node.configurableEntry.sourceFilename) || fileKindIdForSourcePath(node.configurableEntry.sourceFilename, taxonomy) !== entry.fileKindId) problems.push(`${key}.configurableEntry.sourceFilename must be one NFC basename resolving to the entry file kind.`);
        if (!Array.isArray(node.configurableEntry.configurationReferences) || node.configurableEntry.configurationReferences.length === 0) problems.push(`${key}.configurableEntry.configurationReferences must be non-empty.`);
        const actualConfigurationSources: string[] = [];
        for (const [index, reference] of (node.configurableEntry.configurationReferences ?? []).entries()) {
          const scope = `${key}.configurableEntry.configurationReferences[${index}]`;
          exactKeys(reference, ["fixedFilenameContractId", "adapter", "structuredLocation"], scope);
          const fixed = taxonomy.fixedFilenameContracts[reference.fixedFilenameContractId];
          const filename = fixed && fixedContractFilename(fixed);
          if (!fixed || fixed.scope.kind !== "package-root" || fixed.scope.ecosystemId !== entry?.ecosystemId) problems.push(`${scope}.fixedFilenameContractId must own the same package ecosystem.`);
          if ((reference.adapter === "toml" ? !filename?.endsWith(".toml") : reference.adapter === "json" ? !filename?.endsWith(".json") : true) || !/^[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)+$/u.test(reference.structuredLocation)) problems.push(`${scope} must declare an exact structured adapter location.`);
          if (filename) actualConfigurationSources.push(`${filename}:${reference.structuredLocation}`);
        }
        if (entry && actualConfigurationSources.sort().join("\0") !== [...entry.configurationSources].sort().join("\0")) problems.push(`${key}.configurableEntry.configurationReferences must cover every declared configuration source exactly once.`);
      }
    } else problems.push(`${key}.nodeType must be directory or file.`);
    try {
      return `${node.nodeType}:${semanticDescendantNodeRelativePath(node, taxonomy)}`;
    } catch {
      return null;
    }
  };

  if (record(taxonomy.semanticDescendantContracts, "semanticDescendantContracts")) for (const [id, contract] of Object.entries(taxonomy.semanticDescendantContracts)) {
    kebabId(id, `semanticDescendantContracts id ${JSON.stringify(id)}`);
    if ("contractKind" in contract) {
      exactKeys(contract, ["contractKind", "rootDirectoryKindId", "catalogContractId", "leafFileKindId", "rendering", "pathBudgetReserve"], `semanticDescendantContracts[${JSON.stringify(id)}]`);
      if (contract.contractKind !== "catalog") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].contractKind must be catalog.`);
      if (!taxonomy.semanticDirectoryKinds[contract.rootDirectoryKindId]) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rootDirectoryKindId is missing.`);
      const catalog = taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId];
      if (!catalog || !("contractKind" in catalog) || catalog.contractKind !== "distributed-json-manifest-catalog") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].catalogContractId must reference one distributed JSON manifest catalog.`);
      if (!taxonomy.fileKinds[contract.leafFileKindId]) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].leafFileKindId is missing.`);
      if (contract.rendering !== "semantic-member-directory-and-physical-kind-leaf") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rendering is invalid.`);
      exactKeys(contract.pathBudgetReserve, ["derivation", "bytes"], `semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve`);
      if (contract.pathBudgetReserve.derivation !== "longest-rendered-catalog-descendant-suffix") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.derivation is invalid.`);
      if (!Number.isSafeInteger(contract.pathBudgetReserve.bytes) || contract.pathBudgetReserve.bytes <= 0 || contract.pathBudgetReserve.bytes >= taxonomy.collisionPolicy.maxPathBytes) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.bytes must be a positive safe integer below maxPathBytes.`);
      continue;
    }
    exactKeys(contract, ["rootDirectoryKindId", "requiredNodes", "exclusiveAlternatives", "realizedNodeCount", "pathBudgetReserve"], `semanticDescendantContracts[${JSON.stringify(id)}]`);
    if (!projectionDirectoryKinds[contract.rootDirectoryKindId]) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rootDirectoryKindId is missing.`);
    if (!Array.isArray(contract.requiredNodes) || contract.requiredNodes.length === 0) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes must be non-empty.`);
    const requiredKeys = (contract.requiredNodes ?? []).map((node, index) => validateDescendantNode(node, `semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes[${index}]`, contract.rootDirectoryKindId)).filter((key): key is string => key !== null);
    if (new Set(requiredKeys).size !== requiredKeys.length) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes must be unique.`);
    if (!Array.isArray(contract.exclusiveAlternatives)) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives must be an array.`);
    const alternativeIds = new Set<string>();
    const alternativeKeys: string[] = [];
    for (const [groupIndex, alternative] of (contract.exclusiveAlternatives ?? []).entries()) {
      exactKeys(alternative, ["id", "mode", "nodes"], `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}]`);
      kebabId(alternative.id, `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].id`);
      if (alternativeIds.has(alternative.id)) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}] has duplicate alternative id ${JSON.stringify(alternative.id)}.`);
      alternativeIds.add(alternative.id);
      if (alternative.mode !== "exactly-one") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].mode must be exactly-one.`);
      if (!Array.isArray(alternative.nodes) || alternative.nodes.length < 2) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].nodes must contain at least two nodes.`);
      for (const [nodeIndex, node] of (alternative.nodes ?? []).entries()) {
        const nodeKey = validateDescendantNode(node, `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].nodes[${nodeIndex}]`, contract.rootDirectoryKindId);
        if (nodeKey) alternativeKeys.push(nodeKey);
      }
    }
    const allKeys = [...requiredKeys, ...alternativeKeys];
    if (new Set(allKeys).size !== allKeys.length) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}] descendant nodes must not overlap.`);
    const requiredRealizedKeys = new Set(requiredKeys);
    for (const key of requiredKeys) if (key.startsWith("file:")) {
      const segments = key.slice(5).split("/");
      segments.pop();
      let directory = "";
      for (const segment of segments) {
        directory = directory ? `${directory}/${segment}` : segment;
        requiredRealizedKeys.add(`directory:${directory}`);
      }
    }
    if (contract.realizedNodeCount !== requiredRealizedKeys.size + (contract.exclusiveAlternatives?.length ?? 0)) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].realizedNodeCount must equal realized required destination nodes plus one node per exclusive group.`);
    exactKeys(contract.pathBudgetReserve, ["derivation", "bytes"], `semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve`);
    if (contract.pathBudgetReserve.derivation !== "longest-canonical-descendant-suffix") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.derivation is invalid.`);
    const derivedBytes = Math.max(0, ...allKeys.map((key) => new TextEncoder().encode(`/${key.slice(key.indexOf(":") + 1)}`).length));
    if (contract.pathBudgetReserve.bytes !== derivedBytes) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.bytes must equal derived longest suffix ${derivedBytes}.`);
    if (contract.pathBudgetReserve.bytes >= taxonomy.collisionPolicy.maxPathBytes) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve must be below maxPathBytes.`);
    if (id === "mutation-scenario-bundle-v1") {
      const expectedRequired = ["directory:", "file:🦀️.rs", "directory:🦠️mutation", "file:🦠️mutation/🔣️.json", "directory:📸️snapshot", "directory:📸️snapshot/⬅️before", "file:📸️snapshot/⬅️before/🔣️.json", "directory:📸️snapshot/➡️after", "file:📸️snapshot/➡️after/🔣️.json", "directory:🔺️diff", "directory:🎯️outcome", "file:🎯️outcome/🔣️.json"].sort();
      const expectedAlternatives = ["file:🔺️diff/🔣️.json", "file:🔺️diff/🚫️.absent"].sort();
      if ([...requiredKeys].sort().join("\0") !== expectedRequired.join("\0") || [...alternativeKeys].sort().join("\0") !== expectedAlternatives.join("\0")) problems.push("semanticDescendantContracts.mutation-scenario-bundle-v1 must encode the exact 13-node physical bundle and exclusive diff alternatives.");
    }
    if (id === "draw-editor-command-bundle-v1") {
      const expectedRequired = ["directory:", "file:🦀️.rs", "directory:🔄️fsm", "file:🔄️fsm/🦀️.rs", "directory:🔄️fsm/📦️packages", "directory:🔄️fsm/📦️packages/🦀️rust", "file:🔄️fsm/📦️packages/🦀️rust/Cargo.toml", "file:🔄️fsm/📦️packages/🦀️rust/📋️project.json", "file:🔄️fsm/📦️packages/🦀️rust/📜️script.ts", "file:🔄️fsm/📦️packages/🦀️rust/📚️library/🦀️.rs", "directory:🔄️fsm/✨️macros", "file:🔄️fsm/✨️macros/🦀️.rs", "directory:🔄️fsm/✨️macros/📦️packages", "directory:🔄️fsm/✨️macros/📦️packages/🦀️rust", "file:🔄️fsm/✨️macros/📦️packages/🦀️rust/Cargo.toml", "file:🔄️fsm/✨️macros/📦️packages/🦀️rust/📋️project.json", "file:🔄️fsm/✨️macros/📦️packages/🦀️rust/📜️script.ts", "file:🔄️fsm/✨️macros/📦️packages/🦀️rust/📚️library/🦀️.rs"].sort();
      if ([...requiredKeys].sort().join("\0") !== expectedRequired.join("\0") || alternativeKeys.length !== 0 || contract.realizedNodeCount !== 20 || contract.pathBudgetReserve.bytes !== 78) problems.push("semanticDescendantContracts.draw-editor-command-bundle-v1 must encode the exact 20-node configurable-entry bundle and 78-byte reserve.");
    }
  }

  if (record(taxonomy.semanticPathProjectionCatalogContracts, "semanticPathProjectionCatalogContracts")) for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionCatalogContracts)) {
    kebabId(id, `semanticPathProjectionCatalogContracts id ${JSON.stringify(id)}`);
    if ("contractKind" in contract && contract.contractKind === "distributed-json-manifest-catalog") {
      exactKeys(contract, ["contractKind", "ownerArtifactMemberName", "profileVectors", "modelManifestSchema", "modelManifestSourceFilename", "modelIdentityField", "memberIdentityField", "memberVersionField", "requiredMemberVersion", "requiredModelManifest", "categoryRules", "coverage", "unknownCategoryPolicy", "unownedModelPolicy"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
      if (!contract.ownerArtifactMemberName || !contract.modelManifestSchema || !contract.modelManifestSourceFilename || contract.modelIdentityField !== "id" || contract.memberIdentityField !== "id" || contract.memberVersionField !== "version" || !contract.requiredMemberVersion || contract.requiredModelManifest !== true) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must declare exact non-empty manifest authority fields.`);
      if (!Array.isArray(contract.profileVectors) || contract.profileVectors.length === 0) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].profileVectors must be non-empty.`);
      const profiles = new Set<string>();
      for (const [index, vector] of (contract.profileVectors ?? []).entries()) {
        const scope = `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].profileVectors[${index}]`;
        exactKeys(vector, ["artifactId", "standardVersion", "subsetId"], scope);
        const key = [vector.artifactId, vector.standardVersion, vector.subsetId].join("\0");
        if (vector.artifactId !== contract.ownerArtifactMemberName || !vector.standardVersion || !vector.subsetId || profiles.has(key) || [vector.standardVersion, vector.subsetId].some((value) => value !== value.normalize("NFC") || /[\\/]/u.test(value))) problems.push(`${scope} must be one unique NFC owner profile tuple.`);
        profiles.add(key);
      }
      if (!Array.isArray(contract.categoryRules) || contract.categoryRules.length === 0) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].categoryRules must be non-empty.`);
      const categoryNames = new Set<string>();
      for (const [index, rule] of (contract.categoryRules ?? []).entries()) {
        const scope = `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].categoryRules[${index}]`;
        exactKeys(rule, rule.sourceShape === "direct-semantic-json" ? ["sourceDirectoryName", "directoryKindId", "sourceShape", "manifestSchema", "memberDirectoryEmoji"] : ["sourceDirectoryName", "directoryKindId", "sourceShape", "manifestSchema", "fixedSourceFilename"], scope);
        if (categoryNames.has(rule.sourceDirectoryName)) problems.push(`${scope}.sourceDirectoryName is duplicated.`);
        categoryNames.add(rule.sourceDirectoryName);
        if (!taxonomy.semanticDirectoryKinds[rule.directoryKindId]) problems.push(`${scope}.directoryKindId is missing.`);
        if (!rule.manifestSchema) problems.push(`${scope}.manifestSchema must be non-empty.`);
        if (rule.sourceShape === "direct-semantic-json") {
          if (!/^\p{Extended_Pictographic}\uFE0F$/u.test(rule.memberDirectoryEmoji)) problems.push(`${scope}.memberDirectoryEmoji must be one emoji plus U+FE0F.`);
        } else if (rule.sourceShape === "nested-fixed-json") {
          if (!rule.fixedSourceFilename) problems.push(`${scope}.fixedSourceFilename must be non-empty.`);
        } else problems.push(`${scope}.sourceShape is invalid.`);
      }
      if (contract.coverage !== "every-source-file-and-destination-node-exactly-once" || contract.unknownCategoryPolicy !== "problem" || contract.unownedModelPolicy !== "problem") problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must fail closed over exact source and destination ownership.`);
      continue;
    }
    if ("contractKind" in contract && contract.contractKind === "exact-owner-vectors") {
      exactKeys(contract, ["contractKind", "required", "allowEmpty", "identityFields", "coverage", "vectors"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
      if (contract.required !== true || contract.allowEmpty !== false || contract.identityFields.join("\0") !== "artifactId\0standardVersion\0subsetId\0commandDirectoryName" || contract.coverage !== "every-physical-command-bundle-exactly-once" || !Array.isArray(contract.vectors) || contract.vectors.length === 0) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must be the exact non-empty command owner-vector contract.`);
      const owners = new Set<string>();
      for (const [index, vector] of (contract.vectors ?? []).entries()) {
        exactKeys(vector, ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].vectors[${index}]`);
        const owner = [vector.artifactId, vector.standardVersion, vector.subsetId, vector.commandDirectoryName].join("\0");
        if (!vector.artifactId || !vector.standardVersion || !vector.subsetId || !vector.commandDirectoryName || owners.has(owner)) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].vectors[${index}] must be one unique non-empty owner tuple.`);
        owners.add(owner);
      }
      continue;
    }
    exactKeys(contract, ["registryField", "required", "allowEmpty", "runtimeKindsField", "runtimeKindsRelation", "mutationIdField", "sourceMutationDirectoryNameField", "mutationDirectoryNameField", "scenariosField", "scenarioIdField", "scenarioDirectoryNameField", "sourceBundleUniquenessFields", "canonicalBundleUniquenessFields", "coverage"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
    const expected: SemanticPathProjectionCatalogContract = { registryField: "vectors", required: true, allowEmpty: true, runtimeKindsField: "kinds", runtimeKindsRelation: "independent", mutationIdField: "mutationId", sourceMutationDirectoryNameField: "sourceMutationDirectoryName", mutationDirectoryNameField: "mutationDirectoryName", scenariosField: "scenarios", scenarioIdField: "id", scenarioDirectoryNameField: "directoryName", sourceBundleUniquenessFields: ["mutationId", "sourceMutationDirectoryName", "scenarioId"], canonicalBundleUniquenessFields: ["mutationId", "mutationDirectoryName", "scenarioId"], coverage: "every-physical-bundle-exactly-once" };
    if (JSON.stringify(contract) !== JSON.stringify(expected)) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must be the exact independent physical vectors contract.`);
  }

  if (record(taxonomy.semanticPathProjectionContracts, "semanticPathProjectionContracts")) for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionContracts)) {
    kebabId(id, `semanticPathProjectionContracts id ${JSON.stringify(id)}`);
    exactKeys(contract, ["sourceOwnerKindId", ...(contract.sourceArtifactMemberName === undefined ? [] : ["sourceArtifactMemberName"]), "sourceSegments", "profileRendererId", "destinationOwnerKindId", "destinationSegments", "descendantContractId", "catalogContractId", "rationaleRule"], `semanticPathProjectionContracts[${JSON.stringify(id)}]`);
    const artifactProjection = contract.rationaleRule !== "artifact-mutation-test-projection-v1";
    if (artifactProjection !== (typeof contract.sourceArtifactMemberName === "string") || (artifactProjection && !taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId]?.memberNames.includes(contract.sourceArtifactMemberName!))) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceArtifactMemberName must be one exact source-owner member only for artifact projections.`);
    if (!taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceOwnerKindId is missing.`);
    if (!taxonomy.semanticDirectoryMemberKinds[contract.destinationOwnerKindId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationOwnerKindId is missing.`);
    if (!taxonomy.semanticPathProjectionProfileRenderers[contract.profileRendererId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].profileRendererId is missing.`);
    if (!taxonomy.semanticDescendantContracts[contract.descendantContractId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].descendantContractId is missing.`);
    if (!taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].catalogContractId is missing.`);
    if (!["artifact-mutation-test-projection-v1", "artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(contract.rationaleRule)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].rationaleRule is invalid.`);
    const captures = new Set<string>();
    let sourceParentKindId: string | undefined;
    for (const [index, segment] of contract.sourceSegments.entries()) {
      const value = segment as unknown as Record<string, unknown>;
      const hasProjected = typeof value.projectedMemberKindId === "string";
      const hasMember = typeof value.memberKindId === "string";
      exactKeys(value, hasProjected ? ["projectedMemberKindId", "capture"] : hasMember ? ["memberKindId", "literal"] : ["kindId", "literal" in value ? "literal" : "capture"], `semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}]`);
      const kindId = (hasProjected ? value.projectedMemberKindId : hasMember ? value.memberKindId : value.kindId) as string;
      const kind = hasProjected ? taxonomy.semanticProjectedMemberKinds[kindId] : hasMember ? taxonomy.semanticDirectoryMemberKinds[kindId] : taxonomy.semanticDirectoryKinds[kindId];
      if (!kind) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}] references a missing kind.`);
      if (typeof value.literal === "string") {
        const validLiteral = hasMember
          ? Boolean(taxonomy.semanticDirectoryMemberKinds[kindId]?.ownerKindIds.includes(sourceParentKindId ?? "") && taxonomy.semanticDirectoryMemberKinds[kindId]?.memberNames.includes(value.literal))
          : semanticDirectoryKindId(value.literal, taxonomy, { parentKindId: sourceParentKindId }) === kindId;
        if (!validLiteral) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}].literal does not resolve to its kind.`);
      }
      if (typeof value.capture === "string") {
        if (!["standardVersion", "subsetId", "mutationId", "scenarioId", "commandDirectoryName"].includes(value.capture)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}].capture is invalid.`);
        if (captures.has(value.capture)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] captures ${JSON.stringify(value.capture)} more than once.`);
        captures.add(value.capture);
      }
      sourceParentKindId = kindId;
    }
    const requiredCaptures = contract.rationaleRule === "artifact-mutation-test-projection-v1" ? ["standardVersion", "subsetId", "mutationId", "scenarioId"] : contract.rationaleRule === "artifact-example-model-catalog-projection-v1" ? ["standardVersion", "subsetId"] : ["standardVersion", "subsetId", "commandDirectoryName"];
    if (captures.size !== requiredCaptures.length || requiredCaptures.some((field) => !captures.has(field))) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must capture exactly ${requiredCaptures.join(", ")}.`);
    let renderedProfiles = 0;
    let destinationParentKindId: string | undefined;
    for (const [index, segment] of contract.destinationSegments.entries()) {
      const value = segment as unknown as Record<string, unknown>;
      const hasProjected = typeof value.projectedMemberKindId === "string";
      const operation = "literal" in value ? "literal" : "render" in value ? "render" : "copy";
      exactKeys(value, [hasProjected ? "projectedMemberKindId" : "kindId", operation], `semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}]`);
      const kindId = (hasProjected ? value.projectedMemberKindId : value.kindId) as string;
      if (!(hasProjected ? taxonomy.semanticProjectedMemberKinds[kindId] : taxonomy.semanticDirectoryKinds[kindId])) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}] references a missing kind.`);
      if (typeof value.literal === "string" && semanticDirectoryKindId(value.literal, taxonomy, { parentKindId: destinationParentKindId }) !== kindId) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].literal does not resolve to its kind.`);
      if (value.render !== undefined) {
        if (value.render !== "profile") problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].render must be profile.`);
        renderedProfiles += 1;
      }
      if (typeof value.copy === "string" && !captures.has(value.copy)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].copy must reference a source capture.`);
      destinationParentKindId = kindId;
    }
    if (renderedProfiles !== 1) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must render exactly one profile segment.`);
    const projectedReferences = [...contract.sourceSegments, ...contract.destinationSegments].flatMap((segment) => "projectedMemberKindId" in segment ? [segment.projectedMemberKindId] : []);
    for (const [projectedId, projected] of Object.entries(taxonomy.semanticProjectedMemberKinds)) {
      const references = projectedReferences.filter((candidate) => candidate === projectedId).length;
      if (projected.projectionContractId === id && references !== 2) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must reference projected member ${JSON.stringify(projectedId)} exactly once in source and destination.`);
    }
    if (artifactProjection && contract.sourceArtifactMemberName) {
      try {
        if (artifactPathProjectionCatalogRoots(`🗿️artifacts/${contract.sourceArtifactMemberName}`, id, taxonomy).length === 0) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must render at least one exact forward owner profile.`);
      } catch (error) { problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] has invalid forward profile authority: ${error instanceof Error ? error.message : String(error)}`); }
    }
  }

  if (record(taxonomy.semanticPolicyStateCoordinateContracts, "semanticPolicyStateCoordinateContracts")) {
    if (Object.keys(taxonomy.semanticPolicyStateCoordinateContracts).join("\0") !== "dependency-freeze-users-v1") problems.push("semanticPolicyStateCoordinateContracts requires the exact dependency policy-state contract.");
    for (const [id, contract] of Object.entries(taxonomy.semanticPolicyStateCoordinateContracts)) {
      exactKeys(contract, ["contractKind", "statePath", "stateSchemaVersion", "sourceDisposition", "ownerProjectionContractId", "packageIds", "manifestFilename", "dependencyEvidenceField", "coordinatePointer", "preserveNonCoordinateBytes"], `semanticPolicyStateCoordinateContracts.${id}`);
      if (contract.contractKind !== "dependency-freeze-user-coordinates" || contract.statePath !== "🔒️dependencies.json" || contract.stateSchemaVersion !== 2 || contract.sourceDisposition !== "authored-policy-state" || contract.ownerProjectionContractId !== "nested-cargo-packages-v1" || !taxonomy.semanticPackageProjectionContracts[contract.ownerProjectionContractId] || JSON.stringify(contract.packageIds) !== JSON.stringify(["jcoprobe-guest"]) || contract.manifestFilename !== "Cargo.toml" || contract.dependencyEvidenceField !== "witDependency" || contract.coordinatePointer !== "/entries/*/users/*" || contract.preserveNonCoordinateBytes !== true) problems.push(`semanticPolicyStateCoordinateContracts.${id} has invalid exact user-coordinate authority.`);
    }
  }
  if (record(taxonomy.semanticPackageProjectionContracts, "semanticPackageProjectionContracts")) {
    if (Object.keys(taxonomy.semanticPackageProjectionContracts).join("\0") !== "nested-cargo-packages-v1") problems.push("semanticPackageProjectionContracts requires the exact nested Cargo contract.");
    for (const [id, contract] of Object.entries(taxonomy.semanticPackageProjectionContracts)) {
      exactKeys(contract, ["contractKind", "authorityCatalogPath", "authorityCatalogSha256", "packageIds", "sourceLeafCounts", "purityCount", "adapterCount", "derivedLeafCount", "joinedPathBindingCounts", "generatedSourceRetirementCounts", "authoredFragmentCounts", "rationaleRule"], `semanticPackageProjectionContracts.${id}`);
      if (contract.contractKind !== "exact-nested-cargo-package-catalog" || !exactOwnerPath(contract.authorityCatalogPath) || !/^[0-9a-f]{64}$/u.test(contract.authorityCatalogSha256) || JSON.stringify(contract.packageIds) !== JSON.stringify(["wgpu-renderer", "jcoprobe-guest"]) || JSON.stringify(contract.sourceLeafCounts) !== JSON.stringify([32, 4]) || contract.purityCount !== 27 || contract.adapterCount !== 5 || contract.derivedLeafCount !== 1 || JSON.stringify(contract.joinedPathBindingCounts) !== JSON.stringify([1, 0]) || JSON.stringify(contract.generatedSourceRetirementCounts) !== JSON.stringify([1, 0]) || JSON.stringify(contract.authoredFragmentCounts) !== JSON.stringify([31, 0]) || contract.rationaleRule !== "nested-cargo-package-projection-v1") problems.push(`semanticPackageProjectionContracts.${id} has invalid exact authority.`);
    }
  }

  if (record(taxonomy.semanticOwnedFileProjectionContracts, "semanticOwnedFileProjectionContracts")) {
    const ownerTuples = new Set<string>();
    for (const [id, contract] of Object.entries(taxonomy.semanticOwnedFileProjectionContracts)) {
      const scope = `semanticOwnedFileProjectionContracts[${JSON.stringify(id)}]`;
      kebabId(id, `${scope} id`);
      if (!contract || typeof contract !== "object" || Array.isArray(contract)) {
        problems.push(`${scope} must be an object.`);
        continue;
      }
      if (contract.contractKind === "exact-owner-path-catalog") {
        exactKeys(contract, ["contractKind", "authorityCatalogPath", "authorityCatalogSha256", "sourceFileKindId", "sourceBasenames", "destinationDirectoryKinds", "allowedDispositions", "ownerEvidenceKinds", "referenceOwnerIds", "generatorOwnerIds", "expectedCounts", "authoredDocumentCorrections", "rationaleRule", ...(Object.hasOwn(contract, "currentSourceRevisions") ? ["currentSourceRevisions"] : [])], scope);
        if (Object.hasOwn(contract, "currentSourceRevisions")) {
          try { parseSemanticOwnedCurrentSourceRevisions(contract.currentSourceRevisions); } catch (error) { problems.push(scope + ".currentSourceRevisions: " + (error instanceof Error ? error.message : String(error))); }
        }
        if (id !== "readme-license-owner-leaves-v1" || contract.rationaleRule !== "readme-license-owner-projection-v1") problems.push(scope + " must be the exact README/LICENSE owner projection contract.");
        if (!exactOwnerPath(contract.authorityCatalogPath)) problems.push(scope + ".authorityCatalogPath must be one repository-local non-opaque NFC path.");
        if (typeof contract.authorityCatalogSha256 !== "string" || !/^[a-f0-9]{64}$/u.test(contract.authorityCatalogSha256)) problems.push(scope + ".authorityCatalogSha256 must be one SHA-256 digest.");
        if (!taxonomy.fileKinds[contract.sourceFileKindId] || contract.sourceFileKindId !== "markdown") problems.push(scope + ".sourceFileKindId must be markdown.");
        if (!Array.isArray(contract.sourceBasenames) || contract.sourceBasenames.join("\0") !== "LICENSE.md\0README.md") problems.push(scope + ".sourceBasenames must be exactly LICENSE.md and README.md.");
        if (!contract.destinationDirectoryKinds || typeof contract.destinationDirectoryKinds !== "object" || Array.isArray(contract.destinationDirectoryKinds)) problems.push(scope + ".destinationDirectoryKinds must be an object.");
        else {
          exactKeys(contract.destinationDirectoryKinds, ["license", "readme"], scope + ".destinationDirectoryKinds");
          const expectedDestinations = {
            license: { directoryKindId: "owner-license", directoryName: "⚖️license", filename: "📝️.md" },
            readme: { directoryKindId: "owner-readme", directoryName: "📃️readme", filename: "📝️.md" },
          } as const;
          for (const kind of ["license", "readme"] as const) {
            const destination = contract.destinationDirectoryKinds[kind];
            exactKeys(destination, ["directoryKindId", "directoryName", "filename"], scope + ".destinationDirectoryKinds." + kind);
            if (JSON.stringify(destination) !== JSON.stringify(expectedDestinations[kind])) problems.push(scope + ".destinationDirectoryKinds." + kind + " is not canonical.");
            const directory = taxonomy.semanticDirectoryKinds[destination.directoryKindId];
            if (!directory || directory.projectionOnly !== true || semanticDirectoryKindId(destination.directoryName, taxonomy) !== destination.directoryKindId || fileKindIdForSourcePath(destination.filename, taxonomy) !== contract.sourceFileKindId) problems.push(scope + ".destinationDirectoryKinds." + kind + " does not resolve through the registered projection-only directory and Markdown leaf kinds.");
          }
        }
        const exactArrays = [
          [contract.allowedDispositions, "attribution-relocate\0configurable-owner-license-relocate\0fixed\0generated-evidence-relocate\0owner-documentation-relocate", "allowedDispositions"],
          [contract.ownerEvidenceKinds, "configurable-owner-license\0ordinary-owner-doc\0package-publication\0third-party-attribution\0ticket-evidence\0ticket-scratch", "ownerEvidenceKinds"],
          [contract.referenceOwnerIds, "asset-distribution-owner\0bun-package-publisher\0commonmark-scratch-rust-reader\0markdown-relative-reference-adapter\0repo-cli-dev-docs-go\0vscode-package-ignore", "referenceOwnerIds"],
          [contract.generatorOwnerIds, "assets-build", "generatorOwnerIds"],
        ] as const;
        for (const [values, expected, field] of exactArrays) if (!Array.isArray(values) || values.join("\0") !== expected) problems.push(scope + "." + field + " is not the exact frozen registry.");
        if (!contract.expectedCounts || typeof contract.expectedCounts !== "object" || Array.isArray(contract.expectedCounts)) problems.push(scope + ".expectedCounts must be an object.");
        else {
          exactKeys(contract.expectedCounts, ["fixed", "license", "projected", "readme", "referenceBindings", "total"], scope + ".expectedCounts");
          if (JSON.stringify(contract.expectedCounts) !== JSON.stringify({ fixed: 4, license: 8, projected: 36, readme: 32, referenceBindings: 62, total: 40 })) problems.push(scope + ".expectedCounts must freeze the 40-leaf authority.");
        }
        if (!taxonomy.generatorContracts["assets-build"]) problems.push(scope + " requires the assets-build generator owner.");
        try { parseSemanticOwnedDocumentCorrections(contract.authoredDocumentCorrections); } catch (error) { problems.push(scope + ".authoredDocumentCorrections: " + (error instanceof Error ? error.message : String(error))); }
        if (taxonomy.fixedFilenameContracts["root-script"]?.pathPattern !== "**/📜️script.ts") problems.push(scope + ".authoredDocumentCorrections requires the canonical root-script filename contract.");
        continue;
      }
      if (contract.contractKind === "semantic-facet-primary-file") {
        exactKeys(contract, ["contractKind", "sourceRoot", "sourceFilename", "fileKindAuthority", "sourceDisposition", "directoryCaptures", "ownerPathPatterns", "authoringCommand", "referenceConsumer", "rationaleRule"], scope);
        if (id !== "artifact-empty-facet-primary-markdown-v1" || contract.rationaleRule !== id || contract.sourceRoot !== "✏️s/🔌️plugins" || contract.sourceFilename !== "📌️.empty.md" || contract.fileKindAuthority !== "windowEmptyFacetFileKindId" || contract.sourceDisposition !== "authored" || taxonomy.windowEmptyFacetFileKindId !== "markdown") problems.push(`${scope} must use the exact authored empty-facet primary-leaf grammar.`);
        if (record(contract.directoryCaptures, `${scope}.directoryCaptures`)) for (const [capture, spec] of Object.entries(contract.directoryCaptures)) {
          if (!record(spec, `${scope}.directoryCaptures.${capture}`)) continue;
          exactKeys(spec, ["kindIds", ...(spec.names ? ["names"] : [])], `${scope}.directoryCaptures.${capture}`);
          if (!/^[a-zA-Z]+$/u.test(capture) || !Array.isArray(spec.kindIds) || spec.kindIds.length === 0 || new Set(spec.kindIds).size !== spec.kindIds.length || spec.kindIds.some((kindId) => !taxonomy.semanticDirectoryKinds[kindId] && !taxonomy.semanticDirectoryMemberKinds[kindId])) problems.push(`${scope}.directoryCaptures.${capture} must resolve registered semantic directory kinds.`);
          if (spec.names && (!Array.isArray(spec.names) || spec.names.length === 0 || new Set(spec.names).size !== spec.names.length || spec.names.some((name) => typeof name !== "string" || name !== name.normalize("NFC") || /[\\/]/u.test(name)))) problems.push(`${scope}.directoryCaptures.${capture}.names must be exact NFC directory names.`);
        }
        if (record(contract.ownerPathPatterns, `${scope}.ownerPathPatterns`)) {
          if (Object.keys(contract.ownerPathPatterns).join("\0") !== "plugin-commands\0artifact-surface\0artifact-mode\0artifact-window\0engine-mode\0engine-window\0extension-window") problems.push(`${scope}.ownerPathPatterns must contain the seven semantic owner contexts.`);
          const patterns = new Set<string>();
          for (const [form, pattern] of Object.entries(contract.ownerPathPatterns)) {
            if (typeof pattern !== "string" || pattern !== pattern.normalize("NFC") || !pattern.startsWith("{plugin}/") || patterns.has(pattern)) { problems.push(`${scope}.ownerPathPatterns.${form} is not a unique rooted owner grammar.`); continue; }
            patterns.add(pattern);
            for (const segment of pattern.split("/")) {
              const capture = /^\{([a-zA-Z]+)\}$/u.exec(segment)?.[1];
              if (capture ? !contract.directoryCaptures?.[capture] : !segment || /[\\*?{}\0]/u.test(segment) || !semanticDirectoryKindId(segment, taxonomy)) problems.push(`${scope}.ownerPathPatterns.${form} has an unregistered directory segment.`);
            }
          }
        }
        if (record(contract.authoringCommand, `${scope}.authoringCommand`)) {
          exactKeys(contract.authoringCommand, ["scriptPath", "command", "writeDisposition"], `${scope}.authoringCommand`);
          if (contract.authoringCommand.scriptPath !== "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts" || !Array.isArray(contract.authoringCommand.command) || contract.authoringCommand.command.join("\0") !== "new\0surface" || contract.authoringCommand.writeDisposition !== "create-if-absent") problems.push(`${scope}.authoringCommand must be the create-if-absent surface author.`);
        }
        if (record(contract.referenceConsumer, `${scope}.referenceConsumer`)) {
          exactKeys(contract.referenceConsumer, ["path", "ownerRoot", "adapter", "region", "lineTemplate"], `${scope}.referenceConsumer`);
          if (contract.referenceConsumer.path !== "✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/🦀️.rs" || contract.referenceConsumer.ownerRoot !== "✏️s/🔌️plugins/🔋️energy" || contract.referenceConsumer.adapter !== "rust" || contract.referenceConsumer.region !== "✏️👁️Surfaces" || contract.referenceConsumer.lineTemplate !== "// `{filename}` (`🎚️config`/`🎮️commands`/`👥️presence`/`🫧️transient` at every surface/mode level) need") problems.push(`${scope}.referenceConsumer must be the exact energy surface-region prose form.`);
        }
        continue;
      }
      if (contract.contractKind === "owner-primary-file") {
        exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "sourceFileKindId", "sourceFilename", "destinationFilename", "rationaleRule"], scope);
        if (id !== "ticket-document-primary-markdown-v1" || contract.ownerFixedDirectoryContractId !== "ticket-slug" || contract.sourceFileKindId !== "markdown" || contract.sourceFilename !== "ticket.md" || contract.rationaleRule !== id) problems.push(`${scope} must use the exact ticket document primary-leaf grammar.`);
        if (!taxonomy.fixedDirectoryContracts[contract.ownerFixedDirectoryContractId]) problems.push(`${scope}.ownerFixedDirectoryContractId is missing.`);
        if (!taxonomy.fileKinds[contract.sourceFileKindId] || fileKindIdForSourcePath(contract.sourceFilename, taxonomy) !== contract.sourceFileKindId || contract.destinationFilename !== canonicalFilenameForKind(contract.sourceFileKindId, taxonomy)) problems.push(`${scope}.destinationFilename must be the registered primary file-kind leaf.`);
        const tuple = [contract.ownerFixedDirectoryContractId, contract.sourceFilename, contract.contractKind].join("\0");
        if (ownerTuples.has(tuple)) problems.push(`${scope} overlaps another owned-file projection contract.`);
        ownerTuples.add(tuple);
        continue;
      }
      if (contract.contractKind === "owner-sibling-manifest-file") {
        exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "requiredSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "allowedStatuses", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "emptyContentRule", "statusDispositions", "rationaleRule"], scope);
        if (contract.manifestAdapter !== "json" || contract.manifestStatusLocation !== "status" || contract.emptyContentRule !== "zero-byte" || contract.rationaleRule !== "ticket-important-markdown-projection-v1") problems.push(`${scope} must use the exact active ticket important owner-file grammar.`);
        if (!Array.isArray(contract.allowedStatuses) || contract.allowedStatuses.join("\0") !== "closed\0open") problems.push(`${scope}.allowedStatuses must be exactly closed, open.`);
        if (contract.statusDispositions && typeof contract.statusDispositions === "object" && !Array.isArray(contract.statusDispositions)) exactKeys(contract.statusDispositions, ["open", "closed-empty", "closed-nonempty", "invalid"], `${scope}.statusDispositions`);
        else problems.push(`${scope}.statusDispositions must be an object.`);
        if (JSON.stringify(contract.statusDispositions) !== JSON.stringify({ open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" })) problems.push(`${scope}.statusDispositions must be the exact lifecycle mapping.`);
        if (!taxonomy.fixedFilenameContracts[contract.requiredSiblingFixedFilenameContractId]) problems.push(`${scope}.requiredSiblingFixedFilenameContractId is missing.`);
      } else if (contract.contractKind === "owner-optional-sibling-manifest-file") {
        exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "optionalSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "admittedDispositions", "rationaleRule"], scope);
        if (contract.manifestAdapter !== "json" || contract.manifestStatusLocation !== "status" || contract.rationaleRule !== "ticket-important-history-markdown-v1") problems.push(`${scope} must use the exact ticket important history grammar.`);
        if (!Array.isArray(contract.admittedDispositions) || contract.admittedDispositions.join("\0") !== "closed-nonzero\0invalid-manifest\0missing-manifest") problems.push(`${scope}.admittedDispositions must be the exact history mapping.`);
        if (!taxonomy.fixedFilenameContracts[contract.optionalSiblingFixedFilenameContractId]) problems.push(`${scope}.optionalSiblingFixedFilenameContractId is missing.`);
      } else problems.push(`${scope}.contractKind is invalid.`);
      if (!taxonomy.fixedDirectoryContracts[contract.ownerFixedDirectoryContractId]) problems.push(`${scope}.ownerFixedDirectoryContractId is missing.`);
      if (typeof contract.sourceFilename !== "string" || !taxonomy.fileKinds[contract.sourceFileKindId] || fileKindIdForSourcePath(contract.sourceFilename, taxonomy) !== contract.sourceFileKindId) problems.push(`${scope}.sourceFilename must resolve to sourceFileKindId.`);
      const destinationKind = taxonomy.semanticDirectoryKinds[contract.destinationDirectoryKindId];
      if (!destinationKind || destinationKind.projectionOnly !== true || typeof contract.destinationDirectoryName !== "string" || semanticDirectoryKindId(contract.destinationDirectoryName, taxonomy, { parentKindId: "ticket-slug" }) !== contract.destinationDirectoryKindId) problems.push(`${scope}.destination directory must resolve to one ticket-scoped projectionOnly kind.`);
      if (contract.destinationFilename !== "📝️.md" || typeof contract.destinationFilename !== "string" || fileKindIdForSourcePath(contract.destinationFilename, taxonomy) !== contract.sourceFileKindId) problems.push(`${scope}.destinationFilename must be the exact Markdown physical leaf.`);
      for (const [field, value] of [["sourceFilename", contract.sourceFilename], ["destinationDirectoryName", contract.destinationDirectoryName], ["destinationFilename", contract.destinationFilename]] as const) if (typeof value !== "string" || !value || value !== value.normalize("NFC") || /[\\/]/u.test(value)) problems.push(`${scope}.${field} must be one non-empty NFC name.`);
      const siblingContractId = contract.contractKind === "owner-sibling-manifest-file" ? contract.requiredSiblingFixedFilenameContractId : contract.optionalSiblingFixedFilenameContractId;
      const tuple = [contract.ownerFixedDirectoryContractId, siblingContractId, contract.sourceFilename, contract.contractKind].join("\0");
      if (ownerTuples.has(tuple)) problems.push(`${scope} overlaps another owned-file projection contract.`);
      ownerTuples.add(tuple);
    }
    if (Object.keys(taxonomy.semanticOwnedFileProjectionContracts).join("\0") !== "artifact-empty-facet-primary-markdown-v1\0readme-license-owner-leaves-v1\0ticket-document-primary-markdown-v1\0ticket-important-history-markdown-v1\0ticket-important-markdown-v1") problems.push("semanticOwnedFileProjectionContracts must contain the exact artifact-facet, README/LICENSE, ticket-document, active ticket-important, and history ticket-important contracts.");
  }

  if (record(taxonomy.semanticPathProjectionReferenceConsumerContracts, "semanticPathProjectionReferenceConsumerContracts")) {
    const identities = new Set<string>();
    const patterns = new Set<string>();
    const adapters = ["rust", "typescript", "json", "toml"];
    const forms: readonly SemanticPathProjectionReferenceConsumerForm[] = ["path-reference", "artifact-catalog-glob", "artifact-catalog-prose:root-marker", "artifact-catalog-prose:relative-root", "artifact-catalog-prose:interaction-glob", "artifact-catalog-prose:catalog-grammar"];
    for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionReferenceConsumerContracts)) {
      const scope = `semanticPathProjectionReferenceConsumerContracts[${JSON.stringify(id)}]`;
      kebabId(id, `${scope} id`);
      exactKeys(contract, ["projectionContractId", "consumerIdentity", "ownership", "sourcePathPattern", "sourcePathIdentities", "adapters", "supportedForms", "staleMarkers"], scope);
      const projection = taxonomy.semanticPathProjectionContracts[contract.projectionContractId];
      if (!projection || !["artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(projection.rationaleRule)) problems.push(`${scope}.projectionContractId must reference one CAD or Draw artifact projection.`);
      if (contract.consumerIdentity !== id || identities.has(contract.consumerIdentity)) problems.push(`${scope}.consumerIdentity must equal its unique registry id.`);
      identities.add(contract.consumerIdentity);
      if (contract.ownership !== "external") problems.push(`${scope}.ownership must be external.`);
      if (!contract.sourcePathPattern.startsWith("^") || !contract.sourcePathPattern.endsWith("$") || contract.sourcePathPattern !== contract.sourcePathPattern.normalize("NFC") || /\uFE0E/u.test(contract.sourcePathPattern)) problems.push(`${scope}.sourcePathPattern must be one anchored NFC regex without VS15.`);
      try {
        const expression = new RegExp(contract.sourcePathPattern, "u");
        if (!Array.isArray(contract.sourcePathIdentities) || contract.sourcePathIdentities.length === 0 || contract.sourcePathIdentities.some((path) => !path || path !== path.normalize("NFC") || /\uFE0E/u.test(path) || !expression.test(path)) || new Set(contract.sourcePathIdentities).size !== contract.sourcePathIdentities.length) problems.push(`${scope}.sourcePathIdentities must be unique exact NFC paths admitted by sourcePathPattern.`);
      } catch {
        problems.push(`${scope}.sourcePathPattern must compile with Unicode semantics.`);
      }
      const patternKey = `${contract.projectionContractId}\0${contract.sourcePathPattern}`;
      if (patterns.has(patternKey)) problems.push(`${scope}.sourcePathPattern is duplicated for its projection.`);
      patterns.add(patternKey);
      if (!Array.isArray(contract.adapters) || contract.adapters.length === 0 || contract.adapters.some((adapter) => !adapters.includes(adapter)) || new Set(contract.adapters).size !== contract.adapters.length) problems.push(`${scope}.adapters must be unique supported reference adapters.`);
      if (!Array.isArray(contract.supportedForms) || contract.supportedForms.length === 0 || contract.supportedForms.some((form) => !forms.includes(form)) || new Set(contract.supportedForms).size !== contract.supportedForms.length) problems.push(`${scope}.supportedForms must be unique supported structural forms.`);
      if (!Array.isArray(contract.staleMarkers) || contract.staleMarkers.length === 0 || contract.staleMarkers.some((marker) => !marker || marker !== marker.normalize("NFC") || /\uFE0E/u.test(marker)) || new Set(contract.staleMarkers).size !== contract.staleMarkers.length) problems.push(`${scope}.staleMarkers must be unique non-empty NFC markers without VS15.`);
      if (contract.supportedForms.some((form) => form.startsWith("artifact-catalog-")) && (projection?.rationaleRule !== "artifact-example-model-catalog-projection-v1" || !contract.adapters.some((adapter) => adapter === "rust" || adapter === "typescript"))) problems.push(`${scope} artifact-catalog forms require the CAD projection and a Rust or TypeScript adapter.`);
    }
    const required = ["cad-editor-interaction", "cad-editor-runtime", "cad-interaction-spec", "cad-spatial-kernel-geometry", "draw-dependency-registry", "draw-package-cargo", "draw-package-library", "draw-workspace-cargo", "draw-workspace-script"];
    if ([...identities].sort().join("\0") !== required.join("\0")) problems.push("semanticPathProjectionReferenceConsumerContracts must encode the nine exact current external CAD/Draw consumers.");
    const rows = Object.values(taxonomy.semanticPathProjectionReferenceConsumerContracts);
    for (const [index, left] of rows.entries()) for (const right of rows.slice(index + 1)) {
      if (left.projectionContractId !== right.projectionContractId || !left.supportedForms.some((form) => right.supportedForms.includes(form))) continue;
      const leftPattern = new RegExp(left.sourcePathPattern, "u");
      const rightPattern = new RegExp(right.sourcePathPattern, "u");
      if (left.sourcePathIdentities.some((path) => rightPattern.test(path)) || right.sourcePathIdentities.some((path) => leftPattern.test(path))) problems.push(`semanticPathProjectionReferenceConsumerContracts ${JSON.stringify(left.consumerIdentity)} and ${JSON.stringify(right.consumerIdentity)} overlap for one supported form.`);
    }
  }

  const projectionIds = taxonomy.mutationCatalogProjection;
  if (!projectionIds || typeof projectionIds !== "object") problems.push("mutationCatalogProjection must be an object.");
  else {
    exactKeys(projectionIds, ["projectionContractId", "projectedMemberKindId", "descendantContractId", "catalogContractId"], "mutationCatalogProjection");
    const projected = taxonomy.semanticProjectedMemberKinds[projectionIds.projectedMemberKindId];
    const projection = taxonomy.semanticPathProjectionContracts[projectionIds.projectionContractId];
    if (!projected || projected.projectionContractId !== projectionIds.projectionContractId) problems.push("mutationCatalogProjection projected member and projection IDs do not agree.");
    if (!projection || projection.descendantContractId !== projectionIds.descendantContractId || projection.catalogContractId !== projectionIds.catalogContractId) problems.push("mutationCatalogProjection contract IDs do not agree with its projection.");
  }
  const exactProjection = taxonomy.semanticPathProjectionContracts["artifact-mutation-tests-v1"];
  const exactSource = [{ kindId: "standards", literal: "🏅️standards" }, { kindId: "standard", capture: "standardVersion" }, { kindId: "subsets", literal: "🪆️subsets" }, { kindId: "subset", capture: "subsetId" }, { kindId: "schema", literal: "🧬️schema" }, { kindId: "schema", literal: "🧬️mutations" }, { projectedMemberKindId: "mutation-test-subject", capture: "mutationId" }, { kindId: "tests", literal: "🧪️tests" }, { kindId: "test-case", capture: "scenarioId" }];
  const exactDestination = [{ kindId: "tests", literal: "🧪️tests" }, { kindId: "mutation-test-profile", render: "profile" }, { projectedMemberKindId: "mutation-test-subject", copy: "mutationId" }, { kindId: "test-case", copy: "scenarioId" }];
  if (!exactProjection || JSON.stringify(exactProjection.sourceSegments) !== JSON.stringify(exactSource) || JSON.stringify(exactProjection.destinationSegments) !== JSON.stringify(exactDestination)) problems.push("semanticPathProjectionContracts.artifact-mutation-tests-v1 must encode the exact source and destination path grammar.");
  const exactBundle = taxonomy.semanticDescendantContracts["mutation-scenario-bundle-v1"];
  if (!exactBundle || "contractKind" in exactBundle || exactBundle.realizedNodeCount !== 13 || exactBundle.exclusiveAlternatives.length !== 1 || exactBundle.exclusiveAlternatives[0]?.id !== "diff-leaf" || exactBundle.pathBudgetReserve.bytes !== 42) problems.push("semanticDescendantContracts.mutation-scenario-bundle-v1 must encode 13 nodes, one diff alternative, and the derived 42-byte reserve.");
  //#endregion 🪞️SemanticPathProjection

  if (taxonomy.fixedDirectoryContractSets !== undefined && !(taxonomy.fixedDirectoryContractSets && typeof taxonomy.fixedDirectoryContractSets === "object" && !Array.isArray(taxonomy.fixedDirectoryContractSets))) problems.push("fixedDirectoryContractSets must be an object.");
  else for (const [id, values] of Object.entries(taxonomy.fixedDirectoryContractSets ?? {})) {
    if (!/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(id)) problems.push(`fixedDirectoryContractSets[${JSON.stringify(id)}] must use a literal set ID.`);
    try { parseFixedDirectoryContractSetScope({ kind: "fixed-directory-contract-set", fixedDirectoryContractIds: values }, taxonomy.fixedDirectoryContracts); } catch (error) { problems.push(`fixedDirectoryContractSets[${JSON.stringify(id)}]: ${error instanceof Error ? error.message : String(error)}`); }
  }
  const fixedScope = (scope: FixedContractScope, contractPathPattern: string, key: string, filenameContract: boolean): void => {
    if (!(typeof scope === "object" && scope !== null && !Array.isArray(scope)) || typeof scope.kind !== "string") {
      problems.push(`${key} must be a tagged fixed-contract scope.`);
      return;
    }
    if (scope.kind === "exact-path") {
      exactKeys(scope, ["kind", "path"], key);
      pathPattern(scope.path, `${key}.path`);
      if (scope.path !== contractPathPattern || /[*?\[\]{}]/u.test(scope.path)) problems.push(`${key}.path must equal the exact wildcard-free contract path.`);
    } else if (scope.kind === "repository-root") {
      exactKeys(scope, ["kind"], key);
      if (contractPathPattern.includes("/") || /[*?\[\]{}]/u.test(contractPathPattern)) problems.push(`${key} repository-root contract must be one exact basename.`);
    } else if (scope.kind === "package-root") {
      exactKeys(scope, ["kind", "ecosystemId"], key);
      if (!filenameContract) problems.push(`${key} cannot use package-root scope.`);
      if (!scope.ecosystemId || !taxonomy.ecosystems[scope.ecosystemId]) problems.push(`${key}.ecosystemId must reference an ecosystem.`);
    } else if (scope.kind === "directory-kind") {
      exactKeys(scope, ["kind", "directoryKindId"], key);
      if (!scope.directoryKindId || !taxonomy.semanticDirectoryKinds[scope.directoryKindId]) problems.push(`${key}.directoryKindId must reference a semantic directory kind.`);
    } else if (scope.kind === "fixed-directory-contract") {
      exactKeys(scope, ["kind", "fixedDirectoryContractId"], key);
      if (!filenameContract) problems.push(`${key} cannot use fixed-directory-contract scope.`);
      if (!scope.fixedDirectoryContractId || !taxonomy.fixedDirectoryContracts[scope.fixedDirectoryContractId]) problems.push(`${key}.fixedDirectoryContractId must reference a fixed directory contract.`);
    } else if (scope.kind === "fixed-directory-contract-set") {
      if (!filenameContract) problems.push(`${key} cannot use fixed-directory-contract-set scope.`);
      try { parseFixedDirectoryContractSetScope(scope, taxonomy.fixedDirectoryContracts); } catch (error) { problems.push(`${key}: ${error instanceof Error ? error.message : String(error)}`); }
    } else if (scope.kind === "named-fixed-directory-contract-set") {
      if (!filenameContract) problems.push(`${key} cannot use named-fixed-directory-contract-set scope.`);
      try { parseNamedFixedDirectoryContractSetScope(scope, taxonomy.fixedDirectoryContracts, taxonomy.fixedDirectoryContractSets ?? {}); } catch (error) { problems.push(`${key}: ${error instanceof Error ? error.message : String(error)}`); }
    } else if (scope.kind === "sibling-fixed-filename-contract") {
      exactKeys(scope, ["kind", "fixedFilenameContractId"], key);
      if (!filenameContract) problems.push(`${key} cannot use sibling-fixed-filename-contract scope.`);
      if (!scope.fixedFilenameContractId || !taxonomy.fixedFilenameContracts[scope.fixedFilenameContractId]) problems.push(`${key}.fixedFilenameContractId must reference a fixed filename contract.`);
    } else if (scope.kind === "path-pattern") exactKeys(scope, ["kind"], key);
    else problems.push(`${key}.kind is invalid.`);
  };

  if (record(taxonomy.fixedFilenameContracts, "fixedFilenameContracts")) for (const [id, contract] of Object.entries(taxonomy.fixedFilenameContracts)) {
    pathPattern(contract.pathPattern, `fixedFilenameContracts[${JSON.stringify(id)}].pathPattern`);
    if (typeof contract.pathPattern === "string" && /[*?\[\]{}]/u.test(fixedContractFilename(contract))) problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].pathPattern must end in one exact literal basename.`);
    if (!contract.authority || !contract.reason || !contract.verification) problems.push(`fixedFilenameContracts[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
    if (contract.configurability !== "unconfigurable") problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].configurability must be unconfigurable.`);
    fixedScope(contract.scope, contract.pathPattern, `fixedFilenameContracts[${JSON.stringify(id)}].scope`, true);
    if (!(contract.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(contract.expires))) problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
  }
  const cargoTargetEvidence = taxonomy.semanticDirectoryKinds["ticket-cargo-target-evidence"];
  if (!cargoTargetEvidence || cargoTargetEvidence.emoji !== "🧪️" || cargoTargetEvidence.slugPattern !== "^target-[a-z0-9]+(?:-[a-z0-9]+)*$" || cargoTargetEvidence.allowEmojiOnly || cargoTargetEvidence.parentKindIds !== undefined) problems.push("semanticDirectoryKinds.ticket-cargo-target-evidence must remain the exact ticket-local Cargo target authority.");
  const cargoCacheTag = taxonomy.fixedFilenameContracts["cargo-cache-tag"];
  if (!cargoCacheTag || cargoCacheTag.pathPattern !== "**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**/CACHEDIR.TAG" || cargoCacheTag.authority !== "Cargo" || cargoCacheTag.scope.kind !== "directory-kind" || cargoCacheTag.scope.directoryKindId !== "ticket-cargo-target-evidence") problems.push("fixedFilenameContracts.cargo-cache-tag must remain conjunctively scoped to a governed ticket path and the ticket-cargo-target-evidence directory kind.");
  const ticketCargoPattern = "**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**/";
  for (const triple of ["wasm32-unknown-unknown", "wasm32-wasip2"] as const) {
    const directoryId = `cargo-target-triple-${triple}`;
    const directory = taxonomy.fixedDirectoryContracts[directoryId];
    if (!directory || directory.pathPattern !== `${ticketCargoPattern}${triple}` || directory.authority !== "Cargo" || directory.scope.kind !== "directory-kind" || directory.scope.directoryKindId !== "ticket-cargo-target-evidence") problems.push(`fixedDirectoryContracts.${directoryId} must remain the exact governed ticket Cargo target-triple authority.`);
    const cacheId = `cargo-cache-tag-${triple}`;
    const cache = taxonomy.fixedFilenameContracts[cacheId];
    if (!cache || cache.pathPattern !== `${ticketCargoPattern}${triple}/CACHEDIR.TAG` || cache.authority !== "Cargo" || cache.scope.kind !== "fixed-directory-contract" || cache.scope.fixedDirectoryContractId !== directoryId) problems.push(`fixedFilenameContracts.${cacheId} must remain conjunctively scoped through its exact Cargo target-triple contract.`);
  }
  const nxManifestScopes = [
    ["nx-owned-node-package-manifest", "**/package.json", "Nx and Node.js"],
    ["nx-owned-typescript-config", "**/tsconfig.json", "Nx and TypeScript"],
  ] as const;
  for (const [id, pattern, authority] of nxManifestScopes) {
    const contract = taxonomy.fixedFilenameContracts[id];
    if (!contract || contract.pathPattern !== pattern || contract.authority !== authority || contract.scope.kind !== "sibling-fixed-filename-contract" || contract.scope.fixedFilenameContractId !== "nx-project-manifest") problems.push(`fixedFilenameContracts.${id} must remain conjunctively scoped through an adjacent exact Nx project manifest.`);
  }
  const bunPublicationScopes = [
    ["bun-package-readme", "**/README.md"],
    ["bun-package-license", "**/LICENSE.md"],
  ] as const;
  for (const [id, pattern] of bunPublicationScopes) {
    const contract = taxonomy.fixedFilenameContracts[id];
    if (!contract || contract.pathPattern !== pattern || contract.authority !== "Bun package publisher" || contract.scope.kind !== "package-root" || contract.scope.ecosystemId !== "🟦️typescript" || contract.verification !== "bun pm pack --dry-run --ignore-scripts") problems.push(`fixedFilenameContracts.${id} must remain scoped to an exact publishable Bun package root.`);
  }
  const ticketCargoManifest = taxonomy.fixedFilenameContracts["ticket-cargo-manifest"];
  if (!ticketCargoManifest || ticketCargoManifest.pathPattern !== `${ticketCargoPattern}Cargo.toml` || ticketCargoManifest.authority !== "Cargo" || ticketCargoManifest.scope.kind !== "path-pattern") problems.push("fixedFilenameContracts.ticket-cargo-manifest must remain scoped to governed canonical or embedded ticket paths.");
  const ticketCargoLock = taxonomy.fixedFilenameContracts["ticket-cargo-lock"];
  if (!ticketCargoLock || ticketCargoLock.pathPattern !== `${ticketCargoPattern}Cargo.lock` || ticketCargoLock.authority !== "Cargo" || ticketCargoLock.scope.kind !== "path-pattern") problems.push("fixedFilenameContracts.ticket-cargo-lock must remain scoped to governed canonical or embedded ticket paths.");
  const rootCargoLock = taxonomy.fixedFilenameContracts["root-cargo-lock"];
  if (!rootCargoLock || rootCargoLock.pathPattern !== "Cargo.lock" || rootCargoLock.authority !== "Cargo" || rootCargoLock.scope.kind !== "repository-root") problems.push("fixedFilenameContracts.root-cargo-lock must remain the exact repository-root Cargo lock authority.");

  if (record(taxonomy.fixedFilenameRejectionContracts, "fixedFilenameRejectionContracts")) {
    const identities = new Map<string, string>();
    for (const [id, contract] of Object.entries(taxonomy.fixedFilenameRejectionContracts)) {
      exactKeys(contract, ["sourcePathIdentities", "disposition", "reason"], `fixedFilenameRejectionContracts[${JSON.stringify(id)}]`);
      if (!Array.isArray(contract.sourcePathIdentities) || contract.sourcePathIdentities.length === 0) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].sourcePathIdentities must be non-empty.`);
      for (const identity of contract.sourcePathIdentities ?? []) {
        pathPattern(identity, `fixedFilenameRejectionContracts[${JSON.stringify(id)}].sourcePathIdentities`);
        if (/[*?\[]/u.test(identity)) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}] identities must be exact paths.`);
        if (identities.has(identity)) problems.push(`Fixed filename rejection identity ${JSON.stringify(identity)} is duplicated by ${JSON.stringify(identities.get(identity))} and ${JSON.stringify(id)}.`);
        identities.set(identity, id);
      }
      if (!["normalize", "relocate"].includes(contract.disposition)) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].disposition is invalid.`);
      if (!contract.reason) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].reason must be non-empty.`);
    }
  }

  if (record(taxonomy.fixedDirectoryContracts, "fixedDirectoryContracts")) for (const [id, contract] of Object.entries(taxonomy.fixedDirectoryContracts)) {
    pathPattern(contract.pathPattern, `fixedDirectoryContracts[${JSON.stringify(id)}].pathPattern`);
    if (!contract.authority || !contract.reason || !contract.verification) problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
    if (contract.configurability !== "unconfigurable") problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].configurability must be unconfigurable.`);
    if (contract.descendants !== undefined && contract.descendants !== "reserved") problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].descendants must be reserved when present.`);
    fixedScope(contract.scope, contract.pathPattern, `fixedDirectoryContracts[${JSON.stringify(id)}].scope`, false);
    if (!(contract.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(contract.expires))) problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
  }

  if (record(taxonomy.configurableEntryContracts, "configurableEntryContracts")) for (const [id, contract] of Object.entries(taxonomy.configurableEntryContracts)) {
    if (!taxonomy.fileKinds[contract.fileKindId]) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].fileKindId is missing.`);
    else if (!canonicalFilenamesForKind(contract.fileKindId, taxonomy).includes(contract.filename)) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].filename is not canonical for its file kind.`);
    if (!taxonomy.ecosystems[contract.ecosystemId]) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].ecosystemId is missing.`);
    if (!Array.isArray(contract.configurationSources) || contract.configurationSources.length === 0) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].configurationSources must be non-empty.`);
  }

  if (record(taxonomy.packageGlueGrammar, "packageGlueGrammar")) for (const [id, grammar] of Object.entries(taxonomy.packageGlueGrammar)) {
    if (!["rust", "typescript", "javascript", "go", "python", "dotnet", "c-cpp"].includes(grammar.analyzer)) problems.push(`packageGlueGrammar[${JSON.stringify(id)}].analyzer is invalid.`);
    if (!Number.isSafeInteger(grammar.maxDelegationStatements) || grammar.maxDelegationStatements < 0) problems.push(`packageGlueGrammar[${JSON.stringify(id)}].maxDelegationStatements is invalid.`);
    if (!Array.isArray(grammar.allowedRoles) || grammar.allowedRoles.some((role) => !["declaration", "registration", "bootstrap", "thin-delegation"].includes(role))) problems.push(`packageGlueGrammar[${JSON.stringify(id)}].allowedRoles is invalid.`);
  }

  if (record(taxonomy.packageBoundaryRules, "packageBoundaryRules")) for (const [ecosystemId, rule] of Object.entries(taxonomy.packageBoundaryRules)) {
    if (!taxonomy.ecosystems[ecosystemId]) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}] has no ecosystem.`);
    if (rule.manifestContractId && !taxonomy.fixedFilenameContracts[rule.manifestContractId]) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].manifestContractId is missing.`);
    ids(rule.entryContractIds, taxonomy.configurableEntryContracts, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].entryContractIds`);
    ids(rule.allowedFixedContractIds, taxonomy.fixedFilenameContracts, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedFixedContractIds`);
    ids(rule.allowedFileKindIds, taxonomy.fileKinds, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedFileKindIds`);
    ids(rule.allowedDirectoryKindIds, taxonomy.semanticDirectoryKinds, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedDirectoryKindIds`);
    if (!taxonomy.packageGlueGrammar[rule.glueGrammarId]) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].glueGrammarId is missing.`);
    if (rule.recursive !== true || rule.uncertainRole !== "problem" || rule.implementationRole !== "problem") problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}] must recursively block uncertain and implementation roles.`);
  }

  if (record(taxonomy.packageBoundaryProfiles, "packageBoundaryProfiles")) for (const [id, profile] of Object.entries(taxonomy.packageBoundaryProfiles)) {
    exactKeys(profile, ["admission", "allowedFileKindIds", "allowedDirectoryKindIds", "allowedFixedContractIds", "glueGrammarId", "recursive", "uncertainRole", "implementationRole", "reason"], `packageBoundaryProfiles[${JSON.stringify(id)}]`);
    if (profile.admission !== "blocked-until-language-directory-registered") problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}].admission is invalid.`);
    ids(profile.allowedFileKindIds, taxonomy.fileKinds, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedFileKindIds`);
    ids(profile.allowedDirectoryKindIds, taxonomy.semanticDirectoryKinds, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedDirectoryKindIds`);
    ids(profile.allowedFixedContractIds, taxonomy.fixedFilenameContracts, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedFixedContractIds`);
    if (!taxonomy.packageGlueGrammar[profile.glueGrammarId]) problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}].glueGrammarId is missing.`);
    if (profile.recursive !== true || profile.uncertainRole !== "problem" || profile.implementationRole !== "problem" || !profile.reason) problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}] must fail closed with a reason.`);
  }

  if (record(taxonomy.packageSourceDispositions, "packageSourceDispositions")) {
    const expected = new Map<string, "fixed" | "configurable">();
    for (const [id, contract] of Object.entries(taxonomy.fixedFilenameContracts)) {
      const kindId = fileKindIdForSourcePath(fixedContractFilename(contract), taxonomy);
      if (kindId && taxonomy.fileKinds[kindId]?.role === "source") expected.set(id, "fixed");
    }
    for (const [id, contract] of Object.entries(taxonomy.configurableEntryContracts)) if (taxonomy.fileKinds[contract.fileKindId]?.role === "source") expected.set(id, "configurable");
    for (const missing of [...expected.keys()].filter((id) => !taxonomy.packageSourceDispositions[id])) problems.push(`packageSourceDispositions is missing source-format contract ${JSON.stringify(missing)}.`);
    for (const [id, disposition] of Object.entries(taxonomy.packageSourceDispositions)) {
      exactKeys(disposition, ["contractKind", "disposition", "validator", "authority", "verification"], `packageSourceDispositions[${JSON.stringify(id)}]`);
      if (!expected.has(id)) problems.push(`packageSourceDispositions[${JSON.stringify(id)}] does not name a source-format fixed/configurable contract.`);
      else if (expected.get(id) !== disposition.contractKind) problems.push(`packageSourceDispositions[${JSON.stringify(id)}].contractKind does not match its registry.`);
      if (!["adapter-source", "tool-metadata"].includes(disposition.disposition)) problems.push(`packageSourceDispositions[${JSON.stringify(id)}].disposition is invalid.`);
      const TOOL_CONFIG_VALIDATORS: Readonly<Record<string, string>> = { "vitest-configuration": "vitest-config-entry", "tool-config-vitest": "vitest-config", "tool-config-tailwind": "tailwind-config", "tool-config-postcss": "postcss-config", "tool-config-eslint": "eslint-config", "tool-config-dependency-cruiser": "dependency-cruiser-config", "pytest-configuration": "root-pytest-config", "eslint-configuration": "root-eslint-config", "vscode-test-configuration": "vscode-test-cli-config" };
      const configValidatorOwner = TOOL_CONFIG_VALIDATORS[disposition.validator];
      if (!["package-glue", "command-router", ...Object.keys(TOOL_CONFIG_VALIDATORS)].includes(disposition.validator) || (disposition.disposition === "adapter-source") !== (disposition.validator === "package-glue") || (configValidatorOwner !== undefined && id !== configValidatorOwner)) problems.push(`packageSourceDispositions[${JSON.stringify(id)}] disposition/validator pair is invalid.`);
      if (!disposition.authority || !disposition.verification) problems.push(`packageSourceDispositions[${JSON.stringify(id)}] must declare authority and verification.`);
    }
  }

  if (record(taxonomy.generatorContracts, "generatorContracts")) {
    const contractIds = Object.keys(taxonomy.generatorContracts);
    if (contractIds.join("\0") !== [...contractIds].sort().join("\0")) problems.push("generatorContracts ids must be lexically ordered.");
    const opaqueRoots = Object.values(taxonomy.pathExclusions ?? {}).map((entry) => entry.path.replace(/\/$/u, ""));
    const exactTouchesOpaque = (value: string): boolean => opaqueRoots.some((opaque) => value === opaque || value.startsWith(`${opaque}/`) || opaque.startsWith(`${value}/`));
    const patternTouchesOpaque = (value: string): boolean => {
      const first = value.split("/")[0]!;
      return /[*?\[]/u.test(first) || opaqueRoots.some((opaque) => opaque.split("/")[0] === first);
    };
    const nxTarget = (value: unknown, key: string): value is string => {
      const valid = typeof value === "string" && /^(?:@[a-z0-9][a-z0-9._-]*\/)?[a-z0-9][a-z0-9._-]*:[a-z0-9][a-z0-9._-]*$/u.test(value);
      if (!valid) problems.push(`${key} must be one exact Nx project:target identity.`);
      return valid;
    };
    const outputOwners: { id: string; path: string }[] = [];
    const targets = new Map<string, string>();
    for (const [id, contract] of Object.entries(taxonomy.generatorContracts)) {
      if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id)) problems.push(`generatorContracts id ${JSON.stringify(id)} must be kebab-case.`);
      const allowedKeys = new Set(["ownership", "ownerPath", "target", "previewTarget", "previewArguments", "previewLimits", "checkTarget", "inputPatterns", "inputDiscovery", "compilerInputManifest", "packageGeneration", "currentPackageDestination", "projectionActivation", "outputRoots", "reason"]);
      for (const key of Object.keys(contract)) if (!allowedKeys.has(key)) problems.push(`generatorContracts[${JSON.stringify(id)}].${key} is forbidden.`);
      const ownership = contract.ownership as string;
      if (!["owned", "external"].includes(ownership)) problems.push(`generatorContracts[${JSON.stringify(id)}].ownership must be owned or external.`);
      if (!contract.reason) problems.push(`generatorContracts[${JSON.stringify(id)}].reason must be non-empty.`);
      const runnable = ownership === "owned";
      const targetKnown = runnable;
      if (targetKnown) {
        if (workspacePath(contract.ownerPath, `generatorContracts[${JSON.stringify(id)}].ownerPath`) && exactTouchesOpaque(contract.ownerPath)) problems.push(`generatorContracts[${JSON.stringify(id)}].ownerPath crosses an opaque boundary.`);
        if (nxTarget(contract.target, `generatorContracts[${JSON.stringify(id)}].target`)) {
          const prior = targets.get(contract.target);
          if (prior) problems.push(`generatorContracts ${JSON.stringify(prior)} and ${JSON.stringify(id)} duplicate target ${JSON.stringify(contract.target)}.`);
          targets.set(contract.target, id);
        }
      } else if (contract.ownerPath !== null || contract.target !== null) problems.push(`generatorContracts[${JSON.stringify(id)}] unowned/external classifications must have null ownerPath and target.`);
      if (runnable) {
        if (contract.previewTarget === undefined) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget is required for owned contracts.`);
        else if (nxTarget(contract.previewTarget, `generatorContracts[${JSON.stringify(id)}].previewTarget`) && typeof contract.target === "string") {
          try { generatorPreviewScriptArguments(contract); }
          catch (error) { problems.push(`generatorContracts[${JSON.stringify(id)}]: ${error instanceof Error ? error.message : String(error)}`); }
          const prior = targets.get(contract.previewTarget);
          if (prior) problems.push(`generatorContracts ${JSON.stringify(prior)} and ${JSON.stringify(id)} duplicate target ${JSON.stringify(contract.previewTarget)}.`);
          targets.set(contract.previewTarget, id);
        }
      } else if (contract.previewTarget !== undefined) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget is forbidden for external contracts.`);
      if (contract.previewArguments !== undefined && (!runnable || contract.previewTarget === undefined)) problems.push(`generatorContracts[${JSON.stringify(id)}].previewArguments requires an owned preview target.`);
      try { generatorPreviewResourceLimits(contract); } catch (error) { problems.push(`generatorContracts[${JSON.stringify(id)}].previewLimits: ${(error as Error).message}`); }
      if (contract.compilerInputManifest !== undefined) {
        const authority = contract.compilerInputManifest;
        if (!runnable || !authority || typeof authority !== "object" || Array.isArray(authority) || JSON.stringify(Object.keys(authority).sort()) !== JSON.stringify(["kind", "manifestOutputPath", "manifestSchemaPath", "maxFiles", "staticAuthorityPath"])) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest requires one closed owned authority.`);
        else {
          const literal = (path: unknown): path is string => typeof path === "string" && path === path.normalize("NFC") && path.length > 0 && !path.startsWith("/") && !/[\\*?\[\]\u0000-\u001f]/u.test(path) && !path.split("/").some(part => !part || part === "." || part === "..");
          if (authority.kind !== "compiler-input-manifest-v1" || !literal(authority.manifestOutputPath) || !literal(authority.manifestSchemaPath) || !literal(authority.staticAuthorityPath) || !Number.isSafeInteger(authority.maxFiles) || authority.maxFiles < 1 || authority.maxFiles > 10000) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest has invalid bounded fields.`);
          if (!contract.outputRoots?.some(root => authority.manifestOutputPath === root.path || authority.manifestOutputPath.startsWith(`${root.path}/`))) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest manifest is outside its output roots.`);
          for (const path of [authority.manifestSchemaPath, authority.staticAuthorityPath]) if (!contract.inputPatterns?.includes(path)) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest authority is not one exact input: ${JSON.stringify(path)}.`);
        }
      }
      if (contract.checkTarget !== undefined) {
        if (!targetKnown || !nxTarget(contract.checkTarget, `generatorContracts[${JSON.stringify(id)}].checkTarget`)) problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget requires a known target.`);
        else if (typeof contract.target === "string" && contract.checkTarget.slice(0, contract.checkTarget.lastIndexOf(":")) !== contract.target.slice(0, contract.target.lastIndexOf(":"))) problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget must belong to the target project.`);
      }
      if (id === "jco-package-adapter" && contract.currentPackageDestination === undefined) problems.push(`generatorContracts.${id} requires explicit current package destination authority.`);
      if (["jco-package-adapter", "wgpu-frame-worker"].includes(id) && contract.projectionActivation !== undefined) problems.push(`generatorContracts.${id} must use its current catalog instead of historical projection activation.`);
      if (contract.currentPackageDestination !== undefined) {
        try {
          const current = parseCurrentJcoPackageDestination(contract.currentPackageDestination);
          if (!runnable || id !== "jco-package-adapter" || contract.outputRoots.length !== 1 || contract.outputRoots[0]?.path !== current.adapterPath || contract.outputRoots[0]?.inclusion !== "tracked" || [current.cargoManifestPath, current.cargoLockPath, current.componentPath, current.witPath].some((path) => !contract.inputPatterns.includes(path))) problems.push(`generatorContracts.${id} current JCO ownership or inputs disagree.`);
        } catch (error) { problems.push(`generatorContracts.${id}.currentPackageDestination: ${error instanceof Error ? error.message : String(error)}`); }
      }
      if (id === "wgpu-frame-worker" && contract.packageGeneration === undefined) problems.push(`generatorContracts.${id} requires explicit package generation authority.`);
      if (contract.projectionActivation !== undefined) {
        const activation = contract.projectionActivation;
        exactKeys(activation, ["kind", "projectionContractId", "packageId", "sourceManifestPath", "destinationManifestPath"], `generatorContracts.${id}.projectionActivation`);
        if (!runnable || !["jco-package-adapter", "wgpu-frame-worker"].includes(id) || activation.kind !== "canonical-or-planned-package" || activation.projectionContractId !== "nested-cargo-packages-v1" || activation.packageId !== (id === "jco-package-adapter" ? "jcoprobe-guest" : "wgpu-renderer") || !exactOwnerPath(activation.sourceManifestPath) || !exactOwnerPath(activation.destinationManifestPath) || activation.sourceManifestPath === activation.destinationManifestPath) problems.push(`generatorContracts.${id}.projectionActivation has no exact package authority.`);
      }
      if (contract.packageGeneration !== undefined) {
        const generation = contract.packageGeneration;
        exactKeys(generation, ["kind", "previewInput", "catalogPath", "catalogSha256", "browserProfile"], `generatorContracts.${id}.packageGeneration`);
        const protocol = generation.previewInput;
        if (!runnable || id !== "wgpu-frame-worker" || contract.inputDiscovery || generation.kind !== "wgpu-package-artifacts" || !protocol || Object.keys(protocol).sort().join("\0") !== "maxBytes\0maxOperations\0protocol" || protocol.protocol !== "package-projected-inputs-v1" || protocol.maxBytes !== 67108864 || protocol.maxOperations !== 200000) problems.push(`generatorContracts.${id}.packageGeneration requires its exact bounded package protocol.`);
        try {
          const profile = parseSemanticPackageBrowserProfile(generation.browserProfile, taxonomy.pathEmojiPolicy.genericEmojiIdentities);
          if (generation.catalogPath !== profile.ownerPath + "/🪪️package-catalog.json" || !/^[0-9a-f]{64}$/u.test(generation.catalogSha256)) problems.push(`generatorContracts.${id}.packageGeneration requires its exact current catalog and digest.`);
          if (profile.entries.some((entry) => !contract.outputRoots.some((output) => output.path === `${profile.ownerPath}/${entry.outputRelativePath}` && output.inclusion === entry.inclusion))) problems.push(`generatorContracts.${id}.packageGeneration browser outputs disagree with ownership.`);
        } catch (error) { problems.push(`generatorContracts.${id}.packageGeneration: ${error instanceof Error ? error.message : String(error)}`); }
      }
      if (contract.inputDiscovery !== undefined) {
        const input = contract.inputDiscovery;
        exactKeys(input, ["kind", "previewInput", "descriptorRelativePath", "exampleDirectoryName", "exampleFileKindId", "implementationEntryPaths", "workspaceImports"], `generatorContracts[${JSON.stringify(id)}].inputDiscovery`);
        if (!runnable || id !== "plugin-registry" || input.kind !== "registry-catalog") problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery requires the owned registry-catalog authority.`);
        if (!input.previewInput || JSON.stringify(Object.keys(input.previewInput).sort()) !== JSON.stringify(["maxBytes", "maxOperations", "protocol"]) || input.previewInput.protocol !== "registry-projected-inputs-v1" || input.previewInput.maxBytes !== 67108864 || input.previewInput.maxOperations !== 200000) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery requires the bounded registry-projected-inputs-v1 preview protocol.`);
        if (input.descriptorRelativePath !== "../../🔣️.json" || input.exampleDirectoryName !== "📚️examples" || !taxonomy.artifactChildDirs.includes(input.exampleDirectoryName) || input.exampleFileKindId !== taxonomy.exampleFileKinds["🦀️rust"]) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery must preserve the exact descriptor and example vocabulary.`);
        if (!Array.isArray(input.implementationEntryPaths) || input.implementationEntryPaths.length === 0 || input.implementationEntryPaths.join("\0") !== [...new Set(input.implementationEntryPaths)].sort().join("\0")) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery implementation entries must be nonempty, unique and ordered.`);
        else for (const path of input.implementationEntryPaths) if (!workspacePath(path, `generatorContracts[${JSON.stringify(id)}].inputDiscovery entry`) || exactTouchesOpaque(path)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery entry crosses an opaque boundary.`);
        if (!input.workspaceImports || typeof input.workspaceImports !== "object" || Array.isArray(input.workspaceImports)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery.workspaceImports must be an object.`);
        else for (const [name, binding] of Object.entries(input.workspaceImports)) {
          exactKeys(binding, ["manifestPath", "entryPath"], `generatorContracts[${JSON.stringify(id)}].inputDiscovery.workspaceImports[${JSON.stringify(name)}]`);
          if (!/^@[a-z0-9-]+\/[a-z0-9-]+$/u.test(name)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery workspace import must be an exact package name.`);
          for (const path of [binding.manifestPath, binding.entryPath]) if (!workspacePath(path, `generatorContracts[${JSON.stringify(id)}].inputDiscovery workspace binding`) || exactTouchesOpaque(path)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery workspace binding crosses an opaque boundary.`);
        }
      }
      if (!Array.isArray(contract.inputPatterns) || (runnable && contract.inputPatterns.length === 0) || (!runnable && contract.inputPatterns.length > 0)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns must be non-empty only for owned contracts.`);
      if (Array.isArray(contract.inputPatterns)) {
        if (contract.inputPatterns.join("\0") !== [...contract.inputPatterns].sort().join("\0") || new Set(contract.inputPatterns).size !== contract.inputPatterns.length) problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns must be unique and lexically ordered.`);
        for (const [index, input] of contract.inputPatterns.entries()) {
          pathPattern(input, `generatorContracts[${JSON.stringify(id)}].inputPatterns[${index}]`);
          if (typeof input === "string" && patternTouchesOpaque(input)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns[${index}] can cross an opaque boundary.`);
        }
      }
      if (!Array.isArray(contract.outputRoots) || contract.outputRoots.length === 0) problems.push(`generatorContracts[${JSON.stringify(id)}].outputRoots must be non-empty.`);
      if (Array.isArray(contract.outputRoots)) {
        const outputPaths = contract.outputRoots.map((output) => output.path);
        if (outputPaths.join("\0") !== [...outputPaths].sort().join("\0") || new Set(outputPaths).size !== outputPaths.length) problems.push(`generatorContracts[${JSON.stringify(id)}].outputRoots must be unique and lexically ordered.`);
        for (const [index, output] of contract.outputRoots.entries()) {
          const key = `generatorContracts[${JSON.stringify(id)}].outputRoots[${index}]`;
          if (workspacePath(output.path, `${key}.path`) && exactTouchesOpaque(output.path)) problems.push(`${key}.path crosses an opaque boundary.`);
          if (!["tracked", "ignored"].includes(output.inclusion)) problems.push(`${key}.inclusion must be tracked or ignored.`);
          if (runnable && contract.inputPatterns.some((input) => pathMatcher.matches(output.path, input))) problems.push(`${key}.path is also declared as an input.`);
          outputOwners.push({ id, path: output.path });
        }
      }
    }
    for (let left = 0; left < outputOwners.length; left += 1) for (let right = left + 1; right < outputOwners.length; right += 1) {
      const a = outputOwners[left]!;
      const b = outputOwners[right]!;
      if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`)) problems.push(`generatorContracts ${JSON.stringify(a.id)} and ${JSON.stringify(b.id)} have overlapping output roots ${JSON.stringify(a.path)} and ${JSON.stringify(b.path)}.`);
    }
    const unsettled = Object.entries(taxonomy.generatorContracts).filter(([, contract]) => !["owned", "external"].includes(contract.ownership as string)).map(([id]) => id);
    if (unsettled.length > 0) problems.push(`generatorContracts must contain zero unknown or unsafe contracts; found ${unsettled.join(", ")}.`);
    for (const removed of ["ownerless-ui-icons", "root-layering-declarations"]) if (taxonomy.generatorContracts[removed]) problems.push(`generatorContracts.${removed} is false ownership and must remain absent.`);
    const ralphTrackedPaths = [
      ".ralph-tui/config.toml",
      ".ralph-tui/prd/kit-store-architecture-contracts-first-multi-backbone-pointer-based-rs-core/prd.json",
      ".ralph-tui/prd/kit-store-architecture-contracts-first-multi-backbone-pointer-based-rs-core/prd.md",
      ".ralph-tui/progress.md",
      ".ralph-tui/ralph.lock",
      ".ralph-tui/session-meta.json",
      ".ralph-tui/session.json",
    ];
    const setup = taxonomy.generatorContracts["setup-wizard-config"];
    if (!setup || setup.ownership !== "external" || setup.ownerPath !== null || setup.target !== null || setup.inputPatterns.length !== 0 || setup.outputRoots.some((output) => output.inclusion !== "tracked") || setup.outputRoots.map((output) => output.path).join("\0") !== ralphTrackedPaths.join("\0")) problems.push("generatorContracts.setup-wizard-config must externally own exactly the seven tracked Ralph files.");
    const ralphFileContracts: Readonly<Record<string, string>> = {
      "ralph-config": ".ralph-tui/config.toml",
      "ralph-lock": ".ralph-tui/ralph.lock",
      "ralph-prd-json": ".ralph-tui/prd/*/prd.json",
      "ralph-prd-markdown": ".ralph-tui/prd/*/prd.md",
      "ralph-progress": ".ralph-tui/progress.md",
      "ralph-session-meta": ".ralph-tui/session-meta.json",
      "ralph-session": ".ralph-tui/session.json",
    };
    for (const [id, expected] of Object.entries(ralphFileContracts)) {
      const contract = taxonomy.fixedFilenameContracts[id];
      if (!contract || contract.pathPattern !== expected || contract.authority !== "Ralph TUI" || contract.scope.kind !== "path-pattern") problems.push(`fixedFilenameContracts.${id} must be the exact Ralph-owned path contract ${JSON.stringify(expected)}.`);
    }
    const ralphDirectoryContracts: Readonly<Record<string, string>> = { "ralph-metadata": ".ralph-tui", "ralph-prd-root": ".ralph-tui/prd", "ralph-prd-identifier": ".ralph-tui/prd/*" };
    for (const [id, expected] of Object.entries(ralphDirectoryContracts)) {
      const contract = taxonomy.fixedDirectoryContracts[id];
      if (!contract || contract.pathPattern !== expected || contract.authority !== "Ralph TUI") problems.push(`fixedDirectoryContracts.${id} must be the exact Ralph-owned path contract ${JSON.stringify(expected)}.`);
    }
    for (const [id, contract] of [...Object.entries(taxonomy.fixedFilenameContracts), ...Object.entries(taxonomy.fixedDirectoryContracts)]) if (contract.pathPattern.startsWith(".ralph-tui/") && contract.pathPattern.includes("**")) problems.push(`Ralph contract ${JSON.stringify(id)} must not use a recursive wildcard.`);
    for (const output of outputOwners.filter((output) => output.path === ".ralph-tui" || output.path.startsWith(".ralph-tui/"))) if (output.id !== "setup-wizard-config") problems.push(`Ralph path ${JSON.stringify(output.path)} must be owned only by setup-wizard-config.`);
    const fixedRootManifests: Readonly<Record<string, string>> = { "root-package": "package.json", "root-cargo": "Cargo.toml", "root-go-work": "go.work" };
    for (const [id, expected] of Object.entries(fixedRootManifests)) if (taxonomy.fixedFilenameContracts[id]?.pathPattern !== expected) problems.push(`fixedFilenameContracts.${id} must remain the authored root manifest contract ${JSON.stringify(expected)}.`);
    const generatedRootManifests = outputOwners.filter((output) => ["package.json", "Cargo.toml", "go.work"].includes(output.path));
    if (generatedRootManifests.length > 0) problems.push("Root Bun, Cargo, and Go manifests are authored fixed contracts, not generator outputs.");
  }

  if (record(taxonomy.pathExclusions, "pathExclusions")) {
    const entries = Object.entries(taxonomy.pathExclusions);
    if (entries.map(([id]) => id).join("\0") !== "compose\0temp-compose") problems.push('pathExclusions must contain exactly ordered "compose" and "temp-compose" contracts.');
    const compose = taxonomy.pathExclusions.compose;
    if (!compose || compose.path !== "compose/" || compose.mode !== "opaque" || !compose.reason) problems.push('pathExclusions.compose must be the exact opaque "compose/" contract.');
    const tempCompose = taxonomy.pathExclusions["temp-compose"];
    if (!tempCompose || tempCompose.path !== "temp/compose/" || tempCompose.mode !== "opaque" || !tempCompose.reason) problems.push('pathExclusions.temp-compose must be the exact opaque "temp/compose/" contract.');
  }
  if (taxonomy.unicodeNormalization?.form !== "NFC" || taxonomy.unicodeNormalization?.caseFold !== "lower" || taxonomy.unicodeNormalization?.locale !== "und") problems.push("unicodeNormalization must be NFC/lower/und.");
  if (taxonomy.variationSelectorPolicy?.selector !== "\uFE0F" || taxonomy.variationSelectorPolicy?.requiredAfterEmoji !== true || taxonomy.variationSelectorPolicy?.comparison !== "ignore-selector") problems.push("variationSelectorPolicy is invalid.");
  if (taxonomy.pathEmojiPolicy?.inventory !== "git-visible" || taxonomy.pathEmojiPolicy?.identity !== "single-emoji-grapheme" || taxonomy.pathEmojiPolicy?.siblingNamespace !== "files-and-directories") problems.push("pathEmojiPolicy inventory, identity, and sibling namespace are invalid.");
  if (taxonomy.pathEmojiPolicy?.genericEmojiIdentities?.map(foldPathEmojiIdentity).join("\0") !== "📁\0📂\0📄") problems.push("pathEmojiPolicy.genericEmojiIdentities must reject the three generic file/folder glyphs.");
  if (!Array.isArray(taxonomy.pathEmojiPolicy?.reservedSubtreeDirectoryNames) || new Set(taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames.map(foldPathEmojiIdentity)).size !== taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames.length) problems.push("pathEmojiPolicy.reservedSubtreeDirectoryNames must be a unique array.");
  const comparisons = ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"];
  if (taxonomy.collisionPolicy?.comparisons?.join("\0") !== comparisons.join("\0")) problems.push(`collisionPolicy.comparisons must be exactly ${comparisons.join(", ")}.`);
  if (taxonomy.collisionPolicy?.maxPathBytes !== 240 || taxonomy.collisionPolicy?.rejectWindowsReservedNames !== true || taxonomy.collisionPolicy?.rejectTrailingDotsAndSpaces !== true) problems.push("collisionPolicy platform constraints must retain maxPathBytes 240 and reject reserved/trailing names.");
  if (taxonomy.areaEnforcement?.requiredState !== "clean" || taxonomy.areaEnforcement?.undeclaredAreas !== "enforce") problems.push("areaEnforcement must enforce clean declared and undeclared areas.");
  ids(taxonomy.areaEnforcement?.opaquePathExclusionIds, taxonomy.pathExclusions, "areaEnforcement.opaquePathExclusionIds");
  if (taxonomy.areaEnforcement?.opaquePathExclusionIds?.join("\0") !== "compose\0temp-compose") problems.push('areaEnforcement.opaquePathExclusionIds must be exactly ["compose", "temp-compose"].');
  if (record(taxonomy.areas, "areas")) for (const [area, state] of Object.entries(taxonomy.areas)) {
    if (area === "compose" || area.startsWith("compose/") || area === "temp/compose" || area.startsWith("temp/compose/")) problems.push("Opaque compose prefixes must exist only in pathExclusions.");
    if (state !== "clean") problems.push(`areas[${JSON.stringify(area)}] must be "clean".`);
  }
  if (record(taxonomy.areaLayers, "areaLayers")) for (const [area, layer] of Object.entries(taxonomy.areaLayers)) {
    if (area === "compose" || area.startsWith("compose/") || area === "temp/compose" || area.startsWith("temp/compose/")) problems.push("Opaque compose prefixes must not appear in areaLayers.");
    if (!["framework", "implementation", "repo-wide"].includes(layer)) problems.push(`areaLayers[${JSON.stringify(area)}] is invalid.`);
  }

  for (const [lang, ecosystem] of Object.entries(taxonomy.ecosystems ?? {})) {
    const oldShape = ecosystem as unknown as Record<string, unknown>;
    for (const key of ["manifestFilename", "moduleRootFilename", "entryFilenames", "leafFilename", "sourceExtension", "packagingDirNames"]) if (key in oldShape) problems.push(`ecosystems[${JSON.stringify(lang)}].${key} was removed.`);
    if (!['manifest', 'boundary-only'].includes(ecosystem.packageIdentity)) problems.push(`ecosystems[${JSON.stringify(lang)}].packageIdentity is invalid.`);
    if (ecosystem.packageIdentity === "manifest" && (!ecosystem.manifestContractId || !ecosystem.marker)) problems.push(`ecosystems[${JSON.stringify(lang)}] manifest identity requires a manifest contract and marker.`);
    if (ecosystem.packageIdentity === "boundary-only" && (ecosystem.manifestContractId !== null || ecosystem.marker !== null)) problems.push(`ecosystems[${JSON.stringify(lang)}] boundary-only identity cannot declare a manifest or marker.`);
    if (ecosystem.manifestContractId && !taxonomy.fixedFilenameContracts[ecosystem.manifestContractId]) problems.push(`ecosystems[${JSON.stringify(lang)}].manifestContractId is missing.`);
    if (ecosystem.moduleRootContractId && !taxonomy.fixedFilenameContracts[ecosystem.moduleRootContractId]) problems.push(`ecosystems[${JSON.stringify(lang)}].moduleRootContractId is missing.`);
    if (!taxonomy.fileKinds[ecosystem.componentFileKindId]) problems.push(`ecosystems[${JSON.stringify(lang)}].componentFileKindId is missing.`);
    ids(ecosystem.sourceFileKindIds, taxonomy.fileKinds, `ecosystems[${JSON.stringify(lang)}].sourceFileKindIds`);
    ids(ecosystem.entryContractIds, taxonomy.configurableEntryContracts, `ecosystems[${JSON.stringify(lang)}].entryContractIds`);
    ids(ecosystem.packagingDirectoryKindIds ?? [], taxonomy.semanticDirectoryKinds, `ecosystems[${JSON.stringify(lang)}].packagingDirectoryKindIds`);
    if (!taxonomy.packageBoundaryRules[lang]) problems.push(`ecosystems[${JSON.stringify(lang)}] has no packageBoundaryRules entry.`);
  }
  for (const [target, spec] of Object.entries(taxonomy.targets ?? {})) {
    const oldShape = spec as unknown as Record<string, unknown>;
    for (const key of ["leafFilename", "entryFilenames"]) if (key in oldShape) problems.push(`targets[${JSON.stringify(target)}].${key} was removed.`);
    if (!taxonomy.ecosystems[spec.lang]) problems.push(`targets[${JSON.stringify(target)}].lang is missing.`);
    if (!taxonomy.fileKinds[spec.componentFileKindId]) problems.push(`targets[${JSON.stringify(target)}].componentFileKindId is missing.`);
    ids(spec.entryContractIds, taxonomy.configurableEntryContracts, `targets[${JSON.stringify(target)}].entryContractIds`);
  }

  const mappings: [string, Readonly<Record<string, string>>][] = [
    ["componentFileKinds", taxonomy.componentFileKinds], ["exampleFileKinds", taxonomy.exampleFileKinds],
    ["exampleTestFileKinds", taxonomy.exampleTestFileKinds], ["testAdapterFileKinds", taxonomy.testAdapterFileKinds],
    ["artifactSpecFileKinds", taxonomy.artifactSpecFileKinds], ["artifactSchemaSpecFileKinds", taxonomy.artifactSchemaSpecFileKinds],
    ["surfaceSchemaSpecFileKinds", taxonomy.surfaceSchemaSpecFileKinds],
  ];
  for (const [key, mapping] of mappings) for (const [owner, kindId] of Object.entries(mapping ?? {})) if (!taxonomy.fileKinds[kindId]) problems.push(`${key}[${JSON.stringify(owner)}] references missing kind ${JSON.stringify(kindId)}.`);
  for (const [key, kindId] of [
    ["semanticManifestFileKindId", taxonomy.semanticManifestFileKindId], ["subsetsManifestFileKindId", taxonomy.subsetsManifestFileKindId],
    ["storyFileKindId", taxonomy.storyFileKindId], ["testFeatureFileKindId", taxonomy.testFeatureFileKindId],
    ["testContributionFileKindId", taxonomy.testContributionFileKindId], ["testOutputMarkerFileKindId", taxonomy.testOutputMarkerFileKindId],
    ["windowEmptyFacetFileKindId", taxonomy.windowEmptyFacetFileKindId], ["mutationComponentFileKindId", taxonomy.mutationComponentFileKindId], ["mutationDescriptorFileKindId", taxonomy.mutationDescriptorFileKindId], ["testOracleRegistryLocation.fileKindId", taxonomy.testOracleRegistryLocation?.fileKindId],
    ["testSchemaLocation.fileKindId", taxonomy.testSchemaLocation?.fileKindId],
  ] as const) if (!kindId || !taxonomy.fileKinds[kindId]) problems.push(`${key} references a missing file kind.`);
  if (record(taxonomy.testContributionDirectoryOverrides, "testContributionDirectoryOverrides")) {
    for (const [owner, name] of Object.entries(taxonomy.testContributionDirectoryOverrides)) {
      if (!owner || owner.startsWith("/") || owner !== owner.normalize("NFC") || /[\\*?{}\0]/u.test(owner) || owner !== "." && owner.split("/").some((segment) => !segment || segment === "." || segment === "..")) problems.push(`testContributionDirectoryOverrides[${JSON.stringify(owner)}] must name one exact repository owner.`);
      if (typeof name !== "string" || leadingEmojiIdentity(name).rest !== "oracle" || pathEmojiStatuteFindings([{ path: name, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities).length > 0 || !semanticDirectoryKindId(name, taxonomy)) problems.push(`testContributionDirectoryOverrides[${JSON.stringify(owner)}] must name one registered single-emoji oracle directory.`);
    }
  }
  if (taxonomy.semanticManifestFilenameOverrides !== undefined && record(taxonomy.semanticManifestFilenameOverrides, "semanticManifestFilenameOverrides")) {
    for (const [owner, name] of Object.entries(taxonomy.semanticManifestFilenameOverrides)) {
      if (!owner || owner.startsWith("/") || owner !== owner.normalize("NFC") || /[\\*?{}\0]/u.test(owner) || owner.split("/").some((segment) => !segment || segment === "." || segment === "..")) problems.push(`semanticManifestFilenameOverrides[${JSON.stringify(owner)}] must name one exact repository-relative collection owner.`);
      const identity = leadingEmojiIdentity(name);
      const statuteProblems = pathEmojiStatuteFindings([{ path: name, nodeKind: "file" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities);
      if (!canonicalTaxonomyEmoji(identity.first) || identity.rest !== "manifest.json" || statuteProblems.some((problem) => problem.kind !== "duplicate") || fileKindIdForSourcePath(name, taxonomy) !== taxonomy.semanticManifestFileKindId) problems.push(`semanticManifestFilenameOverrides[${JSON.stringify(owner)}] must name one canonical semantic emoji followed by "manifest.json" in the semantic manifest file kind.`);
    }
  }
  if (taxonomy.subsetDirectoryOverrides !== undefined && record(taxonomy.subsetDirectoryOverrides, "subsetDirectoryOverrides")) {
    const subsetSlug = new RegExp(taxonomy.subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$", "u");
    const anyId = taxonomy.subsetAnyId ?? "*";
    for (const [owner, directories] of Object.entries(taxonomy.subsetDirectoryOverrides)) {
      if (!owner || owner.startsWith("/") || owner !== owner.normalize("NFC") || /[\\*?{}\0]/u.test(owner) || owner.split("/").some((segment) => !segment || segment === "." || segment === "..") || posix.basename(owner) !== taxonomy.subsetsDirName) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}] must name one exact repository-relative ${JSON.stringify(taxonomy.subsetsDirName)} owner.`);
      if (!record(directories, `subsetDirectoryOverrides[${JSON.stringify(owner)}]`)) continue;
      const physicalNames = new Set<string>();
      for (const [id, name] of Object.entries(directories)) {
        if (id !== anyId && !subsetSlug.test(id)) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}][${JSON.stringify(id)}] must use the canonical logical subset id grammar.`);
        const expectedRest = id === anyId ? "any" : id;
        const identity = leadingEmojiIdentity(name);
        const statuteProblems = pathEmojiStatuteFindings([{ path: name, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities);
        if (!canonicalTaxonomyEmoji(identity.first) || identity.rest !== expectedRest || statuteProblems.some((problem) => problem.kind !== "duplicate")) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}][${JSON.stringify(id)}] must name one canonical semantic emoji followed by ${JSON.stringify(expectedRest)}.`);
        if (physicalNames.has(foldPathEmojiIdentity(name))) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}] has duplicate physical directory ${JSON.stringify(name)}.`);
        physicalNames.add(foldPathEmojiIdentity(name));
      }
      if (!Object.hasOwn(directories, anyId)) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}] must declare the unconstrained subset id ${JSON.stringify(anyId)}.`);
    }
  }
  ids(taxonomy.textSpecFileKinds, taxonomy.fileKinds, "textSpecFileKinds");
  ids(taxonomy.binarySpecFileKinds, taxonomy.fileKinds, "binarySpecFileKinds");
  ids(taxonomy.rootDataContractIds, taxonomy.fixedFilenameContracts, "rootDataContractIds");
  ids(taxonomy.rootDocumentContractIds, taxonomy.fixedFilenameContracts, "rootDocumentContractIds");
  ids(taxonomy.repoWideContractIds, taxonomy.fixedFilenameContracts, "repoWideContractIds");
  ids(taxonomy.layeringGeneratedContractIds, taxonomy.fixedFilenameContracts, "layeringGeneratedContractIds");
  if (taxonomy.layeringGeneratedContractIds.length !== 0) problems.push("layeringGeneratedContractIds must be empty until an exact deterministic writer exists.");

  for (const [formatId, format] of Object.entries(taxonomy.schemaFormats ?? {})) {
    if ("leafFilename" in (format as unknown as Record<string, unknown>) || "extension" in (format as unknown as Record<string, unknown>)) problems.push(`schemaFormats[${JSON.stringify(formatId)}] contains removed filename fields.`);
    if (!taxonomy.fileKinds[format.fileKindId]) problems.push(`schemaFormats[${JSON.stringify(formatId)}].fileKindId is missing.`);
    if (!["snake", "camel", "kebab"].includes(format.fieldCasing)) problems.push(`schemaFormats[${JSON.stringify(formatId)}].fieldCasing is invalid.`);
  }
  for (const [kindId, kind] of Object.entries(taxonomy.schemaFacetKinds ?? {})) {
    if (!kind.formats.includes(kind.normativeFormat)) problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}].formats must include its normative format.`);
    for (const formatId of kind.formats) if (!taxonomy.schemaFormats[formatId]) problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}] references missing format ${JSON.stringify(formatId)}.`);
  }

  fullPattern(taxonomy.mutationDirectoryPattern, "mutationDirectoryPattern");
  problems.push(...mutationDomainOwnersProblems(taxonomy.mutationDomainOwners, taxonomy.pathEmojiPolicy.genericEmojiIdentities));
  problems.push(...mutationCatalogSourceOwnersProblems(taxonomy));
  const mutationBehaviorFacetDirs = ["🦠️mutation", "🔺️diff", "↩️inverse"];
  if (taxonomy.mutationBehaviorFacetDirs?.join("\0") !== mutationBehaviorFacetDirs.join("\0")) problems.push(`mutationBehaviorFacetDirs must contain exactly ${mutationBehaviorFacetDirs.join(", ")} in canonical order.`);
  const mutationOrganizationalFacetDirs = ["🧩️plan", "📝️text", "💾️binary", "🧬️schema"];
  if (taxonomy.mutationOrganizationalFacetDirs?.join("\0") !== mutationOrganizationalFacetDirs.join("\0")) problems.push(`mutationOrganizationalFacetDirs must contain exactly ${mutationOrganizationalFacetDirs.join(", ")} in canonical order.`);
  for (const dir of [...(taxonomy.mutationBehaviorFacetDirs ?? []), ...(taxonomy.mutationOrganizationalFacetDirs ?? [])]) if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) problems.push(`taxonomyLeafParentDirs must include mutation facet ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.mutationBehaviorFacetDirs ?? []) if (dir !== "🦠️mutation" && !taxonomy.mutationDirectLeafForbiddenRegionMarkers?.[dir]) problems.push(`mutationDirectLeafForbiddenRegionMarkers must name the forbidden inline region marker for behavior facet ${JSON.stringify(dir)}.`);
  if (taxonomy.mutationDirectLeafForbiddenRegionMarkers?.["🔺️diff"] !== "🔖️Diff" || taxonomy.mutationDirectLeafForbiddenRegionMarkers?.["↩️inverse"] !== "🔖️Inverse") problems.push('mutationDirectLeafForbiddenRegionMarkers must map "🔺️diff" to "🔖️Diff" and "↩️inverse" to "🔖️Inverse".');
  if (taxonomy.mutationComponentFileKindId !== "rust-source") problems.push('mutationComponentFileKindId must select the direct owner "rust-source" kind.');
  if (taxonomy.mutationDescriptorFileKindId !== "json") problems.push('mutationDescriptorFileKindId must select the language-neutral "json" kind.');
  if (record(taxonomy.mutationPayloadSchemaLocation, "mutationPayloadSchemaLocation")) {
    const location = taxonomy.mutationPayloadSchemaLocation;
    if (Object.keys(location).sort().join("\0") !== "directoryKindId\0directoryName\0fileKindId" || location.directoryKindId !== "schema" || location.directoryName !== "🧬️schema" || location.fileKindId !== "json" || semanticDirectoryKindId(location.directoryName, taxonomy) !== location.directoryKindId) problems.push("mutationPayloadSchemaLocation must select the registered schema directory and JSON physical leaf.");
  }

  if (record(taxonomy.mutationPayloadSchemaAuthority, "mutationPayloadSchemaAuthority")) {
    const expected = { contractKind: "descriptor-linked-mutation-payload-schema", ownerAuthority: "mutationOwnerIdentity", descriptorFileKindId: "json", descriptorField: "payloadSchema", descriptorSchemaVersion: 1, descriptorCardinality: "one-canonical-no-competing-descriptor", descriptorOwnerField: "owner", descriptorIdentityField: "semanticKind", jsonSchemaDialect: "http://json-schema.org/draft-07/schema#", targetAuthority: "owner-relative-regular-json-schema" };
    const authority = taxonomy.mutationPayloadSchemaAuthority;
    if (Object.keys(authority).length !== Object.keys(expected).length || Object.entries(expected).some(([key, value]) => authority[key as keyof typeof authority] !== value)) problems.push("mutationPayloadSchemaAuthority must bind exact mutation descriptors to regular owner-contained JSON Schema files.");
  }

  const directoryValues = [
    taxonomy.packagesDirName, taxonomy.targetsDirName, taxonomy.elementsDirName, taxonomy.artifactsDirName, taxonomy.modesDirName,
    taxonomy.windowsDirName, taxonomy.standardsDirName, taxonomy.subsetsDirName, taxonomy.viewerDirName, taxonomy.editorDirName,
    taxonomy.exampleAssetsDirName, taxonomy.exampleTestsDirName, ...taxonomy.artifactChildDirs, ...taxonomy.newArtifactChildDirs,
    ...taxonomy.standardChildDirs, ...taxonomy.subsetChildDirs, ...taxonomy.surfaceChildDirs, ...taxonomy.modeChildDirs,
    ...taxonomy.windowChildDirs, ...taxonomy.taxonomyLeafParentDirs, ...taxonomy.pluginChildDirs, ...taxonomy.osChildDirs,
    ...taxonomy.rootDataDirNames, ...taxonomy.schemaChildDirs, ...taxonomy.representationDirs, ...taxonomy.ioDirectionDirs,
    ...taxonomy.ioSemanticCollectionDirNames, ...Object.values(taxonomy.ioDirectionChildDirs), ...taxonomy.mutationBehaviorFacetDirs, ...taxonomy.mutationOrganizationalFacetDirs,
  ];
  for (const directory of new Set(directoryValues)) if (!semanticDirectoryKindId(directory, taxonomy)) problems.push(`Semantic directory ${JSON.stringify(directory)} is not uniquely registered.`);
  for (const dir of taxonomy.artifactComponentDirs) if (!taxonomy.artifactChildDirs.includes(dir)) problems.push(`artifactChildDirs must include ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.windowRequiredChildDirs) if (!taxonomy.windowChildDirs.includes(dir)) problems.push(`windowChildDirs must include ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.surfaceRequiredChildDirs) if (!taxonomy.surfaceChildDirs.includes(dir)) problems.push(`surfaceChildDirs must include ${JSON.stringify(dir)}.`);
  return problems;
}

/** 🏗️ Verifies known Nx identities and required tracked outputs against live workspace metadata. */
/** 🧾️ Frozen leaf ownership and byte preimage, independent of implementation language. */
export interface SemanticExactOwnedFileCase {
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly ownerEvidenceId: string;
  readonly disposition: "attribution-relocate" | "configurable-owner-license-relocate" | "fixed" | "generated-evidence-relocate" | "owner-documentation-relocate";
  readonly fixedContractId: "bun-package-license" | "bun-package-readme" | null;
  readonly projectionContractId: "exact-owner-license-projection" | "exact-owner-readme-projection" | null;
  readonly referenceOwnerIds: readonly string[];
  readonly generatorOwnerId: "assets-build" | null;
  readonly preimage: Readonly<{ sha256: string; mode: string; size: number }>;
}

/** 📚️ One digest-locked owner catalog, including its concrete consumers and owner evidence. */
export interface SemanticExactOwnedFileCatalog {
  readonly cases: readonly SemanticExactOwnedFileCase[];
  readonly ownerEvidence: Readonly<Record<string, Readonly<{ kind: string; evidencePaths: readonly string[]; expectedPackageName?: string; private?: boolean }>>>;
  readonly referenceOwners: Readonly<Record<string, Readonly<{ kind: string; ownerPath: string }>>>;
  readonly generatorOwners: Readonly<Record<string, Readonly<{ ownerPath: string; target: string; currentOutputPath: string; requiredOutputPath: string }>>>;
}

//#region 📦️Nested Cargo Package Authority
/** 🧷️ One exact admitted Cargo-package source leaf and its semantic destination. */
export interface SemanticPackageProjectionMapping {
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly sourceHash: string;
  readonly sourceSize: number;
  readonly disposition: "metadata" | "implementation" | "adapter" | "generated" | "tool-metadata";
  readonly sourceRole: "implementation" | "unresolved" | null;
}

/** 🔌️ Deterministic package entry derived from its externally configured semantic target. */
export interface SemanticPackageAdapter {
  readonly id: string;
  readonly path: string;
  readonly language: "rust" | "typescript";
  readonly expectedRole: "declaration";
  readonly targetPaths: readonly string[];
  readonly content: string;
}

/** 🧷️ Exact source-owned replacement or registration merge, never a generated-output overwrite. */
export interface SemanticPackageSourceSplice {
  readonly id: "library-adapter-replacement" | "renderer-registration-merge";
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly oldValue: string;
  readonly newValue: string;
}

/** 📇️ Source-hash-bound semantic registration output outside every package boundary. */
export interface SemanticPackageDerivedLeaf {
  readonly id: "wgpu-renderer-registration";
  readonly path: string;
  readonly originSourcePath: string;
  readonly originSourceHash: string;
  readonly language: "rust";
  readonly expectedRole: "implementation";
  readonly content: string;
}

/** ♻️ Exact obsolete generated source; canonical bytes are owned independently by its preview. */
export interface SemanticPackageGeneratedSourceRetirement {
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly generatorContractId: "wgpu-frame-worker";
  readonly sourceMode: 420;
}

/** 🧷️ Reviewed authored fragment, bound to its complete package source preimage. */
export interface SemanticPackageAuthoredSourceFragment {
  readonly id: string;
  readonly kind: "reference-path" | "cargo-runtime-coordinate" | "tool-producer";
  readonly sourcePath: string;
  readonly oldValue: string;
  readonly newValue: string;
}

/** 🌐️ Closed browser compiler profile with exact module and workspace import authority. */
export interface SemanticPackageBrowserProfile {
  readonly schemaVersion: 1;
  readonly kind: "wgpu-browser-esm-v1";
  readonly inlineTestDefine: "undefined";
  readonly ownerPath: string;
  readonly entries: readonly Readonly<{ id: "frame-worker" | "browser-boot"; sourceRelativePath: string; outputRelativePath: string; inclusion: "tracked" | "ignored" }>[];
  readonly workspaceImports: Readonly<Record<string, Readonly<{ manifestPath: string; entryPath: string }>>>;
  readonly sourceModulePaths: readonly string[];
}

/** 📏️ Validates explicit browser input authority without discovering or reading unrelated modules. */
export function parseSemanticPackageBrowserProfile(input: unknown, genericEmojiIdentities: readonly string[]): SemanticPackageBrowserProfile {
  const exact = (value: unknown, keys: readonly string[]): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join("\0") === [...keys].sort().join("\0");
  if (!exact(input, ["schemaVersion", "kind", "inlineTestDefine", "ownerPath", "entries", "workspaceImports", "sourceModulePaths"]) || input.schemaVersion !== 1 || input.kind !== "wgpu-browser-esm-v1" || input.inlineTestDefine !== "undefined" || !exactOwnerPath(input.ownerPath)) throw new Error("Invalid WGPU browser profile");
  const expected = [{ id: "frame-worker", inclusion: "tracked" }, { id: "browser-boot", inclusion: "ignored" }];
  if (!Array.isArray(input.entries) || input.entries.length !== expected.length || input.entries.some((entry, index) => !exact(entry, ["id", "sourceRelativePath", "outputRelativePath", "inclusion"]) || entry.id !== expected[index]!.id || entry.inclusion !== expected[index]!.inclusion || !exactOwnerPath(entry.sourceRelativePath) || !exactOwnerPath(entry.outputRelativePath))) throw new Error("WGPU browser entry authority drift");
  const directories = input.entries.map((entry) => ({ path: dirname(entry.sourceRelativePath).replaceAll("\\", "/"), nodeKind: "directory" as const }));
  if (pathEmojiStatuteFindings(directories, genericEmojiIdentities).length || input.entries.some((entry, index) => directories[index]!.path.includes("/") || leadingEmojiIdentity(directories[index]!.path).rest !== entry.id || entry.sourceRelativePath !== directories[index]!.path + "/🟦️.ts" || entry.outputRelativePath !== directories[index]!.path + "/🤖️generated/🟨️.js")) throw new Error("WGPU browser entry requires one explicit semantic source and output owner");
  if (!Array.isArray(input.sourceModulePaths) || input.sourceModulePaths.length < expected.length || input.sourceModulePaths.some((path, index, paths) => !exactOwnerPath(path) || !/\.(?:tsx?|m?js|json)$/u.test(path) || index > 0 && projectionByteCompare(paths[index - 1], path) >= 0)) throw new Error("WGPU browser module paths must be exact, nonopaque, unique and byte ordered");
  if (input.entries.some((entry) => !(input.sourceModulePaths as string[]).includes(input.ownerPath + "/" + entry.sourceRelativePath))) throw new Error("WGPU browser entry is not an exact declared current module");
  if (!exact(input.workspaceImports, ["@semio-tech/framework", "@semio-tech/framework-os", "@semio-tech/framework-replication"])) throw new Error("WGPU browser workspace import identities drifted");
  for (const binding of Object.values(input.workspaceImports)) if (!exact(binding, ["manifestPath", "entryPath"]) || !exactOwnerPath(binding.manifestPath) || basename(binding.manifestPath) !== "package.json" || !exactOwnerPath(binding.entryPath) || !input.sourceModulePaths.includes(binding.entryPath)) throw new Error("WGPU browser workspace import binding drifted");
  return input as unknown as SemanticPackageBrowserProfile;
}

/** 🪪️ Current WGPU package declarations, independent of frozen projection preimages. */
export interface CanonicalWgpuPackageCatalog {
  readonly $schema: "./🧬️package-catalog.schema.json";
  readonly schemaVersion: 1;
  readonly kind: "canonical-wgpu-package";
  readonly ownerPath: string;
  readonly packageRelativePath: "📦️packages/🦀️rust";
  readonly identity: Readonly<{ cargoPackageName: string; nodePackageName: string; nxProjectName: string }>;
  readonly entryPaths: Readonly<{ cargoLibrary: string; cargoBinary: string; cargoBuild: "build.rs"; nodeLibrary: string }>;
  readonly artifacts: readonly Readonly<{ id: string; relativePath: string; targetRelativePath: string | null; language: "rust" | "typescript"; role: "declaration" | "implementation"; content: string }>[];
}

/** 🔐️ Validates one digest-bound current catalog without weakening historical source checks. */
export function parseCanonicalWgpuPackageCatalog(bytes: string, digest: string, profile: SemanticPackageBrowserProfile, taxonomy: Taxonomy): CanonicalWgpuPackageCatalog {
  if (createHash("sha256").update(bytes).digest("hex") !== digest) throw new Error("Current WGPU package catalog digest drift");
  const row = JSON.parse(bytes) as CanonicalWgpuPackageCatalog;
  const exact = (value: unknown, keys: readonly string[]): boolean => value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join("\0") === [...keys].sort().join("\0");
  if (!exact(row, ["$schema", "schemaVersion", "kind", "ownerPath", "packageRelativePath", "identity", "entryPaths", "artifacts"]) || row.$schema !== "./🧬️package-catalog.schema.json" || row.schemaVersion !== 1 || row.kind !== "canonical-wgpu-package" || row.ownerPath !== profile.ownerPath || row.packageRelativePath !== "📦️packages/🦀️rust") throw new Error("Current WGPU package catalog identity drift");
  if (!exact(row.identity, ["cargoPackageName", "nodePackageName", "nxProjectName"]) || row.identity.cargoPackageName !== "semio-framework-os-renderer-wgpu" || row.identity.nodePackageName !== "@semio-tech/framework-renderer-wgpu" || row.identity.nxProjectName !== row.identity.nodePackageName || !exact(row.entryPaths, ["cargoLibrary", "cargoBinary", "cargoBuild", "nodeLibrary"]) || Object.values(row.entryPaths).some((path) => !exactOwnerPath(path)) || row.entryPaths.cargoBuild !== "build.rs") throw new Error("Current WGPU package manifest authority drift");
  const ids = ["build-adapter", "binary-adapter", "typescript-adapter", "renderer-registration"];
  if (!Array.isArray(row.artifacts) || row.artifacts.length !== ids.length || row.artifacts.some((artifact, index) => !exact(artifact, ["id", "relativePath", "targetRelativePath", "language", "role", "content"]) || artifact.id !== ids[index] || !exactOwnerPath(artifact.relativePath) || artifact.targetRelativePath !== null && !exactOwnerPath(artifact.targetRelativePath) || !["rust", "typescript"].includes(artifact.language) || !["declaration", "implementation"].includes(artifact.role) || typeof artifact.content !== "string" || classifyPackageSourceRole(artifact.content, taxonomy.packageGlueGrammar[artifact.language]) !== artifact.role) || new Set(row.artifacts.map((artifact) => artifact.relativePath)).size !== ids.length) throw new Error("Current WGPU package artifact authority drift");
  for (const artifact of row.artifacts) {
    if (artifact.id === "renderer-registration") {
      if (artifact.relativePath !== "🧊️renderer/📇️registry/🦀️.rs" || artifact.targetRelativePath !== null || artifact.role !== "implementation") throw new Error("Current WGPU registration ownership drift");
    } else {
      if (!artifact.relativePath.startsWith(row.packageRelativePath + "/") || !artifact.targetRelativePath || artifact.role !== "declaration") throw new Error("Current WGPU adapter ownership drift");
      const relativeTarget = posix.relative(posix.dirname(artifact.relativePath), artifact.targetRelativePath);
      if (!artifact.content.includes(JSON.stringify(relativeTarget))) throw new Error("Current WGPU adapter target drift");
    }
  }
  return row;
}

/** 🏠️ One exact Cargo identity; standalone fixtures are not inferred from directory basenames. */
export interface SemanticPackageProjectionCase {
  readonly id: "wgpu-renderer" | "jcoprobe-guest";
  readonly sourceRoot: string;
  readonly destinationRoot: string;
  readonly semanticOwnerRoot: string;
  readonly workspaceKind: "repository" | "standalone";
  readonly identity: Readonly<{ cargoPackageName: string; nodePackageName?: string; nxProjectName?: string }>;
  readonly requiredManifestEvidence: Readonly<Record<string, unknown>>;
  readonly ignoredSourcePatterns: readonly string[];
  readonly mappings: readonly SemanticPackageProjectionMapping[];
  readonly adapters: readonly SemanticPackageAdapter[];
  readonly sourceSplices: readonly SemanticPackageSourceSplice[];
  readonly derivedLeaves: readonly SemanticPackageDerivedLeaf[];
  readonly joinedPathBindings: readonly SemanticPackageJoinedPathBinding[];
  readonly generatedSourceRetirements: readonly SemanticPackageGeneratedSourceRetirement[];
  readonly authoredSourceFragments: readonly SemanticPackageAuthoredSourceFragment[];
}

function semanticPackageAuthoredFragmentProblem(owner: SemanticPackageProjectionCase): string | null {
  const fragments = owner.authoredSourceFragments;
  if (!Array.isArray(fragments) || fragments.length > 64 || new Set(fragments.map((fragment) => fragment.id)).size !== fragments.length) return "invalid or duplicated declaration";
  for (const fragment of fragments) {
    const mapping = owner.mappings.find((mapping) => mapping.sourcePath === fragment.sourcePath);
    if (Object.keys(fragment).sort().join("\0") !== "id\0kind\0newValue\0oldValue\0sourcePath" || !/^[a-z][a-z0-9-]*$/u.test(fragment.id) || !["reference-path", "cargo-runtime-coordinate", "tool-producer"].includes(fragment.kind) || !exactOwnerPath(fragment.sourcePath) || !mapping || mapping.disposition === "generated" || typeof fragment.oldValue !== "string" || !fragment.oldValue || typeof fragment.newValue !== "string" || !fragment.newValue || Buffer.byteLength(fragment.oldValue) > 262144 || Buffer.byteLength(fragment.newValue) > 262144 || fragment.kind === "cargo-runtime-coordinate" && !fragment.sourcePath.endsWith(".rs")) return "unowned or malformed declaration";
  }
  return null;
}

/** 🔐️ Projects only exact reviewed source spans; all other program bytes remain authored. */
export function semanticPackageAuthoredFragmentReferences(
  facts: Readonly<{ path: string; content: string; layout: "source" | "destination" }>,
  owner: SemanticPackageProjectionCase,
): Readonly<{ references: readonly SemanticPackageJoinedPathReference[]; problems: readonly string[] }> {
  const reject = (message: string) => ({ references: [], problems: ["Nested Cargo authored fragment: " + message] });
  const malformed = semanticPackageAuthoredFragmentProblem(owner);
  if (malformed) return reject(malformed);
  const fragments = owner.authoredSourceFragments;
  const mapping = owner.mappings.find((mapping) => facts.path === (facts.layout === "source" ? mapping.sourcePath : mapping.destinationPath));
  if (!mapping || !["source", "destination"].includes(facts.layout)) return reject("unknown source or canonical owner");
  if (facts.layout === "source" && (createHash("sha256").update(facts.content).digest("hex") !== mapping.sourceHash || Buffer.byteLength(facts.content) !== mapping.sourceSize)) return reject("complete source preimage changed");
  const references: SemanticPackageJoinedPathReference[] = [];
  for (const fragment of fragments.filter((fragment) => fragment.sourcePath === mapping.sourcePath)) {
    const value = facts.layout === "source" ? fragment.oldValue : fragment.newValue, start = facts.content.indexOf(value);
    if (start < 0 || facts.content.indexOf(value, start + value.length) >= 0) return reject("missing or repeated span " + fragment.id);
    references.push({ start, end: start + value.length, oldValue: value, newValue: fragment.newValue, targetSourcePath: mapping.sourcePath });
  }
  references.sort((left, right) => left.start - right.start || left.end - right.end);
  if (references.some((reference, index) => index > 0 && reference.start < references[index - 1]!.end)) return reject("overlapping spans");
  return { references: facts.layout === "source" ? references : [], problems: [] };
}

//#region 🧵️Source-Owned Joined Paths
/** 🧵️ Exact local directory binding and its source-owned file reads. */
export interface SemanticPackageJoinedPathBinding {
  readonly kind: "source-owned-joined-path-bindings";
  readonly id: string;
  readonly consumerRelativePath: string;
  readonly sourceDirectoryRelativePath: string;
  readonly destinationDirectory: "semantic-owner";
  readonly rootBinding: string;
  readonly directoryBinding: string;
  readonly reads: readonly Readonly<{ binding: string; relativePath: string }>[];
}

/** 🧷️ One exact literal operand; executable source bytes are never synthesized. */
export interface SemanticPackageJoinedPathReference {
  readonly start: number;
  readonly end: number;
  readonly oldValue: string;
  readonly newValue: string;
  readonly targetSourcePath: string;
}

function semanticPackageJoinedPathBindingRecords(contract: SemanticPackageJoinedPathBinding, owner: SemanticPackageProjectionCase) {
  const reject = (message: string): never => { throw new Error("Nested Cargo joined-path authority: " + message); };
  const keys = (value: object, expected: readonly string[]) => JSON.stringify(Object.keys(value).sort()) === JSON.stringify([...expected].sort());
  const identifier = (value: unknown): value is string => typeof value === "string" && /^[A-Za-z_$][\w$]*$/u.test(value);
  if (!contract || !keys(contract, ["kind", "id", "consumerRelativePath", "sourceDirectoryRelativePath", "destinationDirectory", "rootBinding", "directoryBinding", "reads"]) || contract.kind !== "source-owned-joined-path-bindings" || typeof contract.id !== "string" || !/^[a-z][a-z0-9-]*$/u.test(contract.id) || contract.destinationDirectory !== "semantic-owner" || !identifier(contract.rootBinding) || !identifier(contract.directoryBinding) || !exactOwnerPath(contract.consumerRelativePath) || !exactOwnerPath(contract.sourceDirectoryRelativePath) || !Array.isArray(contract.reads) || contract.reads.length !== 2) return reject("invalid binding contract");
  if (contract.reads.some((entry) => !entry || !keys(entry, ["binding", "relativePath"]) || !identifier(entry.binding) || !exactOwnerPath(entry.relativePath)) || new Set([contract.rootBinding, contract.directoryBinding, ...contract.reads.map((entry) => entry.binding)]).size !== 4) return reject("ambiguous read binding");
  const consumers = owner.mappings.filter((mapping) => mapping.sourcePath === owner.sourceRoot + "/" + contract.consumerRelativePath);
  if (consumers.length !== 1) return reject("consumer must have one exact mapped owner");
  const consumer = consumers[0]!, sourceDirectory = owner.sourceRoot + "/" + contract.sourceDirectoryRelativePath;
  const reads = contract.reads.map((entry) => owner.mappings.filter((mapping) => mapping.sourcePath === sourceDirectory + "/" + entry.relativePath));
  if (reads.some((mappings) => mappings.length !== 1 || mappings[0]!.sourceRole !== "implementation" || !mappings[0]!.destinationPath.startsWith(owner.semanticOwnerRoot + "/")) || new Set(reads.map((mappings) => mappings[0]!.sourcePath)).size !== 2) return reject("read target must have one exact implementation owner");
  return { consumer, sourceDirectory, reads };
}

/** 🔐️ Proves three reviewed literal operands using a complete catalog source preimage. */
export function semanticPackageJoinedPathReferenceAuthority(
  facts: Readonly<{ path: string; content: string; layout: "source" | "destination" }>,
  contract: SemanticPackageJoinedPathBinding,
  owner: SemanticPackageProjectionCase,
): Readonly<{ references: readonly SemanticPackageJoinedPathReference[]; problems: readonly string[] }> {
  const reject = (message: string) => ({ references: [], problems: ["Nested Cargo joined-path authority: " + message] });
  let records: ReturnType<typeof semanticPackageJoinedPathBindingRecords>;
  try { records = semanticPackageJoinedPathBindingRecords(contract, owner); } catch (error) { return { references: [], problems: [String(error)] }; }
  const { consumer, sourceDirectory, reads } = records;
  if (facts.layout !== "source" && facts.layout !== "destination" || facts.path !== (facts.layout === "source" ? consumer.sourcePath : consumer.destinationPath)) return reject("consumer path or phase mismatch");
  if (facts.layout === "source" && (createHash("sha256").update(facts.content).digest("hex") !== consumer.sourceHash || Buffer.byteLength(facts.content) !== consumer.sourceSize)) return reject("complete source preimage changed");
  const imports = ['import { readFileSync } from "node:fs";', 'import { dirname, join } from "node:path";', 'import { fileURLToPath } from "node:url";'];
  if (!facts.content.startsWith(imports.join("\n") + "\n") || imports.some((statement) => facts.content.split(statement).length !== 2)) return reject("binding imports changed");
  const oldValues = [posix.relative(posix.dirname(consumer.sourcePath), sourceDirectory), ...contract.reads.map((entry) => entry.relativePath)];
  const newValues = [posix.relative(posix.dirname(consumer.destinationPath), owner.semanticOwnerRoot) || ".", ...reads.map((mappings) => posix.relative(owner.semanticOwnerRoot, mappings[0]!.destinationPath))];
  const values = facts.layout === "source" ? oldValues : newValues;
  const statements = [
    `const ${contract.rootBinding} = dirname(fileURLToPath(import.meta.url));`,
    `const ${contract.directoryBinding} = join(${contract.rootBinding}, ${JSON.stringify(values[0])});`,
    ...contract.reads.map((entry, index) => `const ${entry.binding} = readFileSync(join(${contract.directoryBinding}, ${JSON.stringify(values[index + 1])}), "utf8");`),
  ];
  const fragment = statements.join("\n    "), fragmentStart = facts.content.indexOf(fragment);
  if (fragmentStart < 0 || facts.content.indexOf(fragment, fragmentStart + fragment.length) >= 0 || statements.some((statement) => facts.content.split(statement).length !== 2)) return reject("reviewed consecutive binding statements changed");
  if (facts.layout === "destination") return { references: [], problems: [] };
  const references = statements.slice(1).map((statement, index) => {
    const start = fragmentStart + fragment.indexOf(statement) + statement.indexOf(JSON.stringify(oldValues[index])) + 1;
    return { start, end: start + oldValues[index]!.length, oldValue: oldValues[index]!, newValue: newValues[index]!, targetSourcePath: reads[Math.max(0, index - 1)]![0]!.sourcePath };
  });
  return { references, problems: [] };
}
//#endregion 🧵️Source-Owned Joined Paths

/** 📚️ Digest-locked nested package catalog and exact live reference owners. */
export interface SemanticPackageProjectionCatalog {
  readonly schemaVersion: 1;
  readonly contractKind: "exact-nested-cargo-package-catalog";
  readonly packages: readonly SemanticPackageProjectionCase[];
  readonly referenceConsumers: readonly Readonly<{ packageId: string; path: string; destinationPath?: string; transformId: string; occurrenceCount: number; ownership: "authored" | "generated"; generatorOwnerPaths?: readonly string[] }>[];
  readonly referenceTokenTransforms: Readonly<Record<string, Readonly<{ sourceToken: string; destinationToken: string }>>>;
}

/** 🔐️ Loads the exact package catalog without following any parent or leaf symlink. */
export function semanticPackageProjectionCatalog(repoRoot: string, taxonomy: Taxonomy): SemanticPackageProjectionCatalog | null {
  const contract = taxonomy.semanticPackageProjectionContracts["nested-cargo-packages-v1"];
  const state = exactOwnerRegularFile(repoRoot, contract.authorityCatalogPath);
  if (state === "absent") return null;
  if (state !== "file") throw new Error("Nested Cargo catalog must be a no-follow regular file");
  const bytes = readFileSync(join(repoRoot, contract.authorityCatalogPath));
  if (createHash("sha256").update(bytes).digest("hex") !== contract.authorityCatalogSha256) throw new Error("Nested Cargo catalog digest drift");
  const catalog = JSON.parse(bytes.toString("utf8")) as SemanticPackageProjectionCatalog;
  if (catalog.schemaVersion !== 1 || catalog.contractKind !== contract.contractKind || JSON.stringify(catalog.packages.map((row) => row.id)) !== JSON.stringify(contract.packageIds)) throw new Error("Nested Cargo catalog identity drift");
  const sources = new Set<string>(), destinations = new Set<string>();
  for (const [index, row] of catalog.packages.entries()) {
    if (!exactOwnerPath(row.sourceRoot) || !exactOwnerPath(row.destinationRoot) || !exactOwnerPath(row.semanticOwnerRoot) || row.destinationRoot !== `${row.semanticOwnerRoot}/📦️packages/🦀️rust` || row.mappings.length !== contract.sourceLeafCounts[index]) throw new Error("Nested Cargo catalog boundary drift");
    for (const mapping of row.mappings) {
      const key = mapping.destinationPath.normalize("NFC").toLocaleLowerCase("und").replaceAll("\ufe0f", "");
      if (!exactOwnerPath(mapping.sourcePath) || !exactOwnerPath(mapping.destinationPath) || !mapping.sourcePath.startsWith(row.sourceRoot + "/") || !mapping.destinationPath.startsWith(row.semanticOwnerRoot + "/") || !/^[0-9a-f]{64}$/u.test(mapping.sourceHash) || !Number.isSafeInteger(mapping.sourceSize) || mapping.sourceSize < 0 || sources.has(mapping.sourcePath) || destinations.has(key) || Buffer.byteLength(mapping.destinationPath) > taxonomy.collisionPolicy.maxPathBytes) throw new Error("Nested Cargo catalog leaf authority drift");
      sources.add(mapping.sourcePath);
      destinations.add(key);
    }
    if (!Array.isArray(row.joinedPathBindings) || row.joinedPathBindings.length !== contract.joinedPathBindingCounts[index] || new Set(row.joinedPathBindings.map((binding) => binding.id)).size !== row.joinedPathBindings.length) throw new Error("Nested Cargo joined-path binding census drift");
    for (const binding of row.joinedPathBindings) semanticPackageJoinedPathBindingRecords(binding, row);
    if (row.authoredSourceFragments?.length !== contract.authoredFragmentCounts[index] || semanticPackageAuthoredFragmentProblem(row)) throw new Error("Nested Cargo authored fragment census or ownership drift");
    if (!Array.isArray(row.generatedSourceRetirements) || row.generatedSourceRetirements.length !== contract.generatedSourceRetirementCounts[index] || new Set(row.generatedSourceRetirements.map((retirement) => retirement.sourcePath)).size !== row.generatedSourceRetirements.length || row.mappings.filter((mapping) => mapping.disposition === "generated").length !== row.generatedSourceRetirements.length) throw new Error("Nested Cargo generated source retirement census drift");
    for (const retirement of row.generatedSourceRetirements) {
      const mapping = row.mappings.find((mapping) => mapping.sourcePath === retirement.sourcePath);
      if (Object.keys(retirement).sort().join("\0") !== "destinationPath\0generatorContractId\0sourceMode\0sourcePath" || row.id !== "wgpu-renderer" || !mapping || mapping.disposition !== "generated" || mapping.destinationPath !== retirement.destinationPath || retirement.generatorContractId !== "wgpu-frame-worker" || retirement.sourceMode !== 0o644 || taxonomy.generatorContracts[retirement.generatorContractId]?.ownership !== "owned") throw new Error("Nested Cargo generated source retirement authority drift");
    }
    if (!Array.isArray(row.derivedLeaves) || row.derivedLeaves.length !== (row.id === "wgpu-renderer" ? contract.derivedLeafCount : 0)) throw new Error("Nested Cargo derived leaf census drift");
    for (const leaf of row.derivedLeaves) {
      const mapping = row.mappings.find((mapping) => mapping.sourcePath === leaf.originSourcePath);
      const key = leaf.path.normalize("NFC").toLocaleLowerCase("und").replaceAll("\ufe0f", "");
      if (leaf.id !== "wgpu-renderer-registration" || leaf.path !== row.semanticOwnerRoot + "/🧊️renderer/📇️registry/🦀️.rs" || !exactOwnerPath(leaf.path) || !mapping || leaf.originSourceHash !== mapping.sourceHash || Buffer.byteLength(leaf.path) > taxonomy.collisionPolicy.maxPathBytes || destinations.has(key) || !taxonomy.semanticDirectoryKinds.registry || classifyPackageSourceRole(leaf.content, taxonomy.packageGlueGrammar.rust) !== leaf.expectedRole) throw new Error("Nested Cargo derived registration authority drift");
      destinations.add(key);
    }
    for (const adapter of row.adapters) {
      if (!exactOwnerPath(adapter.path) || !adapter.path.startsWith(row.destinationRoot + "/") || Buffer.byteLength(adapter.path) > taxonomy.collisionPolicy.maxPathBytes || adapter.targetPaths.some((path) => !row.mappings.some((mapping) => mapping.destinationPath === path) && !row.derivedLeaves.some((leaf) => leaf.path === path)) || classifyPackageSourceRole(adapter.content, taxonomy.packageGlueGrammar[adapter.language]) !== adapter.expectedRole) throw new Error("Nested Cargo adapter authority drift");
    }
    if (!Array.isArray(row.sourceSplices) || row.sourceSplices.length !== (row.id === "wgpu-renderer" ? 2 : 0)) throw new Error("Nested Cargo source splice census drift");
    for (const splice of row.sourceSplices) if (!row.mappings.some((mapping) => mapping.sourcePath === splice.sourcePath && mapping.destinationPath === splice.destinationPath) || !splice.oldValue || !splice.newValue) throw new Error("Nested Cargo source splice ownership drift");
    if (row.id === "wgpu-renderer") {
      const [library, renderer] = row.sourceSplices, mapping = row.mappings.find((mapping) => mapping.sourcePath === library!.sourcePath)!;
      const registration = library!.oldValue.match(/#\[cfg\(target_arch = "wasm32"\)\][\s\S]*?(?=#\[cfg\(not\(target_os = "wasi"\)\)\]\ninclude!\("🦀️.rs"\);)/u)?.[0];
      const split = registration?.indexOf('#[cfg(not(target_os = "wasi"))]') ?? -1;
      if (library!.id !== "library-adapter-replacement" || renderer!.id !== "renderer-registration-merge" || createHash("sha256").update(library!.oldValue).digest("hex") !== mapping.sourceHash || Buffer.byteLength(library!.oldValue) !== mapping.sourceSize || library!.newValue !== row.adapters.find((adapter) => adapter.id === "wgpu-library")?.content || !registration || split < 0 || renderer!.destinationPath !== row.semanticOwnerRoot + "/🧊️renderer/🦀️.rs" || renderer!.newValue !== registration.slice(0, split) + renderer!.oldValue || row.derivedLeaves[0]!.content !== "//! 📇️ Renderer macro registration.\n\n" + registration.slice(split)) throw new Error("Nested Cargo renderer registration merge authority drift");
    }
  }
  if (catalog.packages.flatMap((row) => row.mappings).filter((row) => row.sourceRole !== null).length !== contract.purityCount || catalog.packages.flatMap((row) => row.adapters).length !== contract.adapterCount) throw new Error("Nested Cargo purity census drift");
  const generator = taxonomy.generatorContracts["jco-package-adapter"], current = parseCurrentJcoPackageDestination(generator?.currentPackageDestination);
  if (!generator || generator.projectionActivation || [current.cargoManifestPath, current.cargoLockPath, current.componentPath, current.witPath].some((path) => !generator.inputPatterns.includes(path)) || JSON.stringify(generator.outputRoots) !== JSON.stringify([{ path: current.adapterPath, inclusion: "tracked" }])) throw new Error("Current JCO generator destination authority drift");
  return catalog;
}

function nestedCargoField(content: string, section: string, key: string): unknown {
  const escaped = (value: string): string => value.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
  content = content.replace(/"(?:\\.|[^"\\])*"|#[^\r\n]*/gu, (token) => token.startsWith("#") ? "" : token);
  if ([...content.matchAll(new RegExp(`^\\s*\\[${escaped(section)}\\]\\s*(?:#.*)?$`, "gmu"))].length !== 1) return undefined;
  const body = tomlTableBody(content, section);
  if (body === undefined) return undefined;
  const values = [...body.matchAll(new RegExp(`^\\s*${escaped(key)}\\s*=\\s*("[^"\\r\\n]*"|false|true|\\[[\\s\\S]*?\\])\\s*(?:#.*)?$`, "gmu"))];
  if (values.length !== 1) return undefined;
  try { return JSON.parse(values[0]![1]!.replace(/,\s*\]/gu, "]")); } catch { return undefined; }
}

/** 🪺️ Derives optional ignored leaves only from exact package entry and generator output authority. */
export function semanticPackageIgnoredGeneratedOutputPaths(row: SemanticPackageProjectionCase, taxonomy: Taxonomy): readonly string[] {
  const generator = taxonomy.generatorContracts["wgpu-frame-worker"];
  if (row.id !== "wgpu-renderer" || generator?.packageGeneration?.kind !== "wgpu-package-artifacts") return [];
  const profile = parseSemanticPackageBrowserProfile(generator.packageGeneration.browserProfile, taxonomy.pathEmojiPolicy.genericEmojiIdentities);
  if (profile.ownerPath !== row.semanticOwnerRoot) return [];
  return profile.entries.filter((entry) => entry.inclusion === "ignored").flatMap((entry) => {
    const path = profile.ownerPath + "/" + entry.outputRelativePath;
    return generator.outputRoots.some((root) => root.path === path && root.inclusion === "ignored") ? [path] : [];
  });
}

/** 🧭️ Validates complete source or canonical package facts; no physical reads occur here. */
export function semanticPackageProjectionAuthority(
  facts: Readonly<{ packageId: string; nodes: readonly SemanticProjectionAuthorityNode[]; layout?: "source" | "destination"; occupiedPaths?: readonly string[]; cargoWorkspaceContent?: string; nodeWorkspaceContent?: string }>,
  catalog: SemanticPackageProjectionCatalog,
  taxonomy: Taxonomy,
): Readonly<{ packageId: string; mappings: readonly SemanticPackageProjectionMapping[]; adapters: readonly SemanticPackageAdapter[]; sourceDigest: string; problems: readonly string[] }> {
  const row = catalog.packages.find((entry) => entry.id === facts.packageId), problems: string[] = [];
  const finish = () => ({ packageId: facts.packageId, mappings: problems.length ? [] : row!.mappings, adapters: problems.length ? [] : row!.adapters, sourceDigest: createHash("sha256").update(JSON.stringify([...facts.nodes].sort((a, b) => projectionByteCompare(a.path, b.path)))).digest("hex"), problems });
  if (!row) { problems.push("Unregistered nested Cargo package identity"); return finish(); }
  const memberId = row.id === "wgpu-renderer" ? "members-of-wgpu-target" : "members-of-jco-guest";
  const ownerKind = row.id === "wgpu-renderer" ? "wgpu-target" : "jco-guest";
  const members = taxonomy.semanticDirectoryMemberKinds[memberId];
  const requiredMembers = row.mappings.filter((mapping) => !mapping.destinationPath.startsWith(row.destinationRoot + "/")).map((mapping) => mapping.destinationPath.slice(row.semanticOwnerRoot.length + 1).split("/")[0]!);
  if (!taxonomy.semanticDirectoryKinds[ownerKind] || members?.source !== "registry" || !members.ownerKindIds.includes(ownerKind) || requiredMembers.some((member) => !members.memberNames.includes(member))) problems.push("Nested Cargo semantic owner/member registration is missing");
  const boundary = taxonomy.packageBoundaryRules["🦀️rust"];
  const requiredEntries = row.id === "wgpu-renderer" ? ["rust-library-entry", "rust-binary-entry", "rust-build-entry", "vitest-config-entry"] : ["rust-library-entry"];
  if (!boundary || requiredEntries.some((id) => !taxonomy.configurableEntryContracts[id] || !taxonomy.packageSourceDispositions[id]) || (row.id === "wgpu-renderer" ? ["library", "binary", "builder", "tests", "typescript-language"] : ["library"]).some((id) => !boundary?.allowedDirectoryKindIds.includes(id))) problems.push("Nested Cargo package boundary prerequisite is missing");
  const cargoEntries = requiredEntries.filter((id) => id.startsWith("rust-"));
  if (cargoEntries.some((id) => !boundary?.entryContractIds.includes(id) || !taxonomy.ecosystems["🦀️rust"]?.entryContractIds?.includes(id) || row.id === "wgpu-renderer" && !taxonomy.targets["🧊️wgpu"]?.entryContractIds?.includes(id))) problems.push("Nested Cargo configured entry is absent from a package, ecosystem or target registry");
  if (problems.length > 0) return finish();
  const destination = facts.layout === "destination";
  const expected = new Map(row.mappings.map((mapping) => [destination ? mapping.destinationPath : mapping.sourcePath, mapping]));
  const adapters = new Map(row.adapters.map((adapter) => [adapter.path, adapter]));
  const derived = new Map(row.derivedLeaves.map((leaf) => [leaf.path, leaf]));
  const expectedPaths = new Set([...expected.keys(), ...(destination ? [...adapters.keys(), ...derived.keys()] : [])]);
  const ignoredOutputs = new Set<string>();
  if (destination) try { for (const path of semanticPackageIgnoredGeneratedOutputPaths(row, taxonomy)) ignoredOutputs.add(path); }
  catch (error) { problems.push(error instanceof Error ? error.message : String(error)); }
  const admittedPaths = new Set([...expectedPaths, ...ignoredOutputs]);
  const allowedDirectories = new Set<string>();
  for (const path of admittedPaths) for (let parent = dirname(path); parent !== "."; parent = dirname(parent)) allowedDirectories.add(parent);
  const nodes = new Map<string, SemanticProjectionAuthorityNode>();
  for (const node of facts.nodes) {
    if (!exactOwnerPath(node.path) || nodes.has(node.path) || !["file", "directory"].includes(node.nodeKind) || node.nodeKind === "directory" && !allowedDirectories.has(node.path) || node.nodeKind === "file" && (!admittedPaths.has(node.path) || ignoredOutputs.has(node.path) && typeof node.content !== "string")) { problems.push("Unadmitted or duplicate nested Cargo node: " + node.path); continue; }
    nodes.set(node.path, node);
  }
  for (const path of expectedPaths) {
    const node = nodes.get(path), mapping = expected.get(path), adapter = destination ? adapters.get(path) : undefined;
    if (node?.nodeKind !== "file" || typeof node.content !== "string") { problems.push("Missing nested Cargo regular leaf: " + path); continue; }
    if (!destination && mapping && (createHash("sha256").update(node.content).digest("hex") !== mapping.sourceHash || Buffer.byteLength(node.content) !== mapping.sourceSize)) problems.push("Nested Cargo source preimage drift: " + path);
    if (adapter && node.content !== adapter.content) problems.push("Nested Cargo canonical adapter bytes drift: " + path);
    if (destination && derived.has(path) && node.content !== derived.get(path)!.content) problems.push("Nested Cargo canonical registration bytes drift: " + path);
  }
  for (const splice of row.sourceSplices) {
    const content = nodes.get(destination ? splice.destinationPath : splice.sourcePath)?.content ?? "", value = destination ? splice.newValue : splice.oldValue;
    if (!content.startsWith(value) || content.split(value).length - 1 !== 1) problems.push("Nested Cargo registration or adapter splice drift: " + splice.id);
  }
  for (const sourcePath of new Set(row.authoredSourceFragments.map((fragment) => fragment.sourcePath))) {
    const mapping = row.mappings.find((mapping) => mapping.sourcePath === sourcePath)!, path = destination ? mapping.destinationPath : sourcePath;
    problems.push(...semanticPackageAuthoredFragmentReferences({ path, content: nodes.get(path)?.content ?? "", layout: destination ? "destination" : "source" }, row).problems);
  }
  if (problems.length === 0) for (const binding of row.joinedPathBindings) {
    const mapping = row.mappings.find((entry) => entry.sourcePath === row.sourceRoot + "/" + binding.consumerRelativePath)!;
    const path = destination ? mapping.destinationPath : mapping.sourcePath;
    problems.push(...semanticPackageJoinedPathReferenceAuthority({ path, content: nodes.get(path)!.content!, layout: destination ? "destination" : "source" }, binding, row).problems);
  }
  if (!destination) {
    const fold = (path: string): string => path.normalize("NFC").toLocaleLowerCase("und").replaceAll("\ufe0f", "");
    const outputPaths = new Set([...row.mappings.map((mapping) => mapping.destinationPath), ...adapters.keys(), ...derived.keys()]);
    const foldedOutputPaths = [...outputPaths].map(fold);
    for (const path of facts.occupiedPaths ?? []) {
      const foldedPath = fold(path);
      if (foldedOutputPaths.some((output) => foldedPath === output || output.startsWith(foldedPath + "/")) && !allowedDirectories.has(path)) problems.push("Nested Cargo destination collision: " + path);
    }
  }
  const activeRoot = destination ? row.destinationRoot : row.sourceRoot;
  const manifest = nodes.get(activeRoot + "/Cargo.toml")?.content ?? "";
  if (nestedCargoField(manifest, "package", "name") !== row.identity.cargoPackageName) problems.push("Nested Cargo package name is not the exact registered identity");
  if (nestedCargoField(manifest, "lib", "path") !== (destination ? "📚️library/🦀️.rs" : row.id === "wgpu-renderer" ? "🦀️lib.rs" : "🦀️.rs")) problems.push("Nested Cargo library entry authority drift");
  if (JSON.stringify(nestedCargoField(manifest, "lib", "crate-type")) !== JSON.stringify(row.id === "wgpu-renderer" ? ["cdylib", "rlib"] : ["cdylib"])) problems.push("Nested Cargo crate-type authority drift");
  if (row.workspaceKind === "standalone") {
    const workspace = tomlTableBody(manifest, "workspace");
    if (workspace === undefined || workspace.replace(/#[^\r\n]*/gu, "").trim() !== "" || [...manifest.matchAll(/^\s*\[workspace\]\s*$/gmu)].length !== 1 || nestedCargoField(manifest, "package", "publish") !== false) problems.push("JCO requires its exact empty standalone workspace and non-publishing package");
    const lock = nodes.get(activeRoot + "/Cargo.lock")?.content ?? "";
    if (!lock.startsWith("# This file is automatically @generated by Cargo.\n") || !/^version = 4$/mu.test(lock) || !lock.includes('name = "semio-jcoprobe-guest"')) problems.push("JCO Cargo lock authority drift");
    const wit = nodes.get(activeRoot + (destination ? "/🧬️schema/📜️world.wit" : "/🧬️schema/🧪️world/📜️.wit"))?.content ?? "";
    if (!wit.includes("package semio:jcoprobe@0.1.0;") || !/\bworld\s+jcoprobe\s*\{/u.test(wit)) problems.push("JCO WIT world identity drift");
    const implementation = nodes.get(destination ? row.semanticOwnerRoot + "/🧩️component/🦀️.rs" : row.sourceRoot + "/🦀️.rs")?.content ?? "";
    if ([...implementation.matchAll(/^wit_bindgen::generate!\(\{\s*path:\s*"🧬️schema\/📜️world\.wit",\s*world:\s*"jcoprobe",\s*async:\s*true,?\s*\}\);/gmu)].length !== 1 || [...implementation.matchAll(/^wit_bindgen::generate!/gmu)].length !== 1) problems.push("JCO WIT binding must retain its exact Cargo-manifest-relative authority");
  } else {
    if (nestedCargoField(manifest, "package.metadata.semio", "role") !== "framework" || nestedCargoField(manifest, "package.metadata.semio", "id") !== "renderer-wgpu" || nestedCargoField(manifest, "package", "build") !== (destination ? "🏗️builder/🦀️.rs" : "build.rs")) problems.push("WGPU Cargo target/role authority drift");
    const binary = manifest.replace(/^[ \t]*\[\[bin\]\][ \t]*$/gmu, "[bin]");
    if (nestedCargoField(binary, "bin", "name") !== "semio-wgpu-native" || nestedCargoField(binary, "bin", "path") !== (destination ? "💾️binary/🦀️.rs" : "📦️bin.rs") || JSON.stringify(nestedCargoField(binary, "bin", "required-features")) !== JSON.stringify(["native-bin"])) problems.push("WGPU Cargo binary entry authority drift");
    if (!(nestedCargoField(facts.cargoWorkspaceContent ?? "", "workspace", "members") as unknown[] | undefined)?.includes(activeRoot)) problems.push("WGPU is absent from the exact root Cargo workspace");
    try {
      const workspace = JSON.parse(facts.nodeWorkspaceContent ?? "null"), node = JSON.parse(nodes.get(activeRoot + "/package.json")?.content ?? "null"), nx = JSON.parse(nodes.get(activeRoot + "/📋️project.json")?.content ?? "null");
      if (!Array.isArray(workspace?.workspaces) || !workspace.workspaces.includes(activeRoot) || node?.name !== row.identity.nodePackageName || node?.exports?.["."] !== (destination ? "./🟦️typescript/📚️library/🟦️.ts" : "./🟦️.ts") || nx?.name !== row.identity.nxProjectName || nx?.sourceRoot !== activeRoot || !nx?.targets || Object.values(nx.targets).some((target) => (target as { options?: { cwd?: string } }).options?.cwd !== activeRoot) || destination && (!nx?.namedInputs?.default?.includes(`{workspaceRoot}/${row.semanticOwnerRoot}/**/*`) || node?.repository?.directory !== row.destinationRoot)) problems.push("WGPU Node/Nx workspace identity drift");
    } catch { problems.push("WGPU requires valid Node and Nx manifest evidence"); }
    const configPath = destination ? "🟦️typescript/🧪️test/🟦️s.ts" : "🧪️tests/🟦️.ts";
    const config = nodes.get(activeRoot + "/" + configPath)?.content ?? "", script = nodes.get(activeRoot + "/📜️script.ts")?.content ?? "";
    if (classifyPackageSourceDisposition(config, taxonomy.packageSourceDispositions["vitest-config-entry"]!, taxonomy.packageGlueGrammar.typescript!) !== "tool-metadata" || script.split(JSON.stringify(configPath)).length !== 4) problems.push("WGPU exact Vitest configuration authority drift");
  }
  for (const mapping of row.mappings.filter((entry) => entry.sourceRole !== null && entry.disposition !== "adapter" && entry.disposition !== "tool-metadata")) {
    const path = destination ? mapping.destinationPath : mapping.sourcePath, content = nodes.get(path)?.content;
    const grammar = taxonomy.packageGlueGrammar[path.endsWith(".rs") ? "rust" : path.endsWith(".js") ? "javascript" : "typescript"];
    if (content !== undefined && classifyPackageSourceRole(content, grammar) !== mapping.sourceRole) problems.push("Nested Cargo implementation role drift: " + path);
  }
  return finish();
}
function semanticPackageGenerationAuthority(repoRoot: string, packageId: SemanticPackageProjectionCase["id"], taxonomy: Taxonomy) {
  const catalog = semanticPackageProjectionCatalog(repoRoot, taxonomy);
  const row = catalog?.packages.find((entry) => entry.id === packageId);
  if (!catalog || !row) throw new Error("Package adapter catalog is unavailable");
  const source = exactOwnerRegularFile(repoRoot, row.sourceRoot + "/Cargo.toml"), destination = exactOwnerRegularFile(repoRoot, row.destinationRoot + "/Cargo.toml");
  if (source === "invalid" || destination === "invalid" || Number(source === "file") + Number(destination === "file") !== 1) throw new Error("Package adapter requires exactly one source or canonical package");
  const canonical = destination === "file";
  const paths = [...new Set(row.mappings.map((mapping) => canonical ? mapping.destinationPath : mapping.sourcePath))];
  const nodes: SemanticProjectionAuthorityNode[] = paths.map((path) => {
    if (exactOwnerRegularFile(repoRoot, path) !== "file") throw new Error("Package adapter source is not a no-follow regular file: " + path);
    return { path, nodeKind: "file", content: readFileSync(join(repoRoot, path), "utf8") };
  });
  const generated = [...row.adapters.filter((adapter) => !row.mappings.some((mapping) => mapping.destinationPath === adapter.path)), ...row.derivedLeaves];
  for (const leaf of [...row.adapters, ...row.derivedLeaves]) {
    const state = exactOwnerRegularFile(repoRoot, leaf.path);
    if (state === "invalid" || !canonical && state !== "absent") throw new Error("Package generated destination is occupied or invalid: " + leaf.path);
    if (canonical && !paths.includes(leaf.path)) nodes.push({ path: leaf.path, nodeKind: "file", content: state === "file" ? readFileSync(join(repoRoot, leaf.path), "utf8") : leaf.content });
  }
  const workspace = (path: string): string | undefined => {
    if (row.workspaceKind !== "repository") return undefined;
    if (exactOwnerRegularFile(repoRoot, path) !== "file") throw new Error("Package workspace evidence is not a no-follow regular file: " + path);
    return readFileSync(join(repoRoot, path), "utf8");
  };
  const authority = semanticPackageProjectionAuthority({ packageId, nodes, layout: canonical ? "destination" : "source", cargoWorkspaceContent: workspace("Cargo.toml"), nodeWorkspaceContent: workspace("package.json") }, catalog, taxonomy);
  if (authority.problems.length) throw new Error(authority.problems.join(" | "));
  return { authority, generated };
}
/** 🪪️ Rejects noncanonical, colliding or historical coordinates for the single current JCO package. */
export function parseCurrentJcoPackageDestination(input: unknown): CurrentJcoPackageDestination {
  const semanticOwnerRoot = "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest";
  const packageRoot = semanticOwnerRoot + "/📦️packages/🦀️rust";
  const expected: CurrentJcoPackageDestination = { kind: "jco-canonical-package-v1", packageId: "jcoprobe-guest", semanticOwnerRoot, packageRoot, cargoManifestPath: packageRoot + "/Cargo.toml", cargoLockPath: packageRoot + "/Cargo.lock", componentPath: semanticOwnerRoot + "/🧩️component/🦀️.rs", witPath: packageRoot + "/🧬️schema/📜️world.wit", adapterPath: packageRoot + "/📚️library/🦀️.rs" };
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Current JCO destination must be an object");
  const value = input as Record<string, unknown>;
  if (Object.keys(value).sort().join("\0") !== Object.keys(expected).sort().join("\0") || Object.entries(expected).some(([key, path]) => value[key] !== path)) throw new Error("Current JCO destination differs from its explicit handpicked authority");
  const paths = [expected.cargoManifestPath, expected.cargoLockPath, expected.componentPath, expected.witPath, expected.adapterPath];
  const folded = paths.map((path) => path.normalize("NFC").replaceAll("\ufe0f", "").toLocaleLowerCase("und"));
  if (paths.some((path) => !exactOwnerPath(path)) || new Set(folded).size !== paths.length || folded.some((path, index) => folded.some((other, otherIndex) => index !== otherIndex && other.startsWith(path + "/")))) throw new Error("Current JCO paths are invalid or collide");
  return expected;
}

/** 🧩️ Renders unchanged frozen adapter bytes only after validating the complete current no-follow package. */
export function semanticPackageAdapterPreview(repoRoot: string, packageId: "jcoprobe-guest", taxonomy: Taxonomy = loadTaxonomy()): readonly SemanticPackageAdapter[] {
  const current = parseCurrentJcoPackageDestination(taxonomy.generatorContracts["jco-package-adapter"]?.currentPackageDestination);
  const catalog = semanticPackageProjectionCatalog(repoRoot, taxonomy), frozen = catalog?.packages.find((row) => row.id === packageId);
  if (!catalog || !frozen || frozen.mappings.length !== 4 || frozen.adapters.length !== 1 || frozen.adapters[0]?.id !== "jco-library") throw new Error("JCO frozen adapter evidence is unavailable");
  const destinations = [current.cargoLockPath, current.cargoManifestPath, current.componentPath, current.witPath];
  const row: SemanticPackageProjectionCase = { ...frozen, semanticOwnerRoot: current.semanticOwnerRoot, destinationRoot: current.packageRoot, mappings: frozen.mappings.map((mapping, index) => ({ ...mapping, destinationPath: destinations[index]! })), adapters: [{ ...frozen.adapters[0], path: current.adapterPath, targetPaths: [current.componentPath] }] };
  const nodes: SemanticProjectionAuthorityNode[] = row.mappings.map((mapping) => {
    if (exactOwnerRegularFile(repoRoot, mapping.destinationPath) !== "file") throw new Error("Current JCO input is absent or not a no-follow regular file: " + mapping.destinationPath);
    return { path: mapping.destinationPath, nodeKind: "file", content: readFileSync(join(repoRoot, mapping.destinationPath), "utf8") };
  });
  const state = exactOwnerRegularFile(repoRoot, current.adapterPath);
  if (state === "invalid") throw new Error("Current JCO adapter destination is not a no-follow regular file");
  nodes.push({ path: current.adapterPath, nodeKind: "file", content: state === "file" ? readFileSync(join(repoRoot, current.adapterPath), "utf8") : row.adapters[0]!.content });
  const authority = semanticPackageProjectionAuthority({ packageId, nodes, layout: "destination" }, { ...catalog, packages: [row] }, taxonomy);
  if (authority.problems.length) throw new Error(authority.problems.join(" | "));
  return authority.adapters;
}
/** 🏗️ Renders only new generated leaves, excluding authored mappings and their source splices. */
export function semanticPackageGeneratedLeafPreview(repoRoot: string, packageId: SemanticPackageProjectionCase["id"], taxonomy: Taxonomy = loadTaxonomy()): readonly Readonly<{ path: string; content: string }>[] {
  return semanticPackageGenerationAuthority(repoRoot, packageId, taxonomy).generated.map(({ path, content }) => ({ path, content })).sort((left, right) => projectionByteCompare(left.path, right.path));
}
/** 🪪️ Checks current source-layout ownership without authorizing frozen source transformations. */
export function semanticPackageSourceManifestIdentity(facts: Readonly<{ cargoManifestContent: string; nodeManifestContent: string; projectManifestContent: string }>, row: SemanticPackageProjectionCase): boolean {
  if (row.id !== "wgpu-renderer" || row.workspaceKind !== "repository") return false;
  try {
    let cargo = facts.cargoManifestContent;
    for (let index = 0; index < cargo.length; index++) {
      if (cargo[index] === "#") { index = cargo.indexOf("\n", index); if (index < 0) break; }
      const quote = cargo[index];
      if (quote !== "\"" && quote !== "'") continue;
      const delimiter = quote.repeat(3);
      if (cargo.slice(index, index + 3) === delimiter) {
        let end = index + 3;
        for (; end < cargo.length && cargo.slice(end, end + 3) !== delimiter; end++) if (quote === "\"" && cargo[end] === "\\") end++;
        if (cargo.slice(end, end + 3) !== delimiter) return false;
        end += 3;
        for (let extra = 0; extra < 2 && cargo[end] === quote; extra++) end++;
        cargo = cargo.slice(0, index) + cargo.slice(index, end).replace(/[^\r\n]/g, " ") + cargo.slice(end);
        index = end - 1;
        continue;
      }
      for (index++; index < cargo.length && cargo[index] !== quote; index++) {
        if (cargo[index] === "\n" || cargo[index] === "\r") return false;
        if (quote === "\"" && cargo[index] === "\\") {
          if (index + 1 >= cargo.length || cargo[index + 1] === "\n" || cargo[index + 1] === "\r") return false;
          index++;
        }
      }
      if (index >= cargo.length) return false;
    }
    const binary = cargo.replace(/^[ \t]*\[\[bin\]\][ \t]*$/gmu, "[bin]");
    const fields: readonly (readonly [string, string, unknown])[] = [
      ["package", "name", row.identity.cargoPackageName], ["package", "build", "build.rs"],
      ["package.metadata.semio", "role", "framework"], ["package.metadata.semio", "id", "renderer-wgpu"],
      ["lib", "path", "🦀️lib.rs"], ["lib", "crate-type", ["cdylib", "rlib"]],
    ];
    if (fields.some(([section, key, expected]) => JSON.stringify(nestedCargoField(cargo, section, key)) !== JSON.stringify(expected))) return false;
    if (nestedCargoField(binary, "bin", "name") !== "semio-wgpu-native" || nestedCargoField(binary, "bin", "path") !== "📦️bin.rs" || JSON.stringify(nestedCargoField(binary, "bin", "required-features")) !== JSON.stringify(["native-bin"])) return false;
    const record = (value: unknown): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value);
    const node: unknown = JSON.parse(facts.nodeManifestContent), nx: unknown = JSON.parse(facts.projectManifestContent);
    return record(node) && node.name === row.identity.nodePackageName && record(node.exports) && node.exports["."] === "./🟦️.ts" && record(nx) && nx.name === row.identity.nxProjectName && nx.sourceRoot === row.sourceRoot && record(nx.targets) && Object.keys(nx.targets).length > 0 && Object.values(nx.targets).every((target) => record(target) && record(target.options) && target.options.cwd === row.sourceRoot);
  } catch { return false; }
}

/** 🌗️ Recognizes pending source-layout outputs without conferring source-content mutation authority. */
export function semanticPackageSourceOutputPhase(repoRoot: string, generatorId: string, taxonomy: Taxonomy): readonly string[] {
  if (generatorId !== "wgpu-frame-worker") return [];
  try {
    const contract = taxonomy.generatorContracts[generatorId], row = semanticPackageProjectionCatalog(repoRoot, taxonomy)?.packages.find((entry) => entry.id === "wgpu-renderer");
    if (!row || contract?.packageGeneration?.kind !== "wgpu-package-artifacts" || contract.projectionActivation?.packageId !== row.id) return [];
    const root = resolve(repoRoot), gitOptions = { cwd: root, encoding: "utf8" as const, timeout: 5_000, maxBuffer: 1_048_576, stdio: ["ignore", "pipe", "pipe"] as ["ignore", "pipe", "pipe"] };
    if (resolve(execFileSync("git", ["rev-parse", "--show-toplevel"], gitOptions).trim()) !== root) return [];
    const admitted = execFileSync("git", ["--literal-pathspecs", "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", row.sourceRoot], gitOptions).split("\0").filter(Boolean).sort(projectionByteCompare);
    const expected = row.mappings.map((mapping) => mapping.sourcePath).sort(projectionByteCompare);
    if (JSON.stringify(admitted) !== JSON.stringify(expected) || admitted.some((path) => exactOwnerRegularFile(root, path) !== "file")) return [];
    const view = registryCatalogInputView(root, taxonomy);
    const destinations = new Set([...row.mappings.map((mapping) => mapping.destinationPath), ...row.adapters.map((adapter) => adapter.path), ...row.derivedLeaves.map((leaf) => leaf.path), ...contract.outputRoots.map((output) => output.path)]);
    if ([...destinations].some((path) => view.kind(path) !== null)) return [];
    const manifest = (name: string): string => {
      const mapping = row.mappings.find((entry) => entry.sourcePath === row.sourceRoot + "/" + name);
      if (!mapping) throw new Error("Source package manifest mapping is absent");
      return readFileSync(join(root, mapping.sourcePath), "utf8");
    };
    if (!semanticPackageSourceManifestIdentity({ cargoManifestContent: manifest("Cargo.toml"), nodeManifestContent: manifest("package.json"), projectManifestContent: manifest("📋️project.json") }, row)) return [];
    if (exactOwnerRegularFile(root, "Cargo.toml") !== "file" || exactOwnerRegularFile(root, "package.json") !== "file") return [];
    const cargo = readFileSync(join(root, "Cargo.toml"), "utf8"), node = JSON.parse(readFileSync(join(root, "package.json"), "utf8"));
    if (!(nestedCargoField(cargo, "workspace", "members") as unknown[] | undefined)?.includes(row.sourceRoot) || !Array.isArray(node.workspaces) || !node.workspaces.includes(row.sourceRoot)) return [];
    return contract.outputRoots.map((output) => output.path);
  } catch { return []; }
}

function nestedCargoGeneratedPrestate(repoRoot: string, path: string, generatorId: string, taxonomy: Taxonomy): boolean {
  if (!["jco-package-adapter", "external-cargo-locks", "wgpu-frame-worker"].includes(generatorId)) return false;
  try {
    if (generatorId === "wgpu-frame-worker") return semanticPackageSourceOutputPhase(repoRoot, generatorId, taxonomy).includes(path);
    if (generatorId === "jco-package-adapter") return semanticPackageAdapterPreview(repoRoot, "jcoprobe-guest", taxonomy).some((adapter) => adapter.path === path);
    const row = semanticPackageProjectionCatalog(repoRoot, taxonomy)?.packages.find((entry) => entry.id === "jcoprobe-guest");
    const mapping = row?.mappings.find((entry) => entry.destinationPath === path && basename(entry.sourcePath) === "Cargo.lock");
    return Boolean(mapping && exactOwnerRegularFile(repoRoot, mapping.sourcePath) === "file" && createHash("sha256").update(readFileSync(join(repoRoot, mapping.sourcePath))).digest("hex") === mapping.sourceHash);
  } catch { return false; }
}
//#endregion 📦️Nested Cargo Package Authority

function exactOwnerPath(path: unknown): path is string {
  return typeof path === "string" && path.length > 0 && path === path.normalize("NFC") && !path.startsWith("/") && !path.includes("\\") && !/[\u0000-\u001f]/u.test(path) && path.split("/").every((part) => part !== "" && part !== "." && part !== "..") && !["compose", "temp/compose"].some((root) => path === root || path.startsWith(root + "/"));
}

/** 📖️ Resolves the externally meaningful README/LICENSE basename beneath repository emoji identity. */
function semanticOwnedSourceBasename(path: string): string {
  return leadingEmojiIdentity(basename(path)).rest;
}

//#region 🪪️Reviewed Current Owner Preimages
/** 📐️ Parses the closed one-row current-source revision grammar without reading any path. */
export function parseSemanticOwnedCurrentSourceRevisions(input: unknown): Readonly<Record<string, SemanticOwnedCurrentSourceRevision>> {
  const fail = (message: string): never => { throw new Error("current-source-revision-invalid: " + message); };
  const record = (value: unknown, keys: readonly string[], label: string): Record<string, unknown> => {
    if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).sort().join("\0") !== [...keys].sort().join("\0")) return fail(label + " fields");
    return value as Record<string, unknown>;
  };
  const id = "testing-readme-protocol-v2-reviewed", rows = record(input, [id], "registry");
  const row = record(rows[id], ["catalogCaseIndex", "sourcePath", "baselineCommit", "baselineBlob", "baselinePreimage", "currentPreimage", "expectationsPath", "expectationsSha256"], id);
  const tuple = (value: unknown, label: string): SemanticOwnedCurrentSourceRevision["currentPreimage"] => {
    const selected = record(value, ["sha256", "size", "mode"], label);
    if (typeof selected.sha256 !== "string" || !/^[0-9a-f]{64}$/u.test(selected.sha256) || selected.mode !== "0644" || !Number.isSafeInteger(selected.size) || (selected.size as number) < 0) return fail(label + " tuple");
    return { sha256: selected.sha256, size: selected.size as number, mode: selected.mode };
  };
  if (row.catalogCaseIndex !== 31 || typeof row.baselineCommit !== "string" || !/^[0-9a-f]{40}$/u.test(row.baselineCommit) || typeof row.baselineBlob !== "string" || !/^[0-9a-f]{40}$/u.test(row.baselineBlob)) return fail(id + " index or baseline identity");
  if (!exactOwnerPath(row.sourcePath) || /[:*?"<>|]/u.test(row.sourcePath) || semanticOwnedSourceBasename(row.sourcePath) !== "README.md" || Buffer.from(row.sourcePath).toString("utf8") !== row.sourcePath) return fail(id + " raw source coordinate");
  if (!exactOwnerPath(row.expectationsPath) || /[:*?"<>|]/u.test(row.expectationsPath) || !row.expectationsPath.endsWith(".json") || Buffer.from(row.expectationsPath).toString("utf8") !== row.expectationsPath || typeof row.expectationsSha256 !== "string" || !/^[0-9a-f]{64}$/u.test(row.expectationsSha256)) return fail(id + " expectation identity");
  const baselinePreimage = tuple(row.baselinePreimage, "baseline"), currentPreimage = tuple(row.currentPreimage, "current");
  if (baselinePreimage.sha256 === currentPreimage.sha256) return fail(id + " requires one distinct current preimage");
  return { [id]: { catalogCaseIndex: 31, sourcePath: row.sourcePath, baselineCommit: row.baselineCommit, baselineBlob: row.baselineBlob, baselinePreimage, currentPreimage, expectationsPath: row.expectationsPath, expectationsSha256: row.expectationsSha256 } };
}

/** 🧮️ Canonicalizes the closed revision envelope with sorted object keys and retained array order. */
function semanticOwnedCurrentRevisionCanonical(value: unknown): string {
  if (Array.isArray(value)) return "[" + value.map(semanticOwnedCurrentRevisionCanonical).join(",") + "]";
  if (value !== null && typeof value === "object") return "{" + Object.keys(value).sort().map((key) => JSON.stringify(key) + ":" + semanticOwnedCurrentRevisionCanonical((value as Record<string, unknown>)[key])).join(",") + "}";
  return JSON.stringify(value);
}

/** 🧭️ Selects a raw owner's current preimage from exact supplied evidence without filesystem or Git access. */
export function semanticExactOwnedFileCurrentPreimageAuthority(catalog: SemanticExactOwnedFileCatalog, contract: Pick<SemanticExactOwnedFileProjectionContract, "authorityCatalogPath" | "authorityCatalogSha256">, input: unknown, facts: Readonly<{ path: string; nodeKind: string; contentHash: string; mode: number; size: number; expectations: readonly SemanticOwnedCurrentSourceExpectation[] }>): SemanticOwnedCurrentSourcePreimageResult {
  const empty = { catalogCaseIndex: null, preimage: null, revisionId: null, revisionDigest: null, problems: [] } as const;
  const fail = (message: string): SemanticOwnedCurrentSourcePreimageResult => ({ ...empty, disposition: "problem", problems: ["current-source-revision-invalid: " + message] });
  let revisions: Readonly<Record<string, SemanticOwnedCurrentSourceRevision>>;
  try { revisions = parseSemanticOwnedCurrentSourceRevisions(input); } catch (error) { return { ...empty, disposition: "problem", problems: [error instanceof Error ? error.message : String(error)] }; }
  if (!exactOwnerPath(contract.authorityCatalogPath) || !/^[0-9a-f]{64}$/u.test(contract.authorityCatalogSha256) || !Array.isArray(catalog.cases) || catalog.cases.length !== 40) return fail("catalog identity or census");
  if (!exactOwnerPath(facts.path) || /[:*?"<>|]/u.test(facts.path)) return fail("source coordinate");
  const index = catalog.cases.findIndex((entry) => entry.sourcePath === facts.path);
  if (index < 0) return { ...empty, disposition: "none" };
  const owner = catalog.cases[index], selected = Object.entries(revisions).find(([, row]) => row.catalogCaseIndex === index);
  const preimage = selected ? selected[1].currentPreimage : owner.preimage;
  if (!preimage || !/^[0-9a-f]{64}$/u.test(preimage.sha256) || preimage.mode !== "0644" || !Number.isSafeInteger(preimage.size) || preimage.size < 0) return fail("selected preimage tuple");
  if (facts.nodeKind !== "file" || facts.contentHash !== preimage.sha256 || facts.mode !== Number.parseInt(preimage.mode, 8) || facts.size !== preimage.size) return fail("selected source preimage drifted");
  if (!selected) return { ...empty, disposition: "catalog", catalogCaseIndex: index, preimage: { sha256: preimage.sha256, size: preimage.size, mode: preimage.mode } };
  const [id, row] = selected, ownerEvidence = catalog.ownerEvidence[owner.ownerEvidenceId], referenceOwner = catalog.referenceOwners["markdown-relative-reference-adapter"];
  if (row.sourcePath !== owner.sourcePath) return fail("declared raw source does not match its catalog row");
  if (owner.ownerEvidenceId !== "nx-project-owner-documentation" || ownerEvidence?.kind !== "ordinary-owner-doc" || !Array.isArray(ownerEvidence.evidencePaths) || ownerEvidence.evidencePaths.some((path) => !exactOwnerPath(path)) || owner.disposition !== "owner-documentation-relocate" || owner.fixedContractId !== null || owner.generatorOwnerId !== null || owner.projectionContractId !== "exact-owner-readme-projection" || !exactOwnerPath(owner.sourcePath) || posix.basename(owner.sourcePath) !== "README.md" || !exactOwnerPath(owner.destinationPath) || owner.destinationPath !== posix.dirname(owner.sourcePath) + "/📃️readme/📝️.md" || JSON.stringify(owner.referenceOwnerIds) !== JSON.stringify(["markdown-relative-reference-adapter"]) || referenceOwner?.kind !== "reference-adapter" || referenceOwner.ownerPath !== "repo-lib normalization reference graph") return fail("revision is not the approved ordinary README owner");
  const sameTuple = (left: unknown, right: Readonly<{ sha256: string; size: number; mode: string }>): boolean => {
    if (!left || typeof left !== "object" || Array.isArray(left)) return false;
    const value = left as Record<string, unknown>;
    return value.sha256 === right.sha256 && value.size === right.size && value.mode === right.mode;
  };
  if (!sameTuple(owner.preimage, row.baselinePreimage)) return fail("catalog baseline tuple drifted");
  if (!Array.isArray(facts.expectations) || facts.expectations.length !== 1) return fail("missing or duplicate expectation identity");
  const evidence = facts.expectations[0];
  if (!exactOwnerPath(evidence.path) || evidence.path !== row.expectationsPath || evidence.nodeKind !== "file" || evidence.mode !== 0o644 || !Array.isArray(evidence.ancestorNodeKinds) || evidence.ancestorNodeKinds.length !== evidence.path.split("/").length - 1 || evidence.ancestorNodeKinds.some((kind: string) => kind !== "directory") || !(evidence.bytes instanceof Uint8Array)) return fail("expectation is not an exact no-follow regular input");
  const bytes = Buffer.from(evidence.bytes), text = bytes.toString("utf8");
  if (!Buffer.from(text).equals(bytes)) return fail("expectation has lossy UTF-8");
  if (createHash("sha256").update(bytes).digest("hex") !== row.expectationsSha256) return fail("expectation bytes drifted");
  let expectation: Record<string, unknown>;
  try { expectation = JSON.parse(text); } catch { return fail("expectation JSON is invalid"); }
  if (!expectation || typeof expectation !== "object" || Array.isArray(expectation)) return fail("expectation JSON is not an object");
  const lineage = expectation.frozenAuthority as Record<string, unknown> | undefined, documents = expectation.documents as Record<string, unknown> | undefined;
  if (expectation.schemaVersion !== 1 || expectation.contract !== "testing-readme-current-coordinates-v1" || !lineage || !documents || lineage.path !== contract.authorityCatalogPath || lineage.sha256 !== contract.authorityCatalogSha256 || lineage.row !== row.catalogCaseIndex || lineage.baselineCommit !== row.baselineCommit || lineage.baselineBlob !== row.baselineBlob || !sameTuple(lineage.preimage, row.baselinePreimage) || documents.readme !== owner.sourcePath) return fail("expectation baseline or source-owner binding drifted");
  const envelope = {
    kind: "exact-owner-current-source-revision-v1",
    catalogIdentity: { path: contract.authorityCatalogPath, sha256: contract.authorityCatalogSha256 },
    revisionId: id,
    revision: row,
    owner: {
      catalogCaseIndex: index,
      sourcePath: owner.sourcePath,
      destinationPath: owner.destinationPath,
      ownerEvidenceId: owner.ownerEvidenceId,
      ownerEvidence: { kind: ownerEvidence.kind, evidencePaths: ownerEvidence.evidencePaths },
      disposition: owner.disposition,
      fixedContractId: owner.fixedContractId,
      projectionContractId: owner.projectionContractId,
      generatorOwnerId: owner.generatorOwnerId,
      referenceOwners: owner.referenceOwnerIds.map((referenceId: string) => ({ id: referenceId, kind: catalog.referenceOwners[referenceId].kind, ownerPath: catalog.referenceOwners[referenceId].ownerPath })),
    },
  };
  const revisionDigest = createHash("sha256").update(semanticOwnedCurrentRevisionCanonical(envelope)).digest("hex");
  return { disposition: "revised", catalogCaseIndex: index, preimage: { ...row.currentPreimage }, revisionId: id, revisionDigest, problems: [] };
}
//#endregion 🪪️Reviewed Current Owner Preimages

//#region 🖋️Exact Authored Owner Documents
/** 📏️ Validates the closed language-neutral authored-document correction grammar. */
export function parseSemanticOwnedDocumentCorrections(input: unknown): Readonly<Record<string, SemanticOwnedDocumentCorrection>> {
  const record = (value: unknown, keys: readonly string[], label: string): Record<string, unknown> => {
    if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).sort().join("\0") !== [...keys].sort().join("\0")) throw new Error(`Invalid authored document ${label} fields`);
    return value as Record<string, unknown>;
  };
  const rows = record(input, ["repo-library-script-filename-v1"], "registry");
  for (const [id, value] of Object.entries(rows)) {
    const row = record(value, ["contractKind", "activation", "sourcePath", "destinationPath", "preimage", "postimage", "replacementFixedFilenameContractId", "splices", "rationaleRule"], id);
    if (row.contractKind !== "exact-owner-content-splices" || row.activation !== "owner-leaf-move" || row.replacementFixedFilenameContractId !== "root-script" || row.rationaleRule !== "owner-script-filename-documentation-v1" || !exactOwnerPath(row.sourcePath) || semanticOwnedSourceBasename(row.sourcePath) !== "README.md" || !exactOwnerPath(row.destinationPath) || row.destinationPath !== dirname(row.sourcePath) + "/📃️readme/📝️.md") throw new Error(`Invalid authored document ${id} owner or activation`);
    const preimage = record(row.preimage, ["sha256", "mode", "size"], id + " preimage"), postimage = record(row.postimage, ["sha256", "size"], id + " postimage");
    if (typeof preimage.sha256 !== "string" || !/^[0-9a-f]{64}$/u.test(preimage.sha256) || preimage.mode !== "0644" || !Number.isSafeInteger(preimage.size) || (preimage.size as number) < 0 || typeof postimage.sha256 !== "string" || !/^[0-9a-f]{64}$/u.test(postimage.sha256) || !Number.isSafeInteger(postimage.size) || (postimage.size as number) < 0) throw new Error(`Invalid authored document ${id} preimage or postimage`);
    if (!Array.isArray(row.splices) || row.splices.length !== 10) throw new Error(`Authored document ${id} requires exactly ten filename splices`);
    let previousEnd = 0, previousLine = 0, delta = 0;
    for (const value of row.splices) {
      const splice = record(value, ["line", "startByte", "endByte", "oldValue", "newValue", "linePreimage"], id + " splice");
      if (!Number.isSafeInteger(splice.line) || (splice.line as number) < 1 || (splice.line as number) < previousLine || !Number.isSafeInteger(splice.startByte) || (splice.startByte as number) < previousEnd || !Number.isSafeInteger(splice.endByte) || (splice.endByte as number) > (preimage.size as number) || splice.oldValue !== "script.ts" || splice.newValue !== "📜️script.ts" || (splice.endByte as number) - (splice.startByte as number) !== Buffer.byteLength(splice.oldValue) || typeof splice.linePreimage !== "string" || splice.linePreimage.includes("\n") || !splice.linePreimage.includes(splice.oldValue)) throw new Error(`Invalid, repeated, or overlapping authored document ${id} splice`);
      previousEnd = splice.endByte as number;
      previousLine = splice.line as number;
      delta += Buffer.byteLength(splice.newValue) - Buffer.byteLength(splice.oldValue);
    }
    if ((postimage.size as number) !== (preimage.size as number) + delta) throw new Error(`Authored document ${id} postimage size does not match its splices`);
  }
  return rows as unknown as Readonly<Record<string, SemanticOwnedDocumentCorrection>>;
}

/** 🧩️ Produces exact UTF-16 edit offsets from byte-bound authored authority without modifying the frozen source. */
export function semanticExactOwnedDocumentCorrectionAuthority(catalog: SemanticExactOwnedFileCatalog, contract: SemanticExactOwnedFileProjectionContract, facts: Readonly<{ path: string; finalPath: string; content: string; mode: number; moving: boolean }>): Readonly<{ disposition: "none" | "rewrite" | "problem"; splices: readonly Readonly<{ start: number; end: number; oldValue: string; newValue: string; correctionId: string }>[]; problems: readonly string[] }> {
  const failure = (message: string) => ({ disposition: "problem" as const, splices: [], problems: [message] });
  let corrections: Readonly<Record<string, SemanticOwnedDocumentCorrection>>;
  try { corrections = parseSemanticOwnedDocumentCorrections(contract.authoredDocumentCorrections); } catch (error) { return failure(error instanceof Error ? error.message : String(error)); }
  if (!facts.moving) return { disposition: "none", splices: [], problems: [] };
  const selected = Object.entries(corrections).find(([, row]) => row.sourcePath === facts.path);
  if (!selected) return { disposition: "none", splices: [], problems: [] };
  const [id, row] = selected;
  const owner = catalog.cases.find((entry) => entry.sourcePath === row.sourcePath);
  if (!owner || owner.disposition !== "owner-documentation-relocate" || owner.generatorOwnerId !== null || owner.destinationPath !== row.destinationPath || facts.finalPath !== row.destinationPath || owner.preimage.sha256 !== row.preimage.sha256 || owner.preimage.mode !== row.preimage.mode || owner.preimage.size !== row.preimage.size) return failure(`Authored document ${id} does not match its frozen owner move`);
  const bytes = Buffer.from(facts.content), hash = (value: string | Buffer): string => createHash("sha256").update(value).digest("hex");
  if (hash(bytes) !== row.preimage.sha256 || bytes.byteLength !== row.preimage.size || facts.mode !== Number.parseInt(row.preimage.mode, 8)) return failure(`Authored document ${id} source preimage drifted`);
  const lines = facts.content.split("\n");
  const splices: { start: number; end: number; oldValue: string; newValue: string; correctionId: string }[] = [];
  for (const splice of row.splices) {
    const prefix = bytes.subarray(0, splice.startByte), selected = bytes.subarray(splice.startByte, splice.endByte);
    const prefixText = prefix.toString("utf8"), oldValue = selected.toString("utf8");
    if (!Buffer.from(prefixText).equals(prefix) || !Buffer.from(oldValue).equals(selected) || oldValue !== splice.oldValue || prefixText.split("\n").length !== splice.line || lines[splice.line - 1] !== splice.linePreimage) return failure(`Authored document ${id} exact span or line context drifted`);
    splices.push({ start: prefixText.length, end: prefixText.length + oldValue.length, oldValue, newValue: splice.newValue, correctionId: id });
  }
  const result = [...splices].reverse().reduce((text, splice) => text.slice(0, splice.start) + splice.newValue + text.slice(splice.end), facts.content);
  if (hash(result) !== row.postimage.sha256 || Buffer.byteLength(result) !== row.postimage.size) return failure(`Authored document ${id} resulting content drifted`);
  return { disposition: "rewrite", splices, problems: [] };
}
//#endregion 🖋️Exact Authored Owner Documents

function exactOwnerRegularFile(repoRoot: string, path: string): "file" | "absent" | "invalid" {
  if (!exactOwnerPath(path)) return "invalid";
  const segments = path.split("/");
  for (let index = 0; index < segments.length; index++) {
    let stat;
    try { stat = lstatSync(join(repoRoot, ...segments.slice(0, index + 1))); } catch (error) { return (error as NodeJS.ErrnoException).code === "ENOENT" ? "absent" : "invalid"; }
    if (stat.isSymbolicLink() || (index < segments.length - 1 ? !stat.isDirectory() : !stat.isFile())) return "invalid";
  }
  return "file";
}

/** 🛡️ Captures a safe regular input once and rejects changed descriptors or ancestry. */
export function semanticOwnedInputFileSnapshot(repoRoot: string, path: string): SemanticOwnedInputFileSnapshot | null {
  if (!exactOwnerPath(path) || /[:*?"<>|]/u.test(path) || Buffer.from(path).toString("utf8") !== path) throw new Error("Exact owner input has an unsafe coordinate");
  const parts = path.split("/"), witnesses: { path: string; dev: number; ino: number }[] = [];
  let current = repoRoot;
  const root = lstatSync(current);
  if (!root.isDirectory() || root.isSymbolicLink()) throw new Error("Exact owner input repository root is not a no-follow directory");
  witnesses.push({ path: current, dev: root.dev, ino: root.ino });
  for (const [index, part] of parts.entries()) {
    current = join(current, part);
    let node;
    try { node = lstatSync(current); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
    if (node.isSymbolicLink() || (index === parts.length - 1 ? !node.isFile() : !node.isDirectory())) throw new Error("Exact owner input must be a regular file beneath no-follow directories: " + path);
    if (index < parts.length - 1) witnesses.push({ path: current, dev: node.dev, ino: node.ino });
  }
  const before = lstatSync(current), descriptor = openSync(current, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try {
    const node = fstatSync(descriptor);
    if (!node.isFile() || node.dev !== before.dev || node.ino !== before.ino || node.mode !== before.mode || node.size !== before.size || node.mtimeMs !== before.mtimeMs || node.ctimeMs !== before.ctimeMs) throw new Error("Exact owner input changed during open: " + path);
    const bytes = readFileSync(descriptor), after = fstatSync(descriptor), named = lstatSync(current);
    if (bytes.byteLength !== node.size || after.dev !== node.dev || after.ino !== node.ino || after.mode !== node.mode || after.size !== node.size || after.mtimeMs !== node.mtimeMs || after.ctimeMs !== node.ctimeMs || named.isSymbolicLink() || !named.isFile() || named.dev !== node.dev || named.ino !== node.ino || named.mode !== node.mode || named.size !== node.size || named.mtimeMs !== node.mtimeMs || named.ctimeMs !== node.ctimeMs) throw new Error("Exact owner input changed during read: " + path);
    for (const witness of witnesses) {
      const ancestor = lstatSync(witness.path);
      if (!ancestor.isDirectory() || ancestor.isSymbolicLink() || ancestor.dev !== witness.dev || ancestor.ino !== witness.ino) throw new Error("Exact owner input ancestry changed: " + path);
    }
    return { path, nodeKind: "file", contentHash: createHash("sha256").update(bytes).digest("hex"), mode: node.mode & 0o7777, size: bytes.byteLength, ancestorNodeKinds: parts.slice(1).map(() => "directory"), bytes };
  } finally { closeSync(descriptor); }
}

/** 🔐️ Loads only the schema-registered catalog with exact bytes, paths, counts, and owner classifications. */
export function semanticExactOwnedFileCatalog(repoRoot: string, taxonomy: Taxonomy, observe?: (snapshot: SemanticOwnedInputFileSnapshot) => void): SemanticExactOwnedFileCatalog | null {
  const contract = taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  if (contract?.contractKind !== "exact-owner-path-catalog") return null;
  const path = contract.authorityCatalogPath;
  const state = exactOwnerRegularFile(repoRoot, path);
  if (state === "absent") return null;
  if (state !== "file") throw new Error("Exact owner catalog must be a regular file beneath non-symlink parents");
  const snapshot = semanticOwnedInputFileSnapshot(repoRoot, path);
  if (!snapshot) throw new Error("Exact owner catalog disappeared during capture: " + path);
  if (snapshot.mode !== 0o644) throw new Error("Exact owner catalog mode drift: " + path);
  const bytes = Buffer.from(snapshot.bytes);
  if (createHash("sha256").update(bytes).digest("hex") !== contract.authorityCatalogSha256) throw new Error("Exact owner catalog digest drift: " + path);
  if (!Buffer.from(bytes.toString("utf8")).equals(bytes)) throw new Error("Exact owner catalog has lossy UTF-8: " + path);
  const value = JSON.parse(bytes.toString("utf8")) as Record<string, unknown>;
  const object = (input: unknown): Record<string, unknown> => {
    if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Exact owner catalog requires object records");
    return input as Record<string, unknown>;
  };
  const ownerEvidence = object(value.ownerEvidence) as unknown as SemanticExactOwnedFileCatalog["ownerEvidence"];
  const referenceOwners = object(value.referenceOwners) as unknown as SemanticExactOwnedFileCatalog["referenceOwners"];
  const generatorOwners = object(value.generatorOwners) as unknown as SemanticExactOwnedFileCatalog["generatorOwners"];
  if (value.schemaVersion !== 1 || value.cohortId !== "nested-readme-license-owner-authority" || !Array.isArray(value.cases)) throw new Error("Exact owner catalog schema is invalid");
  for (const evidence of Object.values(ownerEvidence)) {
    if (!(contract.ownerEvidenceKinds as readonly string[]).includes(evidence.kind) || !Array.isArray(evidence.evidencePaths) || evidence.evidencePaths.some((path) => !exactOwnerPath(path))) throw new Error("Exact owner catalog has invalid owner evidence");
  }
  if (JSON.stringify(Object.keys(referenceOwners).sort()) !== JSON.stringify([...contract.referenceOwnerIds].sort()) || JSON.stringify(Object.keys(generatorOwners).sort()) !== JSON.stringify([...contract.generatorOwnerIds].sort())) throw new Error("Exact owner catalog consumer registries drifted");
  for (const [id, consumer] of Object.entries(referenceOwners)) if (typeof consumer.kind !== "string" || typeof consumer.ownerPath !== "string" || ["repo-cli-dev-docs-go", "commonmark-scratch-rust-reader", "vscode-package-ignore"].includes(id) && !exactOwnerPath(consumer.ownerPath)) throw new Error("Exact owner catalog has invalid concrete consumer: " + id);
  const sources = new Set<string>(), destinations = new Set<string>();
  const counts = { fixed: 0, license: 0, projected: 0, readme: 0, referenceBindings: 0, total: value.cases.length };
  const cases = value.cases.map((input) => {
    const row = object(input) as unknown as SemanticExactOwnedFileCase;
    const sourceBasename = typeof row.sourcePath === "string" ? semanticOwnedSourceBasename(row.sourcePath) : "";
    if (!exactOwnerPath(row.sourcePath) || !exactOwnerPath(row.destinationPath) || !(contract.sourceBasenames as readonly string[]).includes(sourceBasename) || !contract.allowedDispositions.includes(row.disposition) || !ownerEvidence[row.ownerEvidenceId] || sources.has(row.sourcePath) || destinations.has(row.destinationPath) || Buffer.byteLength(row.destinationPath) > 240) throw new Error("Exact owner catalog has invalid or duplicate source/destination ownership");
    const readme = sourceBasename === "README.md";
    const kind = contract.destinationDirectoryKinds[readme ? "readme" : "license"];
    const evidenceKind = ownerEvidence[row.ownerEvidenceId].kind;
    const expectedDisposition = evidenceKind === "package-publication" ? "fixed" : evidenceKind === "third-party-attribution" ? "attribution-relocate" : evidenceKind === "configurable-owner-license" ? "configurable-owner-license-relocate" : evidenceKind === "ticket-evidence" || evidenceKind === "ticket-scratch" ? "generated-evidence-relocate" : "owner-documentation-relocate";
    const fixed = row.disposition === "fixed";
    if (row.disposition !== expectedDisposition || row.destinationPath !== (fixed ? row.sourcePath : dirname(row.sourcePath) + "/" + kind.directoryName + "/" + kind.filename) || row.fixedContractId !== (fixed ? readme ? "bun-package-readme" : "bun-package-license" : null) || row.projectionContractId !== (fixed ? null : readme ? "exact-owner-readme-projection" : "exact-owner-license-projection")) throw new Error("Exact owner catalog classification or semantic destination drifted: " + row.sourcePath);
    if (!row.preimage || !/^[0-9a-f]{64}$/u.test(row.preimage.sha256) || row.preimage.mode !== "0644" || !Number.isSafeInteger(row.preimage.size) || row.preimage.size < 0 || !Array.isArray(row.referenceOwnerIds) || row.referenceOwnerIds.length === 0 || new Set(row.referenceOwnerIds).size !== row.referenceOwnerIds.length || row.referenceOwnerIds.some((id) => !referenceOwners[id])) throw new Error("Exact owner catalog leaf/reference evidence is invalid: " + row.sourcePath);
    if (row.generatorOwnerId !== null) {
      const generator = generatorOwners[row.generatorOwnerId], registered = taxonomy.generatorContracts[row.generatorOwnerId];
      const currentOutputPath = reservedDocumentationBasename(basename(row.sourcePath)) ? row.sourcePath : row.destinationPath;
      const retiredOutputPath = currentOutputPath === row.sourcePath ? row.destinationPath : row.sourcePath;
      if (!generator || !registered || generator.currentOutputPath !== row.sourcePath || generator.requiredOutputPath !== row.destinationPath || generator.ownerPath !== registered.ownerPath || generator.target !== registered.target || !registered.outputRoots.some((output) => output.path === currentOutputPath) || retiredOutputPath !== currentOutputPath && registered.outputRoots.some((output) => output.path === retiredOutputPath)) throw new Error("Exact owner catalog generator registration drifted: " + row.sourcePath);
    }
    counts[readme ? "readme" : "license"]++;
    counts[fixed ? "fixed" : "projected"]++;
    counts.referenceBindings += row.referenceOwnerIds.length;
    sources.add(row.sourcePath);
    destinations.add(row.destinationPath);
    return row;
  });
  if (Object.keys(counts).some((key) => counts[key as keyof typeof counts] !== contract.expectedCounts[key as keyof typeof counts])) throw new Error("Exact owner catalog census drifted");
  observe?.(snapshot);
  return { cases, ownerEvidence, referenceOwners, generatorOwners };
}

/** 🧭️ Resolves an exact owner leaf from language-neutral facts; raw-source bytes authorize projection once. */
export function semanticExactOwnedFileProjectionAuthority(catalog: SemanticExactOwnedFileCatalog, facts: Readonly<{ path: string; nodeKind: string; contentHash: string; mode: number; size: number; sourcePresent: boolean; destinationPresent: boolean; destinationPreimage?: Readonly<{ contentHash: string; mode: number; size: number }>; occupiedPaths: readonly string[] }>, current?: Readonly<{ contract: Pick<SemanticExactOwnedFileProjectionContract, "authorityCatalogPath" | "authorityCatalogSha256">; revisions: unknown; expectations: readonly SemanticOwnedCurrentSourceExpectation[] }>): Readonly<{ disposition: "none" | "fixed" | "project" | "regenerate" | "canonical" | "problem"; entry: SemanticExactOwnedFileCase | null; problems: readonly string[]; currentSource?: SemanticOwnedCurrentSourcePreimageResult }> {
  const entry = catalog.cases.find((entry) => entry.sourcePath === facts.path || entry.destinationPath === facts.path);
  if (!entry) return { disposition: "none", entry: null, problems: [] };
  const problems: string[] = [];
  const fixed = entry.disposition === "fixed", raw = facts.path === entry.sourcePath;
  let preimage = entry.preimage, currentSource: SemanticOwnedCurrentSourcePreimageResult | undefined;
  if (current) {
    const selected = semanticExactOwnedFileCurrentPreimageAuthority(catalog, current.contract, current.revisions, { ...facts, expectations: current.expectations });
    if (!raw || fixed || selected.disposition !== "revised" || selected.catalogCaseIndex !== catalog.cases.indexOf(entry) || !selected.preimage) problems.push(...(selected.problems.length ? selected.problems : ["Current source proof does not select this revised raw owner"]));
    else { preimage = selected.preimage; currentSource = selected; }
  }
  const convergentGenerator = entry.generatorOwnerId !== null && facts.destinationPreimage?.contentHash === entry.preimage.sha256 && facts.destinationPreimage?.mode === Number.parseInt(entry.preimage.mode, 8) && facts.destinationPreimage?.size === entry.preimage.size;
  if (facts.nodeKind !== "file") problems.push("Owner leaf must be a regular file");
  if (!fixed && facts.sourcePresent && facts.destinationPresent && !convergentGenerator) problems.push("Raw and projected owner leaves coexist");
  if (raw && (facts.contentHash !== preimage.sha256 || facts.mode !== Number.parseInt(preimage.mode, 8) || facts.size !== preimage.size)) problems.push("Frozen owner leaf preimage drifted");
  const fold = (path: string): string => path.normalize("NFC").replaceAll("\uFE0F", "").toLocaleLowerCase("und");
  if (!fixed && raw && facts.occupiedPaths.some((path) => path !== entry.sourcePath && !(convergentGenerator && path === entry.destinationPath) && fold(path) === fold(entry.destinationPath))) problems.push("Projected owner destination is occupied or folded-colliding");
  return { disposition: problems.length ? "problem" : fixed ? "fixed" : raw ? entry.generatorOwnerId ? "regenerate" : "project" : "canonical", entry, problems, ...(currentSource ? { currentSource } : {}) };
}

function exactOwnerGeneratorPrestate(repoRoot: string, outputPath: string, generatorId: string, catalog: SemanticExactOwnedFileCatalog | null): boolean {
  const entry = catalog?.cases.find((entry) => entry.generatorOwnerId === generatorId && entry.destinationPath === outputPath);
  if (!entry || exactOwnerRegularFile(repoRoot, entry.sourcePath) !== "file") return false;
  const stat = lstatSync(join(repoRoot, entry.sourcePath)), bytes = readFileSync(join(repoRoot, entry.sourcePath));
  return (stat.mode & 0o7777) === Number.parseInt(entry.preimage.mode, 8) && bytes.byteLength === entry.preimage.size && createHash("sha256").update(bytes).digest("hex") === entry.preimage.sha256;
}

export function validateGeneratorContractsAgainstWorkspace(repoRoot: string, taxonomy: Taxonomy = readTaxonomyUnchecked()): string[] {
  const problems: string[] = [];
  const root = resolve(repoRoot);
  const catalog = semanticExactOwnedFileCatalog(root, taxonomy);
  for (const [id, contract] of Object.entries(taxonomy.generatorContracts ?? {})) {
    if (contract.target) {
      if (!contract.ownerPath) {
        problems.push(`generatorContracts[${JSON.stringify(id)}] has a target without an ownerPath.`);
      } else {
        const manifestPath = join(root, contract.ownerPath, "📋️project.json");
        if (!existsSync(manifestPath)) {
          problems.push(`generatorContracts[${JSON.stringify(id)}] owner project is missing at ${JSON.stringify(relative(root, manifestPath))}.`);
        } else {
          try {
            const project = JSON.parse(readFileSync(manifestPath, "utf8")) as { name?: unknown; targets?: Record<string, { executor?: unknown; options?: Record<string, unknown> }> };
            const separator = contract.target.lastIndexOf(":");
            const projectName = contract.target.slice(0, separator);
            const targetName = contract.target.slice(separator + 1);
            if (project.name !== projectName || !project.targets?.[targetName]) problems.push(`generatorContracts[${JSON.stringify(id)}].target ${JSON.stringify(contract.target)} is absent from its owner project.`);
            if (contract.previewTarget) {
              const previewSeparator = contract.previewTarget.lastIndexOf(":");
              const previewProject = contract.previewTarget.slice(0, previewSeparator);
              const previewName = contract.previewTarget.slice(previewSeparator + 1);
              const preview = project.targets?.[previewName];
              if (project.name !== previewProject || !preview) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget ${JSON.stringify(contract.previewTarget)} is absent from its owner project.`);
              else if (preview.executor !== "nx:run-commands" || preview.options?.cwd !== contract.ownerPath || preview.options?.command !== `bun ./📜️script.ts ${generatorPreviewScriptArguments(contract).join(" ")}`) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget must route exactly to its declared script invocation in its owner project.`);
            }
            if (contract.checkTarget) {
              const checkSeparator = contract.checkTarget.lastIndexOf(":");
              const checkProject = contract.checkTarget.slice(0, checkSeparator);
              const checkName = contract.checkTarget.slice(checkSeparator + 1);
              if (project.name !== checkProject || !project.targets?.[checkName]) problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget ${JSON.stringify(contract.checkTarget)} is absent from its owner project.`);
            }
          } catch {
            problems.push(`generatorContracts[${JSON.stringify(id)}] owner project is not valid JSON.`);
          }
        }
      }
    }
    for (const output of contract.outputRoots ?? []) {
      const owners = generatorContractIdsForOutputPath(output.path, taxonomy);
      if (owners.length !== 1 || owners[0] !== id) problems.push(`generatorContracts[${JSON.stringify(id)}] output ${JSON.stringify(output.path)} does not have exactly one owner.`);
      if (output.inclusion === "tracked" && !existsSync(join(root, output.path)) && !exactOwnerGeneratorPrestate(root, output.path, id, catalog) && !nestedCargoGeneratedPrestate(root, output.path, id, taxonomy)) problems.push(`generatorContracts[${JSON.stringify(id)}] tracked output ${JSON.stringify(output.path)} is missing.`);
    }
  }
  return problems;
}

/** 📖️ Normative specification file-kind identifier for an artifact facet, if any. */
export function artifactSpecFileKindId(facetDirName: string, taxonomy: Taxonomy = loadTaxonomy()): string | undefined {
  return taxonomy.artifactSpecFileKinds?.[facetDirName];
}

/** 🧭️ Longest-prefix match of a repo-relative path against `taxonomy.areas` keys — `undefined` outside every declared area. */
export function areaOf(repoRelPath: string, taxonomy: Taxonomy = loadTaxonomy()): AreaState | undefined {
  const norm = repoRelPath.replaceAll("\\", "/").replace(/^\.\//, "");
  let bestKey: string | undefined;
  for (const key of Object.keys(taxonomy.areas)) {
    if (norm !== key && !norm.startsWith(`${key}/`)) continue;
    if (!bestKey || key.length > bestKey.length) bestKey = key;
  }
  return bestKey ? taxonomy.areas[bestKey] : undefined;
}
//#endregion 🔣️Taxonomy

//#region 🦀️RustStructure
export type RustStructuralVisibility = "private" | "pub" | `pub(${string})`;
export type RustStructuralFieldStyle = "unit" | "tuple" | "struct";

export interface RustModuleFact {
  readonly name: string;
  readonly visibility: RustStructuralVisibility;
  readonly inline: boolean;
  readonly pathTarget: string | null;
  readonly cfgTest: boolean;
}

export interface RustModuleGraphFact {
  readonly name: string;
  readonly modulePath: readonly string[];
  readonly visibility: RustStructuralVisibility;
  readonly inline: boolean;
  readonly pathTarget: string | null;
  readonly conditional?: true;
}

export interface RustModuleUseFact {
  readonly modulePath: readonly string[];
  readonly specifier: string;
  readonly relation: "import" | "reexport";
  readonly visibility: RustStructuralVisibility;
  readonly conditional?: true;
}

export interface RustEnumVariantFact {
  readonly name: string;
  readonly fieldStyle: RustStructuralFieldStyle;
  readonly fieldTypes: readonly string[];
  readonly wrappedTupleLeafType: string | null;
  readonly conditional?: true;
}

export interface RustEnumFact {
  readonly name: string;
  readonly visibility: RustStructuralVisibility;
  readonly variants: readonly RustEnumVariantFact[];
  readonly conditional?: true;
}

export interface RustPayloadFieldFact {
  readonly name: string | null;
  readonly type: string;
}

export interface RustInlinePayloadFact {
  readonly name: string;
  readonly visibility: RustStructuralVisibility;
  readonly fieldStyle: RustStructuralFieldStyle;
  readonly fields: readonly RustPayloadFieldFact[];
}

export interface RustImplFact {
  readonly traitPath: string | null;
  readonly selfType: string;
  readonly methods: readonly string[];
  readonly associatedConstants: readonly string[];
}

export interface RustConstIdentityFact {
  readonly owner: string | null;
  readonly name: string;
  readonly type: string | null;
  readonly value: string;
  readonly stringValue: string | null;
  readonly identityFields: Readonly<Record<string, string>>;
}

export interface RustMatchArmFact {
  readonly pattern: string;
  readonly variantPath: string | null;
  readonly expression: string;
}

export interface RustIncludeFact {
  readonly macro: "include" | "include_str" | "include_bytes";
  readonly expression: string;
  readonly usesOutDir: boolean;
}

export interface RustStructuralFacts {
  readonly schemaVersion: 1;
  readonly modules: readonly RustModuleFact[];
  readonly enums: readonly RustEnumFact[];
  readonly impls: readonly RustImplFact[];
  readonly inlinePayloads: readonly RustInlinePayloadFact[];
  readonly matchArms: readonly RustMatchArmFact[];
  readonly constants: readonly RustConstIdentityFact[];
  readonly includes: readonly RustIncludeFact[];
  readonly testModules: readonly RustModuleFact[];
}

export interface RustVirtualSourceFact {
  readonly path: string;
  readonly facts: RustStructuralFacts;
}

export interface RustTestModuleFact {
  readonly name: string;
  readonly modulePath: readonly string[];
  readonly mountBase: readonly string[];
  readonly pathTarget: string | null;
  readonly configuration: "enabled" | "disabled" | "ambiguous";
}

export interface RustRunnableTestFact {
  readonly name: string;
  readonly modulePath: readonly string[];
}

export interface RustTestFacts {
  readonly schemaVersion: 1;
  readonly runnableTests: readonly RustRunnableTestFact[];
  readonly mountedModules: readonly RustTestModuleFact[];
}

type RustTokenKind = "identifier" | "string" | "number" | "punctuation";

interface RustToken {
  readonly kind: RustTokenKind;
  readonly text: string;
  readonly start: number;
  readonly end: number;
}

interface RustAttributes {
  readonly ranges: readonly (readonly [number, number])[];
  readonly next: number;
}

interface RustVisibility {
  readonly value: RustStructuralVisibility;
  readonly next: number;
}

/** 🔤️ Recognizes a Rust identifier code point without interpreting comments or literal contents. */
function rustIdentifierPart(character: string): boolean {
  return character === "_" || /[\p{L}\p{N}]/u.test(character);
}

/** 🧵️ Decodes the identity-bearing value of one normal, byte, or raw Rust string token. */
function rustStringValue(token: RustToken | undefined): string | null {
  if (!token || token.kind !== "string") return null;
  let text = token.text;
  if (text.startsWith("b") && !text.startsWith("br")) text = text.slice(1);
  if (text.startsWith("br") || text.startsWith("r")) {
    const quote = text.indexOf('"');
    if (quote < 0) return null;
    const hashes = text.slice(text.startsWith("br") ? 2 : 1, quote).length;
    return text.slice(quote + 1, text.length - hashes - 1);
  }
  try {
    return JSON.parse(text) as string;
  } catch {
    return text.length >= 2 ? text.slice(1, -1) : null;
  }
}

/** 🧱️ Tokenizes Rust while discarding nested comments and keeping strings/chars/raw strings atomic. */
export function rustTokens(source: string): RustToken[] {
  const tokens: RustToken[] = [];
  const punctuation = ["::", "=>", "->", "..=", "...", "..", "&&", "||", "<=", ">=", "==", "!=", "<<=", ">>=", "<<", ">>"];
  let index = 0;
  while (index < source.length) {
    const start = index;
    const character = source[index]!;
    if (/\s/u.test(character)) {
      index += 1;
      continue;
    }
    if (source.startsWith("//", index)) {
      index = source.indexOf("\n", index + 2);
      if (index < 0) break;
      continue;
    }
    if (source.startsWith("/*", index)) {
      let depth = 1;
      index += 2;
      while (index < source.length && depth > 0) {
        if (source.startsWith("/*", index)) {
          depth += 1;
          index += 2;
        } else if (source.startsWith("*/", index)) {
          depth -= 1;
          index += 2;
        } else index += 1;
      }
      continue;
    }
    const rawPrefix = source.startsWith("br", index) ? 2 : source.startsWith("r", index) ? 1 : 0;
    if (rawPrefix > 0) {
      let cursor = index + rawPrefix;
      while (source[cursor] === "#") cursor += 1;
      if (source[cursor] === '"') {
        const hashes = cursor - index - rawPrefix;
        const suffix = `"${"#".repeat(hashes)}`;
        const close = source.indexOf(suffix, cursor + 1);
        index = close < 0 ? source.length : close + suffix.length;
        tokens.push({ kind: "string", text: source.slice(start, index), start, end: index });
        continue;
      }
    }
    if (character === '"' || (character === "b" && source[index + 1] === '"')) {
      index += character === "b" ? 2 : 1;
      while (index < source.length) {
        if (source[index] === "\\") index += 2;
        else if (source[index] === '"') {
          index += 1;
          break;
        } else index += 1;
      }
      tokens.push({ kind: "string", text: source.slice(start, index), start, end: index });
      continue;
    }
    if (character === "'" && source[index + 2] === "'") {
      index += 3;
      tokens.push({ kind: "string", text: source.slice(start, index), start, end: index });
      continue;
    }
    if (source.startsWith("r#", index) && source[index + 2] && rustIdentifierPart(source[index + 2]!) && !/[0-9]/u.test(source[index + 2]!)) {
      index += 2;
      while (index < source.length) {
        const next = String.fromCodePoint(source.codePointAt(index)!);
        if (!rustIdentifierPart(next)) break;
        index += next.length;
      }
      tokens.push({ kind: "identifier", text: source.slice(start, index), start, end: index });
      continue;
    }
    if (rustIdentifierPart(character) && !/[0-9]/u.test(character)) {
      index += character.length;
      while (index < source.length) {
        const next = String.fromCodePoint(source.codePointAt(index)!);
        if (!rustIdentifierPart(next)) break;
        index += next.length;
      }
      tokens.push({ kind: "identifier", text: source.slice(start, index), start, end: index });
      continue;
    }
    if (/[0-9]/u.test(character)) {
      index += 1;
      while (index < source.length && /[\p{L}\p{N}_.]/u.test(source[index]!)) index += 1;
      tokens.push({ kind: "number", text: source.slice(start, index), start, end: index });
      continue;
    }
    const operator = punctuation.find((candidate) => source.startsWith(candidate, index));
    index += operator?.length ?? 1;
    tokens.push({ kind: "punctuation", text: operator ?? character, start, end: index });
  }
  return tokens;
}

/** 🧩️ Pairs Rust delimiter tokens so all structural scans can skip nested syntax exactly. */
export function rustTokenPairs(tokens: readonly RustToken[]): ReadonlyMap<number, number> {
  const pairs = new Map<number, number>();
  const stack: { readonly index: number; readonly token: string }[] = [];
  const closeFor: Readonly<Record<string, string>> = { "(": ")", "[": "]", "{": "}" };
  for (let index = 0; index < tokens.length; index += 1) {
    const text = tokens[index]!.text;
    if (closeFor[text]) stack.push({ index, token: text });
    else if (text === ")" || text === "]" || text === "}") {
      const open = stack.at(-1);
      if (open && closeFor[open.token] === text) {
        stack.pop();
        pairs.set(open.index, index);
        pairs.set(index, open.index);
      }
    }
  }
  return pairs;
}

/** 📝️ Renders one token range in a deterministic compact Rust spelling. */
function rustTokenText(tokens: readonly RustToken[], start: number, end: number): string {
  return tokens.slice(start, end).map((token) => token.text).join(" ")
    .replace(/\s*::\s*/gu, "::")
    .replace(/\s*([<>(){}\[\],;:.!])\s*/gu, "$1")
    .replace(/\s+/gu, " ")
    .trim();
}

/** 🧩️ Splits a token range only at delimiters outside paired nested syntax. */
function rustTokenSegments(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number, delimiter: string): (readonly [number, number])[] {
  const segments: (readonly [number, number])[] = [];
  let segmentStart = start;
  for (let index = start; index < end; index += 1) {
    const pair = pairs.get(index);
    if (pair !== undefined && pair > index) {
      index = pair;
      continue;
    }
    if (tokens[index]!.text !== delimiter) continue;
    if (segmentStart < index) segments.push([segmentStart, index]);
    segmentStart = index + 1;
  }
  if (segmentStart < end) segments.push([segmentStart, end]);
  return segments;
}

/** 🗨️ Exact unescaped message argument owned by a standard Rust assertion macro. */
export interface RustAssertionMessageSpan {
  readonly macroName: string;
  readonly start: number;
  readonly end: number;
  readonly value: string;
}

/** 🧭️ Resolves assertion messages through Rust tokens and balanced macro arguments, never textual lookalikes. */
export function inspectRustAssertionMessageSpans(source: string): readonly RustAssertionMessageSpan[] {
  if (!/\b(?:debug_)?assert(?:_eq|_ne)?\s*!/u.test(source)) return [];
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), rows: RustAssertionMessageSpan[] = [];
  const argumentsByMacro: Readonly<Record<string, number>> = { assert: 1, assert_eq: 2, assert_ne: 2, debug_assert: 1, debug_assert_eq: 2, debug_assert_ne: 2 };
  if (tokens.some((token, index) => /^[()[\]{}]$/u.test(token.text) && !pairs.has(index))) return rows;
  for (let index = 0; index < tokens.length; index++) {
    const macroName = tokens[index]!.text, argumentIndex = argumentsByMacro[macroName];
    if (argumentIndex === undefined || tokens[index + 1]?.text !== "!" || tokens[index + 2]?.text !== "(") continue;
    const close = pairs.get(index + 2);
    if (close === undefined) continue;
    const argument = rustTokenSegments(tokens, pairs, index + 3, close, ",")[argumentIndex];
    if (!argument || argument[1] !== argument[0] + 1) continue;
    const token = tokens[argument[0]]!;
    if (token.kind !== "string" || !token.text.startsWith('"') || !token.text.endsWith('"') || token.text.includes("\\")) continue;
    rows.push({ macroName, start: token.start + 1, end: token.end - 1, value: token.text.slice(1, -1) });
  }
  return rows;
}

/** 🧭️ Recognizes every same-file zero-arg `fn NAME() -> PathBuf` whose ENTIRE body is exactly the
 * ancestor-walk-to-`nx.json` idiom seeded from `CARGO_MANIFEST_DIR` (`test_repo_root`/`find_repo_root`'s
 * shared strategy — 🏃️run/🦀️.rs, 📦️bin.rs): `let mut V = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
 * loop { if V.join("nx.json").is_file() { return V; } assert!(V.pop(), ..); }`. `nx.json` is this
 * repo's one canonical root marker — a single `nx.json` exists repo-wide, at the true root, never
 * nested under any crate — so any successful return from this loop is provably the repo root, the
 * same base a literal `CARGO_MANIFEST_DIR` proves, regardless of how many `.pop()` hops the walk
 * takes at runtime. Matched purely by shape (not by name), so it generalizes to every crate's own
 * copy of this idiom; a `SEMIO_REPO_ROOT`-env-var branch ahead of the loop (`find_repo_root`'s own
 * shape) is deliberately NOT matched — that path can resolve outside `CARGO_MANIFEST_DIR`'s ancestry
 * entirely, so proving it would need a genuinely different, unverified argument. */
function rustRepoRootAncestorWalkHelperNames(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): ReadonlySet<string> {
  const names = new Set<string>();
  const prefixes = [["PathBuf", "::", "from", "("], ["std", "::", "path", "::", "PathBuf", "::", "from", "("], ["Path", "::", "new", "("], ["std", "::", "path", "::", "Path", "::", "new", "("]];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text !== "fn") continue;
    const nameToken = tokens[index + 1];
    if (nameToken?.kind !== "identifier" || tokens[index + 2]?.text !== "(" || tokens[index + 3]?.text !== ")" || tokens[index + 4]?.text !== "->" || tokens[index + 5]?.text !== "PathBuf" || tokens[index + 6]?.text !== "{") continue;
    const bodyOpen = index + 6, bodyClose = pairs.get(bodyOpen);
    if (bodyClose === undefined) continue;
    let cursor = bodyOpen + 1;
    if (tokens[cursor]?.text !== "let") continue;
    cursor += 1;
    if (tokens[cursor]?.text === "mut") cursor += 1;
    const varToken = tokens[cursor];
    if (varToken?.kind !== "identifier") continue;
    const variable = varToken.text;
    cursor += 1;
    if (tokens[cursor]?.text !== "=") continue;
    cursor += 1;
    const prefix = prefixes.find((candidate) => candidate.every((text, offset) => tokens[cursor + offset]?.text === text));
    if (!prefix) continue;
    const openParen = cursor + prefix.length - 1, closeParen = pairs.get(openParen), environment = ["env", "!", "(", '"CARGO_MANIFEST_DIR"', ")"];
    if (closeParen === undefined || closeParen !== openParen + environment.length + 1 || !environment.every((text, offset) => tokens[openParen + offset + 1]?.text === text)) continue;
    cursor = closeParen + 1;
    if (tokens[cursor]?.text !== ";" || tokens[cursor + 1]?.text !== "loop" || tokens[cursor + 2]?.text !== "{") continue;
    const loopOpen = cursor + 2, loopClose = pairs.get(loopOpen);
    if (loopClose === undefined || loopClose + 1 !== bodyClose) continue;
    let inner = loopOpen + 1;
    if (tokens[inner]?.text !== "if" || tokens[inner + 1]?.text !== variable || tokens[inner + 2]?.text !== "." || tokens[inner + 3]?.text !== "join" || tokens[inner + 4]?.text !== "(") continue;
    const joinOpen = inner + 4, joinClose = pairs.get(joinOpen);
    if (joinClose === undefined) continue;
    const markerArgument = tokens.slice(joinOpen + 1, joinClose);
    if (markerArgument.length !== 1 || markerArgument[0]?.kind !== "string") continue;
    let after = joinClose + 1;
    if (tokens[after]?.text !== "." || tokens[after + 1]?.text !== "is_file" || tokens[after + 2]?.text !== "(" || tokens[after + 3]?.text !== ")" || tokens[after + 4]?.text !== "{") continue;
    const ifOpen = after + 4, ifClose = pairs.get(ifOpen);
    if (ifClose === undefined || tokens[ifOpen + 1]?.text !== "return" || tokens[ifOpen + 2]?.text !== variable || tokens[ifOpen + 3]?.text !== ";" || ifOpen + 4 !== ifClose) continue;
    const assertStart = ifClose + 1;
    if (tokens[assertStart]?.text !== "assert" || tokens[assertStart + 1]?.text !== "!" || tokens[assertStart + 2]?.text !== "(") continue;
    const assertOpen = assertStart + 2, assertClose = pairs.get(assertOpen);
    if (assertClose === undefined) continue;
    if (tokens[assertOpen + 1]?.text !== variable || tokens[assertOpen + 2]?.text !== "." || tokens[assertOpen + 3]?.text !== "pop" || tokens[assertOpen + 4]?.text !== "(" || tokens[assertOpen + 5]?.text !== ")" || tokens[assertOpen + 6]?.text !== ",") continue;
    if (tokens[assertClose + 1]?.text !== ";" || assertClose + 2 !== loopClose) continue;
    names.add(nameToken.text);
  }
  return names;
}

/** 🧭️ One path component and the immutable manifest-relative components preceding it. */
export interface RustManifestPathReference {
  readonly start: number;
  readonly end: number;
  readonly value: string;
  readonly base: readonly string[];
}

/** 🔗️ Proves local Path.join literals and single-use literal-array loop arguments from Rust syntax. */
export function inspectRustManifestPathReferences(source: string): readonly RustManifestPathReference[] {
  if (!source.includes("CARGO_MANIFEST_DIR") || !/\.\s*join\s*\(/u.test(source)) return [];
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens);
  if (tokens.some((token, index) => /^[()[\]{}]$/u.test(token.text) && !pairs.has(index))) return [];
  if (tokens.some((token, index) => token.text === "std" && ["mod", "as", "let"].includes(tokens[index - 1]?.text ?? ""))) return [];
  const repoRootHelperNames = rustRepoRootAncestorWalkHelperNames(tokens, pairs);
  type Loop = { readonly values: readonly RustToken[]; readonly uses: Set<number>; valid: boolean };
  type Binding = { readonly kind: "path"; readonly base: readonly string[] } | { readonly kind: "loop"; readonly loop: Loop };
  type Candidate = RustManifestPathReference & { readonly loop?: Loop };
  const rows: Candidate[] = [];
  const literal = (token: RustToken | undefined): token is RustToken & { readonly kind: "string" } => token?.kind === "string" && token.text.startsWith('"') && token.text.endsWith('"') && !token.text.includes("\\");
  const macros = new Set(["assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne"]);
  const formatArguments: Readonly<Record<string, number>> = { format: 0, format_args: 0, print: 0, println: 0, eprint: 0, eprintln: 0, panic: 0, write: 1, writeln: 1, assert: 1, assert_eq: 2, assert_ne: 2, debug_assert: 1, debug_assert_eq: 2, debug_assert_ne: 2 };
  const opaqueMacroContext = tokens.some((token) => token.text === "use" || token.text === "macro_rules");
  const formatCaptures = (text: string, name: string): boolean => {
    for (let index = 0; index < text.length; index++) {
      if (text[index] !== "{") continue;
      if (text[index + 1] === "{") { index++; continue; }
      const end = text.indexOf("}", index + 1), field = text.slice(index + 1, end < 0 ? undefined : end), colon = field.indexOf(":");
      if ((colon < 0 ? field : field.slice(0, colon)).trim() === name || colon >= 0 && [...field.slice(colon + 1).matchAll(/([_\p{ID_Start}][_\p{ID_Continue}]*)\$/gu)].some((match) => match[1] === name)) return true;
      if (end < 0) break;
      index = end;
    }
    return false;
  };
  const macroCaptures = (start: number, end: number, name: string): boolean => {
    for (let index = start; index + 2 < end; index++) {
      if (tokens[index + 1]?.text !== "!" || !["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) continue;
      const close = pairs.get(index + 2);
      if (close === undefined || close >= end) continue;
      const qualified = tokens[index - 1]?.text === "::", standard = qualified ? tokens[index - 2]?.text === "std" && (tokens[index - 3]?.text !== "::" || tokens[index - 4]?.kind !== "identifier") : !opaqueMacroContext;
      const argumentIndex = standard ? formatArguments[tokens[index]!.text] : undefined;
      const argument = argumentIndex === undefined ? undefined : rustTokenSegments(tokens, pairs, index + 3, close, ",")[argumentIndex], value = argument && argument[1] === argument[0] + 1 ? tokens[argument[0]] : undefined;
      if (literal(value)) { if (formatCaptures(value.text.slice(1, -1), name)) return true; }
      else if (tokens.slice(index + 3, close).some((token) => token.kind === "string" && token.text.split(/[^_\p{ID_Continue}]+/u).includes(name))) return true;
    }
    return false;
  };
  const path = (start: number, end: number, bindings: ReadonlyMap<string, Binding>, emit: boolean): { readonly end: number; readonly base: readonly string[] | null } | null => {
    let cursor = start, base: readonly string[] | null = null;
    const binding = bindings.get(tokens[cursor]?.text ?? "");
    if (binding?.kind === "path") { base = binding.base; cursor++; }
    else if (tokens[cursor]?.text === "(") {
      const close = pairs.get(cursor), nested = close === undefined ? null : path(cursor + 1, close, bindings, emit);
      if (!nested || nested.end !== close || nested.base === null) return null;
      base = nested.base; cursor = close! + 1;
    } else {
      if (tokens[cursor]?.text === "::") cursor++;
      const prefix = [["std", "::", "path", "::", "Path", "::", "new", "("], ["std", "::", "path", "::", "PathBuf", "::", "from", "("]].find((candidate) => candidate.every((text, offset) => tokens[cursor + offset]?.text === text));
      if (prefix) {
        const open = cursor + prefix.length - 1, close = pairs.get(open), environment = ["env", "!", "(", '"CARGO_MANIFEST_DIR"', ")"];
        if (close !== open + environment.length + 1 || !environment.every((text, offset) => tokens[open + offset + 1]?.text === text)) return null;
        base = []; cursor = close + 1;
      } else if (tokens[cursor]?.kind === "identifier" && repoRootHelperNames.has(tokens[cursor]!.text) && tokens[cursor + 1]?.text === "(" && tokens[cursor + 2]?.text === ")") {
        base = []; cursor = cursor + 3;
      } else return null;
    }
    while (cursor + 2 < end && tokens[cursor]?.text === "." && tokens[cursor + 1]?.text === "join" && tokens[cursor + 2]?.text === "(") {
      const close = pairs.get(cursor + 2);
      if (close === undefined || close >= end || base === null) break;
      const arguments_ = rustTokenSegments(tokens, pairs, cursor + 3, close, ",");
      const argument = arguments_.length === 1 ? arguments_[0] : undefined;
      if (!argument || argument[1] !== argument[0] + 1) break;
      const token = tokens[argument[0]]!, loop = token.kind === "identifier" ? bindings.get(token.text) : undefined;
      if (literal(token)) {
        const value = token.text.slice(1, -1);
        if (emit) rows.push({ start: token.start + 1, end: token.end - 1, value, base });
        base = [...base, value];
      } else if (loop?.kind === "loop") {
        if (emit) {
          loop.loop.uses.add(argument[0]);
          for (const value of loop.loop.values) rows.push({ start: value.start + 1, end: value.end - 1, value: value.text.slice(1, -1), base, loop: loop.loop });
        }
        base = null;
      } else break;
      cursor = close + 1;
    }
    return { end: cursor, base };
  };
  const visit = (start: number, end: number, bindings: Map<string, Binding>): void => {
    for (let index = start; index < end;) {
      const token = tokens[index]!;
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "") && !macros.has(token.text)) {
        index = (pairs.get(index + 2) ?? end - 1) + 1;
        continue;
      }
      if (token.text === "fn") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{", ";"])), close = pairs.get(open);
        if (open >= 0 && tokens[open]?.text === "{" && close !== undefined) visit(open + 1, close, new Map());
        index = open < 0 ? end : (close ?? open) + 1;
        continue;
      }
      if (token.text === "|" && [undefined, "=", "(", ",", "move"].includes(tokens[index - 1]?.text)) {
        const close = tokens.findIndex((candidate, offset) => offset > index && offset < end && candidate.text === "|");
        if (close < 0) return;
        const child = new Map(bindings), boundary = rustFindTopLevel(tokens, pairs, close + 1, end, new Set([","]));
        for (const parameter of tokens.slice(index + 1, close)) if (parameter.kind === "identifier") child.delete(parameter.text);
        visit(close + 1, boundary < 0 ? end : boundary, child);
        index = boundary < 0 ? end : boundary + 1;
        continue;
      }
      if (["if", "while"].includes(token.text) && tokens[index + 1]?.text === "let") {
        const open = rustFindTopLevel(tokens, pairs, index + 2, end, new Set(["{"])), close = pairs.get(open), equal = rustFindTopLevel(tokens, pairs, index + 2, open, new Set(["="]));
        if (open < 0 || close === undefined || equal < 0) return;
        const child = new Map(bindings);
        visit(equal + 1, open, bindings);
        for (const parameter of tokens.slice(index + 2, equal)) if (parameter.kind === "identifier") child.delete(parameter.text);
        visit(open + 1, close, child);
        index = close + 1;
        continue;
      }
      if (token.text === "match") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])), close = pairs.get(open);
        if (open < 0 || close === undefined) return;
        visit(index + 1, open, bindings);
        for (const [first, last] of rustTokenSegments(tokens, pairs, open + 1, close, ",")) {
          const arrow = rustFindTopLevel(tokens, pairs, first, last, new Set(["=>"]));
          if (arrow < 0) continue;
          const child = new Map(bindings);
          for (const parameter of tokens.slice(first, arrow)) if (parameter.kind === "identifier") child.delete(parameter.text);
          visit(arrow + 1, last, child);
        }
        index = close + 1;
        continue;
      }
      if (token.text === "for") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])), close = pairs.get(open);
        if (open < 0 || close === undefined) return;
        const name = tokens[index + 1], array = index + 3, arrayEnd = pairs.get(array), child = new Map(bindings);
        if (name?.kind === "identifier") child.delete(name.text);
        if (name?.kind === "identifier" && tokens[index + 2]?.text === "in" && tokens[array]?.text === "[" && arrayEnd === open - 1) {
          const elements = rustTokenSegments(tokens, pairs, array + 1, arrayEnd, ",");
          if (elements.length > 0 && elements.every(([first, last]) => first + 1 === last && literal(tokens[first]))) {
            const loop: Loop = { values: elements.map(([first]) => tokens[first]!), uses: new Set(), valid: false };
            child.set(name.text, { kind: "loop", loop });
            visit(open + 1, close, child);
            const uses = tokens.slice(open + 1, close).filter((candidate) => candidate.kind === "identifier" && candidate.text === name.text).length;
            loop.valid = uses === 1 && loop.uses.size === 1 && !macroCaptures(open + 1, close, name.text);
            index = close + 1;
            continue;
          }
        }
        visit(open + 1, close, child);
        index = close + 1;
        continue;
      }
      if (token.text === "let") {
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"]));
        if (boundary < 0) return;
        const equal = rustFindTopLevel(tokens, pairs, index + 1, boundary, new Set(["="])), name = tokens[index + 1]?.text === "mut" ? tokens[index + 2] : tokens[index + 1];
        if (equal >= 0) visit(equal + 1, boundary, bindings);
        const value = equal >= 0 ? path(equal + 1, boundary, bindings, false) : null;
        for (const parameter of tokens.slice(index + 1, equal < 0 ? boundary : equal)) if (parameter.kind === "identifier") bindings.delete(parameter.text);
        if (name?.kind === "identifier") {
          bindings.delete(name.text);
          if (equal === index + 2 && value?.end === boundary && value.base !== null) bindings.set(name.text, { kind: "path", base: value.base });
        }
        index = boundary + 1;
        continue;
      }
      if (token.kind === "identifier" && tokens[index + 1]?.text === "=") bindings.delete(token.text);
      const value = path(index, end, bindings, true);
      if (value) { index = value.end; continue; }
      const close = pairs.get(index);
      if (close !== undefined && close > index) {
        visit(index + 1, close, token.text === "{" ? new Map(bindings) : bindings);
        index = close + 1;
      } else index++;
    }
  };
  visit(0, tokens.length, new Map());
  const grouped = new Map<number, Candidate[]>();
  for (const row of rows.filter((row) => row.loop === undefined || row.loop.valid)) grouped.set(row.start, [...(grouped.get(row.start) ?? []), row]);
  return [...grouped.values()].filter((values) => new Set(values.map((row) => JSON.stringify(row.base))).size === 1).map(([row]) => ({ start: row!.start, end: row!.end, value: row!.value, base: row!.base })).sort((left, right) => left.start - right.start);
}

/** 🧮️ Complete manifest-relative alternatives for one literal, never writable reference authority. */
export interface RustManifestPathCandidate {
  readonly start: number;
  readonly end: number;
  readonly value: string;
  readonly targets: readonly (readonly string[])[];
}

/** 🧮️ Evaluates bounded immutable loop receivers while preserving tuple correlation and source identity. */
export function inspectRustManifestPathCandidates(source: string): readonly RustManifestPathCandidate[] {
  if (!source.includes("CARGO_MANIFEST_DIR") || !/\.\s*join\s*\(/u.test(source)) return [];
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), limit = 256;
  if (tokens.some((token, index) => /^[()[\]{}]$/u.test(token.text) && !pairs.has(index))) return [];
  if (tokens.some((token, index) => token.text === "std" && ["mod", "as", "let", "type", "struct", "use"].includes(tokens[index - 1]?.text ?? ""))) return [];
  const repoRootHelperNames = rustRepoRootAncestorWalkHelperNames(tokens, pairs);
  type State = { valid: boolean };
  type Value = { readonly state: State; readonly dependencies: readonly State[] } & (
    { readonly kind: "string"; readonly token: RustToken } |
    { readonly kind: "path"; readonly parts: readonly string[] } |
    { readonly kind: "array" | "tuple"; readonly values: readonly Value[] } |
    { readonly kind: "metadata" });
  type Row = { readonly token: RustToken; readonly alternatives: Map<string, readonly string[]>; readonly dependencies: State[] };
  const rows = new Map<number, Row>(), standardMacros = new Set(["assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne", "print", "println", "eprint", "eprintln", "format", "format_args", "panic", "write", "writeln"]);
  const shadowedMacros = new Set<string>(), moduleEnds: number[] = [];
  let wildcardMacroImport = false;
  for (let index = 0; index < tokens.length; index++) {
    while (moduleEnds.length && moduleEnds.at(-1)! < index) moduleEnds.pop();
    if (tokens[index]?.text === "mod" && tokens[index + 2]?.text === "{" && pairs.has(index + 2)) moduleEnds.push(pairs.get(index + 2)!);
    if (tokens[index]?.text === "macro_rules" && tokens[index + 1]?.text === "!") shadowedMacros.add(tokens[index + 2]?.text ?? "");
    if (tokens[index]?.text !== "use") continue;
    const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"])), specifier = tokens.slice(index + 1, end < 0 ? tokens.length : end);
    const text = specifier.map((token) => token.text).join(""), localParent = /^(?:super::)+\*$/u.test(text) && text.split("::").length - 1 <= moduleEnds.length;
    if (specifier.some((token) => token.text === "*") && !localParent) wildcardMacroImport = true;
    for (const token of specifier) if (standardMacros.has(token.text) || token.text === "env") shadowedMacros.add(token.text);
  }
  if (wildcardMacroImport || shadowedMacros.has("env") || tokens.some((token) => ["no_std", "no_implicit_prelude", "macro_use"].includes(token.text))) return [];
  let expanded = 0, overflow = false;
  const wrap = (value: Value, parents: readonly State[] = []): Value => ({ ...value, state: { valid: true }, dependencies: [...value.dependencies, value.state, ...parents] });
  const metadata = (): Value => ({ kind: "metadata", state: { valid: true }, dependencies: [] });
  const invalidate = (value: Value | undefined): void => { if (value) value.state.valid = false; };
  const states = (value: Value): readonly State[] => [value.state, ...value.dependencies];
  const parse = (start: number, end: number, bindings: ReadonlyMap<string, Value>, emit: boolean): { value: Value; end: number } | null => {
    let cursor = start, value: Value | undefined, token = tokens[cursor];
    if (!token) return null;
    if (token.kind === "string" && token.text.startsWith('"') && token.text.endsWith('"') && !token.text.includes("\\")) {
      value = { kind: "string", token, state: { valid: true }, dependencies: [] }; cursor++;
    } else if (token.kind === "number" || ["true", "false"].includes(token.text)) { value = metadata(); cursor++; }
    else if (bindings.has(token.text)) { value = bindings.get(token.text)!; cursor++; }
    else if (token.text === "(" || token.text === "[") {
      const close = pairs.get(cursor);
      if (close === undefined || close >= end) return null;
      const segments = rustTokenSegments(tokens, pairs, cursor + 1, close, ","), values: Value[] = [];
      for (const [first, last] of segments) {
        const child = parse(first, last, bindings, emit);
        if (!child || child.end !== last) return null;
        values.push(child.value);
      }
      value = token.text === "(" && values.length === 1 && tokens[close - 1]?.text !== "," ? values[0]! : { kind: token.text === "[" ? "array" : "tuple", values, state: { valid: true }, dependencies: [] };
      cursor = close + 1;
    } else if (token.text === "&" && tokens[cursor + 1]?.text === "[") {
      const close = pairs.get(cursor + 1), child = close === undefined ? null : parse(cursor + 1, close + 1, bindings, emit);
      if (!child || tokens[close! + 1]?.text !== "[" || tokens[close! + 2]?.text !== ".." || tokens[close! + 3]?.text !== "]") return null;
      value = child.value; cursor = close! + 4;
    } else {
      if (token.text === "::") cursor++;
      const prefix = [["std", "::", "path", "::", "Path", "::", "new", "("], ["std", "::", "path", "::", "PathBuf", "::", "from", "("]].find((row) => row.every((text, offset) => tokens[cursor + offset]?.text === text));
      if (prefix) {
        const open = cursor + prefix.length - 1, close = pairs.get(open), environment = ["env", "!", "(", '"CARGO_MANIFEST_DIR"', ")"];
        if (close !== open + environment.length + 1 || !environment.every((text, offset) => tokens[open + offset + 1]?.text === text)) return null;
        value = { kind: "path", parts: [], state: { valid: true }, dependencies: [] }; cursor = close + 1;
      } else if (tokens[cursor]?.kind === "identifier" && repoRootHelperNames.has(tokens[cursor]!.text) && tokens[cursor + 1]?.text === "(" && tokens[cursor + 2]?.text === ")") {
        value = { kind: "path", parts: [], state: { valid: true }, dependencies: [] }; cursor = cursor + 3;
      } else return null;
    }
    while (cursor + 2 < end && tokens[cursor]?.text === "." && tokens[cursor + 2]?.text === "(") {
      const close = pairs.get(cursor + 2);
      if (close === undefined || close >= end) break;
      if (tokens[cursor + 1]?.text === "iter" && value.kind === "array" && close === cursor + 3 && tokens.slice(close + 1, close + 5).map((item) => item.text).join("") === ".enumerate()" && close + 4 < end) {
        value = { kind: "array", values: value.values.map((item) => ({ kind: "tuple", values: [metadata(), wrap(item, states(value!))], state: { valid: true }, dependencies: [] })), state: { valid: true }, dependencies: states(value) };
        cursor = close + 5; continue;
      }
      if (tokens[cursor + 1]?.text !== "join" || value.kind !== "path") break;
      const segments = rustTokenSegments(tokens, pairs, cursor + 3, close, ","), argument = segments.length === 1 ? parse(segments[0]![0], segments[0]![1], bindings, emit) : null;
      if (!argument || argument.end !== segments[0]![1] || argument.value.kind !== "string") break;
      if (/^(?:\/|[A-Za-z]:)/u.test(argument.value.token.text.slice(1, -1))) break;
      const leaf = argument.value, parts: readonly string[] = [...value.parts, leaf.token.text.slice(1, -1)], dependencies: State[] = [...states(value), ...states(leaf)];
      if (emit) {
        const row: Row = rows.get(leaf.token.start) ?? { token: leaf.token, alternatives: new Map(), dependencies: [] };
        row.alternatives.set(JSON.stringify(parts), parts); row.dependencies.push(...dependencies); rows.set(leaf.token.start, row);
        if (row.alternatives.size > limit) overflow = true;
      }
      value = { kind: "path", parts, state: { valid: true }, dependencies }; cursor = close + 1;
    }
    return { value, end: cursor };
  };
  const bind = (start: number, end: number, value: Value, bindings: Map<string, Value>): boolean => {
    if (start + 1 === end && tokens[start]?.kind === "identifier") { bindings.set(tokens[start]!.text, wrap(value)); return true; }
    if (tokens[start]?.text !== "(" || pairs.get(start) !== end - 1 || value.kind !== "tuple") return false;
    const names = rustTokenSegments(tokens, pairs, start + 1, end - 1, ",");
    if (names.length !== value.values.length || names.some(([first, last]) => first + 1 !== last || tokens[first]?.kind !== "identifier") || new Set(names.map(([first]) => tokens[first]!.text).filter((name) => name !== "_")).size !== names.filter(([first]) => tokens[first]!.text !== "_").length) return false;
    for (let index = 0; index < names.length; index++) bindings.set(tokens[names[index]![0]]!.text, wrap(value.values[index]!, states(value)));
    return true;
  };
  const callbackNamespace = !tokens.some((token, index) => {
    if (token.text === "#" || token.text === "std" && ["enum", "union", "trait", "<", ","].includes(tokens[index - 1]?.text ?? "")) return true;
    if (tokens[index + 1]?.text !== "!" || !["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) return false;
    if (token.text !== "env" && !standardMacros.has(token.text)) return true;
    return tokens[index - 1]?.text === "::" ? tokens[index - 2]?.text !== "std" || tokens[index - 3]?.text === "::" && tokens[index - 4]?.kind === "identifier" : shadowedMacros.has(token.text);
  });
  const divergentCallback = (start: number, end: number, bindings: ReadonlyMap<string, Value>): boolean => {
    if (!callbackNamespace || tokens[start - 1]?.text !== "(" || pairs.get(start - 1) !== end || tokens[start - 2]?.text !== "unwrap_or_else" || tokens[start - 3]?.text !== "." || tokens[start - 4]?.text !== ")") return false;
    const readClose = start - 4, readOpen = pairs.get(readClose);
    if (readOpen === undefined || readClose !== readOpen + 7 || !["std", "::", "fs", "::", "read_to_string"].every((text, index) => tokens[readOpen - 5 + index]?.text === text)) return false;
    if (tokens[readOpen - 6]?.text === "." || tokens[readOpen - 6]?.text === "::" && tokens[readOpen - 7]?.kind === "identifier") return false;
    const receiver = tokens[readOpen + 1], label = tokens[readOpen + 5], parameter = tokens[start + 1];
    if (receiver?.kind !== "identifier" || label?.kind !== "identifier" || parameter?.kind !== "identifier" || parameter.text === "_" || bindings.has(parameter.text) || tokens[start + 2]?.text !== "|" || ![".", "join", "("].every((text, index) => tokens[readOpen + 2 + index]?.text === text) || pairs.get(readOpen + 4) !== readOpen + 6) return false;
    const leaf = bindings.get(label.text), path = parse(readOpen + 1, readClose, bindings, false);
    if (leaf?.kind !== "string" || !states(leaf).every((state) => state.valid) || path?.end !== readClose || path.value.kind !== "path" || !states(path.value).every((state) => state.valid) || !path.value.dependencies.includes(leaf.state)) return false;
    let cursor = start + 3;
    if (tokens[cursor]?.text === "::") cursor++;
    if (tokens[cursor]?.text === "std" && tokens[cursor + 1]?.text === "::") cursor += 2;
    else if (cursor !== start + 3 || shadowedMacros.has("panic")) return false;
    if (tokens[cursor]?.text !== "panic" || tokens[cursor + 1]?.text !== "!" || tokens[cursor + 2]?.text !== "(" || pairs.get(cursor + 2) !== cursor + 4 || cursor + 5 !== end) return false;
    const message = tokens[cursor + 3];
    if (message?.kind !== "string" || !message.text.startsWith('"') || !message.text.endsWith('"') || message.text.includes("\\")) return false;
    const text = message.text.slice(1, -1), captures = new Set<string>();
    for (let index = 0; index < text.length;) {
      if (text[index] === "}") return false;
      if (text[index] !== "{") { index++; continue; }
      const close = text.indexOf("}", index + 1), name = text.slice(index + 1, close);
      if (close < 0 || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(name) || name !== label.text && name !== parameter.text) return false;
      captures.add(name); index = close + 1;
    }
    return captures.has(label.text) && captures.has(parameter.text);
  };
  const visit = (start: number, end: number, bindings: Map<string, Value>, readOnly = false): void => {
    for (let index = start; index < end && !overflow;) {
      const token = tokens[index]!;
      if (token.text === "fn") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{", ";"])), close = pairs.get(open);
        if (open >= 0 && tokens[open]?.text === "{" && close !== undefined) visit(open + 1, close, new Map());
        index = open < 0 ? end : (close ?? open) + 1; continue;
      }
      if (["unsafe", "while", "match", "loop"].includes(token.text) || token.text === "if" && tokens[index + 1]?.text === "let") {
        for (const value of bindings.values()) invalidate(value);
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])), close = pairs.get(open);
        index = close === undefined ? end : close + 1; continue;
      }
      const attributeOpen = tokens[index - 1]?.text === "]" ? pairs.get(index - 1) : undefined;
      if (token.text === "|" && ([undefined, "=", "(", ",", "move"].includes(tokens[index - 1]?.text) || attributeOpen !== undefined && tokens[attributeOpen - 1]?.text === "#")) {
        if (divergentCallback(index, end, bindings)) { index = end; continue; }
        for (const value of bindings.values()) invalidate(value);
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"]));
        index = boundary < 0 ? end : boundary + 1; continue;
      }
      if (token.text === "for") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])), close = pairs.get(open), inToken = rustFindTopLevel(tokens, pairs, index + 1, open, new Set(["in"]));
        if (open < 0 || close === undefined || inToken < 0) return;
        const sequence = parse(inToken + 1, open, bindings, false);
        if (sequence?.end === open && sequence.value.kind === "array") {
          for (const item of sequence.value.values) {
            if (++expanded > limit) { overflow = true; return; }
            const child = new Map(bindings);
            if (!bind(index + 1, inToken, wrap(item, states(sequence.value)), child)) { for (const value of bindings.values()) invalidate(value); break; }
            visit(open + 1, close, child);
          }
        } else for (const value of bindings.values()) invalidate(value);
        index = close + 1; continue;
      }
      if (token.text === "let") {
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"]));
        if (boundary < 0) return;
        const equal = rustFindTopLevel(tokens, pairs, index + 1, boundary, new Set(["="])), resolved = equal < 0 ? null : parse(equal + 1, boundary, bindings, false);
        if (equal >= 0) visit(equal + 1, boundary, bindings, resolved?.end === boundary);
        for (const parameter of tokens.slice(index + 1, equal < 0 ? boundary : equal)) if (parameter.kind === "identifier") { invalidate(bindings.get(parameter.text)); bindings.delete(parameter.text); }
        if (equal >= 0 && resolved?.end === boundary) bind(index + 1, equal, resolved.value, bindings);
        index = boundary + 1; continue;
      }
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) {
        const close = pairs.get(index + 2);
        if (close === undefined) return;
        const qualified = tokens[index - 1]?.text === "::", standard = standardMacros.has(token.text) && (qualified ? tokens[index - 2]?.text === "std" && (tokens[index - 3]?.text !== "::" || tokens[index - 4]?.kind !== "identifier") : !wildcardMacroImport && !shadowedMacros.has(token.text));
        if (standard) visit(index + 3, close, bindings, true);
        else for (const item of tokens.slice(index + 3, close)) {
          if (item.kind === "identifier") invalidate(bindings.get(item.text));
          else if (item.kind === "string") for (const [name, value] of bindings) if (item.text.split(/[^_\p{ID_Continue}]+/u).includes(name)) invalidate(value);
        }
        index = close + 1; continue;
      }
      const parsed = parse(index, end, bindings, true);
      if (parsed?.value.kind === "path" && parsed.end > index + 1) { index = parsed.end; continue; }
      if (!readOnly && token.kind === "identifier" && bindings.has(token.text)) invalidate(bindings.get(token.text));
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, token.text === "{" ? new Map(bindings) : bindings, readOnly); index = close + 1; }
      else index++;
    }
  };
  visit(0, tokens.length, new Map());
  if (overflow) return [];
  return [...rows.values()].filter((row) => row.dependencies.every((state) => state.valid)).sort((left, right) => left.token.start - right.token.start).map((row) => ({ start: row.token.start + 1, end: row.token.end - 1, value: row.token.text.slice(1, -1), targets: [...row.alternatives.entries()].sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0).map(([, parts]) => parts) }));
}

/** 🧵️ Proves local standard string-collection delimiters without granting path receiver authority. */
function rustStringCollectionJoinArguments(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, macros: ReadonlySet<string>): ReadonlySet<number> {
  const shadows = new Set<string>(), names = new Set(["Vec", "String", "std", "format"]), moduleEnds: number[] = [];
  let wildcard = false, override = false;
  for (let index = 0; index < tokens.length; index++) {
    const token = tokens[index]!, previous = tokens[index - 1]?.text, next = tokens[index + 1]?.text;
    while (moduleEnds.length > 0 && moduleEnds.at(-1)! < index) moduleEnds.pop();
    if (token.text === "mod" && tokens[index + 1]?.kind === "identifier" && tokens[index + 2]?.text === "{" && pairs.has(index + 2)) moduleEnds.push(pairs.get(index + 2)!);
    if (token.text === "no_implicit_prelude" || token.text === "no_std") wildcard = true;
    if (token.text === "join" && previous === "fn") override = true;
    if (names.has(token.text) && ["struct", "enum", "union", "type", "trait", "mod", "as", "let"].includes(previous ?? "")) shadows.add(token.text);
    if (["fn", "impl", "struct", "enum", "trait", "type", "union"].includes(token.text)) {
      const open = index + (token.text === "impl" ? 1 : 2);
      if (tokens[open]?.text === "<") for (let cursor = open + 1, depth = 1; cursor < tokens.length && depth > 0; cursor++) {
        const value = tokens[cursor]!.text;
        if (depth === 1 && ["<", ","].includes(tokens[cursor - 1]?.text ?? "") && names.has(value)) shadows.add(value);
        if (value === "<") depth++;
        else if (value === ">" || value === ">>") depth -= value.length;
      }
    }
    if (token.text === "macro_rules" && next === "!" && names.has(tokens[index + 2]?.text ?? "")) shadows.add(tokens[index + 2]!.text);
    if (token.text === "use") {
      const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"]));
      const specifier = rustTokenText(tokens, index + 1, end < 0 ? tokens.length : end).replace(/\s/gu, "");
      const localParent = /^(?:super::)+\*$/u.test(specifier) && specifier.split("::").length - 1 <= moduleEnds.length;
      for (const item of tokens.slice(index + 1, end < 0 ? tokens.length : end)) {
        if (item.text === "*" && !localParent) wildcard = true;
        if (names.has(item.text) && item.text !== "std") shadows.add(item.text);
      }
    }
  }
  if (override) return new Set();
  type Collection = { valid: boolean; strings: boolean };
  const rows: { start: number; collection: Collection }[] = [];
  const compact = (start: number, end: number): string => rustTokenText(tokens, start, end).replace(/\s/gu, "");
  const standard = (value: string, unqualified: string, qualified: string): boolean => value === unqualified ? !wildcard && !shadows.has(unqualified.split(/[:<]/u)[0]!) : (value === qualified || value === "::" + qualified) && !shadows.has("std");
  const stringType = (start: number, end: number): boolean => {
    const match = /^(.*)<(.*)>$/u.exec(compact(start, end));
    return match !== null && standard(match[1]!, "Vec", "std::vec::Vec") && standard(match[2]!, "String", "std::string::String");
  };
  const stringValue = (start: number, end: number): boolean => {
    const token = tokens[start];
    if (start + 1 === end && token?.kind === "string" && (token.text.startsWith('"') || /^r#*"/u.test(token.text))) return true;
    let open = start;
    while (open < end && tokens[open]?.text !== "!") open++;
    if (tokens[open + 1]?.text !== "(" || pairs.get(open + 1) !== end - 1) return false;
    return standard(compact(start, open), "format", "std::format");
  };
  const clear = (start: number, end: number, bindings: Map<string, Collection>): void => { for (const token of tokens.slice(start, end)) if (token.kind === "identifier") bindings.delete(token.text); };
  const visit = (start: number, end: number, bindings: Map<string, Collection>): void => {
    for (let index = start; index < end;) {
      const token = tokens[index]!;
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) {
        const close = pairs.get(index + 2);
        if (close === undefined) return;
        if (macros.has(token.text)) visit(index + 3, close, bindings);
        else for (const item of tokens.slice(index + 3, close)) { const value = bindings.get(item.text); if (value) value.valid = false; }
        index = close + 1; continue;
      }
      if (token.text === "fn") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{", ";"])), close = pairs.get(open);
        if (open >= 0 && tokens[open]?.text === "{" && close !== undefined) visit(open + 1, close, new Map());
        index = open < 0 ? end : (close ?? open) + 1; continue;
      }
      if (token.text === "impl") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"]));
        index = open < 0 ? end : open; continue;
      }
      if (token.text === "let") {
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"]));
        if (boundary < 0) return;
        const equal = rustFindTopLevel(tokens, pairs, index + 1, boundary, new Set(["="])), nameIndex = tokens[index + 1]?.text === "mut" ? index + 2 : index + 1;
        if (equal >= 0) visit(equal + 1, boundary, bindings);
        clear(index + 1, equal < 0 ? boundary : equal, bindings);
        if (equal >= 0 && tokens[nameIndex]?.kind === "identifier" && (nameIndex + 1 === equal || tokens[nameIndex + 1]?.text === ":")) {
          const typed = tokens[nameIndex + 1]?.text === ":" && stringType(nameIndex + 2, equal);
          if (typed || standard(compact(equal + 1, boundary), "Vec::new()", "std::vec::Vec::new()")) bindings.set(tokens[nameIndex]!.text, { valid: true, strings: typed });
        }
        index = boundary + 1; continue;
      }
      if (token.text === "for" || ["if", "while"].includes(token.text) && tokens[index + 1]?.text === "let") {
        const patternStart = index + (token.text === "for" ? 1 : 2), split = rustFindTopLevel(tokens, pairs, patternStart, end, new Set([token.text === "for" ? "in" : "="])), open = rustFindTopLevel(tokens, pairs, split + 1, end, new Set(["{"])), close = pairs.get(open);
        if (split < 0 || open < 0 || close === undefined) return;
        visit(split + 1, open, bindings);
        const child = new Map(bindings); clear(patternStart, split, child); visit(open + 1, close, child);
        index = close + 1; continue;
      }
      if (token.text === "match") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])), close = pairs.get(open);
        if (open < 0 || close === undefined) return;
        visit(index + 1, open, bindings);
        for (const [first, last] of rustTokenSegments(tokens, pairs, open + 1, close, ",")) {
          const arrow = rustFindTopLevel(tokens, pairs, first, last, new Set(["=>"]));
          if (arrow < 0) continue;
          const child = new Map(bindings); clear(first, arrow, child); visit(arrow + 1, last, child);
        }
        index = close + 1; continue;
      }
      if (token.text === "|" && [undefined, "=", "(", ",", "move"].includes(tokens[index - 1]?.text)) {
        const close = tokens.findIndex((item, offset) => offset > index && offset < end && item.text === "|");
        if (close < 0) return;
        const boundary = rustFindTopLevel(tokens, pairs, close + 1, end, new Set([",", ";"])), child = new Map(bindings);
        clear(index + 1, close, child); visit(close + 1, boundary < 0 ? end : boundary, child);
        index = boundary < 0 ? end : boundary + 1; continue;
      }
      const collection = bindings.get(token.text);
      if (collection) {
        const method = tokens[index + 2]?.text, open = index + 3, close = pairs.get(open);
        if (tokens[index + 1]?.text === "." && tokens[open]?.text === "(" && close !== undefined) {
          const arguments_ = rustTokenSegments(tokens, pairs, open + 1, close, ","), argument = arguments_.length === 1 ? arguments_[0] : undefined;
          if (method === "push") { if (argument && stringValue(...argument)) collection.strings = true; else collection.valid = false; }
          else if (method === "join" && argument && argument[0] + 1 === argument[1] && tokens[argument[0]]?.kind === "string") rows.push({ start: tokens[argument[0]]!.start, collection });
          else if (!["len", "is_empty"].includes(method ?? "") || arguments_.length !== 0) collection.valid = false;
          visit(open + 1, close, bindings); index = close + 1; continue;
        }
        collection.valid = false;
        if (tokens[index + 1]?.text === "=") bindings.delete(token.text);
      }
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, token.text === "{" ? new Map(bindings) : bindings); index = close + 1; }
      else index++;
    }
  };
  visit(0, tokens.length, new Map());
  /** 🔗️ A second, binding-free pass: `<expr>.collect::<Vec<...>>().join("literal")` chained inline (no
   * intermediate `let`) can never be a `Path`/`PathBuf` receiver — `collect::<Vec<_>>()` is syntactically
   * guaranteed to produce a `Vec`, and `Path`/`PathBuf` are never spelled with a `collect` turbofish — so
   * this is provable purely structurally, without the `bindings` machinery above (which only sees named
   * local variables and therefore missed this exact idiom, the most common way Rust joins a mapped
   * iterator into one string). */
  const chained: number[] = [];
  for (let index = 0; index + 2 < tokens.length; index++) {
    if (tokens[index]!.text !== "." || tokens[index + 1]!.text !== "join" || tokens[index + 2]!.text !== "(") continue;
    const close = pairs.get(index + 2);
    if (close === undefined) continue;
    const arguments_ = rustTokenSegments(tokens, pairs, index + 3, close, ","), argument = arguments_.length === 1 ? arguments_[0] : undefined;
    if (!argument || argument[0] + 1 !== argument[1] || tokens[argument[0]]?.kind !== "string") continue;
    if (tokens[index - 1]?.text !== ")") continue;
    const receiverOpen = pairs.get(index - 1);
    if (receiverOpen === undefined || receiverOpen !== index - 2) continue;
    let depth = 0, cursor = receiverOpen - 1, turbofishOpen = -1;
    while (cursor >= 0) {
      const text = tokens[cursor]!.text, closers = text === ">>" ? 2 : text === ">" ? 1 : 0;
      if (closers > 0) { depth += closers; cursor--; continue; }
      if (text === "<") { depth -= 1; if (depth === 0) { turbofishOpen = cursor; break; } cursor--; continue; }
      if (depth === 0) break;
      cursor--;
    }
    if (turbofishOpen < 0 || tokens[turbofishOpen - 1]?.text !== "::" || tokens[turbofishOpen - 2]?.text !== "collect" || tokens[turbofishOpen - 3]?.text !== ".") continue;
    if (!standard(tokens[turbofishOpen + 1]?.text ?? "", "Vec", "std::vec::Vec")) continue;
    chained.push(tokens[argument[0]]!.start);
  }
  return new Set([...rows.filter(({ collection }) => collection.valid && collection.strings).map(({ start }) => start), ...chained]);
}


/** 🚧️ Identifies literal path candidates while excluding only proven standard collection delimiters. */
export function inspectRustJoinArgumentSpans(source: string): readonly Pick<RustManifestPathReference, "start" | "end" | "value">[] {
  if (!/\.\s*join\s*\(/u.test(source)) return [];
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), rows = new Map<number, Pick<RustManifestPathReference, "start" | "end" | "value">>();
  const macros = new Set(["assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne", "print", "println", "eprint", "eprintln", "format", "write", "writeln", "panic"]);
  const delimiters = rustStringCollectionJoinArguments(tokens, pairs, macros);
  const literal = (token: RustToken | undefined): token is RustToken & { readonly kind: "string" } => token?.kind === "string" && token.text.startsWith('"') && token.text.endsWith('"');
  const add = (token: RustToken): void => { if (!delimiters.has(token.start)) rows.set(token.start + 1, { start: token.start + 1, end: token.end - 1, value: token.text.slice(1, -1) }); };
  const visit = (start: number, end: number, loops: ReadonlyMap<string, readonly RustToken[]>): void => {
    for (let index = start; index < end; index++) {
      const token = tokens[index]!;
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "") && !macros.has(token.text)) { index = pairs.get(index + 2) ?? end; continue; }
      if (token.text === "for" && tokens[index + 1]?.kind === "identifier" && tokens[index + 2]?.text === "in" && tokens[index + 3]?.text === "[") {
        const arrayEnd = pairs.get(index + 3), open = arrayEnd === undefined ? undefined : arrayEnd + 1, close = open === undefined ? undefined : pairs.get(open);
        if (arrayEnd !== undefined && open !== undefined && tokens[open]?.text === "{" && close !== undefined) {
          const parts = rustTokenSegments(tokens, pairs, index + 4, arrayEnd, ","), child = new Map(loops);
          child.delete(tokens[index + 1]!.text);
          if (parts.every(([first, last]) => first + 1 === last && literal(tokens[first]))) child.set(tokens[index + 1]!.text, parts.map(([first]) => tokens[first]!));
          visit(open + 1, close, child); index = close; continue;
        }
      }
      if (token.text === "." && tokens[index + 1]?.text === "join" && tokens[index + 2]?.text === "(") {
        const close = pairs.get(index + 2), argument = tokens[index + 3];
        if (close === index + 4) {
          if (literal(argument)) add(argument);
          else if (argument?.kind === "identifier") for (const value of loops.get(argument.text) ?? []) add(value);
        }
      }
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, loops); index = close; }
    }
  };
  visit(0, tokens.length, new Map());
  return [...rows.values()].sort((left, right) => left.start - right.start);
}

/** 🚱️ Proves a `.join()` call chain's ROOT can never resolve to a manifest-relative path at plan
 * time — `env::temp_dir()`, an `env::var`/`env::var_os` runtime lookup (optionally routed through
 * `std::env::args()`-style CLI access), or a bare `fn` parameter this file never itself binds to a
 * `CARGO_MANIFEST_DIR` proof — so literal segments joined onto it name a freshly synthesized runtime
 * path, not a rewritable repository reference. Distinguishes PROVEN-non-repo from merely-unproven:
 * an unrecognized root is left alone so the caller still treats it as unresolved. One bounded
 * same-file helper-function hop (depth 1) lets a `fn returning_a_path() -> PathBuf { ... }` whose own
 * tail expression is proven non-repo count for its callers too, without unbounded recursion. */
export function inspectRustNonRepoJoinBaseSpans(source: string): ReadonlySet<number> {
  if (!/\.\s*join\s*\(/u.test(source)) return new Set();
  // 🔓️ No whole-file unpaired-bracket bail (unlike the CARGO_MANIFEST_DIR provers): this function
  // only ever SUPPRESSES a block, never emits a rewrite, and `inspectRustJoinArgumentSpans` — which
  // shares this exact tokenizer/pairer and already runs unguarded on every file this feeds — proves
  // the risk is contained; every match below still requires exact adjacent-token shape to fire.
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens);
  const rows = new Set<number>();
  type Binding = { readonly kind: "nonrepo" };
  const literal = (token: RustToken | undefined): token is RustToken & { readonly kind: "string" } => token?.kind === "string" && token.text.startsWith('"') && token.text.endsWith('"') && !token.text.includes("\\");
  const tokensEqual = (start: number, end: number, expected: readonly string[]): boolean => end - start === expected.length && expected.every((text, offset) => tokens[start + offset]?.text === text);
  const matchCall = (cursor: number, segments: readonly string[]): { readonly openParen: number; readonly close: number } | null => {
    let index = cursor;
    if (tokens[index]?.text === "::") index++;
    for (let step = 0; step < segments.length; step++) {
      if (tokens[index]?.text !== segments[step]) return null;
      index++;
      if (step + 1 < segments.length) { if (tokens[index]?.text !== "::") return null; index++; }
    }
    if (tokens[index]?.text !== "(") return null;
    const close = pairs.get(index);
    return close === undefined ? null : { openParen: index, close };
  };
  /** 🧫️ Consumes a bounded whitelist of pass-through combinators that never smuggle a fresh,
   * possibly-manifest-rooted value into the chain — only exact-matched fallbacks are trusted.
   * `parent`/`to_path_buf`/`clone`/`as_path`/`to_owned` are included because each is a pure identity
   * or ancestor-of-self operation on the SAME filesystem path: none of them can turn a path rooted
   * outside the repo into one rooted inside it. `parent` in particular only ever walks UP the tree —
   * a temp directory's ancestor is still under the same non-repo root, however many hops are taken —
   * so it stays sound with no bound on repetition, unlike e.g. `join`, which is handled separately by
   * the caller precisely because it CAN introduce a fresh, potentially manifest-relative segment. */
  const passThroughSuffix = (start: number, end: number): number => {
    let cursor = start;
    while (cursor + 2 < end && tokens[cursor]?.text === "." && tokens[cursor + 2]?.text === "(") {
      const name = tokens[cursor + 1]?.text, close = pairs.get(cursor + 2);
      if (close === undefined || close >= end) break;
      const empty = close === cursor + 3;
      if ((name === "unwrap" || name === "ok" || name === "next" || name === "unwrap_or_default" || name === "parent" || name === "to_path_buf" || name === "clone" || name === "as_path" || name === "to_owned") && empty) { cursor = close + 1; continue; }
      if ((name === "expect" || name === "nth" || name === "skip") && close > cursor + 2) { cursor = close + 1; continue; }
      if (name === "unwrap_or_else" && (tokensEqual(cursor + 3, close, ["std", "::", "env", "::", "temp_dir"]) || tokensEqual(cursor + 3, close, ["env", "::", "temp_dir"]))) { cursor = close + 1; continue; }
      if (name === "map" && (tokensEqual(cursor + 3, close, ["PathBuf", "::", "from"]) || tokensEqual(cursor + 3, close, ["std", "::", "path", "::", "PathBuf", "::", "from"]))) { cursor = close + 1; continue; }
      break;
    }
    return cursor;
  };
  const helperCache = new Map<string, boolean>(), helperInProgress = new Set<string>();
  /** 🪜️ One bounded hop into a same-file free function's own tail expression. Bails (never proves) on
   * anything but exactly one textual `fn NAME` match, keeping this sound without call-site tracing. */
  const helperReturnsNonRepo = (name: string, depth: number): boolean => {
    if (helperCache.has(name)) return helperCache.get(name)!;
    if (helperInProgress.has(name)) return false;
    helperInProgress.add(name);
    let result = false;
    const matches: number[] = [];
    for (let index = 0; index < tokens.length; index++) if (tokens[index]?.text === "fn" && tokens[index + 1]?.text === name) matches.push(index);
    if (matches.length === 1) {
      const cursor = matches[0]! + 2, paramsClose = tokens[cursor]?.text === "(" ? pairs.get(cursor) : undefined;
      const braceStart = paramsClose === undefined ? -1 : rustFindTopLevel(tokens, pairs, paramsClose + 1, tokens.length, new Set(["{", ";"]));
      const braceEnd = braceStart >= 0 && tokens[braceStart]?.text === "{" ? pairs.get(braceStart) : undefined;
      if (braceEnd !== undefined) result = bodyReturnsNonRepo(braceStart + 1, braceEnd, depth);
    }
    helperInProgress.delete(name);
    helperCache.set(name, result);
    return result;
  };
  /** 🌱️ Recognizes one non-repo root/base expression starting at `cursor`, returning the index just
   * past it (before any `.join()` chain), or null if unrecognized — callers must then still block. */
  const rootEnd = (cursor: number, end: number, bindings: ReadonlyMap<string, Binding>, depth: number): number | null => {
    if (tokens[cursor]?.text === "(") {
      const close = pairs.get(cursor);
      if (close === undefined || close >= end) return null;
      const inner = rootEnd(cursor + 1, close, bindings, depth);
      return inner === close ? close + 1 : null;
    }
    if (tokens[cursor]?.kind === "identifier" && tokens[cursor + 1]?.text !== "::" && bindings.get(tokens[cursor]!.text)?.kind === "nonrepo") return passThroughSuffix(cursor + 1, end);
    for (const segments of [["std", "env", "temp_dir"], ["env", "temp_dir"], ["std", "env", "args"], ["env", "args"]]) {
      const call = matchCall(cursor, segments);
      if (call && call.close === call.openParen + 1) return passThroughSuffix(call.close + 1, end);
    }
    // 🧪️ `<any path>::test_support::tempdir()` — a project-owned wrapper, uniquely defined once
    // (🏪️store/🦀️.rs `pub fn tempdir()`) as `std::env::temp_dir().join(...)`, called via
    // varying aliases/qualifiers (`store::test_support::tempdir()`, `crate::os_store::test_support
    // ::tempdir()`, …). Matched by SUFFIX so any qualifying prefix path is accepted, but a BARE
    // `tempdir()` with no `test_support::` qualifier ahead of it is never trusted.
    if (tokens[cursor]?.kind === "identifier") {
      let suffixCursor = cursor;
      while (tokens[suffixCursor]?.kind === "identifier" && tokens[suffixCursor + 1]?.text === "::") suffixCursor += 2;
      if (suffixCursor > cursor && tokens[suffixCursor - 2]?.text === "test_support" && tokens[suffixCursor]?.text === "tempdir" && tokens[suffixCursor + 1]?.text === "(") {
        const close = pairs.get(suffixCursor + 1);
        if (close === suffixCursor + 2) return passThroughSuffix(close + 1, end);
      }
    }
    for (const segments of [["std", "env", "var"], ["env", "var"], ["std", "env", "var_os"], ["env", "var_os"]]) {
      const call = matchCall(cursor, segments);
      if (call && call.close > call.openParen) return passThroughSuffix(call.close + 1, end);
    }
    for (const segments of [["std", "fs", "canonicalize"], ["fs", "canonicalize"]]) {
      const call = matchCall(cursor, segments);
      if (call && rootEnd(call.openParen + 1, call.close, bindings, depth) === call.close) return passThroughSuffix(call.close + 1, end);
    }
    for (const segments of [["Path", "new"], ["std", "path", "Path", "new"], ["PathBuf", "from"], ["std", "path", "PathBuf", "from"]]) {
      const call = matchCall(cursor, segments);
      if (call && rootEnd(call.openParen + 1, call.close, bindings, depth) === call.close) return passThroughSuffix(call.close + 1, end);
    }
    if (depth > 0 && tokens[cursor]?.kind === "identifier" && tokens[cursor + 1]?.text === "(") {
      const close = pairs.get(cursor + 1);
      if (close !== undefined && close < end && helperReturnsNonRepo(tokens[cursor]!.text, depth - 1)) return passThroughSuffix(close + 1, end);
    }
    return null;
  };
  /** 🔗️ Walks a maximal run of `.join(...)` calls off an already-proven non-repo root, recording only
   * bare string-literal arguments — non-literal steps don't break the chain's proven status. */
  const walkJoinChain = (start: number, end: number): number => {
    let cursor = start;
    while (cursor + 2 < end && tokens[cursor]?.text === "." && tokens[cursor + 1]?.text === "join" && tokens[cursor + 2]?.text === "(") {
      const close = pairs.get(cursor + 2);
      if (close === undefined || close >= end) break;
      if (close === cursor + 4 && literal(tokens[cursor + 3])) rows.add(tokens[cursor + 3]!.start + 1);
      cursor = close + 1;
    }
    return cursor;
  };
  /** 🧮️ Sequentially threads `let` bindings through one function body's statements, then classifies
   * only whether its tail (implicit-return) expression's root is proven non-repo — no row recording,
   * since the outer `visit` below independently walks this exact same body for its own literal joins. */
  const bodyReturnsNonRepo = (start: number, end: number, depth: number): boolean => {
    const bindings = new Map<string, Binding>();
    const segments = rustTokenSegments(tokens, pairs, start, end, ";");
    const hasTrailingSemicolon = end > start && tokens[end - 1]?.text === ";";
    for (let index = 0; index < segments.length; index++) {
      const [segStart, segEnd] = segments[index]!, isTail = index === segments.length - 1 && !hasTrailingSemicolon;
      if (!isTail) {
        if (tokens[segStart]?.text === "let") {
          const equal = rustFindTopLevel(tokens, pairs, segStart + 1, segEnd, new Set(["="]));
          const nameIndex = tokens[segStart + 1]?.text === "mut" ? segStart + 2 : segStart + 1, nameToken = tokens[nameIndex];
          if (equal >= 0 && nameToken?.kind === "identifier" && nameIndex + 1 === equal) {
            // 🌓️ Evaluate the RHS BEFORE deleting the old binding: `let x = f(x)` (re-binding
            // shadowing, e.g. `PathBuf::from(out_dir)`) must still see the prior `x`.
            const proven = rootEnd(equal + 1, segEnd, bindings, depth) !== null;
            bindings.delete(nameToken.text);
            if (proven) bindings.set(nameToken.text, { kind: "nonrepo" });
          }
        }
        continue;
      }
      return rootEnd(segStart, segEnd, bindings, depth) !== null;
    }
    return false;
  };
  /** 🔎️ Locates one free function's body and positional parameter names — qualified to a specific
   * `mod NAME { .. }` block when `qualifier` is given, else uniquely by bare name — refusing on any
   * ambiguity (more than one candidate) exactly like `helperReturnsNonRepo`'s same-file uniqueness
   * requirement, just scoped one level narrower so two same-named functions in different modules
   * (e.g. two sibling test modules each declaring their own `materialize`) never collide. */
  const resolveQualifiedFunctionBody = (qualifier: string | null, name: string): { readonly open: number; readonly close: number; readonly params: readonly string[] } | null => {
    let searchStart = 0, searchEnd = tokens.length;
    if (qualifier !== null) {
      const modMatches: number[] = [];
      for (let index = 0; index < tokens.length; index++) if (tokens[index]?.text === "mod" && tokens[index + 1]?.text === qualifier && tokens[index + 2]?.text === "{") modMatches.push(index);
      if (modMatches.length !== 1) return null;
      const modOpen = modMatches[0]! + 2, modClose = pairs.get(modOpen);
      if (modClose === undefined) return null;
      searchStart = modOpen + 1; searchEnd = modClose;
    }
    const fnMatches: number[] = [];
    for (let index = searchStart; index < searchEnd; index++) if (tokens[index]?.text === "fn" && tokens[index + 1]?.text === name) fnMatches.push(index);
    if (fnMatches.length !== 1) return null;
    const fnIndex = fnMatches[0]!;
    if (tokens[fnIndex + 2]?.text !== "(") return null;
    const paramsOpen = fnIndex + 2, paramsClose = pairs.get(paramsOpen);
    if (paramsClose === undefined) return null;
    const braceStart = rustFindTopLevel(tokens, pairs, paramsClose + 1, searchEnd, new Set(["{", ";"]));
    const braceEnd = braceStart >= 0 && tokens[braceStart]?.text === "{" ? pairs.get(braceStart) : undefined;
    if (braceEnd === undefined) return null;
    const params = rustTokenSegments(tokens, pairs, paramsOpen + 1, paramsClose, ",").map(([first, last]) => {
      const nameIndex = tokens[first]?.text === "mut" ? first + 1 : first;
      return tokens[nameIndex]?.kind === "identifier" && tokens[nameIndex + 1]?.text === ":" && nameIndex < last ? tokens[nameIndex]!.text : "";
    });
    return { open: braceStart + 1, close: braceEnd, params };
  };
  /** 🧩️ Evaluates one function body's TUPLE tail into per-position proven-non-repo flags, given the
   * literal string arguments (if any) its caller passed for named parameters. A leading top-level
   * `match <param> { "lit" => .., .., _ => {} }` — reachable only when the body's tail has no
   * separating `;` from a preceding brace-terminated statement — is eliminated (treated as a no-op
   * and skipped) ONLY when every non-wildcard arm is a single string literal that provably does not
   * equal the known argument, and the trailing `_` arm is empty: that proves, for THIS SPECIFIC
   * call's actual argument, only the wildcard arm can run, without evaluating any other arm's effects
   * (sound constant-propagation, not "assume the happy path" — any arm that could match, or a match
   * whose scrutinee isn't a known literal, refuses instead of guessing). Bounded to exactly one same-
   * file hop for the callee's OWN internal lets (`depth: 1` below), matching the one-hop convention
   * used throughout this function. */
  const tupleTailPositions = (bodyOpen: number, bodyClose: number, literalBindings: ReadonlyMap<string, string>): readonly boolean[] | null => {
    const bindings = new Map<string, Binding>();
    const segments = rustTokenSegments(tokens, pairs, bodyOpen, bodyClose, ";");
    const hasTrailingSemicolon = bodyClose > bodyOpen && tokens[bodyClose - 1]?.text === ";";
    for (let index = 0; index < segments.length; index++) {
      const [segStart, segEnd] = segments[index]!, isTail = index === segments.length - 1 && !hasTrailingSemicolon;
      if (!isTail) {
        if (tokens[segStart]?.text === "let") {
          const equal = rustFindTopLevel(tokens, pairs, segStart + 1, segEnd, new Set(["="]));
          const nameIndex = tokens[segStart + 1]?.text === "mut" ? segStart + 2 : segStart + 1, nameToken = tokens[nameIndex];
          if (equal >= 0 && nameToken?.kind === "identifier" && nameIndex + 1 === equal) {
            const proven = rootEnd(equal + 1, segEnd, bindings, 1) !== null;
            bindings.delete(nameToken.text);
            if (proven) bindings.set(nameToken.text, { kind: "nonrepo" });
          }
        }
        continue;
      }
      let tailStart = segStart;
      if (tokens[tailStart]?.text === "match") {
        const braceIndex = rustFindTopLevel(tokens, pairs, tailStart + 1, segEnd, new Set(["{"]));
        const matchClose = braceIndex < 0 ? undefined : pairs.get(braceIndex);
        if (braceIndex < 0 || matchClose === undefined) return null;
        const scrutinee = tokens.slice(tailStart + 1, braceIndex);
        if (scrutinee.length !== 1 || scrutinee[0]?.kind !== "identifier") return null;
        const literalValue = literalBindings.get(scrutinee[0]!.text);
        if (literalValue === undefined) return null;
        const arms = rustTokenSegments(tokens, pairs, braceIndex + 1, matchClose, ",");
        if (arms.length === 0) return null;
        for (let armIndex = 0; armIndex < arms.length; armIndex++) {
          const [armFirst, armLast] = arms[armIndex]!;
          const arrow = rustFindTopLevel(tokens, pairs, armFirst, armLast, new Set(["=>"]));
          if (arrow < 0) return null;
          const patternLength = arrow - armFirst, isLastArm = armIndex === arms.length - 1;
          if (isLastArm) {
            if (patternLength !== 1 || tokens[armFirst]?.text !== "_") return null;
            const bodyTokens = tokens.slice(arrow + 1, armLast);
            const emptyBody = bodyTokens.length === 0 || (bodyTokens.length === 2 && (bodyTokens[0]!.text === "{" && bodyTokens[1]!.text === "}" || bodyTokens[0]!.text === "(" && bodyTokens[1]!.text === ")"));
            if (!emptyBody) return null;
          } else {
            if (patternLength !== 1 || tokens[armFirst]?.kind !== "string") return null;
            if (tokens[armFirst]!.text.slice(1, -1) === literalValue) return null;
          }
        }
        tailStart = matchClose + 1;
        if (tailStart >= segEnd) return null;
      }
      if (tokens[tailStart]?.text === "(" && pairs.get(tailStart) === segEnd - 1) {
        const elements = rustTokenSegments(tokens, pairs, tailStart + 1, segEnd - 1, ",");
        if (elements.length < 2) return null;
        return elements.map(([elFirst, elLast]) => rootEnd(elFirst, elLast, bindings, 1) === elLast);
      }
      return [rootEnd(tailStart, segEnd, bindings, 1) === segEnd];
    }
    return null;
  };
  /** 🪢️ Resolves a tuple-pattern `let`'s right-hand-side call (`qualifier::name(..)` or `name(..)`,
   * consuming the ENTIRE span up to the statement's `;` — no trailing suffix chain trusted) into per-
   * position proven-non-repo flags for its returned tuple, threading any literal string arguments
   * into the callee by parameter name. */
  const tupleCallNonRepoPositions = (start: number, end: number, arity: number): readonly boolean[] | null => {
    let cursor = start, qualifier: string | null = null;
    if (tokens[cursor]?.kind !== "identifier") return null;
    if (tokens[cursor + 1]?.text === "::" && tokens[cursor + 2]?.kind === "identifier") { qualifier = tokens[cursor]!.text; cursor += 2; }
    const nameToken = tokens[cursor];
    if (nameToken?.kind !== "identifier" || tokens[cursor + 1]?.text !== "(") return null;
    const argOpen = cursor + 1, argClose = pairs.get(argOpen);
    if (argClose === undefined || argClose + 1 !== end) return null;
    const resolved = resolveQualifiedFunctionBody(qualifier, nameToken.text);
    if (!resolved || resolved.params.length === 0) return null;
    const argumentSpans = rustTokenSegments(tokens, pairs, argOpen + 1, argClose, ",");
    const literalBindings = new Map<string, string>();
    for (let argIndex = 0; argIndex < argumentSpans.length && argIndex < resolved.params.length; argIndex++) {
      const [argFirst, argLast] = argumentSpans[argIndex]!, parameterName = resolved.params[argIndex];
      if (argLast === argFirst + 1 && literal(tokens[argFirst]) && parameterName) literalBindings.set(parameterName, tokens[argFirst]!.text.slice(1, -1));
    }
    const positions = tupleTailPositions(resolved.open, resolved.close, literalBindings);
    return positions && positions.length === arity ? positions : null;
  };
  const visit = (start: number, end: number, bindings: Map<string, Binding>): void => {
    for (let index = start; index < end;) {
      const token = tokens[index]!;
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) {
        index = (pairs.get(index + 2) ?? end - 1) + 1;
        continue;
      }
      if (token.text === "fn") {
        const paramsOpen = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["("]));
        const paramsClose = paramsOpen < 0 ? undefined : pairs.get(paramsOpen);
        const braceStart = paramsClose === undefined ? -1 : rustFindTopLevel(tokens, pairs, paramsClose + 1, end, new Set(["{", ";"]));
        const braceEnd = braceStart >= 0 && tokens[braceStart]?.text === "{" ? pairs.get(braceStart) : undefined;
        if (paramsClose !== undefined && braceEnd !== undefined) {
          const fresh = new Map<string, Binding>();
          for (const [first, last] of rustTokenSegments(tokens, pairs, paramsOpen + 1, paramsClose, ",")) {
            const nameIndex = tokens[first]?.text === "mut" ? first + 1 : first;
            if (tokens[nameIndex]?.kind === "identifier" && tokens[nameIndex + 1]?.text === ":" && nameIndex < last) fresh.set(tokens[nameIndex]!.text, { kind: "nonrepo" });
          }
          visit(braceStart + 1, braceEnd, fresh);
        }
        index = braceStart < 0 ? (paramsClose === undefined ? index + 1 : paramsClose + 1) : (braceEnd ?? braceStart) + 1;
        continue;
      }
      if (token.text === "let") {
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"])), stop = boundary < 0 ? end : boundary;
        // 🧬️ Tuple-pattern destructure `let (a, _, c) = qualifier::fn(..);` — bind each named (non-`_`)
        // position whose slot is independently proven non-repo by `tupleCallNonRepoPositions`; any
        // unrecognized shape just binds nothing (the prior, always-safe default).
        if (tokens[index + 1]?.text === "(") {
          const patternClose = pairs.get(index + 1);
          const elements = patternClose === undefined ? [] : rustTokenSegments(tokens, pairs, index + 2, patternClose, ",");
          const names: (string | null)[] = [];
          let patternOk = patternClose !== undefined && patternClose < stop && tokens[patternClose + 1]?.text === "=" && elements.length > 0;
          if (patternOk) for (const [first, last] of elements) {
            if (last === first + 1 && tokens[first]?.text === "_") { names.push(null); continue; }
            if (last === first + 1 && tokens[first]?.kind === "identifier") { names.push(tokens[first]!.text); continue; }
            if (last === first + 2 && tokens[first]?.text === "mut" && tokens[first + 1]?.kind === "identifier") { names.push(tokens[first + 1]!.text); continue; }
            patternOk = false; break;
          }
          if (patternOk) {
            for (const name of names) if (name) bindings.delete(name);
            const positions = tupleCallNonRepoPositions(patternClose! + 2, stop, names.length);
            if (positions) for (let position = 0; position < names.length; position++) { const name = names[position]; if (name && positions[position]) bindings.set(name, { kind: "nonrepo" }); }
          }
          index = boundary < 0 ? end : boundary + 1;
          continue;
        }
        const equal = rustFindTopLevel(tokens, pairs, index + 1, stop, new Set(["="]));
        const nameIndex = tokens[index + 1]?.text === "mut" ? index + 2 : index + 1, nameToken = tokens[nameIndex];
        if (equal >= 0 && nameToken?.kind === "identifier" && nameIndex + 1 === equal) {
          // 🌓️ Same ordering fix as `bodyReturnsNonRepo`: prove the RHS against the OLD binding of
          // `nameToken.text` (self-shadowing, e.g. `let out_dir = PathBuf::from(out_dir)`) before
          // clearing it — deleting first would make the reference to the shadowed value unprovable.
          const rootAfter = rootEnd(equal + 1, stop, bindings, 1);
          bindings.delete(nameToken.text);
          if (rootAfter !== null) { bindings.set(nameToken.text, { kind: "nonrepo" }); walkJoinChain(rootAfter, stop); }
        } else if (nameToken?.kind === "identifier") bindings.delete(nameToken.text);
        index = boundary < 0 ? end : boundary + 1;
        continue;
      }
      const rootAfter = rootEnd(index, end, bindings, 1);
      if (rootAfter !== null) { index = walkJoinChain(rootAfter, end); continue; }
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, token.text === "{" ? new Map(bindings) : bindings); index = close + 1; }
      else index++;
    }
  };
  visit(0, tokens.length, new Map());
  return rows;
}

/** 🏷️ Reads consecutive outer attributes attached to one Rust item. */
function rustAttributes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): RustAttributes {
  const ranges: (readonly [number, number])[] = [];
  let index = start;
  while (index + 1 < end && tokens[index]!.text === "#" && tokens[index + 1]!.text === "[") {
    const close = pairs.get(index + 1);
    if (close === undefined || close >= end) break;
    ranges.push([index + 2, close]);
    index = close + 1;
  }
  return { ranges, next: index };
}

/** 👁️ Reads Rust item visibility without flattening restricted `pub(...)` scopes. */
function rustVisibility(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number): RustVisibility {
  if (tokens[start]?.text !== "pub") return { value: "private", next: start };
  if (tokens[start + 1]?.text !== "(") return { value: "pub", next: start + 1 };
  const close = pairs.get(start + 1);
  if (close === undefined) return { value: "pub", next: start + 1 };
  return { value: `pub(${rustTokenText(tokens, start + 2, close)})`, next: close + 1 };
}

/** 📍️ Extracts a decoded `#[path = "..."]` target from parsed item attributes. */
function rustPathAttribute(tokens: readonly RustToken[], attributes: RustAttributes): string | null {
  for (const [start, end] of attributes.ranges) {
    if (tokens[start]?.text !== "path") continue;
    for (let index = start + 1; index < end; index += 1) if (tokens[index]!.text === "=") return rustStringValue(tokens[index + 1]);
  }
  return null;
}

/** 🧪️ Identifies a test-only module from tokenized `cfg(...test...)` attributes. */
function rustCfgTest(tokens: readonly RustToken[], attributes: RustAttributes): boolean {
  return attributes.ranges.some(([start, end]) => tokens[start]?.text === "cfg" && tokens.slice(start + 1, end).some((token) => token.kind === "identifier" && token.text === "test"));
}

//#region 🧪️RustRunnableTests
type RustTestConfiguration = "enabled" | "disabled" | "ambiguous";

interface RustTestAttributes {
  readonly configuration: RustTestConfiguration;
  readonly test: boolean;
  readonly ignored: boolean;
  readonly pathTarget: string | null;
}

/** 🧪️ Combines Rust test configurations without treating unknown cfg values as enabled. */
function rustTestConfigurationAnd(left: RustTestConfiguration, right: RustTestConfiguration): RustTestConfiguration {
  if (left === "disabled" || right === "disabled") return "disabled";
  if (left === "ambiguous" || right === "ambiguous") return "ambiguous";
  return "enabled";
}

/** 🧪️ Negates the test-mode truth value of one parsed cfg predicate. */
function rustTestConfigurationNot(value: RustTestConfiguration): RustTestConfiguration {
  return value === "enabled" ? "disabled" : value === "disabled" ? "enabled" : "ambiguous";
}

/** 🧪️ Evaluates only cfg predicates whose truth is proven by rustc --test. */
function rustTestConfigurationExpression(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): RustTestConfiguration {
  const head = tokens[start];
  if (!head || head.kind !== "identifier") return "ambiguous";
  if (head.text === "test" && start + 1 === end) return "enabled";
  const open = tokens[start + 1]?.text === "(" ? start + 1 : -1;
  const close = open < 0 ? undefined : pairs.get(open);
  if (close === undefined || close + 1 !== end) return "ambiguous";
  const parts = rustTokenSegments(tokens, pairs, open + 1, close, ",");
  if (head.text === "not" && parts.length === 1) return rustTestConfigurationNot(rustTestConfigurationExpression(tokens, pairs, ...parts[0]!));
  if (head.text === "all") return parts.reduce<RustTestConfiguration>((value, part) => rustTestConfigurationAnd(value, rustTestConfigurationExpression(tokens, pairs, ...part)), "enabled");
  if (head.text === "any") {
    const values = parts.map((part) => rustTestConfigurationExpression(tokens, pairs, ...part));
    if (values.includes("enabled")) return "enabled";
    return values.every((value) => value === "disabled") ? "disabled" : "ambiguous";
  }
  return "ambiguous";
}

/** 🧪️ Reads consecutive inner attributes that configure their enclosing module or crate scope. */
function rustInnerAttributes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): RustAttributes {
  const ranges: (readonly [number, number])[] = [];
  let index = start;
  while (index + 2 < end && tokens[index]!.text === "#" && tokens[index + 1]!.text === "!" && tokens[index + 2]!.text === "[") {
    const close = pairs.get(index + 2);
    if (close === undefined || close >= end) break;
    ranges.push([index + 3, close]);
    index = close + 1;
  }
  return { ranges, next: index };
}

/** 🧪️ Recognizes whether an unresolved cfg_attr could change a test's executability or source identity. */
function rustTestAttributeCanAffectExecution(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): boolean {
  const name = tokens[start]?.text;
  if (name === "cfg" || name === "ignore" || name === "path") return true;
  if (name !== "cfg_attr" || tokens[start + 1]?.text !== "(") return true;
  const close = pairs.get(start + 1);
  if (close === undefined || close + 1 !== end) return true;
  const parts = rustTokenSegments(tokens, pairs, start + 2, close, ",");
  return parts.slice(1).some((part) => rustTestAttributeCanAffectExecution(tokens, pairs, ...part));
}

/** 🧪️ Resolves direct and test-enabled cfg_attr attributes without assuming unknown cfg values. */
function rustTestAttributes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, attributes: RustAttributes): RustTestAttributes {
  let configuration: RustTestConfiguration = "enabled";
  let test = false;
  let ignored = false;
  let pathTarget: string | null = null;
  const apply = (start: number, end: number, fromCfgAttr = false): void => {
    const name = tokens[start]?.text;
    if (name === "cfg") {
      if (tokens[start + 1]?.text !== "(") { configuration = rustTestConfigurationAnd(configuration, "ambiguous"); return; }
      const close = pairs.get(start + 1);
      configuration = rustTestConfigurationAnd(configuration, close === undefined || close + 1 !== end ? "ambiguous" : rustTestConfigurationExpression(tokens, pairs, start + 2, close));
      return;
    }
    if (name === "test") { test = true; return; }
    if (name === "ignore") { ignored = true; return; }
    if (name === "path") {
      const equals = tokens[start + 1]?.text === "=" ? start + 1 : -1;
      const target = equals < 0 ? null : rustStringValue(tokens[equals + 1]);
      if (target === null || pathTarget !== null) configuration = rustTestConfigurationAnd(configuration, "ambiguous");
      else pathTarget = target;
      return;
    }
    if (name !== "cfg_attr" || tokens[start + 1]?.text !== "(") {
      if (fromCfgAttr) configuration = rustTestConfigurationAnd(configuration, "ambiguous");
      return;
    }
    const close = pairs.get(start + 1);
    if (close === undefined || close + 1 !== end) { configuration = rustTestConfigurationAnd(configuration, "ambiguous"); return; }
    const parts = rustTokenSegments(tokens, pairs, start + 2, close, ",");
    if (parts.length < 2) { configuration = rustTestConfigurationAnd(configuration, "ambiguous"); return; }
    const condition = rustTestConfigurationExpression(tokens, pairs, ...parts[0]!);
    if (condition === "enabled") for (const part of parts.slice(1)) apply(...part, true);
    else if (condition === "ambiguous" && parts.slice(1).some((part) => rustTestAttributeCanAffectExecution(tokens, pairs, ...part))) configuration = rustTestConfigurationAnd(configuration, "ambiguous");
  };
  for (const range of attributes.ranges) apply(...range);
  return { configuration, test, ignored, pathTarget };
}

/** 🧪️ Preserves only an exact, leaf-local path spelling for an inline module's child mount base. */
function rustTestInlineMountBase(target: string | null, name: string): readonly string[] | null {
  if (target === null) return [name];
  if (!target || target.includes("\0") || target.includes("\\") || target.startsWith("/") || /^[A-Za-z]:/u.test(target)) return null;
  const segments = target.split("/");
  return segments.some((segment) => !segment || segment === "." || segment === "..") ? null : segments;
}

/** 🧪️ Extracts only top-level, enabled, non-ignored test functions and their explicit mounted modules. */
export function inspectRustRunnableTests(source: string): RustTestFacts {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const runnableTests: RustRunnableTestFact[] = [];
  const mountedModules: RustTestModuleFact[] = [];
  const skipItem = (start: number, end: number): number => {
    const boundary = rustFindTopLevel(tokens, pairs, start, end, new Set([";", "{"]));
    if (boundary < 0) return end;
    if (tokens[boundary]!.text === ";") return boundary + 1;
    return (pairs.get(boundary) ?? end - 1) + 1;
  };
  const parseScope = (start: number, end: number, modulePath: readonly string[], mountBase: readonly string[], inherited: RustTestConfiguration): void => {
    const inner = rustInnerAttributes(tokens, pairs, start, end);
    const scopeConfiguration = rustTestConfigurationAnd(inherited, rustTestAttributes(tokens, pairs, inner).configuration);
    for (let index = inner.next; index < end;) {
      const attributes = rustAttributes(tokens, pairs, index, end);
      const visibility = rustVisibility(tokens, pairs, attributes.next);
      const keyword = tokens[visibility.next]?.text;
      const itemAttributes = rustTestAttributes(tokens, pairs, attributes);
      const configuration = rustTestConfigurationAnd(scopeConfiguration, itemAttributes.configuration);
      if (keyword === "mod") {
        const name = tokens[visibility.next + 1];
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 2, end, new Set([";", "{"]));
        if (!name || name.kind !== "identifier" || boundary < 0) return;
        const childPath = [...modulePath, name.text];
        const inline = tokens[boundary]!.text === "{";
        if (!inline) mountedModules.push({ name: name.text, modulePath: childPath, mountBase, pathTarget: itemAttributes.pathTarget, configuration });
        if (!inline) { index = boundary + 1; continue; }
        const close = pairs.get(boundary);
        if (close === undefined) return;
        const inlineBase = rustTestInlineMountBase(itemAttributes.pathTarget, name.text);
        parseScope(boundary + 1, close, childPath, inlineBase === null ? mountBase : [...mountBase, ...inlineBase], inlineBase === null ? rustTestConfigurationAnd(configuration, "ambiguous") : configuration);
        index = close + 1;
        continue;
      }
      if (keyword === "fn") {
        const name = tokens[visibility.next + 1];
        if (name?.kind === "identifier" && configuration === "enabled" && itemAttributes.test && !itemAttributes.ignored) runnableTests.push({ name: name.text, modulePath });
      }
      index = skipItem(attributes.next, end);
    }
  };
  parseScope(0, tokens.length, [], [], "enabled");
  return { schemaVersion: 1, runnableTests, mountedModules };
}
//#endregion 🧪️RustRunnableTests

/** ⏩️ Finds a top-level token while skipping every paired nested group. */
function rustFindTopLevel(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number, wanted: ReadonlySet<string>): number {
  for (let index = start; index < end; index += 1) {
    const pair = pairs.get(index);
    if (pair !== undefined && pair > index) {
      if (wanted.has(tokens[index]!.text)) return index;
      index = pair;
      continue;
    }
    if (wanted.has(tokens[index]!.text)) return index;
  }
  return -1;
}

/** 🔤️ Returns the first path-shaped constructor in one tokenized match pattern. */
function rustPatternVariantPath(tokens: readonly RustToken[], start: number, end: number): string | null {
  for (let index = start; index < end; index += 1) {
    if (tokens[index]?.kind !== "identifier") continue;
    const parts = [tokens[index]!.text];
    let cursor = index + 1;
    while (cursor + 1 < end && tokens[cursor]!.text === "::" && tokens[cursor + 1]!.kind === "identifier") {
      parts.push(tokens[cursor + 1]!.text);
      cursor += 2;
    }
    if (parts.length > 1) return parts.join("::");
  }
  return null;
}

/** 🏷️ Derives stable string identity fields from one struct-literal const expression. */
function rustConstIdentityFields(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): Readonly<Record<string, string>> {
  const open = rustFindTopLevel(tokens, pairs, start, end, new Set(["{"]));
  const close = open < 0 ? undefined : pairs.get(open);
  if (open < 0 || close === undefined || close > end) return {};
  const fields: Record<string, string> = {};
  for (const [fieldStart, fieldEnd] of rustTokenSegments(tokens, pairs, open + 1, close, ",")) {
    const colon = rustFindTopLevel(tokens, pairs, fieldStart, fieldEnd, new Set([":"]));
    const name = tokens[fieldStart];
    const value = colon < 0 ? null : rustStringValue(tokens[colon + 1]);
    if (name?.kind === "identifier" && value !== null) fields[name.text] = value;
  }
  return Object.fromEntries(Object.entries(fields).sort(([left], [right]) => left.localeCompare(right)));
}

/** 🌳️ Parses Rust items from tokens and exposes only stable structural facts. */
class RustStructureParser {
  readonly modules: RustModuleFact[] = [];
  readonly enums: RustEnumFact[] = [];
  readonly impls: RustImplFact[] = [];
  readonly inlinePayloads: RustInlinePayloadFact[] = [];
  readonly constants: RustConstIdentityFact[] = [];

  constructor(readonly tokens: readonly RustToken[], readonly pairs: ReadonlyMap<number, number>) {}

  parse(): void {
    this.parseScope(0, this.tokens.length);
  }

  private parseScope(start: number, end: number, inheritedConditional = false): void {
    let index = start, scopeConditional = inheritedConditional;
    while (index + 2 < end && this.tokens[index]?.text === "#" && this.tokens[index + 1]?.text === "!" && this.tokens[index + 2]?.text === "[") {
      const close = this.pairs.get(index + 2);
      if (close === undefined || close >= end) return;
      scopeConditional ||= rustMetadataAttributes(this.tokens, this.pairs, { ranges: [[index + 3, close]], next: close + 1 }).conditional;
      index = close + 1;
    }
    while (index < end) {
      const attributes = rustAttributes(this.tokens, this.pairs, index, end);
      const visibility = rustVisibility(this.tokens, this.pairs, attributes.next);
      const keyword = this.tokens[visibility.next]?.text;
      const conditional = scopeConditional || rustMetadataAttributes(this.tokens, this.pairs, attributes).conditional;
      if (keyword === "mod") index = this.parseModule(attributes, visibility, end, conditional);
      else if (keyword === "enum") index = this.parseEnum(attributes, visibility, end, conditional);
      else if (keyword === "struct") index = this.parseStruct(visibility, end);
      else if (keyword === "impl") index = this.parseImpl(visibility.next, end);
      else if (keyword === "const" && this.tokens[visibility.next + 1]?.text !== "fn") index = this.parseConst(visibility.next, end, null);
      else index = this.skipItem(attributes.next, end);
    }
  }

  private skipItem(start: number, end: number): number {
    const boundary = rustFindTopLevel(this.tokens, this.pairs, start, end, new Set([";", "{"]));
    if (boundary < 0) return end;
    if (this.tokens[boundary]!.text === ";") return boundary + 1;
    const close = this.pairs.get(boundary);
    return close === undefined ? end : close + 1;
  }

  private parseModule(attributes: RustAttributes, visibility: RustVisibility, end: number, conditional: boolean): number {
    const keyword = visibility.next;
    const name = this.tokens[keyword + 1]?.text ?? "";
    const boundary = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, end, new Set([";", "{"]));
    const inline = boundary >= 0 && this.tokens[boundary]!.text === "{";
    const fact: RustModuleFact = { name, visibility: visibility.value, inline, pathTarget: rustPathAttribute(this.tokens, attributes), cfgTest: rustCfgTest(this.tokens, attributes) };
    this.modules.push(fact);
    if (!inline) return boundary < 0 ? end : boundary + 1;
    const close = this.pairs.get(boundary);
    if (close === undefined) return end;
    this.parseScope(boundary + 1, close, conditional);
    return close + 1;
  }

  private parseEnum(attributes: RustAttributes, visibility: RustVisibility, end: number, conditional: boolean): number {
    const keyword = visibility.next;
    const name = this.tokens[keyword + 1]?.text ?? "";
    const open = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, end, new Set(["{"]));
    const close = open < 0 ? undefined : this.pairs.get(open);
    if (open < 0 || close === undefined) return end;
    const variants: RustEnumVariantFact[] = [];
    for (const [rawStart, rawEnd] of rustTokenSegments(this.tokens, this.pairs, open + 1, close, ",")) {
      const attributes = rustAttributes(this.tokens, this.pairs, rawStart, rawEnd);
      const variantStart = attributes.next;
      const variant = this.tokens[variantStart];
      if (variant?.kind !== "identifier") continue;
      const shape = this.tokens[variantStart + 1]?.text;
      const variantConditional = conditional || rustMetadataAttributes(this.tokens, this.pairs, attributes).conditional;
      if (shape === "(") {
        const fieldClose = this.pairs.get(variantStart + 1) ?? variantStart + 1;
        const fieldTypes = rustTokenSegments(this.tokens, this.pairs, variantStart + 2, fieldClose, ",").map(([fieldStart, fieldEnd]) => rustTokenText(this.tokens, fieldStart, fieldEnd));
        variants.push({ name: variant.text, fieldStyle: "tuple", fieldTypes, wrappedTupleLeafType: fieldTypes.length === 1 ? fieldTypes[0]! : null, ...(variantConditional ? { conditional: true as const } : {}) });
      } else if (shape === "{") {
        const fieldClose = this.pairs.get(variantStart + 1) ?? variantStart + 1;
        const fieldTypes = rustTokenSegments(this.tokens, this.pairs, variantStart + 2, fieldClose, ",").map(([fieldStart, fieldEnd]) => {
          const fieldAttributes = rustAttributes(this.tokens, this.pairs, fieldStart, fieldEnd);
          const fieldVisibility = rustVisibility(this.tokens, this.pairs, fieldAttributes.next);
          const colon = rustFindTopLevel(this.tokens, this.pairs, fieldVisibility.next, fieldEnd, new Set([":"]));
          return colon < 0 ? "" : rustTokenText(this.tokens, colon + 1, fieldEnd);
        }).filter(Boolean);
        variants.push({ name: variant.text, fieldStyle: "struct", fieldTypes, wrappedTupleLeafType: null, ...(variantConditional ? { conditional: true as const } : {}) });
      } else variants.push({ name: variant.text, fieldStyle: "unit", fieldTypes: [], wrappedTupleLeafType: null, ...(variantConditional ? { conditional: true as const } : {}) });
    }
    this.enums.push({ name, visibility: visibility.value, variants, ...(conditional ? { conditional: true as const } : {}) });
    return close + 1;
  }

  private parseStruct(visibility: RustVisibility, end: number): number {
    const keyword = visibility.next;
    const name = this.tokens[keyword + 1]?.text ?? "";
    const boundary = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, end, new Set([";", "{", "("]));
    if (boundary < 0) return end;
    const shape = this.tokens[boundary]!.text;
    if (shape === ";") {
      this.inlinePayloads.push({ name, visibility: visibility.value, fieldStyle: "unit", fields: [] });
      return boundary + 1;
    }
    const close = this.pairs.get(boundary);
    if (close === undefined) return end;
    const fields: RustPayloadFieldFact[] = [];
    for (const [rawStart, rawEnd] of rustTokenSegments(this.tokens, this.pairs, boundary + 1, close, ",")) {
      const attributes = rustAttributes(this.tokens, this.pairs, rawStart, rawEnd);
      const fieldVisibility = rustVisibility(this.tokens, this.pairs, attributes.next);
      const colon = rustFindTopLevel(this.tokens, this.pairs, fieldVisibility.next, rawEnd, new Set([":"]));
      if (shape === "{") {
        const fieldName = this.tokens[fieldVisibility.next];
        if (fieldName?.kind === "identifier" && colon >= 0) fields.push({ name: fieldName.text, type: rustTokenText(this.tokens, colon + 1, rawEnd) });
      } else fields.push({ name: null, type: rustTokenText(this.tokens, fieldVisibility.next, rawEnd) });
    }
    this.inlinePayloads.push({ name, visibility: visibility.value, fieldStyle: shape === "{" ? "struct" : "tuple", fields });
    return close + 1;
  }

  private parseImpl(keyword: number, end: number): number {
    const open = rustFindTopLevel(this.tokens, this.pairs, keyword + 1, end, new Set(["{"]));
    const close = open < 0 ? undefined : this.pairs.get(open);
    if (open < 0 || close === undefined) return end;
    let angleDepth = 0;
    let forIndex = -1;
    for (let index = keyword + 1; index < open; index += 1) {
      const text = this.tokens[index]!.text;
      if (text === "<") angleDepth += 1;
      else if (text === ">") angleDepth = Math.max(0, angleDepth - 1);
      else if (text === ">>") angleDepth = Math.max(0, angleDepth - 2);
      else if (text === "for" && angleDepth === 0) forIndex = index;
    }
    const headerStart = keyword + 1;
    const traitPath = forIndex < 0 ? null : rustTokenText(this.tokens, headerStart, forIndex).replace(/^!/, "");
    const selfType = rustTokenText(this.tokens, forIndex < 0 ? headerStart : forIndex + 1, open);
    const methods: string[] = [];
    const associatedConstants: string[] = [];
    let index = open + 1;
    while (index < close) {
      const attributes = rustAttributes(this.tokens, this.pairs, index, close);
      const visibility = rustVisibility(this.tokens, this.pairs, attributes.next);
      const token = this.tokens[visibility.next]?.text;
      if (token === "fn" || (token === "async" && this.tokens[visibility.next + 1]?.text === "fn")) {
        const nameIndex = visibility.next + (token === "async" ? 2 : 1);
        if (this.tokens[nameIndex]?.kind === "identifier") methods.push(this.tokens[nameIndex]!.text);
        index = this.skipItem(attributes.next, close);
      } else if (token === "const") {
        const name = this.tokens[visibility.next + 1]?.text;
        if (name) associatedConstants.push(name);
        index = this.parseConst(visibility.next, close, selfType);
      } else index = this.skipItem(attributes.next, close);
    }
    this.impls.push({ traitPath, selfType, methods, associatedConstants });
    return close + 1;
  }

  private parseConst(keyword: number, end: number, owner: string | null): number {
    const name = this.tokens[keyword + 1]?.text ?? "";
    const semicolon = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, end, new Set([";"]));
    if (semicolon < 0) return end;
    const colon = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, semicolon, new Set([":"]));
    const equals = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, semicolon, new Set(["="]));
    const valueStart = equals < 0 ? semicolon : equals + 1;
    const value = rustTokenText(this.tokens, valueStart, semicolon);
    this.constants.push({
      owner,
      name,
      type: colon < 0 || equals < 0 ? null : rustTokenText(this.tokens, colon + 1, equals),
      value,
      stringValue: valueStart + 1 === semicolon ? rustStringValue(this.tokens[valueStart]) : null,
      identityFields: rustConstIdentityFields(this.tokens, this.pairs, valueStart, semicolon),
    });
    return semicolon + 1;
  }
}

/** 🎯️ Finds every token-aware match arm, including nested function bodies, without scanning literals. */
function rustMatchArms(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): RustMatchArmFact[] {
  const facts: RustMatchArmFact[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]!.kind !== "identifier" || tokens[index]!.text !== "match") continue;
    const open = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set(["{"]));
    const close = open < 0 ? undefined : pairs.get(open);
    if (open < 0 || close === undefined) continue;
    for (const [armStart, armEnd] of rustTokenSegments(tokens, pairs, open + 1, close, ",")) {
      const arrow = rustFindTopLevel(tokens, pairs, armStart, armEnd, new Set(["=>"]));
      if (arrow < 0) continue;
      facts.push({ pattern: rustTokenText(tokens, armStart, arrow), variantPath: rustPatternVariantPath(tokens, armStart, arrow), expression: rustTokenText(tokens, arrow + 1, armEnd) });
    }
  }
  return facts;
}

/** 📦️ Finds include macros and explicitly records whether their token tree reaches `OUT_DIR`. */
function rustIncludes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): RustIncludeFact[] {
  const facts: RustIncludeFact[] = [];
  const names = new Set(["include", "include_str", "include_bytes"] as const);
  for (let index = 0; index + 2 < tokens.length; index += 1) {
    const name = tokens[index]!.text as RustIncludeFact["macro"];
    if (!names.has(name) || tokens[index + 1]!.text !== "!" || tokens[index + 2]!.text !== "(") continue;
    const close = pairs.get(index + 2);
    if (close === undefined) continue;
    const expressionTokens = tokens.slice(index + 3, close);
    facts.push({
      macro: name,
      expression: rustTokenText(tokens, index + 3, close),
      usesOutDir: expressionTokens.some((token) => (token.kind === "identifier" && token.text === "OUT_DIR") || rustStringValue(token) === "OUT_DIR"),
    });
  }
  return facts;
}

/** 🧠️ Extracts a stable schema-versioned structural report from one Rust source string. */
export function inspectRustStructure(source: string): RustStructuralFacts {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const parser = new RustStructureParser(tokens, pairs);
  parser.parse();
  const modules = parser.modules;
  return {
    schemaVersion: 1,
    modules,
    enums: parser.enums,
    impls: parser.impls,
    inlinePayloads: parser.inlinePayloads,
    matchArms: rustMatchArms(tokens, pairs),
    constants: parser.constants,
    includes: rustIncludes(tokens, pairs),
    testModules: modules.filter((module) => module.cfgTest),
  };
}

//#region 🧬️MutationMetadataFacts
export interface RustMutationMetadataDeclarationFact { readonly name: string; readonly kind: "struct" | "enum" | "union"; readonly visibility: RustStructuralVisibility; readonly modulePath: readonly string[]; readonly derives: readonly string[]; readonly conditional?: true; readonly mutationLeaf: { readonly state: "absent" | "valid" | "malformed" | "ambiguous" | "conditional"; readonly contracts: readonly string[]; readonly detail: string | null }; }
export interface RustCrateAliasFact { readonly kind: "extern" | "self" | "use" | "reexport"; readonly source: string; readonly alias: string; readonly modulePath: readonly string[]; readonly conditional: boolean; readonly restricted?: true; }
export interface RustMutationMetadataFacts { readonly schemaVersion: 1; readonly declarations: readonly RustMutationMetadataDeclarationFact[]; readonly crateAliases: readonly RustCrateAliasFact[]; readonly manualMutationLeafImpls: readonly RustImplFact[]; }

interface RustMetadataAttributeFact { readonly name: string; readonly start: number; readonly end: number; readonly conditional: boolean; }

/** 🧭️ Parses one exact nongeneric Rust path from already-tokenized source. */
function rustMetadataPath(tokens: readonly RustToken[], start: number, end: number, absolute: boolean): string | null {
  let index = start, prefix = "";
  if (tokens[index]?.text === "::") { prefix = "::"; index += 1; }
  else if (absolute) return null;
  const parts: string[] = [];
  while (tokens[index]?.kind === "identifier") {
    parts.push(tokens[index]!.text);
    index += 1;
    if (tokens[index]?.text !== "::") break;
    index += 1;
    if (tokens[index]?.kind !== "identifier") return null;
  }
  return parts.length > 0 && index === end ? `${prefix}${parts.join("::")}` : null;
}

/** 🧭️ Reads one attribute name and its optional parenthesized argument range. */
function rustMetadataAttributeHead(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): { readonly name: string; readonly arguments: readonly [number, number] | null } | null {
  const name = rustMetadataPath(tokens, start, end, false);
  if (name) return { name, arguments: null };
  let index = start;
  const parts: string[] = [];
  while (tokens[index]?.kind === "identifier") {
    parts.push(tokens[index]!.text);
    index += 1;
    if (tokens[index]?.text !== "::") break;
    index += 1;
  }
  if (parts.length === 0 || tokens[index]?.text !== "(") return null;
  const close = pairs.get(index);
  if (close === undefined || close !== end - 1) return null;
  return { name: parts.join("::"), arguments: [index + 1, close] };
}

/** 🧭️ Flattens direct and `cfg_attr` metadata while retaining conditional evidence. */
function rustMetadataAttributes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, attributes: RustAttributes): { readonly items: readonly RustMetadataAttributeFact[]; readonly conditional: boolean } {
  const items: RustMetadataAttributeFact[] = [];
  let conditional = false;
  const add = (start: number, end: number, inheritedConditional: boolean): void => {
    const head = rustMetadataAttributeHead(tokens, pairs, start, end);
    if (!head) return;
    const name = head.name;
    if (name === "cfg") { conditional = true; return; }
    if (name === "cfg_attr") {
      conditional = true;
      if (!head.arguments) return;
      const segments = rustTokenSegments(tokens, pairs, head.arguments[0], head.arguments[1], ",");
      for (const [attributeStart, attributeEnd] of segments.slice(1)) add(attributeStart, attributeEnd, true);
      return;
    }
    items.push({ name, start, end, conditional: inheritedConditional });
  };
  for (const [start, end] of attributes.ranges) add(start, end, false);
  return { items, conditional };
}

/** 🧭️ Extracts exact derive paths only from a complete `derive(...)` metadata item. */
function rustMetadataDerives(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, attributes: readonly RustMetadataAttributeFact[]): readonly string[] {
  const derives: string[] = [];
  for (const attribute of attributes.filter((item) => item.name === "derive")) {
    const head = rustMetadataAttributeHead(tokens, pairs, attribute.start, attribute.end);
    if (!head?.arguments) continue;
    const paths = rustTokenSegments(tokens, pairs, head.arguments[0], head.arguments[1], ",").map(([start, end]) => rustMetadataPath(tokens, start, end, false));
    if (paths.length > 0 && paths.every((path): path is string => path !== null)) derives.push(...paths);
  }
  return derives;
}

/** 🧭️ Produces fail-closed `mutation_leaf` evidence from exact attribute tokens. */
function rustMutationLeafEvidence(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, attributes: readonly RustMetadataAttributeFact[], inheritedConditional: boolean): RustMutationMetadataDeclarationFact["mutationLeaf"] {
  const metadata = attributes.filter((item) => item.name === "mutation_leaf");
  if (metadata.length === 0) return { state: "absent", contracts: [], detail: null };
  const contracts: string[] = [];
  let malformed = false;
  for (const attribute of metadata) {
    const head = rustMetadataAttributeHead(tokens, pairs, attribute.start, attribute.end);
    if (!head?.arguments) { malformed = true; continue; }
    const entries = rustTokenSegments(tokens, pairs, head.arguments[0], head.arguments[1], ",");
    let contractCount = 0;
    for (const [start, end] of entries) {
      const equals = rustFindTopLevel(tokens, pairs, start, end, new Set(["="]));
      if (tokens[start]?.text !== "contract" || equals !== start + 1) { malformed = true; continue; }
      contractCount += 1;
      const contract = rustMetadataPath(tokens, equals + 1, end, true);
      if (contract === null || contract.split("::").some((segment) => ["crate", "self", "super"].includes(segment))) malformed = true;
      else contracts.push(contract);
    }
    if (contractCount !== 1 || entries.length !== 1) malformed = true;
  }
  const conditional = inheritedConditional || metadata.some((item) => item.conditional);
  if (metadata.length > 1 || contracts.length > 1) return { state: "ambiguous", contracts, detail: "duplicate or conflicting mutation_leaf attributes" };
  if (conditional) return { state: "conditional", contracts, detail: "conditional mutation_leaf metadata" };
  if (malformed || contracts.length !== 1) return { state: "malformed", contracts, detail: "invalid mutation_leaf syntax" };
  return { state: "valid", contracts, detail: null };
}

/** 🧭️ Expands one tokenized Rust use tree into exact imported aliases. */
function rustUseAliases(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): readonly { readonly source: string; readonly alias: string }[] {
  const aliases: { source: string; alias: string }[] = [];
  const parse = (segmentStart: number, segmentEnd: number, prefix: readonly string[], absolute: boolean): void => {
    let index = segmentStart;
    if (tokens[index]?.text === "{") {
      const close = pairs.get(index);
      if (close === undefined || close !== segmentEnd - 1) return;
      for (const [childStart, childEnd] of rustTokenSegments(tokens, pairs, index + 1, close, ",")) parse(childStart, childEnd, prefix, absolute);
      return;
    }
    if (tokens[index]?.text === "::") { if (prefix.length > 0) return; absolute = true; index += 1; }
    const parts: string[] = [];
    while (tokens[index]?.kind === "identifier") {
      parts.push(tokens[index]!.text);
      index += 1;
      if (tokens[index]?.text !== "::") break;
      index += 1;
      if (tokens[index]?.text === "{") {
        const close = pairs.get(index);
        if (close === undefined || close !== segmentEnd - 1) return;
        for (const [childStart, childEnd] of rustTokenSegments(tokens, pairs, index + 1, close, ",")) parse(childStart, childEnd, [...prefix, ...parts], absolute);
        return;
      }
      if (tokens[index]?.text === "*" && index + 1 === segmentEnd && parts.length > 0) {
        const source = `${absolute ? "::" : ""}${[...prefix, ...parts].join("::")}`;
        aliases.push({ source, alias: "*" });
        return;
      }
      if (tokens[index]?.kind !== "identifier") return;
    }
    if (parts.length === 0) return;
    let alias: string;
    if (tokens[index]?.text === "as" && tokens[index + 1]?.kind === "identifier" && index + 2 === segmentEnd) alias = tokens[index + 1]!.text;
    else if (index === segmentEnd) alias = parts.at(-1) === "self" ? prefix.at(-1) ?? "self" : parts.at(-1)!;
    else return;
    const sourceParts = parts.at(-1) === "self" ? prefix : [...prefix, ...parts];
    const source = `${absolute ? "::" : ""}${sourceParts.join("::")}`;
    if (sourceParts.length > 0 && alias !== "_" && !["::crate", "::self", "::super"].some((root) => source === root || source.startsWith(`${root}::`))) aliases.push({ source, alias });
  };
  parse(start, end, [], false);
  return aliases;
}

/** 🧬️ Returns token-derived declaration and alias evidence; it deliberately does not resolve crates. */
export function inspectRustMutationMetadataFacts(source: string): RustMutationMetadataFacts {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), declarations: RustMutationMetadataDeclarationFact[] = [], crateAliases: RustCrateAliasFact[] = [];
  const scope = (start: number, end: number, modulePath: readonly string[], inheritedConditional: boolean): void => {
    let index = start, scopeConditional = inheritedConditional;
    while (index + 2 < end && tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[") {
      const close = pairs.get(index + 2);
      if (close === undefined || close >= end) return;
      scopeConditional ||= rustMetadataAttributes(tokens, pairs, { ranges: [[index + 3, close]], next: close + 1 }).conditional;
      index = close + 1;
    }
    for (; index < end;) {
      const rawAttributes = rustAttributes(tokens, pairs, index, end), metadataAttributes = rustMetadataAttributes(tokens, pairs, rawAttributes), visibility = rustVisibility(tokens, pairs, rawAttributes.next), keyword = tokens[visibility.next]?.text, conditional = scopeConditional || metadataAttributes.conditional;
      if (keyword === "mod" && tokens[visibility.next + 1]?.kind === "identifier") {
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 2, end, new Set([";", "{"]));
        if (boundary < 0) return;
        if (tokens[boundary]?.text === "{") {
          const close = pairs.get(boundary);
          if (close !== undefined) scope(boundary + 1, close, [...modulePath, tokens[visibility.next + 1]!.text], conditional);
          index = (close ?? boundary) + 1;
          continue;
        }
        index = boundary + 1;
        continue;
      }
      if (keyword === "extern" && tokens[visibility.next + 1]?.text === "crate") {
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 2, end, new Set([";"]));
        const sourceToken = tokens[visibility.next + 2], aliasToken = tokens[visibility.next + 3]?.text === "as" ? tokens[visibility.next + 4] : sourceToken;
        if (boundary >= 0 && sourceToken?.kind === "identifier" && aliasToken?.kind === "identifier" && (aliasToken === sourceToken ? visibility.next + 3 === boundary : visibility.next + 5 === boundary)) crateAliases.push({ kind: sourceToken.text === "self" ? "self" : "extern", source: sourceToken.text, alias: aliasToken.text, modulePath, conditional, ...(visibility.value === "pub" ? {} : { restricted: true as const }) });
        index = boundary < 0 ? end : boundary + 1;
        continue;
      }
      if (keyword === "use") {
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 1, end, new Set([";"]));
        if (boundary < 0) return;
        for (const { source: aliasSource, alias } of rustUseAliases(tokens, pairs, visibility.next + 1, boundary)) crateAliases.push({ kind: visibility.value === "private" ? "use" : "reexport", source: aliasSource, alias, modulePath, conditional, ...(visibility.value === "pub" ? {} : { restricted: true as const }) });
        index = boundary + 1;
        continue;
      }
      if (["struct", "enum", "union"].includes(keyword ?? "") && tokens[visibility.next + 1]?.kind === "identifier") declarations.push({ name: tokens[visibility.next + 1]!.text, kind: keyword as "struct" | "enum" | "union", visibility: visibility.value, modulePath, derives: rustMetadataDerives(tokens, pairs, metadataAttributes.items), ...(conditional ? { conditional: true as const } : {}), mutationLeaf: rustMutationLeafEvidence(tokens, pairs, metadataAttributes.items, conditional) });
      const boundary = rustFindTopLevel(tokens, pairs, rawAttributes.next, end, new Set([";", "{"]));
      index = boundary < 0 ? end : tokens[boundary]?.text === "{" ? (pairs.get(boundary) ?? boundary) + 1 : boundary + 1;
    }
  };
  scope(0, tokens.length, [], false);
  const structural = inspectRustStructure(source);
  return { schemaVersion: 1, declarations, crateAliases, manualMutationLeafImpls: structural.impls.filter((item) => item.traitPath?.split("::").at(-1) === "MutationLeaf") };
}
//#endregion 🧬️MutationMetadataFacts

/** 📦️ Cargo entry-point facts shared by module consumers and physical path authority. */
export interface RustCargoManifestFacts {
  readonly crateName: string | null;
  readonly libPath: string | null;
  readonly dependencies: readonly string[];
  readonly valid: boolean;
}

/** 📋️ Reads the exact package/lib tables, rejecting ambiguous ownership declarations. */
export function inspectRustCargoManifest(source: string, strict = false): RustCargoManifestFacts {
  if (strict) {
    try {
      const parsed = cargoProviderTomlParser.parse(source) as { package?: { name?: unknown }; lib?: { name?: unknown; path?: unknown }; dependencies?: Record<string, unknown> };
      const packageName = typeof parsed.package?.name === "string" && /^[A-Za-z0-9_-]+$/u.test(parsed.package.name) ? parsed.package.name : null;
      const libName = parsed.lib?.name === undefined ? null : typeof parsed.lib.name === "string" && /^[A-Za-z0-9_-]+$/u.test(parsed.lib.name) ? parsed.lib.name : undefined;
      const libPath = parsed.lib?.path === undefined ? null : typeof parsed.lib.path === "string" ? parsed.lib.path : undefined;
      const valid = packageName !== null && libName !== undefined && libPath !== undefined && (libPath === null || !posix.isAbsolute(libPath) && !/^[A-Za-z]:/u.test(libPath) && !libPath.includes("\\"));
      return { crateName: libName ?? packageName, libPath: libPath ?? null, dependencies: Object.keys(parsed.dependencies ?? {}).map((name) => name.replaceAll("-", "_")).sort(), valid };
    } catch { return { crateName: null, libPath: null, dependencies: [], valid: false }; }
  }
  let valid = true;
  const section = (name: string): string | null => {
    const matches = [...source.matchAll(new RegExp(`^\\s*\\[${name}\\]\\s*$`, "gmu"))];
    if (matches.length > 1) valid = false;
    const start = matches[0];
    if (!start || start.index === undefined) return null;
    const remainder = source.slice(start.index + start[0].length), end = /^\s*\[[^\]]+\]\s*$/mu.exec(remainder);
    return end?.index === undefined ? remainder : remainder.slice(0, end.index);
  };
  const value = (body: string | null, key: string, pattern: string): string | null => {
    if (body === null) return null;
    const candidates = [...body.matchAll(new RegExp(`^\\s*${key}\\s*=`, "gmu"))], values = [...body.matchAll(new RegExp(`^\\s*${key}\\s*=\\s*"(${pattern})"\\s*$`, "gmu"))];
    if (candidates.length > 1 || candidates.length !== values.length) valid = false;
    return values[0]?.[1] ?? null;
  };
  const packageSection = section("package"), libSection = section("lib"), dependencySection = section("dependencies");
  const packageName = value(packageSection, "name", "[A-Za-z0-9_-]+"), libName = value(libSection, "name", "[A-Za-z0-9_-]+"), libPath = value(libSection, "path", "[^\"\\\\]+"), crateName = libName ?? packageName;
  if (!packageName || libPath !== null && (posix.isAbsolute(libPath) || /^[A-Za-z]:/u.test(libPath))) valid = false;
  const dependencies = [...(dependencySection ?? "").matchAll(/^\s*([A-Za-z0-9_-]+)\s*=/gmu)].map((match) => match[1]!.replaceAll("-", "_")).sort();
  return { crateName, libPath, dependencies, valid };
}

/** 🧬️ One proven lexical module context and its Cargo ownership, if declared. */
export interface RustModuleContext {
  readonly crateRoot: string;
  readonly manifestPath: string | null;
  readonly modulePath: readonly string[];
  readonly sourceScope: readonly string[];
  readonly moduleBase: string;
  readonly sourceChain: readonly string[];
}

/** 🕸️ Mounted Rust sources and dependency names, with explicit manifest provenance. */
export interface RustModuleGraph {
  readonly targets: ReadonlyMap<string, string>;
  readonly contexts: ReadonlyMap<string, readonly RustModuleContext[]>;
  readonly namedCrates: ReadonlyMap<string, readonly string[]>;
  readonly dependencies: ReadonlyMap<string, readonly string[]>;
  readonly invalidManifests: ReadonlySet<string>;
}

/** 🦀️ Builds only file-membership-proven module edges; conventional roots never confer manifest authority. */
export function inspectRustModuleGraph(files: readonly string[], readSource: (path: string) => string | undefined, options: Readonly<{ conventionalRoots?: boolean; strictManifests?: boolean; checkCancellation?: () => void }> = {}): RustModuleGraph {
  const compare = (left: string, right: string): number => Buffer.from(left).compare(Buffer.from(right));
  const sourceFiles = new Set(files.filter((path) => path.endsWith(".rs"))), factsBySource = new Map<string, ReturnType<typeof inspectRustModuleGraphFacts>>();
  const targets = new Map<string, string>(), contexts = new Map<string, RustModuleContext[]>(), namedCrates = new Map<string, string[]>(), dependencies = new Map<string, readonly string[]>(), invalidManifests = new Set<string>();
  const manifestRoots = files.filter((path) => path === "Cargo.toml" || path.endsWith("/Cargo.toml")).sort(compare).flatMap((manifest) => {
    options.checkCancellation?.();
    const facts = inspectRustCargoManifest(readSource(manifest) ?? "", options.strictManifests === true);
    if (!facts.valid) invalidManifests.add(manifest);
    if (options.strictManifests && !facts.valid) return [];
    const entry = posix.normalize(posix.join(posix.dirname(manifest), facts.libPath ?? "src/lib.rs"));
    return sourceFiles.has(entry) ? [{ path: entry, manifestPath: manifest, crateName: facts.crateName, dependencies: facts.dependencies }] : [];
  });
  const conventionalRoots = options.conventionalRoots ? [...sourceFiles].filter((path) => /(?:^|\/)(?:lib|main)\.rs$/u.test(path) && !manifestRoots.some((root) => root.path === path)).map((path) => ({ path, manifestPath: null, crateName: null, dependencies: [] as string[] })) : [];
  const addContext = (path: string, context: RustModuleContext): boolean => {
    const existing = contexts.get(path) ?? [];
    if (existing.some((value) => value.crateRoot === context.crateRoot && value.manifestPath === context.manifestPath && value.modulePath.join("::") === context.modulePath.join("::") && value.sourceScope.join("::") === context.sourceScope.join("::") && value.moduleBase === context.moduleBase)) return false;
    contexts.set(path, [...existing, context]);
    return true;
  };
  for (const root of [...manifestRoots, ...conventionalRoots].sort((left, right) => compare(left.path, right.path))) {
    const crateRoot = root.path;
    if (root.crateName) namedCrates.set(root.crateName.replaceAll("-", "_"), [...(namedCrates.get(root.crateName.replaceAll("-", "_")) ?? []), crateRoot]);
    dependencies.set(crateRoot, root.dependencies);
    const pending: { readonly sourcePath: string; readonly context: RustModuleContext }[] = [{ sourcePath: crateRoot, context: { crateRoot, manifestPath: root.manifestPath, modulePath: [], sourceScope: [], moduleBase: posix.dirname(crateRoot), sourceChain: [crateRoot] } }];
    addContext(crateRoot, pending[0]!.context);
    for (let index = 0; index < pending.length; index++) {
      options.checkCancellation?.();
      const { sourcePath, context } = pending[index]!;
      if (!factsBySource.has(sourcePath)) factsBySource.set(sourcePath, inspectRustModuleGraphFacts(readSource(sourcePath) ?? ""));
      for (const module of factsBySource.get(sourcePath)!.modules.filter((fact) => fact.modulePath.length === context.sourceScope.length + 1 && fact.modulePath.slice(0, -1).join("::") === context.sourceScope.join("::"))) {
        if (options.strictManifests && module.pathTarget !== null && (posix.isAbsolute(module.pathTarget) || /^[A-Za-z]:/u.test(module.pathTarget) || module.pathTarget.includes("\\"))) { if (root.manifestPath) invalidManifests.add(root.manifestPath); continue; }
        const base = module.pathTarget !== null && context.sourceScope.length === 0 ? posix.dirname(sourcePath) : context.moduleBase;
        const candidates = module.inline ? [sourcePath] : module.pathTarget === null ? [posix.join(base, `${module.name}.rs`), posix.join(base, module.name, "mod.rs")] : [posix.normalize(posix.join(base, module.pathTarget))];
        const matching = candidates.filter((candidate) => !candidate.startsWith("../") && sourceFiles.has(candidate)), target = matching.length === 1 ? matching[0] : undefined;
        if (!target) continue;
        if (!module.inline && context.sourceChain.includes(target)) { if (root.manifestPath) invalidManifests.add(root.manifestPath); continue; }
        const moduleBase = module.inline ? posix.normalize(posix.join(context.moduleBase, module.pathTarget ?? module.name)) : module.pathTarget === null && target.endsWith(`/${module.name}.rs`) ? posix.join(posix.dirname(target), module.name) : posix.dirname(target);
        const child: RustModuleContext = { crateRoot, manifestPath: root.manifestPath, modulePath: [...context.modulePath, module.name], sourceScope: module.inline ? module.modulePath : [], moduleBase, sourceChain: module.inline ? context.sourceChain : [...context.sourceChain, target] };
        const key = `${crateRoot}\0${child.modulePath.join("::")}`, prior = targets.get(key);
        if (prior && prior !== target) continue;
        targets.set(key, target);
        if (addContext(target, child)) pending.push({ sourcePath: target, context: child });
      }
    }
  }
  for (const [name, roots] of namedCrates) namedCrates.set(name, roots.sort(compare));
  if (options.strictManifests) for (const [path, rows] of contexts) contexts.set(path, rows.filter((row) => row.manifestPath !== null && !invalidManifests.has(row.manifestPath)));
  return { targets, contexts, namedCrates, dependencies, invalidManifests };
}

/** 🕸️ Extracts mounted modules and use items with lexical Rust scope, excluding decoys. */
export function inspectRustModuleGraphFacts(source: string): { readonly modules: readonly RustModuleGraphFact[]; readonly uses: readonly RustModuleUseFact[] } {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const modules: RustModuleGraphFact[] = [];
  const uses: RustModuleUseFact[] = [];
  const skipItem = (start: number, end: number): number => {
    const boundary = rustFindTopLevel(tokens, pairs, start, end, new Set([";", "{"]));
    if (boundary < 0) return end;
    if (tokens[boundary]!.text === ";") return boundary + 1;
    return (pairs.get(boundary) ?? end - 1) + 1;
  };
  const parseScope = (start: number, end: number, modulePath: readonly string[], inheritedConditional = false): void => {
    let index = start, scopeConditional = inheritedConditional;
    while (index + 2 < end && tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[") {
      const close = pairs.get(index + 2);
      if (close === undefined || close >= end) return;
      scopeConditional ||= rustMetadataAttributes(tokens, pairs, { ranges: [[index + 3, close]], next: close + 1 }).conditional;
      index = close + 1;
    }
    for (; index < end;) {
      const attributes = rustAttributes(tokens, pairs, index, end), conditional = scopeConditional || rustMetadataAttributes(tokens, pairs, attributes).conditional;
      const visibility = rustVisibility(tokens, pairs, attributes.next);
      const keyword = tokens[visibility.next]?.text;
      if (keyword === "use") {
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 1, end, new Set([";"]));
        if (boundary < 0) return;
        uses.push({ modulePath, specifier: rustTokenText(tokens, visibility.next + 1, boundary), relation: visibility.value === "private" ? "import" : "reexport", visibility: visibility.value, ...(conditional ? { conditional: true as const } : {}) });
        index = boundary + 1;
        continue;
      }
      if (keyword !== "mod") {
        index = skipItem(attributes.next, end);
        continue;
      }
      const name = tokens[visibility.next + 1];
      const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 2, end, new Set([";", "{"]));
      if (!name || name.kind !== "identifier" || boundary < 0) return;
      const inline = tokens[boundary]!.text === "{";
      const childPath = [...modulePath, name.text];
      modules.push({ name: name.text, modulePath: childPath, visibility: visibility.value, inline, pathTarget: rustPathAttribute(tokens, attributes), ...(conditional ? { conditional: true as const } : {}) });
      if (!inline) {
        index = boundary + 1;
        continue;
      }
      const close = pairs.get(boundary);
      if (close === undefined) return;
      parseScope(boundary + 1, close, childPath, conditional);
      index = close + 1;
    }
  };
  parseScope(0, tokens.length, []);
  return { modules, uses };
}

/** 🪪️ Lists top-level public Rust type declarations without comment or string decoys. */
export function inspectRustPublicTypeNames(source: string): readonly string[] {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), names: string[] = [];
  for (let index = 0; index < tokens.length;) {
    const attributes = rustAttributes(tokens, pairs, index, tokens.length), visibility = rustVisibility(tokens, pairs, attributes.next), keyword = tokens[visibility.next]?.text;
    if (visibility.value === "pub" && (keyword === "struct" || keyword === "enum") && tokens[visibility.next + 1]?.kind === "identifier") names.push(tokens[visibility.next + 1]!.text);
    const boundary = rustFindTopLevel(tokens, pairs, attributes.next, tokens.length, new Set([";", "{"]));
    if (boundary < 0) break;
    if (tokens[boundary]!.text === ";") { index = boundary + 1; continue; }
    const close = pairs.get(boundary); if (close === undefined) break; index = close + 1;
  }
  return [...new Set(names)].sort();
}

/** 🧷️ Locates one unambiguous public mutation aggregate declaration for structured insertion. */
export function inspectRustMutationAggregateSpan(source: string): { declarationStart: number; bodyOpen: number; bodyClose: number; enumName: string } | null {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const matches: { declarationStart: number; bodyOpen: number; bodyClose: number; enumName: string }[] = [];
  let malformed = false;
  for (let index = 0; index < tokens.length;) {
    const declarationIndex = index;
    const attributes = rustAttributes(tokens, pairs, declarationIndex, tokens.length);
    const visibility = rustVisibility(tokens, pairs, attributes.next);
    if (visibility.value !== "pub" || tokens[visibility.next]?.text !== "enum") {
      const boundary = rustFindTopLevel(tokens, pairs, attributes.next, tokens.length, new Set([";", "{"]));
      if (boundary < 0) break;
      if (tokens[boundary]!.text === ";") { index = boundary + 1; continue; }
      const close = pairs.get(boundary);
      if (close === undefined) { malformed = true; break; }
      index = close + 1;
      continue;
    }
    const name = tokens[visibility.next + 1];
    if (!name || name.kind !== "identifier" || !name.text.endsWith("Mutation")) { index = visibility.next + 1; continue; }
    const body = rustFindTopLevel(tokens, pairs, visibility.next + 2, tokens.length, new Set(["{"]));
    const close = body < 0 ? undefined : pairs.get(body);
    if (close === undefined) { malformed = true; break; }
    const attributeStart = tokens[declarationIndex]!.start;
    const docs = /((?:(?:[ \t]*\/\/\/[^\n]*(?:\n|$))|(?:[ \t]*\/\*\*[\s\S]*?\*\/[ \t]*(?:\n|$)))+)$/u.exec(source.slice(0, attributeStart));
    matches.push({ declarationStart: docs?.index ?? attributeStart, bodyOpen: tokens[body]!.start, bodyClose: tokens[close]!.start, enumName: name.text });
    index = close + 1;
  }
  return !malformed && matches.length === 1 ? matches[0]! : null;
}

/** 📜️ Renders the stable Rust structural report as deterministic newline-terminated JSON. */
export function renderRustStructuralFactsJson(facts: RustStructuralFacts): string {
  return `${JSON.stringify(facts, null, 2)}\n`;
}

/** 🪪️ Lists exact Rust identifiers and string identities without accepting comment text or prefixes. */
export function inspectRustSourceIdentities(source: string): readonly string[] {
  return [...new Set(rustTokens(source).flatMap((token) => token.kind === "identifier" ? [token.text] : token.kind === "string" ? [rustStringValue(token) ?? ""] : []))].sort();
}

//#region 🛡️MutationInputs
/** 🏷️ Extracts type identities while excluding path prefixes and string/comment decoys. */
function rustTypeNames(source: string): string[] {
  const tokens = rustTokens(source);
  return [...new Set(tokens.filter((token, index) => token.kind === "identifier" && tokens[index + 1]?.text !== "::" && !["as", "dyn", "impl", "mut", "const"].includes(token.text)).map((token) => token.text))];
}

/** 🔗️ Records local type and renamed-import edges without evaluating or expanding source. */
function rustTypeAliasEdges(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): Map<string, string[]> {
  const aliases = new Map<string, string[]>();
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text === "type" && tokens[index + 1]?.kind === "identifier") {
      const end = rustFindTopLevel(tokens, pairs, index + 2, tokens.length, new Set([";"]));
      const equals = end < 0 ? -1 : rustFindTopLevel(tokens, pairs, index + 2, end, new Set(["="]));
      if (equals >= 0) aliases.set(tokens[index + 1]!.text, rustTypeNames(rustTokenText(tokens, equals + 1, end)));
    }
    if (tokens[index]?.text !== "use") continue;
    const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"]));
    for (let cursor = index + 1; cursor < end; cursor++) {
      if (tokens[cursor]?.text === "as" && tokens[cursor - 1]?.kind === "identifier" && tokens[cursor + 1]?.kind === "identifier") aliases.set(tokens[cursor + 1]!.text, [tokens[cursor - 1]!.text]);
    }
  }
  return aliases;
}

/** 🚧️ Indexes an aggregate once and inspects each leaf for reachable aggregate-state inputs. */
export function createRustMutationInputInspector(aggregateSource: string): (leafSource: string) => readonly string[] {
  const aggregateTokens = rustTokens(aggregateSource);
  const aggregatePairs = rustTokenPairs(aggregateTokens);
  const aggregateAliases = rustTypeAliasEdges(aggregateTokens, aggregatePairs);
  const aggregateTypes = new Set<string>();
  const typeQueue: string[] = [];
  for (let index = 0; index + 3 < aggregateTokens.length; index++) {
    if (aggregateTokens[index]?.text !== "#" || aggregateTokens[index + 1]?.text !== "[" || aggregateTokens[index + 2]?.text !== "mutations" || aggregateTokens[index + 3]?.text !== "(") continue;
    const close = aggregatePairs.get(index + 3);
    if (close === undefined) continue;
    for (const [start, end] of rustTokenSegments(aggregateTokens, aggregatePairs, index + 4, close, ",")) {
      if (["snapshot", "diff"].includes(aggregateTokens[start]?.text ?? "") && aggregateTokens[start + 1]?.text === "=") typeQueue.push(...rustTypeNames(rustTokenText(aggregateTokens, start + 2, end)));
    }
  }
  for (let index = 0; index < typeQueue.length; index++) {
    const name = typeQueue[index]!;
    if (aggregateTypes.has(name)) continue;
    aggregateTypes.add(name);
    typeQueue.push(...aggregateAliases.get(name) ?? []);
  }
  if (aggregateTypes.size === 0) return () => [];
  const owners = new Set(inspectRustStructure(aggregateSource).enums.flatMap((item) => item.variants.flatMap((variant) => variant.fieldTypes.flatMap(rustTypeNames))));
  return (leafSource) => {
    const leafTokens = rustTokens(leafSource);
    const graph = rustTypeAliasEdges(leafTokens, rustTokenPairs(leafTokens));
    const leafFacts = inspectRustStructure(leafSource);
    for (const payload of leafFacts.inlinePayloads) graph.set(payload.name, payload.fields.flatMap((field) => rustTypeNames(field.type)));
    for (const payload of leafFacts.enums) graph.set(payload.name, payload.variants.flatMap((variant) => variant.fieldTypes.flatMap(rustTypeNames)));
    const carriers: string[] = [];
    for (const owner of owners) {
      if (!graph.has(owner)) continue;
      const paths = [[owner]];
      const visited = new Set<string>();
      for (let index = 0; index < paths.length; index++) {
        const path = paths[index]!;
        const name = path.at(-1)!;
        if (visited.has(name)) continue;
        visited.add(name);
        if (aggregateTypes.has(name)) carriers.push(path.join(" -> "));
        else for (const child of graph.get(name) ?? []) paths.push([...path, child]);
      }
    }
    return carriers.sort();
  };
}
//#endregion 🛡️MutationInputs

//#region 🛡️MutationCodecOwnership
export interface RustMutationCodecOwnershipFact {
  readonly kind: "whole-aggregate-serialization" | "whole-aggregate-deserialization" | "aggregate-variant-match";
}

function rustMutationCodecAliasClosure(seeds: readonly string[], aliases: ReadonlyMap<string, readonly string[]>): Set<string> {
  const names = new Set(seeds);
  let changed = true;
  while (changed) {
    changed = false;
    for (const [alias, targets] of aliases) {
      if (names.has(alias) || !targets.some((target) => names.has(target))) continue;
      names.add(alias);
      changed = true;
    }
  }
  return names;
}

function rustMutationCodecCall(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, nameIndex: number): { readonly open: number; readonly close: number; readonly genericStart: number | null; readonly genericEnd: number | null } | null {
  let cursor = nameIndex + 1;
  let genericStart: number | null = null;
  let genericEnd: number | null = null;
  if (tokens[cursor]?.text === "::" && tokens[cursor + 1]?.text === "<") {
    genericStart = cursor + 2;
    let depth = 1;
    cursor += 2;
    for (; cursor < tokens.length; cursor += 1) {
      if (tokens[cursor]!.text === "<") depth += 1;
      else if (tokens[cursor]!.text === ">") depth -= 1;
      if (depth === 0) break;
    }
    if (depth !== 0) return null;
    genericEnd = cursor;
    cursor += 1;
  }
  if (tokens[cursor]?.text !== "(") return null;
  const close = pairs.get(cursor);
  return close === undefined ? null : { open: cursor, close, genericStart, genericEnd };
}

function rustMutationCodecExpressionIsAggregate(tokens: readonly RustToken[], start: number, end: number, aggregateValues: readonly ReadonlyMap<string, boolean>[]): boolean {
  const values = tokens.slice(start, end).filter((token) => !["&", "mut", "(", ")"].includes(token.text));
  if (values.length !== 1 || values[0]?.kind !== "identifier") return false;
  for (let index = aggregateValues.length - 1; index >= 0; index -= 1) if (aggregateValues[index]!.has(values[0].text)) return aggregateValues[index]!.get(values[0].text) === true;
  return false;
}

function rustMutationCodecImplRanges(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, aggregateTypes: ReadonlySet<string>): (readonly [number, number])[] {
  const ranges: (readonly [number, number])[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]?.text !== "impl") continue;
    let open = -1;
    for (let cursor = index + 1; cursor < tokens.length; cursor += 1) {
      if (tokens[cursor]!.text === "{") {
        open = cursor;
        break;
      }
      if (tokens[cursor]!.text === ";") break;
    }
    const close = open < 0 ? undefined : pairs.get(open);
    if (close === undefined) continue;
    const forIndex = rustFindTopLevel(tokens, pairs, index + 1, open, new Set(["for"]));
    let cursor = forIndex < 0 ? index + 1 : forIndex + 1;
    if (tokens[cursor]?.text === "<") {
      let depth = 1;
      cursor += 1;
      for (; cursor < open && depth > 0; cursor += 1) {
        if (tokens[cursor]?.text === "<") depth += 1;
        else if (tokens[cursor]?.text === ">") depth -= 1;
      }
    }
    const path: string[] = [];
    for (; cursor < open; cursor += 1) {
      const token = tokens[cursor]!;
      if (token.kind === "identifier") path.push(token.text);
      else if (token.text === "::") continue;
      else break;
    }
    if (aggregateTypes.has(path.at(-1) ?? "")) ranges.push([open + 1, close]);
    index = close;
  }
  return ranges;
}

function rustMutationCodecSerdeCall(tokens: readonly RustToken[], nameIndex: number, serdeAliases: ReadonlySet<string>): boolean {
  return tokens[nameIndex - 1]?.text === "::" && serdeAliases.has(tokens[nameIndex - 2]?.text ?? "");
}

interface RustMutationCodecFunctionRange {
  readonly start: number;
  readonly bodyStart: number;
  readonly bodyEnd: number;
  readonly aggregateImpl: boolean;
}

function rustMutationCodecFunctions(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, aggregateImplRanges: readonly (readonly [number, number])[]): RustMutationCodecFunctionRange[] {
  const functions: RustMutationCodecFunctionRange[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]?.text !== "fn") continue;
    let bodyStart = -1;
    for (let cursor = index + 1; cursor < tokens.length; cursor += 1) {
      if (tokens[cursor]!.text !== "{") continue;
      bodyStart = cursor;
      break;
    }
    const bodyEnd = bodyStart < 0 ? undefined : pairs.get(bodyStart);
    if (bodyEnd === undefined) continue;
    functions.push({ start: index, bodyStart, bodyEnd, aggregateImpl: aggregateImplRanges.some(([start, end]) => index >= start && index < end) });
    index = bodyEnd;
  }
  return functions;
}

function rustMutationCodecTypeIsAggregate(tokens: readonly RustToken[], start: number, end: number, aggregateTypes: ReadonlySet<string>, aggregateImpl: boolean): boolean {
  const names = rustTypeNames(rustTokenText(tokens, start, end));
  return names.some((name) => aggregateTypes.has(name)) || (aggregateImpl && names.includes("Self"));
}

function rustMutationCodecParameterValues(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, fnRange: RustMutationCodecFunctionRange, aggregateTypes: ReadonlySet<string>): Map<string, boolean> {
  const values = new Map<string, boolean>();
  for (let cursor = fnRange.start + 1; cursor < fnRange.bodyStart; cursor += 1) {
    if (tokens[cursor]?.text !== "(") continue;
    const close = pairs.get(cursor);
    if (close === undefined) break;
    for (const [start, end] of rustTokenSegments(tokens, pairs, cursor + 1, close, ",")) {
      const colon = rustFindTopLevel(tokens, pairs, start, end, new Set([":"]));
      const name = tokens[start]?.text === "mut" ? tokens[start + 1] : tokens[start];
      const receiver = tokens.slice(start, end).find((token) => token.text === "self");
      if (receiver) {
        values.set("self", fnRange.aggregateImpl);
        continue;
      }
      if (name?.kind !== "identifier") continue;
      if (colon >= 0) values.set(name.text, rustMutationCodecTypeIsAggregate(tokens, colon + 1, end, aggregateTypes, fnRange.aggregateImpl));
    }
    break;
  }
  return values;
}

function rustMutationCodecLetValue(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, index: number, end: number, values: readonly Map<string, boolean>[], aggregateTypes: ReadonlySet<string>, aggregateImpl: boolean): void {
  const semicolon = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"]));
  const statementEnd = semicolon < 0 ? end : semicolon;
  const name = tokens[index + (tokens[index + 1]?.text === "mut" ? 2 : 1)];
  const equals = rustFindTopLevel(tokens, pairs, index + 1, statementEnd, new Set(["="]));
  const colon = equals < 0 ? -1 : rustFindTopLevel(tokens, pairs, index + 1, equals, new Set([":"]));
  if (name?.kind === "identifier" && equals >= 0) values.at(-1)!.set(name.text, (colon >= 0 && rustMutationCodecTypeIsAggregate(tokens, colon + 1, equals, aggregateTypes, aggregateImpl)) || rustMutationCodecExpressionIsAggregate(tokens, equals + 1, statementEnd, values));
}

function rustMutationCodecInferredAggregate(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, index: number, callClose: number, fnRange: RustMutationCodecFunctionRange, aggregateTypes: ReadonlySet<string>): boolean {
  let statementStart = fnRange.bodyStart + 1;
  for (let cursor = index - 1; cursor >= statementStart; cursor -= 1) {
    if (![";", "{", "}"].includes(tokens[cursor]?.text ?? "")) continue;
    statementStart = cursor + 1;
    break;
  }
  const letIndex = tokens.slice(statementStart, index).findIndex((token) => token.text === "let");
  if (letIndex >= 0) {
    const start = statementStart + letIndex;
    const equals = rustFindTopLevel(tokens, pairs, start + 1, index, new Set(["="]));
    const colon = equals < 0 ? -1 : rustFindTopLevel(tokens, pairs, start + 1, equals, new Set([":"]));
    if (colon >= 0) return rustMutationCodecTypeIsAggregate(tokens, colon + 1, equals, aggregateTypes, fnRange.aggregateImpl);
  }
  const arrow = rustFindTopLevel(tokens, pairs, fnRange.start + 1, fnRange.bodyStart, new Set(["->"]));
  const returnsCall = tokens.slice(statementStart, index).some((token) => token.text === "return");
  const tailCall = !tokens.slice(callClose + 1, fnRange.bodyEnd).some((token) => token.text === ";");
  return arrow >= 0 && (returnsCall || tailCall) && rustMutationCodecTypeIsAggregate(tokens, arrow + 1, fnRange.bodyStart, aggregateTypes, fnRange.aggregateImpl);
}

/** 🧪️ Finds executable aggregate codec bypasses from Rust token structure without type expansion. */
export function createRustMutationCodecOwnershipInspector(aggregateSource: string): (codecSource: string) => readonly RustMutationCodecOwnershipFact[] {
  const aggregateTokens = rustTokens(aggregateSource);
  const aggregatePairs = rustTokenPairs(aggregateTokens);
  const aggregateAliases = rustTypeAliasEdges(aggregateTokens, aggregatePairs);
  const aggregateEnums = inspectRustStructure(aggregateSource).enums.filter((item) => item.name.includes("Mutation"));
  const aggregateTypeSeeds = aggregateEnums.map((item) => item.name);
  const aggregateVariantNames = new Set(aggregateEnums.flatMap((item) => item.variants.map((variant) => variant.name)));
  return (codecSource) => {
    const tokens = rustTokens(codecSource);
    const pairs = rustTokenPairs(tokens);
    const aliases = new Map([...aggregateAliases, ...rustTypeAliasEdges(tokens, pairs)]);
    const aggregateTypes = rustMutationCodecAliasClosure(aggregateTypeSeeds, aliases);
    const serdeAliases = rustMutationCodecAliasClosure(["serde_json"], aliases);
    const aggregateImplRanges = rustMutationCodecImplRanges(tokens, pairs, aggregateTypes);
    const facts: RustMutationCodecOwnershipFact[] = [];
    for (const fnRange of rustMutationCodecFunctions(tokens, pairs, aggregateImplRanges)) {
      const values: Map<string, boolean>[] = [rustMutationCodecParameterValues(tokens, pairs, fnRange, aggregateTypes)];
      for (let index = fnRange.bodyStart + 1; index < fnRange.bodyEnd; index += 1) {
        if (tokens[index]?.text === "{") values.push(new Map());
        else if (tokens[index]?.text === "}") values.pop();
        if (tokens[index]?.text === "let") rustMutationCodecLetValue(tokens, pairs, index, fnRange.bodyEnd, values, aggregateTypes, fnRange.aggregateImpl);
        const name = tokens[index]?.text;
        if (!name || !rustMutationCodecSerdeCall(tokens, index, serdeAliases)) continue;
        const call = rustMutationCodecCall(tokens, pairs, index);
        if (!call) continue;
        if (["to_vec", "to_string", "to_value", "to_writer"].includes(name) && rustTokenSegments(tokens, pairs, call.open + 1, call.close, ",").some(([start, end]) => rustMutationCodecExpressionIsAggregate(tokens, start, end, values))) facts.push({ kind: "whole-aggregate-serialization" });
        if (!["from_slice", "from_str", "from_reader", "from_value"].includes(name)) continue;
        const genericTypes = call.genericStart === null || call.genericEnd === null ? [] : rustTypeNames(rustTokenText(tokens, call.genericStart, call.genericEnd));
        if (genericTypes.some((type) => aggregateTypes.has(type)) || (genericTypes.includes("Self") && fnRange.aggregateImpl) || (genericTypes.length === 0 && rustMutationCodecInferredAggregate(tokens, pairs, index, call.close, fnRange, aggregateTypes))) facts.push({ kind: "whole-aggregate-deserialization" });
      }
    }
    for (let index = 0; index < tokens.length; index += 1) {
      if (tokens[index]?.text !== "match") continue;
      let open = -1;
      for (let cursor = index + 1; cursor < tokens.length; cursor += 1) if (tokens[cursor]!.text === "{") { open = cursor; break; }
      const close = open < 0 ? undefined : pairs.get(open);
      if (close === undefined) continue;
      const aggregateImpl = aggregateImplRanges.some(([start, end]) => index >= start && index < end);
      for (const [start, end] of rustTokenSegments(tokens, pairs, open + 1, close, ",")) {
        const arrow = rustFindTopLevel(tokens, pairs, start, end, new Set(["=>"]));
        const path = arrow < 0 ? null : rustPatternVariantPath(tokens, start, arrow);
        const parts = path?.split("::") ?? [];
        const owner = parts.at(-2) ?? "";
        if (parts.length > 1 && (aggregateTypes.has(owner) || (aggregateImpl && owner === "Self")) && aggregateVariantNames.has(parts.at(-1) ?? "")) facts.push({ kind: "aggregate-variant-match" });
      }
      index = close;
    }
    return facts;
  };
}
//#endregion 🛡️MutationCodecOwnership

/** 🛡️ Normalizes one virtual repository path without permitting absolute or parent traversal. */
function rustVirtualRelativePath(path: string): string {
  const normalized = path.normalize("NFC").replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/{2,}/gu, "/");
  if (!normalized || normalized.startsWith("/") || /^[A-Za-z]:\//u.test(normalized)) throw new Error(`Rust virtual source path must be repository-relative: ${JSON.stringify(path)}.`);
  const segments = normalized.split("/");
  if (segments.some((segment) => !segment || segment === "." || segment === "..")) throw new Error(`Rust virtual source path contains traversal: ${JSON.stringify(path)}.`);
  return segments.join("/");
}

/** 🚫️ Applies configured opaque prefixes lexically to a virtual path before its reader is called. */
export function taxonomyRelativePathIsExcluded(path: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const normalized = rustVirtualRelativePath(path);
  return Object.values(taxonomy.pathExclusions).some((exclusion) => {
    const prefix = exclusion.path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/+$/u, "");
    return normalized === prefix || normalized.startsWith(`${prefix}/`);
  });
}

/** 🧪️ Inspects an in-memory/virtual Rust tree, filtering opaque paths before any source read. */
export function inspectRustVirtualSources(paths: readonly string[], readSource: (path: string) => string, taxonomy: Taxonomy = loadTaxonomy()): RustVirtualSourceFact[] {
  const candidates = paths.map((path) => ({ original: path, normalized: rustVirtualRelativePath(path) }))
    .filter(({ normalized }) => !taxonomyRelativePathIsExcluded(normalized, taxonomy) && normalized.endsWith(".rs"))
    .sort((left, right) => Buffer.from(left.normalized).compare(Buffer.from(right.normalized)));
  const seen = new Set<string>();
  return candidates.map(({ original, normalized }) => {
    if (seen.has(normalized)) throw new Error(`Rust virtual source path is duplicated after normalization: ${JSON.stringify(normalized)}.`);
    seen.add(normalized);
    return { path: normalized, facts: inspectRustStructure(readSource(original)) };
  });
}
//#endregion 🦀️RustStructure

//#region 🟦️TypeScriptDeclarationFacts
/** 📏️ Half-open UTF-16 coordinates in the unchanged source string. */
export interface TypeScriptDeclarationSpan { readonly start: number; readonly end: number; }
/** 🧩️ Unexpanded declaration structure with source-order member spellings. */
export interface TypeScriptDeclarationStructure { readonly form: "object" | "union" | "reference" | "enum" | "class" | "unresolved"; readonly members: readonly string[]; readonly unresolved: string | null; }
/** 🪪️ One declaration occurrence, without a provider or mutation identity claim. */
export interface TypeScriptDeclarationFact { readonly kind: "type" | "interface" | "enum" | "class" | "variable"; readonly name: string; readonly exported: boolean; readonly modulePath: readonly string[]; readonly span: TypeScriptDeclarationSpan; readonly structure: TypeScriptDeclarationStructure; }
/** 🔗️ One named import or re-export before module resolution. */
export interface TypeScriptDeclarationAliasFact { readonly relation: "import" | "reexport"; readonly typeOnly: boolean; readonly imported: string; readonly local: string; readonly moduleSpecifier: string; readonly modulePath: readonly string[]; readonly span: TypeScriptDeclarationSpan; }
/** 🚧️ Closed syntax-coverage reasons owned by the declaration grammar. */
export type TypeScriptDeclarationDiagnosticCode = "parse-error" | "unresolved-conditional-type" | "unresolved-mapped-type" | "unresolved-computed-property" | "unresolved-jsx" | "unresolved-expression" | "unsupported-function-local" | "unsupported-default-or-namespace-import" | "unsupported-import-equals" | "unsupported-export-star" | "unsupported-binding-pattern" | "unsupported-anonymous-default-class" | "unsupported-module-statement" | "unresolved-object-spread" | "unresolved-heritage" | "unsupported-type-node" | "unsupported-class-member-body" | "unsupported-ambient-module-body" | "unsupported-recovery-suffix";
/** 🩺️ Exact syntax coverage evidence, with compiler-independent coordinates. */
export interface TypeScriptDeclarationDiagnostic { readonly code: TypeScriptDeclarationDiagnosticCode; readonly span: TypeScriptDeclarationSpan; }
/** 🧾️ Pure declaration summaries; complete does not mean resolved types or mutation providers. */
export interface TypeScriptDeclarationFacts { readonly completeness: "complete" | "incomplete"; readonly declarations: readonly TypeScriptDeclarationFact[]; readonly aliases: readonly TypeScriptDeclarationAliasFact[]; readonly diagnostics: readonly TypeScriptDeclarationDiagnostic[]; }
/** 🔤️ Owned tokens keep semantic identifier values separate from physical source spans. */
interface TypeScriptDeclarationToken { readonly kind: "identifier" | "string" | "number" | "template" | "regex" | "jsx" | "punctuation"; readonly text: string; readonly value: string; readonly start: number; readonly end: number; readonly lineBreakBefore: boolean; readonly interpolated?: boolean; }
/** 🚧️ A lexical boundary which cannot be proven from the supported grammar. */
class TypeScriptDeclarationRecovery extends Error { constructor(readonly start: number) { super("TypeScript declaration syntax has an unproven recovery suffix"); } }
/** 🛑️ A proven missing required type at a declaration grammar boundary. */
class TypeScriptDeclarationSyntaxError extends Error { constructor(readonly span: TypeScriptDeclarationSpan) { super("TypeScript declaration syntax has an invalid token boundary"); } }
/** 🔬️ Scans literals atomically and tracks lexical goals independently of physical path analysis. */
class TypeScriptDeclarationScanner {
  readonly tokens: TypeScriptDeclarationToken[] = [];
  readonly pairs = new Map<number, number>();
  cursor: number;
  private goal: "operand" | "operator" | "ambiguous" = "operand";
  private member = false;
  private control = false;
  private declarationBody = false;
  private block = false;
  private statement = true;
  private readonly groups: { index: number; close: string; goal: "operand" | "operator" | "ambiguous"; control: boolean }[] = [];
  constructor(readonly source: string, readonly language: "ts" | "tsx", start = 0) { this.cursor = start; }

  private escape(start: number): { end: number; value: string } {
    const character = this.source[start + 1];
    if (character === undefined) throw new TypeScriptDeclarationRecovery(start);
    if (character === "u" || character === "x") {
      let cursor = start + 2, digits = "";
      if (character === "u" && this.source[cursor] === "{") {
        cursor++;
        while (cursor < this.source.length && /[0-9a-f]/iu.test(this.source[cursor]) && digits.length < 6) digits += this.source[cursor++];
        if (!digits || this.source[cursor] !== "}") throw new TypeScriptDeclarationRecovery(start);
        cursor++;
      } else {
        const length = character === "u" ? 4 : 2;
        digits = this.source.slice(cursor, cursor + length);
        if (digits.length !== length || !/^[0-9a-f]+$/iu.test(digits)) throw new TypeScriptDeclarationRecovery(start);
        cursor += length;
      }
      const value = Number.parseInt(digits, 16);
      if (value > 0x10ffff) throw new TypeScriptDeclarationRecovery(start);
      return { end: cursor, value: String.fromCodePoint(value) };
    }
    if (character === "\r" || character === "\n" || character === "\u2028" || character === "\u2029") return { end: start + (character === "\r" && this.source[start + 2] === "\n" ? 3 : 2), value: "" };
    if (/[1-9]/u.test(character) || character === "0" && /[0-9]/u.test(this.source[start + 2] ?? "")) throw new TypeScriptDeclarationRecovery(start);
    const values: Readonly<Record<string, string>> = { n: "\n", r: "\r", t: "\t", b: "\b", f: "\f", v: "\v", "0": "\0" };
    return { end: start + 2, value: values[character] ?? character };
  }

  private quoted(start: number): { end: number; value: string } {
    const quote = this.source[start];
    let cursor = start + 1, value = "";
    while (cursor < this.source.length) {
      const character = this.source[cursor];
      if (character === quote) return { end: cursor + 1, value };
      if (/[\r\n\u2028\u2029]/u.test(character)) throw new TypeScriptDeclarationRecovery(start);
      if (character === "\\") { const escaped = this.escape(cursor); cursor = escaped.end; value += escaped.value; }
      else value += this.source[cursor++];
    }
    throw new TypeScriptDeclarationRecovery(start);
  }

  private embedded(start: number): number {
    const scanner = new TypeScriptDeclarationScanner(this.source, this.language, start);
    scanner.scan(true);
    return scanner.cursor;
  }

  private template(start: number): { end: number; interpolated: boolean } {
    let cursor = start + 1, interpolated = false;
    while (cursor < this.source.length) {
      if (this.source[cursor] === "\u0060") return { end: cursor + 1, interpolated };
      if (this.source[cursor] === "\\") { cursor = this.escape(cursor).end; continue; }
      if (this.source[cursor] === "$" && this.source[cursor + 1] === "{") { interpolated = true; cursor = this.embedded(cursor + 2); continue; }
      cursor++;
    }
    throw new TypeScriptDeclarationRecovery(start);
  }

  private regex(start: number): number {
    let cursor = start + 1, characterClass = false;
    while (cursor < this.source.length) {
      const character = this.source[cursor];
      if (/[\r\n\u2028\u2029]/u.test(character)) throw new TypeScriptDeclarationRecovery(start);
      if (character === "\\") { if (cursor + 1 >= this.source.length || /[\r\n\u2028\u2029]/u.test(this.source[cursor + 1])) throw new TypeScriptDeclarationRecovery(start); cursor += 2; continue; }
      if (character === "[") characterClass = true;
      else if (character === "]") characterClass = false;
      else if (character === "/" && !characterClass) {
        cursor++;
        while (cursor < this.source.length && /[$_\p{ID_Continue}]/u.test(String.fromCodePoint(this.source.codePointAt(cursor)!))) cursor += String.fromCodePoint(this.source.codePointAt(cursor)!).length;
        return cursor;
      }
      cursor++;
    }
    throw new TypeScriptDeclarationRecovery(start);
  }

  private jsx(start: number): number {
    let cursor = start + 1, name = "";
    if (this.source[cursor] !== ">") {
      const first = String.fromCodePoint(this.source.codePointAt(cursor) ?? 0);
      if (!/[$_\p{ID_Start}]/u.test(first)) throw new TypeScriptDeclarationRecovery(start);
      while (cursor < this.source.length) {
        const character = String.fromCodePoint(this.source.codePointAt(cursor)!);
        if (!/[$_\p{ID_Continue}:.-]/u.test(character)) break;
        name += character; cursor += character.length;
      }
      while (cursor < this.source.length && this.source[cursor] !== ">") {
        if (this.source.startsWith("/>", cursor)) return cursor + 2;
        const character = this.source[cursor];
        if (character === "'" || character === '"') cursor = this.quoted(cursor).end;
        else if (character === "{") cursor = this.embedded(cursor + 1);
        else if (character === "<") throw new TypeScriptDeclarationRecovery(start);
        else cursor++;
      }
    }
    if (this.source[cursor] !== ">") throw new TypeScriptDeclarationRecovery(start);
    cursor++;
    while (cursor < this.source.length) {
      if (this.source.startsWith("</", cursor)) {
        const nameStart = cursor + 2; cursor = nameStart;
        while (cursor < this.source.length && !/[\s>]/u.test(this.source[cursor])) cursor++;
        if (this.source.slice(nameStart, cursor) !== name) throw new TypeScriptDeclarationRecovery(start);
        while (cursor < this.source.length && /\s/u.test(this.source[cursor])) cursor++;
        if (this.source[cursor] !== ">") throw new TypeScriptDeclarationRecovery(start);
        return cursor + 1;
      }
      if (this.source[cursor] === "<") cursor = this.jsx(cursor);
      else if (this.source[cursor] === "{") cursor = this.embedded(cursor + 1);
      else cursor++;
    }
    throw new TypeScriptDeclarationRecovery(start);
  }

  private transition(token: TypeScriptDeclarationToken, index: number): void {
    const text = token.text;
    if (token.kind !== "punctuation") {
      if (token.kind === "identifier" && !this.member) {
        if (["if", "while", "for", "switch", "with", "catch"].includes(text)) { this.control = true; this.goal = "operand"; this.statement = false; return; }
        if (["function", "class", "interface", "namespace", "module", "enum"].includes(text)) this.declarationBody = true;
        if (["return", "throw", "case", "yield", "await", "typeof", "void", "delete", "new", "in", "instanceof"].includes(text)) { this.goal = "operand"; this.statement = false; return; }
        if (["else", "try", "finally", "do"].includes(text)) { this.block = true; this.goal = "operand"; this.statement = true; return; }
        if (["export", "declare", "default", "abstract", "async"].includes(text) && this.statement) return;
      }
      this.member = false; this.goal = "operator"; this.statement = false;
      return;
    }
    if (["(", "[", "{"].includes(text)) {
      const control = text === "(" && this.control;
      const isBlock = text === "{" && (this.block || this.declarationBody || this.statement);
      const closingGoal = text === "{" ? isBlock ? "operand" : this.goal === "operand" ? "operator" : "ambiguous" : control ? "operand" : "operator";
      this.groups.push({ index, close: text === "(" ? ")" : text === "[" ? "]" : "}", goal: closingGoal, control });
      this.control = false; this.block = false;
      if (text === "{") this.declarationBody = false;
      this.goal = "operand"; this.statement = isBlock;
      return;
    }
    if ([")", "]", "}"].includes(text)) {
      const open = this.groups.pop();
      if (!open || open.close !== text) throw new TypeScriptDeclarationRecovery(token.start);
      this.pairs.set(open.index, index); this.pairs.set(index, open.index);
      this.goal = open.goal; this.block = open.control || text === ")"; this.statement = text === "}" && open.goal === "operand";
      return;
    }
    if (text === "." || text === "?.") { this.member = true; this.goal = "operator"; return; }
    if (text === "++" || text === "--") return;
    this.goal = "operand"; this.statement = text === ";";
    if (text === "=>") this.block = true;
  }

  scan(stopAtBrace = false): void {
    const number = /(?:0[xX][0-9a-fA-F](?:_?[0-9a-fA-F])*n?|0[bB][01](?:_?[01])*n?|0[oO][0-7](?:_?[0-7])*n?|(?:[0-9](?:_?[0-9])*(?:\.(?:[0-9](?:_?[0-9])*)?)?|\.[0-9](?:_?[0-9])*)(?:[eE][+-]?[0-9](?:_?[0-9])*)?n?)/y;
    const operators = ["===", "!==", ">>>=", "**=", "&&=", "||=", "??=", "=>", "==", "!=", "<=", ">=", "++", "--", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "&&", "||", "??", "?.", "**", "<<=", ">>=", ">>>", "<<", ">>", "..."];
    let previousEnd = this.cursor;
    while (this.cursor < this.source.length) {
      const start = this.cursor, first = this.source[start];
      if (/\s/u.test(first)) { this.cursor++; continue; }
      if (this.source.startsWith("//", start) || start === 0 && this.source.startsWith("#!", start)) { while (this.cursor < this.source.length && !/[\r\n\u2028\u2029]/u.test(this.source[this.cursor])) this.cursor++; continue; }
      if (this.source.startsWith("/*", start)) { const close = this.source.indexOf("*/", start + 2); if (close < 0) throw new TypeScriptDeclarationRecovery(start); this.cursor = close + 2; continue; }
      if (stopAtBrace && first === "}" && !this.groups.length) { this.cursor++; return; }
      let kind: TypeScriptDeclarationToken["kind"] = "punctuation", value = "", text = "", interpolated: boolean | undefined;
      if (first === "'" || first === '"') { const string = this.quoted(start); kind = "string"; value = string.value; this.cursor = string.end; }
      else if (first === "\u0060") { const template = this.template(start); kind = "template"; interpolated = template.interpolated; this.cursor = template.end; }
      else if (first === "/" && this.goal !== "operator") { if (this.goal === "ambiguous") throw new TypeScriptDeclarationRecovery(start); kind = "regex"; this.cursor = this.regex(start); }
      else if (first === "<" && this.language === "tsx" && this.goal === "operand") { kind = "jsx"; this.cursor = this.jsx(start); }
      else if (first === "\\" || /[$_\p{ID_Start}]/u.test(String.fromCodePoint(this.source.codePointAt(start)!)) || first === "#" && /[$_\p{ID_Start}]/u.test(String.fromCodePoint(this.source.codePointAt(start + 1) ?? 0))) {
        kind = "identifier";
        if (first === "#") { value = "#"; this.cursor++; }
        let initial = true;
        while (this.cursor < this.source.length) {
          const escaped = this.source[this.cursor] === "\\";
          if (escaped && this.source[this.cursor + 1] !== "u") throw new TypeScriptDeclarationRecovery(this.cursor);
          const entry = escaped ? this.escape(this.cursor) : { end: this.cursor + String.fromCodePoint(this.source.codePointAt(this.cursor)!).length, value: String.fromCodePoint(this.source.codePointAt(this.cursor)!) };
          if (!(initial ? /[$_\p{ID_Start}]/u : /[$_\u200c\u200d\p{ID_Continue}]/u).test(entry.value)) { if (escaped) throw new TypeScriptDeclarationRecovery(this.cursor); break; }
          value += entry.value; this.cursor = entry.end; initial = false;
        }
        text = this.source.slice(start, this.cursor);
      } else if (/[0-9]/u.test(first) || first === "." && /[0-9]/u.test(this.source[start + 1] ?? "")) {
        number.lastIndex = start; const match = number.exec(this.source);
        if (!match?.[0]) throw new TypeScriptDeclarationRecovery(start);
        kind = "number"; text = value = match[0]; this.cursor += text.length;
      } else {
        text = operators.find((operator) => this.source.startsWith(operator, start)) ?? first;
        if (!/^[{}()[\].,;:?~!+\-*/%<>=&|^]$/u.test(text) && !operators.includes(text)) throw new TypeScriptDeclarationRecovery(start);
        value = text; this.cursor += text.length;
      }
      const token: TypeScriptDeclarationToken = { kind, text, value, start, end: this.cursor, lineBreakBefore: /[\r\n\u2028\u2029]/u.test(this.source.slice(previousEnd, start)), ...(interpolated === undefined ? {} : { interpolated }) };
      const pieces = kind === "punctuation" && /^[<>]{1,3}=?$/u.test(text) && text.length > 1 ? [...text].map((part, offset) => ({ ...token, text: part, value: part, start: start + offset, end: start + offset + 1, lineBreakBefore: offset === 0 && token.lineBreakBefore })) : [token];
      for (const piece of pieces) { const index = this.tokens.length; this.tokens.push(piece); this.transition(piece, index); }
      previousEnd = this.cursor;
    }
    if (stopAtBrace || this.groups.length) throw new TypeScriptDeclarationRecovery(this.groups.length ? this.tokens[this.groups[0].index].start : this.cursor);
  }
}

/** 🌳️ Parses declaration grammar from owned tokens and retains unsupported syntax explicitly. */
class TypeScriptDeclarationParser {
  readonly declarations: TypeScriptDeclarationFact[] = [];
  readonly aliases: TypeScriptDeclarationAliasFact[] = [];
  readonly diagnostics: TypeScriptDeclarationDiagnostic[] = [];
  readonly parseErrors: TypeScriptDeclarationDiagnostic[] = [];
  constructor(readonly source: string, readonly tokens: readonly TypeScriptDeclarationToken[], readonly pairs: ReadonlyMap<number, number>) {}

  private text(index: number): string { return this.tokens[index]?.text ?? ""; }
  private span(start: number, end: number): TypeScriptDeclarationSpan { return { start: this.tokens[start]?.start ?? this.source.length, end: end > start ? this.tokens[end - 1].end : this.tokens[start]?.start ?? this.source.length }; }
  private raw(start: number, end: number): string { const span = this.span(start, end); return this.source.slice(span.start, span.end); }
  private diagnose(code: TypeScriptDeclarationDiagnosticCode, start: number, end: number): void { this.diagnostics.push({ code, span: this.span(start, end) }); }
  private close(index: number, limit: number): number { const close = this.pairs.get(index); if (close === undefined || close <= index || close >= limit) throw new TypeScriptDeclarationRecovery(this.tokens[index]?.start ?? this.source.length); return close; }

  private angleEnd(start: number, end: number): number {
    let depth = 0;
    for (let index = start; index < end; index++) {
      if (this.text(index) === "<") depth++;
      else if (this.text(index) === ">" && --depth === 0) return index;
      else if (["(", "[", "{"].includes(this.text(index))) index = this.close(index, end);
    }
    throw new TypeScriptDeclarationRecovery(this.tokens[start].start);
  }

  private top(start: number, end: number, selected: readonly string[], angles = false): number[] {
    const result: number[] = [];
    for (let index = start; index < end; index++) {
      const text = this.text(index);
      if (selected.includes(text)) result.push(index);
      if (["(", "[", "{"].includes(text)) index = this.close(index, end);
      else if (angles && text === "<") index = this.angleEnd(index, end);
    }
    return result;
  }

  private segments(start: number, end: number, delimiter: string, angles = false): [number, number][] {
    const result: [number, number][] = [];
    let first = start;
    for (const index of this.top(start, end, [delimiter], angles)) { if (first < index) result.push([first, index]); first = index + 1; }
    if (first < end) result.push([first, end]);
    return result;
  }

  private declarationStart(index: number): boolean {
    const text = this.text(index);
    if (["export", "import", "const", "let", "var", "interface", "enum", "namespace", "module", "class", "function", "declare"].includes(text)) return true;
    return text === "type" && this.tokens[index + 1]?.kind === "identifier" && ["=", "<"].includes(this.text(index + 2));
  }

  private ending(start: number, end: number, comma = false): number {
    for (let index = start; index < end; index++) {
      const text = this.text(index);
      if (text === ";" || comma && text === ",") return index;
      if (index > start && this.tokens[index].lineBreakBefore && this.declarationStart(index) && ![".", "?.", "=", "=>", ":", "?", "+", "-", "*", "/", "|", "&", "||", "&&", "??", ",", "<", ">"].includes(this.text(index - 1))) return index;
      if (["(", "[", "{"].includes(text)) index = this.close(index, end);
    }
    return end;
  }

  private invalid(index: number): never { throw new TypeScriptDeclarationRecovery(this.tokens[index]?.start ?? this.source.length); }

  private typeEnd(start: number, end: number, conditional = true): number {
    let index = this.typeAtom(start, end);
    while (index < end && ["|", "&"].includes(this.text(index))) index = this.typeAtom(index + 1, end);
    if (conditional && this.text(index) === "extends") {
      index = this.typeEnd(index + 1, end, false);
      if (this.text(index) !== "?") this.invalid(index);
      index = this.typeEnd(index + 1, end);
      if (this.text(index) !== ":") this.invalid(index);
      index = this.typeEnd(index + 1, end);
    }
    return index;
  }

  private typeAtom(start: number, end: number): number {
    if (start >= end || [";", ",", "=", ">", ")", "]", "}", ":", "?"].includes(this.text(start))) throw new TypeScriptDeclarationSyntaxError(this.span(start, Math.min(start + 1, this.tokens.length)));
    let index = start, text = this.text(index);
    if (["keyof", "readonly", "unique", "typeof", "infer"].includes(text)) {
      if (text === "infer") {
        if (this.tokens[++index]?.kind !== "identifier") this.invalid(index);
        index++;
        if (this.text(index) === "extends") index = this.typeEnd(index + 1, end, false);
      } else index = this.typeAtom(index + 1, end);
    } else if (text === "(") {
      const close = this.close(index, end);
      if (this.text(close + 1) === "=>") {
        this.parameters(index + 1, close);
        index = this.typeEnd(close + 2, end);
      } else {
        if (this.typeEnd(index + 1, close) !== close) this.invalid(index + 1);
        index = close + 1;
      }
    } else if (text === "<") {
      index = this.typeParameters(index, end);
      if (this.text(index) !== "(") this.invalid(index);
      const close = this.close(index, end);
      this.parameters(index + 1, close);
      if (this.text(close + 1) !== "=>") this.invalid(close + 1);
      index = this.typeEnd(close + 2, end);
    } else if (text === "[") {
      const close = this.close(index, end);
      let cursor = index + 1;
      while (cursor < close) {
        if (this.text(cursor) === "...") cursor++;
        if (this.tokens[cursor]?.kind === "identifier" && [":", "?"].includes(this.text(cursor + 1))) {
          cursor++;
          if (this.text(cursor) === "?") cursor++;
          if (this.text(cursor) !== ":") this.invalid(cursor);
          cursor++;
        }
        cursor = this.typeEnd(cursor, close);
        if (this.text(cursor) === "?") cursor++;
        if (cursor < close && this.text(cursor) !== ",") this.invalid(cursor);
        if (cursor < close) cursor++;
      }
      index = close + 1;
    } else if (text === "{") {
      const close = this.close(index, end);
      if (this.mapped(index, close + 1)) {
        let cursor = index + 1;
        if (["+", "-"].includes(this.text(cursor))) cursor++;
        if (this.text(cursor) === "readonly") cursor++;
        if (this.text(cursor) !== "[") this.invalid(cursor);
        const bindingClose = this.close(cursor, close);
        if (this.tokens[cursor + 1]?.kind !== "identifier" || this.text(cursor + 2) !== "in") this.invalid(cursor + 1);
        cursor = this.typeEnd(cursor + 3, bindingClose);
        if (this.text(cursor) === "as") cursor = this.typeEnd(cursor + 1, bindingClose);
        if (cursor !== bindingClose) this.invalid(cursor);
        cursor++;
        if (["+", "-"].includes(this.text(cursor))) cursor++;
        if (this.text(cursor) === "?") cursor++;
        if (this.text(cursor) !== ":") this.invalid(cursor);
        cursor = this.typeEnd(cursor + 1, close);
        if (this.text(cursor) === ";") cursor++;
        if (cursor !== close) this.invalid(cursor);
      } else this.members(index + 1, close, "type");
      index = close + 1;
    } else if (text === "-" && this.tokens[index + 1]?.kind === "number") index += 2;
    else if (["string", "number", "template"].includes(this.tokens[index]?.kind)) index++;
    else if (this.tokens[index]?.kind === "identifier" && !["extends", "implements", "in", "as", "is", "const", "let", "var", "export", "import", "return", "throw", "class", "function", "interface", "enum"].includes(text)) index++;
    else this.invalid(index);
    while (index < end) {
      text = this.text(index);
      if (text === "." && this.tokens[index + 1]?.kind === "identifier") index += 2;
      else if (text === "<") {
        const close = this.angleEnd(index, end);
        let cursor = index + 1;
        if (cursor === close) this.invalid(cursor);
        while (cursor < close) {
          cursor = this.typeEnd(cursor, close);
          if (cursor < close && this.text(cursor) !== ",") this.invalid(cursor);
          if (cursor < close) cursor++;
        }
        index = close + 1;
      } else if (text === "[") {
        const close = this.close(index, end);
        if (index + 1 < close && this.typeEnd(index + 1, close) !== close) this.invalid(index + 1);
        index = close + 1;
      } else break;
    }
    return index;
  }

  private typeParameters(start: number, end: number): number {
    const close = this.angleEnd(start, end);
    let cursor = start + 1;
    if (cursor === close) this.invalid(cursor);
    while (cursor < close) {
      while (["const", "in", "out"].includes(this.text(cursor)) && this.tokens[cursor + 1]?.kind === "identifier") cursor++;
      if (this.tokens[cursor]?.kind !== "identifier") this.invalid(cursor);
      cursor++;
      if (this.text(cursor) === "extends") cursor = this.typeEnd(cursor + 1, close, false);
      if (this.text(cursor) === "=") cursor = this.typeEnd(cursor + 1, close);
      if (cursor < close && this.text(cursor) !== ",") this.invalid(cursor);
      if (cursor < close) cursor++;
    }
    return close + 1;
  }

  private parameters(start: number, end: number): [number, number][] {
    const annotations: [number, number][] = [];
    let cursor = start;
    while (cursor < end) {
      while (["public", "private", "protected", "readonly", "override"].includes(this.text(cursor)) && this.tokens[cursor + 1]?.kind === "identifier") cursor++;
      if (this.text(cursor) === "...") cursor++;
      if (this.tokens[cursor]?.kind !== "identifier") throw new TypeScriptDeclarationRecovery(this.tokens[cursor]?.start ?? this.source.length);
      cursor++;
      if (this.text(cursor) === "?") cursor++;
      if (this.text(cursor) === ":") { const first = ++cursor; cursor = this.typeEnd(cursor, end); annotations.push([first, cursor]); }
      if (this.text(cursor) === "=") cursor = this.expressionEnd(cursor + 1, end);
      if (cursor < end && this.text(cursor) !== ",") this.invalid(cursor);
      if (cursor < end) cursor++;
    }
    return annotations;
  }

  private expressionEnd(start: number, end: number, minimum = 0): number {
    if (start >= end) this.invalid(start);
    let index = start, text = this.text(index);
    if (["!", "~", "+", "-", "typeof", "void", "delete", "await", "yield", "++", "--"].includes(text)) index = this.expressionEnd(index + 1, end, 15);
    else if (text === "{") { const close = this.close(index, end); this.members(index + 1, close, "object"); index = close + 1; }
    else if (text === "(" || text === "[") {
      const close = this.close(index, end);
      let cursor = index + 1;
      if (text === "(" && cursor === close) this.invalid(cursor);
      while (cursor < close) {
        if (text === "[" && this.text(cursor) === ",") { cursor++; continue; }
        if (this.text(cursor) === "...") cursor++;
        cursor = this.expressionEnd(cursor, close);
        if (cursor < close && this.text(cursor) !== ",") this.invalid(cursor);
        if (cursor < close) cursor++;
      }
      index = close + 1;
    } else if (["string", "number", "template", "regex", "jsx"].includes(this.tokens[index]?.kind) || this.tokens[index]?.kind === "identifier" && !["const", "let", "var", "export", "import", "return", "throw", "class", "function", "interface", "enum", "extends"].includes(text)) index++;
    else throw new TypeScriptDeclarationRecovery(this.tokens[index]?.start ?? this.source.length);
    while (index < end) {
      text = this.text(index);
      if ([".", "?."].includes(text)) {
        if (this.tokens[index + 1]?.kind !== "identifier") throw new TypeScriptDeclarationRecovery(this.tokens[index].start);
        index += 2; continue;
      }
      if (text === "(" || text === "[") {
        const close = this.close(index, end);
        let cursor = index + 1;
        if (text === "[" && cursor === close) this.invalid(cursor);
        while (cursor < close) {
          if (this.text(cursor) === "...") cursor++;
          cursor = this.expressionEnd(cursor, close);
          if (cursor < close && this.text(cursor) !== ",") this.invalid(cursor);
          if (cursor < close) cursor++;
        }
        index = close + 1; continue;
      }
      if (["!", "++", "--"].includes(text) && !this.tokens[index].lineBreakBefore) { index++; continue; }
      if (["as", "satisfies"].includes(text) && minimum <= 10) {
        index = this.text(index + 1) === "const" && text === "as" ? index + 2 : this.typeEnd(index + 1, end);
        continue;
      }
      if (text === "?" && minimum <= 1) {
        index = this.expressionEnd(index + 1, end);
        if (this.text(index) !== ":") this.invalid(index);
        index = this.expressionEnd(index + 1, end, 1); continue;
      }
      const precedence = text === "=" ? 0 : ["??", "||"].includes(text) ? 2 : text === "&&" ? 3 : text === "|" ? 4 : text === "^" ? 5 : text === "&" ? 6 : ["==", "!=", "===", "!=="].includes(text) ? 7 : ["<", ">", "in", "instanceof"].includes(text) ? 8 : ["+", "-"].includes(text) ? 11 : ["*", "/", "%"].includes(text) ? 12 : text === "**" ? 13 : -1;
      if (precedence < minimum) break;
      index = this.expressionEnd(index + 1, end, precedence + (text === "=" || text === "**" ? 0 : 1));
    }
    return index;
  }

  private members(start: number, end: number, mode: "object" | "type" | "class" | "enum"): { names: string[]; computed: boolean; spreads: [number, number][]; bodies: [number, number][]; annotations: [number, number][] } {
    const names: string[] = [], spreads: [number, number][] = [], bodies: [number, number][] = [], annotations: [number, number][] = [];
    let computed = false;
    for (let index = start; index < end;) {
      if (this.text(index) === ";" && (mode === "type" || mode === "class")) { index++; continue; }
      const first = index;
      let bodyComplete = false;
      if (this.text(index) === "...") {
        if (mode !== "object") this.invalid(index);
        index = this.expressionEnd(index + 1, end);
        spreads.push([first, index]);
      } else if (mode === "class" && this.text(index) === "static" && this.text(index + 1) === "{") {
        const close = this.close(index + 1, end);
        if (close > index + 2) bodies.push([index + 1, close + 1]);
        index = close + 1; bodyComplete = true;
      } else {
        while (mode !== "enum" && ["public", "private", "protected", "readonly", "abstract", "declare", "override", "static", "accessor", "async"].includes(this.text(index)) && ![":", "?", "!", "=", "(", "<", ";", ",", ""].includes(this.text(index + 1))) index++;
        if (mode !== "enum" && ["get", "set"].includes(this.text(index)) && ![":", "?", "=", "(", "<", ";", ",", ""].includes(this.text(index + 1))) index++;
        if (mode !== "enum" && this.text(index) === "*") index++;
        const name = index, nameless = mode === "type" && ["(", "<"].includes(this.text(index));
        if (this.text(index) === "[") {
          const close = this.close(index, end);
          if ((mode === "type" || mode === "class") && this.tokens[index + 1]?.kind === "identifier" && this.text(index + 2) === ":") this.invalid(index);
          computed = true; index = close + 1;
        }
        else if (!nameless && ["identifier", "string", "number"].includes(this.tokens[index]?.kind)) {
          if (!(mode === "class" && this.text(index) === "constructor")) names.push(this.raw(index, index + 1));
          index++;
        } else if (!nameless) this.invalid(index);
        if (this.text(index) === "?" && mode !== "enum") index++;
        if (this.text(index) === "!" && mode === "class") index++;
        if (this.text(index) === "<" && mode !== "enum") index = this.typeParameters(index, end);
        if (this.text(index) === "(" && mode !== "enum") {
          const close = this.close(index, end);
          annotations.push(...this.parameters(index + 1, close));
          index = close + 1;
          if (this.text(index) === ":") { const firstType = ++index; index = this.typeEnd(index, end); annotations.push([firstType, index]); }
          if (this.text(index) === "{") {
            if (mode === "type") this.invalid(index);
            const close = this.close(index, end);
            if (close > index + 1) {
              if (mode === "class") bodies.push([index, close + 1]);
              else this.diagnose("unsupported-recovery-suffix", first, close + 1);
            }
            index = close + 1; bodyComplete = true;
          } else if (mode === "object") this.invalid(index);
        } else {
          if (nameless) this.invalid(index);
          if (this.text(index) === ":") {
            if (mode === "enum") this.invalid(index);
            const firstType = ++index;
            index = mode === "object" ? this.expressionEnd(index, end) : this.typeEnd(index, end);
            if (mode !== "object") annotations.push([firstType, index]);
          } else if (mode === "object" && (this.tokens[name]?.kind !== "identifier" || name !== first)) this.invalid(index);
          if (this.text(index) === "=") {
            if (mode === "type" || mode === "object") this.invalid(index);
            index = this.expressionEnd(index + 1, end);
          }
        }
      }
      if (index >= end) break;
      const delimiter = this.text(index);
      if (delimiter === "," && mode !== "class" || delimiter === ";" && (mode === "type" || mode === "class")) { index++; continue; }
      if (mode === "class" && bodyComplete || (mode === "type" || mode === "class") && this.tokens[index].lineBreakBefore) continue;
      this.invalid(index);
    }
    return { names, computed, spreads, bodies, annotations };
  }

  private reference(start: number, end: number): boolean {
    const primitives = ["any", "unknown", "never", "void", "undefined", "null", "string", "number", "boolean", "bigint", "symbol", "object", "intrinsic", "this", "true", "false", "keyof", "typeof", "infer", "readonly", "unique", "new", "abstract"];
    if (this.tokens[start]?.kind !== "identifier" || primitives.includes(this.text(start))) return false;
    let index = start + 1;
    while (index < end && this.text(index) === "." && this.tokens[index + 1]?.kind === "identifier") index += 2;
    if (index < end && this.text(index) === "<") index = this.angleEnd(index, end) + 1;
    return index === end;
  }

  private conditional(start: number, end: number): { extends: number; question: number; colon: number } | null {
    const positions = this.top(start, end, ["extends", "?", ":"], true);
    const extend = positions.find((index) => this.text(index) === "extends");
    const question = extend === undefined ? undefined : positions.find((index) => index > extend && this.text(index) === "?");
    if (extend === undefined || question === undefined) return null;
    let nested = 0;
    for (const index of positions.filter((index) => index > question)) {
      if (this.text(index) === "?") nested++;
      else if (this.text(index) === ":" && nested-- === 0) return { extends: extend, question, colon: index };
    }
    return null;
  }

  private mapped(start: number, end: number): boolean {
    if (this.text(start) !== "{" || this.pairs.get(start) !== end - 1) return false;
    let index = start + 1;
    while (["readonly", "+", "-"].includes(this.text(index))) index++;
    if (this.text(index) !== "[") return false;
    const close = this.close(index, end);
    return this.top(index + 1, close, ["in"])[0] !== undefined;
  }

  private structure(start: number, end: number): TypeScriptDeclarationStructure {
    if (start >= end) return { form: "unresolved", members: [], unresolved: "unsupported-type" };
    if (this.conditional(start, end)) return { form: "unresolved", members: [], unresolved: "conditional" };
    const union = this.segments(start, end, "|", true);
    if (union.length > 1) {
      const unresolved = union.some(([first, last]) => {
        if (this.text(first) === "(" && this.pairs.get(first) === last - 1) { first++; last--; }
        return this.conditional(first, last) !== null || this.mapped(first, last);
      });
      return { form: "union", members: union.map(([first, last]) => this.raw(first, last)), unresolved: unresolved ? "conditional-or-mapped-union-member" : null };
    }
    if (this.mapped(start, end)) return { form: "unresolved", members: [], unresolved: "mapped" };
    if (this.text(start) === "{" && this.pairs.get(start) === end - 1) { const members = this.members(start + 1, end - 1, "type"); return { form: "object", members: members.names, unresolved: members.computed ? "computed-property" : null }; }
    if (this.reference(start, end)) return { form: "reference", members: [this.raw(start, end)], unresolved: null };
    return { form: "unresolved", members: [], unresolved: "unsupported-type" };
  }

  private typeDiagnostics(start: number, end: number, summarized = true, unwrap = false): void {
    if (start >= end) return;
    if (unwrap && this.text(start) === "(" && this.pairs.get(start) === end - 1) { this.typeDiagnostics(start + 1, end - 1, summarized, true); return; }
    const conditional = this.conditional(start, end);
    if (conditional) {
      this.diagnose("unresolved-conditional-type", start, end);
      for (const [first, last] of [[start, conditional.extends], [conditional.extends + 1, conditional.question], [conditional.question + 1, conditional.colon], [conditional.colon + 1, end]]) this.typeDiagnostics(first, last, false, true);
      return;
    }
    const union = this.segments(start, end, "|", true);
    if (union.length > 1) { for (const [first, last] of union) this.typeDiagnostics(first, last, summarized, true); return; }
    if (this.mapped(start, end)) {
      this.diagnose("unresolved-mapped-type", start, end);
      const colon = this.top(start + 1, end - 1, [":"])[0];
      if (colon !== undefined) this.typeDiagnostics(colon + 1, this.text(end - 2) === ";" ? end - 2 : end - 1, false, true);
      return;
    }
    if (this.text(start) === "{" && this.pairs.get(start) === end - 1) {
      const members = this.members(start + 1, end - 1, "type");
      if (members.computed) this.diagnose("unresolved-computed-property", start, end);
      for (const [first, last] of members.annotations) this.typeDiagnostics(first, last, false, true);
      return;
    }
    const reference = this.reference(start, end);
    if (reference) {
      const open = this.top(start, end, ["<"])[0];
      if (open !== undefined) for (const [first, last] of this.segments(open + 1, this.angleEnd(open, end), ",", true)) this.typeDiagnostics(first, last, false, true);
      return;
    }
    if (summarized) this.diagnose("unsupported-type-node", start, end);
    for (let index = start; index < end; index++) {
      if (["(", "[", "{"].includes(this.text(index))) { const close = this.close(index, end); this.typeDiagnostics(index + (this.text(index) === "{" ? 0 : 1), close + (this.text(index) === "{" ? 1 : 0), false, true); index = close; }
    }
  }

  private expression(start: number, end: number): TypeScriptDeclarationStructure {
    const assertions = this.top(start, end, ["as", "satisfies"]);
    let first = start, last = assertions.at(-1) ?? end;
    if (this.text(first) === "<") { const close = this.angleEnd(first, last); first = close + 1; }
    if (first >= last) return { form: "unresolved", members: [], unresolved: "initializer:expression" };
    if (this.text(first) === "{" && this.pairs.get(first) === last - 1) {
      const members = this.members(first + 1, last - 1, "object");
      for (const [left, right] of members.spreads) this.diagnose("unresolved-object-spread", left, right);
      if (members.computed && !members.spreads.length) this.diagnose("unresolved-computed-property", start, end);
      return { form: "object", members: members.names, unresolved: members.spreads.length ? "object-spread" : members.computed ? "computed-property" : null };
    }
    const token = this.tokens[first], kind = last === first + 1 ? token.kind : "punctuation";
    const unresolved = kind === "template" ? token.interpolated ? "initializer:template-interpolation" : "initializer:template-literal" : kind === "string" ? "initializer:string-literal" : kind === "regex" ? "initializer:regex-literal" : kind === "jsx" ? "initializer:jsx" : "initializer:expression";
    if (kind === "jsx") this.diagnose("unresolved-jsx", start, end);
    return { form: "unresolved", members: [], unresolved };
  }

  private importOrExport(start: number, end: number, modulePath: readonly string[]): number {
    const relation = this.text(start) === "import" ? "import" : "reexport", stop = this.ending(start + 1, end), next = stop + (this.text(stop) === ";" ? 1 : 0);
    if (relation === "import" && this.top(start + 1, stop, ["="])[0] !== undefined) { this.diagnose("unsupported-import-equals", start, next); return next; }
    let cursor = start + 1, typeOnly = false;
    if (this.text(cursor) === "type") { typeOnly = true; cursor++; }
    const defaultName = relation === "import" && this.tokens[cursor]?.kind === "identifier" ? cursor : -1;
    if (defaultName >= 0) cursor = this.text(cursor + 1) === "," ? cursor + 2 : stop;
    const close = this.text(cursor) === "{" ? this.close(cursor, stop) : -1, module = close < 0 ? undefined : this.tokens[close + 2];
    if (close < 0 || this.text(close + 1) !== "from" || module?.kind !== "string" || close + 3 !== stop) { this.diagnose(relation === "import" ? "unsupported-default-or-namespace-import" : "unsupported-export-star", start, next); return next; }
    if (defaultName >= 0) this.diagnose("unsupported-default-or-namespace-import", defaultName, defaultName + 1);
    for (const [first, last] of this.segments(cursor + 1, close, ",")) {
      let name = first, elementTypeOnly = false;
      if (this.text(name) === "type" && name + 1 < last && this.text(name + 1) !== "as") { elementTypeOnly = true; name++; }
      const local = this.text(name + 1) === "as" ? name + 2 : name;
      if (!["identifier", "string"].includes(this.tokens[name]?.kind) || this.tokens[local]?.kind !== "identifier" || local + 1 !== last) { this.diagnose(relation === "import" ? "unsupported-default-or-namespace-import" : "unsupported-export-star", first, last); continue; }
      this.aliases.push({ relation, typeOnly: typeOnly || elementTypeOnly, imported: this.tokens[name].value, local: this.tokens[local].value, moduleSpecifier: module.value, modulePath, span: this.span(first, last) });
    }
    return next;
  }

  private variables(start: number, keyword: number, end: number, modulePath: readonly string[], exported: boolean): number {
    let cursor = keyword + 1;
    while (cursor < end) {
      const stop = this.ending(cursor, end, true), name = this.tokens[cursor], equals = this.top(cursor, stop, ["="], true)[0];
      if (stop <= cursor) throw new TypeScriptDeclarationRecovery(name?.start ?? this.source.length);
      if (name.kind !== "identifier") this.diagnose("unsupported-binding-pattern", cursor, stop);
      else {
        let header = cursor + 1;
        if (this.text(header) === "!") header++;
        if (this.text(header) === ":") header = this.typeEnd(header + 1, equals ?? stop);
        if (header !== (equals ?? stop)) this.invalid(header);
        let shape: TypeScriptDeclarationStructure;
        if (equals === undefined) shape = { form: "unresolved", members: [], unresolved: "initializer:absent" };
        else if (equals + 1 >= stop) { shape = { form: "unresolved", members: [], unresolved: "initializer:absent" }; this.parseErrors.push({ code: "parse-error", span: this.span(stop, Math.min(stop + 1, end)) }); }
        else shape = this.expression(equals + 1, stop);
        const colon = this.top(cursor + 1, equals ?? stop, [":"], true)[0];
        if (colon !== undefined) this.typeDiagnostics(colon + 1, equals ?? stop, false, true);
        this.declarations.push({ kind: "variable", name: name.value, exported, modulePath, span: this.span(cursor, stop), structure: shape });
        if (["initializer:absent", "initializer:expression", "initializer:template-interpolation"].includes(shape.unresolved ?? "")) this.diagnose("unresolved-expression", cursor, stop);
      }
      if (this.text(stop) !== ",") return stop + (this.text(stop) === ";" ? 1 : 0);
      cursor = stop + 1;
    }
    return cursor;
  }

  private bodyOpen(start: number, end: number): number {
    for (let index = start; index < end; index++) {
      if (this.text(index) === "{" || this.text(index) === ";") return index;
      if (["(", "["].includes(this.text(index))) index = this.close(index, end);
      else if (this.text(index) === "<") index = this.angleEnd(index, end);
    }
    return end;
  }

  parse(start = 0, end = this.tokens.length, modulePath: readonly string[] = []): void {
    for (let cursor = start; cursor < end;) {
      if (this.text(cursor) === ";") { cursor++; continue; }
      const first = cursor;
      if (this.text(cursor) === "import" || this.text(cursor) === "export" && (["{", "*"].includes(this.text(cursor + 1)) || this.text(cursor + 1) === "type" && ["{", "*"].includes(this.text(cursor + 2)))) { cursor = this.importOrExport(cursor, end, modulePath); continue; }
      let exported = false;
      while (["export", "default", "declare", "abstract", "async"].includes(this.text(cursor))) { exported ||= this.text(cursor) === "export"; cursor++; }
      let keyword = this.text(cursor);
      if (keyword === "const" && this.text(cursor + 1) === "enum") { cursor++; keyword = "enum"; }
      if (["namespace", "module", "global"].includes(keyword)) {
        const names: string[] = [];
        let index = keyword === "global" ? cursor : cursor + 1;
        if (!["identifier", "string"].includes(this.tokens[index]?.kind)) throw new TypeScriptDeclarationRecovery(this.tokens[first].start);
        names.push(this.raw(index, index + 1)); index++;
        while (this.text(index) === "." && this.tokens[index + 1]?.kind === "identifier") { names.push(this.raw(index + 1, index + 2)); index += 2; }
        if (this.text(index) !== "{") { const stop = this.ending(index, end), next = stop + (this.text(stop) === ";" ? 1 : 0); this.diagnose("unsupported-ambient-module-body", first, next); cursor = next; continue; }
        const close = this.close(index, end); this.parse(index + 1, close, [...modulePath, ...names]); cursor = close + 1; continue;
      }
      if (["const", "let", "var"].includes(keyword)) { cursor = this.variables(first, cursor, end, modulePath, exported); continue; }
      if (keyword === "type" && this.tokens[cursor + 1]?.kind === "identifier") {
        const name = this.tokens[cursor + 1];
        let equals = cursor + 2;
        if (this.text(equals) === "<") equals = this.typeParameters(equals, end);
        if (this.text(equals) !== "=") throw new TypeScriptDeclarationRecovery(this.tokens[first].start);
        const stop = this.ending(equals + 1, end), next = stop + (this.text(stop) === ";" ? 1 : 0);
        if (equals + 1 < stop) { const consumed = this.typeEnd(equals + 1, stop); if (consumed !== stop) this.invalid(consumed); }
        const shape = this.structure(equals + 1, stop);
        this.declarations.push({ kind: "type", name: name.value, exported, modulePath, span: this.span(first, next), structure: shape });
        if (equals + 1 === stop) this.parseErrors.push({ code: "parse-error", span: this.span(stop, Math.min(stop + 1, end)) });
        else this.typeDiagnostics(equals + 1, stop);
        cursor = next; continue;
      }
      if (["interface", "enum", "class"].includes(keyword)) {
        const name = this.tokens[cursor + 1], named = name?.kind === "identifier" && !["extends", "implements"].includes(name.text), open = this.bodyOpen(cursor + (named ? 2 : 1), end);
        if (open >= end || this.text(open) !== "{") throw new TypeScriptDeclarationRecovery(this.tokens[first].start);
        const close = this.close(open, end);
        if (!named) { this.diagnose(keyword === "class" ? "unsupported-anonymous-default-class" : "unsupported-module-statement", first, close + 1); cursor = close + 1; continue; }
        let header = cursor + 2;
        if (this.text(header) === "<" && keyword !== "enum") header = this.typeParameters(header, open);
        if (header !== open && !["extends", "implements"].includes(this.text(header))) this.invalid(header);
        if (header !== open && (keyword === "enum" || header + 1 === open)) this.invalid(header);
        const clauses = this.top(cursor + 2, open, ["extends", "implements"], true);
        for (let index = 0; index < clauses.length; index++) this.diagnose("unresolved-heritage", clauses[index], clauses[index + 1] ?? open);
        const members = this.members(open + 1, close, keyword === "interface" ? "type" : keyword as "class" | "enum");
        for (const [left, right] of members.bodies) this.diagnose("unsupported-class-member-body", left, right);
        if (members.computed) this.diagnose("unresolved-computed-property", first, close + 1);
        if (keyword !== "enum") for (const [left, right] of members.annotations) this.typeDiagnostics(left, right, false, true);
        const unresolved = members.bodies.length ? "class-member-body" : clauses.length ? "heritage" : members.computed ? "computed-property" : null;
        this.declarations.push({ kind: keyword as "interface" | "enum" | "class", name: name.value, exported, modulePath, span: this.span(first, close + 1), structure: { form: keyword === "interface" ? "object" : keyword as "enum" | "class", members: members.names, unresolved } });
        cursor = close + 1; continue;
      }
      if (keyword === "function") {
        const open = this.bodyOpen(cursor + 1, end), next = this.text(open) === "{" ? this.close(open, end) + 1 : open + (this.text(open) === ";" ? 1 : 0);
        if (next <= first) throw new TypeScriptDeclarationRecovery(this.tokens[first].start);
        this.diagnose("unsupported-function-local", first, next); cursor = next; continue;
      }
      const stop = this.ending(cursor, end), next = stop + (this.text(stop) === ";" ? 1 : 0);
      if (next <= first) throw new TypeScriptDeclarationRecovery(this.tokens[first].start);
      this.diagnose("unsupported-module-statement", first, next); cursor = next;
    }
  }

  result(): TypeScriptDeclarationFacts {
    const diagnostics = [...this.diagnostics.sort((left, right) => left.span.start - right.span.start), ...this.parseErrors];
    return { completeness: diagnostics.length ? "incomplete" : "complete", declarations: this.declarations, aliases: this.aliases, diagnostics };
  }
}

/** 🧭️ Inspects exact source syntax without IO, compiler dependencies, evaluation or provider inference. */
export function inspectTypeScriptDeclarationFacts(source: string, language: "ts" | "tsx"): TypeScriptDeclarationFacts {
  if (language !== "ts" && language !== "tsx") throw new TypeError("TypeScript declaration facts require an explicit ts or tsx language");
  if (typeof source !== "string") throw new TypeError("TypeScript declaration facts require a source string");
  const scanner = new TypeScriptDeclarationScanner(source, language);
  let parser: TypeScriptDeclarationParser | undefined;
  try {
    scanner.scan();
    parser = new TypeScriptDeclarationParser(source, scanner.tokens, scanner.pairs);
    parser.parse();
    return parser.result();
  } catch (error) {
    if (!(error instanceof TypeScriptDeclarationRecovery) && !(error instanceof TypeScriptDeclarationSyntaxError) && !(error instanceof RangeError)) throw error;
    if (!parser) parser = new TypeScriptDeclarationParser(source, [], new Map());
    if (error instanceof TypeScriptDeclarationSyntaxError) parser.parseErrors.push({ code: "parse-error", span: error.span });
    else parser.diagnostics.push({ code: "unsupported-recovery-suffix", span: { start: error instanceof TypeScriptDeclarationRecovery ? error.start : 0, end: source.length } });
    return parser.result();
  }
}
//#endregion 🟦️TypeScriptDeclarationFacts

//#region 🧭️Discovery
/** 🎭️ Package "kind" declared by the ecosystem's role marker — see `readSemioMarker` and `taxonomy.roles`. */
export type PackageRole = "plugin" | "framework" | "product" | "hub" | "s-module" | "extension" | "testkit" | "tool";

/** 🌐️ Ecosystem a discovered package's manifest belongs to (`taxonomy.langs`). */
export type PackageLang = "🦀️rust" | "🟦️typescript" | "🟨️javascript" | "🐹️go" | "🐍️python" | "🔷️dotnet";

/** 🎯️ A lang's render/build target when the package sits under `🎯️targets/<target>/` (three-level shape) — open vocabulary, e.g. `"⚛️react"`, `"🧊️wgpu"`, `"⌨️tui"`. */
export type PackageTarget = string;

/** 📦️ One package discovered under `<owner>/📦️packages/<lang>/` (two-level, e.g. plugins/styling) or `<owner>/📦️packages/<lang>/🎯️targets/<target>/` (three-level, e.g. ui/renderer-engine), with its role/id marker resolved. */
export interface DiscoveredPackage {
  readonly ownerRel: string;
  readonly lang: PackageLang;
  /** 🎯️ Set only for the three-level shape; `undefined` for a direct `📦️packages/<lang>/` package. */
  readonly target?: PackageTarget;
  /** 📁️ Repo-relative dir holding the manifest (i.e. `dirname(manifestPath)`). */
  readonly packageRel: string;
  readonly manifestPath: string;
  readonly role: PackageRole;
  readonly id: string;
  /** 🗺️ Declared state of the enclosing area (`taxonomy.areas`), `""` outside every declared area. */
  readonly area: string;
  /** 📈️ Derived state of this package's OWNER: `mixed` while the owner still carries forbidden-segment dirs or an owner-root entry file, else `clean`. */
  readonly maturity: PackageMaturity;
}

/** 🏠️ One owner (a dir carrying a `📦️packages` folder) with every package it ships plus its derived migration state. */
export interface DiscoveredOwner {
  readonly ownerRel: string;
  readonly area: string;
  readonly maturity: PackageMaturity;
  readonly langs: readonly PackageLang[];
  readonly targets: readonly PackageTarget[];
  readonly roles: readonly PackageRole[];
  readonly packages: readonly DiscoveredPackage[];
  /** 🔥️ Residual `⚡️implementations`/`⚡️implementation` dirs still inside this owner (burn-down counter — must decrease monotonically to 0). */
  readonly residualImplDirs: number;
  /** 🚪️ Shape V1 leftovers: entry files still sitting at the owner root instead of in `📦️packages/<lang>/`. */
  readonly entryFilesAtOwnerRoot: readonly string[];
}

/** ⚠️ A discovery-time problem `discoverPackages` does not fail on but must not stay silent about — see `discoverPackageProblems`. */
export interface DiscoveryProblem {
  readonly kind: "ambiguous-lang-shape" | "target-without-manifest" | "manifest-without-marker" | "unknown-lang" | "unknown-role" | "packaging-violation" | "package-role-unresolved" | "package-implementation";
  readonly path: string;
  readonly message: string;
}

/** 🔥️ Migration burn-down snapshot: everything that still has to shrink to zero before the finalization flip, derived from disk on every call (no hand-maintained lists). */
export interface DiscoveryBurndown {
  readonly ownersTotal: number;
  readonly packagesTotal: number;
  readonly cleanOwners: number;
  readonly mixedOwners: readonly DiscoveredOwner[];
  /** 🔥️ Every forbidden-segment dir repo-wide, including those outside any migrated owner. */
  readonly implDirsTotal: number;
  readonly implDirsByArea: Readonly<Record<string, number>>;
  /** 🏷️ `📦️packages/<lang>/` manifests carrying no role marker — invisible to `discoverPackages` until they get one. */
  readonly unmarkedManifests: readonly { readonly path: string; readonly area: string }[];
  /** 📦️ Files inside a package dir that are neither packaging code nor its entry file (data/docs belong at the owner root under Shape V2). */
  readonly packagingViolations: readonly { readonly path: string; readonly ownerRel: string }[];
}

/** 🔍️ Extracts a dotted TOML table's body from manifest text (stops at the next `[…]` header) — simple line-scan, no TOML parser dependency (none is a repo dependency for the TS side; mirrors `hasSemioRole` in framework/os/plugin/registry script.ts). */
function tomlTableBody(text: string, table: string): string | undefined {
  const header = `[${table}]`;
  const lines = text.split("\n");
  const start = lines.findIndex((line) => line.trim() === header);
  if (start === -1) return undefined;
  const body: string[] = [];
  for (let i = start + 1; i < lines.length; i++) {
    if (lines[i].trim().startsWith("[")) break;
    body.push(lines[i]);
  }
  return body.join("\n");
}

/** 🔍️ Walks a dotted key path (`"metadata.semio"`, `"semio"`) into a parsed JSON manifest. */
function jsonTable(value: unknown, table: string): Record<string, unknown> | undefined {
  let current: unknown = value;
  for (const key of table.split(".")) {
    if (typeof current !== "object" || current === null) return undefined;
    current = (current as Record<string, unknown>)[key];
  }
  return typeof current === "object" && current !== null ? (current as Record<string, unknown>) : undefined;
}

/** 🏷️ Package name from a Cargo.toml's `[package] name = "…"` line. */
function rustPackageName(text: string): string | undefined {
  return text.match(/^name\s*=\s*"([^"]+)"/m)?.[1];
}

/**
 * 🏷️ Reads a package's role marker as declared by its ecosystem (`taxonomy.ecosystems.<lang>.marker`):
 * rust `[package.metadata.semio]`, typescript `package.json` `"semio"`, go `📋️project.json`
 * `metadata.semio`, python `pyproject.toml` `[tool.semio]`. `undefined` when the manifest is missing, the
 * ecosystem is discovery-opaque (no marker spec, e.g. dotnet), or no `role` is declared.
 */
export function readSemioMarker(manifestPath: string, lang: PackageLang, taxonomy: Taxonomy = loadTaxonomy()): { role: PackageRole; id?: string } | undefined {
  return existsSync(manifestPath) ? parseSemioMarker(readFileSync(manifestPath, "utf8"), lang, taxonomy) : undefined;
}

/** 🏷️ Resolves role metadata from supplied bytes, including transaction preimage views. */
function parseSemioMarker(content: string, lang: PackageLang, taxonomy: Taxonomy): { role: PackageRole; id?: string } | undefined {
  const spec = taxonomy.ecosystems[lang]?.marker;
  if (!spec) return undefined;
  let role: string | undefined;
  let id: string | undefined;
  if (spec.format === "toml") {
    const body = tomlTableBody(content, spec.table);
    if (!body) return undefined;
    role = body.match(new RegExp(`^${spec.roleKey}\\s*=\\s*"([^"]+)"`, "m"))?.[1];
    id = body.match(new RegExp(`^${spec.idKey}\\s*=\\s*"([^"]+)"`, "m"))?.[1];
  } else {
    let parsed: unknown;
    try {
      parsed = JSON.parse(content);
    } catch {
      return undefined;
    }
    const table = jsonTable(parsed, spec.table);
    if (!table) return undefined;
    role = typeof table[spec.roleKey] === "string" ? (table[spec.roleKey] as string) : undefined;
    id = typeof table[spec.idKey] === "string" ? (table[spec.idKey] as string) : undefined;
  }
  if (!role) return undefined;
  return id ? { role: role as PackageRole, id } : { role: role as PackageRole };
}

//#region 🏷️SemioMarkerSubTable
/** 🔧️ Minimal TOML table-body decoder: plain string values (`key = "..."`) and flat string arrays
 * (`key = ["a", "b"]`) — sufficient for opt-in sub-tables (see `readSemioMarkerSubTable`); deliberately
 * NOT a general TOML parser (no numbers/bools/nested-inline-tables/multiline strings — reach for a real
 * TOML dependency if a future consumer needs more than this). */
function tomlTableValues(body: string): Record<string, unknown> {
  const result: Record<string, unknown> = {};
  for (const rawLine of body.split("\n")) {
    const line = rawLine.trim();
    if (!line || line.startsWith("#")) continue;
    const arrayMatch = line.match(/^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*\[([^\]]*)\]\s*$/);
    if (arrayMatch) {
      result[arrayMatch[1]] = [...arrayMatch[2].matchAll(/"((?:[^"\\]|\\.)*)"/g)].map((m) => m[1]);
      continue;
    }
    const scalarMatch = line.match(/^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"((?:[^"\\]|\\.)*)"\s*$/);
    if (scalarMatch) result[scalarMatch[1]] = scalarMatch[2];
  }
  return result;
}

/**
 * 🏷️ Reads an arbitrary OPT-IN sub-table nested under a package's own semio marker table — rust
 * `[package.metadata.semio.<subKey>]`, TS `package.json`'s `"semio": {"<subKey>": {...}}` — the generic
 * mechanism per-concern consumers use to opt a package INTO extra behavior beyond the bare role/id every
 * marked package already declares via `readSemioMarker` (first consumer: Storybook coverage, see
 * `26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`). `undefined` when the
 * manifest/table/sub-key is absent — silence here means "not opted in", never an error. Only ONE table
 * per sub-key is read (no TOML array-of-tables support); a package needing more than one entry, or
 * fields this decoder can't express, is expected to stay hand-curated by its consumer instead.
 */
export function readSemioMarkerSubTable(manifestPath: string, lang: PackageLang, subKey: string, taxonomy: Taxonomy = loadTaxonomy()): Record<string, unknown> | undefined {
  const spec = taxonomy.ecosystems[lang]?.marker;
  if (!spec || !existsSync(manifestPath)) return undefined;
  if (spec.format === "toml") {
    const body = tomlTableBody(readFileSync(manifestPath, "utf8"), `${spec.table}.${subKey}`);
    return body === undefined ? undefined : tomlTableValues(body);
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(readFileSync(manifestPath, "utf8"));
  } catch {
    return undefined;
  }
  return jsonTable(parsed, `${spec.table}.${subKey}`);
}
//#endregion 🏷️SemioMarkerSubTable

const DISCOVERY_SKIP_DIRS = new Set(["node_modules", "target", "dist", "📤️dist", ".git", ".🧬semio", "🤖️generated", "🔌️plugin-modules", "pkg", "storybook-static", "temp", ".venv", "coverage", "__pycache__", "client", "client_bin"]);
const CARGO_TARGET_DIR_PATTERN = /^target(?:-[a-z0-9]+)*$/u;

/** 🎯️ Cargo target roots (`target`, `target-<slug>`) are build output that concurrent lanes create and prune mid-walk; discovery never enters them. */
export function isDiscoverySkipDirectory(name: string): boolean {
  return DISCOVERY_SKIP_DIRS.has(name) || CARGO_TARGET_DIR_PATTERN.test(name);
}

/** 🚫️ Tests a repo-relative path against opaque prefixes without touching the candidate path. */
export function pathIsExcluded(repoRoot: string, candidate: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const rel = relative(resolve(repoRoot), resolve(candidate)).replaceAll("\\", "/").replace(/^\.\//u, "");
  if (rel === ".." || rel.startsWith("../")) return false;
  return Object.values(taxonomy.pathExclusions).some((exclusion) => {
    const prefix = exclusion.path.replace(/^\.\//u, "").replace(/\/+$/u, "");
    return rel === prefix || rel.startsWith(`${prefix}/`);
  });
}

/** 📁️ `readdirSync(dir, { withFileTypes: true })`, defaulting to `[]` for an unreadable/missing dir — a helper (rather than an explicit `ReturnType<typeof readdirSync>` annotation) so the `Dirent<string>` element type infers unambiguously from this specific overload. */
function readdirSafe(absDir: string) {
  try {
    return readdirSync(absDir, { withFileTypes: true });
  } catch {
    return [];
  }
}

/** 🔤️ Drops every non-ASCII codepoint (emoji + variation selectors), e.g. `"✒️writer"` -> `"writer"` — mirrors `policyStripEmoji` in root script.ts. */
function stripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}

/** 🆔️ Falls back to the manifest's own package name, else an emoji-stripped dash-joined owner path, when `readSemioMarker` found no explicit `id`. */
function fallbackPackageId(manifestPath: string, lang: PackageLang, ownerRel: string, content?: string): string {
  try {
    if (lang === "🦀️rust") {
      const name = rustPackageName(content ?? readFileSync(manifestPath, "utf8"));
      if (name) return name;
    } else {
      const name = (JSON.parse(content ?? readFileSync(manifestPath, "utf8")) as { name?: string }).name;
      if (name) return name;
    }
  } catch {
    /* fall through to path-derived id */
  }
  return ownerRel
    .replaceAll("\\", "/")
    .split("/")
    .map(stripEmoji)
    .filter(Boolean)
    .join("-");
}

interface OwnerAccumulator {
  ownerRel: string;
  area: string;
  packages: DiscoveredPackage[];
  residualImplDirs: number;
  entryFilesAtOwnerRoot: string[];
}

interface DiscoveryScan {
  readonly packages: readonly DiscoveredPackage[];
  readonly owners: readonly DiscoveredOwner[];
  readonly problems: readonly DiscoveryProblem[];
  readonly burndown: DiscoveryBurndown;
}

const scanCache = ephemeralMap<string, DiscoveryScan>("framework.products.repo.modules.lib.discovery.component.ts.scanCache");

/** 🧹️ Drops the memoized repo scan — call after mutating the tree inside one process (tests, generators). */
export function clearDiscoveryCache(): void {
  scanCache.clear();
  gitlinkBoundaryCache.clear();
}

//#region 📇️CatalogInputs
/** 🌳️ Logical preimage filesystem used equally by catalog generation and transaction revalidation. */
export interface RegistryCatalogInputView {
  entries(path: string): readonly { readonly name: string; readonly nodeKind: "file" | "directory" | "symlink" }[];
  kind(path: string): "file" | "directory" | "symlink" | null;
  readText(path: string): string;
}

/** 🔮️ Read-only post-operation inputs for one explicitly registered preview owner. */
export interface GeneratorInputProjection<ContractId extends "plugin-registry" | "wgpu-frame-worker" = "plugin-registry" | "wgpu-frame-worker"> {
  readonly contractId: ContractId;
  readonly schemaVersion: 1;
  readonly moves: readonly { readonly sourcePath: string; readonly destinationPath: string; readonly nodeKind: "file" | "symlink" }[];
  readonly edits: readonly { readonly path: string; readonly bytesBase64: string }[];
  readonly removals: readonly string[];
}

/** 📇️ Registry projection keeps its exact producer identity on the shared input grammar. */
export type RegistryCatalogProjection = GeneratorInputProjection<"plugin-registry">;

/** 🔐️ Validates bounded stdin data before any projected path can reach the filesystem. */
export function parseRegistryCatalogProjection(content: string, taxonomy: Taxonomy): RegistryCatalogProjection {
  return parseGeneratorInputProjection(content, taxonomy, "plugin-registry");
}

/** 🛂️ Validates exact producer identity and bounded leaf operations before any filesystem read. */
export function parseGeneratorInputProjection<ContractId extends GeneratorInputProjection["contractId"]>(content: string, taxonomy: Taxonomy, contractId: ContractId): GeneratorInputProjection<ContractId> {
  const authority = contractId === "plugin-registry" ? taxonomy.generatorContracts[contractId]?.inputDiscovery?.previewInput : contractId === "wgpu-frame-worker" ? taxonomy.generatorContracts[contractId]?.packageGeneration?.previewInput : undefined;
  const protocol = contractId === "plugin-registry" ? "registry-projected-inputs-v1" : "package-projected-inputs-v1";
  if (!authority || authority.protocol !== protocol || Buffer.byteLength(content) > authority.maxBytes) throw new Error("Generator projected input payload exceeds its declared authority.");
  const value = JSON.parse(content) as GeneratorInputProjection<ContractId>;
  const keys = (row: unknown, expected: readonly string[]): boolean => Boolean(row && typeof row === "object" && !Array.isArray(row) && Object.keys(row).sort().join("\0") === [...expected].sort().join("\0"));
  const path = (value: unknown): value is string => typeof value === "string" && value.length > 0 && value === value.normalize("NFC") && !value.includes("\\") && !value.includes("\0") && !value.startsWith("/") && !/^[A-Za-z]:/u.test(value) && value.split("/").every((part) => part && part !== "." && part !== "..") && !Object.values(taxonomy.pathExclusions).some((entry) => { const prefix = entry.path.replace(/\/+$/u, ""); return value === prefix || value.startsWith(prefix + "/"); });
  if (!keys(value, ["contractId", "schemaVersion", "moves", "edits", "removals"]) || value.contractId !== contractId || value.schemaVersion !== 1 || !Array.isArray(value.moves) || !Array.isArray(value.edits) || !Array.isArray(value.removals) || value.moves.length + value.edits.length + value.removals.length > authority.maxOperations) throw new Error("Generator projected input payload has unsupported shape or operation count.");
  const sources = new Set<string>(), destinations = new Set<string>(), edits = new Set<string>(), removals = new Set<string>();
  for (const move of value.moves) {
    if (!keys(move, ["sourcePath", "destinationPath", "nodeKind"]) || !path(move.sourcePath) || !path(move.destinationPath) || move.sourcePath === move.destinationPath || !["file", "symlink"].includes(move.nodeKind) || sources.has(move.sourcePath) || destinations.has(move.destinationPath)) throw new Error("Registry projected move is invalid or duplicated.");
    sources.add(move.sourcePath); destinations.add(move.destinationPath);
  }
  for (const edit of value.edits) {
    if (!keys(edit, ["path", "bytesBase64"]) || !path(edit.path) || typeof edit.bytesBase64 !== "string" || Buffer.from(edit.bytesBase64, "base64").toString("base64") !== edit.bytesBase64 || edits.has(edit.path)) throw new Error("Registry projected edit is invalid or duplicated.");
    edits.add(edit.path);
  }
  for (const removal of value.removals) {
    if (!path(removal) || removals.has(removal) || sources.has(removal) || destinations.has(removal)) throw new Error("Registry projected removal is invalid or overlaps a move.");
    removals.add(removal);
  }
  return value;
}

/** 🌳️ Applies only authorized leaf operations and their required ancestor membership in memory. */
export function registryCatalogProjectedInputView(repoRoot: string, taxonomy: Taxonomy, projection: RegistryCatalogProjection, base: RegistryCatalogInputView = registryCatalogInputView(repoRoot, taxonomy)): RegistryCatalogInputView {
  return generatorProjectedInputView(repoRoot, taxonomy, parseRegistryCatalogProjection(JSON.stringify(projection), taxonomy), base);
}

/** 🌲️ Shares no-follow source mapping and projected parent membership across exact preview producers. */
export function generatorProjectedInputView(repoRoot: string, taxonomy: Taxonomy, projection: GeneratorInputProjection, base: RegistryCatalogInputView = registryCatalogInputView(repoRoot, taxonomy)): RegistryCatalogInputView {
  const checked = parseGeneratorInputProjection(JSON.stringify(projection), taxonomy, projection.contractId);
  const sources = new Set(checked.moves.map((move) => move.sourcePath));
  const removed = new Set(checked.removals);
  const moves = new Map(checked.moves.map((move) => [move.destinationPath, move]));
  const edits = new Map(checked.edits.map((edit) => [edit.path, Buffer.from(edit.bytesBase64, "base64").toString("utf8")]));
  const createdDirectories = new Set<string>();
  for (const move of checked.moves) {
    if (base.kind(move.sourcePath) !== move.nodeKind) throw new Error(`Registry projected source drifted: ${move.sourcePath}`);
    if (base.kind(move.destinationPath) !== null && !sources.has(move.destinationPath)) throw new Error(`Registry projected destination is occupied: ${move.destinationPath}`);
    for (let parent = dirname(move.destinationPath); parent !== "."; parent = dirname(parent)) {
      const kind = base.kind(parent);
      if (kind && kind !== "directory") throw new Error(`Registry projected ancestor is not a directory: ${parent}`);
      if (!kind) createdDirectories.add(parent);
    }
  }
  for (const path of removed) if (base.kind(path) === null) throw new Error(`Registry projected removal drifted: ${path}`);
  const kind = (path: string): "file" | "directory" | "symlink" | null => {
    if (moves.has(path)) return moves.get(path)!.nodeKind;
    if (sources.has(path) || removed.has(path)) return null;
    if (createdDirectories.has(path)) return "directory";
    return base.kind(path);
  };
  for (const path of edits.keys()) if (kind(path) !== "file") throw new Error(`Registry projected edit is not a regular post-state file: ${path}`);
  return {
    kind,
    entries(path) {
      if (kind(path) !== "directory") return [];
      const rows = new Map<string, "file" | "directory" | "symlink">();
      if (!createdDirectories.has(path)) for (const entry of base.entries(path)) {
        const child = path ? `${path}/${entry.name}` : entry.name;
        const childKind = kind(child);
        if (childKind) rows.set(entry.name, childKind);
      }
      for (const child of [...moves.keys(), ...createdDirectories]) if ((dirname(child) === "." ? "" : dirname(child)) === path) rows.set(basename(child), kind(child)!);
      return [...rows].map(([name, nodeKind]) => ({ name, nodeKind }));
    },
    readText(path) {
      if (kind(path) !== "file") throw new Error(`Registry projected content is missing or a symlink: ${path}`);
      return edits.get(path) ?? base.readText(moves.get(path)?.sourcePath ?? path);
    },
  };
}

/** 🎯️ Conservative lexical admission before the expensive shared catalog membership scan. */
export function registryCatalogPathMayAffect(path: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const authority = taxonomy.generatorContracts["plugin-registry"]?.inputDiscovery;
  if (!authority || Object.values(taxonomy.pathExclusions).some((entry) => { const prefix = entry.path.replace(/\/+$/u, ""); return path === prefix || path.startsWith(prefix + "/"); })) return false;
  const roots = [...authority.implementationEntryPaths, ...Object.values(authority.workspaceImports).flatMap((entry) => [entry.entryPath, entry.manifestPath])].map((path) => path.split("/")[0]);
  if (roots.includes(path.split("/")[0])) return true;
  return !path.split("/").some((segment) => segment.startsWith(".") || isDiscoverySkipDirectory(segment));
}

const gitlinkBoundaryCache = ephemeralMap<string, ReadonlySet<string>>("framework.products.repo.modules.lib.discovery.component.ts.gitlinkBoundaryCache");

/** 🧷️ Stage-zero `160000` index entries are retained terminal repository boundaries — leaves, never descended. */
function registryCatalogGitlinkBoundaries(repoRoot: string): ReadonlySet<string> {
  const cached = gitlinkBoundaryCache.get(repoRoot);
  if (cached) return cached;
  let boundaries: ReadonlySet<string>;
  try {
    const stdout = execFileSync("git", ["ls-files", "--stage", "-z"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 256 * 1024 * 1024 });
    const found = new Set<string>();
    for (const row of stdout.split("\0")) {
      if (!row) continue;
      const tab = row.indexOf("\t");
      if (tab < 0) continue;
      const [mode, , stage] = row.slice(0, tab).split(" ");
      if (mode === "160000" && stage === "0") found.add(row.slice(tab + 1).normalize("NFC"));
    }
    boundaries = found;
  } catch { boundaries = new Set(); }
  gitlinkBoundaryCache.set(repoRoot, boundaries);
  return boundaries;
}

/** 🛡️ Creates one no-follow, opaque-first catalog filesystem view. */
export function registryCatalogInputView(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): RegistryCatalogInputView {
  const kinds = new Map<string, "file" | "directory" | "symlink" | null>();
  const gitlinkBoundaries = registryCatalogGitlinkBoundaries(repoRoot);
  const checked = (path: string): string => {
    const absolute = resolve(repoRoot, path || ".");
    const normalized = relative(repoRoot, absolute).replaceAll("\\", "/");
    if (normalized !== path || pathIsExcluded(repoRoot, absolute, taxonomy)) throw new Error(`Registry catalog input is outside its nonopaque owner: ${path}`);
    return absolute;
  };
  const kind = (path: string): "file" | "directory" | "symlink" | null => {
    const absolute = checked(path);
    if (kinds.has(path)) return kinds.get(path)!;
    const parent = dirname(path) === "." ? "" : dirname(path);
    if (path && parent !== path) {
      const parentKind = kind(parent);
      if (parentKind === "symlink") throw new Error(`Registry catalog ancestor is a symlink: ${parent}`);
      if (parentKind !== "directory") return null;
    }
    let value: "file" | "directory" | "symlink" | null;
    try {
      const stat = lstatSync(absolute);
      value = stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : stat.isFile() ? "file" : null;
      if (value === null) throw new Error(`Registry catalog input is not a regular node: ${path}`);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
      value = null;
    }
    kinds.set(path, value);
    return value;
  };
  return {
    kind,
    entries(path) {
      const absolute = checked(path);
      const nodeKind = kind(path);
      if (nodeKind === null) return [];
      if (nodeKind !== "directory") throw new Error(`Registry catalog directory is ${nodeKind}: ${path}`);
      if (gitlinkBoundaries.has(path.normalize("NFC"))) return [];
      return readdirSync(absolute).filter((name) => !isDiscoverySkipDirectory(name) && !pathIsExcluded(repoRoot, join(absolute, name), taxonomy)).map((name) => {
        const childPath = relative(repoRoot, join(absolute, name)).replaceAll("\\", "/"), childKind = kind(childPath);
        if (childKind === null) throw new Error(`Registry catalog enumerated input disappeared: ${childPath}`);
        return { name, nodeKind: childKind };
      });
    },
    readText(path) {
      if (kind(path) !== "file") throw new Error(`Registry catalog content input is missing or a symlink: ${path}`);
      return readFileSync(checked(path), "utf8");
    },
  };
}

/** 📦️ Catalog discovery deliberately omits source-role, cache-tag and package-purity diagnostics. */
export function discoverCatalogPackages(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy(), view: RegistryCatalogInputView = registryCatalogInputView(repoRoot, taxonomy)): DiscoveredPackage[] {
  return [...scanRepo(repoRoot, taxonomy, { view, inputs: new Set<string>() }).packages];
}

/** 📚️ Example IDs are directory membership with one schema-owned primary Rust leaf. */
export function registryExampleCatalog(repoRoot: string, cratePath: string, taxonomy: Taxonomy = loadTaxonomy(), view: RegistryCatalogInputView = registryCatalogInputView(repoRoot, taxonomy), inputs = new Set<string>()): string[] {
  const authority = taxonomy.generatorContracts["plugin-registry"]?.inputDiscovery;
  if (!authority || authority.kind !== "registry-catalog") throw new Error("Registry catalog input discovery is not declared.");
  const area = taxonomy.pluginAreas.find((area) => cratePath.startsWith(area + "/"));
  const pluginRoot = area ? cratePath.split("/").slice(0, area.split("/").length + 1).join("/") : cratePath.split("/")[0]!;
  const leaf = canonicalPrimaryFilenameForKind(authority.exampleFileKindId, taxonomy);
  const exists = (path: string): boolean => {
    const kind = view.kind(path);
    if (kind === "symlink") throw new Error(`Registry catalog example input is a symlink: ${path}`);
    if (kind) inputs.add(path);
    return kind !== null;
  };
  const dirs = (path: string): string[] => exists(path) ? view.entries(path).filter((entry) => entry.nodeKind === "directory").map((entry) => entry.name) : [];
  const ids = new Set<string>();
  const slugPattern = new RegExp(taxonomy.exampleSlugPattern, "u");
  const slugs = (path: string): void => {
    for (const slug of dirs(path)) if (slugPattern.test(slug) && !taxonomy.forbiddenExampleSlugs.includes(slug) && exists(`${path}/${slug}/${leaf}`)) ids.add(slug);
  };
  const artifacts = `${pluginRoot}/${taxonomy.artifactsDirName}`;
  for (const artifact of dirs(artifacts)) {
    const artifactPath = `${artifacts}/${artifact}`;
    slugs(`${artifactPath}/${authority.exampleDirectoryName}`);
    const standards = `${artifactPath}/${taxonomy.standardsDirName}`;
    for (const standard of dirs(standards)) {
      const subsets = `${standards}/${standard}/${taxonomy.subsetsDirName}`;
      for (const subset of dirs(subsets)) for (const role of taxonomy.surfaceRoles) {
        const surface = `${subsets}/${subset}/${taxonomy.surfaceDirNames[role]}`;
        if (exists(surface)) slugs(`${surface}/${authority.exampleDirectoryName}`);
      }
    }
  }
  return [...ids].sort();
}

//#region 🔗️RegistryCompilerImports
export interface RegistryCompilerImport {
  readonly path: string;
  readonly kind: string;
}

export type RegistryCompilerLanguage = "ts" | "tsx" | "js" | "jsx";

/** 🔎️ Validates Bun's runtime compiler capability and returns owned import records. */
export function scanRegistryCompilerImports(source: string, language: RegistryCompilerLanguage, platform: unknown = Reflect.get(globalThis, "Bun")): readonly RegistryCompilerImport[] {
  if (language !== "ts" && language !== "tsx" && language !== "js" && language !== "jsx") throw new Error("Registry import discovery requires an explicit supported compiler language.");
  if (platform === null || typeof platform !== "object" && typeof platform !== "function") throw new Error("Registry import discovery requires Bun's compiler runtime.");
  const constructor: unknown = Reflect.get(platform, "Transpiler");
  if (typeof constructor !== "function") throw new Error("Registry import discovery requires Bun.Transpiler.");
  const compiler: unknown = Reflect.construct(constructor, [{ loader: language }]);
  if (compiler === null || typeof compiler !== "object") throw new Error("Bun.Transpiler returned an invalid compiler.");
  const scan: unknown = Reflect.get(compiler, "scanImports");
  if (typeof scan !== "function") throw new Error("Bun.Transpiler does not expose scanImports.");
  const imports: unknown = Reflect.apply(scan, compiler, [source]);
  if (!Array.isArray(imports)) throw new Error("Bun.Transpiler returned invalid import records.");
  return imports.map((entry: unknown) => {
    if (entry === null || typeof entry !== "object") throw new Error("Bun.Transpiler returned an invalid import record.");
    const path: unknown = Reflect.get(entry, "path"), kind: unknown = Reflect.get(entry, "kind");
    if (typeof path !== "string" || typeof kind !== "string") throw new Error("Bun.Transpiler import records require string path and kind.");
    return { path, kind };
  });
}

/** 🔗️ Scans the physical leaf's language without fallback, preserving Unicode source specifiers. */
export function registryStaticImports(source: string, sourcePath: string): readonly string[] {
  const extension = extname(sourcePath);
  const language: RegistryCompilerLanguage | undefined = extension === ".ts" || extension === ".mts" || extension === ".cts" ? "ts" : extension === ".tsx" ? "tsx" : extension === ".js" || extension === ".mjs" || extension === ".cjs" ? "js" : extension === ".jsx" ? "jsx" : undefined;
  if (!language) throw new Error(`Registry implementation language is not supported: ${sourcePath}`);
  let imports: readonly RegistryCompilerImport[];
  try { imports = scanRegistryCompilerImports(source.replace(/^#![^\n]*(?:\n|$)/u, "\n"), language); }
  catch (error) { throw new Error(`Registry compiler import scan failed for ${sourcePath}: ${error instanceof Error ? error.message : String(error)}`); }
  return [...new Set(imports.filter((entry) => entry.kind === "import-statement").map((entry) => {
    const decoded = Buffer.from(entry.path, "latin1").toString("utf8");
    return decoded !== entry.path && !decoded.includes("\ufffd") && source.includes(decoded) ? decoded : entry.path;
  }))].sort();
}

export type RegistryCompilerInputRole = "implementation-entry" | "static-import";

export interface RegistryCompilerInputDependencies {
  readonly kind: "module" | "json-data";
  readonly imports: readonly string[];
}

/** 🧾️ Admits strict JSON only as imported data; compiler entries retain their exact module grammar. */
export function registryCompilerInputDependencies(source: string, sourcePath: string, role: RegistryCompilerInputRole): RegistryCompilerInputDependencies {
  if (role !== "implementation-entry" && role !== "static-import") throw new Error(`Registry compiler input role is not supported: ${String(role)}`);
  if (role === "static-import" && extname(sourcePath) === ".json") {
    try { JSON.parse(source); }
    catch (error) { throw new Error(`Registry imported JSON is invalid: ${sourcePath}: ${error instanceof Error ? error.message : String(error)}`); }
    return { kind: "json-data", imports: [] };
  }
  return { kind: "module", imports: registryStaticImports(source, sourcePath) };
}
//#endregion 🔗️RegistryCompilerImports

/** 📇️ Exact content inputs plus positive membership witnesses; ignored files are not excluded. */
export function registryCatalogInputPaths(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy(), view: RegistryCatalogInputView = registryCatalogInputView(repoRoot, taxonomy)): readonly string[] {
  const authority = taxonomy.generatorContracts["plugin-registry"]?.inputDiscovery;
  if (!authority || authority.kind !== "registry-catalog") throw new Error("Registry catalog input discovery is not declared.");
  const inputs = new Set<string>();
  const packages = scanRepo(repoRoot, taxonomy, { view, inputs }).packages;
  for (const pkg of packages) if (pkg.lang === "🦀️rust" && (pkg.role === "plugin" || pkg.role === "extension")) {
    const descriptor = relative(repoRoot, resolve(repoRoot, pkg.packageRel, authority.descriptorRelativePath)).replaceAll("\\", "/");
    const kind = view.kind(descriptor);
    if (kind === "symlink") throw new Error(`Registry descriptor is a symlink: ${descriptor}`);
    if (kind) inputs.add(descriptor);
    registryExampleCatalog(repoRoot, pkg.packageRel, taxonomy, view, inputs);
  }
  const visited = new Map<string, { source: string; kind: RegistryCompilerInputDependencies["kind"] }>();
  const visit = (path: string, role: RegistryCompilerInputRole): void => {
    const previous = visited.get(path);
    if (previous) {
      if (role === "implementation-entry" && previous.kind === "json-data") registryCompilerInputDependencies(previous.source, path, role);
      return;
    }
    const source = view.readText(path);
    const dependencies = registryCompilerInputDependencies(source, path, role);
    visited.set(path, { source, kind: dependencies.kind });
    inputs.add(path);
    for (const specifier of dependencies.imports) {
      if (specifier.startsWith("node:") || specifier.startsWith("bun:")) continue;
      if (specifier.startsWith(".")) {
        visit(relative(repoRoot, resolve(repoRoot, dirname(path), specifier)).replaceAll("\\", "/"), "static-import");
        continue;
      }
      const binding = authority.workspaceImports[specifier];
      if (!binding) throw new Error(`Registry implementation import is not schema-owned: ${specifier} in ${path}`);
      const manifest = JSON.parse(view.readText(binding.manifestPath)) as { name?: string; exports?: Record<string, unknown> };
      if (manifest.name !== specifier || manifest.exports?.["."] !== "./" + relative(dirname(binding.manifestPath), binding.entryPath).replaceAll("\\", "/")) throw new Error(`Registry workspace import binding drifted: ${specifier}`);
      inputs.add(binding.manifestPath);
      visit(binding.entryPath, "implementation-entry");
    }
  };
  for (const path of authority.implementationEntryPaths) visit(path, "implementation-entry");
  return [...inputs].filter(Boolean).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
}
//#endregion 📇️CatalogInputs

export type PackageSourceRole = "declaration" | "registration" | "bootstrap" | "thin-delegation" | "tool-metadata" | "implementation" | "unresolved";

/** 🧪️ Conservatively classifies one source leaf against its schema-selected package grammar. */
export function classifyPackageSourceRole(content: string, grammar: PackageGlueGrammarSpec): PackageSourceRole {
  const source = content.replace(/\/\*[\s\S]*?\*\//gu, "").replace(/(^|\s)\/\/.*$/gmu, "$1").replace(/(^|\s)#(?!\[).*$/gmu, "$1").trim();
  if (!source) return "declaration";
  if (grammar.analyzer === "rust") {
    if (/\b(?:struct|enum|trait|impl|const|static|fn)\b|\bmacro_rules\s*!/u.test(source)) return "implementation";
    const rest = source.replace(/#!?\[[^\]]*\]/gu, "").replace(/(?:pub\s+)?(?:use|mod)\s+[^;{}]+[;{]/gu, "").replace(/\bextern\s+crate\s+[^;]+;/gu, "").replace(/\binclude!?\s*\([^;]+;/gu, "").replace(/[{};]/gu, "").trim();
    if (/^(?:[A-Za-z_]\w*::)*[A-Za-z_]\w*!\s*\([^{}]*\)$/u.test(rest)) return "registration";
    return rest ? "unresolved" : "declaration";
  }
  if (grammar.analyzer === "typescript" || grammar.analyzer === "javascript") {
    if (/\b(?:class|interface|type|enum|function|namespace)\b/u.test(source) || /\b(?:const|let|var)\s+\w+\s*=\s*(?!await\s+import\b)/u.test(source)) return "implementation";
    const rest = source.replace(/(?:^|\n)\s*(?:import|export)\b[^;]*(?:;|$)/gu, "\n").trim();
    if (!rest) return "declaration";
    const calls = rest.split(";").map((part) => part.trim()).filter(Boolean);
    if (calls.length <= grammar.maxDelegationStatements && calls.every((call) => /^(?:await\s+)?(?:register|mount|bootstrap|start|run|main|[A-Za-z_$][\w$]*\.[A-Za-z_$][\w$]*)\s*\([^{}]*\)$/u.test(call))) return /\b(?:register|mount)\b/u.test(rest) ? "registration" : "thin-delegation";
    return "unresolved";
  }
  if (grammar.analyzer === "go") {
    if (/\btype\s+\w+\s+(?:struct|interface)\b/u.test(source) || /\bfunc\s+(?!main\s*\()/u.test(source)) return "implementation";
    const rest = source.replace(/^package\s+\w+/mu, "").replace(/import\s*(?:\([^)]*\)|"[^"]+")/gu, "").trim();
    if (!rest) return "declaration";
    return /^func\s+main\s*\(\s*\)\s*\{\s*[\w.]+\([^{}]*\)\s*\}\s*$/u.test(rest) ? "bootstrap" : "unresolved";
  }
  if (grammar.analyzer === "python") {
    if (/^(?:async\s+)?def\s|^class\s/mu.test(source)) return "implementation";
    const rest = source.replace(/^(?:from\s+\S+\s+import\s+.+|import\s+.+|__all__\s*=\s*\[[^\]]*\])$/gmu, "").trim();
    if (!rest) return "declaration";
    const calls = rest.split("\n").map((line) => line.trim()).filter(Boolean);
    return calls.length <= grammar.maxDelegationStatements && calls.every((line) => /^(?:register|mount|bootstrap|start|run|main|[A-Za-z_]\w*(?:\.[A-Za-z_]\w*)+)\([^:]*\)$/u.test(line)) ? "thin-delegation" : "unresolved";
  }
  if (grammar.analyzer === "c-cpp") {
    if (/\b(?:class|struct|union|enum)\s+\w+[^;{]*\{/u.test(source)) return "implementation";
    const withoutDirectives = source.replace(/^\s*#\s*(?:include|pragma|define|if|ifdef|ifndef|elif|else|endif)\b.*$/gmu, "").trim();
    const functionBodies = [...withoutDirectives.matchAll(/(?:^|[;}])\s*(?:extern\s+"C"\s+)?[\w:<>,*&\s]+\s+\w+\s*\([^;{}]*\)\s*\{([^{}]*)\}/gu)];
    if (functionBodies.length > 0) {
      const delegated = functionBodies.length <= grammar.maxDelegationStatements && functionBodies.every((match) => /^(?:\s*(?:return\s+)?[A-Za-z_]\w*(?:::\w+)*(?:\.\w+)?\([^;{}]*\)\s*;\s*)$/u.test(match[1] ?? ""));
      return delegated ? "thin-delegation" : "implementation";
    }
    const rest = withoutDirectives
      .replace(/extern\s+"C"\s*\{/gu, "")
      .replace(/\b(?:using\s+[^;]+|typedef\s+[^;]+|(?:class|struct|union|enum)\s+\w+|(?:extern\s+(?:"C"\s+)?)?[\w:<>,*&\s]+\s+\w+\s*\([^;{}]*\))\s*;/gu, "")
      .replace(/[{}]/gu, "")
      .trim();
    return rest ? "unresolved" : "declaration";
  }
  if (/\b(?:class|record|struct|interface|enum)\b/u.test(source) || /\b(?:public|private|protected|internal)\s+(?:static\s+)?\w+[<\w, >]*\s+\w+\s*\(/u.test(source)) return "implementation";
  const rest = source.replace(/^(?:global\s+)?using\s+[^;]+;/gmu, "").replace(/^namespace\s+[\w.]+\s*;?$/gmu, "").trim();
  return rest ? "unresolved" : "declaration";
}

/** 🧾️ Classifies an explicitly dispositioned source-format fixed/configurable package entry. */
export function classifyPackageSourceDisposition(content: string, disposition: PackageSourceDisposition, grammar: PackageGlueGrammarSpec): PackageSourceRole {
  if (disposition.validator === "package-glue") return classifyPackageSourceRole(content, grammar);
  if (disposition.validator === "vitest-configuration") return /^\s*import\s*\{\s*defineConfig\s*\}\s*from\s*["']vitest\/config["'];\s*export\s+default\s+defineConfig\(\{[\s\S]*\}\);\s*$/u.test(content) && classifyPackageSourceRole(content, grammar) === "declaration" ? "tool-metadata" : "unresolved";
  return /\bScriptRouter\b/u.test(content) && /\brunBundleScriptMain\b/u.test(content) ? "tool-metadata" : "unresolved";
}

/**
 * 🗺️ ONE repo walk answering every discovery question. For each `<owner>/📦️packages/<lang>/` it resolves the
 * package as either a direct manifest (two-level — plugins, styling) or a `🎯️targets/<target>/<manifest>` tree
 * (three-level — ui, renderer-engine: one package per render target), reads its role marker via
 * `readSemioMarker`, and — in the same pass — derives each owner's migration state from disk: residual
 * `⚡️implementations`/`⚡️implementation` dirs and Shape V1 entry files still at the owner root. A package's
 * existence IS the migration marker (`taxonomy.migratedMarker`), so no hand-maintained "already migrated" list
 * can drift. A markerless manifest is skipped silently in a `legacy`/`mixed`/`exempt`/undeclared area (not yet
 * migrated) but always shows up in `discoverBurndown`, so nothing vanishes unnoticed.
 */
function scanRepo(repoRoot: string, taxonomy: Taxonomy, catalog?: { readonly view: RegistryCatalogInputView; readonly inputs: Set<string> }): DiscoveryScan {
  if (typeof taxonomy.packagesDirName !== "string" || typeof taxonomy.targetsDirName !== "string" || !taxonomy.roles || !taxonomy.areas || !taxonomy.configurableEntryContracts) throw new Error("Catalog discovery requires the complete validated taxonomy vocabulary.");
  const packagesDirName = taxonomy.packagesDirName;
  const targetsDirName = taxonomy.targetsDirName;
  const forbiddenSegments = new Set(taxonomy.forbiddenPathSegments);
  const owners = new Map<string, OwnerAccumulator>();
  const problems: DiscoveryProblem[] = [];
  const unmarkedManifests: { path: string; area: string }[] = [];
  const packagingViolations: { path: string; ownerRel: string }[] = [];
  const implDirsByArea: Record<string, number> = {};
  let implDirsTotal = 0;
  const rel = (abs: string): string => relative(repoRoot, abs).replaceAll("\\", "/");
  const catalogExists = (abs: string): boolean => {
    if (!catalog) return existsSync(abs);
    const path = rel(abs);
    const kind = catalog.view.kind(path);
    if (kind === "symlink") throw new Error(`Registry catalog input is a symlink: ${path}`);
    if (kind) catalog.inputs.add(path);
    return kind !== null;
  };
  const catalogEntries = (abs: string) => {
    if (!catalog) return readdirSafe(abs);
    const path = rel(abs);
    if (path) catalog.inputs.add(path);
    return catalog.view.entries(path).map((entry) => ({ name: entry.name, isDirectory: () => entry.nodeKind === "directory", isFile: () => entry.nodeKind === "file" }));
  };

  const addPackageProblem = (owner: OwnerAccumulator, path: string, kind: DiscoveryProblem["kind"], detail: string): void => {
    const repoPath = rel(path);
    packagingViolations.push({ path: repoPath, ownerRel: owner.ownerRel });
    problems.push({ kind, path: repoPath, message: `"${repoPath}" ${detail}` });
  };

  const collectPackageRoles = (packageRoot: string, lang: PackageLang, owner: OwnerAccumulator, entryContractIds: readonly string[]): void => {
    if (catalog) return;
    const rule = taxonomy.packageBoundaryRules[lang];
    const grammar = rule && taxonomy.packageGlueGrammar[rule.glueGrammarId];
    if (!rule || !grammar) {
      addPackageProblem(owner, packageRoot, "package-role-unresolved", `has no package boundary rule or glue grammar for ${lang}.`);
      return;
    }
    const fixedNames = new Map<string, string[]>();
    for (const id of rule.allowedFixedContractIds) {
      const contract = taxonomy.fixedFilenameContracts[id];
      if (!contract) continue;
      const name = fixedContractFilename(contract);
      fixedNames.set(name, [...(fixedNames.get(name) ?? []), id]);
    }
    const entryNames = new Map<string, string[]>();
    for (const id of entryContractIds) {
      const contract = taxonomy.configurableEntryContracts[id];
      if (!contract) continue;
      entryNames.set(contract.filename, [...(entryNames.get(contract.filename) ?? []), id]);
    }
    const allowedKinds = new Set(rule.allowedFileKindIds);
    const allowedDirectories = new Set(rule.allowedDirectoryKindIds);
    const visit = (dir: string): void => {
      if (pathIsExcluded(repoRoot, dir, taxonomy)) return;
      for (const entry of readdirSafe(dir).sort((a, b) => a.name.localeCompare(b.name))) {
        const path = join(dir, entry.name);
        if (pathIsExcluded(repoRoot, path, taxonomy)) continue;
        if (entry.isDirectory()) {
          if (isDiscoverySkipDirectory(entry.name)) continue;
          if (existsSync(join(path, "CACHEDIR.TAG")) && /cache directory tag created by cargo/iu.test(readFileSync(join(path, "CACHEDIR.TAG"), "utf8"))) continue;
          const directoryKindId = semanticDirectoryKindId(entry.name, taxonomy);
          if (!directoryKindId || !allowedDirectories.has(directoryKindId)) addPackageProblem(owner, path, "packaging-violation", "is not an allowed semantic package directory.");
          visit(path);
          continue;
        }
        if (!entry.isFile()) {
          addPackageProblem(owner, path, "package-role-unresolved", "is neither a regular file nor a declared package directory.");
          continue;
        }
        const fixedContractIds = fixedNames.get(entry.name) ?? [];
        const configurableContractIds = entryNames.get(entry.name) ?? [];
        const entryContract = configurableContractIds.length > 0;
        const kindId = fileKindIdForSourcePath(entry.name, taxonomy);
        if (!kindId || (fixedContractIds.length === 0 && !entryContract && !allowedKinds.has(kindId))) {
          addPackageProblem(owner, path, "packaging-violation", "has no exact fixed/configurable contract or allowed file-kind identity.");
          continue;
        }
        const kind = taxonomy.fileKinds[kindId]!;
        if (kind.role !== "source") continue;
        let content: string;
        try {
          content = readFileSync(path, "utf8");
        } catch {
          addPackageProblem(owner, path, "package-role-unresolved", "could not be decoded as source.");
          continue;
        }
        const contractIds = [...fixedContractIds, ...configurableContractIds];
        const disposition = contractIds.map((id) => taxonomy.packageSourceDispositions[id]).find((value) => value !== undefined);
        if (contractIds.length > 0 && !disposition) {
          addPackageProblem(owner, path, "package-role-unresolved", "has a source-format fixed/configurable contract without a package source disposition.");
          continue;
        }
        const role = disposition ? classifyPackageSourceDisposition(content, disposition, grammar) : classifyPackageSourceRole(content, grammar);
        if (role === "implementation") addPackageProblem(owner, path, "package-implementation", "contains authored implementation inside a package boundary.");
        else if (role !== "tool-metadata" && (role === "unresolved" || !grammar.allowedRoles.includes(role))) addPackageProblem(owner, path, "package-role-unresolved", `has uncertain or disallowed package role ${JSON.stringify(role)}.`);
      }
    };
    visit(packageRoot);
  };

  const resolveOne = (manifestAbs: string, lang: PackageLang, owner: OwnerAccumulator, target: PackageTarget | undefined): void => {
    const manifestPath = rel(manifestAbs);
    const content = catalog?.view.readText(manifestPath);
    const marker = content === undefined ? readSemioMarker(manifestAbs, lang, taxonomy) : parseSemioMarker(content, lang, taxonomy);
    if (catalog) catalog.inputs.add(manifestPath);
    if (!marker) {
      unmarkedManifests.push({ path: manifestPath, area: owner.area });
      problems.push({ kind: "manifest-without-marker", path: manifestPath, message: `"${manifestPath}" has no resolvable semio role marker; all non-opaque areas require one.` });
      return;
    }
    if (!taxonomy.roles.includes(marker.role)) {
      problems.push({ kind: "unknown-role", path: manifestPath, message: `"${manifestPath}" declares unknown role "${marker.role}".` });
      return;
    }
    owner.packages.push({
      ownerRel: owner.ownerRel,
      lang,
      target,
      packageRel: rel(dirname(manifestAbs)),
      manifestPath,
      role: marker.role,
      id: marker.id ?? fallbackPackageId(manifestAbs, lang, owner.ownerRel, content),
      area: owner.area,
      maturity: "clean",
    });
  };

  const scanPackagesDir = (packagesAbs: string, owner: OwnerAccumulator): void => {
    if (pathIsExcluded(repoRoot, packagesAbs, taxonomy)) return;
    for (const langEntry of catalogEntries(packagesAbs)) {
      if (!langEntry.isDirectory() || langEntry.name.startsWith(".")) continue;
      const lang = langEntry.name as PackageLang;
      const ecosystem = taxonomy.ecosystems[lang];
      if (!ecosystem) {
        problems.push({ kind: "unknown-lang", path: rel(join(packagesAbs, langEntry.name)), message: `"${langEntry.name}" is not a declared language.` });
        continue;
      }
      const manifestFilename = exactContractFilename(ecosystem.manifestContractId, taxonomy);
      const langAbs = join(packagesAbs, langEntry.name);
      if (!manifestFilename) {
        collectPackageRoles(langAbs, lang, owner, ecosystem.entryContractIds);
        continue;
      }
      const directManifestAbs = join(langAbs, manifestFilename);
      const targetsAbs = join(langAbs, targetsDirName);
      const hasDirect = catalogExists(directManifestAbs);
      const hasTargets = catalogExists(targetsAbs);
      if (hasDirect && hasTargets) {
        problems.push({ kind: "ambiguous-lang-shape", path: rel(langAbs), message: `"${rel(langAbs)}" has both a direct manifest and a target directory.` });
        continue;
      }
      if (hasDirect) {
        resolveOne(directManifestAbs, lang, owner, undefined);
        collectPackageRoles(langAbs, lang, owner, ecosystem.entryContractIds);
        continue;
      }
      if (!hasTargets) continue;
      for (const targetEntry of catalogEntries(targetsAbs)) {
        if (!targetEntry.isDirectory()) continue;
        const targetAbs = join(targetsAbs, targetEntry.name);
        if (pathIsExcluded(repoRoot, targetAbs, taxonomy)) continue;
        const targetManifestAbs = join(targetAbs, manifestFilename);
        if (!catalogExists(targetManifestAbs)) {
          problems.push({ kind: "target-without-manifest", path: rel(targetAbs), message: `"${rel(targetAbs)}" has no exact manifest contract ${JSON.stringify(manifestFilename)}.` });
          continue;
        }
        resolveOne(targetManifestAbs, lang, owner, targetEntry.name);
        collectPackageRoles(targetAbs, lang, owner, taxonomy.targets[targetEntry.name]?.entryContractIds ?? ecosystem.entryContractIds);
      }
    }
  };

  const ownerRootEntryFiles = (entries: readonly { name: string; isDirectory: () => boolean }[]): string[] => {
    const names = new Set(Object.values(taxonomy.configurableEntryContracts).map((contract) => contract.filename));
    return entries.filter((entry) => !entry.isDirectory() && names.has(entry.name)).map((entry) => entry.name);
  };

  const walk = (absDir: string, ownerStack: readonly OwnerAccumulator[]): void => {
    if (pathIsExcluded(repoRoot, absDir, taxonomy)) return;
    const entries = catalogEntries(absDir);
    let stack = ownerStack;
    if (entries.some((entry) => entry.isDirectory() && entry.name === packagesDirName)) {
      const ownerRel = rel(absDir);
      const owner: OwnerAccumulator = { ownerRel, area: areaOf(ownerRel, taxonomy) ?? "", packages: [], residualImplDirs: 0, entryFilesAtOwnerRoot: ownerRootEntryFiles(entries) };
      if (catalog) for (const name of owner.entryFilesAtOwnerRoot) catalogExists(join(absDir, name));
      owners.set(ownerRel, owner);
      stack = [...ownerStack, owner];
      scanPackagesDir(join(absDir, packagesDirName), owner);
    }
    for (const entry of entries) {
      if (!entry.isDirectory() || entry.name.startsWith(".") || isDiscoverySkipDirectory(entry.name) || entry.name === packagesDirName) continue;
      const path = join(absDir, entry.name);
      if (pathIsExcluded(repoRoot, path, taxonomy)) continue;
      if (forbiddenSegments.has(entry.name)) {
        if (catalog) catalog.inputs.add(rel(path));
        implDirsTotal += 1;
        const area = stack.at(-1)?.area ?? areaOf(rel(absDir), taxonomy) ?? "";
        implDirsByArea[area] = (implDirsByArea[area] ?? 0) + 1;
        if (stack.length > 0) stack[stack.length - 1]!.residualImplDirs += 1;
        problems.push({ kind: "package-implementation", path: rel(path), message: `"${rel(path)}" is a forbidden implementation boundary in a clean-enforced area.` });
        continue;
      }
      walk(path, stack);
    }
  };
  walk(repoRoot, []);

  const discoveredOwners: DiscoveredOwner[] = [...owners.values()].map((owner) => {
    const maturity: PackageMaturity = owner.residualImplDirs === 0 && owner.entryFilesAtOwnerRoot.length === 0 ? "clean" : "mixed";
    const packages = owner.packages.map((pkg) => ({ ...pkg, maturity }));
    return {
      ownerRel: owner.ownerRel,
      area: owner.area,
      maturity,
      langs: [...new Set(packages.map((pkg) => pkg.lang))],
      targets: [...new Set(packages.flatMap((pkg) => pkg.target ? [pkg.target] : []))],
      roles: [...new Set(packages.map((pkg) => pkg.role))],
      packages,
      residualImplDirs: owner.residualImplDirs,
      entryFilesAtOwnerRoot: owner.entryFilesAtOwnerRoot,
    };
  }).sort((a, b) => a.ownerRel.localeCompare(b.ownerRel));
  const packages = discoveredOwners.flatMap((owner) => owner.packages).sort((a, b) => a.ownerRel.localeCompare(b.ownerRel) || (a.target ?? "").localeCompare(b.target ?? ""));
  return {
    packages,
    owners: discoveredOwners,
    problems,
    burndown: {
      ownersTotal: discoveredOwners.length,
      packagesTotal: packages.length,
      cleanOwners: discoveredOwners.filter((owner) => owner.maturity === "clean").length,
      mixedOwners: discoveredOwners.filter((owner) => owner.maturity === "mixed"),
      implDirsTotal,
      implDirsByArea,
      unmarkedManifests,
      packagingViolations,
    },
  };
}

function scan(repoRoot: string, taxonomy: Taxonomy): DiscoveryScan {
  const cached = scanCache.get(repoRoot);
  if (cached) return cached;
  const result = scanRepo(repoRoot, taxonomy);
  scanCache.set(repoRoot, result);
  return result;
}

/** 📦️ Flat catalog of every marked package in the repo — see `scanRepo`. */
export function discoverPackages(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveredPackage[] {
  return [...scan(repoRoot, taxonomy).packages];
}

/** 🏠️ Owner-level view of the same scan: one row per `📦️packages`-carrying dir, with its langs/targets/roles and derived maturity. */
export function discoverOwners(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveredOwner[] {
  return [...scan(repoRoot, taxonomy).owners];
}

/** ⚠️ Diagnostics half of the scan (ambiguous shapes, dangling target dirs, unknown langs/roles, unmarked manifests outside legacy areas). */
export function discoverPackageProblems(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveryProblem[] {
  return [...scan(repoRoot, taxonomy).problems];
}

/** 🔥️ Burn-down half of the scan: everything that must shrink to zero before the finalization flip. */
export function discoverBurndown(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveryBurndown {
  return scan(repoRoot, taxonomy).burndown;
}
//#endregion 🧭️Discovery

//#region 📦️CargoProviderManifestProjection
/** 🧾️ Repository-owned TOML boundary for pure Cargo provider manifest projections. */
export interface CargoProviderTomlParser {
  parse(source: string): unknown;
}

/** 🪪️ Raw provider manifest input whose locator was already established by source authority. */
export interface CargoProviderManifestProjectionInput {
  readonly locator: string;
  readonly source: string;
}

/** 🧷️ One dependency declaration retained as unapproved provider evidence. */
export interface CargoProviderDependencyProjection {
  readonly key: string;
  readonly packageOverride?: string;
  readonly localPath?: string;
  readonly workspaceInherited: boolean;
  readonly targetCondition?: string;
  readonly source: "version" | "table";
  readonly version?: string;
  readonly unsupported: Readonly<Record<string, unknown>>;
  readonly unapprovedReasons: readonly string[];
}

/** 📚️ One explicit Cargo library target, never inferred from package or filename identity. */
export interface CargoProviderLibraryProjection {
  readonly name?: string;
  readonly path?: string;
  readonly crateTypes: readonly string[];
  readonly procMacro: boolean;
}

/** 🧩️ Pure manifest facts for a later provider authority resolver. */
export interface CargoProviderManifestProjection {
  readonly locator: string;
  readonly package?: Readonly<{ readonly name: string; readonly version?: string; readonly versionInherited?: true; readonly workspaceLocator?: string }>;
  readonly library?: CargoProviderLibraryProjection;
  readonly workspaceDeclared?: true;
  readonly workspaceDependencies: readonly CargoProviderDependencyProjection[];
  readonly dependencies: readonly CargoProviderDependencyProjection[];
  readonly developmentDependencies: readonly CargoProviderDependencyProjection[];
  readonly buildDependencies: readonly CargoProviderDependencyProjection[];
}

/** 🧯️ Bun's parser is hidden behind repository-owned syntax input/output types. */
export const cargoProviderTomlParser: CargoProviderTomlParser = {
  parse(source: string): unknown {
    const runtime = (globalThis as { readonly Bun?: unknown }).Bun;
    const toml = typeof runtime === "object" && runtime !== null ? (runtime as { readonly TOML?: unknown }).TOML : undefined;
    if (typeof toml !== "object" || toml === null || typeof (toml as { readonly parse?: unknown }).parse !== "function") throw new Error("Cargo provider TOML projection requires Bun.TOML.parse.");
    return (toml as { readonly parse: (input: string) => unknown }).parse(source);
  },
};

function cargoProviderRecord(value: unknown, context: string): Readonly<Record<string, unknown>> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error(`${context} must be a TOML table.`);
  return value as Readonly<Record<string, unknown>>;
}

function cargoProviderString(value: unknown, context: string): string {
  if (typeof value !== "string") throw new Error(`${context} must be a string.`);
  return value;
}

function cargoProviderBoolean(value: unknown, context: string): boolean {
  if (typeof value !== "boolean") throw new Error(`${context} must be a boolean.`);
  return value;
}

function cargoProviderTomlValue(value: unknown, context: string): unknown {
  if (value === null || typeof value === "string" || typeof value === "boolean") return value;
  if (typeof value === "number" && Number.isFinite(value)) return value;
  if (Array.isArray(value)) return value.map((entry, index) => cargoProviderTomlValue(entry, `${context}[${index}]`));
  if (typeof value === "object") {
    if (Object.getPrototypeOf(value) !== Object.prototype && Object.getPrototypeOf(value) !== null) throw new Error(`${context} has an unsupported TOML object.`);
    const record = cargoProviderRecord(value, context);
    return Object.fromEntries(Object.entries(record).map(([key, entry]) => [key, cargoProviderTomlValue(entry, `${context}.${key}`)]));
  }
  throw new Error(`${context} has an unsupported TOML value.`);
}

function cargoProviderMultilineString(source: string): boolean {
  let quote: "basic" | "literal" | undefined;
  for (let index = 0; index < source.length; index++) {
    const character = source[index]!;
    if (quote === "basic") {
      if (character === "\\") { index++; continue; }
      if (character === "\"") quote = undefined;
      continue;
    }
    if (quote === "literal") {
      if (character === "'") quote = undefined;
      continue;
    }
    if (character === "#") {
      while (index < source.length && source[index] !== "\n") index++;
      continue;
    }
    if (character === "\"" && source.slice(index, index + 3) === "\"\"\"") return true;
    if (character === "'" && source.slice(index, index + 3) === "'''") return true;
    if (character === "\"") quote = "basic";
    if (character === "'") quote = "literal";
  }
  return false;
}

function cargoProviderLocator(locator: string): string {
  if (typeof locator !== "string" || locator.length === 0 || locator.includes("\\") || /[:\u0000-\u001F\u007F\u0085\u2028\u2029]/u.test(locator)) throw new Error("Provider manifest locator must be a safe repository-relative POSIX path.");
  const segments = locator.split("/");
  if (segments.some((segment) => segment.length === 0 || segment === "." || segment === ".." || /^[A-Za-z]:$/u.test(segment) || segment.toLowerCase() === "compose")) throw new Error("Provider manifest locator is unsafe or excluded.");
  if (segments.at(-1) !== "Cargo.toml") throw new Error("Provider manifest locator must name Cargo.toml.");
  return locator;
}

function cargoProviderDependency(key: string, value: unknown, scope: string, targetCondition?: string): CargoProviderDependencyProjection {
  const context = `${scope}.${key}`;
  if (typeof value === "string") return { key, workspaceInherited: false, ...(targetCondition === undefined ? {} : { targetCondition }), source: "version", version: value, unsupported: {}, unapprovedReasons: ["version-only"] };
  const record = cargoProviderRecord(value, context);
  const packageOverride = record.package === undefined ? undefined : cargoProviderString(record.package, `${context}.package`);
  const localPath = record.path === undefined ? undefined : cargoProviderString(record.path, `${context}.path`);
  const workspaceInherited = record.workspace === undefined ? false : cargoProviderBoolean(record.workspace, `${context}.workspace`);
  const version = record.version === undefined ? undefined : cargoProviderString(record.version, `${context}.version`);
  for (const field of ["git", "registry", "branch", "tag", "rev"] as const) if (record[field] !== undefined) cargoProviderString(record[field], `${context}.${field}`);
  if (workspaceInherited && [localPath, version, record.git, record.registry, record.branch, record.tag, record.rev].some((entry) => entry !== undefined)) throw new Error(`${context} mixes workspace inheritance with another source.`);
  if ([localPath, record.git, record.registry].filter((entry) => entry !== undefined).length > 1) throw new Error(`${context} mixes multiple dependency sources.`);
  const unsupported = Object.fromEntries(Object.entries(record).filter(([field]) => !["package", "path", "workspace", "version"].includes(field)).map(([field, entry]) => [field, cargoProviderTomlValue(entry, `${context}.${field}`)]));
  const unapprovedReasons = [
    ...(workspaceInherited ? ["workspace-inheritance"] : []),
    ...(record.workspace === false ? ["workspace-false"] : []),
    ...(localPath === undefined ? ["no-local-path"] : []),
    ...(version === undefined ? [] : ["version-constraint"]),
    ...(["git", "registry", "branch", "tag", "rev"].some((field) => record[field] !== undefined) ? ["external-source"] : []),
    ...(Object.keys(unsupported).length === 0 ? [] : ["unsupported-fields"]),
  ];
  return { key, ...(packageOverride === undefined ? {} : { packageOverride }), ...(localPath === undefined ? {} : { localPath }), workspaceInherited, ...(targetCondition === undefined ? {} : { targetCondition }), source: "table", ...(version === undefined ? {} : { version }), unsupported, unapprovedReasons };
}

function cargoProviderDependencies(value: unknown, scope: string, targetCondition?: string): readonly CargoProviderDependencyProjection[] {
  if (value === undefined) return [];
  const record = cargoProviderRecord(value, scope);
  return Object.entries(record).map(([key, entry]) => cargoProviderDependency(key, entry, scope, targetCondition));
}

function cargoProviderLibrary(value: unknown): CargoProviderLibraryProjection | undefined {
  if (value === undefined) return undefined;
  const record = cargoProviderRecord(value, "lib");
  const crateTypesValue = record["crate-type"];
  if (crateTypesValue !== undefined && (!Array.isArray(crateTypesValue) || crateTypesValue.some((entry) => typeof entry !== "string"))) throw new Error("lib.crate-type must be an array of strings.");
  return {
    ...(record.name === undefined ? {} : { name: cargoProviderString(record.name, "lib.name") }),
    ...(record.path === undefined ? {} : { path: cargoProviderString(record.path, "lib.path") }),
    crateTypes: crateTypesValue === undefined ? [] : [...crateTypesValue],
    procMacro: record["proc-macro"] === undefined ? false : cargoProviderBoolean(record["proc-macro"], "lib.proc-macro"),
  };
}

/** 🧭️ Projects one caller-supplied Cargo manifest without resolving paths, aliases, or graph authority. */
export function projectCargoProviderManifest(input: CargoProviderManifestProjectionInput, parser: CargoProviderTomlParser = cargoProviderTomlParser): CargoProviderManifestProjection {
  const locator = cargoProviderLocator(input.locator);
  if (typeof input.source !== "string") throw new Error("Provider manifest source must be a string.");
  if (cargoProviderMultilineString(input.source)) throw new Error("Provider manifest multiline strings are unsupported.");
  let parsed: unknown;
  try {
    parsed = parser.parse(input.source);
  } catch (error) {
    throw new Error(`Provider manifest TOML is invalid: ${error instanceof Error ? error.message : String(error)}`);
  }
  const root = cargoProviderRecord(parsed, "manifest");
  const packageProjection = root.package === undefined ? undefined : (() => {
    const table = cargoProviderRecord(root.package, "package");
    const version = table.version;
    const versionProjection = version === undefined ? {} : typeof version === "string"
      ? { version }
      : (() => {
          const inheritance = cargoProviderRecord(version, "package.version");
          if (Object.keys(inheritance).length !== 1 || inheritance.workspace !== true) throw new Error("package.version must be a string or exact workspace inheritance.");
          return { versionInherited: true as const };
        })();
    const workspaceLocator = table.workspace === undefined ? undefined : cargoProviderString(table.workspace, "package.workspace");
    return { name: cargoProviderString(table.name, "package.name"), ...versionProjection, ...(workspaceLocator === undefined ? {} : { workspaceLocator }) };
  })();
  const target = root.target === undefined ? {} : cargoProviderRecord(root.target, "target");
  const targetTables = Object.entries(target).map(([condition, entry]) => [condition, cargoProviderRecord(entry, `target.${condition}`)] as const);
  const targetDependencies = targetTables.flatMap(([condition, table]) => cargoProviderDependencies(table.dependencies, `target.${condition}.dependencies`, condition));
  const targetDevelopmentDependencies = targetTables.flatMap(([condition, table]) => cargoProviderDependencies(table["dev-dependencies"], `target.${condition}.dev-dependencies`, condition));
  const targetBuildDependencies = targetTables.flatMap(([condition, table]) => cargoProviderDependencies(table["build-dependencies"], `target.${condition}.build-dependencies`, condition));
  const workspace = root.workspace === undefined ? {} : cargoProviderRecord(root.workspace, "workspace");
  const library = cargoProviderLibrary(root.lib);
  return {
    locator,
    ...(packageProjection === undefined ? {} : { package: packageProjection }),
    ...(library === undefined ? {} : { library }),
    ...(root.workspace === undefined ? {} : { workspaceDeclared: true as const }),
    workspaceDependencies: cargoProviderDependencies(workspace.dependencies, "workspace.dependencies"),
    dependencies: [...cargoProviderDependencies(root.dependencies, "dependencies"), ...targetDependencies],
    developmentDependencies: [...cargoProviderDependencies(root["dev-dependencies"], "dev-dependencies"), ...targetDevelopmentDependencies],
    buildDependencies: [...cargoProviderDependencies(root["build-dependencies"], "build-dependencies"), ...targetBuildDependencies],
  };
}

/** 🔗️ Exact selected dependency input for a local Cargo provider binding proof. */
export interface CargoProviderBindingInput {
  readonly workspaceRoot: string;
  readonly consumerManifestLocator: string;
  readonly dependencyKey: string;
}

/** 🧷️ Bounded provider identity evidence for one selected local normal dependency. */
export interface CargoProviderBinding {
  readonly consumerManifestLocator: string;
  readonly workspaceManifestLocator: string;
  readonly providerManifestLocator: string;
  readonly librarySourceLocator: string;
  readonly packageName: string;
  readonly libraryName: string;
  readonly procMacro: boolean;
  readonly dependencyKey: string;
  readonly workspaceInherited: boolean;
  readonly pathAuthority: "consumer" | "workspace";
  readonly externName: string;
}

function cargoProviderBindingLocator(locator: string, finalName?: string): string {
  if (typeof locator !== "string" || locator.length === 0 || locator !== locator.normalize("NFC") || locator.includes("\\") || /[:\u0000-\u001F\u007F\u0085\u2028\u2029]/u.test(locator)) throw new Error("Cargo provider binding locator is unsafe.");
  const segments = locator.split("/");
  if (segments.some((segment) => !segment || segment === "." || segment === ".." || segment.toLocaleLowerCase("en-US") === "compose") || finalName !== undefined && segments.at(-1) !== finalName) throw new Error("Cargo provider binding locator is excluded or malformed.");
  return locator;
}

function cargoProviderBindingWorkspaceRoot(root: string): string {
  if (typeof root !== "string" || root.length === 0 || root !== root.normalize("NFC") || !isAbsolute(root) || /[\u0000-\u001F\u007F\u0085\u2028\u2029]/u.test(root)) throw new Error("Cargo provider workspace root is unsafe.");
  const windows = /^[A-Za-z]:[\\/]/u.test(root), posixRoot = root.startsWith("/");
  if ((!windows && !posixRoot) || root.includes(":") && (!windows || root.indexOf(":") !== 1 || root.lastIndexOf(":") !== 1) || posixRoot && root.includes("\\")) throw new Error("Cargo provider workspace root is unsafe.");
  const segments = root.split(/[\\/]/u);
  if (segments.some((segment) => segment === "." || segment === ".." || segment.toLocaleLowerCase("en-US") === "compose")) throw new Error("Cargo provider workspace root is excluded.");
  const authority = workspaceAuthorityPath(root, "Cargo provider workspace root");
  noFollowDirectoryAncestry(authority, "Cargo provider workspace root ancestry");
  return authority;
}

function cargoProviderBindingRead(root: string, locator: string): string {
  const safe = cargoProviderBindingLocator(locator);
  let current = root;
  for (const [index, segment] of safe.split("/").entries()) {
    current = join(current, segment);
    const state = lstatSync(current);
    if (state.isSymbolicLink() || (index + 1 === safe.split("/").length ? !state.isFile() : !state.isDirectory())) throw new Error("Cargo provider binding source is not a no-follow regular file.");
  }
  return readFileSync(current, "utf8");
}

function cargoProviderBindingDirectory(root: string, segments: readonly string[]): void {
  const path = join(root, ...segments);
  const state = lstatSync(path);
  if (state.isSymbolicLink() || !state.isDirectory()) throw new Error("Cargo provider dependency path is not a no-follow directory.");
}

function cargoProviderBindingPath(root: string, baseLocator: string, path: string, finalName: string, pathPointsToFile = false): string {
  if (typeof path !== "string" || path.length === 0 || path !== path.normalize("NFC") || path.includes("\\") || /[:\u0000-\u001F\u007F\u0085\u2028\u2029]/u.test(path) || path.startsWith("/")) throw new Error("Cargo provider dependency path is unsafe.");
  const stack = baseLocator.split("/").slice(0, -1);
  const parts = path.split("/");
  for (const [index, segment] of parts.entries()) {
    if (!segment || segment === "." || segment.toLocaleLowerCase("en-US") === "compose") throw new Error("Cargo provider dependency path is excluded.");
    if (segment === "..") { if (stack.length === 0) throw new Error("Cargo provider dependency path escapes the workspace."); stack.pop(); }
    else {
      stack.push(segment);
      if (!pathPointsToFile || index + 1 !== parts.length) cargoProviderBindingDirectory(root, stack);
    }
  }
  return cargoProviderBindingLocator([...(pathPointsToFile ? stack : [...stack, finalName])].join("/"), finalName);
}

function cargoProviderBindingDependencySettings(dependency: CargoProviderDependencyProjection): boolean {
  const unsupported = dependency.unsupported;
  if (unsupported.features !== undefined && (!Array.isArray(unsupported.features) || unsupported.features.some((feature) => typeof feature !== "string"))) return false;
  if (unsupported["default-features"] !== undefined && typeof unsupported["default-features"] !== "boolean") return false;
  return Object.keys(unsupported).every((field) => field === "features" || field === "default-features");
}

function cargoProviderBindingNearestWorkspace(root: string, manifestLocator: string, manifest: CargoProviderManifestProjection): string {
  if (manifest.workspaceDeclared) return manifestLocator;
  const directory = manifestLocator.split("/").slice(0, -1);
  for (; directory.length >= 0; directory.pop()) {
    const locator = [...directory, "Cargo.toml"].join("/");
    try {
      const ancestor = projectCargoProviderManifest({ locator, source: cargoProviderBindingRead(root, locator) });
      if (ancestor.workspaceDeclared) return locator;
    } catch (error) {
      if (!(error instanceof Error) || !/ENOENT/u.test(error.message)) throw error;
    }
    if (directory.length === 0) break;
  }
  throw new Error("Cargo manifest has no governing workspace on its bounded ancestor chain.");
}

/** 🧭️ Resolves one selected normal local dependency without following a Cargo graph. */
export function resolveCargoProviderBinding(input: CargoProviderBindingInput): CargoProviderBinding {
  const consumerManifestLocator = cargoProviderBindingLocator(input.consumerManifestLocator, "Cargo.toml");
  if (typeof input.dependencyKey !== "string" || input.dependencyKey.length === 0) throw new Error("Cargo provider dependency key is required.");
  const root = cargoProviderBindingWorkspaceRoot(input.workspaceRoot);
  const workspaceManifestLocator = "Cargo.toml", workspace = projectCargoProviderManifest({ locator: workspaceManifestLocator, source: cargoProviderBindingRead(root, workspaceManifestLocator) });
  if (!workspace.workspaceDeclared) throw new Error("Cargo provider workspace manifest must declare [workspace].");
  const consumer = projectCargoProviderManifest({ locator: consumerManifestLocator, source: cargoProviderBindingRead(root, consumerManifestLocator) });
  if (!consumer.package) throw new Error("Cargo consumer manifest requires [package].");
  if (cargoProviderBindingNearestWorkspace(root, consumerManifestLocator, consumer) !== workspaceManifestLocator) throw new Error("Cargo consumer nearest workspace disagrees with the supplied workspace.");
  if (consumer.package.workspaceLocator !== undefined && cargoProviderBindingPath(root, consumerManifestLocator, consumer.package.workspaceLocator, "Cargo.toml") !== workspaceManifestLocator) throw new Error("Cargo consumer explicit workspace locator disagrees with the supplied workspace.");
  const selected = consumer.dependencies.filter((dependency) => dependency.key === input.dependencyKey);
  if (selected.length !== 1 || selected[0]!.targetCondition !== undefined) throw new Error("Cargo selected dependency must occur exactly once in unconditional normal dependencies.");
  const consumerDependency = selected[0]!;
  if (!cargoProviderBindingDependencySettings(consumerDependency) || consumerDependency.unapprovedReasons.includes("workspace-false") || consumerDependency.workspaceInherited && consumerDependency.packageOverride !== undefined) throw new Error("Cargo selected consumer dependency settings are unsupported.");
  const dependency = consumerDependency.workspaceInherited ? (() => {
    const inherited = workspace.workspaceDependencies.filter((entry) => entry.key === input.dependencyKey);
    if (inherited.length !== 1) throw new Error("Cargo workspace dependency inheritance is unresolved or ambiguous.");
    return inherited[0]!;
  })() : consumerDependency;
  if (dependency.localPath === undefined || dependency.version !== undefined || !cargoProviderBindingDependencySettings(dependency) || dependency.unapprovedReasons.includes("workspace-false")) throw new Error("Cargo selected dependency is not a supported local path authority.");
  const pathAuthority = consumerDependency.workspaceInherited ? "workspace" as const : "consumer" as const;
  const providerManifestLocator = cargoProviderBindingPath(root, pathAuthority === "workspace" ? workspaceManifestLocator : consumerManifestLocator, dependency.localPath, "Cargo.toml");
  const provider = projectCargoProviderManifest({ locator: providerManifestLocator, source: cargoProviderBindingRead(root, providerManifestLocator) });
  if (cargoProviderBindingNearestWorkspace(root, providerManifestLocator, provider) !== workspaceManifestLocator) throw new Error("Cargo provider nearest workspace disagrees with the supplied workspace.");
  if (provider.package?.workspaceLocator !== undefined && cargoProviderBindingPath(root, providerManifestLocator, provider.package.workspaceLocator, "Cargo.toml") !== workspaceManifestLocator) throw new Error("Cargo provider explicit workspace locator disagrees with the supplied workspace.");
  const packageName = dependency.packageOverride ?? dependency.key;
  if (provider.package?.name !== packageName) throw new Error("Cargo selected dependency package identity disagrees with the provider manifest.");
  if (!provider.library?.name || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(provider.library.name) || !provider.library.path) throw new Error("Cargo provider requires an explicit valid library name and source path.");
  const librarySourceLocator = cargoProviderBindingPath(root, providerManifestLocator, provider.library.path, provider.library.path.split("/").at(-1)!, true);
  cargoProviderBindingRead(root, librarySourceLocator);
  return { consumerManifestLocator, workspaceManifestLocator, providerManifestLocator, librarySourceLocator, packageName, libraryName: provider.library.name, procMacro: provider.library.procMacro, dependencyKey: dependency.key, workspaceInherited: consumerDependency.workspaceInherited, pathAuthority, externName: dependency.packageOverride === undefined ? provider.library.name : dependency.key.replaceAll("-", "_") };
}
//#region 🧬️MutationMetadataSource
/** 🧬️ One frozen local provider identity required by metadata source proof. */
export interface MutationMetadataProviderIdentity { readonly role: "lower-contract" | "os-facade" | "metadata-derive"; readonly manifestLocator: string; readonly packageName: string; readonly libraryName: string; readonly procMacro: boolean; }
/** 🧬️ FND21's already-proven wrapped declaration origin without a root-script dependency. */
export interface MutationMetadataSourceOrigin { readonly sourcePath: string; readonly declarationName: string; readonly modulePath: readonly string[]; }
/** 🧬️ Bounded caller-owned evidence used to prove one metadata provider route. */
export interface MutationMetadataSourceInput { readonly repositoryRoot: string; readonly consumerManifestLocator: string; readonly origin: MutationMetadataSourceOrigin; readonly files: readonly string[]; readonly readSource: (path: string) => string | undefined; readonly checkCancellation?: () => void; }
/** 🧬️ Exact accepted/rejected metadata source evidence; this does not activate policy. */
export interface MutationMetadataSourceProof { readonly accepted: boolean; readonly diagnostics: readonly string[]; readonly consumerContext: RustModuleContext | null; readonly deriveProvider: MutationMetadataProviderIdentity | null; readonly contractProvider: MutationMetadataProviderIdentity | null; readonly facadeProvider: MutationMetadataProviderIdentity | null; }

const MUTATION_METADATA_PROVIDERS: readonly MutationMetadataProviderIdentity[] = [
  { role: "lower-contract", manifestLocator: "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml", packageName: "semio-framework-replication", libraryName: "protocol", procMacro: false },
  { role: "os-facade", manifestLocator: "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml", packageName: "semio-framework-os-kernel", libraryName: "semio_framework_os_kernel", procMacro: false },
  { role: "metadata-derive", manifestLocator: "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml", packageName: "semio-framework-os-kernel-dsl-derive", libraryName: "dsl_derive", procMacro: true },
];

function mutationMetadataProvider(binding: CargoProviderBinding): MutationMetadataProviderIdentity | null {
  return MUTATION_METADATA_PROVIDERS.find((provider) => provider.manifestLocator === binding.providerManifestLocator && provider.packageName === binding.packageName && provider.libraryName === binding.libraryName && provider.procMacro === binding.procMacro) ?? null;
}

function mutationMetadataPath(path: string): readonly string[] | null {
  const parts = (path.startsWith("::") ? path.slice(2) : path).split("::");
  return parts.length > 0 && parts.every((part) => /^[A-Za-z_][A-Za-z0-9_]*$/u.test(part)) ? parts : null;
}

function mutationMetadataBindings(root: string, manifest: string, crate: string, cache: Map<string, readonly CargoProviderBinding[]>, readSource?: (path: string) => string | undefined): readonly CargoProviderBinding[] {
  const key = `${root}\0${manifest}\0${crate}`;
  const cached = cache.get(key);
  if (cached !== undefined) return cached;
  const manifestSource = readSource?.(manifest), projected = manifestSource === undefined ? undefined : projectCargoProviderManifest({ locator: manifest, source: manifestSource });
  const dependencyKeys = [...new Set([crate, crate.replaceAll("_", "-"), ...(projected?.dependencies.filter((dependency) => dependency.targetCondition === undefined).map((dependency) => dependency.key) ?? [])])];
  const bindings: CargoProviderBinding[] = [];
  for (const dependencyKey of dependencyKeys) try {
    const binding = resolveCargoProviderBinding({ workspaceRoot: root, consumerManifestLocator: manifest, dependencyKey });
    if (dependencyKey === crate || dependencyKey === crate.replaceAll("_", "-") || binding.externName === crate) bindings.push(binding);
  } catch {}
  const resolved = bindings.filter((binding, index) => bindings.findIndex((other) => other.providerManifestLocator === binding.providerManifestLocator && other.dependencyKey === binding.dependencyKey) === index);
  cache.set(key, resolved);
  return resolved;
}

function mutationMetadataAliasPaths(facts: RustMutationMetadataFacts, modulePath: readonly string[], path: readonly string[], publicOnly: boolean): readonly string[][] {
  const [root, ...tail] = path, scoped = facts.crateAliases.filter((alias) => alias.modulePath.join("\0") === modulePath.join("\0") && !alias.conditional && (!publicOnly || alias.kind === "reexport"));
  return [...scoped.filter((alias) => alias.alias === root).map((alias) => [...(mutationMetadataPath(alias.source) ?? []), ...tail]), ...scoped.filter((alias) => alias.alias === "*").map((alias) => [...(mutationMetadataPath(alias.source) ?? []), root, ...tail])].filter((candidate) => candidate.length > 0);
}

function mutationMetadataExternal(root: string, manifest: string, facts: RustMutationMetadataFacts, modulePath: readonly string[], path: string, cache: Map<string, readonly CargoProviderBinding[]>, readSource: (path: string) => string | undefined, publicOnly = false, terminalOverride?: string): readonly { readonly binding: CargoProviderBinding; readonly terminal: string }[] {
  const direct = mutationMetadataPath(path);
  if (!direct) return [];
  const resolve = (candidate: readonly string[], scope: readonly string[], depth: number, requirePublic: boolean): readonly { readonly binding: CargoProviderBinding; readonly terminal: string }[] => {
    if (depth > 16 || candidate.length === 0) return [];
    let target: readonly string[] | null = null, rest = candidate;
    if (candidate[0] === "crate") { target = []; rest = candidate.slice(1); }
    else if (candidate[0] === "self") { target = scope; rest = candidate.slice(1); }
    else if (candidate[0] === "super") { let index = 0, parent = [...scope]; while (candidate[index] === "super") { if (parent.length === 0) return []; parent.pop(); index += 1; } target = parent; rest = candidate.slice(index); }
    if (target === null) {
      const bindings = mutationMetadataBindings(root, manifest, candidate[0]!, cache, readSource);
      if (bindings.length > 0) return bindings.map((binding) => ({ binding, terminal: terminalOverride ?? candidate.at(-1)! }));
      const aliases = mutationMetadataAliasPaths(facts, scope, candidate, requirePublic);
      const transformed = aliases.filter((alias) => alias.join("\0") !== candidate.join("\0"));
      if (transformed.length > 0) return transformed.flatMap((alias) => resolve(alias, scope, depth + 1, requirePublic));
      return [];
    }
    if (rest.length === 0) return [];
    const bindings = mutationMetadataBindings(root, manifest, rest[0]!, cache, readSource);
    if (bindings.length > 0) return bindings.map((binding) => ({ binding, terminal: terminalOverride ?? rest.at(-1)! }));
    const nextScope = [...target, ...rest.slice(0, -1)], aliases = mutationMetadataAliasPaths(facts, nextScope, [rest.at(-1)!], requirePublic || nextScope.join("\0") !== scope.join("\0"));
    return aliases.flatMap((alias) => resolve(alias, [...target, ...nextScope], depth + 1, true));
  };
  return resolve(direct, modulePath, 0, publicOnly);
}

function mutationMetadataFacadeRoutes(input: MutationMetadataSourceInput, graph: RustModuleGraph, binding: CargoProviderBinding, expected: MutationMetadataProviderIdentity, seen: ReadonlySet<string>, cache: Map<string, readonly CargoProviderBinding[]>): readonly ({ readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null } | null)[] {
  const root = (graph.contexts.get(binding.librarySourceLocator) ?? []).filter((context) => context.crateRoot === binding.librarySourceLocator && context.manifestPath === binding.providerManifestLocator && context.modulePath.length === 0);
  if (root.length !== 1) return [];
  const walked = new Set<string>(), symbols = new Set<string>(), factsBySource = new Map<string, RustMutationMetadataFacts>();
  const walk = (sourcePath: string, context: RustModuleContext, path: readonly string[], depth: number): readonly ({ readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null } | null)[] => {
    input.checkCancellation?.();
    if (depth > 32 || path.length === 0) return [];
    const key = `${sourcePath}\0${context.modulePath.join("\0")}\0${path.join("\0")}`;
    if (walked.has(key)) return [];
    const symbol = `${sourcePath}\0${context.modulePath.join("\0")}\0${path[0]!}\0${path.at(-1)!}`;
    if (symbols.has(symbol)) return [];
    walked.add(key);
    symbols.add(symbol);
    const source = input.readSource(sourcePath);
    if (source === undefined) return [null];
    const facts = factsBySource.get(sourcePath) ?? inspectRustMutationMetadataFacts(source);
    factsBySource.set(sourcePath, facts);
    const aliases = mutationMetadataAliasPaths(facts, context.sourceScope, path, true), localName = (candidate: readonly string[]): boolean => { const first = candidate[0], base = first === "crate" ? [] : first === "self" ? context.modulePath : context.modulePath; const rest = first === "crate" || first === "self" ? candidate.slice(1) : candidate; return rest.length > 1 && graph.targets.has(`${binding.librarySourceLocator}\0${[...base, rest[0]!].join("::")}`); }, external = [...mutationMetadataBindings(input.repositoryRoot, binding.providerManifestLocator, path[0]!, cache, input.readSource).map((candidate) => ({ candidate, terminal: path.at(-1)! })), ...aliases.flatMap((path) => ["crate", "self", "super"].includes(path[0] ?? "") || localName(path) ? [] : mutationMetadataBindings(input.repositoryRoot, binding.providerManifestLocator, path[0]!, cache, input.readSource).map((candidate) => ({ candidate, terminal: path.at(-1)! })))].flatMap(({ candidate, terminal }) => { const route = mutationMetadataRoute(input, graph, candidate, terminal, expected, cache, seen, true); return route === undefined ? [] : [route]; });
    const local = [path, ...aliases].flatMap((candidate) => {
      let base = context.modulePath, rest = candidate;
      if (candidate[0] === "crate") { base = []; rest = candidate.slice(1); }
      else if (candidate[0] === "self") { rest = candidate.slice(1); }
      else if (candidate[0] === "super") { let index = 0; base = [...context.modulePath]; while (candidate[index] === "super") { if (base.length === 0) return []; base = base.slice(0, -1); index += 1; } rest = candidate.slice(index); }
      if (rest.length < 2) return [];
      const modulePath = [...base, rest[0]!], target = graph.targets.get(`${binding.librarySourceLocator}\0${modulePath.join("::")}`);
      const next = target === undefined ? [] : (graph.contexts.get(target) ?? []).filter((item) => item.crateRoot === binding.librarySourceLocator && item.manifestPath === binding.providerManifestLocator && item.modulePath.join("\0") === modulePath.join("\0"));
      return next.length === 1 ? walk(target!, next[0]!, rest.slice(1), depth + 1) : [];
    });
    return [...external, ...local];
  };
  return walk(binding.librarySourceLocator, root[0]!, ["MutationLeaf"], 0);
}

function mutationMetadataRoute(input: MutationMetadataSourceInput, graph: RustModuleGraph, binding: CargoProviderBinding, terminal: string, expected: MutationMetadataProviderIdentity, cache: Map<string, readonly CargoProviderBinding[]>, seen: ReadonlySet<string> = new Set(), facadeHop = false): { readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null } | null | undefined {
  input.checkCancellation?.();
  const provider = mutationMetadataProvider(binding);
  if (provider?.role === expected.role && terminal === "MutationLeaf") return { provider, facade: null };
  if (facadeHop && provider !== null && provider.role !== "os-facade") return undefined;
  if (provider?.role !== "os-facade" || expected.role === "os-facade" || terminal !== "MutationLeaf" || seen.has(binding.providerManifestLocator)) return null;
  const facadeRoutes = mutationMetadataFacadeRoutes(input, graph, binding, expected, new Set([...seen, binding.providerManifestLocator]), cache), routes = mutationMetadataResolvedRoutes(facadeRoutes);
  return routes?.length === 1 ? { provider: routes[0]!.provider, facade: provider } : null;
}

function mutationMetadataResolvedRoutes(routes: readonly ({ readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null } | null | undefined)[]): readonly { readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null }[] | null {
  if (routes.length === 0 || routes.some((route) => route === null || route === undefined)) return null;
  return routes.filter((route, index): route is { readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null } => route !== null && route !== undefined && routes.findIndex((other) => other !== null && other !== undefined && other.provider.manifestLocator === route.provider.manifestLocator && other.facade?.manifestLocator === route.facade?.manifestLocator) === index);
}

/** 🧬️ Proves exact derive and lower-trait providers for one already-wrapped consumer declaration. */
export function inspectMutationMetadataSource(input: MutationMetadataSourceInput): MutationMetadataSourceProof {
  const reject = (diagnostic: string, consumerContext: RustModuleContext | null = null): MutationMetadataSourceProof => ({ accepted: false, diagnostics: [diagnostic], consumerContext, deriveProvider: null, contractProvider: null, facadeProvider: null });
  input.checkCancellation?.();
  const graph = inspectRustModuleGraph(input.files, input.readSource, { strictManifests: true, checkCancellation: input.checkCancellation }), bindingCache = new Map<string, readonly CargoProviderBinding[]>();
  const contexts = (graph.contexts.get(input.origin.sourcePath) ?? []).filter((context) => context.manifestPath === input.consumerManifestLocator && context.sourceScope.join("\0") === input.origin.modulePath.join("\0"));
  if (contexts.length !== 1) return reject("wrapped declaration has no unique consumer module context");
  const consumerContext = contexts[0]!, source = input.readSource(input.origin.sourcePath);
  if (source === undefined) return reject("wrapped declaration source is unavailable from the consumer inventory", consumerContext);
  const sourceFacts = inspectRustMutationMetadataFacts(source), rootSource = input.readSource(consumerContext.crateRoot), rootAliases = rootSource === undefined || consumerContext.crateRoot === input.origin.sourcePath ? [] : inspectRustMutationMetadataFacts(rootSource).crateAliases.filter((item) => item.modulePath.length === 0 && (item.kind === "extern" || item.kind === "self")), facts: RustMutationMetadataFacts = { ...sourceFacts, crateAliases: [...sourceFacts.crateAliases, ...rootAliases] }, declaration = facts.declarations.filter((item) => item.name === input.origin.declarationName && item.modulePath.join("\0") === input.origin.modulePath.join("\0") && item.visibility === "pub" && !item.conditional);
  if (declaration.length !== 1) return reject("wrapped declaration is absent, conditional, private, or ambiguous", consumerContext);
  const item = declaration[0]!;
  if (item.mutationLeaf.state !== "valid") return reject("wrapped declaration has no unconditional valid mutation_leaf contract", consumerContext);
  const deriveRoutes = mutationMetadataResolvedRoutes(item.derives.flatMap((path) => mutationMetadataExternal(input.repositoryRoot, input.consumerManifestLocator, facts, item.modulePath, path, bindingCache, input.readSource).map((route) => mutationMetadataRoute(input, graph, route.binding, route.terminal, MUTATION_METADATA_PROVIDERS[2]!, bindingCache))));
  if (deriveRoutes?.length !== 1) return reject("metadata derive route is unresolved, ambiguous, or not canonical", consumerContext);
  const contractRoutes = mutationMetadataResolvedRoutes(item.mutationLeaf.contracts.flatMap((path) => mutationMetadataExternal(input.repositoryRoot, input.consumerManifestLocator, facts, item.modulePath, path, bindingCache, input.readSource, false, "MutationLeaf").map((route) => mutationMetadataRoute(input, graph, route.binding, route.terminal, MUTATION_METADATA_PROVIDERS[0]!, bindingCache))));
  if (contractRoutes?.length !== 1) return reject("mutation_leaf contract route is unresolved, ambiguous, or not canonical", consumerContext);
  const manual = inspectRustStructure(source).impls.filter((impl) => impl.selfType.split("::").at(-1) === item.name && impl.traitPath !== null).flatMap((impl) => mutationMetadataExternal(input.repositoryRoot, input.consumerManifestLocator, facts, item.modulePath, impl.traitPath!, bindingCache, input.readSource).map((route) => mutationMetadataRoute(input, graph, route.binding, route.terminal, MUTATION_METADATA_PROVIDERS[0]!, bindingCache))).some((route) => route !== null);
  if (manual) return reject("wrapped declaration manually implements the genuine MutationLeaf trait", consumerContext);
  const facades = contractRoutes.map((route) => route.facade).filter((provider): provider is MutationMetadataProviderIdentity => provider !== null);
  if (facades.length > 1) return reject("mutation_leaf contract has ambiguous facade routes", consumerContext);
  return { accepted: true, diagnostics: [], consumerContext, deriveProvider: deriveRoutes[0]!.provider, contractProvider: contractRoutes[0]!.provider, facadeProvider: facades[0] ?? null };
}
//#endregion 🧬️MutationMetadataSource
//#endregion 📦️CargoProviderManifestProjection

//#region 🧩️SemanticCollections
/** 🔗️ One resolved source dependency between repository-owned semantic components. */
export interface SemanticConsumerEdge {
  readonly from: string;
  readonly to: string;
  readonly source: string;
  readonly target: string;
  readonly mechanism: "static-import" | "path-attribute" | "project-reference" | "runtime-registration";
  readonly production: boolean;
}

/** 🕸️ Deterministic component graph used to prove production independence and module ownership. */
export interface SemanticConsumerGraph {
  readonly nodes: readonly string[];
  readonly edges: readonly SemanticConsumerEdge[];
}

/** 📋️ Structured semantic-policy finding; report and enforce consume the same records. */
export interface SemanticProblem {
  readonly code: string;
  readonly severity: "error" | "warning";
  readonly path: string;
  readonly componentId?: string;
  readonly message: string;
}

/** 📊️ Deterministic census row for one maximally specific semantic component. */
export interface SemanticCensusRecord {
  readonly id: string;
  readonly currentPath: string;
  readonly collectionPath: string;
  readonly kind: SemanticKind;
  readonly responsibility: string;
  readonly ownerAncestry: readonly string[];
  readonly languageMirrors: readonly string[];
  readonly packages: readonly string[];
  readonly provenance: "authored" | "generated" | "vendor" | "test" | "example";
  readonly publicSymbols: readonly string[];
  readonly schemaContracts: readonly string[];
  readonly staticImports: readonly string[];
  readonly runtimeMounts: readonly string[];
  readonly registrations: readonly string[];
  readonly packageEntrypoints: readonly string[];
  readonly reverseDependencies: readonly string[];
  readonly productionConsumers: readonly string[];
  readonly excludedConsumers: readonly string[];
  readonly currentOwner: string;
  readonly computedLowestCommonOwner: string | null;
  readonly proposedDisposition: "retain" | "split" | "inline" | "promote" | "relocate" | "regenerate" | "delete";
  readonly duplicateClusters: readonly string[];
  readonly applicableInstructions: readonly string[];
  readonly dirtyConflicts: readonly string[];
  readonly generatorInputs: readonly string[];
  readonly tests: readonly string[];
  readonly runtimeSurfaces: readonly string[];
  readonly leaseId: string | null;
}

/** 🧬️ Syntax-duplicate evidence; it never implies semantic equivalence or extraction. */
export interface SemanticDuplicateCluster {
  readonly id: string;
  readonly hash: string;
  readonly componentIds: readonly string[];
  readonly paths: readonly string[];
}

/** 🧰️ Complete deterministic semantic inventory and its validation graph. */
export interface SemanticCensus {
  readonly records: readonly SemanticCensusRecord[];
  readonly graph: SemanticConsumerGraph;
  readonly problems: readonly SemanticProblem[];
  readonly duplicates: readonly SemanticDuplicateCluster[];
}

/** 🦀️ A cumulative Rust `#[path]` resolution from one entry source. */
export interface RustResolvedPath {
  readonly specifier: string;
  readonly target: string;
}

interface SemanticSource {
  readonly abs: string;
  readonly rel: string;
  readonly content: string;
  readonly production: boolean;
}

interface SemanticRecordDraft {
  readonly id: string;
  readonly currentPath: string;
  readonly collectionPath: string;
  readonly collectionDirectory: string;
  readonly kind: SemanticKind;
  readonly responsibility: string;
  readonly member?: SemanticMember;
  readonly sourceFiles: readonly SemanticSource[];
  readonly currentOwner: string;
  readonly ownerAncestry: readonly string[];
}

interface SemanticManifestExtension {
  readonly kind: "collection";
  readonly members: readonly SemanticMember[];
}

interface SemanticResolverIndex {
  readonly packageRoots: ReadonlyMap<string, string>;
  readonly packageExports: ReadonlyMap<string, ReadonlyMap<string, string>>;
  readonly goModules: ReadonlyMap<string, string>;
  readonly pythonRoots: readonly string[];
  readonly tsPaths: readonly { readonly root: string; readonly pattern: string; readonly targets: readonly string[] }[];
}

const SEMANTIC_SKIP_DIRS = new Set(["node_modules", "target", "dist", ".git", ".nx", ".cache", "vendor", "pkg", "storybook-static", "temp"]);
const SEMANTIC_NON_PRODUCTION_SEGMENTS = new Set(["🧪️tests", "tests", "test", "__tests__", "📚️examples", "🧪️examples", "examples", "fixtures", "🧪️fixtures", "🤖️generated"]);

function semanticCompare(a: string, b: string): number {
  return a < b ? -1 : a > b ? 1 : 0;
}

function semanticUnique(values: Iterable<string>): string[] {
  return [...new Set(values)].sort(semanticCompare);
}

function semanticRel(repoRoot: string, path: string): string {
  return relative(repoRoot, path).replaceAll("\\", "/");
}

function semanticProductionPath(path: string): boolean {
  return !path.split("/").some((segment) => SEMANTIC_NON_PRODUCTION_SEGMENTS.has(segment) || /^\./u.test(segment));
}

function semanticProvenance(path: string): SemanticCensusRecord["provenance"] {
  const segments = path.split("/");
  if (segments.some((segment) => segment === "node_modules" || segment === "vendor")) return "vendor";
  if (segments.some((segment) => segment === "🤖️generated" || segment === "generated" || segment === "dist" || segment === "target")) return "generated";
  if (segments.some((segment) => segment === "🧪️tests" || segment === "tests" || segment === "test" || segment === "__tests__")) return "test";
  if (segments.some((segment) => segment === "📚️examples" || segment === "🧪️examples" || segment === "examples")) return "example";
  return "authored";
}

function semanticSourceExtensions(taxonomy: Taxonomy): Set<string> {
  const sourceKinds = Object.values(taxonomy.fileKinds).filter((kind) => kind.role === "source");
  return new Set([...sourceKinds.flatMap((kind) => kind.extensionChains.map((chain) => extname(`source${chain}`))), ".c", ".cc", ".cpp", ".h", ".hpp", ".csproj"]);
}

function semanticWalk(repoRoot: string, root: string, taxonomy: Taxonomy): string[] {
  const files: string[] = [];
  const visited = new Set<string>();
  const walk = (dir: string): void => {
    if (pathIsExcluded(repoRoot, dir, taxonomy)) return;
    let real: string;
    try {
      real = realpathSync(dir);
    } catch {
      return;
    }
    if (visited.has(real)) return;
    visited.add(real);
    for (const entry of readdirSafe(real).sort((a, b) => semanticCompare(a.name, b.name))) {
      const path = join(real, entry.name);
      if (pathIsExcluded(repoRoot, path, taxonomy)) continue;
      if (entry.isDirectory()) {
        if (!entry.name.startsWith(".") && !SEMANTIC_SKIP_DIRS.has(entry.name) && !CARGO_TARGET_DIR_PATTERN.test(entry.name)) walk(path);
      } else if (entry.isFile()) files.push(path);
    }
  };
  walk(root);
  return files.sort(semanticCompare);
}

function semanticCollectionAncestors(repoRoot: string, file: string, taxonomy: Taxonomy): string[] {
  const ancestors: string[] = [];
  let current = dirname(file);
  while (current.startsWith(repoRoot) && current !== repoRoot) {
    if (semanticCollectionSpec(current, taxonomy)) ancestors.push(current);
    current = dirname(current);
  }
  return ancestors;
}

/** 🧭️ Chooses the most-specific declared collection suffix for one on-disk collection root. */
function semanticCollectionSpec(path: string, taxonomy: Taxonomy): SemanticCollectionSpec | null {
  const segments = path.replaceAll("\\", "/").split("/").filter(Boolean);
  for (const [key, spec] of Object.entries(taxonomy.semanticCollections).sort(([a], [b]) => b.split("/").length - a.split("/").length || semanticCompare(a, b))) {
    const suffix = key.split("/");
    if (suffix.length <= segments.length && suffix.every((segment, index) => segments[segments.length - suffix.length + index] === segment)) return spec;
  }
  return null;
}

/** 🗺️ Every declared clean area root, with opaque prefixes filtered before existence checks. */
export function semanticActiveRoots(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const active = Object.entries(taxonomy.areas)
    .map(([path]) => path)
    .filter((path) => !pathIsExcluded(repoRoot, join(repoRoot, path), taxonomy))
    .filter((path) => existsSync(join(repoRoot, path)))
    .sort((a, b) => a.split("/").length - b.split("/").length || semanticCompare(a, b));
  return active.filter((path, index) => !active.some((candidate, other) => other < index && (path === candidate || path.startsWith(`${candidate}/`))));
}

function semanticOwnerAncestry(path: string): string[] {
  const segments = path.split("/").filter(Boolean);
  const owners: string[] = [];
  if (segments[0] === "🧰️framework") owners.push(segments[0]);
  if (segments[0] === "✏️s") owners.push(segments[0]);
  const collections = new Set(["🔌️plugins", "🛍️products", "🎛️apps", "🗿️artifacts", "🏅️standards", "🪆️subsets"]);
  for (let index = 0; index < segments.length - 1; index += 1) {
    if (collections.has(segments[index]!)) owners.push(segments.slice(0, index + 2).join("/"));
  }
  return semanticUnique(owners).sort((a, b) => a.split("/").length - b.split("/").length || semanticCompare(a, b));
}

function semanticOwnerLevel(path: string): SemanticOwnerLevel | null {
  const segments = path.split("/");
  const parent = segments.at(-2);
  if (parent === "🪆️subsets") return "subset";
  if (parent === "🏅️standards") return "standard";
  if (parent === "🗿️artifacts") return "artifact";
  if (parent === "🎛️apps") return "app";
  if (parent === "🔌️plugins") return "plugin";
  if (parent === "🛍️products") return "product";
  if (path === "✏️s") return "s";
  if (path === "🧰️framework") return "framework";
  return null;
}

function semanticLowestCommonOwner(records: readonly SemanticRecordDraft[]): string | null {
  if (records.length === 0) return null;
  const common = records[0]!.ownerAncestry.filter((owner) => records.every((record) => record.ownerAncestry.includes(owner)));
  return common.sort((a, b) => b.split("/").length - a.split("/").length || semanticCompare(a, b))[0] ?? null;
}

function semanticReadManifest(path: string, taxonomy: Taxonomy, problems: SemanticProblem[], collectionPath: string): SemanticManifestExtension | null {
  const filename = basename(path);
  if (!existsSync(path)) {
    problems.push({ code: "collection-manifest-missing", severity: "error", path: collectionPath, message: `Collection is missing canonical ${filename}.` });
    return null;
  }
  try {
    const parsed = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
    const extension = parsed[taxonomy.semanticExtensionKey] as Partial<SemanticManifestExtension> | undefined;
    if (!extension || extension.kind !== "collection" || !Array.isArray(extension.members)) {
      problems.push({ code: "collection-manifest-shape", severity: "error", path: semanticRel(dirname(dirname(path)), path), message: `${taxonomy.semanticExtensionKey} must be { kind: "collection", members: [...] }.` });
      return null;
    }
    return extension as SemanticManifestExtension;
  } catch (error) {
    problems.push({ code: "collection-manifest-invalid", severity: "error", path: collectionPath, message: `${filename} is not valid JSON: ${(error as Error).message}` });
    return null;
  }
}

function semanticMemberProblems(member: SemanticMember, spec: SemanticCollectionSpec, collectionPath: string, taxonomy: Taxonomy): SemanticProblem[] {
  const path = `${collectionPath}/${member.directory}`;
  const problems: SemanticProblem[] = [];
  const add = (code: string, message: string): void => {
    problems.push({ code, severity: "error", path, componentId: member.id, message });
  };
  if (!member.directory || member.directory.includes("*") || member.id.includes("*")) add("member-wildcard", "Member directory and id must be exact, non-wildcard values.");
  if (!member.id.trim()) add("member-id-empty", "Member id must be non-empty.");
  if (!member.responsibility?.trim()) add("member-responsibility-empty", "Member responsibility must be specific and non-empty.");
  if (member.kind !== spec.kind) add("member-kind-mismatch", `Member kind ${JSON.stringify(member.kind)} does not match collection kind ${JSON.stringify(spec.kind)}.`);
  if (member.kind === "inference" && (!member.inference || member.inference.inputs.length === 0 || !member.inference.target.trim())) add("inference-contract-missing", "Inference must declare non-empty inputs and one derived target.");
  if (member.kind === "mutation" && (!member.mutation?.command.trim() || !member.mutation.event.trim())) add("mutation-contract-missing", "Mutation must declare its command and emitted event.");
  if (member.kind === "io" && (!member.io?.format.trim() || !member.io.direction || member.io.direction !== spec.direction)) add("io-contract-missing", `I/O member must declare a format and direction ${JSON.stringify(spec.direction)}.`);
  if (member.kind === "module") {
    const consumers = semanticUnique(member.module?.productionConsumers ?? []);
    if (consumers.length < taxonomy.semanticConsumerMinimum) add("module-consumer-minimum", `Module declares ${consumers.length} independent production consumers; at least ${taxonomy.semanticConsumerMinimum} are required.`);
  }
  const stem = stripEmoji(member.directory).toLowerCase();
  if (taxonomy.bannedNameStems.includes(stem)) add("member-generic-stem", `Specific member uses banned generic stem ${JSON.stringify(stem)}.`);
  return problems;
}

function semanticAssemblyOnly(content: string, extension: string): boolean {
  const lines = content.split(/\r?\n/u).map((line) => line.trim()).filter((line) => line && !/^(\/\/|\/\*|\*|#region|#endregion|\/\/#[a-z])/u.test(line));
  if (extension === ".rs") return lines.every((line) => /^(#\[path\s*=|(?:pub\s+)?mod\s+\w+\s*(?:;|\{)|pub\s+use\s+|[\w:]+!\(|[)};,]+$)/u.test(line));
  if (extension === ".ts" || extension === ".tsx" || extension === ".js" || extension === ".jsx") return lines.every((line) => /^(import\s|export\s(?:\{|\*)|[};,]+$)/u.test(line));
  if (extension === ".py") return lines.every((line) => /^(from\s|import\s|__all__\s*=|[\[\],]+$)/u.test(line));
  return lines.length === 0;
}

/** 🧷️ Mechanical glue and collection assembly establish reachability but never qualify as a production consumer. */
function semanticProductionConsumer(source: SemanticSource, packages: readonly DiscoveredPackage[]): boolean {
  return source.production && !packages.some((pkg) => source.rel.startsWith(`${pkg.packageRel}/`)) && !semanticAssemblyOnly(source.content, extname(source.abs));
}

function semanticPublicSymbols(source: SemanticSource): string[] {
  const symbols: string[] = [];
  const patterns = source.rel.endsWith(".rs")
    ? [/\bpub\s+(?:struct|enum|trait|type|fn|const|static|mod)\s+([A-Za-z_][A-Za-z0-9_]*)/gu]
    : source.rel.endsWith(".go")
      ? [/\b(?:type|func|const|var)\s+([A-Z][A-Za-z0-9_]*)/gu]
      : source.rel.endsWith(".py")
        ? [/^class\s+([A-Za-z_][A-Za-z0-9_]*)/gmu, /^def\s+([A-Za-z_][A-Za-z0-9_]*)/gmu]
        : source.rel.endsWith(".cs")
          ? [/\bpublic\s+(?:class|record|struct|interface|enum)\s+([A-Za-z_][A-Za-z0-9_]*)/gu]
          : [/\bexport\s+(?:default\s+)?(?:class|interface|type|enum|function|const|let|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)/gu];
  for (const pattern of patterns) for (const match of source.content.matchAll(pattern)) if (match[1]) symbols.push(match[1]);
  return semanticUnique(symbols);
}

function semanticImportSpecs(source: SemanticSource): string[] {
  const specs: string[] = [];
  const patterns = source.rel.endsWith(".rs")
    ? [/#\[path\s*=\s*"([^"]+)"\]/gu]
    : source.rel.endsWith(".py")
      ? [/^from\s+(\.+[A-Za-z0-9_.]+)\s+import/gmu]
      : source.rel.endsWith(".csproj")
        ? [/<ProjectReference\s+Include="([^"]+)"/gu]
        : source.rel.endsWith(".go")
          ? [/"([^"\n]+)"/gu]
          : [/(?:import|export)\s+(?:[^"']*?\s+from\s+)?["']([^"']+)["']/gu, /import\s*\(\s*["']([^"']+)["']\s*\)/gu, /require\s*\(\s*["']([^"']+)["']\s*\)/gu];
  for (const pattern of patterns) for (const match of source.content.matchAll(pattern)) if (match[1]) specs.push(match[1]);
  return semanticUnique(specs);
}

/** 🦀️ Relative Rust namespaces are imports too; they must resolve to their semantic member, not a crate barrel. */
function semanticRustUseSpecs(source: SemanticSource): string[] {
  if (!source.rel.endsWith(".rs")) return [];
  const specs: string[] = [];
  for (const match of source.content.matchAll(/\b(?:pub\s+)?use\s+((?:super|self)(?:::[^;]+)+)\s*;/gu)) if (match[1]) specs.push(match[1].replace(/\s+/gu, " ").trim());
  return semanticUnique(specs);
}

/** 🧭️ Resolves a logical Rust namespace to its closest physical taxonomy directory. */
function semanticRustNamespaceDirectory(base: string, segment: string): string | null {
  let current = base;
  for (;;) {
    const child = readdirSafe(current).find((entry) => entry.isDirectory() && stripEmoji(entry.name).replaceAll("-", "_") === segment);
    if (child) return join(current, child.name);
    if (segment !== "modules") return null;
    const parent = dirname(current);
    if (parent === current) return null;
    current = parent;
  }
}

function semanticJson(path: string): Record<string, unknown> | null {
  try {
    const content = readFileSync(path, "utf8").replace(/\/\*[\s\S]*?\*\//gu, "").replace(/(^|\s)\/\/.*$/gmu, "$1").replace(/,\s*([}\]])/gu, "$1");
    return JSON.parse(content) as Record<string, unknown>;
  } catch {
    return null;
  }
}

function semanticFlattenExports(value: unknown, prefix = ".", result = new Map<string, string>()): ReadonlyMap<string, string> {
  if (typeof value === "string") result.set(prefix, value);
  else if (value && typeof value === "object") {
    for (const [key, child] of Object.entries(value as Record<string, unknown>)) {
      if (key.startsWith(".")) semanticFlattenExports(child, key, result);
      else if (["import", "default", "types", "bun", "node"].includes(key) && !result.has(prefix)) semanticFlattenExports(child, prefix, result);
    }
  }
  return result;
}

function semanticResolverIndex(allFiles: readonly string[], taxonomy: Taxonomy): SemanticResolverIndex {
  const packageRoots = new Map<string, string>();
  const packageExports = new Map<string, ReadonlyMap<string, string>>();
  const goModules = new Map<string, string>();
  const pythonRoots: string[] = [];
  const tsPaths: { readonly root: string; readonly pattern: string; readonly targets: readonly string[] }[] = [];
  const nodeManifest = exactContractFilename(taxonomy.ecosystems["🟦️typescript"]?.manifestContractId ?? null, taxonomy);
  const goManifest = exactContractFilename(taxonomy.ecosystems["🐹️go"]?.moduleRootContractId ?? null, taxonomy);
  const pythonManifest = exactContractFilename(taxonomy.ecosystems["🐍️python"]?.manifestContractId ?? null, taxonomy);
  const tsConfigContract = taxonomy.fixedFilenameContracts["typescript-config"];
  const tsConfig = tsConfigContract ? fixedContractFilename(tsConfigContract) : undefined;
  const defaultTypescriptEntry = taxonomy.ecosystems["🟦️typescript"]?.entryContractIds.map((id) => exactContractFilename(id, taxonomy)).find((filename): filename is string => Boolean(filename));
  for (const file of allFiles) {
    if (nodeManifest && basename(file) === nodeManifest) {
      const manifest = semanticJson(file);
      if (typeof manifest?.name === "string") {
        packageRoots.set(manifest.name, dirname(file));
        packageExports.set(manifest.name, semanticFlattenExports(manifest.exports ?? manifest.module ?? manifest.main ?? (defaultTypescriptEntry ? `./${defaultTypescriptEntry}` : undefined)));
      }
    } else if (goManifest && basename(file) === goManifest) {
      const module = readFileSync(file, "utf8").match(/^module\s+(\S+)/mu)?.[1];
      if (module) goModules.set(module, dirname(file));
    } else if (pythonManifest && basename(file) === pythonManifest) {
      pythonRoots.push(dirname(file));
    } else if (tsConfig && basename(file) === tsConfig) {
      const config = semanticJson(file);
      const compiler = config?.compilerOptions as Record<string, unknown> | undefined;
      const base = resolve(dirname(file), typeof compiler?.baseUrl === "string" ? compiler.baseUrl : ".");
      if (compiler?.paths && typeof compiler.paths === "object") {
        for (const [pattern, targets] of Object.entries(compiler.paths as Record<string, unknown>)) if (Array.isArray(targets)) tsPaths.push({ root: base, pattern, targets: targets.filter((target): target is string => typeof target === "string") });
      }
    }
  }
  return { packageRoots, packageExports, goModules, pythonRoots: semanticUnique(pythonRoots), tsPaths: tsPaths.sort((a, b) => b.root.length - a.root.length || semanticCompare(a.pattern, b.pattern)) };
}

function semanticRuntimeEvidence(source: SemanticSource, pattern: RegExp): string[] {
  const evidence: string[] = [];
  for (const [index, line] of source.content.split(/\r?\n/u).entries()) if (pattern.test(line)) evidence.push(`${source.rel}:${index + 1}`);
  return evidence;
}

/** 🦀️ Resolves nested Rust path attributes with the enclosing module's cumulative base. */
export function resolveRustPathAttributes(sourcePath: string, content: string): RustResolvedPath[] {
  const resolved: RustResolvedPath[] = [];
  const scopes: { readonly base: string; readonly depth: number }[] = [{ base: dirname(sourcePath), depth: 0 }];
  let depth = 0;
  let pending: string | null = null;
  for (const line of content.split(/\r?\n/u)) {
    const pathMatch = line.match(/#\[path\s*=\s*"([^"]+)"\]/u);
    if (pathMatch?.[1]) pending = pathMatch[1];
    const moduleMatch = line.match(/(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*([;{])/u);
    const base = scopes.at(-1)!.base;
    if (moduleMatch) {
      const name = moduleMatch[1]!;
      if (moduleMatch[2] === ";") {
        const specifier = pending ?? `${name}.rs`;
        resolved.push({ specifier, target: resolve(base, specifier) });
      } else {
        scopes.push({ base: resolve(base, pending ?? name), depth: depth + 1 });
      }
      pending = null;
    }
    depth += (line.match(/\{/gu) ?? []).length - (line.match(/\}/gu) ?? []).length;
    while (scopes.length > 1 && scopes.at(-1)!.depth > depth) scopes.pop();
  }
  return resolved.sort((a, b) => semanticCompare(a.target, b.target));
}

/** 🦀️ Resolves `use super::…` through emoji-prefixed sibling directories to immediate semantic component leaves. */
function resolveRustRelativeUses(source: SemanticSource, componentRoot: string, componentLeaves: ReadonlyMap<string, string>): RustResolvedPath[] {
  const resolved: RustResolvedPath[] = [];
  for (const specifier of semanticRustUseSpecs(source)) {
    const segments = specifier.split("::").map((segment) => segment.trim()).filter(Boolean);
    let index = 0;
    let base = componentRoot;
    if (segments[index] === "self") index += 1;
    else {
      while (segments[index] === "super") {
        base = dirname(base);
        index += 1;
      }
      if (index === 0) continue;
    }
    const tail = segments.slice(index).join("::");
    const braceAt = tail.indexOf("{");
    const path = (braceAt < 0 ? tail : tail.slice(0, braceAt)).replace(/::$/u, "");
    for (const segment of path.split("::").map((part) => part.trim()).filter(Boolean)) {
      const child = semanticRustNamespaceDirectory(base, segment);
      if (!child) break;
      base = child;
      const target = componentLeaves.get(base);
      if (target) {
        resolved.push({ specifier, target });
        break;
      }
    }
    if (braceAt >= 0) {
      for (const candidate of tail.slice(braceAt).matchAll(/[a-z][A-Za-z0-9_]*/gu)) {
        const child = readdirSafe(base).find((entry) => entry.isDirectory() && stripEmoji(entry.name).replaceAll("-", "_") === candidate[0]);
        if (!child) continue;
        const target = componentLeaves.get(join(base, child.name));
        if (target) resolved.push({ specifier, target });
      }
    }
  }
  return [...new Map(resolved.map((target) => [`${target.specifier}\0${target.target}`, target])).values()].sort((a, b) => semanticCompare(`${a.specifier}\0${a.target}`, `${b.specifier}\0${b.target}`));
}

function semanticResolveCandidate(from: SemanticSource, specifier: string, fileIndex: ReadonlyMap<string, string>, extensions: ReadonlySet<string>, resolvers: SemanticResolverIndex, taxonomy: Taxonomy): string | null {
  let normalized = specifier.replace(/[?#].*$/u, "");
  if (from.rel.endsWith(".py") && normalized.startsWith(".")) normalized = normalized.replace(/^\.+/u, "./").replaceAll(".", "/");
  const bases: string[] = [];
  if (normalized.startsWith(".") || normalized.startsWith("/")) bases.push(resolve(dirname(from.abs), normalized));
  else {
    for (const [name, root] of [...resolvers.packageRoots.entries()].sort((a, b) => b[0].length - a[0].length || semanticCompare(a[0], b[0]))) {
      if (normalized !== name && !normalized.startsWith(`${name}/`)) continue;
      const subpath = normalized === name ? "." : `./${normalized.slice(name.length + 1)}`;
      const defaultEntry = taxonomy.ecosystems["🟦️typescript"]?.entryContractIds.map((id) => exactContractFilename(id, taxonomy)).find((filename): filename is string => Boolean(filename));
      const target = resolvers.packageExports.get(name)?.get(subpath) ?? (subpath === "." && defaultEntry ? `./${defaultEntry}` : subpath);
      bases.push(resolve(root, target));
    }
    for (const [name, root] of [...resolvers.goModules.entries()].sort((a, b) => b[0].length - a[0].length || semanticCompare(a[0], b[0]))) if (normalized === name || normalized.startsWith(`${name}/`)) bases.push(resolve(root, normalized.slice(name.length).replace(/^\//u, "")));
    for (const mapping of resolvers.tsPaths) {
      if (!from.abs.startsWith(`${mapping.root}/`) && !from.abs.startsWith(`${dirname(mapping.root)}/`)) continue;
      const star = mapping.pattern.indexOf("*");
      const captured = star < 0 ? (normalized === mapping.pattern ? "" : null) : normalized.startsWith(mapping.pattern.slice(0, star)) && normalized.endsWith(mapping.pattern.slice(star + 1)) ? normalized.slice(star, normalized.length - mapping.pattern.slice(star + 1).length) : null;
      if (captured === null) continue;
      for (const target of mapping.targets) bases.push(resolve(mapping.root, target.replace("*", captured)));
    }
    if (from.rel.endsWith(".py")) for (const root of resolvers.pythonRoots.filter((root) => from.abs.startsWith(`${root}/`))) bases.push(resolve(root, normalized.replaceAll(".", "/")));
  }
  const candidates = [...bases];
  for (const base of bases) {
    for (const extension of extensions) candidates.push(`${base}${extension}`);
    for (const filename of componentFilenames(taxonomy)) candidates.push(join(base, filename));
    for (const contract of Object.values(taxonomy.configurableEntryContracts)) candidates.push(join(base, contract.filename));
  }
  for (const candidate of candidates) {
    let real = candidate;
    try {
      if (existsSync(candidate) && statSync(candidate).isFile()) real = realpathSync(candidate);
    } catch {
      continue;
    }
    const indexed = fileIndex.get(real);
    if (indexed) return indexed;
  }
  return null;
}

function semanticInstructions(repoRoot: string, componentPath: string): string[] {
  const instructions: string[] = [];
  let current = join(repoRoot, componentPath);
  while (current.startsWith(repoRoot)) {
    const candidate = join(current, "AGENTS.md");
    if (existsSync(candidate)) instructions.push(semanticRel(repoRoot, candidate));
    if (current === repoRoot) break;
    current = dirname(current);
  }
  return instructions.reverse();
}

function semanticNormalizeDuplicate(content: string): string {
  return content.replace(/\/\*[\s\S]*?\*\//gu, "").replace(/(^|\s)\/\/.*$/gmu, "$1").replace(/(^|\s)#(?!\[).*$/gmu, "$1").replace(/\s+/gu, "").trim();
}

function semanticDisposition(kind: SemanticKind, productionConsumers: readonly string[], currentOwner: string, lca: string | null): SemanticCensusRecord["proposedDisposition"] {
  if (kind !== "module") return "retain";
  if (productionConsumers.length === 0) return "delete";
  if (productionConsumers.length === 1) return "inline";
  return lca === currentOwner ? "retain" : "relocate";
}

/** 🕸️ Follows reverse module edges until independent non-module production components; intermediary modules never qualify. */
function semanticTerminalProductionConsumers(componentId: string, edges: readonly SemanticConsumerEdge[], drafts: ReadonlyMap<string, SemanticRecordDraft>): string[] {
  const incoming = new Map<string, SemanticConsumerEdge[]>();
  for (const edge of edges) incoming.set(edge.to, [...(incoming.get(edge.to) ?? []), edge]);
  const terminals = new Set<string>();
  const visited = new Set<string>([componentId]);
  const visit = (target: string): void => {
    for (const edge of incoming.get(target) ?? []) {
      if (!edge.production || visited.has(edge.from)) continue;
      visited.add(edge.from);
      if (drafts.get(edge.from)?.kind === "module") visit(edge.from);
      else terminals.add(edge.from);
    }
  };
  visit(componentId);
  return semanticUnique(terminals);
}

//#region 🧭️SemanticScope
function semanticScopeMatchesId(id: string, scope: string): boolean {
  return id === scope || id.startsWith(`${scope}.`);
}

function semanticCommonPath(paths: readonly string[]): string | null {
  const [first, ...remaining] = paths.map((path) => path.split("/").filter(Boolean));
  if (!first) return null;
  const common = first.filter((segment, index) => remaining.every((candidate) => candidate[index] === segment));
  return common.length === 0 ? null : common.join("/");
}

/** 🧭️ Resolves a semantic-id scope to its real owner boundary so unclassified collection findings remain visible. */
function semanticScopeRoots(records: readonly SemanticCensusRecord[], scope: string): string[] {
  const matched = records.filter((record) => semanticScopeMatchesId(record.id, scope) || record.currentPath === scope || record.currentPath.startsWith(`${scope}/`));
  if (matched.length === 0) return [];
  const ownerName = scope.split(".").filter(Boolean).at(-1);
  const ownerPaths = ownerName ? matched.flatMap((record) => record.ownerAncestry.filter((owner) => stripEmoji(basename(owner)) === ownerName)) : [];
  const root = semanticCommonPath(ownerPaths.length > 0 ? ownerPaths : matched.map((record) => record.currentPath));
  return root ? [root] : [];
}

function semanticPathInRoots(path: string, roots: readonly string[]): boolean {
  return roots.some((root) => path === root || path.startsWith(`${root}/`));
}
//#endregion 🧭️SemanticScope

/** 📊️ Builds the timestamp-free semantic census from taxonomy-defined active scope. */
export function buildSemanticCensus(repoRoot: string, options: { readonly scope?: string } = {}, taxonomy: Taxonomy = loadTaxonomy()): SemanticCensus {
  repoRoot = realpathSync(repoRoot);
  const problems: SemanticProblem[] = validateTaxonomy(taxonomy).map((message) => ({ code: "taxonomy-schema", severity: "error", path: "📋️project.json#metadata.semio.taxonomy", message }));
  for (const pkgProblem of discoverPackageProblems(repoRoot, taxonomy)) {
    problems.push({
      code: pkgProblem.kind,
      severity: "error",
      path: pkgProblem.path,
      message: pkgProblem.message,
    });
  }
  const extensions = semanticSourceExtensions(taxonomy);
  const allFiles = semanticActiveRoots(repoRoot, taxonomy).flatMap((active) => semanticWalk(repoRoot, join(repoRoot, active), taxonomy));
  const sourceFiles: SemanticSource[] = allFiles
    .filter((path) => extensions.has(extname(path)))
    .map((abs) => ({ abs: realpathSync(abs), rel: semanticRel(repoRoot, abs), content: readFileSync(abs, "utf8"), production: semanticProductionPath(semanticRel(repoRoot, abs)) }))
    .sort((a, b) => semanticCompare(a.rel, b.rel));
  const collectionDirs = semanticUnique(allFiles.flatMap((file) => semanticCollectionAncestors(repoRoot, file, taxonomy)).map((dir) => realpathSync(dir)));
  const packages = discoverPackages(repoRoot, taxonomy);
  const drafts: SemanticRecordDraft[] = [];
  for (const collectionAbs of collectionDirs) {
    const collectionPath = semanticRel(repoRoot, collectionAbs);
    const collectionDirectory = basename(collectionAbs);
    const spec = semanticCollectionSpec(collectionAbs, taxonomy)!;
    const manifestFilename = semanticManifestFilenameForCollection(collectionPath, taxonomy);
    const manifest = semanticReadManifest(join(collectionAbs, manifestFilename), taxonomy, problems, collectionPath);
    const actualChildren = readdirSafe(collectionAbs)
      .filter((entry) => entry.isDirectory() && !entry.name.startsWith(".") && !SEMANTIC_SKIP_DIRS.has(entry.name) && !CARGO_TARGET_DIR_PATTERN.test(entry.name) && entry.name !== taxonomy.packagesDirName && entry.name !== "🤖️generated")
      .map((entry) => entry.name)
      .sort(semanticCompare);
    const declaredMembers = manifest?.members ?? [];
    const declaredDirs = declaredMembers.map((member) => member.directory);
    if (actualChildren.length === 0) problems.push({ code: "collection-empty", severity: "error", path: collectionPath, message: "Semantic collection has no specific members." });
    for (const duplicate of semanticUnique(declaredDirs.filter((directory, index) => declaredDirs.indexOf(directory) !== index))) problems.push({ code: "member-directory-duplicate", severity: "error", path: collectionPath, message: `Manifest declares directory ${JSON.stringify(duplicate)} more than once.` });
    const ids = declaredMembers.map((member) => member.id);
    for (const duplicate of semanticUnique(ids.filter((id, index) => ids.indexOf(id) !== index))) problems.push({ code: "member-id-duplicate", severity: "error", path: collectionPath, message: `Manifest declares semantic id ${JSON.stringify(duplicate)} more than once.` });
    for (const directory of actualChildren.filter((directory) => !declaredDirs.includes(directory))) problems.push({ code: "manifest-child-missing", severity: "error", path: `${collectionPath}/${directory}`, message: `Direct child is not declared in ${manifestFilename}.` });
    for (const directory of declaredDirs.filter((directory) => !actualChildren.includes(directory))) problems.push({ code: "manifest-child-extra", severity: "error", path: `${collectionPath}/${directory}`, message: `Manifest member has no exact child directory.` });
    for (const member of declaredMembers) problems.push(...semanticMemberProblems(member, spec, collectionPath, taxonomy));
    const rootSources = sourceFiles.filter((source) => dirname(source.abs) === collectionAbs);
    for (const source of rootSources) if (!semanticAssemblyOnly(source.content, extname(source.abs))) problems.push({ code: "collection-authored-behavior", severity: "error", path: source.rel, message: "Collection language leaf contains authored behavior; list roots may contain generated/mechanical assembly only." });
    for (const directory of actualChildren) {
      const currentPath = `${collectionPath}/${directory}`;
      const member = declaredMembers.find((candidate) => candidate.directory === directory);
      const memberAbs = join(collectionAbs, directory);
      const nestedCollections = collectionDirs.filter((candidate) => candidate !== collectionAbs && candidate.startsWith(`${memberAbs}/`));
      const memberSources = sourceFiles.filter((source) => source.abs.startsWith(`${memberAbs}/`) && !nestedCollections.some((nested) => source.abs === nested || source.abs.startsWith(`${nested}/`)));
      const leafNames = new Set(componentFilenames(taxonomy));
      if (!memberSources.some((source) => dirname(source.abs) === memberAbs && leafNames.has(basename(source.abs)))) problems.push({ code: "member-component-leaf-missing", severity: "error", path: currentPath, componentId: member?.id, message: "Specific member has no immediate canonical component language leaf." });
      if (memberSources.some((source) => semanticProvenance(source.rel) === "generated") && !member?.generator) problems.push({ code: "generated-provenance-missing", severity: "error", path: currentPath, componentId: member?.id, message: "Generated source requires exact generator provenance in the semantic member manifest." });
      const currentOwner = semanticRel(repoRoot, dirname(collectionAbs));
      drafts.push({ id: member?.id || currentPath, currentPath, collectionPath, collectionDirectory, kind: member?.kind ?? spec.kind, responsibility: member?.responsibility ?? stripEmoji(directory), member, sourceFiles: memberSources, currentOwner, ownerAncestry: semanticOwnerAncestry(currentPath) });
    }
  }
  const memberRoots = drafts.map((draft) => [realpathSync(join(repoRoot, draft.currentPath)), draft.id] as const).sort((a, b) => b[0].length - a[0].length || semanticCompare(a[0], b[0]));
  const sourceToComponent = new Map<string, string>();
  const sourceComponentRoots = new Map<string, string>();
  const componentLeaves = new Map<string, string>();
  const leafNames = new Set(componentFilenames(taxonomy));
  for (const source of sourceFiles) {
    const owner = memberRoots.find(([root]) => source.abs === root || source.abs.startsWith(`${root}/`));
    if (owner) {
      sourceToComponent.set(source.abs, owner[1]);
      sourceComponentRoots.set(source.abs, owner[0]);
      if (dirname(source.abs) === owner[0] && leafNames.has(basename(source.abs)) && source.rel.endsWith(".rs")) componentLeaves.set(owner[0], source.abs);
    } else if (leafNames.has(basename(source.abs)) && !source.rel.includes(`/${taxonomy.packagesDirName}/`)) problems.push({ code: "unclassified-component-leaf", severity: "error", path: source.rel, message: "Authored component leaf is not owned by a recognized <collection>/<specific> member." });
  }
  const fileIndex = new Map(sourceFiles.map((source) => [source.abs, source.abs] as const));
  const resolvers = semanticResolverIndex(allFiles, taxonomy);
  const draftById = new Map(drafts.map((draft) => [draft.id, draft] as const));
  const edges: SemanticConsumerEdge[] = [];
  for (const source of sourceFiles) {
    const from = sourceToComponent.get(source.abs);
    if (!from) continue;
    const production = semanticProductionConsumer(source, packages);
    const pathTargets = source.rel.endsWith(".rs") ? resolveRustPathAttributes(source.abs, source.content) : [];
    for (const pathTarget of pathTargets) {
      let targetAbs = pathTarget.target;
      try {
        if (existsSync(targetAbs)) targetAbs = realpathSync(targetAbs);
      } catch {
        continue;
      }
      const to = sourceToComponent.get(targetAbs);
      if (to && to !== from) edges.push({ from, to, source: source.rel, target: semanticRel(repoRoot, targetAbs), mechanism: "path-attribute", production });
    }
    for (const specifier of semanticImportSpecs(source)) {
      const targetAbs = semanticResolveCandidate(source, specifier, fileIndex, extensions, resolvers, taxonomy);
      if (!targetAbs) continue;
      const to = sourceToComponent.get(targetAbs);
      if (to && to !== from) {
        const target = semanticRel(repoRoot, targetAbs);
        edges.push({ from, to, source: source.rel, target, mechanism: source.rel.endsWith(".csproj") ? "project-reference" : "static-import", production });
        if (/\b(?:register|mount)\s*\(/u.test(source.content)) edges.push({ from, to, source: source.rel, target, mechanism: "runtime-registration", production });
      }
    }
    const componentRoot = sourceComponentRoots.get(source.abs);
    if (componentRoot) for (const useTarget of resolveRustRelativeUses(source, componentRoot, componentLeaves)) {
      const to = sourceToComponent.get(useTarget.target);
      if (to && to !== from) edges.push({ from, to, source: source.rel, target: semanticRel(repoRoot, useTarget.target), mechanism: "static-import", production });
    }
  }
  const uniqueEdges = [...new Map(edges.map((edge) => [`${edge.from}\0${edge.to}\0${edge.source}\0${edge.target}\0${edge.mechanism}`, edge])).values()].sort((a, b) => semanticCompare(`${a.from}\0${a.to}\0${a.source}`, `${b.from}\0${b.to}\0${b.source}`));
  const duplicateFiles = new Map<string, SemanticSource[]>();
  for (const source of sourceFiles) {
    const normalized = semanticNormalizeDuplicate(source.content);
    if (normalized.length < 80 || !sourceToComponent.has(source.abs)) continue;
    const hash = createHash("sha256").update(normalized).digest("hex");
    duplicateFiles.set(hash, [...(duplicateFiles.get(hash) ?? []), source]);
  }
  const duplicates: SemanticDuplicateCluster[] = [...duplicateFiles.entries()]
    .map(([hash, sources]) => ({ hash, componentIds: semanticUnique(sources.map((source) => sourceToComponent.get(source.abs)!).filter(Boolean)), paths: semanticUnique(sources.map((source) => source.rel)) }))
    .filter((cluster) => cluster.componentIds.length > 1)
    .map((cluster) => ({ id: `duplicate-${cluster.hash.slice(0, 16)}`, ...cluster }))
    .sort((a, b) => semanticCompare(a.id, b.id));
  const records: SemanticCensusRecord[] = drafts.map((draft) => {
    const incoming = uniqueEdges.filter((edge) => edge.to === draft.id);
    const productionConsumers = draft.kind === "module"
      ? semanticTerminalProductionConsumers(draft.id, uniqueEdges, draftById)
      : semanticUnique(incoming.filter((edge) => edge.production).map((edge) => edge.from));
    const excludedConsumers = semanticUnique(incoming.filter((edge) => !edge.production).map((edge) => edge.from));
    const consumerRecords = productionConsumers.map((id) => draftById.get(id)).filter((record): record is SemanticRecordDraft => Boolean(record));
    const lca = semanticLowestCommonOwner(consumerRecords);
    const declaredConsumers = semanticUnique(draft.member?.module?.productionConsumers ?? []);
    if (draft.kind === "module") {
      const currentLevel = semanticOwnerLevel(draft.currentOwner);
      if (!currentLevel || !taxonomy.semanticAllowedOwnerLevels.includes(currentLevel)) problems.push({ code: "module-owner-level", severity: "error", path: draft.currentPath, componentId: draft.id, message: `Module owner ${JSON.stringify(draft.currentOwner)} is not an allowed semantic owner level.` });
      if (declaredConsumers.join("\0") !== productionConsumers.join("\0")) problems.push({ code: "module-consumer-graph-mismatch", severity: "error", path: draft.currentPath, componentId: draft.id, message: `Declared production consumers (${declaredConsumers.join(", ") || "none"}) do not match resolved graph (${productionConsumers.join(", ") || "none"}).` });
      if (productionConsumers.length < taxonomy.semanticConsumerMinimum) problems.push({ code: "module-production-consumer-minimum", severity: "error", path: draft.currentPath, componentId: draft.id, message: `Resolved reverse closure reaches ${productionConsumers.length} independent production components; ${taxonomy.semanticConsumerMinimum} are required.` });
      if (productionConsumers.length >= taxonomy.semanticConsumerMinimum && lca !== draft.currentOwner) problems.push({ code: "module-lowest-common-owner", severity: "error", path: draft.currentPath, componentId: draft.id, message: `Module is owned by ${JSON.stringify(draft.currentOwner)} but consumers compute ${JSON.stringify(lca)}.` });
    }
    const languageMirrors = semanticUnique(draft.sourceFiles.map((source) => Object.entries(taxonomy.componentFileKinds).find(([, kindId]) => kindId === fileKindIdForFilename(basename(source.abs), taxonomy))?.[0]).filter((value): value is string => Boolean(value)));
    const ownerPackages = packages.filter((pkg) => draft.currentPath === pkg.ownerRel || draft.currentPath.startsWith(`${pkg.ownerRel}/`) || pkg.ownerRel.startsWith(`${draft.currentPath}/`)).map((pkg) => `${pkg.role}:${pkg.ownerRel}${pkg.target ? `#${pkg.target}` : ""}`);
    const duplicateClusters = duplicates.filter((cluster) => cluster.componentIds.includes(draft.id)).map((cluster) => cluster.id);
    const staticImports = semanticUnique(draft.sourceFiles.flatMap((source) => [...semanticImportSpecs(source), ...semanticRustUseSpecs(source)]));
    const runtimeMounts = semanticUnique(draft.sourceFiles.flatMap((source) => semanticRuntimeEvidence(source, /\bmount(?:ed|ing)?\b|\.mount\s*\(/iu)));
    const registrations = semanticUnique(draft.sourceFiles.flatMap((source) => semanticRuntimeEvidence(source, /\bregister(?:ed|ing)?\b|\.register\s*\(|plugin_exports!|inventory::submit/iu)));
    return {
      id: draft.id,
      currentPath: draft.currentPath,
      collectionPath: draft.collectionPath,
      kind: draft.kind,
      responsibility: draft.responsibility,
      ownerAncestry: draft.ownerAncestry,
      languageMirrors,
      packages: semanticUnique(ownerPackages),
      provenance: semanticProvenance(draft.currentPath),
      publicSymbols: semanticUnique(draft.sourceFiles.flatMap(semanticPublicSymbols)),
      schemaContracts: semanticUnique(draft.sourceFiles.filter((source) => [".json", ".proto", ".graphql"].includes(extname(source.abs)) || source.rel.endsWith(".semio")).map((source) => source.rel)),
      staticImports,
      runtimeMounts,
      registrations,
      packageEntrypoints: [],
      reverseDependencies: semanticUnique(incoming.map((edge) => edge.source)),
      productionConsumers,
      excludedConsumers,
      currentOwner: draft.currentOwner,
      computedLowestCommonOwner: lca,
      proposedDisposition: semanticDisposition(draft.kind, productionConsumers, draft.currentOwner, lca),
      duplicateClusters,
      applicableInstructions: semanticInstructions(repoRoot, draft.currentPath),
      dirtyConflicts: [],
      generatorInputs: draft.member?.generator ? [draft.member.generator] : [],
      tests: semanticUnique(draft.sourceFiles.filter((source) => semanticProvenance(source.rel) === "test").map((source) => source.rel)),
      runtimeSurfaces: semanticUnique([...runtimeMounts, ...registrations]),
      leaseId: null,
    };
  }).sort((a, b) => semanticCompare(a.id, b.id));
  const scopedRecords = options.scope ? records.filter((record) => semanticScopeMatchesId(record.id, options.scope!) || record.currentPath === options.scope || record.currentPath.startsWith(`${options.scope}/`)) : records;
  const scopedIds = new Set(scopedRecords.map((record) => record.id));
  const scopedRoots = options.scope ? semanticScopeRoots(records, options.scope) : [];
  const scopedProblems = problems.filter((problem) => !options.scope || semanticPathInRoots(problem.path, scopedRoots) || (problem.componentId !== undefined && semanticScopeMatchesId(problem.componentId, options.scope)));
  return {
    records: scopedRecords,
    graph: { nodes: scopedRecords.map((record) => record.id), edges: uniqueEdges.filter((edge) => scopedIds.has(edge.from) || scopedIds.has(edge.to)) },
    problems: scopedProblems.sort((a, b) => semanticCompare(`${a.path}\0${a.code}\0${a.message}`, `${b.path}\0${b.code}\0${b.message}`)),
    duplicates: duplicates.filter((cluster) => cluster.componentIds.some((id) => scopedIds.has(id))),
  };
}

/** 🗃️ Stable machine-readable census representation. */
export function renderSemanticCensusJson(census: SemanticCensus): string {
  return `${JSON.stringify(census, null, 2)}\n`;
}

function semanticMarkdownCell(value: string): string {
  return value.replaceAll("|", "\\|").replaceAll("\n", " ");
}

/** 📓️ Stable human-readable companion for the machine census. */
export function renderSemanticCensusMarkdown(census: SemanticCensus): string {
  const lines = [
    "# Semantic Census",
    "",
    `- Components: ${census.records.length}`,
    `- Consumer edges: ${census.graph.edges.length}`,
    `- Problems: ${census.problems.length}`,
    `- Duplicate evidence clusters: ${census.duplicates.length}`,
    "",
    "| Semantic ID | Kind | Current path | Owner | Production consumers | Disposition |",
    "|---|---|---|---|---:|---|",
    ...census.records.map((record) => `| ${semanticMarkdownCell(record.id)} | ${record.kind} | ${semanticMarkdownCell(record.currentPath)} | ${semanticMarkdownCell(record.currentOwner)} | ${record.productionConsumers.length} | ${record.proposedDisposition} |`),
    "",
  ];
  return `${lines.join("\n")}\n`;
}

/** 🧬️ Stable machine-readable duplicate-candidate representation. */
export function renderSemanticDuplicatesJson(census: SemanticCensus): string {
  return `${JSON.stringify({ duplicates: census.duplicates }, null, 2)}\n`;
}

/** 📓️ Stable duplicate evidence companion without semantic conclusions. */
export function renderSemanticDuplicatesMarkdown(census: SemanticCensus): string {
  const lines = ["# Semantic Duplicate Evidence", "", "Similarity is evidence only. It never authorizes extraction, relocation, or deletion.", ""];
  for (const cluster of census.duplicates) {
    lines.push(`## ${cluster.id}`, "", `- SHA-256: \`${cluster.hash}\``, `- Components: ${cluster.componentIds.join(", ")}`, "", ...cluster.paths.map((path) => `- ${path}`), "");
  }
  if (census.duplicates.length === 0) lines.push("No cross-component exact-syntax clusters found.", "");
  return `${lines.join("\n")}\n`;
}

/** 🚦️ Stable report shared by non-blocking report and blocking enforce modes. */
export function renderSemanticTaxonomyReport(census: SemanticCensus, scope?: string): string {
  const lines = ["# Semantic Taxonomy Report", "", `- Mode: report`, `- Scope: ${scope ?? "all active taxonomy areas"}`, `- Components: ${census.records.length}`, `- Errors: ${census.problems.filter((problem) => problem.severity === "error").length}`, `- Warnings: ${census.problems.filter((problem) => problem.severity === "warning").length}`, "", "## Findings", ""];
  if (census.problems.length === 0) lines.push("No findings.");
  else for (const problem of census.problems) lines.push(`- [${problem.severity}] ${problem.code} — ${problem.path}: ${problem.message}`);
  return `${lines.join("\n")}\n`;
}
//#endregion 🧩️SemanticCollections
