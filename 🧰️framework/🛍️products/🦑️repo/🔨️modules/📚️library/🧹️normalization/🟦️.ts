//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — deterministic taxonomy inventory, planning, verification and transaction engine.
//#endregion 🧲️Header

//#region 🔌️Adapters
import { execFileSync, spawnSync } from "node:child_process";
import { createHash, randomUUID } from "node:crypto";
import {
  chmodSync,
  closeSync,
  copyFileSync,
  existsSync,
  fsyncSync,
  linkSync,
  lstatSync,
  mkdirSync,
  mkdtempSync,
  openSync,
  readFileSync,
  readdirSync,
  readlinkSync,
  renameSync,
  rmdirSync,
  rmSync,
  symlinkSync,
  writeFileSync,
  writeSync,
} from "node:fs";
import type { Stats } from "node:fs";
import { generatorPreviewResourceLimits, generatorPreviewScriptArguments, registryCatalogInputPaths, registryCatalogInputView, registryCatalogPathMayAffect, semanticPackageAdapterPreview, semanticPackageGeneratedLeafPreview, semanticPackageIgnoredGeneratedOutputPaths, semanticPackageJoinedPathReferenceAuthority, semanticPackageAuthoredFragmentReferences, semanticPackageProjectionAuthority, semanticPackageProjectionCatalog, type GeneratorProjectionActivation, type RegistryCatalogInputDiscovery, type RegistryCatalogInputView, type SemanticPackageGeneration, type SemanticPackageProjectionCase } from "../🔍️discovery/🟦️.ts";
import { tmpdir } from "node:os";
import { parseCanonicalWgpuPackageCatalog, parseSemanticPackageBrowserProfile } from "../🔍️discovery/🟦️.ts";
import { parseFixedDirectoryContractSetScope, parseNamedFixedDirectoryContractSetScope } from "../🔍️discovery/🟦️.ts";
import { parseGeneratorInputProjection, parseSemanticOwnedCurrentSourceRevisions, parseSemanticOwnedDocumentCorrections, semanticExactOwnedDocumentCorrectionAuthority, semanticOwnedInputFileSnapshot, type GeneratorInputProjection, type SemanticOwnedInputFileSnapshot } from "../🔍️discovery/🟦️.ts";
import { inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, inspectRustNonRepoJoinBaseSpans, rustTokens as rustSyntaxTokens, rustTokenPairs, validateFrozenCoordinateEvidenceContracts, type RustModuleGraph, type FrozenCoordinateEvidenceContract } from "../🔍️discovery/🟦️.ts";
import { validateFrozenMarkdownCoordinateEvidenceContracts, type FrozenMarkdownCoordinateEvidenceContract } from "../🔍️discovery/🟦️.ts";
import { jsonDocumentDuplicateKeys, mutationCatalogSourceOwner, mutationCatalogSourceOwnersProblems, mutationOwnerIdentity, mutationOwnerRelativePath, mutationPayloadSchemaProblems, pathEmojiStatuteFindings, reservedDocumentationBasename } from "../🔍️discovery/🟦️.ts";
import { basename, dirname, isAbsolute, join, parse, posix, relative, resolve, sep } from "node:path";
import { artifactPathProjectionCatalogRoots, createTaxonomyPathMatcher, renderArtifactPathProjectionRoot, semanticArtifactEmptyFacetProjectionAuthority, semanticExactOwnedFileCatalog, semanticExactOwnedFileProjectionAuthority, semanticOwnedFileHistoryProjectionAuthority, semanticOwnedFileProjectionAuthority, semanticOwnedPrimaryFileProjectionAuthority, semanticPathProjectionAuthority, semanticPathProjectionReferenceConsumers, validateTaxonomy, type TaxonomyPathMatcher, type SemanticExactOwnedFileCase, type SemanticExactOwnedFileCatalog, type SemanticFacetPrimaryFileProjectionContract, type SemanticPathProjectionReferenceConsumerForm, type SemanticProjectionAuthorityNode, type Taxonomy as DiscoveryTaxonomy } from "../🔍️discovery/🟦️.ts";
//#endregion 🔌️Adapters

//#region 📜️Contracts
export type TaxonomySeverity = "warning" | "error";
export type TaxonomyNodeKind = "directory" | "file" | "symlink";
export type TaxonomyPackageRole = "configuration" | "declaration" | "registration" | "bootstrap" | "thin-delegation" | "implementation" | "unresolved" | "not-package";
export type TaxonomyReferenceAdapter = "rust" | "typescript" | "go" | "python" | "dotnet" | "native" | "json" | "jsonc" | "toml" | "yaml" | "xml" | "markdown" | "gherkin";

export interface TaxonomyViolation {
  readonly code: string;
  readonly severity: TaxonomySeverity;
  readonly path: string;
  readonly message: string;
}

export interface TaxonomyInventoryEntry {
  readonly sourcePath: string;
  readonly normalizedPath: string;
  readonly nodeKind: TaxonomyNodeKind;
  readonly ownerId: string;
  readonly areaId: string;
  readonly fileKind: string | null;
  readonly semanticStem: string | null;
  readonly fixedContractId?: string;
  readonly packageRole?: TaxonomyPackageRole;
  readonly contentHash: string;
  readonly mode: number;
  readonly size: number;
  readonly symlinkTarget?: string;
  readonly referencesIn: readonly string[];
  readonly referencesOut: readonly string[];
  readonly violations: readonly TaxonomyViolation[];
}

export interface ReferenceEdit {
  readonly path: string;
  readonly adapter: TaxonomyReferenceAdapter;
  readonly structuredLocation: string;
  readonly oldValue: string;
  readonly newValue: string;
  readonly preimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }>;
}

export interface TaxonomyMoveSourceAuthority {
  readonly kind: "exact-owner-current-source-revision-v1";
  readonly revisionId: "testing-readme-protocol-v2-reviewed";
  readonly revisionDigest: string;
  readonly inputs: readonly Readonly<{ role: "schema" | "catalog" | "expectation"; path: string; preimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }> }>[];
}

export interface TaxonomyMove {
  readonly operationId: string;
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly sourcePreimage: TaxonomyLeafPreimage;
  readonly rationaleRule: string;
  readonly ownerId: string;
  readonly collisionGroup?: string;
  readonly referenceEdits: readonly ReferenceEdit[];
  readonly sourceAuthority?: TaxonomyMoveSourceAuthority;
}

export type TaxonomyGeneratorNodeRecord =
  | Readonly<{ path: string; nodeKind: "directory"; contentHash: string; mode: number }>
  | Readonly<{ path: string; nodeKind: "file"; contentHash: string; mode: number; size: number }>
  | Readonly<{ path: string; nodeKind: "symlink"; contentHash: string; mode: number; size: number; target: string }>;

export interface TaxonomyGeneratorPreviewNode {
  readonly bytesBase64: string;
  readonly mode: number;
  readonly nodeKind: "directory" | "file";
  readonly path: string;
}

export interface TaxonomyGeneratorPreviewManifest {
  readonly contractId: string;
  readonly nodes: readonly TaxonomyGeneratorPreviewNode[];
  readonly schemaVersion: 1;
  readonly staleRemovals: readonly string[];
}

export interface TaxonomyRegeneration {
  readonly id: string;
  readonly contractId: string;
  readonly cwd: string;
  readonly command: readonly ["bun", "nx", "run", string];
  readonly verifyCommand?: readonly ["bun", "nx", "run", string];
  readonly outputRoots: readonly string[];
  readonly inputs: readonly TaxonomyGeneratorNodeRecord[];
  readonly preOutputs: readonly TaxonomyGeneratorNodeRecord[];
  readonly outputs: readonly TaxonomyGeneratorNodeRecord[];
  readonly preview: TaxonomyGeneratorPreviewManifest;
  readonly previewManifestDigest: string;
  readonly staleRemovals: readonly string[];
}

export interface OpaqueTreeDigest {
  readonly algorithm: "sha256-merkle-v1";
  readonly relativeRoot: string;
  readonly digest: string;
  readonly files: number;
  readonly directories: number;
  readonly symlinks: number;
  readonly others: number;
}

export type TaxonomyDispositionLeafKind = "file" | "symlink";

export type TaxonomyLeafPreimage =
  | Readonly<{ nodeKind: "file"; contentHash: string; mode: number; size: number }>
  | Readonly<{ nodeKind: "symlink"; contentHash: string; mode: number; size: number; target: string }>;

export type TaxonomyPathPreimage =
  | Readonly<{ state: "absent" }>
  | Readonly<{ state: "directory" }>
  | Readonly<{ state: "file"; contentHash: string; mode: number; size: number }>
  | Readonly<{ state: "symlink"; contentHash: string; mode: number; size: number; target: string }>;

export interface TaxonomyNoFollowTreeDigest {
  readonly algorithm: "sha256-no-follow-merkle-v1";
  readonly digest: string;
  readonly files: number;
  readonly directories: number;
  readonly symlinks: number;
  readonly others: number;
}

export interface TaxonomySymlinkTargetEdit {
  readonly operationId: string;
  readonly sourcePath: string;
  readonly finalPath: string;
  readonly oldTarget: string;
  readonly newTarget: string;
  readonly oldTargetHash: string;
  readonly newTargetHash: string;
  readonly logicalTargetSourcePath: string;
  readonly logicalTargetFinalPath: string;
  readonly logicalTargetPreimage: TaxonomyPathPreimage;
  readonly windowsLinkType: "file" | "dir";
  readonly sourceTargetDigest: string;
  readonly rationaleRule: "repository-local-symlink-target-v2";
  readonly ownerId: string;
}

export interface TaxonomyEvidenceMember {
  readonly sourcePath: string;
  readonly finalPath: string;
  readonly disposition: "remove" | "retain" | "relocate";
  readonly preimage: TaxonomyLeafPreimage;
}

export type TaxonomyRemovalAuthority =
  | Readonly<{ kind: "nested-cargo-generated-source"; catalogPath: string; catalogContentHash: string; packageId: "wgpu-renderer"; generatorContractId: string; destinationPath: string; sourcePreimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }>; authorityDigest: string }>
  | Readonly<{ kind: "exact-owner-generated-source"; catalogPath: string; catalogContentHash: string; generatorContractId: string; destinationPath: string; outputPreimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }>; authorityDigest: string }>
  | Readonly<{ kind: "byte-and-mode-identical"; evidenceSetDigest: string; retainedFinalPath: string; members: readonly TaxonomyEvidenceMember[] }>
  | Readonly<{ kind: "exact-path-mutation"; catalogPath: string; catalogContentHash: string; caseId: string; sourcePath: string; sourcePreimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }>; disposition: "remove"; authorityDigest: string }>
  | Readonly<{ kind: "owner-manifest-status"; contractId: "ticket-important-markdown-v1"; ownerPath: string; manifestPath: string; manifestPreimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }>; status: "closed"; contentState: "zero-byte"; authorityDigest: string }>
  | Readonly<{ kind: "serialized-path-sentinel"; fixturePath: string; fixtureContentHash: string; caseId: string; serializedInputPath: string; expectedViolationCode: "windows-reserved-name" | "trailing-dot-or-space"; authorityDigest: string }>;

export interface TaxonomyEvidenceRemoval {
  readonly operationId: string;
  readonly sourcePath: string;
  readonly preimage: TaxonomyLeafPreimage;
  readonly authority: TaxonomyRemovalAuthority;
  readonly embeddedTicketRootId?: string;
  readonly rationaleRule: "redundant-ticket-evidence-v1" | "serialized-platform-sentinel-v1" | "ticket-important-closed-empty-v1" | "ticket-important-exact-empty-residue-v1" | "exact-owner-generated-source-retirement-v1" | "nested-cargo-generated-source-retirement-v1";
  readonly ownerId: string;
}

export interface TaxonomyEmbeddedTicketRootDisposition {
  readonly operationId: string;
  readonly sourceMetadataRoot: string;
  readonly sourceTicketRoot: string;
  readonly canonicalTicketRoot: string;
  readonly ticketId: string;
  readonly sourceTreeDigest: TaxonomyNoFollowTreeDigest;
  readonly residualTreeDigest: TaxonomyNoFollowTreeDigest;
  readonly incomingReferenceDigest: string;
  readonly relocationOperationIds: readonly string[];
  readonly removalOperationIds: readonly string[];
  readonly rationaleRule: "embedded-ticket-root-relocation-v1";
}

export interface TaxonomyEmbeddedTicketRootRelocation {
  readonly operationId: string;
  readonly embeddedTicketRootId: string;
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly relativeEvidencePath: string;
  readonly preimage: TaxonomyLeafPreimage;
  readonly fixedContractId?: string;
  readonly ownerId: string;
  readonly rationaleRule: "embedded-ticket-root-relocation-v1";
}

export type TaxonomyAffectedStateRow =
  | Readonly<{ path: string; state: "absent" }>
  | Readonly<{ path: string; state: "directory" }>
  | Readonly<{ path: string; state: "file"; contentHash: string; mode: number; size: number }>
  | Readonly<{ path: string; state: "symlink"; targetHash: string; targetSize: number }>
  | Readonly<{ path: string; state: "directory-tree"; tree: TaxonomyNoFollowTreeDigest }>
  | Readonly<{ path: string; state: "generator"; contentHash: string }>;

export type TaxonomyDestinationAncestorPreimage = Readonly<{ path: string; state: "absent" | "directory" }>;

export interface TaxonomyPlan {
  readonly schemaVersion: 2;
  readonly taxonomySchemaVersion: 7;
  readonly baselineCommit: string;
  readonly scope?: string;
  readonly sourceTreeDigest: string;
  readonly excludedTreeDigests: readonly OpaqueTreeDigest[];
  readonly moves: readonly TaxonomyMove[];
  readonly embeddedTicketRoots: readonly TaxonomyEmbeddedTicketRootDisposition[];
  readonly embeddedTicketRootRelocations: readonly TaxonomyEmbeddedTicketRootRelocation[];
  readonly symlinkTargetEdits: readonly TaxonomySymlinkTargetEdit[];
  readonly evidenceRemovals: readonly TaxonomyEvidenceRemoval[];
  readonly destinationAncestorPreimages: readonly TaxonomyDestinationAncestorPreimage[];
  readonly edits: readonly ReferenceEdit[];
  readonly regenerations: readonly TaxonomyRegeneration[];
  readonly unresolved: readonly TaxonomyViolation[];
  readonly expectedAffectedPreStateDigest: string;
  readonly expectedPostStateDigest: string;
  readonly planDigest: string;
}

export interface TaxonomyProgress {
  readonly operation: "inventory" | "plan" | "apply" | "verify" | "digest";
  readonly phase: string;
  readonly current: number;
  readonly total: number;
  readonly path?: string;
}

export interface TaxonomyInventoryOptions {
  readonly repoRoot: string;
  readonly scope?: string;
  readonly ticketDir?: string;
  readonly cancelFile?: string;
  readonly workers?: number;
  readonly progress?: (progress: TaxonomyProgress) => void;
  readonly taxonomyPath?: string;
  readonly baselineCommit?: string;
  readonly excludedTreeDigests?: readonly OpaqueTreeDigest[];
}

export type TaxonomySourceOrigin = "tracked" | "nonignored-untracked" | "ignored-generator" | "explicit-ticket";
export type TaxonomySourceObservedKind = TaxonomyNodeKind | "absent" | "unobserved" | "other";

export interface TaxonomySourceIndexEntry {
  readonly stage: number;
  readonly mode: string;
  readonly objectId: string;
}

export interface TaxonomySourceGeneratorOutput {
  readonly contractId: string;
  readonly rootPath: string;
  readonly inclusion: "tracked" | "ignored";
}

export interface TaxonomySourceCandidateObservation {
  readonly sourcePath: string;
  readonly observedKind: TaxonomySourceObservedKind;
  readonly worktreeMode: string | null;
  readonly explicitDirectory: boolean;
  readonly origins: readonly TaxonomySourceOrigin[];
  readonly indexEntries: readonly TaxonomySourceIndexEntry[];
  readonly unsafeAncestor: boolean;
}

export interface TaxonomySourceAdmissionInput {
  readonly scope: string | null;
  readonly cancelledDuring?: string | null;
  readonly opaquePrefixes: readonly string[];
  readonly generatorOutputRoots: readonly TaxonomySourceGeneratorOutput[];
  readonly candidates: readonly TaxonomySourceCandidateObservation[];
}

export interface TaxonomySourceObservation extends Omit<TaxonomySourceCandidateObservation, "unsafeAncestor"> {
  readonly generatorOutputs: readonly TaxonomySourceGeneratorOutput[];
  readonly repositoryBoundary: "gitlink" | null;
}

export interface TaxonomySourceAdmissionDiagnostic {
  readonly code: string;
  readonly path: string;
  readonly message: string;
}

export interface TaxonomySourceAdmission {
  readonly schemaVersion: 1;
  readonly scope: string | null;
  readonly status: "complete" | "rejected";
  readonly observations: readonly TaxonomySourceObservation[];
  readonly diagnostics: readonly TaxonomySourceAdmissionDiagnostic[];
}

export interface TaxonomySourceInventory extends TaxonomySourceAdmission {
  readonly repoRoot: string;
  readonly taxonomyPath: string;
  readonly taxonomyContentHash: string;
  readonly membershipDigest: string;
}

export interface TaxonomyPlanOptions {
  readonly baselineCommit: string;
  readonly excludedTreeDigests: readonly OpaqueTreeDigest[];
  readonly cancelFile?: string;
  readonly progress?: (progress: TaxonomyProgress) => void;
}

export type TaxonomyFailureStage = "after-staging" | "after-embedded-root-staging" | "after-moves" | "after-relocations" | "after-symlink-retargeting" | "after-edits" | "after-regenerations" | "before-verify";

export interface TaxonomyApplyOptions {
  readonly repoRoot: string;
  readonly ticketDir: string;
  readonly explicitTicketDir?: string;
  readonly expectedBaselineCommit: string;
  readonly planArtifactPath?: string;
  readonly expectedPlanDigest?: string;
  readonly cancelFile?: string;
  readonly resumeJournal?: string;
  readonly injectFailureAt?: TaxonomyFailureStage;
  readonly workers?: number;
  readonly progress?: (progress: TaxonomyProgress) => void;
  readonly taxonomyPath?: string;
}

export interface TaxonomyInventory {
  readonly schemaVersion: 1;
  readonly taxonomySchemaVersion: 7;
  readonly repoRoot: string;
  readonly scope?: string;
  readonly taxonomyPath: string;
  readonly pathExclusions: readonly string[];
  readonly activePathExclusions: readonly string[];
  readonly entries: readonly TaxonomyInventoryEntry[];
  readonly violations: readonly TaxonomyViolation[];
  readonly sourceTreeDigest: string;
  readonly inventoryDigest: string;
}

export interface TaxonomyVerification {
  readonly inventory: TaxonomyInventory;
  readonly plan: TaxonomyPlan;
  readonly violations: readonly TaxonomyViolation[];
  readonly clean: boolean;
}

export type TaxonomyJournalState = "prepared" | "staging" | "disposing" | "installing" | "retargeting" | "editing" | "regenerating" | "verifying" | "committed" | "rolling-back" | "rolled-back";

export type TaxonomyBackupRecord =
  | Readonly<{ kind: "absent" }>
  | Readonly<{ kind: "file"; backupPath: string; contentHash: string; mode: number; size: number }>
  | Readonly<{ kind: "symlink"; target: string; targetHash: string; mode: number; size: number }>;

export interface TaxonomyJournalRecord {
  readonly schemaVersion: 2;
  readonly revision: number;
  readonly planDigest: string;
  readonly attemptOrdinal: string;
  readonly state: TaxonomyJournalState;
  readonly stagingRoot: string;
  readonly backupRoot: string;
  readonly preparedMoveIds: readonly string[];
  readonly stagedMoveIds: readonly string[];
  readonly installedMoveIds: readonly string[];
  readonly preparedEmbeddedRelocationIds: readonly string[];
  readonly stagedEmbeddedRelocationIds: readonly string[];
  readonly installedEmbeddedRelocationIds: readonly string[];
  readonly preparedEvidenceRemovalIds: readonly string[];
  readonly stagedEvidenceRemovalIds: readonly string[];
  readonly preparedEmbeddedRootIds: readonly string[];
  readonly stagedEmbeddedRootIds: readonly string[];
  readonly preparedSymlinkTargetEditIds: readonly string[];
  readonly stagedSymlinkTargetEditIds: readonly string[];
  readonly installedSymlinkTargetEditIds: readonly string[];
  readonly appliedEditPaths: readonly string[];
  readonly startedRegenerationIds: readonly string[];
  readonly completedRegenerationIds: readonly string[];
  readonly sourceParentPrunePaths: readonly string[];
  readonly backups: Readonly<Record<string, TaxonomyBackupRecord>>;
  readonly error?: string;
}

export interface TaxonomyApplyResult {
  readonly planDigest: string;
  readonly journalPath: string;
  readonly state: "committed" | "rolled-back";
  readonly appliedMoves: number;
  readonly appliedEmbeddedTicketRootRelocations: number;
  readonly appliedSymlinkTargetEdits: number;
  readonly appliedEvidenceRemovals: number;
  readonly appliedEdits: number;
  readonly appliedRegenerations: number;
}
//#endregion 📜️Contracts

//#region 🔣️Schema
type JsonRecord = Record<string, unknown>;

interface FileKindSpec {
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

type FixedContractScope = Readonly<
  | { kind: "exact-path"; path: string }
  | { kind: "repository-root" }
  | { kind: "package-root"; ecosystemId: string }
  | { kind: "directory-kind"; directoryKindId: string }
  | { kind: "fixed-directory-contract"; fixedDirectoryContractId: string }
  | { kind: "fixed-directory-contract-set"; fixedDirectoryContractIds: readonly string[] }
  | { kind: "sibling-fixed-filename-contract"; fixedFilenameContractId: string }
  | { kind: "path-pattern" }
>;

interface FixedFilenameContract {
  readonly pathPattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly configurability: "unconfigurable";
  readonly scope: FixedContractScope;
  readonly verification: string;
  readonly expires: string | null;
}

interface FixedDirectoryContract {
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

interface ConfigurableEntryContract {
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
  readonly parentDirectoryKindId: string;
  readonly emoji: string;
  readonly extensionChains: readonly string[];
  readonly role: "evidence";
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

type SemanticProjectionCaptureField = "standardVersion" | "subsetId" | "mutationId" | "scenarioId" | "commandDirectoryName";
type SemanticProjectionSourceSegment = Readonly<{ kindId: string; literal: string } | { kindId: string; capture: SemanticProjectionCaptureField } | { memberKindId: string; literal: string } | { projectedMemberKindId: string; capture: SemanticProjectionCaptureField }>;
type SemanticProjectionDestinationSegment = Readonly<{ kindId: string; literal: string } | { kindId: string; render: "profile" } | { kindId: string; copy: SemanticProjectionCaptureField } | { projectedMemberKindId: string; copy: SemanticProjectionCaptureField }>;

interface SemanticPathProjectionProfileRenderer {
  readonly direction: "forward-only";
  readonly captureFields: readonly ["standardVersion", "subsetId"];
  readonly directoryKindId: string;
  readonly template: "🪆️{standardVersion}-{subsetId}";
  readonly tupleCollisionFields: readonly ["artifactId", "standardVersion", "subsetId"];
}

interface SemanticDescendantKindNode {
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

interface SemanticDescendantConfigurableEntryFileNode {
  readonly sourcePathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly destinationPathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly nodeType: "file";
  readonly configurableEntry: Readonly<{ contractId: string; sourceFilename: string; configurationReferences: readonly Readonly<{ fixedFilenameContractId: string; adapter: "json" | "toml"; structuredLocation: string }>[] }>;
}

type SemanticDescendantNode = SemanticDescendantKindNode | SemanticDescendantFixedFileNode | SemanticDescendantConfigurableEntryFileNode;

interface SemanticDescendantAlternative {
  readonly id: string;
  readonly mode: "exactly-one";
  readonly nodes: readonly SemanticDescendantNode[];
}

interface SemanticExactDescendantContract {
  readonly rootDirectoryKindId: string;
  readonly requiredNodes: readonly SemanticDescendantNode[];
  readonly exclusiveAlternatives: readonly SemanticDescendantAlternative[];
  readonly realizedNodeCount: number;
  readonly pathBudgetReserve: Readonly<{ derivation: "longest-canonical-descendant-suffix"; bytes: number }>;
}

type SemanticKindDescendantContract = Omit<SemanticExactDescendantContract, "requiredNodes" | "exclusiveAlternatives"> & Readonly<{
  requiredNodes: readonly SemanticDescendantKindNode[];
  exclusiveAlternatives: readonly Readonly<{ id: string; mode: "exactly-one"; nodes: readonly SemanticDescendantKindNode[] }>[];
}>;

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

interface SemanticDistributedJsonManifestCatalogContract {
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

interface SemanticExactOwnerVectorsCatalogContract {
  readonly contractKind: "exact-owner-vectors";
  readonly required: true;
  readonly allowEmpty: false;
  readonly identityFields: readonly ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"];
  readonly coverage: "every-physical-command-bundle-exactly-once";
  readonly vectors: readonly Readonly<{ artifactId: string; standardVersion: string; subsetId: string; commandDirectoryName: string }>[];
}

type SemanticPathProjectionCatalogContract = SemanticMutationPathProjectionCatalogContract | SemanticDistributedJsonManifestCatalogContract | SemanticExactOwnerVectorsCatalogContract;

interface SemanticPathProjectionContract {
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

interface SemanticPathProjectionReferenceConsumerContract {
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
  readonly projectionContractId: string;
  readonly projectedMemberKindId: string;
  readonly descendantContractId: string;
  readonly catalogContractId: string;
}

type GeneratorOwnership = "owned" | "external";

interface GeneratorOutputRootSpec {
  readonly path: string;
  readonly inclusion: "tracked" | "ignored";
}

interface GeneratorContractSpec {
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
  readonly projectionActivation?: GeneratorProjectionActivation;
  readonly outputRoots: readonly GeneratorOutputRootSpec[];
  readonly reason: string;
}

interface PackageBoundaryRule {
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
  readonly authority: string;
  readonly verification: string;
}

interface EcosystemSpec {
  readonly packageIdentity: "manifest" | "boundary-only";
  readonly manifestContractId: string | null;
}

interface PackageGlueGrammar {
  readonly analyzer: "rust" | "typescript" | "javascript" | "go" | "python" | "dotnet" | "c-cpp";
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

interface LoadedTaxonomy {
  readonly path: string;
  readonly pathMatcher: TaxonomyPathMatcher;
  readonly input?: SemanticOwnedInputFileSnapshot;
  readonly schema: TaxonomyV7;
  readonly discoverySchema: DiscoveryTaxonomy;
  readonly exclusions: readonly { readonly id: string; readonly path: string }[];
  readonly fileKinds: readonly (FileKindSpec & { readonly id: string })[];
  readonly directoryKinds: readonly (SemanticDirectoryKindSpec & { readonly id: string; readonly slugRegex: RegExp })[];
}

const TAXONOMY_RELATIVE_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const TRANSACTION_SENTINEL_CASES_FIXTURE_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🔣️transaction-sentinel-cases.json";
const TICKET_IMPORTANT_EXACT_MUTATIONS_FIXTURE_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🔣️ticket-important-exact-mutations.json";
const TICKET_IMPORTANT_EXACT_GOVERNED_SOURCES = [
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️12/ENFORCE-WINDOW-APP-PANEL-AND-PLUGIN-CONTRACTS-AT-COMPILE-TIME/🧪️window-policy-fixture/🎛️apps/🧪️fixture/🎭️modes/🧪️mode/🪟️windows/🧪️component-window/👥️presence/📌️important.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/📌️important.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/📌️important.md",
] as const;
const LEXICAL_OPAQUE_ROOTS = ["compose", "temp/compose"] as const;
const GENERIC_SEMANTIC_STEMS = new Set(["asset", "assets", "component", "components", "descriptor", "glue", "test", "tests", "implementation", "impl", "index", "cases", "vectors"]);
const WINDOWS_RESERVED = /^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/i;
const SEGMENTER = new Intl.Segmenter("und", { granularity: "grapheme" });

function record(value: unknown, name: string): JsonRecord {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error(`Taxonomy v7 field ${name} must be an object`);
  return value as JsonRecord;
}

function stringArray(value: unknown, name: string): readonly string[] {
  if (!Array.isArray(value) || value.some((item) => typeof item !== "string")) throw new Error(`Taxonomy v7 field ${name} must be a string array`);
  return value;
}

function requiredString(value: unknown, name: string): string {
  if (typeof value !== "string" || value.length === 0) throw new Error(`Taxonomy v7 field ${name} must be a non-empty string`);
  return value;
}

function requireExactKeys(value: JsonRecord, keys: readonly string[], name: string): void {
  const actual = Object.keys(value).sort();
  const expected = [...keys].sort();
  if (canonicalJson(actual) !== canonicalJson(expected)) throw new Error(`Taxonomy v7 field ${name} must contain exactly ${expected.join(", ")}`);
}

function fixedExpiry(value: unknown, name: string): string | null {
  if (value === null) return null;
  const expires = requiredString(value, name);
  if (!/^\d{4}-\d{2}-\d{2}$/u.test(expires)) throw new Error(`Taxonomy v7 ${name} must be null or YYYY-MM-DD`);
  return expires;
}

function parseTaxonomy(raw: unknown, path: string): LoadedTaxonomy {
  const root = record(raw, "root");
  if (root.schemaVersion !== 7) throw new Error(`Taxonomy schemaVersion must be 7 at ${path}`);
  const discoveryProblems = validateTaxonomy(root as unknown as DiscoveryTaxonomy);
  if (discoveryProblems.length > 0) throw new Error(`Taxonomy v7 discovery contract validation failed at ${path}: ${discoveryProblems.join(" | ")}`);
  const pathMatcher = createTaxonomyPathMatcher();
  function validatedContractPattern(value: unknown, name: string, exactBasename: boolean): string {
    const pattern = requiredString(value, name);
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
  const fileKindRows = record(root.fileKinds, "fileKinds");
  const directoryKindRows = record(root.semanticDirectoryKinds, "semanticDirectoryKinds");
  const fixedRows = record(root.fixedFilenameContracts, "fixedFilenameContracts");
  const fixedRejectionRows = record(root.fixedFilenameRejectionContracts, "fixedFilenameRejectionContracts");
  const fixedDirectoryRows = record(root.fixedDirectoryContracts, "fixedDirectoryContracts");
  const configurableRows = record(root.configurableEntryContracts, "configurableEntryContracts");
  const fileResolutionRows = record(root.fileKindResolutionRules, "fileKindResolutionRules");
  const scopedFileRows = record(root.scopedFileKinds, "scopedFileKinds");
  const directoryMemberRows = record(root.semanticDirectoryMemberKinds, "semanticDirectoryMemberKinds");
  const projectedMemberRows = record(root.semanticProjectedMemberKinds, "semanticProjectedMemberKinds");
  const projectionRendererRows = record(root.semanticPathProjectionProfileRenderers, "semanticPathProjectionProfileRenderers");
  const descendantContractRows = record(root.semanticDescendantContracts, "semanticDescendantContracts");
  const projectionCatalogRows = record(root.semanticPathProjectionCatalogContracts, "semanticPathProjectionCatalogContracts");
  const projectionRows = record(root.semanticPathProjectionContracts, "semanticPathProjectionContracts");
  const ownedFileProjectionRows = record(root.semanticOwnedFileProjectionContracts, "semanticOwnedFileProjectionContracts");
  const projectionConsumerRows = record(root.semanticPathProjectionReferenceConsumerContracts, "semanticPathProjectionReferenceConsumerContracts");
  const mutationCatalogProjectionRow = record(root.mutationCatalogProjection, "mutationCatalogProjection");
  const generatorRows = record(root.generatorContracts, "generatorContracts");
  const ecosystemRows = record(root.ecosystems, "ecosystems");
  const boundaryRows = record(root.packageBoundaryRules, "packageBoundaryRules");
  const boundaryProfileRows = record(root.packageBoundaryProfiles, "packageBoundaryProfiles");
  const grammarRows = record(root.packageGlueGrammar, "packageGlueGrammar");
  const sourceDispositionRows = record(root.packageSourceDispositions, "packageSourceDispositions");
  const exclusionRows = record(root.pathExclusions, "pathExclusions");
  const unicode = record(root.unicodeNormalization, "unicodeNormalization");
  const selector = record(root.variationSelectorPolicy, "variationSelectorPolicy");
  const collision = record(root.collisionPolicy, "collisionPolicy");
  const enforcement = record(root.areaEnforcement, "areaEnforcement");
  if (unicode.form !== "NFC" || unicode.caseFold !== "lower" || unicode.locale !== "und") throw new Error("Taxonomy v7 unicodeNormalization must select NFC/lower/und");
  if (selector.selector !== "\uFE0F" || selector.requiredAfterEmoji !== true || selector.comparison !== "ignore-selector") throw new Error("Taxonomy v7 variationSelectorPolicy is not canonical");
  const requiredComparisons = ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"];
  if (canonicalJson(collision.comparisons) !== canonicalJson(requiredComparisons) || !Number.isSafeInteger(collision.maxPathBytes) || (collision.maxPathBytes as number) < 1 || collision.rejectWindowsReservedNames !== true || collision.rejectTrailingDotsAndSpaces !== true) throw new Error("Taxonomy v7 collisionPolicy is incomplete");
  if (enforcement.requiredState !== "clean" || enforcement.undeclaredAreas !== "enforce") throw new Error("Taxonomy v7 areaEnforcement must enforce clean undeclared areas");

  const fileKinds: Record<string, FileKindSpec> = {};
  for (const [id, value] of Object.entries(fileKindRows)) {
    const spec = record(value, `fileKinds.${id}`);
    const emoji = requiredString(spec.emoji, `fileKinds.${id}.emoji`).normalize("NFC");
    const extensionChains = stringArray(spec.extensionChains, `fileKinds.${id}.extensionChains`);
    if (extensionChains.length === 0 || extensionChains.some((chain) => !chain.startsWith("."))) throw new Error(`Taxonomy v7 fileKinds.${id}.extensionChains must contain dotted chains`);
    fileKinds[id] = { emoji, extensionChains: [...new Set(extensionChains)].sort((a, b) => b.length - a.length || a.localeCompare(b)), role: requiredString(spec.role, `fileKinds.${id}.role`) };
  }
  if (Object.keys(fileKinds).length === 0) throw new Error("Taxonomy v7 fileKinds must not be empty");

  const semanticDirectoryKinds: Record<string, SemanticDirectoryKindSpec> = {};
  for (const [id, value] of Object.entries(directoryKindRows)) {
    const spec = record(value, `semanticDirectoryKinds.${id}`);
    const emoji = requiredString(spec.emoji, `semanticDirectoryKinds.${id}.emoji`).normalize("NFC");
    const slugPattern = requiredString(spec.slugPattern, `semanticDirectoryKinds.${id}.slugPattern`);
    new RegExp(slugPattern, "u");
    if (typeof spec.allowEmojiOnly !== "boolean") throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.allowEmojiOnly must be boolean`);
    if (spec.inferWithoutEmoji !== undefined && typeof spec.inferWithoutEmoji !== "boolean") throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.inferWithoutEmoji must be boolean when present`);
    if (spec.projectionOnly !== undefined && typeof spec.projectionOnly !== "boolean") throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.projectionOnly must be boolean when present`);
    semanticDirectoryKinds[id] = { emoji, slugPattern, allowEmojiOnly: spec.allowEmojiOnly, ...(spec.inferWithoutEmoji === undefined ? {} : { inferWithoutEmoji: spec.inferWithoutEmoji }), ...(spec.projectionOnly === undefined ? {} : { projectionOnly: spec.projectionOnly }), ...(spec.parentKindIds === undefined ? {} : { parentKindIds: stringArray(spec.parentKindIds, `semanticDirectoryKinds.${id}.parentKindIds`) }) };
  }
  if (Object.keys(semanticDirectoryKinds).length === 0) throw new Error("Taxonomy v7 semanticDirectoryKinds must not be empty");

  const fixedFilenameContracts: Record<string, FixedFilenameContract> = {};
  for (const [id, value] of Object.entries(fixedRows)) {
    const spec = record(value, `fixedFilenameContracts.${id}`);
    if (spec.configurability !== "unconfigurable") throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.configurability must be unconfigurable`);
    const inputScopeRow = record(spec.scope, `fixedFilenameContracts.${id}.scope`);
    const scopeRow = inputScopeRow.kind === "named-fixed-directory-contract-set" ? parseNamedFixedDirectoryContractSetScope(inputScopeRow, root.fixedDirectoryContracts as DiscoveryTaxonomy["fixedDirectoryContracts"], (root.fixedDirectoryContractSets ?? {}) as NonNullable<DiscoveryTaxonomy["fixedDirectoryContractSets"]>) : inputScopeRow;
    const scopeKind = requiredString(scopeRow.kind, `fixedFilenameContracts.${id}.scope.kind`) as FixedContractScope["kind"];
    if (!["exact-path", "repository-root", "package-root", "directory-kind", "fixed-directory-contract", "fixed-directory-contract-set", "sibling-fixed-filename-contract", "path-pattern"].includes(scopeKind)) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.kind is invalid`);
    const scope: FixedContractScope = scopeKind === "exact-path"
      ? (requireExactKeys(scopeRow, ["kind", "path"], `fixedFilenameContracts.${id}.scope`), { kind: "exact-path", path: normalizeRelative(requiredString(scopeRow.path, `fixedFilenameContracts.${id}.scope.path`)) })
      : scopeKind === "package-root"
        ? (requireExactKeys(scopeRow, ["kind", "ecosystemId"], `fixedFilenameContracts.${id}.scope`), { kind: "package-root", ecosystemId: requiredString(scopeRow.ecosystemId, `fixedFilenameContracts.${id}.scope.ecosystemId`) })
        : scopeKind === "directory-kind"
          ? (requireExactKeys(scopeRow, ["kind", "directoryKindId"], `fixedFilenameContracts.${id}.scope`), { kind: "directory-kind", directoryKindId: requiredString(scopeRow.directoryKindId, `fixedFilenameContracts.${id}.scope.directoryKindId`) })
          : scopeKind === "fixed-directory-contract"
            ? (requireExactKeys(scopeRow, ["kind", "fixedDirectoryContractId"], `fixedFilenameContracts.${id}.scope`), { kind: "fixed-directory-contract", fixedDirectoryContractId: requiredString(scopeRow.fixedDirectoryContractId, `fixedFilenameContracts.${id}.scope.fixedDirectoryContractId`) })
            : scopeKind === "fixed-directory-contract-set"
              ? parseFixedDirectoryContractSetScope(scopeRow, root.fixedDirectoryContracts as DiscoveryTaxonomy["fixedDirectoryContracts"])
            : scopeKind === "sibling-fixed-filename-contract"
              ? (requireExactKeys(scopeRow, ["kind", "fixedFilenameContractId"], `fixedFilenameContracts.${id}.scope`), { kind: "sibling-fixed-filename-contract", fixedFilenameContractId: requiredString(scopeRow.fixedFilenameContractId, `fixedFilenameContracts.${id}.scope.fixedFilenameContractId`) })
          : (requireExactKeys(scopeRow, ["kind"], `fixedFilenameContracts.${id}.scope`), { kind: scopeKind });
    if (scope.kind === "directory-kind" && !semanticDirectoryKinds[scope.directoryKindId]) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.directoryKindId is invalid`);
    fixedFilenameContracts[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `fixedFilenameContracts.${id}.pathPattern`, true),
      authority: requiredString(spec.authority, `fixedFilenameContracts.${id}.authority`),
      reason: requiredString(spec.reason, `fixedFilenameContracts.${id}.reason`),
      configurability: "unconfigurable",
      scope,
      verification: requiredString(spec.verification, `fixedFilenameContracts.${id}.verification`),
      expires: fixedExpiry(spec.expires, `fixedFilenameContracts.${id}.expires`),
    };
  }

  const fixedDirectoryContracts: Record<string, FixedDirectoryContract> = {};
  for (const [id, value] of Object.entries(fixedDirectoryRows)) {
    const spec = record(value, `fixedDirectoryContracts.${id}`);
    if (spec.configurability !== "unconfigurable") throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.configurability must be unconfigurable`);
    const scopeRow = record(spec.scope, `fixedDirectoryContracts.${id}.scope`);
    const scopeKind = requiredString(scopeRow.kind, `fixedDirectoryContracts.${id}.scope.kind`);
    if (!["exact-path", "repository-root", "directory-kind", "path-pattern"].includes(scopeKind)) throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.scope.kind is invalid`);
    const scope: FixedDirectoryContract["scope"] = scopeKind === "exact-path"
      ? (requireExactKeys(scopeRow, ["kind", "path"], `fixedDirectoryContracts.${id}.scope`), { kind: "exact-path", path: normalizeRelative(requiredString(scopeRow.path, `fixedDirectoryContracts.${id}.scope.path`)) })
      : scopeKind === "directory-kind"
        ? (requireExactKeys(scopeRow, ["kind", "directoryKindId"], `fixedDirectoryContracts.${id}.scope`), { kind: "directory-kind", directoryKindId: requiredString(scopeRow.directoryKindId, `fixedDirectoryContracts.${id}.scope.directoryKindId`) })
        : (requireExactKeys(scopeRow, ["kind"], `fixedDirectoryContracts.${id}.scope`), { kind: scopeKind as "repository-root" | "path-pattern" });
    if (scope.kind === "directory-kind" && !semanticDirectoryKinds[scope.directoryKindId]) throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.scope.directoryKindId is invalid`);
    fixedDirectoryContracts[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `fixedDirectoryContracts.${id}.pathPattern`, false),
      authority: requiredString(spec.authority, `fixedDirectoryContracts.${id}.authority`),
      reason: requiredString(spec.reason, `fixedDirectoryContracts.${id}.reason`),
      configurability: "unconfigurable",
      scope,
      verification: requiredString(spec.verification, `fixedDirectoryContracts.${id}.verification`),
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
    const spec = record(value, `fixedFilenameRejectionContracts.${id}`);
    requireExactKeys(spec, ["sourcePathIdentities", "disposition", "reason"], `fixedFilenameRejectionContracts.${id}`);
    if (spec.disposition !== "normalize" && spec.disposition !== "relocate") throw new Error(`Taxonomy v7 fixedFilenameRejectionContracts.${id}.disposition is invalid`);
    const sourcePathIdentities = stringArray(spec.sourcePathIdentities, `fixedFilenameRejectionContracts.${id}.sourcePathIdentities`).map(normalizeRelative);
    if (sourcePathIdentities.length === 0 || sourcePathIdentities.some((path) => rejectedFixedPaths.has(path))) throw new Error(`Taxonomy v7 fixedFilenameRejectionContracts.${id}.sourcePathIdentities are empty or duplicated`);
    for (const path of sourcePathIdentities) rejectedFixedPaths.add(path);
    fixedFilenameRejectionContracts[id] = { sourcePathIdentities, disposition: spec.disposition, reason: requiredString(spec.reason, `fixedFilenameRejectionContracts.${id}.reason`) };
  }
  if (Object.keys(fixedFilenameRejectionContracts).length === 0) throw new Error("Taxonomy v7 fixedFilenameRejectionContracts must not be empty");

  const configurableEntryContracts: Record<string, ConfigurableEntryContract> = {};
  for (const [id, value] of Object.entries(configurableRows)) {
    const spec = record(value, `configurableEntryContracts.${id}`);
    const fileKindId = requiredString(spec.fileKindId, `configurableEntryContracts.${id}.fileKindId`);
    if (!fileKinds[fileKindId]) throw new Error(`Taxonomy v7 configurableEntryContracts.${id} references unknown file kind ${fileKindId}`);
    configurableEntryContracts[id] = {
      filename: requiredString(spec.filename, `configurableEntryContracts.${id}.filename`),
      fileKindId,
      ecosystemId: requiredString(spec.ecosystemId, `configurableEntryContracts.${id}.ecosystemId`),
      role: requiredString(spec.role, `configurableEntryContracts.${id}.role`),
      configurationSources: stringArray(spec.configurationSources, `configurableEntryContracts.${id}.configurationSources`),
    };
  }

  const fileKindResolutionRules: Record<string, FileKindResolutionRuleSpec> = {};
  for (const [id, value] of Object.entries(fileResolutionRows)) {
    const spec = record(value, `fileKindResolutionRules.${id}`);
    const extensionChain = requiredString(spec.extensionChain, `fileKindResolutionRules.${id}.extensionChain`);
    const fileKindId = requiredString(spec.fileKindId, `fileKindResolutionRules.${id}.fileKindId`);
    if (!fileKinds[fileKindId]?.extensionChains.includes(extensionChain)) throw new Error(`Taxonomy v7 fileKindResolutionRules.${id} does not reference an owned extension chain`);
    if (!Number.isSafeInteger(spec.priority)) throw new Error(`Taxonomy v7 fileKindResolutionRules.${id}.priority must be an integer`);
    const filenamePattern = typeof spec.filenamePattern === "string" ? spec.filenamePattern : undefined;
    const pathPattern = typeof spec.pathPattern === "string" ? validatedContractPattern(spec.pathPattern, `fileKindResolutionRules.${id}.pathPattern`, false) : undefined;
    if (filenamePattern) new RegExp(filenamePattern, "u");
    const parentKindIds = spec.parentKindIds === undefined ? undefined : stringArray(spec.parentKindIds, `fileKindResolutionRules.${id}.parentKindIds`);
    const ancestorKindIds = spec.ancestorKindIds === undefined ? undefined : stringArray(spec.ancestorKindIds, `fileKindResolutionRules.${id}.ancestorKindIds`);
    for (const kindId of [...(parentKindIds ?? []), ...(ancestorKindIds ?? [])]) if (!semanticDirectoryKinds[kindId]) throw new Error(`Taxonomy v7 fileKindResolutionRules.${id} references unknown directory kind ${kindId}`);
    fileKindResolutionRules[id] = { extensionChain, fileKindId, priority: spec.priority as number, filenamePattern, pathPattern, parentKindIds, ancestorKindIds };
  }
  if (Object.keys(fileKindResolutionRules).length === 0) throw new Error("Taxonomy v7 fileKindResolutionRules must not be empty");

  const scopedFileKinds: Record<string, ScopedFileKindSpec> = {};
  for (const [id, value] of Object.entries(scopedFileRows)) {
    const spec = record(value, `scopedFileKinds.${id}`);
    const extensionChains = stringArray(spec.extensionChains, `scopedFileKinds.${id}.extensionChains`);
    if (extensionChains.length === 0 || extensionChains.some((chain) => !chain.startsWith("."))) throw new Error(`Taxonomy v7 scopedFileKinds.${id}.extensionChains must contain dotted chains`);
    const sourceFilenamePattern = requiredString(spec.sourceFilenamePattern, `scopedFileKinds.${id}.sourceFilenamePattern`);
    new RegExp(sourceFilenamePattern, "u");
    if (spec.role !== "evidence") throw new Error(`Taxonomy v7 scopedFileKinds.${id}.role must be evidence`);
    const parentDirectoryKindId = requiredString(spec.parentDirectoryKindId, `scopedFileKinds.${id}.parentDirectoryKindId`);
    if (!semanticDirectoryKinds[parentDirectoryKindId]) throw new Error(`Taxonomy v7 scopedFileKinds.${id} references unknown parent directory kind ${parentDirectoryKindId}`);
    scopedFileKinds[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `scopedFileKinds.${id}.pathPattern`, false),
      parentDirectoryKindId,
      emoji: requiredString(spec.emoji, `scopedFileKinds.${id}.emoji`).normalize("NFC"),
      extensionChains: [...new Set(extensionChains)].sort((left, right) => right.length - left.length || left.localeCompare(right)),
      role: "evidence",
      sourceFilenamePattern,
      authority: requiredString(spec.authority, `scopedFileKinds.${id}.authority`),
      reason: requiredString(spec.reason, `scopedFileKinds.${id}.reason`),
      verification: requiredString(spec.verification, `scopedFileKinds.${id}.verification`),
      expires: fixedExpiry(spec.expires, `scopedFileKinds.${id}.expires`),
    };
  }

  const semanticDirectoryMemberKinds: Record<string, SemanticDirectoryMemberKindSpec> = {};
  for (const [id, value] of Object.entries(directoryMemberRows)) {
    const spec = record(value, `semanticDirectoryMemberKinds.${id}`);
    if (spec.source !== "registry") throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id}.source must be registry`);
    const ownerKindIds = stringArray(spec.ownerKindIds, `semanticDirectoryMemberKinds.${id}.ownerKindIds`);
    const memberNames = stringArray(spec.memberNames, `semanticDirectoryMemberKinds.${id}.memberNames`);
    if (ownerKindIds.length === 0 || memberNames.length === 0) throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} must declare owners and members`);
    if (memberNames.some((name) => name !== name.normalize("NFC") || !splitLeadingEmoji(name).emoji)) throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} member names must be NFC emoji-leading evidence`);
    semanticDirectoryMemberKinds[id] = { ownerKindIds: [...new Set(ownerKindIds)].sort(), memberNames: [...new Set(memberNames)].sort(), source: "registry" };
  }
  const directoryContextIds = new Set([...Object.keys(semanticDirectoryKinds), ...Object.keys(semanticDirectoryMemberKinds)]);
  for (const [id, spec] of Object.entries(semanticDirectoryMemberKinds)) for (const ownerId of spec.ownerKindIds) if (!directoryContextIds.has(ownerId)) throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} references unknown owner kind ${ownerId}`);

  const semanticProjectedMemberKinds: Record<string, SemanticProjectedMemberKindSpec> = {};
  for (const [id, value] of Object.entries(projectedMemberRows)) {
    const spec = record(value, `semanticProjectedMemberKinds.${id}`);
    if (spec.identityField !== "mutationDirectoryName" && spec.identityField !== "commandDirectoryName") throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id}.identityField is invalid`);
    const ownerKindIds = stringArray(spec.ownerKindIds, `semanticProjectedMemberKinds.${id}.ownerKindIds`);
    if (ownerKindIds.length === 0) throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id}.ownerKindIds must not be empty`);
    semanticProjectedMemberKinds[id] = { ownerKindIds: [...new Set(ownerKindIds)].sort(), projectionContractId: requiredString(spec.projectionContractId, `semanticProjectedMemberKinds.${id}.projectionContractId`), sourceMemberKindId: requiredString(spec.sourceMemberKindId, `semanticProjectedMemberKinds.${id}.sourceMemberKindId`), identityField: spec.identityField };
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
    const spec = record(value, `semanticPathProjectionProfileRenderers.${id}`);
    if (spec.direction !== "forward-only" || canonicalJson(spec.captureFields) !== canonicalJson(["standardVersion", "subsetId"]) || spec.template !== "🪆️{standardVersion}-{subsetId}" || canonicalJson(spec.tupleCollisionFields) !== canonicalJson(["artifactId", "standardVersion", "subsetId"])) throw new Error(`Taxonomy v7 semanticPathProjectionProfileRenderers.${id} is not the forward-only standard/subset contract`);
    const directoryKindId = requiredString(spec.directoryKindId, `semanticPathProjectionProfileRenderers.${id}.directoryKindId`);
    if (!semanticDirectoryKinds[directoryKindId]) throw new Error(`Taxonomy v7 semanticPathProjectionProfileRenderers.${id} references unknown directory kind ${directoryKindId}`);
    semanticPathProjectionProfileRenderers[id] = { direction: "forward-only", captureFields: ["standardVersion", "subsetId"], directoryKindId, template: "🪆️{standardVersion}-{subsetId}", tupleCollisionFields: ["artifactId", "standardVersion", "subsetId"] };
  }
  if (Object.keys(semanticPathProjectionProfileRenderers).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionProfileRenderers must not be empty");

  const parseDescendantNode = (value: unknown, name: string): SemanticDescendantNode => {
    const spec = record(value, name);
    if (spec.nodeType !== "directory" && spec.nodeType !== "file") throw new Error(`Taxonomy v7 ${name}.nodeType is invalid`);
    const parseSegments = (value: unknown, key: string): readonly Readonly<{ kindId: string; literal: string }>[] => {
      if (!Array.isArray(value)) throw new Error(`Taxonomy v7 ${key} must be an array`);
      return value.map((value, index) => {
        const segment = record(value, `${key}[${index}]`);
        const kindId = requiredString(segment.kindId, `${key}[${index}].kindId`);
        const literal = requiredString(segment.literal, `${key}[${index}].literal`).normalize("NFC");
        const kind = semanticDirectoryKinds[kindId];
        const leading = splitLeadingEmoji(literal);
        if (!kind || emojiFold(leading.emoji) !== emojiFold(kind.emoji) || !new RegExp(kind.slugPattern, "u").test(leading.rest)) throw new Error(`Taxonomy v7 ${key} has an invalid semantic path segment ${literal}`);
        return { kindId, literal };
      });
    };
    if (spec.nodeType === "file" && spec.configurableEntry !== undefined) {
      const sourcePathSegments = parseSegments(spec.sourcePathSegments, `${name}.sourcePathSegments`);
      const destinationPathSegments = parseSegments(spec.destinationPathSegments, `${name}.destinationPathSegments`);
      const configurable = record(spec.configurableEntry, `${name}.configurableEntry`);
      const contractId = requiredString(configurable.contractId, `${name}.configurableEntry.contractId`);
      const contract = configurableEntryContracts[contractId];
      const sourceFilename = requiredString(configurable.sourceFilename, `${name}.configurableEntry.sourceFilename`).normalize("NFC");
      if (!contract || /[\\/]/u.test(sourceFilename) || sourceFilename !== basename(sourceFilename)) throw new Error(`Taxonomy v7 ${name}.configurableEntry is not a registered source basename`);
      if (!Array.isArray(configurable.configurationReferences) || configurable.configurationReferences.length === 0) throw new Error(`Taxonomy v7 ${name}.configurableEntry.configurationReferences must not be empty`);
      const configurationReferences = configurable.configurationReferences.map((value, index) => {
        const reference = record(value, `${name}.configurableEntry.configurationReferences[${index}]`);
        const fixedFilenameContractId = requiredString(reference.fixedFilenameContractId, `${name}.configurableEntry.configurationReferences[${index}].fixedFilenameContractId`);
        if (!fixedFilenameContracts[fixedFilenameContractId] || reference.adapter !== "json" && reference.adapter !== "toml") throw new Error(`Taxonomy v7 ${name}.configurableEntry.configurationReferences[${index}] is invalid`);
        return { fixedFilenameContractId, adapter: reference.adapter, structuredLocation: requiredString(reference.structuredLocation, `${name}.configurableEntry.configurationReferences[${index}].structuredLocation`) } as const;
      });
      return { sourcePathSegments, destinationPathSegments, nodeType: "file", configurableEntry: { contractId, sourceFilename, configurationReferences } };
    }
    const pathSegments = parseSegments(spec.pathSegments, `${name}.pathSegments`);
    if (spec.nodeType === "directory") {
      const kindId = requiredString(spec.kindId, `${name}.kindId`);
      if (!allDirectoryContextIds.has(kindId) || spec.sourceFilename !== undefined || spec.fixedFilenameContractId !== undefined || spec.packageGlue !== undefined) throw new Error(`Taxonomy v7 ${name} references an invalid directory kind ${kindId}`);
      return { pathSegments, nodeType: "directory", kindId };
    }
    const authorities = [spec.kindId !== undefined, spec.fixedFilenameContractId !== undefined].filter(Boolean).length;
    if (authorities !== 1) throw new Error(`Taxonomy v7 ${name} must declare exactly one file authority`);
    if (spec.kindId !== undefined) {
      const kindId = requiredString(spec.kindId, `${name}.kindId`);
      if (!fileKinds[kindId]) throw new Error(`Taxonomy v7 ${name} references unknown file kind ${kindId}`);
      const sourceFilename = spec.sourceFilename === undefined ? undefined : requiredString(spec.sourceFilename, `${name}.sourceFilename`).normalize("NFC");
      if (sourceFilename !== undefined && (kindId !== "rust-source" || sourceFilename !== "🦀️.rs")) throw new Error(`Taxonomy v7 ${name}.sourceFilename is not the frozen Draw Rust source leaf`);
      return { pathSegments, nodeType: "file", kindId, ...(sourceFilename ? { sourceFilename } : {}) };
    }
    if (spec.fixedFilenameContractId !== undefined) {
      const fixedFilenameContractId = requiredString(spec.fixedFilenameContractId, `${name}.fixedFilenameContractId`);
      if (!fixedFilenameContracts[fixedFilenameContractId]) throw new Error(`Taxonomy v7 ${name} references unknown fixed filename contract ${fixedFilenameContractId}`);
      return { pathSegments, nodeType: "file", fixedFilenameContractId };
    }
    throw new Error(`Taxonomy v7 ${name} has no file authority`);
  };
  const semanticDescendantContracts: Record<string, SemanticDescendantContract> = {};
  for (const [id, value] of Object.entries(descendantContractRows)) {
    const spec = record(value, `semanticDescendantContracts.${id}`);
    const rootDirectoryKindId = requiredString(spec.rootDirectoryKindId, `semanticDescendantContracts.${id}.rootDirectoryKindId`);
    if (!allDirectoryContextIds.has(rootDirectoryKindId)) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} references unknown root directory kind ${rootDirectoryKindId}`);
    if (spec.contractKind === "catalog") {
      const catalogContractId = requiredString(spec.catalogContractId, `semanticDescendantContracts.${id}.catalogContractId`);
      const leafFileKindId = requiredString(spec.leafFileKindId, `semanticDescendantContracts.${id}.leafFileKindId`);
      const reserve = record(spec.pathBudgetReserve, `semanticDescendantContracts.${id}.pathBudgetReserve`);
      if (!fileKinds[leafFileKindId] || spec.rendering !== "semantic-member-directory-and-physical-kind-leaf" || reserve.derivation !== "longest-rendered-catalog-descendant-suffix" || !Number.isSafeInteger(reserve.bytes) || (reserve.bytes as number) <= 0) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} is not a valid catalog descendant contract`);
      semanticDescendantContracts[id] = { contractKind: "catalog", rootDirectoryKindId, catalogContractId, leafFileKindId, rendering: "semantic-member-directory-and-physical-kind-leaf", pathBudgetReserve: { derivation: "longest-rendered-catalog-descendant-suffix", bytes: reserve.bytes as number } };
      continue;
    }
    if (!Array.isArray(spec.requiredNodes) || !Array.isArray(spec.exclusiveAlternatives)) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} node lists must be arrays`);
    const requiredNodes = spec.requiredNodes.map((node, index) => parseDescendantNode(node, `semanticDescendantContracts.${id}.requiredNodes[${index}]`));
    const exclusiveAlternatives = spec.exclusiveAlternatives.map((value, index) => {
      const alternative = record(value, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}]`);
      if (alternative.mode !== "exactly-one" || !Array.isArray(alternative.nodes) || alternative.nodes.length < 2) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} alternative must contain exactly-one candidates`);
      return { id: requiredString(alternative.id, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}].id`), mode: "exactly-one" as const, nodes: alternative.nodes.map((node, nodeIndex) => parseDescendantNode(node, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}].nodes[${nodeIndex}]`)) };
    });
    const realizedRequiredCount = requiredNodes.length + requiredNodes.filter((node) => "configurableEntry" in node).length;
    if (!Number.isSafeInteger(spec.realizedNodeCount) || spec.realizedNodeCount !== realizedRequiredCount + exclusiveAlternatives.length) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id}.realizedNodeCount is invalid`);
    const reserve = record(spec.pathBudgetReserve, `semanticDescendantContracts.${id}.pathBudgetReserve`);
    const suffix = (node: SemanticDescendantNode): string => {
      const segments = ("configurableEntry" in node ? node.destinationPathSegments : node.pathSegments).map((segment) => segment.literal);
      if (node.nodeType === "file") {
        if ("configurableEntry" in node) segments.push(configurableEntryContracts[node.configurableEntry.contractId].filename);
        else if ("kindId" in node) {
          const kind = fileKinds[node.kindId];
          if (kind.extensionChains.length !== 1) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} file kind ${node.kindId} must have one physical extension chain`);
          segments.push(`${kind.emoji}${kind.extensionChains[0]}`);
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
    const spec = record(value, `semanticPathProjectionCatalogContracts.${id}`);
    if (spec.contractKind === undefined) {
      if (canonicalJson(value) !== canonicalJson(expectedCatalogContract)) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not the independent required vector registry contract`);
      semanticPathProjectionCatalogContracts[id] = expectedCatalogContract;
      continue;
    }
    if (spec.contractKind === "distributed-json-manifest-catalog") {
      if (spec.modelIdentityField !== "id" || spec.memberIdentityField !== "id" || spec.memberVersionField !== "version" || spec.requiredModelManifest !== true || spec.coverage !== "every-source-file-and-destination-node-exactly-once" || spec.unknownCategoryPolicy !== "problem" || spec.unownedModelPolicy !== "problem" || !Array.isArray(spec.categoryRules) || spec.categoryRules.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not a strict distributed manifest catalog`);
      if (!Array.isArray(spec.profileVectors) || spec.profileVectors.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.profileVectors must be non-empty`);
      const profileVectors = spec.profileVectors.map((value, index) => {
        const vector = record(value, `semanticPathProjectionCatalogContracts.${id}.profileVectors[${index}]`);
        const profile = { artifactId: requiredString(vector.artifactId, "profile artifactId"), standardVersion: requiredString(vector.standardVersion, "profile standardVersion"), subsetId: requiredString(vector.subsetId, "profile subsetId") };
        if (canonicalJson(Object.keys(vector).sort()) !== canonicalJson(["artifactId", "standardVersion", "subsetId"]) || profile.artifactId !== spec.ownerArtifactMemberName || Object.values(profile).some((field) => field !== field.normalize("NFC") || /[\\/]/u.test(field))) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.profileVectors[${index}] is not an exact NFC owner tuple`);
        return profile;
      });
      if (new Set(profileVectors.map((vector) => canonicalJson(vector))).size !== profileVectors.length) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats a profile vector`);
      const categoryRules = spec.categoryRules.map((value, index) => {
        const rule = record(value, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}]`);
        const sourceDirectoryName = requiredString(rule.sourceDirectoryName, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].sourceDirectoryName`).normalize("NFC");
        const directoryKindId = requiredString(rule.directoryKindId, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].directoryKindId`);
        const manifestSchema = requiredString(rule.manifestSchema, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].manifestSchema`);
        if (!semanticDirectoryKinds[directoryKindId]) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}] references an unknown directory kind`);
        if (rule.sourceShape === "direct-semantic-json") return { sourceDirectoryName, directoryKindId, sourceShape: "direct-semantic-json" as const, manifestSchema, memberDirectoryEmoji: requiredString(rule.memberDirectoryEmoji, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].memberDirectoryEmoji`).normalize("NFC") };
        if (rule.sourceShape === "nested-fixed-json") return { sourceDirectoryName, directoryKindId, sourceShape: "nested-fixed-json" as const, manifestSchema, fixedSourceFilename: requiredString(rule.fixedSourceFilename, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].fixedSourceFilename`).normalize("NFC") };
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].sourceShape is invalid`);
      });
      if (new Set(categoryRules.map((rule) => rule.sourceDirectoryName)).size !== categoryRules.length) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats a catalog category`);
      semanticPathProjectionCatalogContracts[id] = { contractKind: "distributed-json-manifest-catalog", ownerArtifactMemberName: requiredString(spec.ownerArtifactMemberName, `semanticPathProjectionCatalogContracts.${id}.ownerArtifactMemberName`).normalize("NFC"), profileVectors, modelManifestSchema: requiredString(spec.modelManifestSchema, `semanticPathProjectionCatalogContracts.${id}.modelManifestSchema`), modelManifestSourceFilename: requiredString(spec.modelManifestSourceFilename, `semanticPathProjectionCatalogContracts.${id}.modelManifestSourceFilename`).normalize("NFC"), modelIdentityField: "id", memberIdentityField: "id", memberVersionField: "version", requiredMemberVersion: requiredString(spec.requiredMemberVersion, `semanticPathProjectionCatalogContracts.${id}.requiredMemberVersion`), requiredModelManifest: true, categoryRules, coverage: "every-source-file-and-destination-node-exactly-once", unknownCategoryPolicy: "problem", unownedModelPolicy: "problem" };
      continue;
    }
    if (spec.contractKind === "exact-owner-vectors") {
      if (spec.required !== true || spec.allowEmpty !== false || canonicalJson(spec.identityFields) !== canonicalJson(["artifactId", "standardVersion", "subsetId", "commandDirectoryName"]) || spec.coverage !== "every-physical-command-bundle-exactly-once" || !Array.isArray(spec.vectors) || spec.vectors.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not a strict exact-owner vector registry`);
      const vectors = spec.vectors.map((value, index) => {
        const vector = record(value, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}]`);
        return { artifactId: requiredString(vector.artifactId, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].artifactId`).normalize("NFC"), standardVersion: requiredString(vector.standardVersion, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].standardVersion`), subsetId: requiredString(vector.subsetId, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].subsetId`), commandDirectoryName: requiredString(vector.commandDirectoryName, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].commandDirectoryName`).normalize("NFC") };
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
    const spec = record(value, name);
    const kindId = typeof spec.kindId === "string" ? spec.kindId : undefined;
    const memberKindId = typeof spec.memberKindId === "string" ? spec.memberKindId : undefined;
    const projectedMemberKindId = typeof spec.projectedMemberKindId === "string" ? spec.projectedMemberKindId : undefined;
    if ((kindId ? 1 : 0) + (memberKindId ? 1 : 0) + (projectedMemberKindId ? 1 : 0) !== 1) throw new Error(`Taxonomy v7 ${name} must identify exactly one kind`);
    if (kindId && !allDirectoryContextIds.has(kindId)) throw new Error(`Taxonomy v7 ${name} references unknown directory kind ${kindId}`);
    if (memberKindId && !semanticDirectoryMemberKinds[memberKindId]) throw new Error(`Taxonomy v7 ${name} references unknown semantic member kind ${memberKindId}`);
    if (projectedMemberKindId && !semanticProjectedMemberKinds[projectedMemberKindId]) throw new Error(`Taxonomy v7 ${name} references unknown projected member kind ${projectedMemberKindId}`);
    if (destination) {
      if (memberKindId) throw new Error(`Taxonomy v7 ${name} cannot render a source member kind`);
      if (spec.literal !== undefined && kindId) return { kindId, literal: requiredString(spec.literal, `${name}.literal`) };
      if (spec.render === "profile" && kindId) return { kindId, render: "profile" };
      if (typeof spec.copy === "string" && captureFields.has(spec.copy as SemanticProjectionCaptureField)) return projectedMemberKindId ? { projectedMemberKindId, copy: spec.copy as SemanticProjectionCaptureField } : { kindId: kindId!, copy: spec.copy as SemanticProjectionCaptureField };
    } else {
      if (spec.literal !== undefined && kindId) return { kindId, literal: requiredString(spec.literal, `${name}.literal`) };
      if (spec.literal !== undefined && memberKindId) {
        const literal = requiredString(spec.literal, `${name}.literal`).normalize("NFC");
        if (!semanticDirectoryMemberKinds[memberKindId].memberNames.includes(literal)) throw new Error(`Taxonomy v7 ${name}.literal is not registered by ${memberKindId}`);
        return { memberKindId, literal };
      }
      if (typeof spec.capture === "string" && captureFields.has(spec.capture as SemanticProjectionCaptureField)) return projectedMemberKindId ? { projectedMemberKindId, capture: spec.capture as SemanticProjectionCaptureField } : { kindId: kindId!, capture: spec.capture as SemanticProjectionCaptureField };
    }
    throw new Error(`Taxonomy v7 ${name} has an invalid ${destination ? "destination" : "source"} operation`);
  };
  const semanticPathProjectionContracts: Record<string, SemanticPathProjectionContract> = {};
  for (const [id, value] of Object.entries(projectionRows)) {
    const spec = record(value, `semanticPathProjectionContracts.${id}`);
    if (!Array.isArray(spec.sourceSegments) || !Array.isArray(spec.destinationSegments) || !["artifact-mutation-test-projection-v1", "artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(String(spec.rationaleRule))) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} is invalid`);
    const sourceOwnerKindId = requiredString(spec.sourceOwnerKindId, `semanticPathProjectionContracts.${id}.sourceOwnerKindId`);
    const destinationOwnerKindId = requiredString(spec.destinationOwnerKindId, `semanticPathProjectionContracts.${id}.destinationOwnerKindId`);
    if (!semanticDirectoryMemberKinds[sourceOwnerKindId] || !semanticDirectoryMemberKinds[destinationOwnerKindId]) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} owner kind is invalid`);
    const profileRendererId = requiredString(spec.profileRendererId, `semanticPathProjectionContracts.${id}.profileRendererId`);
    const descendantContractId = requiredString(spec.descendantContractId, `semanticPathProjectionContracts.${id}.descendantContractId`);
    const catalogContractId = requiredString(spec.catalogContractId, `semanticPathProjectionContracts.${id}.catalogContractId`);
    if (!semanticPathProjectionProfileRenderers[profileRendererId] || !semanticDescendantContracts[descendantContractId] || !semanticPathProjectionCatalogContracts[catalogContractId]) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} references an unknown registry`);
    const rationaleRule = spec.rationaleRule as SemanticPathProjectionContract["rationaleRule"];
    const sourceArtifactMemberName = spec.sourceArtifactMemberName === undefined ? undefined : requiredString(spec.sourceArtifactMemberName, `semanticPathProjectionContracts.${id}.sourceArtifactMemberName`).normalize("NFC");
    const expectedArtifact = rationaleRule === "artifact-example-model-catalog-projection-v1" ? "📐️cad" : rationaleRule === "artifact-editor-command-projection-v1" ? "🖍️drawing" : undefined;
    if (sourceArtifactMemberName !== expectedArtifact) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id}.sourceArtifactMemberName does not match its rationale`);
    const sourceSegments = spec.sourceSegments.map((segment, index) => parseProjectionSegment(segment, `semanticPathProjectionContracts.${id}.sourceSegments[${index}]`, false) as SemanticProjectionSourceSegment);
    const destinationSegments = spec.destinationSegments.map((segment, index) => parseProjectionSegment(segment, `semanticPathProjectionContracts.${id}.destinationSegments[${index}]`, true) as SemanticProjectionDestinationSegment);
    const captures = sourceSegments.flatMap((segment) => "capture" in segment ? [segment.capture] : []);
    const expectedCaptures: readonly SemanticProjectionCaptureField[] = rationaleRule === "artifact-mutation-test-projection-v1" ? ["standardVersion", "subsetId", "mutationId", "scenarioId"] : rationaleRule === "artifact-editor-command-projection-v1" ? ["standardVersion", "subsetId", "commandDirectoryName"] : ["standardVersion", "subsetId"];
    if (canonicalJson(captures) !== canonicalJson(expectedCaptures)) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} has invalid captures for ${rationaleRule}`);
    const descendant = semanticDescendantContracts[descendantContractId];
    const catalog = semanticPathProjectionCatalogContracts[catalogContractId];
    if (rationaleRule === "artifact-mutation-test-projection-v1" ? "contractKind" in descendant || "contractKind" in catalog : rationaleRule === "artifact-example-model-catalog-projection-v1" ? !("contractKind" in descendant && descendant.contractKind === "catalog" && "contractKind" in catalog && catalog.contractKind === "distributed-json-manifest-catalog") : "contractKind" in descendant || !("contractKind" in catalog && catalog.contractKind === "exact-owner-vectors")) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} references incompatible descendant/catalog authorities`);
    const descendantNodes = "contractKind" in descendant ? [] : [...descendant.requiredNodes, ...descendant.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)];
    const sourceNamedNodes = descendantNodes.filter((node): node is SemanticDescendantKindNode & { readonly sourceFilename: string } => "kindId" in node && node.sourceFilename !== undefined);
    if (rationaleRule === "artifact-editor-command-projection-v1" ? sourceNamedNodes.length !== 3 || descendantNodes.filter((node) => "kindId" in node && node.nodeType === "file" && node.kindId === "rust-source").length !== 3 : sourceNamedNodes.length !== 0) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} has invalid source-filename descendant authority`);
    semanticPathProjectionContracts[id] = { sourceOwnerKindId, ...(sourceArtifactMemberName ? { sourceArtifactMemberName } : {}), sourceSegments, profileRendererId, destinationOwnerKindId, destinationSegments, descendantContractId, catalogContractId, rationaleRule };
  }
  if (Object.keys(semanticPathProjectionContracts).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionContracts must not be empty");
  for (const [id, spec] of Object.entries(semanticProjectedMemberKinds)) if (!semanticPathProjectionContracts[spec.projectionContractId]) throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown projection contract ${spec.projectionContractId}`);
  const semanticOwnedFileProjectionContracts: Record<string, SemanticOwnedFileProjectionContract> = {};
  for (const [id, value] of Object.entries(ownedFileProjectionRows)) {
    const name = `semanticOwnedFileProjectionContracts.${id}`;
    const spec = record(value, name);
    if (spec.contractKind === "exact-owner-path-catalog") {
      requireExactKeys(spec, ["contractKind", "authorityCatalogPath", "authorityCatalogSha256", "sourceFileKindId", "sourceBasenames", "destinationDirectoryKinds", "allowedDispositions", "ownerEvidenceKinds", "referenceOwnerIds", "generatorOwnerIds", "expectedCounts", "authoredDocumentCorrections", "rationaleRule", ...(Object.hasOwn(spec, "currentSourceRevisions") ? ["currentSourceRevisions"] : [])], name);
      const authorityCatalogPath = normalizeRelative(requiredString(spec.authorityCatalogPath, name + ".authorityCatalogPath"));
      const authorityCatalogSha256 = requiredString(spec.authorityCatalogSha256, name + ".authorityCatalogSha256");
      const sourceBasenames = stringArray(spec.sourceBasenames, name + ".sourceBasenames");
      const allowedDispositions = stringArray(spec.allowedDispositions, name + ".allowedDispositions");
      const ownerEvidenceKinds = stringArray(spec.ownerEvidenceKinds, name + ".ownerEvidenceKinds");
      const referenceOwnerIds = stringArray(spec.referenceOwnerIds, name + ".referenceOwnerIds");
      const generatorOwnerIds = stringArray(spec.generatorOwnerIds, name + ".generatorOwnerIds");
      const destinations = record(spec.destinationDirectoryKinds, name + ".destinationDirectoryKinds");
      requireExactKeys(destinations, ["license", "readme"], name + ".destinationDirectoryKinds");
      const parseDestination = (kind: "license" | "readme"): { readonly directoryKindId: "owner-license" | "owner-readme"; readonly directoryName: "⚖️license" | "📃️readme"; readonly filename: "📝️.md" } => {
        const destination = record(destinations[kind], name + ".destinationDirectoryKinds." + kind);
        requireExactKeys(destination, ["directoryKindId", "directoryName", "filename"], name + ".destinationDirectoryKinds." + kind);
        return { directoryKindId: requiredString(destination.directoryKindId, name + ".destinationDirectoryKinds." + kind + ".directoryKindId") as "owner-license" | "owner-readme", directoryName: requiredString(destination.directoryName, name + ".destinationDirectoryKinds." + kind + ".directoryName") as "⚖️license" | "📃️readme", filename: requiredString(destination.filename, name + ".destinationDirectoryKinds." + kind + ".filename") as "📝️.md" };
      };
      const destinationDirectoryKinds = { license: parseDestination("license"), readme: parseDestination("readme") };
      const expectedCounts = record(spec.expectedCounts, name + ".expectedCounts");
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
      for (const [capture, row] of Object.entries(record(spec.directoryCaptures, `${name}.directoryCaptures`))) {
        const captureSpec = record(row, `${name}.directoryCaptures.${capture}`);
        directoryCaptures[capture] = { kindIds: stringArray(captureSpec.kindIds, `${name}.directoryCaptures.${capture}.kindIds`), ...(captureSpec.names ? { names: stringArray(captureSpec.names, `${name}.directoryCaptures.${capture}.names`) } : {}) };
      }
      const ownerPathPatterns = Object.fromEntries(Object.entries(record(spec.ownerPathPatterns, `${name}.ownerPathPatterns`)).map(([form, pattern]) => [form, requiredString(pattern, `${name}.ownerPathPatterns.${form}`)]));
      const authoring = record(spec.authoringCommand, `${name}.authoringCommand`), consumer = record(spec.referenceConsumer, `${name}.referenceConsumer`);
      semanticOwnedFileProjectionContracts[id] = { contractKind: "semantic-facet-primary-file", sourceRoot: requiredString(spec.sourceRoot, `${name}.sourceRoot`), sourceFilename: requiredString(spec.sourceFilename, `${name}.sourceFilename`), fileKindAuthority: "windowEmptyFacetFileKindId", sourceDisposition: "authored", directoryCaptures, ownerPathPatterns, authoringCommand: { scriptPath: requiredString(authoring.scriptPath, `${name}.authoringCommand.scriptPath`), command: ["new", "surface"], writeDisposition: "create-if-absent" }, referenceConsumer: { path: requiredString(consumer.path, `${name}.referenceConsumer.path`), ownerRoot: requiredString(consumer.ownerRoot, `${name}.referenceConsumer.ownerRoot`), adapter: "rust", region: "✏️👁️Surfaces", lineTemplate: requiredString(consumer.lineTemplate, `${name}.referenceConsumer.lineTemplate`) }, rationaleRule: "artifact-empty-facet-primary-markdown-v1" };
      continue;
    }
    if (spec.contractKind === "owner-primary-file") {
      requireExactKeys(spec, ["contractKind", "ownerFixedDirectoryContractId", "sourceFileKindId", "sourceFilename", "destinationFilename", "rationaleRule"], name);
      const contract = { contractKind: "owner-primary-file" as const, ownerFixedDirectoryContractId: requiredString(spec.ownerFixedDirectoryContractId, `${name}.ownerFixedDirectoryContractId`), sourceFileKindId: requiredString(spec.sourceFileKindId, `${name}.sourceFileKindId`), sourceFilename: requiredString(spec.sourceFilename, `${name}.sourceFilename`), destinationFilename: requiredString(spec.destinationFilename, `${name}.destinationFilename`), rationaleRule: "ticket-document-primary-markdown-v1" as const };
      if (id !== contract.rationaleRule || spec.rationaleRule !== contract.rationaleRule || contract.ownerFixedDirectoryContractId !== "ticket-slug" || !fixedDirectoryRows[contract.ownerFixedDirectoryContractId] || contract.sourceFileKindId !== "markdown" || !fileKinds.markdown || fileKinds.markdown.extensionChains.length !== 1 || contract.sourceFilename !== "ticket.md" || contract.destinationFilename !== `${fileKinds.markdown.emoji}${fileKinds.markdown.extensionChains[0]}`) throw new Error(`Taxonomy v7 ${name} does not use the exact ticket document primary-leaf grammar`);
      semanticOwnedFileProjectionContracts[id] = contract;
      continue;
    }
    const ownerFixedDirectoryContractId = requiredString(spec.ownerFixedDirectoryContractId, `${name}.ownerFixedDirectoryContractId`);
    const sourceFileKindId = requiredString(spec.sourceFileKindId, `${name}.sourceFileKindId`);
    const destinationDirectoryKindId = requiredString(spec.destinationDirectoryKindId, `${name}.destinationDirectoryKindId`);
    if (!fixedDirectoryRows[ownerFixedDirectoryContractId] || !fileKinds[sourceFileKindId] || semanticDirectoryKinds[destinationDirectoryKindId]?.projectionOnly !== true) throw new Error(`Taxonomy v7 ${name} references unknown or non-projection authority`);
    const common = { ownerFixedDirectoryContractId, sourceFileKindId, sourceFilename: requiredString(spec.sourceFilename, `${name}.sourceFilename`), destinationDirectoryKindId, destinationDirectoryName: requiredString(spec.destinationDirectoryName, `${name}.destinationDirectoryName`), destinationFilename: requiredString(spec.destinationFilename, `${name}.destinationFilename`) };
    if (spec.contractKind === "owner-sibling-manifest-file") {
      requireExactKeys(spec, ["contractKind", "ownerFixedDirectoryContractId", "requiredSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "allowedStatuses", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "emptyContentRule", "statusDispositions", "rationaleRule"], name);
      const allowedStatuses = stringArray(spec.allowedStatuses, `${name}.allowedStatuses`);
      const statusDispositions = record(spec.statusDispositions, `${name}.statusDispositions`);
      requireExactKeys(statusDispositions, ["open", "closed-empty", "closed-nonempty", "invalid"], `${name}.statusDispositions`);
      const requiredSiblingFixedFilenameContractId = requiredString(spec.requiredSiblingFixedFilenameContractId, `${name}.requiredSiblingFixedFilenameContractId`);
      if (spec.manifestAdapter !== "json" || spec.manifestStatusLocation !== "status" || canonicalJson(allowedStatuses) !== canonicalJson(["closed", "open"]) || spec.emptyContentRule !== "zero-byte" || spec.rationaleRule !== "ticket-important-markdown-projection-v1" || canonicalJson(statusDispositions) !== canonicalJson({ open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" }) || !fixedFilenameContracts[requiredSiblingFixedFilenameContractId]) throw new Error(`Taxonomy v7 ${name} does not use the exact active owner-file projection grammar`);
      semanticOwnedFileProjectionContracts[id] = { contractKind: "owner-sibling-manifest-file", ...common, requiredSiblingFixedFilenameContractId, manifestAdapter: "json", manifestStatusLocation: "status", allowedStatuses: ["closed", "open"], emptyContentRule: "zero-byte", statusDispositions: { open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" }, rationaleRule: "ticket-important-markdown-projection-v1" };
    } else if (spec.contractKind === "owner-optional-sibling-manifest-file") {
      requireExactKeys(spec, ["contractKind", "ownerFixedDirectoryContractId", "optionalSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "admittedDispositions", "rationaleRule"], name);
      const optionalSiblingFixedFilenameContractId = requiredString(spec.optionalSiblingFixedFilenameContractId, `${name}.optionalSiblingFixedFilenameContractId`);
      const admittedDispositions = stringArray(spec.admittedDispositions, `${name}.admittedDispositions`);
      if (spec.manifestAdapter !== "json" || spec.manifestStatusLocation !== "status" || spec.rationaleRule !== "ticket-important-history-markdown-v1" || canonicalJson(admittedDispositions) !== canonicalJson(["closed-nonzero", "invalid-manifest", "missing-manifest"]) || !fixedFilenameContracts[optionalSiblingFixedFilenameContractId]) throw new Error(`Taxonomy v7 ${name} does not use the exact historical owner-file projection grammar`);
      semanticOwnedFileProjectionContracts[id] = { contractKind: "owner-optional-sibling-manifest-file", ...common, optionalSiblingFixedFilenameContractId, manifestAdapter: "json", manifestStatusLocation: "status", admittedDispositions: ["closed-nonzero", "invalid-manifest", "missing-manifest"], rationaleRule: "ticket-important-history-markdown-v1" };
    } else throw new Error(`Taxonomy v7 ${name}.contractKind is invalid`);
  }
  if (canonicalJson(Object.keys(semanticOwnedFileProjectionContracts)) !== canonicalJson(["artifact-empty-facet-primary-markdown-v1", "readme-license-owner-leaves-v1", "ticket-document-primary-markdown-v1", "ticket-important-history-markdown-v1", "ticket-important-markdown-v1"])) throw new Error("Taxonomy v7 semanticOwnedFileProjectionContracts must contain the exact artifact-facet, README/LICENSE, ticket-document, active, and history contracts");
  const semanticPathProjectionReferenceConsumerContracts: Record<string, SemanticPathProjectionReferenceConsumerContract> = {};
  const referenceConsumerForms = new Set<SemanticPathProjectionReferenceConsumerForm>(["path-reference", "artifact-catalog-glob", "artifact-catalog-prose:root-marker", "artifact-catalog-prose:relative-root", "artifact-catalog-prose:interaction-glob", "artifact-catalog-prose:catalog-grammar"]);
  const referenceConsumerAdapters = new Set<SemanticPathProjectionReferenceConsumerContract["adapters"][number]>(["rust", "typescript", "json", "toml"]);
  const referenceConsumerIdentities = new Set<string>();
  for (const [id, value] of Object.entries(projectionConsumerRows)) {
    const spec = record(value, `semanticPathProjectionReferenceConsumerContracts.${id}`);
    requireExactKeys(spec, ["projectionContractId", "consumerIdentity", "ownership", "sourcePathPattern", "sourcePathIdentities", "adapters", "supportedForms", "staleMarkers"], `semanticPathProjectionReferenceConsumerContracts.${id}`);
    const projectionContractId = requiredString(spec.projectionContractId, `semanticPathProjectionReferenceConsumerContracts.${id}.projectionContractId`);
    if (!semanticPathProjectionContracts[projectionContractId]) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} references an unknown projection contract`);
    if (spec.ownership !== "external") throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.ownership must be external`);
    const consumerIdentity = requiredString(spec.consumerIdentity, `semanticPathProjectionReferenceConsumerContracts.${id}.consumerIdentity`);
    if (referenceConsumerIdentities.has(consumerIdentity)) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts repeats consumer identity ${consumerIdentity}`);
    referenceConsumerIdentities.add(consumerIdentity);
    const sourcePathPattern = requiredString(spec.sourcePathPattern, `semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathPattern`);
    if (!sourcePathPattern.startsWith("^") || !sourcePathPattern.endsWith("$")) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathPattern must be a full-match expression`);
    const sourcePathRegex = new RegExp(sourcePathPattern, "u");
    const sourcePathIdentities = stringArray(spec.sourcePathIdentities, `semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathIdentities`);
    const adapters = stringArray(spec.adapters, `semanticPathProjectionReferenceConsumerContracts.${id}.adapters`) as readonly SemanticPathProjectionReferenceConsumerContract["adapters"][number][];
    const supportedForms = stringArray(spec.supportedForms, `semanticPathProjectionReferenceConsumerContracts.${id}.supportedForms`) as readonly SemanticPathProjectionReferenceConsumerForm[];
    const staleMarkers = stringArray(spec.staleMarkers, `semanticPathProjectionReferenceConsumerContracts.${id}.staleMarkers`);
    if (sourcePathIdentities.length === 0 || adapters.length === 0 || supportedForms.length === 0 || staleMarkers.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} must be nonempty`);
    if (new Set(sourcePathIdentities).size !== sourcePathIdentities.length || sourcePathIdentities.some((path) => path !== normalizeRelative(path) || !sourcePathRegex.test(path))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathIdentities are invalid`);
    if (new Set(adapters).size !== adapters.length || adapters.some((adapter) => !referenceConsumerAdapters.has(adapter))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.adapters are invalid`);
    if (new Set(supportedForms).size !== supportedForms.length || supportedForms.some((form) => !referenceConsumerForms.has(form))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.supportedForms are invalid`);
    if (new Set(staleMarkers).size !== staleMarkers.length || staleMarkers.some((marker) => !marker || marker !== marker.normalize("NFC"))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.staleMarkers are invalid`);
    semanticPathProjectionReferenceConsumerContracts[id] = { projectionContractId, consumerIdentity, ownership: "external", sourcePathPattern, sourcePathIdentities: [...sourcePathIdentities], adapters: [...adapters], supportedForms: [...supportedForms], staleMarkers: [...staleMarkers] };
  }
  if (Object.keys(semanticPathProjectionReferenceConsumerContracts).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionReferenceConsumerContracts must not be empty");
  const mutationCatalogProjection: MutationCatalogProjectionContractIds = {
    projectionContractId: requiredString(mutationCatalogProjectionRow.projectionContractId, "mutationCatalogProjection.projectionContractId"),
    projectedMemberKindId: requiredString(mutationCatalogProjectionRow.projectedMemberKindId, "mutationCatalogProjection.projectedMemberKindId"),
    descendantContractId: requiredString(mutationCatalogProjectionRow.descendantContractId, "mutationCatalogProjection.descendantContractId"),
    catalogContractId: requiredString(mutationCatalogProjectionRow.catalogContractId, "mutationCatalogProjection.catalogContractId"),
  };
  if (!semanticPathProjectionContracts[mutationCatalogProjection.projectionContractId] || !semanticProjectedMemberKinds[mutationCatalogProjection.projectedMemberKindId] || !semanticDescendantContracts[mutationCatalogProjection.descendantContractId] || !semanticPathProjectionCatalogContracts[mutationCatalogProjection.catalogContractId]) throw new Error("Taxonomy v7 mutationCatalogProjection references unknown projection registries");

  const generatorContracts: Record<string, GeneratorContractSpec> = {};
  const generatorRoots: { readonly id: string; readonly path: string }[] = [];
  for (const [id, value] of Object.entries(generatorRows)) {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id)) throw new Error(`Taxonomy v7 generatorContracts.${id} has an invalid identifier`);
    const spec = record(value, `generatorContracts.${id}`);
    if (spec.ownership !== "owned" && spec.ownership !== "external") throw new Error(`Taxonomy v7 generatorContracts.${id}.ownership is invalid`);
    const ownership = spec.ownership as GeneratorOwnership;
    const ownerPath = spec.ownerPath === null ? null : normalizeRelative(requiredString(spec.ownerPath, `generatorContracts.${id}.ownerPath`));
    const target = spec.target === null ? null : requiredString(spec.target, `generatorContracts.${id}.target`);
    const previewTarget = spec.previewTarget === undefined ? undefined : requiredString(spec.previewTarget, `generatorContracts.${id}.previewTarget`);
    const checkTarget = spec.checkTarget === undefined ? undefined : requiredString(spec.checkTarget, `generatorContracts.${id}.checkTarget`);
    if ((ownership === "owned") !== (ownerPath !== null && target !== null)) throw new Error(`Taxonomy v7 generatorContracts.${id} owner and target do not match ownership`);
    if (target && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(target)) throw new Error(`Taxonomy v7 generatorContracts.${id}.target must be one exact Nx target`);
    if (ownership === "owned" ? !previewTarget : previewTarget !== undefined) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget does not match ownership`);
    if (previewTarget && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(previewTarget)) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget must be one exact Nx target`);
    const previewArguments = spec.previewArguments === undefined ? undefined : stringArray(spec.previewArguments, `generatorContracts.${id}.previewArguments`);
    if (target) generatorPreviewScriptArguments({ ownership, target, previewTarget, previewArguments });
    else if (previewArguments !== undefined) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewArguments requires owned output authority`);
    const previewLimits = spec.previewLimits === undefined ? undefined : generatorPreviewResourceLimits({ ownership, previewTarget, previewLimits: spec.previewLimits as GeneratorContractSpec["previewLimits"] });
    const compilerInputManifest = spec.compilerInputManifest === undefined ? undefined : record(spec.compilerInputManifest, `generatorContracts.${id}.compilerInputManifest`) as unknown as GeneratorContractSpec["compilerInputManifest"];
    if (checkTarget && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(checkTarget)) throw new Error(`Taxonomy v7 generatorContracts.${id}.checkTarget must be one exact Nx target`);
    const inputPatterns = stringArray(spec.inputPatterns, `generatorContracts.${id}.inputPatterns`).map((pattern, index) => validatedContractPattern(pattern, `generatorContracts.${id}.inputPatterns[${index}]`, false));
    if (ownership === "owned" ? inputPatterns.length === 0 : inputPatterns.length !== 0) throw new Error(`Taxonomy v7 generatorContracts.${id}.inputPatterns do not match ownership`);
    const outputRows = spec.outputRoots;
    if (!Array.isArray(outputRows) || outputRows.length === 0) throw new Error(`Taxonomy v7 generatorContracts.${id}.outputRoots must not be empty`);
    const outputRoots = outputRows.map((value, index) => {
      const output = record(value, `generatorContracts.${id}.outputRoots[${index}]`);
      const outputPath = requiredString(output.path, `generatorContracts.${id}.outputRoots[${index}].path`);
      if (outputPath !== normalizeRelative(outputPath) || /[*?\[\]]/u.test(outputPath)) throw new Error(`Taxonomy v7 generatorContracts.${id} output path must be one literal NFC repository path`);
      if (output.inclusion !== "tracked" && output.inclusion !== "ignored") throw new Error(`Taxonomy v7 generatorContracts.${id} output inclusion is invalid`);
      generatorRoots.push({ id, path: outputPath });
      return { path: outputPath, inclusion: output.inclusion } as GeneratorOutputRootSpec;
    }).sort((left, right) => left.path.localeCompare(right.path));
    if (new Set(outputRoots.map((output) => output.path)).size !== outputRoots.length) throw new Error(`Taxonomy v7 generatorContracts.${id} repeats an output root`);
    const inputDiscovery = spec.inputDiscovery === undefined ? undefined : record(spec.inputDiscovery, `generatorContracts.${id}.inputDiscovery`) as unknown as RegistryCatalogInputDiscovery;
    if (inputDiscovery && (id !== "plugin-registry" || ownership !== "owned" || inputDiscovery.kind !== "registry-catalog")) throw new Error(`Taxonomy v7 generatorContracts.${id}.inputDiscovery has no exact catalog authority`);
    const projectionActivation = spec.projectionActivation === undefined ? undefined : record(spec.projectionActivation, `generatorContracts.${id}.projectionActivation`) as unknown as GeneratorProjectionActivation;
    const packageGeneration = spec.packageGeneration === undefined ? undefined : record(spec.packageGeneration, `generatorContracts.${id}.packageGeneration`) as unknown as SemanticPackageGeneration;
    generatorContracts[id] = { ownership, ownerPath, target, previewTarget, previewArguments, previewLimits, checkTarget, inputPatterns: [...new Set(inputPatterns)].sort(), inputDiscovery, compilerInputManifest, packageGeneration, projectionActivation, outputRoots, reason: requiredString(spec.reason, `generatorContracts.${id}.reason`) };
  }
  if (Object.keys(generatorContracts).length === 0) throw new Error("Taxonomy v7 generatorContracts must not be empty");
  for (let left = 0; left < generatorRoots.length; left++) for (let right = left + 1; right < generatorRoots.length; right++) {
    const a = generatorRoots[left];
    const b = generatorRoots[right];
    if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`)) throw new Error(`Taxonomy v7 generator output roots overlap: ${a.id}:${a.path} and ${b.id}:${b.path}`);
  }

  const ecosystems: Record<string, EcosystemSpec> = {};
  for (const [id, value] of Object.entries(ecosystemRows)) {
    const spec = record(value, `ecosystems.${id}`);
    if (spec.packageIdentity !== "manifest" && spec.packageIdentity !== "boundary-only") throw new Error(`Taxonomy v7 ecosystems.${id}.packageIdentity is invalid`);
    const manifestContractId = spec.manifestContractId === null ? null : requiredString(spec.manifestContractId, `ecosystems.${id}.manifestContractId`);
    if ((spec.packageIdentity === "manifest") !== (manifestContractId !== null)) throw new Error(`Taxonomy v7 ecosystems.${id} manifest identity is incomplete`);
    ecosystems[id] = { packageIdentity: spec.packageIdentity, manifestContractId };
  }
  if (Object.keys(ecosystems).length === 0) throw new Error("Taxonomy v7 ecosystems must not be empty");

  const packageGlueGrammar: Record<string, PackageGlueGrammar> = {};
  for (const [id, value] of Object.entries(grammarRows)) {
    const spec = record(value, `packageGlueGrammar.${id}`);
    if (!["rust", "typescript", "javascript", "go", "python", "dotnet", "c-cpp"].includes(String(spec.analyzer))) throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.analyzer is invalid`);
    const allowedRoles = stringArray(spec.allowedRoles, `packageGlueGrammar.${id}.allowedRoles`) as PackageGlueGrammar["allowedRoles"];
    if (allowedRoles.some((role) => !["declaration", "registration", "bootstrap", "thin-delegation"].includes(role)) || new Set(allowedRoles).size !== allowedRoles.length) throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.allowedRoles is invalid`);
    if (!Number.isSafeInteger(spec.maxDelegationStatements) || (spec.maxDelegationStatements as number) < 0) throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.maxDelegationStatements is invalid`);
    packageGlueGrammar[id] = { analyzer: spec.analyzer as PackageGlueGrammar["analyzer"], allowedRoles, maxDelegationStatements: spec.maxDelegationStatements as number };
  }

  const packageBoundaryRules: Record<string, PackageBoundaryRule> = {};
  for (const [id, value] of Object.entries(boundaryRows)) {
    const spec = record(value, `packageBoundaryRules.${id}`);
    const glueGrammarId = requiredString(spec.glueGrammarId, `packageBoundaryRules.${id}.glueGrammarId`);
    if (!packageGlueGrammar[glueGrammarId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown grammar ${glueGrammarId}`);
    if (spec.recursive !== true || spec.uncertainRole !== "problem" || spec.implementationRole !== "problem") throw new Error(`Taxonomy v7 packageBoundaryRules.${id} must be recursive and fail closed`);
    packageBoundaryRules[id] = {
      manifestContractId: spec.manifestContractId === null ? null : requiredString(spec.manifestContractId, `packageBoundaryRules.${id}.manifestContractId`),
      entryContractIds: stringArray(spec.entryContractIds, `packageBoundaryRules.${id}.entryContractIds`),
      allowedFixedContractIds: stringArray(spec.allowedFixedContractIds, `packageBoundaryRules.${id}.allowedFixedContractIds`),
      allowedFileKindIds: stringArray(spec.allowedFileKindIds, `packageBoundaryRules.${id}.allowedFileKindIds`),
      allowedDirectoryKindIds: stringArray(spec.allowedDirectoryKindIds, `packageBoundaryRules.${id}.allowedDirectoryKindIds`),
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
    const spec = record(value, `packageBoundaryProfiles.${id}`);
    requireExactKeys(spec, ["admission", "allowedFileKindIds", "allowedDirectoryKindIds", "allowedFixedContractIds", "glueGrammarId", "recursive", "uncertainRole", "implementationRole", "reason"], `packageBoundaryProfiles.${id}`);
    if (spec.admission !== "blocked-until-language-directory-registered" || spec.recursive !== true || spec.uncertainRole !== "problem" || spec.implementationRole !== "problem") throw new Error(`Taxonomy v7 packageBoundaryProfiles.${id} must remain fail-closed`);
    const glueGrammarId = requiredString(spec.glueGrammarId, `packageBoundaryProfiles.${id}.glueGrammarId`);
    if (!packageGlueGrammar[glueGrammarId]) throw new Error(`Taxonomy v7 packageBoundaryProfiles.${id} references unknown grammar`);
    packageBoundaryProfiles[id] = { admission: "blocked-until-language-directory-registered", allowedFileKindIds: stringArray(spec.allowedFileKindIds, `packageBoundaryProfiles.${id}.allowedFileKindIds`), allowedDirectoryKindIds: stringArray(spec.allowedDirectoryKindIds, `packageBoundaryProfiles.${id}.allowedDirectoryKindIds`), allowedFixedContractIds: stringArray(spec.allowedFixedContractIds, `packageBoundaryProfiles.${id}.allowedFixedContractIds`), glueGrammarId, recursive: true, uncertainRole: "problem", implementationRole: "problem", reason: requiredString(spec.reason, `packageBoundaryProfiles.${id}.reason`) };
  }
  if (Object.keys(packageBoundaryProfiles).length === 0) throw new Error("Taxonomy v7 packageBoundaryProfiles must not be empty");
  /** 🔖️ Each externally-mandated tool-config validator token is reserved for exactly one contract id, mirroring the pre-existing vitest-configuration/vitest-config-entry pinning. */
  const TOOL_CONFIG_VALIDATORS: Readonly<Record<string, string>> = { "vitest-configuration": "vitest-config-entry", "tool-config-vitest": "vitest-config", "tool-config-tailwind": "tailwind-config", "tool-config-postcss": "postcss-config", "tool-config-eslint": "eslint-config", "tool-config-dependency-cruiser": "dependency-cruiser-config", "pytest-configuration": "root-pytest-config", "eslint-configuration": "root-eslint-config", "vscode-test-configuration": "vscode-test-cli-config" };
  const packageSourceDispositions: Record<string, PackageSourceDisposition> = {};
  for (const [id, value] of Object.entries(sourceDispositionRows)) {
    const spec = record(value, `packageSourceDispositions.${id}`);
    requireExactKeys(spec, ["contractKind", "disposition", "validator", "authority", "verification"], `packageSourceDispositions.${id}`);
    const configValidatorOwner = TOOL_CONFIG_VALIDATORS[spec.validator as string];
    if (spec.contractKind !== "fixed" && spec.contractKind !== "configurable" || spec.disposition !== "adapter-source" && spec.disposition !== "tool-metadata" || spec.validator !== "package-glue" && spec.validator !== "command-router" && configValidatorOwner === undefined || (configValidatorOwner !== undefined && id !== configValidatorOwner)) throw new Error(`Taxonomy v7 packageSourceDispositions.${id} is invalid`);
    packageSourceDispositions[id] = { contractKind: spec.contractKind, disposition: spec.disposition, validator: spec.validator, authority: requiredString(spec.authority, `packageSourceDispositions.${id}.authority`), verification: requiredString(spec.verification, `packageSourceDispositions.${id}.verification`) };
  }
  if (Object.keys(packageSourceDispositions).length === 0) throw new Error("Taxonomy v7 packageSourceDispositions must not be empty");
  for (const [id, contract] of Object.entries(fixedFilenameContracts)) if (contract.scope.kind === "package-root" && !packageBoundaryRules[contract.scope.ecosystemId]) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id} references unknown ecosystem ${contract.scope.ecosystemId}`);

  const pathExclusions: Record<string, { path: string; mode: "opaque"; reason: string }> = {};
  const exclusions: { id: string; path: string }[] = [];
  for (const [id, value] of Object.entries(exclusionRows)) {
    const spec = record(value, `pathExclusions.${id}`);
    if (spec.mode !== "opaque") throw new Error(`Taxonomy v7 pathExclusions.${id}.mode must be opaque`);
    const excludedPath = normalizeRelative(requiredString(spec.path, `pathExclusions.${id}.path`));
    pathExclusions[id] = { path: excludedPath, mode: "opaque", reason: requiredString(spec.reason, `pathExclusions.${id}.reason`) };
    exclusions.push({ id, path: excludedPath });
  }
  if (canonicalJson(Object.entries(pathExclusions).map(([id, spec]) => [id, spec.path])) !== canonicalJson([["compose", "compose"], ["temp-compose", "temp/compose"]])) throw new Error("Taxonomy v7 pathExclusions must contain exactly opaque compose and temp/compose");
  for (const id of stringArray(enforcement.opaquePathExclusionIds, "areaEnforcement.opaquePathExclusionIds")) {
    if (!pathExclusions[id]) throw new Error(`Taxonomy v7 areaEnforcement references unknown opaque exclusion ${id}`);
  }
  if (canonicalJson(enforcement.opaquePathExclusionIds) !== canonicalJson(["compose", "temp-compose"])) throw new Error("Taxonomy v7 areaEnforcement must require compose and temp-compose in order");
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
    windowEmptyFacetFileKindId: requiredString(root.windowEmptyFacetFileKindId, "windowEmptyFacetFileKindId"),
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
    path,
    pathMatcher,
    schema,
    discoverySchema: root as unknown as DiscoveryTaxonomy,
    exclusions: exclusions.sort((a, b) => a.path.localeCompare(b.path)),
    fileKinds: Object.entries(fileKinds).map(([id, spec]) => ({ id, ...spec })).sort((a, b) => a.id.localeCompare(b.id)),
    directoryKinds: Object.entries(semanticDirectoryKinds).map(([id, spec]) => ({ id, ...spec, slugRegex: new RegExp(`^(?:${spec.slugPattern})$`, "u") })).sort((a, b) => a.id.localeCompare(b.id)),
  };
}

function loadTaxonomy(options: Pick<TaxonomyInventoryOptions, "repoRoot" | "taxonomyPath">): LoadedTaxonomy {
  const path = assertLexicalInputOutsideOpaque(options.repoRoot, options.taxonomyPath ?? TAXONOMY_RELATIVE_PATH, "taxonomyPath", true);
  const input = semanticOwnedInputFileSnapshot(options.repoRoot, relative(resolve(options.repoRoot), path).replaceAll("\\", "/"));
  if (!input) throw new Error("Taxonomy schema is absent: " + path);
  const bytes = Buffer.from(input.bytes), text = bytes.toString("utf8");
  if (!Buffer.from(text).equals(bytes)) throw new Error("Taxonomy schema has lossy UTF-8: " + path);
  return { ...parseTaxonomy(JSON.parse(text) as unknown, path), input };
}
//#endregion 🔣️Schema

//#region 🧮️Canonicalization
function sha256(value: string | Uint8Array): string {
  return createHash("sha256").update(value).digest("hex");
}

function canonicalArrayKey(value: unknown): string | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  const row = value as JsonRecord;
  const keys = ["operationId", "sourcePath", "path", "id", "destinationPath", "code", "relativeRoot", "structuredLocation"];
  const parts = keys.filter((key) => typeof row[key] === "string").map((key) => `${key}:${row[key] as string}`);
  return parts.length > 0 ? parts.join("\u0000") : null;
}

function canonicalValue(value: unknown): unknown {
  if (Array.isArray(value)) {
    const rows = value.map(canonicalValue);
    if (rows.every((row) => canonicalArrayKey(row) !== null)) return [...rows].sort((a, b) => Buffer.from(canonicalArrayKey(a) as string).compare(Buffer.from(canonicalArrayKey(b) as string)));
    return rows;
  }
  if (!value || typeof value !== "object") return value;
  const source = value as JsonRecord;
  const target: JsonRecord = {};
  for (const key of Object.keys(source).sort()) {
    if (source[key] !== undefined) target[key] = canonicalValue(source[key]);
  }
  return target;
}

/** 🧾️ Serializes repository-owned records with recursively sorted keys and contract-identifier arrays. */
export function canonicalJson(value: unknown): string {
  return JSON.stringify(canonicalValue(value));
}

const PLAN_HASH = /^[a-f0-9]{64}$/u;
const PLAN_OPERATION_ID = /^[a-f0-9]{24}$/u;
const PLAN_COMMIT_ID = /^[a-f0-9]{40}$/u;

function planRecord(value: unknown, name: string, requiredKeys: readonly string[], optionalKeys: readonly string[] = []): JsonRecord {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error(`${name} must be an object`);
  const row = value as JsonRecord;
  const allowed = new Set([...requiredKeys, ...optionalKeys]);
  const keys = Object.keys(row);
  if (requiredKeys.some((key) => !(key in row)) || keys.some((key) => !allowed.has(key))) throw new Error(`${name} has missing or unknown keys`);
  return row;
}

function planString(value: unknown, name: string, pattern?: RegExp): string {
  if (typeof value !== "string" || (pattern && !pattern.test(value))) throw new Error(`${name} is invalid`);
  return value;
}

function planPath(value: unknown, name: string): string {
  const path = planString(value, name);
  if (path === "" || path !== normalizeRelative(path) || path !== path.normalize("NFC")) throw new Error(`${name} is not a canonical repository-relative path`);
  return path;
}

function planInteger(value: unknown, name: string, maximum = Number.MAX_SAFE_INTEGER): number {
  if (!Number.isSafeInteger(value) || (value as number) < 0 || (value as number) > maximum) throw new Error(`${name} is invalid`);
  return value as number;
}

function planStringArray(value: unknown, name: string, pattern?: RegExp): string[] {
  if (!Array.isArray(value)) throw new Error(`${name} must be an array`);
  return value.map((entry, index) => planString(entry, `${name}[${index}]`, pattern));
}

function parseLeafPreimage(value: unknown, name: string): TaxonomyLeafPreimage {
  const base = planRecord(value, name, ["nodeKind", "contentHash", "mode", "size"], ["target"]);
  const row = base.nodeKind === "symlink" ? planRecord(value, name, ["nodeKind", "contentHash", "mode", "size", "target"]) : planRecord(value, name, ["nodeKind", "contentHash", "mode", "size"]);
  if (row.nodeKind !== "file" && row.nodeKind !== "symlink") throw new Error(`${name}.nodeKind is invalid`);
  const contentHash = planString(row.contentHash, `${name}.contentHash`, PLAN_HASH);
  const mode = planInteger(row.mode, `${name}.mode`, 0o7777);
  const size = planInteger(row.size, `${name}.size`);
  if (row.nodeKind === "file") return { nodeKind: "file", contentHash, mode, size };
  const target = planString(row.target, `${name}.target`);
  if (sha256(target) !== contentHash || Buffer.byteLength(target) !== size) throw new Error(`${name} symlink target does not match its hash and size`);
  return { nodeKind: "symlink", contentHash, mode, size, target };
}

function parsePathPreimage(value: unknown, name: string): TaxonomyPathPreimage {
  const base = planRecord(value, name, ["state"], ["contentHash", "mode", "size", "target"]);
  if (!["absent", "directory", "file", "symlink"].includes(String(base.state))) throw new Error(`${name}.state is invalid`);
  if (base.state === "absent" || base.state === "directory") {
    if (Object.keys(base).length !== 1) throw new Error(`${name} absent preimage cannot have payload`);
    return { state: base.state };
  }
  if (typeof base.contentHash !== "string" || !PLAN_HASH.test(base.contentHash) || !Number.isSafeInteger(base.mode) || !Number.isSafeInteger(base.size)) throw new Error(`${name} present preimage requires hash, mode and size`);
  const contentHash = base.contentHash, mode = planInteger(base.mode, `${name}.mode`, 0o7777), size = planInteger(base.size, `${name}.size`);
  if (base.state === "file") {
    if (base.target !== undefined) throw new Error(`${name} file preimage cannot have a symlink target`);
    return { state: "file", contentHash, mode, size };
  }
  const target = planString(base.target, `${name}.target`);
  if (sha256(target) !== contentHash || Buffer.byteLength(target) !== size) throw new Error(`${name} symlink target does not match its hash and size`);
  return { state: "symlink", contentHash, mode, size, target };
}

function parseNoFollowTreeDigest(value: unknown, name: string): TaxonomyNoFollowTreeDigest {
  const row = planRecord(value, name, ["algorithm", "digest", "files", "directories", "symlinks", "others"]);
  if (row.algorithm !== "sha256-no-follow-merkle-v1") throw new Error(`${name}.algorithm is invalid`);
  return { algorithm: row.algorithm, digest: planString(row.digest, `${name}.digest`, PLAN_HASH), files: planInteger(row.files, `${name}.files`), directories: planInteger(row.directories, `${name}.directories`), symlinks: planInteger(row.symlinks, `${name}.symlinks`), others: planInteger(row.others, `${name}.others`) };
}

function dispositionOperationId(domain: string, value: object): string {
  return sha256(`${domain}\u0000${canonicalJson(value)}`).slice(0, 24);
}

function parseEvidenceMember(value: unknown, name: string): TaxonomyEvidenceMember {
  const row = planRecord(value, name, ["sourcePath", "finalPath", "disposition", "preimage"]);
  if (!["remove", "retain", "relocate"].includes(String(row.disposition))) throw new Error(`${name}.disposition is invalid`);
  return { sourcePath: planPath(row.sourcePath, `${name}.sourcePath`), finalPath: planPath(row.finalPath, `${name}.finalPath`), disposition: row.disposition as TaxonomyEvidenceMember["disposition"], preimage: parseLeafPreimage(row.preimage, `${name}.preimage`) };
}

function parseRemovalAuthority(value: unknown, name: string): TaxonomyRemovalAuthority {
  const candidate = planRecord(value, name, ["kind"], ["evidenceSetDigest", "retainedFinalPath", "members", "catalogPath", "catalogContentHash", "caseId", "sourcePath", "sourcePreimage", "disposition", "contractId", "ownerPath", "manifestPath", "manifestPreimage", "status", "contentState", "fixturePath", "fixtureContentHash", "serializedInputPath", "expectedViolationCode", "authorityDigest", "generatorContractId", "destinationPath", "outputPreimage", "packageId"]);
  if (candidate.kind === "nested-cargo-generated-source") {
    const row = planRecord(value, name, ["kind", "catalogPath", "catalogContentHash", "packageId", "generatorContractId", "destinationPath", "sourcePreimage", "authorityDigest"]);
    const sourcePreimage = parseLeafPreimage(row.sourcePreimage, name + ".sourcePreimage");
    if (row.packageId !== "wgpu-renderer" || sourcePreimage.nodeKind !== "file") throw new Error(name + " requires the exact generated WGPU source file");
    const result = { kind: "nested-cargo-generated-source" as const, catalogPath: planPath(row.catalogPath, name + ".catalogPath"), catalogContentHash: planString(row.catalogContentHash, name + ".catalogContentHash", PLAN_HASH), packageId: row.packageId, generatorContractId: planString(row.generatorContractId, name + ".generatorContractId"), destinationPath: planPath(row.destinationPath, name + ".destinationPath"), sourcePreimage, authorityDigest: planString(row.authorityDigest, name + ".authorityDigest", PLAN_HASH) };
    const { authorityDigest: _digest, ...digestible } = result;
    if (result.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(name + ".authorityDigest does not match nested Cargo generated source authority");
    return result;
  }
  if (candidate.kind === "exact-owner-generated-source") {
    const row = planRecord(value, name, ["kind", "catalogPath", "catalogContentHash", "generatorContractId", "destinationPath", "outputPreimage", "authorityDigest"]);
    const outputPreimage = parseLeafPreimage(row.outputPreimage, name + ".outputPreimage");
    if (outputPreimage.nodeKind !== "file") throw new Error(name + " generated owner output must be a regular file");
    const result = { kind: "exact-owner-generated-source" as const, catalogPath: planPath(row.catalogPath, name + ".catalogPath"), catalogContentHash: planString(row.catalogContentHash, name + ".catalogContentHash", PLAN_HASH), generatorContractId: planString(row.generatorContractId, name + ".generatorContractId"), destinationPath: planPath(row.destinationPath, name + ".destinationPath"), outputPreimage, authorityDigest: planString(row.authorityDigest, name + ".authorityDigest", PLAN_HASH) };
    const { authorityDigest: _digest, ...digestible } = result;
    if (result.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(name + ".authorityDigest does not match generated source authority");
    return result;
  }
  if (candidate.kind === "byte-and-mode-identical") {
    const row = planRecord(value, name, ["kind", "evidenceSetDigest", "retainedFinalPath", "members"]);
    if (!Array.isArray(row.members) || row.members.length < 2) throw new Error(`${name}.members must contain complete retained evidence`);
    const members = row.members.map((entry, index) => parseEvidenceMember(entry, `${name}.members[${index}]`));
    const keys = members.map((entry) => Buffer.from(entry.sourcePath).toString("hex"));
    if (keys.some((key, index) => index > 0 && keys[index - 1] >= key)) throw new Error(`${name}.members are not unique and bytewise path sorted`);
    const identity = canonicalJson(members[0].preimage);
    if (members.some((entry) => canonicalJson(entry.preimage) !== identity)) throw new Error(`${name}.members are not byte, kind, mode and size identical`);
    const retainedFinalPath = planPath(row.retainedFinalPath, `${name}.retainedFinalPath`);
    if (!members.some((entry) => entry.disposition !== "remove" && entry.finalPath === retainedFinalPath)) throw new Error(`${name}.retainedFinalPath has no retained member`);
    const digestible = { algorithm: "sha256-byte-mode-evidence-set-v1", members, retainedFinalPath };
    const evidenceSetDigest = planString(row.evidenceSetDigest, `${name}.evidenceSetDigest`, PLAN_HASH);
    if (evidenceSetDigest !== sha256(canonicalJson(digestible))) throw new Error(`${name}.evidenceSetDigest does not match its members`);
    return { kind: "byte-and-mode-identical", evidenceSetDigest, retainedFinalPath, members };
  }
  if (candidate.kind === "owner-manifest-status") {
    const row = planRecord(value, name, ["kind", "contractId", "ownerPath", "manifestPath", "manifestPreimage", "status", "contentState", "authorityDigest"]);
    const manifestPreimage = parseLeafPreimage(row.manifestPreimage, `${name}.manifestPreimage`);
    if (manifestPreimage.nodeKind !== "file" || row.contractId !== "ticket-important-markdown-v1" || row.status !== "closed" || row.contentState !== "zero-byte") throw new Error(`${name} does not use the exact closed-empty ticket authority`);
    const result = { kind: "owner-manifest-status" as const, contractId: row.contractId, ownerPath: planPath(row.ownerPath, `${name}.ownerPath`), manifestPath: planPath(row.manifestPath, `${name}.manifestPath`), manifestPreimage, status: row.status, contentState: row.contentState, authorityDigest: planString(row.authorityDigest, `${name}.authorityDigest`, PLAN_HASH) };
    const { authorityDigest: _digest, ...digestible } = result;
    if (result.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(`${name}.authorityDigest does not match its authority`);
    return result;
  }
  if (candidate.kind === "exact-path-mutation") {
    const row = planRecord(value, name, ["kind", "catalogPath", "catalogContentHash", "caseId", "sourcePath", "sourcePreimage", "disposition", "authorityDigest"]);
    const sourcePreimage = parseLeafPreimage(row.sourcePreimage, `${name}.sourcePreimage`);
    if (sourcePreimage.nodeKind !== "file" || sourcePreimage.size !== 0 || row.disposition !== "remove") throw new Error(`${name} does not use exact empty-file removal evidence`);
    const result = { kind: "exact-path-mutation" as const, catalogPath: planPath(row.catalogPath, `${name}.catalogPath`), catalogContentHash: planString(row.catalogContentHash, `${name}.catalogContentHash`, PLAN_HASH), caseId: planString(row.caseId, `${name}.caseId`), sourcePath: planPath(row.sourcePath, `${name}.sourcePath`), sourcePreimage, disposition: row.disposition, authorityDigest: planString(row.authorityDigest, `${name}.authorityDigest`, PLAN_HASH) };
    const { authorityDigest: _digest, ...digestible } = result;
    if (result.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(`${name}.authorityDigest does not match its authority`);
    return result;
  }
  if (candidate.kind === "serialized-path-sentinel") {
    const row = planRecord(value, name, ["kind", "fixturePath", "fixtureContentHash", "caseId", "serializedInputPath", "expectedViolationCode", "authorityDigest"]);
    if (row.expectedViolationCode !== "windows-reserved-name" && row.expectedViolationCode !== "trailing-dot-or-space") throw new Error(`${name}.expectedViolationCode is invalid`);
    const result = { kind: "serialized-path-sentinel" as const, fixturePath: planPath(row.fixturePath, `${name}.fixturePath`), fixtureContentHash: planString(row.fixtureContentHash, `${name}.fixtureContentHash`, PLAN_HASH), caseId: planString(row.caseId, `${name}.caseId`), serializedInputPath: planString(row.serializedInputPath, `${name}.serializedInputPath`), expectedViolationCode: row.expectedViolationCode as "windows-reserved-name" | "trailing-dot-or-space", authorityDigest: planString(row.authorityDigest, `${name}.authorityDigest`, PLAN_HASH) };
    const { authorityDigest: _digest, ...digestible } = result;
    if (result.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(`${name}.authorityDigest does not match its authority`);
    return result;
  }
  throw new Error(`${name}.kind is invalid`);
}

function parseReferenceEdit(value: unknown, name: string): ReferenceEdit {
  const row = planRecord(value, name, ["path", "adapter", "structuredLocation", "oldValue", "newValue", "preimage"]);
  const adapters: readonly TaxonomyReferenceAdapter[] = ["rust", "typescript", "go", "python", "dotnet", "native", "json", "jsonc", "toml", "yaml", "xml", "markdown", "gherkin"];
  if (!adapters.includes(row.adapter as TaxonomyReferenceAdapter)) throw new Error(`${name}.adapter is invalid`);
  const preimage = parseLeafPreimage(row.preimage, `${name}.preimage`);
  if (preimage.nodeKind !== "file") throw new Error(`${name}.preimage must be a regular file`);
  return { path: planPath(row.path, `${name}.path`), adapter: row.adapter as TaxonomyReferenceAdapter, structuredLocation: planString(row.structuredLocation, `${name}.structuredLocation`), oldValue: planString(row.oldValue, `${name}.oldValue`), newValue: planString(row.newValue, `${name}.newValue`), preimage };
}

/** 🪪️ Parses exact role-labelled current-source evidence without authenticating its current bytes. */
function parseMoveSourceAuthority(value: unknown, name: string): TaxonomyMoveSourceAuthority {
  const row = planRecord(value, name, ["kind", "revisionId", "revisionDigest", "inputs"]);
  if (row.kind !== "exact-owner-current-source-revision-v1" || row.revisionId !== "testing-readme-protocol-v2-reviewed") throw new Error(`${name} has an unsupported current-source revision`);
  if (!Array.isArray(row.inputs) || row.inputs.length !== 3) throw new Error(`${name}.inputs must contain exactly three role-labelled files`);
  const roles = new Set<string>(), paths = new Set<string>();
  const inputs = row.inputs.map((value: unknown, index: number): TaxonomyMoveSourceAuthority["inputs"][number] => {
    const label = `${name}.inputs[${index}]`, input = planRecord(value, label, ["role", "path", "preimage"]);
    if (input.role !== "schema" && input.role !== "catalog" && input.role !== "expectation") throw new Error(`${label}.role is invalid`);
    const path = planPath(input.path, `${label}.path`);
    if (/[\\\u0000-\u001f:*?"<>|]/u.test(path) || Buffer.from(path).toString("utf8") !== path || /^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) throw new Error(`${label}.path is unsafe`);
    if (roles.has(input.role) || paths.has(path)) throw new Error(`${name}.inputs have duplicate roles or paths`);
    roles.add(input.role); paths.add(path);
    const preimage = parseLeafPreimage(input.preimage, `${label}.preimage`);
    if (preimage.nodeKind !== "file") throw new Error(`${label}.preimage must be a regular file`);
    return { role: input.role, path, preimage };
  });
  return { kind: row.kind, revisionId: row.revisionId, revisionDigest: planString(row.revisionDigest, `${name}.revisionDigest`, PLAN_HASH), inputs };
}

function parseMove(value: unknown, name: string): TaxonomyMove {
  const row = planRecord(value, name, ["operationId", "sourcePath", "destinationPath", "sourcePreimage", "rationaleRule", "ownerId", "referenceEdits"], ["collisionGroup", "sourceAuthority"]);
  if (!Array.isArray(row.referenceEdits)) throw new Error(`${name}.referenceEdits must be an array`);
  const result: TaxonomyMove = { operationId: planString(row.operationId, `${name}.operationId`, PLAN_OPERATION_ID), sourcePath: planPath(row.sourcePath, `${name}.sourcePath`), destinationPath: planPath(row.destinationPath, `${name}.destinationPath`), sourcePreimage: parseLeafPreimage(row.sourcePreimage, `${name}.sourcePreimage`), rationaleRule: planString(row.rationaleRule, `${name}.rationaleRule`), ownerId: planString(row.ownerId, `${name}.ownerId`), collisionGroup: row.collisionGroup === undefined ? undefined : planString(row.collisionGroup, `${name}.collisionGroup`), referenceEdits: row.referenceEdits.map((entry, index) => parseReferenceEdit(entry, `${name}.referenceEdits[${index}]`)), ...(row.sourceAuthority === undefined ? {} : { sourceAuthority: parseMoveSourceAuthority(row.sourceAuthority, `${name}.sourceAuthority`) }) };
  if (result.operationId !== dispositionOperationId("move-v2", { sourcePath: result.sourcePath, destinationPath: result.destinationPath, sourcePreimage: result.sourcePreimage })) throw new Error(`${name}.operationId does not match its fields`);
  return result;
}

function parseGeneratorNodeRecord(value: unknown, name: string): TaxonomyGeneratorNodeRecord {
  const base = planRecord(value, name, ["path", "nodeKind", "contentHash", "mode"], ["size", "target"]);
  const path = planPath(base.path, `${name}.path`), contentHash = planString(base.contentHash, `${name}.contentHash`, PLAN_HASH), mode = planInteger(base.mode, `${name}.mode`, 0o7777);
  if (base.nodeKind === "directory") {
    if (base.size !== undefined || base.target !== undefined) throw new Error(`${name} directory cannot carry leaf evidence`);
    return { path, nodeKind: "directory", contentHash, mode };
  }
  const size = planInteger(base.size, `${name}.size`);
  if (base.nodeKind === "file") {
    if (base.target !== undefined) throw new Error(`${name} file cannot carry a symlink target`);
    return { path, nodeKind: "file", contentHash, mode, size };
  }
  if (base.nodeKind !== "symlink") throw new Error(`${name}.nodeKind is invalid`);
  const target = planString(base.target, `${name}.target`);
  if (sha256(target) !== contentHash || Buffer.byteLength(target) !== size) throw new Error(`${name} symlink target does not match its hash and size`);
  return { path, nodeKind: "symlink", contentHash, mode, size, target };
}

function parseRegeneration(value: unknown, name: string): TaxonomyRegeneration {
  const row = planRecord(value, name, ["id", "contractId", "cwd", "command", "outputRoots", "inputs", "preOutputs", "outputs", "preview", "previewManifestDigest", "staleRemovals"], ["verifyCommand"]);
  const command = row.command;
  if (!Array.isArray(command) || command.length !== 4 || command[0] !== "bun" || command[1] !== "nx" || command[2] !== "run" || typeof command[3] !== "string") throw new Error(`${name}.command is invalid`);
  const verifyCommand = row.verifyCommand;
  if (verifyCommand !== undefined && (!Array.isArray(verifyCommand) || verifyCommand.length !== 4 || verifyCommand[0] !== "bun" || verifyCommand[1] !== "nx" || verifyCommand[2] !== "run" || typeof verifyCommand[3] !== "string")) throw new Error(`${name}.verifyCommand is invalid`);
  if (!["outputRoots", "inputs", "preOutputs", "outputs", "staleRemovals"].every((key) => Array.isArray(row[key]))) throw new Error(`${name} array fields are invalid`);
  const preview = planRecord(row.preview, `${name}.preview`, ["contractId", "nodes", "schemaVersion", "staleRemovals"]);
  if (preview.schemaVersion !== 1 || preview.contractId !== row.contractId || !Array.isArray(preview.nodes) || !Array.isArray(preview.staleRemovals)) throw new Error(`${name}.preview is invalid`);
  const previewNodes = preview.nodes.map((value, index) => {
    const node = planRecord(value, `${name}.preview.nodes[${index}]`, ["bytesBase64", "mode", "nodeKind", "path"]);
    if (node.nodeKind !== "directory" && node.nodeKind !== "file") throw new Error(`${name}.preview.nodes[${index}].nodeKind is invalid`);
    return { bytesBase64: planString(node.bytesBase64, `${name}.preview.nodes[${index}].bytesBase64`), mode: planInteger(node.mode, `${name}.preview.nodes[${index}].mode`, 0o7777), nodeKind: node.nodeKind, path: planPath(node.path, `${name}.preview.nodes[${index}].path`) };
  });
  const result: TaxonomyRegeneration = { id: planString(row.id, `${name}.id`, PLAN_OPERATION_ID), contractId: planString(row.contractId, `${name}.contractId`), cwd: planPath(row.cwd, `${name}.cwd`), command: command as unknown as TaxonomyRegeneration["command"], verifyCommand: verifyCommand as TaxonomyRegeneration["verifyCommand"], outputRoots: (row.outputRoots as unknown[]).map((entry, index) => planPath(entry, `${name}.outputRoots[${index}]`)), inputs: (row.inputs as unknown[]).map((entry, index) => parseGeneratorNodeRecord(entry, `${name}.inputs[${index}]`)), preOutputs: (row.preOutputs as unknown[]).map((entry, index) => parseGeneratorNodeRecord(entry, `${name}.preOutputs[${index}]`)), outputs: (row.outputs as unknown[]).map((entry, index) => parseGeneratorNodeRecord(entry, `${name}.outputs[${index}]`)), preview: { contractId: preview.contractId as string, nodes: previewNodes, schemaVersion: 1, staleRemovals: (preview.staleRemovals as unknown[]).map((entry, index) => planPath(entry, `${name}.preview.staleRemovals[${index}]`)) }, previewManifestDigest: planString(row.previewManifestDigest, `${name}.previewManifestDigest`, PLAN_HASH), staleRemovals: (row.staleRemovals as unknown[]).map((entry, index) => planPath(entry, `${name}.staleRemovals[${index}]`)) };
  const provisional = { contractId: result.contractId, cwd: result.cwd, command: result.command, verifyCommand: result.verifyCommand, outputRoots: result.outputRoots, inputs: result.inputs, preOutputs: result.preOutputs, outputs: result.outputs, preview: result.preview, previewManifestDigest: result.previewManifestDigest, staleRemovals: result.staleRemovals };
  if (result.id !== sha256(canonicalJson(provisional)).slice(0, 24)) throw new Error(`${name}.id does not match its fields`);
  return result;
}

function parseOpaqueDigest(value: unknown, name: string): OpaqueTreeDigest {
  const row = planRecord(value, name, ["algorithm", "relativeRoot", "digest", "files", "directories", "symlinks", "others"]);
  if (row.algorithm !== "sha256-merkle-v1") throw new Error(`${name}.algorithm is invalid`);
  return { algorithm: row.algorithm, relativeRoot: planPath(row.relativeRoot, `${name}.relativeRoot`), digest: planString(row.digest, `${name}.digest`, PLAN_HASH), files: planInteger(row.files, `${name}.files`), directories: planInteger(row.directories, `${name}.directories`), symlinks: planInteger(row.symlinks, `${name}.symlinks`), others: planInteger(row.others, `${name}.others`) };
}

function parsePlanViolation(value: unknown, name: string): TaxonomyViolation {
  const row = planRecord(value, name, ["code", "severity", "path", "message"]);
  if (row.severity !== "warning" && row.severity !== "error") throw new Error(`${name}.severity is invalid`);
  return { code: planString(row.code, `${name}.code`), severity: row.severity, path: planPath(row.path, `${name}.path`), message: planString(row.message, `${name}.message`) };
}

/** 🧿️ Strictly parses a complete schema-v2 taxonomy plan without defaults or v1 compatibility. */
export function parseTaxonomyPlan(value: unknown): TaxonomyPlan {
  const row = planRecord(value, "taxonomy plan", ["schemaVersion", "taxonomySchemaVersion", "baselineCommit", "sourceTreeDigest", "excludedTreeDigests", "moves", "embeddedTicketRoots", "embeddedTicketRootRelocations", "symlinkTargetEdits", "evidenceRemovals", "destinationAncestorPreimages", "edits", "regenerations", "unresolved", "expectedAffectedPreStateDigest", "expectedPostStateDigest", "planDigest"], ["scope"]);
  if (row.schemaVersion !== 2 || row.taxonomySchemaVersion !== 7) throw new Error("Taxonomy plan must use schemaVersion 2 and taxonomySchemaVersion 7");
  if (!["excludedTreeDigests", "moves", "embeddedTicketRoots", "embeddedTicketRootRelocations", "symlinkTargetEdits", "evidenceRemovals", "destinationAncestorPreimages", "edits", "regenerations", "unresolved"].every((key) => Array.isArray(row[key]))) throw new Error("Taxonomy plan operation and evidence fields must be arrays");
  const destinationAncestorPreimages = (row.destinationAncestorPreimages as unknown[]).map((value, index) => {
    const name = `taxonomy plan destinationAncestorPreimages[${index}]`;
    const entry = planRecord(value, name, ["path", "state"]);
    if (entry.state !== "absent" && entry.state !== "directory") throw new Error(`${name}.state is invalid`);
    return { path: planPath(entry.path, `${name}.path`), state: entry.state } as TaxonomyDestinationAncestorPreimage;
  });
  if (destinationAncestorPreimages.some((entry, index) => index > 0 && generatorPathCompare(destinationAncestorPreimages[index - 1].path, entry.path) >= 0)) throw new Error("Taxonomy plan destinationAncestorPreimages must be unique and bytewise sorted");
  const parseOperationId = (entry: JsonRecord, name: string): string => planString(entry.operationId, `${name}.operationId`, PLAN_OPERATION_ID);
  const embeddedTicketRootRelocations = (row.embeddedTicketRootRelocations as unknown[]).map((value, index) => {
    const name = `taxonomy plan embeddedTicketRootRelocations[${index}]`;
    const entry = planRecord(value, name, ["operationId", "embeddedTicketRootId", "sourcePath", "destinationPath", "relativeEvidencePath", "preimage", "ownerId", "rationaleRule"], ["fixedContractId"]);
    if (entry.rationaleRule !== "embedded-ticket-root-relocation-v1") throw new Error(`${name}.rationaleRule is invalid`);
    const operationId = parseOperationId(entry, name);
    const result: TaxonomyEmbeddedTicketRootRelocation = { operationId, embeddedTicketRootId: planString(entry.embeddedTicketRootId, `${name}.embeddedTicketRootId`, PLAN_OPERATION_ID), sourcePath: planPath(entry.sourcePath, `${name}.sourcePath`), destinationPath: planPath(entry.destinationPath, `${name}.destinationPath`), relativeEvidencePath: planPath(entry.relativeEvidencePath, `${name}.relativeEvidencePath`), preimage: parseLeafPreimage(entry.preimage, `${name}.preimage`), fixedContractId: entry.fixedContractId === undefined ? undefined : planString(entry.fixedContractId, `${name}.fixedContractId`), ownerId: planString(entry.ownerId, `${name}.ownerId`), rationaleRule: entry.rationaleRule };
    const { operationId: _id, ...digestible } = result;
    if (operationId !== dispositionOperationId("embedded-ticket-root-relocation", digestible)) throw new Error(`${name}.operationId does not match its fields`);
    return result;
  });
  const evidenceRemovals = (row.evidenceRemovals as unknown[]).map((value, index) => {
    const name = `taxonomy plan evidenceRemovals[${index}]`;
    const entry = planRecord(value, name, ["operationId", "sourcePath", "preimage", "authority", "rationaleRule", "ownerId"], ["embeddedTicketRootId"]);
    if (entry.rationaleRule !== "redundant-ticket-evidence-v1" && entry.rationaleRule !== "serialized-platform-sentinel-v1" && entry.rationaleRule !== "ticket-important-closed-empty-v1" && entry.rationaleRule !== "ticket-important-exact-empty-residue-v1" && entry.rationaleRule !== "exact-owner-generated-source-retirement-v1" && entry.rationaleRule !== "nested-cargo-generated-source-retirement-v1") throw new Error(`${name}.rationaleRule is invalid`);
    const operationId = parseOperationId(entry, name);
    const result: TaxonomyEvidenceRemoval = { operationId, sourcePath: planPath(entry.sourcePath, `${name}.sourcePath`), preimage: parseLeafPreimage(entry.preimage, `${name}.preimage`), authority: parseRemovalAuthority(entry.authority, `${name}.authority`), embeddedTicketRootId: entry.embeddedTicketRootId === undefined ? undefined : planString(entry.embeddedTicketRootId, `${name}.embeddedTicketRootId`, PLAN_OPERATION_ID), rationaleRule: entry.rationaleRule, ownerId: planString(entry.ownerId, `${name}.ownerId`) };
    if (result.authority.kind === "nested-cargo-generated-source") {
      if (result.rationaleRule !== "nested-cargo-generated-source-retirement-v1" || result.embeddedTicketRootId !== undefined || canonicalJson(result.preimage) !== canonicalJson(result.authority.sourcePreimage)) throw new Error(name + " nested Cargo generated source is not bound to its frozen preimage");
    } else if (result.authority.kind === "exact-owner-generated-source") {
      if (result.rationaleRule !== "exact-owner-generated-source-retirement-v1" || result.embeddedTicketRootId !== undefined || canonicalJson(result.preimage) !== canonicalJson(result.authority.outputPreimage)) throw new Error(name + " generated source retirement is not byte-and-mode-preserving");
    } else if (result.authority.kind === "byte-and-mode-identical") {
      const removals = result.authority.members.filter((member) => member.disposition === "remove" && member.sourcePath === result.sourcePath && canonicalJson(member.preimage) === canonicalJson(result.preimage));
      if (result.rationaleRule !== "redundant-ticket-evidence-v1" || removals.length !== 1) throw new Error(`${name} is not bound to exactly one redundant evidence member`);
    } else if (result.authority.kind === "owner-manifest-status") {
      if (result.rationaleRule !== "ticket-important-closed-empty-v1" || result.embeddedTicketRootId !== undefined || dirname(result.sourcePath) !== result.authority.ownerPath || result.authority.manifestPath !== `${result.authority.ownerPath}/🎫️ticket.json` || basename(result.sourcePath) !== "📌️important.md" || result.preimage.nodeKind !== "file" || result.preimage.size !== 0) throw new Error(`${name} ticket lifecycle authority is not bound to its exact owner and zero-byte source`);
    } else if (result.authority.kind === "exact-path-mutation") {
      if (result.rationaleRule !== "ticket-important-exact-empty-residue-v1" || result.embeddedTicketRootId !== undefined || result.sourcePath !== result.authority.sourcePath || canonicalJson(result.preimage) !== canonicalJson(result.authority.sourcePreimage)) throw new Error(`${name} exact path authority is not bound to its frozen removal`);
    } else if (result.rationaleRule !== "serialized-platform-sentinel-v1" || result.embeddedTicketRootId !== undefined) throw new Error(`${name} serialized sentinel authority has an invalid rationale or embedded-root binding`);
    const { operationId: _id, ...digestible } = result;
    if (operationId !== dispositionOperationId("evidence-removal", digestible)) throw new Error(`${name}.operationId does not match its fields`);
    return result;
  });
  const symlinkTargetEdits = (row.symlinkTargetEdits as unknown[]).map((value, index) => {
    const name = `taxonomy plan symlinkTargetEdits[${index}]`;
    const entry = planRecord(value, name, ["operationId", "sourcePath", "finalPath", "oldTarget", "newTarget", "oldTargetHash", "newTargetHash", "logicalTargetSourcePath", "logicalTargetFinalPath", "logicalTargetPreimage", "windowsLinkType", "sourceTargetDigest", "rationaleRule", "ownerId"]);
    if (entry.rationaleRule !== "repository-local-symlink-target-v2" || (entry.windowsLinkType !== "file" && entry.windowsLinkType !== "dir")) throw new Error(`${name} has invalid literals`);
    const operationId = parseOperationId(entry, name);
    const result: TaxonomySymlinkTargetEdit = { operationId, sourcePath: planPath(entry.sourcePath, `${name}.sourcePath`), finalPath: planPath(entry.finalPath, `${name}.finalPath`), oldTarget: planString(entry.oldTarget, `${name}.oldTarget`), newTarget: planString(entry.newTarget, `${name}.newTarget`), oldTargetHash: planString(entry.oldTargetHash, `${name}.oldTargetHash`, PLAN_HASH), newTargetHash: planString(entry.newTargetHash, `${name}.newTargetHash`, PLAN_HASH), logicalTargetSourcePath: planPath(entry.logicalTargetSourcePath, `${name}.logicalTargetSourcePath`), logicalTargetFinalPath: planPath(entry.logicalTargetFinalPath, `${name}.logicalTargetFinalPath`), logicalTargetPreimage: parsePathPreimage(entry.logicalTargetPreimage, `${name}.logicalTargetPreimage`), windowsLinkType: entry.windowsLinkType, sourceTargetDigest: planString(entry.sourceTargetDigest, `${name}.sourceTargetDigest`, PLAN_HASH), rationaleRule: entry.rationaleRule, ownerId: planString(entry.ownerId, `${name}.ownerId`) };
    const absoluteTarget = result.oldTarget.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(result.oldTarget) || /^(?:\\\\|\/\/)[^\\/]+[\\/][^\\/]+/u.test(result.oldTarget);
    if (result.oldTarget === "" || result.oldTarget.includes("\u0000") || result.newTarget === "" || result.newTarget.includes("\\") || result.newTarget.startsWith("/") || /^[A-Za-z]:/u.test(result.newTarget)) throw new Error(`${name} target syntax is invalid`);
    if (!absoluteTarget) {
      let resolvedOldTarget: string;
      try { resolvedOldTarget = normalizeRelative(posix.join(posix.dirname(result.sourcePath), result.oldTarget.replaceAll("\\", "/"))); } catch { throw new Error(`${name}.oldTarget does not resolve inside the repository`); }
      if (resolvedOldTarget !== result.logicalTargetSourcePath) throw new Error(`${name}.oldTarget does not resolve to logicalTargetSourcePath`);
    }
    const resolvedNewTarget = posix.normalize(posix.join(posix.dirname(result.finalPath), result.newTarget));
    if (resolvedNewTarget !== result.logicalTargetFinalPath) throw new Error(`${name}.newTarget does not resolve to logicalTargetFinalPath`);
    const targetDigestible = { sourcePath: result.sourcePath, finalPath: result.finalPath, oldTarget: result.oldTarget, newTarget: result.newTarget, logicalTargetSourcePath: result.logicalTargetSourcePath, logicalTargetFinalPath: result.logicalTargetFinalPath, logicalTargetPreimage: result.logicalTargetPreimage };
    if (result.oldTargetHash !== sha256(result.oldTarget) || result.newTargetHash !== sha256(result.newTarget) || result.sourceTargetDigest !== sha256(canonicalJson(targetDigestible))) throw new Error(`${name} target hashes do not match raw targets`);
    const { operationId: _id, ...digestible } = result;
    if (operationId !== dispositionOperationId("symlink-target-edit", digestible)) throw new Error(`${name}.operationId does not match its fields`);
    return result;
  });
  const embeddedTicketRoots = (row.embeddedTicketRoots as unknown[]).map((value, index) => {
    const name = `taxonomy plan embeddedTicketRoots[${index}]`;
    const entry = planRecord(value, name, ["operationId", "sourceMetadataRoot", "sourceTicketRoot", "canonicalTicketRoot", "ticketId", "sourceTreeDigest", "residualTreeDigest", "incomingReferenceDigest", "relocationOperationIds", "removalOperationIds", "rationaleRule"]);
    if (entry.rationaleRule !== "embedded-ticket-root-relocation-v1") throw new Error(`${name}.rationaleRule is invalid`);
    const operationId = parseOperationId(entry, name);
    const result: TaxonomyEmbeddedTicketRootDisposition = { operationId, sourceMetadataRoot: planPath(entry.sourceMetadataRoot, `${name}.sourceMetadataRoot`), sourceTicketRoot: planPath(entry.sourceTicketRoot, `${name}.sourceTicketRoot`), canonicalTicketRoot: planPath(entry.canonicalTicketRoot, `${name}.canonicalTicketRoot`), ticketId: planString(entry.ticketId, `${name}.ticketId`), sourceTreeDigest: parseNoFollowTreeDigest(entry.sourceTreeDigest, `${name}.sourceTreeDigest`), residualTreeDigest: parseNoFollowTreeDigest(entry.residualTreeDigest, `${name}.residualTreeDigest`), incomingReferenceDigest: planString(entry.incomingReferenceDigest, `${name}.incomingReferenceDigest`, PLAN_HASH), relocationOperationIds: planStringArray(entry.relocationOperationIds, `${name}.relocationOperationIds`, PLAN_OPERATION_ID), removalOperationIds: planStringArray(entry.removalOperationIds, `${name}.removalOperationIds`, PLAN_OPERATION_ID), rationaleRule: entry.rationaleRule };
    const { operationId: _id, relocationOperationIds: _relocations, removalOperationIds: _removals, ...digestible } = result;
    if (operationId !== dispositionOperationId("embedded-ticket-root", digestible)) throw new Error(`${name}.operationId does not match its fields`);
    return result;
  });
  const allOperationIds = [...(row.moves as JsonRecord[]).map((entry, index) => planString(entry.operationId, `taxonomy plan moves[${index}].operationId`, PLAN_OPERATION_ID)), ...embeddedTicketRoots.map((entry) => entry.operationId), ...embeddedTicketRootRelocations.map((entry) => entry.operationId), ...symlinkTargetEdits.map((entry) => entry.operationId), ...evidenceRemovals.map((entry) => entry.operationId), ...(row.regenerations as JsonRecord[]).map((entry, index) => planString(entry.id, `taxonomy plan regenerations[${index}].id`, PLAN_OPERATION_ID))];
  if (new Set(allOperationIds).size !== allOperationIds.length) throw new Error("Taxonomy plan operation IDs are not globally unique");
  const relocationIds = new Set(embeddedTicketRootRelocations.map((entry) => entry.operationId));
  const removalIds = new Set(evidenceRemovals.map((entry) => entry.operationId));
  const rootIds = new Set(embeddedTicketRoots.map((entry) => entry.operationId));
  if (embeddedTicketRootRelocations.some((entry) => !rootIds.has(entry.embeddedTicketRootId)) || evidenceRemovals.some((entry) => entry.embeddedTicketRootId !== undefined && !rootIds.has(entry.embeddedTicketRootId))) throw new Error("Embedded ticket disposition references an unknown root");
  for (const root of embeddedTicketRoots) {
    if (!root.sourceTicketRoot.startsWith(`${root.sourceMetadataRoot}/`)) throw new Error(`Embedded ticket root ${root.operationId} source ticket root escapes its metadata root`);
    const ticketSegments = root.canonicalTicketRoot.split("/").slice(-4);
    const expectedTicketId = `${splitLeadingEmoji(ticketSegments[0] ?? "").rest}/${splitLeadingEmoji(ticketSegments[1] ?? "").rest}/${splitLeadingEmoji(ticketSegments[2] ?? "").rest}/${ticketSegments[3] ?? ""}`;
    if (root.ticketId !== expectedTicketId || !/^[0-9]{2}\/[0-9]{2}\/[0-9]{2}\/.+/u.test(root.ticketId)) throw new Error(`Embedded ticket root ${root.operationId} ticketId does not match its canonical root`);
    const orderedChildren = [...root.relocationOperationIds, ...root.removalOperationIds];
    if ([root.relocationOperationIds, root.removalOperationIds].some((ids) => ids.some((id, index) => index > 0 && Buffer.from(ids[index - 1]).compare(Buffer.from(id)) >= 0))) throw new Error(`Embedded ticket root ${root.operationId} child IDs are not unique and bytewise sorted`);
    if (root.relocationOperationIds.some((id) => !relocationIds.has(id)) || root.removalOperationIds.some((id) => !removalIds.has(id))) throw new Error(`Embedded ticket root ${root.operationId} has dangling disposition IDs`);
    const actualChildren = [...embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId), ...evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId)];
    if (new Set(orderedChildren).size !== orderedChildren.length || canonicalJson(orderedChildren.sort(generatorPathCompare)) !== canonicalJson(actualChildren.sort(generatorPathCompare))) throw new Error(`Embedded ticket root ${root.operationId} does not exhaust its child dispositions`);
    if (actualChildren.length !== root.sourceTreeDigest.files + root.sourceTreeDigest.symlinks || root.sourceTreeDigest.others !== 0 || root.residualTreeDigest.files !== 0 || root.residualTreeDigest.symlinks !== 0 || root.residualTreeDigest.others !== 0) throw new Error(`Embedded ticket root ${root.operationId} tree closure does not equal its child dispositions`);
    for (const relocation of embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId)) if (!relocation.sourcePath.startsWith(`${root.sourceTicketRoot}/`) || relocation.destinationPath !== `${root.canonicalTicketRoot}/${relocation.relativeEvidencePath}` || relocation.relativeEvidencePath !== relocation.sourcePath.slice(root.sourceTicketRoot.length + 1)) throw new Error(`Embedded ticket root ${root.operationId} relocation escapes its frozen roots`);
    for (const removal of evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId)) if (!removal.sourcePath.startsWith(`${root.sourceTicketRoot}/`)) throw new Error(`Embedded ticket root ${root.operationId} removal escapes its source ticket root`);
  }
  if (new Set(embeddedTicketRoots.map((entry) => entry.sourceMetadataRoot)).size !== embeddedTicketRoots.length || new Set(embeddedTicketRoots.map((entry) => entry.sourceTicketRoot)).size !== embeddedTicketRoots.length) throw new Error("Embedded ticket root source roots are not unique");
  const excludedTreeDigests = (row.excludedTreeDigests as unknown[]).map((entry, index) => parseOpaqueDigest(entry, `taxonomy plan excludedTreeDigests[${index}]`));
  const moves = (row.moves as unknown[]).map((entry, index) => parseMove(entry, `taxonomy plan moves[${index}]`));
  for (const edit of symlinkTargetEdits) {
    const owningMoves = moves.filter((move) => move.sourcePath === edit.sourcePath && move.destinationPath === edit.finalPath);
    if (edit.sourcePath !== edit.finalPath && owningMoves.length !== 1) throw new Error(`Symlink target edit ${edit.operationId} finalPath is not its exact move destination`);
  }
  const edits = (row.edits as unknown[]).map((entry, index) => parseReferenceEdit(entry, `taxonomy plan edits[${index}]`));
  const regenerations = (row.regenerations as unknown[]).map((entry, index) => parseRegeneration(entry, `taxonomy plan regenerations[${index}]`));
  const unresolved = (row.unresolved as unknown[]).map((entry, index) => parsePlanViolation(entry, `taxonomy plan unresolved[${index}]`));
  const result: TaxonomyPlan = { schemaVersion: 2, taxonomySchemaVersion: 7, baselineCommit: planString(row.baselineCommit, "taxonomy plan baselineCommit", PLAN_COMMIT_ID), scope: row.scope === undefined ? undefined : planPath(row.scope, "taxonomy plan scope"), sourceTreeDigest: planString(row.sourceTreeDigest, "taxonomy plan sourceTreeDigest", PLAN_HASH), excludedTreeDigests, moves, embeddedTicketRoots, embeddedTicketRootRelocations, symlinkTargetEdits, evidenceRemovals, destinationAncestorPreimages, edits, regenerations, unresolved, expectedAffectedPreStateDigest: planString(row.expectedAffectedPreStateDigest, "taxonomy plan expectedAffectedPreStateDigest", PLAN_HASH), expectedPostStateDigest: planString(row.expectedPostStateDigest, "taxonomy plan expectedPostStateDigest", PLAN_HASH), planDigest: planString(row.planDigest, "taxonomy plan planDigest", PLAN_HASH) };
  const requiredAncestorPaths = new Set<string>();
  for (const destination of [...moves.map((entry) => entry.destinationPath), ...embeddedTicketRootRelocations.map((entry) => entry.destinationPath), ...symlinkTargetEdits.map((entry) => entry.finalPath), ...regenerations.flatMap((entry) => entry.outputRoots)]) for (let path = posix.dirname(destination); path !== "." && path !== ""; path = posix.dirname(path)) requiredAncestorPaths.add(path);
  if (canonicalJson(destinationAncestorPreimages.map((entry) => entry.path)) !== canonicalJson([...requiredAncestorPaths].sort(generatorPathCompare))) throw new Error("Taxonomy plan destinationAncestorPreimages do not exhaust mutation destination parents");
  planString(result.sourceTreeDigest, "taxonomy plan sourceTreeDigest", PLAN_HASH);
  planString(result.expectedAffectedPreStateDigest, "taxonomy plan expectedAffectedPreStateDigest", PLAN_HASH);
  planString(result.expectedPostStateDigest, "taxonomy plan expectedPostStateDigest", PLAN_HASH);
  planString(result.planDigest, "taxonomy plan planDigest", PLAN_HASH);
  if (result.scope !== undefined) planPath(result.scope, "taxonomy plan scope");
  if (taxonomyPlanDigest(result) !== result.planDigest) throw new Error("Taxonomy plan digest does not match canonical plan bytes");
  return result;
}

function generatorPathCompare(left: string, right: string): number {
  return Buffer.from(left).compare(Buffer.from(right));
}

function generatorPreviewJson(manifest: TaxonomyGeneratorPreviewManifest): string {
  return JSON.stringify({
    contractId: manifest.contractId,
    nodes: manifest.nodes.map((node) => ({ bytesBase64: node.bytesBase64, mode: node.mode, nodeKind: node.nodeKind, path: node.path })),
    schemaVersion: manifest.schemaVersion,
    staleRemovals: manifest.staleRemovals,
  });
}

/** 🛰️ Parses one byte-canonical, repository-rooted, JSON-only generator preview manifest. */
export function parseGeneratorPreviewManifest(content: string, expectedContractId: string, outputRoots: readonly string[], excludedRoots: readonly string[] = []): TaxonomyGeneratorPreviewManifest {
  let value: unknown;
  try {
    value = JSON.parse(content);
  } catch {
    throw new Error(`Generator preview stdout is not one canonical JSON document: bytes=${Buffer.byteLength(content)}, sha256=${sha256(content)}`);
  }
  const root = record(value, "generator preview");
  if (Object.keys(root).join("\u0000") !== "contractId\u0000nodes\u0000schemaVersion\u0000staleRemovals") throw new Error("Generator preview has noncanonical top-level keys or order");
  if (root.schemaVersion !== 1) throw new Error("Generator preview schemaVersion must be 1");
  if (root.contractId !== expectedContractId) throw new Error(`Generator preview contractId does not match ${expectedContractId}`);
  if (!Array.isArray(root.nodes) || !Array.isArray(root.staleRemovals)) throw new Error("Generator preview nodes and staleRemovals must be arrays");
  const roots = [...new Set(outputRoots.map((path) => normalizeRelative(path)))].sort(generatorPathCompare);
  if (roots.length !== outputRoots.length || roots.some((path, index) => path !== outputRoots[index])) throw new Error("Generator preview output roots must be unique, NFC, repository-relative, and byte-sorted");
  const exclusions = excludedRoots.map(normalizeRelative);
  const withinRoot = (path: string): boolean => roots.some((candidate) => path === candidate || path.startsWith(`${candidate}/`));
  const excluded = (path: string): boolean => exclusions.some((candidate) => path === candidate || path.startsWith(`${candidate}/`));
  const nodes: TaxonomyGeneratorPreviewNode[] = root.nodes.map((value, index) => {
    const node = record(value, `generator preview nodes[${index}]`);
    if (Object.keys(node).join("\u0000") !== "bytesBase64\u0000mode\u0000nodeKind\u0000path") throw new Error(`Generator preview node ${index} has noncanonical keys or order`);
    const path = requiredString(node.path, `generator preview nodes[${index}].path`);
    if (path !== normalizeRelative(path) || path !== path.normalize("NFC") || !withinRoot(path) || excluded(path)) throw new Error(`Generator preview node path is unsafe or outside registered roots: ${path}`);
    if (node.nodeKind !== "directory" && node.nodeKind !== "file") throw new Error(`Generator preview nodeKind is invalid at ${path}`);
    if (!Number.isSafeInteger(node.mode) || (node.mode as number) < 0 || (node.mode as number) > 0o7777) throw new Error(`Generator preview mode is invalid at ${path}`);
    if (typeof node.bytesBase64 !== "string" || !/^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/u.test(node.bytesBase64)) throw new Error(`Generator preview base64 is invalid at ${path}`);
    const decoded = Buffer.from(node.bytesBase64, "base64");
    if (decoded.toString("base64") !== node.bytesBase64 || (node.nodeKind === "directory" && node.bytesBase64 !== "")) throw new Error(`Generator preview base64 is noncanonical at ${path}`);
    return { bytesBase64: node.bytesBase64, mode: node.mode as number, nodeKind: node.nodeKind, path };
  });
  const nodeByPath = new Map<string, TaxonomyGeneratorPreviewNode>();
  for (let index = 0; index < nodes.length; index++) {
    const node = nodes[index];
    if (nodeByPath.has(node.path) || (index > 0 && generatorPathCompare(nodes[index - 1].path, node.path) >= 0)) throw new Error(`Generator preview nodes repeat or are not byte-sorted at ${node.path}`);
    nodeByPath.set(node.path, node);
  }
  for (const registeredRoot of roots) if (!nodeByPath.has(registeredRoot)) throw new Error(`Generator preview omits registered output root ${registeredRoot}`);
  for (const node of nodes) {
    let parent = posix.dirname(node.path);
    const registeredRoot = roots.filter((candidate) => node.path === candidate || node.path.startsWith(`${candidate}/`)).sort((left, right) => right.length - left.length)[0];
    while (registeredRoot && parent !== posix.dirname(registeredRoot)) {
      const parentNode = nodeByPath.get(parent);
      if (!parentNode || parentNode.nodeKind !== "directory") throw new Error(`Generator preview omits directory node ${parent}`);
      if (parent === registeredRoot) break;
      parent = posix.dirname(parent);
    }
    if (node.nodeKind === "file" && nodes.some((candidate) => candidate.path.startsWith(`${node.path}/`))) throw new Error(`Generator preview file has descendants at ${node.path}`);
  }
  const staleRemovals = root.staleRemovals.map((value, index) => {
    const path = requiredString(value, `generator preview staleRemovals[${index}]`);
    if (path !== normalizeRelative(path) || path !== path.normalize("NFC") || !withinRoot(path) || excluded(path)) throw new Error(`Generator preview stale removal is unsafe or outside registered roots: ${path}`);
    if (nodeByPath.has(path) || nodes.some((node) => node.path.startsWith(`${path}/`))) throw new Error(`Generator preview stale removal overlaps expected output ${path}`);
    return path;
  });
  for (let index = 0; index < staleRemovals.length; index++) if ((index > 0 && generatorPathCompare(staleRemovals[index - 1], staleRemovals[index]) >= 0) || staleRemovals.some((path, candidate) => candidate !== index && path.startsWith(`${staleRemovals[index]}/`))) throw new Error(`Generator preview stale removals repeat, overlap, or are not byte-sorted at ${staleRemovals[index]}`);
  const manifest: TaxonomyGeneratorPreviewManifest = { contractId: expectedContractId, nodes, schemaVersion: 1, staleRemovals };
  if (content !== `${generatorPreviewJson(manifest)}\n`) throw new Error("Generator preview stdout is noisy or not byte-canonical JSON");
  return manifest;
}

function normalizeRelative(value: string): string {
  return sourceRelative(value).normalize("NFC");
}

function sourceRelative(value: string): string {
  const slash = value.replaceAll("\\", "/").replace(/^\.\//, "");
  const normalized = posix.normalize(slash);
  if (normalized === ".") return "";
  if (normalized === ".." || normalized.startsWith("../") || normalized.startsWith("/") || normalized.includes("\u0000")) throw new Error(`Path escapes repository scope: ${value}`);
  return normalized.replace(/\/$/, "");
}

function absolutePath(repoRoot: string, path: string): string {
  const root = resolve(repoRoot);
  const result = resolve(root, ...sourceRelative(path).split("/").filter(Boolean));
  const rel = relative(root, result);
  if (rel === ".." || rel.startsWith(`..${sep}`) || rel.startsWith("../") || rel.startsWith("..\\") || isAbsolute(rel)) throw new Error(`Path escapes repository root: ${path}`);
  return result;
}

function assertNoFollowAncestors(repoRoot: string, target: string, label: string, rejectLeafSymlink = false): void {
  const root = resolve(repoRoot);
  const relativeTarget = relative(root, target);
  const segments = relativeTarget.split(sep).filter(Boolean);
  let current = root;
  const end = segments.length - (rejectLeafSymlink ? 0 : 1);
  for (let index = 0; index < end; index++) {
    current = join(current, segments[index]);
    const stat = lstatOrNull(current);
    const leaf = rejectLeafSymlink && index === segments.length - 1;
    if (stat?.isSymbolicLink() || (!leaf && stat && !stat.isDirectory())) throw new Error(`${label} has a non-directory or symlink ancestor: ${segments.slice(0, index + 1).join("/")}`);
  }
}

function assertLexicalInputOutsideOpaque(repoRoot: string, path: string, label: string, rejectLeafSymlink = false): string {
  const root = resolve(repoRoot);
  const target = isAbsolute(path) ? resolve(path) : resolve(root, path);
  const nativeRelative = relative(root, target);
  if (nativeRelative === ".." || nativeRelative.startsWith(`..${sep}`) || nativeRelative.startsWith("../") || nativeRelative.startsWith("..\\") || isAbsolute(nativeRelative)) throw new Error(`${label} must be repository-local`);
  const repositoryRelative = posix.normalize(nativeRelative.replaceAll("\\", "/"));
  if (LEXICAL_OPAQUE_ROOTS.some((opaque) => repositoryRelative === opaque || repositoryRelative.startsWith(`${opaque}/`))) throw new Error(`${label} is inside an opaque path: ${repositoryRelative}`);
  assertNoFollowAncestors(root, target, label, rejectLeafSymlink);
  return target;
}

function isExcluded(path: string, taxonomy: LoadedTaxonomy): boolean {
  const normalized = normalizeRelative(path);
  return taxonomy.exclusions.some((entry) => normalized === entry.path || normalized.startsWith(`${entry.path}/`));
}

function inScope(path: string, scope?: string): boolean {
  if (!scope) return true;
  const normalizedScope = normalizeRelative(scope);
  const normalizedPath = normalizeRelative(path);
  return normalizedPath === normalizedScope || normalizedPath.startsWith(`${normalizedScope}/`) || normalizedScope.startsWith(`${normalizedPath}/`);
}

function isProperScopeAncestor(path: string, scope?: string): boolean {
  if (!scope) return false;
  const normalizedScope = normalizeRelative(scope);
  const normalizedPath = normalizeRelative(path);
  return normalizedPath !== normalizedScope && normalizedScope.startsWith(`${normalizedPath}/`);
}

function emojiFold(value: string): string {
  return value.normalize("NFC").replaceAll("\uFE0F", "");
}

function graphemes(value: string): readonly string[] {
  return [...SEGMENTER.segment(value)].map((entry) => entry.segment);
}

function isEmojiGrapheme(value: string): boolean {
  return /[\p{Extended_Pictographic}\p{Emoji_Presentation}\uFE0F\u20E3]/u.test(value);
}

function splitLeadingEmoji(value: string): { emoji: string; rest: string } {
  const first = SEGMENTER.segment(value)[Symbol.iterator]().next().value?.segment;
  if (!first || !isEmojiGrapheme(first)) return { emoji: "", rest: value };
  return { emoji: first, rest: value.slice(first.length) };
}

function splitLeadingEmojiIdentity(value: string): { sequence: string; first: string; rest: string } {
  let sequence = "", first = "";
  for (const { segment } of SEGMENTER.segment(value.normalize("NFC"))) {
    if (!isEmojiGrapheme(segment)) break;
    if (!first) first = segment;
    sequence += segment;
  }
  return { sequence, first, rest: value.slice(sequence.length) };
}

function matchDirectoryKind(name: string, taxonomy: LoadedTaxonomy, parentKindId?: string, ancestorKindIds: readonly string[] = []): { kind: { readonly id: string; readonly emoji: string } | null; slug: string; ambiguous: readonly string[] } {
  const normalized = name.normalize("NFC");
  const identity = splitLeadingEmojiIdentity(normalized);
  const leading = { emoji: identity.first, rest: identity.rest };
  const contextAllows = (kind: LoadedTaxonomy["directoryKinds"][number]): boolean => (kind.parentKindIds?.length ?? 0) === 0 || (parentKindId !== undefined && kind.parentKindIds?.includes(parentKindId) === true);
  if (leading.emoji) {
    const global = taxonomy.directoryKinds.filter((kind) => emojiFold(kind.emoji) === emojiFold(leading.emoji) && ((leading.rest.length === 0 && kind.allowEmojiOnly) || kind.slugRegex.test(leading.rest)));
    const exact = global.filter((kind) => contextAllows(kind) && kind.id.normalize("NFC").toLocaleLowerCase("und") === leading.rest.toLocaleLowerCase("und"));
    if (exact.length === 1) return { kind: exact[0], slug: leading.rest, ambiguous: [] };
    if (exact.length > 1) return { kind: null, slug: leading.rest, ambiguous: exact.map((entry) => entry.id) };
    const contextual = parentKindId === undefined ? [] : global.filter((kind) => kind.parentKindIds?.includes(parentKindId) === true);
    const ordinary = contextual.length > 0 ? contextual : global.filter((kind) => (kind.parentKindIds?.length ?? 0) === 0);
    if (ordinary.length === 1) return { kind: ordinary[0], slug: leading.rest, ambiguous: [] };
    const contexts = [parentKindId, ...ancestorKindIds].filter((kindId, index, rows): kindId is string => Boolean(kindId) && rows.indexOf(kindId) === index);
    const overlays = Object.entries(taxonomy.schema.semanticDirectoryMemberKinds)
      .filter(([, spec]) => spec.memberNames.some((memberName) => emojiFold(memberName) === emojiFold(`${identity.first}${identity.rest}`)))
      .map(([id, spec]) => ({ id, distance: contexts.findIndex((kindId) => spec.ownerKindIds.includes(kindId)) }))
      .filter((entry) => entry.distance >= 0)
      .sort((left, right) => left.distance - right.distance || left.id.localeCompare(right.id));
    if (overlays.length > 0) {
      const nearest = overlays.filter((entry) => entry.distance === overlays[0].distance);
      if (nearest.length === 1) return { kind: { id: nearest[0].id, emoji: leading.emoji }, slug: leading.rest, ambiguous: [] };
      return { kind: null, slug: leading.rest, ambiguous: nearest.map((entry) => entry.id) };
    }
    return { kind: null, slug: leading.rest, ambiguous: ordinary.map((entry) => entry.id) };
  }
  const exact = taxonomy.directoryKinds.filter((kind) => contextAllows(kind) && kind.inferWithoutEmoji !== false && kind.id.normalize("NFC").toLocaleLowerCase("und") === normalized.toLocaleLowerCase("und"));
  if (exact.length === 1) return { kind: exact[0], slug: normalized, ambiguous: [] };
  if (exact.length > 1) return { kind: null, slug: normalized, ambiguous: exact.map((entry) => entry.id) };
  const matching = taxonomy.directoryKinds.filter((kind) => kind.inferWithoutEmoji !== false && kind.slugRegex.test(normalized));
  const contextual = parentKindId === undefined ? [] : matching.filter((kind) => kind.parentKindIds?.includes(parentKindId) === true);
  const matches = contextual.length > 0 ? contextual : matching.filter((kind) => (kind.parentKindIds?.length ?? 0) === 0);
  return { kind: matches.length === 1 ? matches[0] : null, slug: normalized, ambiguous: matches.map((entry) => entry.id) };
}

function resolveFileKind(
  path: string,
  taxonomy: LoadedTaxonomy,
  parentKindId: string | undefined,
  ancestorKindIds: readonly string[],
  forcedId?: string,
  contentKindId?: string,
): { kind: (FileKindSpec & { readonly id: string }) | null; extension: string; stem: string; ambiguous: readonly string[] } {
  const name = basename(path);
  const normalized = name.normalize("NFC");
  const folded = normalized.toLocaleLowerCase("und");
  const scoped = Object.entries(taxonomy.schema.scopedFileKinds)
    .flatMap(([id, spec]) => {
      if (!taxonomy.pathMatcher.matches(path, spec.pathPattern) || !new RegExp(spec.sourceFilenamePattern, "u").test(normalized)) return [];
      const extensions = spec.extensionChains.filter((chain) => folded.endsWith(chain.toLocaleLowerCase("und"))).sort((left, right) => right.length - left.length || left.localeCompare(right));
      return extensions.length > 0 ? [{ id, spec, extension: extensions[0] }] : [];
    })
    .sort((left, right) => left.id.localeCompare(right.id));
  if (scoped.length > 1) return { kind: null, extension: "", stem: normalized, ambiguous: scoped.map(({ id }) => `scoped:${id}`) };
  if (scoped.length === 1) {
    const selected = scoped[0];
    const kind = { id: `scoped:${selected.id}`, emoji: selected.spec.emoji, extensionChains: selected.spec.extensionChains, role: selected.spec.role };
    const withoutExtension = normalized.slice(0, -selected.extension.length);
    const leading = splitLeadingEmoji(withoutExtension);
    return { kind, extension: selected.extension, stem: leading.emoji && emojiFold(leading.emoji) === emojiFold(kind.emoji) ? leading.rest : withoutExtension, ambiguous: [] };
  }
  const forced = forcedId ? taxonomy.fileKinds.find((kind) => kind.id === forcedId) : undefined;
  if (forced) {
    const extensions = forced.extensionChains.filter((chain) => normalized.endsWith(chain)).sort((left, right) => right.length - left.length || left.localeCompare(right));
    if (extensions.length > 0) {
      const extension = extensions[0];
      const withoutExtension = normalized.slice(0, -extension.length);
      const leading = splitLeadingEmoji(withoutExtension);
      return { kind: forced, extension, stem: leading.emoji && emojiFold(leading.emoji) === emojiFold(forced.emoji) ? leading.rest : withoutExtension, ambiguous: [] };
    }
  }
  const extensionRows = Object.entries(taxonomy.schema.fileKindResolutionRules)
    .filter(([, rule]) => normalized.endsWith(rule.extensionChain))
    .sort((left, right) => right[1].extensionChain.length - left[1].extensionChain.length || left[0].localeCompare(right[0]));
  const longest = extensionRows[0]?.[1].extensionChain.length ?? 0;
  const candidates = extensionRows
    .filter(([, rule]) => rule.extensionChain.length === longest)
    .filter(([, rule]) => !rule.filenamePattern || new RegExp(rule.filenamePattern, "u").test(normalized))
    .filter(([, rule]) => !rule.pathPattern || taxonomy.pathMatcher.matches(path, rule.pathPattern))
    .filter(([, rule]) => !rule.parentKindIds || (parentKindId !== undefined && rule.parentKindIds.includes(parentKindId)))
    .filter(([, rule]) => !rule.ancestorKindIds || rule.ancestorKindIds.some((kindId) => ancestorKindIds.includes(kindId)))
    .map(([id, rule]) => ({ id, rule, predicates: Number(Boolean(rule.filenamePattern)) + Number(Boolean(rule.pathPattern)) + Number(Boolean(rule.parentKindIds)) + Number(Boolean(rule.ancestorKindIds)) }))
    .sort((left, right) => right.rule.priority - left.rule.priority || right.predicates - left.predicates || left.id.localeCompare(right.id));
  if (candidates.length === 0) {
    const contentKind = contentKindId ? taxonomy.fileKinds.find((kind) => kind.id === contentKindId) : undefined;
    if (!contentKind) return { kind: null, extension: "", stem: normalized, ambiguous: [] };
    const extension = [...contentKind.extensionChains].sort((left, right) => left.length - right.length || left.localeCompare(right))[0];
    const leading = splitLeadingEmoji(normalized);
    const stem = (leading.emoji && emojiFold(leading.emoji) === emojiFold(contentKind.emoji) ? leading.rest : normalized).trim().replace(/[. ]+$/u, "");
    return { kind: contentKind, extension, stem, ambiguous: [] };
  }
  const top = candidates.filter((entry) => entry.rule.priority === candidates[0].rule.priority && entry.predicates === candidates[0].predicates);
  const kindIds = [...new Set(top.map((entry) => entry.rule.fileKindId))];
  const extension = top[0].rule.extensionChain;
  const withoutExtension = normalized.slice(0, normalized.length - extension.length);
  if (kindIds.length !== 1) return { kind: null, extension, stem: withoutExtension, ambiguous: top.map((entry) => `${entry.id}:${entry.rule.fileKindId}`) };
  const selected = taxonomy.fileKinds.find((kind) => kind.id === kindIds[0]);
  if (!selected) return { kind: null, extension, stem: withoutExtension, ambiguous: kindIds };
  const leading = splitLeadingEmoji(withoutExtension);
  const stem = leading.emoji && emojiFold(leading.emoji) === emojiFold(selected.emoji) ? leading.rest : withoutExtension;
  return { kind: selected, extension, stem, ambiguous: [] };
}

interface ContentKindHint {
  readonly kindId: string | null;
  readonly violation?: TaxonomyViolation;
}

function shebangCommand(line: string): string | null {
  const raw = line.startsWith("#!") ? line.slice(2).trim() : "";
  if (!raw) return null;
  const tokens = raw.split(/\s+/u).filter(Boolean);
  let command = tokens.shift() ?? "";
  if (basename(command).toLocaleLowerCase("und") === "env") {
    while (tokens[0]?.startsWith("-") || /^[A-Za-z_][A-Za-z0-9_]*=/u.test(tokens[0] ?? "")) tokens.shift();
    command = tokens.shift() ?? "";
  }
  return command ? basename(command).replace(/\.exe$/iu, "").toLocaleLowerCase("und") : null;
}

function typescriptSyntax(text: string): boolean {
  return /\b(?:interface|namespace|enum)\s+[A-Za-z_$]|\btype\s+[A-Za-z_$][\w$]*\s*=|\b(?:const|let|var)\s+[A-Za-z_$][\w$]*\s*:\s*[^=]|\b(?:satisfies|as\s+const)\b/u.test(text);
}

function extensionlessContentKind(path: string, bytes: Uint8Array | undefined, taxonomy: LoadedTaxonomy): ContentKindHint {
  const name = basename(path);
  if (name.includes(".") || !bytes) return { kindId: null };
  if (bytes.includes(0)) return { kindId: "binary" };
  let text: string;
  try {
    text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    return { kindId: "binary" };
  }
  if (/[\u0001-\u0008\u000B\u000C\u000E-\u001F]/u.test(text)) return { kindId: null, violation: violation("content-kind-ambiguous", path, "Extensionless content contains non-text control bytes without a binary signature") };
  if (text.startsWith("#!")) {
    const command = shebangCommand(text.split(/\r?\n/u, 1)[0] ?? "");
    const kindId = command && /^(?:ba|da|z|k|fi)?sh$/u.test(command) ? "shell"
      : command && /^python(?:\d+(?:\.\d+)*)?$/u.test(command) ? "python-source"
        : command && /^(?:pwsh|powershell)$/u.test(command) ? "powershell"
          : command && /^(?:node|nodejs)$/u.test(command) ? (typescriptSyntax(text) ? "typescript-source" : "javascript-source")
            : command && /^(?:bun|deno|tsx|ts-node)$/u.test(command) ? (typescriptSyntax(text) ? "typescript-source" : "javascript-source")
              : null;
    if (!kindId) return { kindId: null, violation: violation("shebang-kind-unresolved", path, `Extensionless shebang interpreter is unknown or contradictory: ${command ?? "missing"}`) };
    if (!taxonomy.schema.fileKinds[kindId]) return { kindId: null, violation: violation("shebang-kind-unregistered", path, `Shebang resolved to unregistered file kind ${kindId}`) };
    return { kindId };
  }
  if (!taxonomy.schema.fileKinds["plain-text"]) return { kindId: null, violation: violation("text-kind-unregistered", path, "Extensionless UTF-8 content requires registered plain-text kind") };
  return { kindId: "plain-text" };
}

function ownerId(path: string): string {
  const parts = path.split("/");
  if (parts[0] === ".🧬semio" && parts[1] === "🦑️repo" && parts[2] === "🎫️tickets" && parts.length >= 7) return parts.slice(0, 7).join("/");
  if (parts[0] === "✏️s" && (parts[1] === "🔌️plugins" || parts[1] === "🔨️modules") && parts[2]) return parts.slice(0, 3).join("/");
  if (parts[0] === "🧰️framework" && (parts[1] === "🛍️products" || parts[1] === "🔨️modules") && parts[2]) return parts.slice(0, 3).join("/");
  if ((parts[0] === "🌎️hub" || parts[0] === "♻️mit-bestand") && parts[1]) return parts.slice(0, 2).join("/");
  return parts[0] ?? "";
}

function areaId(path: string): string {
  const first = path.split("/")[0] ?? "";
  if (first === "✏️s") return path.split("/").slice(0, 2).join("/");
  return first;
}

function violation(code: string, path: string, message: string, severity: TaxonomySeverity = "error"): TaxonomyViolation {
  return { code, severity, path, message };
}

function stableViolations(rows: readonly TaxonomyViolation[]): readonly TaxonomyViolation[] {
  return [...new Map(rows.map((entry) => [`${entry.path}\u0000${entry.code}\u0000${entry.severity}\u0000${entry.message}`, entry])).values()].sort((a, b) => a.path.localeCompare(b.path) || a.code.localeCompare(b.code) || a.message.localeCompare(b.message));
}

function report(progress: TaxonomyInventoryOptions["progress"] | TaxonomyPlanOptions["progress"] | TaxonomyApplyOptions["progress"], operation: TaxonomyProgress["operation"], phase: string, current: number, total: number, path?: string): void {
  progress?.({ operation, phase, current, total, path });
}

class TaxonomyCancellationError extends Error {
  constructor() {
    super("Taxonomy operation cancelled");
  }
}

function checkCancellation(repoRoot: string, cancelFile?: string): void {
  if (!cancelFile) return;
  const path = assertLexicalInputOutsideOpaque(repoRoot, cancelFile, "cancelFile", true);
  if (existsSync(path)) throw new TaxonomyCancellationError();
}

function cancellationRequested(repoRoot: string, cancelFile?: string): boolean {
  if (!cancelFile) return false;
  return existsSync(assertLexicalInputOutsideOpaque(repoRoot, cancelFile, "cancelFile", true));
}
//#endregion 🧮️Canonicalization

//#region 📚️Inventory
//#region 🧾️Source Admission
const SOURCE_ADMISSION_ORIGINS: readonly TaxonomySourceOrigin[] = ["tracked", "nonignored-untracked", "ignored-generator", "explicit-ticket"];
const sourceAdmissionByteCompare = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));

function sourceAdmissionSafePath(path: string): boolean {
  return path.length > 0 && !path.startsWith("/") && !/^[A-Za-z]:/u.test(path) && !path.includes("\\") && !/[\u0000-\u001f\u007f]/u.test(path) && Buffer.from(path).toString("utf8") === path && path.split("/").every((part) => part.length > 0 && part !== "." && part !== "..");
}

function sourceAdmissionOpaque(path: string, prefixes: readonly string[]): boolean {
  const normalized = path.normalize("NFC");
  return prefixes.some((prefix) => {
    const expected = prefix.normalize("NFC");
    return expected === "compose" ? normalized.split("/").some((part) => part.toLowerCase() === "compose") : normalized === expected || normalized.startsWith(expected + "/");
  });
}

function sourceAdmissionRecord(value: unknown, required: readonly string[], optional: readonly string[] = []): value is Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value) && required.every((key) => Object.hasOwn(value, key)) && Object.keys(value).every((key) => required.includes(key) || optional.includes(key));
}

function sourceAdmissionInputShape(value: unknown): value is TaxonomySourceAdmissionInput {
  if (!sourceAdmissionRecord(value, ["scope", "opaquePrefixes", "generatorOutputRoots", "candidates"], ["cancelledDuring"])) return false;
  if (value.scope !== null && typeof value.scope !== "string") return false;
  if (value.cancelledDuring !== undefined && value.cancelledDuring !== null && typeof value.cancelledDuring !== "string") return false;
  if (!Array.isArray(value.opaquePrefixes) || value.opaquePrefixes.some((prefix) => typeof prefix !== "string" || prefix.length === 0) || new Set(value.opaquePrefixes).size !== value.opaquePrefixes.length) return false;
  if (!Array.isArray(value.generatorOutputRoots) || value.generatorOutputRoots.some((root) => !sourceAdmissionRecord(root, ["contractId", "rootPath", "inclusion"]) || typeof root.contractId !== "string" || !root.contractId || typeof root.rootPath !== "string" || !root.rootPath || (root.inclusion !== "tracked" && root.inclusion !== "ignored"))) return false;
  if (!Array.isArray(value.candidates)) return false;
  return value.candidates.every((row) => {
    if (!sourceAdmissionRecord(row, ["sourcePath", "observedKind", "worktreeMode", "explicitDirectory", "origins", "indexEntries", "unsafeAncestor"])) return false;
    if (typeof row.sourcePath !== "string" || !row.sourcePath || !["file", "directory", "symlink", "absent", "unobserved", "other"].includes(row.observedKind as string) || ![null, "100644", "100755", "120000", "160000", "040000"].includes(row.worktreeMode as string | null) || typeof row.explicitDirectory !== "boolean" || typeof row.unsafeAncestor !== "boolean") return false;
    if (!Array.isArray(row.origins) || row.origins.some((origin) => !SOURCE_ADMISSION_ORIGINS.includes(origin)) || new Set(row.origins).size !== row.origins.length || !Array.isArray(row.indexEntries)) return false;
    return row.indexEntries.every((entry) => sourceAdmissionRecord(entry, ["stage", "mode", "objectId"]) && Number.isInteger(entry.stage) && Number(entry.stage) >= 0 && Number(entry.stage) <= 3 && ["100644", "100755", "120000", "160000"].includes(entry.mode as string) && typeof entry.objectId === "string" && /^(?:[a-f0-9]{40}|[a-f0-9]{64})$/u.test(entry.objectId));
  });
}

function sourceAdmissionPhysicalConsistent(row: TaxonomySourceCandidateObservation): boolean {
  return (row.observedKind === "file" && (row.worktreeMode === "100644" || row.worktreeMode === "100755") && !row.explicitDirectory)
    || (row.observedKind === "directory" && row.worktreeMode === "040000" && row.explicitDirectory)
    || (row.observedKind === "symlink" && row.worktreeMode === "120000" && !row.explicitDirectory)
    || (["absent", "unobserved", "other"].includes(row.observedKind) && row.worktreeMode === null && !row.explicitDirectory);
}

function sourceAdmissionRepositoryFences(rows: readonly { readonly path: string; readonly entry: TaxonomySourceIndexEntry }[]): readonly string[] {
  return [...new Set(rows.filter((row) => row.entry.mode === "160000" && sourceAdmissionSafePath(row.path)).map((row) => row.path))].sort(sourceAdmissionByteCompare);
}

function sourceAdmissionContainingRepository(path: string, fences: readonly string[], includeRoot: boolean): string | null {
  const normalized = path.normalize("NFC");
  return fences.find((fence) => {
    const root = fence.normalize("NFC");
    return (includeRoot && normalized === root) || normalized.startsWith(root + "/");
  }) ?? null;
}

function sourceAdmissionAssertRepositoryPath(path: string, fences: readonly string[], label: string, allowRoot: boolean): void {
  const boundary = sourceAdmissionContainingRepository(path, fences, !allowRoot);
  if (boundary !== null) throw new Error(`${label} crosses an index-owned repository boundary: ${path} (${boundary})`);
}

/** 🧾️ Projects supplied source observations; only inventoryTaxonomySources performs filesystem admission. */
export function projectTaxonomySourceAdmission(value: unknown): TaxonomySourceAdmission {
  if (!sourceAdmissionInputShape(value)) return { schemaVersion: 1, scope: null, status: "rejected", observations: [], diagnostics: [{ code: "invalid-admission-input", path: "$", message: "Source admission input does not satisfy its closed schema" }] };
  const input = value;
  if (input.cancelledDuring !== undefined && input.cancelledDuring !== null) return { schemaVersion: 1, scope: input.scope, status: "rejected", observations: [], diagnostics: [{ code: "cancelled", path: input.cancelledDuring || "$", message: "Cancellation prevents a partial-success admission result" }] };
  if (input.scope !== null && !sourceAdmissionSafePath(input.scope)) return { schemaVersion: 1, scope: input.scope, status: "rejected", observations: [], diagnostics: [{ code: "invalid-scope", path: input.scope || "$", message: "Scope is not a safe repository-relative slash path" }] };
  const repositoryFences = sourceAdmissionRepositoryFences(input.candidates.flatMap((row) => row.indexEntries.filter((entry) => entry.mode === "160000").map((entry) => ({ path: row.sourcePath, entry }))));
  if (input.scope !== null && sourceAdmissionContainingRepository(input.scope, repositoryFences, false) !== null) return { schemaVersion: 1, scope: input.scope, status: "rejected", observations: [], diagnostics: [{ code: "scope-inside-repository-boundary", path: input.scope, message: "Scope is below an index-owned repository boundary" }] };
  const diagnostics: TaxonomySourceAdmissionDiagnostic[] = [];
  const diagnose = (code: string, path: string, message: string): void => { diagnostics.push({ code, path, message }); };
  for (const prefix of input.opaquePrefixes) if (!sourceAdmissionSafePath(prefix)) diagnose("invalid-opaque-prefix", prefix, "Opaque prefix is not a safe repository-relative slash path");
  for (const output of input.generatorOutputRoots) if (!sourceAdmissionSafePath(output.rootPath)) diagnose("invalid-generator-root", output.rootPath, "Generator output root is not a safe repository-relative slash path");
  for (const output of input.generatorOutputRoots) if (sourceAdmissionSafePath(output.rootPath) && sourceAdmissionContainingRepository(output.rootPath, repositoryFences, false) !== null) diagnose("generator-root-inside-repository-boundary", output.rootPath, "Generator output root is below an index-owned repository boundary");
  const generatorPolicies = new Map<string, TaxonomySourceGeneratorOutput[]>();
  for (const output of input.generatorOutputRoots) {
    const key = JSON.stringify([output.contractId, output.rootPath]);
    const group = generatorPolicies.get(key) ?? [];
    group.push(output);
    generatorPolicies.set(key, group);
  }
  for (const group of generatorPolicies.values()) if (new Set(group.map((output) => output.inclusion)).size > 1) diagnose("contradictory-generator-output", group[0].rootPath, "One generator contract/root identity declares conflicting inclusion policies");
  const groups = new Map<string, TaxonomySourceCandidateObservation[]>();
  for (const row of input.candidates) {
    if (!sourceAdmissionSafePath(row.sourcePath)) { diagnose("invalid-source-path", row.sourcePath, "Candidate sourcePath is not a safe repository-relative slash path"); continue; }
    if (sourceAdmissionContainingRepository(row.sourcePath, repositoryFences, false) !== null) diagnose("repository-boundary-descendant", row.sourcePath, "Candidate is below an index-owned repository boundary");
    if (!inScope(row.sourcePath, input.scope ?? undefined)) continue;
    const group = groups.get(row.sourcePath) ?? [];
    group.push(row);
    groups.set(row.sourcePath, group);
  }
  const observations = [...groups].map(([sourcePath, rows]): TaxonomySourceObservation => {
    const first = rows[0], normalized = sourcePath.normalize("NFC");
    const physical = new Set(rows.map((row) => JSON.stringify([row.observedKind, row.worktreeMode, row.explicitDirectory])));
    const indexEntries = [...new Map(rows.flatMap((row) => row.indexEntries).map((entry) => [JSON.stringify([entry.stage, entry.mode, entry.objectId]), entry])).values()].sort((left, right) => left.stage - right.stage || sourceAdmissionByteCompare(left.mode, right.mode) || sourceAdmissionByteCompare(left.objectId, right.objectId));
    const generatorOutputs = [...new Map(input.generatorOutputRoots.filter((root) => normalized === root.rootPath.normalize("NFC") || normalized.startsWith(root.rootPath.normalize("NFC") + "/")).map((root) => [JSON.stringify([root.contractId, root.rootPath, root.inclusion]), root])).values()].sort((left, right) => sourceAdmissionByteCompare(left.contractId, right.contractId) || sourceAdmissionByteCompare(left.rootPath, right.rootPath) || sourceAdmissionByteCompare(left.inclusion, right.inclusion));
    const supplied = new Set(rows.flatMap((row) => row.origins)), hasIgnored = generatorOutputs.some((root) => root.inclusion === "ignored");
    const opaque = sourceAdmissionOpaque(sourcePath, input.opaquePrefixes), unsafe = rows.some((row) => row.unsafeAncestor);
    const stageZero = indexEntries.some((entry) => entry.stage === 0), conflicted = indexEntries.some((entry) => entry.stage !== 0);
    const repositoryBoundary = !opaque && !unsafe && physical.size === 1 && supplied.has("tracked") && indexEntries.length === 1 && indexEntries[0].stage === 0 && indexEntries[0].mode === "160000" && sourceAdmissionPhysicalConsistent(first) && (first.observedKind === "directory" || first.observedKind === "absent") ? "gitlink" as const : null;
    if (physical.size > 1) diagnose("contradictory-physical-observation", sourcePath, "Duplicate rows disagree on observed physical kind, mode, or directory status");
    if (new Set(indexEntries.map((entry) => entry.stage)).size !== indexEntries.length) diagnose("contradictory-index-entry", sourcePath, "Duplicate rows disagree on an exact Git index stage identity");
    if (supplied.has("ignored-generator") && !hasIgnored) diagnose("untrusted-generator-origin", sourcePath, "Ignored-generator authority is derived only from declared ignored output roots");
    if (opaque) diagnose("opaque-path", sourcePath, "Configured opaque prefix rejected before candidate projection");
    if (unsafe) diagnose("unsafe-ancestor", sourcePath, "A symlink or non-directory ancestor prevented observation");
    if (rows.some((row) => !sourceAdmissionPhysicalConsistent(row) && row.worktreeMode !== "160000")) diagnose("inconsistent-physical-observation", sourcePath, "Observed kind, worktree mode, and explicit-directory status are inconsistent");
    if (rows.some((row) => row.observedKind === "other" || row.worktreeMode === "160000") || (repositoryBoundary === null && indexEntries.some((entry) => entry.mode === "160000"))) diagnose("nonregular-node", sourcePath, "Gitlink and other nonregular nodes cannot be admitted as authored source");
    if (stageZero && !supplied.has("tracked")) diagnose("index-without-tracked-origin", sourcePath, "Stage-zero index identity requires tracked admission provenance");
    if (supplied.has("tracked") && !stageZero && !conflicted) diagnose("tracked-origin-without-stage-zero", sourcePath, "Tracked admission requires an exact stage-zero index identity");
    if (conflicted) diagnose("conflicted-index", sourcePath, "Nonzero Git index stages prevent unambiguous source admission");
    if (rows.some((row) => row.observedKind === "unobserved") && !opaque && !unsafe && !conflicted) diagnose("unobserved-without-error", sourcePath, "Unobserved candidates require an explicit unsafe, opaque, conflict, or cancellation cause");
    if (rows.some((row) => row.observedKind === "absent") && stageZero) diagnose("tracked-path-absent", sourcePath, "Stage-zero index identity is retained although the worktree path is absent");
    if (hasIgnored) supplied.add("ignored-generator");
    const origins = opaque ? [] : SOURCE_ADMISSION_ORIGINS.filter((origin) => supplied.has(origin) && (origin !== "ignored-generator" || hasIgnored));
    if (!opaque && origins.length === 0 && !conflicted && !stageZero && !supplied.has("ignored-generator")) diagnose("no-admission-origin", sourcePath, "Candidate has no admitted source authority");
    return {
      sourcePath,
      observedKind: physical.size === 1 && !opaque && !unsafe ? first.observedKind : "unobserved",
      worktreeMode: physical.size === 1 && !opaque && !unsafe ? first.worktreeMode : null,
      explicitDirectory: physical.size === 1 && !opaque && !unsafe && first.explicitDirectory,
      origins,
      indexEntries: opaque ? [] : indexEntries,
      generatorOutputs: opaque ? [] : generatorOutputs,
      repositoryBoundary,
    };
  }).sort((left, right) => sourceAdmissionByteCompare(left.sourcePath, right.sourcePath));
  diagnostics.sort((left, right) => sourceAdmissionByteCompare(left.path, right.path) || sourceAdmissionByteCompare(left.code, right.code) || sourceAdmissionByteCompare(left.message, right.message));
  return { schemaVersion: 1, scope: input.scope, status: diagnostics.some((row) => row.code !== "tracked-path-absent") ? "rejected" : "complete", observations, diagnostics };
}
//#endregion 🧾️Source Admission

interface CandidatePath {
  readonly path: string;
  readonly mode: string;
  readonly objectId?: string;
  readonly explicitDirectory?: boolean;
}

export interface TaxonomyScopedGitPathspec {
  readonly normalizedScope: string | null;
  readonly conservativePrefix: string;
  readonly positivePathspec: string;
  readonly exclusionPathspecs: readonly string[];
}

interface MutableInventoryEntry {
  sourcePath: string;
  normalizedPath: string;
  nodeKind: TaxonomyNodeKind;
  ownerId: string;
  areaId: string;
  fileKind: string | null;
  semanticStem: string | null;
  fixedContractId?: string;
  packageRole?: TaxonomyPackageRole;
  contentHash: string;
  referencesIn: string[];
  referencesOut: string[];
  violations: TaxonomyViolation[];
  mode: number;
  size: number;
  symlinkTarget?: string;
}

/** 🧲️ Renders a byte-literal Git candidate prefix while retaining NFC scope authority in memory. */
export function taxonomyScopedGitPathspec(inputScope: string | null | undefined, opaqueExclusions: readonly string[]): TaxonomyScopedGitPathspec {
  const normalizedScope = inputScope === null || inputScope === undefined ? null : normalizeRelative(inputScope) || null;
  const stable: string[] = [];
  for (const segment of normalizedScope?.split("/") ?? []) {
    if (segment.normalize("NFD") !== segment) break;
    stable.push(segment);
  }
  const conservativePrefix = normalizedScope && stable.length > 0 ? stable.join("/") : ".";
  const intersects = (exclusion: string): boolean => conservativePrefix === "." || exclusion === conservativePrefix || exclusion.startsWith(`${conservativePrefix}/`) || conservativePrefix.startsWith(`${exclusion}/`);
  const exclusionPathspecs = [...new Set(opaqueExclusions.map(normalizeRelative))]
    .filter(intersects)
    .sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))
    .map((path) => `:(exclude,top,literal)${path}`);
  return {
    normalizedScope,
    conservativePrefix,
    positivePathspec: conservativePrefix === "." ? "." : `:(top,literal)${conservativePrefix}`,
    exclusionPathspecs,
  };
}

function scopedGitPathspec(repoRoot: string, scope: string | undefined, taxonomy: LoadedTaxonomy): TaxonomyScopedGitPathspec {
  const exclusions = taxonomy.exclusions.map((entry) => entry.path);
  const candidate = taxonomyScopedGitPathspec(scope, exclusions);
  if (!candidate.normalizedScope || candidate.conservativePrefix === ".") return candidate;
  const segments = candidate.normalizedScope.split("/");
  for (let index = 1; index < segments.length; index++) {
    const ancestor = segments.slice(0, index).join("/");
    const stat = lstatOrNull(absolutePath(repoRoot, ancestor));
    if (stat && (stat.isSymbolicLink() || !stat.isDirectory())) return taxonomyScopedGitPathspec(undefined, exclusions);
    const indexed = spawnSync("git", ["rev-parse", "--verify", "--quiet", "--end-of-options", `:${ancestor}`], { cwd: repoRoot, encoding: "utf8" });
    if (indexed.status === 0) return taxonomyScopedGitPathspec(undefined, exclusions);
    if (indexed.status !== 1) throw new Error(`Git index ancestor probe failed for ${ancestor}: ${indexed.stderr.trim() || `exit ${indexed.status ?? "unknown"}`}`);
  }
  return candidate;
}

function gitRows(repoRoot: string, taxonomy: LoadedTaxonomy, pathspec = taxonomyScopedGitPathspec(undefined, taxonomy.exclusions.map((entry) => entry.path))): readonly CandidatePath[] {
  const stdout = execFileSync("git", ["ls-files", "--stage", "-z", "--", pathspec.positivePathspec, ...pathspec.exclusionPathspecs], { cwd: repoRoot, encoding: "buffer", maxBuffer: 256 * 1024 * 1024 });
  return stdout
    .toString("utf8")
    .split("\u0000")
    .filter(Boolean)
    .map((row) => {
      const tab = row.indexOf("\t");
      const [mode, objectId, stage] = row.slice(0, tab).split(" ");
      return { path: sourceRelative(row.slice(tab + 1)), mode, objectId, stage };
    })
    .filter((row) => row.stage === "0")
    .map(({ path, mode, objectId }) => ({ path, mode, objectId }));
}

function untrackedGitPaths(repoRoot: string, taxonomy: LoadedTaxonomy, pathspec = taxonomyScopedGitPathspec(undefined, taxonomy.exclusions.map((entry) => entry.path))): readonly string[] {
  const stdout = execFileSync("git", ["ls-files", "--others", "--exclude-standard", "-z", "--", pathspec.positivePathspec, ...pathspec.exclusionPathspecs], { cwd: repoRoot, encoding: "buffer", maxBuffer: 256 * 1024 * 1024 });
  return stdout.toString("utf8").split("\u0000").filter(Boolean).map(sourceRelative).sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
}

function worktreeCandidate(repoRoot: string, path: string): CandidatePath | null {
  const stat = lstatOrNull(absolutePath(repoRoot, path));
  if (!stat) return null;
  if (stat.isSymbolicLink()) return { path, mode: "120000" };
  if (stat.isDirectory()) return { path, mode: "040000", explicitDirectory: true };
  return { path, mode: (stat.mode & 0o111) !== 0 ? "100755" : "100644" };
}

/** 🗑️ Reserved ticket-root child directory for a pipeline's own generated output; structurally excluded from {@link explicitTicketRows} so a tool's artifacts can never re-enter its own reference closure. Already gitignored repo-wide (see `.gitignore`). */
export const TICKET_GENERATED_OUTPUT_DIRECTORY = "🗑️temp";

function explicitTicketRows(repoRoot: string, ticketDir: string | undefined, taxonomy: LoadedTaxonomy, scope?: string, cancelFile?: string): readonly CandidatePath[] {
  if (!ticketDir) return [];
  const rel = sourceRelative(isAbsolute(ticketDir) ? relative(resolve(repoRoot), resolve(ticketDir)) : ticketDir);
  if (isExcluded(rel, taxonomy) || !inScope(rel, scope)) return [];
  const root = absolutePath(repoRoot, rel);
  if (!existsSync(root)) return [];
  const rows: CandidatePath[] = [];
  const walk = (currentRel: string): void => {
    checkCancellation(repoRoot, cancelFile);
    if (isExcluded(currentRel, taxonomy) || !inScope(currentRel, scope)) return;
    const currentAbs = absolutePath(repoRoot, currentRel);
    const stat = lstatSync(currentAbs);
    if (stat.isSymbolicLink()) {
      rows.push({ path: currentRel, mode: "120000" });
      return;
    }
    if (!stat.isDirectory()) {
      rows.push({ path: currentRel, mode: (stat.mode & 0o111) !== 0 ? "100755" : "100644" });
      return;
    }
    rows.push({ path: currentRel, mode: "040000", explicitDirectory: true });
    const nestedGit = taxonomy.schema.fixedDirectoryContracts["nested-git-metadata"];
    if (nestedGit && basename(currentRel) === ".git" && taxonomy.pathMatcher.matches(currentRel, nestedGit.pathPattern)) return;
    const children = readdirSync(currentAbs).sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
    for (const child of children) {
      const childRel = sourceRelative(`${currentRel}/${child}`);
      if (isExcluded(childRel, taxonomy) || (currentRel === rel && child === TICKET_GENERATED_OUTPUT_DIRECTORY)) continue;
      walk(childRel);
    }
  };
  walk(rel);
  return rows;
}

function generatorContractsForOutputPath(path: string, taxonomy: LoadedTaxonomy): readonly { readonly id: string; readonly contract: GeneratorContractSpec }[] {
  const normalized = normalizeRelative(path);
  return Object.entries(taxonomy.schema.generatorContracts)
    .filter(([, contract]) => contract.outputRoots.some((root) => normalized === root.path || normalized.startsWith(`${root.path}/`)))
    .map(([id, contract]) => ({ id, contract }))
    .sort((left, right) => left.id.localeCompare(right.id));
}

function ignoredGeneratorRows(repoRoot: string, taxonomy: LoadedTaxonomy, scope?: string): readonly CandidatePath[] {
  const rows = new Map<string, CandidatePath>();
  const walk = (path: string): void => {
    if (isExcluded(path, taxonomy) || !inScope(path, scope)) return;
    const stat = lstatOrNull(absolutePath(repoRoot, path));
    if (!stat) return;
    if (stat.isSymbolicLink()) {
      rows.set(path, { path, mode: "120000" });
      return;
    }
    if (!stat.isDirectory()) {
      rows.set(path, { path, mode: (stat.mode & 0o111) !== 0 ? "100755" : "100644" });
      return;
    }
    rows.set(path, { path, mode: "040000", explicitDirectory: true });
    for (const child of readdirSync(absolutePath(repoRoot, path)).sort((a, b) => Buffer.from(a).compare(Buffer.from(b)))) walk(sourceRelative(`${path}/${child}`));
  };
  for (const contract of Object.values(taxonomy.schema.generatorContracts)) for (const root of contract.outputRoots) if (root.inclusion === "ignored") walk(root.path);
  return [...rows.values()].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
}

//#region 🔐️Source Admission IO
class SourceAdmissionUnsafeAncestorError extends Error {}

interface SourceAdmissionPreparedOptions {
  readonly repoRoot: string;
  readonly scope?: string;
  readonly taxonomyPath: string;
  readonly ticketDir?: string;
  readonly cancelFile?: string;
  readonly indexRows: readonly { readonly path: string; readonly entry: TaxonomySourceIndexEntry }[];
  readonly repositoryFences: readonly string[];
}

function sourceAdmissionAssertLexical(value: string, label: string, allowAbsolute: boolean): void {
  if (typeof value !== "string" || !value || /[\u0000-\u001f\u007f]/u.test(value) || Buffer.from(value).toString("utf8") !== value) throw new Error(`${label} is not a lossless path`);
  if (value.replaceAll("\\", "/").split("/").some((segment) => segment.toLowerCase() === "compose")) throw new Error(`${label} is opaque`);
  if (!allowAbsolute && !sourceAdmissionSafePath(value)) throw new Error(`${label} is not a safe repository-relative path`);
  if (!allowAbsolute) return;
  const nativeRoot = isAbsolute(value) ? parse(value).root : "";
  const tail = value.slice(nativeRoot.length).split(sep).join("/");
  if ((tail && !sourceAdmissionSafePath(tail)) || (!nativeRoot && !tail)) throw new Error(`${label} has ambiguous or escaping path segments`);
}

function sourceAdmissionDirectoryChain(repoRoot: string): readonly { readonly path: string; readonly stat: Stats }[] {
  const root = parse(repoRoot).root;
  const paths = [root];
  for (const segment of repoRoot.slice(root.length).split(sep).filter(Boolean)) paths.push(join(paths[paths.length - 1], segment));
  return paths.map((path) => {
    const stat = lstatSync(path);
    if (stat.isSymbolicLink() || !stat.isDirectory()) throw new SourceAdmissionUnsafeAncestorError(`Source admission root has unsafe ancestry: ${path}`);
    return { path, stat };
  });
}

function sourceAdmissionLstat(repoRoot: string, path: string): Stats | null {
  sourceAdmissionAssertLexical(path, "Source admission candidate", false);
  const ancestors = [...sourceAdmissionDirectoryChain(repoRoot)];
  const segments = path.split("/");
  let absolute = repoRoot;
  for (let index = 0; index + 1 < segments.length; index++) {
    absolute = join(absolute, segments[index]);
    const stat = lstatOrNull(absolute);
    if (!stat) return null;
    if (stat.isSymbolicLink() || !stat.isDirectory()) throw new SourceAdmissionUnsafeAncestorError(`Source admission candidate has unsafe ancestry: ${path}`);
    ancestors.push({ path: absolute, stat });
  }
  const observed = lstatOrNull(join(absolute, segments[segments.length - 1]));
  for (const ancestor of ancestors) {
    const current = lstatOrNull(ancestor.path);
    if (!current || current.isSymbolicLink() || !current.isDirectory() || current.dev !== ancestor.stat.dev || current.ino !== ancestor.stat.ino || current.mode !== ancestor.stat.mode) throw new Error(`Source admission ancestry changed during observation: ${path}`);
  }
  return observed;
}

function sourceAdmissionPrepareOptions(options: TaxonomyInventoryOptions): SourceAdmissionPreparedOptions {
  if (options.repoRoot !== ".") sourceAdmissionAssertLexical(options.repoRoot, "repoRoot", true);
  if (options.scope !== undefined) sourceAdmissionAssertLexical(options.scope, "scope", false);
  for (const [label, value] of [["ticketDir", options.ticketDir], ["taxonomyPath", options.taxonomyPath ?? TAXONOMY_RELATIVE_PATH], ["cancelFile", options.cancelFile]] as const) if (value !== undefined) sourceAdmissionAssertLexical(value, label, true);
  const repoRoot = resolve(options.repoRoot);
  sourceAdmissionAssertLexical(repoRoot, "repoRoot", true);
  const local = (value: string, label: string): string => {
    const path = relative(repoRoot, isAbsolute(value) ? value : join(repoRoot, value)).split(sep).join("/");
    sourceAdmissionAssertLexical(path, label, false);
    return path;
  };
  const taxonomyPath = local(options.taxonomyPath ?? TAXONOMY_RELATIVE_PATH, "taxonomyPath");
  const ticketDir = options.ticketDir === undefined ? undefined : local(options.ticketDir, "ticketDir");
  const cancelFile = options.cancelFile === undefined ? undefined : local(options.cancelFile, "cancelFile");
  sourceAdmissionDirectoryChain(repoRoot);
  report(options.progress, "inventory", "tracked-enumeration", 0, 1, options.scope);
  const indexRows = sourceAdmissionGitRows(repoRoot, taxonomyScopedGitPathspec(undefined, ["compose"]));
  const repositoryFences = sourceAdmissionRepositoryFences(indexRows);
  if (options.scope !== undefined) sourceAdmissionAssertRepositoryPath(options.scope, repositoryFences, "Source admission scope", true);
  if (ticketDir !== undefined) sourceAdmissionAssertRepositoryPath(ticketDir, repositoryFences, "Source admission ticket", true);
  sourceAdmissionAssertRepositoryPath(taxonomyPath, repositoryFences, "Source admission taxonomy", false);
  if (cancelFile !== undefined) sourceAdmissionAssertRepositoryPath(cancelFile, repositoryFences, "Source admission cancellation", false);
  report(options.progress, "inventory", "tracked-enumeration", 1, 1, options.scope);
  const schema = sourceAdmissionLstat(repoRoot, taxonomyPath);
  if (!schema?.isFile() || schema.isSymbolicLink()) throw new Error("Taxonomy schema is not a no-follow regular file");
  return { repoRoot, scope: options.scope?.normalize("NFC"), taxonomyPath: join(repoRoot, ...taxonomyPath.split("/")), ticketDir, cancelFile, indexRows, repositoryFences };
}

function sourceAdmissionCheckCancellation(repoRoot: string, cancelFile: string | undefined, repositoryFences: readonly string[]): void {
  if (!cancelFile) return;
  sourceAdmissionAssertRepositoryPath(cancelFile, repositoryFences, "Source admission cancellation", false);
  const stat = sourceAdmissionLstat(repoRoot, cancelFile);
  if (stat?.isSymbolicLink()) throw new Error("Source admission cancellation path is a symlink");
  if (stat) throw new TaxonomyCancellationError();
}

function sourceAdmissionGitRecords(bytes: Uint8Array, label: string): readonly string[] {
  if (bytes.length === 0) return [];
  if (bytes[bytes.length - 1] !== 0) throw new Error(`${label} is missing its terminal NUL`);
  const rows = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes).slice(0, -1).split("\0");
  if (rows.some((row) => !row)) throw new Error(`${label} contains an empty record`);
  return rows;
}

function sourceAdmissionGitExclusions(pathspec: TaxonomyScopedGitPathspec): readonly string[] {
  return [...pathspec.exclusionPathspecs, ":(exclude,icase,glob)**/compose", ":(exclude,icase,glob)**/compose/**"];
}

function sourceAdmissionGitRows(repoRoot: string, pathspec: TaxonomyScopedGitPathspec): readonly { readonly path: string; readonly entry: TaxonomySourceIndexEntry }[] {
  const bytes = execFileSync("git", ["ls-files", "--stage", "-z", "--", pathspec.positivePathspec, ...sourceAdmissionGitExclusions(pathspec)], { cwd: repoRoot, encoding: "buffer", maxBuffer: 256 * 1024 * 1024 });
  return sourceAdmissionGitRecords(bytes, "Git stage output").map((row) => {
    const tab = row.indexOf("\t"), match = /^(100644|100755|120000|160000) ([0-9a-f]{40}|[0-9a-f]{64}) ([0-3])$/u.exec(row.slice(0, tab));
    const path = row.slice(tab + 1);
    if (tab < 1 || !match || !sourceAdmissionSafePath(path)) throw new Error("Git stage output has an invalid header or source path");
    return { path, entry: { mode: match[1], objectId: match[2], stage: Number(match[3]) } };
  });
}

function sourceAdmissionUntrackedRows(repoRoot: string, pathspec: TaxonomyScopedGitPathspec, taxonomy: LoadedTaxonomy, repositoryFences: readonly string[]): readonly { readonly path: string; readonly directoryMarker: boolean }[] {
  const literal = (path: string): string => path.replace(/[\\*?\[\]#! ]/gu, "\\$&");
  const exclusions = [...taxonomy.exclusions.map((entry) => entry.path), ...repositoryFences].map((path) => `--exclude=/${literal(path)}`);
  const boundaries = repositoryFences.map((path) => `:(exclude,top,literal)${path}`);
  const bytes = execFileSync("git", ["ls-files", "--others", "--exclude-standard", "--exclude=[cC][oO][mM][pP][oO][sS][eE]", ...exclusions, "-z", "--", pathspec.positivePathspec, ...sourceAdmissionGitExclusions(pathspec), ...boundaries], { cwd: repoRoot, encoding: "buffer", maxBuffer: 256 * 1024 * 1024 });
  return sourceAdmissionGitRecords(bytes, "Git untracked output").map((record) => {
    const directoryMarker = record.endsWith("/"), path = directoryMarker ? record.slice(0, -1) : record;
    if (!sourceAdmissionSafePath(path)) throw new Error("Git untracked output has an invalid source path");
    return { path, directoryMarker };
  }).sort((left, right) => sourceAdmissionByteCompare(left.path, right.path));
}

function sourceAdmissionWalk(repoRoot: string, root: string, taxonomy: LoadedTaxonomy, scope: string | undefined, cancelFile: string | undefined, repositoryFences: readonly string[]): readonly string[] {
  const rows: string[] = [];
  const opaquePrefixes = ["compose", ...taxonomy.exclusions.map((entry) => entry.path)];
  const visit = (path: string): void => {
    if (!sourceAdmissionSafePath(path)) throw new Error(`Source admission walk has an invalid path: ${path}`);
    if (sourceAdmissionOpaque(path, opaquePrefixes) || !inScope(path, scope)) return;
    sourceAdmissionAssertRepositoryPath(path, repositoryFences, "Source admission walk", true);
    sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
    let stat: Stats | null;
    try { stat = sourceAdmissionLstat(repoRoot, path); }
    catch (error) {
      if (!(error instanceof SourceAdmissionUnsafeAncestorError)) throw error;
      rows.push(path);
      return;
    }
    if (!stat) return;
    rows.push(path);
    if (!stat.isDirectory() || stat.isSymbolicLink()) return;
    if (sourceAdmissionContainingRepository(path, repositoryFences, true) !== null) return;
    const nestedGit = taxonomy.schema.fixedDirectoryContracts["nested-git-metadata"];
    if (nestedGit && basename(path) === ".git" && taxonomy.pathMatcher.matches(path, nestedGit.pathPattern)) return;
    const children = readdirSync(join(repoRoot, ...path.split("/")), { encoding: "buffer" }).map((name) => new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(name));
    const current = sourceAdmissionLstat(repoRoot, path);
    if (!current?.isDirectory() || current.isSymbolicLink() || current.dev !== stat.dev || current.ino !== stat.ino || current.mode !== stat.mode || current.mtimeMs !== stat.mtimeMs || current.ctimeMs !== stat.ctimeMs) throw new Error(`Source admission directory changed during enumeration: ${path}`);
    for (const child of children.sort(sourceAdmissionByteCompare)) visit(`${path}/${child}`);
  };
  visit(root);
  return rows.sort(sourceAdmissionByteCompare);
}

function sourceAdmissionObservation(repoRoot: string, path: string, origins: readonly TaxonomySourceOrigin[], indexEntries: readonly TaxonomySourceIndexEntry[]): TaxonomySourceCandidateObservation {
  try {
    const stat = sourceAdmissionLstat(repoRoot, path);
    if (!stat) return { sourcePath: path, observedKind: "absent", worktreeMode: null, explicitDirectory: false, origins, indexEntries, unsafeAncestor: false };
    if (stat.isSymbolicLink()) return { sourcePath: path, observedKind: "symlink", worktreeMode: "120000", explicitDirectory: false, origins, indexEntries, unsafeAncestor: false };
    if (stat.isDirectory()) return { sourcePath: path, observedKind: "directory", worktreeMode: "040000", explicitDirectory: true, origins, indexEntries, unsafeAncestor: false };
    if (stat.isFile()) return { sourcePath: path, observedKind: "file", worktreeMode: (stat.mode & 0o111) !== 0 ? "100755" : "100644", explicitDirectory: false, origins, indexEntries, unsafeAncestor: false };
    return { sourcePath: path, observedKind: "other", worktreeMode: null, explicitDirectory: false, origins, indexEntries, unsafeAncestor: false };
  } catch (error) {
    if (!(error instanceof SourceAdmissionUnsafeAncestorError)) throw error;
    return { sourcePath: path, observedKind: "unobserved", worktreeMode: null, explicitDirectory: false, origins, indexEntries, unsafeAncestor: true };
  }
}

/** 🧭️ Collects source admission without reading admitted leaf content. */
function collectTaxonomySourceAdmission(options: TaxonomyInventoryOptions, taxonomy: LoadedTaxonomy, prepared: SourceAdmissionPreparedOptions): CollectedTaxonomySourceAdmission {
  const { repoRoot, scope, cancelFile, indexRows, repositoryFences } = prepared;
  if (taxonomy.path !== prepared.taxonomyPath || !taxonomy.input) throw new Error("Source admission requires the exact loaded taxonomy input");
  const opaquePrefixes = ["compose", ...taxonomy.exclusions.map((entry) => entry.path)];
  if (scope && sourceAdmissionOpaque(scope, opaquePrefixes)) throw new Error(`Source admission scope is opaque: ${scope}`);
  for (const prefix of opaquePrefixes) if (!sourceAdmissionSafePath(prefix)) throw new Error("Source admission has an invalid opaque prefix");
  const generatorOutputRoots = Object.entries(taxonomy.schema.generatorContracts).flatMap(([contractId, contract]) => contract.outputRoots.map((root) => ({ contractId, rootPath: root.path, inclusion: root.inclusion === "ignored" ? "ignored" as const : "tracked" as const })));
  for (const output of generatorOutputRoots) {
    if (!sourceAdmissionSafePath(output.rootPath)) throw new Error("Source admission has an invalid generator output root");
    sourceAdmissionAssertRepositoryPath(output.rootPath, repositoryFences, "Source admission generator output", true);
  }
  const pathspec = taxonomyScopedGitPathspec(scope, opaquePrefixes);
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
  const rows = new Map<string, { origins: Set<TaxonomySourceOrigin>; indexEntries: TaxonomySourceIndexEntry[]; directoryMarker: boolean }>();
  const add = (path: string, origin: TaxonomySourceOrigin, entry?: TaxonomySourceIndexEntry, directoryMarker = false): void => {
    if (!sourceAdmissionSafePath(path)) throw new Error(`Source admission has an invalid candidate: ${path}`);
    if (sourceAdmissionOpaque(path, opaquePrefixes) || !inScope(path, scope)) return;
    sourceAdmissionAssertRepositoryPath(path, repositoryFences, "Source admission candidate", true);
    const row = rows.get(path) ?? { origins: new Set<TaxonomySourceOrigin>(), indexEntries: [], directoryMarker: false };
    row.directoryMarker ||= directoryMarker;
    row.origins.add(origin); if (entry) row.indexEntries.push(entry); rows.set(path, row);
  };
  for (const row of indexRows) add(row.path, "tracked", row.entry);
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
  report(options.progress, "inventory", "untracked-enumeration", 0, 1, scope);
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
  for (const row of sourceAdmissionUntrackedRows(repoRoot, pathspec, taxonomy, repositoryFences)) add(row.path, "nonignored-untracked", undefined, row.directoryMarker);
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences); report(options.progress, "inventory", "untracked-enumeration", 1, 1, scope);
  report(options.progress, "inventory", "ignored-generator-admission", 0, 1, scope);
  for (const output of generatorOutputRoots) if (output.inclusion === "ignored") for (const path of sourceAdmissionWalk(repoRoot, output.rootPath, taxonomy, scope, cancelFile, repositoryFences)) add(path, "ignored-generator");
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences); report(options.progress, "inventory", "ignored-generator-admission", 1, 1, scope);
  report(options.progress, "inventory", "explicit-ticket-admission", 0, 1, scope);
  if (prepared.ticketDir) for (const path of sourceAdmissionWalk(repoRoot, prepared.ticketDir, taxonomy, scope, cancelFile, repositoryFences)) add(path, "explicit-ticket");
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences); report(options.progress, "inventory", "explicit-ticket-admission", 1, 1, scope);
  const candidates: TaxonomySourceCandidateObservation[] = [];
  report(options.progress, "inventory", "source-observation", 0, rows.size, scope);
  for (const [path, row] of rows) {
    sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
    const observation = sourceAdmissionObservation(repoRoot, path, SOURCE_ADMISSION_ORIGINS.filter((origin) => row.origins.has(origin)), row.indexEntries);
    if (row.directoryMarker && (observation.observedKind !== "directory" || observation.unsafeAncestor)) throw new Error(`Git untracked directory marker no longer matches a directory: ${path}`);
    candidates.push(observation);
    report(options.progress, "inventory", "source-observation", candidates.length, rows.size, path);
  }
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
  const input: TaxonomySourceAdmissionInput = { scope: scope ?? null, opaquePrefixes: [...new Set(opaquePrefixes)], generatorOutputRoots, candidates };
  const inputText = JSON.stringify(input);
  const admission = projectTaxonomySourceAdmission(input);
  const inventory: TaxonomySourceInventory = { ...admission, repoRoot, taxonomyPath: relative(repoRoot, prepared.taxonomyPath).split(sep).join("/"), taxonomyContentHash: taxonomy.input.contentHash, membershipDigest: sha256(canonicalJson(admission)) };
  return { inventory, inputText };
}

/** 🧭️ Enumerates source admission without reading admitted leaf content. */
export function inventoryTaxonomySources(options: TaxonomyInventoryOptions): TaxonomySourceInventory {
  const prepared = sourceAdmissionPrepareOptions(options);
  const taxonomy = loadTaxonomy({ repoRoot: prepared.repoRoot, taxonomyPath: prepared.taxonomyPath });
  return collectTaxonomySourceAdmission(options, taxonomy, prepared).inventory;
}
//#endregion 🔐️Source Admission IO

function contentOf(repoRoot: string, row: CandidatePath): { readonly kind: TaxonomyNodeKind; readonly hash: string; readonly mode: number; readonly size: number; readonly symlinkTarget?: string; readonly bytes?: Uint8Array; readonly violation?: TaxonomyViolation } {
  if (row.mode === "040000") return { kind: "directory", hash: "", mode: 0, size: 0 };
  const path = absolutePath(repoRoot, row.path);
  if (!existsSync(path) && row.mode !== "120000") return { kind: "file", hash: row.objectId ?? sha256(""), mode: 0, size: 0, violation: violation("tracked-path-missing", row.path, "Tracked path is missing from the worktree") };
  try {
    const stat = lstatSync(path);
    if (stat.isSymbolicLink() || row.mode === "120000") {
      const target = readlinkSync(path);
      return { kind: "symlink", hash: sha256(target), mode: stat.mode & 0o7777, size: Buffer.byteLength(target), symlinkTarget: target };
    }
    if (stat.isDirectory()) return { kind: "directory", hash: "", mode: stat.mode & 0o7777, size: 0 };
    const bytes = readFileSync(path);
    return { kind: "file", hash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength, bytes };
  } catch (error) {
    return { kind: row.mode === "120000" ? "symlink" : "file", hash: row.objectId ?? sha256(""), mode: 0, size: 0, violation: violation("path-read-failed", row.path, error instanceof Error ? error.message : String(error)) };
  }
}

function packageLocation(path: string, taxonomy: LoadedTaxonomy): { readonly owner: string; readonly packageRoot: string; readonly ecosystemId: string | null; readonly rule: PackageBoundaryRule | null } | null {
  const parts = path.split("/");
  const packageIndex = parts.findIndex((part) => {
    const leading = splitLeadingEmojiIdentity(part);
    return leading.rest === "packages" && taxonomy.directoryKinds.some((kind) => kind.id === "packages" && emojiFold(kind.emoji) === emojiFold(leading.first));
  });
  if (packageIndex >= 0) {
    const owner = parts.slice(0, packageIndex).join("/");
    const ecosystemSegment = parts[packageIndex + 1] ?? "";
    const ecosystemIds = Object.keys(taxonomy.schema.ecosystems).filter((id) => emojiFold(id) === emojiFold(ecosystemSegment));
    const selected = ecosystemIds.length === 1 && taxonomy.schema.packageBoundaryRules[ecosystemIds[0]] ? [ecosystemIds[0], taxonomy.schema.packageBoundaryRules[ecosystemIds[0]]] as const : null;
    return { owner, packageRoot: parts.slice(0, packageIndex + 2).join("/"), ecosystemId: selected?.[0] ?? null, rule: selected?.[1] ?? null };
  }
  // 🦀️ A `generator-crate` directory IS a standalone Rust package root by construction — always
  // paired with its own Cargo.toml/Cargo.lock directly inside it (e.g.
  // `…/🏭️generator/🦀️note-oracle-codec`), never nested under a `packages/🦀️rust` ecosystem folder.
  // Recognizes the same package-root property `packages/<ecosystem>` marks structurally, one level
  // deep instead of two, so a plugin-nested generator crate is not silently treated as ownerless.
  const generatorKind = taxonomy.directoryKinds.find((kind) => kind.id === "generator");
  const generatorCrateKind = taxonomy.directoryKinds.find((kind) => kind.id === "generator-crate");
  const crateIndex = generatorKind && generatorCrateKind ? parts.findIndex((part, index) => {
    if (index === 0) return false;
    const leading = splitLeadingEmojiIdentity(part);
    if (emojiFold(leading.first) !== emojiFold(generatorCrateKind.emoji)) return false;
    const parentLeading = splitLeadingEmojiIdentity(parts[index - 1]);
    return parentLeading.rest === "generator" && emojiFold(parentLeading.first) === emojiFold(generatorKind.emoji);
  }) : -1;
  if (crateIndex < 0) return null;
  const ecosystemId = "🦀️rust";
  const rule = taxonomy.schema.packageBoundaryRules[ecosystemId] ?? null;
  return { owner: parts.slice(0, crateIndex).join("/"), packageRoot: parts.slice(0, crateIndex + 1).join("/"), ecosystemId: rule ? ecosystemId : null, rule };
}

type FixedContract = FixedFilenameContract | FixedDirectoryContract;
type FixedSpecificity = readonly [literalSegments: number, literalCodePoints: number, negativeWildcardTokens: number, scopeRank: number];

/** 🎯️ Ranks `FixedContractScope` kinds by how narrowly they constrain a match, most permissive
 * first — an unrestricted `path-pattern` scope matches anywhere, while `sibling-fixed-filename-
 * contract` and `fixed-directory-contract` additionally require another already-resolved, more
 * specific contract to independently hold. Two contracts sharing one `pathPattern` (e.g.
 * `node-package-manifest` vs `nx-owned-node-package-manifest`, both `**\/package.json`) previously
 * tied at equal specificity whenever both scopes matched, producing `fixed-contract-ambiguous`; this
 * grades every scope kind distinctly so the narrower one wins deterministically instead. */
export type FixedContractScopeKind = FixedContractScope["kind"];

const FIXED_CONTRACT_SCOPE_SPECIFICITY: Readonly<Record<FixedContractScopeKind, number>> = {
  "path-pattern": 0,
  "repository-root": 1,
  "directory-kind": 2,
  "package-root": 3,
  "fixed-directory-contract": 4,
  "fixed-directory-contract-set": 4,
  "sibling-fixed-filename-contract": 5,
  "exact-path": 6,
};

/** 🧪️ Explicit, minimal re-export of the scope-kind specificity ladder for `fixedSpecificity` — kept
 * separate from the internal `FixedContractScope` union type so tests depend on a stable string-enum
 * boundary instead of reaching into the taxonomy engine's own contract shapes. */
export function fixedContractScopeSpecificityRank(scopeKind: FixedContractScopeKind): number {
  return FIXED_CONTRACT_SCOPE_SPECIFICITY[scopeKind];
}

function fixedSpecificity(contract: FixedContract): FixedSpecificity {
  const segments = contract.pathPattern.split("/");
  const tokens = contract.pathPattern.match(/\*\*|\*|\?|\[[^\]]+\]/gu) ?? [];
  const literals = contract.pathPattern.replaceAll("/", "").replace(/\*\*|\*|\?|\[[^\]]+\]/gu, "");
  return [segments.filter((segment) => !/[?*\[]/u.test(segment)).length, [...literals].length, -tokens.length, FIXED_CONTRACT_SCOPE_SPECIFICITY[contract.scope.kind]];
}

function compareFixedSpecificity(left: FixedSpecificity, right: FixedSpecificity): number {
  for (let index = 0; index < left.length; index++) if (left[index] !== right[index]) return right[index] - left[index];
  return 0;
}

function equalFixedSpecificity(left: FixedSpecificity, right: FixedSpecificity): boolean {
  return left.every((value, index) => value === right[index]);
}

function fixedScopeMatches(contract: FixedContract, path: string, packageInfo: ReturnType<typeof packageLocation>, parentKindId?: string, parentFixedDirectoryContractId?: string, siblingFixedFilenameContractIds: readonly string[] = []): boolean {
  if (contract.scope.kind === "exact-path") return path === contract.scope.path;
  if (contract.scope.kind === "repository-root") return !path.includes("/");
  if (contract.scope.kind === "package-root") return packageInfo?.packageRoot === dirname(path) && packageInfo.ecosystemId === contract.scope.ecosystemId;
  if (contract.scope.kind === "directory-kind") return parentKindId === contract.scope.directoryKindId;
  if (contract.scope.kind === "fixed-directory-contract") return parentFixedDirectoryContractId === contract.scope.fixedDirectoryContractId;
  if (contract.scope.kind === "fixed-directory-contract-set") return parentFixedDirectoryContractId !== undefined && contract.scope.fixedDirectoryContractIds.includes(parentFixedDirectoryContractId);
  if (contract.scope.kind === "sibling-fixed-filename-contract") return siblingFixedFilenameContractIds.includes(contract.scope.fixedFilenameContractId);
  return true;
}

function matchingFixedContracts<T extends FixedContract>(path: string, contracts: Readonly<Record<string, T>>, taxonomy: LoadedTaxonomy, packageInfo: ReturnType<typeof packageLocation>, parentKindId?: string, parentFixedDirectoryContractId?: string, siblingFixedFilenameContractIds: readonly string[] = []): { readonly selected: readonly [string, T] | null; readonly ambiguous: readonly string[] } {
  const matches = Object.entries(contracts)
    .filter(([, contract]) => taxonomy.pathMatcher.matches(path, contract.pathPattern) && fixedScopeMatches(contract, path, packageInfo, parentKindId, parentFixedDirectoryContractId, siblingFixedFilenameContractIds))
    .map(([id, contract]) => ({ id, contract, specificity: fixedSpecificity(contract) }))
    .sort((left, right) => compareFixedSpecificity(left.specificity, right.specificity) || left.id.localeCompare(right.id));
  if (matches.length === 0) return { selected: null, ambiguous: [] };
  const top = matches.filter((entry) => equalFixedSpecificity(entry.specificity, matches[0].specificity));
  return top.length === 1 ? { selected: [top[0].id, top[0].contract], ambiguous: [] } : { selected: null, ambiguous: top.map((entry) => entry.id) };
}

function configurableContract(path: string, taxonomy: LoadedTaxonomy, packageInfo: ReturnType<typeof packageLocation>): [string, ConfigurableEntryContract] | null {
  const rows = Object.entries(taxonomy.schema.configurableEntryContracts).filter(([, contract]) => basename(path).normalize("NFC") === contract.filename.normalize("NFC") && packageInfo?.ecosystemId === contract.ecosystemId);
  return rows.length === 1 ? rows[0] : null;
}

/** 🧵️ Every alternative a flat sequence of Rust wiring statements is allowed to be made of: bare
 * `mod`/`use` statements, `extern crate`, a `type` alias (covers `#[cfg(...)] pub type Alias = ...;`
 * backend-selection glue such as `🖥️host/📦️packages/🦀️rust/🦀️backend_alias.rs`), an attribute —
 * though `classifyGlue`'s own comment-stripping already removes every `#`-led line before this ever
 * runs, so the attribute alternative only matters for direct callers of `isRustDeclarativeStatementSequence`
 * — or one call to the framework's own `plugin_exports!` macro (any qualifying `::`-path prefix,
 * e.g. `semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::NoteApps);`): every one of
 * the 33 plugin `🦀️.rs` files ends with exactly this, registering the crate's `Plugin`/`Apps`
 * builder with the framework — never a local definition, never expanded here, pure wiring like the
 * `mod`/`use` lines around it. Named narrowly (not "any macro call") so an actual proc-macro or
 * declarative-macro body elsewhere in a glue file still fails this check and reports its real role.
 * `*`, not `+`: `stripDeclarativeRustModuleBlocks` re-tests a `mod`'s OWN body after already
 * stripping its purely-declarative nested `mod {}` children — a body that held nothing else (e.g.
 * `pub mod any { mod component; pub use component::*; }`) reduces to pure whitespace, and `+` used
 * to reject that empty remainder as "not declarative", poisoning every ancestor wrapper all the way
 * to the file root (this is exactly why every one of the 33 plugin `🦀️.rs` files — pure `mod`
 * nesting around leaf `component.rs` re-exports — read `unresolved` instead of `declaration`). */
const RUST_DECLARATIVE_STATEMENT_SEQUENCE = /^(?:\s*(?:pub\s+)?(?:mod|use)\b[^;]*;|\s*(?:pub\s+)?extern\s+crate\b[^;]*;|\s*(?:pub\s+)?type\s+\w+[^=;{]*=[^;]*;|\s*#!?\[[^\]]+\]\s*|\s*(?:[\w]+::)*plugin_exports!\s*\([^()]*\)\s*;)*$/s;

function isRustDeclarativeStatementSequence(source: string): boolean {
  return RUST_DECLARATIVE_STATEMENT_SEQUENCE.test(source.trim());
}

/** 🪆️ Recursively strips `mod name { ... }`/`pub mod name { ... }` blocks whose entire body is
 * itself nothing but declarative wiring, so a namespacing wrapper around otherwise-thin re-exports —
 * e.g. `📡️replication/📦️packages/🦀️rust/🦀️.rs`'s `pub mod codec { #[path = "..."] mod x; pub use
 * x::*; }` — does not defeat the flat grammar above, which only recognizes semicolon-terminated
 * statements. A block whose body is NOT purely declarative is left untouched, so the outer check
 * correctly still fails on its stray `{`/`}`. */
function stripDeclarativeRustModuleBlocks(source: string): string {
  const opener = /(?:pub\s+)?mod\s+\w+\s*\{/g;
  let result = "";
  let cursor = 0;
  let match: RegExpExecArray | null;
  while ((match = opener.exec(source))) {
    if (match.index < cursor) continue;
    const bodyStart = match.index + match[0].length;
    let depth = 1;
    let index = bodyStart;
    while (index < source.length && depth > 0) {
      if (source[index] === "{") depth++;
      else if (source[index] === "}") depth--;
      index++;
    }
    if (depth !== 0) break;
    const body = source.slice(bodyStart, index - 1);
    if (isRustDeclarativeStatementSequence(stripDeclarativeRustModuleBlocks(body))) {
      result += source.slice(cursor, match.index);
      cursor = index;
    }
    opener.lastIndex = index;
  }
  result += source.slice(cursor);
  return result;
}

/** 🧵️ Blanks the contents of every quoted string/template literal (keeping the quotes) so a data
 * value — a file path, say — can never be misread as a code keyword. `🖱️ui`'s React target keeps a
 * `vitest.config.ts` whose `include` list names `🏷️class-name-composition`; `\bclass\b` matches that
 * substring exactly as it would the real keyword, which used to force the whole file to
 * "implementation" before ever reaching either keyword-sniffing check below. */
function stripStringLiterals(source: string): string {
  return source.replace(/"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|`(?:[^`\\]|\\.)*`/g, '""');
}

/** 🗂️ A TypeScript/JavaScript module shaped as pure configuration data: only imports, re-exports,
 * `const` bindings and one `export default`, with no function/class/control-flow anywhere in its
 * body — e.g. `export default { root, test: {...} };` or `export default defineConfig({...});`.
 * `classifyGlue`'s default fallback otherwise calls any non-declaration TS/JS content
 * "implementation", which wrongly caught data-only config modules (`🧪️tests/🟦️.ts`,
 * `🎨️postcss.config.ts`, `🟦️eslint.config.ts`) sitting inside a package boundary. */
function isConfigDelegationModule(normalized: string): boolean {
  const withoutTrailingLineComments = normalized.replace(/\/\/[^\n]*$/gm, "");
  const withoutStringLiterals = stripStringLiterals(withoutTrailingLineComments);
  const statements = splitTopLevelStatements(withoutTrailingLineComments);
  if (statements.length === 0) return false;
  let sawDefaultExport = false;
  for (const statement of statements) {
    if (/^import\b/.test(statement)) continue;
    if (/^export\s+(?:\*|\{[^}]*\}|type\b|interface\b|enum\b)/.test(statement)) continue;
    if (/^export\s+default\b/.test(statement)) {
      sawDefaultExport = true;
      continue;
    }
    if (/^const\s+\w+/.test(statement)) continue;
    return false;
  }
  return sawDefaultExport && !/\bfunction\b|=>|\bclass\b|\bif\s*\(|\bfor\s*\(|\bwhile\s*\(|\bswitch\s*\(|\btry\b/.test(withoutStringLiterals);
}

/** ✂️ Splits source into top-level (bracket-depth-zero) `;`-terminated statements, tolerant of
 * arbitrarily nested `{}`/`()`/`[]` — used by `isConfigDelegationModule` instead of a single regex so
 * a deeply-nested config object (`vitest.config.ts`'s `test: { coverage: { include: [...] } }`) does
 * not need its own bespoke nesting depth. */
function splitTopLevelStatements(source: string): readonly string[] {
  const statements: string[] = [];
  let depth = 0;
  let start = 0;
  for (let index = 0; index < source.length; index++) {
    const char = source[index];
    if (char === "{" || char === "(" || char === "[") depth++;
    else if (char === "}" || char === ")" || char === "]") depth--;
    else if (char === ";" && depth === 0) {
      statements.push(source.slice(start, index + 1));
      start = index + 1;
    }
  }
  const rest = source.slice(start).trim();
  if (rest) statements.push(rest);
  return statements.map((statement) => statement.trim()).filter(Boolean);
}

export type PackageGlueAnalyzer = PackageGlueGrammar["analyzer"];

/** 🧪️ Explicit, minimal re-export of the package-boundary content classifier for tests — see
 * `TaxonomyPackageRole` for the result vocabulary. Keeps `PackageGlueGrammar` itself internal; only
 * the plain string-literal `analyzer` id crosses the boundary. */
export function classifyPackageGlueContent(analyzer: PackageGlueAnalyzer, content: string, maxDelegationStatements: number): TaxonomyPackageRole {
  return classifyGlue(analyzer, content, maxDelegationStatements);
}

function classifyGlue(analyzer: PackageGlueGrammar["analyzer"], content: string, maxStatements: number): TaxonomyPackageRole {
  const normalized = content.replace(/\/\*[\s\S]*?\*\//g, "").replace(/^\s*\/\/.*$/gm, "").replace(/^\s*#.*$/gm, "").trim();
  if (normalized.length === 0) return "declaration";
  if (analyzer === "rust") {
    if (/\b(?:struct|enum|trait|union|impl)\b/.test(normalized)) return "implementation";
    const bodies = [...normalized.matchAll(/\bfn\s+\w+[^\{]*\{([\s\S]*?)\}/g)].map((match) => match[1].split(";").map((part) => part.trim()).filter(Boolean).length);
    if (bodies.some((count) => count > maxStatements)) return "implementation";
    if (/\bfn\s+(?:main|start|bootstrap)\b/.test(normalized)) return "bootstrap";
    if (/\b(?:register|provide|bind)\w*\s*\(/i.test(normalized)) return "registration";
    if (isRustDeclarativeStatementSequence(stripDeclarativeRustModuleBlocks(normalized))) return "declaration";
    return bodies.length > 0 ? "thin-delegation" : "unresolved";
  }
  if (analyzer === "typescript" || analyzer === "javascript") {
    if (/\b(?:class|namespace)\b/.test(stripStringLiterals(normalized))) return "implementation";
    if (/^(?:\s*(?:import\b[^;]*;?|export\s+(?:\*|\{[^}]*\}|type\b[^;]*|interface\b[^{]*\{[^}]*\}|enum\b[^{]*\{[^}]*\})[^;]*;?)\s*)+$/s.test(normalized)) return "declaration";
    if (/\b(?:register|provide|bind)\w*\s*\(/i.test(normalized)) return "registration";
    const functionBodies = [...normalized.matchAll(/(?:function\s+([\w$]+)[^{]*|(?:const|let)\s+([\w$]+)\s*=\s*(?:async\s*)?\([^)]*\)\s*=>)\{([\s\S]*?)\}/g)];
    if (functionBodies.length > 0) {
      const thin = functionBodies.every((match) => {
        const name = match[1] ?? match[2] ?? "";
        const statements = match[3].split(";").map((part) => part.trim()).filter(Boolean);
        return /^(?:main|start|bootstrap|run)$/i.test(name) && statements.length <= maxStatements && statements.every((statement) => /^(?:return\s+)?(?:await\s+)?[\w$.]+\([^;]*\)$/.test(statement));
      });
      return thin ? "thin-delegation" : "implementation";
    }
    if (/=>|\bfunction\b|\.(?:reduce|map|filter|flatMap|sort)\s*\(/.test(normalized)) return "implementation";
    if (isConfigDelegationModule(normalized)) return "thin-delegation";
    return "implementation";
  }
  if (analyzer === "go") {
    if (/\btype\s+\w+\s+(?:struct|interface)\b/.test(normalized)) return "implementation";
    const bodies = [...normalized.matchAll(/\bfunc\s+(?:main|init)\s*\([^)]*\)\s*\{([\s\S]*?)\}/g)];
    if (bodies.length > 0 && bodies.every((match) => match[1].split("\n").map((line) => line.trim()).filter(Boolean).length <= maxStatements)) return "bootstrap";
    if (/^package\s+\w+\s+(?:import\s*(?:\([^)]*\)|"[^"]+")\s*)?$/s.test(normalized)) return "declaration";
    return "implementation";
  }
  if (analyzer === "python") {
    if (/^\s*(?:class|def)\s+/m.test(normalized)) return "implementation";
    if (/^(?:\s*(?:from\s+\S+\s+import|import\s+|__all__\s*=)[^\n]*\n?)+$/s.test(normalized)) return "declaration";
    const statements = normalized.split("\n").map((line) => line.trim()).filter(Boolean).length;
    if (statements <= maxStatements && /if\s+__name__\s*==\s*["']__main__["']/.test(normalized)) return "bootstrap";
    return "implementation";
  }
  if (analyzer === "c-cpp") {
    if (/\b(?:class|struct|union|enum)\b|\w+\s*\([^;{}]*\)\s*\{/u.test(normalized)) return "implementation";
    if (/^(?:\s*(?:#\s*(?:include|define|pragma)\b[^\n]*|(?:using|typedef|extern)\b[^;]*;)\s*)+$/su.test(normalized)) return "declaration";
    return "unresolved";
  }
  if (/\b(?:class|struct|interface|record|enum)\b/.test(normalized)) return "implementation";
  if (/\b(?:AddSingleton|AddScoped|AddTransient|Register)\b/.test(normalized)) return "registration";
  if (/^(?:\s*(?:using|global\s+using|\[assembly:)[^;\n]*(?:;|\])\s*)+$/s.test(normalized)) return "declaration";
  return "unresolved";
}

function classifyPackageRole(path: string, kindId: string | null, fixedId: string | undefined, content: string | null, taxonomy: LoadedTaxonomy): TaxonomyPackageRole {
  const location = packageLocation(path, taxonomy);
  if (!location) return "not-package";
  if (fixedId || configurableContract(path, taxonomy, location)) return "configuration";
  if (!location.rule || !location.ecosystemId) return "unresolved";
  if (kindId && !location.rule.allowedFileKindIds.includes(kindId)) return "implementation";
  if (!content) return "configuration";
  const grammar = taxonomy.schema.packageGlueGrammar[location.rule.glueGrammarId];
  const role = classifyGlue(grammar.analyzer, content, grammar.maxDelegationStatements);
  return grammar.allowedRoles.includes(role as PackageGlueGrammar["allowedRoles"][number]) ? role : role === "implementation" ? "implementation" : "unresolved";
}

function canonicalDirectory(path: string, parentCanonical: string, parentKindId: string | undefined, ancestorKindIds: readonly string[], taxonomy: LoadedTaxonomy): { readonly path: string; readonly kindId: string | null; readonly fixedId?: string; readonly violations: readonly TaxonomyViolation[] } {
  const name = basename(path).normalize("NFC");
  const domains = taxonomy.discoverySchema.mutationDomainOwners[dirname(path)], domainOwner = mutationDomainOwnerLocation(path, taxonomy);
  if (domains && Object.hasOwn(domains, name) || domainOwner && path === `${domainOwner.root}/${domainOwner.relativePath}`) return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: "members-of-schema", violations: [] };
  const fixed = matchingFixedContracts(path, taxonomy.schema.fixedDirectoryContracts, taxonomy, packageLocation(path, taxonomy), parentKindId);
  if (fixed.ambiguous.length > 0) return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation("fixed-directory-contract-ambiguous", path, `Equal-specificity fixed directory contracts match: ${fixed.ambiguous.join(", ")}`)] };
  if (fixed.selected) {
    const context = matchDirectoryKind(name, taxonomy, parentKindId, ancestorKindIds);
    return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: context.kind?.id ?? null, fixedId: fixed.selected[0], violations: [] };
  }
  if (parentKindId === "packages") {
    const packageKinds = Object.keys(taxonomy.schema.packageBoundaryRules).filter((id) => emojiFold(id) === emojiFold(name));
    if (packageKinds.length > 1) return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation("package-language-ambiguous", path, `Package language boundary is ambiguous: ${packageKinds.join(", ")}`)] };
  }
  const match = matchDirectoryKind(name, taxonomy, parentKindId, ancestorKindIds);
  if (!match.kind) {
    const message = match.ambiguous.length > 1 ? `Directory semantic kind is ambiguous: ${match.ambiguous.join(", ")}` : "Directory has no registered semantic kind";
    return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation(match.ambiguous.length > 1 ? "directory-kind-ambiguous" : "directory-kind-unresolved", path, message)] };
  }
  const identity = splitLeadingEmojiIdentity(name);
  const canonicalName = identity.sequence !== identity.first ? name : `${match.kind.emoji}${match.slug}`.normalize("NFC");
  const violations = identity.first ? pathEmojiStatuteFindings([{ path, nodeKind: "directory" }], []).map((finding) => violation(`path-emoji-${finding.kind}`, path, "Directory emoji must be handpicked, singular, and correctly presented.")) : [];
  return { path: parentCanonical ? `${parentCanonical}/${canonicalName}` : canonicalName, kindId: match.kind.id, violations };
}

function canonicalFile(
  path: string,
  parentCanonical: string,
  parentKindId: string | undefined,
  ancestorKindIds: readonly string[],
  directoryKindByPath: ReadonlyMap<string, string>,
  fixedDirectoryContractByPath: ReadonlyMap<string, string>,
  siblingFixedFilenameContractIdsByParent: ReadonlyMap<string, readonly string[]>,
  taxonomy: LoadedTaxonomy,
  contentKindId?: string,
): { readonly path: string; readonly fileKind: string | null; readonly stem: string | null; readonly fixedId?: string; readonly semanticDirectoryName?: string; readonly violations: readonly TaxonomyViolation[] } {
  const packageInfo = packageLocation(path, taxonomy);
  let fixedName = basename(path);
  const parent = dirname(path);
  let fixed = matchingFixedContracts(path, taxonomy.schema.fixedFilenameContracts, taxonomy, packageInfo, directoryKindByPath.get(parent), fixedDirectoryContractByPath.get(parent), siblingFixedFilenameContractIdsByParent.get(parent));
  const decoratedFixedName = splitLeadingEmoji(fixedName);
  if (!fixed.selected && fixed.ambiguous.length === 0 && decoratedFixedName.emoji && decoratedFixedName.rest) {
    const candidatePath = dirname(path) === "." ? decoratedFixedName.rest : `${dirname(path)}/${decoratedFixedName.rest}`;
    const candidate = matchingFixedContracts(candidatePath, taxonomy.schema.fixedFilenameContracts, taxonomy, packageLocation(candidatePath, taxonomy), directoryKindByPath.get(parent), fixedDirectoryContractByPath.get(parent), siblingFixedFilenameContractIdsByParent.get(parent));
    if (candidate.selected || candidate.ambiguous.length > 0) {
      fixed = candidate;
      fixedName = decoratedFixedName.rest;
    }
  }
  if (fixed.ambiguous.length > 0) return { path: parentCanonical ? `${parentCanonical}/${basename(path)}` : basename(path), fileKind: null, stem: null, violations: [violation("fixed-contract-ambiguous", path, `Equal-specificity fixed filename contracts match: ${fixed.ambiguous.join(", ")}`)] };
  if (fixed.selected) return { path: parentCanonical ? `${parentCanonical}/${fixedName}` : fixedName, fileKind: null, stem: null, fixedId: fixed.selected[0], violations: [] };
  const documentationName = reservedDocumentationBasename(basename(path));
  if (documentationName) return { path: parentCanonical ? `${parentCanonical}/${documentationName}` : documentationName, fileKind: null, stem: null, violations: [] };
  const configurable = configurableContract(path, taxonomy, packageInfo);
  const resolvedKind = resolveFileKind(path, taxonomy, parentKindId, ancestorKindIds, configurable?.[1].fileKindId, contentKindId);
  if (!resolvedKind.kind) {
    const message = resolvedKind.ambiguous.length > 1 ? `File kind is ambiguous: ${resolvedKind.ambiguous.join(", ")}` : "No file kind owns the longest extension chain";
    return { path: parentCanonical ? `${parentCanonical}/${basename(path).normalize("NFC")}` : basename(path).normalize("NFC"), fileKind: null, stem: null, violations: [violation(resolvedKind.ambiguous.length > 1 ? "file-kind-ambiguous" : "file-kind-unresolved", path, message)] };
  }
  const sourceIdentity = splitLeadingEmojiIdentity(resolvedKind.stem);
  if (sourceIdentity.first) {
    const preserved = basename(path).normalize("NFC");
    const violations = pathEmojiStatuteFindings([{ path, nodeKind: "file" }], []).map((finding) => violation(`path-emoji-${finding.kind}`, path, "File emoji must be handpicked, singular, and correctly presented."));
    return { path: parentCanonical ? `${parentCanonical}/${preserved}` : preserved, fileKind: resolvedKind.kind.id, stem: sourceIdentity.rest || null, violations };
  }
  const leadingSemantic = splitLeadingEmoji(resolvedKind.stem);
  const semanticEvidence = leadingSemantic.emoji || "";
  const sourceStem = semanticEvidence ? leadingSemantic.rest : resolvedKind.stem;
  const testSuffix = sourceStem.endsWith(".test");
  const semanticStem = testSuffix ? sourceStem.slice(0, -".test".length) : sourceStem;
  const kindOnly = `${resolvedKind.kind.emoji}${resolvedKind.extension}`.normalize("NFC");
  if (!semanticStem || configurable || GENERIC_SEMANTIC_STEMS.has(semanticStem.toLocaleLowerCase("und"))) return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem || null, violations: [] };
  const parentSlug = splitLeadingEmoji(basename(dirname(path))).rest;
  if (parentSlug.normalize("NFC").toLocaleLowerCase("und") === semanticStem.normalize("NFC").toLocaleLowerCase("und")) return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem, violations: [] };
  const roleContext = testSuffix ? "tests" : resolvedKind.kind.role === "asset" ? "assets" : resolvedKind.kind.role === "test" ? "tests" : parentKindId;
  const semantic = matchDirectoryKind(`${semanticEvidence}${semanticStem}`, taxonomy, roleContext);
  if (!semantic.kind) {
    const message = semantic.ambiguous.length > 1 ? `Semantic stem matches multiple directory kinds: ${semantic.ambiguous.join(", ")}` : "Semantic stem has no registered directory kind";
    return { path: parentCanonical ? `${parentCanonical}/${basename(path).normalize("NFC")}` : basename(path).normalize("NFC"), fileKind: resolvedKind.kind.id, stem: semanticStem, violations: [violation(semantic.ambiguous.length > 1 ? "semantic-stem-ambiguous" : "semantic-stem-unresolved", path, message)] };
  }
  if (parentKindId === semantic.kind.id && parentSlug === semanticStem) return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem, violations: [] };
  const semanticDirectory = `${semantic.kind.emoji}${semanticStem}`.normalize("NFC");
  return { path: parentCanonical ? `${parentCanonical}/${semanticDirectory}/${kindOnly}` : `${semanticDirectory}/${kindOnly}`, fileKind: resolvedKind.kind.id, stem: semanticStem, semanticDirectoryName: semanticDirectory, violations: [] };
}

function packageImplementationDestination(
  sourcePath: string,
  canonical: ReturnType<typeof canonicalFile>,
  canonicalDirectoryByPath: ReadonlyMap<string, string>,
  directoryKindByPath: ReadonlyMap<string, string>,
  taxonomy: LoadedTaxonomy,
): string | null {
  const location = packageLocation(sourcePath, taxonomy);
  if (!location || !canonical.fileKind) return null;
  const ownerCanonical = canonicalDirectoryByPath.get(location.owner) ?? location.owner.normalize("NFC");
  const fileName = basename(canonical.path);
  const stem = canonical.stem?.normalize("NFC") ?? "";
  if (!stem || GENERIC_SEMANTIC_STEMS.has(stem.toLocaleLowerCase("und"))) return ownerCanonical ? `${ownerCanonical}/${fileName}` : fileName;
  if (canonical.semanticDirectoryName) return ownerCanonical ? `${ownerCanonical}/${canonical.semanticDirectoryName}/${fileName}` : `${canonical.semanticDirectoryName}/${fileName}`;
  const semantic = matchDirectoryKind(stem, taxonomy, directoryKindByPath.get(location.owner));
  if (!semantic.kind) return null;
  const directoryName = `${semantic.kind.emoji}${stem}`.normalize("NFC");
  return ownerCanonical ? `${ownerCanonical}/${directoryName}/${fileName}` : `${directoryName}/${fileName}`;
}

function directoryHash(path: string, children: readonly Pick<TaxonomyInventoryEntry, "sourcePath" | "nodeKind" | "mode" | "contentHash">[]): string {
  const prefix = path ? `${path}/` : "";
  const rows = [...children]
    .sort((a, b) => Buffer.from(a.sourcePath).compare(Buffer.from(b.sourcePath)))
    .map((entry) => `${entry.nodeKind}\u0000${entry.mode ?? ""}\u0000${entry.sourcePath.slice(prefix.length)}\u0000${entry.contentHash}`);
  return sha256(rows.join("\u0000"));
}

function inventoryDigestOf(inventory: Omit<TaxonomyInventory, "inventoryDigest" | "repoRoot" | "taxonomyPath">): string {
  return sha256(canonicalJson(inventory));
}
//#endregion 📚️Inventory

//#region 🔗️References
interface CollectedTaxonomySourceAdmission {
  readonly inventory: TaxonomySourceInventory;
  readonly inputText: string;
}

interface RetainedSourceAdmission {
  readonly originInventory: TaxonomyInventory;
  readonly originalInputText: string;
  readonly sourceInventoryText: string;
  readonly repositoryAuthority: TransactionRepositoryAuthority;
  readonly originSourceTreeDigest: string;
  readonly originInventoryDigest: string;
}

type ReferenceInventorySourceAdmission =
  | Readonly<{ state: "captured"; retained: RetainedSourceAdmission }>
  | Readonly<{ state: "derived-unproven"; retained: RetainedSourceAdmission }>
  | Readonly<{ state: "uncaptured" }>;

interface ReferenceInventoryContext {
  readonly ticketDir?: string;
  readonly transactionRoots: readonly string[];
  readonly exactEvidencePaths: readonly string[];
  readonly sourceAdmission: ReferenceInventorySourceAdmission;
}

interface IncomingReferenceSnapshot {
  readonly paths: readonly string[];
  readonly coordinateRoots: readonly string[];
  readonly entries: readonly TaxonomyInventoryEntry[];
  readonly contents: ReadonlyMap<string, string>;
}

const referenceInventoryContexts = new WeakMap<TaxonomyInventory, ReferenceInventoryContext>();
const incomingReferenceSnapshots = new WeakMap<TaxonomyInventory, IncomingReferenceSnapshot>();

function inheritReferenceInventoryContext(source: TaxonomyInventory, target: TaxonomyInventory, transactionRoot?: string, exactEvidencePath?: string): TaxonomyInventory {
  const prior = referenceInventoryContexts.get(source);
  const priorAdmission = prior?.sourceAdmission;
  const sourceAdmission: ReferenceInventorySourceAdmission = priorAdmission?.state === "captured" || priorAdmission?.state === "derived-unproven"
    ? Object.freeze({ state: "derived-unproven" as const, retained: priorAdmission.retained })
    : Object.freeze({ state: "uncaptured" as const });
  referenceInventoryContexts.set(target, { ticketDir: prior?.ticketDir, transactionRoots: [...new Set([...(prior?.transactionRoots ?? []), ...(transactionRoot ? [transactionRoot] : [])])], exactEvidencePaths: [...new Set([...(prior?.exactEvidencePaths ?? []), ...(exactEvidencePath ? [exactEvidencePath] : [])])], sourceAdmission });
  return target;
}

/** 🗂️ Anchored owning-ticket-root prefix of a governed ticket path — see {@link historicalDocumentEvidence}. */
const HISTORICAL_TICKET_ROOT_PATTERN = /^\.🧬semio\/🦑️repo\/🎫️tickets\/🎆️\d{2}\/🌙️\d{2}\/☀️\d{2}\/[^/]+/u;

/** 🗂️ Anchored owning-directory prefix of a governed dev prompt-log path — see {@link historicalDocumentEvidence}. Unlike a ticket, a prompt log has no per-item lifecycle root below it: the population's own directory (`💬️prompts/`) is the whole boundary. */
const HISTORICAL_PROMPT_LOG_ROOT_PATTERN = /^\.🧬semio\/🦑️repo\/💬️prompts/u;

/** 🧱️ Literal basenames of every `fixedFilenameContracts` entry that marks a package-root manifest (`scope.kind === "package-root"`) — Cargo.toml, package.json, go.mod, … — derived from the schema so a future contract addition is protected automatically, never hardcoded. */
function packageRootManifestBasenames(taxonomy: LoadedTaxonomy): ReadonlySet<string> {
  return new Set(Object.values(taxonomy.discoverySchema.fixedFilenameContracts).filter((contract) => contract.scope.kind === "package-root").map((contract) => posix.basename(contract.pathPattern)));
}

/** 🗂️ True when `ancestor` or any directory between it and `path`'s owning historical-evidence boundary root (inclusive) directly contains a package-root manifest — i.e. `path` sits inside an embedded package boundary (a ticket-embedded crate, or — equally — a hypothetical package rooted directly under `💬️prompts/`) and must stay a live reference regardless of any historical-document population match. Only reached by {@link historicalDocumentEvidence} for a non-`ticket-report` population match: a `📓️` narrative report can never `open()` a file, so proximity to a manifest cannot make it live and this function is never consulted for one, however deeply the manifest shadows the rest of the directory. Deliberately not narrowed to the manifest's own ecosystem's source extensions: a co-located non-source file in a ticket-embedded package directory can be a genuinely live, load-bearing generator/build script with its own real repo-path references (verified concretely — `…/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/generate_w1_a_gltf_create_scene.mjs`, a co-located `.mjs` sibling of that ticket's `Cargo.toml`, joins `process.cwd()` with a real repo-relative path and writes generated source under it at run time), so "shares a directory with the manifest" is the safe, already-tested boundary — narrowing it by extension would silently stop protecting exactly this kind of file. */
function historicalEvidenceBoundaryOwns(path: string, boundaryRoot: string, taxonomy: LoadedTaxonomy, repoRoot: string): boolean {
  const manifests = packageRootManifestBasenames(taxonomy);
  for (let dir = posix.dirname(path); dir === boundaryRoot || dir.startsWith(`${boundaryRoot}/`); dir = posix.dirname(dir)) {
    let entries: readonly string[];
    try { entries = readdirSync(absolutePath(repoRoot, dir)); } catch { entries = []; }
    if (entries.some((name) => manifests.has(name))) return true;
    if (dir === boundaryRoot) break;
  }
  return false;
}

/** 🗂️ True when `path` is a whole-document historical-evidence population (`🔣️taxonomy.json#historicalDocumentEvidencePopulations`) — a ticket's own 📓️-slugged narrative report, ticket workspace (evidence snapshots, scratch scripts, working notes), a Cursor plan snapshot, or a developer's own `💬️prompts/` transcript — and therefore excluded from every reference-candidate scan (never a reference source, never rewritten, never blocks a move). Document kind alone is the discriminator, not ticket lifecycle status — and kind is exactly what separates the two negatives below: the `ticket-report` population is prose by construction (`^📓️.+\.md$`), so it is exempt outright once matched; every other population (`ticket-workspace` most of all, whose `^.+$` leaf admits any file, evidence snapshot, scratch script or generator alike) can be code and stays subject to the package-boundary negative, because a `Cargo.toml`/`package.json`/… beside it means the file may genuinely be live, load-bearing source or a build script, not narrative. Never overrides a real machine-read contract: refuses whenever `path` itself matches any `fixedFilenameContracts` pattern; for every non-`ticket-report` population, refuses whenever `path` also sits inside a directory (up to the ticket root, or up to `💬️prompts/` itself for a prompt log) that owns a package-root manifest — so a ticket-embedded Cargo/Node/Go package (or, equally, a hypothetical one dropped directly under `💬️prompts/`) keeps its ordinary live-reference treatment. */
export function historicalDocumentEvidence(path: string, taxonomy: LoadedTaxonomy, repoRoot: string): boolean {
  const populations = taxonomy.discoverySchema.historicalDocumentEvidencePopulations;
  const matches = (population: { directoryPattern: string; leafPattern: string }) => taxonomy.pathMatcher.matches(path, population.directoryPattern) && new RegExp(population.leafPattern, "u").test(posix.basename(path));
  const ticketReport = populations["ticket-report"];
  const isNarrativeReport = Boolean(ticketReport && matches(ticketReport));
  const populated = isNarrativeReport || Object.values(populations).some(matches);
  if (!populated) return false;
  if (Object.values(taxonomy.discoverySchema.fixedFilenameContracts).some((contract) => taxonomy.pathMatcher.matches(path, contract.pathPattern))) return false;
  if (isNarrativeReport) return true;
  const boundaryRoot = HISTORICAL_TICKET_ROOT_PATTERN.exec(path)?.[0] ?? HISTORICAL_PROMPT_LOG_ROOT_PATTERN.exec(path)?.[0];
  if (boundaryRoot && historicalEvidenceBoundaryOwns(path, boundaryRoot, taxonomy, repoRoot)) return false;
  return true;
}

function repositoryReferenceCandidatePaths(repoRoot: string, taxonomy: LoadedTaxonomy, context?: ReferenceInventoryContext, cancelFile?: string): readonly string[] {
  checkCancellation(repoRoot, cancelFile);
  const ignored = (path: string): boolean => isExcluded(path, taxonomy) || Boolean(context?.exactEvidencePaths.includes(path) || context?.transactionRoots.some((root) => path === root || path.startsWith(`${root}/`))) || historicalDocumentEvidence(path, taxonomy, repoRoot);
  const paths = new Set<string>();
  for (const row of gitRows(repoRoot, taxonomy)) if (!ignored(row.path)) paths.add(row.path);
  checkCancellation(repoRoot, cancelFile);
  for (const path of untrackedGitPaths(repoRoot, taxonomy)) if (!ignored(path)) paths.add(path);
  if (context?.ticketDir) for (const row of explicitTicketRows(repoRoot, context.ticketDir, taxonomy, undefined, cancelFile)) if (!ignored(row.path)) paths.add(row.path);
  return [...paths].sort(generatorPathCompare);
}

function referenceCoordinateRoots(repoRoot: string, paths: Iterable<string>, taxonomy: LoadedTaxonomy, cancelFile?: string, observe?: (path: string, stat: Stats | null, bytes?: Uint8Array) => void, progress?: TaxonomyPlanOptions["progress"], operation: TaxonomyProgress["operation"] = "plan"): readonly string[] {
  checkCancellation(repoRoot, cancelFile);
  const directories = new Set<string>(), roots = new Set<string>();
  let callingGitDirectory: string | undefined;
  for (const path of paths) for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) directories.add(parent);
  const ordered = [...directories].sort((left, right) => left.split("/").length - right.split("/").length || generatorPathCompare(left, right));
  if (ordered.length === 0) report(progress, operation, "incoming-coordinate-roots", 0, 0);
  for (const [index, path] of ordered.entries()) {
    report(progress, operation, "incoming-coordinate-roots", index, ordered.length, path);
    checkCancellation(repoRoot, cancelFile);
    if (isExcluded(path, taxonomy) || path.split("/").includes(".git") || ancestorReferenceCoordinateRoot(path, roots)) continue;
    const marker = assertLexicalInputOutsideOpaque(repoRoot, `${path}/.git`, "Reference repository marker"), stat = lstatOrNull(marker);
    if (!stat) { observe?.(`${path}/.git`, null); continue; }
    if (stat.isSymbolicLink()) throw new Error(`Reference repository marker is a symlink: ${path}/.git`);
    if (stat.isFile()) {
      const bytes = readFileSync(marker), content = bytes.toString("utf8"), target = content.match(/^gitdir:\s*([^\r\n]+)\r?\n?$/u)?.[1];
      observe?.(`${path}/.git`, stat, bytes);
      if (!target) continue;
      const gitdir = assertLexicalInputOutsideOpaque(repoRoot, resolve(dirname(marker), target), "Reference repository gitdir", true);
      const gitdirStat = lstatOrNull(gitdir);
      observe?.(relative(repoRoot, gitdir).replaceAll("\\", "/"), gitdirStat);
      if (!gitdirStat?.isDirectory()) throw new Error(`Reference repository gitdir is not a local directory: ${path}/.git`);
      callingGitDirectory ??= resolve(repoRoot, execFileSync("git", ["rev-parse", "--absolute-git-dir"], { cwd: repoRoot, encoding: "utf8" }).trim());
      if (resolve(gitdir) === callingGitDirectory) continue;
    } else {
      observe?.(`${path}/.git`, stat);
      if (!stat.isDirectory()) continue;
    }
    const parent = assertLexicalInputOutsideOpaque(repoRoot, path, "Reference repository owner", true);
    const result = spawnSync("git", ["rev-parse", "--show-toplevel"], { cwd: parent, encoding: "utf8" });
    if (result.status === 0 && resolve(result.stdout.trim()) === resolve(parent)) roots.add(path);
  }
  checkCancellation(repoRoot, cancelFile);
  if (ordered.length > 0) report(progress, operation, "incoming-coordinate-roots", ordered.length, ordered.length);
  return [...roots].sort((left, right) => right.split("/").length - left.split("/").length || generatorPathCompare(left, right));
}

function ancestorReferenceCoordinateRoot(path: string, roots: ReadonlySet<string>): string | undefined {
  for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) if (roots.has(parent)) return parent;
  return undefined;
}

function incomingReferenceLexicalAdmission(targets: Iterable<string>): (content: string) => boolean {
  const needles = new Set<string>();
  for (const target of targets) {
    const name = posix.basename(target).normalize("NFC");
    needles.add(name);
    const stem = name.replace(/\.[^/.]+(?:\.[^/.]+)*$/u, "");
    if (stem) needles.add(stem);
    if (name === "__init__.py") needles.add(posix.basename(posix.dirname(target)).normalize("NFC"));
  }
  const values = [...needles].filter(Boolean);
  return (content) => {
    if (/\\(?:u(?:\{|[a-f0-9])|x[a-f0-9])/iu.test(content) || /["'`]\.\.?(?:\/\.\.?)*\/?["'`]/u.test(content)) return true;
    const normalized = content.normalize("NFC");
    return values.some((needle) => normalized.includes(needle));
  };
}

function referenceEntry(repoRoot: string, path: string, taxonomy: LoadedTaxonomy): TaxonomyInventoryEntry | null {
  if (isExcluded(path, taxonomy)) throw new Error(`Reference candidate is opaque: ${path}`);
  const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Reference candidate"), stat = lstatOrNull(absolute);
  if (!stat) return null;
  const nodeKind = stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : "file";
  if (nodeKind === "file" && !stat.isFile()) throw new Error(`Reference candidate is not a regular file: ${path}`);
  const target = nodeKind === "symlink" ? readlinkSync(absolute) : undefined;
  const bytes = nodeKind === "directory" ? Buffer.from("directory") : nodeKind === "symlink" ? Buffer.from(target!) : readFileSync(absolute);
  return { sourcePath: path, normalizedPath: path, nodeKind, ownerId: ownerId(path), areaId: areaId(path), fileKind: null, semanticStem: null, contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: nodeKind === "directory" ? 0 : bytes.byteLength, ...(target === undefined ? {} : { symlinkTarget: target }), referencesIn: [], referencesOut: [], violations: [] };
}

function* referenceCandidatesWithProgress(paths: readonly string[], operation: TaxonomyProgress["operation"], progress?: TaxonomyPlanOptions["progress"]): Generator<readonly [number, string]> {
  report(progress, operation, "incoming-candidates", 0, paths.length);
  for (const [index, path] of paths.entries()) {
    try {
      yield [index, path];
    } finally {
      report(progress, operation, "incoming-candidates", index + 1, paths.length, path);
    }
  }
}

function incomingReferenceSnapshot(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy, options: Pick<TaxonomyPlanOptions, "cancelFile" | "progress"> = {}): IncomingReferenceSnapshot {
  checkCancellation(inventory.repoRoot, options.cancelFile);
  const cached = incomingReferenceSnapshots.get(inventory);
  if (cached) return cached;
  const changing = new Set(inventory.entries.filter((entry) => entry.sourcePath !== entry.normalizedPath).map((entry) => entry.sourcePath));
  const context = referenceInventoryContexts.get(inventory);
  const paths = changing.size > 0 ? repositoryReferenceCandidatePaths(inventory.repoRoot, taxonomy, context, options.cancelFile) : [];
  const knownPaths = new Set([...paths, ...inventory.entries.map((entry) => entry.sourcePath)]);
  for (const path of paths) for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) knownPaths.add(parent);
  validateObservedFrozenEvidenceNodes(inventory.repoRoot, knownPaths, taxonomy);
  const coordinateRoots = referenceCoordinateRoots(inventory.repoRoot, paths, taxonomy, options.cancelFile, undefined, options.progress, "plan");
  const known = referencePathIndex(knownPaths, inventory.repoRoot, coordinateRoots, undefined, options.cancelFile, changing), admitted = new Set(inventory.entries.map((entry) => entry.sourcePath));
  const admitsText = incomingReferenceLexicalAdmission(changing);
  const entries: TaxonomyInventoryEntry[] = [], contents = new Map<string, string>();
  for (const [index, path] of referenceCandidatesWithProgress(paths, "plan", options.progress)) {
    checkCancellation(inventory.repoRoot, options.cancelFile);
    if (admitted.has(path)) continue;
    const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, path, "Incoming reference candidate"), stat = lstatOrNull(absolute);
    if (!stat || stat.isDirectory()) continue;
    if (stat.isSymbolicLink()) {
      const target = readlinkSync(absolute), logical = logicalRepositorySymlinkTargetPath(inventory.repoRoot, path, target);
      if (logical && !isExcluded(logical, taxonomy) && projectedPath(logical, inventory.entries) !== logical) entries.push(referenceEntry(inventory.repoRoot, path, taxonomy)!);
    } else if (stat.isFile() && textualPath(path)) {
      const bytes = readFileSync(absolute), content = bytes.toString("utf8");
      frozenEvidenceCoordinateAuthority(path, bytes, taxonomy);
      if (!admitsText(content)) continue;
      report(options.progress, "plan", "incoming-parse", index + 1, paths.length, path);
      const relevant = referenceTokensIncludingUnsupported(path, content, known).some((token) => {
        if (token.unsupportedReason && token.physicalTargets?.some((target) => changing.has(target))) return true;
        const target = resolveReferenceTokenPath(path, token, known);
        return target !== null && changing.has(target) && !isFrozenSourceCoordinateToken(path, bytes, token, target, taxonomy, inventory.repoRoot);
      });
      if (relevant) {
        const current = lstatSync(assertLexicalInputOutsideOpaque(inventory.repoRoot, path, "Incoming reference preimage", true));
        if (!current.isFile() || current.mode !== stat.mode || current.size !== stat.size || current.mtimeMs !== stat.mtimeMs || bytes.byteLength !== stat.size) throw new Error(`Incoming reference changed during its snapshot: ${path}`);
        entries.push({ sourcePath: path, normalizedPath: path, nodeKind: "file", ownerId: ownerId(path), areaId: areaId(path), fileKind: null, semanticStem: null, contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength, referencesIn: [], referencesOut: [], violations: [] });
        contents.set(path, content);
      }
    }
  }
  checkCancellation(inventory.repoRoot, options.cancelFile);
  const result = { paths: [...knownPaths].sort(generatorPathCompare), coordinateRoots, entries: entries.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), contents };
  incomingReferenceSnapshots.set(inventory, result);
  return result;
}

interface ReferenceToken {
  readonly adapter: TaxonomyReferenceAdapter;
  readonly structuredLocation: string;
  readonly start: number;
  readonly end: number;
  readonly value: string;
  readonly targetValues?: readonly string[];
  readonly physicalTargets?: readonly string[];
  readonly physicalInterpretation?: "rust-finite-manifest-targets";
  readonly rewriteKind?: "rust-mod" | "rust-path-join" | "python-entrypoint" | "artifact-uri" | "projection-prose" | "structural-projection" | "path-prefix" | "artifact-catalog-glob" | "artifact-catalog-prose" | "exact-owner-reference";
  readonly rewriteData?: Readonly<Record<string, string>>;
  readonly unsupportedReason?: string;
}

let indexedLineContent = "";
let indexedLineStarts: readonly number[] = [0];

function lineLocation(content: string, start: number, label: string): string {
  if (indexedLineContent !== content) {
    const starts = [0];
    for (let index = content.indexOf("\n"); index >= 0; index = content.indexOf("\n", index + 1)) starts.push(index + 1);
    indexedLineContent = content;
    indexedLineStarts = starts;
  }
  let low = 0;
  let high = indexedLineStarts.length;
  while (low < high) {
    const middle = (low + high) >>> 1;
    if (indexedLineStarts[middle] <= start) low = middle + 1;
    else high = middle;
  }
  const line = Math.max(1, low);
  const column = start - indexedLineStarts[line - 1] + 1;
  return `${label}:${line}:${column}@${start}`;
}

function regexTokens(content: string, adapter: TaxonomyReferenceAdapter, label: string, patterns: readonly RegExp[]): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const pattern of patterns) {
    for (const match of content.matchAll(pattern)) {
      const value = match[1];
      if (typeof value !== "string" || match.index === undefined) continue;
      const relativeIndex = match[0].indexOf(value);
      const start = match.index + relativeIndex;
      rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + value.length, value });
    }
  }
  return rows;
}

function argumentTokens(content: string, fragment: string, fragmentStart: number, adapter: TaxonomyReferenceAdapter, label: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const match of fragment.matchAll(/"([^"]+)"|'([^']+)'|([^\s()[\],;]+)/gu)) {
    if (match.index === undefined) continue;
    const value = match[1] ?? match[2] ?? match[3];
    if (!value || /^(?:=>|PUBLIC|PRIVATE|INTERFACE|EXCLUDE_FROM_ALL)$/u.test(value)) continue;
    const inner = match[0].indexOf(value);
    const start = fragmentStart + match.index + inner;
    rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + value.length, value });
  }
  return rows;
}

function embeddedArgumentTokens(content: string, value: string, valueStart: number, adapter: TaxonomyReferenceAdapter, label: string): ReferenceToken[] {
  if (!/\s|(?:^|\s)--?[\w-]+=|\$\{(?:workspaceFolder|workspaceRoot)\}/u.test(value)) return [];
  const rows: ReferenceToken[] = [];
  for (const match of value.matchAll(/[^\s"'`]+/gu)) {
    if (match.index === undefined) continue;
    let candidate = match[0].replace(/^[[(]+|[\]),;]+$/gu, "");
    let offset = match[0].indexOf(candidate);
    const assignment = candidate.match(/^--?[\w-]+=(.+)$/u);
    if (assignment) {
      offset += candidate.indexOf(assignment[1]);
      candidate = assignment[1];
    }
    const workspace = candidate.match(/^\$\{(?:workspaceFolder|workspaceRoot)\}\/(.+)$/u);
    if (workspace) {
      offset += candidate.indexOf(workspace[1]);
      candidate = workspace[1];
    }
    if (!candidate || /^(?:bun|node|python|python3|go|cargo|nx|run|test|build)$/u.test(candidate)) continue;
    const start = valueStart + match.index + offset;
    rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + candidate.length, value: candidate });
  }
  for (const match of value.matchAll(/(?:\.\.?\/|\/)[^\s\\"'`()\],;]+/gu)) {
    if (match.index === undefined) continue;
    const start = valueStart + match.index;
    rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + match[0].length, value: match[0] });
  }
  return [...new Map(rows.map((entry) => [`${entry.start}\u0000${entry.end}\u0000${entry.value}`, entry])).values()].sort((left, right) => left.start - right.start || left.value.localeCompare(right.value));
}

interface MutationStructuralPath {
  readonly value: string;
  readonly start: number;
  readonly standard: string;
  readonly subset: string;
  readonly mutation: string;
  readonly scenario: string;
  readonly suffix: string;
}

const MUTATION_SOURCE_TEST_PREFIX = "🏅️standards/🔖️([^/\\s\"'`|]+)\\/🪆️subsets/✳️([^/\\s\"'`|]+)\\/🧬️schema/🧬️mutations\\/([^/\\s\"'`|]+(?:/[^/\\s\"'`|]+)?)\\/🧪️tests\\/";
const MUTATION_SOURCE_STRUCTURE = `${MUTATION_SOURCE_TEST_PREFIX}([^/\\s\"'\u0060|]+)(\\/[^\\s\"'\u0060|)>}\\]]+)?`;

function artifactRootForPath(path: string): string | null {
  const segments = normalizeRelative(path).split("/");
  const index = segments.findIndex((segment) => emojiFold(segment) === emojiFold("🗿️artifacts"));
  if (index >= 0 && index + 1 < segments.length) return segments.slice(0, index + 2).join("/");
  const standards = segments.findIndex((segment) => emojiFold(segment) === emojiFold("🏅️standards"));
  return standards > 0 ? segments.slice(0, standards).join("/") : null;
}

function mutationStructuralPaths(content: string, fragmentStart = 0): readonly MutationStructuralPath[] {
  const rows: MutationStructuralPath[] = [];
  const pattern = new RegExp(MUTATION_SOURCE_STRUCTURE, "gu");
  for (const match of content.matchAll(pattern)) {
    if (match.index === undefined) continue;
    rows.push({ value: match[0], start: fragmentStart + match.index, standard: match[1], subset: match[2], mutation: match[3], scenario: match[4], suffix: match[5] ?? "" });
  }
  return rows;
}

function canonicalProjectionSuffix(suffix: string): string {
  const segments = suffix.split("/");
  const name = segments.at(-1) ?? "";
  const leading = splitLeadingEmoji(name);
  if (leading.emoji && /^component\.[a-z0-9.]+$/u.test(leading.rest)) segments[segments.length - 1] = `${leading.emoji}.${leading.rest.slice("component.".length)}`;
  return segments.join("/");
}

function projectionKey(artifactRoot: string, standard: string, subset: string): string {
  return `${artifactRoot}\u0000${standard}\u0000${subset}`;
}

/** 🧭️ Limits structural fallback to complete, exact active owners; physical references use their destination map. */
function mutationReferenceProjectionState(token: ReferenceToken, target: string | null, activeKeys: ReadonlySet<string>, scope?: string): "active" | "inactive" | "unproven" {
  if (target !== null) return "inactive";
  const owner = token.rewriteData?.artifactRoot, profile = token.rewriteData?.projectionProfile;
  if (!owner || !profile) return "inactive";
  const active = profile === "*" ? [...activeKeys].some((key) => key.startsWith(`${owner}\u0000`)) : activeKeys.has(`${owner}\u0000${profile}`);
  if (!active) return "inactive";
  if (token.targetValues?.length) return "unproven";
  const parts = profile.split("\u0000");
  if (profile !== "*" && (parts.length !== 2 || parts.some((part) => !part || part.includes("/")))) return "unproven";
  const required = profile === "*" ? owner : `${owner}/🏅️standards/🔖️${parts[0]}/🪆️subsets/✳️${parts[1]}/🧬️schema/🧬️mutations`;
  return !scope || required === scope || required.startsWith(`${scope}/`) ? "active" : "unproven";
}

function projectedStructuralValue(row: MutationStructuralPath): string {
  const scenario = splitLeadingEmoji(row.scenario).emoji ? row.scenario : `🧪️${row.scenario}`;
  return `🪆️tests${row.standard}-${row.subset}/${row.mutation}/${scenario}${canonicalProjectionSuffix(row.suffix)}`.normalize("NFC");
}

function structuralProjectionToken(content: string, row: MutationStructuralPath, adapter: TaxonomyReferenceAdapter, label: string, artifactRoot: string | null, prefix = ""): ReferenceToken {
  const value = `${prefix}${row.value}`;
  const start = row.start - prefix.length;
  const target = artifactRoot && !/[<>]/u.test(row.value) ? `${artifactRoot}/${row.value}` : undefined;
  return {
    adapter,
    structuredLocation: label.startsWith("/") ? `${label}@${start}` : lineLocation(content, start, label),
    start,
    end: start + value.length,
    value,
    targetValues: target ? [target] : undefined,
    rewriteKind: prefix === "asset://" ? "artifact-uri" : "projection-prose",
    rewriteData: {
      newValue: `${prefix}${projectedStructuralValue(row)}`,
      projectionKey: artifactRoot ? projectionKey(artifactRoot, row.standard, row.subset) : "",
      projectionProfile: `${row.standard}\u0000${row.subset}`,
      artifactRoot: artifactRoot ?? "",
    },
  };
}

function structuralTokensInFragment(content: string, fragment: string, fragmentStart: number, adapter: TaxonomyReferenceAdapter, label: string, artifactRoot: string | null): readonly ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const structural of mutationStructuralPaths(fragment, fragmentStart)) {
    const localStart = structural.start - fragmentStart;
    const before = fragment.slice(0, localStart);
    const prefix = before.endsWith("asset://") ? "asset://" : before.match(/(?:(?:\.\.\/|\.\/)+)$/u)?.[0] ?? "";
    rows.push(structuralProjectionToken(content, structural, adapter, prefix === "asset://" && adapter === "gherkin" ? "gherkin" : label, artifactRoot, prefix));
  }
  return rows;
}

function jsonTokens(path: string, content: string, adapter: "json" | "jsonc"): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  let ordinal = 0;
  let artifactRoot: string | null | undefined;
  const embeddedArgv = /(?:^|\/)(?:launch(?:\.seed)?\.jsonc?|tasks\.json|project\.json|package\.json)$/iu.test(path);
  for (const match of content.matchAll(/"((?:\\.|[^"\\])*)"/g)) {
    if (match.index === undefined) continue;
    const tail = content.slice(match.index + match[0].length).match(/^\s*/)?.[0].length ?? 0;
    const key = content[match.index + match[0].length + tail] === ":";
    let value: string;
    try {
      value = JSON.parse(match[0]) as string;
    } catch {
      continue;
    }
    const raw = match[1];
    const start = match.index + 1;
    if (raw === value) rows.push({ adapter, structuredLocation: `${key ? "/@key" : "/@value"}[${ordinal++}]@${start}`, start, end: start + raw.length, value });
    const workspaceGlob = !key && raw === value ? value.match(/^\{workspaceRoot\}\/(.+?)(\/\*\*\/\*[^/]*)$/u) : null;
    if (workspaceGlob) rows.push({ adapter, structuredLocation: `${key ? "/@key" : "/@value"}[${Math.max(0, ordinal - 1)}]/workspace-glob@${start}`, start, end: start + raw.length, value, targetValues: [workspaceGlob[1]], rewriteKind: "path-prefix", rewriteData: { prefix: "{workspaceRoot}/", suffix: workspaceGlob[2] } });
    if (!key && raw !== value && /^\{workspaceRoot\}\/.+\/\*\*/u.test(value)) rows.push({ adapter, structuredLocation: `/@value[${Math.max(0, ordinal - 1)}]/workspace-glob@${start}`, start, end: start + raw.length, value: raw, rewriteKind: "path-prefix", unsupportedReason: "Escaped workspace projection glob has no proven decoded-to-raw offset map" });
    /** 🧷️ An Nx `{workspaceRoot}/<path>` value with no glob wildcard names one concrete file; `{projectRoot}/…` is left alone since it only ever appears as a `**\/*` glob in this repo. */
    const workspaceFile = !key && raw === value ? value.match(/^\{workspaceRoot\}\/([^*]+)$/u) : null;
    if (workspaceFile) rows.push({ adapter, structuredLocation: `${key ? "/@key" : "/@value"}[${Math.max(0, ordinal - 1)}]/workspace-file@${start}`, start, end: start + raw.length, value, targetValues: [workspaceFile[1]], rewriteKind: "path-prefix", rewriteData: { prefix: "{workspaceRoot}/", suffix: "" } });
    if (!key && raw !== value && /^\{workspaceRoot\}\/[^*]+$/u.test(value)) rows.push({ adapter, structuredLocation: `/@value[${Math.max(0, ordinal - 1)}]/workspace-file@${start}`, start, end: start + raw.length, value: raw, rewriteKind: "path-prefix", unsupportedReason: "Escaped workspace projection file reference has no proven decoded-to-raw offset map" });
    if (!key && raw === value) {
      if (artifactRoot === undefined) artifactRoot = artifactRootForPath(path);
      rows.push(...structuralTokensInFragment(content, raw, start, adapter, `/@value[${Math.max(0, ordinal - 1)}]/prose`, artifactRoot));
    }
    if (!key && raw !== value && mutationStructuralPaths(value).length > 0) rows.push({ adapter, structuredLocation: `/@value[${Math.max(0, ordinal - 1)}]/prose@${start}`, start, end: start + raw.length, value: raw, unsupportedReason: "Escaped JSON projection prose has no proven decoded-to-raw offset map" });
    if (!key && embeddedArgv) rows.push(...embeddedArgumentTokens(content, raw, start, adapter, "embedded-argv"));
  }
  return rows;
}

function tomlTokens(path: string, content: string): ReferenceToken[] {
  const adapter: TaxonomyReferenceAdapter = "toml";
  const rows: ReferenceToken[] = [];
  for (const match of content.matchAll(/"([^"\r\n]+)"|'([^'\r\n]+)'/gu)) {
    if (match.index === undefined) continue;
    const value = match[1] ?? match[2];
    const start = match.index + match[0].indexOf(value);
    const entrypoint = value.match(/^([A-Za-z_]\w*(?:\.[A-Za-z_]\w*)+):([A-Za-z_]\w*)$/u);
    const prefix = content.slice(0, start);
    const section = [...prefix.matchAll(/^\s*\[([^\]]+)\]\s*$/gmu)].at(-1)?.[1];
    const lineStart = prefix.lastIndexOf("\n") + 1;
    const key = content.slice(lineStart, start).match(/^\s*([A-Za-z0-9_.-]+)\s*=\s*["']/u)?.[1];
    const label = section && key ? `${section}.${key}` : "toml-string";
    rows.push(entrypoint
      ? { adapter, structuredLocation: lineLocation(content, start, "python-entrypoint"), start, end: start + value.length, value, targetValues: [entrypoint[1]], rewriteKind: "python-entrypoint", rewriteData: { suffix: `:${entrypoint[2]}` } }
      : { adapter, structuredLocation: lineLocation(content, start, label), start, end: start + value.length, value });
  }
  return rows;
}

/** 🪡️ A `rust-comment-path` backtick span that embeds a real path inside a longer illustrative string
 * (a shell command example, e.g. `` `grep -n "pattern" 🧰️framework/…/component.rs` ``) still names one
 * real, rewritable target — the LAST whitespace-delimited word, which is what the generic
 * `unsupportedReferenceTokens` fallback already extracts and resolves independently, flagging it
 * `reference-syntax-unsupported` because this scanner previously only ever offered the WHOLE span as
 * a token. Extracting the same trailing word here, with its true narrower start/end, lets that
 * fallback candidate's span/value match and be properly rewritten in place instead. */
function rustTokens(path: string, content: string, index?: ReferencePathIndex): ReferenceToken[] {
  // 🪪️ `owner_file: "<repo-relative path>"` is `bounded_first_step_tool_proofs!`'s self-declared
  // authority field (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`) — 36 editor
  // `🦀️.rs` files each name their OWN path here as a plain literal, never through `.join()`,
  // so none of the join-chain detectors above ever see it; mirrors the existing `#[path=…]`/`include!`
  // bare-literal style rather than inventing a second one.
  const rows = regexTokens(content, "rust", "rust-string-path", [/#\s*\[\s*path\s*=\s*"([^"]+)"/gu, /\b(?:include|include_str|include_bytes)!\s*\(\s*"([^"]+)"/gu, /\bowner_file\s*:\s*"([^"]+)"/gu]);
  if (index) rows.push(...rustManifestReferenceTokens(path, content, index));
  for (const message of inspectRustAssertionMessageSpans(content)) for (const token of unsupportedReferenceTokens(message.value, "rust")) {
    const start = message.start + token.start;
    rows.push({ adapter: "rust", structuredLocation: lineLocation(content, start, `rust-assertion-message:${message.macroName}`), start, end: message.start + token.end, value: token.value });
  }
  for (const match of content.matchAll(/\.join\(\s*"([^"]*🖼️assets\/🏗️modelDefinitions)"\s*\)/gu)) {
    if (match.index === undefined) continue;
    const start = match.index + match[0].indexOf(match[1]);
    rows.push({ adapter: "rust", structuredLocation: lineLocation(content, start, "artifact-catalog-root-join"), start, end: start + match[1].length, value: match[1], rewriteKind: "artifact-catalog-prose", rewriteData: { form: "relative-root" } });
  }
  for (const match of content.matchAll(/^([ \t]*)((?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;)/gmu)) {
    if (match.index === undefined) continue;
    const statement = match[2];
    const start = match.index + match[1].length;
    const name = match[3];
    rows.push({
      adapter: "rust",
      structuredLocation: lineLocation(content, start, "rust-mod"),
      start,
      end: start + statement.length,
      value: statement,
      targetValues: [`./${name}.rs`, `./${name}/mod.rs`],
      rewriteKind: "rust-mod",
      rewriteData: { indentation: match[1], declaration: statement },
    });
  }
  for (const match of content.matchAll(/(?:^|\n)[ \t]*(?:(?:\/\/\/)|(?:\/\/!)|(?:\/\/))([^\r\n]*)/gu)) {
    if (match.index === undefined) continue;
    const fragment = match[1];
    const start = match.index + match[0].indexOf(fragment);
    rows.push(...structuralTokensInFragment(content, fragment, start, "rust", "rust-comment", artifactRootForPath(path)));
    for (const quoted of fragment.matchAll(/`([^`]+)`/gu)) {
      if (quoted.index === undefined) continue;
      const raw = quoted[1], rawStart = start + quoted.index + quoted[0].indexOf(raw);
      const segment = /\s/u.test(raw) ? raw.match(/(?:^|\s)([^\s]*[/.][^\s]*)$/u) : null;
      const value = segment ? segment[1] : raw;
      const tokenStart = segment ? rawStart + raw.lastIndexOf(segment[1]) : rawStart;
      if (!/[/.]/u.test(value)) continue;
      rows.push({ adapter: "rust", structuredLocation: lineLocation(content, tokenStart, "rust-comment-path"), start: tokenStart, end: tokenStart + value.length, value });
    }
    const catalog = fragment.match(/(🖼️assets\/🏗️modelDefinitions\/\*\/🎬️interactions\/\*\.json)/u);
    if (catalog) {
      const tokenStart = start + fragment.indexOf(catalog[1]);
      rows.push({ adapter: "rust", structuredLocation: lineLocation(content, tokenStart, "artifact-catalog-comment"), start: tokenStart, end: tokenStart + catalog[1].length, value: catalog[1], rewriteKind: "artifact-catalog-prose", rewriteData: { form: "interaction-glob" } });
    }
  }
  return rows;
}

function pythonTokens(path: string, content: string): ReferenceToken[] {
  const rows = regexTokens(content, "python", "python-reference", [/^\s*from\s+([\w.]+)\s+import\s+/gmu, /^\s*import\s+([\w.]+)(?:\s+as\s+\w+)?\s*$/gmu, /\b(?:open|Path|joinpath|files|read_text|read_bytes)\s*\(\s*["']([^"']+)["']/gu, /__file__[^\r\n]*?\/\s*["']([^"']+)["']/gu]);
  for (const match of content.matchAll(/^\s*([A-Z][A-Z0-9_]*VECTOR_ROOT|VECTOR_ROOT)\s*=\s*["'](asset:\/\/🏅️standards\/🔖️([^/"']+)\/🪆️subsets\/✳️([^/"']+)\/🧬️schema\/🧬️mutations)["']/gmu)) {
    if (match.index === undefined) continue;
    const value = match[2];
    const start = match.index + match[0].indexOf(value);
    rows.push({ adapter: "python", structuredLocation: lineLocation(content, start, `python-string:${match[1]}`), start, end: start + value.length, value, rewriteKind: "structural-projection", rewriteData: { newValue: `asset://🧪️tests/🪆️${match[3]}-${match[4]}`, projectionKey: "", projectionProfile: `${match[3]}\u0000${match[4]}`, artifactRoot: artifactRootForPath(path) ?? "" } });
  }
  for (const match of content.matchAll(/^\s*(stem)\s*=\s*["'](%s\/%s\/🧪️tests\/%s)["']\s*%/gmu)) {
    if (match.index === undefined) continue;
    const value = match[2];
    const start = match.index + match[0].indexOf(value);
    rows.push({ adapter: "python", structuredLocation: lineLocation(content, start, `python-format:${match[1]}`), start, end: start + value.length, value, rewriteKind: "structural-projection", rewriteData: { newValue: "%s/%s/🧪️%s", projectionKey: "", projectionProfile: "*", artifactRoot: artifactRootForPath(path) ?? "" } });
  }
  return rows;
}

/** 🥒️ Finds single-backtick inline-code spans inside a Gherkin `Feature:`'s free-form description prose — the lines between the `Feature:` header and the first tag/keyword that opens a `Background`/`Scenario`/`Rule`, never inside a step. Mirrors markdownSourceCoordinateSpans's per-line backtick-run discipline (escaped backticks, longest-matching same-length nested runs, no span crossing a line or a blank-line paragraph break) so a description path reads with the same rigor as a frozen Markdown one, but stays live and rewritable here — see 📓️goal-gherkin-report.md in this ticket. */
function gherkinDescriptionInlineCodeSpans(content: string): readonly Readonly<{ start: number; end: number }>[] {
  const feature = content.match(/^Feature:[^\r\n]*(?:\r\n|\r|\n)?/mu);
  if (!feature || feature.index === undefined) return [];
  const descriptionStart = feature.index + feature[0].length;
  const boundary = content.slice(descriptionStart).match(/^[ \t]*(?:@\S|Background:|Scenario(?: Outline| Template)?:|Rule:)/mu);
  const descriptionEnd = boundary?.index === undefined ? content.length : descriptionStart + boundary.index;
  const rows: { start: number; end: number }[] = [];
  let inline = 0;
  for (const match of content.slice(descriptionStart, descriptionEnd).matchAll(/[^\r\n]*(?:\r\n|\r|\n|$)/gu)) {
    const line = match[0].replace(/(?:\r\n|\r|\n)$/u, ""), offset = descriptionStart + match.index!;
    if (!line.trim()) { inline = 0; continue; }
    const runs = [...line.matchAll(/`+/gu)];
    for (let index = 0; index < runs.length; index++) {
      const run = runs[index], start = run.index!;
      if (inline) { if (run[0].length === inline) inline = 0; continue; }
      if ((line.slice(0, start).match(/\\+$/u)?.[0].length ?? 0) % 2) continue;
      const close = runs.findIndex((candidate, candidateIndex) => candidateIndex > index && candidate[0].length === run[0].length);
      if (close < 0) { inline = run[0].length; continue; }
      if (run[0].length === 1) rows.push({ start: offset + start + 1, end: offset + runs[close].index! });
      index = close;
    }
  }
  return rows;
}

function gherkinTokens(path: string, content: string): readonly ReferenceToken[] {
  const rows = [...structuralTokensInFragment(content, content, 0, "gherkin", "gherkin-description", artifactRootForPath(path))];
  for (const span of gherkinDescriptionInlineCodeSpans(content)) {
    if (rows.some((token) => token.start <= span.start && token.end >= span.end)) continue;
    const value = content.slice(span.start, span.end);
    rows.push({ adapter: "gherkin", structuredLocation: lineLocation(content, span.start, "gherkin-description-inline-code"), start: span.start, end: span.end, value });
  }
  return rows;
}

export type TicketImportantProseReference = Readonly<{ start: number; end: number; structuredLocation: string; value: "📌️important.md" }>;

/** 📌️ Admits only the exact ticket-leaf prose form used by governed handoff generators. */
export function ticketImportantProseReferenceAuthority(content: string): readonly TicketImportantProseReference[] {
  const rows: TicketImportantProseReference[] = [];
  for (const match of content.matchAll(/\bsee\s+(📌️important\.md)(?=$|[\s"'`,.;:!?)}\]])/gu)) {
    if (match.index === undefined) continue;
    const start = match.index + match[0].indexOf(match[1]);
    rows.push({ start, end: start + match[1].length, structuredLocation: lineLocation(content, start, "typescript-ticket-leaf-prose"), value: "📌️important.md" });
  }
  return rows;
}

/** 📖️ Resolves unescaped single-backtick paths only inside the leading JSDoc block. */
export function typescriptLeadingDocumentationReferenceAuthority(content: string): readonly Readonly<{ start: number; end: number; value: string; structuredLocation: string }>[] {
  const start = content.match(/^\s*\/\*\*/u)?.[0].length;
  if (start === undefined) return [];
  const end = content.indexOf("*/", start);
  if (end < 0) return [];
  const rows: { start: number; end: number; value: string; structuredLocation: string }[] = [];
  for (const match of content.slice(start, end).matchAll(/(?<![\\`])`([^`\\\r\n]+)`(?!`)/gu)) {
    if (!/[/.]/u.test(match[1]) || /\s/u.test(match[1])) continue;
    const offset = start + match.index! + 1;
    rows.push({ start: offset, end: offset + match[1].length, value: match[1], structuredLocation: lineLocation(content, offset, "typescript-leading-jsdoc-path") });
  }
  return rows;
}

/** 💬️ Resolves unescaped single-backtick paths inside EVERY comment in a TypeScript file — every `/* … *\/`/`/** … *\/` block (not only the leading JSDoc {@link typescriptLeadingDocumentationReferenceAuthority} already covers) and every whole-line `//` comment — mirroring `rustTokens`'s unconditional `rust-comment-path` scan so both languages give a moved file's design-rationale prose the same live-reference treatment. A trailing end-of-line `//` after real code is deliberately NOT scanned (mirrors the Rust scanner's own whole-line restriction), which keeps this from ever touching a `//` inside a string literal on a code line. */
function typescriptCommentPathReferenceAuthority(content: string): readonly Readonly<{ start: number; end: number; value: string; structuredLocation: string }>[] {
  const rows: { start: number; end: number; value: string; structuredLocation: string }[] = [];
  for (const block of content.matchAll(/\/\*[\s\S]*?\*\//gu)) {
    if (block.index === undefined) continue;
    for (const match of block[0].matchAll(/(?<![\\`])`([^`\\\r\n]+)`(?!`)/gu)) {
      if (!/[/.]/u.test(match[1]) || /\s/u.test(match[1])) continue;
      const offset = block.index + match.index! + 1;
      rows.push({ start: offset, end: offset + match[1].length, value: match[1], structuredLocation: lineLocation(content, offset, "typescript-comment-block-path") });
    }
  }
  for (const match of content.matchAll(/(?:^|\n)[ \t]*\/\/([^\r\n]*)/gu)) {
    if (match.index === undefined) continue;
    const fragment = match[1], fragmentStart = match.index + match[0].indexOf(fragment);
    for (const quoted of fragment.matchAll(/`([^`]+)`/gu)) {
      if (quoted.index === undefined || !/[/.]/u.test(quoted[1]) || /\s/u.test(quoted[1])) continue;
      const start = fragmentStart + quoted.index + quoted[0].indexOf(quoted[1]);
      rows.push({ start, end: start + quoted[1].length, value: quoted[1], structuredLocation: lineLocation(content, start, "typescript-comment-line-path") });
    }
  }
  return rows;
}

/** 🚧️ Resolves quoted repository-relative path literals inside `.dependency-cruiser.cjs`'s own boundary arrays (e.g. `allowed…Targets`/`forbidden…Targets`) — a closed, purpose-built carve-out (matched by exact basename, never generalized to arbitrary TypeScript/JS arrays) since these arrays exist ONLY to enumerate real repo paths for architectural-boundary comparison, not to be read from disk, so no naming or trailing-usage heuristic like {@link typescriptPathCollectionReferenceAuthority} applies. Every element without a `/` (`"fs"`, `"node:path"`, bare package names) is skipped so only genuinely path-shaped literals become candidates; a value that never resolves to a real repo path is simply never rewritten. */
function dependencyCruiserBoundaryReferenceAuthority(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const array of content.matchAll(/\bconst\s+[A-Za-z_$][\w$]*\s*=\s*\[([\s\S]*?)\]/gu)) {
    if (array.index === undefined) continue;
    const fragmentStart = array.index + array[0].indexOf(array[1]);
    for (const element of regexTokens(array[1], "typescript", "dependency-cruiser-boundary", [/["']([^"']+)["']/gu])) {
      if (!element.value.includes("/")) continue;
      rows.push({ ...element, start: fragmentStart + element.start, end: fragmentStart + element.end, structuredLocation: lineLocation(content, fragmentStart + element.start, "dependency-cruiser-boundary") });
    }
  }
  return rows;
}

interface TypeScriptCollectionToken {
  readonly text: string;
  readonly start: number;
  readonly end: number;
  readonly kind: "identifier" | "string" | "template" | "number" | "punctuation" | "regex";
  readonly group: number;
  readonly scope: number;
}

interface TypeScriptCollectionSyntax {
  readonly tokens: readonly TypeScriptCollectionToken[];
  readonly pairs: ReadonlyMap<number, number>;
}

/** 🔬️ Reads bounded source tokens and balanced delimiters without evaluating code or decoding editable spans. */
function typescriptCollectionSyntax(content: string): TypeScriptCollectionSyntax | null {
  const tokens: TypeScriptCollectionToken[] = [], pairs = new Map<number, number>(), stack: number[] = [];
  let cursor = 0;
  while (cursor < content.length) {
    if (/\s/u.test(content[cursor])) { cursor++; continue; }
    if (content.startsWith("//", cursor)) { const end = content.indexOf("\n", cursor + 2); cursor = end < 0 ? content.length : end + 1; continue; }
    if (content.startsWith("/*", cursor)) { const end = content.indexOf("*/", cursor + 2); if (end < 0) return null; cursor = end + 2; continue; }
    const start = cursor, first = content[cursor];
    let kind: TypeScriptCollectionToken["kind"] = "punctuation";
    if (first === '"' || first === "'" || first === "\u0060") {
      kind = first === "\u0060" ? "template" : "string";
      cursor++;
      while (cursor < content.length && content[cursor] !== first) {
        if (content[cursor] === "\\") { cursor += 2; continue; }
        if (kind === "string" && /[\r\n]/u.test(content[cursor])) return null;
        cursor++;
      }
      if (cursor >= content.length) return null;
      cursor++;
    } else if (/[A-Za-z_$]/u.test(first)) {
      kind = "identifier";
      while (cursor < content.length && /[A-Za-z0-9_$]/u.test(content[cursor])) cursor++;
    } else if (/[0-9]/u.test(first)) {
      kind = "number";
      while (cursor < content.length && /[A-Za-z0-9_.]/u.test(content[cursor])) cursor++;
    } else if (first === "/" && (!tokens.length || /^(?:[([{=:;,!?]|=>|&&|\|\||return|throw|case)$/u.test(tokens.at(-1)!.text))) {
      kind = "regex";
      let characterClass = false;
      cursor++;
      while (cursor < content.length) {
        if (content[cursor] === "\\") { cursor += 2; continue; }
        if (/[\r\n]/u.test(content[cursor])) return null;
        if (content[cursor] === "[") characterClass = true;
        if (content[cursor] === "]") characterClass = false;
        if (content[cursor] === "/" && !characterClass) break;
        cursor++;
      }
      if (cursor >= content.length) return null;
      cursor++;
      while (/[A-Za-z]/u.test(content[cursor] ?? "")) cursor++;
    } else {
      const operator = /^(?:\.\.\.|===|!==|>>>=|>>>|>>=|<<=|\*\*=|&&=|\|\|=|\?\?=|=>|==|!=|<=|>=|\+\+|--|\+=|-=|\*=|\/=|%=|&=|\|=|\^=|&&|\|\||\?\?|\?\.|\*\*|<<|>>)/u.exec(content.slice(cursor))?.[0];
      if (!operator && !/[{}()[\].,;:?~!+\-*/%<>=&|^]/u.test(first)) return null;
      cursor += operator?.length ?? 1;
    }
    const text = content.slice(start, cursor), group = stack.at(-1) ?? -1, scope = [...stack].reverse().find((index) => tokens[index].text === "{") ?? -1;
    const index = tokens.length;
    tokens.push({ text, start, end: cursor, kind, group, scope });
    if (kind !== "punctuation") continue;
    if (["(", "[", "{"].includes(text)) stack.push(index);
    else if ([")", "]", "}"].includes(text)) {
      const open = stack.pop();
      if (open === undefined || "([{".indexOf(tokens[open].text) !== ")]}".indexOf(text)) return null;
      pairs.set(open, index);
      pairs.set(index, open);
    }
  }
  return stack.length ? null : { tokens, pairs };
}

/** 🧩️ Splits only top-level token segments, keeping nested calls, arrays, and blocks opaque. */
function typescriptCollectionSegments(syntax: TypeScriptCollectionSyntax, start: number, end: number): readonly (readonly [number, number])[] {
  const rows: [number, number][] = [];
  let left = start;
  for (let cursor = start; cursor < end; cursor++) {
    const close = syntax.pairs.get(cursor);
    if (close !== undefined && close > cursor) { cursor = close; continue; }
    if (syntax.tokens[cursor].text === ",") { rows.push([left, cursor]); left = cursor + 1; }
  }
  if (left < end) rows.push([left, end]);
  return rows;
}

/** 🪆️ Exposes simple template expressions for binding checks and rejects unparsed nested expression syntax. */
function typescriptCollectionEmbeddedExpressions(syntax: TypeScriptCollectionSyntax): readonly TypeScriptCollectionSyntax[] | null {
  const expressions: TypeScriptCollectionSyntax[] = [];
  for (const token of syntax.tokens) if (token.kind === "template") {
    for (let cursor = 1; cursor < token.text.length - 1; cursor++) {
      if (token.text[cursor] === "\\") { cursor++; continue; }
      if (!token.text.startsWith("$" + "{", cursor)) continue;
      const end = token.text.indexOf("}", cursor + 2), source = token.text.slice(cursor + 2, end);
      if (end < 0 || /[{}\u0060]/u.test(source)) return null;
      const expression = typescriptCollectionSyntax(source);
      if (!expression) return null;
      expressions.push(expression);
      cursor = end;
    }
  }
  return expressions;
}

/** 🧷️ Detects direct, property, computed-property, and destructuring writes to a proven binding. */
function typescriptCollectionChangedBinding(syntax: TypeScriptCollectionSyntax, name: string, declaration = -1): boolean {
  const { tokens, pairs } = syntax;
  const assignment = (text?: string): boolean => /^(?:=|\+=|-=|\*=|\/=|%=|&=|\|=|\^=|&&=|\|\|=|\?\?=|\*\*=|<<=|>>=|>>>=|\+\+|--)$/u.test(text ?? "");
  return tokens.some((token, index) => {
    if (token.kind !== "identifier" || token.text !== name || index === declaration) return false;
    if (assignment(tokens[index + 1]?.text) || ["++", "--"].includes(tokens[index - 1]?.text)) return true;
    let cursor = index + 1;
    while (tokens[cursor]?.text === "." && tokens[cursor + 1]?.kind === "identifier" || tokens[cursor]?.text === "[" && pairs.has(cursor)) cursor = tokens[cursor].text === "[" ? pairs.get(cursor)! + 1 : cursor + 2;
    if (cursor > index + 1 && assignment(tokens[cursor]?.text)) return true;
    for (let group = token.group; group >= 0; group = tokens[group].group) if (assignment(tokens[(pairs.get(group) ?? -2) + 1]?.text)) return true;
    return false;
  });
}

/** 🚧️ Prevents a rejected for-of proof from falling through the independent map-only authority. */
function typescriptCollectionHasForOf(content: string): boolean {
  const firstFor = content.search(/\bfor\b/u);
  if (firstFor < 0 || !/\bof\b/u.test(content.slice(firstFor + 3))) return false;
  const syntax = typescriptCollectionSyntax(content);
  if (!syntax) return true;
  const { tokens, pairs } = syntax;
  return tokens.some((token, index) => {
    if (token.text !== "for") return false;
    const open = index + (tokens[index + 1]?.text === "await" ? 2 : 1), close = pairs.get(open);
    return tokens[open]?.text === "(" && close !== undefined && tokens.slice(open + 1, close).some((part) => part.kind === "identifier" && part.text === "of" && part.group === open);
  });
}

/** 🛡️ Proves immutable for-of reader bindings and emits only exact unescaped physical leaf spans. */
function typescriptPathCollectionReferenceAuthority(content: string): ReferenceToken[] {
  if (!content.includes("for") || !content.includes("readFileSync") || !content.includes("node:path") || !content.includes("node:fs")) return [];
  const syntax = typescriptCollectionSyntax(content);
  if (!syntax) return [];
  const embedded = typescriptCollectionEmbeddedExpressions(syntax);
  if (!embedded) return [];
  const { tokens, pairs } = syntax;
  type Declaration = { name: string; index: number; kind: string; start: number; end: number; scope: number; exported: boolean };
  const declarations = new Map<string, Declaration[]>(), bindings = new Map<string, number[]>(), imports = new Map<string, { index: number; module: string; name: string }>(), functions: [number, number][] = [];
  const bind = (index: number): void => { if (tokens[index]?.kind === "identifier") bindings.set(tokens[index].text, [...(bindings.get(tokens[index].text) ?? []), index]); };
  const endOf = (start: number, stops: readonly string[]): number => {
    for (let cursor = start; cursor < tokens.length; cursor++) {
      if (stops.includes(tokens[cursor].text)) return cursor;
      const close = pairs.get(cursor);
      if (close !== undefined && close > cursor) cursor = close;
    }
    return tokens.length;
  };
  for (let index = 0; index < tokens.length; index++) {
    const token = tokens[index];
    if (["const", "let", "var"].includes(token.text)) {
      let cursor = index + 1;
      while (cursor < tokens.length) {
        const name = tokens[cursor];
        if (name.kind !== "identifier") { const close = pairs.get(cursor); if (close !== undefined) for (let part = cursor + 1; part < close; part++) bind(part); break; }
        bind(cursor);
        const equal = endOf(cursor + 1, ["=", ",", ";", "of", "in", ")"]);
        const end = tokens[equal]?.text === "=" ? endOf(equal + 1, [",", ";", ")"]) : equal;
        const row = { name: name.text, index: cursor, kind: token.text, start: tokens[equal]?.text === "=" ? equal + 1 : end, end, scope: token.scope, exported: tokens[index - 1]?.text === "export" };
        declarations.set(name.text, [...(declarations.get(name.text) ?? []), row]);
        if (tokens[end]?.text !== ",") break;
        cursor = end + 1;
      }
    }
    let importOpen = token.text === "import" ? index + 1 : -1;
    if (tokens[importOpen]?.kind === "identifier") { bind(importOpen); importOpen = tokens[importOpen + 1]?.text === "," ? importOpen + 2 : -1; }
    if (tokens[importOpen]?.text === "*" && tokens[importOpen + 1]?.text === "as") bind(importOpen + 2);
    if (tokens[importOpen]?.text === "{") {
      const close = pairs.get(importOpen), module = close === undefined ? undefined : tokens[close + 2];
      if (close !== undefined && tokens[close + 1]?.text === "from" && module?.kind === "string" && !module.text.includes("\\")) {
        for (const [start, end] of typescriptCollectionSegments(syntax, importOpen + 1, close)) {
          const local = end - start === 3 && tokens[start + 1].text === "as" ? start + 2 : end - start === 1 ? start : -1;
          if (local >= 0) { bind(local); imports.set(tokens[local].text, { index: local, module: module.text.slice(1, -1), name: tokens[start].text }); }
        }
      }
    }
    if (["function", "class"].includes(token.text)) bind(index + 1);
    if (token.text === "(") {
      const close = pairs.get(index), previous = tokens[index - 1]?.text, next = close === undefined ? undefined : tokens[close + 1]?.text;
      if (close !== undefined && (next === "=>" || ["{", ":"].includes(next) && !["for", "if", "while", "switch", "with"].includes(previous))) {
        for (const [start, end] of typescriptCollectionSegments(syntax, index + 1, close)) {
          const limit = Math.min(endOf(start, [":", "="]), end);
          for (let cursor = start; cursor < limit; cursor++) bind(cursor);
        }
        if (next === "{") functions.push([close + 1, pairs.get(close + 1)!]);
      }
    }
    if (token.text === "=>") {
      if (tokens[index - 1]?.kind === "identifier") bind(index - 1);
      functions.push([index + 1, tokens[index + 1]?.text === "{" ? pairs.get(index + 1)! : endOf(index + 1, [";", ",", ")", "]"])]);
    }
  }
  const changed = (name: string, declaration = -1): boolean => typescriptCollectionChangedBinding(syntax, name, declaration) || embedded.some((expression) => typescriptCollectionChangedBinding(expression, name));
  const immutable = (row?: Declaration): row is Declaration => Boolean(row && row.kind === "const" && bindings.get(row.name)?.length === 1 && !changed(row.name, row.index));
  const unique = (name: string): Declaration | undefined => declarations.get(name)?.length === 1 ? declarations.get(name)![0] : undefined;
  const ancestor = (outer: number, inner: number): boolean => { for (let scope = inner; scope >= 0; scope = tokens[scope].scope) if (scope === outer) return true; return outer === -1; };
  const staticValue = (start: number, end: number, scope: number, seen = new Set<string>()): string | null => {
    if (end - start !== 1) return null;
    const token = tokens[start];
    if (token.kind === "string") return token.text.includes("\\") ? null : token.text.slice(1, -1);
    if (token.kind === "identifier") {
      const row = unique(token.text);
      return immutable(row) && row.end < start && ancestor(row.scope, scope) && !seen.has(row.name) ? staticValue(row.start, row.end, row.scope, new Set([...seen, row.name])) : null;
    }
    if (token.kind !== "template" || token.text.includes("\\")) return null;
    let valid = true;
    const value = token.text.slice(1, -1).replace(/\$\{([A-Za-z_$][\w$]*)\}/gu, (_match, name: string) => {
      const row = unique(name);
      const part = immutable(row) && row.end < start && ancestor(row.scope, scope) && !seen.has(name) ? staticValue(row.start, row.end, row.scope, new Set([...seen, name])) : null;
      if (part === null) valid = false;
      return part ?? "";
    });
    return valid && !value.includes("$" + "{") ? value : null;
  };
  const imported = (index: number, module: string, name: string): boolean => {
    const token = tokens[index], owner = imports.get(token?.text);
    return Boolean(token?.kind === "identifier" && ![".", "?."].includes(tokens[index - 1]?.text) && owner?.module === module && owner.name === name && bindings.get(token.text)?.length === 1 && !changed(token.text, owner.index));
  };
  const rootValue = (index: number): boolean => {
    const row = unique(tokens[index]?.text);
    return immutable(row) && row.end < index && ancestor(row.scope, tokens[index].scope) && tokens.slice(row.start, row.end).map((token) => token.text).join(" ") === "process . cwd ( )" && !bindings.has("process") && !changed("process");
  };
  const loops: { index: number; item: number; collection: number; body: number; end: number }[] = [];
  for (let index = 0; index < tokens.length; index++) if (tokens[index].text === "for" && tokens[index + 1]?.text === "(" && tokens[index + 2]?.text === "const" && tokens[index + 3]?.kind === "identifier" && tokens[index + 4]?.text === "of" && tokens[index + 5]?.kind === "identifier" && pairs.get(index + 1) === index + 6 && tokens[index + 7]?.text === "{") loops.push({ index, item: index + 3, collection: index + 5, body: index + 7, end: pairs.get(index + 7)! });
  const relativeLeaf = (value: string): boolean => Boolean(value) && !value.startsWith("/") && !/^[A-Za-z]:/u.test(value) && !/[\\\u0000]/u.test(value) && !value.split("/").some((part) => !part || part === "." || part === "..");
  const rows: ReferenceToken[] = [];
  for (const entries of declarations.values()) {
    const row = entries.length === 1 ? entries[0] : undefined;
    if (!immutable(row) || row.exported || tokens[row.start]?.text !== "[") continue;
    const close = pairs.get(row.start);
    if (close === undefined || close !== row.end - 1 && !(close === row.end - 3 && tokens[close + 1]?.text === "as" && tokens[close + 2]?.text === "const")) continue;
    const elements = typescriptCollectionSegments(syntax, row.start + 1, close);
    if (!elements.length || elements.some(([start, end]) => { const value = staticValue(start, end, row.scope); return value === null || !relativeLeaf(value); })) continue;
    const uses = tokens.flatMap((token, index) => token.kind === "identifier" && token.text === row.name && index !== row.index ? [index] : []);
    if (embedded.some((expression) => expression.tokens.some((token) => token.kind === "identifier" && token.text === row.name))) continue;
    if (uses.some((index) => !loops.some((loop) => loop.collection === index) && !(tokens[index - 1]?.text === "..." && tokens[tokens[index].group]?.text === "[" && [",", "]"].includes(tokens[index + 1]?.text)))) continue;
    const reader = loops.some((loop) => {
      const item = unique(tokens[loop.item].text);
      if (tokens[loop.collection].text !== row.name || row.end >= loop.index || row.scope !== tokens[loop.index].scope || !immutable(item) || item.index !== loop.item) return false;
      const joined = (start: number, end: number): boolean => {
        if (!imported(start, "node:path", "join") || tokens[start + 1]?.text !== "(" || pairs.get(start + 1) !== end - 1) return false;
        const arguments_ = typescriptCollectionSegments(syntax, start + 2, end - 1);
        return arguments_.length === 2 && arguments_.every(([left, right]) => right - left === 1) && rootValue(arguments_[0][0]) && tokens[arguments_[1][0]].kind === "identifier" && tokens[arguments_[1][0]].text === item.name;
      };
      for (let index = loop.body + 1; index < loop.end; index++) {
        if (tokens[index].scope !== loop.body || !imported(index, "node:fs", "readFileSync") || tokens[index + 1]?.text !== "(" || functions.some(([start, end]) => start > loop.body && start <= index && index < end)) continue;
        const close = pairs.get(index + 1), arguments_ = close === undefined ? [] : typescriptCollectionSegments(syntax, index + 2, close);
        if (!arguments_.length) continue;
        const [start, end] = arguments_[0];
        if (joined(start, end)) return true;
        const alias = end - start === 1 && tokens[start].kind === "identifier" ? unique(tokens[start].text) : undefined;
        if (immutable(alias) && alias.end < index && alias.scope === loop.body && joined(alias.start, alias.end)) return true;
      }
      return false;
    });
    if (reader) for (const [index] of elements) if (tokens[index].kind === "string") {
      const token = tokens[index], value = token.text.slice(1, -1), start = token.start + 1;
      rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, start, "path-collection-for-of"), start, end: token.end - 1, value, physicalTargets: [value] });
    }
  }
  return rows.sort((left, right) => left.start - right.start);
}

/** 🧪️ `runVitest(bundleRoot, segments, config?)`'s optional third argument names a config file
 * resolved relative to `bundleRoot` at runtime (every `📜️script.ts` router's own `cwd`), not to
 * whatever else the call happens to quote first — `segments` is very often a literal array of
 * quoted test filenames (e.g. `runVitest(this.root, ["a.test.ts", …], "🧪️tests/🟦️.ts")`), so
 * the generic first-quoted-string scanners this file otherwise uses would misidentify a segment
 * name as the config path. This takes the LAST quoted string in the call instead, matching the
 * parameter's trailing position; a call with no quoted config argument (the common, default-using
 * case) yields no token. */
function runVitestConfigArgumentTokens(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const match of content.matchAll(/\brunVitest\s*\(([^;\r\n]*)\)/gu)) {
    if (match.index === undefined) continue;
    const args = match[1];
    if (!args) continue;
    const argsStart = match.index + match[0].indexOf(args);
    const last = [...args.matchAll(/["']([^"']+)["']/gu)].at(-1);
    if (!last || last.index === undefined) continue;
    const start = argsStart + last.index + 1;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, start, "run-vitest-config"), start, end: start + last[1]!.length, value: last[1]! });
  }
  return rows;
}

/** 🧪️ A vitest config's own `test.includeSource` / `test.coverage.include` arrays name the
 * same package's in-source (`import.meta.vitest`) suite files by a path relative to the config
 * file's own directory — the house convention documented inline in every such config ("add new
 * in-source files to includeSource/coverage.include only"). Neither key is a call argument, so the
 * call-scoped scanners above never see them; this extracts every quoted literal that is a direct
 * element of either array. Deliberately narrower than the ordinary `test.include` glob key (a
 * standard vitest option, already populated with genuine test globs in several configs and never
 * previously scanned) — only `includeSource` and `include` nested one level inside a `coverage: {…}`
 * block are matched, so this cannot start tracking an unrelated pre-existing `include` array. A
 * glob literal (e.g. `"*.ts"`, seen paired with a real sibling path in `🎠️kernel`'s config) names no
 * single physical file, so any element containing `*` is skipped rather than treated as a target. */
function vitestConfigIncludeArrayTokens(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  const collect = (match: RegExpMatchArray): void => {
    if (match.index === undefined) return;
    const inner = match[1]!;
    const innerStart = match.index + match[0].indexOf("[") + 1;
    for (const literal of inner.matchAll(/["']([^"']+)["']/gu)) {
      if (literal.index === undefined || literal[1]!.includes("*")) continue;
      const start = innerStart + literal.index + 1;
      rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, start, "vitest-config-include"), start, end: start + literal[1]!.length, value: literal[1]! });
    }
  };
  for (const match of content.matchAll(/\bincludeSource\s*:\s*\[([^\]]*)\]/gu)) collect(match);
  for (const match of content.matchAll(/\bcoverage\s*:\s*\{[^{}]*?\binclude\s*:\s*\[([^\]]*)\]/gu)) collect(match);
  return rows;
}

function typescriptTokens(path: string, content: string): ReferenceToken[] {
  const rows = regexTokens(content, "typescript", "typescript-path", [
    /(?:\bfrom\s*|\bimport\s*\(|\brequire\s*\(|\bimport\s+)["'\s]*([^"'\s)]+)["']/gu,
    /\b(?:worker|url)\s*\(\s*["']([^"']+)["']/giu,
    /\b(?:[A-Za-z_$][\w$]*(?:Path|File|Filename|Root|Schema|Taxonomy|Config|Entry|Target|Source|Output|Input)[\w$]*|(?:path|file|filename|root|schema|taxonomy|config|entry|target|source|output|input))\s*(?:=|:)\s*["']([^"']+)["']/giu,
    /\b(?:resolve|join|readFileSync|writeFileSync|existsSync|openSync|Bun\.file|policyReadFileSafe)\s*\([^;\r\n]*?["']([^"']+)["']/giu,
  ]);
  rows.push(...runVitestConfigArgumentTokens(content));
  if (basename(path) === "🟦️.ts" && basename(dirname(path)) === "🧪️tests") rows.push(...vitestConfigIncludeArrayTokens(content));
  if (basename(path) === ".dependency-cruiser.cjs") rows.push(...dependencyCruiserBoundaryReferenceAuthority(content));
  rows.push(...ticketImportantProseReferenceAuthority(content).map((entry) => ({ ...entry, adapter: "typescript" as const })));
  rows.push(...typescriptLeadingDocumentationReferenceAuthority(content).map((entry) => ({ ...entry, adapter: "typescript" as const })));
  rows.push(...typescriptCommentPathReferenceAuthority(content).map((entry) => ({ ...entry, adapter: "typescript" as const })));
  let catalogLineStart = 0, catalogLineEnd = content.indexOf("\n"), previousCatalogLine = -1;
  for (const match of content.matchAll(/\bimport\.meta\.glob\s*\(/gu)) {
    if (match.index === undefined) continue;
    while (catalogLineEnd >= 0 && catalogLineEnd < match.index) {
      catalogLineStart = catalogLineEnd + 1;
      catalogLineEnd = content.indexOf("\n", catalogLineStart);
    }
    if (previousCatalogLine === catalogLineStart) continue;
    previousCatalogLine = catalogLineStart;
    const end = catalogLineEnd < 0 ? content.length : catalogLineEnd + 1, value = content.slice(catalogLineStart, end);
    const selectors = [...value.matchAll(/["']([^"']*modelDefinitions[^"']*)["']/gu)].map((row) => row[1]);
    if (selectors.length === 0) continue;
    const start = catalogLineStart;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, start, "artifact-catalog-glob"), start, end, value, rewriteKind: "artifact-catalog-glob", rewriteData: { selectors: JSON.stringify(selectors) } });
  }
  for (const match of content.matchAll(/(🖼️assets\/🏗️modelDefinitions\/<modelDefinition>\/\{[\s\S]*?\})/gu)) {
    if (match.index === undefined) continue;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, match.index, "artifact-catalog-comment"), start: match.index, end: match.index + match[1].length, value: match[1], rewriteKind: "artifact-catalog-prose", rewriteData: { form: "catalog-grammar" } });
  }
  for (const match of content.matchAll(/(🖼️assets\/🏗️modelDefinitions\/)/gu)) {
    if (match.index === undefined || rows.some((token) => token.start <= match.index! && token.end >= match.index! + match[1].length)) continue;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, match.index, "artifact-catalog-marker"), start: match.index, end: match.index + match[1].length, value: match[1], rewriteKind: "artifact-catalog-prose", rewriteData: { form: "root-marker" } });
  }
  rows.push(...typescriptPathCollectionReferenceAuthority(content));
  const hasForOfCollection = typescriptCollectionHasForOf(content);
  for (const declaration of content.matchAll(/\bconst\s+((?:[A-Za-z_$][\w$]*(?:Sources|Paths|Files)|paths|sources|files))\s*=\s*\[([\s\S]*?)\]\s*(?:\.map\b|;)/gu)) {
    if (hasForOfCollection) continue;
    if (declaration.index === undefined || !new RegExp(`\\b${declaration[1]}\\b[\\s\\S]*?\\.map\\([\\s\\S]*?\\b(?:policyReadFileSafe|readFileSync|Bun\\.file)\\b`, "u").test(content.slice(declaration.index))) continue;
    const fragmentStart = declaration.index + declaration[0].indexOf(declaration[2]);
    for (const token of regexTokens(declaration[2], "typescript", "path-collection", [/["']([^"']+)["']/gu])) rows.push({ ...token, start: fragmentStart + token.start, end: fragmentStart + token.end, structuredLocation: lineLocation(content, fragmentStart + token.start, "path-collection") });
  }
  return rows;
}

function goTokens(path: string, content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  if (path.toLowerCase().endsWith(".go")) {
    rows.push(...regexTokens(content, "go", "go-import", [/^\s*(?:[\w.]+\s+)?"([^"]+)"\s*$/gmu]));
    for (const match of content.matchAll(/^\s*\/\/go:(?:embed|generate)\s+([^\r\n]+)$/gmu)) if (match.index !== undefined) rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-directive"));
    return rows;
  }
  for (const match of content.matchAll(/\buse\s*\(([\s\S]*?)\)/gu)) if (match.index !== undefined) rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-work-use"));
  for (const match of content.matchAll(/^\s*use\s+([^\r\n(][^\r\n]*)$/gmu)) if (match.index !== undefined) rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-work-use"));
  for (const match of content.matchAll(/^\s*replace\s+[^\r\n=]+=>\s*([^\s]+).*$/gmu)) if (match.index !== undefined) rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-mod-replace"));
  return rows;
}

function cmakeTokens(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const match of content.matchAll(/\b(?:add_subdirectory|add_executable|add_library|target_sources|include|configure_file|set)\s*\(([\s\S]*?)\)/giu)) if (match.index !== undefined) rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "native", "cmake-argument"));
  return rows;
}

function htmlTokens(content: string, adapter: "xml" | "markdown"): ReferenceToken[] {
  return regexTokens(content, adapter, "html-attribute", [/<(?:a|img|script|link|source|video|audio|form)\b[^>]*\b(?:href|src|srcset|poster|data|action)\s*=\s*["']([^"']+)["'][^>]*>/giu]);
}

/** 🔗 Collects exact inline destinations with monotonic shared lookaheads. */
function markdownInlineTokens(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [], delimiterPattern = /[)\s]/gu, titlePattern = /\S/gu;
  let cursor = 0, delimiter = -1, titleStart = -1, titleEnd = -1;
  while (cursor < content.length) {
    const open = content.indexOf("[", cursor);
    if (open < 0) break;
    const close = content.indexOf("]", open + 1);
    if (close < 0) break;
    cursor = close + 1;
    if (content[cursor] !== "(") continue;
    const start = cursor + 1;
    if (delimiter < start) {
      delimiterPattern.lastIndex = start;
      delimiter = delimiterPattern.exec(content)?.index ?? content.length;
    }
    if (delimiter === start || delimiter === content.length) continue;
    let end = delimiter;
    if (content[end] !== ")") {
      if (titleStart <= delimiter) {
        titlePattern.lastIndex = delimiter;
        titleStart = titlePattern.exec(content)?.index ?? content.length;
      }
      if (content[titleStart] !== '"') continue;
      if (titleEnd <= titleStart) {
        const next = content.indexOf('"', titleStart + 1);
        titleEnd = next < 0 ? content.length : next;
      }
      if (content[titleEnd + 1] !== ")") continue;
      end = titleEnd + 1;
    }
    rows.push({ adapter: "markdown", structuredLocation: lineLocation(content, start, "markdown-link"), start, end: delimiter, value: content.slice(start, delimiter) });
    cursor = end + 1;
  }
  return rows;
}

/** 🗒️ Resolves real repo-relative paths named as plain single-backtick inline code or a bare
 * path-only list item, outside every opaque Markdown block (fenced/indented code, blockquotes,
 * HTML) — reusing {@link markdownSourceCoordinateSpans}'s already-proven span discipline (built for
 * `frozenMarkdownCoordinateEvidenceCoordinates`) so a design-rationale doc gets the same live-
 * reference treatment `rustTokens`/`typescriptCommentPathReferenceAuthority` already give source
 * comments. A span registered as frozen evidence stays protected regardless — the shared
 * `frozenEvidence`/`isFrozenSourceCoordinateToken` checks in the caller run over every adapter's
 * tokens before any rewrite, and a value that never resolves to a real repo path is simply ignored. */
function markdownCommentPathReferenceAuthority(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const span of markdownSourceCoordinateSpans(content)) {
    const value = content.slice(span.start, span.end);
    if (!value || /[\s[\](){}<>"'`\\]/u.test(value) || !/[/.]/u.test(value)) continue;
    rows.push({ adapter: "markdown", structuredLocation: lineLocation(content, span.start, `markdown-${span.form}`), start: span.start, end: span.end, value });
  }
  return rows;
}

function referenceTokens(path: string, content: string, index?: ReferencePathIndex): readonly ReferenceToken[] {
  const lower = path.toLowerCase();
  if (lower.endsWith(".rs")) return rustTokens(path, content, index);
  if (lower.endsWith(".feature")) return gherkinTokens(path, content);
  if (/\.(?:ts|tsx|js|jsx|mjs|cjs|mts|cts)$/u.test(lower)) return typescriptTokens(path, content);
  if (/\.(?:go|mod|work)$/u.test(lower) || /(?:^|\/)go\.(?:mod|work)$/u.test(lower)) return goTokens(path, content);
  if (lower.endsWith(".py")) return pythonTokens(path, content);
  if (/\.(?:csproj|fsproj|vbproj|sln|props|targets|cs|fs|vb)$/u.test(lower)) return regexTokens(content, "dotnet", "dotnet-reference", [/(?:Include|Update|Remove|Link|HintPath)\s*=\s*["']([^"']+)["']/giu, /^Project\([^\r\n]+?=\s*[^,]+,\s*"([^"]+)"/gmu, /\b(?:GetManifestResourceStream|ReadAllText|ReadAllBytes)\s*\(\s*["']([^"']+)["']/gu]);
  if (/\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx|cmake)$/u.test(lower) || basename(path) === "CMakeLists.txt") return [...regexTokens(content, "native", "native-path", [/^\s*#\s*include\s*[<"]([^>"]+)[>"]/gmu, /["']([^"']+\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx))["']/gu]), ...cmakeTokens(content)];
  if (lower.endsWith(".json")) return jsonTokens(path, content, "json");
  if (lower.endsWith(".jsonc")) return jsonTokens(path, content, "jsonc");
  if (lower.endsWith(".toml")) return tomlTokens(path, content);
  if (/\.ya?ml$/u.test(lower)) {
    const direct = regexTokens(content, "yaml", "yaml-value", [/^\s*(?:-\s*)?[\w.-]+\s*:\s*["']?([^"'\s][^\r\n#]*?)["']?\s*(?:#.*)?$/gmu, /^\s*-\s*["']?([^"'\s][^\r\n#]*?)["']?\s*(?:#.*)?$/gmu]);
    const embeddedArgv = /(?:workflow|action|launch|task|project|(?:^|\/)ci(?:\/|$))/iu.test(path);
    return embeddedArgv ? [...direct, ...direct.flatMap((token) => embeddedArgumentTokens(content, token.value, token.start, "yaml", "embedded-argv"))] : direct;
  }
  if (/\.(?:xml|html|htm)$/u.test(lower)) return [...regexTokens(content, "xml", "xml-attribute", [/(?:href|src|path|include|file|link|hintpath)\s*=\s*["']([^"']+)["']/giu]), ...htmlTokens(content, "xml")];
  if (/\.(?:md|mdx)$/u.test(lower)) return [...markdownInlineTokens(content), ...regexTokens(content, "markdown", "markdown-link", [/^\s*\[[^\]]+\]:\s*(\S+)/gmu]), ...htmlTokens(content, "markdown"), ...markdownCommentPathReferenceAuthority(content)];
  return [];
}

function textualPath(path: string): boolean {
  return /(?:\.rs|\.tsx?|\.jsx?|\.mjs|\.cjs|\.mts|\.cts|\.go|\.mod|\.work|\.py|\.cs|\.fs|\.vb|\.csproj|\.fsproj|\.vbproj|\.sln|\.props|\.targets|\.c|\.cc|\.cpp|\.cxx|\.h|\.hh|\.hpp|\.hxx|\.cmake|\.jsonc?|\.toml|\.ya?ml|\.xml|\.html?|\.mdx?|\.feature)$/iu.test(path) || basename(path) === "CMakeLists.txt";
}

function splitTokenSuffix(value: string): { readonly path: string; readonly suffix: string } {
  const index = value.search(/[?#]/);
  return index < 0 ? { path: value, suffix: "" } : { path: value.slice(0, index), suffix: value.slice(index) };
}

interface ReferencePathIndex {
  readonly exact: ReadonlySet<string>;
  readonly nfc: ReadonlyMap<string, string | null>;
  readonly extensionless: ReadonlyMap<string, string | null>;
  readonly pythonModule: ReadonlyMap<string, string | null>;
  readonly repoRoot?: string;
  readonly coordinateRoots: readonly string[];
  readonly coordinateRootSet: ReadonlySet<string>;
  readonly coordinateRootByReference: Map<string, string | undefined>;
  readonly contextPaths: readonly string[];
  readonly contextPathSet: ReadonlySet<string>;
  readonly affectedPaths: ReadonlySet<string>;
  readonly cancelFile?: string;
}

interface RustReferenceGraphView {
  readonly graph: RustModuleGraph;
  readonly hashes: ReadonlyMap<string, string>;
  readonly unreadableInputs: ReadonlyMap<string, Error>;
}

const rustReferenceGraphs = new WeakMap<ReferencePathIndex, Map<string, RustReferenceGraphView>>();
const rustUnprovenReferenceTargets = new WeakMap<ReferencePathIndex, Map<string, readonly string[]>>();
const rustReferenceContextFiles = new WeakMap<ReferencePathIndex, Map<string, readonly string[]>>();

function rustContextFiles(path: string, index: ReferencePathIndex): readonly string[] {
  const coordinateRoot = ancestorReferenceCoordinateRoot(path, index.coordinateRootSet) ?? "";
  const views = rustReferenceContextFiles.get(index) ?? new Map<string, readonly string[]>();
  rustReferenceContextFiles.set(index, views);
  const cached = views.get(coordinateRoot);
  if (cached) return cached;
  const files = index.contextPaths.filter((candidate) => (candidate.endsWith(".rs") || basename(candidate) === "Cargo.toml") && (ancestorReferenceCoordinateRoot(candidate, index.coordinateRootSet) ?? "") === coordinateRoot);
  views.set(coordinateRoot, files);
  return files;
}

function unprovenRustReferenceTargets(referencePath: string, value: string, index: ReferencePathIndex): readonly string[] {
  const cache = rustUnprovenReferenceTargets.get(index) ?? new Map<string, readonly string[]>();
  rustUnprovenReferenceTargets.set(index, cache);
  const coordinateRoot = ancestorReferenceCoordinateRoot(referencePath, index.coordinateRootSet), key = `${coordinateRoot ?? ""}\0${value}`;
  const cached = cache.get(key);
  if (cached) return cached;
  const decoded = unsupportedReferenceTokens(`"${value}"`, "rust")[0]?.targetValues?.[0] ?? value, suffix = decoded.replace(/^(?:\.\.?\/)+/u, "");
  const explicitEscape = /^(?:\.\.\/|\/|[A-Za-z]:[\\/])/u.test(decoded);
  const targets = [...index.exact].filter((candidate) => (!coordinateRoot || candidate.startsWith(`${coordinateRoot}/`) || explicitEscape) && suffix !== "" && (candidate === suffix || candidate.endsWith(`/${suffix}`)));
  cache.set(key, targets);
  return targets;
}

function rustReferenceNeedsOwnership(path: string, references: ReturnType<typeof inspectRustManifestPathReferences>, index: ReferencePathIndex, candidates: ReturnType<typeof inspectRustManifestPathCandidates> = []): boolean {
  if (index.affectedPaths.has(path)) return true;
  const affected = (candidate: string): boolean => index.affectedPaths.has(candidate) || index.affectedPaths.has(index.nfc.get(candidate.normalize("NFC")) ?? "");
  const manifests = rustContextFiles(path, index).filter((candidate) => basename(candidate) === "Cargo.toml");
  for (const reference of references) {
    if (unprovenRustReferenceTargets(path, reference.value, index).some(affected)) return true;
    for (const manifest of manifests) {
      try { if (affected(normalizeRelative(posix.join(posix.dirname(manifest), ...reference.base, reference.value)))) return true; } catch {}
    }
  }
  for (const candidate of candidates) {
    if (unprovenRustReferenceTargets(path, candidate.value, index).some(affected)) return true;
    for (const manifest of manifests) for (const parts of candidate.targets) {
      try { if (affected(normalizeRelative(posix.join(posix.dirname(manifest), ...parts)))) return true; } catch {}
    }
  }
  return false;
}

function rustReferenceGraph(path: string, index: ReferencePathIndex): RustReferenceGraphView | null {
  if (!index.repoRoot) return null;
  const coordinateRoot = ancestorReferenceCoordinateRoot(path, index.coordinateRootSet) ?? "";
  const views = rustReferenceGraphs.get(index) ?? new Map<string, RustReferenceGraphView>();
  rustReferenceGraphs.set(index, views);
  const cached = views.get(coordinateRoot);
  if (cached) return cached;
  const files = rustContextFiles(path, index);
  const contents = new Map<string, string | undefined>(), hashes = new Map<string, string>(), unreadableInputs = new Map<string, Error>();
  const read = (candidate: string): string | undefined => {
    checkCancellation(index.repoRoot!, index.cancelFile);
    if (contents.has(candidate)) return contents.get(candidate);
    let absolute: string;
    try { absolute = assertLexicalInputOutsideOpaque(index.repoRoot!, candidate, "Rust module ownership", true); }
    catch (error) { unreadableInputs.set(candidate, error instanceof Error ? error : new Error(String(error))); contents.set(candidate, undefined); return undefined; }
    const stat = lstatOrNull(absolute);
    if (!stat) { contents.set(candidate, undefined); return undefined; }
    if (!stat.isFile()) { unreadableInputs.set(candidate, new Error(`Rust module ownership is not a regular file: ${candidate}`)); contents.set(candidate, undefined); return undefined; }
    const bytes = readFileSync(absolute), after = lstatSync(absolute);
    if (after.mode !== stat.mode || after.size !== stat.size || after.mtimeMs !== stat.mtimeMs || bytes.byteLength !== stat.size) throw new Error(`Rust module ownership changed during its snapshot: ${candidate}`);
    hashes.set(candidate, sha256(bytes));
    contents.set(candidate, bytes.toString("utf8"));
    return contents.get(candidate);
  };
  const graph = inspectRustModuleGraph(files, read, { strictManifests: true, checkCancellation: () => checkCancellation(index.repoRoot!, index.cancelFile) });
  const view = { graph, hashes, unreadableInputs };
  views.set(coordinateRoot, view);
  return view;
}

/** 🧮️ Admits a complete finite interpretation only through one unchanged physical Cargo source chain. An ancestor's glob import (`use x::*`) is never disqualifying here — file participation comes only from `mod`/`#[path]` declarations, which `inspectRustModuleGraphFacts` already tracks completely regardless of glob re-exports; a glob only affects NAME resolution, never which physical files exist in the graph. A non-literal `.join(...)` argument is separately, structurally unrepresentable by every extractor this function consumes (`inspectRustManifestPathReferences`/`inspectRustManifestPathCandidates`/`inspectRustJoinArgumentSpans` all require a string literal, or a loop bound to string literals, to record anything at all) — so it can never reach this proof as a false `finite` positive regardless of glob imports. */
/** 🛡️ Exact, fully-qualified invocation heads of framework-owned macros verified (by reading their `macro_rules!` bodies in full) to expand to zero `mod` items, so calling them can never hide additional module structure from the static crate-graph prover below. Every plugin's crate-root `glue.rs` calls exactly the first of these once; matched only against the comment-free, single-space-joined token stream `rustCodeOnlyTextForMacroTrust` builds, never raw text, so a decoy in a string literal or comment cannot match. Adding an entry here is a safety-relevant claim about a macro's expansion, not a formatting nicety — verify the full `macro_rules!` body contains no `mod` token before adding one. `derive_artifact_facets!` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, both its primary arm and its `@children_ty` dispatch arms) expands only to `struct`/`impl` items built from `$crate`-qualified type paths and method delegations — zero `mod` tokens anywhere in its body — and is invoked once per artifact subset's `🧬️schema/🦀️component.rs` across the plugin tree, so leaving it untrusted was blocking the crate-graph proof for every ancestor that calls it, not just one. */
const RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS: readonly string[] = [
  "semio_framework_plugin :: plugin_exports !",
  "semio_framework_plugin :: derive_artifact_facets !",
];

/** 🛡️ Bare names of crate-LOCAL `macro_rules!` macros — declared directly inside an ancestor file this guard scans, not merely called from one — verified (by reading the full `macro_rules! name { … }` body) to expand to zero `mod` items. `stdio`'s crate-root `🦀️.rs` declares `impl_serde_op_codec!` this way: its body is two trait impls (`protocol::OpText`/`protocol::OpBinary`) built entirely from method calls (`serde_json::to_string`/`to_vec`/`from_str`/`from_slice`, `.expect`/`.map_err`) — zero `mod` tokens anywhere in the expansion template. Unlike `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS` (an external macro's exact call-site text), a name registered here needs BOTH its definition head (`macro_rules ! name`) and every local invocation (`name !`) scrubbed, since both live in the same ancestor file being trusted. */
const RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS: readonly string[] = [
  "impl_serde_op_codec",
];

/** 🛡️ Bare names of Rust standard-library macros that the language itself guarantees are expression/statement-position only — none can ever expand to, or contain, an item-position `mod`, so their mere presence can never hide module structure. This is a closed, deliberately narrow allowlist (not "trust all of std"): only macros actually encountered in a repository ancestor file get registered, one at a time, exactly like every other entry on this page. `stdio`'s `🦀️.rs` uses `format!` (`semantic_fingerprint`'s error path) and `unreachable!` (`hash_hex_bytes`'s exhaustiveness arm) — both ordinary value-producing expressions, no different in kind from a plain function call as far as module structure is concerned. The `assert*!` family is the same kind of claim: `assert!`/`assert_eq!`/`assert_ne!`/`debug_assert!`/`debug_assert_eq!`/`debug_assert_ne!` expand only to a boolean check plus a `panic!` arm (itself already covered by this same guarantee) — never an item — and appear in essentially every `#[cfg(test)] mod tests` block repo-wide, so leaving them untrusted was disqualifying nearly any ancestor file with tests, independent of whichever other construct also happened to be in it. `vec!` is the same guarantee again: it expands only to a sequence of `Vec::new()`/`.push(...)` calls (or `<[_]>::into_vec(box [...])`), always in expression position — `stdio.pdf`'s `✳️a` schema ancestor (`🧬️schema/🦀️component.rs`) builds a `PdfSnapshot { pages: vec![...] }` test fixture with it, which was disqualifying the pdf `✳️a`/`✳️x` mutation files' `CARGO_MANIFEST_DIR` proof (confirmed by direct instrumentation of `rustFiniteManifestTargets`, not by re-reading the regex). */
const RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS: readonly string[] = [
  "format",
  "unreachable",
  "panic",
  "assert",
  "assert_eq",
  "assert_ne",
  "debug_assert",
  "debug_assert_eq",
  "debug_assert_ne",
  "vec",
];

/** 🧼️ Comment-free, single-space-joined token text, further scrubbed of every call to a macro registered in `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS`, every crate-local macro registered in `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS` (both its `macro_rules!` head and its local invocations), every std expression macro registered in `RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS`, every bare attribute named in `RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_NAMES`, and every argument-free attribute path registered in `RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS` — used only to decide whether an ancestor's attribute/macro usage should make the module-graph prover distrust it, never to decide what the prover treats as physically present. `cfg`/`cfg_attr` are safe to scrub for a structural reason: `inspectRustModuleGraphFacts` already records a `mod` under either attribute as an ordinary module fact (flagged `conditional: true`, never dropped), so the actual proof below already treats a cfg-gated `mod` — and the real, on-disk `#[path]` file it names — as a complete participant; cfg governs whether the module COMPILES, not whether the path reference exists in source text. `allow`/`derive` are safe for a different, simpler reason: the Rust language guarantees both are inert with respect to compilation and module structure for any argument, so their mere presence can never hide a `mod`. Attribute PATHS (registered by their exact fully-qualified, argument-free spelling, e.g. a `#[proc_macro_attribute]` invoked as `#[crate::name]`) are a third, per-macro claim exactly like `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS` — verify the proc-macro's own body emits zero `mod` tokens and cannot structurally emit one before adding an entry. Excluding any of these from the *trust* scan does not exclude what they attach to from the *proof*. */
const RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_NAMES: ReadonlySet<string> = new Set(["cfg", "cfg_attr", "allow", "derive"]);

/** 🛡️ Exact, argument-free, fully-qualified attribute-macro paths verified (by reading the proc-macro's own implementation in full) to expand to zero `mod` items and to structurally be unable to emit one — e.g. `semio_framework_async_macros::async_test` (`🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️.rs`, `expand_async_test`) parses its input as a `syn::ItemFn`, rejects anything that isn't a bare `async fn`, and re-emits only `#[test]`/`fn`/`struct`/`impl` items — no code path produces a `mod`. Matched only against the comment-free, single-space-joined token stream between one attribute's brackets, so a decoy in a string literal or comment cannot match. */
const RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS: ReadonlySet<string> = new Set([
  "semio_framework_async_macros :: async_test",
]);

/** 🔑️ Reserved Rust keywords (2021 edition, plus the unused-but-reserved future set) — never valid
 * macro names, so an identifier-KIND token spelling one of these can precede a bare `!` only as an
 * expression-starting keyword (`if !x`, `while !x`, `return !x`, `match !x { … }`, …), never as a
 * `name!` macro invocation. Distinguishes `rustCodeOnlyTextForMacroTrust`'s bare-`!` scrub from a
 * false positive on `if !(…)`/`while !(…)` — the tokenizer gives keywords `kind: "identifier"` too,
 * same as any real identifier, so "preceded by an identifier" alone is not a precise enough test. */
const RUST_RESERVED_KEYWORDS: ReadonlySet<string> = new Set([
  "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "try", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
]);

function rustCodeOnlyTextForMacroTrust(source: string): string {
  const tokens = rustSyntaxTokens(source), pairs = rustTokenPairs(tokens), kept: string[] = [];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]!.text === "#") {
      let bracket = index + 1;
      if (tokens[bracket]?.text === "!") bracket += 1;
      if (tokens[bracket]?.text === "[") {
        const close = pairs.get(bracket), name = tokens[bracket + 1]?.text;
        if (close !== undefined && name !== undefined && RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_NAMES.has(name) && tokens[bracket + 2]?.text === "(") { index = close; continue; }
        if (close !== undefined && RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS.has(tokens.slice(bracket + 1, close).map((token) => token.text).join(" "))) { index = close; continue; }
      }
    }
    // 🚫️ A bare `!` immediately after a real identifier is a macro-invocation bang (`name!`/`path::name!`)
    // — the only Rust construct a lone `!` token can start that is capable of expanding to an item —
    // and stays in `kept` for the macro-trust checks below. Every other bare `!` is prefix boolean
    // negation (`!expr`), which the grammar restricts to expression position: it can never expand
    // into, or hide, a `mod`. A RESERVED KEYWORD is never a valid macro name, so `if !(…)`/`while
    // !(…)`/`return !x` are negation too, even though the tokenizer gives keywords `kind: "identifier"`
    // the same as real identifiers — `RUST_RESERVED_KEYWORDS` disambiguates. Confirmed by direct
    // instrumentation of `rustFiniteManifestTargets`: `stdio.pdf`'s `✳️x` schema ancestor's `if
    // !(page.width > 0.0 && …)` was tripping the downstream `/[#!]/` scan and disqualifying an
    // otherwise-trusted ancestor.
    if (tokens[index]!.text === "!" && (tokens[index - 1]?.kind !== "identifier" || RUST_RESERVED_KEYWORDS.has(tokens[index - 1]!.text))) continue;
    // 🚫️ `!=` is one punctuation token (see `rustTokens`'s `punctuation` list), never a macro bang —
    // it still literally CONTAINS a `!` character, which the downstream `/[#!]/` scan matches as a
    // textual substring regardless of token boundaries. Same false-positive family as the bare-`!`
    // negation case just above (`stdio.pdf`'s `✳️a` schema ancestor's `d.code.0 != CODE_TEXT_EMPTY`
    // was tripping it after the bare-`!` fix), so it is neutralized the same way: comparison operators
    // are expression-position only and can never expand into, or hide, a `mod`.
    if (tokens[index]!.text === "!=") { kept.push("<>"); continue; }
    kept.push(tokens[index]!.text);
  }
  let text = kept.join(" ");
  for (const invocation of RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS) text = text.split(invocation).join(" ");
  for (const name of RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS) text = text.split(`macro_rules ! ${name}`).join(" ").split(`${name} !`).join(" ");
  for (const name of RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS) text = text.split(`${name} !`).join(" ");
  return text;
}

function rustFiniteManifestTargets(path: string, content: string, candidates: ReturnType<typeof inspectRustManifestPathCandidates>, index: ReferencePathIndex, view: RustReferenceGraphView | null): ReadonlyMap<number, readonly string[]> {
  const result = new Map<number, readonly string[]>(), contexts = view?.graph.contexts.get(path) ?? [];
  if (!index.repoRoot || !view || !contexts.length || contexts.some((context) => context.manifestPath === null)) return result;
  const manifests = [...new Set(contexts.map((context) => context.manifestPath!))];
  if (manifests.length !== 1 || view.hashes.get(path) !== sha256(content)) return result;
  const coordinateRoot = ancestorReferenceCoordinateRoot(path, index.coordinateRootSet) ?? "";
  const sameRoot = (target: string): boolean => (index.coordinateRootSet.has(target) ? target : ancestorReferenceCoordinateRoot(target, index.coordinateRootSet) ?? "") === coordinateRoot;
  const proofPaths = [...new Set(contexts.flatMap((context) => [context.manifestPath!, ...context.sourceChain]))];
  if (!proofPaths.includes(path)) return result;
  const physicalPath = (base: string, parts: readonly string[], requireDirectory = false): string => {
    if (parts.length === 0 || parts.some((part) => posix.isAbsolute(part) || /^[A-Za-z]:/u.test(part) || part.includes("\\") || part.includes("\u0000"))) throw new Error("Rust finite path has no local physical identity");
    let current = normalizeRelative(base);
    if (!sameRoot(current) || !lstatOrNull(assertLexicalInputOutsideOpaque(index.repoRoot!, current, "Rust finite physical base", true))?.isDirectory()) throw new Error("Rust finite path has no coordinate-local physical base");
    const segments = parts.join("/").split("/");
    for (let step = 0; step < segments.length; step++) {
      checkCancellation(index.repoRoot!, index.cancelFile);
      current = normalizeRelative(posix.join(current, segments[step]!));
      if (!sameRoot(current)) throw new Error("Rust finite path step escapes its coordinate root");
      const absolute = assertLexicalInputOutsideOpaque(index.repoRoot!, current, "Rust finite path step", true), stat = lstatOrNull(absolute);
      const directory = requireDirectory || step + 1 < segments.length || ["", ".", ".."].includes(segments[step]!);
      if (!stat || (directory ? !stat.isDirectory() : !stat.isFile() && !stat.isDirectory())) throw new Error("Rust finite path step is not a physical file or directory");
    }
    return current;
  };
  const contents = new Map<string, string>();
  try {
    for (const source of proofPaths) {
      checkCancellation(index.repoRoot, index.cancelFile);
      if (!index.contextPathSet.has(source) || !sameRoot(source) || !view.hashes.has(source)) return result;
      const absolute = assertLexicalInputOutsideOpaque(index.repoRoot, source, "Rust finite source authority", true), before = lstatOrNull(absolute);
      if (!before?.isFile()) return result;
      const bytes = readFileSync(absolute), after = lstatSync(absolute);
      if (after.mode !== before.mode || after.size !== before.size || after.mtimeMs !== before.mtimeMs || bytes.byteLength !== before.size || sha256(bytes) !== view.hashes.get(source)) return result;
      contents.set(source, bytes.toString("utf8"));
    }
    const manifest = inspectRustCargoManifest(contents.get(manifests[0]!)!, true);
    if (!manifest.valid || manifest.dependencies.includes("std")) return result;
    const facts = new Map(proofPaths.filter((source) => source.endsWith(".rs")).map((source) => [source, inspectRustModuleGraphFacts(contents.get(source)!)]));
    const parentImports = facts.get(path)?.uses.some((use) => /^(?:super::)+\*$/u.test(use.specifier)) ?? false;
    for (const source of proofPaths.filter((source) => source.endsWith(".rs") && source !== path)) {
      const text = rustCodeOnlyTextForMacroTrust(contents.get(source)!);
      const withoutPathAttributes = text.replace(/#\s*\[\s*path\s*=\s*"[^"\\]*"\s*\]/gu, "");
      if (/[#!]/u.test(withoutPathAttributes) || /\bmacro\b/u.test(text) || parentImports && /\b(?:std|env)\b/u.test(text)) return result;
    }
    for (const context of contexts) {
      if (physicalPath(posix.dirname(manifests[0]!), [manifest.libPath ?? "src/lib.rs"]) !== context.crateRoot) return result;
      for (let chain = 0; chain + 1 < context.sourceChain.length; chain++) {
        const source = context.sourceChain[chain]!, next = context.sourceChain[chain + 1]!;
        const owners = (view.graph.contexts.get(source) ?? []).filter((owner) => owner.manifestPath === context.manifestPath && owner.crateRoot === context.crateRoot && owner.sourceChain.length === chain + 1 && owner.sourceChain.every((item, index) => item === context.sourceChain[index]) && owner.modulePath.every((item, index) => item === context.modulePath[index]));
        let proven = 0;
        for (const owner of owners) for (const module of facts.get(source)?.modules ?? []) {
          if (module.modulePath.length !== owner.sourceScope.length + 1 || !owner.sourceScope.every((item, index) => item === module.modulePath[index])) continue;
          const modulePath = [...owner.modulePath, module.name];
          if (!modulePath.every((item, index) => item === context.modulePath[index])) continue;
          if (module.inline) { if (module.pathTarget !== null) physicalPath(owner.moduleBase, [module.pathTarget], true); continue; }
          if (view.graph.targets.get(`${context.crateRoot}\0${modulePath.join("::")}`) !== next) continue;
          const base = module.pathTarget !== null && owner.sourceScope.length === 0 ? posix.dirname(source) : owner.moduleBase;
          const raw = module.pathTarget ?? (next === posix.join(base, `${module.name}.rs`) ? `${module.name}.rs` : `${module.name}/mod.rs`);
          if (physicalPath(base, [raw]) !== next) return result;
          proven++;
        }
        if (proven !== 1) return result;
      }
    }
  } catch (error) {
    if (error instanceof TaxonomyCancellationError) throw error;
    return result;
  }
  for (const candidate of candidates) {
    if (!Number.isInteger(candidate.start) || !Number.isInteger(candidate.end) || candidate.start < 0 || candidate.end <= candidate.start || content.slice(candidate.start, candidate.end) !== candidate.value || candidate.targets.length === 0 || candidate.targets.length > 256) continue;
    try {
      const targets = new Set<string>();
      for (const parts of candidate.targets) {
        checkCancellation(index.repoRoot, index.cancelFile);
        const target = normalizeRelative(posix.join(posix.dirname(manifests[0]!), ...parts));
        if (!index.contextPathSet.has(target) || !sameRoot(target)) throw new Error("Rust finite target lacks coordinate-local admission");
        if (physicalPath(posix.dirname(manifests[0]!), parts) !== target) throw new Error("Rust finite target identity changed");
        targets.add(target);
      }
      result.set(candidate.start, [...targets].sort(generatorPathCompare));
    } catch (error) {
      if (error instanceof TaxonomyCancellationError) throw error;
    }
  }
  return result;
}

function rustManifestReferenceTokens(path: string, content: string, index: ReferencePathIndex): readonly ReferenceToken[] {
  const references = inspectRustManifestPathReferences(content);
  const arguments_ = inspectRustJoinArgumentSpans(content);
  const candidates = inspectRustManifestPathCandidates(content);
  if (references.length === 0 && arguments_.length === 0 && candidates.length === 0) return [];
  const view = !rustReferenceNeedsOwnership(path, references, index, candidates) ? null : rustReferenceGraph(path, index), contexts = view?.graph.contexts.get(path) ?? [];
  for (const [candidate, error] of view?.unreadableInputs ?? []) if (candidate === path || basename(candidate) === "Cargo.toml" && (posix.dirname(candidate) === "." || path.startsWith(`${posix.dirname(candidate)}/`))) throw error;
  if (view?.hashes.has(path) && view.hashes.get(path) !== sha256(content)) throw new Error(`Rust reference source changed during ownership resolution: ${path}`);
  const manifests = [...new Set(contexts.map((context) => context.manifestPath).filter((manifest): manifest is string => manifest !== null))];
  const proofPaths = [...new Set(contexts.flatMap((context) => [context.manifestPath!, ...context.sourceChain]))].sort(generatorPathCompare);
  const digest = sha256(canonicalJson(proofPaths.map((source) => ({ path: source, sha256: view?.hashes.get(source) }))));
  const conflicts = (left: Pick<ReferenceToken, "start" | "end">, right: Pick<ReferenceToken, "start" | "end">): boolean => left.start === right.start || left.start < right.end && right.start < left.end;
  const writableInputs = new Set(references.filter((reference, index) => !references.some((other, otherIndex) => index !== otherIndex && conflicts(reference, other))));
  const finiteInputs = new Set(candidates.filter((candidate, index) => !references.some((reference) => conflicts(candidate, reference)) && !candidates.some((other, otherIndex) => index !== otherIndex && conflicts(candidate, other))));
  const finite = rustFiniteManifestTargets(path, content, [...Array.from(writableInputs, (reference) => ({ start: reference.start, end: reference.end, value: reference.value, targets: [[...reference.base, reference.value]] })), ...finiteInputs], index, view);
  const rows: ReferenceToken[] = references.map((reference) => {
    let sourceBase: string | undefined, physicalTargets: string[] = [], unsupportedReason: string | undefined;
    const targets = writableInputs.has(reference) ? finite.get(reference.start) : undefined;
    try {
      if (manifests.length !== 1) unsupportedReason = `Rust manifest-relative path requires one proven Cargo owner, found ${manifests.length}`;
      // 🔀️ Two structurally distinct failures were previously conflated into one message: `targets`
      // is `undefined` when this reference never reached `rustFiniteManifestTargets`'s per-candidate
      // map at all (an early whole-file guard bailed, or its own try/catch threw for this candidate
      // specifically) — no proof was ever attempted. `targets.length !== 1` with `targets` DEFINED
      // means the proof ran to completion and found zero or several distinct physical targets — a
      // genuine ambiguity, not a missing proof. Three earlier diagnoses on this codebase (see
      // `rust-path-join` ticket history) mistook the first for the second; keep them apart.
      else if (targets === undefined) unsupportedReason = "Rust manifest-relative path was never admitted into a proven physical source chain";
      else if (targets.length !== 1) unsupportedReason = `Rust manifest-relative path resolved to ${targets.length} distinct physical targets, not exactly one`;
      else { physicalTargets = [...targets]; sourceBase = normalizeRelative(posix.join(dirname(manifests[0]!), ...reference.base)); }
    } catch (error) { unsupportedReason = error instanceof Error ? error.message : String(error); }
    if (unsupportedReason) physicalTargets = [...unprovenRustReferenceTargets(path, reference.value, index)];
    return { adapter: "rust" as const, structuredLocation: lineLocation(content, reference.start, `rust-path-join:${digest}`), start: reference.start, end: reference.end, value: reference.value, physicalTargets, ...(unsupportedReason ? { unsupportedReason } : { rewriteKind: "rust-path-join" as const, rewriteData: { sourceBase: sourceBase! } }) };
  });
  for (const candidate of candidates) if (!rows.some((row) => row.start === candidate.start && row.end === candidate.end)) {
    const physicalTargets = finiteInputs.has(candidate) ? finite.get(candidate.start) : undefined;
    rows.push({ adapter: "rust", structuredLocation: lineLocation(content, candidate.start, physicalTargets ? `rust-finite-manifest-targets:${digest}` : "rust-path-join-unproven"), start: candidate.start, end: candidate.end, value: candidate.value, physicalTargets: physicalTargets ?? unprovenRustReferenceTargets(path, candidate.value, index), ...(physicalTargets ? { physicalInterpretation: "rust-finite-manifest-targets" as const } : {}), unsupportedReason: "Rust finite candidate has no writable literal authority" });
  }
  // 🚱️ `env::temp_dir()`/`env::var[_os]()`/`env::args()`/a bare `fn` parameter with no local
  // `CARGO_MANIFEST_DIR` anchor PROVES the join's root can never name a repository file at plan
  // time — that's not an unprovable reference, it's a proven non-reference, so it must not be
  // recorded at all (never emitted, not merely marked resolved). An unrecognized base is untouched
  // by this set and still falls through to the `rust-path-join-unproven` block below unchanged.
  const nonRepoBases = arguments_.length > 0 ? inspectRustNonRepoJoinBaseSpans(content) : new Set<number>();
  for (const argument of arguments_) if (!nonRepoBases.has(argument.start) && !rows.some((row) => row.start === argument.start && row.end === argument.end)) rows.push({ adapter: "rust", structuredLocation: lineLocation(content, argument.start, "rust-path-join-unproven"), ...argument, physicalTargets: unprovenRustReferenceTargets(path, argument.value, index), unsupportedReason: "Rust join argument has no proven immutable manifest-relative base" });
  return rows;
}

function rustReferenceInterpretationCovers(token: ReferenceToken, candidate: ReferenceToken): boolean {
  if (token.adapter !== "rust" || candidate.adapter !== "rust") return false;
  if (token.rewriteKind === "rust-path-join") return token.start <= candidate.start && token.end >= candidate.end;
  return token.physicalInterpretation === "rust-finite-manifest-targets" && token.rewriteKind === undefined && Boolean(token.unsupportedReason) && (token.physicalTargets?.length ?? 0) > 0 && token.start === candidate.start && token.end === candidate.end && token.value === candidate.value;
}

function referenceTokensIncludingUnsupported(path: string, content: string, index: ReferencePathIndex): readonly ReferenceToken[] {
  const supported = referenceTokens(path, content, index);
  return [...supported, ...unsupportedReferenceTokens(content, referenceAdapter(path)).filter((candidate) => candidate.adapter !== "rust" || !supported.some((token) => rustReferenceInterpretationCovers(token, candidate)))];
}

function addUniqueIndex(index: Map<string, string | null>, key: string, value: string): void {
  if (!key) return;
  const existing = index.get(key);
  if (existing === undefined) index.set(key, value);
  else if (existing !== value) index.set(key, null);
}

function referencePathIndex(paths: Iterable<string>, repoRoot?: string, coordinateRoots: readonly string[] = [], contextPaths?: Iterable<string>, cancelFile?: string, affectedPaths?: ReadonlySet<string>): ReferencePathIndex {
  const exact = new Set<string>();
  const nfc = new Map<string, string | null>();
  const extensionless = new Map<string, string | null>();
  const pythonModule = new Map<string, string | null>();
  for (const path of paths) {
    exact.add(path);
    const normalized = path.normalize("NFC");
    addUniqueIndex(nfc, normalized, path);
    addUniqueIndex(extensionless, normalized.replace(/\.[^/.]+(?:\.[^/.]+)*$/u, ""), path);
    if (!normalized.endsWith(".py")) continue;
    const moduleSegments = (normalized.endsWith("/__init__.py") ? dirname(normalized) : normalized.slice(0, -3)).split("/").filter(Boolean);
    for (let index = 0; index < moduleSegments.length; index++) addUniqueIndex(pythonModule, moduleSegments.slice(index).join("."), path);
  }
  const contexts = [...(contextPaths ?? exact)];
  return { exact, nfc, extensionless, pythonModule, repoRoot, coordinateRoots, coordinateRootSet: new Set(coordinateRoots), coordinateRootByReference: new Map(), contextPaths: contexts, contextPathSet: new Set(contexts), affectedPaths: affectedPaths ?? exact, cancelFile };
}

/** 🧭️ Resolves a reference token to a repository-relative path. A bare single-segment token (no
 * `/` anywhere) tries its same-directory sibling before its root/coordinate-root reading — a
 * `cwd`-relative filename argument reads as "beside me" far more often than "at the repository
 * root", and this order only changes the outcome when both an unrelated root/coordinate-root file
 * and a same-named sibling exist. Every other token shape (containing a `/`, or an explicit
 * absolute/`./`/`../` form) keeps the root-or-coordinate-root-first order. */
function resolveReferencePath(referencePath: string, token: string, index: ReferencePathIndex): string | null {
  const split = splitTokenSuffix(token);
  const absoluteRoot = index.repoRoot?.replaceAll("\\", "/").replace(/\/+$/u, "");
  const absoluteLocal = absoluteRoot && split.path.startsWith(`${absoluteRoot}/`) ? split.path.slice(absoluteRoot.length + 1) : null;
  if (!split.path || absoluteLocal === null && /^(?:[a-z][a-z0-9+.-]*:|#|@|\$|\{)/i.test(split.path) || /[*{}]/.test(split.path)) return null;
  const candidates: string[] = [];
  if (!index.coordinateRootByReference.has(referencePath)) index.coordinateRootByReference.set(referencePath, ancestorReferenceCoordinateRoot(referencePath, index.coordinateRootSet));
  const coordinateRoot = index.coordinateRootByReference.get(referencePath);
  const pushRootCandidate = (): void => { try { if (!/^\.\.?\//u.test(split.path)) candidates.push(normalizeRelative(absoluteLocal ?? (coordinateRoot ? `${coordinateRoot}/${split.path.replace(/^\//u, "")}` : split.path.replace(/^\//u, "")))); } catch {} };
  const pushSiblingCandidate = (): void => { try { candidates.push(normalizeRelative(posix.join(dirname(referencePath), split.path))); } catch {} };
  if (absoluteLocal === null && !split.path.includes("/")) { pushSiblingCandidate(); pushRootCandidate(); } else { pushRootCandidate(); pushSiblingCandidate(); }
  for (const candidate of candidates) {
    if (index.exact.has(candidate)) return candidate;
    const comparison = candidate.normalize("NFC");
    const nfc = index.nfc.get(comparison);
    if (nfc) return nfc;
    const extensionless = index.extensionless.get(comparison);
    if (extensionless) return extensionless;
  }
  if (/^[\w.]+$/.test(split.path)) {
    const python = index.pythonModule.get(split.path.normalize("NFC"));
    if (python) return python;
  }
  return null;
}

function resolveReferenceTokenPath(referencePath: string, token: ReferenceToken, index: ReferencePathIndex): string | null {
  if (token.physicalTargets !== undefined) {
    const matches = [...new Set(token.physicalTargets.map((value) => index.exact.has(value) ? value : index.nfc.get(value.normalize("NFC"))).filter((value): value is string => typeof value === "string"))];
    return matches.length === 1 ? matches[0]! : null;
  }
  const matches = [...new Set((token.targetValues ?? [token.value]).map((value) => resolveReferencePath(referencePath, value, index)).filter((value): value is string => value !== null))];
  return matches.length === 1 ? matches[0] : null;
}

function lexicalOpaqueReferenceTarget(referencePath: string, token: ReferenceToken, taxonomy: LoadedTaxonomy): string | null {
  for (const value of token.targetValues ?? [token.value]) {
    const path = splitTokenSuffix(value).path;
    if (!path || /^(?:[a-z][a-z0-9+.-]*:|#|@|\$|\{)/iu.test(path) || /[*{}]/u.test(path)) continue;
    const candidates = [path.replace(/^\//u, ""), posix.join(dirname(referencePath), path)];
    for (const candidate of candidates) {
      try {
        const normalized = normalizeRelative(candidate);
        if (isExcluded(normalized, taxonomy)) return normalized;
      } catch {}
    }
  }
  return null;
}

function rewriteReferenceValue(referencePath: string, oldValue: string, oldTarget: string, newTarget: string, sourceReferencePath = referencePath, repoRoot?: string): string {
  const split = splitTokenSuffix(oldValue);
  const absoluteRoot = repoRoot?.replaceAll("\\", "/").replace(/\/+$/u, "");
  if (absoluteRoot && split.path.startsWith(`${absoluteRoot}/`)) return `${absoluteRoot}/${newTarget}${split.suffix}`;
  if (/^[\w.]+$/.test(split.path) && oldTarget.endsWith(".py")) {
    const modulePath = newTarget.replace(/(?:\/__init__)?\.py$/, "").replaceAll("/", ".");
    return `${modulePath}${split.suffix}`;
  }
  const absoluteStyle = split.path.startsWith("/");
  const relativeStyle = split.path.startsWith("./") || split.path.startsWith("../");
  let localBareStyle = false;
  if (!absoluteStyle && !relativeStyle) {
    try {
      localBareStyle = normalizeRelative(posix.join(dirname(sourceReferencePath), split.path)) === oldTarget;
    } catch {}
  }
  const omittedExtension = !posix.extname(split.path);
  let value = absoluteStyle ? `/${newTarget}` : relativeStyle || localBareStyle ? posix.relative(dirname(referencePath), newTarget) : newTarget;
  if (relativeStyle && !value.startsWith(".")) value = `./${value}`;
  if (omittedExtension) {
    const finalName = basename(newTarget);
    const extensionStart = finalName.indexOf(".");
    const extensionChain = extensionStart <= 0 ? "" : finalName.slice(extensionStart);
    if (extensionChain && value.endsWith(extensionChain)) value = value.slice(0, -extensionChain.length);
  }
  if (oldValue.includes("\\")) value = value.replaceAll("/", "\\");
  return `${value}${split.suffix}`;
}

function rewriteReferenceToken(referencePath: string, sourceReferencePath: string, token: ReferenceToken, oldTarget: string, newTarget: string, repoRoot?: string, entries: readonly TaxonomyInventoryEntry[] = []): string {
  if (token.rewriteKind === "rust-path-join") {
    const base = token.rewriteData?.sourceBase;
    if (base === undefined) throw new Error("Rust join has no exact physical base");
    let value = posix.relative(projectedPath(base, entries), newTarget);
    if (token.value.startsWith("./") && !value.startsWith(".")) value = `./${value}`;
    return value || ".";
  }
  if (token.rewriteKind === "rust-mod") {
    let relativeTarget = posix.relative(dirname(referencePath), newTarget);
    if (!relativeTarget.startsWith(".")) relativeTarget = `./${relativeTarget}`;
    const indentation = token.rewriteData?.indentation ?? "";
    const declaration = token.rewriteData?.declaration ?? token.value;
    return `#[path = ${JSON.stringify(relativeTarget)}]\n${indentation}${declaration}`;
  }
  if (token.rewriteKind === "python-entrypoint") {
    const targetValue = token.targetValues?.[0] ?? token.value;
    return `${rewriteReferenceValue(referencePath, targetValue, oldTarget, newTarget, sourceReferencePath)}${token.rewriteData?.suffix ?? ""}`;
  }
  if (token.rewriteKind === "artifact-uri") {
    const artifactRoot = token.rewriteData?.artifactRoot;
    if (!artifactRoot || !newTarget.startsWith(`${artifactRoot}/`)) throw new Error(`Artifact URI target escapes its captured owner: ${newTarget}`);
    return `asset://${newTarget.slice(artifactRoot.length + 1)}`;
  }
  if (token.rewriteKind === "path-prefix") return `${token.rewriteData?.prefix ?? ""}${newTarget}${token.rewriteData?.suffix ?? ""}`;
  if (token.rewriteKind === "projection-prose" && token.value.startsWith("🏅️standards/")) {
    const artifactRoot = token.rewriteData?.artifactRoot;
    if (!artifactRoot || !newTarget.startsWith(`${artifactRoot}/`)) throw new Error(`Projection prose target escapes its captured owner: ${newTarget}`);
    return newTarget.slice(artifactRoot.length + 1);
  }
  return rewriteReferenceValue(referencePath, token.value, oldTarget, newTarget, sourceReferencePath, repoRoot);
}

function unsupportedReferenceTokens(content: string, adapter: TaxonomyReferenceAdapter): readonly ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  const patterns = [/"((?:\\.|[^"\\\r\n])+)"|'((?:\\.|[^'\\\r\n])+)'|`((?:\\.|[^`\\\r\n])+)`/gu, /(?:^|[\s(=,:])((?:\.\.?\/|\/)?[^\s"'`()\],;]+\/[^\s"'`()\],;]+|[A-Za-z0-9_.@-]+\.[A-Za-z0-9.]+)(?=$|[\s"'`),;\]])/gmu];
  for (const pattern of patterns) {
    for (const match of content.matchAll(pattern)) {
      if (match.index === undefined) continue;
      const value = match[1] ?? match[2] ?? match[3];
      if (!value || /^\/\//u.test(value) || /^[\\/]+$/u.test(value) || /^(?:\/{2,}|\.{1,2}\/?|\*+)$/u.test(value) || !/[\\/]/u.test(value) && !/^\.{1,2}$/u.test(value) && !/\.[A-Za-z0-9][A-Za-z0-9.-]*$/u.test(value)) continue;
      const start = match.index + match[0].indexOf(value);
      const decoded = value.replace(/\\(?:u\{([a-f0-9]{1,6})\}|u([a-f0-9]{4})|x([a-f0-9]{2})|([\\/"'`]))/giu, (sequence, point: string | undefined, unit: string | undefined, byte: string | undefined, escaped: string | undefined) => {
        const numeric = point ?? unit ?? byte;
        if (!numeric) return escaped ?? sequence;
        const code = Number.parseInt(numeric, 16);
        return code <= 0x10ffff ? String.fromCodePoint(code) : sequence;
      });
      rows.push({ adapter, structuredLocation: lineLocation(content, start, "unsupported-path-syntax"), start, end: start + value.length, value, ...(decoded === value ? {} : { targetValues: [decoded] }) });
    }
  }
  return rows;
}

function referenceAdapter(path: string): TaxonomyReferenceAdapter {
  const lower = path.toLocaleLowerCase("und");
  if (lower.endsWith(".feature")) return "gherkin";
  if (lower.endsWith(".rs")) return "rust";
  if (/\.(?:ts|tsx|js|jsx|mjs|cjs|mts|cts)$/u.test(lower)) return "typescript";
  if (/\.(?:go|mod|work)$/u.test(lower)) return "go";
  if (lower.endsWith(".py")) return "python";
  if (/\.(?:cs|fs|vb|csproj|fsproj|vbproj|sln|props|targets)$/u.test(lower)) return "dotnet";
  if (/\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx|cmake)$/u.test(lower) || basename(path) === "CMakeLists.txt") return "native";
  if (lower.endsWith(".jsonc")) return "jsonc";
  if (lower.endsWith(".json")) return "json";
  if (lower.endsWith(".toml")) return "toml";
  if (/\.ya?ml$/u.test(lower)) return "yaml";
  if (/\.(?:xml|html|htm)$/u.test(lower)) return "xml";
  return "markdown";
}

function applyEditsToContent(content: string, edits: readonly ReferenceEdit[]): string {
  let result = content;
  const offset = (edit: ReferenceEdit): number => {
    const value = edit.structuredLocation.match(/@(\d+)$/)?.[1];
    if (value === undefined) throw new Error(`Reference edit lacks a structured offset at ${edit.path}:${edit.structuredLocation}`);
    return Number.parseInt(value, 10);
  };
  const sorted = [...edits].sort((a, b) => offset(b) - offset(a) || b.structuredLocation.localeCompare(a.structuredLocation));
  for (const edit of sorted) {
    const start = offset(edit);
    const end = start + edit.oldValue.length;
    if (result.slice(start, end) !== edit.oldValue) throw new Error(`Reference edit preimage mismatch at ${edit.path}:${edit.structuredLocation}`);
    result = `${result.slice(0, start)}${edit.newValue}${result.slice(end)}`;
  }
  return result;
}

function referenceGraph(repoRoot: string, entries: ReadonlyMap<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy, progress?: TaxonomyInventoryOptions["progress"], cancelFile?: string): void {
  const known = referencePathIndex(entries.keys());
  const files = [...entries.values()].filter((entry) => entry.nodeKind === "file" && textualPath(entry.sourcePath) && (entry.size ?? 0) <= 16 * 1024 * 1024);
  report(progress, "inventory", "references", 0, files.length);
  for (let index = 0; index < files.length; index++) {
    checkCancellation(repoRoot, cancelFile);
    const entry = files[index];
    if (isExcluded(entry.sourcePath, taxonomy)) {
      report(progress, "inventory", "references", index + 1, files.length, entry.sourcePath);
      continue;
    }
    let content: string;
    try {
      content = readFileSync(absolutePath(repoRoot, entry.sourcePath), "utf8");
    } catch {
      report(progress, "inventory", "references", index + 1, files.length, entry.sourcePath);
      continue;
    }
    for (const token of referenceTokens(entry.sourcePath, content, known)) {
      if (token.unsupportedReason) continue;
      const target = resolveReferenceTokenPath(entry.sourcePath, token, known);
      if (!target || !entries.has(target)) {
        const opaque = lexicalOpaqueReferenceTarget(entry.sourcePath, token, taxonomy);
        if (opaque) entry.violations.push(violation("opaque-reference-target", entry.sourcePath, `${token.adapter} ${token.structuredLocation} lexically targets excluded ${opaque}`, "warning"));
        continue;
      }
      entry.referencesOut.push(target);
      entries.get(target)?.referencesIn.push(entry.sourcePath);
    }
    report(progress, "inventory", "references", index + 1, files.length, entry.sourcePath);
  }
  checkCancellation(repoRoot, cancelFile);
  for (const entry of entries.values()) {
    entry.referencesIn = [...new Set(entry.referencesIn)].sort();
    entry.referencesOut = [...new Set(entry.referencesOut)].sort();
  }
}

function referenceEditIdentity(edit: ReferenceEdit): string {
  return `${edit.path}\u0000${edit.structuredLocation}\u0000${edit.oldValue}\u0000${edit.newValue}`;
}

interface ArtifactReferenceProjection {
  readonly id: string;
  readonly artifactRoot: string;
  readonly sourceRoot: string;
  readonly destinationRoot: string;
  readonly rationaleRule: ArtifactProjectionRationale;
  readonly catalog: SemanticDistributedJsonManifestCatalogContract | SemanticExactOwnerVectorsCatalogContract;
  readonly mappings: readonly Readonly<{ sourcePath: string; destinationPath: string }>[];
  readonly authorityReferenceEdits: readonly Readonly<{ path: string; adapter: "json" | "toml"; structuredLocation: string; oldValue: string; newValue: string; preimageHash: string }>[];
  readonly authorityProblems: readonly string[];
}

function artifactReferenceProjections(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): readonly ArtifactReferenceProjection[] {
  const rows: ArtifactReferenceProjection[] = [];
  const entries = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const catalog = taxonomy.schema.semanticPathProjectionCatalogContracts[contract.catalogContractId];
    if (!("contractKind" in catalog)) continue;
    for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind === "directory")) {
      const location = artifactProjectionSourceLocation(entry.sourcePath, contract, taxonomy);
      if (!location || location.sourceRoot !== entry.sourcePath) continue;
      const mappings = moves.filter((move) => move.rationaleRule === contract.rationaleRule && move.sourcePath.startsWith(`${location.sourceRoot}/`)).map(({ sourcePath, destinationPath }) => ({ sourcePath, destinationPath }));
      if (mappings.length === 0) continue;
      const rendered = renderArtifactPathProjectionRoot({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot }, taxonomy.discoverySchema);
      const authority = semanticPathProjectionAuthority({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot, nodes: artifactProjectionAuthorityNodes(inventory.repoRoot, location.sourceRoot, entries, taxonomy) }, taxonomy.discoverySchema);
      const orderedMappings = mappings.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
      const mappingProblems = canonicalJson(orderedMappings) === canonicalJson(authority.mappings) ? [] : ["Planned artifact mappings do not equal the schema projection authority"];
      rows.push({ id, artifactRoot: location.artifactRoot, sourceRoot: location.sourceRoot, destinationRoot: rendered.destinationRoot, rationaleRule: contract.rationaleRule, catalog, mappings: orderedMappings, authorityReferenceEdits: authority.referenceEdits, authorityProblems: [...rendered.problems, ...authority.problems, ...mappingProblems] });
    }
  }
  return rows.sort((left, right) => generatorPathCompare(left.sourceRoot, right.sourceRoot) || left.id.localeCompare(right.id));
}

/** 🗿️ Returns the canonical artifact-owned suffix used to bind structural reference tokens. */
export function artifactProjectionTail(path: string): string {
  const marker = "🗿️artifacts/";
  const index = path.indexOf(marker);
  return index < 0 ? path : path.slice(index);
}

function artifactReferenceForm(token: ReferenceToken): SemanticPathProjectionReferenceConsumerForm | null {
  if (token.rewriteKind === "artifact-catalog-glob") return "artifact-catalog-glob";
  if (token.rewriteKind !== "artifact-catalog-prose") return null;
  const form = token.rewriteData?.form;
  return form === "root-marker" || form === "relative-root" || form === "interaction-glob" || form === "catalog-grammar" ? `artifact-catalog-prose:${form}` : null;
}

function registeredArtifactConsumers(context: ArtifactReferenceProjection, referencePath: string, token: ReferenceToken, taxonomy: LoadedTaxonomy): readonly string[] {
  const form = artifactReferenceForm(token);
  if (!form || !["rust", "typescript", "json", "toml"].includes(token.adapter)) return [];
  return semanticPathProjectionReferenceConsumers(context.id, referencePath, token.adapter as "rust" | "typescript" | "json" | "toml", form, taxonomy.discoverySchema).map((row) => row.id);
}

function catalogProjectionForToken(referencePath: string, token: ReferenceToken, contexts: readonly ArtifactReferenceProjection[], taxonomy: LoadedTaxonomy): ArtifactReferenceProjection | string | null {
  const selectors = token.rewriteData?.selectors ? JSON.parse(token.rewriteData.selectors) as string[] : [];
  const matches = contexts.filter((context) => {
    if (context.rationaleRule !== "artifact-example-model-catalog-projection-v1") return false;
    const authorized = referencePath === context.artifactRoot || referencePath.startsWith(`${context.artifactRoot}/`) || registeredArtifactConsumers(context, referencePath, token, taxonomy).length === 1;
    const selectorMatches = selectors.length === 0 || selectors.some((selector) => selector.includes(artifactProjectionTail(context.sourceRoot)));
    return authorized && selectorMatches;
  });
  if (matches.length === 1) return matches[0];
  if (matches.length > 1) return `Reference form matches multiple artifact projection owners: ${matches.map((row) => row.id).join(", ")}`;
  const cad = contexts.filter((context) => context.rationaleRule === "artifact-example-model-catalog-projection-v1");
  if (cad.length > 0) return selectors.length > 0 ? "Artifact selector or reference file does not match a registered source owner" : "Artifact catalog prose occurs outside an authorized owner or consumer location";
  return null;
}

function renderCatalogGlob(referencePath: string, token: ReferenceToken, context: ArtifactReferenceProjection): string | Readonly<{ problem: string }> {
  if (context.catalog.contractKind !== "distributed-json-manifest-catalog") return { problem: `${context.id} has no distributed catalog grammar` };
  const selectors = JSON.parse(token.rewriteData?.selectors ?? "[]") as string[];
  if (selectors.length === 0 || selectors.some((selector) => typeof selector !== "string")) return { problem: "Artifact catalog glob has no exact literal selectors" };
  const pathMatcher = createTaxonomyPathMatcher();
  const sourceTail = artifactProjectionTail(context.sourceRoot);
  const baseRelative = posix.relative(dirname(referencePath), context.destinationRoot);
  const base = baseRelative.startsWith(".") ? baseRelative : `./${baseRelative}`;
  const zeroSource = [/\/\*\*\/🔣️extension\.json$/u, /\/\*\*\/🏷️properties\/\*\.json$/u, /\/\*\*\/🔧️properties\/\*\.json$/u];
  const rendered: string[] = [];
  for (const selector of selectors) {
    const tailIndex = selector.indexOf(sourceTail);
    if (tailIndex < 0) return { problem: `Artifact selector does not contain its registered source owner: ${selector}` };
    const suffix = selector.slice(tailIndex + sourceTail.length);
    const sourcePattern = `${context.sourceRoot}${suffix}`;
    const admitted = context.mappings.filter((mapping) => pathMatcher.matches(mapping.sourcePath, sourcePattern));
    if (admitted.length === 0) {
      if (zeroSource.some((pattern) => pattern.test(suffix))) continue;
      return { problem: `Nonempty artifact selector has no exact authority mapping: ${selector}` };
    }
    if (suffix.endsWith(`/${context.catalog.modelManifestSourceFilename}`)) {
      rendered.push(`${base}/*/🔣️.json`);
      continue;
    }
    const rules = context.catalog.categoryRules.filter((rule) => suffix.includes(`/${rule.sourceDirectoryName}/`));
    if (rules.length !== 1) return { problem: `Artifact selector has no unique registered category: ${selector}` };
    rendered.push(`${base}/**/${rules[0].sourceDirectoryName}/${rules[0].sourceShape === "nested-fixed-json" ? "**" : "*"}/🔣️.json`);
  }
  if (rendered.length === 0) {
    if (!/^\s*[A-Za-z_$][\w$]*\s*:\s*import\.meta\.glob\([^\r\n]+\),?\s*(?:as\s+[^;]+)?;?\s*(?:\r?\n)?$/u.test(token.value)) return { problem: "Zero-source artifact selectors cannot be removed without owning their complete object member" };
    return "";
  }
  if (rendered.length !== selectors.length) return { problem: "Artifact selector list mixes registered and zero-source selectors" };
  let result = token.value;
  for (let index = 0; index < selectors.length; index++) {
    const quoted = JSON.stringify(selectors[index]);
    if (!result.includes(quoted)) return { problem: "Artifact selector raw literal is not canonical JSON-compatible syntax" };
    result = result.replace(quoted, JSON.stringify(rendered[index]));
  }
  return result;
}

function artifactStructuralReferenceRewrite(referencePath: string, token: ReferenceToken, contexts: readonly ArtifactReferenceProjection[], taxonomy: LoadedTaxonomy): Readonly<{ newValue?: string; problem?: string }> | null {
  if (token.rewriteKind !== "artifact-catalog-glob" && token.rewriteKind !== "artifact-catalog-prose") return null;
  const selected = catalogProjectionForToken(referencePath, token, contexts, taxonomy);
  if (typeof selected === "string") return { problem: selected };
  if (!selected) return null;
  if (token.rewriteKind === "artifact-catalog-glob") {
    const rendered = renderCatalogGlob(referencePath, token, selected);
    return typeof rendered === "string" ? { newValue: rendered } : rendered;
  }
  if (selected.catalog.contractKind !== "distributed-json-manifest-catalog") return { problem: `${selected.id} cannot render catalog prose` };
  const root = posix.relative(selected.artifactRoot, selected.destinationRoot);
  if (token.rewriteData?.form === "root-marker") return { newValue: `${root}/` };
  if (token.rewriteData?.form === "relative-root") {
    const value = posix.relative(dirname(referencePath), selected.destinationRoot);
    return { newValue: value.startsWith(".") ? value : `./${value}` };
  }
  if (token.rewriteData?.form === "interaction-glob") return { newValue: `${root}/*/🎬️interactions/*/🔣️.json` };
  if (token.rewriteData?.form === "catalog-grammar") {
    const members = selected.catalog.categoryRules.map((rule) => `${rule.sourceDirectoryName}/<member>/🔣️.json`).sort(generatorPathCompare);
    return { newValue: `${root}/<model>/{${members.join(",")},🔣️.json}` };
  }
  return { problem: `Unknown artifact catalog prose form ${token.rewriteData?.form ?? ""}` };
}

function externalProjectionReferenceEntries(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], contexts: readonly ArtifactReferenceProjection[], taxonomy: LoadedTaxonomy): readonly TaxonomyInventoryEntry[] {
  const active = new Set(contexts.map((context) => context.id));
  if (moves.some((move) => move.rationaleRule === "artifact-mutation-test-projection-v1")) active.add(taxonomy.schema.mutationCatalogProjection.projectionContractId);
  const admitted = new Set(inventory.entries.map((entry) => entry.sourcePath));
  const rows: TaxonomyInventoryEntry[] = [];
  for (const path of Object.values(taxonomy.schema.semanticPathProjectionReferenceConsumerContracts)
    .filter((contract) => active.has(contract.projectionContractId))
    .flatMap((contract) => contract.sourcePathIdentities)
    .filter((path, index, values) => values.indexOf(path) === index && !admitted.has(path))
    .sort(generatorPathCompare)) {
    if (isExcluded(path, taxonomy)) throw new Error(`Declared projection consumer crosses opaque path ${path}`);
    const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, path, "Declared projection consumer", true);
    const stat = lstatOrNull(absolute);
    if (!stat) continue;
    if (!stat.isFile() || stat.isSymbolicLink()) throw new Error(`Declared projection consumer is not a regular file: ${path}`);
    const bytes = readFileSync(absolute);
    rows.push({ sourcePath: path, normalizedPath: path, nodeKind: "file", ownerId: ownerId(path), areaId: areaId(path), fileKind: null, semanticStem: null, contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength, referencesIn: [], referencesOut: [], violations: [] });
  }
  return rows;
}

function exactOwnedReferenceTokens(path: string, content: string, catalog: SemanticExactOwnedFileCatalog | null, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): Readonly<{ tokens: readonly ReferenceToken[]; problems: readonly string[] }> {
  if (!catalog) return { tokens: [], problems: [] };
  const moving = new Set(moves.filter((move) => move.rationaleRule === "readme-license-owner-projection-v1").map((move) => move.sourcePath));
  const owners = catalog.cases.filter((entry) => moving.has(entry.sourcePath));
  const tokens: ReferenceToken[] = [], problems: string[] = [];
  const authoredMove = moves.find((move) => move.sourcePath === path && moving.has(move.sourcePath));
  const authoredContract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  if (authoredMove && authoredContract?.contractKind === "exact-owner-path-catalog") {
    const result = semanticExactOwnedDocumentCorrectionAuthority(catalog, authoredContract, { path, finalPath: authoredMove.destinationPath, content, mode: authoredMove.sourcePreimage.mode, moving: true });
    problems.push(...result.problems);
    for (const splice of result.splices) tokens.push({ adapter: "markdown", structuredLocation: lineLocation(content, splice.start, "exact-owner-authored-" + splice.correctionId), start: splice.start, end: splice.end, value: splice.oldValue, targetValues: [path], rewriteKind: "exact-owner-reference", rewriteData: { newValue: splice.newValue } });
  }
  for (const id of ["repo-cli-dev-docs-go", "commonmark-scratch-rust-reader"]) {
    const selected = owners.filter((entry) => entry.referenceOwnerIds.includes(id));
    if (selected.length === 0 || catalog.referenceOwners[id].ownerPath !== path) continue;
    const forms = id === "repo-cli-dev-docs-go" ? [
      { oldValue: 'filepath.Join(rootDir, technology.Root, "README.md")', newValue: 'filepath.Join(rootDir, technology.Root, "📃️readme", "📝️.md")', count: 2 },
      { oldValue: 'filepath.Join(rootDir, bundle.Root, "README.md")', newValue: 'filepath.Join(rootDir, bundle.Root, "📃️readme", "📝️.md")', count: 2 },
      { oldValue: 'filepath.Join(bundleRoot, "README.md")', newValue: 'filepath.Join(bundleRoot, "📃️readme", "📝️.md")', count: 2 },
      { oldValue: 'filepath.Join(rootDir, name, "README.md")', newValue: 'filepath.Join(rootDir, name, "📃️readme", "📝️.md")', count: 2 },
      { oldValue: 'd.Name() == "README.md"', newValue: 'd.Name() == "📝️.md" && filepath.Base(filepath.Dir(path)) == "📃️readme"', count: 1 },
    ] : [{ oldValue: 'std::fs::read_to_string("README.md")', newValue: 'std::fs::read_to_string("📃️readme/📝️.md")', count: 1 }];
    for (const [index, form] of forms.entries()) {
      const count = (value: string): number => content.split(value).length - 1;
      if (count(form.oldValue) + count(form.newValue) !== form.count) {
        problems.push(id + " concrete consumer form " + index + " drifted");
        continue;
      }
      for (let start = content.indexOf(form.oldValue); start >= 0; start = content.indexOf(form.oldValue, start + form.oldValue.length)) {
        tokens.push({ adapter: id === "repo-cli-dev-docs-go" ? "go" : "rust", structuredLocation: lineLocation(content, start, "exact-owner-" + id + "-" + index), start, end: start + form.oldValue.length, value: form.oldValue, targetValues: [selected[0].sourcePath], rewriteKind: "exact-owner-reference", rewriteData: { newValue: form.newValue } });
      }
    }
  }
  return { tokens, problems };
}

function externalExactOwnedReferenceEntries(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], catalog: SemanticExactOwnedFileCatalog | null, taxonomy: LoadedTaxonomy): readonly TaxonomyInventoryEntry[] {
  if (!catalog || !moves.some((move) => move.rationaleRule === "readme-license-owner-projection-v1")) return [];
  const admitted = new Set(inventory.entries.map((entry) => entry.sourcePath));
  const moving = new Set(moves.map((move) => move.sourcePath));
  const required = new Set(catalog.cases.filter((entry) => moving.has(entry.sourcePath)).flatMap((entry) => entry.referenceOwnerIds).filter((id) => ["repo-cli-dev-docs-go", "commonmark-scratch-rust-reader"].includes(id)).map((id) => catalog.referenceOwners[id].ownerPath));
  const paths = new Set(required);
  for (const entry of catalog.cases) {
    const source = assertLexicalInputOutsideOpaque(inventory.repoRoot, entry.sourcePath, "Exact owner reference", true);
    const active = lstatOrNull(source) ? entry.sourcePath : entry.destinationPath;
    const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, active, "Exact owner reference", true);
    if (lstatOrNull(absolute)?.isFile()) paths.add(active);
  }
  const rows: TaxonomyInventoryEntry[] = [];
  for (const path of [...paths].sort(generatorPathCompare)) {
    if (admitted.has(path)) continue;
    if (isExcluded(path, taxonomy)) throw new Error("Exact owner consumer crosses an opaque path: " + path);
    const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, path, "Exact owner consumer", true), stat = lstatOrNull(absolute);
    if (!stat?.isFile()) {
      if (required.has(path)) throw new Error("Exact owner consumer is absent or not a regular file: " + path);
      continue;
    }
    const bytes = readFileSync(absolute);
    rows.push({ sourcePath: path, normalizedPath: path, nodeKind: "file", ownerId: ownerId(path), areaId: areaId(path), fileKind: null, semanticStem: null, contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength, referencesIn: [], referencesOut: [], violations: [] });
  }
  return rows;
}

function exactOwnedMarkdownTarget(referencePath: string, token: ReferenceToken, catalog: SemanticExactOwnedFileCatalog | null, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): string | null {
  if (token.rewriteKind === "exact-owner-reference" || token.adapter !== "markdown" || !catalog?.cases.some((entry) => entry.sourcePath === referencePath && entry.referenceOwnerIds.includes("markdown-relative-reference-adapter")) || !moves.some((move) => move.sourcePath === referencePath && move.rationaleRule === "readme-license-owner-projection-v1")) return null;
  const value = splitTokenSuffix(token.value).path;
  if (!value || /^(?:[a-z][a-z0-9+.-]*:|\/|#|\$|\{)/iu.test(value) || /[*{}]/u.test(value)) return null;
  try {
    const path = normalizeRelative(posix.join(dirname(referencePath), decodeURI(value)));
    return isExcluded(path, taxonomy) ? null : path;
  } catch { return null; }
}

function artifactEmptyFacetReferenceMove(moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): TaxonomyMove | undefined {
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"];
  if (contract?.contractKind !== "semantic-facet-primary-file") return undefined;
  return moves.find((move) => move.rationaleRule === contract.rationaleRule && move.sourcePath.startsWith(`${contract.referenceConsumer.ownerRoot}/`));
}

function externalArtifactEmptyFacetReferenceEntries(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): readonly TaxonomyInventoryEntry[] {
  if (!artifactEmptyFacetReferenceMove(moves, taxonomy)) return [];
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"] as SemanticFacetPrimaryFileProjectionContract;
  const path = contract.referenceConsumer.path;
  if (inventory.entries.some((entry) => entry.sourcePath === path)) return [];
  const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, path, "Artifact empty-facet consumer", true), stat = lstatOrNull(absolute);
  if (!stat) return [];
  if (!stat.isFile()) throw new Error(`Artifact empty-facet consumer must be a regular file: ${path}`);
  const bytes = readFileSync(absolute);
  return [{ sourcePath: path, normalizedPath: path, nodeKind: "file", ownerId: ownerId(path), areaId: areaId(path), fileKind: null, semanticStem: null, contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength, referencesIn: [], referencesOut: [], violations: [] }];
}

function artifactEmptyFacetReferenceTokens(path: string, content: string, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): Readonly<{ tokens: readonly ReferenceToken[]; problems: readonly string[] }> {
  const move = artifactEmptyFacetReferenceMove(moves, taxonomy);
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"];
  if (!move || contract?.contractKind !== "semantic-facet-primary-file" || contract.referenceConsumer.path !== path) return { tokens: [], problems: [] };
  const consumer = contract.referenceConsumer;
  const boundaries = [...content.matchAll(/^\/\/#(region|endregion) ([^\r\n]+)$/gmu)].filter((match) => match[2] === consumer.region);
  const invalid = { tokens: [], problems: ["Artifact empty-facet energy surface-region prose drifted"] };
  if (boundaries.length !== 2 || boundaries[0]![1] !== "region" || boundaries[1]![1] !== "endregion") return invalid;
  const oldValue = consumer.lineTemplate.replace("{filename}", contract.sourceFilename);
  const fileKind = taxonomy.schema.fileKinds[taxonomy.schema.windowEmptyFacetFileKindId]!;
  const newValue = consumer.lineTemplate.replace("{filename}", `${fileKind.emoji}${fileKind.extensionChains[0]}`);
  const lines = [...content.matchAll(/^[^\r\n]*$/gmu)].filter((match) => match.index! > boundaries[0]!.index! && match.index! < boundaries[1]!.index! && (match[0] === oldValue || match[0] === newValue));
  if (lines.length !== 1 || content.split(contract.sourceFilename).length - 1 !== Number(lines[0]![0] === oldValue)) return invalid;
  if (lines[0]![0] === newValue) return { tokens: [], problems: [] };
  const start = lines[0]!.index!;
  return { tokens: [{ adapter: "rust", structuredLocation: lineLocation(content, start, "artifact-empty-facet-energy-surfaces"), start, end: start + oldValue.length, value: oldValue, targetValues: [move.sourcePath], rewriteKind: "exact-owner-reference", rewriteData: { newValue } }], problems: [] };
}

function nestedCargoReferenceTokens(path: string, content: string, packages: readonly SemanticPackageProjectionCase[]): Readonly<{ tokens: readonly ReferenceToken[]; problems: readonly string[] }> {
  const row = packages.find((entry) => entry.mappings.some((mapping) => mapping.sourcePath === path));
  if (!row) return { tokens: [], problems: [] };
  const mapping = row.mappings.find((entry) => entry.sourcePath === path)!;
  if (sha256(content) !== mapping.sourceHash || Buffer.byteLength(content) !== mapping.sourceSize) return { tokens: [], problems: ["Nested Cargo reference source preimage drift: " + path] };
  const tokens: ReferenceToken[] = [], problems: string[] = [];
  const authored = semanticPackageAuthoredFragmentReferences({ path, content, layout: "source" }, row);
  problems.push(...authored.problems);
  for (const reference of authored.references) tokens.push({ adapter: referenceAdapter(path), structuredLocation: lineLocation(content, reference.start, "nested-cargo-authored-fragment"), start: reference.start, end: reference.end, value: reference.oldValue, targetValues: [reference.targetSourcePath], rewriteKind: "exact-owner-reference", rewriteData: { newValue: reference.newValue } });
  for (const binding of row.joinedPathBindings.filter((binding) => path === row.sourceRoot + "/" + binding.consumerRelativePath)) {
    const authority = semanticPackageJoinedPathReferenceAuthority({ path, content, layout: "source" }, binding, row);
    problems.push(...authority.problems);
    for (const reference of authority.references) tokens.push({ adapter: "typescript", structuredLocation: lineLocation(content, reference.start, "nested-cargo-joined-path"), start: reference.start, end: reference.end, value: reference.oldValue, targetValues: [reference.targetSourcePath], rewriteKind: "exact-owner-reference", rewriteData: { newValue: reference.newValue } });
  }
  const add = (needle: string, value: string, newValue: string, target: string, adapter: "rust" | "toml" | "json" | "typescript") => {
    const start = content.indexOf(needle);
    if (start < 0 || content.indexOf(needle, start + needle.length) >= 0) { problems.push("Nested Cargo exact reference is missing or repeated: " + needle); return; }
    const offset = start + needle.indexOf(value);
    tokens.push({ adapter, structuredLocation: lineLocation(content, offset, "nested-cargo-reference"), start: offset, end: offset + value.length, value, targetValues: [target], rewriteKind: "exact-owner-reference", rewriteData: { newValue } });
  };
  for (const splice of row.sourceSplices) if (path === splice.sourcePath) add(splice.oldValue, splice.oldValue, splice.newValue, splice.sourcePath, "rust");
  if (path === row.sourceRoot + "/Cargo.toml") {
    if (row.id === "jcoprobe-guest") add('path = "🦀️.rs"', "🦀️.rs", "📚️library/🦀️.rs", row.sourceRoot + "/🦀️.rs", "toml");
    else {
      add('build = "build.rs"', "build.rs", "🏗️builder/🦀️.rs", row.sourceRoot + "/build.rs", "toml");
      add('path = "🦀️lib.rs"', "🦀️lib.rs", "📚️library/🦀️.rs", row.sourceRoot + "/🦀️lib.rs", "toml");
      add('path = "📦️bin.rs"', "📦️bin.rs", "💾️binary/🦀️.rs", row.sourceRoot + "/📦️bin.rs", "toml");
    }
  }
  if (row.id === "wgpu-renderer" && path === row.sourceRoot + "/package.json") {
    add('".": "./🟦️.ts"', ".", ".", row.sourceRoot, "json");
    add('".": "./🟦️.ts"', "./🟦️.ts", "./🟦️typescript/📚️library/🟦️.ts", row.sourceRoot + "/🟦️.ts", "json");
    add('"directory": "framework/product/os/module/renderer/wgpu"', "framework/product/os/module/renderer/wgpu", row.destinationRoot, row.sourceRoot, "json");
  }
  if (row.id === "wgpu-renderer" && path === row.sourceRoot + "/📋️project.json") {
    const named = content.match(/"namedInputs"\s*:\s*\{\s*"default"\s*:\s*(\[[^\]]+\])/u);
    if (!named || !Array.isArray(JSON.parse(named[1]!))) problems.push("Nested Cargo exact Nx default inputs drifted");
    else add(named[0], named[1]!, JSON.stringify([...JSON.parse(named[1]!), `{workspaceRoot}/${row.semanticOwnerRoot}/**/*`]), row.sourceRoot, "json");
  }
  if (row.id === "wgpu-renderer" && path === row.sourceRoot + "/📜️script.ts") {
    const config = row.mappings.find((entry) => entry.sourcePath === row.sourceRoot + "/🧪️vitest.config.ts")!;
    const calls = [...content.matchAll(/^    await runVitest\(this\.root, [^\r\n]+\);$/gmu)];
    if (calls.length !== 3 || calls.some((call) => !call[0].endsWith(', "🧪️vitest.config.ts");'))) problems.push("Nested Cargo exact Vitest call authority drifted");
    else for (const call of calls) {
      add(call[0], basename(config.sourcePath), posix.relative(row.destinationRoot, config.destinationPath), config.sourcePath, "typescript");
      for (const selector of row.mappings.filter((entry) => entry.sourcePath.endsWith(".test.ts") && call[0].includes(JSON.stringify(basename(entry.sourcePath))))) add(call[0], basename(selector.sourcePath), posix.relative(row.destinationRoot, selector.destinationPath), selector.sourcePath, "typescript");
    }
  }
  if (row.id === "jcoprobe-guest" && path === row.sourceRoot + "/🦀️.rs") {
    const target = row.sourceRoot + "/🧬️schema/📜️world.wit", rendered = "../📦️packages/🦀️rust/🧬️schema/📜️world.wit";
    add('path: "🧬️schema/📜️world.wit"', "🧬️schema/📜️world.wit", "🧬️schema/📜️world.wit", target, "rust");
    add("`👽️guest/🧬️schema/📜️world.wit`", "👽️guest/🧬️schema/📜️world.wit", rendered, target, "rust");
  }
  return { tokens, problems };
}

//#region 🔒️Dependency Policy State
interface DependencyPolicyStateTokens {
  readonly contentHash: string;
  readonly active: boolean;
  readonly manifestSources: ReadonlySet<string>;
  readonly tokens: readonly ReferenceToken[];
  readonly problems: readonly string[];
}

/** 🔒️ Rewrites only manifest-user coordinates, never dependency approvals or generated metadata. */
function dependencyPolicyStateTokens(repoRoot: string, path: string, content: string, packages: readonly SemanticPackageProjectionCase[], moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): DependencyPolicyStateTokens {
  const contract = taxonomy.discoverySchema.semanticPolicyStateCoordinateContracts["dependency-freeze-users-v1"]!;
  const owners = packages.filter((row) => contract.packageIds.some((id) => id === row.id));
  const active = path === contract.statePath && owners.length > 0, tokens: ReferenceToken[] = [], problems: string[] = [], manifestSources = new Set<string>();
  const finish = (): DependencyPolicyStateTokens => ({ contentHash: sha256(content), active, manifestSources: problems.length ? new Set() : manifestSources, tokens: problems.length ? [] : tokens, problems });
  if (!active) return finish();
  try {
    const baseline = record(JSON.parse(content), "Dependency policy state"), coordinates = jsonStringCoordinates(content);
    if (baseline.schemaVersion !== contract.stateSchemaVersion || typeof baseline.generatedAt !== "string" || typeof baseline.commit !== "string" || !Array.isArray(baseline.entries) || coordinates.length === 0) throw new Error("Dependency policy state requires exact schema, metadata, entries and unambiguous JSON coordinates");
    const identities = new Set<string>();
    const entries = baseline.entries.map((value, index) => {
      const entry = record(value, `Dependency policy entry ${index}`), key = `${entry.ecosystem}:${entry.name}`;
      if (typeof entry.ecosystem !== "string" || typeof entry.name !== "string" || !entry.name || typeof entry.version !== "string" || !Array.isArray(entry.kinds) || entry.kinds.some((kind) => !["production-runtime", "production-build", "repository-tooling", "test-runner", "test-oracle"].includes(String(kind))) || typeof entry.productionReachable !== "boolean" || !Array.isArray(entry.users) || entry.users.some((user) => typeof user !== "string") || new Set(entry.users).size !== entry.users.length || identities.has(key)) throw new Error("Dependency policy state has missing users, duplicate identities/users or invalid classifications");
      identities.add(key);
      return entry as JsonRecord & { users: string[]; kinds: string[] };
    });
    for (const row of owners) {
      const source = row.sourceRoot + "/" + contract.manifestFilename, destination = row.destinationRoot + "/" + contract.manifestFilename;
      const mapping = row.mappings.find((entry) => entry.sourcePath === source), approved = moves.filter((move) => move.sourcePath === source && move.destinationPath === destination);
      const declaration = record(row.requiredManifestEvidence[contract.dependencyEvidenceField], "Dependency policy declaration");
      const bytes = readFileSync(assertLexicalInputOutsideOpaque(repoRoot, source, "Dependency policy manifest", true));
      if (!mapping || approved.length !== 1 || approved[0]!.sourcePreimage.nodeKind !== "file" || approved[0]!.sourcePreimage.contentHash !== mapping.sourceHash || approved[0]!.sourcePreimage.size !== mapping.sourceSize || sha256(bytes) !== mapping.sourceHash || bytes.byteLength !== mapping.sourceSize || typeof declaration.name !== "string" || typeof declaration.version !== "string") throw new Error("Dependency policy manifest is not one exact source-preimage-proven move");
      const owned = entries.flatMap((entry, index) => entry.users.includes(source) ? [{ entry, index }] : []);
      if (owned.length !== 1) throw new Error("Dependency policy manifest requires exactly one unambiguous approved user entry");
      if (entries.some((entry) => entry.users.some((user) => user.startsWith(row.sourceRoot + "/") && user !== source))) throw new Error("Dependency policy contains an ambiguous source/canonical or unproved package owner");
      const { entry, index } = owned[0]!;
      if (entry.ecosystem !== "rust" || entry.name !== declaration.name || entry.version !== declaration.version || !entry.kinds.includes("production-runtime") || entry.productionReachable !== true) throw new Error("Dependency policy manifest would introduce an unapproved dependency or classification");
      const pointer = `/entries/${index}/users/${entry.users.indexOf(source)}`, coordinate = coordinates.find((coordinate) => coordinate.pointer === pointer && coordinate.value === source);
      const occurrences = (value: unknown): number => typeof value === "string" ? Number(value === source) : Array.isArray(value) ? value.reduce((count, child) => count + occurrences(child), 0) : value && typeof value === "object" ? Object.entries(value).reduce((count, [key, child]) => count + Number(key === source) + occurrences(child), 0) : 0;
      if (!coordinate || occurrences(baseline) !== 1) throw new Error("Dependency policy requires one exact unescaped users token and no undeclared source-coordinate fields");
      manifestSources.add(source);
      tokens.push({ adapter: "json", structuredLocation: `${pointer}@${coordinate.start}`, start: coordinate.start, end: coordinate.end, value: source, targetValues: [source], rewriteKind: "exact-owner-reference", rewriteData: { newValue: destination } });
    }
  } catch (error) { problems.push(error instanceof Error ? error.message : String(error)); }
  return finish();
}
//#endregion 🔒️Dependency Policy State

function buildReferenceEdits(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy, options: TaxonomyPlanOptions, known: ReferencePathIndex, removals: readonly TaxonomyEvidenceRemoval[]): { readonly edits: readonly ReferenceEdit[]; readonly editTargets: ReadonlyMap<string, string>; readonly resultHashes: ReadonlyMap<string, string>; readonly resultSizes: ReadonlyMap<string, number>; readonly unresolved: readonly TaxonomyViolation[] } {
  const incoming = incomingReferenceSnapshot(inventory, taxonomy, options);
  const ownerCatalog = exactOwnedFileCatalog(inventory.repoRoot, taxonomy);
  const ownerConsumers = externalExactOwnedReferenceEntries(inventory, moves, ownerCatalog, taxonomy);
  const facetConsumers = externalArtifactEmptyFacetReferenceEntries(inventory, moves, taxonomy);
  known = referencePathIndex([...known.exact, ...incoming.paths, ...ownerConsumers.map((entry) => entry.sourcePath), ...facetConsumers.map((entry) => entry.sourcePath), ...(ownerCatalog?.cases.flatMap((entry) => [entry.sourcePath, entry.destinationPath]) ?? [])], inventory.repoRoot, incoming.coordinateRoots, undefined, options.cancelFile, new Set(inventory.entries.filter((entry) => entry.sourcePath !== entry.normalizedPath).map((entry) => entry.sourcePath)));
  const moveBySource = new Map(moves.map((move) => [move.sourcePath, move]));
  const destinationBySource = new Map(inventory.entries
    .filter((entry) => entry.sourcePath !== entry.normalizedPath && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0)
    .map((entry) => [entry.sourcePath, entry.normalizedPath]));
  const edits: ReferenceEdit[] = [];
  const editTargets = new Map<string, string>();
  const unresolved: TaxonomyViolation[] = [];
  const packageCatalog = semanticPackageProjectionCatalog(inventory.repoRoot, taxonomy.discoverySchema);
  const packages = packageCatalog?.packages.filter((row) => row.mappings.some((mapping) => moves.some((move) => move.sourcePath === mapping.sourcePath && move.destinationPath === mapping.destinationPath))) ?? [];
  const policyStates = new Map<string, DependencyPolicyStateTokens>();
  for (const contract of Object.values(taxonomy.discoverySchema.semanticPolicyStateCoordinateContracts)) if (packages.some((row) => contract.packageIds.some((id) => id === row.id))) {
    try {
      const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, contract.statePath, "Dependency policy state", true), stat = lstatOrNull(absolute);
      if (!stat) continue;
      if (!stat.isFile()) throw new Error("Dependency policy state must be a no-follow regular file");
      const state = dependencyPolicyStateTokens(inventory.repoRoot, contract.statePath, incoming.contents.get(contract.statePath) ?? readFileSync(absolute, "utf8"), packages, moves, taxonomy);
      policyStates.set(contract.statePath, state);
      for (const problem of state.problems) unresolved.push(violation("policy-state-authority-invalid", contract.statePath, problem));
    } catch (error) { unresolved.push(violation("policy-state-authority-invalid", contract.statePath, error instanceof Error ? error.message : String(error))); }
  }
  for (const row of packages) {
    if (row.id === "wgpu-renderer") unresolved.push(violation("nested-cargo-generation-unresolved", row.sourceRoot, "WGPU package generation and transaction acceptance must be complete before projection"));
    for (const consumer of packageCatalog!.referenceConsumers.filter((entry) => entry.packageId === row.id && entry.ownership === "generated")) {
      if (policyStates.get(consumer.path)?.manifestSources.has(row.sourceRoot + "/Cargo.toml")) continue;
      const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, consumer.path, "Nested Cargo generated consumer", true), stat = lstatOrNull(absolute);
      if (!stat) continue;
      const content = stat.isFile() ? readFileSync(absolute, "utf8") : "";
      const transform = packageCatalog!.referenceTokenTransforms[consumer.transformId]!;
      if (!stat.isFile() || !transform || content.replaceAll(transform.destinationToken, "").includes(transform.sourceToken)) unresolved.push(violation("nested-cargo-generated-consumer-unresolved", consumer.path, `Generated ${row.id} consumer needs an exact regeneration authority before projection`));
    }
  }
  const resultHashes = new Map<string, string>();
  const resultSizes = new Map<string, number>();
  const accountedIncoming = new Set<string>();
  const artifactContexts = artifactReferenceProjections(inventory, moves, taxonomy);
  for (const context of artifactContexts) for (const problem of context.authorityProblems) unresolved.push(violation("projection-reference-authority-invalid", context.sourceRoot, `${context.id}: ${problem}`));
  const activeProjectionKeys = new Set<string>();
  for (const move of moves.filter((entry) => entry.rationaleRule === "artifact-mutation-test-projection-v1")) {
    const structural = mutationStructuralPaths(move.sourcePath)[0];
    const artifactRoot = artifactRootForPath(move.sourcePath);
    if (!structural || !artifactRoot) continue;
    activeProjectionKeys.add(projectionKey(artifactRoot, structural.standard, structural.subset));
  }
  const generatedOwnerSources = new Set(ownerCatalog?.cases.filter((entry) => entry.generatorOwnerId !== null).map((entry) => entry.sourcePath) ?? []);
  for (const removal of removals) if (removal.authority.kind === "nested-cargo-generated-source") {
    const authority = removal.authority, catalogContract = taxonomy.schema.semanticPackageProjectionContracts["nested-cargo-packages-v1"];
    const row = packageCatalog?.packages.find((row) => row.id === authority.packageId), mapping = row?.mappings.find((mapping) => mapping.sourcePath === removal.sourcePath);
    const entry = inventory.entries.find((entry) => entry.sourcePath === removal.sourcePath);
    if (!mapping || mapping.disposition !== "generated" || authority.catalogPath !== catalogContract.authorityCatalogPath || authority.catalogContentHash !== catalogContract.authorityCatalogSha256 || authority.destinationPath !== mapping.destinationPath || removal.preimage.nodeKind !== "file" || removal.preimage.contentHash !== mapping.sourceHash || removal.preimage.size !== mapping.sourceSize || !entry || canonicalJson(inventoryLeafPreimage(entry)) !== canonicalJson(removal.preimage) || !row!.generatedSourceRetirements.some((item) => item.sourcePath === removal.sourcePath && item.destinationPath === authority.destinationPath && item.generatorContractId === authority.generatorContractId && item.sourceMode === removal.preimage.mode)) throw new Error("Generated source reference exclusion lacks exact retirement authority: " + removal.sourcePath);
    generatedOwnerSources.add(removal.sourcePath);
  }
  const candidates = [...new Map([...inventory.entries, ...incoming.entries, ...externalProjectionReferenceEntries(inventory, moves, artifactContexts, taxonomy), ...ownerConsumers, ...facetConsumers].map((entry) => [entry.sourcePath, entry])).values()].filter((entry) => entry.nodeKind === "file" && textualPath(entry.sourcePath) && !generatedOwnerSources.has(entry.sourcePath) && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0).sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
  for (let index = 0; index < candidates.length; index++) {
    checkCancellation(inventory.repoRoot, options.cancelFile);
    const entry = candidates[index];
    if (isExcluded(entry.sourcePath, taxonomy)) continue;
    let content: string;
    try {
      content = incoming.contents.get(entry.sourcePath) ?? readFileSync(assertLexicalInputOutsideOpaque(inventory.repoRoot, entry.sourcePath, "Reference preimage", true), "utf8");
      if (sha256(content) !== entry.contentHash || Buffer.byteLength(content) !== entry.size) throw new Error(`Reference preimage changed since inventory: ${entry.sourcePath}`);
    } catch (error) {
      unresolved.push(violation("reference-preimage-unreadable", entry.sourcePath, error instanceof Error ? error.message : String(error)));
      continue;
    }
    const finalReferencePath = moveBySource.get(entry.sourcePath)?.destinationPath ?? entry.normalizedPath;
    const contentBytes = Buffer.from(content);
    const frozenEvidence = frozenEvidenceCoordinateAuthority(entry.sourcePath, contentBytes, taxonomy);
    const fileEdits: ReferenceEdit[] = [];
    const fileTargets = new Map<string, string>();
    const ownedTokens = exactOwnedReferenceTokens(entry.sourcePath, content, ownerCatalog, moves, taxonomy);
    const facetTokens = artifactEmptyFacetReferenceTokens(entry.sourcePath, content, moves, taxonomy);
    const packageTokens = nestedCargoReferenceTokens(entry.sourcePath, content, packages);
    const policyTokens = policyStates.get(entry.sourcePath);
    if (policyTokens?.active && policyTokens.contentHash !== entry.contentHash) { unresolved.push(violation("policy-state-authority-invalid", entry.sourcePath, "Dependency policy preimage changed during planning")); continue; }
    const exactTokens = { tokens: [...ownedTokens.tokens, ...facetTokens.tokens, ...packageTokens.tokens, ...(policyTokens?.tokens ?? [])], problems: [...ownedTokens.problems, ...facetTokens.problems, ...packageTokens.problems] };
    for (const problem of exactTokens.problems) unresolved.push(violation("owner-reference-authority-invalid", entry.sourcePath, problem));
    const genericTokens = referenceTokens(entry.sourcePath, content, known).filter((token) => !exactTokens.tokens.some((exact) => exact.start <= token.start && exact.end >= token.end));
    const tokens = [...new Map([...genericTokens, ...exactTokens.tokens].map((token) => [`${token.start}\u0000${token.end}\u0000${token.value}\u0000${(token.targetValues ?? []).join("\u0000")}`, token])).values()].sort((left, right) => left.start - right.start || left.end - right.end || left.structuredLocation.localeCompare(right.structuredLocation));
    const supported = tokens.map((token) => ({ token, target: exactOwnedMarkdownTarget(entry.sourcePath, token, ownerCatalog, moves, taxonomy) ?? resolveReferenceTokenPath(entry.sourcePath, token, known) }));
    for (const { token, target: oldTarget } of supported) {
      if (token.unsupportedReason && token.physicalTargets !== undefined && (finalReferencePath !== entry.sourcePath || token.physicalTargets.some((target) => destinationBySource.has(target)))) {
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${token.structuredLocation}: ${token.unsupportedReason}`));
        continue;
      }
      if (oldTarget && isFrozenSourceCoordinateToken(entry.sourcePath, contentBytes, token, oldTarget, taxonomy, inventory.repoRoot)) continue;
      const rebase = exactOwnedMarkdownTarget(entry.sourcePath, token, ownerCatalog, moves, taxonomy);
      const destination = oldTarget ? destinationBySource.get(oldTarget) ?? (rebase || finalReferencePath !== entry.sourcePath ? oldTarget : undefined) : undefined;
      const artifactRewrite = artifactStructuralReferenceRewrite(finalReferencePath, token, artifactContexts, taxonomy);
      if (artifactRewrite?.problem) {
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${token.structuredLocation}: ${artifactRewrite.problem}`));
        continue;
      }
      const projectionState = mutationReferenceProjectionState(token, oldTarget, activeProjectionKeys, inventory.scope);
      if (projectionState === "unproven") {
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${token.structuredLocation}: Structural projection requires an exact physical target or complete declared artifact-profile scope`));
        continue;
      }
      const projectionActive = projectionState === "active";
      const artifactProjectionActive = artifactContexts.length > 0 && (token.rewriteKind === "path-prefix" || token.rewriteKind === "artifact-catalog-glob" || token.rewriteKind === "artifact-catalog-prose");
      if (token.unsupportedReason && (projectionActive || artifactProjectionActive)) {
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${token.structuredLocation}: ${token.unsupportedReason}`));
        continue;
      }
      if ((!oldTarget || !destination) && !(projectionActive && token.rewriteData?.newValue) && artifactRewrite?.newValue === undefined) continue;
      if (frozenEvidence !== null) {
        unresolved.push(violation("frozen-coordinate-evidence-unowned", entry.sourcePath, `${token.structuredLocation} is outside the exact digest-bound coordinate authority`));
        continue;
      }
      if (policyTokens?.active && !policyTokens.tokens.some((approved) => approved.start === token.start && approved.end === token.end && approved.value === token.value)) {
        unresolved.push(violation("policy-state-coordinate-unowned", entry.sourcePath, `${token.structuredLocation} is outside the exact users-only policy authority`));
        continue;
      }
      if (frozenPlanCoordinateAuthority(entry.sourcePath, contentBytes).planLike) {
        unresolved.push(violation("immutable-plan-reference-unowned", entry.sourcePath, `${token.structuredLocation} is not an exact typed coordinate in a canonical digest-verified plan`));
        continue;
      }
      const newValue = token.rewriteKind === "exact-owner-reference" ? token.rewriteData!.newValue : artifactRewrite?.newValue ?? (oldTarget && destination ? rewriteReferenceToken(finalReferencePath, entry.sourcePath, token, oldTarget, destination, inventory.repoRoot, inventory.entries) : token.rewriteData!.newValue);
      if (oldTarget) accountedIncoming.add(`${oldTarget}\u0000${entry.sourcePath}`);
      if (newValue === token.value) continue;
      const edit = {
        path: finalReferencePath,
        adapter: token.adapter,
        structuredLocation: token.structuredLocation,
        oldValue: token.value,
        newValue,
        preimage: { nodeKind: "file" as const, contentHash: entry.contentHash, mode: entry.mode, size: entry.size },
      } satisfies ReferenceEdit;
      fileEdits.push(edit);
      if (oldTarget) fileTargets.set(referenceEditIdentity(edit), oldTarget);
    }
    for (const candidate of unsupportedReferenceTokens(content, referenceAdapter(entry.sourcePath))) {
      if (supported.some(({ token }) => rustReferenceInterpretationCovers(token, candidate))) continue;
      const oldTarget = resolveReferenceTokenPath(entry.sourcePath, candidate, known);
      if (!oldTarget || !destinationBySource.has(oldTarget) && finalReferencePath === entry.sourcePath || isFrozenSourceCoordinateToken(entry.sourcePath, contentBytes, candidate, oldTarget, taxonomy, inventory.repoRoot)) continue;
      if (frozenEvidence !== null) {
        unresolved.push(violation("frozen-coordinate-evidence-unowned", entry.sourcePath, `${candidate.structuredLocation} is outside the exact digest-bound coordinate authority`));
        continue;
      }
      if (frozenPlanCoordinateAuthority(entry.sourcePath, contentBytes).planLike) {
        unresolved.push(violation("immutable-plan-reference-unowned", entry.sourcePath, `${candidate.structuredLocation} is not an exact typed coordinate in a canonical digest-verified plan`));
        continue;
      }
      const covered = supported.some(({ token, target }) => (target === oldTarget || token.rewriteKind === "exact-owner-reference") && token.start <= candidate.start && token.end >= candidate.end);
      const destination = destinationBySource.get(oldTarget) ?? oldTarget;
      const unchanged = rewriteReferenceValue(finalReferencePath, candidate.value, oldTarget, destination, entry.sourcePath, inventory.repoRoot) === candidate.value;
      if (!covered && !unchanged) unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${candidate.adapter} ${candidate.structuredLocation} contains unsupported path-bearing token ${JSON.stringify(candidate.value)} targeting ${oldTarget}`));
    }
    if (fileEdits.length > 0) {
      const deduplicated = [...new Map(fileEdits.map((edit) => [`${edit.structuredLocation}:${edit.newValue}`, edit])).values()].sort(referenceEditCompare);
      edits.push(...deduplicated);
      for (const edit of deduplicated) {
        const target = fileTargets.get(referenceEditIdentity(edit));
        if (target) editTargets.set(referenceEditIdentity(edit), target);
      }
      const rendered = applyEditsToContent(content, deduplicated);
      resultHashes.set(finalReferencePath, sha256(rendered));
      resultSizes.set(finalReferencePath, Buffer.byteLength(rendered));
    }
    report(options.progress, "plan", "references", index + 1, candidates.length, entry.sourcePath);
  }
  for (const move of moves) {
    const entry = inventory.entries.find((candidate) => candidate.sourcePath === move.sourcePath);
    if (entry?.fileKind) {
      const role = taxonomy.schema.fileKinds[entry.fileKind]?.role;
      const unaccounted = entry.referencesIn.filter((source) => !accountedIncoming.has(`${entry.sourcePath}\u0000${source}`));
      if (role === "binary" && unaccounted.length > 0) unresolved.push(violation("opaque-reference-rewrite-unresolved", entry.sourcePath, `Binary target has unsupported incoming references from ${unaccounted.join(", ")}`));
      if (role === "generated" && entry.referencesIn.length > 0) unresolved.push(violation("generated-reference-rewrite-unresolved", entry.sourcePath, "Generated target requires an explicit regeneration contract before its incoming references can move"));
    }
  }
  const semanticLocationMatches = (actual: string, expected: string): boolean => actual === expected || actual.startsWith(`${expected}:`) || actual.startsWith(`${expected}@`);
  for (const context of artifactContexts) {
    const requirements = context.authorityReferenceEdits;
    const concrete = edits.filter((edit) => requirements.some((required) => edit.path === required.path && edit.adapter === required.adapter && semanticLocationMatches(edit.structuredLocation, required.structuredLocation)));
    for (const required of requirements) {
      const matches = concrete.filter((edit) => edit.path === required.path && edit.adapter === required.adapter && semanticLocationMatches(edit.structuredLocation, required.structuredLocation) && edit.oldValue === required.oldValue && edit.newValue === required.newValue && edit.preimage.contentHash === required.preimageHash);
      if (matches.length !== 1) unresolved.push(violation("projection-reference-authority-invalid", required.path, `${context.id} requires exactly one ${required.adapter} ${required.structuredLocation} edit with its declared values and preimage; found ${matches.length}`));
    }
    if (concrete.length !== requirements.length) unresolved.push(violation("projection-reference-authority-invalid", context.sourceRoot, `${context.id} declares ${requirements.length} configuration reference edits but planning produced ${concrete.length}`));
  }
  return { edits: edits.sort(referenceEditCompare), editTargets, resultHashes, resultSizes, unresolved: stableViolations(unresolved) };
}

function referenceEditCompare(a: ReferenceEdit, b: ReferenceEdit): number {
  return generatorPathCompare(a.path, b.path) || generatorPathCompare(a.structuredLocation, b.structuredLocation) || generatorPathCompare(a.oldValue, b.oldValue) || generatorPathCompare(a.newValue, b.newValue);
}
//#endregion 🔗️References

//#region 📋️Inventory API
/** 🪟️ Evaluates checkout-hostile path strings without materializing filesystem nodes. */
export function taxonomyPlatformPathViolationCodes(path: string, maxPathBytes = 240): readonly string[] {
  const rows: string[] = [];
  if (Buffer.byteLength(path, "utf8") > maxPathBytes) rows.push("path-too-long");
  for (const segment of path.replaceAll("\\", "/").split("/")) {
    if (WINDOWS_RESERVED.test(segment)) rows.push("windows-reserved-name");
    if (/[. ]$/u.test(segment)) rows.push("trailing-dot-or-space");
  }
  return [...new Set(rows)];
}

function pathPolicyViolations(path: string, taxonomy: LoadedTaxonomy): readonly TaxonomyViolation[] {
  const rows: TaxonomyViolation[] = [];
  if (Buffer.byteLength(path, "utf8") > taxonomy.schema.collisionPolicy.maxPathBytes) rows.push(violation("path-too-long", path, `Path exceeds ${taxonomy.schema.collisionPolicy.maxPathBytes} UTF-8 bytes`));
  for (const segment of path.split("/")) {
    if (taxonomy.schema.collisionPolicy.rejectWindowsReservedNames && WINDOWS_RESERVED.test(segment)) rows.push(violation("windows-reserved-name", path, `Path segment is Windows-reserved: ${segment}`));
    if (taxonomy.schema.collisionPolicy.rejectTrailingDotsAndSpaces && /[. ]$/.test(segment)) rows.push(violation("trailing-dot-or-space", path, `Path segment ends with a dot or space: ${segment}`));
  }
  return rows;
}

function sourceTreeDigest(entries: readonly TaxonomyInventoryEntry[]): string {
  return sha256(canonicalJson(entries.map((entry) => ({ sourcePath: entry.sourcePath, nodeKind: entry.nodeKind, contentHash: entry.contentHash, mode: entry.mode, size: entry.size, symlinkTarget: entry.symlinkTarget }))));
}

function inventoryWithoutTransactionEvidence(inventory: TaxonomyInventory, transactionRoot: string, exactPlanArtifactPath?: string): TaxonomyInventory {
  const suppressed = (path: string): boolean => path === exactPlanArtifactPath || path === transactionRoot || path.startsWith(`${transactionRoot}/`);
  const entries = inventory.entries
    .filter((entry) => !suppressed(entry.sourcePath))
    .map((entry) => ({ ...entry, referencesIn: entry.referencesIn.filter((path) => !suppressed(path)), referencesOut: entry.referencesOut.filter((path) => !suppressed(path)) }));
  return inheritReferenceInventoryContext(inventory, { ...inventory, entries, violations: inventory.violations.filter((entry) => !suppressed(entry.path)), sourceTreeDigest: sourceTreeDigest(entries) }, transactionRoot, exactPlanArtifactPath);
}

function ancestorDirectoryKindIds(path: string, kinds: ReadonlyMap<string, string>): readonly string[] {
  const rows: string[] = [];
  let current = dirname(path);
  while (current && current !== ".") {
    const kindId = kinds.get(current);
    if (kindId) rows.push(kindId);
    current = dirname(current);
  }
  return rows;
}

//#region 🧭️Artifact Mutation Projection
function mutationDomainOwnerLocation(path: string, taxonomy: LoadedTaxonomy): { root: string; relativePath: string; identity: string } | null {
  for (const root of Object.keys(taxonomy.discoverySchema.mutationDomainOwners)) {
    if (!path.startsWith(`${root}/`)) continue;
    const relativePath = path.slice(root.length + 1).split("/").slice(0, 2).join("/");
    const identity = mutationOwnerIdentity(root, relativePath, taxonomy.discoverySchema);
    if (identity !== null) return { root, relativePath, identity };
  }
  return null;
}

interface MutationProjectionSource {
  readonly artifactRoot: string;
  readonly artifactId: string;
  readonly standardVersion: string;
  readonly standardDirectoryName: string;
  readonly subsetId: string;
  readonly subsetDirectoryName: string;
  readonly mutationId: string;
  readonly mutationDirectoryName: string;
  readonly sourceScenarioId: string;
  readonly sourceScenarioDirectoryName: string;
  readonly subsetRoot: string;
  readonly mutationRoot: string;
  readonly scenarioRoot: string;
}

interface MutationProjectionVector {
  readonly mutationId: string;
  readonly sourceMutationDirectoryName: string;
  readonly mutationDirectoryName: string;
  readonly scenarioId: string;
  readonly scenarioDirectoryName: string;
  readonly catalogOwner?: string;
  readonly catalogPath?: string;
}

function projectionDirectorySlug(name: string, kindId: string, taxonomy: LoadedTaxonomy): string | null {
  const kind = taxonomy.schema.semanticDirectoryKinds[kindId];
  if (!kind) return null;
  const leading = splitLeadingEmoji(name.normalize("NFC"));
  if (emojiFold(leading.emoji) !== emojiFold(kind.emoji) || !new RegExp(kind.slugPattern, "u").test(leading.rest)) return null;
  return leading.rest;
}

function projectionSourceAt(
  path: string,
  scope: string | undefined,
  entries: ReadonlyMap<string, MutableInventoryEntry>,
  kinds: ReadonlyMap<string, string>,
  taxonomy: LoadedTaxonomy,
): MutationProjectionSource | null {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const contract = taxonomy.schema.semanticPathProjectionContracts[ids.projectionContractId];
  const domainOwner = mutationDomainOwnerLocation(path, taxonomy);
  if (!domainOwner && Object.keys(taxonomy.discoverySchema.mutationDomainOwners).some((root) => path.startsWith(`${root}/`))) return null;
  const physicalSegments = path.split("/"), domainIndex = domainOwner ? domainOwner.root.split("/").length : null;
  const segments = physicalSegments.filter((_segment, index) => index !== domainIndex);
  const physicalPrefix = (length: number): string => physicalSegments.slice(0, length + (domainIndex !== null && length > domainIndex ? 1 : 0)).join("/");
  if (segments.length <= contract.sourceSegments.length) return null;
  const start = segments.length - contract.sourceSegments.length;
  const artifactRoot = physicalPrefix(start);
  const ownerRegistry = taxonomy.schema.semanticDirectoryMemberKinds[contract.sourceOwnerKindId];
  const ownerMatches = ownerRegistry.memberNames.filter((name) => emojiFold(name) === emojiFold(basename(artifactRoot)));
  if (ownerMatches.length !== 1 && !(scope && (artifactRoot === scope || artifactRoot.startsWith(`${scope}/`)))) return null;
  const captures = new Map<SemanticProjectionCaptureField, string>();
  for (let index = 0; index < contract.sourceSegments.length; index++) {
    const segment = contract.sourceSegments[index];
    const currentPath = physicalPrefix(start + index + 1);
    const current = entries.get(currentPath);
    if (!current || current.nodeKind !== "directory") return null;
    const canonicalName = basename(current.normalizedPath);
    if ("literal" in segment) {
      if (canonicalName !== segment.literal || kinds.get(currentPath) !== segment.kindId) return null;
      continue;
    }
    if ("projectedMemberKindId" in segment) {
      if (segment.projectedMemberKindId !== ids.projectedMemberKindId) return null;
      const sourceName = basename(current.sourcePath).normalize("NFC");
      const slug = splitLeadingEmoji(sourceName).rest;
      if (!slug) return null;
      captures.set(segment.capture, domainOwner?.identity ?? slug);
      continue;
    }
    const sourceName = basename(current.sourcePath).normalize("NFC");
    if (segment.capture === "scenarioId" && pathEmojiStatuteFindings([{ path: sourceName, nodeKind: "directory" }], taxonomy.discoverySchema.pathEmojiPolicy.genericEmojiIdentities).length === 0 && new RegExp(taxonomy.schema.semanticDirectoryKinds[segment.kindId].slugPattern, "u").test(splitLeadingEmoji(sourceName).rest)) {
      captures.set(segment.capture, splitLeadingEmoji(sourceName).rest);
      continue;
    }
    const contextualUnprefixed = segment.capture === "scenarioId" && !splitLeadingEmoji(sourceName).emoji && new RegExp(taxonomy.schema.semanticDirectoryKinds[segment.kindId].slugPattern, "u").test(sourceName);
    if (kinds.get(currentPath) !== segment.kindId && !contextualUnprefixed) return null;
    const slug = contextualUnprefixed ? sourceName : projectionDirectorySlug(canonicalName, segment.kindId, taxonomy);
    if (!slug) return null;
    captures.set(segment.capture, slug);
  }
  const standardVersion = captures.get("standardVersion");
  const subsetId = captures.get("subsetId");
  const mutationId = captures.get("mutationId");
  const sourceScenarioId = captures.get("scenarioId");
  if (!standardVersion || !subsetId || !mutationId || !sourceScenarioId) return null;
  const source = contract.sourceSegments.map((_segment, index) => physicalPrefix(start + index + 1));
  return {
    artifactRoot,
    artifactId: splitLeadingEmoji(basename(artifactRoot)).rest || basename(artifactRoot),
    standardVersion,
    standardDirectoryName: basename(source[1]),
    subsetId,
    subsetDirectoryName: basename(source[3]),
    mutationId,
    mutationDirectoryName: basename(source[6]),
    sourceScenarioId,
    sourceScenarioDirectoryName: basename(source[8]),
    subsetRoot: source[3],
    mutationRoot: source[6],
    scenarioRoot: source[8],
  };
}

function projectionCatalogVectors(path: string, source: Pick<MutationProjectionSource, "standardDirectoryName" | "subsetDirectoryName">): { readonly vectors: readonly MutationProjectionVector[]; readonly error?: string } {
  let root: Record<string, unknown>;
  try {
    root = record(JSON.parse(readFileSync(path, "utf8")), "mutation projection catalog");
  } catch (error) {
    return { vectors: [], error: error instanceof Error ? error.message : String(error) };
  }
  if (!Array.isArray(root.mutationCatalogs)) return { vectors: [], error: "mutationCatalogs must be an array" };
  const vectors: MutationProjectionVector[] = [];
  const seenSource = new Set<string>();
  const seenCanonical = new Set<string>();
  try {
    for (let catalogIndex = 0; catalogIndex < root.mutationCatalogs.length; catalogIndex++) {
      const catalog = record(root.mutationCatalogs[catalogIndex], `mutationCatalogs[${catalogIndex}]`);
      requiredString(catalog.id, `mutationCatalogs[${catalogIndex}].id`);
      requiredString(catalog.capability, `mutationCatalogs[${catalogIndex}].capability`);
      if (requiredString(catalog.standardDirectoryName, `mutationCatalogs[${catalogIndex}].standardDirectoryName`) !== source.standardDirectoryName || requiredString(catalog.subsetDirectoryName, `mutationCatalogs[${catalogIndex}].subsetDirectoryName`) !== source.subsetDirectoryName) throw new Error(`mutationCatalogs[${catalogIndex}] owner identity does not match its physical standard/subset`);
      stringArray(catalog.kinds, `mutationCatalogs[${catalogIndex}].kinds`);
      if (!Array.isArray(catalog.vectors)) throw new Error(`mutationCatalogs[${catalogIndex}].vectors must be an array`);
      for (let vectorIndex = 0; vectorIndex < catalog.vectors.length; vectorIndex++) {
        const vector = record(catalog.vectors[vectorIndex], `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}]`);
        const mutationId = requiredString(vector.mutationId, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].mutationId`);
        const sourceMutationDirectoryName = requiredString(vector.sourceMutationDirectoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].sourceMutationDirectoryName`);
        if (sourceMutationDirectoryName !== sourceMutationDirectoryName.normalize("NFC") || sourceMutationDirectoryName.includes("/")) throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].sourceMutationDirectoryName is not one exact NFC basename`);
        const mutationDirectoryName = requiredString(vector.mutationDirectoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].mutationDirectoryName`).normalize("NFC");
        if (!Array.isArray(vector.scenarios)) throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}] has an invalid physical mutation identity`);
        for (let scenarioIndex = 0; scenarioIndex < vector.scenarios.length; scenarioIndex++) {
          const scenario = record(vector.scenarios[scenarioIndex], `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}]`);
          const scenarioId = requiredString(scenario.id, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}].id`);
          const scenarioDirectoryName = requiredString(scenario.directoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}].directoryName`).normalize("NFC");
          if (splitLeadingEmoji(scenarioDirectoryName).rest !== scenarioId) throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}] has an invalid physical scenario identity`);
          const sourceKey = `${mutationId}\u0000${sourceMutationDirectoryName}\u0000${scenarioId}`;
          const canonicalKey = `${mutationId}\u0000${mutationDirectoryName}\u0000${scenarioId}`;
          if (seenSource.has(sourceKey) || seenCanonical.has(canonicalKey)) throw new Error(`Duplicate physical vector identity ${sourceKey.replaceAll("\u0000", "/")}`);
          seenSource.add(sourceKey);
          seenCanonical.add(canonicalKey);
          vectors.push({ mutationId, sourceMutationDirectoryName, mutationDirectoryName, scenarioId, scenarioDirectoryName });
        }
      }
    }
  } catch (error) {
    return { vectors: [], error: error instanceof Error ? error.message : String(error) };
  }
  return { vectors: vectors.sort((left, right) => left.sourceMutationDirectoryName.localeCompare(right.sourceMutationDirectoryName) || left.scenarioDirectoryName.localeCompare(right.scenarioDirectoryName)) };
}

function projectionCatalogEntryForSubset(entries: ReadonlyMap<string, MutableInventoryEntry>, subsetRoot: string, taxonomy: LoadedTaxonomy): MutableInventoryEntry | null {
  const oracleRoot = `${subsetRoot}/${taxonomy.discoverySchema.testContributionDirectoryOverrides[subsetRoot] ?? "🔮️oracle"}`;
  const candidates = [...entries.values()].filter((entry) => entry.nodeKind === "file" && entry.fileKind === "json" && dirname(entry.sourcePath) === oracleRoot && basename(entry.normalizedPath) === "🔣️.json");
  return candidates.length === 1 ? candidates[0] : null;
}

function projectionCatalogsForMutationSource(repoRoot: string, entries: ReadonlyMap<string, MutableInventoryEntry>, sourceOwner: string, taxonomy: LoadedTaxonomy): { owner: string; path: string; entry: MutableInventoryEntry | null; vectors: readonly MutationProjectionVector[]; error?: string }[] {
  const problems = mutationCatalogSourceOwnersProblems(taxonomy.discoverySchema);
  if (problems.length > 0) throw new Error(problems.join("\n"));
  const owners = [sourceOwner, ...Object.entries(taxonomy.discoverySchema.mutationCatalogSourceOwners).filter(([, source]) => source === sourceOwner).map(([owner]) => owner)];
  return owners.map((owner) => {
    const entry = projectionCatalogEntryForSubset(entries, owner, taxonomy);
    const path = entry?.sourcePath ?? `${owner}/${taxonomy.discoverySchema.testContributionDirectoryOverrides[owner] ?? "🔮️oracle"}/🔣️.json`;
    const profile = { standardDirectoryName: owner.split("/").at(-3)!, subsetDirectoryName: basename(owner) };
    const catalog = entry ? projectionCatalogVectors(absolutePath(repoRoot, path), profile) : { vectors: [], error: `catalog is missing at ${path}` };
    return { owner, path, entry, ...catalog, vectors: catalog.vectors.map((vector) => ({ ...vector, catalogOwner: owner, catalogPath: path })) };
  });
}

function mutationDescendantContract(taxonomy: LoadedTaxonomy): SemanticKindDescendantContract {
  const contract = taxonomy.schema.semanticDescendantContracts[taxonomy.schema.mutationCatalogProjection.descendantContractId];
  if (!contract || "contractKind" in contract || [...contract.requiredNodes, ...contract.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].some((node) => !("kindId" in node))) throw new Error("Mutation projection must reference one physical-kind exact descendant contract");
  return contract as SemanticKindDescendantContract;
}

function projectionDescendantPath(node: SemanticDescendantKindNode, taxonomy: LoadedTaxonomy): string {
  const segments = node.pathSegments.map((segment) => segment.literal);
  if (node.nodeType === "file") {
    const kind = taxonomy.schema.fileKinds[node.kindId];
    if (!kind || kind.extensionChains.length !== 1) throw new Error(`Projection descendant kind ${node.kindId} is not a single physical leaf`);
    segments.push(`${kind.emoji}${kind.extensionChains[0]}`.normalize("NFC"));
  }
  return segments.join("/");
}

function canonicalProjectedMemberName(name: string, taxonomy: LoadedTaxonomy): string | null {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const sourceKind = taxonomy.schema.semanticProjectedMemberKinds[ids.projectedMemberKindId].sourceMemberKindId;
  const matches = taxonomy.schema.semanticDirectoryMemberKinds[sourceKind].memberNames.filter((candidate) => emojiFold(candidate) === emojiFold(name.normalize("NFC")));
  return matches.length === 1 ? matches[0] : null;
}

function canonicalProjectedMutationOwner(name: string, identity: string, subsetRoot: string, taxonomy: LoadedTaxonomy): string | null {
  const sourceOwner = mutationCatalogSourceOwner(subsetRoot, taxonomy.discoverySchema);
  if (sourceOwner === null) return null;
  const root = `${sourceOwner}/🧬️schema/🧬️mutations`;
  if (!Object.hasOwn(taxonomy.discoverySchema.mutationDomainOwners, root)) return canonicalProjectedMemberName(name, taxonomy);
  const owner = mutationOwnerRelativePath(root, identity, taxonomy.discoverySchema);
  return owner && basename(owner) === name ? owner : null;
}

function projectionBundleProblem(source: MutationProjectionSource, entries: ReadonlyMap<string, MutableInventoryEntry>, kinds: ReadonlyMap<string, string>, contract: SemanticDescendantContract, taxonomy: LoadedTaxonomy): string | null {
  const root = entries.get(source.scenarioRoot);
  if (!root) return "scenario root is absent";
  const actual = [...entries.values()].filter((entry) => entry.sourcePath === source.scenarioRoot || entry.sourcePath.startsWith(`${source.scenarioRoot}/`));
  if (actual.length !== contract.realizedNodeCount) return `bundle has ${actual.length} nodes, expected ${contract.realizedNodeCount}`;
  if (actual.some((entry) => entry.nodeKind === "symlink")) return "bundle contains a symlink";
  const byKey = new Map<string, MutableInventoryEntry>();
  for (const entry of actual) {
    const relativePath = entry.normalizedPath === root.normalizedPath ? "" : entry.normalizedPath.startsWith(`${root.normalizedPath}/`) ? entry.normalizedPath.slice(root.normalizedPath.length + 1) : null;
    if (relativePath === null) return `bundle node normalizes outside its scenario: ${entry.sourcePath}`;
    const key = `${entry.nodeKind}\u0000${relativePath}`;
    if (byKey.has(key)) return `bundle normalization duplicates ${relativePath}`;
    byKey.set(key, entry);
  }
  const matches = (node: SemanticDescendantNode): boolean => {
    const entry = byKey.get(`${node.nodeType}\u0000${projectionDescendantPath(node, taxonomy)}`);
    if (!entry) return false;
    return node.nodeType === "file" ? entry.fileKind === node.kindId : node.pathSegments.length === 0 && entry.sourcePath === source.scenarioRoot && node.kindId === contract.rootDirectoryKindId || kinds.get(entry.sourcePath) === node.kindId;
  };
  const missing = contract.requiredNodes.filter((node) => !matches(node));
  if (missing.length > 0) return `bundle is missing ${projectionDescendantPath(missing[0], taxonomy) || "scenario root"}`;
  for (const alternative of contract.exclusiveAlternatives) if (alternative.nodes.filter(matches).length !== 1) return `bundle must realize exactly one ${alternative.id} alternative`;
  const allowed = new Set([...contract.requiredNodes, ...contract.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].filter(matches).map((node) => `${node.nodeType}\u0000${projectionDescendantPath(node, taxonomy)}`));
  const extra = [...byKey.keys()].find((key) => !allowed.has(key));
  return extra ? `bundle contains unregistered node ${extra.slice(extra.indexOf("\u0000") + 1)}` : null;
}

function setProjectedPath(entry: MutableInventoryEntry, destination: string, taxonomy: LoadedTaxonomy, authority: "exact-descendant" | "parent-rebase" = "exact-descendant"): void {
  entry.normalizedPath = destination.normalize("NFC");
  const superseded = new Set(["path-too-long", "windows-reserved-name", "trailing-dot-or-space", ...(authority === "exact-descendant" ? ["directory-kind-ambiguous", "directory-kind-unresolved", "file-kind-ambiguous", "file-kind-unresolved", "semantic-stem-ambiguous", "semantic-stem-unresolved"] : [])]);
  entry.violations = [...entry.violations.filter((row) => !superseded.has(row.code)), ...pathPolicyViolations(entry.normalizedPath, taxonomy)];
}

function mutationProjectionRationale(sourcePath: string, destinationPath: string, taxonomy: LoadedTaxonomy): "artifact-mutation-test-projection-v1" | "artifact-mutation-source-canonicalization-v1" | null {
  const structural = mutationStructuralPaths(sourcePath)[0];
  const artifactRoot = artifactRootForPath(sourcePath);
  if (!artifactRoot) return null;
  const relativeDestination = destinationPath.startsWith(`${artifactRoot}/`) ? destinationPath.slice(artifactRoot.length + 1).split("/") : [];
  if (structural && relativeDestination[0] === "🧪️tests") {
    const sourceOwner = `${artifactRoot}/🏅️standards/🔖️${structural.standard}/🪆️subsets/✳️${structural.subset}`;
    const catalogOwners = [sourceOwner, ...Object.entries(taxonomy.discoverySchema.mutationCatalogSourceOwners).filter(([, source]) => source === sourceOwner).map(([owner]) => owner)];
    const admittedProfile = catalogOwners.some((owner) => mutationCatalogSourceOwner(owner, taxonomy.discoverySchema) === sourceOwner && relativeDestination[1] === `🪆️${structural.standard}-${splitLeadingEmoji(basename(owner)).rest}`);
    const root = `${sourceOwner}/🧬️schema/🧬️mutations`;
    const registered = Object.hasOwn(taxonomy.discoverySchema.mutationDomainOwners, root), depth = registered ? 2 : 1;
    const owner = relativeDestination.slice(2, 2 + depth).join("/"), scenario = relativeDestination[2 + depth] ?? "";
    const identity = mutationOwnerIdentity(root, structural.mutation, taxonomy.discoverySchema);
    const validOwner = registered ? identity !== null && mutationOwnerRelativePath(root, identity, taxonomy.discoverySchema) === owner : !structural.mutation.includes("/") && canonicalProjectedMemberName(owner, taxonomy) === owner;
    if (admittedProfile && validOwner && /^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(splitLeadingEmoji(scenario).rest) && pathEmojiStatuteFindings([{ path: scenario, nodeKind: "directory" }], taxonomy.discoverySchema.pathEmojiPolicy.genericEmojiIdentities).length === 0) return "artifact-mutation-test-projection-v1";
  }
  const relativeSource = sourcePath.slice(artifactRoot.length + 1).split("/");
  const prefix = ["🏅️standards", relativeSource[1], "🪆️subsets", relativeSource[3], "🧬️schema", "🧬️mutations"];
  if (relativeSource.length > 7 && prefix.every((segment, index) => relativeSource[index] === segment) && prefix.every((segment, index) => relativeDestination[index] === segment) && relativeSource[6] !== relativeDestination[6] && canonicalProjectedMemberName(relativeDestination[6] ?? "", taxonomy) === relativeDestination[6]) return "artifact-mutation-source-canonicalization-v1";
  return null;
}

function projectMutationTestBundles(
  repoRoot: string,
  scope: string | undefined,
  entries: Map<string, MutableInventoryEntry>,
  kinds: ReadonlyMap<string, string>,
  taxonomy: LoadedTaxonomy,
): void {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const projection = taxonomy.schema.semanticPathProjectionContracts[ids.projectionContractId];
  const descendant = mutationDescendantContract(taxonomy);
  const renderer = taxonomy.schema.semanticPathProjectionProfileRenderers[projection.profileRendererId];
  const sources = [...entries.values()].filter((entry) => entry.nodeKind === "directory").map((entry) => projectionSourceAt(entry.sourcePath, scope, entries, kinds, taxonomy)).filter((entry): entry is MutationProjectionSource => entry !== null).sort((left, right) => left.scenarioRoot.localeCompare(right.scenarioRoot));
  const bySubset = new Map<string, MutationProjectionSource[]>();
  for (const source of sources) bySubset.set(source.subsetRoot, [...(bySubset.get(source.subsetRoot) ?? []), source]);
  const profileOwners = new Map<string, Set<string>>();
  for (const source of sources) {
    const profile = renderer.template.replace("{standardVersion}", source.standardVersion).replace("{subsetId}", source.subsetId);
    const key = `${source.artifactRoot}\u0000${emojiFold(profile).toLocaleLowerCase("und")}`;
    const owners = profileOwners.get(key) ?? new Set<string>();
    owners.add(`${source.artifactId}\u0000${source.standardVersion}\u0000${source.subsetId}`);
    profileOwners.set(key, owners);
  }
  for (const [subsetRoot, subsetSources] of [...bySubset.entries()].sort(([left], [right]) => left.localeCompare(right))) {
    const catalogs = projectionCatalogsForMutationSource(repoRoot, entries, subsetRoot, taxonomy);
    const invalid = catalogs.filter((catalog) => catalog.error);
    if (invalid.length > 0) {
      for (const source of subsetSources) for (const catalog of invalid) entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-invalid", catalog.path, catalog.error!));
      continue;
    }
    const catalog = { vectors: catalogs.flatMap((entry) => entry.vectors) };
    for (const owner of catalogs) {
      const subsetId = splitLeadingEmoji(basename(owner.owner)).rest;
      const profile = renderer.template.replace("{standardVersion}", subsetSources[0]!.standardVersion).replace("{subsetId}", subsetId);
      const key = `${subsetSources[0]!.artifactRoot}\u0000${emojiFold(profile).toLocaleLowerCase("und")}`;
      const owners = profileOwners.get(key) ?? new Set<string>();
      owners.add(`${subsetSources[0]!.artifactId}\u0000${subsetSources[0]!.standardVersion}\u0000${subsetId}`);
      profileOwners.set(key, owners);
    }
    const vectorsByMutation = new Map<string, MutationProjectionVector[]>();
    for (const vector of catalog.vectors) {
      const key = vector.mutationId;
      vectorsByMutation.set(key, [...(vectorsByMutation.get(key) ?? []), vector]);
    }
    const sourcesByMutation = new Map<string, MutationProjectionSource[]>();
    for (const source of subsetSources) {
      const key = source.mutationId;
      sourcesByMutation.set(key, [...(sourcesByMutation.get(key) ?? []), source]);
    }
    const consumed = new Set<string>();
    const canonicalizedMutationRoots = new Set<string>();
    for (const [mutationKey, mutationSources] of sourcesByMutation) {
      const vectors = vectorsByMutation.get(mutationKey) ?? [];
      const canonicalNames = [...new Set(vectors.map((vector) => canonicalProjectedMutationOwner(vector.mutationDirectoryName, vector.mutationId, subsetRoot, taxonomy)).filter((name): name is string => name !== null))];
      const mutationName = canonicalNames.length === 1 ? canonicalNames[0] : null;
      if (!mutationName || vectors.some((vector) => vector.mutationDirectoryName !== basename(mutationName) || mutationSources.some((source) => source.mutationDirectoryName !== vector.sourceMutationDirectoryName))) {
        for (const source of mutationSources) entries.get(source.scenarioRoot)?.violations.push(violation("projection-member-unresolved", source.mutationRoot, `Mutation member ${source.mutationDirectoryName} has no unique canonical registry identity`));
        continue;
      }
      const exact = new Map(vectors.map((vector) => [vector.scenarioDirectoryName, vector]));
      const assignments = new Map<MutationProjectionSource, MutationProjectionVector>();
      for (const source of mutationSources) {
        const vector = exact.get(source.sourceScenarioDirectoryName);
        if (vector) assignments.set(source, vector);
      }
      const unmatchedSources = mutationSources.filter((source) => !assignments.has(source));
      const matchedVectors = new Set(assignments.values());
      const unmatchedVectors = vectors.filter((vector) => !matchedVectors.has(vector));
      if (unmatchedSources.length === 1 && unmatchedVectors.length === 1) assignments.set(unmatchedSources[0], unmatchedVectors[0]);
      if (assignments.size !== mutationSources.length || assignments.size !== vectors.length) {
        for (const source of mutationSources) entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-coverage", source.scenarioRoot, `Physical mutation ${mutationName} does not have an exact one-to-one vector registry`));
        continue;
      }
      for (const [source, vector] of assignments) {
        const vectorKey = `${vector.mutationId}\u0000${vector.sourceMutationDirectoryName}\u0000${vector.scenarioId}`;
        if (consumed.has(vectorKey)) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-duplicate", source.scenarioRoot, `Vector ${vectorKey.replaceAll("\u0000", "/")} owns more than one physical bundle`));
          continue;
        }
        const problem = projectionBundleProblem(source, entries, kinds, descendant, taxonomy);
        if (problem) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-bundle-invalid", source.scenarioRoot, problem));
          continue;
        }
        const subsetId = vector.catalogOwner ? splitLeadingEmoji(basename(vector.catalogOwner)).rest : source.subsetId;
        const profile = renderer.template.replace("{standardVersion}", source.standardVersion).replace("{subsetId}", subsetId).normalize("NFC");
        const profileKey = `${source.artifactRoot}\u0000${emojiFold(profile).toLocaleLowerCase("und")}`;
        if ((profileOwners.get(profileKey)?.size ?? 0) !== 1) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-profile-collision", source.scenarioRoot, `Profile ${profile} is not a unique standard/subset rendering`));
          continue;
        }
        const destinationSegments = projection.destinationSegments.map((segment) => {
          if ("literal" in segment) return segment.literal;
          if ("render" in segment) return profile;
          if ("projectedMemberKindId" in segment) return mutationName;
          return vector.scenarioDirectoryName;
        });
        const destinationRoot = `${source.artifactRoot}/${destinationSegments.join("/")}`.normalize("NFC");
        if (Buffer.byteLength(destinationRoot, "utf8") + descendant.pathBudgetReserve.bytes > taxonomy.schema.collisionPolicy.maxPathBytes) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-path-budget", source.scenarioRoot, `Projected scenario plus reserved descendant suffix exceeds ${taxonomy.schema.collisionPolicy.maxPathBytes} bytes`));
          continue;
        }
        consumed.add(vectorKey);
        const root = entries.get(source.scenarioRoot)!;
        root.violations = root.violations.filter((row) => row.code !== "directory-kind-unresolved");
        const initialRoot = root.normalizedPath;
        for (const entry of entries.values()) {
          if (entry.sourcePath !== source.scenarioRoot && !entry.sourcePath.startsWith(`${source.scenarioRoot}/`)) continue;
          const suffix = entry.normalizedPath === initialRoot ? "" : entry.normalizedPath.slice(initialRoot.length + 1);
          setProjectedPath(entry, suffix ? `${destinationRoot}/${suffix}` : destinationRoot, taxonomy);
        }
        if (!canonicalizedMutationRoots.has(source.mutationRoot)) {
          const mutation = entries.get(source.mutationRoot);
          const testsRoot = dirname(source.scenarioRoot);
          if (mutation) {
            const initialMutationRoot = mutation.normalizedPath;
            const canonicalMutationRoot = `${subsetRoot}/🧬️schema/🧬️mutations/${mutationName}`.normalize("NFC");
            const mutationEntries = [...entries.values()].filter((entry) => entry.sourcePath === source.mutationRoot || entry.sourcePath.startsWith(`${source.mutationRoot}/`)).filter((entry) => entry.sourcePath !== testsRoot && !entry.sourcePath.startsWith(`${testsRoot}/`));
            mutation.violations = mutation.violations.filter((row) => row.code !== "directory-kind-unresolved");
            for (const entry of mutationEntries) {
              const suffix = entry.normalizedPath === initialMutationRoot ? "" : entry.normalizedPath.startsWith(`${initialMutationRoot}/`) ? entry.normalizedPath.slice(initialMutationRoot.length + 1) : entry.sourcePath.slice(source.mutationRoot.length + 1);
              setProjectedPath(entry, suffix ? `${canonicalMutationRoot}/${suffix}` : canonicalMutationRoot, taxonomy, "parent-rebase");
            }
          }
          canonicalizedMutationRoots.add(source.mutationRoot);
        }
      }
    }
    for (const vector of catalog.vectors) {
      const key = `${vector.mutationId}\u0000${vector.sourceMutationDirectoryName}\u0000${vector.scenarioId}`;
      if (!consumed.has(key) && vector.catalogPath) entries.get(vector.catalogPath)?.violations.push(violation("projection-catalog-unrealized", vector.catalogPath, `Registered vector ${key.replaceAll("\u0000", "/")} has no physical bundle`));
    }
  }
}

function validateProjectedMutationTestBundles(
  repoRoot: string,
  scope: string | undefined,
  entries: Map<string, MutableInventoryEntry>,
  kinds: ReadonlyMap<string, string>,
  taxonomy: LoadedTaxonomy,
): void {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const projection = taxonomy.schema.semanticPathProjectionContracts[ids.projectionContractId];
  const descendant = mutationDescendantContract(taxonomy);
  const renderer = taxonomy.schema.semanticPathProjectionProfileRenderers[projection.profileRendererId];
  const expected = new Set<string>();
  const profileDepths = new Map<string, number>();
  const catalogs = [...entries.values()].filter((entry) => entry.nodeKind === "file" && entry.fileKind === "json" && basename(dirname(entry.sourcePath)) === (taxonomy.discoverySchema.testContributionDirectoryOverrides[dirname(dirname(entry.sourcePath))] ?? "🔮️oracle") && basename(entry.normalizedPath) === "🔣️.json").sort((left, right) => left.sourcePath.localeCompare(right.sourcePath));
  for (const catalogEntry of catalogs) {
    const subsetRoot = dirname(dirname(catalogEntry.sourcePath));
    const segments = subsetRoot.split("/");
    if (segments.length < 5) continue;
    const artifactRoot = segments.slice(0, -4).join("/");
    const [standardsName, standardDirectoryName, subsetsName, subsetDirectoryName] = segments.slice(-4);
    if (standardsName !== "🏅️standards" || subsetsName !== "🪆️subsets") continue;
    const ownerRegistry = taxonomy.schema.semanticDirectoryMemberKinds[projection.sourceOwnerKindId];
    const ownerMatches = ownerRegistry.memberNames.filter((name) => emojiFold(name) === emojiFold(basename(artifactRoot)));
    if (ownerMatches.length !== 1 && !(scope && (artifactRoot === scope || artifactRoot.startsWith(`${scope}/`)))) continue;
    const standardVersion = projectionDirectorySlug(standardDirectoryName, "standard", taxonomy);
    const subsetId = projectionDirectorySlug(subsetDirectoryName, "subset", taxonomy);
    if (!standardVersion || !subsetId) continue;
    const catalog = projectionCatalogVectors(absolutePath(repoRoot, catalogEntry.sourcePath), { standardDirectoryName, subsetDirectoryName });
    if (catalog.error) {
      catalogEntry.violations.push(violation("projection-catalog-invalid", catalogEntry.sourcePath, catalog.error));
      continue;
    }
    const profile = renderer.template.replace("{standardVersion}", standardVersion).replace("{subsetId}", subsetId).normalize("NFC");
    const sourceOwner = mutationCatalogSourceOwner(subsetRoot, taxonomy.discoverySchema);
    if (sourceOwner === null) {
      catalogEntry.violations.push(violation("projection-catalog-invalid", catalogEntry.sourcePath, "Catalog source ownership is invalid"));
      continue;
    }
    profileDepths.set(`${artifactRoot}/🧪️tests/${profile}`, Object.hasOwn(taxonomy.discoverySchema.mutationDomainOwners, `${sourceOwner}/🧬️schema/🧬️mutations`) ? 3 : 2);
    for (const vector of catalog.vectors) {
      const ownerRelativePath = canonicalProjectedMutationOwner(vector.mutationDirectoryName, vector.mutationId, subsetRoot, taxonomy);
      if (!ownerRelativePath) {
        catalogEntry.violations.push(violation("projection-member-unresolved", catalogEntry.sourcePath, `Mutation member ${vector.mutationDirectoryName} has no unique canonical registry identity`));
        continue;
      }
      const mutationDirectoryName = basename(ownerRelativePath);
      const mutationRoot = `${artifactRoot}/🧪️tests/${profile}/${ownerRelativePath}`;
      const scenarioRoot = `${mutationRoot}/${vector.scenarioDirectoryName}`;
      expected.add(scenarioRoot);
      const root = entries.get(scenarioRoot);
      if (!root) continue;
      const source: MutationProjectionSource = { artifactRoot, artifactId: splitLeadingEmoji(basename(artifactRoot)).rest || basename(artifactRoot), standardVersion, standardDirectoryName, subsetId, subsetDirectoryName, mutationId: vector.mutationId, mutationDirectoryName, sourceScenarioId: vector.scenarioId, sourceScenarioDirectoryName: vector.scenarioDirectoryName, subsetRoot, mutationRoot, scenarioRoot };
      const problem = projectionBundleProblem(source, entries, kinds, descendant, taxonomy);
      if (problem) {
        root.violations.push(violation("projection-bundle-invalid", scenarioRoot, problem));
        continue;
      }
      root.violations = root.violations.filter((row) => row.code !== "directory-kind-unresolved");
      const initialRoot = root.normalizedPath;
      for (const entry of entries.values()) {
        if (entry.sourcePath !== scenarioRoot && !entry.sourcePath.startsWith(`${scenarioRoot}/`)) continue;
        const suffix = entry.normalizedPath === initialRoot ? "" : entry.normalizedPath.slice(initialRoot.length + 1);
        setProjectedPath(entry, suffix ? `${scenarioRoot}/${suffix}` : scenarioRoot, taxonomy);
      }
      const mutation = entries.get(mutationRoot);
      if (mutation) {
        mutation.violations = mutation.violations.filter((row) => row.code !== "directory-kind-unresolved");
        setProjectedPath(mutation, mutationRoot, taxonomy);
      }
    }
  }
  for (const entry of entries.values()) {
    if (entry.nodeKind !== "directory" || expected.has(entry.sourcePath)) continue;
    if ([...profileDepths].some(([profile, depth]) => entry.sourcePath.startsWith(`${profile}/`) && entry.sourcePath.slice(profile.length + 1).split("/").length === depth)) entry.violations.push(violation("projection-destination-unregistered", entry.sourcePath, "Projected scenario has no exact catalog vector identity"));
  }
}
//#endregion 🧭️Artifact Mutation Projection

//#region 🛤️Artifact Catalog Projection
type ArtifactProjectionRationale = "artifact-example-model-catalog-projection-v1" | "artifact-editor-command-projection-v1";

interface ArtifactProjectionLocation {
  readonly artifactRoot: string;
  readonly sourceRoot: string;
}

function artifactProjectionContracts(taxonomy: LoadedTaxonomy): readonly Readonly<{ id: string; contract: SemanticPathProjectionContract & { readonly sourceArtifactMemberName: string; readonly rationaleRule: ArtifactProjectionRationale } }>[] {
  return Object.entries(taxonomy.schema.semanticPathProjectionContracts)
    .filter((entry): entry is [string, SemanticPathProjectionContract & { readonly sourceArtifactMemberName: string; readonly rationaleRule: ArtifactProjectionRationale }] => entry[1].sourceArtifactMemberName !== undefined && (entry[1].rationaleRule === "artifact-example-model-catalog-projection-v1" || entry[1].rationaleRule === "artifact-editor-command-projection-v1"))
    .map(([id, contract]) => ({ id, contract }))
    .sort((left, right) => left.id.localeCompare(right.id));
}

function artifactProjectionSourceLocation(path: string, contract: SemanticPathProjectionContract & { readonly sourceArtifactMemberName: string }, taxonomy: LoadedTaxonomy): ArtifactProjectionLocation | null {
  const segments = path.split("/");
  for (let artifactIndex = 0; artifactIndex < segments.length; artifactIndex++) {
    if (segments[artifactIndex] !== contract.sourceArtifactMemberName) continue;
    const owner = taxonomy.schema.semanticDirectoryMemberKinds[contract.sourceOwnerKindId];
    if (!owner?.memberNames.includes(segments[artifactIndex]) || artifactIndex === 0 || segments[artifactIndex - 1] !== canonicalDirectoryName(taxonomy, "artifacts", "artifacts")) continue;
    const sourceNames = segments.slice(artifactIndex + 1, artifactIndex + 1 + contract.sourceSegments.length);
    if (sourceNames.length !== contract.sourceSegments.length) continue;
    const matches = contract.sourceSegments.every((segment, index) => "literal" in segment ? sourceNames[index] === segment.literal : sourceNames[index] !== "");
    if (!matches) continue;
    return { artifactRoot: segments.slice(0, artifactIndex + 1).join("/"), sourceRoot: segments.slice(0, artifactIndex + 1 + contract.sourceSegments.length).join("/") };
  }
  return null;
}

function artifactProjectionAuthorityNodes(repoRoot: string, sourceRoot: string, entries: ReadonlyMap<string, Pick<TaxonomyInventoryEntry, "sourcePath" | "nodeKind">>, taxonomy: LoadedTaxonomy): readonly SemanticProjectionAuthorityNode[] {
  const nodes: SemanticProjectionAuthorityNode[] = [];
  for (const entry of [...entries.values()].filter((candidate) => candidate.sourcePath === sourceRoot || candidate.sourcePath.startsWith(`${sourceRoot}/`)).sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath))) {
    if (isExcluded(entry.sourcePath, taxonomy)) throw new Error(`Artifact projection crosses opaque path ${entry.sourcePath}`);
    if (entry.nodeKind !== "file") {
      nodes.push({ path: entry.sourcePath, nodeKind: entry.nodeKind });
      continue;
    }
    let content: string;
    try {
      content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(absolutePath(repoRoot, entry.sourcePath)));
    } catch {
      throw new Error(`Artifact projection source is not readable UTF-8: ${entry.sourcePath}`);
    }
    nodes.push({ path: entry.sourcePath, nodeKind: "file", content });
  }
  return nodes;
}

function commonProjectionDirectory(paths: readonly string[], floor: string): string {
  if (paths.length === 0) return floor;
  const parts = paths.map((path) => path.split("/"));
  const common: string[] = [];
  for (let index = 0; index < Math.min(...parts.map((row) => row.length)); index++) {
    const segment = parts[0][index];
    if (!parts.every((row) => row[index] === segment)) break;
    common.push(segment);
  }
  const result = common.join("/");
  return result === floor || result.startsWith(`${floor}/`) ? result : floor;
}

function artifactProjectionDirectoryMappings(sourceRoot: string, destinationRoot: string, mappings: readonly Readonly<{ sourcePath: string; destinationPath: string }>[], entries: ReadonlyMap<string, MutableInventoryEntry>): ReadonlyMap<string, string> {
  const result = new Map<string, string>([[sourceRoot, destinationRoot]]);
  const directories = [...entries.values()].filter((entry) => entry.nodeKind === "directory" && entry.sourcePath.startsWith(`${sourceRoot}/`)).sort((left, right) => left.sourcePath.split("/").length - right.sourcePath.split("/").length || generatorPathCompare(left.sourcePath, right.sourcePath));
  for (const entry of directories) {
    const descendants = mappings.filter((mapping) => mapping.sourcePath.startsWith(`${entry.sourcePath}/`)).map((mapping) => dirname(mapping.destinationPath));
    if (descendants.length === 0) continue;
    const parentDestination = result.get(dirname(entry.sourcePath)) ?? destinationRoot;
    const common = commonProjectionDirectory(descendants, parentDestination);
    const commonSegments = common.split("/");
    const parentSegments = parentDestination.split("/");
    let destination = common;
    for (let index = commonSegments.length - 1; index >= parentSegments.length; index--) if (commonSegments[index] === basename(entry.sourcePath)) {
      destination = commonSegments.slice(0, index + 1).join("/");
      break;
    }
    result.set(entry.sourcePath, destination);
  }
  return result;
}

function applyArtifactProjectionPath(entry: MutableInventoryEntry, destinationPath: string, taxonomy: LoadedTaxonomy): void {
  entry.normalizedPath = destinationPath.normalize("NFC");
  const superseded = new Set(["directory-kind-ambiguous", "directory-kind-unresolved", "file-kind-ambiguous", "file-kind-unresolved", "semantic-stem-ambiguous", "semantic-stem-unresolved", "path-too-long", "trailing-dot-or-space", "windows-reserved-name"]);
  entry.violations = [...entry.violations.filter((row) => !superseded.has(row.code)), ...pathPolicyViolations(entry.normalizedPath, taxonomy)];
}

function projectArtifactCatalogs(repoRoot: string, entries: Map<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy): void {
  const projectedSources = new Map<string, string>();
  const projectedDestinations = new Map<string, string>();
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const candidates = [...entries.values()]
      .filter((entry) => entry.nodeKind === "directory")
      .map((entry) => ({ entry, location: artifactProjectionSourceLocation(entry.sourcePath, contract, taxonomy) }))
      .filter((row): row is { readonly entry: MutableInventoryEntry; readonly location: ArtifactProjectionLocation } => row.location !== null && row.entry.sourcePath === row.location.sourceRoot)
      .sort((left, right) => generatorPathCompare(left.entry.sourcePath, right.entry.sourcePath));
    for (const { entry: root, location } of candidates) {
      const nodes = artifactProjectionAuthorityNodes(repoRoot, location.sourceRoot, entries, taxonomy);
      const occupiedPaths = [...entries.keys()].filter((path) => path !== location.sourceRoot && !path.startsWith(`${location.sourceRoot}/`)).sort(generatorPathCompare);
      const authority = semanticPathProjectionAuthority({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot, nodes, occupiedPaths }, taxonomy.discoverySchema);
      if (authority.problems.length > 0) {
        root.violations.push(violation("projection-authority-invalid", root.sourcePath, `${id}: ${authority.problems.join(" | ")}`));
        continue;
      }
      const accepted = authority.mappings.map((mapping) => ({ sourcePath: normalizeRelative(mapping.sourcePath), destinationPath: normalizeRelative(mapping.destinationPath) }));
      for (const mapping of accepted) {
        const source = entries.get(mapping.sourcePath);
        const priorSource = projectedSources.get(mapping.sourcePath);
        const priorDestination = projectedDestinations.get(mapping.destinationPath);
        if (!source || source.nodeKind !== "file" || priorSource && priorSource !== mapping.destinationPath || priorDestination && priorDestination !== mapping.sourcePath) {
          root.violations.push(violation("projection-mapping-collision", mapping.sourcePath, `${id} does not own one unique admitted source/destination pair`));
          continue;
        }
        projectedSources.set(mapping.sourcePath, mapping.destinationPath);
        projectedDestinations.set(mapping.destinationPath, mapping.sourcePath);
        applyArtifactProjectionPath(source, mapping.destinationPath, taxonomy);
      }
      for (const [sourcePath, destinationPath] of artifactProjectionDirectoryMappings(location.sourceRoot, authority.destinationRoot, accepted, entries)) {
        const source = entries.get(sourcePath);
        if (source) applyArtifactProjectionPath(source, destinationPath, taxonomy);
      }
    }
    const artifactRoots = [...entries.values()].filter((entry) => entry.nodeKind === "directory" && basename(entry.sourcePath) === contract.sourceArtifactMemberName && basename(posix.dirname(entry.sourcePath)) === canonicalDirectoryName(taxonomy, "artifacts", "artifacts"));
    for (const artifact of artifactRoots) {
      const registered = new Map(artifactPathProjectionCatalogRoots(artifact.sourcePath, id, taxonomy.discoverySchema).map((root) => [root.destinationRoot, root]));
      const destinations = [...entries.values()].filter((entry) => {
        if (entry.nodeKind !== "directory" || !entry.sourcePath.startsWith(`${artifact.sourcePath}/`)) return false;
        const names = entry.sourcePath.slice(artifact.sourcePath.length + 1).split("/");
        return names.length === contract.destinationSegments.length && contract.destinationSegments.every((segment, index) => "literal" in segment ? names[index] === segment.literal : names[index] !== "");
      });
      for (const destination of destinations) {
        const root = registered.get(destination.sourcePath);
        if (!root) {
          destination.violations.push(violation("projection-destination-unregistered", destination.sourcePath, `${id} destination has no exact forward profile vector`));
          continue;
        }
        const nodes = artifactProjectionAuthorityNodes(repoRoot, root.destinationRoot, entries, taxonomy);
        const admitted = new Set(nodes.map(({ path }) => path));
        const incomplete = nodes.some((node) => node.nodeKind === "directory" && readdirSync(assertLexicalInputOutsideOpaque(repoRoot, node.path, "Canonical artifact catalog", true)).some((name) => !admitted.has(`${node.path}/${name}`)));
        const authority = semanticPathProjectionAuthority({ artifactRoot: artifact.sourcePath, contractId: id, sourceRoot: root.sourceRoot, nodes, layout: "destination" }, taxonomy.discoverySchema);
        if (incomplete || authority.problems.length > 0) {
          destination.violations.push(violation("projection-authority-invalid", destination.sourcePath, `${id}: ${incomplete ? "Canonical catalog has an unadmitted physical child" : authority.problems.join(" | ")}`));
          continue;
        }
        for (const node of nodes) {
          const entry = entries.get(node.path);
          if (entry) applyArtifactProjectionPath(entry, entry.sourcePath, taxonomy);
        }
      }
    }
  }
}

function artifactCatalogProjectionRationale(sourcePath: string, destinationPath: string, taxonomy: LoadedTaxonomy): ArtifactProjectionRationale | null {
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const location = artifactProjectionSourceLocation(sourcePath, contract, taxonomy);
    if (!location) continue;
    const rendered = renderArtifactPathProjectionRoot({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot }, taxonomy.discoverySchema);
    if (rendered.problems.length === 0 && (destinationPath === rendered.destinationRoot || destinationPath.startsWith(`${rendered.destinationRoot}/`))) return contract.rationaleRule;
  }
  return null;
}
//#endregion 🛤️Artifact Catalog Projection

//#region 📦️Nested Cargo Packages
function projectNestedCargoPackages(repoRoot: string, entries: Map<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy, prunableSourceParents: ReadonlySet<string>): void {
  const catalog = semanticPackageProjectionCatalog(repoRoot, taxonomy.discoverySchema);
  if (!catalog) return;
  for (const row of catalog.packages) {
    const sourceManifest = `${row.sourceRoot}/Cargo.toml`, destinationManifest = `${row.destinationRoot}/Cargo.toml`;
    if (!entries.has(sourceManifest) && !entries.has(destinationManifest)) continue;
    const destination = entries.has(destinationManifest), root = destination ? row.semanticOwnerRoot : row.sourceRoot;
    const candidates = [...entries.values()].filter((entry) => (entry.sourcePath === root || entry.sourcePath.startsWith(root + "/")) && !(entry.nodeKind === "directory" && prunableSourceParents.has(entry.sourcePath)));
    const anchor = entries.get(destination ? destinationManifest : sourceManifest)!;
    try {
      const nodes = candidates.map((entry): SemanticProjectionAuthorityNode => ({ path: entry.sourcePath, nodeKind: entry.nodeKind, ...(entry.nodeKind === "file" ? { content: readFileSync(assertLexicalInputOutsideOpaque(repoRoot, entry.sourcePath, "Nested Cargo source", true), "utf8") } : {}) }));
      const admitted = new Set(candidates.map((entry) => entry.sourcePath));
      const ignoredOutputParents = new Set(destination ? semanticPackageIgnoredGeneratedOutputPaths(row, taxonomy.discoverySchema).map((path) => posix.dirname(path)) : []);
      for (const directory of candidates.filter((entry) => entry.nodeKind === "directory")) {
        for (const name of readdirSync(assertLexicalInputOutsideOpaque(repoRoot, directory.sourcePath, "Nested Cargo directory", true))) {
          const path = directory.sourcePath + "/" + name;
          if (admitted.has(path)) continue;
          if (ignoredOutputParents.has(path)) {
            const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Nested Cargo ignored output parent", true);
            const stat = lstatOrNull(absolute);
            if (stat?.isDirectory() && !stat.isSymbolicLink() && readdirSync(absolute).length === 0) { nodes.push({ path, nodeKind: "directory" }); continue; }
          }
          if (prunableSourceParents.has(path)) {
            if (!lstatOrNull(assertLexicalInputOutsideOpaque(repoRoot, path, "Nested Cargo prunable source parent", true))?.isDirectory()) throw new Error("Nested Cargo prunable source parent changed kind: " + path);
            continue;
          }
          const relativePath = path.slice(row.sourceRoot.length + 1);
          if (!destination && row.ignoredSourcePatterns.some((pattern) => taxonomy.pathMatcher.matches(relativePath, pattern) || pattern.endsWith("/**") && relativePath === pattern.slice(0, -3))) continue;
          throw new Error("Nested Cargo has an unadmitted physical child: " + path);
        }
      }
      const occupiedPaths = [...entries.keys()].filter((path) => !admitted.has(path));
      if (!destination) for (const path of new Set([...row.mappings.map((mapping) => mapping.destinationPath), ...row.adapters.map((adapter) => adapter.path), ...row.derivedLeaves.map((leaf) => leaf.path)])) {
        if (lstatOrNull(assertLexicalInputOutsideOpaque(repoRoot, path, "Nested Cargo destination", true))) occupiedPaths.push(path);
      }
      const evidence = row.workspaceKind === "repository" ? { cargoWorkspaceContent: readFileSync(assertLexicalInputOutsideOpaque(repoRoot, "Cargo.toml", "Nested Cargo workspace", true), "utf8"), nodeWorkspaceContent: readFileSync(assertLexicalInputOutsideOpaque(repoRoot, "package.json", "Nested Node workspace", true), "utf8") } : {};
      const authority = semanticPackageProjectionAuthority({ packageId: row.id, nodes, layout: destination ? "destination" : "source", occupiedPaths, ...evidence }, catalog, taxonomy.discoverySchema);
      if (authority.problems.length > 0) { anchor.violations.push(violation("nested-cargo-authority-invalid", anchor.sourcePath, authority.problems.join(" | "))); continue; }
      const mappingByPath = new Map(authority.mappings.map((mapping) => [destination ? mapping.destinationPath : mapping.sourcePath, mapping]));
      for (const entry of candidates) {
        const mapping = mappingByPath.get(entry.sourcePath);
        applyArtifactProjectionPath(entry, mapping?.destinationPath ?? (entry.sourcePath === row.sourceRoot && !destination && row.sourceRoot !== row.semanticOwnerRoot ? row.destinationRoot : entry.sourcePath), taxonomy);
        entry.violations = entry.violations.filter((problem) => !["package-role-unresolved", "package-implementation-file", "package-implementation-destination-unresolved"].includes(problem.code));
        if (entry.nodeKind === "file") {
          entry.semanticStem = null;
          entry.packageRole = mapping?.disposition === "implementation" || mapping?.disposition === "generated" || row.derivedLeaves.some((leaf) => leaf.path === entry.sourcePath) ? "implementation" : "configuration";
          if (basename(entry.sourcePath) === "Cargo.toml") entry.fixedContractId = "cargo-manifest";
        }
      }
    } catch (error) { anchor.violations.push(violation("nested-cargo-authority-invalid", anchor.sourcePath, error instanceof Error ? error.message : String(error))); }
  }
}

//#endregion 📦️Nested Cargo Packages

//#region 📃️Exact Owner Leaves
function exactOwnedFileCatalog(repoRoot: string, taxonomy: LoadedTaxonomy): SemanticExactOwnedFileCatalog | null {
  return semanticExactOwnedFileCatalog(repoRoot, taxonomy.discoverySchema);
}

interface ExactOwnedCatalogSnapshot {
  readonly catalog: SemanticExactOwnedFileCatalog | null;
  readonly input: SemanticOwnedInputFileSnapshot | null;
}

function exactOwnedCatalogSnapshot(repoRoot: string, taxonomy: LoadedTaxonomy): ExactOwnedCatalogSnapshot {
  let input: SemanticOwnedInputFileSnapshot | null = null;
  const catalog = semanticExactOwnedFileCatalog(repoRoot, taxonomy.discoverySchema, (snapshot) => { input = snapshot; });
  return { catalog, input };
}

function exactOwnedCurrentRevisions(taxonomy: LoadedTaxonomy): ReturnType<typeof parseSemanticOwnedCurrentSourceRevisions> | undefined {
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  return contract?.contractKind === "exact-owner-path-catalog" ? contract.currentSourceRevisions : undefined;
}

function exactOwnedCurrentRawPath(path: string, snapshot: ExactOwnedCatalogSnapshot, taxonomy: LoadedTaxonomy): boolean {
  return Object.values(exactOwnedCurrentRevisions(taxonomy) ?? {}).some((row) => row.sourcePath === path || snapshot.catalog?.cases[row.catalogCaseIndex]?.sourcePath === path);
}

function exactOwnedFileResolution(repoRoot: string, entry: TaxonomyInventoryEntry, snapshot: ExactOwnedCatalogSnapshot, taxonomy: LoadedTaxonomy): Readonly<{ result: ReturnType<typeof semanticExactOwnedFileProjectionAuthority>; sourceAuthority?: TaxonomyMoveSourceAuthority }> {
  const catalog = snapshot.catalog, owner = catalog?.cases.find((row) => row.sourcePath === entry.sourcePath || row.destinationPath === entry.sourcePath);
  if (!catalog || !owner) throw new Error("Exact owner catalog is absent or does not govern this source: " + entry.sourcePath);
  const source = assertLexicalInputOutsideOpaque(repoRoot, owner.sourcePath, "Exact owner source", true), destination = assertLexicalInputOutsideOpaque(repoRoot, owner.destinationPath, "Exact owner destination", true);
  const destinationPreimage = lstatOrNull(destination)?.isFile() ? leafPreimage(destination) : undefined;
  const facts = { path: entry.sourcePath, nodeKind: entry.nodeKind, contentHash: entry.contentHash, mode: entry.mode, size: entry.size, sourcePresent: lstatOrNull(source) !== null, destinationPresent: lstatOrNull(destination) !== null, destinationPreimage, occupiedPaths: exactOwnedDestinationOccupancy(repoRoot, owner.destinationPath) };
  if (!exactOwnedCurrentRawPath(entry.sourcePath, snapshot, taxonomy)) return { result: semanticExactOwnedFileProjectionAuthority(catalog, facts) };
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  if (contract?.contractKind !== "exact-owner-path-catalog" || !contract.currentSourceRevisions || !taxonomy.input || !snapshot.input) throw new Error("Current source is missing its exact parsed schema or catalog snapshot");
  const selected = Object.values(contract.currentSourceRevisions).find((row) => row.catalogCaseIndex === catalog.cases.indexOf(owner));
  if (!selected || selected.sourcePath !== owner.sourcePath || entry.sourcePath !== owner.sourcePath) throw new Error("Current source coordinate does not match its original catalog row");
  const sourceInput = semanticOwnedInputFileSnapshot(repoRoot, entry.sourcePath);
  if (!sourceInput || canonicalJson(inventoryLeafPreimage(entry)) !== canonicalJson({ nodeKind: "file", contentHash: sourceInput.contentHash, mode: sourceInput.mode, size: sourceInput.size })) throw new Error("Current source changed since inventory: " + entry.sourcePath);
  const expectation = semanticOwnedInputFileSnapshot(repoRoot, selected.expectationsPath);
  if (!expectation) throw new Error("Current source expectation is absent: " + selected.expectationsPath);
  const result = semanticExactOwnedFileProjectionAuthority(catalog, facts, { contract, revisions: contract.currentSourceRevisions, expectations: [expectation] });
  const authority = result.currentSource;
  if (result.disposition !== "project" || result.problems.length || authority?.disposition !== "revised" || authority.revisionId !== "testing-readme-protocol-v2-reviewed" || !authority.revisionDigest) return { result };
  const inputs: TaxonomyMoveSourceAuthority["inputs"] = ([{ role: "schema", input: taxonomy.input }, { role: "catalog", input: snapshot.input }, { role: "expectation", input: expectation }] as const).map(({ role, input }) => ({ role, path: input.path, preimage: { nodeKind: "file" as const, contentHash: input.contentHash, mode: input.mode, size: input.size } })).sort((left, right) => generatorPathCompare(left.path, right.path));
  return { result, sourceAuthority: { kind: "exact-owner-current-source-revision-v1", revisionId: authority.revisionId, revisionDigest: authority.revisionDigest, inputs } };
}

function exactOwnedDestinationOccupancy(repoRoot: string, path: string): readonly string[] {
  const owner = dirname(dirname(path));
  const segments = [basename(dirname(path)), basename(path)];
  const occupied: string[] = [];
  let parent = owner;
  for (let index = 0; index < segments.length; index++) {
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, parent, "Exact owner destination", true);
    const stat = lstatOrNull(absolute);
    if (!stat) break;
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Exact owner destination has an occupied non-directory parent: " + parent);
    const matches = readdirSync(absolute).filter((child) => collisionKey(child, "vs16-fold") === collisionKey(segments[index], "vs16-fold"));
    for (const match of matches) {
      const found = [owner, ...segments.slice(0, index), match, ...segments.slice(index + 1)].join("/");
      if (match !== segments[index] || index === segments.length - 1) occupied.push(found);
    }
    if (!matches.includes(segments[index])) break;
    parent += "/" + segments[index];
  }
  return occupied;
}
function exactOwnedEvidenceProblems(repoRoot: string, entry: SemanticExactOwnedFileCase, catalog: SemanticExactOwnedFileCatalog): readonly string[] {
  const evidence = catalog.ownerEvidence[entry.ownerEvidenceId];
  const problems: string[] = [];
  for (const declared of evidence.evidencePaths) {
    const projected = catalog.cases.find((candidate) => candidate.sourcePath === declared);
    const path = projected && !lstatOrNull(absolutePath(repoRoot, declared)) ? projected.destinationPath : declared;
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Exact owner evidence", true);
    const stat = lstatOrNull(absolute);
    if (!stat?.isFile()) {
      problems.push("Exact owner evidence is absent or not a regular file: " + path);
      continue;
    }
    if (evidence.expectedPackageName && basename(path) === "package.json") {
      let manifest: JsonRecord;
      try { manifest = record(JSON.parse(readFileSync(absolute, "utf8")), "Exact publisher manifest"); } catch { problems.push("Exact publisher manifest is invalid: " + path); continue; }
      if (manifest.name !== evidence.expectedPackageName || (manifest.private === true) !== evidence.private) problems.push("Exact publisher identity or private status drifted: " + path);
    }
  }
  return problems;
}

function projectExactOwnedFiles(repoRoot: string, entries: Map<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy): void {
  const snapshot = exactOwnedCatalogSnapshot(repoRoot, taxonomy), catalog = snapshot.catalog;
  if (!catalog) {
    for (const row of Object.values(exactOwnedCurrentRevisions(taxonomy) ?? {})) {
      const entry = entries.get(row.sourcePath);
      if (!entry) continue;
      entry.normalizedPath = entry.sourcePath;
      entry.violations.push(violation("owner-leaf-authority-invalid", entry.sourcePath, "Current source requires its exact original catalog"));
    }
    return;
  }
  const governed = new Map(catalog.cases.flatMap((entry) => [[entry.sourcePath, entry], [entry.destinationPath, entry]] as const));
  for (const entry of entries.values()) {
    const owner = governed.get(entry.sourcePath);
    if (!owner || entry.nodeKind === "directory") continue;
    if (reservedDocumentationBasename(basename(entry.sourcePath))) {
      entry.normalizedPath = entry.sourcePath;
      continue;
    }
    let problems: readonly string[] = [], disposition = "problem";
    try {
      const { result } = exactOwnedFileResolution(repoRoot, entry, snapshot, taxonomy);
      problems = [...result.problems, ...exactOwnedEvidenceProblems(repoRoot, owner, catalog)];
      disposition = result.disposition;
    } catch (error) { problems = [error instanceof Error ? error.message : String(error)]; }
    if (problems.length > 0 || disposition === "problem") {
      entry.normalizedPath = entry.sourcePath;
      entry.violations.push(violation("owner-leaf-authority-invalid", entry.sourcePath, problems.join(" | ")));
      continue;
    }
    applyArtifactProjectionPath(entry, owner.destinationPath, taxonomy);
    entry.fileKind = "markdown";
    entry.semanticStem = null;
    entry.packageRole = packageLocation(entry.sourcePath, taxonomy) ? "configuration" : "not-package";
    entry.violations = entry.violations.filter((problem) => !["package-role-unresolved", "package-implementation-file", "package-implementation-destination-unresolved"].includes(problem.code));
    if (owner.fixedContractId) entry.fixedContractId = owner.fixedContractId;
  }
  const destinations = new Map(catalog.cases.filter((entry) => entry.disposition !== "fixed").map((entry) => [dirname(entry.destinationPath), entry]));
  for (const directory of entries.values()) {
    if (directory.nodeKind !== "directory" || !["📃️readme", "⚖️license"].includes(basename(directory.sourcePath))) continue;
    const owner = destinations.get(directory.sourcePath);
    let valid = false;
    if (owner) {
      const sourcePresent = lstatOrNull(absolutePath(repoRoot, owner.sourcePath)) !== null;
      const absolute = assertLexicalInputOutsideOpaque(repoRoot, directory.sourcePath, "Exact owner directory", true);
      const names = readdirSync(absolute);
      const leaf = lstatOrNull(absolutePath(repoRoot, owner.destinationPath));
      const generatedPair = owner.generatorOwnerId !== null && sourcePresent && leaf?.isFile() && canonicalJson(leafPreimage(absolutePath(repoRoot, owner.destinationPath))) === canonicalJson({ nodeKind: "file", contentHash: owner.preimage.sha256, mode: Number.parseInt(owner.preimage.mode, 8), size: owner.preimage.size });
      valid = (!sourcePresent || generatedPair) && names.length === 1 && names[0] === "📝️.md" && Boolean(leaf?.isFile() && !leaf.isSymbolicLink());
    }
    if (valid) applyArtifactProjectionPath(directory, directory.sourcePath, taxonomy);
    else {
      const problem = violation("projection-only-owner-invalid", directory.sourcePath, "Owner documentation directories require an exact registered owner, one Markdown leaf, and no raw sibling");
      directory.violations.push(problem);
      for (const child of entries.values()) if (dirname(child.sourcePath) === directory.sourcePath) child.violations.push({ ...problem, path: child.sourcePath });
    }
  }
}
//#endregion 📃️Exact Owner Leaves

//#region 📌️Ticket Important Projection
export interface TicketImportantExactMutationAuthority {
  readonly catalogPath: string;
  readonly catalogContentHash: string;
  readonly caseId: string;
  readonly disposition: "move" | "remove";
  readonly sourcePath: string;
  readonly destinationPath: string | null;
  readonly sourcePreimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }>;
}

function ticketImportantExactMutationCases(repoRoot: string): readonly TicketImportantExactMutationAuthority[] {
  const catalogPath = TICKET_IMPORTANT_EXACT_MUTATIONS_FIXTURE_PATH;
  const catalog = absolutePath(repoRoot, catalogPath);
  const catalogStat = lstatOrNull(catalog);
  if (!catalogStat) {
    const governed = TICKET_IMPORTANT_EXACT_GOVERNED_SOURCES.find((path) => lstatOrNull(absolutePath(repoRoot, path)) !== null);
    if (governed) throw new Error(`Ticket important exact mutation catalog is absent for governed source ${governed}`);
    return [];
  }
  if (!catalogStat.isFile() || catalogStat.isSymbolicLink()) throw new Error("Ticket important exact mutation catalog must be a regular file");
  const bytes = readFileSync(catalog);
  const root = planRecord(JSON.parse(bytes.toString("utf8")) as unknown, "ticket important exact mutation catalog", ["schemaVersion", "cases"]);
  if (root.schemaVersion !== 1 || !Array.isArray(root.cases)) throw new Error("Ticket important exact mutation catalog schema is invalid");
  const catalogContentHash = sha256(bytes);
  const rows = root.cases.map((value, index) => {
    const row = planRecord(value, `ticket important exact mutation catalog cases[${index}]`, ["id", "disposition", "sourcePath", "destinationPath", "sourcePreimage"]);
    if (row.disposition !== "move" && row.disposition !== "remove") throw new Error(`Ticket important exact mutation catalog case ${index} has an invalid disposition`);
    const sourcePreimage = parseLeafPreimage(row.sourcePreimage, `ticket important exact mutation catalog cases[${index}].sourcePreimage`);
    if (sourcePreimage.nodeKind !== "file" || row.disposition === "remove" && (sourcePreimage.size !== 0 || row.destinationPath !== null) || row.disposition === "move" && typeof row.destinationPath !== "string") throw new Error(`Ticket important exact mutation catalog case ${index} has invalid leaf evidence`);
    return { catalogPath, catalogContentHash, caseId: planString(row.id, `ticket important exact mutation catalog cases[${index}].id`), disposition: row.disposition, sourcePath: planPath(row.sourcePath, `ticket important exact mutation catalog cases[${index}].sourcePath`), destinationPath: row.destinationPath === null ? null : planPath(row.destinationPath, `ticket important exact mutation catalog cases[${index}].destinationPath`), sourcePreimage } as TicketImportantExactMutationAuthority;
  });
  if (new Set(rows.map((entry) => entry.caseId)).size !== rows.length || new Set(rows.map((entry) => entry.sourcePath)).size !== rows.length || rows.some((entry, index) => index > 0 && generatorPathCompare(rows[index - 1].caseId, entry.caseId) >= 0)) throw new Error("Ticket important exact mutation catalog identities must be unique and bytewise sorted");
  return rows;
}

/** 🧩️ Resolves only a frozen exact ticket-important physical mutation with complete leaf evidence. */
export function ticketImportantExactMutationAuthority(repoRoot: string, sourcePath: string, sourcePreimage: TaxonomyLeafPreimage): TicketImportantExactMutationAuthority | null {
  return ticketImportantExactMutationCases(repoRoot).find((entry) => entry.sourcePath === sourcePath && canonicalJson(entry.sourcePreimage) === canonicalJson(sourcePreimage)) ?? null;
}

function ticketManifestState(content: string | undefined): "closed" | "invalid" | "missing" | "open" {
  if (content === undefined) return "missing";
  try {
    const value = JSON.parse(content) as unknown;
    const status = typeof value === "object" && value !== null && !Array.isArray(value) ? (value as JsonRecord).status : undefined;
    return status === "closed" || status === "open" ? status : "invalid";
  } catch { return "invalid"; }
}

function projectArtifactEmptyFacetFiles(repoRoot: string, entries: Map<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy): void {
  for (const entry of entries.values()) {
    if (entry.nodeKind !== "file" || basename(entry.sourcePath) !== "📌️.empty.md") continue;
    const authority = semanticArtifactEmptyFacetProjectionAuthority({ sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "" }, taxonomy.discoverySchema);
    if (authority.disposition !== "project" || !authority.destinationPath) continue;
    applyArtifactProjectionPath(entry, authority.destinationPath, taxonomy);
    const owner = absolutePath(repoRoot, dirname(entry.sourcePath));
    assertNoFollowAncestors(repoRoot, owner, "Artifact empty-facet owner", true);
    const destinationName = basename(authority.destinationPath);
    if (readdirSync(owner).some((name) => collisionKey(name, "vs16-fold") === collisionKey(destinationName, "vs16-fold"))) entry.violations.push(violation("projection-destination-occupied", entry.sourcePath, "Artifact empty-facet primary destination is occupied or folded-colliding"));
  }
}

function projectTicketDocumentFiles(repoRoot: string, entries: Map<string, MutableInventoryEntry>, fixedDirectoryContractByPath: ReadonlyMap<string, string>, taxonomy: LoadedTaxonomy): void {
  for (const entry of entries.values()) {
    if (entry.nodeKind !== "file" || basename(entry.sourcePath) !== "ticket.md") continue;
    const ownerPath = dirname(entry.sourcePath);
    const authority = semanticOwnedPrimaryFileProjectionAuthority({ ownerPath, ownerFixedDirectoryContractIds: fixedDirectoryContractByPath.get(ownerPath) === "ticket-slug" ? ["ticket-slug"] : [], sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "" }, taxonomy.discoverySchema);
    if (authority.disposition !== "project" || !authority.destinationPath) continue;
    applyArtifactProjectionPath(entry, authority.destinationPath, taxonomy);
    const owner = absolutePath(repoRoot, ownerPath);
    assertNoFollowAncestors(repoRoot, owner, "Ticket document owner", true);
    const destinationName = basename(authority.destinationPath);
    if (readdirSync(owner).some((name) => collisionKey(name, "vs16-fold") === collisionKey(destinationName, "vs16-fold"))) entry.violations.push(violation("projection-destination-occupied", entry.sourcePath, "Ticket document primary destination is occupied or folded-colliding"));
  }
}

function projectTicketImportantFiles(repoRoot: string, entries: Map<string, MutableInventoryEntry>, fixedDirectoryContractByPath: ReadonlyMap<string, string>, taxonomy: LoadedTaxonomy): void {
  for (const entry of [...entries.values()].filter((candidate) => candidate.nodeKind === "file" && basename(candidate.sourcePath) === "📌️important.md").sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath))) {
    const ownerPath = dirname(entry.sourcePath);
    const manifestPath = `${ownerPath}/🎫️ticket.json`;
    const manifest = entries.get(manifestPath);
    const manifestClaimed = manifest?.nodeKind === "file" && manifest.fixedContractId === "ticket-manifest";
    const manifestContent = manifestClaimed ? readFileSync(absolutePath(repoRoot, manifestPath), "utf8") : undefined;
    if (manifestContent !== undefined) {
      const active = semanticOwnedFileProjectionAuthority({ ownerPath, ownerFixedDirectoryContractIds: fixedDirectoryContractByPath.get(ownerPath) === "ticket-slug" ? ["ticket-slug"] : [], manifestPath, manifestFixedFilenameContractIds: ["ticket-manifest"], manifestContent, sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "", sourceByteLength: entry.size }, taxonomy.discoverySchema);
      if (active.disposition === "project" && active.destinationPath) {
        applyArtifactProjectionPath(entry, active.destinationPath, taxonomy);
        continue;
      }
      if (active.disposition === "remove") {
        applyArtifactProjectionPath(entry, entry.sourcePath, taxonomy);
        continue;
      }
    }
    const history = semanticOwnedFileHistoryProjectionAuthority({ ownerPath, ownerFixedDirectoryContractIds: fixedDirectoryContractByPath.get(ownerPath) === "ticket-slug" ? ["ticket-slug"] : [], manifestPath: manifestClaimed ? manifestPath : undefined, manifestFixedFilenameContractIds: manifestClaimed ? ["ticket-manifest"] : [], manifestContent, sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "", sourceByteLength: entry.size }, taxonomy.discoverySchema);
    if (history.disposition === "project" && history.destinationPath) {
      applyArtifactProjectionPath(entry, history.destinationPath, taxonomy);
      continue;
    }
    const exact = ticketImportantExactMutationAuthority(repoRoot, entry.sourcePath, inventoryLeafPreimage(entry));
    if (exact?.disposition === "move" && exact.destinationPath) applyArtifactProjectionPath(entry, exact.destinationPath, taxonomy);
    else if (exact?.disposition === "remove") applyArtifactProjectionPath(entry, entry.sourcePath, taxonomy);
  }
}

function validateTicketImportantDirectories(repoRoot: string, entries: Map<string, MutableInventoryEntry>, directoryKindByPath: ReadonlyMap<string, string>, fixedDirectoryContractByPath: ReadonlyMap<string, string>): void {
  for (const directory of [...entries.values()].filter((entry) => entry.nodeKind === "directory" && ["ticket-important", "ticket-important-history"].includes(directoryKindByPath.get(entry.sourcePath) ?? ""))) {
    const kindId = directoryKindByPath.get(directory.sourcePath)!;
    const ownerPath = dirname(directory.sourcePath);
    const children = [...entries.values()].filter((entry) => dirname(entry.sourcePath) === directory.sourcePath);
    const leaf = children.length === 1 && children[0]?.nodeKind === "file" && basename(children[0].sourcePath) === "📝️.md" ? children[0] : undefined;
    const manifestPath = `${ownerPath}/🎫️ticket.json`;
    const manifest = entries.get(manifestPath);
    const manifestContent = manifest?.nodeKind === "file" && manifest.fixedContractId === "ticket-manifest" ? readFileSync(absolutePath(repoRoot, manifestPath), "utf8") : undefined;
    const manifestState = ticketManifestState(manifestContent);
    const ownerClaimed = fixedDirectoryContractByPath.get(ownerPath) === "ticket-slug";
    const rawAbsent = !entries.has(`${ownerPath}/📌️important.md`);
    const lifecycleAdmitted = kindId === "ticket-important" && manifestState === "open";
    const historyAdmitted = kindId === "ticket-important-history" && (manifestState === "missing" || manifestState === "invalid" || manifestState === "closed" && Boolean(leaf && leaf.size > 0));
    if (ownerClaimed && leaf && rawAbsent && (lifecycleAdmitted || historyAdmitted)) continue;
    const problem = violation("projection-only-owner-invalid", directory.sourcePath, `${kindId} must be an exact ticket-owned lifecycle/history directory with one Markdown leaf and no raw sibling`);
    directory.violations.push(problem);
    for (const child of children) child.violations.push({ ...problem, path: child.sourcePath });
  }
}
//#endregion 📌️Ticket Important Projection

//#region 🧬️Mutation Payload Schema Authority
/** 🪢️ Validates each descriptor's authored schema pointer without relocating its payload file. */
function validateMutationPayloadSchemas(repoRoot: string, entries: Map<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy): void {
  const contract = taxonomy.discoverySchema.mutationPayloadSchemaAuthority;
  const descriptorKind = taxonomy.discoverySchema.fileKinds[contract.descriptorFileKindId]!;
  const descriptorFilename = `${descriptorKind.emoji}${descriptorKind.extensionChains[0]}`;
  const owners = new Map<string, { identity: string; children: MutableInventoryEntry[] }>();
  for (const entry of entries.values()) {
    const owner = entry.nodeKind === "directory" ? entry.sourcePath : dirname(entry.sourcePath), marker = owner.lastIndexOf("/🧬️mutations/");
    if (marker < 0) continue;
    const root = owner.slice(0, marker + "/🧬️mutations".length), identity = mutationOwnerIdentity(root, owner.slice(root.length + 1), taxonomy.discoverySchema);
    if (identity === null) continue;
    const state = owners.get(owner) ?? { identity, children: [] };
    state.children.push(entry);
    owners.set(owner, state);
  }
  for (const [owner, { identity, children }] of owners) {
    const descriptorPath = `${owner}/${descriptorFilename}`, descriptorEntry = entries.get(descriptorPath);
    const entry = descriptorEntry ?? entries.get(owner) ?? children[0]!;
    try {
      if (descriptorEntry?.nodeKind !== "file") throw new Error("Mutation owner requires one canonical regular-file descriptor");
      const content = (path: string): string => {
        const bytes = readFileSync(assertLexicalInputOutsideOpaque(repoRoot, path, "Mutation payload authority", true)), value = bytes.toString("utf8");
        if (!Buffer.from(value, "utf8").equals(bytes)) throw new Error("Payload authority must be exact UTF-8");
        return value;
      };
      for (const child of children) {
        if (child.nodeKind !== "file" || child.sourcePath === descriptorPath || !child.sourcePath.endsWith(".json")) continue;
        let other: unknown;
        try { other = JSON.parse(content(child.sourcePath)); } catch { continue; }
        if (other !== null && typeof other === "object" && !Array.isArray(other) && ["schemaVersion", contract.descriptorOwnerField, contract.descriptorIdentityField, contract.descriptorField].every((field) => Object.hasOwn(other, field))) throw new Error("Mutation owner contains a competing descriptor");
      }
      const descriptorContent = content(descriptorPath), descriptor = record(JSON.parse(descriptorContent), "Mutation payload descriptor"), pointer = descriptor[contract.descriptorField];
      if (!(Array.isArray(descriptor.requiredLanguageSurfaces) && descriptor.requiredLanguageSurfaces.includes("json-schema")) && !(typeof pointer === "string" && pointer.endsWith(".json"))) continue;
      if (jsonDocumentDuplicateKeys(descriptorContent).length > 0) throw new Error("Mutation payload descriptor has duplicate JSON members");
      if (descriptor.schemaVersion !== contract.descriptorSchemaVersion || descriptor[contract.descriptorOwnerField] !== owner || descriptor[contract.descriptorIdentityField] !== identity) throw new Error("Descriptor version and semantic identity must belong to the exact source owner");
      const problems = mutationPayloadSchemaProblems(owner, pointer, (path) => {
        const source = entries.get(path);
        return { kind: source?.nodeKind ?? "absent", ...(source?.nodeKind === "file" ? { content: content(path) } : {}), repositoryBoundary: isExcluded(path, taxonomy) };
      }, contract.jsonSchemaDialect);
      if (problems.length > 0) throw new Error(problems.join("; "));
    } catch (error) {
      entry.violations.push(violation("mutation-payload-schema-authority-invalid", entry.sourcePath, error instanceof Error ? error.message : String(error)));
    }
  }
}

//#endregion 🧬️Mutation Payload Schema Authority

/** 🧱️ Inventories Git-index paths and explicitly admitted ticket paths without traversing opaque exclusions or following symlinks. */
export function inventoryTaxonomy(options: TaxonomyInventoryOptions): TaxonomyInventory {
  return inventoryTaxonomyWithSourceParentPruning(options, new Set());
}

/** 🪵️ Projects only transaction-proven empty source parents into package authority before final classification. */
function inventoryTaxonomyWithSourceParentPruning(options: TaxonomyInventoryOptions, prunableSourceParents: ReadonlySet<string>): TaxonomyInventory {
  const prepared = sourceAdmissionPrepareOptions(options), { repoRoot, scope } = prepared;
  report(options.progress, "inventory", "setup", 0, 1, scope);
  if (options.workers !== undefined && (!Number.isSafeInteger(options.workers) || options.workers < 1)) throw new Error("workers must be a positive integer");
  const taxonomy = loadTaxonomy({ repoRoot, taxonomyPath: prepared.taxonomyPath });
  if (scope && isExcluded(scope, taxonomy)) throw new Error(`Inventory scope is opaque: ${scope}`);
  sourceAdmissionCheckCancellation(repoRoot, prepared.cancelFile, prepared.repositoryFences);
  report(options.progress, "inventory", "setup", 1, 1, scope);
  const activeExclusions: string[] = [];
  const collectedSourceAdmission = collectTaxonomySourceAdmission(options, taxonomy, prepared);
  const sourceAdmission = collectedSourceAdmission.inventory;
  const blockingAdmission = sourceAdmission.diagnostics.filter((row) => row.code !== "tracked-path-absent");
  if (blockingAdmission.length > 0) throw new Error(`Source admission rejected: ${blockingAdmission.map((row) => `${row.code}:${row.path}`).join(", ")}`);
  const repositoryBoundary = sourceAdmission.observations.find((row) => row.repositoryBoundary === "gitlink");
  if (repositoryBoundary) throw new Error(`Normalization requires an explicit repository-boundary decision before authored classification: ${repositoryBoundary.sourcePath}`);
  const admitted = new Map<string, CandidatePath>();
  for (const row of sourceAdmission.observations) {
    if (!["file", "directory", "symlink"].includes(row.observedKind)) continue;
    const stageZero = row.indexEntries.find((entry) => entry.stage === 0);
    admitted.set(row.sourcePath, { path: row.sourcePath, mode: row.worktreeMode ?? (row.observedKind === "directory" ? "040000" : row.observedKind === "symlink" ? "120000" : "100644"), objectId: stageZero?.objectId, explicitDirectory: row.explicitDirectory });
  }
  const directoryPaths = new Set<string>();
  for (const row of admitted.values()) {
    if (row.explicitDirectory || row.mode === "040000") directoryPaths.add(row.path);
    let parent = dirname(row.path);
    while (parent && parent !== ".") {
      if (inScope(parent, scope)) directoryPaths.add(parent);
      parent = dirname(parent);
    }
  }
  const entries = new Map<string, MutableInventoryEntry>();
  const canonicalDirectoryByPath = new Map<string, string>();
  const directoryKindByPath = new Map<string, string>();
  const fixedDirectoryContractByPath = new Map<string, string>();
  const directories = [...directoryPaths].sort((a, b) => a.split("/").length - b.split("/").length || Buffer.from(a).compare(Buffer.from(b)));
  report(options.progress, "inventory", "directories", 0, directories.length);
  for (let index = 0; index < directories.length; index++) {
    checkCancellation(repoRoot, options.cancelFile);
    const path = directories[index];
    const parentCanonical = canonicalDirectoryByPath.get(dirname(path)) ?? "";
    const parentContextId = directoryKindByPath.get(dirname(path)) ?? fixedDirectoryContractByPath.get(dirname(path));
    const canonical = canonicalDirectory(path, parentCanonical, parentContextId, ancestorDirectoryKindIds(path, directoryKindByPath), taxonomy);
    canonicalDirectoryByPath.set(path, canonical.path);
    if (canonical.kindId) directoryKindByPath.set(path, canonical.kindId);
    if (canonical.fixedId) fixedDirectoryContractByPath.set(path, canonical.fixedId);
    entries.set(path, {
      sourcePath: path,
      normalizedPath: canonical.path,
      nodeKind: "directory",
      ownerId: ownerId(path),
      areaId: areaId(path),
      fileKind: null,
      semanticStem: splitLeadingEmoji(basename(path)).rest || null,
      fixedContractId: canonical.fixedId,
      contentHash: "",
      referencesIn: [],
      referencesOut: [],
      violations: [...canonical.violations, ...pathPolicyViolations(canonical.path, taxonomy)],
      mode: (lstatOrNull(absolutePath(repoRoot, path))?.mode ?? 0) & 0o7777,
      size: 0,
    });
    report(options.progress, "inventory", "directories", index + 1, directories.length, path);
  }
  checkCancellation(repoRoot, options.cancelFile);
  const leaves = [...admitted.values()].filter((row) => row.mode !== "040000" && !row.explicitDirectory).sort((a, b) => Buffer.from(a.path).compare(Buffer.from(b.path)));
  report(options.progress, "inventory", "files", 0, leaves.length);
  const siblingFixedFilenameContractIdsByParent = new Map<string, readonly string[]>();
  const siblingIds = new Map<string, Set<string>>();
  for (const row of leaves) {
    const parent = dirname(row.path);
    const fixed = matchingFixedContracts(row.path, taxonomy.schema.fixedFilenameContracts, taxonomy, packageLocation(row.path, taxonomy), directoryKindByPath.get(parent), fixedDirectoryContractByPath.get(parent));
    if (!fixed.selected) continue;
    const ids = siblingIds.get(parent) ?? new Set<string>();
    ids.add(fixed.selected[0]);
    siblingIds.set(parent, ids);
  }
  for (const [parent, ids] of siblingIds) siblingFixedFilenameContractIdsByParent.set(parent, [...ids].sort(generatorPathCompare));
  for (let index = 0; index < leaves.length; index++) {
    checkCancellation(repoRoot, options.cancelFile);
    const row = leaves[index];
    const content = contentOf(repoRoot, row);
    const parent = dirname(row.path) === "." ? "" : dirname(row.path);
    const contentKind: ContentKindHint = content.kind === "file" ? extensionlessContentKind(row.path, content.bytes, taxonomy) : { kindId: null };
    const parentContextId = directoryKindByPath.get(parent) ?? fixedDirectoryContractByPath.get(parent);
    const canonical = canonicalFile(row.path, canonicalDirectoryByPath.get(parent) ?? "", parentContextId, ancestorDirectoryKindIds(row.path, directoryKindByPath), directoryKindByPath, fixedDirectoryContractByPath, siblingFixedFilenameContractIdsByParent, taxonomy, contentKind.kindId ?? undefined);
    const violations = [...canonical.violations];
    if (content.violation) violations.push(content.violation);
    if (contentKind.violation && !canonical.fixedId) violations.push(contentKind.violation);
    let text: string | null = null;
    if (content.kind === "file" && content.size <= 16 * 1024 * 1024 && (textualPath(row.path) || (contentKind.kindId !== null && contentKind.kindId !== "binary"))) {
      try {
        text = new TextDecoder("utf-8", { fatal: true }).decode(content.bytes);
      } catch {
        text = null;
      }
    }
    const role = classifyPackageRole(row.path, canonical.fileKind, canonical.fixedId, text, taxonomy);
    let normalizedPath = canonical.path;
    if (role === "implementation") {
      const extracted = packageImplementationDestination(row.path, canonical, canonicalDirectoryByPath, directoryKindByPath, taxonomy);
      if (extracted) {
        normalizedPath = extracted;
        violations.push(violation("package-implementation-file", row.path, `Package implementation must move to ${extracted}`, "warning"));
      } else violations.push(violation("package-implementation-destination-unresolved", row.path, "Package implementation has no deterministic semantic owner"));
    }
    if (role === "unresolved") violations.push(violation("package-role-unresolved", row.path, "Package role cannot be proven by the configured glue grammar"));
    violations.push(...pathPolicyViolations(normalizedPath, taxonomy));
    if (content.kind === "symlink") {
      try {
        const target = readlinkSync(absolutePath(repoRoot, row.path));
        if (isAbsolute(target)) violations.push(violation("symlink-absolute-target", row.path, "Absolute symlink target cannot be proven repository-local"));
        else {
          const lexicalTarget = normalizeRelative(posix.join(dirname(row.path), target.replaceAll("\\", "/")));
          if (isExcluded(lexicalTarget, taxonomy)) violations.push(violation("symlink-opaque-boundary", row.path, `Symlink lexically targets opaque path ${lexicalTarget}`));
        }
      } catch (error) {
        violations.push(violation("symlink-target-unreadable", row.path, error instanceof Error ? error.message : String(error)));
      }
    }
    entries.set(row.path, {
      sourcePath: row.path,
      normalizedPath,
      nodeKind: content.kind,
      ownerId: ownerId(row.path),
      areaId: areaId(row.path),
      fileKind: canonical.fileKind,
      semanticStem: canonical.stem,
      fixedContractId: canonical.fixedId,
      packageRole: role,
      contentHash: content.hash,
      referencesIn: [],
      referencesOut: [],
      violations,
      mode: content.mode,
      size: content.size,
      symlinkTarget: content.symlinkTarget,
    });
    report(options.progress, "inventory", "files", index + 1, leaves.length, row.path);
  }
  checkCancellation(repoRoot, options.cancelFile);
  projectExactOwnedFiles(repoRoot, entries, taxonomy);
  projectArtifactEmptyFacetFiles(repoRoot, entries, taxonomy);
  projectTicketDocumentFiles(repoRoot, entries, fixedDirectoryContractByPath, taxonomy);
  projectTicketImportantFiles(repoRoot, entries, fixedDirectoryContractByPath, taxonomy);
  validateTicketImportantDirectories(repoRoot, entries, directoryKindByPath, fixedDirectoryContractByPath);
  projectMutationTestBundles(repoRoot, scope, entries, directoryKindByPath, taxonomy);
  validateMutationPayloadSchemas(repoRoot, entries, taxonomy);
  validateProjectedMutationTestBundles(repoRoot, scope, entries, directoryKindByPath, taxonomy);
  projectArtifactCatalogs(repoRoot, entries, taxonomy);
  projectNestedCargoPackages(repoRoot, entries, taxonomy, prunableSourceParents);
  const childrenByParent = new Map<string, MutableInventoryEntry[]>();
  for (const entry of entries.values()) {
    const parent = dirname(entry.sourcePath);
    const children = childrenByParent.get(parent) ?? [];
    children.push(entry);
    childrenByParent.set(parent, children);
  }
  for (const path of [...directoryPaths].sort((a, b) => b.split("/").length - a.split("/").length || b.localeCompare(a))) {
    const entry = entries.get(path);
    if (entry) entry.contentHash = directoryHash(path, childrenByParent.get(path) ?? []);
  }
  referenceGraph(repoRoot, entries, taxonomy, options.progress, options.cancelFile);
  const frozenEntries: TaxonomyInventoryEntry[] = [...entries.values()]
    .sort((a, b) => Buffer.from(a.sourcePath).compare(Buffer.from(b.sourcePath)))
    .map((entry) => ({
      sourcePath: entry.sourcePath,
      normalizedPath: entry.normalizedPath,
      nodeKind: entry.nodeKind,
      ownerId: entry.ownerId,
      areaId: entry.areaId,
      fileKind: entry.fileKind,
      semanticStem: entry.semanticStem,
      fixedContractId: entry.fixedContractId,
      packageRole: entry.packageRole,
      contentHash: entry.contentHash,
      mode: entry.mode,
      size: entry.size,
      symlinkTarget: entry.symlinkTarget,
      referencesIn: [...entry.referencesIn],
      referencesOut: [...entry.referencesOut],
      violations: stableViolations(entry.violations),
    }));
  const violations = stableViolations(frozenEntries.flatMap((entry) => entry.violations));
  const sourceDigest = sourceTreeDigest(frozenEntries);
  const partial = {
    schemaVersion: 1 as const,
    taxonomySchemaVersion: 7 as const,
    scope,
    pathExclusions: taxonomy.exclusions.map((entry) => entry.path),
    activePathExclusions: activeExclusions,
    entries: frozenEntries,
    violations,
    sourceTreeDigest: sourceDigest,
  };
  const inventory: TaxonomyInventory = {
    ...partial,
    repoRoot,
    taxonomyPath: taxonomy.path,
    inventoryDigest: inventoryDigestOf(partial),
  };
  referenceInventoryContexts.set(inventory, {
    ticketDir: options.ticketDir,
    transactionRoots: [],
    exactEvidencePaths: [],
    sourceAdmission: Object.freeze({
      state: "captured" as const,
      retained: Object.freeze({
        originInventory: inventory,
        originalInputText: collectedSourceAdmission.inputText,
        sourceInventoryText: JSON.stringify(sourceAdmission),
        repositoryAuthority: new TransactionRepositoryAuthority(repoRoot, prepared.indexRows),
        originSourceTreeDigest: inventory.sourceTreeDigest,
        originInventoryDigest: inventory.inventoryDigest,
      }),
    }),
  });
  report(options.progress, "inventory", "complete", frozenEntries.length, frozenEntries.length);
  return inventory;
}
//#endregion 📋️Inventory API

//#region 🧠️Planning API
interface CollisionGroup {
  readonly id: string;
  readonly comparison: string;
  readonly paths: readonly string[];
  readonly sources: readonly string[];
}

function collisionKey(path: string, comparison: string): string {
  if (comparison === "byte" || comparison === "same-kind") return path;
  if (comparison === "nfc") return path.normalize("NFC");
  if (comparison === "case-fold") return path.normalize("NFC").toLocaleLowerCase("und");
  return emojiFold(path).toLocaleLowerCase("und");
}

function collisionGroups(entries: readonly TaxonomyInventoryEntry[], taxonomy: LoadedTaxonomy): readonly CollisionGroup[] {
  const groups: CollisionGroup[] = [];
  for (const comparison of taxonomy.schema.collisionPolicy.comparisons) {
    const buckets = new Map<string, TaxonomyInventoryEntry[]>();
    for (const entry of entries) {
      const key = comparison === "same-kind" ? `${entry.nodeKind}\u0000${entry.fileKind ?? "fixed"}\u0000${collisionKey(entry.normalizedPath, comparison)}` : collisionKey(entry.normalizedPath, comparison);
      const rows = buckets.get(key) ?? [];
      rows.push(entry);
      buckets.set(key, rows);
    }
    for (const [key, rows] of buckets) {
      if (rows.length < 2) continue;
      const sources = rows.map((entry) => entry.sourcePath).sort();
      groups.push({ id: sha256(`${comparison}\u0000${key}\u0000${sources.join("\u0000")}`).slice(0, 24), comparison, paths: [...new Set(rows.map((entry) => entry.normalizedPath))].sort(), sources });
    }
  }
  return groups.sort((a, b) => a.comparison.localeCompare(b.comparison) || a.id.localeCompare(b.id));
}

function generatorNodeRecord(repoRoot: string, path: string, taxonomy: LoadedTaxonomy): TaxonomyGeneratorNodeRecord {
  if (isExcluded(path, taxonomy)) throw new Error(`Generator node is opaque: ${path}`);
  const absolute = absolutePath(repoRoot, path);
  const stat = lstatSync(absolute);
  const nodeKind: TaxonomyNodeKind = stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : "file";
  const normalized = normalizeRelative(path), mode = stat.mode & 0o7777;
  if (nodeKind === "directory") return { path: normalized, nodeKind, contentHash: sha256("directory"), mode };
  if (nodeKind === "symlink") {
    const target = readlinkSync(absolute);
    return { path: normalized, nodeKind, contentHash: sha256(target), mode, size: Buffer.byteLength(target), target };
  }
  const bytes = readFileSync(absolute);
  return { path: normalized, nodeKind, contentHash: sha256(bytes), mode, size: bytes.byteLength };
}

function generatorTreeInventory(repoRoot: string, roots: readonly string[], taxonomy: LoadedTaxonomy): readonly TaxonomyGeneratorNodeRecord[] {
  const rows = new Map<string, TaxonomyGeneratorNodeRecord>();
  const walk = (path: string): void => {
    if (isExcluded(path, taxonomy)) throw new Error(`Generator output root is opaque: ${path}`);
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat) return;
    rows.set(path, generatorNodeRecord(repoRoot, path, taxonomy));
    if (!stat.isDirectory() || stat.isSymbolicLink()) return;
    for (const child of readdirSync(absolute).sort((a, b) => Buffer.from(a).compare(Buffer.from(b)))) walk(sourceRelative(`${path}/${child}`));
  };
  for (const root of [...new Set(roots.map(normalizeRelative))].sort(generatorPathCompare)) walk(root);
  return [...rows.values()].sort((left, right) => generatorPathCompare(left.path, right.path));
}

/** 📇️ Selects schema-owned generator inputs through physical or logical preimage membership. */
function compilerInputManifestRows(value: unknown, authority: NonNullable<GeneratorContractSpec["compilerInputManifest"]>, label: string): readonly Readonly<{ path: string; bytes: number; sha256: string }>[] {
  const manifest = record(value, label);
  if (Object.keys(manifest).sort().join("|") !== "contractId|inputs|layoutSha256|outputs|version" || manifest.version !== 1 || typeof manifest.contractId !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(manifest.contractId) || typeof manifest.layoutSha256 !== "string" || !/^[a-f0-9]{64}$/u.test(manifest.layoutSha256) || !Array.isArray(manifest.outputs) || !Array.isArray(manifest.inputs) || manifest.inputs.length < 1 || manifest.inputs.length > authority.maxFiles) throw new Error(`${label} is not one bounded compiler input manifest`);
  const seen = new Set<string>();
  const rows = manifest.inputs.map((value, index) => {
    const row = record(value, `${label}.inputs[${index}]`);
    if (Object.keys(row).sort().join("|") !== "bytes|path|sha256" || typeof row.path !== "string" || row.path !== normalizeRelative(row.path) || !Number.isSafeInteger(row.bytes) || row.bytes < 0 || typeof row.sha256 !== "string" || !/^[a-f0-9]{64}$/u.test(row.sha256) || seen.has(row.path)) throw new Error(`${label} contains an invalid compiler input witness`);
    seen.add(row.path);
    return { path: row.path, bytes: row.bytes as number, sha256: row.sha256 };
  });
  if (rows.some((row, index) => index > 0 && generatorPathCompare(rows[index - 1]!.path, row.path) > 0)) throw new Error(`${label} compiler inputs are not path-sorted`);
  return rows;
}

function compilerInputRecords(repoRoot: string, contract: GeneratorContractSpec, taxonomy: LoadedTaxonomy, manifest: unknown, label: string): readonly TaxonomyGeneratorNodeRecord[] {
  const authority = contract.compilerInputManifest;
  if (!authority) return [];
  return compilerInputManifestRows(manifest, authority, label).map(row => {
    if (isExcluded(row.path, taxonomy) || contract.outputRoots.some(root => row.path === root.path || row.path.startsWith(`${root.path}/`))) throw new Error(`${label} owns an opaque or self-produced compiler input: ${row.path}`);
    const node = generatorNodeRecord(repoRoot, row.path, taxonomy);
    if (node.nodeKind !== "file" || node.size !== row.bytes || node.contentHash !== row.sha256) throw new Error(`${label} compiler input bytes differ: ${row.path}`);
    return node;
  });
}

function compilerPreviewInputRecords(repoRoot: string, contract: GeneratorContractSpec, taxonomy: LoadedTaxonomy, preview: TaxonomyGeneratorPreviewManifest): readonly TaxonomyGeneratorNodeRecord[] {
  const authority = contract.compilerInputManifest;
  if (!authority) return [];
  const node = preview.nodes.find(node => node.path === authority.manifestOutputPath);
  if (!node || node.nodeKind !== "file") throw new Error(`Compiler preview lacks its declared input manifest: ${authority.manifestOutputPath}`);
  return compilerInputRecords(repoRoot, contract, taxonomy, JSON.parse(Buffer.from(node.bytesBase64, "base64").toString("utf8")), "Compiler preview manifest");
}

export function generatorInputPaths(inventory: Pick<TaxonomyInventory, "repoRoot">, contract: GeneratorContractSpec, taxonomy: LoadedTaxonomy, cancelFile?: string, view?: RegistryCatalogInputView): readonly string[] {
  const paths = new Set<string>();
  const inputView = view ?? registryCatalogInputView(inventory.repoRoot, taxonomy.discoverySchema);
  if (contract.inputDiscovery) {
    const catalogTaxonomy = taxonomy.discoverySchema;
    const catalogView = inputView;
    const cancellableView: RegistryCatalogInputView = {
      entries(path) { checkCancellation(inventory.repoRoot, cancelFile); return catalogView.entries(path); },
      kind(path) { checkCancellation(inventory.repoRoot, cancelFile); return catalogView.kind(path); },
      readText(path) { checkCancellation(inventory.repoRoot, cancelFile); return catalogView.readText(path); },
    };
    for (const path of registryCatalogInputPaths(inventory.repoRoot, catalogTaxonomy, cancellableView)) paths.add(path);
  }
  const candidatesByPrefix = new Map<string, readonly string[]>();
  for (const pattern of contract.inputPatterns) {
    const segments = pattern.split("/");
    const wildcard = segments.findIndex((segment) => /[*?[]/u.test(segment));
    const prefix = (wildcard < 0 ? segments : segments.slice(0, wildcard)).join("/");
    if (!prefix || isExcluded(prefix, taxonomy)) throw new Error("Generator input requires a nonopaque literal owner prefix: " + pattern);
    if (wildcard < 0) { paths.add(prefix); continue; }
    let candidates = candidatesByPrefix.get(prefix);
    if (!candidates) {
      const nodes = new Set<string>();
      const visit = (path: string): void => {
        checkCancellation(inventory.repoRoot, cancelFile);
        if (isExcluded(path, taxonomy)) return;
        const kind = inputView.kind(path);
        if (kind === null) return;
        if (kind === "symlink") throw new Error(`Generator input prefix is a symlink: ${path}`);
        nodes.add(path);
        if (kind === "directory") for (const entry of inputView.entries(path)) {
          const child = `${path}/${entry.name}`;
          if (isExcluded(child, taxonomy)) continue;
          if (entry.nodeKind === "directory") visit(child);
          else nodes.add(child);
        }
      };
      visit(prefix);
      candidates = [...nodes];
      candidatesByPrefix.set(prefix, candidates);
    }
    for (const candidate of candidates) {
      checkCancellation(inventory.repoRoot, cancelFile);
      if (taxonomy.pathMatcher.matches(candidate, pattern)) paths.add(candidate);
    }
  }
  if (contract.compilerInputManifest) {
    const manifestPath = absolutePath(inventory.repoRoot, contract.compilerInputManifest.manifestOutputPath), node = lstatOrNull(manifestPath);
    if (node?.isSymbolicLink()) throw new Error(`Compiler input manifest is a symlink: ${contract.compilerInputManifest.manifestOutputPath}`);
    if (node?.isFile()) for (const row of compilerInputRecords(inventory.repoRoot, contract, taxonomy, JSON.parse(readFileSync(manifestPath, "utf8")), "Current compiler input manifest")) paths.add(row.path);
    else if (node !== null) throw new Error(`Compiler input manifest is not a regular file: ${contract.compilerInputManifest.manifestOutputPath}`);
  }
  return [...paths].filter((path) => !isExcluded(path, taxonomy) && !contract.outputRoots.some((output) => path === output.path || path.startsWith(output.path + "/"))).sort(generatorPathCompare).filter((path) => {
    checkCancellation(inventory.repoRoot, cancelFile);
    const kind = inputView.kind(path);
    if (kind === "symlink") throw new Error(`Generator input is a symlink: ${path}`);
    return kind !== null;
  });
}

function generatorInputInventory(inventory: TaxonomyInventory, contract: GeneratorContractSpec, taxonomy: LoadedTaxonomy, cancelFile?: string): readonly TaxonomyGeneratorNodeRecord[] {
  return generatorInputPaths(inventory, contract, taxonomy, cancelFile).map((path) => generatorNodeRecord(inventory.repoRoot, path, taxonomy));
}
function previewNodeRecords(manifest: TaxonomyGeneratorPreviewManifest): readonly TaxonomyGeneratorNodeRecord[] {
  return manifest.nodes.map((node) => {
    if (node.nodeKind === "directory") return { path: node.path, nodeKind: "directory" as const, contentHash: sha256("directory"), mode: node.mode };
    const bytes = Buffer.from(node.bytesBase64, "base64");
    return { path: node.path, nodeKind: "file" as const, contentHash: sha256(bytes), mode: node.mode, size: bytes.byteLength };
  });
}

function validatePreviewPreState(manifest: TaxonomyGeneratorPreviewManifest, preOutputs: readonly TaxonomyGeneratorNodeRecord[]): void {
  const expected = new Set(manifest.nodes.map((node) => node.path));
  const prePaths = new Set(preOutputs.map((node) => node.path));
  for (const stale of manifest.staleRemovals) if (![...prePaths].some((path) => path === stale || path.startsWith(`${stale}/`))) throw new Error(`Generator preview stale removal does not exist in the output pre-state: ${stale}`);
  for (const path of prePaths) if (!expected.has(path) && !manifest.staleRemovals.some((stale) => path === stale || path.startsWith(`${stale}/`))) throw new Error(`Generator preview omits stale output from staleRemovals: ${path}`);
}

function generatorPreviewProjection(contractId: GeneratorInputProjection["contractId"], inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], edits: readonly ReferenceEdit[], removals: readonly TaxonomyEvidenceRemoval[], taxonomy: LoadedTaxonomy, cancelFile?: string): GeneratorInputProjection {
  const projectedEdits = [...new Set(edits.map((edit) => edit.path))].sort(generatorPathCompare).map((path) => {
    checkCancellation(inventory.repoRoot, cancelFile);
    const sourcePath = moves.find((move) => move.destinationPath === path)?.sourcePath ?? inventory.entries.find((entry) => entry.normalizedPath === path)?.sourcePath ?? path;
    const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, sourcePath, "Registry projected edit", true);
    const bytes = applyEditsToContent(readFileSync(absolute, "utf8"), edits.filter((edit) => edit.path === path));
    return { path, bytesBase64: Buffer.from(bytes).toString("base64") };
  });
  return parseGeneratorInputProjection(canonicalJson({ contractId, schemaVersion: 1, moves: moves.map((move) => ({ sourcePath: move.sourcePath, destinationPath: move.destinationPath, nodeKind: move.sourcePreimage.nodeKind })), edits: projectedEdits, removals: removals.map((removal) => removal.sourcePath) }), taxonomy.discoverySchema, contractId);
}

function invokeGeneratorPreview(inventory: TaxonomyInventory, id: string, contract: GeneratorContractSpec, taxonomy: LoadedTaxonomy, projection?: GeneratorInputProjection, cancelFile?: string): { readonly manifest: TaxonomyGeneratorPreviewManifest; readonly digest: string } {
  if (!contract.ownerPath || !contract.previewTarget) throw new Error(`Owned generator ${id} has no preview target`);
  assertGeneratorPreviewTarget(inventory.repoRoot, contract);
  checkCancellation(inventory.repoRoot, cancelFile);
  const protocol = contract.inputDiscovery?.previewInput ?? contract.packageGeneration?.previewInput;
  const input = projection ? canonicalJson(projection) + "\n" : undefined;
  if (input && (!protocol || Buffer.byteLength(input) > protocol.maxBytes)) throw new Error(`Generator ${id} projected input exceeds its declared byte limit`);
  const cancellationPath = cancelFile ? assertLexicalInputOutsideOpaque(inventory.repoRoot, cancelFile, "Generator preview cancellation", true) : "";
  const limits = generatorPreviewResourceLimits(contract);
  const result = spawnSync("bun", ["./📜️script.ts", ...generatorPreviewScriptArguments(contract)], { cwd: absolutePath(inventory.repoRoot, contract.ownerPath), encoding: "utf8", input, maxBuffer: limits.maxOutputBytes, timeout: limits.timeoutMs, env: { ...process.env, REPO_ROOT: inventory.repoRoot, SEMIO_GENERATOR_PREVIEW: "1", SEMIO_GENERATOR_PREVIEW_PROTOCOL: projection ? protocol!.protocol : "", SEMIO_GENERATOR_PREVIEW_CANCEL_FILE: cancellationPath } });
  checkCancellation(inventory.repoRoot, cancelFile);
  const stdout = result.stdout ?? "", stderr = result.stderr ?? "";
  if (result.error || result.status !== 0 || result.signal !== null || stderr !== "") throw new Error(`Generator preview command failed for ${id}: status=${result.status ?? -1}, stdout=${sha256(stdout)}, stderr=${sha256(stderr)}`);
  const roots = contract.outputRoots.map((root) => root.path).sort(generatorPathCompare);
  const manifest = parseGeneratorPreviewManifest(stdout, id, roots, taxonomy.exclusions.map((entry) => entry.path));
  return { manifest, digest: sha256(stdout) };
}

interface GeneratorPlanningResult {
  readonly regenerations: readonly TaxonomyRegeneration[];
  readonly violations: readonly TaxonomyViolation[];
}

function projectedPath(path: string, entries: readonly TaxonomyInventoryEntry[]): string {
  const mappings = entries.filter((entry) => entry.sourcePath !== entry.normalizedPath && (path === entry.sourcePath || path.startsWith(`${entry.sourcePath}/`))).sort((left, right) => right.sourcePath.length - left.sourcePath.length || generatorPathCompare(left.sourcePath, right.sourcePath));
  if (mappings.length === 0) return path;
  const longest = mappings[0].sourcePath.length;
  const destinations = new Set(mappings.filter((entry) => entry.sourcePath.length === longest).map((entry) => `${entry.normalizedPath}${path.slice(entry.sourcePath.length)}`));
  if (destinations.size !== 1) throw new Error(`Path projection is ambiguous for ${path}`);
  return [...destinations][0];
}

/** 🧷️ Resolves a POSIX, drive-rooted, or UNC absolute target only when it is lexically inside the same repository root. */
export function repositoryLocalSymlinkTargetPath(repoRoot: string, target: string): string | null {
  if (repoRoot.includes("\u0000") || target.includes("\u0000")) return null;
  const slash = (value: string): string => value.replaceAll("\\", "/").replace(/\/+$/u, "");
  const root = slash(repoRoot);
  const candidate = slash(target);
  const drive = /^([A-Za-z]):\/(.*)$/u;
  const rootDrive = drive.exec(root);
  const targetDrive = drive.exec(candidate);
  const unc = /^\/\/([^/]+)\/([^/]+)(?:\/(.*))?$/u;
  const rootUnc = unc.exec(root);
  const targetUnc = unc.exec(candidate);
  let suffix: string;
  if (rootDrive || targetDrive) {
    if (!rootDrive || !targetDrive || rootDrive[1].toLowerCase() !== targetDrive[1].toLowerCase()) return null;
    const rootTail = rootDrive[2].replace(/\/+$/u, "");
    const targetTail = targetDrive[2];
    if (targetTail.toLowerCase() !== rootTail.toLowerCase() && !targetTail.toLowerCase().startsWith(`${rootTail.toLowerCase()}/`)) return null;
    suffix = targetTail.slice(rootTail.length).replace(/^\//u, "");
  } else if (rootUnc || targetUnc) {
    if (!rootUnc || !targetUnc || rootUnc[1].toLowerCase() !== targetUnc[1].toLowerCase() || rootUnc[2].toLowerCase() !== targetUnc[2].toLowerCase()) return null;
    const rootTail = (rootUnc[3] ?? "").replace(/\/+$/u, "");
    const targetTail = targetUnc[3] ?? "";
    if (targetTail.toLowerCase() !== rootTail.toLowerCase() && !targetTail.toLowerCase().startsWith(`${rootTail.toLowerCase()}/`)) return null;
    suffix = targetTail.slice(rootTail.length).replace(/^\//u, "");
  } else {
    if (!root.startsWith("/") || !candidate.startsWith("/") || (candidate !== root && !candidate.startsWith(`${root}/`))) return null;
    suffix = candidate.slice(root.length).replace(/^\//u, "");
  }
  if (!suffix || suffix.split("/").some((segment) => segment === "" || segment === "." || segment === "..")) return null;
  try { return normalizeRelative(suffix); } catch { return null; }
}

function logicalRepositorySymlinkTargetPath(repoRoot: string, sourcePath: string, target: string): string | null {
  const absoluteSyntax = target.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(target) || /^(?:\\\\|\/\/)/u.test(target);
  if (absoluteSyntax) return repositoryLocalSymlinkTargetPath(repoRoot, target);
  try { return normalizeRelative(posix.join(posix.dirname(sourcePath), target.replaceAll("\\", "/"))); } catch { return null; }
}

function planSymlinkTargetEdits(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy, options: TaxonomyPlanOptions): { readonly edits: TaxonomySymlinkTargetEdit[]; readonly violations: TaxonomyViolation[] } {
  const edits: TaxonomySymlinkTargetEdit[] = [];
  const violations: TaxonomyViolation[] = [];
  const bySource = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  const links = [...inventory.entries, ...incomingReferenceSnapshot(inventory, taxonomy, options).entries].filter((candidate) => candidate.nodeKind === "symlink" && candidate.symlinkTarget !== undefined);
  for (const entry of links) {
    const oldTarget = entry.symlinkTarget!;
    const logicalTargetSourcePath = logicalRepositorySymlinkTargetPath(inventory.repoRoot, entry.sourcePath, oldTarget);
    if (logicalTargetSourcePath === null || isExcluded(logicalTargetSourcePath, taxonomy)) {
      violations.push(violation("symlink-target-authority-invalid", entry.sourcePath, "Symlink target is external, escaping, or opaque"));
      continue;
    }
    const finalPath = projectedPath(entry.sourcePath, inventory.entries);
    const logicalTargetFinalPath = projectedPath(logicalTargetSourcePath, inventory.entries);
    const targetEntry = bySource.get(logicalTargetSourcePath) ?? referenceEntry(inventory.repoRoot, logicalTargetSourcePath, taxonomy);
    if (targetEntry?.nodeKind === "directory") {
      violations.push(violation("symlink-target-directory-authority-unresolved", entry.sourcePath, `Directory target requires a recursive no-follow authority: ${logicalTargetSourcePath}`));
      continue;
    }
    const logicalTargetPreimage: TaxonomyPathPreimage = !targetEntry ? { state: "absent" } : targetEntry.nodeKind === "directory" ? { state: "directory" } : targetEntry.nodeKind === "symlink" ? { state: "symlink", contentHash: targetEntry.contentHash, mode: targetEntry.mode, size: targetEntry.size, target: targetEntry.symlinkTarget! } : { state: "file", contentHash: targetEntry.contentHash, mode: targetEntry.mode, size: targetEntry.size };
    const extension = resolveFileKind(logicalTargetSourcePath, taxonomy, [], []).kind;
    if (!targetEntry && !extension) {
      violations.push(violation("symlink-target-kind-unresolved", entry.sourcePath, `Broken target kind cannot be proven: ${logicalTargetSourcePath}`));
      continue;
    }
    const newTarget = posix.relative(posix.dirname(finalPath), logicalTargetFinalPath);
    if (!newTarget || newTarget.startsWith("/") || isExcluded(posix.normalize(posix.join(posix.dirname(finalPath), newTarget)), taxonomy)) {
      violations.push(violation("symlink-target-render-invalid", entry.sourcePath, "Relative target rendering is empty, absolute, or opaque"));
      continue;
    }
    if (newTarget === oldTarget) continue;
    const targetDigestible = { sourcePath: entry.sourcePath, finalPath, oldTarget, newTarget, logicalTargetSourcePath, logicalTargetFinalPath, logicalTargetPreimage };
    const provisional = { sourcePath: entry.sourcePath, finalPath, oldTarget, newTarget, oldTargetHash: sha256(oldTarget), newTargetHash: sha256(newTarget), logicalTargetSourcePath, logicalTargetFinalPath, logicalTargetPreimage, windowsLinkType: (targetEntry?.nodeKind === "directory" ? "dir" : "file") as "file" | "dir", sourceTargetDigest: sha256(canonicalJson(targetDigestible)), rationaleRule: "repository-local-symlink-target-v2" as const, ownerId: entry.ownerId };
    edits.push({ operationId: dispositionOperationId("symlink-target-edit", provisional), ...provisional });
  }
  return { edits: edits.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), violations: stableViolations(violations) };
}

interface EmbeddedDispositionPlanning {
  readonly roots: readonly TaxonomyEmbeddedTicketRootDisposition[];
  readonly relocations: readonly TaxonomyEmbeddedTicketRootRelocation[];
  readonly removals: readonly TaxonomyEvidenceRemoval[];
  readonly violations: readonly TaxonomyViolation[];
}

function incomingEmbeddedReferences(inventory: TaxonomyInventory, root: string): readonly string[] {
  const rows = new Set<string>();
  for (const entry of inventory.entries.filter((candidate) => candidate.sourcePath === root || candidate.sourcePath.startsWith(`${root}/`))) for (const source of entry.referencesIn) if (source !== root && !source.startsWith(`${root}/`)) rows.add(`text\u0000${source}\u0000${entry.sourcePath}`);
  for (const link of inventory.entries.filter((candidate) => candidate.nodeKind === "symlink" && candidate.symlinkTarget !== undefined && candidate.sourcePath !== root && !candidate.sourcePath.startsWith(`${root}/`))) {
    let target: string | null = null;
    if (link.symlinkTarget!.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(link.symlinkTarget!) || /^(?:\\\\|\/\/)/u.test(link.symlinkTarget!)) target = repositoryLocalSymlinkTargetPath(inventory.repoRoot, link.symlinkTarget!);
    else try { target = normalizeRelative(posix.join(posix.dirname(link.sourcePath), link.symlinkTarget!.replaceAll("\\", "/"))); } catch { target = null; }
    if (target && (target === root || target.startsWith(`${root}/`))) rows.add(`symlink\u0000${link.sourcePath}\u0000${target}`);
  }
  return [...rows].sort(generatorPathCompare);
}

interface JsonStringCoordinate { readonly pointer: string; readonly start: number; readonly end: number; readonly value: string; readonly rawValue?: string }

function jsonStringCoordinates(content: string, includeEscaped = false): readonly JsonStringCoordinate[] {
  try { JSON.parse(content); } catch { return []; }
  const lexemes = [...content.matchAll(/"(?:\\.|[^"\\])*"|[{}\[\]:,]|true|false|null|-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/gu)];
  const rows: JsonStringCoordinate[] = [];
  let cursor = 0;
  const require = (value: string): void => { if (lexemes[cursor++]?.[0] !== value) throw new Error("Invalid JSON coordinate syntax"); };
  const visit = (pointer: string): void => {
    const token = lexemes[cursor++];
    if (!token) throw new Error("Missing JSON coordinate value");
    if (token[0] === "{") {
      const keys = new Set<string>();
      while (lexemes[cursor]?.[0] !== "}") {
        const key = JSON.parse(lexemes[cursor++]![0]) as string;
        if (typeof key !== "string" || keys.has(key)) throw new Error("Duplicate JSON coordinate key");
        keys.add(key);
        require(":");
        visit(pointer + "/" + key.replaceAll("~", "~0").replaceAll("/", "~1"));
        if (lexemes[cursor]?.[0] === "}") break;
        require(",");
      }
      require("}");
    } else if (token[0] === "[") {
      let index = 0;
      while (lexemes[cursor]?.[0] !== "]") {
        visit(pointer + "/" + index++);
        if (lexemes[cursor]?.[0] === "]") break;
        require(",");
      }
      require("]");
    } else if (token[0].startsWith('"')) {
      const value = JSON.parse(token[0]) as string;
      const rawValue = token[0].slice(1, -1);
      if (rawValue === value || includeEscaped) rows.push({ pointer, start: token.index! + 1, end: token.index! + token[0].length - 1, value, ...(rawValue === value ? {} : { rawValue }) });
    }
  };
  try { visit(""); return cursor === lexemes.length ? rows : []; } catch { return []; }
}

/** 🧷️ An exact JSON coordinate span, optionally derived from one declared fixed identity prefix. */
export interface FrozenEvidenceCoordinate extends JsonStringCoordinate { readonly kind: "source" | "destination" }

/** 🔒️ Resolves only explicitly supplied evidence contracts; it performs no filesystem access. */
export function frozenCoordinateEvidenceCoordinates(path: string, bytes: Uint8Array, contracts: Readonly<Record<string, FrozenCoordinateEvidenceContract>>): readonly FrozenEvidenceCoordinate[] | null {
  const fail = (message: string): never => { throw new Error(`frozen-coordinate-evidence-invalid: ${path}: ${message}`); };
  const problems = validateFrozenCoordinateEvidenceContracts(contracts);
  if (problems.length) fail(problems.join("; "));
  const contract = Object.values(contracts).find((row) => row.path === path);
  if (!contract) return null;
  if (sha256(bytes) !== contract.sha256) fail("document digest does not match registered bytes");
  const content = Buffer.from(bytes).toString("utf8");
  if (!Buffer.from(content, "utf8").equals(bytes)) fail("document is not exact UTF-8");
  let document: unknown;
  try { document = JSON.parse(content); } catch { fail("document is not valid JSON"); }
  if (document === null || typeof document !== "object" || Array.isArray(document) !== (contract.rootKind === "array")) fail("document differs from its exact JSON root authority");
  if (contract.schemaVersion === null ? Object.hasOwn(document!, "schemaVersion") : !Object.hasOwn(document!, "schemaVersion") || (document as Record<string, unknown>).schemaVersion !== contract.schemaVersion) fail("document schemaVersion differs from registered presence/value authority");
  const spans = new Map(jsonStringCoordinates(content, true).map((row) => [row.pointer, row])), rows: FrozenEvidenceCoordinate[] = [], seen = new Set<string>();
  for (const declaration of contract.coordinates) {
    const segments = declaration.pointer.slice(1).split("/").map((segment) => segment.replaceAll("~1", "/").replaceAll("~0", "~"));
    const before = rows.length;
    const visit = (value: unknown, index: number, pointer: string): void => {
      if (index === segments.length) {
        if (typeof value !== "string" || !value || /[\u0000-\u001f]/u.test(value)) fail(`coordinate ${pointer} must be a physical repository-relative path string`);
        const ownerIdentity = "representation" in declaration && declaration.representation === "recorded-package-owner-identity";
        const escapedSource = "representation" in declaration && declaration.representation === "json-escaped-source-path";
        const prefix = "representation" in declaration ? declaration.representation === "recorded-package-owner-identity" ? declaration.identityPrefix : declaration.representation === "recorded-repository-absolute" ? declaration.recordedRepositoryRoot + "/" : "" : "";
        if (prefix && !(value as string).startsWith(prefix)) fail(`coordinate ${pointer} does not match its exact recorded prefix`);
        const relativeValue = prefix ? (value as string).slice(prefix.length) : value as string;
        if (!relativeValue || /^[A-Za-z]:/u.test(relativeValue) || ownerIdentity && relativeValue.includes(":")) fail(`coordinate ${pointer} must have a nonempty repository-relative suffix`);
        if (escapedSource && (Buffer.from(relativeValue, "utf8").toString("utf8") !== relativeValue || /[\\:*?"<>|\u007f]/u.test(relativeValue) || LEXICAL_OPAQUE_ROOTS.some((root) => relativeValue === root || relativeValue.startsWith(root + "/")))) fail(`coordinate ${pointer} must decode once to an exact non-opaque physical path`);
        try { if (normalizeRelative(relativeValue) !== relativeValue) fail(`coordinate ${pointer} is not repository-relative`); } catch { fail(`coordinate ${pointer} is not repository-relative`); }
        const span = spans.get(pointer);
        if (!span || span.value !== value || escapedSource !== (span.rawValue !== undefined)) fail(`coordinate ${pointer} has no exact declared JSON value encoding`);
        if (seen.has(pointer)) fail(`coordinate selectors overlap at ${pointer}`);
        seen.add(pointer);
        rows.push({ pointer, start: span!.start + (ownerIdentity ? prefix.length : 0), end: span!.end, value: escapedSource ? span!.rawValue! : ownerIdentity ? relativeValue : span!.value, kind: declaration.kind });
        return;
      }
      const segment = segments[index];
      if (segment === "*") {
        if (!Array.isArray(value)) fail(`wildcard ${declaration.pointer} must select an array`);
        (value as unknown[]).forEach((child, childIndex) => visit(child, index + 1, `${pointer}/${childIndex}`));
      } else {
        if (value === null || typeof value !== "object" || !Object.hasOwn(value, segment) || Array.isArray(value) && !/^(?:0|[1-9][0-9]*)$/u.test(segment)) fail(`coordinate pointer ${declaration.pointer} is missing`);
        visit((value as Record<string, unknown>)[segment], index + 1, `${pointer}/${segment.replaceAll("~", "~0").replaceAll("/", "~1")}`);
      }
    };
    visit(document, 0, "");
    if (rows.length === before) fail(`coordinate pointer ${declaration.pointer} matched no value`);
  }
  return rows.sort((left, right) => left.start - right.start);
}

/** 📝️ Finds only plain single-backtick or path-only-list coordinates outside opaque Markdown blocks (fenced/indented code, blockquotes, HTML). ATX headings get ordinary inline backtick-span recognition too (CommonMark parses their inline content the same as any other leaf block; only block-level constructs are opaque here). */
function markdownSourceCoordinateSpans(content: string): readonly Readonly<{ start: number; end: number; form: "inline-code" | "path-list-item" }>[] {
  const rows: { start: number; end: number; form: "inline-code" | "path-list-item" }[] = [];
  let fence = "", html = "", inline = 0;
  for (const match of content.matchAll(/[^\r\n]*(?:\r\n|\r|\n|$)/gu)) {
    const line = match[0].replace(/(?:\r\n|\r|\n)$/u, ""), offset = match.index!;
    if (!line.trim()) { inline = 0; if (html === "block") html = ""; continue; }
    const prefix = line.replace(/^ {0,3}(?:[-+*][ \t]+|\d+[.)][ \t]+)/u, ""), marker = prefix.match(/^ {0,3}(`{3,}|~{3,})(.*)$/u);
    if (fence) { if (marker && marker[1][0] === fence[0] && marker[1].length >= fence.length && !marker[2].trim()) fence = ""; continue; }
    if (marker && !(marker[1][0] === "`" && marker[2].includes("`"))) { fence = marker[1]; inline = 0; continue; }
    if (html) { if (html === "comment" && line.includes("-->")) html = ""; continue; }
    if (line.includes("<!--")) { if (!line.slice(line.indexOf("<!--") + 4).includes("-->")) html = "comment"; inline = 0; continue; }
    if (/^ {0,3}</u.test(line)) { html = "block"; inline = 0; continue; }
    if (/^(?: {4}| *\t| {0,3}>)/u.test(line)) { inline = 0; continue; }
    const first = rows.length;
    let visible = line;
    const list = line.match(/^ {0,3}[-+*][ \t]+([^\s]+)[ \t]*$/u);
    if (list && !inline) { const start = offset + line.indexOf(list[1]); rows.push({ start, end: start + list[1].length, form: "path-list-item" }); }
    const runs = [...line.matchAll(/`+/gu)];
    for (let index = 0; index < runs.length; index++) {
      const run = runs[index], start = run.index!;
      if (inline) { if (run[0].length === inline) { inline = 0; visible = " ".repeat(start + run[0].length) + visible.slice(start + run[0].length); } continue; }
      if ((line.slice(0, start).match(/\\+$/u)?.[0].length ?? 0) % 2) continue;
      const close = runs.findIndex((candidate, candidateIndex) => candidateIndex > index && candidate[0].length === run[0].length);
      if (close < 0) { inline = run[0].length; continue; }
      const end = runs[close].index! + runs[close][0].length;
      visible = visible.slice(0, start) + " ".repeat(end - start) + visible.slice(end);
      if (run[0].length === 1) rows.push({ start: offset + start + 1, end: offset + runs[close].index!, form: "inline-code" });
      index = close;
    }
    if (/[\[\]<>]/u.test(visible)) rows.splice(first);
  }
  return rows;
}

/** 🔏️ Resolves declared historical Markdown source spans without filesystem access or prose rewriting. */
export function frozenMarkdownCoordinateEvidenceCoordinates(path: string, bytes: Uint8Array, contracts: Readonly<Record<string, FrozenMarkdownCoordinateEvidenceContract>>): readonly FrozenEvidenceCoordinate[] | null {
  const fail = (message: string): never => { throw new Error(`frozen-coordinate-evidence-invalid: ${path}: ${message}`); };
  const problems = validateFrozenMarkdownCoordinateEvidenceContracts(contracts);
  if (problems.length) fail(problems.join("; "));
  const contract = Object.values(contracts).find((row) => row.path === path);
  if (!contract) return null;
  if (sha256(bytes) !== contract.sha256) fail("document digest does not match registered bytes");
  const content = Buffer.from(bytes).toString("utf8");
  if (!Buffer.from(content, "utf8").equals(bytes)) fail("document is not exact UTF-8");
  const spans = new Set(markdownSourceCoordinateSpans(content).map((row) => `${row.form}\0${row.start}\0${row.end}`));
  return contract.coordinates.map((coordinate): FrozenEvidenceCoordinate => {
    const value = content.slice(coordinate.start, coordinate.end);
    if (!spans.has(`${coordinate.form}\0${coordinate.start}\0${coordinate.end}`)) fail("coordinate has no exact admitted Markdown source span");
    if (!value || /[\\:*?"<>|`\u0000-\u0020]/u.test(value) || /^(?:compose|temp\/compose)(?:\/|$)/u.test(value) || value.split("/").some((part) => !part || part === "." || part === "..")) fail("coordinate is not one non-opaque repository-relative source path");
    if (sha256(value) !== coordinate.valueSha256) fail("coordinate value digest differs from its exact source authority");
    return { pointer: `markdown:${coordinate.form}@${coordinate.start}`, start: coordinate.start, end: coordinate.end, value, kind: "source" };
  }).sort((left, right) => left.start - right.start);
}

const frozenEvidenceContractIndexes = new WeakMap<LoadedTaxonomy, ReadonlyMap<string, Readonly<{ path: string }>>>();
const frozenEvidenceCoordinateCache = new WeakMap<Uint8Array, { readonly path: string; readonly taxonomy: LoadedTaxonomy; readonly coordinates: ReadonlySet<string> }>();

function frozenEvidenceContractIndex(taxonomy: LoadedTaxonomy): ReadonlyMap<string, Readonly<{ path: string }>> {
  const cached = frozenEvidenceContractIndexes.get(taxonomy);
  if (cached) return cached;
  const index = new Map([...Object.values(taxonomy.discoverySchema.frozenCoordinateEvidenceContracts), ...Object.values(taxonomy.discoverySchema.frozenMarkdownCoordinateEvidenceContracts)].map((contract) => [contract.path, contract]));
  frozenEvidenceContractIndexes.set(taxonomy, index);
  return index;
}

function validateObservedFrozenEvidenceNodes(repoRoot: string, knownPaths: ReadonlySet<string>, taxonomy: LoadedTaxonomy): void {
  for (const path of frozenEvidenceContractIndex(taxonomy).keys()) {
    if (!knownPaths.has(path)) continue;
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Frozen coordinate evidence"), stat = lstatOrNull(absolute);
    if (!stat?.isFile()) throw new Error(`frozen-coordinate-evidence-invalid: ${path}: observed document must remain a no-follow regular file`);
  }
}

function frozenEvidenceCoordinateAuthority(path: string, bytes: Uint8Array, taxonomy: LoadedTaxonomy): ReadonlySet<string> | null {
  if (!frozenEvidenceContractIndex(taxonomy).has(path)) return null;
  const cached = frozenEvidenceCoordinateCache.get(bytes);
  if (cached?.path === path && cached.taxonomy === taxonomy) return cached.coordinates;
  const rows = (frozenCoordinateEvidenceCoordinates(path, bytes, taxonomy.discoverySchema.frozenCoordinateEvidenceContracts) ?? frozenMarkdownCoordinateEvidenceCoordinates(path, bytes, taxonomy.discoverySchema.frozenMarkdownCoordinateEvidenceContracts))!;
  const coordinates = new Set(rows.map((row) => `${row.start}\0${row.end}\0${row.value}`));
  frozenEvidenceCoordinateCache.set(bytes, { path, taxonomy, coordinates });
  return coordinates;
}

interface FrozenPlanCoordinateAuthority { readonly planLike: boolean; readonly coordinates: ReadonlySet<string> }

const frozenPlanCoordinateCache = new WeakMap<Uint8Array, FrozenPlanCoordinateAuthority>();

function frozenPlanCoordinateAuthority(path: string, bytes: Uint8Array): FrozenPlanCoordinateAuthority {
  if (!path.endsWith(".json")) return { planLike: false, coordinates: new Set() };
  const cached = frozenPlanCoordinateCache.get(bytes);
  if (cached) return cached;
  const content = Buffer.from(bytes).toString("utf8"), coordinates = new Set<string>();
  let planLike = false;
  try {
    const value = JSON.parse(content);
    planLike = value !== null && typeof value === "object" && value.schemaVersion === 2 && value.taxonomySchemaVersion === 7 && "planDigest" in value;
    if (planLike) {
      const parsed = parseTaxonomyPlan(value);
      if (content !== canonicalJson(parsed) + "\n") throw new Error("Retained plan bytes are not canonical");
      const typedPath = /^(?:\/scope|\/excludedTreeDigests\/\d+\/relativeRoot|\/destinationAncestorPreimages\/\d+\/path|\/moves\/\d+\/(?:sourcePath|destinationPath|ownerId|sourcePreimage\/target|referenceEdits\/\d+\/(?:path|oldValue|newValue))|\/edits\/\d+\/(?:path|oldValue|newValue)|\/embeddedTicketRoots\/\d+\/(?:sourceMetadataRoot|sourceTicketRoot|canonicalTicketRoot)|\/embeddedTicketRootRelocations\/\d+\/(?:sourcePath|destinationPath|relativeEvidencePath|ownerId|preimage\/target)|\/symlinkTargetEdits\/\d+\/(?:sourcePath|finalPath|oldTarget|newTarget|logicalTargetSourcePath|logicalTargetFinalPath|ownerId|logicalTargetPreimage\/target)|\/evidenceRemovals\/\d+\/(?:sourcePath|ownerId|preimage\/target|authority\/(?:catalogPath|destinationPath|retainedFinalPath|sourcePath|ownerPath|manifestPath|fixturePath|serializedInputPath|members\/\d+\/(?:sourcePath|finalPath|preimage\/target)))|\/regenerations\/\d+\/(?:cwd|outputRoots\/\d+|(?:inputs|preOutputs|outputs)\/\d+\/(?:path|target)|preview\/(?:nodes\/\d+\/path|staleRemovals\/\d+)|staleRemovals\/\d+)|\/unresolved\/\d+\/path)$/u;
      for (const row of jsonStringCoordinates(content)) if (typedPath.test(row.pointer)) coordinates.add(`${row.start}\0${row.end}\0${row.value}`);
    }
  } catch {}
  const result = { planLike, coordinates };
  frozenPlanCoordinateCache.set(bytes, result);
  return result;
}

const frozenCoordinateCache = new WeakMap<Uint8Array, { readonly path: string; readonly taxonomy: LoadedTaxonomy; readonly coordinates: ReadonlySet<string>; readonly targets: ReadonlySet<string> }>();

function isFrozenSourceCoordinateToken(path: string, bytes: Uint8Array, token: ReferenceToken, target: string, taxonomy: LoadedTaxonomy, repoRoot: string): boolean {
  if (frozenEvidenceCoordinateAuthority(path, bytes, taxonomy)?.has(`${token.start}\0${token.end}\0${token.value}`)) return true;
  if (frozenPlanCoordinateAuthority(path, bytes).coordinates.has(`${token.start}\0${token.end}\0${token.value}`)) return true;
  const owner = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  const taxonomyPath = sourceRelative(relative(repoRoot, taxonomy.path));
  if (path !== taxonomyPath && path !== taxonomy.schema.semanticPackageProjectionContracts["nested-cargo-packages-v1"].authorityCatalogPath && !(owner?.contractKind === "exact-owner-path-catalog" && path === owner.authorityCatalogPath)) return false;
  let cached = frozenCoordinateCache.get(bytes);
  if (!cached || cached.path !== path || cached.taxonomy !== taxonomy) {
    const content = Buffer.from(bytes).toString("utf8"), allowed = new Map<string, string>(), targets = new Set<string>();
    const ownerContract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
    const packageContract = taxonomy.schema.semanticPackageProjectionContracts["nested-cargo-packages-v1"];
    const pointer = (value: string): string => value.replaceAll("~", "~0").replaceAll("/", "~1");
    const add = (location: string, value: string): void => { allowed.set(location, value); targets.add(value); };
    if (ownerContract?.contractKind === "exact-owner-path-catalog" && path === ownerContract.authorityCatalogPath && sha256(bytes) === ownerContract.authorityCatalogSha256) {
      const catalog = exactOwnedFileCatalog(repoRoot, taxonomy)!;
      catalog.cases.forEach((row, index) => add(`/cases/${index}/sourcePath`, row.sourcePath));
      for (const [id, row] of Object.entries(catalog.ownerEvidence)) row.evidencePaths.forEach((value, index) => add(`/ownerEvidence/${pointer(id)}/evidencePaths/${index}`, value));
      for (const [id, row] of Object.entries(catalog.referenceOwners)) add(`/referenceOwners/${pointer(id)}/ownerPath`, row.ownerPath);
      for (const [id, row] of Object.entries(catalog.generatorOwners)) { add(`/generatorOwners/${pointer(id)}/ownerPath`, row.ownerPath); add(`/generatorOwners/${pointer(id)}/currentOutputPath`, row.currentOutputPath); }
    }
    if (path === packageContract.authorityCatalogPath && sha256(bytes) === packageContract.authorityCatalogSha256) {
      const catalog = semanticPackageProjectionCatalog(repoRoot, taxonomy.discoverySchema)!;
      catalog.packages.forEach((row, index) => { add(`/packages/${index}/sourceRoot`, row.sourceRoot); row.mappings.forEach((mapping, mappingIndex) => add(`/packages/${index}/mappings/${mappingIndex}/sourcePath`, mapping.sourcePath)); row.sourceSplices.forEach((splice, spliceIndex) => add(`/packages/${index}/sourceSplices/${spliceIndex}/sourcePath`, splice.sourcePath)); row.derivedLeaves.forEach((leaf, leafIndex) => add(`/packages/${index}/derivedLeaves/${leafIndex}/originSourcePath`, leaf.originSourcePath)); row.generatedSourceRetirements.forEach((retirement, retirementIndex) => add(`/packages/${index}/generatedSourceRetirements/${retirementIndex}/sourcePath`, retirement.sourcePath)); });
      catalog.packages.forEach((row, index) => row.authoredSourceFragments.forEach((fragment, fragmentIndex) => add(`/packages/${index}/authoredSourceFragments/${fragmentIndex}/sourcePath`, fragment.sourcePath)));
      catalog.referenceConsumers.forEach((row, index) => add(`/referenceConsumers/${index}/path`, row.path));
      for (const [id, row] of Object.entries(catalog.referenceTokenTransforms)) add(`/referenceTokenTransforms/${pointer(id)}/sourceToken`, row.sourceToken);
    }
    if (path === taxonomyPath) {
      if (ownerContract?.contractKind === "exact-owner-path-catalog") {
        const data = JSON.parse(content).semanticOwnedFileProjectionContracts?.["readme-license-owner-leaves-v1"]?.authoredDocumentCorrections;
        if (canonicalJson(data) === canonicalJson(ownerContract.authoredDocumentCorrections)) {
          const owners = exactOwnedFileCatalog(repoRoot, taxonomy);
          for (const [id, row] of Object.entries(ownerContract.authoredDocumentCorrections)) {
            const owner = owners?.cases.find((entry) => entry.sourcePath === row.sourcePath && entry.destinationPath === row.destinationPath && entry.disposition === "owner-documentation-relocate" && canonicalJson(entry.preimage) === canonicalJson(row.preimage));
            if (owner) add(`/semanticOwnedFileProjectionContracts/readme-license-owner-leaves-v1/authoredDocumentCorrections/${pointer(id)}/sourcePath`, row.sourcePath);
          }
        }
        if (ownerContract.currentSourceRevisions && taxonomy.input?.path === taxonomyPath && taxonomy.input.contentHash === sha256(bytes) && taxonomy.input.size === bytes.byteLength && Buffer.from(taxonomy.input.bytes).equals(bytes)) {
          let revisions: ReturnType<typeof parseSemanticOwnedCurrentSourceRevisions> | undefined;
          try { revisions = parseSemanticOwnedCurrentSourceRevisions(JSON.parse(content).semanticOwnedFileProjectionContracts?.["readme-license-owner-leaves-v1"]?.currentSourceRevisions); } catch {}
          if (revisions && canonicalJson(revisions) === canonicalJson(ownerContract.currentSourceRevisions)) {
            const owners = exactOwnedFileCatalog(repoRoot, taxonomy);
            for (const [id, row] of Object.entries(revisions)) {
              const owner = owners?.cases[row.catalogCaseIndex];
              if (owner?.sourcePath === row.sourcePath && owner.disposition === "owner-documentation-relocate" && owner.fixedContractId === null && owner.generatorOwnerId === null && owner.ownerEvidenceId === "nx-project-owner-documentation" && canonicalJson(owner.preimage) === canonicalJson(row.baselinePreimage)) add(`/semanticOwnedFileProjectionContracts/readme-license-owner-leaves-v1/currentSourceRevisions/${pointer(id)}/sourcePath`, row.sourcePath);
            }
          }
        }
      }
      const catalog = semanticPackageProjectionCatalog(repoRoot, taxonomy.discoverySchema);
      for (const [id, contract] of Object.entries(taxonomy.schema.generatorContracts)) {
        const activation = contract.projectionActivation, row = catalog?.packages.find((row) => row.id === activation?.packageId);
        if (!activation || !row || activation.sourceManifestPath !== `${row.sourceRoot}/Cargo.toml` || activation.destinationManifestPath !== `${row.destinationRoot}/Cargo.toml`) continue;
        const data = JSON.parse(content).generatorContracts?.[id];
        if (canonicalJson(data?.projectionActivation) !== canonicalJson(activation)) continue;
        add(`/generatorContracts/${pointer(id)}/projectionActivation/sourceManifestPath`, activation.sourceManifestPath);
        if (Array.isArray(data.inputPatterns)) data.inputPatterns.forEach((value: unknown, index: number) => { if (typeof value === "string" && contract.inputPatterns.includes(value) && row.mappings.some((mapping) => mapping.sourcePath === value)) add(`/generatorContracts/${pointer(id)}/inputPatterns/${index}`, value); });
        const generation = contract.packageGeneration;
        if (generation && canonicalJson(data.packageGeneration) === canonicalJson(generation)) generation.browserProfile.sourceModulePaths.forEach((value, index) => { if (row.mappings.some((mapping) => mapping.sourcePath === value)) add(`/generatorContracts/${pointer(id)}/packageGeneration/browserProfile/sourceModulePaths/${index}`, value); });
      }
    }
    const coordinates = new Set(jsonStringCoordinates(content).filter((row) => allowed.get(row.pointer) === row.value).map((row) => `${row.start}\0${row.end}\0${row.value}`));
    cached = { path, taxonomy, coordinates, targets };
    frozenCoordinateCache.set(bytes, cached);
  }
  return cached.targets.has(target) && cached.coordinates.has(`${token.start}\0${token.end}\0${token.value}`);
}

interface PreflightReferenceBasis {
  readonly repoRoot: string;
  readonly taxonomy: LoadedTaxonomy;
  readonly ticketDir?: string;
  readonly transactionRoot?: string;
  readonly authorityPlan?: TaxonomyPlan;
  readonly candidates: readonly string[];
  readonly knownPaths: ReadonlySet<string>;
  readonly coordinateRoots: readonly string[];
  readonly markers: ReadonlyMap<string, string>;
  readonly observed: Map<string, { readonly witness: string; readonly content: boolean }>;
  readonly changedNodes: Set<string>;
}

function preflightReferenceNodeWitness(stat: Stats | null, bytes?: string | Uint8Array, identity = false): string {
  if (!stat) return "absent";
  const kind = stat.isSymbolicLink() ? "symlink" : stat.isFile() ? "file" : stat.isDirectory() ? "directory" : "other";
  return canonicalJson({ kind, mode: stat.mode & 0o7777, ...(kind === "file" || kind === "symlink" ? { size: stat.size } : {}), ...(bytes === undefined ? {} : { hash: sha256(bytes) }), ...(identity ? { dev: stat.dev, ino: stat.ino } : {}) });
}

function observePreflightReferenceNode(basis: PreflightReferenceBasis | undefined, path: string, stat: Stats | null, bytes?: string | Uint8Array): void {
  if (!basis) return;
  const witness = preflightReferenceNodeWitness(stat, bytes), prior = basis.observed.get(path);
  if (!prior) basis.observed.set(path, { witness, content: bytes !== undefined });
  else if (prior.witness !== witness) basis.changedNodes.add(path);
}

function capturePreflightReferenceBasis(repoRoot: string, taxonomy: LoadedTaxonomy, ticketDir?: string, transactionRoot?: string, authorityPlan?: TaxonomyPlan, cancelFile?: string, progress?: TaxonomyApplyOptions["progress"]): PreflightReferenceBasis {
  checkCancellation(repoRoot, cancelFile);
  const context = { ticketDir, transactionRoots: transactionRoot ? [transactionRoot] : [], exactEvidencePaths: [] };
  const candidates = [...new Set([...repositoryReferenceCandidatePaths(repoRoot, taxonomy, context, cancelFile), ...(authorityPlan ? planVerificationCandidatePaths(repoRoot, authorityPlan, taxonomy, ticketDir) : [])])].sort(generatorPathCompare);
  const knownPaths = new Set(candidates), markers = new Map<string, string>();
  for (const path of candidates) for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) knownPaths.add(parent);
  validateObservedFrozenEvidenceNodes(repoRoot, knownPaths, taxonomy);
  const coordinateRoots = referenceCoordinateRoots(repoRoot, candidates, taxonomy, cancelFile, (path, stat, bytes) => {
    const witness = preflightReferenceNodeWitness(stat, bytes, true), prior = markers.get(path);
    if (prior !== undefined && prior !== witness) throw new Error(`Preflight incoming reference marker witness changed: ${path}`);
    markers.set(path, witness);
  }, progress, "apply");
  return { repoRoot, taxonomy, ticketDir, transactionRoot, authorityPlan, candidates, knownPaths, coordinateRoots, markers, observed: new Map(), changedNodes: new Set() };
}

function validatePreflightReferenceBasis(basis: PreflightReferenceBasis, cancelFile?: string, progress?: TaxonomyApplyOptions["progress"]): void {
  checkCancellation(basis.repoRoot, cancelFile);
  const fresh = capturePreflightReferenceBasis(basis.repoRoot, basis.taxonomy, basis.ticketDir, basis.transactionRoot, basis.authorityPlan, cancelFile, progress);
  const unowned = (path: string): boolean => !basis.transactionRoot || path !== basis.transactionRoot && !path.startsWith(`${basis.transactionRoot}/`);
  if (canonicalJson(basis.candidates.filter(unowned)) !== canonicalJson(fresh.candidates.filter(unowned))) throw new Error("Preflight incoming reference candidate membership changed");
  const markerRows = (value: PreflightReferenceBasis) => [...value.markers].filter(([path]) => unowned(path)).sort(([left], [right]) => generatorPathCompare(left, right));
  if (canonicalJson(markerRows(basis)) !== canonicalJson(markerRows(fresh)) || canonicalJson(basis.coordinateRoots.filter(unowned)) !== canonicalJson(fresh.coordinateRoots.filter(unowned))) throw new Error("Preflight incoming reference marker witness changed");
  for (const [, path] of referenceCandidatesWithProgress([...basis.observed.keys()], "apply", progress)) {
    checkCancellation(basis.repoRoot, cancelFile);
    const expected = basis.observed.get(path)!;
    const absolute = assertLexicalInputOutsideOpaque(basis.repoRoot, path, "Preflight incoming reference freshness"), stat = lstatOrNull(absolute);
    const bytes = stat?.isSymbolicLink() ? readlinkSync(absolute) : expected.content && stat?.isFile() ? readFileSync(absolute) : undefined;
    if (basis.changedNodes.has(path) || expected.witness !== preflightReferenceNodeWitness(stat, bytes)) throw new Error(`Preflight incoming reference node changed: ${path}`);
  }
}

function lexicalTargetIncomingReferences(repoRoot: string, targetPaths: ReadonlySet<string>, ignoredSourceRoots: readonly string[], taxonomy: LoadedTaxonomy, ticketDir?: string, planAuthority?: Readonly<{ path: string; bytes: Uint8Array }>, transactionRoot?: string, authorityPlan?: TaxonomyPlan, cancelFile?: string, progress?: TaxonomyApplyOptions["progress"], projectReference?: ReturnType<typeof removalReferenceProjection>, basis?: PreflightReferenceBasis): readonly string[] {
  checkCancellation(repoRoot, cancelFile);
  if (targetPaths.size === 0) return [];
  if (basis && (basis.repoRoot !== repoRoot || basis.taxonomy !== taxonomy || basis.ticketDir !== ticketDir || basis.transactionRoot !== transactionRoot || basis.authorityPlan !== authorityPlan)) throw new Error("Preflight incoming reference basis context changed");
  const context = { ticketDir, transactionRoots: transactionRoot ? [transactionRoot] : [], exactEvidencePaths: [] };
  const candidates = basis ? new Set(basis.candidates) : new Set<string>([...repositoryReferenceCandidatePaths(repoRoot, taxonomy, context, cancelFile), ...(authorityPlan ? planVerificationCandidatePaths(repoRoot, authorityPlan, taxonomy, ticketDir) : [])]);
  const knownPaths = basis?.knownPaths ?? new Set(candidates);
  if (!basis) for (const path of candidates) for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) (knownPaths as Set<string>).add(parent);
  validateObservedFrozenEvidenceNodes(repoRoot, knownPaths, taxonomy);
  const targetIndex = referencePathIndex(targetPaths, repoRoot, basis?.coordinateRoots ?? referenceCoordinateRoots(repoRoot, candidates, taxonomy, cancelFile, undefined, progress, "apply"), knownPaths, cancelFile);
  const admitsText = incomingReferenceLexicalAdmission(targetPaths);
  const rows: string[] = [];
  for (const [, path] of referenceCandidatesWithProgress([...candidates].sort(generatorPathCompare), "apply", progress)) {
    checkCancellation(repoRoot, cancelFile);
    if (isExcluded(path, taxonomy) || ignoredSourceRoots.some((root) => path === root || path.startsWith(`${root}/`)) || (transactionRoot && (path === transactionRoot || path.startsWith(`${transactionRoot}/`)))) continue;
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Terminal incoming reference");
    const stat = lstatOrNull(absolute);
    if (!stat || stat.isDirectory()) { observePreflightReferenceNode(basis, path, stat); continue; }
    if (stat.isSymbolicLink()) {
      const raw = readlinkSync(absolute);
      observePreflightReferenceNode(basis, path, stat, raw);
      let target: string | null = null;
      if (raw.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(raw) || /^(?:\\\\|\/\/)/u.test(raw)) target = repositoryLocalSymlinkTargetPath(repoRoot, raw);
      else try { target = normalizeRelative(posix.join(posix.dirname(path), raw.replaceAll("\\", "/"))); } catch { target = null; }
      if (target && targetPaths.has(target)) rows.push(`symlink\u0000${path}\u0000${target}`);
      continue;
    }
    if (!stat.isFile() || !textualPath(path)) { observePreflightReferenceNode(basis, path, stat); continue; }
    const physicalBytes = readFileSync(absolute);
    observePreflightReferenceNode(basis, path, stat, physicalBytes);
    if (planAuthority?.path === path && physicalBytes.equals(planAuthority.bytes)) continue;
    const projected = projectReference?.(path, physicalBytes, stat.mode & 0o7777) ?? { path, bytes: physicalBytes }, bytes = projected.bytes;
    frozenEvidenceCoordinateAuthority(projected.path, bytes, taxonomy);
    const content = bytes.toString("utf8");
    if (!admitsText(content)) continue;
    for (const token of referenceTokensIncludingUnsupported(projected.path, content, targetIndex)) {
      if (token.unsupportedReason && token.physicalTargets !== undefined) for (const target of token.physicalTargets) if (targetPaths.has(target)) rows.push(`text\u0000${path}\u0000${target}`);
      const target = resolveReferenceTokenPath(projected.path, token, targetIndex);
      if (target && targetPaths.has(target) && !isFrozenSourceCoordinateToken(projected.path, bytes, token, target, taxonomy, repoRoot)) rows.push(`text\u0000${path}\u0000${target}`);
    }
  }
  return [...new Set(rows)].sort(generatorPathCompare);
}

function embeddedTargetPaths(plan: TaxonomyPlan, root: TaxonomyEmbeddedTicketRootDisposition): ReadonlySet<string> {
  const targetPaths = new Set<string>([root.sourceMetadataRoot, root.sourceTicketRoot]);
  for (const id of [...root.relocationOperationIds, ...root.removalOperationIds]) {
    const leaf = plan.embeddedTicketRootRelocations.find((entry) => entry.operationId === id)?.sourcePath ?? plan.evidenceRemovals.find((entry) => entry.operationId === id)?.sourcePath;
    if (!leaf) continue;
    targetPaths.add(leaf);
    for (let parent = posix.dirname(leaf); parent === root.sourceMetadataRoot || parent.startsWith(`${root.sourceMetadataRoot}/`); parent = posix.dirname(parent)) {
      targetPaths.add(parent);
      if (parent === root.sourceMetadataRoot) break;
    }
  }
  return targetPaths;
}

function lexicalEmbeddedIncomingReferences(repoRoot: string, plan: TaxonomyPlan, root: TaxonomyEmbeddedTicketRootDisposition, taxonomy: LoadedTaxonomy, ticketDir?: string, planAuthority?: Readonly<{ path: string; bytes: Uint8Array }>, transactionRoot?: string, cancelFile?: string, progress?: TaxonomyApplyOptions["progress"]): readonly string[] {
  return lexicalTargetIncomingReferences(repoRoot, embeddedTargetPaths(plan, root), [root.sourceMetadataRoot], taxonomy, ticketDir, planAuthority, transactionRoot, plan, cancelFile, progress);
}

function planEmbeddedTicketRoots(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy): EmbeddedDispositionPlanning {
  const violations: TaxonomyViolation[] = [];
  const roots: TaxonomyEmbeddedTicketRootDisposition[] = [];
  const relocations: TaxonomyEmbeddedTicketRootRelocation[] = [];
  const removals: TaxonomyEvidenceRemoval[] = [];
  const nested = new Map<string, { sourceTicketRoot: string; canonicalTicketRoot: string; ticketId: string }>();
  for (const entry of inventory.entries) {
    const parts = entry.sourcePath.split("/");
    for (let index = 1; index + 7 < parts.length; index++) {
      if (parts[index] !== ".🧬semio" || parts[index + 1] !== "🦑️repo" || parts[index + 2] !== "🎫️tickets") continue;
      const sourceMetadataRoot = parts.slice(0, index + 1).join("/");
      const suffix = parts.slice(index + 3, index + 7);
      if (!/^🎆️[0-9]{2}$/u.test(suffix[0] ?? "") || !/^🌙️[0-9]{2}$/u.test(suffix[1] ?? "") || !/^☀️[0-9]{2}$/u.test(suffix[2] ?? "") || !suffix[3]) { violations.push(violation("embedded-ticket-root-identity-invalid", entry.sourcePath, "Nested metadata path has no exact ticket identity")); continue; }
      const canonicalTicketRoot = [".🧬semio", "🦑️repo", "🎫️tickets", ...suffix].join("/");
      const rootContract = matchingFixedContracts(canonicalTicketRoot, taxonomy.schema.fixedDirectoryContracts, taxonomy, null).selected;
      const manifestPath = `${canonicalTicketRoot}/🎫️ticket.json`;
      const manifestContract = matchingFixedContracts(manifestPath, taxonomy.schema.fixedFilenameContracts, taxonomy, null).selected;
      const manifestStat = lstatOrNull(absolutePath(inventory.repoRoot, manifestPath));
      if (rootContract?.[0] !== "ticket-slug" || manifestContract?.[0] !== "ticket-manifest" || !manifestStat?.isFile() || manifestStat.isSymbolicLink()) { violations.push(violation("embedded-ticket-root-authority-missing", entry.sourcePath, "Canonical ticket root and exact ticket manifest authority must exist")); continue; }
      nested.set(sourceMetadataRoot, { sourceTicketRoot: parts.slice(0, index + 7).join("/"), canonicalTicketRoot, ticketId: `${splitLeadingEmoji(suffix[0]).rest}/${splitLeadingEmoji(suffix[1]).rest}/${splitLeadingEmoji(suffix[2]).rest}/${suffix[3]}` });
    }
  }
  const occupancy = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  const candidates: { root: string; rootId: string; sourcePath: string; destinationPath: string; entry: TaxonomyInventoryEntry }[] = [];
  for (const [sourceMetadataRoot, identity] of [...nested].sort(([left], [right]) => generatorPathCompare(left, right))) {
    const allLeaves = inventory.entries.filter((entry) => entry.nodeKind !== "directory" && entry.sourcePath.startsWith(`${sourceMetadataRoot}/`));
    const leaves = allLeaves.filter((entry) => entry.sourcePath.startsWith(`${identity.sourceTicketRoot}/`));
    if (allLeaves.length !== leaves.length) { violations.push(violation("embedded-ticket-root-residual-leaf", sourceMetadataRoot, "Nested metadata root contains a leaf outside its exact ticket root")); continue; }
    if (leaves.some((entry) => entry.nodeKind === "symlink" || !entry.fixedContractId)) { violations.push(violation("embedded-ticket-root-leaf-unresolved", sourceMetadataRoot, "Nested metadata root contains a symlink or a leaf without exact fixed-contract authority")); continue; }
    const sourceTreeDigest = noFollowTreeDigest(inventory.repoRoot, sourceMetadataRoot);
    const residualTreeDigest = noFollowTreeDigestExcluding(inventory.repoRoot, sourceMetadataRoot, leaves.map((entry) => entry.sourcePath));
    const incoming = incomingEmbeddedReferences(inventory, sourceMetadataRoot);
    const incomingReferenceDigest = sha256(`sha256-taxonomy-reference-set-v1\u0000${canonicalJson(incoming)}`);
    if (incoming.length > 0) { violations.push(violation("embedded-ticket-root-incoming-reference", sourceMetadataRoot, `Nested metadata root has ${incoming.length} incoming reference(s)`)); continue; }
    const authority = { sourceMetadataRoot, sourceTicketRoot: identity.sourceTicketRoot, canonicalTicketRoot: identity.canonicalTicketRoot, ticketId: identity.ticketId, sourceTreeDigest, residualTreeDigest, incomingReferenceDigest, rationaleRule: "embedded-ticket-root-relocation-v1" as const };
    const rootId = dispositionOperationId("embedded-ticket-root", authority);
    for (const entry of leaves) {
      const destinationPath = `${identity.canonicalTicketRoot}/${entry.sourcePath.slice(identity.sourceTicketRoot.length + 1)}`;
      if (destinationPath !== normalizeRelative(destinationPath) || pathPolicyViolations(destinationPath, taxonomy).length > 0 || !entry.fixedContractId) { violations.push(violation("embedded-ticket-root-destination-invalid", destinationPath, "Embedded evidence destination is noncanonical, over budget, or lacks exact fixed authority")); continue; }
      candidates.push({ root: sourceMetadataRoot, rootId, sourcePath: entry.sourcePath, destinationPath, entry });
    }
    roots.push({ operationId: rootId, ...authority, relocationOperationIds: [], removalOperationIds: [] });
  }
  const byDestination = new Map<string, typeof candidates>();
  for (const candidate of candidates) byDestination.set(candidate.destinationPath, [...(byDestination.get(candidate.destinationPath) ?? []), candidate]);
  for (const [destinationPath, group] of [...byDestination].sort(([left], [right]) => generatorPathCompare(left, right))) {
    const sorted = [...group].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
    const occupied = occupancy.get(destinationPath);
    const evidenceIdentity = (entry: TaxonomyInventoryEntry): string => canonicalJson({ nodeKind: entry.nodeKind, contentHash: entry.contentHash, mode: entry.mode, size: entry.size, ownerId: entry.ownerId, fixedContractId: entry.fixedContractId, packageRole: entry.packageRole });
    const identity = evidenceIdentity(sorted[0].entry);
    if (sorted.some((entry) => evidenceIdentity(entry.entry) !== identity) || (occupied && evidenceIdentity(occupied) !== identity)) { violations.push(violation("embedded-ticket-root-destination-conflict", destinationPath, "Many-to-one ticket evidence is not byte, mode, kind, size, owner, role and contract identical")); continue; }
    const installer = occupied ? null : sorted[0];
    if (installer) {
      const provisional = { embeddedTicketRootId: installer.rootId, sourcePath: installer.sourcePath, destinationPath, relativeEvidencePath: installer.sourcePath.slice(nested.get(installer.root)!.sourceTicketRoot.length + 1), preimage: inventoryLeafPreimage(installer.entry), fixedContractId: installer.entry.fixedContractId, ownerId: installer.entry.ownerId, rationaleRule: "embedded-ticket-root-relocation-v1" as const };
      relocations.push({ operationId: dispositionOperationId("embedded-ticket-root-relocation", provisional), ...provisional });
    }
    for (const candidate of sorted.filter((entry) => entry !== installer)) {
      const members: TaxonomyEvidenceMember[] = [
        ...sorted.map((entry) => ({ sourcePath: entry.sourcePath, finalPath: destinationPath, disposition: entry === installer ? "relocate" as const : "remove" as const, preimage: inventoryLeafPreimage(entry.entry) })),
        ...(occupied ? [{ sourcePath: destinationPath, finalPath: destinationPath, disposition: "retain" as const, preimage: inventoryLeafPreimage(occupied) }] : []),
      ].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
      const retainedFinalPath = destinationPath;
      const evidenceSetDigest = sha256(canonicalJson({ algorithm: "sha256-byte-mode-evidence-set-v1", members, retainedFinalPath }));
      const authority: TaxonomyRemovalAuthority = { kind: "byte-and-mode-identical", evidenceSetDigest, retainedFinalPath, members };
      const provisional = { sourcePath: candidate.sourcePath, preimage: inventoryLeafPreimage(candidate.entry), authority, embeddedTicketRootId: candidate.rootId, rationaleRule: "redundant-ticket-evidence-v1" as const, ownerId: candidate.entry.ownerId };
      removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
    }
  }
  for (const root of roots) {
    const children = relocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).length + removals.filter((entry) => entry.embeddedTicketRootId === root.operationId).length;
    if (children !== root.sourceTreeDigest.files + root.sourceTreeDigest.symlinks) violations.push(violation("embedded-ticket-root-closure-incomplete", root.sourceMetadataRoot, `Frozen tree has ${root.sourceTreeDigest.files + root.sourceTreeDigest.symlinks} leaves but ${children} dispositions`));
  }
  if (violations.length > 0) return { roots: [], relocations: [], removals: [], violations: stableViolations(violations) };
  const finalizedRoots = roots.map((root) => ({ ...root, relocationOperationIds: relocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId).sort(generatorPathCompare), removalOperationIds: removals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId).sort(generatorPathCompare) }));
  return { roots: finalizedRoots, relocations: relocations.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), removals: removals.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), violations: stableViolations(violations) };
}

function planTrailingEvidenceRemovals(inventory: TaxonomyInventory): readonly TaxonomyEvidenceRemoval[] {
  const rows: TaxonomyEvidenceRemoval[] = [];
  for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind !== "directory" && candidate.sourcePath.startsWith(".🧬semio/🦑️repo/🎫️tickets/") && /^[. ]+$/u.test(basename(candidate.sourcePath)))) {
    const parent = posix.dirname(entry.sourcePath);
    const identical = inventory.entries.filter((candidate) => candidate.nodeKind === entry.nodeKind && posix.dirname(candidate.sourcePath) === parent && candidate.contentHash === entry.contentHash && candidate.mode === entry.mode && candidate.size === entry.size && candidate.ownerId === entry.ownerId && candidate.fixedContractId === entry.fixedContractId && candidate.packageRole === entry.packageRole && candidate.sourcePath !== entry.sourcePath && !/^[. ]+$/u.test(basename(candidate.sourcePath))).sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
    if (identical.length === 0 || entry.referencesIn.length > 0) continue;
    const retainedFinalPath = identical[0].normalizedPath;
    const members: TaxonomyEvidenceMember[] = [{ sourcePath: entry.sourcePath, finalPath: retainedFinalPath, disposition: "remove", preimage: inventoryLeafPreimage(entry) }, ...identical.map((candidate) => ({ sourcePath: candidate.sourcePath, finalPath: candidate.normalizedPath, disposition: "retain" as const, preimage: inventoryLeafPreimage(candidate) }))].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
    const evidenceSetDigest = sha256(canonicalJson({ algorithm: "sha256-byte-mode-evidence-set-v1", members, retainedFinalPath }));
    const provisional = { sourcePath: entry.sourcePath, preimage: inventoryLeafPreimage(entry), authority: { kind: "byte-and-mode-identical" as const, evidenceSetDigest, retainedFinalPath, members }, rationaleRule: "redundant-ticket-evidence-v1" as const, ownerId: entry.ownerId };
    rows.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  return rows.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
}

function planExactGeneratedOwnerRemovals(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy): readonly TaxonomyEvidenceRemoval[] {
  const catalog = exactOwnedFileCatalog(inventory.repoRoot, taxonomy);
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  if (!catalog || contract?.contractKind !== "exact-owner-path-catalog") return [];
  const rows: TaxonomyEvidenceRemoval[] = [];
  for (const owner of catalog.cases.filter((entry) => entry.generatorOwnerId !== null)) {
    const source = inventory.entries.find((entry) => entry.sourcePath === owner.sourcePath);
    if (!source || source.nodeKind !== "file" || source.violations.some((entry) => entry.severity === "error") || source.normalizedPath !== owner.destinationPath) continue;
    const outputPreimage = { nodeKind: "file" as const, contentHash: owner.preimage.sha256, mode: Number.parseInt(owner.preimage.mode, 8), size: owner.preimage.size };
    if (canonicalJson(inventoryLeafPreimage(source)) !== canonicalJson(outputPreimage)) continue;
    const authorityBase = { kind: "exact-owner-generated-source" as const, catalogPath: contract.authorityCatalogPath, catalogContentHash: contract.authorityCatalogSha256, generatorContractId: owner.generatorOwnerId!, destinationPath: owner.destinationPath, outputPreimage };
    const authority = { ...authorityBase, authorityDigest: sha256(canonicalJson(authorityBase)) };
    const provisional = { sourcePath: owner.sourcePath, preimage: outputPreimage, authority, rationaleRule: "exact-owner-generated-source-retirement-v1" as const, ownerId: source.ownerId };
    rows.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  return rows.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
}

function planNestedCargoGeneratedSourceRemovals(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy): readonly TaxonomyEvidenceRemoval[] {
  const catalog = semanticPackageProjectionCatalog(inventory.repoRoot, taxonomy.discoverySchema);
  const contract = taxonomy.discoverySchema.semanticPackageProjectionContracts["nested-cargo-packages-v1"];
  if (!catalog || !contract) return [];
  const entries = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry])), removals: TaxonomyEvidenceRemoval[] = [];
  for (const row of catalog.packages) {
    if (row.id !== "wgpu-renderer" || !row.mappings.every((mapping) => { const source = entries.get(mapping.sourcePath); return source?.nodeKind === "file" && source.normalizedPath === mapping.destinationPath && source.contentHash === mapping.sourceHash && source.size === mapping.sourceSize && !source.violations.some((violation) => violation.severity === "error"); })) continue;
    for (const retirement of row.generatedSourceRetirements) {
      const source = entries.get(retirement.sourcePath)!, sourcePreimage = inventoryLeafPreimage(source);
      if (sourcePreimage.nodeKind !== "file" || sourcePreimage.mode !== retirement.sourceMode) continue;
      const digestible = { kind: "nested-cargo-generated-source" as const, catalogPath: contract.authorityCatalogPath, catalogContentHash: contract.authorityCatalogSha256, packageId: row.id, generatorContractId: retirement.generatorContractId, destinationPath: retirement.destinationPath, sourcePreimage };
      const authority = { ...digestible, authorityDigest: sha256(canonicalJson(digestible)) };
      const provisional = { sourcePath: retirement.sourcePath, preimage: sourcePreimage, authority, rationaleRule: "nested-cargo-generated-source-retirement-v1" as const, ownerId: source.ownerId };
      removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
    }
  }
  return removals.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
}

function planTicketImportantRemovals(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy): { readonly removals: readonly TaxonomyEvidenceRemoval[]; readonly violations: readonly TaxonomyViolation[] } {
  const removals: TaxonomyEvidenceRemoval[] = [];
  const violations: TaxonomyViolation[] = [];
  const entries = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind === "file" && basename(candidate.sourcePath) === "📌️important.md" && candidate.size === 0)) {
    const ownerPath = dirname(entry.sourcePath);
    const owner = entries.get(ownerPath);
    const manifestPath = `${ownerPath}/🎫️ticket.json`;
    const manifest = entries.get(manifestPath);
    if (owner?.fixedContractId !== "ticket-slug" || manifest?.nodeKind !== "file" || manifest.fixedContractId !== "ticket-manifest") continue;
    const manifestContent = readFileSync(absolutePath(inventory.repoRoot, manifestPath), "utf8");
    const decision = semanticOwnedFileProjectionAuthority({ ownerPath, ownerFixedDirectoryContractIds: ["ticket-slug"], manifestPath, manifestFixedFilenameContractIds: ["ticket-manifest"], manifestContent, sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "", sourceByteLength: entry.size }, taxonomy.discoverySchema);
    if (decision.disposition !== "remove") continue;
    if (entry.referencesIn.length > 0) {
      violations.push(violation("ticket-important-removal-referenced", entry.sourcePath, `Closed empty ticket important document has incoming references: ${entry.referencesIn.join(", ")}`));
      continue;
    }
    const manifestPreimage = inventoryLeafPreimage(manifest);
    if (manifestPreimage.nodeKind !== "file") throw new Error(`Ticket manifest preimage is not a regular file: ${manifestPath}`);
    const authorityBase = { kind: "owner-manifest-status" as const, contractId: "ticket-important-markdown-v1" as const, ownerPath, manifestPath, manifestPreimage, status: "closed" as const, contentState: "zero-byte" as const };
    const authority = { ...authorityBase, authorityDigest: sha256(canonicalJson(authorityBase)) };
    const provisional = { sourcePath: entry.sourcePath, preimage: inventoryLeafPreimage(entry), authority, rationaleRule: "ticket-important-closed-empty-v1" as const, ownerId: entry.ownerId };
    removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  const exactCases = ticketImportantExactMutationCases(inventory.repoRoot).filter((entry) => entry.disposition === "remove");
  const exactSources = exactCases.filter((entry) => entries.has(entry.sourcePath));
  for (const exact of exactSources) {
    const entry = entries.get(exact.sourcePath)!;
    if (canonicalJson(inventoryLeafPreimage(entry)) !== canonicalJson(exact.sourcePreimage) || entry.referencesIn.length > 0) {
      violations.push(violation("ticket-important-exact-removal-invalid", exact.sourcePath, `Exact empty residue preimage or incoming references changed for ${exact.caseId}`));
      continue;
    }
    const authorityBase = { kind: "exact-path-mutation" as const, catalogPath: exact.catalogPath, catalogContentHash: exact.catalogContentHash, caseId: exact.caseId, sourcePath: exact.sourcePath, sourcePreimage: exact.sourcePreimage, disposition: "remove" as const };
    const authority = { ...authorityBase, authorityDigest: sha256(canonicalJson(authorityBase)) };
    const provisional = { sourcePath: entry.sourcePath, preimage: inventoryLeafPreimage(entry), authority, rationaleRule: "ticket-important-exact-empty-residue-v1" as const, ownerId: entry.ownerId };
    removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  return { removals: removals.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), violations: stableViolations(violations) };
}

interface SerializedSentinelCase {
  readonly id: string;
  readonly inputPath: string;
  readonly physicalSourcePath: string | null;
  readonly expectedViolationCode: "windows-reserved-name" | "trailing-dot-or-space";
  readonly sourceContentHash: string;
}

function serializedSentinelCases(repoRoot: string): { readonly fixtureContentHash: string; readonly cases: readonly SerializedSentinelCase[] } | null {
  const absolute = absolutePath(repoRoot, TRANSACTION_SENTINEL_CASES_FIXTURE_PATH);
  const stat = lstatOrNull(absolute);
  if (!stat) return null;
  if (!stat.isFile() || stat.isSymbolicLink()) throw new Error("Transaction sentinel cases authority fixture must be a regular no-follow file");
  const bytes = readFileSync(absolute);
  const value = record(JSON.parse(bytes.toString("utf8")) as unknown, "transaction sentinel cases fixture");
  requireExactKeys(value, ["schemaVersion", "virtualPathPolicyCases", "symlinkFlavorCases"], "transaction sentinel cases fixture");
  if (value.schemaVersion !== 1 || !Array.isArray(value.virtualPathPolicyCases) || !Array.isArray(value.symlinkFlavorCases)) throw new Error("Transaction sentinel cases fixture has an invalid schema");
  const cases = value.virtualPathPolicyCases.map((item, index) => {
    const row = record(item, `transaction sentinel cases fixture.virtualPathPolicyCases[${index}]`);
    requireExactKeys(row, ["id", "inputPath", "physicalSourcePath", "expectedViolationCode", "sourceContentHash"], `transaction sentinel cases fixture.virtualPathPolicyCases[${index}]`);
    if (row.expectedViolationCode !== "windows-reserved-name" && row.expectedViolationCode !== "trailing-dot-or-space") throw new Error("Transaction sentinel cases fixture has an invalid violation code");
    if (row.physicalSourcePath !== null && typeof row.physicalSourcePath !== "string") throw new Error("Transaction sentinel cases fixture has an invalid physical source path");
    return { id: planString(row.id, "sentinel case id"), inputPath: planPath(row.inputPath, "sentinel input path"), physicalSourcePath: row.physicalSourcePath === null ? null : planPath(row.physicalSourcePath, "sentinel physical source path"), expectedViolationCode: row.expectedViolationCode, sourceContentHash: planString(row.sourceContentHash, "sentinel content hash", PLAN_HASH) } as SerializedSentinelCase;
  }).sort((left, right) => generatorPathCompare(left.id, right.id));
  if (new Set(cases.map((entry) => entry.id)).size !== cases.length || new Set(cases.map((entry) => entry.inputPath)).size !== cases.length) throw new Error("Transaction sentinel cases must have unique IDs and input paths");
  return { fixtureContentHash: sha256(bytes), cases };
}

function planSerializedEvidenceRemovals(inventory: TaxonomyInventory): { readonly removals: readonly TaxonomyEvidenceRemoval[]; readonly violations: readonly TaxonomyViolation[] } {
  const fixtureEntry = inventory.entries.find((entry) => entry.sourcePath === TRANSACTION_SENTINEL_CASES_FIXTURE_PATH);
  if (!fixtureEntry) return { removals: [], violations: [] };
  const authority = serializedSentinelCases(inventory.repoRoot);
  if (!authority || fixtureEntry.nodeKind !== "file" || fixtureEntry.contentHash !== authority.fixtureContentHash) return { removals: [], violations: [violation("serialized-sentinel-authority-invalid", TRANSACTION_SENTINEL_CASES_FIXTURE_PATH, "Serialized sentinel fixture bytes are not frozen by inventory")] };
  const removals: TaxonomyEvidenceRemoval[] = [];
  const violations: TaxonomyViolation[] = [];
  for (const sentinel of authority.cases) {
    if (sentinel.physicalSourcePath === null) continue;
    const entry = inventory.entries.find((candidate) => candidate.sourcePath === sentinel.physicalSourcePath);
    if (!entry) continue;
    if (entry.nodeKind !== "file" || entry.contentHash !== sentinel.sourceContentHash || entry.referencesIn.length > 0 || !entry.violations.some((row) => row.code === sentinel.expectedViolationCode)) {
      violations.push(violation("serialized-sentinel-source-invalid", sentinel.inputPath, `Physical sentinel does not match serialized case ${sentinel.id}`));
      continue;
    }
    const removalAuthority = { kind: "serialized-path-sentinel" as const, fixturePath: TRANSACTION_SENTINEL_CASES_FIXTURE_PATH, fixtureContentHash: authority.fixtureContentHash, caseId: sentinel.id, serializedInputPath: sentinel.inputPath, expectedViolationCode: sentinel.expectedViolationCode, authorityDigest: "" };
    const { authorityDigest: _authorityDigest, ...digestible } = removalAuthority;
    const frozenAuthority = { ...removalAuthority, authorityDigest: sha256(canonicalJson(digestible)) };
    const provisional = { sourcePath: entry.sourcePath, preimage: inventoryLeafPreimage(entry), authority: frozenAuthority, rationaleRule: "serialized-platform-sentinel-v1" as const, ownerId: entry.ownerId };
    removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  return { removals: removals.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), violations: stableViolations(violations) };
}

interface MoveReferenceAuthority {
  readonly moves: readonly TaxonomyMove[];
  readonly edits: readonly ReferenceEdit[];
  readonly editTargets: ReadonlyMap<string, string>;
  readonly resultHashes: ReadonlyMap<string, string>;
  readonly resultSizes: ReadonlyMap<string, number>;
  readonly unresolved: readonly TaxonomyViolation[];
  readonly collisionGroups: readonly CollisionGroup[];
}

function planMoveReferenceAuthority(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy, options: TaxonomyPlanOptions, embeddedRoots: readonly TaxonomyEmbeddedTicketRootDisposition[], evidenceRemovals: readonly TaxonomyEvidenceRemoval[]): MoveReferenceAuthority {
  const ownerSnapshot = exactOwnedCatalogSnapshot(inventory.repoRoot, taxonomy);
  const ownerMappings = new Map(ownerSnapshot.catalog?.cases.filter((entry) => entry.disposition !== "fixed").map((entry) => [entry.sourcePath, entry.destinationPath]) ?? []);
  const packageMappings = new Map(semanticPackageProjectionCatalog(inventory.repoRoot, taxonomy.discoverySchema)?.packages.flatMap((row) => row.mappings.map((mapping) => [mapping.sourcePath, mapping.destinationPath] as const)) ?? []);
  const embeddedPrefixes = embeddedRoots.map((root) => root.sourceMetadataRoot);
  const removalSources = new Set(evidenceRemovals.map((entry) => entry.sourcePath));
  const isEmbedded = (path: string): boolean => embeddedPrefixes.some((root) => path === root || path.startsWith(`${root}/`));
  const groups = collisionGroups(inventory.entries.filter((entry) => !removalSources.has(entry.sourcePath)), taxonomy);
  const groupBySource = new Map<string, string>();
  for (const group of groups) for (const source of group.sources) if (!groupBySource.has(source)) groupBySource.set(source, group.id);
  const preliminaryMoves: TaxonomyMove[] = inventory.entries
    .filter((entry) => entry.nodeKind !== "directory" && entry.sourcePath !== entry.normalizedPath && !groupBySource.has(entry.sourcePath) && !isEmbedded(entry.sourcePath) && !removalSources.has(entry.sourcePath) && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0)
    .map((entry) => {
      const sourcePreimage = inventoryLeafPreimage(entry);
      const exactTicketImportant = ticketImportantExactMutationAuthority(inventory.repoRoot, entry.sourcePath, sourcePreimage);
      const emptyFacet = basename(entry.sourcePath) === "📌️.empty.md" ? semanticArtifactEmptyFacetProjectionAuthority({ sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "" }, taxonomy.discoverySchema) : undefined;
      let sourceAuthority: TaxonomyMoveSourceAuthority | undefined;
      if (exactOwnedCurrentRawPath(entry.sourcePath, ownerSnapshot, taxonomy)) {
        const fresh = exactOwnedFileResolution(inventory.repoRoot, entry, ownerSnapshot, taxonomy);
        if (!fresh.sourceAuthority || fresh.result.disposition !== "project" || fresh.result.problems.length || fresh.result.entry?.destinationPath !== entry.normalizedPath) throw new Error("owner-leaf-authority-invalid: " + entry.sourcePath + ": " + fresh.result.problems.join(" | "));
        sourceAuthority = fresh.sourceAuthority;
      }
      return ({
      operationId: dispositionOperationId("move-v2", { sourcePath: entry.sourcePath, destinationPath: entry.normalizedPath, sourcePreimage }),
      sourcePath: entry.sourcePath,
      destinationPath: entry.normalizedPath,
      sourcePreimage,
      rationaleRule: emptyFacet?.disposition === "project" && emptyFacet.destinationPath === entry.normalizedPath ? emptyFacet.contractId : ownerMappings.get(entry.sourcePath) === entry.normalizedPath ? "readme-license-owner-projection-v1" : packageMappings.get(entry.sourcePath) === entry.normalizedPath ? "nested-cargo-package-projection-v1" : basename(entry.sourcePath) === "ticket.md" && entry.normalizedPath === `${dirname(entry.sourcePath)}/📝️.md` ? "ticket-document-primary-markdown-v1" : exactTicketImportant?.disposition === "move" ? "ticket-important-presence-owned-markdown-v1" : basename(entry.sourcePath) === "📌️important.md" && entry.normalizedPath.endsWith("/📌️important/📝️.md") ? "ticket-important-markdown-projection-v1" : basename(entry.sourcePath) === "📌️important.md" && entry.normalizedPath.endsWith("/📓️important/📝️.md") ? "ticket-important-history-markdown-v1" : artifactCatalogProjectionRationale(entry.sourcePath, entry.normalizedPath, taxonomy) ?? mutationProjectionRationale(entry.sourcePath, entry.normalizedPath, taxonomy) ?? (entry.semanticStem ? "semantic-stem-resolution" : entry.fixedContractId ? "fixed-contract-preservation" : "canonical-kind-name"),
      ownerId: entry.ownerId,
      collisionGroup: groupBySource.get(entry.sourcePath),
      referenceEdits: [],
      ...(sourceAuthority ? { sourceAuthority } : {}),
    }); })
    .sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath) || generatorPathCompare(left.destinationPath, right.destinationPath));
  const generatedReferenceMoves = evidenceRemovals.flatMap((entry): TaxonomyMove[] => entry.authority.kind === "exact-owner-generated-source" || entry.authority.kind === "nested-cargo-generated-source" ? [{ operationId: entry.operationId, sourcePath: entry.sourcePath, destinationPath: entry.authority.destinationPath, sourcePreimage: entry.preimage, rationaleRule: entry.authority.kind === "nested-cargo-generated-source" ? "nested-cargo-package-projection-v1" : "readme-license-owner-projection-v1", ownerId: entry.ownerId, referenceEdits: [] }] : []);
  const references = buildReferenceEdits(inventory, [...preliminaryMoves, ...generatedReferenceMoves], taxonomy, options, referencePathIndex(inventory.entries.map((entry) => entry.sourcePath)), evidenceRemovals);
  return {
    moves: preliminaryMoves.map((move) => ({ ...move, referenceEdits: references.edits.filter((edit) => references.editTargets.get(referenceEditIdentity(edit)) === move.sourcePath) })),
    edits: references.edits,
    editTargets: references.editTargets,
    resultHashes: references.resultHashes,
    resultSizes: references.resultSizes,
    unresolved: references.unresolved,
    collisionGroups: groups,
  };
}

function packageGeneratorActivated(repoRoot: string, moves: readonly TaxonomyMove[], contract: GeneratorContractSpec, taxonomy: LoadedTaxonomy, removals: readonly TaxonomyEvidenceRemoval[]): boolean {
  if (contract.target === taxonomy.discoverySchema.generatorContracts["jco-package-adapter"]?.target) {
    semanticPackageAdapterPreview(repoRoot, "jcoprobe-guest", taxonomy.discoverySchema);
    return true;
  }
  if (contract.packageGeneration) {
    const generation = contract.packageGeneration, profile = parseSemanticPackageBrowserProfile(generation.browserProfile, taxonomy.discoverySchema.pathEmojiPolicy.genericEmojiIdentities);
    const catalogPath = assertLexicalInputOutsideOpaque(repoRoot, generation.catalogPath, "Current WGPU catalog activation", true), catalogState = lstatOrNull(catalogPath);
    if (!catalogState?.isFile() || catalogState.isSymbolicLink()) throw new Error("Current WGPU activation requires its no-follow catalog");
    const catalog = parseCanonicalWgpuPackageCatalog(readFileSync(catalogPath, "utf8"), generation.catalogSha256, profile, taxonomy.discoverySchema);
    const manifestPath = assertLexicalInputOutsideOpaque(repoRoot, catalog.ownerPath + "/" + catalog.packageRelativePath + "/Cargo.toml", "Current WGPU package activation", true), manifest = lstatOrNull(manifestPath);
    if (!manifest?.isFile() || manifest.isSymbolicLink()) throw new Error("Current WGPU activation requires its no-follow package manifest");
    return true;
  }
  const activation = contract.projectionActivation;
  if (!activation) return true;
  const source = lstatOrNull(assertLexicalInputOutsideOpaque(repoRoot, activation.sourceManifestPath, "Generator source activation", true));
  const destination = lstatOrNull(assertLexicalInputOutsideOpaque(repoRoot, activation.destinationManifestPath, "Generator canonical activation", true));
  if (!source && !destination) return false;
  if (source && (!source.isFile() || source.isSymbolicLink()) || destination && (!destination.isFile() || destination.isSymbolicLink()) || source && destination) throw new Error("Generator package activation requires one exact source or canonical manifest");
  const row = semanticPackageProjectionCatalog(repoRoot, taxonomy.discoverySchema)?.packages.find((row) => row.id === activation.packageId);
  if (!row || activation.sourceManifestPath !== `${row.sourceRoot}/Cargo.toml` || activation.destinationManifestPath !== `${row.destinationRoot}/Cargo.toml`) throw new Error("Generator package activation disagrees with its exact catalog");
  const relevant = moves.filter((move) => row.mappings.some((mapping) => mapping.sourcePath === move.sourcePath));
  const retired = removals.filter((removal) => removal.authority.kind === "nested-cargo-generated-source" && removal.authority.packageId === row.id);
  const projected = relevant.length + retired.length;
  if (projected > 0 && (projected !== row.mappings.length || !row.mappings.every((mapping) => mapping.disposition === "generated" ? retired.some((removal) => removal.sourcePath === mapping.sourcePath && removal.preimage.nodeKind === "file" && removal.preimage.contentHash === mapping.sourceHash && removal.preimage.size === mapping.sourceSize && removal.authority.kind === "nested-cargo-generated-source" && removal.authority.destinationPath === mapping.destinationPath && row.generatedSourceRetirements.some((entry) => entry.sourcePath === mapping.sourcePath && entry.generatorContractId === removal.authority.generatorContractId && entry.sourceMode === removal.preimage.mode)) : relevant.some((move) => move.sourcePath === mapping.sourcePath && move.destinationPath === mapping.destinationPath && move.sourcePreimage.contentHash === mapping.sourceHash && move.sourcePreimage.size === mapping.sourceSize)))) throw new Error("Generator package activation has an incomplete projection");
  if (source && projected === 0) return false;
  if (activation.packageId === "jcoprobe-guest") semanticPackageAdapterPreview(repoRoot, activation.packageId, taxonomy.discoverySchema);
  else semanticPackageGeneratedLeafPreview(repoRoot, activation.packageId, taxonomy.discoverySchema);
  return Boolean(destination) || projected === row.mappings.length;
}

function generatorPlanning(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], edits: readonly ReferenceEdit[], taxonomy: LoadedTaxonomy, options: TaxonomyPlanOptions, evidenceRemovals: readonly TaxonomyEvidenceRemoval[] = []): GeneratorPlanningResult {
  const jcoActivation = taxonomy.schema.generatorContracts["jco-package-adapter"]?.projectionActivation;
  const packageCatalog = jcoActivation && moves.some((move) => inScope(move.sourcePath, dirname(jcoActivation.sourceManifestPath))) ? semanticPackageProjectionCatalog(inventory.repoRoot, taxonomy.discoverySchema) : null;
  const jco = packageCatalog?.packages.find((row) => row.id === "jcoprobe-guest");
  const preservedLocks = new Set(jco?.mappings.filter((mapping) => basename(mapping.sourcePath) === "Cargo.lock" && moves.some((move) => move.sourcePath === mapping.sourcePath && move.destinationPath === mapping.destinationPath && move.sourcePreimage.contentHash === mapping.sourceHash && move.sourcePreimage.size === mapping.sourceSize) && !edits.some((edit) => edit.path === mapping.destinationPath)).map((mapping) => mapping.destinationPath) ?? []);
  const mutations = new Set<string>();
  const generatedRetirements = evidenceRemovals.filter((entry): entry is TaxonomyEvidenceRemoval & { authority: Extract<TaxonomyRemovalAuthority, { kind: "exact-owner-generated-source" | "nested-cargo-generated-source" }> } => entry.authority.kind === "exact-owner-generated-source" || entry.authority.kind === "nested-cargo-generated-source");
  for (const entry of generatedRetirements) mutations.add(entry.authority.destinationPath);
  for (const move of moves) {
    mutations.add(move.sourcePath);
    mutations.add(move.destinationPath);
  }
  for (const edit of edits) {
    mutations.add(edit.path);
    const source = inventory.entries.find((entry) => entry.normalizedPath === edit.path)?.sourcePath;
    if (source) mutations.add(source);
  }
  const rows: TaxonomyViolation[] = [];
  const regenerations: TaxonomyRegeneration[] = [];
  const contracts = Object.entries(taxonomy.schema.generatorContracts).sort(([left], [right]) => left.localeCompare(right));
  for (let index = 0; index < contracts.length; index++) {
    const [id, contract] = contracts[index];
    const roots = contract.outputRoots.map((root) => root.path).sort(generatorPathCompare);
    const outputEntries = inventory.entries.filter((entry) => roots.some((root) => entry.sourcePath === root || entry.sourcePath.startsWith(`${root}/`)));
    const outputProblem = outputEntries.some((entry) => !roots.includes(entry.sourcePath) && (entry.sourcePath !== entry.normalizedPath || entry.violations.some((entry) => entry.severity === "error")));
    const outputMutation = [...mutations].some((path) => roots.some((root) => pathsOverlap(path, root)) && !(id === "external-cargo-locks" && preservedLocks.has(path)));
    const catalogInputs = contract.inputDiscovery && [...mutations].some((path) => registryCatalogPathMayAffect(path, taxonomy.discoverySchema)) ? generatorInputInventory(inventory, contract, taxonomy, options.cancelFile) : undefined;
    const compilerInputs = contract.compilerInputManifest ? generatorInputInventory(inventory, contract, taxonomy, options.cancelFile) : undefined;
    const discoveredInputs = catalogInputs ?? compilerInputs;
    const inputMutation = [...mutations].some((path) => contract.inputPatterns.some((pattern) => taxonomy.pathMatcher.matches(path, pattern))) || Boolean(discoveredInputs && (edits.some((edit) => discoveredInputs.some((input) => input.path === edit.path)) || moves.some((move) => discoveredInputs.some((input) => input.path === move.sourcePath || input.path === move.destinationPath || input.nodeKind === "directory" && (inScope(move.sourcePath, input.path) || inScope(move.destinationPath, input.path))))));
    const packageOwnerPath = contract.packageGeneration?.browserProfile.ownerPath;
    const packageOutputVerification = packageOwnerPath && (!inventory.scope || pathsOverlap(inventory.scope, packageOwnerPath));
    const compilerOutputVerification = contract.compilerInputManifest && (!inventory.scope || pathsOverlap(inventory.scope, contract.ownerPath!));
    if (!outputProblem && !outputMutation && !inputMutation && !packageOutputVerification && !compilerOutputVerification) continue;
    try { if (!packageGeneratorActivated(inventory.repoRoot, moves, contract, taxonomy, evidenceRemovals)) continue; }
    catch (error) { rows.push(violation("generator-activation-invalid", roots[0], `Generator ${id}: ${error instanceof Error ? error.message : String(error)}`)); continue; }
    const inputs = discoveredInputs ?? generatorInputInventory(inventory, contract, taxonomy, options.cancelFile);
    const preOutputs = generatorTreeInventory(inventory.repoRoot, roots, taxonomy);
    const inputDigest = sha256(canonicalJson(inputs));
    const preOutputDigest = sha256(canonicalJson(preOutputs));
    const path = roots[0];
    if (contract.ownership !== "owned") {
      rows.push(violation(`generator-ownership-${contract.ownership}`, path, `Generator contract ${id} is ${contract.ownership}; ${contract.reason}; input ${inputDigest}, output ${preOutputDigest}`));
      continue;
    }
    try {
      checkCancellation(inventory.repoRoot, options.cancelFile);
      const projection = contract.inputDiscovery ? generatorPreviewProjection("plugin-registry", inventory, moves, edits, evidenceRemovals, taxonomy, options.cancelFile) : contract.packageGeneration ? generatorPreviewProjection("wgpu-frame-worker", inventory, moves, edits, evidenceRemovals, taxonomy, options.cancelFile) : undefined;
      const preview = invokeGeneratorPreview(inventory, id, contract, taxonomy, projection, options.cancelFile);
      checkCancellation(inventory.repoRoot, options.cancelFile);
      validatePreviewPreState(preview.manifest, preOutputs);
      const outputs = previewNodeRecords(preview.manifest);
      const freshInputs = [...new Map([...inputs, ...compilerPreviewInputRecords(inventory.repoRoot, contract, taxonomy, preview.manifest)].map(row => [row.path, row])).values()].sort((left, right) => generatorPathCompare(left.path, right.path));
      const retirements = generatedRetirements.filter((entry) => entry.authority.generatorContractId === id);
      for (const retirement of retirements) {
        const generated = outputs.find((output) => output.path === retirement.authority.destinationPath);
        if (!generated || generated.nodeKind !== "file") throw new Error("Generated retirement preview lacks its exact canonical file: " + retirement.sourcePath);
        if (retirement.authority.kind === "exact-owner-generated-source" && (generated.contentHash !== retirement.authority.outputPreimage.contentHash || generated.mode !== retirement.authority.outputPreimage.mode || generated.size !== retirement.authority.outputPreimage.size)) throw new Error("Exact generated owner preview differs from frozen source: " + retirement.sourcePath);
        if (retirement.authority.kind === "nested-cargo-generated-source" && generated.mode !== retirement.authority.sourcePreimage.mode) throw new Error("Nested Cargo generated retirement changes output mode: " + retirement.sourcePath);
      }
      const changed = retirements.length > 0 || canonicalJson(preOutputs) !== canonicalJson(outputs) || preview.manifest.staleRemovals.length > 0;
      if (inputMutation || outputMutation || changed) {
        const command = ["bun", "nx", "run", contract.target!] as const;
        const verifyCommand = contract.checkTarget ? (["bun", "nx", "run", contract.checkTarget] as const) : undefined;
        const provisional = { contractId: id, cwd: contract.ownerPath!, command, verifyCommand, outputRoots: roots, inputs: freshInputs, preOutputs, outputs, preview: preview.manifest, previewManifestDigest: preview.digest, staleRemovals: preview.manifest.staleRemovals };
        regenerations.push({ id: sha256(canonicalJson(provisional)).slice(0, 24), ...provisional });
      }
      report(options.progress, "plan", "generator-preview", index + 1, contracts.length, id);
    } catch (error) {
      checkCancellation(inventory.repoRoot, options.cancelFile);
      const message = error instanceof Error ? error.message.replaceAll(resolve(inventory.repoRoot), "<repo>") : String(error);
      rows.push(violation("generator-preview-invalid", path, `Generator ${id} preview was rejected: ${message}`));
    }
  }
  return { regenerations: regenerations.sort((left, right) => left.contractId.localeCompare(right.contractId) || left.id.localeCompare(right.id)), violations: stableViolations(rows) };
}

function affectedStateDigest(rows: readonly TaxonomyAffectedStateRow[]): string {
  const sorted = [...rows].sort((left, right) => generatorPathCompare(left.path, right.path));
  const unique = new Map<string, TaxonomyAffectedStateRow>();
  for (const row of sorted) {
    const prior = unique.get(row.path);
    if (prior && canonicalJson(prior) !== canonicalJson(row)) throw new Error(`Conflicting affected path-state rows at ${row.path}`);
    unique.set(row.path, row);
  }
  return sha256(`sha256-affected-path-state-v2\u0000${canonicalJson([...unique.values()])}`);
}

function entryStateRow(path: string, entry?: TaxonomyInventoryEntry): TaxonomyAffectedStateRow {
  if (!entry) return { path, state: "absent" };
  if (entry.nodeKind === "symlink") return { path, state: "symlink", targetHash: entry.contentHash, targetSize: entry.size };
  if (entry.nodeKind === "file") return { path, state: "file", contentHash: entry.contentHash, mode: entry.mode, size: entry.size };
  throw new Error(`Affected directory requires an explicit no-follow tree digest: ${path}`);
}

function pathPreimageRow(path: string, preimage: TaxonomyPathPreimage): TaxonomyAffectedStateRow {
  if (preimage.state === "absent") return { path, state: "absent" };
  if (preimage.state === "directory") throw new Error(`Directory logical target requires recursive tree authority: ${path}`);
  return preimage.state === "symlink" ? { path, state: "symlink", targetHash: preimage.contentHash, targetSize: preimage.size } : { path, state: "file", contentHash: preimage.contentHash, mode: preimage.mode, size: preimage.size };
}

function destinationAncestorPreimages(repoRoot: string, destinations: readonly string[]): readonly TaxonomyDestinationAncestorPreimage[] {
  const rows = new Map<string, TaxonomyDestinationAncestorPreimage>();
  for (const destination of destinations) {
    for (let path = posix.dirname(destination); path !== "." && path !== ""; path = posix.dirname(path)) {
      const stat = lstatOrNull(absolutePath(repoRoot, path));
      if (stat?.isSymbolicLink() || stat && !stat.isDirectory()) throw new Error(`Mutation destination ancestor is not a no-follow directory: ${path}`);
      rows.set(path, { path, state: stat ? "directory" : "absent" });
    }
  }
  return [...rows.values()].sort((left, right) => generatorPathCompare(left.path, right.path));
}

function plannedAffectedStateDigests(inventory: TaxonomyInventory, plan: Pick<TaxonomyPlan, "moves" | "embeddedTicketRoots" | "embeddedTicketRootRelocations" | "symlinkTargetEdits" | "evidenceRemovals" | "destinationAncestorPreimages" | "edits" | "regenerations">, resultHashes: ReadonlyMap<string, string>, resultSizes: ReadonlyMap<string, number>): { readonly pre: string; readonly post: string } {
  const entries = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  const authorityStateRow = (path: string): TaxonomyAffectedStateRow => {
    const entry = entries.get(path);
    if (entry) return entryStateRow(path, entry);
    const absolute = absolutePath(inventory.repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat) return { path, state: "absent" };
    if (stat.isSymbolicLink()) { const target = readlinkSync(absolute); return { path, state: "symlink", targetHash: sha256(target), targetSize: Buffer.byteLength(target) }; }
    if (stat.isFile()) return { path, state: "file", contentHash: sha256(readFileSync(absolute)), mode: stat.mode & 0o7777, size: stat.size };
    throw new Error(`Affected authority path must be a no-follow leaf: ${path}`);
  };
  const pre: TaxonomyAffectedStateRow[] = [];
  const post: TaxonomyAffectedStateRow[] = [];
  for (const ancestor of plan.destinationAncestorPreimages) {
    pre.push(ancestor);
    post.push({ path: ancestor.path, state: "directory" });
  }
  for (const move of plan.moves) {
    const source = entries.get(move.sourcePath);
    const targetEdit = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath);
    const postSource = source && targetEdit ? { ...source, sourcePath: move.destinationPath, contentHash: targetEdit.newTargetHash, size: Buffer.byteLength(targetEdit.newTarget), symlinkTarget: targetEdit.newTarget } : source ? { ...source, sourcePath: move.destinationPath, contentHash: resultHashes.get(move.destinationPath) ?? source.contentHash, size: resultSizes.get(move.destinationPath) ?? source.size } : undefined;
    pre.push(entryStateRow(move.sourcePath, source), entryStateRow(move.destinationPath, entries.get(move.destinationPath)));
    post.push({ path: move.sourcePath, state: "absent" }, entryStateRow(move.destinationPath, postSource));
  }
  for (const relocation of plan.embeddedTicketRootRelocations) {
    pre.push(entryStateRow(relocation.sourcePath, entries.get(relocation.sourcePath)), entryStateRow(relocation.destinationPath, entries.get(relocation.destinationPath)));
    post.push({ path: relocation.sourcePath, state: "absent" }, relocation.preimage.nodeKind === "symlink" ? { path: relocation.destinationPath, state: "symlink", targetHash: relocation.preimage.contentHash, targetSize: relocation.preimage.size } : { path: relocation.destinationPath, state: "file", contentHash: relocation.preimage.contentHash, mode: relocation.preimage.mode, size: relocation.preimage.size });
  }
  for (const removal of plan.evidenceRemovals) {
    pre.push(entryStateRow(removal.sourcePath, entries.get(removal.sourcePath)));
    post.push({ path: removal.sourcePath, state: "absent" });
    if (removal.authority.kind === "byte-and-mode-identical") for (const member of removal.authority.members.filter((member) => member.disposition !== "remove")) {
      pre.push(entryStateRow(member.sourcePath, entries.get(member.sourcePath)));
      post.push(member.preimage.nodeKind === "symlink" ? { path: member.finalPath, state: "symlink", targetHash: member.preimage.contentHash, targetSize: member.preimage.size } : { path: member.finalPath, state: "file", contentHash: member.preimage.contentHash, mode: member.preimage.mode, size: member.preimage.size });
    }
    if (removal.authority.kind === "owner-manifest-status") { const manifest = authorityStateRow(removal.authority.manifestPath); pre.push(manifest); post.push(manifest); }
    if (removal.authority.kind === "exact-path-mutation" || removal.authority.kind === "exact-owner-generated-source" || removal.authority.kind === "nested-cargo-generated-source") { const catalog = authorityStateRow(removal.authority.catalogPath); pre.push(catalog); post.push(catalog); }
    if (removal.authority.kind === "serialized-path-sentinel") { const fixture = authorityStateRow(removal.authority.fixturePath); pre.push(fixture); post.push(fixture); }
  }
  for (const root of plan.embeddedTicketRoots) { pre.push({ path: root.sourceMetadataRoot, state: "directory-tree", tree: root.sourceTreeDigest }); post.push({ path: root.sourceMetadataRoot, state: "absent" }); }
  for (const edit of plan.symlinkTargetEdits) {
    pre.push({ path: edit.sourcePath, state: "symlink", targetHash: edit.oldTargetHash, targetSize: Buffer.byteLength(edit.oldTarget) });
    pre.push(pathPreimageRow(edit.logicalTargetSourcePath, edit.logicalTargetPreimage));
    post.push({ path: edit.finalPath, state: "symlink", targetHash: edit.newTargetHash, targetSize: Buffer.byteLength(edit.newTarget) });
    const logicalPost = pathPreimageRow(edit.logicalTargetFinalPath, edit.logicalTargetPreimage);
    const logicalTargetEdit = plan.symlinkTargetEdits.find((candidate) => candidate.sourcePath === edit.logicalTargetSourcePath && candidate.finalPath === edit.logicalTargetFinalPath);
    post.push(logicalPost.state === "symlink" && logicalTargetEdit ? { ...logicalPost, targetHash: logicalTargetEdit.newTargetHash, targetSize: Buffer.byteLength(logicalTargetEdit.newTarget) } : logicalPost.state === "file" && resultHashes.has(edit.logicalTargetFinalPath) ? { ...logicalPost, contentHash: resultHashes.get(edit.logicalTargetFinalPath)!, size: resultSizes.get(edit.logicalTargetFinalPath) ?? logicalPost.size } : logicalPost);
  }
  for (const [path, hash] of new Map(plan.edits.map((edit) => [edit.path, resultHashes.get(edit.path) ?? edit.preimage.contentHash]))) {
    const entry = entries.get(path) ?? inventory.entries.find((candidate) => candidate.normalizedPath === path);
    if (entry?.nodeKind === "file") { pre.push(entryStateRow(entry.sourcePath, entry)); post.push({ path, state: "file", contentHash: hash, mode: entry.mode, size: resultSizes.get(path) ?? entry.size }); }
    else {
      const preimage = plan.edits.find((edit) => edit.path === path)!.preimage;
      pre.push({ path, state: "file", contentHash: preimage.contentHash, mode: preimage.mode, size: preimage.size });
      post.push({ path, state: "file", contentHash: hash, mode: preimage.mode, size: resultSizes.get(path) ?? preimage.size });
    }
  }
  for (const regeneration of plan.regenerations) {
    pre.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(regeneration.preOutputs)) });
    post.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(regeneration.outputs)) });
  }
  return { pre: affectedStateDigest(pre), post: affectedStateDigest(post) };
}

/** 📜️ Hashes a canonical plan while excluding its self-referential planDigest field. */
export function taxonomyPlanDigest(plan: TaxonomyPlan): string {
  const { planDigest: _planDigest, ...digestible } = plan;
  return sha256(canonicalJson(digestible));
}

/** 🧭️ Produces a deterministic, fail-closed move and structured-reference plan from an inventory snapshot. */
export function planTaxonomy(inventory: TaxonomyInventory, options: TaxonomyPlanOptions): TaxonomyPlan {
  if (inventory.taxonomySchemaVersion !== 7) throw new Error("Inventory taxonomy schemaVersion must be 7");
  if (inventory.sourceTreeDigest !== sourceTreeDigest(inventory.entries)) throw new Error("Inventory sourceTreeDigest does not match inventory entries");
  const taxonomy = loadTaxonomy({ repoRoot: inventory.repoRoot, taxonomyPath: inventory.taxonomyPath });
  const baselineCommit = options.baselineCommit.trim();
  if (!PLAN_COMMIT_ID.test(baselineCommit)) throw new Error("baselineCommit must be a full lowercase SHA-1 commit ID");
  checkCancellation(inventory.repoRoot, options.cancelFile);
  const embedded = planEmbeddedTicketRoots(inventory, taxonomy);
  const trailingRemovals = planTrailingEvidenceRemovals(inventory);
  const serializedRemovals = planSerializedEvidenceRemovals(inventory);
  const ticketImportantRemovals = planTicketImportantRemovals(inventory, taxonomy);
  const generatedOwnerRemovals = [...planExactGeneratedOwnerRemovals(inventory, taxonomy), ...planNestedCargoGeneratedSourceRemovals(inventory, taxonomy)];
  const serializedSources = new Set(serializedRemovals.removals.map((entry) => entry.sourcePath));
  const ticketImportantSources = new Set(ticketImportantRemovals.removals.map((entry) => entry.sourcePath));
  const evidenceRemovals = [...embedded.removals, ...trailingRemovals.filter((entry) => !serializedSources.has(entry.sourcePath) && !ticketImportantSources.has(entry.sourcePath)), ...serializedRemovals.removals, ...ticketImportantRemovals.removals, ...generatedOwnerRemovals].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
  const ownedRemovals = new Set(evidenceRemovals.map((entry) => entry.sourcePath));
  const embeddedPrefixes = embedded.roots.map((root) => root.sourceMetadataRoot);
  const isEmbedded = (path: string): boolean => embeddedPrefixes.some((root) => path === root || path.startsWith(`${root}/`));
  const references = planMoveReferenceAuthority(inventory, taxonomy, options, embedded.roots, evidenceRemovals);
  const moves = references.moves;
  const generators = generatorPlanning(inventory, moves, references.edits, taxonomy, options, evidenceRemovals);
  const symlinks = planSymlinkTargetEdits(inventory, taxonomy, options);
  const destinationAncestors = destinationAncestorPreimages(inventory.repoRoot, [...moves.map((entry) => entry.destinationPath), ...embedded.relocations.map((entry) => entry.destinationPath), ...symlinks.edits.map((entry) => entry.finalPath), ...generators.regenerations.flatMap((entry) => entry.outputRoots)]);
  const ownedSymlinks = new Set(symlinks.edits.map((entry) => entry.sourcePath));
  const unresolved: TaxonomyViolation[] = [
    ...inventory.violations.filter((entry) => entry.severity === "error" && !isProperScopeAncestor(entry.path, inventory.scope) && !isEmbedded(entry.path) && generatorContractsForOutputPath(entry.path, taxonomy).length === 0 && !(entry.code === "symlink-absolute-target" && ownedSymlinks.has(entry.path)) && !(entry.code === "trailing-dot-or-space" && ownedRemovals.has(entry.path))),
    ...references.unresolved,
    ...generators.violations,
    ...symlinks.violations,
    ...embedded.violations,
    ...serializedRemovals.violations,
    ...ticketImportantRemovals.violations,
  ];
  for (const group of references.collisionGroups) if (group.sources.some((source) => !isEmbedded(source))) unresolved.push(violation(`collision-${group.comparison}`, group.paths[0] ?? group.sources[0], `Normalization collision ${group.id}: ${group.sources.join(", ")}`));
  for (const digest of options.excludedTreeDigests) {
    if (digest.algorithm !== "sha256-merkle-v1") unresolved.push(violation("opaque-digest-algorithm", digest.relativeRoot, `Unsupported opaque digest algorithm ${digest.algorithm}`));
    if (!inventory.pathExclusions.includes(normalizeRelative(digest.relativeRoot))) unresolved.push(violation("opaque-digest-unregistered", digest.relativeRoot, "Opaque digest is not registered by taxonomy pathExclusions"));
  }
  const affected = plannedAffectedStateDigests(inventory, { moves, embeddedTicketRoots: embedded.roots, embeddedTicketRootRelocations: embedded.relocations, symlinkTargetEdits: symlinks.edits, evidenceRemovals, destinationAncestorPreimages: destinationAncestors, edits: references.edits, regenerations: generators.regenerations }, references.resultHashes, references.resultSizes);
  const provisionalBase: TaxonomyPlan = {
    schemaVersion: 2,
    taxonomySchemaVersion: 7,
    baselineCommit,
    scope: inventory.scope,
    sourceTreeDigest: inventory.sourceTreeDigest,
    excludedTreeDigests: [...options.excludedTreeDigests].sort((a, b) => a.relativeRoot.localeCompare(b.relativeRoot)),
    moves,
    embeddedTicketRoots: embedded.roots,
    embeddedTicketRootRelocations: embedded.relocations,
    symlinkTargetEdits: symlinks.edits,
    evidenceRemovals,
    destinationAncestorPreimages: destinationAncestors,
    edits: [...references.edits].sort(referenceEditCompare),
    regenerations: generators.regenerations,
    unresolved: stableViolations(unresolved),
    expectedAffectedPreStateDigest: affected.pre,
    expectedPostStateDigest: affected.post,
    planDigest: "",
  };
  const provisional: TaxonomyPlan = { ...provisionalBase, unresolved: stableViolations([...provisionalBase.unresolved, ...projectionStaleViolations(inventory.repoRoot, provisionalBase, taxonomy, inventory)]) };
  const plan = { ...provisional, planDigest: taxonomyPlanDigest(provisional) };
  const plannedOperations = moves.length + embedded.roots.length + embedded.relocations.length + symlinks.edits.length + evidenceRemovals.length + references.edits.length + generators.regenerations.length;
  report(options.progress, "plan", "complete", plannedOperations, plannedOperations);
  return plan;
}
//#endregion 🧠️Planning API

//#region 🧱️Opaque Digests
interface OpaqueCounts {
  files: number;
  directories: number;
  symlinks: number;
  others: number;
}

function noFollowMerkleNode(path: string, counts: OpaqueCounts, format: "opaque-v1" | "path-state-v1", excluded: ReadonlySet<string> = new Set()): string {
  const stat = lstatSync(path);
  const numericMode = stat.mode & 0o7777;
  const mode = numericMode.toString(8);
  if (stat.isSymbolicLink()) {
    counts.symlinks++;
    const target = readlinkSync(path);
    return format === "opaque-v1" ? sha256(`symlink\u0000${mode}\u0000${target}`) : sha256(canonicalJson({ kind: "symlink", mode: numericMode, target }));
  }
  if (stat.isFile()) {
    counts.files++;
    return sha256(Buffer.concat([Buffer.from(format === "opaque-v1" ? `file\u0000${mode}\u0000` : `file\u0000${numericMode}\u0000${stat.size}\u0000`), readFileSync(path)]));
  }
  if (stat.isDirectory()) {
    counts.directories++;
    const children = readdirSync(path).sort((left, right) => Buffer.from(left).compare(Buffer.from(right))).filter((name) => !excluded.has(join(path, name))).map((name) => ({ name: Buffer.from(name).toString("hex"), digest: noFollowMerkleNode(join(path, name), counts, format, excluded) }));
    return format === "opaque-v1" ? sha256(`directory\u0000${mode}\u0000${children.map((child) => `${child.name}\u0000${child.digest}`).join("\u0000")}`) : sha256(canonicalJson({ kind: "directory", mode: numericMode, children }));
  }
  counts.others++;
  return format === "opaque-v1" ? sha256(`other\u0000${mode}\u0000${stat.size}`) : sha256(canonicalJson({ kind: "other", mode: numericMode, size: stat.size }));
}

function opaqueNodeDigest(path: string, counts: OpaqueCounts): string { return noFollowMerkleNode(path, counts, "opaque-v1"); }
function noFollowNodeDigest(path: string, counts: OpaqueCounts, excluded: ReadonlySet<string> = new Set()): string { return noFollowMerkleNode(path, counts, "path-state-v1", excluded); }

function noFollowTreeDigestExcluding(root: string, relativeRoot: string, excludedPaths: readonly string[]): TaxonomyNoFollowTreeDigest {
  const path = absolutePath(root, normalizeRelative(relativeRoot));
  const excluded = new Set(excludedPaths.map((entry) => absolutePath(root, entry)));
  const counts: OpaqueCounts = { files: 0, directories: 0, symlinks: 0, others: 0 };
  const digest = noFollowNodeDigest(path, counts, excluded);
  return { algorithm: "sha256-no-follow-merkle-v1", digest, ...counts };
}

function transactionTreeDigest(authority: TransactionRepositoryAuthority, logicalRoot: string, physicalRoot: string, excludedPaths: readonly string[]): TaxonomyNoFollowTreeDigest {
  if (!TransactionRepositoryAuthority.owns(authority)) throw new TransactionRepositoryAuthorityError("missing-authority", new Error("Missing captured transaction repository authority"));
  let path: string, excluded: Set<string>;
  try {
    if (!Array.isArray(excludedPaths)) throw new Error("Transaction tree exclusions must be an explicit array");
    assertTransactionRepositoryPath(authority, logicalRoot, "subtree", "Transaction logical tree");
    assertTransactionRepositoryPath(authority, physicalRoot, "subtree", "Transaction physical tree");
    for (const path of excludedPaths) sourceAdmissionAssertLexical(path, "Transaction tree exclusion", false);
    path = absolutePath(authority.repoRoot, physicalRoot);
    excluded = new Set(excludedPaths.map((entry: string) => absolutePath(authority.repoRoot, entry)));
    assertNoFollowAncestors(authority.repoRoot, path, "Transaction physical tree");
  } catch (cause) {
    if (isTransactionRepositoryAuthorityError(cause)) throw cause;
    throw new TransactionRepositoryAuthorityError("invalid-access", cause);
  }
  const counts: OpaqueCounts = { files: 0, directories: 0, symlinks: 0, others: 0 };
  const digest = noFollowNodeDigest(path, counts, excluded);
  return { algorithm: "sha256-no-follow-merkle-v1", digest, ...counts };
}

/** 🌲️ Computes an exact recursive tree identity without following any symlink. */
export function noFollowTreeDigest(root: string, relativeRoot: string): TaxonomyNoFollowTreeDigest {
  const path = absolutePath(root, normalizeRelative(relativeRoot));
  const counts: OpaqueCounts = { files: 0, directories: 0, symlinks: 0, others: 0 };
  const digest = noFollowNodeDigest(path, counts);
  return { algorithm: "sha256-no-follow-merkle-v1", digest, ...counts };
}

/** 🛡️ Computes a no-follow Merkle digest for an explicitly named opaque tree. */
export function opaqueTreeDigest(root: string, relativeRoot: string): OpaqueTreeDigest {
  const normalized = normalizeRelative(relativeRoot);
  const path = absolutePath(root, normalized);
  const counts: OpaqueCounts = { files: 0, directories: 0, symlinks: 0, others: 0 };
  const digest = opaqueNodeDigest(path, counts);
  return { algorithm: "sha256-merkle-v1", relativeRoot: normalized, digest, ...counts };
}
//#endregion 🧱️Opaque Digests

//#region ✅️Verification API
function repositoryHead(repoRoot: string): string {
  return execFileSync("git", ["rev-parse", "HEAD"], { cwd: repoRoot, encoding: "utf8" }).trim();
}

/** 🔍️ Verifies that an admitted scope has no unresolved taxonomy problem, move, or structured reference edit. */
export function verifyTaxonomy(options: TaxonomyInventoryOptions): TaxonomyVerification {
  const inventory = inventoryTaxonomy(options);
  const plan = planTaxonomy(inventory, {
    baselineCommit: options.baselineCommit ?? repositoryHead(inventory.repoRoot),
    excludedTreeDigests: options.excludedTreeDigests ?? [],
    cancelFile: options.cancelFile,
    progress: options.progress,
  });
  const violations: TaxonomyViolation[] = [...plan.unresolved];
  for (const move of plan.moves) violations.push(violation("normalization-move-required", move.sourcePath, `Path must move to ${move.destinationPath}`));
  for (const relocation of plan.embeddedTicketRootRelocations) violations.push(violation("embedded-ticket-root-relocation-required", relocation.sourcePath, `Embedded ticket evidence must relocate to ${relocation.destinationPath}`));
  for (const edit of plan.symlinkTargetEdits) violations.push(violation("symlink-target-edit-required", edit.sourcePath, `Repository-local symlink target must become ${edit.newTarget}`));
  for (const removal of plan.evidenceRemovals) violations.push(violation("evidence-removal-required", removal.sourcePath, "Redundant evidence must be disposition-staged"));
  for (const edit of plan.edits) violations.push(violation("reference-edit-required", edit.path, `Structured reference must change at ${edit.structuredLocation}`));
  const stable = stableViolations(violations);
  const clean = stable.every((entry) => entry.severity !== "error");
  report(options.progress, "verify", "complete", stable.length, stable.length);
  return { inventory, plan, violations: stable, clean };
}
//#endregion ✅️Verification API

//#region 🔐️Transaction Internals
interface MutableJournalRecord {
  schemaVersion: 2;
  revision: number;
  planDigest: string;
  attemptOrdinal: string;
  state: TaxonomyJournalState;
  stagingRoot: string;
  backupRoot: string;
  journalWriteDirectory: string;
  jsonWritePreparationName: (pid: number, token: string) => string;
  jsonPreviousName: string;
  probe?: (phase: string, path?: string) => void;
  preparedMoveIds: string[];
  stagedMoveIds: string[];
  installedMoveIds: string[];
  preparedEmbeddedRelocationIds: string[];
  stagedEmbeddedRelocationIds: string[];
  installedEmbeddedRelocationIds: string[];
  preparedEvidenceRemovalIds: string[];
  stagedEvidenceRemovalIds: string[];
  preparedEmbeddedRootIds: string[];
  stagedEmbeddedRootIds: string[];
  preparedSymlinkTargetEditIds: string[];
  stagedSymlinkTargetEditIds: string[];
  installedSymlinkTargetEditIds: string[];
  appliedEditPaths: string[];
  startedRegenerationIds: string[];
  completedRegenerationIds: string[];
  sourceParentPrunePaths: string[];
  backups: Record<string, TaxonomyBackupRecord>;
  error?: string;
}

function fsyncDirectory(path: string): void {
  try {
    const directory = openSync(path, "r");
    try { fsyncSync(directory); } finally { closeSync(directory); }
  } catch (error) {
    if (!["EINVAL", "ENOTSUP", "EISDIR"].includes(String((error as NodeJS.ErrnoException).code))) throw error;
  }
}

function fsyncFile(path: string): void {
  const file = openSync(path, "r");
  try { fsyncSync(file); } finally { closeSync(file); }
}

function durableRename(source: string, destination: string): void {
  renameSync(source, destination);
  fsyncDirectory(dirname(source));
  if (dirname(destination) !== dirname(source)) fsyncDirectory(dirname(destination));
}

function durableSymlink(target: string, path: string, type?: "file" | "dir"): void {
  symlinkSync(target, path, type);
  fsyncDirectory(dirname(path));
}

function durableRemove(path: string, recursive = false): void {
  rmSync(path, { recursive, force: true });
  fsyncDirectory(dirname(path));
}

function durablySyncGeneratorRecords(repoRoot: string, records: readonly TaxonomyGeneratorNodeRecord[]): void {
  for (const record of [...records].sort((left, right) => right.path.split("/").length - left.path.split("/").length || generatorPathCompare(left.path, right.path))) {
    const path = absolutePath(repoRoot, record.path);
    if (record.nodeKind === "file") fsyncFile(path);
    else if (record.nodeKind === "directory") fsyncDirectory(path);
    else fsyncDirectory(dirname(path));
  }
}

function writeCanonicalFile(path: string, value: unknown): void {
  mkdirSync(dirname(path), { recursive: true });
  const temporary = `${path}.writing`;
  writeFileSync(temporary, `${canonicalJson(value)}\n`, "utf8");
  const file = openSync(temporary, "r");
  try { fsyncSync(file); } finally { closeSync(file); }
  durableRename(temporary, path);
}

function canonicalJsonFile(path: string): boolean {
  try {
    const bytes = readFileSync(path, "utf8");
    return bytes === `${canonicalJson(JSON.parse(bytes))}\n`;
  } catch (error) {
    if (isTransactionRepositoryAuthorityError(error)) throw error;
    return false;
  }
}

function publishCanonicalJsonCandidate(container: string, finalName: string, previousName: string, value: unknown, preparationName: (pid: number, token: string) => string, finalPath?: string, probe?: (phase: string, path?: string) => void, phasePrefix = "transaction-json"): void {
  mkdirSync(container, { recursive: true });
  const root = join(container, preparationName(process.pid, randomUUID()));
  const leaf = join(root, finalName);
  mkdirSync(root);
  fsyncDirectory(container);
  probe?.(`${phasePrefix}-write-mkdir`, root);
  const descriptor = openSync(leaf, "wx", 0o600);
  try { writeFileSync(descriptor, `${canonicalJson(value)}\n`, "utf8"); fsyncSync(descriptor); } finally { closeSync(descriptor); }
  fsyncDirectory(root);
  probe?.(`${phasePrefix}-candidate-written`, leaf);
  const final = finalPath ?? join(container, finalName);
  const previous = join(root, previousName), finalStat = lstatOrNull(final);
  if (finalStat && (!finalStat.isFile() || finalStat.isSymbolicLink())) throw new Error(`Canonical JSON destination is occupied: ${final}`);
  if (finalStat && readFileSync(final).equals(readFileSync(leaf))) durableRemove(leaf);
  else {
    if (finalStat) { durableRename(final, previous); probe?.(`${phasePrefix}-previous-exchanged`, previous); }
    durableRename(leaf, final);
    probe?.(`${phasePrefix}-canonical-exchanged`, final);
    if (lstatOrNull(previous)) durableRemove(previous);
  }
  durableRemove(root, true);
}

function recoverCanonicalJsonCandidates(container: string, finalName: string, previousName: string, preparationName: (pid: number, token: string) => string, validate: (path: string) => void, serialized: boolean, validateOnly = false, finalPath?: string): string | undefined {
  const stat = lstatOrNull(container);
  if (!stat) return undefined;
  if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Canonical JSON container must be a no-follow directory: ${container}`);
  const actions: { root: string; candidate?: string; previous?: string; exchange: boolean; publish: boolean }[] = [];
  const final = finalPath ?? join(container, finalName);
  let prospective = lstatOrNull(final) ? final : undefined;
  for (const name of readdirSync(container).sort(generatorPathCompare)) {
    if (name === finalName) continue;
    const match = /^write-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(match[2]) || name !== preparationName(Number.parseInt(match[1], 10), match[2])) throw new Error(`Canonical JSON write preparation name is invalid: ${name}`);
    const root = join(container, name), rootStat = lstatOrNull(root);
    if (!rootStat?.isDirectory() || rootStat.isSymbolicLink()) throw new Error(`Canonical JSON write preparation must be a no-follow directory: ${name}`);
    const children = readdirSync(root).sort(generatorPathCompare);
    if (children.length > 2 || children.some((child) => child !== finalName && child !== previousName)) throw new Error(`Canonical JSON write preparation contains unexpected evidence: ${name}`);
    const previous = children.includes(previousName) ? join(root, previousName) : undefined;
    if (previous) {
      const previousStat = lstatOrNull(previous);
      if (!previousStat?.isFile() || previousStat.isSymbolicLink() || !canonicalJsonFile(previous)) throw new Error(`Canonical JSON previous evidence is invalid: ${previous}`);
      validate(previous);
    }
    if (children.length === 0) {
      if (!serialized && transactionLeaseProcessIsAlive(Number.parseInt(match[1], 10))) throw new Error(`Canonical JSON write preparation is active for pid ${match[1]}`);
      actions.push({ root, exchange: false, publish: false });
      continue;
    }
    const leaf = children.includes(finalName) ? join(root, finalName) : undefined, leafStat = leaf ? lstatOrNull(leaf) : undefined;
    if (leaf && (!leafStat?.isFile() || leafStat.isSymbolicLink())) throw new Error(`Canonical JSON write candidate must be a regular no-follow file: ${leaf}`);
    if (leaf && !canonicalJsonFile(leaf)) {
      if (previous) throw new Error(`Canonical JSON exchanged candidate is invalid: ${leaf}`);
      if (!serialized && transactionLeaseProcessIsAlive(Number.parseInt(match[1], 10))) throw new Error(`Canonical JSON write preparation is active for pid ${match[1]}`);
      actions.push({ root, exchange: false, publish: false });
      continue;
    }
    if (leaf) validate(leaf);
    const finalStat = lstatOrNull(final);
    if (finalStat && (!finalStat.isFile() || finalStat.isSymbolicLink() || !canonicalJsonFile(final))) throw new Error(`Canonical JSON destination is invalid: ${final}`);
    if (leaf && previous && finalStat) throw new Error(`Canonical JSON exchange has simultaneous previous and durable destinations: ${root}`);
    if (!leaf && previous && !finalStat) throw new Error(`Canonical JSON previous-only state has no durable destination: ${root}`);
    if (!leaf && previous) validate(final);
    const equal = Boolean(leaf && finalStat && readFileSync(final).equals(readFileSync(leaf)));
    const exchange = Boolean(leaf && finalStat && !equal && !previous);
    const publish = Boolean(leaf && !finalStat);
    prospective = leaf && (exchange || publish) ? leaf : finalStat ? final : prospective;
    actions.push({ root, candidate: leaf, previous, exchange, publish });
  }
  if (actions.length > 1) throw new Error(`Canonical JSON container has duplicate write preparations: ${container}`);
  if (!validateOnly) for (const action of actions) {
    const previous = join(action.root, previousName);
    if (action.exchange) durableRename(final, previous);
    if (action.publish || action.exchange) durableRename(action.candidate!, final);
    if (lstatOrNull(previous)) durableRemove(previous);
    durableRemove(action.root, true);
  }
  return validateOnly ? prospective : lstatOrNull(final) ? final : undefined;
}

interface TransactionLeaseRecord {
  readonly schemaVersion: 1;
  readonly planDigest: string;
  readonly attemptOrdinal: string;
  readonly token: string;
  readonly pid: number;
}

interface TransactionLeaseHandle {
  readonly root: string;
  readonly filename: string;
  readonly record: TransactionLeaseRecord;
}

const TRANSACTION_LEASE_TOKEN = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u;

function transactionLeaseSnapshot(record: TransactionLeaseRecord): TransactionLeaseRecord {
  return { schemaVersion: 1, planDigest: record.planDigest, attemptOrdinal: record.attemptOrdinal, token: record.token, pid: record.pid };
}

function parseTransactionLease(path: string, planDigest: string, attemptOrdinal: string, expectedToken?: string): TransactionLeaseRecord {
  const file = lstatOrNull(path);
  if (!file?.isFile() || file.isSymbolicLink()) throw new Error(`Transaction lease metadata must be a regular no-follow file: ${path}`);
  const bytes = readFileSync(path, "utf8");
  const value = planRecord(JSON.parse(bytes), "transaction lease", ["schemaVersion", "planDigest", "attemptOrdinal", "token", "pid"]);
  if (value.schemaVersion !== 1 || value.planDigest !== planDigest || value.attemptOrdinal !== attemptOrdinal || typeof value.token !== "string" || !TRANSACTION_LEASE_TOKEN.test(value.token) || expectedToken !== undefined && value.token !== expectedToken || !Number.isSafeInteger(value.pid) || (value.pid as number) < 1) throw new Error(`Transaction lease identity is invalid: ${path}`);
  const record = transactionLeaseSnapshot(value as unknown as TransactionLeaseRecord);
  if (bytes !== `${canonicalJson(record)}\n`) throw new Error(`Transaction lease is not canonical JSON: ${path}`);
  return record;
}

function readTransactionLease(root: string, filename: string, planDigest: string, attemptOrdinal: string, expectedToken?: string): TransactionLeaseRecord {
  const stat = lstatOrNull(root);
  if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction lease must be a direct no-follow directory: ${root}`);
  if (canonicalJson(readdirSync(root).sort(generatorPathCompare)) !== canonicalJson([filename])) throw new Error(`Transaction lease contains unexpected evidence: ${root}`);
  return parseTransactionLease(join(root, filename), planDigest, attemptOrdinal, expectedToken);
}

function transactionLeaseProcessIsAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    const code = String((error as NodeJS.ErrnoException).code);
    if (code === "EPERM") return true;
    if (code === "ESRCH") return false;
    throw error;
  }
}

function assertTransactionLeaseRepositoryAuthority(authority: TransactionRepositoryAuthority, request: { kind: "acquire"; attemptRelative: string; backupRelative: string; leaseDirectory: string; filename: string; previousName: string } | { kind: "release"; expectedLogicalLeasePath: string; handle: TransactionLeaseHandle }): void {
  if (!TransactionRepositoryAuthority.owns(authority)) throw new TransactionRepositoryAuthorityError("missing-authority", new Error("Missing captured transaction repository authority"));
  let attemptRelative: string, leaseRelative: string, physicalRelative: string, backupRelative: string | undefined;
  try {
    const component = (value: string, label: string): void => { sourceAdmissionAssertLexical(value, label, false); if (value.includes("/")) throw new Error(label + " is not a single component"); };
    if (request.kind === "acquire") {
      attemptRelative = request.attemptRelative; backupRelative = request.backupRelative;
      sourceAdmissionAssertLexical(attemptRelative, "Transaction lease attempt", false);
      sourceAdmissionAssertLexical(backupRelative, "Transaction lease backup", false);
      component(request.leaseDirectory, "Transaction lease directory"); component(request.filename, "Transaction lease filename"); component(request.previousName, "Transaction lease previous filename");
      leaseRelative = attemptRelative + "/" + request.leaseDirectory; physicalRelative = leaseRelative;
    } else if (request.kind === "release") {
      leaseRelative = request.expectedLogicalLeasePath;
      sourceAdmissionAssertLexical(leaseRelative, "Transaction lease logical root", false);
      attemptRelative = posix.dirname(leaseRelative);
      sourceAdmissionAssertLexical(attemptRelative, "Transaction lease attempt", false);
      component(request.handle.filename, "Transaction lease filename");
      const physicalRoot = request.handle.root;
      sourceAdmissionAssertLexical(physicalRoot, "Transaction lease physical root", true);
      if (!isAbsolute(physicalRoot) || resolve(physicalRoot) !== physicalRoot || physicalRoot !== absolutePath(authority.repoRoot, leaseRelative)) throw new Error("Transaction lease physical root does not match its logical authority");
      physicalRelative = relative(authority.repoRoot, physicalRoot).split(sep).join("/");
      sourceAdmissionAssertLexical(physicalRelative, "Transaction lease physical coordinate", false);
    } else throw new Error("Transaction lease access role is invalid");
  } catch (cause) {
    if (isTransactionRepositoryAuthorityError(cause)) throw cause;
    throw new TransactionRepositoryAuthorityError("invalid-access", cause);
  }
  assertTransactionRepositoryPath(authority, attemptRelative, "point", "Transaction lease attempt metadata");
  assertTransactionRepositoryPath(authority, leaseRelative, "subtree", "Transaction lease logical root");
  assertTransactionRepositoryPath(authority, physicalRelative, "subtree", "Transaction lease physical root");
  if (backupRelative !== undefined) assertTransactionRepositoryPath(authority, backupRelative, "subtree", "Transaction lease backup root");
  assertTransactionRepositoryWitness(authority, captureTransactionRepositoryAuthority(authority.repoRoot).indexRows);
}

function acquireTransactionLease(authority: TransactionRepositoryAuthority, attemptRelative: string, backupRelative: string, leaseDirectory: string, leasePreparationName: (pid: number, token: string, state: "preparing" | "stale") => string, jsonWritePreparationName: (pid: number, token: string) => string, filename: string, previousName: string, planDigest: string, attemptOrdinal: string, beforePublish?: (owned?: TransactionLeaseRecord) => void, probe?: (phase: string, path?: string) => void): TransactionLeaseHandle {
  assertTransactionLeaseRepositoryAuthority(authority, { kind: "acquire", attemptRelative, backupRelative, leaseDirectory, filename, previousName });
  const repoRoot = authority.repoRoot;
  const invokeLeaseCallback = (invoke: () => void): void => {
    try { invoke(); } catch (error) {
      if (isTransactionRepositoryAuthorityError(error)) throw error;
      assertTransactionLeaseRepositoryAuthority(authority, { kind: "acquire", attemptRelative, backupRelative, leaseDirectory, filename, previousName });
      throw error;
    }
    assertTransactionLeaseRepositoryAuthority(authority, { kind: "acquire", attemptRelative, backupRelative, leaseDirectory, filename, previousName });
  };
  const attemptRoot = absolutePath(repoRoot, attemptRelative);
  const backupRoot = absolutePath(repoRoot, backupRelative);
  const leaseRoot = join(attemptRoot, leaseDirectory);
  const backup = lstatOrNull(backupRoot);
  if (!backup?.isDirectory() || backup.isSymbolicLink()) throw new Error(`Transaction backup authority is unavailable for lease acquisition: ${backupRelative}`);
  const leasePreparations = (rejectLive = true): readonly { root: string; pid: number; token: string; state: "preparing" | "stale"; record?: TransactionLeaseRecord }[] => {
    const rows: { root: string; pid: number; token: string; state: "preparing" | "stale"; record?: TransactionLeaseRecord }[] = [];
    for (const name of readdirSync(backupRoot).sort(generatorPathCompare)) {
      const match = /^lease-([1-9][0-9]*)-([0-9a-f-]+)-(preparing|stale)$/u.exec(splitLeadingEmoji(name).rest);
      if (!match) continue;
      const root = join(backupRoot, name), pid = Number.parseInt(match[1], 10), token = match[2], state = match[3] as "preparing" | "stale";
      if (!Number.isSafeInteger(pid) || !TRANSACTION_LEASE_TOKEN.test(token) || name !== leasePreparationName(pid, token, state)) throw new Error(`Transaction lease preparation name is invalid: ${name}`);
      const stat = lstatOrNull(root);
      if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction lease preparation must be a no-follow directory: ${name}`);
      recoverCanonicalJsonCandidates(root, filename, previousName, jsonWritePreparationName, (path) => { const candidate = parseTransactionLease(path, planDigest, attemptOrdinal, token); if (candidate.pid !== pid) throw new Error(`Transaction lease preparation pid is invalid: ${name}`); }, false, true);
      const canonical = join(root, filename);
      const record = lstatOrNull(canonical) ? parseTransactionLease(canonical, planDigest, attemptOrdinal, token) : undefined;
      if (record && record.pid !== pid) throw new Error(`Transaction lease preparation pid is invalid: ${name}`);
      if (rejectLive && transactionLeaseProcessIsAlive(pid)) throw new Error(`Transaction attempt lease preparation is active for pid ${pid}`);
      rows.push({ root, pid, token, state, record });
    }
    return rows;
  };
  leasePreparations();
  if (beforePublish != null) invokeLeaseCallback(() => beforePublish());
  if (probe != null) invokeLeaseCallback(() => probe("transaction-lease-scanned", attemptRelative));
  let quarantinedLease: string | undefined;
  const record: TransactionLeaseRecord = { schemaVersion: 1, planDigest, attemptOrdinal, token: randomUUID(), pid: process.pid };
  const preparing = join(backupRoot, leasePreparationName(record.pid, record.token, "preparing"));
  try {
    const current = lstatOrNull(leaseRoot);
    if (current) {
      const staleRecord = readTransactionLease(leaseRoot, filename, planDigest, attemptOrdinal);
      if (transactionLeaseProcessIsAlive(staleRecord.pid)) throw new Error(`Transaction attempt is leased by active pid ${staleRecord.pid}`);
      const stale = join(backupRoot, leasePreparationName(staleRecord.pid, staleRecord.token, "stale"));
      if (lstatOrNull(stale)) throw new Error(`Transaction attempt contains duplicate stale lease evidence: ${staleRecord.token}`);
      durableRename(leaseRoot, stale);
      quarantinedLease = stale;
      if (probe != null) invokeLeaseCallback(() => probe("transaction-lease-stale-quarantined", normalizeRelative(relative(repoRoot, stale).replaceAll("\\", "/"))));
    }
    if (lstatOrNull(preparing)) throw new Error(`Transaction lease token collision: ${record.token}`);
    mkdirSync(preparing);
    fsyncDirectory(backupRoot);
    if (probe != null) invokeLeaseCallback(() => probe("transaction-lease-preparation-mkdir", normalizeRelative(relative(repoRoot, preparing).replaceAll("\\", "/"))));
    if (beforePublish != null) invokeLeaseCallback(() => beforePublish(record));
    publishCanonicalJsonCandidate(preparing, filename, previousName, record, jsonWritePreparationName, undefined, probe, "transaction-lease-json");
    if (probe != null) invokeLeaseCallback(() => probe("transaction-lease-prepared", normalizeRelative(relative(repoRoot, preparing).replaceAll("\\", "/"))));
    if (lstatOrNull(leaseRoot)) throw new Error("Transaction lease acquisition fence found concurrent canonical lease evidence");
    readTransactionLease(preparing, filename, planDigest, attemptOrdinal, record.token);
    if (beforePublish != null) invokeLeaseCallback(() => beforePublish(record));
    durableRename(preparing, leaseRoot);
    if (probe != null) invokeLeaseCallback(() => probe("transaction-lease-canonical-published", normalizeRelative(relative(repoRoot, leaseRoot).replaceAll("\\", "/"))));
    if (beforePublish != null) invokeLeaseCallback(() => beforePublish(record));
    for (let pass = 0; pass < 100; pass++) {
      const preparations = leasePreparations(false);
      const live = preparations.filter((preparation) => transactionLeaseProcessIsAlive(preparation.pid));
      for (const preparation of preparations.filter((entry) => !live.includes(entry))) {
        recoverCanonicalJsonCandidates(preparation.root, filename, previousName, jsonWritePreparationName, (path) => { const candidate = parseTransactionLease(path, planDigest, attemptOrdinal, preparation.token); if (candidate.pid !== preparation.pid) throw new Error(`Transaction lease preparation pid is invalid: ${basename(preparation.root)}`); }, true);
        const canonical = join(preparation.root, filename);
        if (lstatOrNull(canonical)) {
          const stale = parseTransactionLease(canonical, planDigest, attemptOrdinal, preparation.token);
          if (stale.pid !== preparation.pid) throw new Error(`Transaction lease preparation pid is invalid: ${basename(preparation.root)}`);
        }
        durableRemove(preparation.root, true);
      }
      if (live.length === 0) break;
      if (pass === 99) throw new Error("Transaction lease acquisition timed out waiting for a live contender to retire its preparation");
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 2);
    }
  } catch (error) {
    if (isTransactionRepositoryAuthorityError(error)) throw error;
    if (lstatOrNull(leaseRoot)) {
      const currentLease = readTransactionLease(leaseRoot, filename, planDigest, attemptOrdinal);
      if (canonicalJson(currentLease) === canonicalJson(record)) durableRemove(leaseRoot, true);
    }
    if (lstatOrNull(preparing)) {
      const stat = lstatOrNull(preparing);
      if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error("Transaction lease preparation ownership changed during failed acquisition");
      recoverCanonicalJsonCandidates(preparing, filename, previousName, jsonWritePreparationName, (path) => {
        const owned = parseTransactionLease(path, planDigest, attemptOrdinal, record.token);
        if (owned.pid !== record.pid) throw new Error("Transaction lease preparation ownership changed during failed acquisition");
      }, true, true);
      const canonical = join(preparing, filename);
      if (lstatOrNull(canonical)) {
        const owned = parseTransactionLease(canonical, planDigest, attemptOrdinal, record.token);
        if (owned.pid !== record.pid) throw new Error("Transaction lease preparation ownership changed during failed acquisition");
      }
      durableRemove(preparing, true);
    }
    if (quarantinedLease && lstatOrNull(quarantinedLease)) {
      if (lstatOrNull(leaseRoot)) throw new Error("Transaction stale lease destination changed during failed acquisition");
      durableRename(quarantinedLease, leaseRoot);
    }
    throw new Error(`Transaction attempt lease acquisition failed: ${error instanceof Error ? error.message : String(error)}`);
  }
  return { root: leaseRoot, filename, record };
}

function releaseTransactionLease(authority: TransactionRepositoryAuthority, expectedLogicalLeasePath: string, handle: TransactionLeaseHandle): void {
  assertTransactionLeaseRepositoryAuthority(authority, { kind: "release", expectedLogicalLeasePath, handle });
  const current = readTransactionLease(handle.root, handle.filename, handle.record.planDigest, handle.record.attemptOrdinal, handle.record.token);
  if (current.pid !== handle.record.pid) throw new Error("Transaction lease ownership changed before release");
  durableRemove(handle.root, true);
}

function journalSnapshot(journal: MutableJournalRecord): TaxonomyJournalRecord {
  return {
    schemaVersion: 2,
    revision: journal.revision,
    planDigest: journal.planDigest,
    attemptOrdinal: journal.attemptOrdinal,
    state: journal.state,
    stagingRoot: journal.stagingRoot,
    backupRoot: journal.backupRoot,
    preparedMoveIds: [...journal.preparedMoveIds].sort(generatorPathCompare),
    stagedMoveIds: [...journal.stagedMoveIds].sort(generatorPathCompare),
    installedMoveIds: [...journal.installedMoveIds].sort(generatorPathCompare),
    preparedEmbeddedRelocationIds: [...journal.preparedEmbeddedRelocationIds].sort(generatorPathCompare),
    stagedEmbeddedRelocationIds: [...journal.stagedEmbeddedRelocationIds].sort(generatorPathCompare),
    installedEmbeddedRelocationIds: [...journal.installedEmbeddedRelocationIds].sort(generatorPathCompare),
    preparedEvidenceRemovalIds: [...journal.preparedEvidenceRemovalIds].sort(generatorPathCompare),
    stagedEvidenceRemovalIds: [...journal.stagedEvidenceRemovalIds].sort(generatorPathCompare),
    preparedEmbeddedRootIds: [...journal.preparedEmbeddedRootIds].sort(generatorPathCompare),
    stagedEmbeddedRootIds: [...journal.stagedEmbeddedRootIds].sort(generatorPathCompare),
    preparedSymlinkTargetEditIds: [...journal.preparedSymlinkTargetEditIds].sort(generatorPathCompare),
    stagedSymlinkTargetEditIds: [...journal.stagedSymlinkTargetEditIds].sort(generatorPathCompare),
    installedSymlinkTargetEditIds: [...journal.installedSymlinkTargetEditIds].sort(generatorPathCompare),
    appliedEditPaths: [...journal.appliedEditPaths].sort(generatorPathCompare),
    startedRegenerationIds: [...journal.startedRegenerationIds].sort(generatorPathCompare),
    completedRegenerationIds: [...journal.completedRegenerationIds].sort(generatorPathCompare),
    sourceParentPrunePaths: [...journal.sourceParentPrunePaths].sort(generatorPathCompare),
    backups: Object.fromEntries(Object.entries(journal.backups).sort(([a], [b]) => generatorPathCompare(a, b))),
    error: journal.error,
  };
}

function persistJournal(repoRoot: string, path: string, journal: MutableJournalRecord): void {
  const stageRoot = absolutePath(repoRoot, journal.stagingRoot);
  const walRoot = join(stageRoot, journal.journalWriteDirectory);
  const walStat = lstatOrNull(walRoot);
  if (walStat && (!walStat.isDirectory() || walStat.isSymbolicLink() || readdirSync(walRoot).length > 0)) throw new Error(`Taxonomy journal WAL is occupied: ${walRoot}`);
  mkdirSync(walRoot, { recursive: true });
  const next = { ...journal, revision: journal.revision + 1 };
  publishCanonicalJsonCandidate(walRoot, basename(path), journal.jsonPreviousName, journalSnapshot(next), journal.jsonWritePreparationName, path, journal.probe, "transaction-journal");
  journal.probe?.("transaction-wal-prepared", normalizeRelative(relative(repoRoot, walRoot).replaceAll("\\", "/")));
  journal.revision = next.revision;
  durableRemove(walRoot, true);
}

function readJournal(path: string, journalWriteDirectory: string, jsonWritePreparationName: (pid: number, token: string) => string, jsonPreviousName: string): MutableJournalRecord {
  const bytes = readFileSync(path, "utf8");
  const parsed = JSON.parse(bytes);
  const value = planRecord(parsed, "taxonomy journal", ["schemaVersion", "revision", "planDigest", "attemptOrdinal", "state", "stagingRoot", "backupRoot", "preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds", "sourceParentPrunePaths", "backups"], ["error"]) as Partial<TaxonomyJournalRecord>;
  const arrays = ["preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds", "sourceParentPrunePaths"] as const;
  const states: readonly TaxonomyJournalState[] = ["prepared", "staging", "disposing", "installing", "retargeting", "editing", "regenerating", "verifying", "committed", "rolling-back", "rolled-back"];
  if (value.schemaVersion !== 2 || !Number.isSafeInteger(value.revision) || (value.revision as number) < 0 || typeof value.planDigest !== "string" || !PLAN_HASH.test(value.planDigest) || typeof value.attemptOrdinal !== "string" || !/^[0-9]{6}$/u.test(value.attemptOrdinal) || !states.includes(value.state as TaxonomyJournalState) || typeof value.stagingRoot !== "string" || typeof value.backupRoot !== "string" || !arrays.every((key) => Array.isArray(value[key])) || !value.backups || typeof value.backups !== "object" || (value.error !== undefined && typeof value.error !== "string")) throw new Error(`Invalid taxonomy journal at ${path}`);
  planPath(value.stagingRoot, "taxonomy journal stagingRoot");
  planPath(value.backupRoot, "taxonomy journal backupRoot");
  for (const key of arrays) {
    const ids = value[key] as readonly unknown[];
    const pattern = key === "appliedEditPaths" || key === "sourceParentPrunePaths" ? undefined : PLAN_OPERATION_ID;
    const parsedIds = ids.map((entry, index) => pattern ? planString(entry, `taxonomy journal ${key}[${index}]`, pattern) : planPath(entry, `taxonomy journal ${key}[${index}]`));
    if (new Set(parsedIds).size !== parsedIds.length || parsedIds.some((entry, index) => index > 0 && Buffer.from(parsedIds[index - 1]).compare(Buffer.from(entry)) >= 0)) throw new Error(`Taxonomy journal ${key} must be unique and bytewise sorted`);
  }
  for (const [logicalPath, backup] of Object.entries(value.backups)) {
    planPath(logicalPath, "taxonomy journal backup path");
    const candidate = planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind"], ["backupPath", "contentHash", "mode", "size", "target", "targetHash"]);
    if (candidate.kind === "absent") planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind"]);
    else if (candidate.kind === "file") {
      planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind", "backupPath", "contentHash", "mode", "size"]);
      planPath(candidate.backupPath, `taxonomy journal backup ${logicalPath}.backupPath`);
      planString(candidate.contentHash, `taxonomy journal backup ${logicalPath}.contentHash`, PLAN_HASH);
      planInteger(candidate.mode, `taxonomy journal backup ${logicalPath}.mode`, 0o7777);
      planInteger(candidate.size, `taxonomy journal backup ${logicalPath}.size`);
    } else if (candidate.kind === "symlink") {
      planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind", "target", "targetHash", "mode", "size"]);
      const target = planString(candidate.target, `taxonomy journal backup ${logicalPath}.target`);
      if (planString(candidate.targetHash, `taxonomy journal backup ${logicalPath}.targetHash`, PLAN_HASH) !== sha256(target) || planInteger(candidate.mode, `taxonomy journal backup ${logicalPath}.mode`, 0o7777) < 0 || planInteger(candidate.size, `taxonomy journal backup ${logicalPath}.size`) !== Buffer.byteLength(target)) throw new Error(`Taxonomy journal backup ${logicalPath} symlink preimage changed`);
    } else throw new Error(`Taxonomy journal backup ${logicalPath}.kind is invalid`);
  }
  const journal: MutableJournalRecord = {
    schemaVersion: 2,
    revision: value.revision,
    planDigest: value.planDigest,
    attemptOrdinal: value.attemptOrdinal,
    state: value.state as TaxonomyJournalState,
    stagingRoot: value.stagingRoot,
    backupRoot: value.backupRoot,
    journalWriteDirectory,
    jsonWritePreparationName,
    jsonPreviousName,
    preparedMoveIds: [...value.preparedMoveIds!],
    stagedMoveIds: [...value.stagedMoveIds!],
    installedMoveIds: [...value.installedMoveIds!],
    preparedEmbeddedRelocationIds: [...value.preparedEmbeddedRelocationIds!],
    stagedEmbeddedRelocationIds: [...value.stagedEmbeddedRelocationIds!],
    installedEmbeddedRelocationIds: [...value.installedEmbeddedRelocationIds!],
    preparedEvidenceRemovalIds: [...value.preparedEvidenceRemovalIds!],
    stagedEvidenceRemovalIds: [...value.stagedEvidenceRemovalIds!],
    preparedEmbeddedRootIds: [...value.preparedEmbeddedRootIds!],
    stagedEmbeddedRootIds: [...value.stagedEmbeddedRootIds!],
    preparedSymlinkTargetEditIds: [...value.preparedSymlinkTargetEditIds!],
    stagedSymlinkTargetEditIds: [...value.stagedSymlinkTargetEditIds!],
    installedSymlinkTargetEditIds: [...value.installedSymlinkTargetEditIds!],
    appliedEditPaths: [...value.appliedEditPaths!],
    startedRegenerationIds: [...value.startedRegenerationIds!],
    completedRegenerationIds: [...value.completedRegenerationIds!],
    sourceParentPrunePaths: [...value.sourceParentPrunePaths!],
    backups: { ...value.backups! },
    error: value.error,
  };
  if (bytes !== `${canonicalJson(journalSnapshot(journal))}\n`) throw new Error(`Taxonomy journal at ${path} is not canonical JSON`);
  if (journal.state !== "committed" && journal.sourceParentPrunePaths.length > 0) throw new Error("Source-parent pruning authority requires a committed journal");
  const empty = (keys: readonly (keyof MutableJournalRecord)[]): boolean => keys.every((key) => Array.isArray(journal[key]) && (journal[key] as unknown[]).length === 0);
  const moveFuture = ["installedMoveIds"] as const;
  const disposalFuture = ["preparedEmbeddedRootIds", "stagedEmbeddedRootIds"] as const;
  const relocationFuture = ["installedEmbeddedRelocationIds"] as const;
  const linkFuture = ["preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds"] as const;
  const editFuture = ["appliedEditPaths"] as const;
  const regenerationFuture = ["startedRegenerationIds", "completedRegenerationIds"] as const;
  if ((journal.state === "prepared" && (!empty(["preparedMoveIds", "stagedMoveIds", ...moveFuture, "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", ...relocationFuture, "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", ...disposalFuture, ...linkFuture, ...editFuture, ...regenerationFuture]) || Object.keys(journal.backups).length > 0))
    || (journal.state === "staging" && !empty([...moveFuture, ...disposalFuture, ...relocationFuture, ...linkFuture, ...editFuture, ...regenerationFuture]))
    || (journal.state === "disposing" && !empty([...moveFuture, ...relocationFuture, ...linkFuture, ...editFuture, ...regenerationFuture]))
    || (journal.state === "installing" && !empty([...linkFuture, ...editFuture, ...regenerationFuture]))
    || (journal.state === "retargeting" && !empty([...editFuture, ...regenerationFuture]))
    || (journal.state === "editing" && !empty(regenerationFuture))
    || (journal.state === "rolled-back" && !empty(["preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds"]))) throw new Error(`Taxonomy journal state ${journal.state} contains operations from an impossible phase`);
  return journal;
}

const JOURNAL_OPERATION_ARRAYS = ["preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds"] as const;

function assertJournalTransition(current: MutableJournalRecord, next: MutableJournalRecord): void {
  const order: readonly TaxonomyJournalState[] = ["prepared", "staging", "disposing", "installing", "retargeting", "editing", "regenerating", "verifying", "committed"];
  const currentRank = order.indexOf(current.state), nextRank = order.indexOf(next.state);
  const transition = current.state === "committed" || current.state === "rolled-back"
    ? next.state === current.state
    : current.state === "rolling-back" ? next.state === "rolling-back" || next.state === "rolled-back"
      : next.state === current.state || nextRank === currentRank + 1 || next.state === "rolling-back";
  if (!transition) throw new Error(`Taxonomy journal WAL has an invalid ${current.state} -> ${next.state} transition`);
  if (!(current.state === "verifying" && next.state === "committed") && canonicalJson(current.sourceParentPrunePaths) !== canonicalJson(next.sourceParentPrunePaths)) throw new Error("Taxonomy journal WAL cannot change source-parent pruning authority outside commit");
  const contains = (parent: readonly string[], child: readonly string[]): boolean => child.every((entry) => parent.includes(entry));
  const currentTerminal = current.state === "committed" || current.state === "rolled-back";
  const rollingBack = current.state === "rolling-back";
  for (const key of JOURNAL_OPERATION_ARRAYS) {
    const valid = currentTerminal
      ? canonicalJson(current[key]) === canonicalJson(next[key])
      : rollingBack ? contains(current[key], next[key]) : contains(next[key], current[key]);
    if (!valid) throw new Error(`Taxonomy journal WAL ${key} is not a legal monotonic transition`);
  }
  const currentBackupKeys = Object.keys(current.backups);
  const backupsValid = currentTerminal
    ? canonicalJson(current.backups) === canonicalJson(next.backups)
    : currentBackupKeys.every((key) => canonicalJson(current.backups[key]) === canonicalJson(next.backups[key]));
  if (!backupsValid) throw new Error("Taxonomy journal WAL backups are not a legal monotonic transition");
  if (currentTerminal && next.error !== current.error) throw new Error("Taxonomy journal WAL cannot alter terminal error evidence");
}

function reconcileJournalWal(repoRoot: string, path: string, current: MutableJournalRecord, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, validateOnly = false): MutableJournalRecord {
  const walRoot = join(absolutePath(repoRoot, current.stagingRoot), current.journalWriteDirectory);
  const stat = lstatOrNull(walRoot);
  if (!stat) return current;
  if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Taxonomy journal WAL must be a direct no-follow directory: ${walRoot}`);
  if (readdirSync(walRoot).length === 0) {
    if (!validateOnly) durableRemove(walRoot, true);
    return current;
  }
  const validateCandidate = (candidatePath: string): void => {
    const candidate = readJournal(candidatePath, current.journalWriteDirectory, current.jsonWritePreparationName, current.jsonPreviousName);
    if (canonicalJson(journalSnapshot(candidate)) === canonicalJson(journalSnapshot(current))) return;
    if (candidate.revision + 1 === current.revision && candidate.planDigest === current.planDigest && candidate.attemptOrdinal === current.attemptOrdinal && candidate.stagingRoot === current.stagingRoot && candidate.backupRoot === current.backupRoot) {
      assertJournalTransition(candidate, current);
      assertJournalPlanMembership(plan, candidate);
      assertJournalPhaseMembership(plan, candidate);
      assertJournalBackupAuthority(plan, candidate);
      return;
    }
    if (candidate.revision !== current.revision + 1 || candidate.planDigest !== current.planDigest || candidate.attemptOrdinal !== current.attemptOrdinal || candidate.stagingRoot !== current.stagingRoot || candidate.backupRoot !== current.backupRoot) throw new Error("Taxonomy journal WAL candidate identity or revision differs from its durable attempt");
    assertJournalTransition(current, candidate);
    assertJournalPlanMembership(plan, candidate);
    assertJournalPhaseMembership(plan, candidate);
    assertJournalBackupAuthority(plan, candidate);
    assertActiveTransactionEvidence(repoRoot, plan, candidate, false, true);
    const changed = candidate.state === "rolling-back" || candidate.state === "rolled-back" ? reconcileRollbackTuples(repoRoot, plan, candidate, taxonomy) : reconcileTransactionOwnedTuples(repoRoot, plan, candidate, taxonomy);
    if (changed) throw new Error("Taxonomy journal WAL candidate does not exactly match its durable filesystem tuples");
  };
  const prospectiveWal = recoverCanonicalJsonCandidates(walRoot, basename(path), current.jsonPreviousName, current.jsonWritePreparationName, validateCandidate, true, validateOnly, path);
  if (!prospectiveWal) {
    if (!validateOnly) durableRemove(walRoot, true);
    return current;
  }
  const walPath = validateOnly ? prospectiveWal : path;
  const walStat = lstatOrNull(walPath);
  if (!walStat?.isFile() || walStat.isSymbolicLink()) throw new Error(`Taxonomy journal WAL snapshot is not a regular no-follow file: ${walPath}`);
  const next = readJournal(walPath, current.journalWriteDirectory, current.jsonWritePreparationName, current.jsonPreviousName);
  if (canonicalJson(journalSnapshot(next)) === canonicalJson(journalSnapshot(current))) {
    if (!validateOnly) durableRemove(walRoot, true);
    return current;
  }
  if (next.revision !== current.revision + 1 || next.planDigest !== current.planDigest || next.attemptOrdinal !== current.attemptOrdinal || next.stagingRoot !== current.stagingRoot || next.backupRoot !== current.backupRoot) throw new Error("Taxonomy journal WAL identity or revision differs from its durable attempt");
  assertJournalTransition(current, next);
  assertJournalPlanMembership(plan, next);
  assertJournalPhaseMembership(plan, next);
  assertJournalBackupAuthority(plan, next);
  assertActiveTransactionEvidence(repoRoot, plan, next, false, true);
  const tupleChanged = next.state === "rolling-back" || next.state === "rolled-back" ? reconcileRollbackTuples(repoRoot, plan, next, taxonomy) : reconcileTransactionOwnedTuples(repoRoot, plan, next, taxonomy);
  if (tupleChanged) throw new Error("Taxonomy journal WAL does not exactly match its durable filesystem tuples");
  if (!validateOnly) {
    durableRemove(walRoot, true);
  }
  return next;
}

function lstatOrNull(path: string): Stats | null {
  try {
    return lstatSync(path);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
    throw error;
  }
}

function hashPath(path: string): string {
  const stat = lstatSync(path);
  if (stat.isSymbolicLink()) return sha256(readlinkSync(path));
  if (!stat.isFile()) throw new Error(`Expected file or symlink at ${path}`);
  return sha256(readFileSync(path));
}

function inventoryLeafPreimage(entry: Pick<TaxonomyInventoryEntry, "nodeKind" | "contentHash" | "mode" | "size" | "symlinkTarget">): TaxonomyLeafPreimage {
  if (entry.nodeKind === "file") return { nodeKind: "file", contentHash: entry.contentHash, mode: entry.mode, size: entry.size };
  if (entry.nodeKind !== "symlink" || entry.symlinkTarget === undefined || sha256(entry.symlinkTarget) !== entry.contentHash || Buffer.byteLength(entry.symlinkTarget) !== entry.size) throw new Error("Inventory leaf lacks exact no-follow file/symlink authority");
  return { nodeKind: "symlink", contentHash: entry.contentHash, mode: entry.mode, size: entry.size, target: entry.symlinkTarget };
}

function leafPreimage(path: string): TaxonomyLeafPreimage {
  const stat = lstatSync(path);
  if (!stat.isFile() && !stat.isSymbolicLink()) throw new Error(`Expected no-follow leaf at ${path}`);
  const nodeKind = stat.isSymbolicLink() ? "symlink" : "file";
  const bytes = nodeKind === "symlink" ? Buffer.from(readlinkSync(path)) : readFileSync(path);
  const core = { contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength };
  return nodeKind === "symlink" ? { nodeKind, ...core, target: bytes.toString() } : { nodeKind, ...core };
}

function leafPathPreimage(path: string): Extract<TaxonomyPathPreimage, { state: "file" | "symlink" }> {
  const leaf = leafPreimage(path);
  return leaf.nodeKind === "symlink" ? { state: "symlink", contentHash: leaf.contentHash, mode: leaf.mode, size: leaf.size, target: leaf.target } : { state: "file", contentHash: leaf.contentHash, mode: leaf.mode, size: leaf.size };
}

function assertLeafPreimage(repoRoot: string, path: string, expected: TaxonomyLeafPreimage): void {
  const absolute = absolutePath(repoRoot, path);
  if (!lstatOrNull(absolute) || canonicalJson(leafPreimage(absolute)) !== canonicalJson(expected)) throw new Error(`Disposition preimage changed: ${path}`);
}

function retargetedMovePreimage(move: TaxonomyMove, edit?: TaxonomySymlinkTargetEdit): TaxonomyLeafPreimage {
  if (!edit) return move.sourcePreimage;
  if (move.sourcePreimage.nodeKind !== "symlink" || edit.oldTarget !== move.sourcePreimage.target || edit.oldTargetHash !== move.sourcePreimage.contentHash) throw new Error(`Symlink edit is not bound to move preimage: ${move.sourcePath}`);
  return { nodeKind: "symlink", contentHash: edit.newTargetHash, mode: move.sourcePreimage.mode, size: Buffer.byteLength(edit.newTarget), target: edit.newTarget };
}

function assertDirectoryOnlyTree(path: string): void {
  const stat = lstatSync(path);
  if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Embedded root residual node is not a directory: ${path}`);
  for (const name of readdirSync(path).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))) assertDirectoryOnlyTree(join(path, name));
}

function assertWritableAncestors(repoRoot: string, logicalPath: string): void {
  const target = absolutePath(repoRoot, logicalPath);
  for (let current = dirname(target); current !== repoRoot && current !== dirname(current); current = dirname(current)) {
    const stat = lstatOrNull(current);
    if (stat?.isSymbolicLink() || (stat && !stat.isDirectory())) throw new Error(`Destination ancestor is not a no-follow directory: ${logicalPath}`);
  }
}

function canonicalDirectoryName(taxonomy: LoadedTaxonomy, kindId: string, slug: string, parentKindId?: string): string {
  const kind = taxonomy.directoryKinds.find((entry) => entry.id === kindId);
  if (!kind || !kind.slugRegex.test(slug) || ((kind.parentKindIds?.length ?? 0) > 0 && !kind.parentKindIds?.includes(parentKindId ?? ""))) throw new Error(`Taxonomy directory kind ${kindId} cannot own slug ${slug}`);
  return `${kind.emoji}${slug}`.normalize("NFC");
}

function canonicalKindOnlyFilename(taxonomy: LoadedTaxonomy, kindId: string, extension: string): string {
  const kind = taxonomy.fileKinds.find((entry) => entry.id === kindId);
  if (!kind || !kind.extensionChains.includes(extension)) throw new Error(`Taxonomy file kind ${kindId} cannot own extension ${extension}`);
  return `${kind.emoji}${extension}`.normalize("NFC");
}

function canonicalScopedKindOnlyFilename(taxonomy: LoadedTaxonomy, kindId: string, parentKindId: string, extension: string): string {
  const kind = taxonomy.schema.scopedFileKinds[kindId];
  const filename = `${kind?.emoji ?? ""}${extension}`.normalize("NFC");
  if (!kind || kind.parentDirectoryKindId !== parentKindId || !kind.extensionChains.includes(extension) || !new RegExp(kind.sourceFilenamePattern, "u").test(filename)) throw new Error(`Taxonomy scoped file kind ${kindId} cannot own ${filename} below ${parentKindId}`);
  return filename;
}

function pathsOverlap(left: string, right: string): boolean {
  const a = normalizeRelative(left);
  const b = normalizeRelative(right);
  return a === b || a === "" || b === "" || a.startsWith(`${b}/`) || b.startsWith(`${a}/`);
}

function assertGeneratorNodeRecords(records: readonly TaxonomyGeneratorNodeRecord[], roots: readonly string[], label: string): void {
  const seen = new Set<string>();
  for (const record of records) {
    const path = normalizeRelative(record.path);
    if (path !== record.path || seen.has(path)) throw new Error(`${label} contains a duplicate or noncanonical path: ${record.path}`);
    if (!roots.some((root) => path === root || path.startsWith(`${root}/`))) throw new Error(`${label} path is outside registered roots: ${path}`);
    if (!["directory", "file", "symlink"].includes(record.nodeKind) || !/^[a-f0-9]{64}$/u.test(record.contentHash) || !Number.isSafeInteger(record.mode) || record.mode < 0 || record.mode > 0o7777) throw new Error(`${label} contains an invalid node record: ${path}`);
    if (record.nodeKind !== "directory" && (!Number.isSafeInteger(record.size) || record.size < 0) || record.nodeKind === "symlink" && (sha256(record.target) !== record.contentHash || Buffer.byteLength(record.target) !== record.size)) throw new Error(`${label} contains incomplete no-follow leaf evidence: ${path}`);
    seen.add(path);
  }
  if (records.some((record, index) => index > 0 && generatorPathCompare(records[index - 1].path, record.path) > 0)) throw new Error(`${label} must be path-sorted`);
}

function nxTargetRecord(repoRoot: string, ownerPath: string, target: string): JsonRecord {
  const manifestPath = absolutePath(repoRoot, `${ownerPath}/📋️project.json`);
  const manifest = record(JSON.parse(readFileSync(manifestPath, "utf8")) as unknown, `Nx manifest ${manifestPath}`);
  const separator = target.lastIndexOf(":");
  const project = target.slice(0, separator);
  const targetName = target.slice(separator + 1);
  const targets = record(manifest.targets, `Nx manifest ${manifestPath}.targets`);
  if (manifest.name !== project || !Object.hasOwn(targets, targetName)) throw new Error(`Nx manifest ${manifestPath} does not own target ${target}`);
  return record(targets[targetName], `Nx target ${target}`);
}

function assertNxTarget(repoRoot: string, ownerPath: string, target: string): void {
  nxTargetRecord(repoRoot, ownerPath, target);
}

function assertGeneratorPreviewTarget(repoRoot: string, contract: GeneratorContractSpec): void {
  const { ownerPath, previewTarget: target } = contract;
  if (!ownerPath || !target) throw new Error("Generator lacks an exact owner JSON preview command");
  const preview = nxTargetRecord(repoRoot, ownerPath, target);
  const options = record(preview.options, `Nx target ${target}.options`);
  if (preview.executor !== "nx:run-commands" || options.cwd !== ownerPath || options.command !== `bun ./📜️script.ts ${generatorPreviewScriptArguments(contract).join(" ")}`) throw new Error(`Nx target ${target} is not the exact owner JSON preview command`);
}

function assertRegenerationContract(regeneration: TaxonomyRegeneration, taxonomy: LoadedTaxonomy, repoRoot: string): GeneratorContractSpec {
  const contract = taxonomy.schema.generatorContracts[regeneration.contractId];
  if (!contract || contract.ownership !== "owned" || !contract.ownerPath || !contract.target || !contract.previewTarget) throw new Error(`Regeneration ${regeneration.id} does not reference an owned generator contract`);
  const roots = contract.outputRoots.map((output) => output.path).sort(generatorPathCompare);
  if (regeneration.cwd !== contract.ownerPath || canonicalJson(regeneration.command) !== canonicalJson(["bun", "nx", "run", contract.target])) throw new Error(`Regeneration ${regeneration.id} command is not schema-owned`);
  const expectedVerify = contract.checkTarget ? ["bun", "nx", "run", contract.checkTarget] : undefined;
  if (canonicalJson(regeneration.verifyCommand) !== canonicalJson(expectedVerify)) throw new Error(`Regeneration ${regeneration.id} verification command is not schema-owned`);
  if (canonicalJson([...regeneration.outputRoots].sort()) !== canonicalJson(roots)) throw new Error(`Regeneration ${regeneration.id} output roots do not match its contract`);
  assertGeneratorNodeRecords(regeneration.preOutputs, roots, `Regeneration ${regeneration.id} preOutputs`);
  assertGeneratorNodeRecords(regeneration.outputs, roots, `Regeneration ${regeneration.id} outputs`);
  assertGeneratorNodeRecords(regeneration.inputs, regeneration.inputs.map((input) => input.path), `Regeneration ${regeneration.id} inputs`);
  const preview = parseGeneratorPreviewManifest(`${generatorPreviewJson(regeneration.preview)}\n`, regeneration.contractId, roots, taxonomy.exclusions.map((entry) => entry.path));
  const compilerRecords = compilerPreviewInputRecords(repoRoot, contract, taxonomy, preview), compilerPaths = new Set(compilerRecords.map(row => row.path));
  for (const row of compilerRecords) if (canonicalJson(regeneration.inputs.find(input => input.path === row.path)) !== canonicalJson(row)) throw new Error(`Regeneration ${regeneration.id} omits its compiler input witness: ${row.path}`);
  for (const input of regeneration.inputs) if (!contract.inputDiscovery && !contract.inputPatterns.some((pattern) => taxonomy.pathMatcher.matches(input.path, pattern)) && !compilerPaths.has(input.path)) throw new Error(`Regeneration ${regeneration.id} input is not schema-owned: ${input.path}`);
  if (regeneration.previewManifestDigest !== sha256(`${generatorPreviewJson(preview)}\n`) || canonicalJson(regeneration.staleRemovals) !== canonicalJson(preview.staleRemovals) || canonicalJson(regeneration.outputs) !== canonicalJson(previewNodeRecords(preview))) throw new Error(`Regeneration ${regeneration.id} does not match its frozen preview manifest`);
  validatePreviewPreState(preview, regeneration.preOutputs);
  const identity = sha256(canonicalJson({ contractId: regeneration.contractId, cwd: regeneration.cwd, command: regeneration.command, verifyCommand: regeneration.verifyCommand, outputRoots: roots, inputs: regeneration.inputs, preOutputs: regeneration.preOutputs, outputs: regeneration.outputs, preview, previewManifestDigest: regeneration.previewManifestDigest, staleRemovals: regeneration.staleRemovals })).slice(0, 24);
  if (regeneration.id !== identity) throw new Error(`Regeneration ${regeneration.id} does not match canonical regeneration bytes`);
  assertNxTarget(repoRoot, contract.ownerPath, contract.target);
  assertGeneratorPreviewTarget(repoRoot, contract);
  if (contract.checkTarget) assertNxTarget(repoRoot, contract.ownerPath, contract.checkTarget);
  return contract;
}

function removalAuthorityPaths(authority: TaxonomyRemovalAuthority): readonly string[] {
  if (authority.kind === "byte-and-mode-identical") return authority.members.flatMap((member) => [member.sourcePath, member.finalPath]);
  if (authority.kind === "owner-manifest-status") return [authority.manifestPath];
  if (authority.kind === "exact-path-mutation" || authority.kind === "exact-owner-generated-source" || authority.kind === "nested-cargo-generated-source") return [authority.catalogPath];
  return [authority.fixturePath];
}

function removalIncomingIgnoredSourceRoots(removal: TaxonomyEvidenceRemoval): readonly string[] {
  if (removal.authority.kind === "exact-path-mutation" || removal.authority.kind === "exact-owner-generated-source" || removal.authority.kind === "nested-cargo-generated-source") return [removal.sourcePath, removal.authority.catalogPath];
  if (removal.authority.kind === "serialized-path-sentinel") return [removal.sourcePath, removal.authority.fixturePath];
  return [removal.sourcePath];
}

/** 🪞️ Projects only immutable move/edit preimages during generated-source retirement preflight. */
function removalReferenceProjection(repoRoot: string, plan: TaxonomyPlan, journal?: MutableJournalRecord): (path: string, bytes: Buffer, mode: number) => Readonly<{ path: string; bytes: Buffer }> {
  const sourceMoves = new Map(plan.moves.map((move) => [move.sourcePath, move])), destinationMoves = new Map(plan.moves.map((move) => [move.destinationPath, move]));
  const groups = new Map<string, ReferenceEdit[]>();
  for (const edit of plan.edits) groups.set(edit.path, [...(groups.get(edit.path) ?? []), edit]);
  for (const [path, edits] of groups) {
    const preimage = edits[0]!.preimage, move = destinationMoves.get(path);
    if (preimage.nodeKind !== "file" || edits.some((edit) => canonicalJson(edit.preimage) !== canonicalJson(preimage)) || move && (path !== move.destinationPath || canonicalJson(move.sourcePreimage) !== canonicalJson(preimage))) throw new Error(`Retirement reference has conflicting frozen preimages: ${path}`);
    const spans = edits.map((edit) => ({ start: Number(edit.structuredLocation.match(/@(\d+)$/u)?.[1] ?? NaN), length: edit.oldValue.length })).sort((left, right) => left.start - right.start);
    if (spans.some((span, index) => !Number.isSafeInteger(span.start) || span.start < 0 || span.length === 0 || index > 0 && span.start < spans[index - 1]!.start + spans[index - 1]!.length)) throw new Error(`Retirement reference has malformed or overlapping planned edits: ${path}`);
  }
  return (path, bytes, mode) => {
    const candidates = [...new Set([sourceMoves.get(path), destinationMoves.get(path)].filter((move): move is TaxonomyMove => Boolean(move)))];
    const active = candidates.filter((move) => path === (journal?.installedMoveIds.includes(move.operationId) ? move.destinationPath : journal?.stagedMoveIds.includes(move.operationId) ? null : move.sourcePath));
    if (candidates.length && active.length !== 1) throw new Error(`Retirement reference move phase differs from its journal: ${path}`);
    const move = active[0], finalPath = move?.destinationPath ?? path, edits = groups.get(finalPath) ?? [];
    if (!move && edits.length === 0) return { path, bytes };
    const preimage = edits[0]?.preimage ?? move!.sourcePreimage;
    if (preimage.nodeKind !== "file") throw new Error(`Retirement reference requires a frozen regular file: ${path}`);
    if (journal?.appliedEditPaths.includes(finalPath)) {
      const backup = journal.backups[finalPath];
      if (backup?.kind !== "file" || canonicalJson(backup) !== canonicalJson(expectedBackupRecord(finalPath, preimage))) throw new Error(`Retirement reference lacks its exact applied-edit backup: ${path}`);
      assertLeafPreimage(repoRoot, `${journal.backupRoot}/${backup.backupPath}`, preimage);
      const result = referenceEditResult(repoRoot, plan, journal, finalPath).bytes;
      if (path !== finalPath || mode !== preimage.mode || !bytes.equals(result)) throw new Error(`Retirement reference applied result drift: ${path}`);
      return { path, bytes };
    }
    if (mode !== preimage.mode || bytes.byteLength !== preimage.size || sha256(bytes) !== preimage.contentHash) throw new Error(`Retirement reference source preimage drift: ${path}`);
    return { path: finalPath, bytes: Buffer.from(applyEditsToContent(bytes.toString("utf8"), edits)) };
  };
}

function evidenceRemovalIncomingReferences(repoRoot: string, removal: TaxonomyEvidenceRemoval, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, ticketDir?: string, planAuthority?: Readonly<{ path: string; bytes: Uint8Array }>, transactionRoot?: string, journal?: MutableJournalRecord, cancelFile?: string, progress?: TaxonomyApplyOptions["progress"]): readonly string[] {
  const project = removal.authority.kind === "nested-cargo-generated-source" || removal.authority.kind === "exact-owner-generated-source" ? removalReferenceProjection(repoRoot, plan, journal) : undefined;
  return lexicalTargetIncomingReferences(repoRoot, new Set([removal.sourcePath]), removalIncomingIgnoredSourceRoots(removal), taxonomy, ticketDir, planAuthority, transactionRoot, plan, cancelFile, progress, project);
}

function assertTicketImportantRemovalAuthority(repoRoot: string, removal: TaxonomyEvidenceRemoval, taxonomy: LoadedTaxonomy): void {
  if (removal.authority.kind === "nested-cargo-generated-source") {
    const contract = taxonomy.discoverySchema.semanticPackageProjectionContracts["nested-cargo-packages-v1"], authority = removal.authority;
    const row = semanticPackageProjectionCatalog(repoRoot, taxonomy.discoverySchema)?.packages.find((entry) => entry.id === authority.packageId);
    const declaration = row?.generatedSourceRetirements.find((entry) => entry.sourcePath === removal.sourcePath), mapping = row?.mappings.find((entry) => entry.sourcePath === removal.sourcePath);
    if (!contract || !declaration || !mapping || mapping.disposition !== "generated" || contract.authorityCatalogPath !== authority.catalogPath || contract.authorityCatalogSha256 !== authority.catalogContentHash || declaration.generatorContractId !== authority.generatorContractId || declaration.destinationPath !== authority.destinationPath || mapping.sourceHash !== authority.sourcePreimage.contentHash || mapping.sourceSize !== authority.sourcePreimage.size || declaration.sourceMode !== authority.sourcePreimage.mode || canonicalJson(removal.preimage) !== canonicalJson(authority.sourcePreimage)) throw new Error("Nested Cargo generated retirement authority changed: " + removal.sourcePath);
    return;
  }
  if (removal.authority.kind === "exact-owner-generated-source") {
    const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
    const owner = exactOwnedFileCatalog(repoRoot, taxonomy)?.cases.find((entry) => entry.sourcePath === removal.sourcePath);
    const authority = removal.authority;
    if (contract?.contractKind !== "exact-owner-path-catalog" || !owner || owner.generatorOwnerId !== authority.generatorContractId || owner.destinationPath !== authority.destinationPath || contract.authorityCatalogPath !== authority.catalogPath || contract.authorityCatalogSha256 !== authority.catalogContentHash || canonicalJson(removal.preimage) !== canonicalJson(authority.outputPreimage) || owner.preimage.sha256 !== authority.outputPreimage.contentHash || owner.preimage.size !== authority.outputPreimage.size || Number.parseInt(owner.preimage.mode, 8) !== authority.outputPreimage.mode) throw new Error("Exact generated owner retirement authority changed: " + removal.sourcePath);
    return;
  }
  if (removal.authority.kind !== "owner-manifest-status") return;
  const authority = removal.authority;
  if (dirname(removal.sourcePath) !== authority.ownerPath || basename(removal.sourcePath) !== "📌️important.md" || authority.manifestPath !== `${authority.ownerPath}/🎫️ticket.json`) throw new Error(`Ticket important removal escapes its owner: ${removal.sourcePath}`);
  if (!taxonomy.pathMatcher.matches(authority.ownerPath, taxonomy.schema.fixedDirectoryContracts["ticket-slug"].pathPattern) || !taxonomy.pathMatcher.matches(authority.manifestPath, taxonomy.schema.fixedFilenameContracts["ticket-manifest"].pathPattern)) throw new Error(`Ticket important removal owner contracts changed: ${removal.sourcePath}`);
  assertLeafPreimage(repoRoot, authority.manifestPath, authority.manifestPreimage);
  if (ticketManifestState(readFileSync(absolutePath(repoRoot, authority.manifestPath), "utf8")) !== "closed") throw new Error(`Ticket important removal manifest status changed: ${authority.manifestPath}`);
  const { authorityDigest: _digest, ...digestible } = authority;
  if (authority.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(`Ticket important removal authority digest changed: ${removal.sourcePath}`);
}

function assertTicketImportantExactRemovalAuthority(repoRoot: string, removal: TaxonomyEvidenceRemoval): void {
  if (removal.authority.kind !== "exact-path-mutation") return;
  const authority = removal.authority;
  const exact = ticketImportantExactMutationCases(repoRoot).find((entry) => entry.caseId === authority.caseId);
  if (!exact || exact.disposition !== "remove" || exact.catalogPath !== authority.catalogPath || exact.catalogContentHash !== authority.catalogContentHash || exact.sourcePath !== removal.sourcePath || canonicalJson(exact.sourcePreimage) !== canonicalJson(removal.preimage)) throw new Error(`Ticket important exact mutation authority changed: ${authority.caseId}`);
  const { authorityDigest: _digest, ...digestible } = authority;
  if (authority.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(`Ticket important exact mutation digest changed: ${authority.caseId}`);
}

function assertPlanOutsideTransaction(plan: TaxonomyPlan, transactionRoot: string, taxonomy: LoadedTaxonomy, repoRoot: string): void {
  transactionBackupAuthorities(plan);
  for (const removal of plan.evidenceRemovals) if (removal.authority.kind === "nested-cargo-generated-source") {
    const authority = removal.authority;
    const owners = plan.regenerations.filter((regeneration) => regeneration.contractId === authority.generatorContractId && regeneration.outputs.some((output) => output.path === authority.destinationPath && output.nodeKind === "file" && output.mode === authority.sourcePreimage.mode));
    if (owners.length !== 1 || owners[0]!.outputRoots.some((root) => pathsOverlap(root, removal.sourcePath))) throw new Error("Nested Cargo generated retirement requires exactly one disjoint canonical generator: " + removal.sourcePath);
  }
  const paths = [
    ...(plan.scope ? [plan.scope] : []),
    ...plan.moves.flatMap((move) => [move.sourcePath, move.destinationPath]),
    ...plan.embeddedTicketRoots.flatMap((root) => [root.sourceMetadataRoot, root.sourceTicketRoot, root.canonicalTicketRoot]),
    ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
    ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
    ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...removalAuthorityPaths(entry.authority)]),
    ...plan.destinationAncestorPreimages.map((entry) => entry.path),
    ...plan.edits.map((edit) => edit.path),
    ...plan.regenerations.flatMap((regeneration) => [regeneration.cwd, ...regeneration.outputRoots, ...regeneration.inputs.map((input) => input.path), ...regeneration.preOutputs.map((output) => output.path), ...regeneration.outputs.map((output) => output.path), ...regeneration.staleRemovals]),
  ];
  const excludedPath = paths.find((path) => isExcluded(path, taxonomy));
  if (excludedPath) throw new Error(`Plan path crosses a lexical opaque exclusion: ${excludedPath}`);
  const ancestorAuthority = new Map(plan.destinationAncestorPreimages.map((entry) => [entry.path, entry]));
  const overlap = paths.find((path) => pathsOverlap(path, transactionRoot) && !(ancestorAuthority.get(path)?.state === "directory" && transactionRoot.startsWith(`${path}/`)));
  if (overlap) throw new Error(`Plan path overlaps taxonomy transaction root: ${overlap} <> ${transactionRoot}`);
  assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, transactionRoot), "taxonomy transaction root", true);
  for (const path of paths) assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, path), `plan path ${path}`);
  const directoryRoles = [
    ...(plan.scope ? [plan.scope] : []),
    ...plan.embeddedTicketRoots.flatMap((root) => [root.sourceMetadataRoot, root.sourceTicketRoot, root.canonicalTicketRoot]),
    ...plan.regenerations.flatMap((regeneration) => [regeneration.cwd, ...regeneration.outputRoots]),
  ];
  for (const path of directoryRoles) assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, path), `plan directory ${path}`, true);
  const sourceRoles = new Map<string, string>();
  const ownSource = (path: string, role: string): void => { const prior = sourceRoles.get(path); if (prior) throw new Error(`Plan source has conflicting ${prior} and ${role} operations: ${path}`); sourceRoles.set(path, role); };
  for (const move of plan.moves) ownSource(move.sourcePath, "move");
  for (const relocation of plan.embeddedTicketRootRelocations) ownSource(relocation.sourcePath, "embedded relocation");
  for (const removal of plan.evidenceRemovals) ownSource(removal.sourcePath, "evidence removal");
  const destinations = [...plan.moves.map((entry) => entry.destinationPath), ...plan.embeddedTicketRootRelocations.map((entry) => entry.destinationPath)];
  if (new Set(destinations).size !== destinations.length || destinations.some((path, index) => destinations.some((candidate, other) => index !== other && pathsOverlap(path, candidate)))) throw new Error("Plan contains duplicate or overlapping move/relocation destinations");
  const removalSources = new Set(plan.evidenceRemovals.map((entry) => entry.sourcePath));
  if (destinations.some((path) => removalSources.has(path))) throw new Error("Plan destination overlaps an evidence-removal source");
  const relocationSources = new Set(plan.embeddedTicketRootRelocations.map((entry) => entry.sourcePath));
  if (plan.moves.some((entry) => relocationSources.has(entry.destinationPath)) || plan.embeddedTicketRootRelocations.some((entry) => sourceRoles.has(entry.destinationPath))) throw new Error("Move/relocation destination overlaps another mutable source");
  if (plan.embeddedTicketRoots.some((root, index) => plan.embeddedTicketRoots.some((candidate, other) => index !== other && pathsOverlap(root.sourceMetadataRoot, candidate.sourceMetadataRoot)))) throw new Error("Embedded metadata roots overlap");
  for (const root of plan.embeddedTicketRoots) {
    const forbidden = [
      ...plan.moves.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
      ...plan.edits.map((entry) => entry.path),
      ...plan.regenerations.flatMap((entry) => [entry.cwd, ...entry.outputRoots, ...entry.inputs.map((input) => input.path), ...entry.preOutputs.map((output) => output.path), ...entry.outputs.map((output) => output.path), ...entry.staleRemovals]),
    ].find((path) => pathsOverlap(path, root.sourceMetadataRoot));
    if (forbidden) throw new Error(`Operation conflicts with embedded metadata-root ownership: ${forbidden} <> ${root.sourceMetadataRoot}`);
  }
  if (new Set(plan.symlinkTargetEdits.map((entry) => entry.sourcePath)).size !== plan.symlinkTargetEdits.length || new Set(plan.symlinkTargetEdits.map((entry) => entry.finalPath)).size !== plan.symlinkTargetEdits.length) throw new Error("Symlink target edits do not have unique source/final paths");
  for (const edit of plan.symlinkTargetEdits) {
    const move = plan.moves.filter((candidate) => candidate.sourcePath === edit.sourcePath && candidate.destinationPath === edit.finalPath);
    if (edit.sourcePath !== edit.finalPath && move.length !== 1) throw new Error(`Symlink target edit is not bound to its exact move: ${edit.sourcePath}`);
    if (plan.evidenceRemovals.some((entry) => entry.sourcePath === edit.sourcePath) || plan.embeddedTicketRootRelocations.some((entry) => entry.sourcePath === edit.sourcePath) || plan.edits.some((entry) => entry.path === edit.finalPath)) throw new Error(`Symlink target edit conflicts with another mutation: ${edit.sourcePath}`);
  }
  for (const edit of plan.edits) {
    const sourceMove = plan.moves.find((move) => move.sourcePath === edit.path && move.destinationPath !== edit.path);
    if (sourceMove || plan.evidenceRemovals.some((entry) => entry.sourcePath === edit.path) || plan.embeddedTicketRootRelocations.some((entry) => entry.sourcePath === edit.path || entry.destinationPath === edit.path)) throw new Error(`Text edit conflicts with a mutable source: ${edit.path}`);
  }
  for (const regeneration of plan.regenerations) {
    assertRegenerationContract(regeneration, taxonomy, repoRoot);
    const conflict = [...plan.moves.flatMap((move) => [move.sourcePath, move.destinationPath]), ...plan.embeddedTicketRoots.flatMap((entry) => [entry.sourceMetadataRoot, entry.sourceTicketRoot, entry.canonicalTicketRoot]), ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]), ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...removalAuthorityPaths(entry.authority)]), ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath]), ...plan.edits.map((edit) => edit.path)].find((path) => regeneration.outputRoots.some((root) => pathsOverlap(path, root)));
    if (conflict) throw new Error(`Generated output must be regenerated source-first, not moved or edited directly: ${conflict}`);
  }
  type MutablePathRole = Readonly<{ path: string; role: "move-source" | "move-destination" | "relocation-source" | "relocation-destination" | "removal-source" | "root-source" | "symlink-source" | "symlink-final" | "edit" | "generator-output"; owner: string }>;
  const roles: MutablePathRole[] = [
    ...plan.moves.flatMap((entry) => [{ path: entry.sourcePath, role: "move-source" as const, owner: entry.operationId }, { path: entry.destinationPath, role: "move-destination" as const, owner: entry.operationId }]),
    ...plan.embeddedTicketRootRelocations.flatMap((entry) => [{ path: entry.sourcePath, role: "relocation-source" as const, owner: entry.operationId }, { path: entry.destinationPath, role: "relocation-destination" as const, owner: entry.operationId }]),
    ...plan.evidenceRemovals.map((entry) => ({ path: entry.sourcePath, role: "removal-source" as const, owner: entry.operationId })),
    ...plan.embeddedTicketRoots.map((entry) => ({ path: entry.sourceMetadataRoot, role: "root-source" as const, owner: entry.operationId })),
    ...plan.symlinkTargetEdits.flatMap((entry) => [{ path: entry.sourcePath, role: "symlink-source" as const, owner: entry.operationId }, { path: entry.finalPath, role: "symlink-final" as const, owner: entry.operationId }]),
    ...[...new Set(plan.edits.map((entry) => entry.path))].map((path) => ({ path, role: "edit" as const, owner: path })),
    ...plan.regenerations.flatMap((entry) => entry.outputRoots.map((path) => ({ path, role: "generator-output" as const, owner: entry.id }))),
  ];
  const allowedOverlap = (left: MutablePathRole, right: MutablePathRole): boolean => {
    if (left.role === "edit" && right.role === "edit" && left.path === right.path) return true;
    if (left.owner === right.owner && left.path === right.path && new Set([left.role, right.role]).has("symlink-source") && new Set([left.role, right.role]).has("symlink-final")) return true;
    const pair = new Set([left.role, right.role]);
    if (pair.has("move-source") && pair.has("symlink-source") || pair.has("move-destination") && pair.has("symlink-final")) {
      const move = left.role.startsWith("move-") ? left : right;
      const symlink = left.role.startsWith("symlink-") ? left : right;
      return plan.symlinkTargetEdits.some((entry) => entry.operationId === symlink.owner && plan.moves.some((candidate) => candidate.operationId === move.owner && candidate.sourcePath === entry.sourcePath && candidate.destinationPath === entry.finalPath));
    }
    if (pair.has("move-destination") && pair.has("edit") && left.path === right.path) return true;
    if (pair.has("root-source") && (pair.has("relocation-source") || pair.has("removal-source"))) {
      const root = left.role === "root-source" ? left : right, child = left.role === "root-source" ? right : left;
      return child.path.startsWith(`${root.path}/`) && (child.role === "relocation-source" ? plan.embeddedTicketRootRelocations.some((entry) => entry.operationId === child.owner && entry.embeddedTicketRootId === root.owner) : plan.evidenceRemovals.some((entry) => entry.operationId === child.owner && entry.embeddedTicketRootId === root.owner));
    }
    return false;
  };
  for (let leftIndex = 0; leftIndex < roles.length; leftIndex++) for (let rightIndex = leftIndex + 1; rightIndex < roles.length; rightIndex++) {
    const left = roles[leftIndex], right = roles[rightIndex];
    if (pathsOverlap(left.path, right.path) && !allowedOverlap(left, right)) throw new Error(`Plan mutable path roles overlap: ${left.role}:${left.path} <> ${right.role}:${right.path}`);
  }
}

function cleanupCommittedTransaction(repoRoot: string, journal: MutableJournalRecord, plan: TaxonomyPlan, ticketRoot: string, validateOnly = false): void {
  if (journal.state !== "committed") throw new Error("Committed cleanup requires a terminal committed journal");
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot);
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  if (!lstatOrNull(stagingRoot) && !lstatOrNull(backupRoot)) return;
  const pruning = committedSourceParentPrunePaths(repoRoot, plan, journal, ticketRoot);
  const stagingStat = lstatOrNull(stagingRoot);
  if (stagingStat) {
    if (!stagingStat.isDirectory() || stagingStat.isSymbolicLink()) throw new Error("Committed staging root must be a no-follow directory");
    const expectedTop = [
      ...plan.embeddedTicketRoots.map((entry) => `root-${entry.operationId}`),
      ...plan.evidenceRemovals.map((entry) => `removal-${entry.operationId}`),
      ...plan.symlinkTargetEdits.map((entry) => `symlink-${entry.operationId}`),
    ].sort(generatorPathCompare);
    if (canonicalJson(readdirSync(stagingRoot).sort(generatorPathCompare)) !== canonicalJson(expectedTop)) throw new Error("Committed staging root has unexpected or missing operation evidence");
  }
  for (const root of plan.embeddedTicketRoots) {
    const staged = normalizeRelative(`${journal.stagingRoot}/root-${root.operationId}`);
    if (lstatOrNull(absolutePath(repoRoot, staged)) && canonicalJson(noFollowTreeDigest(repoRoot, staged)) !== canonicalJson(root.residualTreeDigest)) throw new Error(`Committed embedded root residual tree changed: ${root.operationId}`);
  }
  for (const removal of plan.evidenceRemovals) {
    const staged = join(absolutePath(repoRoot, journal.stagingRoot), `removal-${removal.operationId}`);
    if (lstatOrNull(staged) && canonicalJson(leafPreimage(staged)) !== canonicalJson(removal.preimage)) throw new Error(`Committed removal stage preimage changed: ${removal.operationId}`);
  }
  for (const edit of plan.symlinkTargetEdits) {
    const staged = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    if (lstatOrNull(staged) && (!lstatSync(staged).isSymbolicLink() || readlinkSync(staged) !== edit.oldTarget)) throw new Error(`Committed symlink stage preimage changed: ${edit.operationId}`);
  }
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (backup.kind !== "file") continue;
    const stored = join(backupRoot, backup.backupPath);
    const stat = lstatOrNull(stored);
    if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(stored) !== backup.contentHash || (stat.mode & 0o7777) !== backup.mode || stat.size !== backup.size) throw new Error(`Committed typed backup changed: ${path}`);
  }
  const backupStat = lstatOrNull(backupRoot);
  if (backupStat) {
    if (!backupStat.isDirectory() || backupStat.isSymbolicLink()) throw new Error("Committed backup root must be a no-follow directory");
    const expected = Object.values(journal.backups).filter((entry): entry is Extract<TaxonomyBackupRecord, { kind: "file" }> => entry.kind === "file").map((entry) => entry.backupPath).sort(generatorPathCompare);
    if (canonicalJson(readdirSync(backupRoot).sort(generatorPathCompare)) !== canonicalJson(expected)) throw new Error("Committed backup root has unexpected or missing evidence");
  } else if (Object.values(journal.backups).some((entry) => entry.kind === "file")) throw new Error("Committed backup root is missing frozen file evidence");
  if (!validateOnly) {
    pruneEmptySourceParents(repoRoot, pruning);
    if (stagingStat) {
      durableRemove(stagingRoot, true);
      journal.probe?.("transaction-terminal-committed-stage-removed", journal.attemptOrdinal);
    }
    if (backupStat) durableRemove(backupRoot, true);
  }
}

function cleanupRolledBackTransaction(repoRoot: string, journal: MutableJournalRecord, plan: TaxonomyPlan, validateOnly = false): void {
  if (journal.state !== "rolled-back") throw new Error("Rolled-back cleanup requires a terminal rolled-back journal");
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot);
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const stagingStat = lstatOrNull(stagingRoot);
  const backupStat = lstatOrNull(backupRoot);
  if (!stagingStat && !backupStat) return;
  if (actualAffectedPreDigest(repoRoot, plan) !== plan.expectedAffectedPreStateDigest) throw new Error("Rolled-back cleanup pre-state digest changed");
  if (stagingStat) {
    if (!stagingStat.isDirectory() || stagingStat.isSymbolicLink()) throw new Error("Rolled-back staging root is not a no-follow directory");
    if (readdirSync(stagingRoot).length > 0) throw new Error("Rolled-back staging root contains unexpected evidence");
  }
  if (backupStat) {
    if (!backupStat.isDirectory() || backupStat.isSymbolicLink()) throw new Error("Rolled-back backup root is not a no-follow directory");
    const expected = Object.values(journal.backups).filter((entry): entry is Extract<TaxonomyBackupRecord, { kind: "file" }> => entry.kind === "file").map((entry) => entry.backupPath).sort(generatorPathCompare);
    const actual = readdirSync(backupRoot).sort(generatorPathCompare);
    if (canonicalJson(actual) !== canonicalJson(expected)) throw new Error("Rolled-back backup root contains unexpected or missing evidence");
    for (const [path, backup] of Object.entries(journal.backups)) {
      if (backup.kind !== "file") continue;
      const stored = join(backupRoot, backup.backupPath);
      const stat = lstatOrNull(stored);
      if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(stored) !== backup.contentHash || (stat.mode & 0o7777) !== backup.mode || stat.size !== backup.size) throw new Error(`Rolled-back backup changed: ${path}`);
    }
  }
  if (!validateOnly) {
    if (stagingStat) {
      durableRemove(stagingRoot, true);
      journal.probe?.("transaction-terminal-rolled-back-stage-removed", journal.attemptOrdinal);
    }
    if (backupStat) durableRemove(backupRoot, true);
  }
}

function assertActiveTransactionEvidence(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, exact: boolean, backupPreparationsAlreadyClassified = false): void {
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot);
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const stage = lstatOrNull(stagingRoot), backup = lstatOrNull(backupRoot);
  if (!stage?.isDirectory() || stage.isSymbolicLink() || !backup?.isDirectory() || backup.isSymbolicLink()) throw new Error("Active transaction stage/backup roots must be direct no-follow directories");
  const allowedStage = new Set([
    ...plan.moves.map((entry) => entry.operationId),
    ...plan.embeddedTicketRootRelocations.map((entry) => `relocation-${entry.operationId}`),
    ...plan.evidenceRemovals.map((entry) => `removal-${entry.operationId}`),
    ...plan.embeddedTicketRoots.map((entry) => `root-${entry.operationId}`),
    ...plan.symlinkTargetEdits.map((entry) => `symlink-${entry.operationId}`),
    journal.journalWriteDirectory,
  ]);
  const actualStage = readdirSync(stagingRoot).sort(generatorPathCompare);
  const unexpectedStage = actualStage.find((name) => !allowedStage.has(name));
  if (unexpectedStage) throw new Error(`Active transaction staging root contains unexpected evidence: ${unexpectedStage}`);
  const expectedBackups = Object.values(journal.backups).filter((entry): entry is Extract<TaxonomyBackupRecord, { kind: "file" }> => entry.kind === "file").map((entry) => entry.backupPath).sort(generatorPathCompare);
  const actualBackups = readdirSync(backupRoot).sort(generatorPathCompare);
  if (!backupPreparationsAlreadyClassified && canonicalJson(actualBackups) !== canonicalJson(expectedBackups)) throw new Error("Active transaction backup root contains unexpected or missing evidence");
  if (!exact) return;
  const expectedStage = [
    ...plan.moves.filter((entry) => journal.stagedMoveIds.includes(entry.operationId) && !journal.installedMoveIds.includes(entry.operationId)).map((entry) => entry.operationId),
    ...plan.embeddedTicketRootRelocations.filter((entry) => journal.stagedEmbeddedRelocationIds.includes(entry.operationId) && !journal.installedEmbeddedRelocationIds.includes(entry.operationId)).map((entry) => `relocation-${entry.operationId}`),
    ...plan.evidenceRemovals.filter((entry) => journal.stagedEvidenceRemovalIds.includes(entry.operationId)).map((entry) => `removal-${entry.operationId}`),
    ...plan.embeddedTicketRoots.filter((entry) => journal.stagedEmbeddedRootIds.includes(entry.operationId)).map((entry) => `root-${entry.operationId}`),
    ...plan.symlinkTargetEdits.filter((entry) => journal.stagedSymlinkTargetEditIds.includes(entry.operationId)).map((entry) => `symlink-${entry.operationId}`),
  ].sort(generatorPathCompare);
  if (canonicalJson(actualStage) !== canonicalJson(expectedStage)) throw new Error("Active transaction staging root does not match its exact journal tuple set");
}

class TaxonomyStartedRegenerationPartialError extends Error {
  readonly regenerationId: string;
  constructor(regenerationId: string) {
    super(`resume-state-drift: started regeneration has a transaction-owned partial output tree: ${regenerationId}`);
    this.regenerationId = regenerationId;
  }
}

class TaxonomyGeneratorInputDriftError extends Error {
  readonly regenerationId: string;
  constructor(regenerationId: string, cause: unknown) {
    super(cause instanceof Error ? cause.message : String(cause), { cause });
    this.regenerationId = regenerationId;
  }
}

class TaxonomyMoveSourceInputDriftError extends Error {
  readonly operationId: string;
  constructor(operationId: string, cause: unknown) {
    super(cause instanceof Error ? cause.message : String(cause), { cause });
    this.operationId = operationId;
  }
}

function resumeGeneratorInputAuthority(plan: TaxonomyPlan, journal: MutableJournalRecord) {
  return {
    moves: new Map(plan.moves.map((entry) => [entry.sourcePath, entry])),
    relocations: new Map(plan.embeddedTicketRootRelocations.map((entry) => [entry.sourcePath, entry])),
    removals: new Map(plan.evidenceRemovals.map((entry) => [entry.sourcePath, entry])),
    roots: plan.embeddedTicketRoots,
    links: new Map(plan.symlinkTargetEdits.filter((entry) => journal.stagedSymlinkTargetEditIds.includes(entry.operationId)).map((entry) => [entry.sourcePath, entry])),
    editPaths: new Set(plan.edits.map((entry) => entry.path)),
  };
}

/** 🧭️ Resolves one logical input to its journal-proven preimage location. */
function resumeGeneratorInputPhysicalPath(authority: ReturnType<typeof resumeGeneratorInputAuthority>, journal: MutableJournalRecord, path: string): string {
  const move = authority.moves.get(path);
  const relocation = authority.relocations.get(path);
  const removal = authority.removals.get(path);
  const root = authority.roots.find((entry) => path === entry.sourceMetadataRoot || path.startsWith(`${entry.sourceMetadataRoot}/`));
  const finalPath = move?.destinationPath ?? relocation?.destinationPath ?? path;
  let current = path;
  if (move) current = journal.installedMoveIds.includes(move.operationId) ? move.destinationPath : journal.stagedMoveIds.includes(move.operationId) ? `${journal.stagingRoot}/${move.operationId}` : move.sourcePath;
  else if (relocation) current = journal.installedEmbeddedRelocationIds.includes(relocation.operationId) ? relocation.destinationPath : journal.stagedEmbeddedRelocationIds.includes(relocation.operationId) ? `${journal.stagingRoot}/relocation-${relocation.operationId}` : relocation.sourcePath;
  else if (removal && journal.stagedEvidenceRemovalIds.includes(removal.operationId)) current = `${journal.stagingRoot}/removal-${removal.operationId}`;
  else if (root && journal.stagedEmbeddedRootIds.includes(root.operationId)) current = `${journal.stagingRoot}/root-${root.operationId}${path.slice(root.sourceMetadataRoot.length)}`;
  const link = authority.links.get(path);
  if (link) current = `${journal.stagingRoot}/symlink-${link.operationId}`;
  const backup = journal.backups[finalPath];
  if (current === finalPath && backup?.kind === "file" && authority.editPaths.has(finalPath)) current = `${journal.backupRoot}/${backup.backupPath}`;
  return current;
}

/** 🧾️ Reads an input preimage at its journal-proven source, stage, destination, or edit backup after tuple validation. */
function resumeGeneratorInputRecord(repoRoot: string, authority: ReturnType<typeof resumeGeneratorInputAuthority>, journal: MutableJournalRecord, input: TaxonomyGeneratorNodeRecord, taxonomy: LoadedTaxonomy): TaxonomyGeneratorNodeRecord {
  const current = resumeGeneratorInputPhysicalPath(authority, journal, input.path);
  assertLexicalInputOutsideOpaque(repoRoot, current, "Journal-bound generator input");
  return { ...generatorNodeRecord(repoRoot, current, taxonomy), path: input.path };
}

/** 🌳️ Reconstructs logical input membership without hiding any unplanned physical child. */
function resumeGeneratorInputView(repoRoot: string, plan: TaxonomyPlan, regeneration: TaxonomyRegeneration, authority: ReturnType<typeof resumeGeneratorInputAuthority>, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): RegistryCatalogInputView {
  const native = registryCatalogInputView(repoRoot, taxonomy.discoverySchema);
  const sources = new Set([...authority.moves.keys(), ...authority.relocations.keys(), ...authority.removals.keys(), ...authority.links.keys(), ...authority.roots.map(entry => entry.sourceMetadataRoot)]);
  const children = new Map<string, Set<string>>();
  for (const source of sources) {
    const parent = posix.dirname(source) === "." ? "" : posix.dirname(source);
    const names = children.get(parent) ?? new Set<string>();
    names.add(posix.basename(source));
    children.set(parent, names);
  }
  const installed = new Set([
    ...plan.moves.filter(entry => journal.installedMoveIds.includes(entry.operationId)).map(entry => entry.destinationPath),
    ...plan.embeddedTicketRootRelocations.filter(entry => journal.installedEmbeddedRelocationIds.includes(entry.operationId)).map(entry => entry.destinationPath),
  ]);
  const created = new Set(plan.destinationAncestorPreimages.filter(entry => entry.state === "absent").map(entry => entry.path));
  const hiddenCache = new Map<string, boolean>();
  const pathFor = (path: string, name: string): string => path ? `${path}/${name}` : name;
  const hidden = (path: string): boolean => {
    if (installed.has(path) || regeneration.outputRoots.some(root => path === root || path.startsWith(`${root}/`))) return true;
    if (!created.has(path)) return false;
    if (hiddenCache.has(path)) return hiddenCache.get(path)!;
    const nodeKind = native.kind(path);
    const invisible = nodeKind === null || nodeKind === "directory" && native.entries(path).every(entry => hidden(pathFor(path, entry.name)));
    hiddenCache.set(path, invisible);
    return invisible;
  };
  const kind = (path: string): ReturnType<RegistryCatalogInputView["kind"]> => {
    const physical = resumeGeneratorInputPhysicalPath(authority, journal, path);
    if (!sources.has(path) && hidden(path)) return null;
    return native.kind(physical);
  };
  return {
    kind,
    entries(path) {
      const parentKind = kind(path);
      if (parentKind === null) return [];
      if (parentKind !== "directory") throw new Error(`Generator input directory changed kind: ${path}`);
      const entries = new Map(native.entries(resumeGeneratorInputPhysicalPath(authority, journal, path)).map(entry => [entry.name, entry.nodeKind]));
      const names = new Set([...entries.keys(), ...(children.get(path) ?? [])]);
      return [...names].sort(generatorPathCompare).flatMap(name => {
        const child = pathFor(path, name);
        if (!sources.has(child) && hidden(child)) return [];
        const nodeKind = sources.has(child) ? kind(child) : entries.get(name) ?? kind(child);
        return nodeKind === null ? [] : [{ name, nodeKind }];
      });
    },
    readText(path) { return native.readText(resumeGeneratorInputPhysicalPath(authority, journal, path)); },
  };
}

/** 🔐️ Reconciles only authenticated transaction writes and output tuples, independently of forward read inputs. */
function reconcileTransactionOwnedTuples(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): boolean {
  let reconciled = false;
  const present = (...paths: string[]): boolean[] => paths.map((path) => Boolean(lstatOrNull(path)));
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const stageRoot = absolutePath(repoRoot, journal.stagingRoot);
  const pendingEditPaths = new Set<string>();
  for (const path of new Set(plan.edits.map((entry) => entry.path))) {
    const identity = sha256(path).slice(0, 24);
    if (readdirSync(stageRoot).some((name) => splitLeadingEmoji(name).rest.startsWith(`edit-${identity}-`))) pendingEditPaths.add(path);
  }
  for (const path of [...new Set(plan.edits.map((entry) => entry.path))].sort(generatorPathCompare)) {
    if (journal.appliedEditPaths.includes(path)) continue;
    const backup = journal.backups[path];
    if (backup?.kind !== "file") continue;
    const current = absolutePath(repoRoot, path), stat = lstatOrNull(current);
    const expected = applyEditsToContent(readFileSync(join(backupRoot, backup.backupPath), "utf8"), plan.edits.filter((entry) => entry.path === path));
    if (stat?.isFile() && !stat.isSymbolicLink() && readFileSync(current, "utf8") === expected && (stat.mode & 0o7777) === backup.mode && stat.size === Buffer.byteLength(expected)) {
      journal.appliedEditPaths.push(path);
      reconciled = true;
    }
  }
  for (const move of plan.moves) {
    const source = absolutePath(repoRoot, move.sourcePath), stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination = absolutePath(repoRoot, move.destinationPath);
    const states = present(source, stage, destination);
    if (!journal.installedMoveIds.includes(move.operationId) && journal.stagedMoveIds.includes(move.operationId) && !states[0] && !states[1] && states[2] && canonicalJson(leafPreimage(destination)) === canonicalJson(move.sourcePreimage)) { journal.installedMoveIds.push(move.operationId); reconciled = true; }
    if (!journal.stagedMoveIds.includes(move.operationId) && journal.preparedMoveIds.includes(move.operationId) && !states[0] && states[1] && !states[2]) { journal.stagedMoveIds.push(move.operationId); reconciled = true; }
    const expected = journal.installedMoveIds.includes(move.operationId) ? 2 : journal.stagedMoveIds.includes(move.operationId) ? 1 : journal.preparedMoveIds.includes(move.operationId) && states[1] ? 1 : 0;
    const pendingEditExchange = expected === 2 && pendingEditPaths.has(move.destinationPath) && states.every((state) => !state);
    if (!pendingEditExchange && (states.filter(Boolean).length !== 1 || !states[expected])) throw new Error(`resume-state-drift: move ${move.operationId}`);
    if (pendingEditExchange) continue;
    const current = [source, stage, destination][expected];
    const installedLink = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath && journal.installedSymlinkTargetEditIds.includes(edit.operationId));
    if (!journal.appliedEditPaths.includes(move.destinationPath) && canonicalJson(leafPreimage(current)) !== canonicalJson(retargetedMovePreimage(move, installedLink))) throw new Error(`resume-state-drift: move preimage ${move.operationId}`);
  }
  for (const entry of plan.embeddedTicketRootRelocations) {
    const states = present(absolutePath(repoRoot, entry.sourcePath), join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${entry.operationId}`), absolutePath(repoRoot, entry.destinationPath));
    if (!journal.installedEmbeddedRelocationIds.includes(entry.operationId) && journal.stagedEmbeddedRelocationIds.includes(entry.operationId) && !states[0] && !states[1] && states[2] && canonicalJson(leafPreimage(absolutePath(repoRoot, entry.destinationPath))) === canonicalJson(entry.preimage)) { journal.installedEmbeddedRelocationIds.push(entry.operationId); reconciled = true; }
    if (!journal.stagedEmbeddedRelocationIds.includes(entry.operationId) && journal.preparedEmbeddedRelocationIds.includes(entry.operationId) && !states[0] && states[1] && !states[2]) { journal.stagedEmbeddedRelocationIds.push(entry.operationId); reconciled = true; }
    const expected = journal.installedEmbeddedRelocationIds.includes(entry.operationId) ? 2 : journal.stagedEmbeddedRelocationIds.includes(entry.operationId) ? 1 : journal.preparedEmbeddedRelocationIds.includes(entry.operationId) && states[1] ? 1 : 0;
    if (states.filter(Boolean).length !== 1 || !states[expected]) throw new Error(`resume-state-drift: embedded relocation ${entry.operationId}`);
    if (canonicalJson(leafPreimage([absolutePath(repoRoot, entry.sourcePath), join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${entry.operationId}`), absolutePath(repoRoot, entry.destinationPath)][expected])) !== canonicalJson(entry.preimage)) throw new Error(`resume-state-drift: embedded relocation preimage ${entry.operationId}`);
  }
  for (const entry of plan.evidenceRemovals) {
    const states = present(absolutePath(repoRoot, entry.sourcePath), join(absolutePath(repoRoot, journal.stagingRoot), `removal-${entry.operationId}`));
    if (!journal.stagedEvidenceRemovalIds.includes(entry.operationId) && journal.preparedEvidenceRemovalIds.includes(entry.operationId) && !states[0] && states[1]) { journal.stagedEvidenceRemovalIds.push(entry.operationId); reconciled = true; }
    const expected = journal.stagedEvidenceRemovalIds.includes(entry.operationId) ? 1 : journal.preparedEvidenceRemovalIds.includes(entry.operationId) && states[1] ? 1 : 0;
    if (states.filter(Boolean).length !== 1 || !states[expected]) throw new Error(`resume-state-drift: evidence removal ${entry.operationId}`);
    if (canonicalJson(leafPreimage([absolutePath(repoRoot, entry.sourcePath), join(absolutePath(repoRoot, journal.stagingRoot), `removal-${entry.operationId}`)][expected])) !== canonicalJson(entry.preimage)) throw new Error(`resume-state-drift: evidence removal preimage ${entry.operationId}`);
    if (entry.authority.kind === "byte-and-mode-identical") for (const member of entry.authority.members.filter((candidate) => candidate.disposition === "retain")) {
      const owningMove = plan.moves.find((move) => move.sourcePath === member.sourcePath && move.destinationPath === member.finalPath);
      const retained = owningMove
        ? journal.installedMoveIds.includes(owningMove.operationId) ? absolutePath(repoRoot, owningMove.destinationPath)
          : journal.stagedMoveIds.includes(owningMove.operationId) ? join(absolutePath(repoRoot, journal.stagingRoot), owningMove.operationId)
            : absolutePath(repoRoot, owningMove.sourcePath)
        : absolutePath(repoRoot, member.finalPath);
      if (!lstatOrNull(retained) || canonicalJson(leafPreimage(retained)) !== canonicalJson(member.preimage)) throw new Error(`resume-state-drift: retained evidence ${member.finalPath}`);
    } else if (entry.authority.kind === "serialized-path-sentinel") {
      const fixture = serializedSentinelCases(repoRoot);
      const sentinel = fixture?.cases.find((candidate) => candidate.id === entry.authority.caseId);
      if (!fixture || fixture.fixtureContentHash !== entry.authority.fixtureContentHash || !sentinel || sentinel.inputPath !== entry.authority.serializedInputPath || sentinel.physicalSourcePath !== entry.sourcePath || sentinel.expectedViolationCode !== entry.authority.expectedViolationCode || sentinel.sourceContentHash !== entry.preimage.contentHash) throw new Error(`resume-state-drift: serialized sentinel authority ${entry.operationId}`);
    } else if (entry.authority.kind === "exact-path-mutation") assertTicketImportantExactRemovalAuthority(repoRoot, entry);
    else assertTicketImportantRemovalAuthority(repoRoot, entry, taxonomy);
  }
  for (const root of plan.embeddedTicketRoots) {
    const states = present(absolutePath(repoRoot, root.sourceMetadataRoot), join(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`));
    if (!journal.stagedEmbeddedRootIds.includes(root.operationId) && journal.preparedEmbeddedRootIds.includes(root.operationId) && !states[0] && states[1]) { journal.stagedEmbeddedRootIds.push(root.operationId); reconciled = true; }
    const expected = journal.stagedEmbeddedRootIds.includes(root.operationId) ? 1 : journal.preparedEmbeddedRootIds.includes(root.operationId) && states[1] ? 1 : 0;
    if (states.filter(Boolean).length !== 1 || !states[expected]) throw new Error(`resume-state-drift: embedded root ${root.operationId}`);
    const current = states[1] ? normalizeRelative(`${journal.stagingRoot}/root-${root.operationId}`) : root.sourceMetadataRoot;
    const children = [...plan.embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath), ...plan.evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath)];
    const currentTree = states[1] ? noFollowTreeDigest(repoRoot, current) : noFollowTreeDigestExcluding(repoRoot, current, children);
    if (canonicalJson(currentTree) !== canonicalJson(root.residualTreeDigest)) throw new Error(`resume-state-drift: embedded root tree ${root.operationId}`);
  }
  for (const edit of plan.symlinkTargetEdits) {
    const link = absolutePath(repoRoot, edit.finalPath), stage = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    const linkStat = lstatOrNull(link), stageStat = lstatOrNull(stage);
    if (!journal.installedSymlinkTargetEditIds.includes(edit.operationId) && journal.stagedSymlinkTargetEditIds.includes(edit.operationId) && linkStat?.isSymbolicLink() && readlinkSync(link) === edit.newTarget && stageStat?.isSymbolicLink() && readlinkSync(stage) === edit.oldTarget) { journal.installedSymlinkTargetEditIds.push(edit.operationId); reconciled = true; }
    if (!journal.stagedSymlinkTargetEditIds.includes(edit.operationId) && journal.preparedSymlinkTargetEditIds.includes(edit.operationId) && !linkStat && stageStat?.isSymbolicLink() && readlinkSync(stage) === edit.oldTarget) { journal.stagedSymlinkTargetEditIds.push(edit.operationId); reconciled = true; }
    if (journal.installedSymlinkTargetEditIds.includes(edit.operationId)) {
      if (!linkStat?.isSymbolicLink() || readlinkSync(link) !== edit.newTarget || !stageStat?.isSymbolicLink() || readlinkSync(stage) !== edit.oldTarget) throw new Error(`resume-state-drift: symlink edit ${edit.operationId}`);
    } else if (journal.stagedSymlinkTargetEditIds.includes(edit.operationId) || (journal.preparedSymlinkTargetEditIds.includes(edit.operationId) && stageStat)) {
      if (linkStat || !stageStat?.isSymbolicLink() || readlinkSync(stage) !== edit.oldTarget) throw new Error(`resume-state-drift: symlink stage ${edit.operationId}`);
    } else if (!linkStat?.isSymbolicLink() || readlinkSync(link) !== edit.oldTarget) throw new Error(`resume-state-drift: symlink source ${edit.operationId}`);
    const targetMove = plan.moves.find((move) => move.sourcePath === edit.logicalTargetSourcePath && move.destinationPath === edit.logicalTargetFinalPath);
    const targetPath = targetMove ? journal.installedMoveIds.includes(targetMove.operationId) ? absolutePath(repoRoot, targetMove.destinationPath) : journal.stagedMoveIds.includes(targetMove.operationId) ? join(absolutePath(repoRoot, journal.stagingRoot), targetMove.operationId) : absolutePath(repoRoot, targetMove.sourcePath) : absolutePath(repoRoot, edit.logicalTargetSourcePath);
    const targetStat = lstatOrNull(targetPath);
    if (edit.logicalTargetPreimage.state === "absent") {
      if (targetStat) throw new Error(`resume-state-drift: symlink logical target ${edit.operationId}`);
    } else if (edit.logicalTargetPreimage.state === "directory") {
      if (!targetStat?.isDirectory() || targetStat.isSymbolicLink()) throw new Error(`resume-state-drift: symlink logical target ${edit.operationId}`);
    } else if (!journal.appliedEditPaths.includes(edit.logicalTargetFinalPath) && !pendingEditPaths.has(edit.logicalTargetFinalPath)) {
      const targetLinkEdit = plan.symlinkTargetEdits.find((candidate) => candidate.sourcePath === edit.logicalTargetSourcePath && candidate.finalPath === edit.logicalTargetFinalPath && journal.installedSymlinkTargetEditIds.includes(candidate.operationId));
      if (targetLinkEdit) {
        if (!targetStat?.isSymbolicLink() || readlinkSync(targetPath) !== targetLinkEdit.newTarget) throw new Error(`resume-state-drift: nested symlink logical target ${edit.operationId}`);
      } else if (!targetStat || canonicalJson(leafPathPreimage(targetPath)) !== canonicalJson(edit.logicalTargetPreimage)) throw new Error(`resume-state-drift: symlink logical target ${edit.operationId}`);
    }
  }
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (backup.kind !== "file") continue;
    const stored = join(backupRoot, backup.backupPath);
    const stat = lstatOrNull(stored);
    if (!stat?.isFile() || hashPath(stored) !== backup.contentHash || (stat.mode & 0o7777) !== backup.mode || stat.size !== backup.size) throw new Error(`resume-state-drift: backup ${path}`);
  }
  for (const path of journal.appliedEditPaths) {
    const backup = journal.backups[path];
    if (!backup || backup.kind !== "file") throw new Error(`resume-state-drift: applied edit backup ${path}`);
    const edits = plan.edits.filter((edit) => edit.path === path);
    const expected = applyEditsToContent(readFileSync(join(backupRoot, backup.backupPath), "utf8"), edits);
    const current = absolutePath(repoRoot, path);
    const stat = lstatOrNull(current);
    if (!stat?.isFile() || stat.isSymbolicLink() || readFileSync(current, "utf8") !== expected || (stat.mode & 0o7777) !== backup.mode || stat.size !== Buffer.byteLength(expected)) throw new Error(`resume-state-drift: applied edit ${path}`);
  }
  const editPaths = [...new Set(plan.edits.map((entry) => entry.path))].sort(generatorPathCompare);
  for (const path of editPaths.filter((entry) => !journal.appliedEditPaths.includes(entry))) {
    if (pendingEditPaths.has(path)) continue;
    const backup = journal.backups[path];
    if (!backup) continue;
    if (backup.kind !== "file") throw new Error(`resume-state-drift: edit backup kind ${path}`);
    const current = absolutePath(repoRoot, path);
    const stat = lstatOrNull(current);
    const edits = plan.edits.filter((entry) => entry.path === path);
    const expected = applyEditsToContent(readFileSync(join(backupRoot, backup.backupPath), "utf8"), edits);
    if (stat?.isFile() && !stat.isSymbolicLink() && readFileSync(current, "utf8") === expected && (stat.mode & 0o7777) === backup.mode && stat.size === Buffer.byteLength(expected)) {
      journal.appliedEditPaths.push(path);
      reconciled = true;
    } else if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(current) !== backup.contentHash || (stat.mode & 0o7777) !== backup.mode || stat.size !== backup.size) throw new Error(`resume-state-drift: prepared edit ${path}`);
  }
  const startedOutputs = new Set(plan.regenerations.filter((entry) => journal.startedRegenerationIds.includes(entry.id)).flatMap((entry) => entry.preOutputs.map((output) => output.path)));
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (journal.appliedEditPaths.includes(path) || startedOutputs.has(path) || backup.kind === "file") continue;
    const current = lstatOrNull(absolutePath(repoRoot, path));
    if ((backup.kind === "absent" && current) || (backup.kind === "symlink" && (!current?.isSymbolicLink() || canonicalJson(leafPreimage(absolutePath(repoRoot, path))) !== canonicalJson({ nodeKind: "symlink", contentHash: backup.targetHash, mode: backup.mode, size: backup.size, target: backup.target })))) throw new Error(`resume-state-drift: typed backup source ${path}`);
  }
  for (const regeneration of plan.regenerations) {
    const outputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
    const pre = canonicalJson(outputs) === canonicalJson(regeneration.preOutputs);
    const post = canonicalJson(outputs) === canonicalJson(regeneration.outputs);
    if (journal.completedRegenerationIds.includes(regeneration.id) && !post) throw new Error(`resume-state-drift: regeneration outputs ${regeneration.id}`);
    if (journal.startedRegenerationIds.includes(regeneration.id) && !journal.completedRegenerationIds.includes(regeneration.id) && !pre && !post) throw new TaxonomyStartedRegenerationPartialError(regeneration.id);
    if (!journal.startedRegenerationIds.includes(regeneration.id) && !pre) throw new Error(`resume-state-drift: regeneration outputs ${regeneration.id}`);
    if (journal.startedRegenerationIds.includes(regeneration.id) && !journal.completedRegenerationIds.includes(regeneration.id) && post) {
      journal.completedRegenerationIds.push(regeneration.id);
      reconciled = true;
    }
  }
  return reconciled;
}

/** 🪪️ Requires every exact current-source input at its journal-proven logical preimage location. */
function validateForwardMoveSourceInputs(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): void {
  const inputAuthority = resumeGeneratorInputAuthority(plan, journal);
  for (const move of plan.moves) {
    if (!move.sourceAuthority) continue;
    try {
      for (const input of move.sourceAuthority.inputs) {
        const frozen = { path: input.path, ...input.preimage };
        const current = resumeGeneratorInputRecord(repoRoot, inputAuthority, journal, frozen, taxonomy);
        if (canonicalJson(current) !== canonicalJson(frozen)) throw new Error(`resume-state-drift: move source authority input ${move.operationId} ${input.role} ${input.path}`);
      }
    } catch (cause) { if (isTransactionRepositoryAuthorityError(cause)) throw cause; throw new TaxonomyMoveSourceInputDriftError(move.operationId, cause); }
  }
}

/** 📥️ Requires the complete current logical generator read set before any forward continuation. */
function validateForwardGeneratorInputs(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): void {
  const inputAuthority = resumeGeneratorInputAuthority(plan, journal);
  for (const regeneration of plan.regenerations) {
    try {
      const inputs = regeneration.inputs.map((input) => resumeGeneratorInputRecord(repoRoot, inputAuthority, journal, input, taxonomy));
      if (canonicalJson(inputs) !== canonicalJson(regeneration.inputs)) throw new Error(`resume-state-drift: regeneration inputs ${regeneration.id}`);
      const contract = taxonomy.schema.generatorContracts[regeneration.contractId];
      const view = resumeGeneratorInputView(repoRoot, plan, regeneration, inputAuthority, journal, taxonomy);
      const paths = generatorInputPaths({ repoRoot }, contract, taxonomy, undefined, view);
      if (canonicalJson(paths) !== canonicalJson(regeneration.inputs.map(input => input.path))) {
        const expected = new Set(regeneration.inputs.map(input => input.path)), actual = new Set(paths);
        const missing = [...expected].filter(path => !actual.has(path)), added = paths.filter(path => !expected.has(path));
        throw new Error(`resume-state-drift: regeneration input membership ${regeneration.id}; missing(${missing.length})=${JSON.stringify(missing.slice(0, 8))}; added(${added.length})=${JSON.stringify(added.slice(0, 8))}`);
      }
    } catch (cause) { if (isTransactionRepositoryAuthorityError(cause)) throw cause; throw new TaxonomyGeneratorInputDriftError(regeneration.id, cause); }
  }
}

function validateResumeTuples(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): boolean {
  const reconciled = reconcileTransactionOwnedTuples(repoRoot, plan, journal, taxonomy);
  validateForwardMoveSourceInputs(repoRoot, plan, journal, taxonomy);
  validateForwardGeneratorInputs(repoRoot, plan, journal, taxonomy);
  return reconciled;
}

function assertJournalPhaseMembership(plan: TaxonomyPlan, journal: MutableJournalRecord): void {
  if (journal.state === "rolling-back" || journal.state === "rolled-back") return;
  const rank: Record<TaxonomyJournalState, number> = { prepared: 0, staging: 1, disposing: 2, installing: 3, retargeting: 4, editing: 5, regenerating: 6, verifying: 7, committed: 8, "rolling-back": -1, "rolled-back": -1 };
  const exact = (actual: readonly string[], expected: readonly string[], label: string): void => {
    const sorted = [...expected].sort(generatorPathCompare);
    if (canonicalJson(actual) !== canonicalJson(sorted)) throw new Error(`Resume journal ${label} is incomplete for state ${journal.state}`);
  };
  const phase = rank[journal.state];
  if (phase >= 2) {
    exact(journal.stagedMoveIds, plan.moves.map((entry) => entry.operationId), "staged moves");
    exact(journal.stagedEmbeddedRelocationIds, plan.embeddedTicketRootRelocations.map((entry) => entry.operationId), "staged embedded relocations");
    exact(journal.stagedEvidenceRemovalIds, plan.evidenceRemovals.map((entry) => entry.operationId), "staged evidence removals");
  }
  if (phase >= 3) exact(journal.stagedEmbeddedRootIds, plan.embeddedTicketRoots.map((entry) => entry.operationId), "staged embedded roots");
  if (phase >= 4) {
    exact(journal.installedMoveIds, plan.moves.map((entry) => entry.operationId), "installed moves");
    exact(journal.installedEmbeddedRelocationIds, plan.embeddedTicketRootRelocations.map((entry) => entry.operationId), "installed embedded relocations");
  }
  if (phase >= 5) exact(journal.installedSymlinkTargetEditIds, plan.symlinkTargetEdits.map((entry) => entry.operationId), "installed symlink target edits");
  if (phase >= 6) exact(journal.appliedEditPaths, [...new Set(plan.edits.map((entry) => entry.path))], "applied edit paths");
  if (phase >= 7) exact(journal.completedRegenerationIds, plan.regenerations.map((entry) => entry.id), "completed regenerations");
}

function assertJournalPlanMembership(plan: TaxonomyPlan, journal: MutableJournalRecord): void {
  if (journal.sourceParentPrunePaths.some((path) => !plan.moves.some((move) => move.sourcePath.startsWith(`${path}/`)) || plan.moves.some((move) => move.destinationPath.startsWith(`${path}/`)))) throw new Error("Resume journal source-parent pruning does not match the plan");
  const subset = (child: readonly string[], parent: readonly string[]): boolean => child.every((id) => parent.includes(id));
  const exactPlanIds = (ids: readonly string[], records: readonly { readonly operationId: string }[]): boolean => ids.every((id) => records.some((record) => record.operationId === id));
  if (!subset(journal.stagedMoveIds, journal.preparedMoveIds) || !subset(journal.installedMoveIds, journal.stagedMoveIds) || !exactPlanIds(journal.preparedMoveIds, plan.moves) || !subset(journal.stagedEmbeddedRelocationIds, journal.preparedEmbeddedRelocationIds) || !subset(journal.installedEmbeddedRelocationIds, journal.stagedEmbeddedRelocationIds) || !exactPlanIds(journal.preparedEmbeddedRelocationIds, plan.embeddedTicketRootRelocations) || !subset(journal.stagedEvidenceRemovalIds, journal.preparedEvidenceRemovalIds) || !exactPlanIds(journal.preparedEvidenceRemovalIds, plan.evidenceRemovals) || !subset(journal.stagedEmbeddedRootIds, journal.preparedEmbeddedRootIds) || !exactPlanIds(journal.preparedEmbeddedRootIds, plan.embeddedTicketRoots) || !subset(journal.stagedSymlinkTargetEditIds, journal.preparedSymlinkTargetEditIds) || !subset(journal.installedSymlinkTargetEditIds, journal.stagedSymlinkTargetEditIds) || !exactPlanIds(journal.preparedSymlinkTargetEditIds, plan.symlinkTargetEdits) || !subset(journal.completedRegenerationIds, journal.startedRegenerationIds) || journal.startedRegenerationIds.some((id) => !plan.regenerations.some((record) => record.id === id)) || journal.appliedEditPaths.some((path) => !plan.edits.some((edit) => edit.path === path))) throw new Error("Resume journal operation state does not match the plan");
}

function assertJournalBackupAuthority(plan: TaxonomyPlan, journal: MutableJournalRecord): void {
  const editPaths = new Set(plan.edits.map((entry) => entry.path));
  const generatorPreimages = new Map(plan.regenerations.filter((entry) => journal.state === "rolled-back" || journal.startedRegenerationIds.includes(entry.id)).flatMap((entry) => entry.preOutputs.filter((output) => output.nodeKind !== "directory").map((output) => [output.path, output] as const)));
  const editBackupsAllowed = ["editing", "regenerating", "verifying", "committed", "rolling-back", "rolled-back"].includes(journal.state);
  const seenStored = new Set<string>();
  for (const [path, backup] of Object.entries(journal.backups)) {
    const generatorPreimage = generatorPreimages.get(path);
    if (!editPaths.has(path) && !generatorPreimage) throw new Error(`Resume journal has an unauthorized backup path: ${path}`);
    if (editPaths.has(path) && !editBackupsAllowed) throw new Error(`Resume journal contains a reference-edit backup before the editing phase: ${path}`);
    if (editPaths.has(path) && backup.kind !== "file") throw new Error(`Reference-edit backup must be a regular file: ${path}`);
    if (editPaths.has(path)) {
      const preimages = new Map(plan.edits.filter((entry) => entry.path === path).map((entry) => [canonicalJson(entry.preimage), entry.preimage]));
      const preimage = [...preimages.values()][0];
      if (preimages.size !== 1 || !preimage || backup.kind !== "file" || backup.contentHash !== preimage.contentHash || backup.mode !== preimage.mode || backup.size !== preimage.size) throw new Error(`Reference-edit backup does not match its frozen preimage: ${path}`);
    }
    if (generatorPreimage) {
      const matches = generatorPreimage.nodeKind === "file"
        ? backup.kind === "file" && backup.contentHash === generatorPreimage.contentHash && backup.mode === generatorPreimage.mode && backup.size === generatorPreimage.size
        : generatorPreimage.nodeKind === "symlink" ? backup.kind === "symlink" && backup.targetHash === generatorPreimage.contentHash && backup.mode === generatorPreimage.mode && backup.size === generatorPreimage.size && backup.target === generatorPreimage.target : false;
      if (!matches) throw new Error(`Generator backup does not match its frozen preOutput: ${path}`);
    }
    if (backup.kind !== "file") continue;
    const expected = `${sha256(path).slice(0, 24)}.backup`;
    if (backup.backupPath !== expected || seenStored.has(backup.backupPath)) throw new Error(`Resume journal backup storage identity is invalid: ${path}`);
    seenStored.add(backup.backupPath);
  }
  for (const path of journal.appliedEditPaths) if (!journal.backups[path]) throw new Error(`Applied reference edit has no frozen typed backup: ${path}`);
  for (const regeneration of plan.regenerations.filter((entry) => journal.startedRegenerationIds.includes(entry.id))) {
    for (const output of regeneration.preOutputs.filter((entry) => entry.nodeKind !== "directory")) if (!journal.backups[output.path]) throw new Error(`Started regeneration has no frozen typed backup: ${regeneration.id}:${output.path}`);
  }
}

type TaxonomyBackupExpectedPreimage =
  | Readonly<{ nodeKind: "file"; contentHash: string; mode: number; size: number }>
  | Readonly<{ nodeKind: "symlink"; contentHash: string; mode: number; size: number; target: string }>;

function transactionBackupAuthorities(plan: TaxonomyPlan): ReadonlyMap<string, Readonly<{ path: string; expected: TaxonomyBackupExpectedPreimage; edit: boolean; regenerationIds: readonly string[] }>> {
  const byPath = new Map<string, { path: string; expected: TaxonomyBackupExpectedPreimage; edit: boolean; regenerationIds: string[] }>();
  const add = (path: string, expected: TaxonomyBackupExpectedPreimage, edit: boolean, regenerationId?: string): void => {
    const prior = byPath.get(path);
    if (prior) {
      if (canonicalJson(prior.expected) !== canonicalJson(expected)) throw new Error(`Transaction backup path has incompatible frozen authorities: ${path}`);
      prior.edit ||= edit;
      if (regenerationId && !prior.regenerationIds.includes(regenerationId)) prior.regenerationIds.push(regenerationId);
      return;
    }
    byPath.set(path, { path, expected, edit, regenerationIds: regenerationId ? [regenerationId] : [] });
  };
  for (const edit of plan.edits) add(edit.path, edit.preimage, true);
  for (const regeneration of plan.regenerations) for (const output of regeneration.preOutputs) if (output.nodeKind !== "directory") add(output.path, output, false, regeneration.id);
  const byIdentity = new Map<string, Readonly<{ path: string; expected: TaxonomyBackupExpectedPreimage; edit: boolean; regenerationIds: readonly string[] }>>();
  for (const authority of byPath.values()) {
    const identity = sha256(authority.path).slice(0, 24);
    const prior = byIdentity.get(identity);
    if (prior && prior.path !== authority.path) throw new Error(`Transaction backup storage identity collision: ${prior.path} <> ${authority.path}`);
    byIdentity.set(identity, authority);
  }
  return byIdentity;
}

function expectedBackupRecord(path: string, expected: TaxonomyBackupExpectedPreimage): TaxonomyBackupRecord {
  if (expected.nodeKind === "symlink") {
    if (sha256(expected.target) !== expected.contentHash || Buffer.byteLength(expected.target) !== expected.size) throw new Error(`Frozen symlink backup preimage is incomplete: ${path}`);
    return { kind: "symlink", target: expected.target, targetHash: expected.contentHash, mode: expected.mode, size: expected.size };
  }
  return { kind: "file", backupPath: `${sha256(path).slice(0, 24)}.backup`, contentHash: expected.contentHash, mode: expected.mode, size: expected.size };
}

function assertStoredFileBackup(path: string, record: Extract<TaxonomyBackupRecord, { kind: "file" }>): void {
  const stat = lstatOrNull(path);
  if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(path) !== record.contentHash || (stat.mode & 0o7777) !== record.mode || stat.size !== record.size) throw new Error(`Stored transaction backup does not match its frozen preimage: ${path}`);
}

interface TransactionBinaryWritePreparation {
  readonly root: string;
  readonly leaf?: string;
}

function transactionBinaryWritePreparations(container: string, preparationName: (pid: number, token: string) => string, candidateName: string): readonly TransactionBinaryWritePreparation[] {
  const preparations: TransactionBinaryWritePreparation[] = [];
  for (const name of readdirSync(container).sort(generatorPathCompare)) {
    const match = /^write-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match) continue;
    const pid = Number.parseInt(match[1], 10), token = match[2];
    if (!Number.isSafeInteger(pid) || !TRANSACTION_LEASE_TOKEN.test(token) || name !== preparationName(pid, token)) throw new Error(`Transaction binary write preparation name is invalid: ${name}`);
    const root = join(container, name), stat = lstatOrNull(root);
    if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction binary write preparation must be a no-follow directory: ${name}`);
    const children = readdirSync(root).sort(generatorPathCompare);
    if (children.length > 1 || children.length === 1 && children[0] !== candidateName) throw new Error(`Transaction binary write preparation contains unexpected evidence: ${name}`);
    const leaf = children.length === 1 ? join(root, candidateName) : undefined;
    const leafStat = leaf ? lstatOrNull(leaf) : undefined;
    if (leaf && (!leafStat?.isFile() || leafStat.isSymbolicLink())) throw new Error(`Transaction binary write candidate must be a regular no-follow file: ${leaf}`);
    preparations.push({ root, leaf });
  }
  if (preparations.length > 1) throw new Error(`Transaction binary writer has duplicate preparations: ${container}`);
  return preparations;
}

function writeTransactionBinaryCandidate(container: string, preparationName: (pid: number, token: string) => string, candidateName: string, bytes: Buffer, mode: number, probe?: (phase: string, path?: string) => void, phasePrefix = "transaction-binary", logicalPath?: string): TransactionBinaryWritePreparation {
  const root = join(container, preparationName(process.pid, randomUUID()));
  mkdirSync(root);
  fsyncDirectory(container);
  probe?.(`${phasePrefix}-write-mkdir`, logicalPath ?? root);
  const leaf = join(root, candidateName), descriptor = openSync(leaf, "wx", mode);
  try {
    const midpoint = Math.ceil(bytes.byteLength / 2);
    if (midpoint > 0) writeSync(descriptor, bytes.subarray(0, midpoint));
    fsyncSync(descriptor);
    probe?.(`${phasePrefix}-write-mid`, logicalPath ?? leaf);
    if (midpoint < bytes.byteLength) writeSync(descriptor, bytes.subarray(midpoint));
    fsyncSync(descriptor);
  } finally { closeSync(descriptor); }
  chmodSync(leaf, mode);
  fsyncFile(leaf);
  fsyncDirectory(root);
  return { root, leaf };
}

function backupPath(repoRoot: string, logicalPath: string, backupRoot: string, journal: MutableJournalRecord, expected: TaxonomyBackupExpectedPreimage, preparationName: (identity: string, pid: number, token: string) => string, writePreparationName: (pid: number, token: string) => string, writeCandidateName: string): void {
  if (journal.backups[logicalPath] !== undefined) return;
  const source = absolutePath(repoRoot, logicalPath);
  const stat = lstatOrNull(source);
  if (!stat) throw new Error(`Backup source is absent: ${logicalPath}`);
  if (stat.isSymbolicLink()) {
    const target = readlinkSync(source);
    const actual: TaxonomyBackupExpectedPreimage = { nodeKind: "symlink", contentHash: sha256(target), mode: stat.mode & 0o7777, size: Buffer.byteLength(target), target };
    if (expected.nodeKind !== "symlink" || canonicalJson(actual) !== canonicalJson(expected)) throw new Error(`Backup source changed from its frozen symlink preimage: ${logicalPath}`);
    journal.backups[logicalPath] = expectedBackupRecord(logicalPath, expected);
    return;
  }
  if (!stat.isFile() || expected.nodeKind !== "file") throw new Error(`Backup target kind changed: ${logicalPath}`);
  const bytes = readFileSync(source);
  const record = expectedBackupRecord(logicalPath, expected);
  if (record.kind !== "file" || sha256(bytes) !== record.contentHash || bytes.byteLength !== record.size || (stat.mode & 0o7777) !== record.mode) throw new Error(`Backup source changed from its frozen file preimage: ${logicalPath}`);
  const token = randomUUID();
  const candidate = join(backupRoot, preparationName(record.backupPath.slice(0, 24), process.pid, token));
  const candidateLeaf = join(candidate, record.backupPath);
  const destination = join(backupRoot, record.backupPath);
  mkdirSync(candidate);
  fsyncDirectory(backupRoot);
  const writer = writeTransactionBinaryCandidate(candidate, writePreparationName, writeCandidateName, bytes, record.mode, journal.probe, "transaction-backup", logicalPath);
  assertStoredFileBackup(writer.leaf!, record);
  journal.probe?.("transaction-backup-write-prepared", logicalPath);
  const sourceAfter = leafPreimage(source);
  if (sourceAfter.nodeKind !== "file" || sourceAfter.contentHash !== record.contentHash || sourceAfter.mode !== record.mode || sourceAfter.size !== record.size) throw new Error(`Backup source changed during frozen snapshot publication: ${logicalPath}`);
  durableRename(writer.leaf!, candidateLeaf);
  journal.probe?.("transaction-backup-inner-exchange", logicalPath);
  durableRemove(writer.root, true);
  journal.probe?.("transaction-backup-exchange", logicalPath);
  const published = lstatOrNull(destination);
  if (published) assertStoredFileBackup(destination, record);
  else {
    try { linkSync(candidateLeaf, destination); }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
      assertStoredFileBackup(destination, record);
    }
    fsyncDirectory(backupRoot);
  }
  journal.probe?.("transaction-backup-retained", logicalPath);
  durableRemove(candidate, true);
  journal.backups[logicalPath] = record;
}

function recoverTransactionBackups(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, preparationName: (identity: string, pid: number, token: string) => string, writePreparationName: (pid: number, token: string) => string, writeCandidateName: string, validateOnly = false): boolean {
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const authorities = transactionBackupAuthorities(plan);
  let changed = false;
  const rootChildren = readdirSync(backupRoot).sort(generatorPathCompare);
  const recordForStored = (identity: string, path: string): Extract<TaxonomyBackupRecord, { kind: "file" }> => {
    const authority = authorities.get(identity);
    if (!authority || authority.expected.nodeKind !== "file") throw new Error(`Transaction backup evidence has no unique frozen file authority: ${basename(path)}`);
    const stat = lstatOrNull(path);
    const record: Extract<TaxonomyBackupRecord, { kind: "file" }> = { kind: "file", backupPath: `${identity}.backup`, contentHash: authority.expected.contentHash, mode: authority.expected.mode, size: stat?.size ?? -1 };
    if (!stat?.isFile() || stat.isSymbolicLink() || authority.expected.size !== undefined && stat.size !== authority.expected.size) throw new Error(`Transaction backup evidence is not a regular exact-size file: ${path}`);
    assertStoredFileBackup(path, record);
    return record;
  };
  const candidates: { root: string; leaf?: string; writer?: TransactionBinaryWritePreparation; identity: string; record: Extract<TaxonomyBackupRecord, { kind: "file" }>; discard: boolean }[] = [];
  const records = new Map<string, Extract<TaxonomyBackupRecord, { kind: "file" }>>();
  for (const name of rootChildren) {
    const rest = splitLeadingEmoji(name).rest;
    const final = /^([0-9a-f]{24})\.backup$/u.exec(name);
    if (final) {
      const record = recordForStored(final[1], join(backupRoot, name));
      records.set(final[1], record);
      continue;
    }
    const match = /^backup-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(match[3])) {
      if (rest.startsWith("restore-") || rest.startsWith("lease-")) continue;
      throw new Error(`Transaction backup root contains unauthorized evidence: ${name}`);
    }
    const identity = match[1], pid = Number.parseInt(match[2], 10), token = match[3];
    if (name !== preparationName(identity, pid, token)) throw new Error(`Transaction backup preparation is not canonical: ${name}`);
    if (candidates.some((entry) => entry.identity === identity)) throw new Error(`Transaction backup root contains duplicate candidates for ${identity}`);
    const candidate = join(backupRoot, name);
    const candidateStat = lstatOrNull(candidate);
    if (!candidateStat?.isDirectory() || candidateStat.isSymbolicLink()) throw new Error(`Transaction backup preparation must be a no-follow directory: ${name}`);
    const expectedLeaf = `${identity}.backup`;
    const writers = transactionBinaryWritePreparations(candidate, writePreparationName, writeCandidateName);
    const writerNames = new Set(writers.map((entry) => basename(entry.root)));
    const candidateChildren = readdirSync(candidate).sort(generatorPathCompare);
    if (candidateChildren.some((child) => child !== expectedLeaf && !writerNames.has(child))) throw new Error(`Transaction backup preparation has unexpected evidence: ${name}`);
    const candidateLeaf = join(candidate, expectedLeaf);
    const authority = authorities.get(identity);
    if (!authority || authority.expected.nodeKind !== "file") throw new Error(`Transaction backup preparation has no frozen file authority: ${name}`);
    const record = expectedBackupRecord(authority.path, authority.expected);
    if (record.kind !== "file") throw new Error(`Transaction backup preparation kind is invalid: ${name}`);
    const outerPresent = Boolean(lstatOrNull(candidateLeaf));
    if (outerPresent) assertStoredFileBackup(candidateLeaf, record);
    const writer = writers[0];
    if (outerPresent && writer?.leaf) throw new Error(`Transaction backup preparation has duplicate outer and nested candidates: ${name}`);
    const writerExact = Boolean(writer?.leaf && (() => { try { assertStoredFileBackup(writer.leaf!, record); return true; } catch (error) { if (isTransactionRepositoryAuthorityError(error)) throw error; return false; } })());
    if (!outerPresent && !writerExact) {
      const current = absolutePath(repoRoot, authority.path), currentStat = lstatOrNull(current);
      const pre = currentStat?.isFile() && !currentStat.isSymbolicLink() && hashPath(current) === record.contentHash && (currentStat.mode & 0o7777) === record.mode && currentStat.size === record.size;
      if (!pre) throw new Error(`Incomplete transaction backup writer has no exact source preimage: ${name}`);
      candidates.push({ root: candidate, writer, identity, record, discard: true });
      continue;
    }
    const destination = join(backupRoot, expectedLeaf);
    if (lstatOrNull(destination)) {
      if (!outerPresent) throw new Error(`Published transaction backup has an unreachable preparation without its outer candidate: ${name}`);
      assertStoredFileBackup(destination, record);
      const finalRecord = records.get(identity) ?? recordForStored(identity, destination);
      if (canonicalJson(finalRecord) !== canonicalJson(record)) throw new Error(`Transaction backup candidate differs from its published leaf: ${identity}`);
      records.set(identity, finalRecord);
    } else records.set(identity, record);
    candidates.push({ root: candidate, leaf: outerPresent ? candidateLeaf : writer!.leaf, writer, identity, record, discard: false });
  }
  for (const [identity, record] of records) {
    const authority = authorities.get(identity);
    if (!authority) throw new Error(`Transaction backup leaf has no unique plan authority: ${identity}.backup`);
    const prior = journal.backups[authority.path];
    if (prior) {
      if (canonicalJson(prior) !== canonicalJson(record)) throw new Error(`Transaction backup journal evidence differs from its stored leaf: ${authority.path}`);
      continue;
    }
    if (!(authority.edit && journal.state === "editing") && !(authority.regenerationIds.length > 0 && journal.state === "regenerating")) throw new Error(`Transaction backup orphan is unreachable in journal state ${journal.state}: ${authority.path}`);
    const current = absolutePath(repoRoot, authority.path);
    const currentStat = lstatOrNull(current);
    const pre = currentStat?.isFile() && !currentStat.isSymbolicLink() && hashPath(current) === record.contentHash && (currentStat.mode & 0o7777) === record.mode && currentStat.size === record.size;
    if (!pre) throw new Error(`Transaction backup orphan source is not its frozen preimage: ${authority.path}`);
  }
  if (!validateOnly) for (const candidate of candidates) {
      if (candidate.discard) {
        durableRemove(candidate.root, true);
        continue;
      }
      const outerLeaf = join(candidate.root, candidate.record.backupPath);
      if (!lstatOrNull(outerLeaf)) durableRename(candidate.leaf!, outerLeaf);
      if (candidate.writer && lstatOrNull(candidate.writer.root)) durableRemove(candidate.writer.root, true);
      const destination = join(backupRoot, candidate.record.backupPath);
      if (!lstatOrNull(destination)) {
        try { linkSync(outerLeaf, destination); }
        catch (error) {
          if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
          assertStoredFileBackup(destination, candidate.record);
        }
        fsyncDirectory(backupRoot);
      }
      durableRemove(candidate.root, true);
    }
  for (const [identity, record] of records) {
    const authority = authorities.get(identity)!;
    if (!journal.backups[authority.path] && !validateOnly) {
      journal.backups[authority.path] = record;
      changed = true;
    } else if (!journal.backups[authority.path]) changed = true;
  }
  for (const regeneration of plan.regenerations) {
    const outputs = regeneration.preOutputs.filter((entry) => entry.nodeKind !== "directory");
    const hasOrphan = outputs.some((entry) => journal.backups[entry.path] !== undefined);
    if (!hasOrphan || journal.startedRegenerationIds.includes(regeneration.id)) continue;
    if (journal.state !== "regenerating") throw new Error(`Transaction generator backup orphan is outside its regenerating phase: ${regeneration.id}`);
    if (!validateOnly) {
      for (const output of outputs) backupPath(repoRoot, output.path, backupRoot, journal, output, preparationName, writePreparationName, writeCandidateName);
      journal.startedRegenerationIds.push(regeneration.id);
    }
    changed = true;
  }
  return changed;
}

function restoreBackup(repoRoot: string, plan: TaxonomyPlan, logicalPath: string, backupRoot: string, encoded: TaxonomyBackupRecord, preparationName: (identity: string, pid: number, token: string) => string, probe?: (phase: string, path?: string) => void): void {
  const destination = absolutePath(repoRoot, logicalPath);
  mkdirSync(dirname(destination), { recursive: true });
  const current = lstatOrNull(destination);
  if (encoded.kind === "absent") {
    if (current?.isDirectory()) throw new Error(`Cannot remove directory while restoring absent backup: ${logicalPath}`);
    if (current) durableRemove(destination);
    return;
  }
  if (current?.isDirectory()) throw new Error(`Cannot replace directory while restoring backup: ${logicalPath}`);
  const identity = sha256(logicalPath).slice(0, 24), token = randomUUID();
  const candidateRoot = join(backupRoot, preparationName(identity, process.pid, token));
  const candidateLeaf = join(candidateRoot, `${identity}.backup`);
  const postLeaf = join(candidateRoot, `${identity}.post`);
  mkdirSync(candidateRoot);
  fsyncDirectory(backupRoot);
  probe?.("transaction-restore-mkdir", logicalPath);
  if (encoded.kind === "symlink") {
    if (encoded.targetHash !== sha256(encoded.target)) throw new Error(`Symlink backup target hash changed: ${logicalPath}`);
    symlinkSync(encoded.target, candidateLeaf);
    fsyncDirectory(candidateRoot);
  } else {
    const source = join(backupRoot, encoded.backupPath);
    assertStoredFileBackup(source, encoded);
    copyFileSync(source, candidateLeaf);
    chmodSync(candidateLeaf, encoded.mode);
    fsyncFile(candidateLeaf);
    fsyncDirectory(candidateRoot);
  }
  probe?.("transaction-restore-prepared", logicalPath);
  const candidatePreimage = leafPreimage(candidateLeaf);
  const expected = encoded.kind === "symlink" ? { nodeKind: "symlink", contentHash: encoded.targetHash, mode: encoded.mode, size: encoded.size, target: encoded.target } as TaxonomyLeafPreimage : { nodeKind: "file", contentHash: encoded.contentHash, mode: encoded.mode, size: encoded.size } as TaxonomyLeafPreimage;
  if (canonicalJson(candidatePreimage) !== canonicalJson(expected)) throw new Error(`Restore candidate does not match its typed backup: ${logicalPath}`);
  if (current && canonicalJson(leafPreimage(destination)) === canonicalJson(expected)) { durableRemove(candidateRoot, true); return; }
  if (current) {
    const edits = plan.edits.filter((entry) => entry.path === logicalPath);
    const transactionOwned = edits.length > 0 && encoded.kind === "file" && (() => {
      const rendered = applyEditsToContent(readFileSync(join(backupRoot, encoded.backupPath), "utf8"), edits);
      return current.isFile() && !current.isSymbolicLink() && readFileSync(destination, "utf8") === rendered && (current.mode & 0o7777) === encoded.mode;
    })();
    if (!transactionOwned) throw new Error(`Restore destination is not an exact transaction-owned postimage: ${logicalPath}`);
    durableRename(destination, postLeaf);
    probe?.("transaction-restore-exchange", logicalPath);
  }
  durableRename(candidateLeaf, destination);
  probe?.("transaction-restore-canonical-exchange", logicalPath);
  if (lstatOrNull(postLeaf)) durableRemove(postLeaf);
  durableRemove(candidateRoot, true);
}

function recoverRestorePreparations(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, preparationName: (identity: string, pid: number, token: string) => string, validateOnly = false): void {
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const byIdentity = new Map<string, { path: string; backup: TaxonomyBackupRecord }>();
  for (const [path, backup] of Object.entries(journal.backups)) {
    const identity = sha256(path).slice(0, 24);
    if (byIdentity.has(identity)) throw new Error(`Restore preparation identity is ambiguous: ${identity}`);
    byIdentity.set(identity, { path, backup });
  }
  const actions: { root: string; backupLeaf?: string; postLeaf?: string; destination: string; destinationPre: boolean; destinationPost: boolean }[] = [];
  for (const name of readdirSync(backupRoot).sort(generatorPathCompare)) {
    const rest = splitLeadingEmoji(name).rest;
    if (!rest.startsWith("restore-")) continue;
    const match = /^restore-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(match[3]) || name !== preparationName(match[1], Number.parseInt(match[2], 10), match[3])) throw new Error(`Restore preparation name is invalid: ${name}`);
    const authority = byIdentity.get(match[1]);
    if (!authority || journal.state !== "rolling-back") throw new Error(`Restore preparation has no rolling-back journal authority: ${name}`);
    const root = join(backupRoot, name), leaf = join(root, `${match[1]}.backup`), postLeaf = join(root, `${match[1]}.post`), destination = absolutePath(repoRoot, authority.path);
    const stat = lstatOrNull(root);
    const children = stat?.isDirectory() && !stat.isSymbolicLink() ? readdirSync(root).sort(generatorPathCompare) : [];
    if (!stat?.isDirectory() || stat.isSymbolicLink() || children.some((child) => child !== basename(leaf) && child !== basename(postLeaf)) || new Set(children).size !== children.length) throw new Error(`Restore preparation has incomplete or unexpected evidence: ${name}`);
    const backupPresent = children.includes(basename(leaf)), postPresent = children.includes(basename(postLeaf));
    const candidate = backupPresent ? leafPreimage(leaf) : undefined;
    const expected = authority.backup.kind === "file" ? { nodeKind: "file", contentHash: authority.backup.contentHash, mode: authority.backup.mode, size: authority.backup.size } as TaxonomyLeafPreimage : authority.backup.kind === "symlink" ? { nodeKind: "symlink", contentHash: authority.backup.targetHash, mode: authority.backup.mode, size: authority.backup.size, target: authority.backup.target } as TaxonomyLeafPreimage : undefined;
    if (!expected || candidate && canonicalJson(candidate) !== canonicalJson(expected)) throw new Error(`Restore preparation bytes differ from journal backup authority: ${name}`);
    const currentStat = lstatOrNull(destination);
    const restored = Boolean(currentStat && !currentStat.isDirectory() && canonicalJson(leafPreimage(destination)) === canonicalJson(expected));
    let transactionOwned = false;
    const edits = plan.edits.filter((entry) => entry.path === authority.path);
    if (edits.length > 0 && currentStat?.isFile() && !currentStat.isSymbolicLink() && authority.backup.kind === "file") {
      const rendered = applyEditsToContent(readFileSync(join(backupRoot, authority.backup.backupPath), "utf8"), edits);
      transactionOwned = readFileSync(destination, "utf8") === rendered && (currentStat.mode & 0o7777) === authority.backup.mode;
    }
    if (postPresent) {
      if (edits.length === 0 || authority.backup.kind !== "file") throw new Error(`Restore postimage has no exact edit authority: ${name}`);
      const rendered = applyEditsToContent(readFileSync(join(backupRoot, authority.backup.backupPath), "utf8"), edits), post = lstatOrNull(postLeaf);
      if (!post?.isFile() || post.isSymbolicLink() || readFileSync(postLeaf, "utf8") !== rendered || (post.mode & 0o7777) !== authority.backup.mode) throw new Error(`Restore postimage differs from transaction output: ${name}`);
    }
    const startedGeneratorAbsent = !currentStat && plan.regenerations.some((entry) => journal.startedRegenerationIds.includes(entry.id) && entry.preOutputs.some((output) => output.path === authority.path));
    const stateValid = !backupPresent && !postPresent ? transactionOwned || restored || startedGeneratorAbsent
      : backupPresent && postPresent ? !currentStat
        : backupPresent ? transactionOwned || !currentStat || restored
          : restored;
    if (!stateValid) throw new Error(`Restore preparation has an impossible exchange tuple: ${name}`);
    actions.push({ root, backupLeaf: backupPresent ? leaf : undefined, postLeaf: postPresent ? postLeaf : undefined, destination, destinationPre: restored, destinationPost: transactionOwned });
  }
  if (!validateOnly) for (const action of actions) {
    if (action.backupLeaf && !action.destinationPre) {
      if (action.destinationPost && !action.postLeaf) {
        const post = join(action.root, basename(action.backupLeaf).replace(/\.backup$/u, ".post"));
        durableRename(action.destination, post);
        action.postLeaf = post;
      }
      mkdirSync(dirname(action.destination), { recursive: true });
      durableRename(action.backupLeaf, action.destination);
    }
    if (action.postLeaf && lstatOrNull(action.postLeaf)) durableRemove(action.postLeaf);
    durableRemove(action.root, true);
  }
}

function referenceEditResult(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, path: string): Readonly<{ bytes: Buffer; preimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }> }> {
  const edits = plan.edits.filter((entry) => entry.path === path);
  const preimages = new Map(edits.map((entry) => [canonicalJson(entry.preimage), entry.preimage]));
  const preimage = [...preimages.values()][0];
  const backup = journal.backups[path];
  if (preimages.size !== 1 || !preimage || backup?.kind !== "file" || canonicalJson(backup) !== canonicalJson(expectedBackupRecord(path, preimage))) throw new Error(`Reference edit lacks one exact frozen preimage and backup: ${path}`);
  return { bytes: Buffer.from(applyEditsToContent(readFileSync(join(absolutePath(repoRoot, journal.backupRoot), backup.backupPath), "utf8"), edits)), preimage };
}

function applyReferenceEditAtomically(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, path: string, preparationName: (identity: string, pid: number, token: string) => string, writePreparationName: (pid: number, token: string) => string, writeCandidateName: string): void {
  const target = absolutePath(repoRoot, path), identity = sha256(path).slice(0, 24), result = referenceEditResult(repoRoot, plan, journal, path);
  if (canonicalJson(leafPreimage(target)) !== canonicalJson(result.preimage)) throw new Error(`Reference edit preimage changed: ${path}`);
  const root = join(absolutePath(repoRoot, journal.stagingRoot), preparationName(identity, process.pid, randomUUID()));
  const leaf = join(root, `${identity}.edit`);
  const preLeaf = join(root, `${identity}.pre`);
  mkdirSync(root);
  fsyncDirectory(dirname(root));
  const writer = writeTransactionBinaryCandidate(root, writePreparationName, writeCandidateName, result.bytes, result.preimage.mode, journal.probe, "transaction-edit", path);
  const candidate = leafPreimage(writer.leaf!);
  if (candidate.nodeKind !== "file" || candidate.contentHash !== sha256(result.bytes) || candidate.mode !== result.preimage.mode || candidate.size !== result.bytes.byteLength) throw new Error(`Reference edit candidate differs from rendered bytes: ${path}`);
  if (canonicalJson(leafPreimage(target)) !== canonicalJson(result.preimage)) throw new Error(`Reference edit source changed during candidate publication: ${path}`);
  journal.probe?.("transaction-edit-write-prepared", path);
  durableRename(writer.leaf!, leaf);
  journal.probe?.("transaction-edit-inner-exchange", path);
  durableRemove(writer.root, true);
  durableRename(target, preLeaf);
  journal.probe?.("transaction-edit-exchange", path);
  durableRename(leaf, target);
  journal.probe?.("transaction-edit-canonical-exchange", path);
  durableRemove(preLeaf);
  durableRemove(root, true);
}

function recoverReferenceEditPreparations(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, preparationName: (identity: string, pid: number, token: string) => string, writePreparationName: (pid: number, token: string) => string, writeCandidateName: string, validateOnly = false): void {
  const stageRoot = absolutePath(repoRoot, journal.stagingRoot);
  const editPaths = new Map<string, string>();
  for (const path of new Set(plan.edits.map((entry) => entry.path))) {
    const identity = sha256(path).slice(0, 24);
    if (editPaths.has(identity)) throw new Error(`Reference edit preparation identity collision: ${identity}`);
    editPaths.set(identity, path);
  }
  const actions: { root: string; leaf?: string; preLeaf?: string; writer?: TransactionBinaryWritePreparation; publishWriter: boolean; target: string; targetPre: boolean; targetPost: boolean; discard: boolean }[] = [];
  for (const name of readdirSync(stageRoot).sort(generatorPathCompare)) {
    const rest = splitLeadingEmoji(name).rest;
    if (!rest.startsWith("edit-")) continue;
    const match = /^edit-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(match[3]) || name !== preparationName(match[1], Number.parseInt(match[2], 10), match[3]) || journal.state !== "editing") throw new Error(`Reference edit preparation is invalid or unreachable: ${name}`);
    const path = editPaths.get(match[1]);
    if (!path || !journal.backups[path]) throw new Error(`Reference edit preparation has no frozen plan backup: ${name}`);
    const root = join(stageRoot, name), target = absolutePath(repoRoot, path), rootStat = lstatOrNull(root), expectedLeaf = `${match[1]}.edit`, expectedPreLeaf = `${match[1]}.pre`;
    if (!rootStat?.isDirectory() || rootStat.isSymbolicLink()) throw new Error(`Reference edit preparation must be a no-follow directory: ${name}`);
    const writers = transactionBinaryWritePreparations(root, writePreparationName, writeCandidateName);
    const writerNames = new Set(writers.map((entry) => basename(entry.root)));
    const children = readdirSync(root).sort(generatorPathCompare);
    if (children.some((child) => child !== expectedLeaf && child !== expectedPreLeaf && !writerNames.has(child)) || children.length > 3) throw new Error(`Reference edit preparation contains unexpected evidence: ${name}`);
    const result = referenceEditResult(repoRoot, plan, journal, path), targetStat = lstatOrNull(target);
    const pre = targetStat?.isFile() && !targetStat.isSymbolicLink() && canonicalJson(leafPreimage(target)) === canonicalJson(result.preimage);
    const post = targetStat?.isFile() && !targetStat.isSymbolicLink() && sha256(readFileSync(target)) === sha256(result.bytes) && (targetStat.mode & 0o7777) === result.preimage.mode && targetStat.size === result.bytes.byteLength;
    const outerLeaf = children.includes(expectedLeaf) ? join(root, expectedLeaf) : undefined;
    const preLeaf = children.includes(expectedPreLeaf) ? join(root, expectedPreLeaf) : undefined;
    if (preLeaf && canonicalJson(leafPreimage(preLeaf)) !== canonicalJson(result.preimage)) throw new Error(`Reference edit preimage exchange bytes are forged: ${name}`);
    const exactCandidate = (candidatePath: string): boolean => {
      const candidate = leafPreimage(candidatePath);
      return candidate.nodeKind === "file" && candidate.contentHash === sha256(result.bytes) && candidate.mode === result.preimage.mode && candidate.size === result.bytes.byteLength;
    };
    if (outerLeaf && !exactCandidate(outerLeaf)) throw new Error(`Reference edit preparation bytes are forged: ${name}`);
    const writer = writers[0], writerExact = Boolean(writer?.leaf && exactCandidate(writer.leaf));
    if (outerLeaf && writer?.leaf) throw new Error(`Reference edit preparation has duplicate outer and nested candidates: ${name}`);
    if (writer && (preLeaf || post)) throw new Error(`Reference edit writer coexists with a target exchange tuple: ${name}`);
    const leaf = outerLeaf ?? (writerExact ? writer!.leaf : undefined);
    const discard = Boolean(writer && !outerLeaf && !writerExact);
    if (discard && (!pre || preLeaf)) throw new Error(`Incomplete reference edit writer has an unreachable target tuple: ${name}`);
    const stateValid = !leaf && !preLeaf ? pre || post
      : leaf && preLeaf ? !targetStat
        : leaf ? pre
          : post;
    if (!stateValid) throw new Error(`Reference edit preparation has an impossible target tuple: ${name}`);
    actions.push({ root, leaf, preLeaf, writer, publishWriter: Boolean(!outerLeaf && writerExact), target, targetPre: Boolean(pre), targetPost: Boolean(post), discard });
  }
  if (!validateOnly) for (const action of actions) {
    if (action.discard) {
      durableRemove(action.root, true);
      continue;
    }
    if (action.publishWriter) {
      const outerLeaf = join(action.root, `${sha256(normalizeRelative(relative(repoRoot, action.target).replaceAll("\\", "/"))).slice(0, 24)}.edit`);
      durableRename(action.leaf!, outerLeaf);
      action.leaf = outerLeaf;
    }
    if (action.writer && lstatOrNull(action.writer.root)) durableRemove(action.writer.root, true);
    if (!action.discard && action.leaf && !action.targetPost) {
      if (action.targetPre && !action.preLeaf) {
        const preLeaf = join(action.root, basename(action.leaf).replace(/\.edit$/u, ".pre"));
        durableRename(action.target, preLeaf);
        action.preLeaf = preLeaf;
      }
      durableRename(action.leaf, action.target);
    }
    if (action.preLeaf && lstatOrNull(action.preLeaf)) durableRemove(action.preLeaf);
    durableRemove(action.root, true);
  }
}

function assertRecoveryRootNames(
  repoRoot: string,
  plan: TaxonomyPlan,
  journal: MutableJournalRecord,
  backupPreparationName: (identity: string, pid: number, token: string) => string,
  restorePreparationName: (identity: string, pid: number, token: string) => string,
  editPreparationName: (identity: string, pid: number, token: string) => string,
  leasePreparationName: (pid: number, token: string, state: "preparing" | "stale") => string,
): void {
  const stageKnown = new Set([
    journal.journalWriteDirectory,
    ...plan.moves.map((entry) => entry.operationId),
    ...plan.embeddedTicketRootRelocations.map((entry) => `relocation-${entry.operationId}`),
    ...plan.evidenceRemovals.map((entry) => `removal-${entry.operationId}`),
    ...plan.embeddedTicketRoots.map((entry) => `root-${entry.operationId}`),
    ...plan.symlinkTargetEdits.map((entry) => `symlink-${entry.operationId}`),
  ]);
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot), backupRoot = absolutePath(repoRoot, journal.backupRoot);
  for (const name of (lstatOrNull(stagingRoot) ? readdirSync(stagingRoot) : []).sort(generatorPathCompare)) {
    if (stageKnown.has(name)) continue;
    const match = /^edit-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(match[3]) || name !== editPreparationName(match[1], Number.parseInt(match[2], 10), match[3])) throw new Error(`Transaction recovery staging root contains unauthorized evidence: ${name}`);
  }
  for (const name of (lstatOrNull(backupRoot) ? readdirSync(backupRoot) : []).sort(generatorPathCompare)) {
    if (/^[0-9a-f]{24}\.backup$/u.test(name)) continue;
    const rest = splitLeadingEmoji(name).rest;
    const leaf = /^(backup|restore)-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (leaf && TRANSACTION_LEASE_TOKEN.test(leaf[4])) {
      const expected = leaf[1] === "backup" ? backupPreparationName(leaf[2], Number.parseInt(leaf[3], 10), leaf[4]) : restorePreparationName(leaf[2], Number.parseInt(leaf[3], 10), leaf[4]);
      if (name === expected) continue;
    }
    const lease = /^lease-([1-9][0-9]*)-([0-9a-f-]+)-(preparing|stale)$/u.exec(rest);
    if (lease && TRANSACTION_LEASE_TOKEN.test(lease[2]) && name === leasePreparationName(Number.parseInt(lease[1], 10), lease[2], lease[3] as "preparing" | "stale")) continue;
    throw new Error(`Transaction recovery backup root contains unauthorized evidence: ${name}`);
  }
}

function validateLeasePreparationEvidence(
  backupRoot: string,
  leasePreparationName: (pid: number, token: string, state: "preparing" | "stale") => string,
  jsonWritePreparationName: (pid: number, token: string) => string,
  filename: string,
  previousName: string,
  planDigest: string,
  attemptOrdinal: string,
): void {
  for (const name of readdirSync(backupRoot).sort(generatorPathCompare)) {
    const match = /^lease-([1-9][0-9]*)-([0-9a-f-]+)-(preparing|stale)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match) continue;
    const pid = Number.parseInt(match[1], 10), token = match[2], state = match[3] as "preparing" | "stale";
    if (!TRANSACTION_LEASE_TOKEN.test(token) || name !== leasePreparationName(pid, token, state)) throw new Error(`Transaction lease preparation name is invalid: ${name}`);
    const root = join(backupRoot, name), stat = lstatOrNull(root);
    if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction lease preparation must be a no-follow directory: ${name}`);
    recoverCanonicalJsonCandidates(root, filename, previousName, jsonWritePreparationName, (path) => {
      const record = parseTransactionLease(path, planDigest, attemptOrdinal, token);
      if (record.pid !== pid) throw new Error(`Transaction lease preparation pid is invalid: ${name}`);
    }, false, true);
    const canonical = join(root, filename);
    if (lstatOrNull(canonical)) {
      const record = parseTransactionLease(canonical, planDigest, attemptOrdinal, token);
      if (record.pid !== pid) throw new Error(`Transaction lease preparation pid is invalid: ${name}`);
    }
  }
}

function actualAffectedDigest(repoRoot: string, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy): string {
  const row = (path: string): TaxonomyAffectedStateRow => {
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat) return { path, state: "absent" };
    if (stat.isSymbolicLink()) { const target = readlinkSync(absolute); return { path, state: "symlink", targetHash: sha256(target), targetSize: Buffer.byteLength(target) }; }
    if (stat.isFile()) return { path, state: "file", contentHash: sha256(readFileSync(absolute)), mode: stat.mode & 0o7777, size: stat.size };
    return { path, state: "directory-tree", tree: noFollowTreeDigest(repoRoot, path) };
  };
  const rows: TaxonomyAffectedStateRow[] = [];
  for (const ancestor of plan.destinationAncestorPreimages) {
    const stat = lstatOrNull(absolutePath(repoRoot, ancestor.path));
    rows.push(!stat ? { path: ancestor.path, state: "absent" } : stat.isDirectory() && !stat.isSymbolicLink() ? { path: ancestor.path, state: "directory" } : row(ancestor.path));
  }
  for (const move of plan.moves) rows.push(row(move.sourcePath), row(move.destinationPath));
  for (const relocation of plan.embeddedTicketRootRelocations) rows.push(row(relocation.sourcePath), row(relocation.destinationPath));
  for (const removal of plan.evidenceRemovals) { rows.push(row(removal.sourcePath)); if (removal.authority.kind === "byte-and-mode-identical") for (const member of removal.authority.members.filter((member) => member.disposition !== "remove")) rows.push(row(member.finalPath)); else for (const path of removalAuthorityPaths(removal.authority)) rows.push(row(path)); }
  for (const root of plan.embeddedTicketRoots) rows.push(row(root.sourceMetadataRoot));
  for (const edit of plan.symlinkTargetEdits) rows.push(row(edit.finalPath), row(edit.logicalTargetFinalPath));
  for (const path of new Set(plan.edits.map((edit) => edit.path))) rows.push(row(path));
  for (const regeneration of plan.regenerations) rows.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy))) });
  return affectedStateDigest(rows);
}

function actualAffectedPreDigest(repoRoot: string, plan: TaxonomyPlan): string {
  const row = (path: string): TaxonomyAffectedStateRow => {
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat) return { path, state: "absent" };
    if (stat.isSymbolicLink()) { const target = readlinkSync(absolute); return { path, state: "symlink", targetHash: sha256(target), targetSize: Buffer.byteLength(target) }; }
    if (stat.isFile()) return { path, state: "file", contentHash: sha256(readFileSync(absolute)), mode: stat.mode & 0o7777, size: stat.size };
    return { path, state: "directory-tree", tree: noFollowTreeDigest(repoRoot, path) };
  };
  const rows: TaxonomyAffectedStateRow[] = [];
  for (const ancestor of plan.destinationAncestorPreimages) {
    const stat = lstatOrNull(absolutePath(repoRoot, ancestor.path));
    rows.push(!stat ? { path: ancestor.path, state: "absent" } : stat.isDirectory() && !stat.isSymbolicLink() ? { path: ancestor.path, state: "directory" } : row(ancestor.path));
  }
  for (const move of plan.moves) rows.push(row(move.sourcePath), row(move.destinationPath));
  for (const relocation of plan.embeddedTicketRootRelocations) rows.push(row(relocation.sourcePath), row(relocation.destinationPath));
  for (const removal of plan.evidenceRemovals) { rows.push(row(removal.sourcePath)); if (removal.authority.kind === "byte-and-mode-identical") for (const member of removal.authority.members.filter((member) => member.disposition !== "remove")) rows.push(row(member.sourcePath)); else for (const path of removalAuthorityPaths(removal.authority)) rows.push(row(path)); }
  for (const root of plan.embeddedTicketRoots) rows.push(row(root.sourceMetadataRoot));
  for (const edit of plan.symlinkTargetEdits) rows.push(row(edit.sourcePath), row(edit.logicalTargetSourcePath));
  for (const path of new Set(plan.edits.map((edit) => plan.moves.find((move) => move.destinationPath === edit.path)?.sourcePath ?? edit.path))) rows.push(row(path));
  for (const regeneration of plan.regenerations) rows.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(regeneration.preOutputs)) });
  return affectedStateDigest(rows);
}

interface ArtifactStaleGroup {
  readonly id: string;
  readonly rationaleRule: ArtifactProjectionRationale;
  readonly ownerRoot: string;
  readonly markers: readonly string[];
}

function isProjectionConsumerPath(contract: SemanticPathProjectionReferenceConsumerContract, path: string): boolean {
  return contract.sourcePathIdentities.includes(path) && new RegExp(contract.sourcePathPattern, "u").test(path);
}

function artifactStaleGroups(paths: Iterable<string>, taxonomy: LoadedTaxonomy): readonly ArtifactStaleGroup[] {
  const values = [...new Set(paths)];
  const artifacts = canonicalDirectoryName(taxonomy, "artifacts", "artifacts");
  const consumers = Object.values(taxonomy.schema.semanticPathProjectionReferenceConsumerContracts);
  const rows = new Map<string, ArtifactStaleGroup>();
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) for (const path of values) {
    const segments = path.split("/");
    for (let index = 1; index < segments.length; index++) {
      if (segments[index - 1] !== artifacts || segments[index] !== contract.sourceArtifactMemberName) continue;
      const ownerRoot = segments.slice(0, index - 1).join("/");
      if (!ownerRoot) continue;
      const markers = [...new Set(consumers.filter((consumer) => consumer.projectionContractId === id).flatMap((consumer) => consumer.staleMarkers))].sort(generatorPathCompare);
      if (markers.length === 0) continue;
      rows.set(`${id}\u0000${ownerRoot}`, { id, rationaleRule: contract.rationaleRule, ownerRoot, markers });
    }
  }
  return [...rows.values()].sort((left, right) => generatorPathCompare(left.ownerRoot, right.ownerRoot) || left.id.localeCompare(right.id));
}

function canonicalMutationProjectionPresent(paths: Iterable<string>, taxonomy: LoadedTaxonomy): boolean {
  const projection = taxonomy.schema.semanticPathProjectionContracts[taxonomy.schema.mutationCatalogProjection.projectionContractId];
  const renderer = taxonomy.schema.semanticPathProjectionProfileRenderers[projection.profileRendererId];
  const tests = canonicalDirectoryName(taxonomy, "tests", "tests");
  for (const path of paths) {
    const segments = path.split("/");
    for (let index = 0; index + 1 < segments.length; index++) if (segments[index] === tests && matchDirectoryKind(segments[index + 1], taxonomy, "tests").kind?.id === renderer.directoryKindId) return true;
  }
  return false;
}

function staleProjectionContentViolations(path: string, content: string, groups: readonly ArtifactStaleGroup[], taxonomy: LoadedTaxonomy, mutationActive: boolean): readonly TaxonomyViolation[] {
  const rows: TaxonomyViolation[] = [];
  if (mutationActive) {
    const pattern = new RegExp(MUTATION_SOURCE_TEST_PREFIX, "gu");
    for (const match of content.matchAll(pattern)) if (match.index !== undefined) rows.push(violation("projection-old-token-stale", path, `Old artifact mutation test hierarchy remains at raw offset ${match.index}`));
  }
  const consumers = Object.values(taxonomy.schema.semanticPathProjectionReferenceConsumerContracts);
  for (const group of groups) {
    const internal = path === group.ownerRoot || path.startsWith(`${group.ownerRoot}/`);
    const external = consumers.some((contract) => contract.projectionContractId === group.id && isProjectionConsumerPath(contract, path));
    if (!internal && !external) continue;
    for (const marker of group.markers) for (let index = content.indexOf(marker); index >= 0; index = content.indexOf(marker, index + marker.length)) rows.push(violation("projection-old-token-stale", path, `Old ${group.rationaleRule} token remains at raw offset ${index}`));
  }
  return rows;
}

function activeProjectionContractIds(plan: TaxonomyPlan, taxonomy: LoadedTaxonomy): ReadonlySet<string> {
  const ids = new Set<string>();
  if (plan.moves.some((move) => move.rationaleRule === "artifact-mutation-test-projection-v1")) ids.add(taxonomy.schema.mutationCatalogProjection.projectionContractId);
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) if (plan.moves.some((move) => move.rationaleRule === contract.rationaleRule)) ids.add(id);
  return ids;
}

function declaredProjectionConsumerPaths(plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, inventory?: TaxonomyInventory): readonly string[] {
  const active = new Set(activeProjectionContractIds(plan, taxonomy));
  if (inventory) for (const group of artifactStaleGroups(inventory.entries.map((entry) => entry.normalizedPath), taxonomy)) active.add(group.id);
  return [...new Set(Object.values(taxonomy.schema.semanticPathProjectionReferenceConsumerContracts)
    .filter((contract) => active.has(contract.projectionContractId))
    .flatMap((contract) => contract.sourcePathIdentities))]
    .sort(generatorPathCompare);
}

function planVerificationCandidatePaths(repoRoot: string, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, ticketDir?: string): readonly string[] {
  const paths = new Set<string>();
  const pathspec = scopedGitPathspec(repoRoot, plan.scope, taxonomy);
  for (const row of gitRows(repoRoot, taxonomy, pathspec)) if (!isExcluded(row.path, taxonomy) && inScope(row.path, plan.scope) && lstatOrNull(absolutePath(repoRoot, row.path))) paths.add(row.path);
  for (const path of untrackedGitPaths(repoRoot, taxonomy, pathspec)) if (!isExcluded(path, taxonomy) && inScope(path, plan.scope)) paths.add(path);
  if (ticketDir) for (const row of explicitTicketRows(repoRoot, ticketDir, taxonomy, plan.scope)) if (!isExcluded(row.path, taxonomy) && inScope(row.path, plan.scope)) paths.add(row.path);
  for (const path of [
    ...plan.moves.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
    ...plan.embeddedTicketRoots.flatMap((entry) => [entry.sourceMetadataRoot, entry.sourceTicketRoot, entry.canonicalTicketRoot]),
    ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
    ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
    ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...removalAuthorityPaths(entry.authority)]),
    ...plan.edits.map((entry) => entry.path),
    ...plan.regenerations.flatMap((entry) => [entry.cwd, ...entry.outputRoots, ...entry.inputs.map((input) => input.path), ...entry.preOutputs.map((output) => output.path), ...entry.outputs.map((output) => output.path), ...entry.staleRemovals]),
    ...declaredProjectionConsumerPaths(plan, taxonomy),
  ]) if (!isExcluded(path, taxonomy)) paths.add(path);
  return [...paths].sort(generatorPathCompare);
}

function projectionStaleViolations(repoRoot: string, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, inventory?: TaxonomyInventory, ticketDir?: string): readonly TaxonomyViolation[] {
  if (inventory) {
    const moveBySource = new Map(plan.moves.map((move) => [move.sourcePath, move.destinationPath]));
    const finalPaths = inventory.entries.map((entry) => moveBySource.get(entry.sourcePath) ?? entry.normalizedPath);
    const mutationActive = plan.moves.some((move) => move.rationaleRule === "artifact-mutation-test-projection-v1") || canonicalMutationProjectionPresent(finalPaths, taxonomy);
    const groups = artifactStaleGroups(finalPaths, taxonomy);
    const rows: TaxonomyViolation[] = [];
    for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind === "file" && textualPath(candidate.sourcePath))) {
      const path = moveBySource.get(entry.sourcePath) ?? entry.normalizedPath;
      let content: string;
      try {
        content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(absolutePath(repoRoot, entry.sourcePath)));
        const edits = plan.edits.filter((edit) => edit.path === path);
        if (edits.length > 0) content = applyEditsToContent(content, edits);
      } catch (error) {
        if (isTransactionRepositoryAuthorityError(error)) throw error;
        continue;
      }
      rows.push(...staleProjectionContentViolations(path, content, groups, taxonomy, mutationActive));
    }
    const admitted = new Set(inventory.entries.map((entry) => entry.sourcePath));
    for (const path of declaredProjectionConsumerPaths(plan, taxonomy, inventory).filter((candidate) => !admitted.has(candidate))) {
      const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Declared projection consumer", true);
      const stat = lstatOrNull(absolute);
      if (!stat?.isFile() || stat.isSymbolicLink() || stat.size > 16 * 1024 * 1024 || !textualPath(path)) continue;
      try {
        let content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(absolute));
        const edits = plan.edits.filter((edit) => edit.path === path);
        if (edits.length > 0) content = applyEditsToContent(content, edits);
        rows.push(...staleProjectionContentViolations(path, content, groups, taxonomy, mutationActive));
      } catch (error) { if (isTransactionRepositoryAuthorityError(error)) throw error; }
    }
    return stableViolations(rows);
  }
  const paths = new Set(planVerificationCandidatePaths(repoRoot, plan, taxonomy, ticketDir));
  const mutationActive = plan.moves.some((move) => move.rationaleRule === "artifact-mutation-test-projection-v1") || canonicalMutationProjectionPresent(paths, taxonomy);
  const groups = artifactStaleGroups(paths, taxonomy);
  if (!mutationActive && groups.length === 0) return [];
  const rows: TaxonomyViolation[] = [];
  for (const path of [...paths].filter(textualPath).sort(generatorPathCompare)) {
    if (isExcluded(path, taxonomy)) continue;
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat?.isFile() || stat.size > 16 * 1024 * 1024) continue;
    try {
      const content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(absolute));
      rows.push(...staleProjectionContentViolations(path, content, groups, taxonomy, mutationActive));
    } catch (error) { if (isTransactionRepositoryAuthorityError(error)) throw error; }
  }
  return stableViolations(rows);
}

function projectionPostApplyViolations(repoRoot: string, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy): readonly TaxonomyViolation[] {
  const moves = plan.moves.filter((move) => move.rationaleRule === "artifact-mutation-test-projection-v1");
  if (moves.length === 0) return [];
  const ids = taxonomy.schema.mutationCatalogProjection;
  const descendant = mutationDescendantContract(taxonomy);
  const groups = new Map<string, TaxonomyMove[]>();
  for (const move of moves) {
    const artifactRoot = artifactRootForPath(move.sourcePath);
    if (!artifactRoot || !move.destinationPath.startsWith(`${artifactRoot}/`)) continue;
    const relativeSegments = move.destinationPath.slice(artifactRoot.length + 1).split("/");
    if (relativeSegments.length < 5) continue;
    const scenarioRoot = `${artifactRoot}/${relativeSegments.slice(0, 4).join("/")}`;
    groups.set(scenarioRoot, [...(groups.get(scenarioRoot) ?? []), move]);
  }
  const rows: TaxonomyViolation[] = [];
  const expectedRequired = descendant.requiredNodes.map((node) => `${node.nodeType}\u0000${projectionDescendantPath(node, taxonomy)}`);
  for (const [scenarioRoot, group] of [...groups.entries()].sort(([left], [right]) => left.localeCompare(right))) {
    if (group.length !== 6) {
      rows.push(violation("projection-apply-move-count", scenarioRoot, `Projected scenario has ${group.length} file moves, expected 6`));
      continue;
    }
    for (const move of group) if (lstatOrNull(absolutePath(repoRoot, move.sourcePath))) rows.push(violation("projection-source-file-stale", move.sourcePath, "Projected source file remains after staged move installation"));
    const actual = new Set<string>();
    const walk = (path: string): void => {
      if (isExcluded(path, taxonomy)) throw new Error(`Projection destination crosses opaque path ${path}`);
      const stat = lstatOrNull(absolutePath(repoRoot, path));
      if (!stat) return;
      const relativePath = path === scenarioRoot ? "" : path.slice(scenarioRoot.length + 1);
      if (stat.isSymbolicLink()) {
        rows.push(violation("projection-bundle-symlink", path, "Projected bundle contains a symlink"));
        return;
      }
      actual.add(`${stat.isDirectory() ? "directory" : "file"}\u0000${relativePath}`);
      if (stat.isDirectory()) for (const name of readdirSync(absolutePath(repoRoot, path)).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))) walk(`${path}/${name}`);
    };
    walk(scenarioRoot);
    const alternatives = descendant.exclusiveAlternatives.map((alternative) => alternative.nodes.map((node) => `${node.nodeType}\u0000${projectionDescendantPath(node, taxonomy)}`).filter((key) => actual.has(key)));
    if (actual.size !== descendant.realizedNodeCount || expectedRequired.some((key) => !actual.has(key)) || alternatives.some((matches) => matches.length !== 1)) rows.push(violation("projection-apply-bundle-invalid", scenarioRoot, `Projected destination does not realize the exact ${descendant.realizedNodeCount}-node descendant contract`));
  }
  if (groups.size * 6 !== moves.length) rows.push(violation("projection-apply-group-unresolved", moves[0].sourcePath, `${moves.length - groups.size * 6} projection move(s) do not resolve to an exact artifact scenario root`));
  return stableViolations(rows);
}

function artifactProjectionPostApplyViolations(repoRoot: string, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, prunableSourceParents: ReadonlySet<string>): readonly TaxonomyViolation[] {
  const rows: TaxonomyViolation[] = [];
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const moves = plan.moves.filter((move) => move.rationaleRule === contract.rationaleRule);
    const groups = new Map<string, TaxonomyMove[]>();
    for (const move of moves) {
      const location = artifactProjectionSourceLocation(move.sourcePath, contract, taxonomy);
      if (location) groups.set(location.sourceRoot, [...(groups.get(location.sourceRoot) ?? []), move]);
    }
    for (const [sourceRoot, group] of [...groups].sort(([left], [right]) => generatorPathCompare(left, right))) {
      if (lstatOrNull(absolutePath(repoRoot, sourceRoot)) && !prunableSourceParents.has(sourceRoot)) rows.push(violation("projection-source-directory-stale", sourceRoot, `${id} source owner retains a nonempty or unplanned child outside exact move-source parent pruning`));
      const location = artifactProjectionSourceLocation(sourceRoot, contract, taxonomy);
      if (!location) {
        rows.push(violation("projection-apply-group-unresolved", sourceRoot, `${id} source root cannot be reconstructed from its frozen contract`));
        continue;
      }
      const rendered = renderArtifactPathProjectionRoot({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot }, taxonomy.discoverySchema);
      if (rendered.problems.length > 0) {
        rows.push(violation("projection-apply-group-unresolved", sourceRoot, rendered.problems.join(" | ")));
        continue;
      }
      const expected = new Set<string>([`directory\u0000${rendered.destinationRoot}`]);
      for (const move of group) {
        if (lstatOrNull(absolutePath(repoRoot, move.sourcePath))) rows.push(violation("projection-source-file-stale", move.sourcePath, "Projected source file remains after staged move installation"));
        const destination = lstatOrNull(absolutePath(repoRoot, move.destinationPath));
        if (!destination?.isFile() || destination.isSymbolicLink()) rows.push(violation("projection-destination-file-invalid", move.destinationPath, "Projected destination is missing, non-file, or a symlink"));
        expected.add(`file\u0000${move.destinationPath}`);
        for (let path = dirname(move.destinationPath); path === rendered.destinationRoot || path.startsWith(`${rendered.destinationRoot}/`); path = dirname(path)) {
          expected.add(`directory\u0000${path}`);
          if (path === rendered.destinationRoot) break;
        }
      }
      const actual = new Set<string>();
      const walk = (path: string): void => {
        if (isExcluded(path, taxonomy)) throw new Error(`Artifact projection destination crosses opaque path ${path}`);
        const stat = lstatOrNull(absolutePath(repoRoot, path));
        if (!stat) return;
        if (stat.isSymbolicLink()) {
          rows.push(violation("projection-bundle-symlink", path, "Projected destination contains a symlink"));
          return;
        }
        actual.add(`${stat.isDirectory() ? "directory" : "file"}\u0000${path}`);
        if (stat.isDirectory()) for (const name of readdirSync(absolutePath(repoRoot, path)).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))) walk(`${path}/${name}`);
      };
      walk(rendered.destinationRoot);
      const missing = [...expected].filter((row) => !actual.has(row));
      const unexpected = [...actual].filter((row) => !expected.has(row));
      if (missing.length > 0 || unexpected.length > 0) rows.push(violation("projection-apply-descendants-invalid", rendered.destinationRoot, `${id} exact descendant mismatch: ${missing.length} missing, ${unexpected.length} unexpected`));
    }
    if ([...groups.values()].reduce((count, group) => count + group.length, 0) !== moves.length && moves.length > 0) rows.push(violation("projection-apply-group-unresolved", moves[0].sourcePath, `${id} has moves outside its exact source root`));
  }
  return stableViolations(rows);
}

function injectFailure(options: TaxonomyApplyOptions, stage: TaxonomyFailureStage): void {
  if (options.injectFailureAt === stage) throw new Error(`Injected taxonomy failure at ${stage}`);
}

function emptySourceParents(repoRoot: string, plan: TaxonomyPlan, ticketRoot: string): readonly string[] {
  const candidates = new Set<string>();
  const ticketRelative = sourceRelative(relative(repoRoot, ticketRoot));
  for (const move of plan.moves) {
    let parent = posix.dirname(move.sourcePath);
    while (parent && parent !== "." && parent !== ticketRelative && !ticketRelative.startsWith(`${parent}/`)) {
      candidates.add(parent);
      parent = posix.dirname(parent);
    }
  }
  const empty = new Set<string>();
  for (const path of [...candidates].sort((left, right) => right.split("/").length - left.split("/").length || generatorPathCompare(right, left))) {
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Move source parent", true);
    const stat = lstatOrNull(absolute);
    if (!stat) continue;
    if (!stat.isDirectory()) throw new Error(`Move source parent is no longer a directory: ${path}`);
    if (readdirSync(absolute).every((name) => empty.has(`${path}/${name}`))) empty.add(path);
  }
  return [...empty];
}

function inventoryAfterSourceParentPruning(inventory: TaxonomyInventory, paths: readonly string[]): TaxonomyInventory {
  if (paths.length === 0) return inventory;
  const pruned = new Set(paths);
  if (inventory.entries.some((entry) => pruned.has(entry.sourcePath) && entry.nodeKind !== "directory")) throw new Error("Source-parent pruning may only project exact directory entries");
  const entries = inventory.entries.filter((entry) => !pruned.has(entry.sourcePath)).map((entry) => ({ ...entry, referencesIn: entry.referencesIn.filter((path) => !pruned.has(path)) }));
  const referenced = entries.find((entry) => entry.referencesOut.some((path) => pruned.has(path)));
  if (referenced) throw new Error(`Prunable source directory remains referenced by ${referenced.sourcePath}`);
  const childrenByParent = new Map<string, typeof entries>();
  for (const entry of entries) {
    const parent = posix.dirname(entry.sourcePath);
    const children = childrenByParent.get(parent) ?? [];
    children.push(entry);
    childrenByParent.set(parent, children);
  }
  for (const entry of entries.filter((entry) => entry.nodeKind === "directory").sort((left, right) => right.sourcePath.split("/").length - left.sourcePath.split("/").length)) entry.contentHash = directoryHash(entry.sourcePath, childrenByParent.get(entry.sourcePath) ?? []);
  const { inventoryDigest: _inventoryDigest, repoRoot, taxonomyPath, ...metadata } = inventory;
  const partial = { ...metadata, entries, violations: inventory.violations.filter((entry) => !pruned.has(entry.path)), sourceTreeDigest: sourceTreeDigest(entries) };
  return inheritReferenceInventoryContext(inventory, { ...partial, repoRoot, taxonomyPath, inventoryDigest: inventoryDigestOf(partial) });
}

function pruneEmptySourceParents(repoRoot: string, paths: readonly string[]): void {
  for (const path of paths) {
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Committed move source parent", true);
    const stat = lstatOrNull(absolute);
    if (!stat) continue;
    if (!stat.isDirectory() || readdirSync(absolute).length !== 0) throw new Error(`Committed source-parent pruning preimage changed: ${path}`);
    rmdirSync(absolute);
    fsyncDirectory(dirname(absolute));
  }
}

function committedSourceParentPrunePaths(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, ticketRoot: string): readonly string[] {
  assertJournalPlanMembership(plan, journal);
  const ticketRelative = sourceRelative(relative(repoRoot, ticketRoot));
  const paths = [...journal.sourceParentPrunePaths].sort((left, right) => right.split("/").length - left.split("/").length || generatorPathCompare(right, left));
  const approved = new Set(paths);
  for (const path of paths) {
    if (path === ticketRelative || ticketRelative.startsWith(`${path}/`)) throw new Error(`Committed source-parent pruning cannot remove the ticket root or its ancestors: ${path}`);
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Journal-bound source parent", true);
    const stat = lstatOrNull(absolute);
    if (!stat) continue;
    if (!stat.isDirectory() || readdirSync(absolute).some((name) => !approved.has(`${path}/${name}`))) throw new Error(`Committed source-parent pruning preimage changed: ${path}`);
  }
  return paths;
}

function rollbackDestinationAncestors(repoRoot: string, plan: TaxonomyPlan): void {
  for (const ancestor of plan.destinationAncestorPreimages.filter((entry) => entry.state === "absent").sort((left, right) => right.path.split("/").length - left.path.split("/").length || generatorPathCompare(right.path, left.path))) {
    const path = absolutePath(repoRoot, ancestor.path);
    const stat = lstatOrNull(path);
    if (!stat) continue;
    if (!stat.isDirectory() || stat.isSymbolicLink() || readdirSync(path).length > 0) throw new Error(`Rollback-created destination ancestor is occupied: ${ancestor.path}`);
    rmdirSync(path);
    fsyncDirectory(dirname(path));
  }
}

function reconcileRollbackTuples(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): boolean {
  let changed = false;
  const remove = (array: string[], id: string): void => { const next = array.filter((entry) => entry !== id); if (next.length !== array.length) { array.splice(0, array.length, ...next); changed = true; } };
  const add = (array: string[], id: string): void => { if (!array.includes(id)) { array.push(id); changed = true; } };
  const present = (path: string): boolean => Boolean(lstatOrNull(path));
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const pendingRestorePaths = new Set<string>();
  for (const path of Object.keys(journal.backups)) {
    const identity = sha256(path).slice(0, 24);
    if (readdirSync(backupRoot).some((name) => splitLeadingEmoji(name).rest.startsWith(`restore-${identity}-`))) pendingRestorePaths.add(path);
  }
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (backup.kind === "symlink" && (backup.targetHash !== sha256(backup.target) || backup.size !== Buffer.byteLength(backup.target))) throw new Error(`rollback-state-drift: symlink backup ${path}`);
    if (backup.kind !== "file") continue;
    const stored = join(backupRoot, backup.backupPath);
    const stat = lstatOrNull(stored);
    if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(stored) !== backup.contentHash || (stat.mode & 0o7777) !== backup.mode || stat.size !== backup.size) throw new Error(`rollback-state-drift: file backup ${path}`);
  }
  for (const move of plan.moves) {
    const source = absolutePath(repoRoot, move.sourcePath), stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination = absolutePath(repoRoot, move.destinationPath);
    const states = [present(source), present(stage), present(destination)];
    const pendingRestoreExchange = states.every((state) => !state) && pendingRestorePaths.has(move.destinationPath);
    if (!pendingRestoreExchange && states.filter(Boolean).length !== 1) throw new Error(`rollback-state-drift: move ${move.operationId}`);
    if (pendingRestoreExchange) continue;
    const index = states.findIndex(Boolean);
    const current = [source, stage, destination][index];
    const installedLink = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath && journal.installedSymlinkTargetEditIds.includes(edit.operationId));
    let edited = false;
    let expectedPreimage = retargetedMovePreimage(move, installedLink);
    if (index === 2 && journal.appliedEditPaths.includes(move.destinationPath)) {
      const backup = journal.backups[move.destinationPath];
      if (!backup || backup.kind !== "file") throw new Error(`rollback-state-drift: move edit backup ${move.operationId}`);
      const result = applyEditsToContent(readFileSync(join(absolutePath(repoRoot, journal.backupRoot), backup.backupPath), "utf8"), plan.edits.filter((edit) => edit.path === move.destinationPath));
      const postimage = { nodeKind: "file", contentHash: sha256(result), mode: backup.mode, size: Buffer.byteLength(result) } as const;
      edited = canonicalJson(leafPreimage(current)) === canonicalJson(postimage);
      if (edited) expectedPreimage = postimage;
    }
    if (canonicalJson(leafPreimage(current)) !== canonicalJson(expectedPreimage)) throw new Error(`rollback-state-drift: move preimage ${move.operationId}`);
    if (states[0]) { remove(journal.installedMoveIds, move.operationId); remove(journal.stagedMoveIds, move.operationId); remove(journal.preparedMoveIds, move.operationId); }
    else if (states[1]) { remove(journal.installedMoveIds, move.operationId); add(journal.stagedMoveIds, move.operationId); add(journal.preparedMoveIds, move.operationId); }
    else if (!journal.installedMoveIds.includes(move.operationId)) throw new Error(`rollback-state-drift: unowned move destination ${move.operationId}`);
  }
  for (const entry of plan.embeddedTicketRootRelocations) {
    const source = absolutePath(repoRoot, entry.sourcePath), stage = join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${entry.operationId}`), destination = absolutePath(repoRoot, entry.destinationPath);
    const states = [present(source), present(stage), present(destination)];
    if (states.filter(Boolean).length !== 1) throw new Error(`rollback-state-drift: relocation ${entry.operationId}`);
    const current = [source, stage, destination][states.findIndex(Boolean)];
    if (canonicalJson(leafPreimage(current)) !== canonicalJson(entry.preimage)) throw new Error(`rollback-state-drift: relocation preimage ${entry.operationId}`);
    if (states[0]) { remove(journal.installedEmbeddedRelocationIds, entry.operationId); remove(journal.stagedEmbeddedRelocationIds, entry.operationId); remove(journal.preparedEmbeddedRelocationIds, entry.operationId); }
    else if (states[1]) { remove(journal.installedEmbeddedRelocationIds, entry.operationId); add(journal.stagedEmbeddedRelocationIds, entry.operationId); add(journal.preparedEmbeddedRelocationIds, entry.operationId); }
    else if (!journal.installedEmbeddedRelocationIds.includes(entry.operationId)) throw new Error(`rollback-state-drift: unowned relocation destination ${entry.operationId}`);
  }
  for (const entry of plan.evidenceRemovals) {
    const source = absolutePath(repoRoot, entry.sourcePath), stage = join(absolutePath(repoRoot, journal.stagingRoot), `removal-${entry.operationId}`);
    const states = [present(source), present(stage)];
    if (states.filter(Boolean).length !== 1 || canonicalJson(leafPreimage(states[0] ? source : stage)) !== canonicalJson(entry.preimage)) throw new Error(`rollback-state-drift: removal ${entry.operationId}`);
    if (states[0]) { remove(journal.stagedEvidenceRemovalIds, entry.operationId); remove(journal.preparedEvidenceRemovalIds, entry.operationId); }
    else { add(journal.stagedEvidenceRemovalIds, entry.operationId); add(journal.preparedEvidenceRemovalIds, entry.operationId); }
  }
  for (const root of plan.embeddedTicketRoots) {
    const source = absolutePath(repoRoot, root.sourceMetadataRoot), stage = join(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`);
    const states = [present(source), present(stage)];
    if (states.filter(Boolean).length !== 1) throw new Error(`rollback-state-drift: embedded root ${root.operationId}`);
    const current = states[0] ? root.sourceMetadataRoot : normalizeRelative(`${journal.stagingRoot}/root-${root.operationId}`);
    const children = [...plan.embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath), ...plan.evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath)];
    const tree = states[0] ? noFollowTreeDigestExcluding(repoRoot, current, children) : noFollowTreeDigest(repoRoot, current);
    if (canonicalJson(tree) !== canonicalJson(root.residualTreeDigest)) throw new Error(`rollback-state-drift: embedded root tree ${root.operationId}`);
    if (states[0]) { remove(journal.stagedEmbeddedRootIds, root.operationId); remove(journal.preparedEmbeddedRootIds, root.operationId); }
    else { add(journal.stagedEmbeddedRootIds, root.operationId); add(journal.preparedEmbeddedRootIds, root.operationId); }
  }
  for (const edit of plan.symlinkTargetEdits) {
    const link = absolutePath(repoRoot, edit.finalPath), stage = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    const linkStat = lstatOrNull(link), stageStat = lstatOrNull(stage);
    const oldAtLink = linkStat?.isSymbolicLink() && readlinkSync(link) === edit.oldTarget;
    const newAtLink = linkStat?.isSymbolicLink() && readlinkSync(link) === edit.newTarget;
    const oldAtStage = stageStat?.isSymbolicLink() && readlinkSync(stage) === edit.oldTarget;
    if (oldAtLink && !stageStat) { remove(journal.installedSymlinkTargetEditIds, edit.operationId); remove(journal.stagedSymlinkTargetEditIds, edit.operationId); remove(journal.preparedSymlinkTargetEditIds, edit.operationId); }
    else if ((!linkStat || newAtLink) && oldAtStage) { add(journal.stagedSymlinkTargetEditIds, edit.operationId); add(journal.preparedSymlinkTargetEditIds, edit.operationId); if (newAtLink) add(journal.installedSymlinkTargetEditIds, edit.operationId); else remove(journal.installedSymlinkTargetEditIds, edit.operationId); }
    else throw new Error(`rollback-state-drift: symlink edit ${edit.operationId}`);
  }
  for (const path of [...new Set(plan.edits.map((entry) => entry.path))].filter((entry) => journal.backups[entry])) {
    if (pendingRestorePaths.has(path)) continue;
    const backup = journal.backups[path];
    if (!backup || backup.kind !== "file") throw new Error(`rollback-state-drift: edit backup ${path}`);
    const current = absolutePath(repoRoot, path);
    const stat = lstatOrNull(current);
    const pre = stat?.isFile() && !stat.isSymbolicLink() && hashPath(current) === backup.contentHash && (stat.mode & 0o7777) === backup.mode && stat.size === backup.size;
    const result = applyEditsToContent(readFileSync(join(backupRoot, backup.backupPath), "utf8"), plan.edits.filter((edit) => edit.path === path));
    const post = stat?.isFile() && !stat.isSymbolicLink() && readFileSync(current, "utf8") === result && (stat.mode & 0o7777) === backup.mode && stat.size === Buffer.byteLength(result);
    if (pre) {
      if (!journal.appliedEditPaths.includes(path)) throw new Error(`rollback-state-drift: unowned restored edit ${path}`);
    }
    else if (post) add(journal.appliedEditPaths, path);
    else throw new Error(`rollback-state-drift: edit ${path}`);
  }
  for (const regeneration of plan.regenerations) {
    if (journal.startedRegenerationIds.includes(regeneration.id)) continue;
    if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs)) throw new Error(`rollback-state-drift: unstarted regeneration ${regeneration.id}`);
  }
  return changed;
}

function rollbackTransaction(repoRoot: string, plan: TaxonomyPlan, journalPath: string, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy, options: TaxonomyApplyOptions): void {
  if (journal.state === "rolling-back") {
    if (reconcileRollbackTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
  } else {
    if (reconcileRollbackTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
    journal.state = "rolling-back";
    persistJournal(repoRoot, journalPath, journal);
  }
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const restorePreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-restore-preparation", `restore-${identity}-${pid}-${token}`, "transaction-backup");
  const started = new Set(journal.startedRegenerationIds);
  for (const regeneration of [...plan.regenerations].reverse()) {
    if (!started.has(regeneration.id)) continue;
    for (const root of [...regeneration.outputRoots].sort((left, right) => right.length - left.length || generatorPathCompare(right, left))) {
      const path = absolutePath(repoRoot, root);
      if (lstatOrNull(dirname(path))) durableRemove(path, true);
    }
    for (const directory of regeneration.preOutputs.filter((entry) => entry.nodeKind === "directory").sort((left, right) => left.path.split("/").length - right.path.split("/").length || generatorPathCompare(left.path, right.path))) {
      const path = absolutePath(repoRoot, directory.path);
      mkdirSync(path, { recursive: true });
      chmodSync(path, directory.mode);
      fsyncDirectory(path);
      fsyncDirectory(dirname(path));
    }
  }
  for (const [path, backup] of Object.entries(journal.backups).sort(([a], [b]) => generatorPathCompare(b, a))) {
    restoreBackup(repoRoot, plan, path, backupRoot, backup, restorePreparationName, journal.probe);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const regeneration of plan.regenerations.filter((entry) => journal.startedRegenerationIds.includes(entry.id))) {
    if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs)) throw new Error(`Rollback regeneration pre-state is incomplete: ${regeneration.id}`);
  }
  for (const edit of [...plan.symlinkTargetEdits].reverse()) {
    if (!journal.stagedSymlinkTargetEditIds.includes(edit.operationId)) continue;
    const link = absolutePath(repoRoot, edit.finalPath);
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    if (lstatOrNull(link)) durableRemove(link);
    if (lstatOrNull(stage)) { mkdirSync(dirname(link), { recursive: true }); durableRename(stage, link); }
    report(options.progress, "apply", "rolling-back-symlink-target-edits", 1, plan.symlinkTargetEdits.length, edit.finalPath);
    journal.installedSymlinkTargetEditIds = journal.installedSymlinkTargetEditIds.filter((id) => id !== edit.operationId);
    journal.stagedSymlinkTargetEditIds = journal.stagedSymlinkTargetEditIds.filter((id) => id !== edit.operationId);
    journal.preparedSymlinkTargetEditIds = journal.preparedSymlinkTargetEditIds.filter((id) => id !== edit.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const relocation of [...plan.embeddedTicketRootRelocations].reverse()) {
    if (!journal.installedEmbeddedRelocationIds.includes(relocation.operationId)) continue;
    const destination = absolutePath(repoRoot, relocation.destinationPath);
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`);
    if (!lstatOrNull(stage) && lstatOrNull(destination)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(destination, stage); }
    journal.installedEmbeddedRelocationIds = journal.installedEmbeddedRelocationIds.filter((id) => id !== relocation.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  const activeIds = new Set([...journal.stagedMoveIds, ...journal.installedMoveIds]);
  for (const move of [...plan.moves].reverse()) {
    if (!journal.installedMoveIds.includes(move.operationId)) continue;
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
    const destination = absolutePath(repoRoot, move.destinationPath);
    if (!lstatOrNull(stage) && lstatOrNull(destination)) {
      mkdirSync(dirname(stage), { recursive: true });
      durableRename(destination, stage);
    }
    journal.installedMoveIds = journal.installedMoveIds.filter((id) => id !== move.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const root of [...plan.embeddedTicketRoots].reverse()) {
    if (!journal.stagedEmbeddedRootIds.includes(root.operationId)) continue;
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`);
    const source = absolutePath(repoRoot, root.sourceMetadataRoot);
    if (lstatOrNull(stage)) { if (lstatOrNull(source)) throw new Error(`Rollback embedded root source is occupied: ${root.sourceMetadataRoot}`); mkdirSync(dirname(source), { recursive: true }); durableRename(stage, source); }
    journal.stagedEmbeddedRootIds = journal.stagedEmbeddedRootIds.filter((id) => id !== root.operationId);
    journal.preparedEmbeddedRootIds = journal.preparedEmbeddedRootIds.filter((id) => id !== root.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const relocation of [...plan.embeddedTicketRootRelocations].reverse()) {
    if (!journal.stagedEmbeddedRelocationIds.includes(relocation.operationId)) continue;
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`);
    const source = absolutePath(repoRoot, relocation.sourcePath);
    if (lstatOrNull(stage)) { if (lstatOrNull(source)) throw new Error(`Rollback relocation source is occupied: ${relocation.sourcePath}`); mkdirSync(dirname(source), { recursive: true }); durableRename(stage, source); }
    journal.stagedEmbeddedRelocationIds = journal.stagedEmbeddedRelocationIds.filter((id) => id !== relocation.operationId);
    journal.preparedEmbeddedRelocationIds = journal.preparedEmbeddedRelocationIds.filter((id) => id !== relocation.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const removal of [...plan.evidenceRemovals].reverse()) {
    if (!journal.stagedEvidenceRemovalIds.includes(removal.operationId)) continue;
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `removal-${removal.operationId}`);
    const source = absolutePath(repoRoot, removal.sourcePath);
    if (lstatOrNull(stage)) { if (lstatOrNull(source)) throw new Error(`Rollback removal source is occupied: ${removal.sourcePath}`); mkdirSync(dirname(source), { recursive: true }); durableRename(stage, source); }
    journal.stagedEvidenceRemovalIds = journal.stagedEvidenceRemovalIds.filter((id) => id !== removal.operationId);
    journal.preparedEvidenceRemovalIds = journal.preparedEvidenceRemovalIds.filter((id) => id !== removal.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const move of [...plan.moves].reverse()) {
    if (!activeIds.has(move.operationId)) continue;
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
    const source = absolutePath(repoRoot, move.sourcePath);
    if (lstatOrNull(stage)) {
      mkdirSync(dirname(source), { recursive: true });
      if (lstatOrNull(source)) throw new Error(`Rollback source is occupied: ${move.sourcePath}`);
      durableRename(stage, source);
    }
    journal.stagedMoveIds = journal.stagedMoveIds.filter((id) => id !== move.operationId);
    journal.preparedMoveIds = journal.preparedMoveIds.filter((id) => id !== move.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  journal.appliedEditPaths = [];
  journal.startedRegenerationIds = [];
  journal.completedRegenerationIds = [];
  journal.installedMoveIds = [];
  journal.stagedMoveIds = [];
  journal.preparedMoveIds = [];
  journal.preparedEmbeddedRelocationIds = [];
  journal.stagedEmbeddedRelocationIds = [];
  journal.installedEmbeddedRelocationIds = [];
  journal.preparedEvidenceRemovalIds = [];
  journal.stagedEvidenceRemovalIds = [];
  journal.preparedEmbeddedRootIds = [];
  journal.stagedEmbeddedRootIds = [];
  journal.preparedSymlinkTargetEditIds = [];
  journal.stagedSymlinkTargetEditIds = [];
  journal.installedSymlinkTargetEditIds = [];
  rollbackDestinationAncestors(repoRoot, plan);
  if (actualAffectedPreDigest(repoRoot, plan) !== plan.expectedAffectedPreStateDigest) throw new Error("Rollback did not restore the affected pre-state digest");
  journal.state = "rolled-back";
  persistJournal(repoRoot, journalPath, journal);
  cleanupRolledBackTransaction(repoRoot, journal, plan);
}
//#endregion 🔐️Transaction Internals

//#region 🚚️Apply API
type TransactionRepositoryAuthorityFailureReason = "missing-authority" | "invalid-index" | "index-drift" | "repository-boundary" | "invalid-access";

class TransactionRepositoryAuthorityError extends Error {
  constructor(readonly reason: TransactionRepositoryAuthorityFailureReason, cause: unknown) {
    super(cause instanceof Error ? cause.message : String(cause), { cause });
    this.name = "TransactionRepositoryAuthorityError";
  }
}

function isTransactionRepositoryAuthorityError(error: unknown): error is TransactionRepositoryAuthorityError {
  return error instanceof TransactionRepositoryAuthorityError;
}

function transactionRepositoryFinally(body: () => void, release: () => void): void {
  let preserve = false;
  try { body(); }
  catch (error) { preserve = isTransactionRepositoryAuthorityError(error); throw error; }
  finally { if (!preserve) release(); }
}

class TransactionRepositoryAuthority {
  readonly #validated = true;
  readonly repoRoot: string;
  readonly indexRows: readonly { readonly path: string; readonly entry: TaxonomySourceIndexEntry }[];
  readonly repositoryFences: readonly string[];
  readonly indexWitness: string;

  constructor(repoRoot: string, rows: unknown) {
    try {
      sourceAdmissionAssertLexical(repoRoot, "Transaction repository root", true);
      if (!isAbsolute(repoRoot) || resolve(repoRoot) !== repoRoot) throw new Error("Transaction repository root is not absolute and canonical");
    } catch (cause) {
      if (isTransactionRepositoryAuthorityError(cause)) throw cause;
      throw new TransactionRepositoryAuthorityError("invalid-access", cause);
    }
    if (!Array.isArray(rows)) throw new TransactionRepositoryAuthorityError("invalid-index", new Error("Transaction index rows are invalid"));
    this.repoRoot = repoRoot;
    this.indexRows = Object.freeze(rows.map((row: unknown) => {
      if (!sourceAdmissionRecord(row, ["path", "entry"]) || typeof row.path !== "string" || !sourceAdmissionSafePath(row.path) || !sourceAdmissionRecord(row.entry, ["stage", "mode", "objectId"])) throw new TransactionRepositoryAuthorityError("invalid-index", new Error("Transaction index row is invalid"));
      const entry = row.entry;
      if (!Number.isInteger(entry.stage) || Number(entry.stage) < 0 || Number(entry.stage) > 3 || !["100644", "100755", "120000", "160000"].includes(entry.mode as string) || typeof entry.objectId !== "string" || !/^(?:[a-f0-9]{40}|[a-f0-9]{64})$/u.test(entry.objectId)) throw new TransactionRepositoryAuthorityError("invalid-index", new Error("Transaction index entry is invalid"));
      return Object.freeze({ path: row.path, entry: Object.freeze({ stage: entry.stage as number, mode: entry.mode as string, objectId: entry.objectId }) });
    }));
    this.repositoryFences = Object.freeze(sourceAdmissionRepositoryFences(this.indexRows));
    this.indexWitness = sha256(this.indexRows.map((row) => canonicalJson(row)).sort(sourceAdmissionByteCompare).join("\0"));
    Object.freeze(this);
  }

  static owns(value: unknown): value is TransactionRepositoryAuthority {
    return typeof value === "object" && value !== null && #validated in value;
  }
}

function captureTransactionRepositoryAuthority(repoRoot: string): TransactionRepositoryAuthority {
  try {
    sourceAdmissionAssertLexical(repoRoot, "Transaction repository root", true);
    if (!isAbsolute(repoRoot) || resolve(repoRoot) !== repoRoot) throw new Error("Transaction repository root is not absolute and canonical");
    sourceAdmissionDirectoryChain(repoRoot);
  } catch (cause) {
    if (isTransactionRepositoryAuthorityError(cause)) throw cause;
    throw new TransactionRepositoryAuthorityError("invalid-access", cause);
  }
  try { return new TransactionRepositoryAuthority(repoRoot, sourceAdmissionGitRows(repoRoot, taxonomyScopedGitPathspec(undefined, ["compose"]))); }
  catch (cause) {
    if (isTransactionRepositoryAuthorityError(cause)) throw cause;
    throw new TransactionRepositoryAuthorityError("invalid-index", cause);
  }
}

/** 🚧️ "subtree" rejects any overlap (a real recursive filesystem operation could relocate a nested boundary); "point"/"input" reject only paths at-or-below a fence — "input" additionally admits the fence path itself, since a terminal generator-input node is recorded, never descended. */
function assertTransactionRepositoryPath(authority: TransactionRepositoryAuthority, path: string, role: "point" | "subtree" | "input", label: string): void {
  if (!TransactionRepositoryAuthority.owns(authority)) throw new TransactionRepositoryAuthorityError("missing-authority", new Error("Missing captured transaction repository authority"));
  try {
    if (role !== "point" && role !== "subtree" && role !== "input") throw new Error("Transaction repository access role is invalid");
    if (path !== "") sourceAdmissionAssertLexical(path, label, false);
  } catch (cause) {
    if (isTransactionRepositoryAuthorityError(cause)) throw cause;
    throw new TransactionRepositoryAuthorityError("invalid-access", cause);
  }
  const boundary = role === "subtree"
    ? authority.repositoryFences.find((fence) => pathsOverlap(path, fence))
    : sourceAdmissionContainingRepository(path, authority.repositoryFences, role === "point");
  if (boundary !== null && boundary !== undefined) throw new TransactionRepositoryAuthorityError("repository-boundary", new Error(label + " crosses an index-owned repository boundary: " + path + " (" + boundary + ")"));
}

function assertTransactionRepositoryWitness(authority: TransactionRepositoryAuthority, rows: unknown): void {
  if (!TransactionRepositoryAuthority.owns(authority)) throw new TransactionRepositoryAuthorityError("missing-authority", new Error("Missing captured transaction repository authority"));
  if (new TransactionRepositoryAuthority(authority.repoRoot, rows).indexWitness !== authority.indexWitness) throw new TransactionRepositoryAuthorityError("index-drift", new Error("Transaction repository index changed since capture"));
}

function transactionRepositoryBootstrapPaths(plan: TaxonomyPlan, options: TaxonomyApplyOptions, repoRoot: string): Readonly<{
  taxonomyPath: string;
  ticketDir: string;
  planArtifactPath: string;
  resumeJournal?: string;
  accesses: readonly Readonly<{ path: string; role: "point" | "subtree" | "input"; label: string }>[];
}> {
  if (options.repoRoot !== ".") sourceAdmissionAssertLexical(options.repoRoot, "repoRoot", true);
  sourceAdmissionAssertLexical(repoRoot, "repoRoot", true);
  const accesses: { path: string; role: "point" | "subtree" | "input"; label: string }[] = [];
  const add = (path: string, role: "point" | "subtree" | "input", label: string): void => {
    if (path !== "") sourceAdmissionAssertLexical(path, label, false);
    accesses.push({ path, role, label });
  };
  const local = (value: string, label: string): string => {
    sourceAdmissionAssertLexical(value, label, true);
    const path = relative(repoRoot, isAbsolute(value) ? value : join(repoRoot, value)).split(sep).join("/");
    sourceAdmissionAssertLexical(path, label, false);
    add(path, "point", label);
    return path;
  };
  const taxonomyPath = local(options.taxonomyPath ?? TAXONOMY_RELATIVE_PATH, "taxonomyPath");
  const ticketDir = local(options.ticketDir, "ticketDir");
  const planArtifactPath = local(options.planArtifactPath ?? `${ticketDir}/${TICKET_GENERATED_OUTPUT_DIRECTORY}/📊️taxonomy-plan/🔣️.json`, "planArtifactPath");
  const resumeJournal = options.resumeJournal === undefined ? undefined : local(options.resumeJournal, "resumeJournal");
  if (options.cancelFile !== undefined) local(options.cancelFile, "cancelFile");
  if (options.explicitTicketDir !== undefined) add(local(options.explicitTicketDir, "explicitTicketDir"), "subtree", "Explicit source ticket");
  add(plan.scope ?? "", "subtree", "Plan scope");
  for (const move of plan.moves) {
    add(move.sourcePath, "subtree", "Move source"); add(move.destinationPath, "subtree", "Move destination");
    for (const input of move.sourceAuthority?.inputs ?? []) add(input.path, "point", "Move source authority");
    for (const edit of move.referenceEdits) add(edit.path, "point", "Move reference");
  }
  for (const root of plan.embeddedTicketRoots) for (const path of [root.sourceMetadataRoot, root.sourceTicketRoot, root.canonicalTicketRoot]) add(path, "subtree", "Embedded root");
  for (const relocation of plan.embeddedTicketRootRelocations) for (const path of [relocation.sourcePath, relocation.destinationPath]) add(path, "subtree", "Embedded relocation");
  for (const edit of plan.symlinkTargetEdits) {
    add(edit.sourcePath, "subtree", "Symlink source"); add(edit.finalPath, "subtree", "Symlink destination");
    add(edit.logicalTargetSourcePath, "point", "Symlink logical source"); add(edit.logicalTargetFinalPath, "point", "Symlink logical destination");
  }
  for (const removal of plan.evidenceRemovals) {
    add(removal.sourcePath, "subtree", "Evidence removal");
    for (const path of removalAuthorityPaths(removal.authority)) add(path, "point", "Removal authority");
  }
  for (const ancestor of plan.destinationAncestorPreimages) add(ancestor.path, "point", "Destination ancestor metadata");
  for (const edit of plan.edits) add(edit.path, "subtree", "Reference edit");
  for (const regeneration of plan.regenerations) {
    add(regeneration.cwd, "point", "Generator working directory");
    add(regeneration.cwd + "/📋️project.json", "point", "Generator project manifest");
    for (const root of regeneration.outputRoots) add(root, "subtree", "Generator output root");
    for (const input of regeneration.inputs) add(input.path, "input", "Generator input");
    for (const output of [...regeneration.preOutputs, ...regeneration.outputs]) add(output.path, "subtree", "Generator output");
    for (const path of regeneration.staleRemovals) add(path, "subtree", "Generator stale removal");
    for (const node of regeneration.preview.nodes) add(node.path, "subtree", "Generator preview");
    for (const path of regeneration.preview.staleRemovals) add(path, "subtree", "Generator preview stale removal");
  }
  return { taxonomyPath, ticketDir, planArtifactPath, resumeJournal, accesses };
}

/** 🛠️ Applies a digest-verified plan through two-phase staging, journaled backups, verification, cancellation, resume and rollback. */
export function applyTaxonomyPlan(plan: TaxonomyPlan, options: TaxonomyApplyOptions): TaxonomyApplyResult {
  plan = parseTaxonomyPlan(plan);
  const repoRoot = resolve(options.repoRoot);
  if (!PLAN_COMMIT_ID.test(options.expectedBaselineCommit) || options.expectedBaselineCommit !== plan.baselineCommit) throw new Error("Plan baselineCommit does not match expectedBaselineCommit authority");
  if (options.workers !== undefined && (!Number.isSafeInteger(options.workers) || options.workers < 1)) throw new Error("workers must be a positive integer");
  const digest = taxonomyPlanDigest(plan);
  if (plan.planDigest !== digest) throw new Error("Plan digest does not match canonical plan bytes");
  if (options.expectedPlanDigest !== undefined && options.expectedPlanDigest !== digest) throw new Error("Plan digest does not match expectedPlanDigest");
  if (plan.unresolved.some((entry) => entry.severity === "error")) throw new Error("Plan has unresolved blocking violations");
  const bootstrap = transactionRepositoryBootstrapPaths(plan, options, repoRoot);
  const repositoryAuthority = captureTransactionRepositoryAuthority(repoRoot);
  for (const access of bootstrap.accesses) assertTransactionRepositoryPath(repositoryAuthority, access.path, access.role, access.label);
  const taxonomy = loadTaxonomy({ repoRoot, taxonomyPath: absolutePath(repoRoot, bootstrap.taxonomyPath) });
  const transactionProbe = (phase: string, path?: string): void => report(options.progress, "apply", phase, 0, 0, path);
  const ticketRelative = bootstrap.ticketDir;
  if (isExcluded(ticketRelative, taxonomy)) throw new Error(`Ticket directory is opaque: ${ticketRelative}`);
  const ticketRoot = absolutePath(repoRoot, ticketRelative);
  const transactionDirectory = canonicalDirectoryName(taxonomy, "taxonomy-transaction", "taxonomy-transaction");
  const digestDirectory = canonicalDirectoryName(taxonomy, "transaction-digest", digest, "taxonomy-transaction");
  const transactionRootRelative = normalizeRelative(`${ticketRelative}/${transactionDirectory}`);
  assertTransactionRepositoryPath(repositoryAuthority, transactionRootRelative, "subtree", "Taxonomy transaction executor");
  const transactionRelative = normalizeRelative(`${transactionRootRelative}/${digestDirectory}`);
  const attemptsDirectory = canonicalDirectoryName(taxonomy, "transaction-attempts", "attempts", "transaction-digest");
  const stageDirectory = canonicalDirectoryName(taxonomy, "transaction-stage", "stage", "transaction-attempt");
  const backupDirectory = canonicalDirectoryName(taxonomy, "transaction-backup", "backup", "transaction-attempt");
  const leaseDirectory = canonicalDirectoryName(taxonomy, "transaction-lease", "lease", "transaction-attempt");
  const journalWriteDirectory = canonicalDirectoryName(taxonomy, "transaction-journal-write", "journal", "transaction-stage");
  const attemptPreparationName = (ordinal: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-attempt-preparation", `prepare-${ordinal}-${pid}-${token}`, "transaction-attempts");
  const leasePreparationName = (pid: number, token: string, state: "preparing" | "stale"): string => canonicalDirectoryName(taxonomy, "transaction-lease-preparation", `lease-${pid}-${token}-${state}`, "transaction-backup");
  const journalJsonWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-json-write-preparation", `write-${pid}-${token}`, "transaction-journal-write");
  const leaseJsonWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-json-write-preparation", `write-${pid}-${token}`, "transaction-lease-preparation");
  const backupPreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-backup-preparation", `backup-${identity}-${pid}-${token}`, "transaction-backup");
  const backupWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-backup-write-preparation", `write-${pid}-${token}`, "transaction-backup-preparation");
  const restorePreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-restore-preparation", `restore-${identity}-${pid}-${token}`, "transaction-backup");
  const editPreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-edit-preparation", `edit-${identity}-${pid}-${token}`, "transaction-stage");
  const editWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-edit-write-preparation", `write-${pid}-${token}`, "transaction-edit-preparation");
  const attemptsRelative = normalizeRelative(`${transactionRelative}/${attemptsDirectory}`);
  const journalFilename = canonicalKindOnlyFilename(taxonomy, "json", ".json");
  const jsonPreviousName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-json-previous", "transaction-json-write-preparation", ".json");
  const backupWriteCandidateName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-backup-write-candidate", "transaction-backup-write-preparation", ".backup");
  const editWriteCandidateName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-edit-write-candidate", "transaction-edit-write-preparation", ".edit");
  const planBytes = Buffer.from(`${canonicalJson(plan)}\n`);
  const planAuthority = (() => {
    const candidateRelative = bootstrap.planArtifactPath;
    if (options.planArtifactPath) assertLexicalInputOutsideOpaque(repoRoot, options.planArtifactPath, "planArtifactPath", true);
    const candidate = absolutePath(repoRoot, candidateRelative);
    if (!options.planArtifactPath) assertNoFollowAncestors(repoRoot, candidate, "canonical plan artifact", true);
    const stat = lstatOrNull(candidate);
    if (options.planArtifactPath && (!stat?.isFile() || stat.isSymbolicLink() || !readFileSync(candidate).equals(planBytes))) throw new Error("planArtifactPath must be a regular no-follow file containing the exact canonical plan bytes");
    return stat?.isFile() && !stat.isSymbolicLink() && readFileSync(candidate).equals(planBytes) ? { path: candidateRelative, bytes: planBytes } : undefined;
  })();
  const resumeRelative = bootstrap.resumeJournal;
  assertPlanOutsideTransaction(plan, transactionRootRelative, taxonomy, repoRoot);
  if (options.cancelFile) {
    const cancelAbsolute = assertLexicalInputOutsideOpaque(repoRoot, options.cancelFile, "cancelFile", true);
    const cancelRelative = normalizeRelative(relative(repoRoot, cancelAbsolute).replaceAll("\\", "/"));
    const mutationPaths = [
      ...plan.moves.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.embeddedTicketRoots.flatMap((entry) => [entry.sourceMetadataRoot, entry.sourceTicketRoot, entry.canonicalTicketRoot]),
      ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...removalAuthorityPaths(entry.authority)]),
      ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
      ...plan.edits.map((entry) => entry.path),
      ...plan.regenerations.flatMap((entry) => [entry.cwd, ...entry.outputRoots, ...entry.inputs.map((input) => input.path), ...entry.preOutputs.map((output) => output.path), ...entry.outputs.map((output) => output.path), ...entry.staleRemovals]),
    ];
    if (pathsOverlap(cancelRelative, transactionRootRelative) || mutationPaths.some((path) => pathsOverlap(cancelRelative, path))) throw new Error(`cancelFile overlaps transaction or mutation authority: ${cancelRelative}`);
  }
  const existingAttempts: { readonly ordinal: string; readonly attemptRelative: string; readonly journal: MutableJournalRecord; readonly journalRelative: string }[] = [];
  const unpublishedAttempts: { readonly ordinal: string; readonly pid: number; readonly token: string; readonly path: string }[] = [];
  const attemptsAbsolute = absolutePath(repoRoot, attemptsRelative);
  assertNoFollowAncestors(repoRoot, attemptsAbsolute, "transaction attempts root", true);
  const transactionRootAbsolute = absolutePath(repoRoot, transactionRootRelative);
  const transactionRootStat = lstatOrNull(transactionRootAbsolute);
  if (transactionRootStat) {
    if (!transactionRootStat.isDirectory() || transactionRootStat.isSymbolicLink()) throw new Error("Taxonomy transaction root must be a no-follow directory");
    for (const name of readdirSync(transactionRootAbsolute).sort(generatorPathCompare)) {
      const childDigest = splitLeadingEmoji(name).rest;
      if (!PLAN_HASH.test(childDigest) || name !== canonicalDirectoryName(taxonomy, "transaction-digest", childDigest, "taxonomy-transaction")) throw new Error(`Unexpected taxonomy transaction-root entry: ${name}`);
      const stat = lstatOrNull(join(transactionRootAbsolute, name));
      if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction digest must be a no-follow directory: ${name}`);
    }
  }
  const digestAbsolute = absolutePath(repoRoot, transactionRelative);
  const digestStat = lstatOrNull(digestAbsolute);
  if (digestStat) {
    if (!digestStat.isDirectory() || digestStat.isSymbolicLink()) throw new Error("Selected transaction digest must be a no-follow directory");
    const digestChildren = readdirSync(digestAbsolute).sort(generatorPathCompare);
    if (digestChildren.some((name) => name !== attemptsDirectory)) throw new Error("Selected transaction digest contains an unexpected artifact");
  }
  const attemptsStat = lstatOrNull(attemptsAbsolute);
  if (attemptsStat) {
    if (!attemptsStat.isDirectory() || attemptsStat.isSymbolicLink()) throw new Error("Transaction attempts authority must be a no-follow directory");
    for (const name of readdirSync(attemptsAbsolute).sort(generatorPathCompare)) {
      const childSlug = splitLeadingEmoji(name).rest;
      const preparation = /^prepare-([0-9]{6})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(childSlug);
      if (preparation) {
        const ordinal = preparation[1], pid = Number.parseInt(preparation[2], 10), token = preparation[3];
        if (!Number.isSafeInteger(pid) || !TRANSACTION_LEASE_TOKEN.test(token) || name !== attemptPreparationName(ordinal, pid, token)) throw new Error(`Unexpected transaction attempt preparation: ${name}`);
        const path = join(attemptsAbsolute, name);
        const stat = lstatOrNull(path);
        if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction attempt preparation must be a no-follow directory: ${name}`);
        unpublishedAttempts.push({ ordinal, pid, token, path });
        continue;
      }
      const ordinal = childSlug;
      if (!/^[0-9]{6}$/u.test(ordinal) || name !== canonicalDirectoryName(taxonomy, "transaction-attempt", ordinal, "transaction-attempts")) throw new Error(`Unexpected transaction attempt entry: ${name}`);
      const attemptRelative = normalizeRelative(`${attemptsRelative}/${name}`);
      const attemptAbsolute = absolutePath(repoRoot, attemptRelative);
      const attemptStat = lstatOrNull(attemptAbsolute);
      if (!attemptStat?.isDirectory() || attemptStat.isSymbolicLink()) throw new Error(`Transaction attempt must be a no-follow directory: ${attemptRelative}`);
      const childNames = readdirSync(attemptAbsolute).sort(generatorPathCompare);
      if (childNames.some((child) => child !== journalFilename && child !== stageDirectory && child !== backupDirectory && child !== leaseDirectory)) throw new Error(`Transaction attempt contains an unexpected artifact: ${attemptRelative}`);
      const attemptJournalRelative = normalizeRelative(`${attemptRelative}/${journalFilename}`);
      const attemptJournalAbsolute = absolutePath(repoRoot, attemptJournalRelative);
      const journalStat = lstatOrNull(attemptJournalAbsolute);
      const expectedStaging = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
      const expectedBackup = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
      if (journalStat && (!journalStat.isFile() || journalStat.isSymbolicLink())) throw new Error(`Transaction attempt journal must be a regular no-follow file: ${attemptJournalRelative}`);
      const walRoot = join(absolutePath(repoRoot, expectedStaging), journalWriteDirectory);
      const prospectiveJournal = lstatOrNull(walRoot) ? recoverCanonicalJsonCandidates(walRoot, journalFilename, jsonPreviousName, journalJsonWritePreparationName, (candidatePath) => {
        const candidate = readJournal(candidatePath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
        if (candidate.planDigest !== digest || candidate.attemptOrdinal !== ordinal || candidate.stagingRoot !== expectedStaging || candidate.backupRoot !== expectedBackup) throw new Error(`Transaction attempt WAL identity does not match its canonical path: ${attemptJournalRelative}`);
        assertJournalPlanMembership(plan, candidate);
        assertJournalPhaseMembership(plan, candidate);
        assertJournalBackupAuthority(plan, candidate);
      }, true, true, attemptJournalAbsolute) : journalStat ? attemptJournalAbsolute : undefined;
      if (!prospectiveJournal) throw new Error(`Transaction attempt has no recoverable durable journal: ${attemptRelative}`);
      const attemptJournal = readJournal(journalStat ? attemptJournalAbsolute : prospectiveJournal, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
      if (attemptJournal.planDigest !== digest || attemptJournal.attemptOrdinal !== ordinal) throw new Error(`Transaction attempt identity does not match its canonical path: ${attemptJournalRelative}`);
      if (attemptJournal.stagingRoot !== expectedStaging || attemptJournal.backupRoot !== expectedBackup) throw new Error(`Transaction attempt roots do not match ordinal ${ordinal}`);
      const stageStat = lstatOrNull(absolutePath(repoRoot, expectedStaging));
      const backupStat = lstatOrNull(absolutePath(repoRoot, expectedBackup));
      const leaseStat = lstatOrNull(join(attemptAbsolute, leaseDirectory));
      if (stageStat && (!stageStat.isDirectory() || stageStat.isSymbolicLink()) || backupStat && (!backupStat.isDirectory() || backupStat.isSymbolicLink())) throw new Error(`Transaction attempt stage/backup must be direct no-follow directories: ${ordinal}`);
      if (leaseStat && (!leaseStat.isDirectory() || leaseStat.isSymbolicLink())) throw new Error(`Transaction attempt lease must be a direct no-follow directory: ${ordinal}`);
      if (attemptJournal.state !== "rolled-back" && attemptJournal.state !== "committed" && (!stageStat || !backupStat)) throw new Error(`Active transaction attempt is missing stage/backup roots: ${ordinal}`);
      assertJournalPhaseMembership(plan, attemptJournal);
      assertJournalBackupAuthority(plan, attemptJournal);
      existingAttempts.push({ ordinal, attemptRelative, journal: attemptJournal, journalRelative: attemptJournalRelative });
    }
  }
  for (let index = 0; index < existingAttempts.length; index++) {
    if (existingAttempts[index].ordinal !== String(index + 1).padStart(6, "0")) throw new Error("Transaction attempt ordinals are not contiguous");
    if (index < existingAttempts.length - 1 && existingAttempts[index].journal.state !== "rolled-back") throw new Error("Only rolled-back attempts may precede another transaction attempt");
  }
  const unpublishedOrdinals = new Set<string>();
  const publishedOrdinals = new Set(existingAttempts.map((entry) => entry.ordinal));
  const exactNextUnpublishedOrdinal = String(existingAttempts.length + 1).padStart(6, "0");
  for (const preparation of unpublishedAttempts) {
    if (publishedOrdinals.has(preparation.ordinal)) throw new Error(`Transaction attempt preparation collides with canonical ordinal ${preparation.ordinal}`);
    if (preparation.ordinal !== exactNextUnpublishedOrdinal) throw new Error(`Transaction attempt preparation ordinal ${preparation.ordinal} is not exact next ordinal ${exactNextUnpublishedOrdinal}`);
    if (unpublishedOrdinals.has(preparation.ordinal)) throw new Error(`Transaction attempt preparations duplicate ordinal ${preparation.ordinal}`);
    unpublishedOrdinals.add(preparation.ordinal);
  }
  const validateUnpublishedAttempt = (preparation: (typeof unpublishedAttempts)[number]): void => {
    if (transactionLeaseProcessIsAlive(preparation.pid)) throw new Error(`Transaction attempt preparation is active for pid ${preparation.pid}`);
    const allowed = new Set([stageDirectory, backupDirectory, leaseDirectory, journalFilename]);
    const children = readdirSync(preparation.path).sort(generatorPathCompare);
    if (children.some((name) => !allowed.has(name))) throw new Error(`Dead transaction attempt preparation contains unexpected evidence: ${basename(preparation.path)}`);
    const finalAttemptDirectory = canonicalDirectoryName(taxonomy, "transaction-attempt", preparation.ordinal, "transaction-attempts");
    const finalAttempt = normalizeRelative(`${attemptsRelative}/${finalAttemptDirectory}`);
    const assertPreparedIdentity = (prepared: MutableJournalRecord, path: string): void => {
      if (prepared.revision !== 0 || prepared.state !== "prepared" || prepared.planDigest !== digest || prepared.attemptOrdinal !== preparation.ordinal || prepared.stagingRoot !== normalizeRelative(`${finalAttempt}/${stageDirectory}`) || prepared.backupRoot !== normalizeRelative(`${finalAttempt}/${backupDirectory}`)) throw new Error(`Dead attempt preparation journal identity is invalid: ${path}`);
      assertJournalPlanMembership(plan, prepared);
      assertJournalPhaseMembership(plan, prepared);
      assertJournalBackupAuthority(plan, prepared);
    };
    for (const name of children) {
      const path = join(preparation.path, name);
      const stat = lstatOrNull(path);
      if (name === journalFilename) {
        if (!stat?.isFile() || stat.isSymbolicLink()) throw new Error(`Dead attempt preparation journal is not a no-follow file: ${path}`);
        assertPreparedIdentity(readJournal(path, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), path);
      } else {
        if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Dead attempt preparation child is not a no-follow directory: ${path}`);
        if (name === leaseDirectory) {
          recoverCanonicalJsonCandidates(path, journalFilename, jsonPreviousName, leaseJsonWritePreparationName, (candidate) => { const lease = parseTransactionLease(candidate, digest, preparation.ordinal); if (lease.pid !== preparation.pid) throw new Error(`Dead attempt preparation lease pid is invalid: ${path}`); }, false, true);
          if (lstatOrNull(join(path, journalFilename))) {
            const lease = parseTransactionLease(join(path, journalFilename), digest, preparation.ordinal);
            if (lease.pid !== preparation.pid) throw new Error(`Dead attempt preparation lease pid is invalid: ${path}`);
          }
        } else if (name === stageDirectory && readdirSync(path).length > 0) {
          const nested = readdirSync(path).sort(generatorPathCompare);
          if (canonicalJson(nested) !== canonicalJson([journalWriteDirectory])) throw new Error(`Dead attempt preparation stage contains unexpected evidence: ${path}`);
          const wal = join(path, journalWriteDirectory);
          const walStat = lstatOrNull(wal);
          if (!walStat?.isDirectory() || walStat.isSymbolicLink()) throw new Error(`Dead attempt preparation WAL is invalid: ${wal}`);
          recoverCanonicalJsonCandidates(wal, journalFilename, jsonPreviousName, journalJsonWritePreparationName, (candidate) => assertPreparedIdentity(readJournal(candidate, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), candidate), false, true);
          const walChildren = readdirSync(wal).sort(generatorPathCompare);
          if (walChildren.length > 0) {
            if (children.includes(journalFilename)) throw new Error(`Dead attempt preparation has both a durable journal and a pending initial WAL: ${wal}`);
            const walPath = join(wal, journalFilename);
            if (lstatOrNull(walPath)) {
              if (lstatSync(walPath).isSymbolicLink()) throw new Error(`Dead attempt preparation WAL snapshot is not a no-follow file: ${walPath}`);
              assertPreparedIdentity(readJournal(walPath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), walPath);
            }
          }
        } else if (readdirSync(path).length > 0) throw new Error(`Dead attempt preparation executor root is not empty: ${path}`);
      }
    }
  };
  for (const preparation of unpublishedAttempts) validateUnpublishedAttempt(preparation);
  const validateTerminalAttempt = (attempt: (typeof existingAttempts)[number], ownedLease?: TransactionLeaseRecord): void => {
    if (attempt.journal.state !== "rolled-back" && attempt.journal.state !== "committed") return;
    const attemptAbsolute = absolutePath(repoRoot, attempt.attemptRelative);
    const stageAbsolute = absolutePath(repoRoot, attempt.journal.stagingRoot);
    const backupAbsolute = absolutePath(repoRoot, attempt.journal.backupRoot);
    const leaseAbsolute = join(attemptAbsolute, leaseDirectory);
    const leaseStat = lstatOrNull(leaseAbsolute);
    if (leaseStat) {
      const lease = readTransactionLease(leaseAbsolute, journalFilename, digest, attempt.ordinal);
      if (transactionLeaseProcessIsAlive(lease.pid) && (!ownedLease || canonicalJson(lease) !== canonicalJson(ownedLease))) throw new Error(`Terminal transaction attempt is leased by active pid ${lease.pid}`);
    }
    const stageStat = lstatOrNull(stageAbsolute), backupStat = lstatOrNull(backupAbsolute);
    if (!stageStat && !backupStat) {
      if (attempt.journal.state === "committed" && actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed before stale lease recovery: ${attempt.ordinal}`);
      return;
    }
    if (stageStat && (!stageStat.isDirectory() || stageStat.isSymbolicLink()) || backupStat && (!backupStat.isDirectory() || backupStat.isSymbolicLink()) || stageStat && !backupStat) throw new Error(`Terminal transaction attempt has an invalid executor tree: ${attempt.ordinal}`);
    assertRecoveryRootNames(repoRoot, plan, attempt.journal, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName);
    validateLeasePreparationEvidence(backupAbsolute, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attempt.ordinal);
    recoverTransactionBackups(repoRoot, plan, attempt.journal, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, true);
    recoverRestorePreparations(repoRoot, plan, attempt.journal, restorePreparationName, true);
    if (stageStat) {
      recoverReferenceEditPreparations(repoRoot, plan, attempt.journal, editPreparationName, editWritePreparationName, editWriteCandidateName, true);
      reconcileJournalWal(repoRoot, absolutePath(repoRoot, attempt.journalRelative), attempt.journal, plan, taxonomy, true);
    }
    const transient = [...(stageStat ? readdirSync(stageAbsolute) : []), ...readdirSync(backupAbsolute)].some((name) => splitLeadingEmoji(name).rest.startsWith("edit-") || splitLeadingEmoji(name).rest.startsWith("backup-") || splitLeadingEmoji(name).rest.startsWith("restore-") || splitLeadingEmoji(name).rest.startsWith("lease-") || name === journalWriteDirectory);
    if (!transient) {
      if (attempt.journal.state === "rolled-back") cleanupRolledBackTransaction(repoRoot, attempt.journal, plan, true);
      else {
        if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed: ${attempt.ordinal}`);
        cleanupCommittedTransaction(repoRoot, attempt.journal, plan, ticketRoot, true);
      }
    }
  };
  for (const attempt of existingAttempts) validateTerminalAttempt(attempt);
  const closeTerminalAttempt = (attempt: (typeof existingAttempts)[number]): void => {
    if (attempt.journal.state !== "rolled-back" && attempt.journal.state !== "committed") return;
    let terminalJournal = attempt.journal;
    const attemptAbsolute = absolutePath(repoRoot, attempt.attemptRelative);
    const stageAbsolute = absolutePath(repoRoot, attempt.journal.stagingRoot);
    const backupAbsolute = absolutePath(repoRoot, attempt.journal.backupRoot);
    const leaseAbsolute = join(attemptAbsolute, leaseDirectory);
    const hasResidue = Boolean(lstatOrNull(stageAbsolute) || lstatOrNull(backupAbsolute));
    let terminalLease: TransactionLeaseHandle | undefined;
    let createdLeaseBackup = false;
    if (hasResidue || lstatOrNull(leaseAbsolute)) {
      if (!hasResidue && attempt.journal.state === "committed" && actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed before stale lease recovery: ${attempt.ordinal}`);
      if (!lstatOrNull(backupAbsolute)) {
        mkdirSync(backupAbsolute);
        fsyncDirectory(attemptAbsolute);
        createdLeaseBackup = true;
      }
      terminalLease = acquireTransactionLease(repositoryAuthority, attempt.attemptRelative, attempt.journal.backupRoot, leaseDirectory, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attempt.ordinal, (owned) => validateTerminalAttempt(attempt, owned), transactionProbe);
      if (!hasResidue && createdLeaseBackup) durableRemove(backupAbsolute, true);
      terminalJournal = reconcileJournalWal(repoRoot, absolutePath(repoRoot, attempt.journalRelative), terminalJournal, plan, taxonomy);
    }
    transactionRepositoryFinally(() => {
      if (terminalJournal.state === "rolled-back") cleanupRolledBackTransaction(repoRoot, terminalJournal, plan);
      else {
        if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed: ${attempt.ordinal}`);
        cleanupCommittedTransaction(repoRoot, terminalJournal, plan, ticketRoot);
      }
    }, () => {
      if (terminalLease) releaseTransactionLease(repositoryAuthority, `${attempt.attemptRelative}/${leaseDirectory}`, terminalLease);
    });
    if (canonicalJson(readdirSync(attemptAbsolute).sort(generatorPathCompare)) !== canonicalJson([journalFilename])) throw new Error(`Terminal transaction attempt is not closed: ${attempt.ordinal}`);
  };
  let attemptOrdinal: string;
  let journalRelative: string;
  let selectedAttempt: (typeof existingAttempts)[number] | undefined;
  if (resumeRelative) {
    const match = existingAttempts.find((entry) => entry.journalRelative === resumeRelative);
    if (!match) throw new Error(`Resume journal is not an exact existing canonical attempt for plan ${digest}`);
    if (match.journal.state === "rolled-back") throw new Error("Cannot resume journal in state rolled-back");
    selectedAttempt = match;
    attemptOrdinal = match.ordinal;
    journalRelative = match.journalRelative;
  } else {
    const active = existingAttempts.find((entry) => entry.journal.state !== "rolled-back" && entry.journal.state !== "committed");
    if (active) throw new Error(`Transaction attempt ${active.ordinal} is active and must be resumed`);
    if (existingAttempts.some((entry) => entry.journal.state === "committed")) throw new Error("Plan already has a committed transaction attempt");
    const next = existingAttempts.length === 0 ? 1 : Math.max(...existingAttempts.map((entry) => Number.parseInt(entry.ordinal, 10))) + 1;
    if (next > 999999) throw new Error("Transaction attempt ordinal space is exhausted");
    attemptOrdinal = String(next).padStart(6, "0");
    const attemptDirectory = canonicalDirectoryName(taxonomy, "transaction-attempt", attemptOrdinal, "transaction-attempts");
    journalRelative = normalizeRelative(`${attemptsRelative}/${attemptDirectory}/${journalFilename}`);
  }
  const attemptRelative = posix.dirname(journalRelative);
  const journalPath = absolutePath(repoRoot, journalRelative);
  let leaseHandle: TransactionLeaseHandle | undefined;
  const acquireLease = (backupRoot: string, beforePublish?: (owned?: TransactionLeaseRecord) => void): void => {
    leaseHandle = acquireTransactionLease(repositoryAuthority, attemptRelative, backupRoot, leaseDirectory, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attemptOrdinal, beforePublish, transactionProbe);
  };
  const releaseLease = (): void => {
    if (!leaseHandle) return;
    releaseTransactionLease(repositoryAuthority, `${attemptRelative}/${leaseDirectory}`, leaseHandle);
    leaseHandle = undefined;
  };
  for (const edit of plan.symlinkTargetEdits) {
    const localTarget = logicalRepositorySymlinkTargetPath(repoRoot, edit.sourcePath, edit.oldTarget);
    if (localTarget !== edit.logicalTargetSourcePath) throw new Error(`Symlink target authority is not repository-local or does not match its logical source: ${edit.sourcePath}`);
    const linkMoves = plan.moves.filter((move) => move.sourcePath === edit.sourcePath);
    const targetMoves = plan.moves.filter((move) => move.sourcePath === edit.logicalTargetSourcePath);
    const expectedFinalPath = linkMoves.length === 0 ? edit.sourcePath : linkMoves.length === 1 ? linkMoves[0].destinationPath : "";
    const expectedTargetFinalPath = targetMoves.length === 0 ? edit.logicalTargetSourcePath : targetMoves.length === 1 ? targetMoves[0].destinationPath : "";
    if (edit.finalPath !== expectedFinalPath || edit.logicalTargetFinalPath !== expectedTargetFinalPath) throw new Error(`Symlink target projection does not match exact plan moves: ${edit.sourcePath}`);
    const expectedTarget = posix.relative(posix.dirname(expectedFinalPath), expectedTargetFinalPath);
    if (!expectedTarget || expectedTarget !== edit.newTarget || expectedTarget.startsWith("/") || posix.normalize(posix.join(posix.dirname(expectedFinalPath), expectedTarget)) !== expectedTargetFinalPath) throw new Error(`Symlink relative target does not resolve to its frozen logical target: ${edit.sourcePath}`);
    if (edit.logicalTargetPreimage.state === "directory" || edit.windowsLinkType !== "file") throw new Error(`Symlink directory target lacks recursive no-follow authority: ${edit.sourcePath}`);
    if (edit.logicalTargetPreimage.state === "absent" && !resolveFileKind(edit.logicalTargetSourcePath, taxonomy, [], []).kind) throw new Error(`Broken symlink target kind cannot be proven: ${edit.sourcePath}`);
    const targetDigestible = { sourcePath: edit.sourcePath, finalPath: edit.finalPath, oldTarget: edit.oldTarget, newTarget: edit.newTarget, logicalTargetSourcePath: edit.logicalTargetSourcePath, logicalTargetFinalPath: edit.logicalTargetFinalPath, logicalTargetPreimage: edit.logicalTargetPreimage };
    if (edit.sourceTargetDigest !== sha256(canonicalJson(targetDigestible))) throw new Error(`Symlink source-target authority digest changed: ${edit.sourcePath}`);
  }
  execFileSync("git", ["cat-file", "-e", `${plan.baselineCommit}^{commit}`], { cwd: repoRoot, stdio: "ignore" });
  if (plan.excludedTreeDigests.length > 0) throw new Error("Opaque digest filesystem access is disabled; replan with empty excludedTreeDigests");
  if (!options.resumeJournal) checkCancellation(repoRoot, options.cancelFile);
  let preflightReferenceBasis: PreflightReferenceBasis | undefined;
  if (!options.resumeJournal) {
    assertTransactionRepositoryWitness(repositoryAuthority, captureTransactionRepositoryAuthority(repoRoot).indexRows);
    for (const access of bootstrap.accesses) assertTransactionRepositoryPath(repositoryAuthority, access.path, access.role, access.label);
    assertTransactionRepositoryPath(repositoryAuthority, transactionRootRelative, "subtree", "Taxonomy transaction executor");
    if (actualAffectedPreDigest(repoRoot, plan) !== plan.expectedAffectedPreStateDigest) throw new Error("Affected pre-state digest does not match plan expectation");
    for (const path of [...plan.moves.map((entry) => entry.destinationPath), ...plan.embeddedTicketRootRelocations.map((entry) => entry.destinationPath), ...plan.symlinkTargetEdits.map((entry) => entry.finalPath), ...plan.edits.map((entry) => entry.path), ...plan.regenerations.flatMap((entry) => entry.outputRoots)]) assertWritableAncestors(repoRoot, path);
    const referenceInventory = inventoryTaxonomy({ repoRoot, scope: plan.scope, ticketDir: options.explicitTicketDir, taxonomyPath: options.taxonomyPath, cancelFile: options.cancelFile });
    const moveSources = new Set(plan.moves.map((move) => move.sourcePath));
    for (const move of plan.moves) {
      const source = absolutePath(repoRoot, move.sourcePath);
      assertLeafPreimage(repoRoot, move.sourcePath, move.sourcePreimage);
      if (lstatOrNull(absolutePath(repoRoot, move.destinationPath)) && !moveSources.has(move.destinationPath)) throw new Error(`Move destination is occupied: ${move.destinationPath}`);
    }
    for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
      assertLeafPreimage(repoRoot, relocation.sourcePath, relocation.preimage);
      if (lstatOrNull(absolutePath(repoRoot, relocation.destinationPath))) throw new Error(`Embedded relocation destination is occupied: ${relocation.destinationPath}`);
    }
    for (const removal of plan.evidenceRemovals) assertLeafPreimage(repoRoot, removal.sourcePath, removal.preimage);
    for (const removal of plan.evidenceRemovals) {
      if (removal.authority.kind === "byte-and-mode-identical") for (const member of removal.authority.members) assertLeafPreimage(repoRoot, member.sourcePath, member.preimage);
      else if (removal.authority.kind === "serialized-path-sentinel") {
        const fixture = serializedSentinelCases(repoRoot);
        const sentinel = fixture?.cases.find((entry) => entry.id === removal.authority.caseId);
        if (removal.authority.fixturePath !== TRANSACTION_SENTINEL_CASES_FIXTURE_PATH || !fixture || fixture.fixtureContentHash !== removal.authority.fixtureContentHash || !sentinel || sentinel.inputPath !== removal.authority.serializedInputPath || sentinel.physicalSourcePath !== removal.sourcePath || sentinel.expectedViolationCode !== removal.authority.expectedViolationCode || sentinel.sourceContentHash !== removal.preimage.contentHash) throw new Error(`Serialized sentinel authority changed: ${removal.authority.caseId}`);
      } else if (removal.authority.kind === "exact-path-mutation") assertTicketImportantExactRemovalAuthority(repoRoot, removal);
      else assertTicketImportantRemovalAuthority(repoRoot, removal, taxonomy);
    }
    for (const edit of plan.symlinkTargetEdits) {
      const link = absolutePath(repoRoot, edit.sourcePath);
      if (!lstatOrNull(link)?.isSymbolicLink() || readlinkSync(link) !== edit.oldTarget) throw new Error(`Symlink target preimage changed: ${edit.sourcePath}`);
      const logical = lstatOrNull(absolutePath(repoRoot, edit.logicalTargetSourcePath));
      if ((edit.logicalTargetPreimage.state === "absent" && logical) || (edit.logicalTargetPreimage.state === "directory" && !logical?.isDirectory()) || ((edit.logicalTargetPreimage.state === "file" || edit.logicalTargetPreimage.state === "symlink") && (!logical || canonicalJson(leafPathPreimage(absolutePath(repoRoot, edit.logicalTargetSourcePath))) !== canonicalJson(edit.logicalTargetPreimage)))) throw new Error(`Logical symlink target preimage changed: ${edit.logicalTargetSourcePath}`);
    }
    for (const root of plan.embeddedTicketRoots) {
      if (canonicalJson(transactionTreeDigest(repositoryAuthority, root.sourceMetadataRoot, root.sourceMetadataRoot, [])) !== canonicalJson(root.sourceTreeDigest)) throw new Error(`Embedded root tree preimage changed: ${root.sourceMetadataRoot}`);
      const children = [
        ...plan.embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath),
        ...plan.evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath),
      ].sort(generatorPathCompare);
      if (children.some((path) => !path.startsWith(`${root.sourceTicketRoot}/`) || !path.startsWith(`${root.sourceMetadataRoot}/`)) || canonicalJson(transactionTreeDigest(repositoryAuthority, root.sourceMetadataRoot, root.sourceMetadataRoot, children)) !== canonicalJson(root.residualTreeDigest)) throw new Error(`Embedded root residual authority changed: ${root.sourceMetadataRoot}`);
      const incoming = incomingEmbeddedReferences(referenceInventory, root.sourceMetadataRoot).filter((row) => {
        const source = row.split("\u0000")[1];
        if (!planAuthority || source !== planAuthority.path) return true;
        const stat = lstatOrNull(absolutePath(repoRoot, source));
        return !stat?.isFile() || stat.isSymbolicLink() || !readFileSync(absolutePath(repoRoot, source)).equals(planAuthority.bytes);
      });
      if (sha256(`sha256-taxonomy-reference-set-v1\u0000${canonicalJson(incoming)}`) !== root.incomingReferenceDigest || incoming.length > 0) throw new Error(`Embedded root incoming reference set changed: ${root.sourceMetadataRoot}`);
      preflightReferenceBasis ??= capturePreflightReferenceBasis(repoRoot, taxonomy, options.explicitTicketDir, transactionRootRelative, plan, options.cancelFile, options.progress);
      const lexicalIncoming = lexicalTargetIncomingReferences(repoRoot, embeddedTargetPaths(plan, root), [root.sourceMetadataRoot], taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, plan, options.cancelFile, options.progress, undefined, preflightReferenceBasis);
      if (lexicalIncoming.length > 0) throw new Error(`Embedded root structured incoming reference set changed: ${root.sourceMetadataRoot}`);
    }
    for (const removal of plan.evidenceRemovals) {
      preflightReferenceBasis ??= capturePreflightReferenceBasis(repoRoot, taxonomy, options.explicitTicketDir, transactionRootRelative, plan, options.cancelFile, options.progress);
      const project = removal.authority.kind === "nested-cargo-generated-source" || removal.authority.kind === "exact-owner-generated-source" ? removalReferenceProjection(repoRoot, plan) : undefined;
      const incoming = lexicalTargetIncomingReferences(repoRoot, new Set([removal.sourcePath]), removalIncomingIgnoredSourceRoots(removal), taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, plan, options.cancelFile, options.progress, project, preflightReferenceBasis);
      if (incoming.length > 0) throw new Error(`Evidence-removal structured incoming reference set changed: ${removal.sourcePath}`);
    }
    for (const regeneration of plan.regenerations) {
      const actualInputs = regeneration.inputs.map((input) => generatorNodeRecord(repoRoot, input.path, taxonomy));
      if (canonicalJson(actualInputs) !== canonicalJson(regeneration.inputs)) throw new Error(`Regeneration input preimage changed: ${regeneration.id}`);
      const actualOutputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
      if (canonicalJson(actualOutputs) !== canonicalJson(regeneration.preOutputs)) throw new Error(`Regeneration output preimage changed: ${regeneration.id}`);
    }
    checkCancellation(repoRoot, options.cancelFile);
    {
      const authorityInventory = inventoryWithoutTransactionEvidence(referenceInventory, transactionRootRelative, planAuthority?.path);
      const authorityPlan = planTaxonomy(authorityInventory, { baselineCommit: plan.baselineCommit, excludedTreeDigests: [], cancelFile: options.cancelFile, progress: options.progress });
      const operationSets = [
        ["source-tree digest", plan.sourceTreeDigest, authorityPlan.sourceTreeDigest],
        ["affected pre-state digest", plan.expectedAffectedPreStateDigest, authorityPlan.expectedAffectedPreStateDigest],
        ["affected post-state digest", plan.expectedPostStateDigest, authorityPlan.expectedPostStateDigest],
        ["moves", plan.moves, authorityPlan.moves],
        ["embedded roots", plan.embeddedTicketRoots, authorityPlan.embeddedTicketRoots],
        ["embedded relocations", plan.embeddedTicketRootRelocations, authorityPlan.embeddedTicketRootRelocations],
        ["symlink target edits", plan.symlinkTargetEdits, authorityPlan.symlinkTargetEdits],
        ["evidence removals", plan.evidenceRemovals, authorityPlan.evidenceRemovals],
        ["destination ancestor preimages", plan.destinationAncestorPreimages, authorityPlan.destinationAncestorPreimages],
        ["reference edits", plan.edits, authorityPlan.edits],
        ["regenerations", plan.regenerations, authorityPlan.regenerations],
        ["unresolved findings", plan.unresolved, authorityPlan.unresolved],
      ] as const;
      const mismatch = operationSets.find(([, submitted, derived]) => canonicalJson(submitted) !== canonicalJson(derived));
      if (mismatch) throw new Error(`Plan ${mismatch[0]} cannot be rederived exactly from current schema-owned authority`);
    }
  }
  const validateSelectedResumeSnapshot = (ownedLease?: TransactionLeaseRecord): MutableJournalRecord | undefined => {
    if (!selectedAttempt || selectedAttempt.journal.state === "committed" || selectedAttempt.journal.state === "rolled-back") return selectedAttempt?.journal;
    const selectedLeaseRoot = join(absolutePath(repoRoot, selectedAttempt.attemptRelative), leaseDirectory);
    if (lstatOrNull(selectedLeaseRoot)) {
      const selectedLease = readTransactionLease(selectedLeaseRoot, journalFilename, digest, selectedAttempt.ordinal);
      if (transactionLeaseProcessIsAlive(selectedLease.pid) && (!ownedLease || canonicalJson(selectedLease) !== canonicalJson(ownedLease))) throw new Error(`Transaction attempt is leased by active pid ${selectedLease.pid}`);
    }
    const canonicalJournal = absolutePath(repoRoot, selectedAttempt.journalRelative);
    const selectedWal = join(absolutePath(repoRoot, selectedAttempt.journal.stagingRoot), journalWriteDirectory);
    const selectedProspective = lstatOrNull(selectedWal) ? recoverCanonicalJsonCandidates(selectedWal, journalFilename, jsonPreviousName, journalJsonWritePreparationName, (candidatePath) => {
      const candidate = readJournal(candidatePath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
      if (candidate.planDigest !== digest || candidate.attemptOrdinal !== selectedAttempt!.ordinal || candidate.stagingRoot !== selectedAttempt!.journal.stagingRoot || candidate.backupRoot !== selectedAttempt!.journal.backupRoot) throw new Error("Resume WAL identity differs from the selected canonical attempt");
      assertJournalPlanMembership(plan, candidate);
      assertJournalPhaseMembership(plan, candidate);
      assertJournalBackupAuthority(plan, candidate);
    }, true, true, canonicalJournal) : lstatOrNull(canonicalJournal) ? canonicalJournal : undefined;
    if (!selectedProspective) throw new Error("Selected resume attempt has no recoverable durable journal");
    const current = readJournal(lstatOrNull(canonicalJournal) ? canonicalJournal : selectedProspective, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
    const durable = reconcileJournalWal(repoRoot, canonicalJournal, current, plan, taxonomy, true);
    assertRecoveryRootNames(repoRoot, plan, durable, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName);
    validateLeasePreparationEvidence(absolutePath(repoRoot, durable.backupRoot), leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, selectedAttempt.ordinal);
    recoverTransactionBackups(repoRoot, plan, durable, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, true);
    recoverRestorePreparations(repoRoot, plan, durable, restorePreparationName, true);
    recoverReferenceEditPreparations(repoRoot, plan, durable, editPreparationName, editWritePreparationName, editWriteCandidateName, true);
    const tupleProbe: MutableJournalRecord = { ...durable, ...Object.fromEntries(JOURNAL_OPERATION_ARRAYS.map((key) => [key, [...durable[key]]])), backups: { ...durable.backups } } as MutableJournalRecord;
    try {
      if (durable.state === "rolling-back") reconcileRollbackTuples(repoRoot, plan, tupleProbe, taxonomy);
      else reconcileTransactionOwnedTuples(repoRoot, plan, tupleProbe, taxonomy);
    } catch (error) {
      if (!(error instanceof TaxonomyStartedRegenerationPartialError)) throw error;
    }
    for (const root of plan.embeddedTicketRoots) {
      const incoming = lexicalEmbeddedIncomingReferences(repoRoot, plan, root, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: embedded incoming references ${root.sourceMetadataRoot}`);
    }
    for (const removal of plan.evidenceRemovals) {
      const incoming = evidenceRemovalIncomingReferences(repoRoot, removal, plan, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, tupleProbe, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: evidence-removal incoming references ${removal.sourcePath}`);
    }
    return durable;
  };
  validateSelectedResumeSnapshot();
  let lockedResumeJournal: MutableJournalRecord | undefined;
  try {
    if (selectedAttempt && selectedAttempt.journal.state !== "committed") {
      acquireLease(selectedAttempt.journal.backupRoot, validateSelectedResumeSnapshot);
      lockedResumeJournal = validateSelectedResumeSnapshot(leaseHandle?.record);
    }
    for (const preparation of unpublishedAttempts) durableRemove(preparation.path, true);
    for (const attempt of existingAttempts) closeTerminalAttempt(attempt);
  } catch (error) {
    if (isTransactionRepositoryAuthorityError(error)) throw error;
    releaseLease();
    throw error;
  }
  if (options.resumeJournal && selectedAttempt?.journal.state === "committed") {
    if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error("Committed resume post-state digest changed");
    const terminalRoot = absolutePath(repoRoot, selectedAttempt.attemptRelative);
    if (canonicalJson(readdirSync(terminalRoot).sort(generatorPathCompare)) !== canonicalJson([journalFilename])) throw new Error("Committed resume attempt is not terminal journal-only evidence");
    return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
  }
  let journal: MutableJournalRecord;
  if (options.resumeJournal) {
    try {
    if (!selectedAttempt || !lockedResumeJournal) throw new Error("Resume journal has no selected owned-lease snapshot");
    journal = lockedResumeJournal;
    journal = reconcileJournalWal(repoRoot, journalPath, journal, plan, taxonomy);
    journal.probe = transactionProbe;
    if (lockedResumeJournal && canonicalJson(journalSnapshot(journal)) !== canonicalJson(journalSnapshot(lockedResumeJournal))) throw new Error("Resume journal changed after its owned-lease snapshot");
    if (journal.state !== "committed" && journal.state !== "rolled-back") {
      const recovered: MutableJournalRecord = { ...journal, startedRegenerationIds: [...journal.startedRegenerationIds], backups: { ...journal.backups } };
      const recoveredBackups = recoverTransactionBackups(repoRoot, plan, recovered, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
      recoverRestorePreparations(repoRoot, plan, recovered, restorePreparationName);
      recoverReferenceEditPreparations(repoRoot, plan, recovered, editPreparationName, editWritePreparationName, editWriteCandidateName);
      journal = reconcileJournalWal(repoRoot, journalPath, journal, plan, taxonomy);
      if (recoveredBackups) {
        let changed = false;
        for (const [path, backup] of Object.entries(recovered.backups)) {
          if (journal.backups[path]) {
            if (canonicalJson(journal.backups[path]) !== canonicalJson(backup)) throw new Error(`Recovered backup differs from promoted journal authority: ${path}`);
          } else { journal.backups[path] = backup; changed = true; }
        }
        for (const id of recovered.startedRegenerationIds) if (!journal.startedRegenerationIds.includes(id)) { journal.startedRegenerationIds.push(id); changed = true; }
        if (changed) persistJournal(repoRoot, journalPath, journal);
      }
    }
    assertJournalBackupAuthority(plan, journal);
    const expectedStagingRoot = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
    const expectedBackupRoot = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
    if (journal.attemptOrdinal !== attemptOrdinal || journal.stagingRoot !== expectedStagingRoot || journal.backupRoot !== expectedBackupRoot) throw new Error("Resume journal attempt identity and transaction roots do not match the canonical plan attempt");
    assertJournalPlanMembership(plan, journal);
    assertJournalPhaseMembership(plan, journal);
    if (journal.planDigest !== digest) throw new Error("Resume journal belongs to a different plan");
    if (journal.state !== "committed" && journal.state !== "rolled-back") assertActiveTransactionEvidence(repoRoot, plan, journal, false);
    if (journal.state === "committed") {
      if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error("Committed resume post-state digest changed");
      cleanupCommittedTransaction(repoRoot, journal, plan, ticketRoot);
      return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
    }
    if (journal.state === "rolled-back") throw new Error(`Cannot resume journal in state ${journal.state}`);
    if (journal.state === "rolling-back") {
      transactionRepositoryFinally(() => { rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options); }, () => { releaseLease(); });
      return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
    }
    let partialOutput: TaxonomyStartedRegenerationPartialError | undefined;
    try { if (reconcileTransactionOwnedTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal); }
    catch (error) {
      if (!(error instanceof TaxonomyStartedRegenerationPartialError)) throw error;
      partialOutput = error;
    }
    if (partialOutput || cancellationRequested(repoRoot, options.cancelFile)) {
      if (partialOutput) {
        journal.state = "rolling-back";
        journal.error = partialOutput.message;
        persistJournal(repoRoot, journalPath, journal);
      }
      transactionRepositoryFinally(() => { rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options); }, () => { releaseLease(); });
      return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
    }
    for (const root of plan.embeddedTicketRoots) {
      const incoming = lexicalEmbeddedIncomingReferences(repoRoot, plan, root, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: embedded incoming references ${root.sourceMetadataRoot}`);
    }
    for (const removal of plan.evidenceRemovals) {
      const incoming = evidenceRemovalIncomingReferences(repoRoot, removal, plan, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, journal, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: evidence-removal incoming references ${removal.sourcePath}`);
    }
    try {
      if (validateResumeTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
    } catch (error) {
      if (!(error instanceof TaxonomyStartedRegenerationPartialError) && !(error instanceof TaxonomyGeneratorInputDriftError) && !(error instanceof TaxonomyMoveSourceInputDriftError)) throw error;
      journal.state = "rolling-back";
      journal.error = error.message;
      persistJournal(repoRoot, journalPath, journal);
      transactionRepositoryFinally(() => { rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options); }, () => { releaseLease(); });
      return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
    }
    assertActiveTransactionEvidence(repoRoot, plan, journal, true);
    } catch (error) {
      if (isTransactionRepositoryAuthorityError(error)) throw error;
      releaseLease();
      throw error;
    }
  } else {
    const stagingRoot = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
    const backupRoot = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
    journal = { schemaVersion: 2, revision: 0, planDigest: digest, attemptOrdinal, state: "prepared", stagingRoot, backupRoot, journalWriteDirectory, jsonWritePreparationName: journalJsonWritePreparationName, jsonPreviousName, probe: transactionProbe, preparedMoveIds: [], stagedMoveIds: [], installedMoveIds: [], preparedEmbeddedRelocationIds: [], stagedEmbeddedRelocationIds: [], installedEmbeddedRelocationIds: [], preparedEvidenceRemovalIds: [], stagedEvidenceRemovalIds: [], preparedEmbeddedRootIds: [], stagedEmbeddedRootIds: [], preparedSymlinkTargetEditIds: [], stagedSymlinkTargetEditIds: [], installedSymlinkTargetEditIds: [], appliedEditPaths: [], startedRegenerationIds: [], completedRegenerationIds: [], sourceParentPrunePaths: [], backups: {} };
    checkCancellation(repoRoot, options.cancelFile);
    assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, attemptsRelative), "transaction attempts root", true);
    const allocationAncestors = [attemptsAbsolute, absolutePath(repoRoot, transactionRelative), transactionRootAbsolute].map((path) => ({ path, existed: Boolean(lstatOrNull(path)) }));
    const preparationToken = randomUUID();
    const preparationRelative = normalizeRelative(`${attemptsRelative}/${attemptPreparationName(attemptOrdinal, process.pid, preparationToken)}`);
    const preparationRoot = absolutePath(repoRoot, preparationRelative);
    const preparationStage = join(preparationRoot, stageDirectory);
    const preparationBackup = join(preparationRoot, backupDirectory);
    const preparationLease = join(preparationRoot, leaseDirectory);
    const leaseRecord: TransactionLeaseRecord = { schemaVersion: 1, planDigest: digest, attemptOrdinal, token: randomUUID(), pid: process.pid };
    try {
      mkdirSync(attemptsAbsolute, { recursive: true });
      fsyncDirectory(dirname(attemptsAbsolute));
      try { mkdirSync(preparationRoot); }
      catch (error) {
        if ((error as NodeJS.ErrnoException).code === "EEXIST") throw new Error(`Transaction attempt preparation collision at ${preparationRelative}`);
        throw error;
      }
      fsyncDirectory(attemptsAbsolute);
      transactionProbe("transaction-attempt-preparation-mkdir", preparationRelative);
      mkdirSync(preparationStage);
      mkdirSync(preparationBackup);
      mkdirSync(preparationLease);
      transactionProbe("transaction-attempt-preparation-children", preparationRelative);
      publishCanonicalJsonCandidate(preparationLease, journalFilename, jsonPreviousName, leaseRecord, leaseJsonWritePreparationName, undefined, transactionProbe, "transaction-initial-lease-json");
      transactionProbe("transaction-initial-lease-prepared", preparationRelative);
      const initialWalRoot = join(preparationStage, journalWriteDirectory);
      const initialWal = join(initialWalRoot, journalFilename);
      mkdirSync(initialWalRoot);
      transactionProbe("transaction-initial-wal-mkdir", preparationRelative);
      publishCanonicalJsonCandidate(initialWalRoot, journalFilename, jsonPreviousName, journalSnapshot(journal), journalJsonWritePreparationName, undefined, transactionProbe, "transaction-initial-journal");
      durableRename(initialWal, join(preparationRoot, journalFilename));
      transactionProbe("transaction-initial-journal-canonical", preparationRelative);
      durableRemove(initialWalRoot, true);
      fsyncFile(join(preparationRoot, journalFilename));
      fsyncDirectory(preparationStage);
      fsyncDirectory(preparationBackup);
      fsyncDirectory(preparationRoot);
      durableRename(preparationRoot, absolutePath(repoRoot, attemptRelative));
    } catch (error) {
      if (isTransactionRepositoryAuthorityError(error)) throw error;
      if (lstatOrNull(preparationRoot)) durableRemove(preparationRoot, true);
      for (const ancestor of allocationAncestors) if (!ancestor.existed && lstatOrNull(ancestor.path)?.isDirectory() && readdirSync(ancestor.path).length === 0) durableRemove(ancestor.path, true);
      if ((error as NodeJS.ErrnoException).code === "EEXIST" || (error as NodeJS.ErrnoException).code === "ENOTEMPTY") throw new Error(`Transaction attempt allocation race at ${attemptRelative}`);
      throw error;
    }
    leaseHandle = { root: absolutePath(repoRoot, `${attemptRelative}/${leaseDirectory}`), filename: journalFilename, record: leaseRecord };
  }
  const sourceSet = new Set(plan.moves.map((move) => move.sourcePath));
  try {
    if (!options.resumeJournal) transactionProbe("transaction-attempt-canonical-published", attemptRelative);
    checkCancellation(repoRoot, options.cancelFile);
    for (const move of plan.moves) {
      if (journal.stagedMoveIds.includes(move.operationId)) {
        const candidates = journal.installedMoveIds.includes(move.operationId)
          ? [absolutePath(repoRoot, move.destinationPath), join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), absolutePath(repoRoot, move.sourcePath)]
          : [join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), absolutePath(repoRoot, move.sourcePath)];
        const resumedPath = candidates.find((path) => lstatOrNull(path));
        if (!resumedPath) throw new Error(`Resume move state is invalid: ${move.operationId}`);
        const installedLink = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath && journal.installedSymlinkTargetEditIds.includes(edit.operationId));
        if (!journal.appliedEditPaths.includes(move.destinationPath) && canonicalJson(leafPreimage(resumedPath)) !== canonicalJson(retargetedMovePreimage(move, installedLink))) throw new Error(`Resume move preimage changed: ${move.operationId}`);
        continue;
      }
      const source = absolutePath(repoRoot, move.sourcePath);
      const destination = absolutePath(repoRoot, move.destinationPath);
      const sourceStat = lstatOrNull(source);
      if (!sourceStat) throw new Error(`Move source is missing: ${move.sourcePath}`);
      if (canonicalJson(leafPreimage(source)) !== canonicalJson(move.sourcePreimage)) throw new Error(`Move source preimage changed: ${move.sourcePath}`);
      if (lstatOrNull(destination) && !sourceSet.has(move.destinationPath)) throw new Error(`Move destination is occupied: ${move.destinationPath}`);
    }
    if (preflightReferenceBasis) validatePreflightReferenceBasis(preflightReferenceBasis, options.cancelFile, options.progress);
    journal.state = "staging";
    persistJournal(repoRoot, journalPath, journal);
    for (let index = 0; index < plan.moves.length; index++) {
      checkCancellation(repoRoot, options.cancelFile);
      const move = plan.moves[index];
      const stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
      if (!journal.preparedMoveIds.includes(move.operationId)) {
        journal.preparedMoveIds.push(move.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      if (!lstatOrNull(stage) && !journal.installedMoveIds.includes(move.operationId)) {
        mkdirSync(dirname(stage), { recursive: true });
        durableRename(absolutePath(repoRoot, move.sourcePath), stage);
      }
      if (!journal.stagedMoveIds.includes(move.operationId)) {
        journal.stagedMoveIds.push(move.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "staging", index + 1, plan.moves.length, move.sourcePath);
    }
    for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const stage = join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`);
      if (!journal.preparedEmbeddedRelocationIds.includes(relocation.operationId)) { journal.preparedEmbeddedRelocationIds.push(relocation.operationId); persistJournal(repoRoot, journalPath, journal); }
      if (!lstatOrNull(stage) && !journal.installedEmbeddedRelocationIds.includes(relocation.operationId)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(absolutePath(repoRoot, relocation.sourcePath), stage); }
      if (!journal.stagedEmbeddedRelocationIds.includes(relocation.operationId)) { journal.stagedEmbeddedRelocationIds.push(relocation.operationId); persistJournal(repoRoot, journalPath, journal); }
      report(options.progress, "apply", "staging-embedded-relocations", index + 1, plan.embeddedTicketRootRelocations.length, relocation.sourcePath);
      checkCancellation(repoRoot, options.cancelFile);
    }
    for (const [index, removal] of plan.evidenceRemovals.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const stage = join(absolutePath(repoRoot, journal.stagingRoot), `removal-${removal.operationId}`);
      if (!journal.preparedEvidenceRemovalIds.includes(removal.operationId)) { journal.preparedEvidenceRemovalIds.push(removal.operationId); persistJournal(repoRoot, journalPath, journal); }
      if (!lstatOrNull(stage) && !journal.stagedEvidenceRemovalIds.includes(removal.operationId)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(absolutePath(repoRoot, removal.sourcePath), stage); }
      if (!journal.stagedEvidenceRemovalIds.includes(removal.operationId)) { journal.stagedEvidenceRemovalIds.push(removal.operationId); persistJournal(repoRoot, journalPath, journal); }
      report(options.progress, "apply", "staging-evidence-removals", index + 1, plan.evidenceRemovals.length, removal.sourcePath);
      checkCancellation(repoRoot, options.cancelFile);
    }
    injectFailure(options, "after-staging");
    journal.state = "disposing";
    persistJournal(repoRoot, journalPath, journal);
    for (const [index, root] of plan.embeddedTicketRoots.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const stage = join(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`);
      if (!journal.preparedEmbeddedRootIds.includes(root.operationId)) { journal.preparedEmbeddedRootIds.push(root.operationId); persistJournal(repoRoot, journalPath, journal); }
      if (!lstatOrNull(stage) && !journal.stagedEmbeddedRootIds.includes(root.operationId)) { assertDirectoryOnlyTree(absolutePath(repoRoot, root.sourceMetadataRoot)); if (canonicalJson(noFollowTreeDigest(repoRoot, root.sourceMetadataRoot)) !== canonicalJson(root.residualTreeDigest)) throw new Error(`Embedded root residual tree differs from frozen structure: ${root.sourceMetadataRoot}`); mkdirSync(dirname(stage), { recursive: true }); durableRename(absolutePath(repoRoot, root.sourceMetadataRoot), stage); }
      if (!journal.stagedEmbeddedRootIds.includes(root.operationId)) { journal.stagedEmbeddedRootIds.push(root.operationId); persistJournal(repoRoot, journalPath, journal); }
      report(options.progress, "apply", "disposing-embedded-roots", index + 1, plan.embeddedTicketRoots.length, root.sourceMetadataRoot);
      checkCancellation(repoRoot, options.cancelFile);
    }
    injectFailure(options, "after-embedded-root-staging");
    journal.state = "installing";
    persistJournal(repoRoot, journalPath, journal);
    for (let index = 0; index < plan.moves.length; index++) {
      checkCancellation(repoRoot, options.cancelFile);
      const move = plan.moves[index];
      const destination = absolutePath(repoRoot, move.destinationPath);
      const installed = journal.installedMoveIds.includes(move.operationId);
      if (!lstatOrNull(destination)) {
        mkdirSync(dirname(destination), { recursive: true });
        durableRename(join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination);
      }
      if (!installed) {
        journal.installedMoveIds.push(move.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "moves", index + 1, plan.moves.length, move.destinationPath);
    }
    injectFailure(options, "after-moves");
    for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const destination = absolutePath(repoRoot, relocation.destinationPath);
      if (!journal.installedEmbeddedRelocationIds.includes(relocation.operationId)) {
        if (!lstatOrNull(destination)) { mkdirSync(dirname(destination), { recursive: true }); durableRename(join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`), destination); }
        journal.installedEmbeddedRelocationIds.push(relocation.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "installing-embedded-relocations", index + 1, plan.embeddedTicketRootRelocations.length, relocation.destinationPath);
    }
    injectFailure(options, "after-relocations");
    journal.state = "retargeting";
    persistJournal(repoRoot, journalPath, journal);
    for (const [index, edit] of plan.symlinkTargetEdits.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const link = absolutePath(repoRoot, edit.finalPath);
      const stage = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
      if (!journal.preparedSymlinkTargetEditIds.includes(edit.operationId)) { journal.preparedSymlinkTargetEditIds.push(edit.operationId); persistJournal(repoRoot, journalPath, journal); }
      if (!journal.stagedSymlinkTargetEditIds.includes(edit.operationId)) {
        if (!lstatOrNull(stage)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(link, stage); }
        journal.stagedSymlinkTargetEditIds.push(edit.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      if (!journal.installedSymlinkTargetEditIds.includes(edit.operationId)) {
        if (!lstatOrNull(link)) durableSymlink(edit.newTarget, link, process.platform === "win32" ? edit.windowsLinkType : undefined);
        if (readlinkSync(link) !== edit.newTarget) throw new Error(`Symlink retarget verification failed: ${edit.finalPath}`);
        journal.installedSymlinkTargetEditIds.push(edit.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "retargeting-symlinks", index + 1, plan.symlinkTargetEdits.length, edit.finalPath);
      checkCancellation(repoRoot, options.cancelFile);
    }
    injectFailure(options, "after-symlink-retargeting");
    journal.state = "editing";
    persistJournal(repoRoot, journalPath, journal);
    const editGroups = new Map<string, ReferenceEdit[]>();
    for (const edit of plan.edits) editGroups.set(edit.path, [...(editGroups.get(edit.path) ?? []), edit]);
    const sortedEditGroups = [...editGroups.entries()].sort(([a], [b]) => a.localeCompare(b));
    for (let index = 0; index < sortedEditGroups.length; index++) {
      checkCancellation(repoRoot, options.cancelFile);
      const [path, edits] = sortedEditGroups[index];
      if (!journal.appliedEditPaths.includes(path)) {
        const preimages = new Map(edits.map((edit) => [canonicalJson(edit.preimage), edit.preimage]));
        const preimage = [...preimages.values()][0];
        if (preimages.size !== 1 || !preimage || canonicalJson(leafPreimage(absolutePath(repoRoot, path))) !== canonicalJson(preimage)) throw new Error(`Reference edit preimage changed: ${path}`);
        backupPath(repoRoot, path, absolutePath(repoRoot, journal.backupRoot), journal, preimage, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
        persistJournal(repoRoot, journalPath, journal);
        applyReferenceEditAtomically(repoRoot, plan, journal, path, editPreparationName, editWritePreparationName, editWriteCandidateName);
        journal.appliedEditPaths.push(path);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "edits", index + 1, sortedEditGroups.length, path);
    }
    checkCancellation(repoRoot, options.cancelFile);
    injectFailure(options, "after-edits");
    journal.state = "regenerating";
    persistJournal(repoRoot, journalPath, journal);
    for (let index = 0; index < plan.regenerations.length; index++) {
      const regeneration = plan.regenerations[index];
      checkCancellation(repoRoot, options.cancelFile);
      if (validateResumeTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
      if (journal.completedRegenerationIds.includes(regeneration.id)) {
        if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.outputs)) throw new Error(`Completed regeneration output changed: ${regeneration.id}`);
        if (regeneration.verifyCommand) execFileSync(regeneration.verifyCommand[0], [...regeneration.verifyCommand.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), env: { ...process.env }, stdio: "inherit" });
        report(options.progress, "apply", "regenerations", index + 1, plan.regenerations.length, regeneration.contractId);
        continue;
      }
      if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs)) throw new Error(`Regeneration output preimage changed before execution: ${regeneration.id}`);
      if (!journal.startedRegenerationIds.includes(regeneration.id)) {
        for (const output of regeneration.preOutputs) if (output.nodeKind !== "directory") backupPath(repoRoot, output.path, absolutePath(repoRoot, journal.backupRoot), journal, output as TaxonomyBackupExpectedPreimage, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
        journal.startedRegenerationIds.push(regeneration.id);
        persistJournal(repoRoot, journalPath, journal);
      }
      execFileSync(regeneration.command[0], [...regeneration.command.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), env: { ...process.env }, stdio: "inherit" });
      checkCancellation(repoRoot, options.cancelFile);
      const actualOutputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
      if (canonicalJson(actualOutputs) !== canonicalJson(regeneration.outputs)) throw new Error(`Regeneration ${regeneration.id} produced missing, stale, unexpected, byte-different, or mode-different output`);
      durablySyncGeneratorRecords(repoRoot, actualOutputs);
      if (regeneration.verifyCommand) execFileSync(regeneration.verifyCommand[0], [...regeneration.verifyCommand.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), env: { ...process.env }, stdio: "inherit" });
      checkCancellation(repoRoot, options.cancelFile);
      journal.completedRegenerationIds.push(regeneration.id);
      persistJournal(repoRoot, journalPath, journal);
      report(options.progress, "apply", "regenerations", index + 1, plan.regenerations.length, regeneration.contractId);
    }
    injectFailure(options, "after-regenerations");
    checkCancellation(repoRoot, options.cancelFile);
    if ((plan.regenerations.length > 0 || plan.moves.some((move) => move.sourceAuthority !== undefined)) && validateResumeTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
    journal.state = "verifying";
    persistJournal(repoRoot, journalPath, journal);
    injectFailure(options, "before-verify");
    const prunableSourceParents = emptySourceParents(repoRoot, plan, ticketRoot);
    const projectionState = [...projectionPostApplyViolations(repoRoot, plan, taxonomy), ...artifactProjectionPostApplyViolations(repoRoot, plan, taxonomy, new Set(prunableSourceParents))];
    if (projectionState.length > 0) throw new Error(`Projection verification failed: ${projectionState[0].code} at ${projectionState[0].path}`);
    const staleProjectionTokens = projectionStaleViolations(repoRoot, plan, taxonomy, undefined, options.explicitTicketDir);
    if (staleProjectionTokens.length > 0) throw new Error(`Projection verification found ${staleProjectionTokens.length} stale old-hierarchy token(s): ${staleProjectionTokens[0].path}`);
    if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error("Post-state digest does not match plan expectation");
    const oldTargets = new Set<string>([...plan.moves.map((entry) => entry.sourcePath), ...plan.evidenceRemovals.map((entry) => entry.sourcePath)]);
    for (const root of plan.embeddedTicketRoots) for (const path of embeddedTargetPaths(plan, root)) oldTargets.add(path);
    const staleTransactionReferences = lexicalTargetIncomingReferences(repoRoot, oldTargets, [], taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, plan, options.cancelFile, options.progress);
    if (staleTransactionReferences.length > 0) throw new Error(`Post-state contains ${staleTransactionReferences.length} structured reference(s) to disposed source paths`);
    const postInventoryRaw = inventoryTaxonomyWithSourceParentPruning({ repoRoot, scope: plan.scope, ticketDir: options.explicitTicketDir, taxonomyPath: options.taxonomyPath, cancelFile: options.cancelFile }, new Set(prunableSourceParents));
    const exactPlanArtifact = (() => {
      if (!planAuthority) return false;
      const stat = lstatOrNull(absolutePath(repoRoot, planAuthority.path));
      return Boolean(stat?.isFile() && !stat.isSymbolicLink() && readFileSync(absolutePath(repoRoot, planAuthority.path)).equals(planAuthority.bytes));
    })();
    const postInventory = inventoryAfterSourceParentPruning(inventoryWithoutTransactionEvidence(postInventoryRaw, transactionRootRelative, exactPlanArtifact ? planAuthority!.path : undefined), prunableSourceParents);
    const postPlan = planTaxonomy(postInventory, { baselineCommit: plan.baselineCommit, excludedTreeDigests: [], cancelFile: options.cancelFile });
    const pendingPostOperations = postPlan.moves.length + postPlan.embeddedTicketRoots.length + postPlan.embeddedTicketRootRelocations.length + postPlan.symlinkTargetEdits.length + postPlan.evidenceRemovals.length + postPlan.edits.length + postPlan.regenerations.length;
    if (pendingPostOperations > 0 || postPlan.unresolved.some((entry) => entry.severity === "error")) throw new Error(`Post-state does not converge to an empty plan: ${pendingPostOperations} operation(s), ${postPlan.unresolved.length} finding(s)${postPlan.unresolved[0] ? `; first ${postPlan.unresolved[0].code} at ${postPlan.unresolved[0].path}` : ""}`);
    checkCancellation(repoRoot, options.cancelFile);
    if (canonicalJson(emptySourceParents(repoRoot, plan, ticketRoot)) !== canonicalJson(prunableSourceParents)) throw new Error("Exact source-parent pruning set changed before commit");
    journal.sourceParentPrunePaths = [...prunableSourceParents].sort(generatorPathCompare);
    journal.state = "committed";
    persistJournal(repoRoot, journalPath, journal);
    transactionProbe("transaction-source-parent-pruning-committed", sourceRelative(relative(repoRoot, journalPath)));
    cleanupCommittedTransaction(repoRoot, journal, plan, ticketRoot);
    const appliedOperations = plan.moves.length + plan.embeddedTicketRoots.length + plan.embeddedTicketRootRelocations.length + plan.symlinkTargetEdits.length + plan.evidenceRemovals.length + plan.edits.length + plan.regenerations.length;
    report(options.progress, "apply", "complete", appliedOperations, appliedOperations);
    releaseLease();
    return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
  } catch (error) {
    if (isTransactionRepositoryAuthorityError(error)) throw error;
    const failureMessage = error instanceof Error ? error.message : String(error);
    const committedFailure = {};
    try {
      let durable = (() => {
        if (lstatOrNull(journalPath)) return readJournal(journalPath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
        const walRoot = join(absolutePath(repoRoot, journal.stagingRoot), journalWriteDirectory);
        const walStat = lstatOrNull(walRoot);
        if (!walStat?.isDirectory() || walStat.isSymbolicLink()) throw new Error("Taxonomy journal recovery has no canonical journal or direct WAL authority");
        const preparations = readdirSync(walRoot).filter((name) => name !== journalFilename);
        if (preparations.length !== 1) throw new Error("Taxonomy journal recovery has ambiguous previous-image authority");
        const previous = join(walRoot, preparations[0], jsonPreviousName);
        const previousStat = lstatOrNull(previous);
        if (!previousStat?.isFile() || previousStat.isSymbolicLink()) throw new Error("Taxonomy journal recovery previous-image authority is unavailable");
        const recovered = readJournal(previous, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
        if (recovered.planDigest !== digest || recovered.attemptOrdinal !== attemptOrdinal || recovered.stagingRoot !== journal.stagingRoot || recovered.backupRoot !== journal.backupRoot) throw new Error("Taxonomy journal recovery previous-image identity differs from the active attempt");
        assertJournalPlanMembership(plan, recovered);
        assertJournalPhaseMembership(plan, recovered);
        assertJournalBackupAuthority(plan, recovered);
        return recovered;
      })();
      durable.probe = transactionProbe;
      if (durable.state === "committed") throw committedFailure;
      assertRecoveryRootNames(repoRoot, plan, durable, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName);
      validateLeasePreparationEvidence(absolutePath(repoRoot, durable.backupRoot), leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attemptOrdinal);
      const prospective = reconcileJournalWal(repoRoot, journalPath, durable, plan, taxonomy, true);
      recoverTransactionBackups(repoRoot, plan, prospective, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, true);
      recoverRestorePreparations(repoRoot, plan, prospective, restorePreparationName, true);
      recoverReferenceEditPreparations(repoRoot, plan, prospective, editPreparationName, editWritePreparationName, editWriteCandidateName, true);
      durable = reconcileJournalWal(repoRoot, journalPath, durable, plan, taxonomy);
      if (durable.state === "committed") throw committedFailure;
      const recovered: MutableJournalRecord = { ...durable, ...Object.fromEntries(JOURNAL_OPERATION_ARRAYS.map((key) => [key, [...durable[key]]])), backups: { ...durable.backups } } as MutableJournalRecord;
      const recoveredBackups = recoverTransactionBackups(repoRoot, plan, recovered, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
      recoverRestorePreparations(repoRoot, plan, recovered, restorePreparationName);
      recoverReferenceEditPreparations(repoRoot, plan, recovered, editPreparationName, editWritePreparationName, editWriteCandidateName);
      let recoveredTuples = false;
      try { recoveredTuples = reconcileTransactionOwnedTuples(repoRoot, plan, recovered, taxonomy); }
      catch (resumeError) { if (!(resumeError instanceof TaxonomyStartedRegenerationPartialError)) throw resumeError; }
      if (recoveredBackups || recoveredTuples || canonicalJson(journalSnapshot(durable)) !== canonicalJson(journalSnapshot(recovered))) {
        durable = recovered;
        persistJournal(repoRoot, journalPath, durable);
      }
      journal = durable;
      journal.error = failureMessage;
      rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options);
    } catch (rollbackError) {
      if (isTransactionRepositoryAuthorityError(rollbackError)) throw rollbackError;
      if (rollbackError === committedFailure) {
        releaseLease();
        throw error;
      }
      journal.error = `${failureMessage}; rollback failed: ${rollbackError instanceof Error ? rollbackError.message : String(rollbackError)}`;
      try { persistJournal(repoRoot, journalPath, journal); } catch (persistError) { if (isTransactionRepositoryAuthorityError(persistError)) throw persistError; }
      releaseLease();
      throw new Error(journal.error);
    }
    report(options.progress, "apply", "rolled-back", 0, plan.moves.length + plan.embeddedTicketRoots.length + plan.embeddedTicketRootRelocations.length + plan.symlinkTargetEdits.length + plan.evidenceRemovals.length + plan.edits.length + plan.regenerations.length);
    releaseLease();
    return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
  }
}
//#endregion 🚚️Apply API
