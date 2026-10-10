//! 🔗️ Causal binary representation implements the original replication domain contracts.
import type { MutationEnvelope, WireMutationEnvelope, ExactWireMutationEnvelope, WireFrontierSummary } from "../../../🟦️.ts";
import { writeVarintU64, readVarintU64, writeVarintU64Exact, writeStr, readStr, writeBytes, readBytes, writeHash32, readHash32, writeBool, readBool } from "../🟦️.ts";

/** 📦️ Owned interface to the host-selected schema-less pack implementation. */
export interface ReplicationPackCodec {
  encode(value: unknown): Uint8Array;
  decode(bytes: Uint8Array): unknown;
}

/** 🌉️ Maps the actor-protocol {@link MutationEnvelope} into a {@link WireMutationEnvelope}. */
export function mutationEnvelopeToWire(envelope: MutationEnvelope, timestamp: WireMutationEnvelope["timestamp"], codec: ReplicationPackCodec): WireMutationEnvelope {
  const packPayload = (value: unknown): number[] => Array.from(codec.encode(value));
  return {
    mutation_id: envelope.id,
    document_id: envelope.document,
    actor: envelope.actor,
    dependencies: [...(envelope.deps ?? [])],
    observed: null,
    target: [],
    diff: { schema: envelope.diff.schemaId, payload: packPayload(envelope.diff.payload) },
    inverse: { schema: envelope.inverse.inverseDiff.schemaId, payload: packPayload(envelope.inverse.inverseDiff.payload) },
    timestamp,
    transaction: envelope.transaction ?? null,
    verb: envelope.verb ?? null,
    line: null,
  };
}

/** 🌉️ Inverse of {@link mutationEnvelopeToWire}. */
export function mutationEnvelopeFromWire(envelope: WireMutationEnvelope, codec: ReplicationPackCodec): MutationEnvelope {
  const decodePayload = (bytes: readonly number[]) => codec.decode(new Uint8Array(bytes));
  const payload = decodePayload(envelope.diff.payload);
  const sequenceNumber = payload !== null && typeof payload === "object" && "sequenceNumber" in payload ? Number((payload as Record<string, unknown>).sequenceNumber) : 0;
  return {
    id: envelope.mutation_id,
    actor: envelope.actor,
    document: envelope.document_id,
    schemaVersion: envelope.diff.schema,
    deps: [...envelope.dependencies],
    payloadHash: "",
    diff: { schemaId: envelope.diff.schema, payload },
    inverse: {
      targetOperation: envelope.mutation_id,
      inverseDiff: { schemaId: envelope.inverse.schema, payload: decodePayload(envelope.inverse.payload) },
      baseVersion: Number.isFinite(sequenceNumber) ? Math.max(0, sequenceNumber) : 0,
      dependencies: [],
      undoPolicy: "exactBaseOnly",
    },
    ...(envelope.transaction === null ? {} : { transaction: envelope.transaction }),
    ...(envelope.verb === null ? {} : { verb: envelope.verb }),
  };
}

//#region 🔖️Combinators
export function writeOptStr(out: number[], value: string | null): void {
  writeBool(out, value !== null);
  if (value !== null) writeStr(out, value);
}
export function readOptStr(bytes: Uint8Array, pos: [number]): string | null {
  return readBool(bytes, pos) ? readStr(bytes, pos) : null;
}
export function writeOptBytes(out: number[], value: readonly number[] | null): void {
  writeBool(out, value !== null);
  if (value !== null) writeBytes(out, value);
}
export function readOptBytes(bytes: Uint8Array, pos: [number]): number[] | null {
  return readBool(bytes, pos) ? readBytes(bytes, pos) : null;
}
export function writeOptFrontier(out: number[], value: WireFrontierSummary | null): void {
  writeBool(out, value !== null);
  if (value !== null) encodeFrontier(out, value);
}
export function readOptFrontier(bytes: Uint8Array, pos: [number]): WireFrontierSummary | null {
  return readBool(bytes, pos) ? decodeFrontier(bytes, pos) : null;
}
export function writeVecStr(out: number[], values: readonly string[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) writeStr(out, value);
}
export function readVecStr(bytes: Uint8Array, pos: [number]): string[] {
  const count = readVarintU64(bytes, pos);
  const result: string[] = [];
  for (let i = 0; i < count; i++) result.push(readStr(bytes, pos));
  return result;
}
export function writeVecEnvelope(out: number[], values: readonly WireMutationEnvelope[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) encodeEnvelope(out, value);
}
export function readVecEnvelope(bytes: Uint8Array, pos: [number]): WireMutationEnvelope[] {
  const count = readVarintU64(bytes, pos);
  const result: WireMutationEnvelope[] = [];
  for (let i = 0; i < count; i++) result.push(decodeEnvelope(bytes, pos));
  return result;
}
//#endregion 🔖️Combinators

