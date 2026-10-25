import type { PackageDirectoryAdmission, PackageImplementationAdmission } from "../🧱️package-implementation-admission/🟦️.ts";
/** 🗺️ Repository-wide enforcement state; version 7 has no quiet or transitional area mode. */
/** 🧭️ `clean` is the enforced state of every production area; `exempt` names a subtree the discovery library itself skips. */
export type AreaState = "clean" | "exempt";

/** 📈️ Derived (never declared) state of one already-migrated owner: `clean` once nothing but the taxonomy shape is left, `mixed` while residuals survive — see `discoverOwners`. */
export type PackageMaturity = "clean" | "mixed";

/** 🧩️ Semantic responsibility owned by one collection member. */
export type SemanticKind = "inference" | "mutation" | "io" | "module" | "artifact" | "standard" | "subset" | "plugin" | "product" | "extension" | "capability" | "ui" | "action" | "app" | "command";

/** 🚪️ Exact boundary direction of one I/O semantic collection. */
export type SemanticIoDirection = "import" | "export" | "transport";

/** 🎚️ Lowest legal owner of a reusable module. */
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

/** 📍️ Where an ecosystem's semio role marker lives: a `table` inside the package manifest, addressed dotted (`package.metadata.semio` for TOML, `metadata.semio` for JSON). */
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

/** 📕️ Contextual source-name rule selecting one global file kind without extension guessing. */
export interface FileKindResolutionRuleSpec {
  readonly extensionChain: string;
  readonly fileKindId: string;
  readonly priority: 0;
}

/** 🎟️ Owner-scoped file kind whose suffix or role is never admitted globally. */
export interface ScopedFileKindSpec {
  readonly pathPattern: string;
  readonly parentDirectoryKindId?: string;
  readonly emoji: string;
  readonly extensionChains: readonly string[];
  readonly role: FileKindSpec["role"] | "evidence";
  readonly sourceFilenamePattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly verification: string;
  readonly expires: string | null;
}

