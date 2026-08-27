//! 📡️ Replication contract — TypeScript twin of the Rust `protocol` crate.
//!
//! Byte-for-byte identical to `📦️packages/🦀️rust`'s encoders: the 20 frames in `🧫️fixtures/wire/`
//! are the shared gate both sides must reproduce. Frame layout is `lane u8`, `frame tag u8`, then
//! fields in declaration order — no length prefix, no per-field tags.

//#region 🔖️SyncProtocol
export * from "./📡️wire/🏠️local-interaction/🟦️component.ts";
export * from "./📡️wire/🏠️local-interaction/📡️transport/🟦️component.ts";
/**
 * 🔁️ TS mirror of `store_sync`'s Rust actor protocol (`ArtifactActorConfig`/`ArtifactActorMsg`/
 * `ArtifactEvent`/`ArtifactSyncStatus`/`RemoteState`/`PersistenceBinding`) — the wire/postMessage
 * shapes `🟦️backbone-worker.ts` speaks, kept camelCase-tag-identical to the Rust side (`#[serde(tag =
 * "kind", rename_all = "camelCase")]`) so a shared JSON fixture suite (`store/sync/fixtures/`)
 * stays plausible across both runtimes even though this file is a deliberately dumb TS twin (no
 * materialization — it only relays queues, exactly like the Rust actor's `ChannelBackbone` side).
 */
export type MutationEnvelope = {
  readonly id: string;
  readonly actor: string;
  readonly document: string;
  readonly schemaVersion: string;
  readonly deps?: readonly string[];
  readonly payloadHash: string;
  readonly diff: { readonly schemaId: string; readonly payload: unknown };
  readonly inverse: {
    readonly targetOperation: string;
    readonly inverseDiff: { readonly schemaId: string; readonly payload: unknown };
    readonly baseVersion: number;
    readonly dependencies?: readonly string[];
    readonly undoPolicy: string;
  };
};

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
    diff: { schema: envelope.diff.schemaId, payload: packPayload(envelope.diff.payload) },
    inverse: { schema: envelope.inverse.inverseDiff.schemaId, payload: packPayload(envelope.inverse.inverseDiff.payload) },
    timestamp,
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
  };
}

/** 📡️ Wire-protocol presence identity v3 — distinct from the UI-rendering {@link PresencePeer} scene
 * prop. `cursor`/`viewport` are DELETED (ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-
 * UNIVERSAL-ARTIFACT-CREATION C7.1) — replaced by `views` (artifact scope, one entry per open
 * window/surface) and `ui` (app scope, `data-ui-path` hover/focus/press). `color`/`surface` are
 * stamped by the client actor (`🟦️backbone-worker.ts`'s `stampSession`) — shells never fill them. */
export type ArtifactPresencePeer = {
  readonly actor: string;
  readonly connectedAtMs: number;
  readonly label?: string;
  readonly presencePack?: readonly number[];
  readonly userId?: string;
  readonly role?: string;
  readonly dragGhostJson?: string;
  /** 🕹️ Twin of Rust `PresenceInteraction` (presence bit 5) — the peer's live hover/selection per
   * interaction domain. Optional because a peer that has never interacted encodes no bit-5 section. */
  readonly interaction?: ArtifactPresenceInteraction;
  /** 🎨️ Hub-assigned palette index (bit 6), stamped by the client actor. */
  readonly color?: number;
  /** 🪟️ Canonical surface id (bit 7), stamped by the client actor. */
  readonly surface?: string;
  /** 🪟️ Every open window/surface's live camera + in-view pointer (bit 8, ARTIFACT scope), matched
   * by `space`. Empty when the peer has no open windows for this document. */
  readonly views: readonly ArtifactPresenceWindowView[];
  /** 🖱️ Live `data-ui-path` hover/focus/press state (bit 9, APP scope). */
  readonly ui?: ArtifactPresenceUi;
};

/** 🪟️ Twin of Rust `PresenceWindowView`. */
export type ArtifactPresenceWindowView = {
  readonly windowId: string;
  readonly space: string;
  readonly kind: ArtifactPresenceViewKind;
  readonly size: readonly [number, number];
  readonly pointer?: readonly [number, number, number];
};

/** 🎥️ Twin of Rust `PresenceViewKind` — internally tagged `kind`, camelCase (`{"kind":"orbit",…}`). */
export type ArtifactPresenceViewKind =
  | { readonly kind: "canvas"; readonly x: number; readonly y: number; readonly zoom: number }
  | { readonly kind: "orbit"; readonly position: readonly [number, number, number]; readonly target: readonly [number, number, number]; readonly up: readonly [number, number, number]; readonly fov: number }
  | { readonly kind: "geo"; readonly lng: number; readonly lat: number; readonly zoom: number; readonly bearing: number; readonly pitch: number };

/** 🖱️ Twin of Rust `PresenceUi`. */
export type ArtifactPresenceUi = {
  readonly hoveredPath?: string;
  readonly focusedPath?: string;
  readonly pressedPath?: string;
};

/** 🕹️ Twin of Rust `PresenceInteraction`. */
export type ArtifactPresenceInteraction = {
  readonly app_id: string;
  readonly domains: readonly ArtifactPresenceDomain[];
};

/** 🕹️ Twin of Rust `PresenceDomain` — one domain's granularity plus its selected/hovered ids. */
export type ArtifactPresenceDomain = {
  readonly domain: string;
  readonly granularity: string;
  readonly selected: readonly string[];
  readonly hovered: readonly string[];
};

/** 🌐️ One causally-ordered operation crossing the wire — mirrors Rust `protocol_causal::
 * MutationEnvelope` byte-for-byte. Wire-only shape, distinct from {@link MutationEnvelope} (this
 * file's postMessage/actor-protocol shape, camelCase-tagged): this type crosses `protocol_wire`'s
 * binary codec (see `encodeClientFrame`/`decodeClientFrame` below), where Rust field names are
 * plain (not renamed), so it stays snake_case like the Rust source. 🎯️ W5: `diff`/`inverse` payloads
 * are opaque bytes now (a JSON number array here, matching every other `Vec<u8>` field on this
 * boundary), not a schema-erased JSON value — `protocol_causal::ArtifactDiff`/`InverseMutation`
 * both flipped from `serde_json::Value` to `Vec<u8>`. */
export type WireMutationEnvelope = {
  readonly mutation_id: string;
  readonly document_id: string;
  readonly actor: string;
  readonly dependencies: readonly string[];
  readonly diff: { readonly schema: string; readonly payload: readonly number[] };
  readonly inverse: { readonly schema: string; readonly payload: readonly number[] };
  readonly timestamp: { readonly actor: number; readonly physical_ms: number; readonly logical: number };
};

/** 🏔️ Runtime/wire frontier summary — mirrors Rust `protocol_causal::FrontierSummary`
 * (`protocol::RuntimeFrontierSummary`). */
