import {parseArtifactHashHex} from "../🪪️artifact-hash/🟦️.ts";
import type {AdmittedCheckpointSelectionV1,DocumentOpenCheckpointV1} from "../../../🧬️schema/🟦️.ts";
/** 🪢️ Native binary checkpoint framing; pure admission lives in Directory schema. */
import { artifactFrontierIsGenesisForV1, artifactFrontierIsEditedForV1, DOCUMENT_OPEN_ID_MAX_BYTES, type ArtifactHash, type ArtifactFrontier, type DocumentScope, type PublishedArtifactBlob, type CanonicalCheckpointPairV1 } from "../../../🧬️schema/🟦️.ts";

/** 🪢️ The hub's canonical-checkpoint-pair media type — the ONE transport that hands a cold client
 * the published pack+SPR pair of a document's active checkpoint (`GET
 * /spaces/{spaceId}/documents/{documentId}/active-checkpoint/pair`). The socket's `Welcome` never
 * carries a pair: its bootstrap plan is computed by the database replay, whose only outcomes are
 * `None`, `Tail` and the database-private `Snapshot` an artifact client refuses by contract. */
export const CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1 = "application/vnd.semio.canonical-checkpoint-pair.v1";
export const CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES = 16 * 1024;
export const CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES = 4 * 1024;
export const CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS = 16_384;
export const CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES = 64 * 1024 * 1024;
const CANONICAL_CHECKPOINT_PAIR_FORMAT_VERSION = 1;
const CANONICAL_CHECKPOINT_PAIR_HEADER = 1;
const CANONICAL_CHECKPOINT_PAIR_DATA = 2;
const CANONICAL_CHECKPOINT_PAIR_TERMINAL = 3;
const CANONICAL_CHECKPOINT_PAIR_PART_PACK = 1;
const CANONICAL_CHECKPOINT_PAIR_PART_SPR = 2;
const CANONICAL_CHECKPOINT_PAIR_TERMINAL_COMPLETE = 0;

class CanonicalPairCursor {
  private readonly bytes: Uint8Array;
  offset: number;
  constructor(bytes: Uint8Array, offset = 0) {
    this.bytes = bytes;
    this.offset = offset;
  }

  get exhausted(): boolean {
    return this.offset === this.bytes.length;
  }

  take(length: number): Uint8Array {
    const end = this.offset + length;
    if (!Number.isSafeInteger(end) || end > this.bytes.length) throw new Error("canonical-checkpoint-pair.truncated");
    const value = this.bytes.subarray(this.offset, end);
    this.offset = end;
    return value;
  }

  byte(): number {
    return this.take(1)[0] as number;
  }

  u32(): number {
    const slice = this.take(4);
    return (((slice[0] as number) << 24) >>> 0) + ((slice[1] as number) << 16) + ((slice[2] as number) << 8) + (slice[3] as number);
  }

