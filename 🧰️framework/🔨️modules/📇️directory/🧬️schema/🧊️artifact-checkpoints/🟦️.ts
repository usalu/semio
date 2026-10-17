/** 🧊️ Artifact checkpoints: blob references, published checkpoints, retention, the document descriptor validator, document and invite views. */

import { type ArtifactFrontier, type ArtifactHash, type DocumentScope } from "../../../📄️document/🪪️scope/🟦️.ts";
import type { DirectorySpaceRole } from "../🟦️.ts";
import type { CheckpointId, DocumentDescriptor } from "../👀️views/🟦️.ts";

//#region 🔖️Views
export interface ArtifactBlobRef {
  sha256: ArtifactHash;
  byteLength: number;
  storageKey: string;
}

export interface PublishedArtifactBlob {
  sha256: ArtifactHash;
  byteLength: number;
}

export interface PublishedArtifactCheckpoint {
  scope: DocumentScope;
  checkpointId: CheckpointId;
  parentCheckpointId?: CheckpointId;
  descriptorDigestV1: ArtifactHash;
  baselineFrontier: ArtifactFrontier;
  pack: PublishedArtifactBlob;
  spr: PublishedArtifactBlob;
  aggregateSha256: ArtifactHash;
  publishedAtMs: number;
}

export interface ArtifactCheckpoint {
  scope: DocumentScope;
  checkpointId: CheckpointId;
  parentCheckpointId?: CheckpointId;
  descriptorDigestV1: ArtifactHash;
  baselineFrontier: ArtifactFrontier;
  pack: ArtifactBlobRef;
  spr: ArtifactBlobRef;
  aggregateSha256: ArtifactHash;
  publishedAtMs: number;
}

export interface ArtifactRetention {
  scope: DocumentScope;
  retainedCheckpointId: CheckpointId;
  retainedFloor: ArtifactFrontier;
  checkpointLineageHead: CheckpointId;
}

/** 🪪️ Validates immutable descriptor metadata without physical encoding or hashing. */
export function validateDocumentDescriptorV1(descriptor: DocumentDescriptor): void {
  if (descriptor.bootstrapVersion === 0) throw new Error("descriptor.invalid-bootstrap-version");
  if (descriptor.bootstrapFrontier.commitSeq > descriptor.bootstrapFrontier.headSeq) throw new Error("descriptor.invalid-bootstrap-frontier");
  for (const [field, value] of [
    ["space-id", descriptor.spaceId], ["document-id", descriptor.documentId],
    ["artifact-kind", descriptor.artifactKind], ["artifact-schema", descriptor.artifactSchema],
    ["owner-plugin-id", descriptor.owner.pluginId], ["owner-package-id", descriptor.owner.packageId], ["owner-version", descriptor.owner.version],
  ] as const) if (value.length === 0) throw new Error(`descriptor.empty-${field}`);
  for (const [field, value] of [
    ["owner-package-hash", descriptor.owner.packageHash], ["pack-schema-hash", descriptor.packSchemaHash], ["bootstrap-snapshot-hash", descriptor.bootstrapSnapshotHash],
  ] as const) if (!/^[0-9a-f]{64}$/.test(value) || /^0{64}$/.test(value)) throw new Error(`descriptor.invalid-${field}`);
  for (const [field, value, maximum] of [
    ["bootstrap-version", descriptor.bootstrapVersion, 0xffff_ffff],
    ["bootstrap-head-seq", descriptor.bootstrapFrontier.headSeq, Number.MAX_SAFE_INTEGER],
    ["bootstrap-commit-seq", descriptor.bootstrapFrontier.commitSeq, Number.MAX_SAFE_INTEGER],
    ["bootstrap-epoch", descriptor.bootstrapFrontier.epoch, Number.MAX_SAFE_INTEGER],
  ] as const) if (!Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error(`descriptor.invalid-${field}`);
}

export interface DocumentView {
  descriptor: DocumentDescriptor;
  headSeq: number;
  commitSeq: number;
  epoch: number;
}

export interface InviteView {
  id: string;
  spaceId: string;
  role: DirectorySpaceRole;
  createdAtMs: number;
  expiresAtMs: number;
  revoked: boolean;
}
//#endregion 🔖️Views
