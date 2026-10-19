import type { WireMutationEnvelope, MutationEnvelope, ExactWireMutationEnvelope, ArtifactPresencePeer } from "@semio-tech/framework-replication";
import { encodePackValue, decodePackValue, isPackInteger, packUIntSafeOrNull, encodeBackboneMessage } from "../../../../🟦️";
import type { PackValue } from "../../../../🟦️";
import { encodeDocumentBackboneEnvelopeBatchExact, decodePresenceInteraction, decodePresenceToolRun, decodePresenceHistoryEdit } from "@semio-tech/framework-replication";
import { fetchWithTimeout } from "@semio-tech/framework";
import { type ArtifactState, emitEvent } from "../🧾️document-state/🟦️.ts";
import { hubBinding } from "../🧩️actor-wire-helpers/🟦️.ts";
import type { FolderCanonicalBootstrapMirrorOwner } from "../📁️folder/🏷️mirror-owner/🟦️.ts";
import { CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH, FOLDER_FETCH_TIMEOUT_MS } from "../⚙️constants/🟦️.ts";

/** 🧮️ A stable, deterministic 32-bit seed for an actor id string, for `WireMutationEnvelope.
 * timestamp.actor` — the TS twin of the Rust actor's `actor_seed` (`framework/sync/rs/lib.rs`
 * `🔖️WireBridge`). Not cryptographic, just a cheap deterministic fold — matches the Rust side's own
 * `DefaultHasher`-based approach in spirit (both are wire-local ordering metadata, never round-
 * tripped back into an app-level {@link MutationEnvelope}). */
function actorSeed(actor: string): number {
  let hash = 0;
  for (let index = 0; index < actor.length; index++) {
    hash = (Math.imul(hash, 31) + actor.charCodeAt(index)) | 0;
  }
  return hash >>> 0;
}

/** ⏰️ Advances `state.hlcCounter` and stamps a fresh wire timestamp for an outbound envelope —
 * the TS twin of the Rust actor's `next_timestamp`. */
export function nextWireTimestamp(state: ArtifactState): WireMutationEnvelope["timestamp"] {
  state.hlcCounter += 1;
  return { actor: actorSeed(state.actor), physical_ms: Date.now(), logical: state.hlcCounter };
}

/** #⃣ A cheap, non-cryptographic FNV-1a-style digest for {@link toWireEnvelope}'s placeholder
 * `payloadHash` on the way back through {@link fromWireEnvelope}. This TS fallback never verifies
 * `payloadHash` against anything (it's a "deliberately dumb" relay twin — see this file's header
 * doc — the real content-addressed check happens Rust-side via `semio_framework_hash::hash_bytes`
 * once the wasm actor is available), so a real blake3 dependency isn't worth adding here just to
 * fill an otherwise-unused field. */
function placeholderPayloadHash(payload: unknown): string {
  const packed = encodePackValue(payload);
  let hash = 0x811c9dc5;
  for (let index = 0; index < packed.length; index++) {
    hash ^= packed[index]!;
    hash = Math.imul(hash, 0x01000193);
  }
  return (hash >>> 0).toString(16).padStart(8, "0");
}

/** 🎞️ `store::pack_rt` wire bytes for {@link toWireEnvelope}'s diff/inverse payloads — the TS twin
 * of the Rust actor's `encode_wire_value` call in `to_wire_envelope`. */
function encodePackPayload(value: unknown): number[] {
  return Array.from(encodePackValue(value));
}

/** 🎞️ The inverse of {@link encodePackPayload} — the TS twin of `decode_wire_value` in the Rust
 * actor's `from_wire_envelope`. */
export function decodePackPayload(bytes: ArrayLike<number>): PackValue {
  return decodePackValue(new Uint8Array(bytes));
}

/** 🌉️ Converts this fallback's local, camelCase {@link MutationEnvelope} into the snake_case
 * {@link WireMutationEnvelope} `protocol_wire::ClientFrame::Commands`/`ServerFrame::Commands`
 * carry — the TS twin of the Rust actor's `to_wire_envelope`. */
/** 🔁️ A server envelope's operation identity, whichever of the two shapes the frame carries. */
export function wireEnvelopeId(envelope: WireMutationEnvelope | { readonly id?: string }): string {
  return "mutation_id" in envelope ? String(envelope.mutation_id) : String(envelope.id ?? "");
}

export function toWireEnvelope(envelope: MutationEnvelope, timestamp: WireMutationEnvelope["timestamp"], actor = envelope.actor): WireMutationEnvelope {
  return {
    mutation_id: envelope.id,
    document_id: envelope.document,
    actor,
    dependencies: [...(envelope.deps ?? [])],
    observed: null,
    target: [],
    diff: { schema: envelope.diff.schemaId, payload: encodePackPayload(envelope.diff.payload) },
    inverse: { schema: envelope.inverse.inverseDiff.schemaId, payload: encodePackPayload(envelope.inverse.inverseDiff.payload) },
    timestamp,
    transaction: envelope.transaction ?? null,
    verb: envelope.verb ?? null,
    line: null,
  };
}

