/** 🪪️ The identity of one document inside a space and the ledger frontier that names its history, shared by the os directory contract and by every plugin that reads a document through the SDK entry `@semio-tech/framework/document/scope`. */

/** 🪪️ One document of one space. */
export interface DocumentScope {
  spaceId: string;
  documentId: string;
}

export type ArtifactHash = readonly number[];

export const DOCUMENT_OPEN_ID_MAX_BYTES = 256;

/** 🌊️ One point of a document ledger: counters, the tip edit id and the content chain hash. */
export interface ArtifactFrontier {
  documentId: string;
  headEditOrdinal: number;
  headEditId: string;
  lastCommitSeq: number;
  chainHash: ArtifactHash;
}

/** 🌱️ Exact scope-bound empty history, never an invented edit identifier. */
export function artifactFrontierIsGenesisForV1(scope: DocumentScope, frontier: ArtifactFrontier): boolean {
  const text = (value: string): boolean => value.length > 0 && new TextEncoder().encode(value).length <= DOCUMENT_OPEN_ID_MAX_BYTES && !/[\u0000-\u001f\u007f-\u009f\ud800-\udfff]/u.test(value);
  return text(scope.spaceId) && text(scope.documentId) && frontier.documentId === scope.documentId && frontier.headEditOrdinal === 0 && frontier.headEditId === "" && frontier.lastCommitSeq === 0 && frontier.chainHash.length === 32 && frontier.chainHash.every((byte) => byte === 0);
}

/** 🌿️ Normal edited history has exact positive counters and a nonzero authenticated head. */
export function artifactFrontierIsEditedForV1(scope: DocumentScope, frontier: ArtifactFrontier): boolean {
  const text = (value: string): boolean => value.length > 0 && new TextEncoder().encode(value).length <= DOCUMENT_OPEN_ID_MAX_BYTES && !/[\u0000-\u001f\u007f-\u009f\ud800-\udfff]/u.test(value);
  return text(scope.spaceId) && frontier.documentId === scope.documentId && text(frontier.documentId) && text(frontier.headEditId)
    && Number.isSafeInteger(frontier.headEditOrdinal) && frontier.headEditOrdinal > 0 && Number.isSafeInteger(frontier.lastCommitSeq) && frontier.lastCommitSeq > 0
    && frontier.chainHash.length === 32 && frontier.chainHash.every((byte) => Number.isInteger(byte) && byte >= 0 && byte <= 255) && frontier.chainHash.some((byte) => byte !== 0);
}