export type WireFrontierSummary = {
  readonly document_id: string;
  readonly head_edit_ordinal: number;
  readonly head_edit_id: string;
  readonly last_commit_seq: number;
  readonly chain_hash: readonly number[];
};

/** 🛣️ Which logical channel a wire frame travels on — mirrors Rust `protocol_wire::Lane`. */
export type WireLane = "command" | "preview";

/** 🚀️ How a `ServerFrame.Welcome` seeds a client — mirrors Rust `protocol_wire::Bootstrap`. */
export type WireBootstrap = "None" | { readonly Snapshot: { readonly pack_hash: readonly number[]; readonly inline: readonly number[] | null } } | "Tail";

/** ⚖️ How the hub resolved one submitted batch against concurrent history — mirrors Rust
 * `protocol_wire::ApplyOutcome`. */
export type WireApplyOutcome = "Accepted" | { readonly Transformed: { readonly envelope: WireMutationEnvelope } } | { readonly Rejected: { readonly reason: string; readonly messages: readonly number[] } };

/** 🪜️ One stage of a submitted batch's lifecycle — mirrors Rust `protocol_wire::AckStage`. */
export type WireAckStage = "Received" | "Persisted" | { readonly Applied: { readonly outcome: WireApplyOutcome } };

/** 📨️ Client→server hub wire frames — mirrors Rust `protocol_wire::ClientFrame` byte-for-byte.
 * Externally-tagged plain enum (serde's default representation, no `#[serde(tag = ...)]` on the
 * Rust side): a struct variant serializes as `{ VariantName: { ...fields } }`, a unit variant as
 * the bare string `"VariantName"`. Encode/decode with {@link encodeClientFrame}/
 * {@link decodeClientFrame} below — never hand-construct the JSON. */
export type ClientFrame =
  | {
      readonly Hello: {
        readonly wire_version: number;
        readonly protocol_version: number;
        readonly schema: string;
        readonly pack_schema_hash: readonly number[];
        readonly actor: string;
        readonly token: string | null;
        readonly resume_token: string | null;
        readonly frontier: WireFrontierSummary | null;
      };
    }
  | { readonly Commands: { readonly batch_id: number; readonly envelopes: readonly WireMutationEnvelope[] } }
  | { readonly FrontierAdvertise: { readonly frontier: WireFrontierSummary } }
  | { readonly PreviewPublish: { readonly key: string; readonly seq: number; readonly payload: readonly number[] } }
  | { readonly Presence: { readonly peer: readonly number[] } }
  | { readonly CreditGrant: { readonly n: number } }
  | "Bye";

/** 📬️ Server→client hub wire frames — mirrors Rust `protocol_wire::ServerFrame` byte-for-byte. See
 * {@link ClientFrame}'s doc comment for the externally-tagged encoding this shares. */
export type ServerFrame =
  | { readonly Welcome: { readonly session_id: string; readonly resume_token: string; readonly server_frontier: WireFrontierSummary; readonly bootstrap: WireBootstrap } }
  | { readonly SnapshotChunk: { readonly seq: number; readonly bytes: readonly number[] } }
  | { readonly SnapshotDone: { readonly seq_count: number } }
  | { readonly Commands: { readonly envelopes: readonly WireMutationEnvelope[]; readonly origin: string; readonly frontier: WireFrontierSummary } }
  | { readonly Ack: { readonly batch_id: number; readonly stages: readonly WireAckStage[]; readonly frontier: WireFrontierSummary } }
  | { readonly Preview: { readonly actor: string; readonly key: string; readonly seq: number; readonly payload: readonly number[] } }
  | { readonly Presence: { readonly peers: readonly (readonly number[])[] } }
  | { readonly CreditGrant: { readonly n: number } }
  | { readonly Error: { readonly code: string; readonly message: string } }
  /** 🎨️ The hub's one-time session assignment for this connection — see Rust `ServerFrame::Session`'s
   * doc comment. Sent exactly once per connection, after `Welcome` and before any `Presence` frame. */
  | { readonly Session: { readonly actor: string; readonly color: number } };

/** 🎞️ Writes an unsigned LEB128 varint (minimal length) — a byte-for-byte TS twin of
 * `protocol_core`'s `write_varint_u64` (`protocol/core/rs/lib.rs` `🔖️WireCodec`). */
export function writeVarintU64(out: number[], value: number): void {
  let remaining = value;
  for (;;) {
    const byte = remaining & 0x7f;
    remaining = Math.floor(remaining / 128);
    if (remaining === 0) {
      out.push(byte);
      return;
    }
    out.push(byte | 0x80);
  }
}

/** 🎞️ Reads an unsigned LEB128 varint starting at `pos[0]`, advancing it past it — the TS twin of
 * `protocol_core`'s `read_varint_u64`. */
export function readVarintU64(bytes: Uint8Array, pos: [number]): number {
  let result = 0;
  let shift = 1;
  for (let i = 0; i < 10; i++) {
    const byte = bytes[pos[0]];
    if (byte === undefined) throw new Error("wire frame varint: truncated");
    pos[0] += 1;
    result += (byte & 0x7f) * shift;
    if ((byte & 0x80) === 0) return result;
    shift *= 128;
  }
  throw new Error("wire frame varint: overlong varint (exceeds 10 bytes)");
}

/** 🎞️ `varint-u64 len | utf8 bytes` — the TS twin of `protocol_core::write_str`. */
export function writeStr(out: number[], value: string): void {
  const bytes = new TextEncoder().encode(value);
  writeVarintU64(out, bytes.length);
  for (const byte of bytes) out.push(byte);
}

/** 🎞️ The inverse of {@link writeStr} — the TS twin of `protocol_core::read_str`. */
export function readStr(bytes: Uint8Array, pos: [number]): string {
  const len = readVarintU64(bytes, pos);
  const slice = bytes.subarray(pos[0], pos[0] + len);
  if (slice.length !== len) throw new Error("wire str: truncated");
  pos[0] += len;
  return new TextDecoder().decode(slice);
}

/** 🎞️ `varint-u64 len | raw bytes` — the TS twin of `protocol_core::write_bytes`. */
export function writeBytes(out: number[], value: readonly number[]): void {
  writeVarintU64(out, value.length);
  for (const byte of value) out.push(byte);
}

/** 🎞️ The inverse of {@link writeBytes} — the TS twin of `protocol_core::read_bytes`. */
export function readBytes(bytes: Uint8Array, pos: [number]): number[] {
  const len = readVarintU64(bytes, pos);
  const slice = bytes.subarray(pos[0], pos[0] + len);
  if (slice.length !== len) throw new Error("wire bytes: truncated");
  pos[0] += len;
  return Array.from(slice);
}

/** 🎞️ 32 raw bytes, no length prefix — the TS twin of `protocol_core::write_hash32`. */
export function writeHash32(out: number[], value: readonly number[]): void {
  if (value.length !== 32) throw new Error("wire hash32: expected 32 bytes");
  for (const byte of value) out.push(byte);
}