//#region 🔖️EnvelopeCodec
/** 🎞️ `actor varint | physical_ms varint | logical varint` — the TS twin of `protocol_causal`'s
 * private `encode_hlc`. */
export function encodeHlc(out: number[], hlc: { readonly actor: number; readonly physical_ms: number; readonly logical: number }): void {
  writeVarintU64(out, hlc.actor);
  writeVarintU64(out, hlc.physical_ms);
  writeVarintU64(out, hlc.logical);
}
export function decodeHlc(bytes: Uint8Array, pos: [number]): { readonly actor: number; readonly physical_ms: number; readonly logical: number } {
  const actor = readVarintU64(bytes, pos);
  const physical_ms = readVarintU64(bytes, pos);
  const logical = readVarintU64(bytes, pos);
  return { actor, physical_ms, logical };
}

/** 🎯️ `mutation_id str | document_id str | actor str | dependencies vec<str> | observed (0 | 1 str) |
 * target vec<str> | diff.schema str | diff.payload bytes | inverse.schema str | inverse.payload bytes | hlc |
 * trailing flags varint (bit 0 transaction, bit 1 verb, bit 2 line) | [transaction id str tool str] | [verb str] | [line str]` — the TS twin of
 * Rust `protocol_causal::encode_envelope`. */
export function encodeEnvelope(out: number[], envelope: WireMutationEnvelope): void {
  encodeEnvelopeFields(out, envelope, () => encodeHlc(out, envelope.timestamp));
}

/** 🔬️ {@link encodeEnvelope} over an exact envelope: every HLC field keeps its whole u64 (a replica's HLC actor is random
 * entropy, far past 2^53). */
export function encodeExactEnvelope(out: number[], envelope: ExactWireMutationEnvelope): void {
  encodeEnvelopeFields(out, envelope, () => {
    writeVarintU64Exact(out, envelope.timestamp.actor);
    writeVarintU64Exact(out, envelope.timestamp.physical_ms);
    writeVarintU64Exact(out, envelope.timestamp.logical);
  });
}

/** 🗃️ Every envelope field in wire order, the HLC written by `hlc`. */
export function encodeEnvelopeFields(out: number[], envelope: Omit<WireMutationEnvelope, "timestamp" | "diff" | "inverse"> & Readonly<{ diff: Readonly<{ schema: string; payload: readonly number[] | Uint8Array }>; inverse: Readonly<{ schema: string; payload: readonly number[] | Uint8Array }> }>, hlc: () => void): void {
  writeStr(out, envelope.mutation_id);
  writeStr(out, envelope.document_id);
  writeStr(out, envelope.actor);
  writeVecStr(out, envelope.dependencies);
  if (envelope.observed === null) writeVarintU64(out, 0);
  else {
    writeVarintU64(out, 1);
    writeStr(out, envelope.observed);
  }
  writeVecStr(out, envelope.target);
  writeStr(out, envelope.diff.schema);
  writeBytes(out, envelope.diff.payload);
  writeStr(out, envelope.inverse.schema);
  writeBytes(out, envelope.inverse.payload);
  hlc();
  writeVarintU64(out, (envelope.transaction === null ? 0 : 1) | (envelope.verb === null ? 0 : 2) | (envelope.line === null ? 0 : 4));
  if (envelope.transaction !== null) {
    writeStr(out, envelope.transaction.id);
    writeStr(out, envelope.transaction.tool);
  }
  if (envelope.verb !== null) writeStr(out, envelope.verb);
  if (envelope.line !== null) writeStr(out, envelope.line);
}

