import type { ExactWireMutationEnvelope } from "../../../../🟦️.ts";

const MUTATION_DAG_CAPACITY = 8_192;
const MUTATION_DAG_IDENTIFIER_BYTES = 256;

export const DOCUMENT_BACKBONE_BATCH_LIMITS = {
  maximumBytes: 262_144,
  maximumEnvelopes: MUTATION_DAG_CAPACITY,
  maximumDependenciesPerEnvelope: MUTATION_DAG_CAPACITY,
  maximumTotalDependencies: MUTATION_DAG_CAPACITY,
  maximumTargetSegmentsPerEnvelope: MUTATION_DAG_CAPACITY,
  maximumTotalTargetSegments: MUTATION_DAG_CAPACITY,
  maximumIdentifierBytes: MUTATION_DAG_IDENTIFIER_BYTES,
  maximumSchemaBytes: MUTATION_DAG_IDENTIFIER_BYTES,
  maximumPayloadBytes: 262_144,
} as const;

export type DocumentBackboneBatchLimits = Readonly<{
  maximumBytes: number;
  maximumEnvelopes: number;
  maximumDependenciesPerEnvelope: number;
  maximumTotalDependencies: number;
  maximumTargetSegmentsPerEnvelope: number;
  maximumTotalTargetSegments: number;
  maximumIdentifierBytes: number;
  maximumSchemaBytes: number;
  maximumPayloadBytes: number;
}>;

export class DocumentBackboneBatchError extends Error {
  readonly outcome: "limit" | "malformed";
  readonly reason: string;

  constructor(outcome: "limit" | "malformed", reason: string) {
    super(`document backbone batch: ${reason}`);
    this.name = "DocumentBackboneBatchError";
    this.outcome = outcome;
    this.reason = reason;
  }
}

const U64_MAXIMUM = 0xffff_ffff_ffff_ffffn;