/** 🎞️ The inverse of {@link writeHash32} — the TS twin of `protocol_core::read_hash32`. */
export function readHash32(bytes: Uint8Array, pos: [number]): number[] {
  const slice = bytes.subarray(pos[0], pos[0] + 32);
  if (slice.length !== 32) throw new Error("wire hash32: truncated");
  pos[0] += 32;
  return Array.from(slice);
}

/** 🎞️ One byte, `0`/`1` — the TS twin of `protocol_core::write_bool`. */
export function writeBool(out: number[], value: boolean): void {
  out.push(value ? 1 : 0);
}

/** 🎞️ The inverse of {@link writeBool} — the TS twin of `protocol_core::read_bool`. */
export function readBool(bytes: Uint8Array, pos: [number]): boolean {
  const byte = bytes[pos[0]];
  if (byte === undefined) throw new Error("wire bool: truncated");
  pos[0] += 1;
  return byte !== 0;
}

/** 🎞️ 8 raw little-endian bytes — the TS twin of `protocol_core::write_f64`. */
export function writeF64(out: number[], value: number): void {
  const buffer = new ArrayBuffer(8);
  new DataView(buffer).setFloat64(0, value, true);
  for (const byte of new Uint8Array(buffer)) out.push(byte);
}

/** 🎞️ The inverse of {@link writeF64} — the TS twin of `protocol_core::read_f64`. */
export function readF64(bytes: Uint8Array, pos: [number]): number {
  const slice = bytes.subarray(pos[0], pos[0] + 8);
  if (slice.length !== 8) throw new Error("wire f64: truncated");
  pos[0] += 8;
  return new DataView(slice.buffer, slice.byteOffset, 8).getFloat64(0, true);
}

/** 🎞️ `varint-u64 len | raw bytes` per entry — the TS twin of `protocol_wire::write_vec_bytes`. */
export function writeVecBytes(out: number[], values: readonly (readonly number[])[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) writeBytes(out, value);
}

/** 🎞️ The inverse of {@link writeVecBytes} — the TS twin of `protocol_wire::read_vec_bytes`. */
export function readVecBytes(bytes: Uint8Array, pos: [number]): number[][] {
  const count = readVarintU64(bytes, pos);
  const result: number[][] = [];
  for (let i = 0; i < count; i++) result.push(readBytes(bytes, pos));
  return result;
}

/** 🎯️ `actor str | flags varint_u64 | connected_at_ms varint | fields present per bitmask, strictly
 * in bit order (label str? | presence_pack bytes? | user_id str? | role str? | drag_ghost_json str? |
 * interaction? | color u8? | surface str? | views? | ui?)` — the TS twin of Rust
 * `encode_presence_peer` (`📡️wire/🦀️component.rs`). This is what `ClientFrame::Presence.peer`/
 * `ServerFrame::Presence.peers[]` actually carry — real binary, not JSON bytes. `flags` is a varint
 * (not a single byte) now that bit 9 exceeds a byte's range. */
/** @emoji 🕳️ Presence-flag guard. A field is "present" only when it is neither `undefined` **nor
 * `null`**: these peers are reconstructed from JSON view state, where an absent optional arrives as
 * `null`, and `null !== undefined` is true — so a bare `!== undefined` check set the flag and then
 * handed `null` to `writeStr`/`writeBytes`, throwing `Cannot read properties of null (reading
 * 'length')` on every heartbeat and wedging the plugin instance. */
function presencePresent<T>(value: T | null | undefined): value is T {
  return value !== undefined && value !== null;
}

export function encodePresencePeer(peer: ArtifactPresencePeer): number[] {
  const out: number[] = [];
  writeStr(out, peer.actor);
  let flags = 0;
  if (presencePresent(peer.label)) flags |= 1 << 0;
  if (presencePresent(peer.presencePack)) flags |= 1 << 1;
  if (presencePresent(peer.userId)) flags |= 1 << 2;
  if (presencePresent(peer.role)) flags |= 1 << 3;
  if (presencePresent(peer.dragGhostJson)) flags |= 1 << 4;
  if (presencePresent(peer.interaction)) flags |= 1 << 5;
  if (presencePresent(peer.color)) flags |= 1 << 6;
  if (presencePresent(peer.surface)) flags |= 1 << 7;
  if (peer.views.length > 0) flags |= 1 << 8;
  if (presencePresent(peer.ui)) flags |= 1 << 9;
  writeVarintU64(out, flags);
  writeVarintU64(out, peer.connectedAtMs ?? 0);
  if (presencePresent(peer.label)) writeStr(out, peer.label);
  if (presencePresent(peer.presencePack)) writeBytes(out, peer.presencePack);
  if (presencePresent(peer.userId)) writeStr(out, peer.userId);
  if (presencePresent(peer.role)) writeStr(out, peer.role);
  if (presencePresent(peer.dragGhostJson)) writeStr(out, peer.dragGhostJson);
  if (presencePresent(peer.interaction)) writePresenceInteraction(out, peer.interaction);
  if (presencePresent(peer.color)) out.push(peer.color);
  if (presencePresent(peer.surface)) writeStr(out, peer.surface);
  if (peer.views.length > 0) writeVecPresenceWindowView(out, peer.views);
  if (presencePresent(peer.ui)) writePresenceUi(out, peer.ui);
  return out;
}

/** 🎯️ The inverse of {@link encodePresencePeer} — the TS twin of Rust `decode_presence_peer`. Any
 * flag bit ≥ 10 set throws — no silent forward compatibility, matching the Rust decoder's
 * `ProtocolError::Malformed { what: "presence peer flags", .. }`. */
export function decodePresencePeer(bytes: Uint8Array, pos: [number]): ArtifactPresencePeer {
  const actor = readStr(bytes, pos);
  const flags = readVarintU64(bytes, pos);
  if (flags >> 10 !== 0) throw new Error(`presence peer flags: unknown flag bits set: ${flags.toString(16)}`);
  const connectedAtMs = readVarintU64(bytes, pos);
  const label = flags & (1 << 0) ? readStr(bytes, pos) : undefined;
  const presencePack = flags & (1 << 1) ? readBytes(bytes, pos) : undefined;
  const userId = flags & (1 << 2) ? readStr(bytes, pos) : undefined;
  const role = flags & (1 << 3) ? readStr(bytes, pos) : undefined;
  const dragGhostJson = flags & (1 << 4) ? readStr(bytes, pos) : undefined;
  const interaction = flags & (1 << 5) ? readPresenceInteraction(bytes, pos) : undefined;
  const color = flags & (1 << 6) ? readU8(bytes, pos) : undefined;
  const surface = flags & (1 << 7) ? readStr(bytes, pos) : undefined;
  const views = flags & (1 << 8) ? readVecPresenceWindowView(bytes, pos) : [];
  const ui = flags & (1 << 9) ? readPresenceUi(bytes, pos) : undefined;
  return { actor, connectedAtMs, label, presencePack, userId, role, dragGhostJson, interaction, color, surface, views, ui };
}

