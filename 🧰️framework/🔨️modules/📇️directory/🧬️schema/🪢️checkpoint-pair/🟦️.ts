/** 🪢️ Canonical checkpoint pair: the decoded pair, the admitted selection and the admission checks. */

import { type ArtifactFrontier, type ArtifactHash, type DocumentScope } from "../../../📄️document/🪪️scope/🟦️.ts";
import type { PublishedArtifactBlob } from "../🧊️artifact-checkpoints/🟦️.ts";

//#region 🪢️CanonicalCheckpointPair
/** 🪢️ One decoded, length-and-shape-checked canonical pair: its selection identity and its bytes.
 * The bytes are NOT yet proven against the selection's digests — `verifyCanonicalCheckpointPairV1`
 * does that, because hashing needs `crypto.subtle` and this decoder stays synchronous. */
export interface CanonicalCheckpointPairV1 {
  scope: DocumentScope;
  descriptorDigestV1: ArtifactHash;
  activeCheckpointId: ArtifactHash;
  baselineFrontier: ArtifactFrontier;
  pack: PublishedArtifactBlob;
  spr: PublishedArtifactBlob;
  aggregateSha256: ArtifactHash;
  packBytes: Uint8Array;
  sprBytes: Uint8Array;
}

/** 🎫️ Native-admitted checkpoint metadata for pure typed identity decisions. */
export interface AdmittedCheckpointSelectionV1 {
  checkpointId: ArtifactHash;
  descriptorDigestV1: ArtifactHash;
  baselineFrontier: ArtifactFrontier;
  aggregateSha256: ArtifactHash;
}

function sameArtifactHashV1(left: ArtifactHash, right: ArtifactHash): boolean {
  return left.length === right.length && left.every((byte, index) => byte === right[index]);
}

/** 🎫️ Admits a decoded pair only as exactly the checkpoint the hub authorized for this open (the execution-target
 * lease's or the open plan's `checkpoint`): a checkpoint that moved, a changed descriptor or a foreign scope is
 * refused by name, never mounted — the twin of the kernel's `CanonicalCheckpointPairV1::admit`. */
export function admitCanonicalCheckpointPairV1(pair: CanonicalCheckpointPairV1, scope: DocumentScope, expected: AdmittedCheckpointSelectionV1): void {
  if (pair.scope.spaceId !== scope.spaceId || pair.scope.documentId !== scope.documentId) throw new Error("canonical-checkpoint-pair.scope");
  if (!sameArtifactHashV1(pair.activeCheckpointId, expected.checkpointId)) throw new Error("canonical-checkpoint-pair.checkpoint");
  if (!sameArtifactHashV1(pair.descriptorDigestV1, expected.descriptorDigestV1)) throw new Error("canonical-checkpoint-pair.descriptor");
  const baseline = expected.baselineFrontier;
  const frontier = pair.baselineFrontier;
  if (frontier.documentId !== baseline.documentId || frontier.headEditOrdinal !== baseline.headEditOrdinal || frontier.headEditId !== baseline.headEditId || frontier.lastCommitSeq !== baseline.lastCommitSeq || !sameArtifactHashV1(frontier.chainHash, baseline.chainHash)) throw new Error("canonical-checkpoint-pair.baseline");
  if (!sameArtifactHashV1(pair.aggregateSha256, expected.aggregateSha256)) throw new Error("canonical-checkpoint-pair.aggregate");
}

/** 🛟️ Admits a decoded pair only as exactly the checkpoint a hub `RebootstrapRequired` control names — the twin of the kernel's
 * `CanonicalCheckpointPairV1::admit_rebootstrap` (the control carries no aggregate; the digests prove the bytes). */
export function admitCanonicalCheckpointPairForRebootstrapV1(pair: CanonicalCheckpointPairV1, control: { readonly scope: DocumentScope; readonly checkpointId: ArtifactHash; readonly descriptorDigestV1: ArtifactHash; readonly baselineFrontier: ArtifactFrontier }): void {
  if (pair.scope.spaceId !== control.scope.spaceId || pair.scope.documentId !== control.scope.documentId) throw new Error("canonical-checkpoint-pair.scope");
  if (!sameArtifactHashV1(pair.activeCheckpointId, control.checkpointId)) throw new Error("canonical-checkpoint-pair.checkpoint");
  if (!sameArtifactHashV1(pair.descriptorDigestV1, control.descriptorDigestV1)) throw new Error("canonical-checkpoint-pair.descriptor");
  const frontier = pair.baselineFrontier;
  const baseline = control.baselineFrontier;
  if (frontier.documentId !== baseline.documentId || frontier.headEditOrdinal !== baseline.headEditOrdinal || frontier.headEditId !== baseline.headEditId || frontier.lastCommitSeq !== baseline.lastCommitSeq || !sameArtifactHashV1(frontier.chainHash, baseline.chainHash)) throw new Error("canonical-checkpoint-pair.baseline");
}
//#endregion 🪢️CanonicalCheckpointPair