/** 🌉️ The inverse of {@link toWireEnvelope} — the TS twin of the Rust actor's `from_wire_envelope`.
 * `baseVersion` is recovered from the payload's own `sequenceNumber` (this actor's payloads are
 * always edit-shaped JSON), mirroring the Rust side's identical recovery. */
export function fromWireEnvelope(envelope: WireMutationEnvelope | ExactWireMutationEnvelope): MutationEnvelope {
  const payload = decodePackPayload(envelope.diff.payload);
  const sequenceNumber = payload !== null && typeof payload === "object" && !Array.isArray(payload) && !isPackInteger(payload) && "sequenceNumber" in payload ? packUIntSafeOrNull((payload as Record<string, PackValue>).sequenceNumber??null) : 0;
  if (sequenceNumber === null) throw new Error("wire envelope: sequenceNumber is not an exact unsigned integer");
  return {
    id: envelope.mutation_id,
    actor: envelope.actor,
    document: envelope.document_id,
    schemaVersion: envelope.diff.schema,
    deps: [...envelope.dependencies],
    payloadHash: placeholderPayloadHash(payload),
    diff: { schemaId: envelope.diff.schema, payload },
    inverse: {
      targetOperation: envelope.mutation_id,
      inverseDiff: { schemaId: envelope.inverse.schema, payload: decodePackPayload(envelope.inverse.payload) },
      baseVersion: sequenceNumber,
      dependencies: [],
      undoPolicy: "exactBaseOnly",
    },
    ...(envelope.transaction === null ? {} : { transaction: envelope.transaction }),
    ...(envelope.verb === null ? {} : { verb: envelope.verb }),
  };
}

/** 🌉️ Document-backbone twin of {@link fromWireEnvelope}: a Rust store's events carry opaque
 * `OpBinary` operation or history-transition payloads this worker relays and persists but never
 * interprets, so they stay exact bytes instead of being decoded as pack values. */