/** 📂️ Owner-local semantic member identity loaded after structural directory kinds: exact `memberNames` always win, then the optional anchored `memberSlugPattern` admits any RGI emoji followed by a matching slug below the nearest owner scope; two pattern rows matching at the same nearest owner distance are ambiguous. */
export interface SemanticDirectoryMemberKindSpec {
  readonly ownerKindIds: readonly string[];
  readonly memberNames: readonly string[];
  readonly memberSlugPattern?: string;
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

/** ➡️ One literal or captured source segment in a projection grammar. */
export type SemanticProjectionSourceSegment =
  | Readonly<{ kindId: string; literal: string }>
  | Readonly<{ kindId: string; capture: SemanticProjectionCaptureField }>
  | Readonly<{ memberKindId: string; literal: string }>
  | Readonly<{ projectedMemberKindId: string; capture: SemanticProjectionCaptureField }>;

/** ⏩️ One literal, forward-rendered, or identity-copied destination segment. */
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

/** 🌿️ One physical-format leaf inside an exact projected bundle. */
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

export type SemanticDescendantNode = SemanticDescendantDirectoryNode | SemanticDescendantKindFileNode | SemanticDescendantFixedFileNode;

/** ↔️ Exactly-one group within an otherwise required descendant bundle. */
export interface SemanticDescendantAlternative {
  readonly id: string;
  readonly mode: "exactly-one";
  readonly nodes: readonly SemanticDescendantNode[];
}

/** 💎️ Exact physical descendant shape and its derived deepest suffix reserve. */
export interface SemanticExactDescendantContract {
  readonly rootDirectoryKindId: string;
  readonly requiredNodes: readonly SemanticDescendantNode[];
  readonly exclusiveAlternatives: readonly SemanticDescendantAlternative[];
  readonly realizedNodeCount: number;
  readonly pathBudgetReserve: Readonly<{ derivation: "longest-canonical-descendant-suffix"; utf16Units: number }>;
}

/** 📖️ A manifest-owned recursive catalog whose realized descendants are data-driven. */
export interface SemanticCatalogDescendantContract {
  readonly contractKind: "catalog";
  readonly rootDirectoryKindId: string;
  readonly catalogContractId: string;
  readonly leafFileKindId: string;
  readonly rendering: "semantic-member-directory-and-physical-kind-leaf";
  readonly pathBudgetReserve: Readonly<{ derivation: "longest-rendered-catalog-descendant-suffix"; utf16Units: number }>;
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
  readonly rationaleRule: "artifact-example-model-catalog-projection-v1" | "artifact-editor-command-projection-v1";
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
export interface CurrentSourceApproval {
  readonly sourcePath: string;
  readonly baselineCommit: string;
  readonly baselineBlob: string;
  readonly baselinePreimage: Readonly<{ sha256: string; size: number; mode: "0644" }>;
  readonly currentPreimage: Readonly<{ sha256: string; size: number; mode: "0644" }>;
  readonly approvalPath: string;
  readonly approvalSha256: string;
}

/** 🧾️ Exact observed expectation bytes and no-follow node witnesses supplied by the caller. */
export interface CurrentSourceApprovalSnapshot {
  readonly path: string;
  readonly nodeKind: string;
  readonly mode: number;
  readonly ancestorNodeKinds: readonly string[];
  readonly bytes: Uint8Array;
}

/** 📸️ Exact bytes and file tuple captured through one no-follow descriptor. */
export interface SemanticOwnedInputFileSnapshot extends CurrentSourceApprovalSnapshot {
  readonly nodeKind: "file";
  readonly contentHash: string;
  readonly size: number;
}

/** ☑️ A selected source tuple and its revision digest without altering historical catalog rows. */
export interface CurrentSourceApprovalResult {
  readonly disposition: "none" | "catalog" | "revised" | "problem";
  readonly sourcePath: string | null;
  readonly preimage: Readonly<{ sha256: string; size: number; mode: "0644" }> | null;
  readonly revisionId: string | null;
  readonly revisionDigest: string | null;
  readonly problems: readonly string[];
}

/** 🔐️ One digest-locked exact owner-file catalog with no basename-wide inference. */
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
  readonly referenceOwnerIds: readonly string[];
  readonly generatorOwnerIds: readonly string[];
  readonly authoredDocumentCorrections: Readonly<Record<string, SemanticOwnedDocumentCorrection>>;
  readonly currentSourceRevisions?: Readonly<Record<string, CurrentSourceApproval>>;
  readonly rationaleRule: "readme-license-owner-projection-v1";
}

/** 🧬️ One primary physical leaf governed directly by its semantic owner. */
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

/** ⛓️ Exact manifest-user coordinates in authoritative dependency approval state. */
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

export type SemanticPathProjectionReferenceConsumerForm = "path-reference" | "artifact-catalog-glob" | "artifact-catalog-prose:root-marker" | "artifact-catalog-prose:relative-root" | "artifact-catalog-prose:category-glob" | "artifact-catalog-prose:catalog-grammar";

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

/** 🥒️ How a committed fixture bundle is covered when no per-case implementation case holds it: the subset's exhaustive mutation case declares it in a feature, the subset's fixture runner discovers it at run time, or the case table of its leaf lists it. */
export interface MutationFixtureCoverageContract {
  readonly implementationCase: string;
  readonly exhaustiveCase: Readonly<{ featureFileKindId: string; stepUriPrefix: string; bundleFacetDirectoryNames: readonly string[] }>;
  readonly fixtureRunner: Readonly<{ relativePath: string }>;
  readonly leafCaseTable: Readonly<{ fileKindId: string; casesField: string }>;
}

/** 🔗️ Exact canonical implementation/fixture pair consumed by MutationCatalog validation. */
export interface MutationCatalogProjectionContractIds {
  readonly contractKind: "canonical-mutation-case-pair";
  readonly contractId: "canonical-mutation-case-pair-v1";
  readonly sourceOwnerKindId: string;
  readonly projectedMemberKindId: string;
  readonly implementationSegments: readonly SemanticProjectionSourceSegment[];
  readonly fixtureSegments: readonly SemanticProjectionSourceSegment[];
  readonly implementationDescendantContractId: string;
  readonly fixtureDescendantContractId: string;
  readonly catalogContractId: string;
  readonly coverage: "every-catalog-vector-has-one-fixture-bundle-with-declared-coverage";
  readonly fixtureCoverage: MutationFixtureCoverageContract;
}

/** 💊️ Tagged scope predicate for an exact externally-authoritative path contract. */
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

/** ⚓️ Exact externally-authoritative filename at one semantic scope. */
export interface FixedFilenameContract {
  readonly pathPattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly configurability: "unconfigurable";
  readonly scope: FixedContractScope;
  readonly verification: string;
  readonly expires: string | null;
}

/** 📜️ Exact externally-authoritative directory path at one semantic scope. */
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

/** ⚖️ Recursive package boundary contract; uncertain content is always a problem. */
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
  readonly implementationAdmission: PackageImplementationAdmission | null;
  readonly directoryAdmissions: readonly PackageDirectoryAdmission[];
}

