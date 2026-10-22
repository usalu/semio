import { type TaxonomyNodeKind } from "../🚪️source-admission/🟦️.ts";
import { type TaxonomyProgress } from "../🏃️operation/🟦️.ts";
import { type SemanticExactDescendantContract, type SemanticDescendantKindNode } from "../🔣️taxonomy/🌳️descendants/🟦️.ts";

/** 📋️ Refuses an absent required taxonomy entry before its fields or paths are consumed. */
export function requireTaxonomyValue<T>(value:T|undefined,context:string):T{if(value===undefined)throw new Error(`Required taxonomy ${context} is missing`);return value;}

export type TaxonomySeverity = "warning" | "error";

export type TaxonomyPackageRole = "configuration" | "declaration" | "registration" | "bootstrap" | "thin-delegation" | "implementation" | "admitted-implementation" | "unresolved" | "not-package";

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
  readonly kind: "exact-owner-current-source-approval-v1";
  readonly revisionId: string;
  readonly revisionDigest: string;
  readonly inputs: readonly Readonly<{ role: "schema" | "catalog" | "approval"; path: string; preimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }> }>[];
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
  readonly command: readonly ["repo", "run", string];
  readonly verifyCommand?: readonly ["repo", "run", string];
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
  | Readonly<{ kind: "owner-manifest-status"; contractId: "ticket-important-markdown-v1"; ownerPath: string; manifestPath: string; manifestPreimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }>; status: "closed"; contentState: "zero-byte"; authorityDigest: string }>;

export interface TaxonomyEvidenceRemoval {
  readonly operationId: string;
  readonly sourcePath: string;
  readonly preimage: TaxonomyLeafPreimage;
  readonly authority: TaxonomyRemovalAuthority;
  readonly embeddedTicketRootId?: string;
  readonly rationaleRule: "redundant-ticket-evidence-v1" | "ticket-important-closed-empty-v1" | "ticket-important-exact-empty-residue-v1" | "exact-owner-generated-source-retirement-v1" | "nested-cargo-generated-source-retirement-v1";
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

export interface TaxonomyInventoryOptions {
  readonly repoRoot: string;
  readonly scope?: string;
  readonly ticketDir?: string;
  readonly cancelFile?: string;
  readonly structuralDirectoryNames?: readonly string[];
  readonly workers?: number;
  readonly progress?: (progress: TaxonomyProgress) => void;
  readonly taxonomyPath?: string;
  readonly baselineCommit?: string;
  readonly excludedTreeDigests?: readonly OpaqueTreeDigest[];
}

export type TaxonomyCapturedSourceRead = (absolutePath: string) => Uint8Array;

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

/** 🗝️ The options of {@link verifyTaxonomyScopes}: {@link TaxonomyInventoryOptions} with the admitted scopes in place of one scope. */
export type TaxonomyScopesOptions = Omit<TaxonomyInventoryOptions, "scope"> & { readonly scopes: readonly string[] };

/** 🏁️ One scope's verdict of {@link verifyTaxonomyScopes}: the violations and cleanliness {@link verifyTaxonomy} reports for that scope. */
export interface TaxonomyScopeVerification {
  readonly scope: string;
  readonly violations: readonly TaxonomyViolation[];
  readonly clean: boolean;
  readonly closure?: string;
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

export type SemanticKindDescendantContract = Omit<SemanticExactDescendantContract, "requiredNodes" | "exclusiveAlternatives"> & Readonly<{
  requiredNodes: readonly SemanticDescendantKindNode[];
  exclusiveAlternatives: readonly Readonly<{ id: string; mode: "exactly-one"; nodes: readonly SemanticDescendantKindNode[] }>[];
}>;

export const TICKET_IMPORTANT_EXACT_MUTATIONS_CATALOG_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/💉️ticket-important-exact-mutations/🔣️.json";

export const TICKET_IMPORTANT_EXACT_GOVERNED_SOURCES = [
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️12/WINDOW-APP-PANEL-CONTRACTS/🧪️window-policy-fixture/🎛️apps/🧪️fixture/🎭️modes/🧪️mode/🪟️windows/🧪️component-window/👥️presence/📌️important.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/📌️important.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/📌️important.md",
] as const;

export const GENERIC_SEMANTIC_STEMS = new Set(["asset", "assets", "component", "components", "descriptor", "glue", "test", "tests", "implementation", "impl", "index", "cases", "vectors"]);

export const WINDOWS_RESERVED = /^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/i;

export const PLAN_HASH = /^[a-f0-9]{64}$/u;

export const PLAN_OPERATION_ID = /^[a-f0-9]{24}$/u;

export const PLAN_COMMIT_ID = /^[a-f0-9]{40}$/u;
