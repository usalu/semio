//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js: shared taxonomy vocabulary + repo-wide package discovery contract.
//#endregion 🧲️Header

//#region 🔌️Adapters
import { ephemeralMap, ephemeralBox } from "@semio-tech/framework";
import { createHash } from "node:crypto";
import { existsSync, readFileSync, readdirSync, realpathSync, statSync } from "node:fs";
import { basename, dirname, extname, join, relative, resolve } from "node:path";
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

/** 📌️ One owner-and-sibling-manifest governed semantic leaf projection. */
export interface SemanticOwnedFileProjectionContract {
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
  readonly validator: "package-glue" | "command-router";
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

/** ⚙️ Schema-owned generator identity; runnable commands derive only from exact Nx targets. */
export interface GeneratorContract {
  readonly ownership: GeneratorOwnership;
  readonly ownerPath: string | null;
  readonly target: string | null;
  readonly previewTarget?: string;
  readonly checkTarget?: string;
  readonly inputPatterns: readonly string[];
  readonly outputRoots: readonly GeneratorOutputRoot[];
  readonly reason: string;
}

/** 🚫️ A path prefix that must be filtered lexically before filesystem access. */
export interface PathExclusion {
  readonly path: string;
  readonly mode: "opaque";
  readonly reason: string;
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
  readonly semanticPathProjectionReferenceConsumerContracts: Readonly<Record<string, SemanticPathProjectionReferenceConsumerContract>>;
  readonly mutationCatalogProjection: MutationCatalogProjectionContractIds;
  readonly fixedFilenameContracts: Readonly<Record<string, FixedFilenameContract>>;
  readonly fixedFilenameRejectionContracts: Readonly<Record<string, FixedFilenameRejectionContract>>;
  readonly fixedDirectoryContracts: Readonly<Record<string, FixedDirectoryContract>>;
  readonly configurableEntryContracts: Readonly<Record<string, ConfigurableEntryContract>>;
  readonly packageBoundaryRules: Readonly<Record<string, PackageBoundaryRule>>;
  readonly packageBoundaryProfiles: Readonly<Record<string, PackageBoundaryProfile>>;
  readonly packageGlueGrammar: Readonly<Record<string, PackageGlueGrammarSpec>>;
  readonly packageSourceDispositions: Readonly<Record<string, PackageSourceDisposition>>;
  readonly generatorContracts: Readonly<Record<string, GeneratorContract>>;
  readonly pathExclusions: Readonly<Record<string, PathExclusion>>;
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
  /** 🧬️ Required children of each LEAF `🧬️mutations/<mutation>/` dir: mutation struct, per-mutation diff, inverse. */
  readonly mutationChildDirs: readonly string[];
  /** 🧩️ Required children of each COMPOSITE `🧬️mutations/<mutation>/` dir: mutation struct plus its plan. A composite derives its diff and inverse by folding the plan, so it owns neither `🔺️diff` nor `↩️inverse`. */
  readonly compositeMutationChildDirs?: readonly string[];
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
  readonly storyFileKindId: string;
  readonly testFeatureFileKindId: string;
  readonly testAdapterFileKinds: Readonly<Record<string, string>>;
  readonly testContributionFileKindId: string;
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

function readTaxonomyUnchecked(): Taxonomy {
  return JSON.parse(readFileSync(join(__dirname, "../🔣️taxonomy.json"), "utf8")) as Taxonomy;
}

function taxonomyWorkspaceRoot(): string | null {
  for (let candidate = __dirname; dirname(candidate) !== candidate; candidate = dirname(candidate)) {
    if (existsSync(join(candidate, "nx.json")) && existsSync(join(candidate, "📋️project.json"))) return candidate;
  }
  return null;
}

/** 📖️ Reads, strictly validates, and caches the incompatible version-7 taxonomy contract. */
export function loadTaxonomy(): Taxonomy {
  if (cachedTaxonomy.current) return cachedTaxonomy.current;
  const taxonomy = readTaxonomyUnchecked();
  const workspaceRoot = taxonomyWorkspaceRoot();
  const problems = [...validateTaxonomy(taxonomy), ...(workspaceRoot ? validateGeneratorContractsAgainstWorkspace(workspaceRoot, taxonomy) : ["generatorContracts workspace root could not be resolved."])];
  if (problems.length > 0) throw new Error(`Invalid taxonomy schema:\n${problems.map((problem) => `- ${problem}`).join("\n")}`);
  cachedTaxonomy.current = taxonomy;
  return cachedTaxonomy.current;
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
  const leading = normalized.match(/^(\p{Extended_Pictographic}\uFE0F?(?:\u200D\p{Extended_Pictographic}\uFE0F?)*)/u)?.[1];
  if (!leading) return normalized;
  const canonicalEmoji = leading.replace(/(\p{Extended_Pictographic})(?!\uFE0F)/gu, "$1\uFE0F");
  return `${canonicalEmoji}${normalized.slice(leading.length)}`;
}

/** 📁️ Resolves one canonical semantic directory name to its unique global or owner-local registry identifier. */
export function semanticDirectoryKindId(name: string, taxonomy: Taxonomy = loadTaxonomy(), context: SemanticDirectoryMatchContext = {}): string | null {
  const normalized = canonicalSemanticDirectoryName(name, taxonomy);
  const parentKindId = context.parentKindId;
  const matches = Object.entries(taxonomy.semanticDirectoryKinds).filter(([, spec]) => {
    if (spec.parentKindIds && !parentKindId) return false;
    if (spec.parentKindIds && !spec.parentKindIds.includes(parentKindId!)) return false;
    if (!normalized.startsWith(spec.emoji)) return false;
    const slug = normalized.slice(spec.emoji.length);
    return (slug.length === 0 && spec.allowEmojiOnly) || (slug.length > 0 && new RegExp(spec.slugPattern, "u").test(slug));
  });
  const contextual = parentKindId ? matches.filter(([, spec]) => spec.parentKindIds?.includes(parentKindId)) : [];
  const resolved = contextual.length > 0 ? contextual : matches.filter(([, spec]) => !spec.parentKindIds);
  if (resolved.length === 1) return resolved[0]![0];
  if (resolved.length > 1) return null;
  const owners = [context.parentKindId, ...(context.ancestorKindIds ?? [])].filter((id): id is string => Boolean(id));
  for (const ownerKindId of owners) {
    const memberMatches = Object.entries(taxonomy.semanticDirectoryMemberKinds).filter(([, spec]) => spec.ownerKindIds.includes(ownerKindId) && spec.memberNames.includes(normalized));
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
  const entries = Object.entries(taxonomy.semanticOwnedFileProjectionContracts);
  if (entries.length !== 1) throw new Error("Exactly one semantic owned-file projection contract is required.");
  const [contractId, contract] = entries[0]!;
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
      if ((sourceMutationDirectoryName.match(/[a-z0-9][a-z0-9-]*$/u)?.[0] ?? "") !== mutationId) problems.push(`${vectorScope}.sourceMutationDirectoryName must render mutationId.`);
      if ((mutationDirectoryName.match(/[a-z0-9][a-z0-9-]*$/u)?.[0] ?? "") !== mutationId) problems.push(`${vectorScope}.mutationDirectoryName must render mutationId.`);
      const canonical = members ? canonicalSemanticDirectoryName(mutationDirectoryName, taxonomy) : mutationDirectoryName;
      if (!members?.memberNames.includes(canonical)) problems.push(`${vectorScope}.mutationDirectoryName has no exact canonical schema membership.`);
      if (!Array.isArray(vector.scenarios) || vector.scenarios.length === 0) problems.push(`${vectorScope}.scenarios must be non-empty.`);
      for (const [scenarioIndex, scenario] of (vector.scenarios ?? []).entries()) {
        const scenarioScope = `${vectorScope}.scenarios[${scenarioIndex}]`;
        if (Object.keys(scenario).sort().join("\0") !== ["directoryName", "id"].join("\0")) problems.push(`${scenarioScope} must contain exactly id and directoryName.`);
        if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(scenario.id) || scenario.directoryName !== `🧪️${scenario.id}` || scenario.directoryName !== scenario.directoryName.normalize("NFC")) problems.push(`${scenarioScope} must be one canonical NFC test-case identity.`);
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
  const pathOwners = new Map<string, SemanticProjectionAuthorityNode>();
  for (const node of options.nodes) {
    if (node.path !== node.path.normalize("NFC") || /\uFE0E/u.test(node.path)) problems.push(`Projection node ${JSON.stringify(node.path)} must be NFC and must not contain VS15.`);
    if (!(node.path === options.sourceRoot || node.path.startsWith(`${options.sourceRoot}/`))) problems.push(`Projection node ${JSON.stringify(node.path)} is outside sourceRoot.`);
    if (pathOwners.has(node.path)) problems.push(`Projection node ${JSON.stringify(node.path)} is duplicated.`);
    pathOwners.set(node.path, node);
    if (node.nodeKind === "symlink") problems.push(`Projection source contains forbidden symlink ${JSON.stringify(node.path)}.`);
  }
  if (pathOwners.get(options.sourceRoot)?.nodeKind !== "directory") problems.push("Projection sourceRoot must be present as a directory node.");
  const actualFiles = options.nodes.filter((node) => node.nodeKind === "file").map((node) => node.path).sort(projectionByteCompare);
  const expectedSourceDirectories = projectionDirectories(options.sourceRoot, actualFiles);
  const actualSourceDirectories = options.nodes.filter((node) => node.nodeKind === "directory").map((node) => node.path).sort(projectionByteCompare);
  if (expectedSourceDirectories.join("\0") !== actualSourceDirectories.join("\0")) problems.push("Projection source directories must be exactly those owned by source files.");
  const candidateMappings: SemanticPathProjectionMapping[] = [];
  const configurableEntries: { sourcePath: string; destinationPath: string; configurationReferences: SemanticDescendantConfigurableEntryFileNode["configurableEntry"]["configurationReferences"] }[] = [];
  if (contract && root.destinationRoot) {
    const catalog = taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId];
    if (catalog && "contractKind" in catalog && catalog.contractKind === "distributed-json-manifest-catalog") {
      const modelIds = new Set<string>();
      const memberIds = new Set<string>();
      const modelDirectories = actualSourceDirectories.filter((path) => dirname(path) === options.sourceRoot).map((path) => basename(path));
      for (const modelDirectoryName of modelDirectories) {
        const manifestPath = `${options.sourceRoot}/${modelDirectoryName}/${catalog.modelManifestSourceFilename}`;
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
      for (const sourcePath of actualFiles) {
        const relativePath = sourcePath.slice(options.sourceRoot.length + 1);
        const segments = relativePath.split("/");
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
        const node = pathOwners.get(sourcePath)!;
        const manifest = projectionJsonManifest(node, `Catalog manifest ${JSON.stringify(sourcePath)}`, problems);
        if (manifest && (manifest.schema !== expectedSchema || manifest[catalog.memberVersionField] !== catalog.requiredMemberVersion || typeof manifest[catalog.memberIdentityField] !== "string" || manifest[catalog.memberIdentityField] === "")) problems.push(`Catalog manifest ${JSON.stringify(sourcePath)} has an invalid manifest schema, identity, or version.`);
        if (manifest && typeof manifest[catalog.memberIdentityField] === "string") {
          const key = `${modelDirectoryName}\0${segments[1] ?? ""}\0${manifest[catalog.memberIdentityField]}`;
          if (memberIds.has(key)) problems.push(`Distributed catalog member identity ${JSON.stringify(key)} is duplicated.`);
          memberIds.add(key);
        }
        if (destinationRelativePath) candidateMappings.push({ sourcePath, destinationPath: `${root.destinationRoot}/${destinationRelativePath}` });
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
          expectedSourceNodes.add(sourceRelativePath ? `${options.sourceRoot}/${sourceRelativePath}` : options.sourceRoot);
        }
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
    const manifestNode = pathOwners.get(sourceManifestPath);
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
    if (actualValue !== oldValue) {
      problems.push(`Configuration reference ${JSON.stringify(`${sourceManifestPath}:${reference.structuredLocation}`)} must resolve exactly to ${JSON.stringify(oldValue)}.`);
      continue;
    }
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

function fixedScopeMatches(contract: FixedFilenameContract | FixedDirectoryContract, path: string, context: FixedContractMatchContext): boolean {
  if (contract.scope.kind === "exact-path") return path === contract.scope.path;
  if (contract.scope.kind === "repository-root") return !path.includes("/");
  if (contract.scope.kind === "package-root") return context.packageRoot === true && context.ecosystemId === contract.scope.ecosystemId;
  if (contract.scope.kind === "directory-kind") return context.parentDirectoryKindId === contract.scope.directoryKindId;
  if (contract.scope.kind === "fixed-directory-contract") return context.parentFixedDirectoryContractIds?.includes(contract.scope.fixedDirectoryContractId) === true;
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
  const scopeStrength = { "path-pattern": 0, "directory-kind": 1, "package-root": 2, "fixed-directory-contract": 3, "sibling-fixed-filename-contract": 3, "repository-root": 4, "exact-path": 5 }[contract.scope.kind];
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
    .filter(([, contract]) => taxonomyPathPatternMatches(normalized, contract.pathPattern) && fixedScopeMatches(contract, normalized, context)));
}

/** 📁️ Resolves the single deterministic fixed directory winner. */
export function fixedDirectoryContractIdsForPath(path: string, taxonomy: Taxonomy = loadTaxonomy(), context: FixedContractMatchContext = {}): string[] {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/$/u, "").normalize(taxonomy.unicodeNormalization.form);
  return fixedContractWinner("directory", Object.entries(taxonomy.fixedDirectoryContracts)
    .filter(([, contract]) => taxonomyPathPatternMatches(normalized, contract.pathPattern) && fixedScopeMatches(contract, normalized, context)));
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
  return [...new Set(Object.values(taxonomy.componentFileKinds).map((kindId) => canonicalStemmedFilenameForKind(kindId, "component", taxonomy)))];
}

function semanticManifestFilename(taxonomy: Taxonomy): string {
  return canonicalStemmedFilenameForKind(taxonomy.semanticManifestFileKindId, "component", taxonomy);
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
      return { kind: "fixed", dirs: [...new Set([...(taxonomy.mutationChildDirs ?? []), ...(taxonomy.compositeMutationChildDirs ?? [])])] };
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
    if (level.kind === "wildcard") {
      if (!isEmojiPrefixedSlugDir(segment, taxonomy)) return false;
    } else {
      const dirs = level.dirs;
      const allowWildcard = dirs.includes("*");
      const fixed = dirs.filter((d) => d !== "*");
      if (!(fixed.includes(segment) || (allowWildcard && isEmojiPrefixedSlugDir(segment, taxonomy)))) return false;
    }
    parents.push(segment);
  }
  return true;
}

/**
 * 🚦️ Internal-consistency audit of the vocabulary itself: the completeness/structural artifact lists must
 * stay in their superset relation, `mutationChildDirs` must be declared and covered by `taxonomyLeafParentDirs`,
 * every registry, kind mapping, exact contract, package grammar, and clean area must agree. Returns human
 * readable problems (empty = healthy) so a vocabulary edit can never silently blind or flood the rules that
 * consume it.
 */
export function validateTaxonomy(taxonomy: Taxonomy = readTaxonomyUnchecked()): string[] {
  const problems: string[] = [];
  const document = taxonomy as unknown as Record<string, unknown>;
  const removedKeys = [
    "semanticManifestFilename", "subsetsManifestFilename", "packagingFileNames", "packagingFileSuffixes", "packagingDirNames",
    "surfaceSchemaSpecFilenames", "textSpecFilenames", "binarySpecFilenames", "artifactSchemaSpecFilenames", "exampleLeafFilenames",
    "exampleTestLeafFilenames", "semioDataLeafPrefix", "semioFileExtension", "artifactSpecFilenames", "windowEmptyFacetFilename",
    "taxonomyLeafFilenames", "entryFilenames", "storyLeafFilename", "requireEmojiPrefixWithVs16", "rootDataFileNames",
    "rootDocFileNames", "areaStates", "pluginTaxonomyStates", "repoWideFiles", "testFeatureFilename", "testAdapterFilenames",
    "testContributionFilename", "testOutputMarkerFilename", "testExcludedPathPrefixes", "testOracleRegistryPath", "testSchemaPath",
    "layeringGeneratedInventories", "semanticProjectionContracts", "projectedMemberKinds", "projectionContracts", "profileRenderers",
    "descendantContracts", "mutationCatalogProjectionContractId", "mutationCatalogKindBijection",
  ];
  for (const key of removedKeys) if (key in document) problems.push(`${key} was removed by schema version 7; use kind IDs or exact contracts.`);
  if (taxonomy.schemaVersion !== 7) problems.push(`schemaVersion must be exactly 7, got ${JSON.stringify(taxonomy.schemaVersion)}.`);

  const record = (value: unknown, key: string): value is Record<string, unknown> => {
    const valid = typeof value === "object" && value !== null && !Array.isArray(value);
    if (!valid || Object.keys(value as Record<string, unknown>).length === 0) problems.push(`${key} must be a non-empty object.`);
    return valid;
  };
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
      void taxonomyPatternExpression(value);
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
      if (!/^\p{Extended_Pictographic}\uFE0F$/u.test(spec.emoji) || spec.emoji !== spec.emoji.normalize("NFC")) problems.push(`fileKinds[${JSON.stringify(id)}].emoji must be one NFC emoji plus U+FE0F.`);
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
    if (typeof spec.emoji !== "string" || !/^\p{Extended_Pictographic}\uFE0F$/u.test(spec.emoji) || spec.emoji !== spec.emoji.normalize("NFC")) problems.push(`scopedFileKinds[${JSON.stringify(id)}].emoji must be one NFC emoji plus U+FE0F.`);
    if (!Array.isArray(spec.extensionChains) || spec.extensionChains.length === 0) problems.push(`scopedFileKinds[${JSON.stringify(id)}].extensionChains must be non-empty.`);
    for (const extension of spec.extensionChains ?? []) extensionChain(extension, `scopedFileKinds[${JSON.stringify(id)}].extensionChains`);
    if (spec.role !== "evidence") problems.push(`scopedFileKinds[${JSON.stringify(id)}].role must be evidence.`);
    fullPattern(spec.sourceFilenamePattern, `scopedFileKinds[${JSON.stringify(id)}].sourceFilenamePattern`);
    if (!spec.authority || !spec.reason || !spec.verification) problems.push(`scopedFileKinds[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
    if (!(spec.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(spec.expires))) problems.push(`scopedFileKinds[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
  }

  if (record(taxonomy.semanticDirectoryKinds, "semanticDirectoryKinds")) for (const [id, spec] of Object.entries(taxonomy.semanticDirectoryKinds)) {
      if (!/^\p{Extended_Pictographic}\uFE0F(?:\u200D\p{Extended_Pictographic}\uFE0F)*$/u.test(spec.emoji) || spec.emoji !== spec.emoji.normalize("NFC")) problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence with U+FE0F.`);
    if (typeof spec.slugPattern !== "string") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].slugPattern must be a string.`);
    else pattern(spec.slugPattern, `semanticDirectoryKinds[${JSON.stringify(id)}].slugPattern`);
    if (typeof spec.allowEmojiOnly !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].allowEmojiOnly must be boolean.`);
    if (spec.inferWithoutEmoji !== undefined && typeof spec.inferWithoutEmoji !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].inferWithoutEmoji must be boolean when present.`);
    if (spec.projectionOnly !== undefined && typeof spec.projectionOnly !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].projectionOnly must be boolean when present.`);
    ids(spec.parentKindIds ?? [], { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds }, `semanticDirectoryKinds[${JSON.stringify(id)}].parentKindIds`);
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
        if (typeof name !== "string" || name !== name.normalize("NFC") || /[\\/]/u.test(name) || !/^\p{Extended_Pictographic}\uFE0F/u.test(name)) problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}] has invalid exact member ${JSON.stringify(name)}.`);
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
      exactKeys(contract, ["contractKind", "ownerArtifactMemberName", "modelManifestSchema", "modelManifestSourceFilename", "modelIdentityField", "memberIdentityField", "memberVersionField", "requiredMemberVersion", "requiredModelManifest", "categoryRules", "coverage", "unknownCategoryPolicy", "unownedModelPolicy"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
      if (!contract.ownerArtifactMemberName || !contract.modelManifestSchema || !contract.modelManifestSourceFilename || contract.modelIdentityField !== "id" || contract.memberIdentityField !== "id" || contract.memberVersionField !== "version" || !contract.requiredMemberVersion || contract.requiredModelManifest !== true) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must declare exact non-empty manifest authority fields.`);
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
      exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "requiredSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "allowedStatuses", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "emptyContentRule", "statusDispositions", "rationaleRule"], scope);
      if (contract.contractKind !== "owner-sibling-manifest-file" || contract.manifestAdapter !== "json" || contract.manifestStatusLocation !== "status" || contract.emptyContentRule !== "zero-byte" || contract.rationaleRule !== "ticket-important-markdown-projection-v1") problems.push(`${scope} must use the exact ticket important owner-file grammar.`);
      if (!Array.isArray(contract.allowedStatuses) || contract.allowedStatuses.join("\0") !== "closed\0open") problems.push(`${scope}.allowedStatuses must be exactly closed, open.`);
      if (contract.statusDispositions && typeof contract.statusDispositions === "object" && !Array.isArray(contract.statusDispositions)) exactKeys(contract.statusDispositions, ["open", "closed-empty", "closed-nonempty", "invalid"], `${scope}.statusDispositions`);
      else problems.push(`${scope}.statusDispositions must be an object.`);
      if (JSON.stringify(contract.statusDispositions) !== JSON.stringify({ open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" })) problems.push(`${scope}.statusDispositions must be the exact lifecycle mapping.`);
      if (!taxonomy.fixedDirectoryContracts[contract.ownerFixedDirectoryContractId]) problems.push(`${scope}.ownerFixedDirectoryContractId is missing.`);
      if (!taxonomy.fixedFilenameContracts[contract.requiredSiblingFixedFilenameContractId]) problems.push(`${scope}.requiredSiblingFixedFilenameContractId is missing.`);
      if (typeof contract.sourceFilename !== "string" || !taxonomy.fileKinds[contract.sourceFileKindId] || fileKindIdForSourcePath(contract.sourceFilename, taxonomy) !== contract.sourceFileKindId) problems.push(`${scope}.sourceFilename must resolve to sourceFileKindId.`);
      const destinationKind = taxonomy.semanticDirectoryKinds[contract.destinationDirectoryKindId];
      if (!destinationKind || destinationKind.projectionOnly !== true || typeof contract.destinationDirectoryName !== "string" || semanticDirectoryKindId(contract.destinationDirectoryName, taxonomy) !== contract.destinationDirectoryKindId) problems.push(`${scope}.destination directory must resolve to one projectionOnly kind.`);
      if (contract.destinationFilename !== "📝️.md" || typeof contract.destinationFilename !== "string" || fileKindIdForSourcePath(contract.destinationFilename, taxonomy) !== contract.sourceFileKindId) problems.push(`${scope}.destinationFilename must be the exact Markdown physical leaf.`);
      for (const [field, value] of [["sourceFilename", contract.sourceFilename], ["destinationDirectoryName", contract.destinationDirectoryName], ["destinationFilename", contract.destinationFilename]] as const) if (typeof value !== "string" || !value || value !== value.normalize("NFC") || /[\\/]/u.test(value)) problems.push(`${scope}.${field} must be one non-empty NFC name.`);
      const tuple = [contract.ownerFixedDirectoryContractId, contract.requiredSiblingFixedFilenameContractId, contract.sourceFilename].join("\0");
      if (ownerTuples.has(tuple)) problems.push(`${scope} overlaps another owned-file projection contract.`);
      ownerTuples.add(tuple);
    }
    if (Object.keys(taxonomy.semanticOwnedFileProjectionContracts).join("\0") !== "ticket-important-markdown-v1") problems.push("semanticOwnedFileProjectionContracts must contain only ticket-important-markdown-v1.");
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
    const required = ["cad-spatial-kernel-geometry", "draw-dependency-registry", "draw-workspace-cargo", "draw-workspace-script"];
    if ([...identities].sort().join("\0") !== required.join("\0")) problems.push("semanticPathProjectionReferenceConsumerContracts must encode the four exact current external CAD/Draw consumers.");
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

  const fixedScope = (scope: FixedContractScope, contractPathPattern: string, key: string, filenameContract: boolean): void => {
    if (!(typeof scope === "object" && scope !== null && !Array.isArray(scope)) || typeof scope.kind !== "string") {
      problems.push(`${key} must be a tagged fixed-contract scope.`);
      return;
    }
    if (scope.kind === "exact-path") {
      exactKeys(scope, ["kind", "path"], key);
      pathPattern(scope.path, `${key}.path`);
      if (scope.path !== contractPathPattern || /[*?\[]/u.test(scope.path)) problems.push(`${key}.path must equal the exact wildcard-free contract path.`);
    } else if (scope.kind === "repository-root") {
      exactKeys(scope, ["kind"], key);
      if (contractPathPattern.includes("/") || /[*?\[]/u.test(contractPathPattern)) problems.push(`${key} repository-root contract must be one exact basename.`);
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
    } else if (scope.kind === "sibling-fixed-filename-contract") {
      exactKeys(scope, ["kind", "fixedFilenameContractId"], key);
      if (!filenameContract) problems.push(`${key} cannot use sibling-fixed-filename-contract scope.`);
      if (!scope.fixedFilenameContractId || !taxonomy.fixedFilenameContracts[scope.fixedFilenameContractId]) problems.push(`${key}.fixedFilenameContractId must reference a fixed filename contract.`);
    } else if (scope.kind === "path-pattern") exactKeys(scope, ["kind"], key);
    else problems.push(`${key}.kind is invalid.`);
  };

  if (record(taxonomy.fixedFilenameContracts, "fixedFilenameContracts")) for (const [id, contract] of Object.entries(taxonomy.fixedFilenameContracts)) {
    pathPattern(contract.pathPattern, `fixedFilenameContracts[${JSON.stringify(id)}].pathPattern`);
    if (typeof contract.pathPattern === "string" && /[*?\[]/u.test(fixedContractFilename(contract))) problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].pathPattern must end in one exact literal basename.`);
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
      if (!["package-glue", "command-router"].includes(disposition.validator) || (disposition.disposition === "adapter-source") !== (disposition.validator === "package-glue")) problems.push(`packageSourceDispositions[${JSON.stringify(id)}] disposition/validator pair is invalid.`);
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
      const allowedKeys = new Set(["ownership", "ownerPath", "target", "previewTarget", "checkTarget", "inputPatterns", "outputRoots", "reason"]);
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
          const expected = `${contract.target.slice(0, contract.target.lastIndexOf(":"))}:preview-generated`;
          if (contract.previewTarget !== expected) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget must be the exact owner preview-generated target ${JSON.stringify(expected)}.`);
          const prior = targets.get(contract.previewTarget);
          if (prior) problems.push(`generatorContracts ${JSON.stringify(prior)} and ${JSON.stringify(id)} duplicate target ${JSON.stringify(contract.previewTarget)}.`);
          targets.set(contract.previewTarget, id);
        }
      } else if (contract.previewTarget !== undefined) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget is forbidden for external contracts.`);
      if (contract.checkTarget !== undefined) {
        if (!targetKnown || !nxTarget(contract.checkTarget, `generatorContracts[${JSON.stringify(id)}].checkTarget`)) problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget requires a known target.`);
        else if (typeof contract.target === "string" && contract.checkTarget.slice(0, contract.checkTarget.lastIndexOf(":")) !== contract.target.slice(0, contract.target.lastIndexOf(":"))) problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget must belong to the target project.`);
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
          if (runnable && contract.inputPatterns.some((input) => taxonomyPathPatternMatches(output.path, input))) problems.push(`${key}.path is also declared as an input.`);
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
    ["windowEmptyFacetFileKindId", taxonomy.windowEmptyFacetFileKindId], ["testOracleRegistryLocation.fileKindId", taxonomy.testOracleRegistryLocation?.fileKindId],
    ["testSchemaLocation.fileKindId", taxonomy.testSchemaLocation?.fileKindId],
  ] as const) if (!kindId || !taxonomy.fileKinds[kindId]) problems.push(`${key} references a missing file kind.`);
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