/** 🧠️ Conservative source-role grammar selected by ecosystem. */
export interface PackageGlueGrammarSpec {
  readonly analyzer: "rust" | "typescript" | "javascript" | "go" | "python" | "dotnet" | "c-cpp" | "tex" | "css";
  readonly allowedRoles: readonly ("declaration" | "registration" | "bootstrap" | "thin-delegation")[];
  readonly maxDelegationStatements: number;
}

/** 🪵️ Explicit authority and validator for a source-format fixed or configurable package entry. */
export interface PackageSourceDisposition {
  readonly contractKind: "fixed" | "configurable";
  readonly disposition: "adapter-source" | "tool-metadata";
  readonly validator: "package-glue" | "command-router" | "vitest-configuration" | "tool-config-vitest" | "tool-config-tailwind" | "tool-config-postcss" | "tool-config-eslint" | "tool-config-dependency-cruiser" | "pytest-configuration" | "eslint-configuration";
  readonly grammarId?: string;
  readonly authority: string;
  readonly verification: string;
}

/** 👤️ Native package boundary declared before an authoritative language-directory identity exists. */
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

/** 🧹️ Whether one generator is runnable by normalization or retained only as a fail-closed ownership finding. */
export type GeneratorOwnership = "owned" | "external";

/** 🏭️ Exact generated output root plus its Git inventory channel. */
export interface GeneratorOutputRoot {
  readonly path: string;
  readonly inclusion: "tracked" | "ignored";
  readonly producer?: { readonly ownerPath: string; readonly target: string };
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

/** 🌊️ Explicit current JCO coordinates, separate from digest-locked historical projection evidence. */
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

/** 🔑️ A path prefix that must be filtered lexically before filesystem access. */
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

const RGI_EMOJI_SEQUENCE = new RegExp("^(?:\\p{RGI_Emoji})$", "v");

/** 🔠️Recognizes one RGI emoji or one explicitly presented pictograph. */
export function canonicalTaxonomyEmoji(value: unknown): value is string {
  return typeof value === "string" && value === value.normalize("NFC") && (RGI_EMOJI_SEQUENCE.test(value) || /^\p{Extended_Pictographic}\uFE0F$/u.test(value));
}

/** 🦀️ One valid way of writing the entry file's `#[path]` strings — see `Taxonomy.rustEntryPathRules`. */
export interface RustEntryPathConvention {
  readonly id: string;
  readonly outerReset: string | null;
  readonly groupingReset: string;
  readonly leafPrefix: string;
}

/** 📋️ Cumulative-`#[path]` base rules for the relocated (Shape V2) rust entry file. */
export interface RustEntryPathRules {
  readonly _comment?: string;
  readonly entryDirFromOwner: string;
  readonly resolution: "cumulative";
  readonly groupingResetPath: string;
  readonly leafPathPrefix: string;
  readonly conventions: readonly RustEntryPathConvention[];
}