  u64(): number {
    const slice = this.take(8);
    let value = 0n;
    for (const byte of slice) value = (value << 8n) | BigInt(byte);
    if (value > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("canonical-checkpoint-pair.capacity");
    return Number(value);
  }

  hash(): ArtifactHash {
    return Array.from(this.take(32));
  }

  /** #️⃣ A hash the hub refuses to publish as all-zero: an unset digest must never read as a match. */
  digest(): ArtifactHash {
    const hash = this.hash();
    if (hash.every((byte) => byte === 0)) throw new Error("canonical-checkpoint-pair.zero-digest");
    return hash;
  }

  text(maximum: number): string {
    const length = this.u32();
    if (length > maximum) throw new Error("canonical-checkpoint-pair.oversized-text");
    return new TextDecoder("utf-8", { fatal: true }).decode(this.take(length));
  }

  frame(maximum: number): CanonicalPairCursor {
    const length = this.u32();
    if (length === 0 || length > maximum) throw new Error("canonical-checkpoint-pair.frame-length");
    return new CanonicalPairCursor(this.take(length));
  }
}

/** 🧮️ Exact record count the hub emits for a pair of these two lengths — pack records first, then
 * SPR records, each `CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES` except the last of each part. */
function canonicalCheckpointPairRecordCountV1(packLength: number, sprLength: number): number {
  return Math.ceil(packLength / CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES) + Math.ceil(sprLength / CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES);
}

/** 🪢️ Decodes the hub's canonical-checkpoint-pair body, the exact inverse of the hub's
 * `append_canonical_pair_{header,data,terminal}`: a stream of `u32be length | payload` frames whose
 * first payload is the selection header, whose middle payloads carry strictly ordered pack-then-SPR
 * data records at contiguous offsets, and whose last payload is the `Complete` terminal. Every
 * refusal is named; a partial, reordered, over-long or unterminated body is never half-accepted. */
export function decodeCanonicalCheckpointPairV1(input: Uint8Array): CanonicalCheckpointPairV1 {
  const maximumWire = CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES + CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES + CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS * 22 + 6;
  if (input.length > maximumWire) throw new Error("canonical-checkpoint-pair.oversized");
  const stream = new CanonicalPairCursor(input);
  const header = stream.frame(CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES);
  if (header.byte() !== CANONICAL_CHECKPOINT_PAIR_HEADER || header.u32() !== CANONICAL_CHECKPOINT_PAIR_FORMAT_VERSION) throw new Error("canonical-checkpoint-pair.header");
  const scope: DocumentScope = { spaceId: header.text(DOCUMENT_OPEN_ID_MAX_BYTES), documentId: header.text(DOCUMENT_OPEN_ID_MAX_BYTES) };
  const descriptorDigestV1 = header.digest();
  const activeCheckpointId = header.digest();
  const baselineFrontier: ArtifactFrontier = {
    documentId: header.text(DOCUMENT_OPEN_ID_MAX_BYTES),
    headEditOrdinal: header.u64(),
    headEditId: header.text(DOCUMENT_OPEN_ID_MAX_BYTES),
    lastCommitSeq: header.u64(),
    chainHash: header.hash(),
  };
  const pack: PublishedArtifactBlob = { sha256: header.digest(), byteLength: header.u64() };
  const spr: PublishedArtifactBlob = { sha256: header.digest(), byteLength: header.u64() };
  const aggregateSha256 = header.digest();
  if (!header.exhausted) throw new Error("canonical-checkpoint-pair.header-trailing-bytes");
  if (!artifactFrontierIsGenesisForV1(scope, baselineFrontier) && !artifactFrontierIsEditedForV1(scope, baselineFrontier)) throw new Error("canonical-checkpoint-pair.baseline-frontier");
  if (pack.byteLength + spr.byteLength > CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES || pack.byteLength === 0 || spr.byteLength === 0) throw new Error("canonical-checkpoint-pair.pair-length");
  const records = canonicalCheckpointPairRecordCountV1(pack.byteLength, spr.byteLength);
  if (records > CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS) throw new Error("canonical-checkpoint-pair.record-count");
  const packBytes = new Uint8Array(pack.byteLength);
  const sprBytes = new Uint8Array(spr.byteLength);
  let packFilled = 0;
  let sprFilled = 0;
  for (let ordinal = 0; ordinal < records; ordinal += 1) {
    const record = stream.frame(CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES + 18);
    if (record.byte() !== CANONICAL_CHECKPOINT_PAIR_DATA) throw new Error("canonical-checkpoint-pair.record-tag");
    const part = record.byte();
    if (record.u32() !== ordinal) throw new Error("canonical-checkpoint-pair.record-ordinal");
    const offset = record.u64();
    const length = record.u32();
    if (length === 0 || length > CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES) throw new Error("canonical-checkpoint-pair.record-length");
    const bytes = record.take(length);
    if (!record.exhausted) throw new Error("canonical-checkpoint-pair.record-trailing-bytes");
    if (part === CANONICAL_CHECKPOINT_PAIR_PART_PACK && packFilled < pack.byteLength) {
      if (offset !== packFilled || packFilled + length > pack.byteLength) throw new Error("canonical-checkpoint-pair.record-offset");
      packBytes.set(bytes, packFilled);
      packFilled += length;
    } else if (part === CANONICAL_CHECKPOINT_PAIR_PART_SPR && packFilled === pack.byteLength) {
      if (offset !== sprFilled || sprFilled + length > spr.byteLength) throw new Error("canonical-checkpoint-pair.record-offset");
      sprBytes.set(bytes, sprFilled);
      sprFilled += length;
    } else throw new Error("canonical-checkpoint-pair.record-part");
  }
  const terminal = stream.frame(2);
  if (terminal.byte() !== CANONICAL_CHECKPOINT_PAIR_TERMINAL || terminal.byte() !== CANONICAL_CHECKPOINT_PAIR_TERMINAL_COMPLETE) throw new Error("canonical-checkpoint-pair.terminal");
  if (!stream.exhausted || packFilled !== pack.byteLength || sprFilled !== spr.byteLength) throw new Error("canonical-checkpoint-pair.incomplete");
  return { scope, descriptorDigestV1, activeCheckpointId, baselineFrontier, pack, spr, aggregateSha256, packBytes, sprBytes };
}


/** 🎫️ Admits externally textual checkpoint identities before the pure pair decision. */
export function admitCheckpointSelectionV1(expected: DocumentOpenCheckpointV1): AdmittedCheckpointSelectionV1 {
  const checkpointId = parseArtifactHashHex(expected.checkpointId);
  if (!checkpointId) throw new Error("canonical-checkpoint-pair.checkpoint");
  const descriptorDigestV1 = parseArtifactHashHex(expected.descriptorDigestV1);
  if (!descriptorDigestV1) throw new Error("canonical-checkpoint-pair.descriptor");
  const aggregateSha256 = parseArtifactHashHex(expected.aggregateSha256);
  if (!aggregateSha256) throw new Error("canonical-checkpoint-pair.aggregate");
  return {checkpointId,descriptorDigestV1,baselineFrontier:expected.baselineFrontier,aggregateSha256};
}