function documentBackboneBatchLimit(name: keyof DocumentBackboneBatchLimits, value: number, maximum: number): number {
  if (!Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error(`document backbone batch: invalid ${name}`);
  return value;
}

function normalizedDocumentBackboneBatchLimits(limits: DocumentBackboneBatchLimits): DocumentBackboneBatchLimits {
  return {
    maximumBytes: documentBackboneBatchLimit("maximumBytes", limits.maximumBytes, DOCUMENT_BACKBONE_BATCH_LIMITS.maximumBytes),
    maximumEnvelopes: documentBackboneBatchLimit("maximumEnvelopes", limits.maximumEnvelopes, MUTATION_DAG_CAPACITY),
    maximumDependenciesPerEnvelope: documentBackboneBatchLimit("maximumDependenciesPerEnvelope", limits.maximumDependenciesPerEnvelope, MUTATION_DAG_CAPACITY),
    maximumTotalDependencies: documentBackboneBatchLimit("maximumTotalDependencies", limits.maximumTotalDependencies, MUTATION_DAG_CAPACITY),
    maximumTargetSegmentsPerEnvelope: documentBackboneBatchLimit("maximumTargetSegmentsPerEnvelope", limits.maximumTargetSegmentsPerEnvelope, MUTATION_DAG_CAPACITY),
    maximumTotalTargetSegments: documentBackboneBatchLimit("maximumTotalTargetSegments", limits.maximumTotalTargetSegments, MUTATION_DAG_CAPACITY),
    maximumIdentifierBytes: documentBackboneBatchLimit("maximumIdentifierBytes", limits.maximumIdentifierBytes, MUTATION_DAG_IDENTIFIER_BYTES),
    maximumSchemaBytes: documentBackboneBatchLimit("maximumSchemaBytes", limits.maximumSchemaBytes, MUTATION_DAG_IDENTIFIER_BYTES),
    maximumPayloadBytes: documentBackboneBatchLimit("maximumPayloadBytes", limits.maximumPayloadBytes, DOCUMENT_BACKBONE_BATCH_LIMITS.maximumPayloadBytes),
  };
}

function documentBackboneWriteU64(out: number[], value: bigint): void {
  if (value < 0n || value > U64_MAXIMUM) throw new Error("document backbone batch: invalid u64");
  let remaining = value;
  do {
    const byte = Number(remaining & 0x7fn);
    remaining >>= 7n;
    out.push(remaining === 0n ? byte : byte | 0x80);
  } while (remaining !== 0n);
}

function documentBackboneWriteBytes(out: number[], value: Uint8Array): void {
  documentBackboneWriteU64(out, BigInt(value.length));
  for (const byte of value) out.push(byte);
}

function documentBackboneWriteText(out: number[], value: string): void {
  documentBackboneWriteBytes(out, new TextEncoder().encode(value));
}

/** 🧷️ Canonically encodes the exact causal batch retained by a document-backbone port. */
export function encodeDocumentBackboneEnvelopeBatchExact(envelopes: readonly ExactWireMutationEnvelope[]): Uint8Array {
  const out: number[] = [];
  documentBackboneWriteU64(out, BigInt(envelopes.length));
  for (const envelope of envelopes) {
    documentBackboneWriteText(out, envelope.mutation_id);
    documentBackboneWriteText(out, envelope.document_id);
    documentBackboneWriteText(out, envelope.actor);
    documentBackboneWriteU64(out, BigInt(envelope.dependencies.length));
    for (const dependency of envelope.dependencies) documentBackboneWriteText(out, dependency);
    if (envelope.observed === null) documentBackboneWriteU64(out, 0n);
    else {
      documentBackboneWriteU64(out, 1n);
      documentBackboneWriteText(out, envelope.observed);
    }
    documentBackboneWriteU64(out, BigInt(envelope.target.length));
    for (const segment of envelope.target) documentBackboneWriteText(out, segment);
    documentBackboneWriteText(out, envelope.diff.schema);
    documentBackboneWriteBytes(out, envelope.diff.payload);
    documentBackboneWriteText(out, envelope.inverse.schema);
    documentBackboneWriteBytes(out, envelope.inverse.payload);
    documentBackboneWriteU64(out, envelope.timestamp.actor);
    documentBackboneWriteU64(out, envelope.timestamp.physical_ms);
    documentBackboneWriteU64(out, envelope.timestamp.logical);
    documentBackboneWriteU64(out, (envelope.transaction === null ? 0n : 1n) | (envelope.verb === null ? 0n : 2n) | (envelope.line === null ? 0n : 4n));
    if (envelope.transaction !== null) {
      documentBackboneWriteText(out, envelope.transaction.id);
      documentBackboneWriteText(out, envelope.transaction.tool);
    }
    if (envelope.verb !== null) documentBackboneWriteText(out, envelope.verb);
    if (envelope.line !== null) documentBackboneWriteText(out, envelope.line);
  }
  return new Uint8Array(out);
}

function readDocumentBackboneEnvelopeBatchAtExact(
  bytes: Uint8Array,
  initialPosition: number,
  suppliedLimits: DocumentBackboneBatchLimits,
  terminal: boolean,
): Readonly<{ envelopes: readonly ExactWireMutationEnvelope[]; position: number; bytes: Uint8Array }> {
  const limits = normalizedDocumentBackboneBatchLimits(suppliedLimits);
  if (!(bytes instanceof Uint8Array)) throw new DocumentBackboneBatchError("malformed", "byte-array");
  if (!Number.isSafeInteger(initialPosition) || initialPosition < 0 || initialPosition > bytes.length) throw new DocumentBackboneBatchError("malformed", "position");
  if (terminal && bytes.length - initialPosition > limits.maximumBytes) throw new DocumentBackboneBatchError("limit", "batch-bytes");
  const maximumPosition = Math.min(bytes.length, initialPosition + limits.maximumBytes);
  const position: [number] = [initialPosition];
  const readU64 = (): bigint => {
    const start = position[0];
    let value = 0n;
    for (let index = 0; index < 10; index++) {
      if (position[0] >= maximumPosition && maximumPosition < bytes.length) throw new DocumentBackboneBatchError("limit", "batch-bytes");
      const byte = bytes[position[0]];
      if (byte === undefined) throw new DocumentBackboneBatchError("malformed", "truncated");
      position[0] += 1;
      const payload = BigInt(byte & 0x7f);
      if (index === 9 && ((byte & 0x80) !== 0 || payload > 1n)) throw new DocumentBackboneBatchError("malformed", "u64-overflow");
      value |= payload << BigInt(index * 7);
      if ((byte & 0x80) === 0) {
        if (position[0] - start > 1 && payload === 0n) throw new DocumentBackboneBatchError("malformed", "nonminimal-varint");
        return value;
      }
    }
    throw new DocumentBackboneBatchError("malformed", "u64-overflow");
  };
  const readCount = (maximum: number, reason: string): number => {
    const value = readU64();
    if (value > BigInt(maximum)) throw new DocumentBackboneBatchError("limit", reason);
    return Number(value);
  };
  const readBytes = (maximum: number, reason: string): Uint8Array => {
    const length = readCount(maximum, reason);
    if (length > maximumPosition - position[0]) {
      if (maximumPosition < bytes.length) throw new DocumentBackboneBatchError("limit", "batch-bytes");
      throw new DocumentBackboneBatchError("malformed", "truncated");
    }
    const value = bytes.slice(position[0], position[0] + length);
    position[0] += length;
    return value;
  };
  const decoder = new TextDecoder("utf-8", { fatal: true });
  const readText = (maximum: number, reason: string): string => {
    try {
      return decoder.decode(readBytes(maximum, reason));
    } catch (error) {
      if (error instanceof DocumentBackboneBatchError) throw error;
      throw new DocumentBackboneBatchError("malformed", "utf8");
    }
  };
  const envelopeCount = readCount(limits.maximumEnvelopes, "envelopes");
  const envelopes: ExactWireMutationEnvelope[] = [];
  let totalDependencies = 0;
  let totalTargetSegments = 0;
  let totalPayloadBytes = 0;
  for (let envelopeIndex = 0; envelopeIndex < envelopeCount; envelopeIndex++) {
    const mutation_id = readText(limits.maximumIdentifierBytes, "identifier-bytes");
    const document_id = readText(limits.maximumIdentifierBytes, "identifier-bytes");
    const actor = readText(limits.maximumIdentifierBytes, "identifier-bytes");
    const dependencyCount = readCount(limits.maximumDependenciesPerEnvelope, "dependencies");
    if (dependencyCount > limits.maximumTotalDependencies - totalDependencies) throw new DocumentBackboneBatchError("limit", "dependencies");
    totalDependencies += dependencyCount;
    const dependencies: string[] = [];
    for (let dependencyIndex = 0; dependencyIndex < dependencyCount; dependencyIndex++) dependencies.push(readText(limits.maximumIdentifierBytes, "identifier-bytes"));
    const observedFlag = readU64();
    if (observedFlag > 1n) throw new DocumentBackboneBatchError("malformed", "observed-flag");
    const observed = observedFlag === 1n ? readText(limits.maximumIdentifierBytes, "identifier-bytes") : null;
    const targetCount = readCount(limits.maximumTargetSegmentsPerEnvelope, "target-segments");
    if (targetCount > limits.maximumTotalTargetSegments - totalTargetSegments) throw new DocumentBackboneBatchError("limit", "target-segments");
    totalTargetSegments += targetCount;
    const target: string[] = [];
    for (let segmentIndex = 0; segmentIndex < targetCount; segmentIndex++) target.push(readText(limits.maximumIdentifierBytes, "identifier-bytes"));
    const diffSchema = readText(limits.maximumSchemaBytes, "schema-bytes");
    const diffPayload = readBytes(limits.maximumPayloadBytes - totalPayloadBytes, "payload-bytes");
    totalPayloadBytes += diffPayload.length;
    const inverseSchema = readText(limits.maximumSchemaBytes, "schema-bytes");
    const inversePayload = readBytes(limits.maximumPayloadBytes - totalPayloadBytes, "payload-bytes");
    totalPayloadBytes += inversePayload.length;
    const timestamp = { actor: readU64(), physical_ms: readU64(), logical: readU64() };
    const flags = readU64();
    if (flags > 0b111n) throw new DocumentBackboneBatchError("malformed", "trailing-flags");
    const transaction = (flags & 0b01n) !== 0n ? { id: readText(limits.maximumIdentifierBytes, "identifier-bytes"), tool: readText(limits.maximumIdentifierBytes, "identifier-bytes") } : null;
    const verb = (flags & 0b10n) !== 0n ? readText(limits.maximumIdentifierBytes, "identifier-bytes") : null;
    const line = (flags & 0b100n) !== 0n ? readText(limits.maximumIdentifierBytes, "identifier-bytes") : null;
    envelopes.push({
      mutation_id,
      document_id,
      actor,
      dependencies,
      observed,
      target,
      diff: { schema: diffSchema, payload: diffPayload },
      inverse: { schema: inverseSchema, payload: inversePayload },
      timestamp,
      transaction,
      verb,
      line,
    });
  }
  if (terminal && position[0] !== bytes.length) throw new DocumentBackboneBatchError("malformed", "trailing-bytes");
  const canonical = encodeDocumentBackboneEnvelopeBatchExact(envelopes);
  const consumed = bytes.subarray(initialPosition, position[0]);
  if (canonical.length !== consumed.length || canonical.some((value, index) => value !== consumed[index])) throw new DocumentBackboneBatchError("malformed", "noncanonical");
  return { envelopes, position: position[0], bytes: consumed.slice() };
}

/** 🔐️ Decodes one bounded canonical causal batch without narrowing any Rust `u64` HLC limb. */
export function decodeDocumentBackboneEnvelopeBatchExact(
  bytes: Uint8Array,
  suppliedLimits: DocumentBackboneBatchLimits = DOCUMENT_BACKBONE_BATCH_LIMITS,
): readonly ExactWireMutationEnvelope[] {
  return readDocumentBackboneEnvelopeBatchAtExact(bytes, 0, suppliedLimits, true).envelopes;
}

/** 🧭 Reads a bounded exact causal batch embedded at `position[0]` and advances to its terminal byte. */
export function readDocumentBackboneEnvelopeBatchExact(
  bytes: Uint8Array,
  position: [number],
  suppliedLimits: DocumentBackboneBatchLimits = DOCUMENT_BACKBONE_BATCH_LIMITS,
): Readonly<{ envelopes: readonly ExactWireMutationEnvelope[]; bytes: Uint8Array }> {
  const decoded = readDocumentBackboneEnvelopeBatchAtExact(bytes, position[0], suppliedLimits, false);
  position[0] = decoded.position;
  return { envelopes: decoded.envelopes, bytes: decoded.bytes };
}

