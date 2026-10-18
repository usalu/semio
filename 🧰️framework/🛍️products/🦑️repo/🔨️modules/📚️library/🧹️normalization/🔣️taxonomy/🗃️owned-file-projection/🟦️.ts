import { parseCurrentSourceApprovals, parseSemanticOwnedDocumentCorrections } from "../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts";
import type { SemanticFacetPrimaryFileProjectionContract } from "../../../🔍️discovery/📜️taxonomy-contracts/🟦️.ts";

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
  readonly referenceOwnerIds: readonly string[];
  readonly generatorOwnerIds: readonly string[];
  readonly authoredDocumentCorrections: ReturnType<typeof parseSemanticOwnedDocumentCorrections>;
  readonly currentSourceRevisions?: ReturnType<typeof parseCurrentSourceApprovals>;
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

export type SemanticOwnedFileProjectionContract = SemanticExactOwnedFileProjectionContract | SemanticFacetPrimaryFileProjectionContract | SemanticHistoryOwnedFileProjectionContract | SemanticLifecycleOwnedFileProjectionContract | SemanticPrimaryOwnedFileProjectionContract;