/** 🎞️ One raw byte — the TS twin of `protocol_core::read_u8`-shaped inline reads. */
export function readU8(bytes: Uint8Array, pos: [number]): number {
  const byte = bytes[pos[0]];
  if (byte === undefined) throw new Error("presence peer color: truncated");
  pos[0] += 1;
  return byte;
}

/** 🕹️ Twin of Rust `encode_presence_interaction` — `pub` (C7.4) so a guest that never enables the
 * kernel's `sync` feature can still call it directly. */
export function encodePresenceInteraction(interaction: ArtifactPresenceInteraction): number[] {
  const out: number[] = [];
  writePresenceInteraction(out, interaction);
  return out;
}

function writePresenceInteraction(out: number[], interaction: ArtifactPresenceInteraction): void {
  writeStr(out, interaction.app_id);
  writeVarintU64(out, interaction.domains.length);
  for (const domain of interaction.domains) {
    writeStr(out, domain.domain);
    writeStr(out, domain.granularity);
    writeVecStr(out, domain.selected);
    writeVecStr(out, domain.hovered);
  }
}

/** 🕹️ Twin of Rust `decode_presence_interaction` — `pub` for the same reason as
 * {@link encodePresenceInteraction}. */
export function decodePresenceInteraction(bytes: Uint8Array, pos: [number]): ArtifactPresenceInteraction {
  return readPresenceInteraction(bytes, pos);
}

/** 🕹️ Twin of Rust `decode_presence_interaction` — app id, then a varint-counted run of domains. */
function readPresenceInteraction(bytes: Uint8Array, pos: [number]): ArtifactPresenceInteraction {
  const app_id = readStr(bytes, pos);
  const count = Number(readVarintU64(bytes, pos));
  const domains: ArtifactPresenceDomain[] = [];
  for (let index = 0; index < count; index += 1) {
    domains.push({ domain: readStr(bytes, pos), granularity: readStr(bytes, pos), selected: readVecStr(bytes, pos), hovered: readVecStr(bytes, pos) });
  }
  return { app_id, domains };
}

//#region 🔖️PresenceView
/** 🎥️ Twin of Rust `encode_presence_view_kind` — discriminant `u8` (0 Canvas, 1 Orbit, 2 Geo) then
 * that variant's `f64` fields in declared order. */
function writePresenceViewKind(out: number[], kind: ArtifactPresenceViewKind): void {
  if (kind.kind === "canvas") {
    out.push(0);
    writeF64(out, kind.x);
    writeF64(out, kind.y);
    writeF64(out, kind.zoom);
  } else if (kind.kind === "orbit") {
    out.push(1);
    for (const value of [...kind.position, ...kind.target, ...kind.up]) writeF64(out, value);
    writeF64(out, kind.fov);
  } else {
    out.push(2);
    writeF64(out, kind.lng);
    writeF64(out, kind.lat);
    writeF64(out, kind.zoom);
    writeF64(out, kind.bearing);
    writeF64(out, kind.pitch);
  }
}

function readPresenceViewKind(bytes: Uint8Array, pos: [number]): ArtifactPresenceViewKind {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("presence view kind tag: truncated");
  pos[0] += 1;
  if (tag === 0) return { kind: "canvas", x: readF64(bytes, pos), y: readF64(bytes, pos), zoom: readF64(bytes, pos) };
  if (tag === 1) {
    const read3 = (): readonly [number, number, number] => [readF64(bytes, pos), readF64(bytes, pos), readF64(bytes, pos)];
    const position = read3();
    const target = read3();
    const up = read3();
    return { kind: "orbit", position, target, up, fov: readF64(bytes, pos) };
  }
  if (tag === 2) return { kind: "geo", lng: readF64(bytes, pos), lat: readF64(bytes, pos), zoom: readF64(bytes, pos), bearing: readF64(bytes, pos), pitch: readF64(bytes, pos) };
  throw new Error(`presence view kind tag: unknown tag ${tag}`);
}

function writePresenceWindowView(out: number[], view: ArtifactPresenceWindowView): void {
  writeStr(out, view.windowId);
  writeStr(out, view.space);
  writePresenceViewKind(out, view.kind);
  writeF64(out, view.size[0]);
  writeF64(out, view.size[1]);
  writeBool(out, presencePresent(view.pointer));
  if (presencePresent(view.pointer)) for (const value of view.pointer) writeF64(out, value);
}

function readPresenceWindowView(bytes: Uint8Array, pos: [number]): ArtifactPresenceWindowView {
  const windowId = readStr(bytes, pos);
  const space = readStr(bytes, pos);
  const kind = readPresenceViewKind(bytes, pos);
  const size: readonly [number, number] = [readF64(bytes, pos), readF64(bytes, pos)];
  const pointer: readonly [number, number, number] | undefined = readBool(bytes, pos) ? [readF64(bytes, pos), readF64(bytes, pos), readF64(bytes, pos)] : undefined;
  return { windowId, space, kind, size, pointer };
}

function writeVecPresenceWindowView(out: number[], values: readonly ArtifactPresenceWindowView[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) writePresenceWindowView(out, value);
}

function readVecPresenceWindowView(bytes: Uint8Array, pos: [number]): ArtifactPresenceWindowView[] {
  const count = readVarintU64(bytes, pos);
  const result: ArtifactPresenceWindowView[] = [];
  for (let i = 0; i < count; i++) result.push(readPresenceWindowView(bytes, pos));
  return result;
}

function writePresenceUi(out: number[], ui: ArtifactPresenceUi): void {
  writeOptStr(out, ui.hoveredPath ?? null);
  writeOptStr(out, ui.focusedPath ?? null);
  writeOptStr(out, ui.pressedPath ?? null);
}

function readPresenceUi(bytes: Uint8Array, pos: [number]): ArtifactPresenceUi {
  const hoveredPath = readOptStr(bytes, pos) ?? undefined;
  const focusedPath = readOptStr(bytes, pos) ?? undefined;
  const pressedPath = readOptStr(bytes, pos) ?? undefined;
  return { hoveredPath, focusedPath, pressedPath };
}
//#endregion 🔖️PresenceView