  const directoryValues = [
    taxonomy.packagesDirName, taxonomy.targetsDirName, taxonomy.elementsDirName, taxonomy.artifactsDirName, taxonomy.modesDirName,
    taxonomy.windowsDirName, taxonomy.standardsDirName, taxonomy.subsetsDirName, taxonomy.viewerDirName, taxonomy.editorDirName,
    taxonomy.exampleAssetsDirName, taxonomy.exampleTestsDirName, ...taxonomy.artifactChildDirs, ...taxonomy.newArtifactChildDirs,
    ...taxonomy.standardChildDirs, ...taxonomy.subsetChildDirs, ...taxonomy.surfaceChildDirs, ...taxonomy.modeChildDirs,
    ...taxonomy.windowChildDirs, ...taxonomy.taxonomyLeafParentDirs, ...taxonomy.pluginChildDirs, ...taxonomy.osChildDirs,
    ...taxonomy.rootDataDirNames, ...taxonomy.schemaChildDirs, ...taxonomy.representationDirs, ...taxonomy.ioDirectionDirs,
    ...taxonomy.ioSemanticCollectionDirNames, ...Object.values(taxonomy.ioDirectionChildDirs),
  ];
  for (const directory of new Set(directoryValues)) if (!semanticDirectoryKindId(directory, taxonomy)) problems.push(`Semantic directory ${JSON.stringify(directory)} is not uniquely registered.`);
  for (const dir of taxonomy.artifactComponentDirs) if (!taxonomy.artifactChildDirs.includes(dir)) problems.push(`artifactChildDirs must include ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.windowRequiredChildDirs) if (!taxonomy.windowChildDirs.includes(dir)) problems.push(`windowChildDirs must include ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.surfaceRequiredChildDirs) if (!taxonomy.surfaceChildDirs.includes(dir)) problems.push(`surfaceChildDirs must include ${JSON.stringify(dir)}.`);
  return problems;
}