export function opaqueEnvelopeFromWire(envelope: ExactWireMutationEnvelope): MutationEnvelope {
  const payload = envelope.diff.payload.slice();
  return {
    id: envelope.mutation_id,
    actor: envelope.actor,
    document: envelope.document_id,
    schemaVersion: envelope.diff.schema,
    deps: [...envelope.dependencies],
    payloadHash: placeholderPayloadHash(payload),
    diff: { schemaId: envelope.diff.schema, payload },
    inverse: { targetOperation: envelope.mutation_id, inverseDiff: { schemaId: envelope.inverse.schema, payload: envelope.inverse.payload.slice() }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
    ...(envelope.transaction === null ? {} : { transaction: envelope.transaction }),
    ...(envelope.verb === null ? {} : { verb: envelope.verb }),
  };
}

export function exactWireEnvelope(envelope: WireMutationEnvelope): ExactWireMutationEnvelope {
  const exactU64 = (value: number, field: string): bigint => {
    if (!Number.isSafeInteger(value) || value < 0) throw new Error(`wire envelope: ${field} is not an exact u64`);
    return BigInt(value);
  };
  return {
    mutation_id: envelope.mutation_id,
    document_id: envelope.document_id,
    actor: envelope.actor,
    dependencies: envelope.dependencies,
    observed: envelope.observed,
    target: envelope.target,
    diff: { schema: envelope.diff.schema, payload: Uint8Array.from(envelope.diff.payload) },
    inverse: { schema: envelope.inverse.schema, payload: Uint8Array.from(envelope.inverse.payload) },
    timestamp: {
      actor: exactU64(envelope.timestamp.actor, "timestamp.actor"),
      physical_ms: exactU64(envelope.timestamp.physical_ms, "timestamp.physical_ms"),
      logical: exactU64(envelope.timestamp.logical, "timestamp.logical"),
    },
    transaction: envelope.transaction,
    verb: envelope.verb,
    line: envelope.line,
  };
}

function documentBackboneMessage(envelopes: readonly ExactWireMutationEnvelope[]): Uint8Array {
  return encodeBackboneMessage({ kind: "mutations", envelopes: encodeDocumentBackboneEnvelopeBatchExact(envelopes) });
}

export function documentBackboneMessageFromDomain(state: ArtifactState, envelopes: readonly MutationEnvelope[]): Uint8Array {
  return documentBackboneMessage(envelopes.map((envelope) => state.exactLocalEnvelopes.get(envelope)?.envelope ?? exactWireEnvelope(toWireEnvelope(envelope, nextWireTimestamp(state)))));
}

export function emitMutationEvent(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  if (envelopes.length === 0) return;
  if (hubBinding(state.config)) emitEvent(state, { kind: "documentBackbone", message: documentBackboneMessageFromDomain(state, envelopes) });
  else emitEvent(state, { kind: "remoteMutations", envelopes });
}

/** 🎨️ Stamps `state`'s hub-assigned session color and canonical surface onto an outbound
 * `ArtifactPresencePeer` right before `encodePresencePeer` — the ONE place either field is ever
 * filled; shells never set `peer.color`/`peer.surface` themselves (contract-freeze §C7.4). The TS
 * twin of the Rust actor's `stamp_session`. An actor-bound document's presence pack and interaction
 * slice are its verified actor's own last `Ephemeral` (the native twin keeps the same frame per
 * program, `ProgramBridge::observe_ephemeral`), never the Shell's local instance's, which never sees
 * the live document: without this a React human's selections never reached a peer (ticket 26/09/23
 * C10, WG8 relay). The same holds for the actor's tool-run and history-edit summaries (presence bits 10 and 13,
 * ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING). */
export function stampSession(peer: ArtifactPresencePeer, state: ArtifactState): ArtifactPresencePeer {
  const stamped = { ...peer, color: state.sessionColor ?? undefined, surface: hubBinding(state.config)?.installedTarget?.surface.surfaceId };
  const reservation = state.browserActorReservation;
  if (!reservation) return stamped;
  const ephemeral = reservation.ephemeralSnapshot;
  const { presencePack: _local, interaction: _localInteraction, toolRun: _localToolRun, historyEdit: _localHistoryEdit, ...rest } = stamped;
  return {
    ...rest,
    ...(ephemeral !== null && ephemeral.presence.length > 0 ? { presencePack: ephemeral.presence } : {}),
    ...(ephemeral !== null && ephemeral.interaction.length > 0 ? { interaction: decodePresenceInteraction(Uint8Array.from(ephemeral.interaction), [0]) } : {}),
    ...(ephemeral !== null && ephemeral.toolRun.length > 0 ? { toolRun: decodePresenceToolRun(Uint8Array.from(ephemeral.toolRun)) } : {}),
    ...(ephemeral !== null && ephemeral.historyEdit.length > 0 ? { historyEdit: decodePresenceHistoryEdit(Uint8Array.from(ephemeral.historyEdit)) } : {}),
  };
}

/** ↩️ Synthesizes a local "undo" envelope from a speculative envelope's own precomputed `inverse` —
 * the TS twin of the Rust actor's `rollback_envelope` (see that function's doc comment for why
 * replaying the envelope's own inverse, rather than calling into typed operation-inverse machinery,
 * is the right move for this schema-agnostic relay). */
export function rollbackEnvelope(envelope: MutationEnvelope): MutationEnvelope {
  const undoId = `${envelope.id}~undo`;
  return {
    id: undoId,
    actor: envelope.actor,
    document: envelope.document,
    schemaVersion: envelope.schemaVersion,
    deps: [envelope.id],
    payloadHash: placeholderPayloadHash(envelope.inverse.inverseDiff.payload),
    diff: { schemaId: envelope.inverse.inverseDiff.schemaId, payload: envelope.inverse.inverseDiff.payload },
    inverse: { targetOperation: undoId, inverseDiff: { schemaId: envelope.diff.schemaId, payload: envelope.diff.payload }, baseVersion: envelope.inverse.baseVersion, dependencies: [], undoPolicy: envelope.inverse.undoPolicy },
  };
}

export function folderCanonicalBootstrapMirrorUrl(owner: Pick<FolderCanonicalBootstrapMirrorOwner, "binding" | "documentId">, action: "reserve" | "stage" | "publish" | "retire"): string {
  return `${CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH}/${action}?uri=${encodeURIComponent(`folder://${owner.binding.path}`)}&documentId=${encodeURIComponent(owner.documentId)}`;
}

export function folderCanonicalBootstrapMirrorHeaders(owner: FolderCanonicalBootstrapMirrorOwner): Record<string, string> {
  return { authorization: `SemioFolderBootstrap ${owner.capability}`, "x-semio-canonical-bootstrap-epoch": String(owner.epoch) };
}

export async function retireFolderCanonicalBootstrapMirror(owner: FolderCanonicalBootstrapMirrorOwner): Promise<boolean> {
  try {
    const response = await fetchWithTimeout(folderCanonicalBootstrapMirrorUrl(owner, "retire"), { method: "POST", headers: folderCanonicalBootstrapMirrorHeaders(owner) }, { timeoutMs: FOLDER_FETCH_TIMEOUT_MS });
    return response.ok || response.status === 409;
  } catch {
    return false;
  }
}