/** 🎯️ Inverse of {@link encodeEnvelope} — the TS twin of Rust `protocol_causal::decode_envelope`. */
export function decodeEnvelope(bytes: Uint8Array, pos: [number]): WireMutationEnvelope {
  const mutation_id = readStr(bytes, pos);
  const document_id = readStr(bytes, pos);
  const actor = readStr(bytes, pos);
  const dependencies = readVecStr(bytes, pos);
  const observedFlag = readVarintU64(bytes, pos);
  if (observedFlag !== 0 && observedFlag !== 1) throw new Error(`mutation envelope: observed flag ${observedFlag}`);
  const observed = observedFlag === 1 ? readStr(bytes, pos) : null;
  const target = readVecStr(bytes, pos);
  const diffSchema = readStr(bytes, pos);
  const diffPayload = readBytes(bytes, pos);
  const inverseSchema = readStr(bytes, pos);
  const inversePayload = readBytes(bytes, pos);
  const timestamp = decodeHlc(bytes, pos);
  const flags = readVarintU64(bytes, pos);
  if (flags > 0b111) throw new Error(`mutation envelope: trailing flags ${flags}`);
  const transaction = (flags & 0b01) !== 0 ? { id: readStr(bytes, pos), tool: readStr(bytes, pos) } : null;
  const verb = (flags & 0b10) !== 0 ? readStr(bytes, pos) : null;
  const line = (flags & 0b100) !== 0 ? readStr(bytes, pos) : null;
  return { mutation_id, document_id, actor, dependencies, observed, target, diff: { schema: diffSchema, payload: diffPayload }, inverse: { schema: inverseSchema, payload: inversePayload }, timestamp, transaction, verb, line };
}

/** 🎯️ `document_id str | head_edit_ordinal varint | head_edit_id str | last_commit_seq varint |
 * chain_hash 32` — the TS twin of Rust `protocol_causal::encode_frontier`. */
export function encodeFrontier(out: number[], frontier: WireFrontierSummary): void {
  writeStr(out, frontier.document_id);
  writeVarintU64(out, frontier.head_edit_ordinal);
  writeStr(out, frontier.head_edit_id);
  writeVarintU64(out, frontier.last_commit_seq);
  writeHash32(out, frontier.chain_hash);
}

/** 🎯️ Inverse of {@link encodeFrontier} — the TS twin of Rust `protocol_causal::decode_frontier`. */
export function decodeFrontier(bytes: Uint8Array, pos: [number]): WireFrontierSummary {
  const document_id = readStr(bytes, pos);
  const head_edit_ordinal = readVarintU64(bytes, pos);
  const head_edit_id = readStr(bytes, pos);
  const last_commit_seq = readVarintU64(bytes, pos);
  const chain_hash = readHash32(bytes, pos);
  return { document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash };
}
export function decodeCausalEnvelopeBatch(bytes: readonly number[], codec: ReplicationPackCodec): MutationEnvelope[] {
  const pos: [number] = [0];
  return readVecEnvelope(new Uint8Array(bytes), pos).map((envelope) => mutationEnvelopeFromWire(envelope, codec));
}

export function encodeCausalEnvelopeBatch(envelopes: readonly MutationEnvelope[], codec: ReplicationPackCodec): readonly number[] {
  const out: number[] = [];
  writeVecEnvelope(out, envelopes.map((envelope, index) => mutationEnvelopeToWire(envelope, { actor: 0, physical_ms: 0, logical: index + 1 }, codec)));
  return out;
}