/** 🏗️ Verifies known Nx identities and required tracked outputs against live workspace metadata. */
export function validateGeneratorContractsAgainstWorkspace(repoRoot: string, taxonomy: Taxonomy = readTaxonomyUnchecked()): string[] {
  const problems: string[] = [];
  const root = resolve(repoRoot);
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
              else if (preview.executor !== "nx:run-commands" || preview.options?.cwd !== contract.ownerPath || preview.options?.command !== "bun ./📜️script.ts preview-generated") problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget must route exactly to bun ./📜️script.ts preview-generated in its owner project.`);
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
      if (output.inclusion === "tracked" && !existsSync(join(root, output.path))) problems.push(`generatorContracts[${JSON.stringify(id)}] tracked output ${JSON.stringify(output.path)} is missing.`);
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
  const spec = taxonomy.ecosystems[lang]?.marker;
  if (!spec || !existsSync(manifestPath)) return undefined;
  let role: string | undefined;
  let id: string | undefined;
  if (spec.format === "toml") {
    const body = tomlTableBody(readFileSync(manifestPath, "utf8"), spec.table);
    if (!body) return undefined;
    role = body.match(new RegExp(`^${spec.roleKey}\\s*=\\s*"([^"]+)"`, "m"))?.[1];
    id = body.match(new RegExp(`^${spec.idKey}\\s*=\\s*"([^"]+)"`, "m"))?.[1];
  } else {
    let parsed: unknown;
    try {
      parsed = JSON.parse(readFileSync(manifestPath, "utf8"));
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
function fallbackPackageId(manifestPath: string, lang: PackageLang, ownerRel: string): string {
  try {
    if (lang === "🦀️rust") {
      const name = rustPackageName(readFileSync(manifestPath, "utf8"));
      if (name) return name;
    } else {
      const name = (JSON.parse(readFileSync(manifestPath, "utf8")) as { name?: string }).name;
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
}

export type PackageSourceRole = "declaration" | "registration" | "bootstrap" | "thin-delegation" | "tool-metadata" | "implementation" | "unresolved";

/** 🧪️ Conservatively classifies one source leaf against its schema-selected package grammar. */
export function classifyPackageSourceRole(content: string, grammar: PackageGlueGrammarSpec): PackageSourceRole {
  const source = content.replace(/\/\*[\s\S]*?\*\//gu, "").replace(/(^|\s)\/\/.*$/gmu, "$1").replace(/(^|\s)#(?!\[).*$/gmu, "$1").trim();
  if (!source) return "declaration";
  if (grammar.analyzer === "rust") {
    if (/\b(?:struct|enum|trait|impl|const|static|fn)\b/u.test(source)) return "implementation";
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
function scanRepo(repoRoot: string, taxonomy: Taxonomy): DiscoveryScan {
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

  const addPackageProblem = (owner: OwnerAccumulator, path: string, kind: DiscoveryProblem["kind"], detail: string): void => {
    const repoPath = rel(path);
    packagingViolations.push({ path: repoPath, ownerRel: owner.ownerRel });
    problems.push({ kind, path: repoPath, message: `"${repoPath}" ${detail}` });
  };

  const collectPackageRoles = (packageRoot: string, lang: PackageLang, owner: OwnerAccumulator, entryContractIds: readonly string[]): void => {
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
          if (DISCOVERY_SKIP_DIRS.has(entry.name)) continue;
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
    const marker = readSemioMarker(manifestAbs, lang, taxonomy);
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
      id: marker.id ?? fallbackPackageId(manifestAbs, lang, owner.ownerRel),
      area: owner.area,
      maturity: "clean",
    });
  };

  const scanPackagesDir = (packagesAbs: string, owner: OwnerAccumulator): void => {
    if (pathIsExcluded(repoRoot, packagesAbs, taxonomy)) return;
    for (const langEntry of readdirSafe(packagesAbs)) {
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
      const hasDirect = existsSync(directManifestAbs);
      const hasTargets = existsSync(targetsAbs);
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
      for (const targetEntry of readdirSafe(targetsAbs)) {
        if (!targetEntry.isDirectory()) continue;
        const targetAbs = join(targetsAbs, targetEntry.name);
        if (pathIsExcluded(repoRoot, targetAbs, taxonomy)) continue;
        const targetManifestAbs = join(targetAbs, manifestFilename);
        if (!existsSync(targetManifestAbs)) {
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
    const entries = readdirSafe(absDir);
    let stack = ownerStack;
    if (entries.some((entry) => entry.isDirectory() && entry.name === packagesDirName)) {
      const ownerRel = rel(absDir);
      const owner: OwnerAccumulator = { ownerRel, area: areaOf(ownerRel, taxonomy) ?? "", packages: [], residualImplDirs: 0, entryFilesAtOwnerRoot: ownerRootEntryFiles(entries) };
      owners.set(ownerRel, owner);
      stack = [...ownerStack, owner];
      scanPackagesDir(join(absDir, packagesDirName), owner);
    }
    for (const entry of entries) {
      if (!entry.isDirectory() || entry.name.startsWith(".") || DISCOVERY_SKIP_DIRS.has(entry.name) || entry.name === packagesDirName) continue;
      const path = join(absDir, entry.name);
      if (pathIsExcluded(repoRoot, path, taxonomy)) continue;
      if (forbiddenSegments.has(entry.name)) {
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
        if (!entry.name.startsWith(".") && !SEMANTIC_SKIP_DIRS.has(entry.name)) walk(path);
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
  const filename = semanticManifestFilename(taxonomy);
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
function semanticProductionConsumer(source: SemanticSource, taxonomy: Taxonomy): boolean {
  const entries = new Set(Object.values(taxonomy.configurableEntryContracts).map((contract) => contract.filename));
  return source.production && !entries.has(basename(source.abs)) && !semanticAssemblyOnly(source.content, extname(source.abs));
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
  const problems: SemanticProblem[] = validateTaxonomy(taxonomy).map((message) => ({ code: "taxonomy-schema", severity: "error", path: semanticRel(repoRoot, join(__dirname, "../🔣️taxonomy.json")), message }));
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
    const manifestFilename = semanticManifestFilename(taxonomy);
    const manifest = semanticReadManifest(join(collectionAbs, manifestFilename), taxonomy, problems, collectionPath);
    const actualChildren = readdirSafe(collectionAbs)
      .filter((entry) => entry.isDirectory() && !entry.name.startsWith(".") && !SEMANTIC_SKIP_DIRS.has(entry.name) && entry.name !== taxonomy.packagesDirName && entry.name !== "🤖️generated")
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
    const production = semanticProductionConsumer(source, taxonomy);
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
    const languageMirrors = semanticUnique(draft.sourceFiles.map((source) => Object.entries(taxonomy.componentFileKinds).find(([, kindId]) => canonicalStemmedFilenameForKind(kindId, "component", taxonomy) === basename(source.abs))?.[0]).filter((value): value is string => Boolean(value)));
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