//#region 🔖️Combinators
function writeOptStr(out: number[], value: string | null): void {
  writeBool(out, value !== null);
  if (value !== null) writeStr(out, value);
}
function readOptStr(bytes: Uint8Array, pos: [number]): string | null {
  return readBool(bytes, pos) ? readStr(bytes, pos) : null;
}
function writeOptBytes(out: number[], value: readonly number[] | null): void {
  writeBool(out, value !== null);
  if (value !== null) writeBytes(out, value);
}
function readOptBytes(bytes: Uint8Array, pos: [number]): number[] | null {
  return readBool(bytes, pos) ? readBytes(bytes, pos) : null;
}
function writeOptFrontier(out: number[], value: WireFrontierSummary | null): void {
  writeBool(out, value !== null);
  if (value !== null) encodeFrontier(out, value);
}
function readOptFrontier(bytes: Uint8Array, pos: [number]): WireFrontierSummary | null {
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
function encodeHlc(out: number[], hlc: { readonly actor: number; readonly physical_ms: number; readonly logical: number }): void {
  writeVarintU64(out, hlc.actor);
  writeVarintU64(out, hlc.physical_ms);
  writeVarintU64(out, hlc.logical);
}
function decodeHlc(bytes: Uint8Array, pos: [number]): { readonly actor: number; readonly physical_ms: number; readonly logical: number } {
  const actor = readVarintU64(bytes, pos);
  const physical_ms = readVarintU64(bytes, pos);
  const logical = readVarintU64(bytes, pos);
  return { actor, physical_ms, logical };
}

/** 🎯️ `mutation_id str | document_id str | actor str | dependencies vec<str> | diff.schema str |
 * diff.payload bytes | inverse.schema str | inverse.payload bytes | hlc` — the TS twin of Rust
 * `protocol_causal::encode_envelope`. */
function encodeEnvelope(out: number[], envelope: WireMutationEnvelope): void {
  writeStr(out, envelope.mutation_id);
  writeStr(out, envelope.document_id);
  writeStr(out, envelope.actor);
  writeVecStr(out, envelope.dependencies);
  writeStr(out, envelope.diff.schema);
  writeBytes(out, envelope.diff.payload);
  writeStr(out, envelope.inverse.schema);
  writeBytes(out, envelope.inverse.payload);
  encodeHlc(out, envelope.timestamp);
}

/** 🎯️ Inverse of {@link encodeEnvelope} — the TS twin of Rust `protocol_causal::decode_envelope`. */
function decodeEnvelope(bytes: Uint8Array, pos: [number]): WireMutationEnvelope {
  const mutation_id = readStr(bytes, pos);
  const document_id = readStr(bytes, pos);
  const actor = readStr(bytes, pos);
  const dependencies = readVecStr(bytes, pos);
  const diffSchema = readStr(bytes, pos);
  const diffPayload = readBytes(bytes, pos);
  const inverseSchema = readStr(bytes, pos);
  const inversePayload = readBytes(bytes, pos);
  const timestamp = decodeHlc(bytes, pos);
  return { mutation_id, document_id, actor, dependencies, diff: { schema: diffSchema, payload: diffPayload }, inverse: { schema: inverseSchema, payload: inversePayload }, timestamp };
}

/** 🎯️ `document_id str | head_edit_ordinal varint | head_edit_id str | last_commit_seq varint |
 * chain_hash 32` — the TS twin of Rust `protocol_causal::encode_frontier`. */
function encodeFrontier(out: number[], frontier: WireFrontierSummary): void {
  writeStr(out, frontier.document_id);
  writeVarintU64(out, frontier.head_edit_ordinal);
  writeStr(out, frontier.head_edit_id);
  writeVarintU64(out, frontier.last_commit_seq);
  writeHash32(out, frontier.chain_hash);
}

/** 🎯️ Inverse of {@link encodeFrontier} — the TS twin of Rust `protocol_causal::decode_frontier`. */
function decodeFrontier(bytes: Uint8Array, pos: [number]): WireFrontierSummary {
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

//#region 🔖️NestedEnums
function encodeBootstrap(out: number[], bootstrap: WireBootstrap): void {
  if (bootstrap === "None") {
    out.push(0);
    return;
  }
  if (bootstrap === "Tail") {
    out.push(2);
    return;
  }
  out.push(1);
  writeHash32(out, bootstrap.Snapshot.pack_hash);
  writeOptBytes(out, bootstrap.Snapshot.inline);
}
function decodeBootstrap(bytes: Uint8Array, pos: [number]): WireBootstrap {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire bootstrap tag: truncated");
  pos[0] += 1;
  if (tag === 0) return "None";
  if (tag === 2) return "Tail";
  if (tag === 1) return { Snapshot: { pack_hash: readHash32(bytes, pos), inline: readOptBytes(bytes, pos) } };
  throw new Error(`wire bootstrap tag: unknown tag ${tag}`);
}

function encodeApplyOutcome(out: number[], outcome: WireApplyOutcome): void {
  if (outcome === "Accepted") {
    out.push(0);
    return;
  }
  if ("Transformed" in outcome) {
    out.push(1);
    encodeEnvelope(out, outcome.Transformed.envelope);
    return;
  }
  out.push(2);
  writeStr(out, outcome.Rejected.reason);
  writeBytes(out, outcome.Rejected.messages);
}
function decodeApplyOutcome(bytes: Uint8Array, pos: [number]): WireApplyOutcome {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire apply-outcome tag: truncated");
  pos[0] += 1;
  if (tag === 0) return "Accepted";
  if (tag === 1) return { Transformed: { envelope: decodeEnvelope(bytes, pos) } };
  if (tag === 2) return { Rejected: { reason: readStr(bytes, pos), messages: readBytes(bytes, pos) } };
  throw new Error(`wire apply-outcome tag: unknown tag ${tag}`);
}

function encodeAckStage(out: number[], stage: WireAckStage): void {
  if (stage === "Received") {
    out.push(0);
    return;
  }
  if (stage === "Persisted") {
    out.push(1);
    return;
  }
  out.push(2);
  encodeApplyOutcome(out, stage.Applied.outcome);
}
function decodeAckStage(bytes: Uint8Array, pos: [number]): WireAckStage {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire ack-stage tag: truncated");
  pos[0] += 1;
  if (tag === 0) return "Received";
  if (tag === 1) return "Persisted";
  if (tag === 2) return { Applied: { outcome: decodeApplyOutcome(bytes, pos) } };
  throw new Error(`wire ack-stage tag: unknown tag ${tag}`);
}
function writeVecAckStage(out: number[], values: readonly WireAckStage[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) encodeAckStage(out, value);
}
function readVecAckStage(bytes: Uint8Array, pos: [number]): WireAckStage[] {
  const count = readVarintU64(bytes, pos);
  const result: WireAckStage[] = [];
  for (let i = 0; i < count; i++) result.push(decodeAckStage(bytes, pos));
  return result;
}
//#endregion 🔖️NestedEnums

const WIRE_LANE_BYTES: Record<WireLane, number> = { command: 0, preview: 1 };
const WIRE_BYTE_LANES: readonly WireLane[] = ["command", "preview"];

/** 📤️ Encodes one `ClientFrame` on the given lane: `lane u8 | tag u8 | fields` — the TS twin of
 * `protocol_wire::encode_client_frame` (see that module's doc comment: W5 flipped the whole wire
 * codec from a JSON body to this hand-rolled binary layout, byte-for-byte with the Rust side). */
export function encodeClientFrame(frame: ClientFrame, lane: WireLane): Uint8Array {
  const out: number[] = [WIRE_LANE_BYTES[lane]];
  if (frame === "Bye") {
    out.push(6);
    return new Uint8Array(out);
  }
  if ("Hello" in frame) {
    out.push(0);
    const hello = frame.Hello;
    writeVarintU64(out, hello.wire_version);
    writeVarintU64(out, hello.protocol_version);
    writeStr(out, hello.schema);
    writeHash32(out, hello.pack_schema_hash);
    writeStr(out, hello.actor);
    writeOptStr(out, hello.token);
    writeOptStr(out, hello.resume_token);
    writeOptFrontier(out, hello.frontier);
  } else if ("Commands" in frame) {
    out.push(1);
    writeVarintU64(out, frame.Commands.batch_id);
    writeVecEnvelope(out, frame.Commands.envelopes);
  } else if ("FrontierAdvertise" in frame) {
    out.push(2);
    encodeFrontier(out, frame.FrontierAdvertise.frontier);
  } else if ("PreviewPublish" in frame) {
    out.push(3);
    writeStr(out, frame.PreviewPublish.key);
    writeVarintU64(out, frame.PreviewPublish.seq);
    writeBytes(out, frame.PreviewPublish.payload);
  } else if ("Presence" in frame) {
    out.push(4);
    writeBytes(out, frame.Presence.peer);
  } else if ("CreditGrant" in frame) {
    out.push(5);
    writeVarintU64(out, frame.CreditGrant.n);
  } else {
    throw new Error("encodeClientFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}

/** 📥️ Decodes one `ClientFrame` — the TS twin of `protocol_wire::decode_client_frame`. */
export function decodeClientFrame(bytes: Uint8Array): { readonly lane: WireLane; readonly frame: ClientFrame } {
  if (bytes.length === 0) throw new Error("wire frame: empty frame");
  const lane = WIRE_BYTE_LANES[bytes[0]];
  if (lane === undefined) throw new Error(`wire frame lane byte: unknown lane ${bytes[0]}`);
  const pos: [number] = [1];
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire client-frame tag: truncated");
  pos[0] += 1;
  let frame: ClientFrame;
  switch (tag) {
    case 0: {
      const wire_version = readVarintU64(bytes, pos);
      const protocol_version = readVarintU64(bytes, pos);
      const schema = readStr(bytes, pos);
      const pack_schema_hash = readHash32(bytes, pos);
      const actor = readStr(bytes, pos);
      const token = readOptStr(bytes, pos);
      const resume_token = readOptStr(bytes, pos);
      const frontier = readOptFrontier(bytes, pos);
      frame = { Hello: { wire_version, protocol_version, schema, pack_schema_hash, actor, token, resume_token, frontier } };
      break;
    }
    case 1:
      frame = { Commands: { batch_id: readVarintU64(bytes, pos), envelopes: readVecEnvelope(bytes, pos) } };
      break;
    case 2:
      frame = { FrontierAdvertise: { frontier: decodeFrontier(bytes, pos) } };
      break;
    case 3:
      frame = { PreviewPublish: { key: readStr(bytes, pos), seq: readVarintU64(bytes, pos), payload: readBytes(bytes, pos) } };
      break;
    case 4:
      frame = { Presence: { peer: readBytes(bytes, pos) } };
      break;
    case 5:
      frame = { CreditGrant: { n: readVarintU64(bytes, pos) } };
      break;
    case 6:
      frame = "Bye";
      break;
    default:
      throw new Error(`wire client-frame tag: unknown tag ${tag}`);
  }
  return { lane, frame };
}

/** 📤️ Encodes one `ServerFrame` on the given lane: `lane u8 | tag u8 | fields` — the TS twin of
 * `protocol_wire::encode_server_frame`. */
export function encodeServerFrame(frame: ServerFrame, lane: WireLane): Uint8Array {
  const out: number[] = [WIRE_LANE_BYTES[lane]];
  if ("Welcome" in frame) {
    out.push(0);
    writeStr(out, frame.Welcome.session_id);
    writeStr(out, frame.Welcome.resume_token);
    encodeFrontier(out, frame.Welcome.server_frontier);
    encodeBootstrap(out, frame.Welcome.bootstrap);
  } else if ("SnapshotChunk" in frame) {
    out.push(1);
    writeVarintU64(out, frame.SnapshotChunk.seq);
    writeBytes(out, frame.SnapshotChunk.bytes);
  } else if ("SnapshotDone" in frame) {
    out.push(2);
    writeVarintU64(out, frame.SnapshotDone.seq_count);
  } else if ("Commands" in frame) {
    out.push(3);
    writeVecEnvelope(out, frame.Commands.envelopes);
    writeStr(out, frame.Commands.origin);
    encodeFrontier(out, frame.Commands.frontier);
  } else if ("Ack" in frame) {
    out.push(4);
    writeVarintU64(out, frame.Ack.batch_id);
    writeVecAckStage(out, frame.Ack.stages);
    encodeFrontier(out, frame.Ack.frontier);
  } else if ("Preview" in frame) {
    out.push(5);
    writeStr(out, frame.Preview.actor);
    writeStr(out, frame.Preview.key);
    writeVarintU64(out, frame.Preview.seq);
    writeBytes(out, frame.Preview.payload);
  } else if ("Presence" in frame) {
    out.push(6);
    writeVecBytes(out, frame.Presence.peers);
  } else if ("CreditGrant" in frame) {
    out.push(7);
    writeVarintU64(out, frame.CreditGrant.n);
  } else if ("Error" in frame) {
    out.push(8);
    writeStr(out, frame.Error.code);
    writeStr(out, frame.Error.message);
  } else if ("Session" in frame) {
    out.push(9);
    writeStr(out, frame.Session.actor);
    out.push(frame.Session.color);
  } else {
    throw new Error("encodeServerFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}

/** 📥️ Decodes one `ServerFrame` — the TS twin of `protocol_wire::decode_server_frame`. */
export function decodeServerFrame(bytes: Uint8Array): { readonly lane: WireLane; readonly frame: ServerFrame } {
  if (bytes.length === 0) throw new Error("wire frame: empty frame");
  const lane = WIRE_BYTE_LANES[bytes[0]];
  if (lane === undefined) throw new Error(`wire frame lane byte: unknown lane ${bytes[0]}`);
  const pos: [number] = [1];
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire server-frame tag: truncated");
  pos[0] += 1;
  let frame: ServerFrame;
  switch (tag) {
    case 0:
      frame = { Welcome: { session_id: readStr(bytes, pos), resume_token: readStr(bytes, pos), server_frontier: decodeFrontier(bytes, pos), bootstrap: decodeBootstrap(bytes, pos) } };
      break;
    case 1:
      frame = { SnapshotChunk: { seq: readVarintU64(bytes, pos), bytes: readBytes(bytes, pos) } };
      break;
    case 2:
      frame = { SnapshotDone: { seq_count: readVarintU64(bytes, pos) } };
      break;
    case 3:
      frame = { Commands: { envelopes: readVecEnvelope(bytes, pos), origin: readStr(bytes, pos), frontier: decodeFrontier(bytes, pos) } };
      break;
    case 4:
      frame = { Ack: { batch_id: readVarintU64(bytes, pos), stages: readVecAckStage(bytes, pos), frontier: decodeFrontier(bytes, pos) } };
      break;
    case 5:
      frame = { Preview: { actor: readStr(bytes, pos), key: readStr(bytes, pos), seq: readVarintU64(bytes, pos), payload: readBytes(bytes, pos) } };
      break;
    case 6:
      frame = { Presence: { peers: readVecBytes(bytes, pos) } };
      break;
    case 7:
      frame = { CreditGrant: { n: readVarintU64(bytes, pos) } };
      break;
    case 8:
      frame = { Error: { code: readStr(bytes, pos), message: readStr(bytes, pos) } };
      break;
    case 9:
      frame = { Session: { actor: readStr(bytes, pos), color: readU8(bytes, pos) } };
      break;
    default:
      throw new Error(`wire server-frame tag: unknown tag ${tag}`);
  }
  return { lane, frame };
}

/** 🗃️ A durable place a document synchronizes with — mirrors Rust `PersistenceBinding`. `surface`
 * (contract-freeze §C0 "Presence scope") travels out of band on the document WS URL's `?surface=`
 * query param — see `connectHub` in `🟦️backbone-worker.ts`'s `🔖️Hub` region. No `PresencePeer` wire
 * change: its flag byte is full and the file is peer-leased. */
//#endregion 🔖️SyncProtocol

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("wire fixtures", () => {
      // 🎬️ Shared fixtures: the exact same bytes `store/sync/rs/lib.rs`'s
      // `wire_fixtures_stay_byte_identical_across_rust_and_ts` test generates and verifies Rust-side
      // (19 fixtures, one per `ClientFrame`/`ServerFrame` variant plus a `Bootstrap`/`ApplyOutcome`
      // sub-variant each — see that test's doc). Decoding them here, then re-encoding the decoded
      // value and diffing against the original bytes, proves the TS codec agrees with
      // `protocol_wire`'s Rust codec byte-for-byte, not just shape-wise. `diff.payload`/
      // `inverse.payload` are opaque `DemoOperation::encode_op()` bytes (W5) — this test only checks
      // they're non-empty and format-tagged (`op_rt::OP_BINARY_FORMAT = 1`), not their semantic
      // content (decoding a real op needs `DslVariants`, which this TS-only fallback has no twin of).
      it("decodes the Rust-generated binary wire fixtures byte-identically", async () => {
        const { readFileSync } = await import("node:fs");
        const { fileURLToPath } = await import("node:url");
        const { dirname, join } = await import("node:path");
        // 📦️ Written by `wire_fixtures_stay_byte_identical_across_rust_and_ts` in
        // `📡️wire/🦀️component.rs`, which resolves them as `CARGO_MANIFEST_DIR/../../🧫️fixtures/wire`
        // — i.e. beside the os-kernel crate, not under the sync module. The old path here pointed at a
        // pre-restructure location that no longer exists, so this cross-language byte-identity check had
        // been silently ENOENT-ing instead of comparing anything.
        const fixturesDir = join(dirname(fileURLToPath(import.meta.url)), "🧫️fixtures/wire");
  
        function loadClient(name: string) {
          const bytes = new Uint8Array(readFileSync(join(fixturesDir, name)));
          const decoded = decodeClientFrame(bytes);
          expect(encodeClientFrame(decoded.frame, decoded.lane)).toEqual(bytes);
          return decoded;
        }
        function loadServer(name: string) {
          const bytes = new Uint8Array(readFileSync(join(fixturesDir, name)));
          const decoded = decodeServerFrame(bytes);
          expect(encodeServerFrame(decoded.frame, decoded.lane)).toEqual(bytes);
          return decoded;
        }
        function assertOpBinaryPayload(payload: readonly number[]) {
          expect(payload.length).toBeGreaterThan(0);
          expect(payload[0]).toBe(1); // dsl::op_rt::OP_BINARY_FORMAT
        }
  
        const hello = loadClient("📦️client-hello.bin");
        expect(hello.lane).toBe("command");
        if (typeof hello.frame === "string" || !("Hello" in hello.frame)) throw new Error("expected a Hello frame");
        expect(hello.frame.Hello.schema).toBe("demo/v1");
        expect(hello.frame.Hello.actor).toBe("actor-1");
  
        const commands = loadClient("📦️client-commands.bin");
        if (typeof commands.frame === "string" || !("Commands" in commands.frame)) throw new Error("expected a Commands frame");
        expect(commands.frame.Commands.envelopes).toHaveLength(1);
        assertOpBinaryPayload(commands.frame.Commands.envelopes[0]?.diff.payload ?? []);
  
        const frontierAdvertise = loadClient("📦️client-frontier-advertise.bin");
        if (typeof frontierAdvertise.frame === "string" || !("FrontierAdvertise" in frontierAdvertise.frame)) throw new Error("expected a FrontierAdvertise frame");
  
        const previewPublish = loadClient("📦️client-preview-publish.bin");
        if (typeof previewPublish.frame === "string" || !("PreviewPublish" in previewPublish.frame)) throw new Error("expected a PreviewPublish frame");
        expect(previewPublish.frame.PreviewPublish.key).toBe("cursor");
  
        const presence = loadClient("📦️client-presence.bin");
        if (typeof presence.frame === "string" || !("Presence" in presence.frame)) throw new Error("expected a Presence frame");
        // 👥️ The fixture's `peer` bytes ARE a real `encode_presence_peer` blob now (Rust writes
        // `sample_presence_peer_with_interaction()`), so this decodes them with the TS twin and checks
        // the fields survive the crossing — the strongest form of this assertion, and the one the old
        // `JSON.parse` could never make. It read the blob as JSON and would have failed the moment the
        // fixture became real; it never did, because the fixture path above pointed at a directory that
        // no longer existed.
        // 👥️ A REAL `encode_presence_peer` blob (Rust writes `sample_presence_peer_with_interaction()`,
        // whose own doc says these fixtures exist "so the TS vitest twin exercises every `PresencePeer`
        // v3 flag bit (§C7.1) with a realistic payload"). Decoding it here proves the scalar fields,
        // the bit-5 interaction section, the color/surface/views/ui fields all cross the language
        // boundary, and re-encoding proves it byte-for-byte.
        const peer = decodePresencePeer(new Uint8Array(presence.frame.Presence.peer), [0]);
        expect(peer.actor).toBe("actor-1");
        expect(peer.label).toBe("Ada");
        expect(peer.userId).toBe("user-9");
        expect(peer.role).toBe("owner");
        expect(peer.connectedAtMs).toBe(1_700_000_000_000);
        expect(peer.color).toBe(5);
        expect(peer.surface).toBe("s.space.home@1/*#editor");
        expect(peer.views).toHaveLength(2);
        expect(peer.views[0]).toEqual({ windowId: "w1", space: "world", kind: { kind: "orbit", position: [1, 2, 3], target: [0, 0, 0], up: [0, 1, 0], fov: 45 }, size: [1024, 768], pointer: [0.5, 0.5, 0.5] });
        expect(peer.views[1]).toEqual({ windowId: "w2", space: "canvas", kind: { kind: "canvas", x: 12.5, y: -4, zoom: 1 }, size: [800, 600], pointer: undefined });
        expect(peer.ui).toEqual({ hoveredPath: "row[2]#t1", focusedPath: undefined, pressedPath: undefined });
        expect(peer.interaction?.app_id).toBe("space");
        expect(peer.interaction?.domains).toEqual([
          { domain: "outline", granularity: "task", selected: ["t1", "t2"], hovered: [] },
          { domain: "board", granularity: "card", selected: [], hovered: ["c1"] },
          { domain: "canvas", granularity: "node", selected: ["n9"], hovered: ["n9", "n10"] },
        ]);
        expect(encodePresencePeer(peer)).toEqual(Array.from(new Uint8Array(presence.frame.Presence.peer)));
  
        const creditGrant = loadClient("📦️client-credit-grant.bin");
        if (typeof creditGrant.frame === "string" || !("CreditGrant" in creditGrant.frame)) throw new Error("expected a CreditGrant frame");
        expect(creditGrant.frame.CreditGrant.n).toBe(16);
  
        const bye = loadClient("📦️client-bye.bin");
        expect(bye.frame).toBe("Bye");
  
        const welcomeTail = loadServer("📦️server-welcome-tail.bin");
        if (typeof welcomeTail.frame === "string" || !("Welcome" in welcomeTail.frame)) throw new Error("expected a Welcome frame");
        expect(welcomeTail.frame.Welcome.resume_token).toBe("resume-1");
        expect(welcomeTail.frame.Welcome.bootstrap).toBe("Tail");
  
        const welcomeSnapshot = loadServer("📦️server-welcome-snapshot-inline.bin");
        if (typeof welcomeSnapshot.frame === "string" || !("Welcome" in welcomeSnapshot.frame)) throw new Error("expected a Welcome frame");
        if (welcomeSnapshot.frame.Welcome.bootstrap === "None" || welcomeSnapshot.frame.Welcome.bootstrap === "Tail" || !("Snapshot" in welcomeSnapshot.frame.Welcome.bootstrap)) throw new Error("expected a Snapshot bootstrap");
        expect(welcomeSnapshot.frame.Welcome.bootstrap.Snapshot.inline).toEqual([9, 9, 9]);
  
        const snapshotChunk = loadServer("📦️server-snapshot-chunk.bin");
        if (typeof snapshotChunk.frame === "string" || !("SnapshotChunk" in snapshotChunk.frame)) throw new Error("expected a SnapshotChunk frame");
        expect(snapshotChunk.frame.SnapshotChunk.bytes).toEqual([1, 2, 3, 4]);
  
        const snapshotDone = loadServer("📦️server-snapshot-done.bin");
        if (typeof snapshotDone.frame === "string" || !("SnapshotDone" in snapshotDone.frame)) throw new Error("expected a SnapshotDone frame");
        expect(snapshotDone.frame.SnapshotDone.seq_count).toBe(4);
  
        const serverCommands = loadServer("📦️server-commands.bin");
        if (typeof serverCommands.frame === "string" || !("Commands" in serverCommands.frame)) throw new Error("expected a Commands frame");
        expect(serverCommands.frame.Commands.envelopes).toHaveLength(1);
  
        const ackAccepted = loadServer("📦️server-ack-accepted.bin");
        if (typeof ackAccepted.frame === "string" || !("Ack" in ackAccepted.frame)) throw new Error("expected an Ack frame");
        expect(ackAccepted.frame.Ack.batch_id).toBe(1);
        expect(ackAccepted.frame.Ack.stages).toHaveLength(3);
  
        const ackTransformed = loadServer("📦️server-ack-transformed.bin");
        if (typeof ackTransformed.frame === "string" || !("Ack" in ackTransformed.frame)) throw new Error("expected an Ack frame");
        expect(ackTransformed.frame.Ack.batch_id).toBe(2);
  
        const ackRejected = loadServer("📦️server-ack-rejected.bin");
        if (typeof ackRejected.frame === "string" || !("Ack" in ackRejected.frame)) throw new Error("expected an Ack frame");
        expect(ackRejected.frame.Ack.batch_id).toBe(3);
        const rejectedStage = ackRejected.frame.Ack.stages.find((stage) => typeof stage !== "string" && "Applied" in stage);
        if (typeof rejectedStage === "string" || rejectedStage === undefined || !("Applied" in rejectedStage) || typeof rejectedStage.Applied.outcome === "string" || !("Rejected" in rejectedStage.Applied.outcome)) throw new Error("expected a rejected apply outcome");
        expect(rejectedStage.Applied.outcome.Rejected.messages).toEqual([1, 2, 3]);
  
        const preview = loadServer("📦️server-preview.bin");
        if (typeof preview.frame === "string" || !("Preview" in preview.frame)) throw new Error("expected a Preview frame");
        expect(preview.frame.Preview.key).toBe("cursor");
  
        const serverPresence = loadServer("📦️server-presence.bin");
        if (typeof serverPresence.frame === "string" || !("Presence" in serverPresence.frame)) throw new Error("expected a Presence frame");
        // 👥️ Two peers, deliberately mixed by the Rust fixture: a plain JSON blob and a real
        // `encode_presence_peer` payload — so this asserts the frame carries opaque per-peer bytes
        // through untouched, and that the real one still decodes into the same peer as above.
        expect(serverPresence.frame.Presence.peers).toHaveLength(2);
        expect(JSON.parse(new TextDecoder().decode(new Uint8Array(serverPresence.frame.Presence.peers[0]!)))).toEqual({ id: "a" });
        expect(decodePresencePeer(new Uint8Array(serverPresence.frame.Presence.peers[1]!), [0])).toEqual(peer);
  
        const creditGrantServer = loadServer("📦️server-credit-grant.bin");
        if (typeof creditGrantServer.frame === "string" || !("CreditGrant" in creditGrantServer.frame)) throw new Error("expected a CreditGrant frame");
        expect(creditGrantServer.frame.CreditGrant.n).toBe(32);
  
        const error = loadServer("📦️server-error.bin");
        if (typeof error.frame === "string" || !("Error" in error.frame)) throw new Error("expected an Error frame");
        expect(error.frame.Error.code).toBe("rejected");
  
        const session = loadServer("📦️server-session.bin");
        if (typeof session.frame === "string" || !("Session" in session.frame)) throw new Error("expected a Session frame");
        expect(session.frame.Session.actor).toBe("actor-1");
        expect(session.frame.Session.color).toBe(5);
      });
  });
}
