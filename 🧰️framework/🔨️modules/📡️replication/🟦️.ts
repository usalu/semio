//! 📡️ Replication contract — TypeScript twin of the Rust `protocol` crate.
//!
//! Byte-for-byte identical to `📦️packages/🦀️rust`'s encoders: the 20 frames in `🧫️fixtures/📡️wire`
//! are the shared gate both sides must reproduce. Frame layout is `lane u8`, `frame tag u8`, then
//! fields in declaration order — no length prefix, no per-field tags.

//#region 🔖️SyncProtocol
export * from "./📡️wire/🏠️local-interaction/🟦️.ts";
export * from "./📡️wire/🏠️local-interaction/📡️transport/🟦️.ts";
/**
 * 🔁️ TS mirror of `store_sync`'s Rust actor protocol (`ArtifactActorConfig`/`ArtifactActorMsg`/
 * `ArtifactEvent`/`ArtifactSyncStatus`/`RemoteState`/`PersistenceBinding`) — the wire/postMessage
 * shapes `🧵️backbone-worker.ts` speaks, kept camelCase-tag-identical to the Rust side (`#[serde(tag =
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
 * window/surface) and `ui` (app scope, `data-ui-path` hover/focus/press). Client stamping is only
 * a local hint; the Hub replaces `color` and `surface` at authenticated presence ingress. */
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
  /** 🎨️ Hub-assigned palette index (bit 6), normalized at authenticated ingress. */
  readonly color?: number;
  /** 🪟️ Canonical plan-bound surface id (bit 7), normalized by the Hub. */
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

/** 🧩️ The indivisible canonical artifact payload: pack state plus SPR history. */
export type ArtifactBootstrapPair = Readonly<{ pack: Uint8Array; spr: Uint8Array }>;

/** 📦️ Descriptor-bound metadata and optional inline pair for one public artifact bootstrap. */
export type WireArtifactBootstrap = Readonly<{
  format_version: number;
  descriptor_hash: readonly number[];
  artifact_schema: string;
  artifact_kind: string;
  pack_schema_hash: readonly number[];
  baseline_frontier: WireFrontierSummary;
  pack_hash: readonly number[];
  spr_hash: readonly number[];
  pack_length: number;
  spr_length: number;
  chunk_count: number;
  aggregate_hash: readonly number[];
  required_tail_frontier: WireFrontierSummary;
  inline: Readonly<{ pack: readonly number[]; spr: readonly number[] }> | null;
}>;

/** 🛟️ Storage-key-free checkpoint identity sent before a lagged live stream closes. */
export type WireRebootstrapRequired = Readonly<{
  space_id: string;
  document_id: string;
  checkpoint_id: readonly number[];
  descriptor_hash: readonly number[];
  baseline_frontier: WireFrontierSummary;
}>;

/** 🚀️ How a `ServerFrame.Welcome` seeds a client — mirrors Rust `protocol_wire::Bootstrap`. */
export type WireBootstrap = "None" | { readonly Snapshot: { readonly pack_hash: readonly number[]; readonly inline: readonly number[] | null } } | "Tail" | { readonly ArtifactBootstrap: WireArtifactBootstrap };

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
      readonly SocketHelloV1: {
        readonly wire_version: number;
        readonly protocol_version: number;
        readonly schema: string;
        readonly pack_schema_hash: readonly number[];
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
  | { readonly Session: { readonly actor: string; readonly color: number } }
  | { readonly ArtifactBootstrapChunk: { readonly descriptor_hash: readonly number[]; readonly index: number; readonly bytes: readonly number[] } }
  | { readonly ArtifactBootstrapDone: { readonly descriptor_hash: readonly number[]; readonly chunk_count: number } }
  | { readonly RebootstrapRequired: { readonly control: WireRebootstrapRequired } };

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
 * `encode_presence_peer` (`📡️wire/🦀️.rs`). This is what `ClientFrame::Presence.peer`/
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

/** 🛡️ Fixed hostile-input ceilings shared byte-for-byte with Rust. */
export const PRESENCE_PEER_WIRE_LIMITS_V1 = Object.freeze({
  maximumEntryBytes: 4_096,
  maximumTextBytes: 1_024,
  maximumPresencePackBytes: 2_048,
  maximumViews: 16,
  maximumInteractionDomains: 16,
  maximumDomainIds: 64,
  maximumConnectedAtMs: Number.MAX_SAFE_INTEGER,
});

class PresencePeerReader {
  readonly bytes: Uint8Array;
  position: number;

  constructor(bytes: Uint8Array, position: number) {
    this.bytes = bytes;
    this.position = position;
  }

  fail(what: string, detail: string): never {
    throw new Error(`${what} at ${this.position}: ${detail}`);
  }

  varint(what: string): number {
    const start = this.position;
    let value = 0n;
    for (let index = 0; index < 10; index += 1) {
      const byte = this.bytes[this.position];
      if (byte === undefined) this.fail(what, "truncated varint");
      this.position += 1;
      if (index === 9 && byte > 1) this.fail(what, "varint exceeds u64");
      value |= BigInt(byte & 0x7f) << BigInt(index * 7);
      if ((byte & 0x80) === 0) {
        if (this.position - start > 1 && byte === 0) this.fail(what, "noncanonical varint");
        if (value > BigInt(Number.MAX_SAFE_INTEGER)) this.fail(what, "integer exceeds exact TypeScript range");
        return Number(value);
      }
    }
    return this.fail(what, "overlong varint");
  }

  length(maximum: number, what: string): number {
    const length = this.varint(what);
    if (length > maximum) this.fail(what, "limit exceeded");
    if (length > this.bytes.length - this.position) this.fail(what, "truncated field");
    return length;
  }

  count(maximum: number, what: string): number {
    return this.length(maximum, what);
  }

  text(what: string): string {
    const length = this.length(PRESENCE_PEER_WIRE_LIMITS_V1.maximumTextBytes, what);
    const end = this.position + length;
    let value: string;
    try { value = new TextDecoder("utf-8", { fatal: true }).decode(this.bytes.subarray(this.position, end)); }
    catch { return this.fail(what, "invalid utf8"); }
    this.position = end;
    return value;
  }

  blob(what: string): number[] {
    const length = this.length(PRESENCE_PEER_WIRE_LIMITS_V1.maximumPresencePackBytes, what);
    const end = this.position + length;
    const value = Array.from(this.bytes.subarray(this.position, end));
    this.position = end;
    return value;
  }

  byte(what: string): number {
    const value = this.bytes[this.position];
    if (value === undefined) this.fail(what, "truncated byte");
    this.position += 1;
    return value;
  }

  boolean(what: string): boolean {
    const value = this.byte(what);
    if (value !== 0 && value !== 1) this.fail(what, "boolean must be zero or one");
    return value === 1;
  }

  number(what: string): number {
    const end = this.position + 8;
    if (end > this.bytes.length) this.fail(what, "truncated f64");
    const value = new DataView(this.bytes.buffer, this.bytes.byteOffset + this.position, 8).getFloat64(0, true);
    if (!Number.isFinite(value)) this.fail(what, "non-finite f64");
    this.position = end;
    return value;
  }

  triple(what: string): readonly [number, number, number] {
    return [this.number(what), this.number(what), this.number(what)];
  }

  strings(what: string): string[] {
    const count = this.count(PRESENCE_PEER_WIRE_LIMITS_V1.maximumDomainIds, what);
    const values: string[] = [];
    for (let index = 0; index < count; index += 1) values.push(this.text(what));
    return values;
  }

  interaction(): ArtifactPresenceInteraction {
    const app_id = this.text("presence interaction app id");
    const count = this.count(PRESENCE_PEER_WIRE_LIMITS_V1.maximumInteractionDomains, "presence interaction domains");
    const domains: ArtifactPresenceDomain[] = [];
    for (let index = 0; index < count; index += 1) domains.push({ domain: this.text("presence interaction domain"), granularity: this.text("presence interaction granularity"), selected: this.strings("presence interaction selected"), hovered: this.strings("presence interaction hovered") });
    return { app_id, domains };
  }

  viewKind(): ArtifactPresenceViewKind {
    const tag = this.byte("presence view kind");
    if (tag === 0) return { kind: "canvas", x: this.number("presence canvas x"), y: this.number("presence canvas y"), zoom: this.number("presence canvas zoom") };
    if (tag === 1) return { kind: "orbit", position: this.triple("presence orbit position"), target: this.triple("presence orbit target"), up: this.triple("presence orbit up"), fov: this.number("presence orbit fov") };
    if (tag === 2) return { kind: "geo", lng: this.number("presence geo longitude"), lat: this.number("presence geo latitude"), zoom: this.number("presence geo zoom"), bearing: this.number("presence geo bearing"), pitch: this.number("presence geo pitch") };
    return this.fail("presence view kind", `unknown tag ${tag}`);
  }

  views(): ArtifactPresenceWindowView[] {
    const count = this.count(PRESENCE_PEER_WIRE_LIMITS_V1.maximumViews, "presence views");
    const views: ArtifactPresenceWindowView[] = [];
    for (let index = 0; index < count; index += 1) {
      const windowId = this.text("presence view window id");
      const space = this.text("presence view space");
      const kind = this.viewKind();
      const size: readonly [number, number] = [this.number("presence view width"), this.number("presence view height")];
      const pointer = this.boolean("presence view pointer") ? this.triple("presence view pointer") : undefined;
      views.push({ windowId, space, kind, size, pointer });
    }
    return views;
  }

  optionalText(what: string): string | undefined {
    return this.boolean(what) ? this.text(what) : undefined;
  }

  ui(): ArtifactPresenceUi {
    return { hoveredPath: this.optionalText("presence ui hovered path"), focusedPath: this.optionalText("presence ui focused path"), pressedPath: this.optionalText("presence ui pressed path") };
  }
}

/** 🎯️ Exact, allocation-bounded inverse of {@link encodePresencePeer}. */
export function decodePresencePeer(bytes: Uint8Array, pos: [number]): ArtifactPresencePeer {
  if (!Number.isSafeInteger(pos[0]) || pos[0] < 0 || pos[0] > bytes.length) throw new Error("presence peer position: invalid");
  if (bytes.length - pos[0] > PRESENCE_PEER_WIRE_LIMITS_V1.maximumEntryBytes) throw new Error("presence peer entry bytes: limit exceeded");
  const reader = new PresencePeerReader(bytes, pos[0]);
  const actor = reader.text("presence peer actor");
  const flags = reader.varint("presence peer flags");
  if (flags > 0x3ff) reader.fail("presence peer flags", `unknown flag bits set: ${flags.toString(16)}`);
  const connectedAtMs = reader.varint("presence peer connected at");
  if (connectedAtMs > PRESENCE_PEER_WIRE_LIMITS_V1.maximumConnectedAtMs) reader.fail("presence peer connected at", "limit exceeded");
  const label = flags & (1 << 0) ? reader.text("presence peer label") : undefined;
  const presencePack = flags & (1 << 1) ? reader.blob("presence peer pack") : undefined;
  const userId = flags & (1 << 2) ? reader.text("presence peer user id") : undefined;
  const role = flags & (1 << 3) ? reader.text("presence peer role") : undefined;
  const dragGhostJson = flags & (1 << 4) ? reader.text("presence peer drag ghost") : undefined;
  const interaction = flags & (1 << 5) ? reader.interaction() : undefined;
  const color = flags & (1 << 6) ? reader.byte("presence peer color") : undefined;
  const surface = flags & (1 << 7) ? reader.text("presence peer surface") : undefined;
  const views = flags & (1 << 8) ? reader.views() : [];
  const ui = flags & (1 << 9) ? reader.ui() : undefined;
  if (reader.position !== bytes.length) reader.fail("presence peer", "trailing bytes");
  pos[0] = reader.position;
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

function writePresenceWindowView(out: number[], view: ArtifactPresenceWindowView): void {
  writeStr(out, view.windowId);
  writeStr(out, view.space);
  writePresenceViewKind(out, view.kind);
  writeF64(out, view.size[0]);
  writeF64(out, view.size[1]);
  writeBool(out, presencePresent(view.pointer));
  if (presencePresent(view.pointer)) for (const value of view.pointer) writeF64(out, value);
}

function writeVecPresenceWindowView(out: number[], values: readonly ArtifactPresenceWindowView[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) writePresenceWindowView(out, value);
}

function writePresenceUi(out: number[], ui: ArtifactPresenceUi): void {
  writeOptStr(out, ui.hoveredPath ?? null);
  writeOptStr(out, ui.focusedPath ?? null);
  writeOptStr(out, ui.pressedPath ?? null);
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

const MUTATION_DAG_CAPACITY = 8_192;
const MUTATION_DAG_IDENTIFIER_BYTES = 256;

export const DOCUMENT_BACKBONE_BATCH_LIMITS = {
  maximumBytes: 262_144,
  maximumEnvelopes: MUTATION_DAG_CAPACITY,
  maximumDependenciesPerEnvelope: MUTATION_DAG_CAPACITY,
  maximumTotalDependencies: MUTATION_DAG_CAPACITY,
  maximumIdentifierBytes: MUTATION_DAG_IDENTIFIER_BYTES,
  maximumSchemaBytes: MUTATION_DAG_IDENTIFIER_BYTES,
  maximumPayloadBytes: 262_144,
} as const;

export type DocumentBackboneBatchLimits = Readonly<{
  maximumBytes: number;
  maximumEnvelopes: number;
  maximumDependenciesPerEnvelope: number;
  maximumTotalDependencies: number;
  maximumIdentifierBytes: number;
  maximumSchemaBytes: number;
  maximumPayloadBytes: number;
}>;

export type ExactWireMutationEnvelope = Readonly<{
  mutation_id: string;
  document_id: string;
  actor: string;
  dependencies: readonly string[];
  diff: Readonly<{ schema: string; payload: Uint8Array }>;
  inverse: Readonly<{ schema: string; payload: Uint8Array }>;
  timestamp: Readonly<{ actor: bigint; physical_ms: bigint; logical: bigint }>;
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
    documentBackboneWriteText(out, envelope.diff.schema);
    documentBackboneWriteBytes(out, envelope.diff.payload);
    documentBackboneWriteText(out, envelope.inverse.schema);
    documentBackboneWriteBytes(out, envelope.inverse.payload);
    documentBackboneWriteU64(out, envelope.timestamp.actor);
    documentBackboneWriteU64(out, envelope.timestamp.physical_ms);
    documentBackboneWriteU64(out, envelope.timestamp.logical);
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
    const diffSchema = readText(limits.maximumSchemaBytes, "schema-bytes");
    const diffPayload = readBytes(limits.maximumPayloadBytes - totalPayloadBytes, "payload-bytes");
    totalPayloadBytes += diffPayload.length;
    const inverseSchema = readText(limits.maximumSchemaBytes, "schema-bytes");
    const inversePayload = readBytes(limits.maximumPayloadBytes - totalPayloadBytes, "payload-bytes");
    totalPayloadBytes += inversePayload.length;
    envelopes.push({
      mutation_id,
      document_id,
      actor,
      dependencies,
      diff: { schema: diffSchema, payload: diffPayload },
      inverse: { schema: inverseSchema, payload: inversePayload },
      timestamp: { actor: readU64(), physical_ms: readU64(), logical: readU64() },
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

//#region 🔖️ArtifactBootstrap
export const ARTIFACT_BOOTSTRAP_FORMAT_VERSION = 1;
export const ARTIFACT_BOOTSTRAP_CHUNK_BYTES = 4 * 1024;
export const ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES = 64 * 1024 * 1024;
export const ARTIFACT_BOOTSTRAP_MAX_CHUNKS = 16 * 1024;

/** 🛡️ Caller-selected assembly ceilings, always constrained by the wire chunk maximum. */
export type ArtifactBootstrapLimits = Readonly<{ maxTotalBytes: number; maxChunks: number; maxChunkBytes: number }>;

/** 📈️ Observable transfer progress; byte and chunk counts never decrease within one assembler. */
export type ArtifactBootstrapProgress = Readonly<{ receivedBytes: number; totalBytes: number; receivedChunks: number; totalChunks: number }>;

/** ⏱️ Host-provided cancellation, monotonic clock, and progress boundary. */
export interface ArtifactBootstrapControl {
  isCancelled(): boolean;
  nowMs(): number;
  onProgress(progress: ArtifactBootstrapProgress): void;
}

export const DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS: ArtifactBootstrapLimits = Object.freeze({ maxTotalBytes: ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, maxChunks: ARTIFACT_BOOTSTRAP_MAX_CHUNKS, maxChunkBytes: ARTIFACT_BOOTSTRAP_CHUNK_BYTES });

function artifactBootstrapError(message: string): Error {
  return new Error(`artifact bootstrap ${message}`);
}

function equalBytes(left: readonly number[] | Readonly<Uint8Array>, right: readonly number[] | Readonly<Uint8Array>): boolean {
  return left.length === right.length && left.every((byte, index) => byte === right[index]);
}

function validateBytes(name: string, value: readonly number[]): void {
  if (value.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255)) throw artifactBootstrapError(`${name} contains an invalid byte`);
}

function validateHash(name: string, value: readonly number[], nonzero: boolean): void {
  if (value.length !== 32) throw artifactBootstrapError(`${name} must contain 32 bytes`);
  validateBytes(name, value);
  if (nonzero && value.every((byte) => byte === 0)) throw artifactBootstrapError(`${name} must be nonzero`);
}

function validateNatural(name: string, value: number, minimum: number): void {
  if (!Number.isSafeInteger(value) || value < minimum) throw artifactBootstrapError(`${name} is invalid`);
}

function validateLimits(limits: ArtifactBootstrapLimits): void {
  validateNatural("max total bytes", limits.maxTotalBytes, 1);
  validateNatural("max chunks", limits.maxChunks, 1);
  validateNatural("max chunk bytes", limits.maxChunkBytes, 1);
  if (limits.maxChunkBytes > ARTIFACT_BOOTSTRAP_CHUNK_BYTES) throw artifactBootstrapError("max chunk bytes exceeds wire limit");
}

function artifactBootstrapTotal(bootstrap: WireArtifactBootstrap): number {
  validateNatural("pack length", bootstrap.pack_length, 1);
  validateNatural("SPR length", bootstrap.spr_length, 1);
  const total = bootstrap.pack_length + bootstrap.spr_length;
  if (!Number.isSafeInteger(total)) throw artifactBootstrapError("total bytes overflow");
  return total;
}

function validateArtifactBootstrapHeader(bootstrap: WireArtifactBootstrap, inline: boolean, limits: ArtifactBootstrapLimits): number {
  validateLimits(limits);
  if (bootstrap.format_version !== ARTIFACT_BOOTSTRAP_FORMAT_VERSION) throw artifactBootstrapError(`version ${bootstrap.format_version} is unsupported`);
  validateHash("descriptor hash", bootstrap.descriptor_hash, true);
  validateHash("pack schema hash", bootstrap.pack_schema_hash, true);
  validateHash("pack hash", bootstrap.pack_hash, true);
  validateHash("SPR hash", bootstrap.spr_hash, true);
  validateHash("aggregate hash", bootstrap.aggregate_hash, true);
  validateHash("baseline frontier chain hash", bootstrap.baseline_frontier.chain_hash, false);
  validateHash("required tail frontier chain hash", bootstrap.required_tail_frontier.chain_hash, false);
  const schemaBytes = new TextEncoder().encode(bootstrap.artifact_schema).length;
  const kindBytes = new TextEncoder().encode(bootstrap.artifact_kind).length;
  if (schemaBytes === 0 || schemaBytes > 256) throw artifactBootstrapError("artifact schema length is invalid");
  if (kindBytes === 0 || kindBytes > 256) throw artifactBootstrapError("artifact kind length is invalid");
  if (bootstrap.baseline_frontier.document_id.length === 0 || bootstrap.baseline_frontier.document_id !== bootstrap.required_tail_frontier.document_id) throw artifactBootstrapError("frontier document mismatch");
  validateNatural("baseline head", bootstrap.baseline_frontier.head_edit_ordinal, 0);
  validateNatural("baseline commit", bootstrap.baseline_frontier.last_commit_seq, 0);
  validateNatural("required tail head", bootstrap.required_tail_frontier.head_edit_ordinal, 0);
  validateNatural("required tail commit", bootstrap.required_tail_frontier.last_commit_seq, 0);
  if (bootstrap.required_tail_frontier.head_edit_ordinal < bootstrap.baseline_frontier.head_edit_ordinal || bootstrap.required_tail_frontier.last_commit_seq < bootstrap.baseline_frontier.last_commit_seq) throw artifactBootstrapError("required tail frontier precedes baseline");
  const total = artifactBootstrapTotal(bootstrap);
  if (total > limits.maxTotalBytes) throw artifactBootstrapError("total bytes exceed assembler budget");
  validateNatural("chunk count", bootstrap.chunk_count, 0);
  if (inline) {
    if (bootstrap.chunk_count !== 0) throw artifactBootstrapError("inline pair must declare zero chunks");
  } else {
    if (bootstrap.chunk_count === 0 || bootstrap.chunk_count > limits.maxChunks) throw artifactBootstrapError("chunk count exceeds assembler budget");
    if (bootstrap.chunk_count > total || total > bootstrap.chunk_count * limits.maxChunkBytes) throw artifactBootstrapError("chunk count cannot cover declared bytes");
  }
  return total;
}

function validateArtifactBootstrap(bootstrap: WireArtifactBootstrap, limits: ArtifactBootstrapLimits): number {
  const inline = bootstrap.inline !== null;
  const total = validateArtifactBootstrapHeader(bootstrap, inline, limits);
  if (bootstrap.inline !== null) {
    if (bootstrap.inline.pack.length !== bootstrap.pack_length || bootstrap.inline.spr.length !== bootstrap.spr_length) throw artifactBootstrapError("inline pair lengths do not match metadata");
    validateBytes("inline pack", bootstrap.inline.pack);
    validateBytes("inline SPR", bootstrap.inline.spr);
  }
  return total;
}

/** #️⃣ Browser-safe SHA-256 backed by the host Web Crypto implementation. */
export async function artifactBootstrapSha256(bytes: Uint8Array): Promise<Uint8Array> {
  const owned = Uint8Array.from(bytes);
  return new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", owned.buffer));
}

/** 🔗️ SHA-256 over the exact declared `pack || spr` content stream. */
export async function artifactBootstrapAggregateHash(pack: Uint8Array, spr: Uint8Array): Promise<Uint8Array> {
  const content = new Uint8Array(pack.byteLength + spr.byteLength);
  content.set(pack, 0);
  content.set(spr, pack.byteLength);
  return artifactBootstrapSha256(content);
}

/** 🧱️ Bounded, cancellable staging owner that yields a pair only after complete integrity validation. */
export class ArtifactBootstrapAssembler {
  readonly bootstrap: WireArtifactBootstrap;
  readonly limits: ArtifactBootstrapLimits;
  readonly deadlineMs: number | null;
  #storage: Uint8Array | null;
  #received = 0;
  #nextIndex = 0;

  constructor(bootstrap: WireArtifactBootstrap, expectedDescriptorHash: readonly number[], limits: ArtifactBootstrapLimits = DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, deadlineMs: number | null = null, control: ArtifactBootstrapControl) {
    const total = validateArtifactBootstrap(bootstrap, limits);
    validateHash("expected descriptor hash", expectedDescriptorHash, true);
    if (!equalBytes(bootstrap.descriptor_hash, expectedDescriptorHash)) throw artifactBootstrapError("descriptor mismatch");
    if (control.isCancelled()) throw artifactBootstrapError("cancelled");
    if (deadlineMs !== null && control.nowMs() >= deadlineMs) throw artifactBootstrapError("deadline exceeded");
    this.bootstrap = bootstrap;
    this.limits = limits;
    this.deadlineMs = deadlineMs;
    this.#storage = new Uint8Array(total);
    control.onProgress(this.progress);
    if (bootstrap.inline !== null) {
      this.#storage.set(bootstrap.inline.pack, 0);
      this.#storage.set(bootstrap.inline.spr, bootstrap.pack_length);
      this.#received = total;
      control.onProgress(this.progress);
    }
  }

  /** 💾️ Bytes currently owned by the assembler; zero after abort or completion. */
  get retainedBytes(): number {
    return this.#storage?.byteLength ?? 0;
  }

  /** 📈️ Current monotonic progress snapshot. */
  get progress(): ArtifactBootstrapProgress {
    return { receivedBytes: this.#received, totalBytes: this.bootstrap.pack_length + this.bootstrap.spr_length, receivedChunks: this.#nextIndex, totalChunks: this.bootstrap.chunk_count };
  }

  /** 🧹️ Drops all staged bytes without producing a completion value. */
  abort(): void {
    this.#storage?.fill(0);
    this.#storage = null;
  }

  #fail(message: string): never {
    this.abort();
    throw artifactBootstrapError(message);
  }

  #guard(control: ArtifactBootstrapControl): void {
    if (control.isCancelled()) this.#fail("cancelled");
    if (this.deadlineMs !== null && control.nowMs() >= this.deadlineMs) this.#fail("deadline exceeded");
    if (this.#storage === null) throw artifactBootstrapError("is not active");
  }

  /** 🧩️ Appends exactly the next descriptor-bound, nonempty bounded chunk. */
  push(chunk: Readonly<{ descriptor_hash: readonly number[]; index: number; bytes: readonly number[] }>, control: ArtifactBootstrapControl): ArtifactBootstrapProgress {
    this.#guard(control);
    if (this.bootstrap.inline !== null) this.#fail("inline transfer cannot accept chunks");
    if (!equalBytes(chunk.descriptor_hash, this.bootstrap.descriptor_hash)) this.#fail("chunk descriptor mismatch");
    if (chunk.index !== this.#nextIndex) this.#fail(`chunk index ${chunk.index} does not equal expected ${this.#nextIndex}`);
    if (chunk.bytes.length === 0 || chunk.bytes.length > this.limits.maxChunkBytes) this.#fail("chunk bytes exceed assembler budget");
    try {
      validateBytes("chunk", chunk.bytes);
    } catch {
      this.#fail("chunk contains an invalid byte");
    }
    if (chunk.index >= this.bootstrap.chunk_count) this.#fail("chunk index exceeds declared count");
    const end = this.#received + chunk.bytes.length;
    if (end > this.progress.totalBytes) this.#fail("chunk bytes exceed declared total");
    this.#storage!.set(chunk.bytes, this.#received);
    this.#received = end;
    this.#nextIndex += 1;
    control.onProgress(this.progress);
    return this.progress;
  }

  /** ✅️ Verifies completion and all hashes, transfers ownership of the pair, then retires staging. */
  async finish(done: Readonly<{ descriptor_hash: readonly number[]; chunk_count: number }> | null, control: ArtifactBootstrapControl): Promise<ArtifactBootstrapPair> {
    try {
      this.#guard(control);
      if (this.bootstrap.inline === null) {
        if (done === null) this.#fail("is incomplete without done frame");
        if (!equalBytes(done.descriptor_hash, this.bootstrap.descriptor_hash)) this.#fail("done descriptor mismatch");
        if (done.chunk_count !== this.bootstrap.chunk_count) this.#fail("done chunk count mismatch");
      } else if (done !== null && (!equalBytes(done.descriptor_hash, this.bootstrap.descriptor_hash) || done.chunk_count !== 0)) {
        this.#fail("inline done metadata mismatch");
      }
      if (this.#nextIndex !== this.bootstrap.chunk_count || this.#received !== this.progress.totalBytes) this.#fail("is incomplete");
      const storage = this.#storage!;
      const pack = storage.subarray(0, this.bootstrap.pack_length);
      const spr = storage.subarray(this.bootstrap.pack_length);
      const [packHash, sprHash, aggregateHash] = await Promise.all([artifactBootstrapSha256(pack), artifactBootstrapSha256(spr), artifactBootstrapSha256(storage)]);
      this.#guard(control);
      if (!equalBytes(packHash, this.bootstrap.pack_hash)) this.#fail("pack hash mismatch");
      if (!equalBytes(sprHash, this.bootstrap.spr_hash)) this.#fail("SPR hash mismatch");
      if (!equalBytes(aggregateHash, this.bootstrap.aggregate_hash)) this.#fail("aggregate hash mismatch");
      this.#storage = null;
      return { pack, spr };
    } catch (error) {
      this.abort();
      throw error;
    }
  }
}
//#endregion 🔖️ArtifactBootstrap

//#region 🔖️NestedEnums
function encodeArtifactBootstrap(out: number[], bootstrap: WireArtifactBootstrap): void {
  writeVarintU64(out, bootstrap.format_version);
  writeHash32(out, bootstrap.descriptor_hash);
  writeStr(out, bootstrap.artifact_schema);
  writeStr(out, bootstrap.artifact_kind);
  writeHash32(out, bootstrap.pack_schema_hash);
  encodeFrontier(out, bootstrap.baseline_frontier);
  writeHash32(out, bootstrap.pack_hash);
  writeHash32(out, bootstrap.spr_hash);
  writeVarintU64(out, bootstrap.pack_length);
  writeVarintU64(out, bootstrap.spr_length);
  writeVarintU64(out, bootstrap.chunk_count);
  writeHash32(out, bootstrap.aggregate_hash);
  encodeFrontier(out, bootstrap.required_tail_frontier);
  writeBool(out, bootstrap.inline !== null);
  if (bootstrap.inline !== null) {
    writeBytes(out, bootstrap.inline.pack);
    writeBytes(out, bootstrap.inline.spr);
  }
}

function readArtifactBootstrapString(bytes: Uint8Array, pos: [number], name: string): string {
  const length = readVarintU64(bytes, pos);
  if (!Number.isSafeInteger(length) || length === 0 || length > 256) throw artifactBootstrapError(`${name} length is invalid`);
  const slice = bytes.subarray(pos[0], pos[0] + length);
  if (slice.length !== length) throw artifactBootstrapError(`${name} is truncated`);
  pos[0] += length;
  return new TextDecoder("utf-8", { fatal: true }).decode(slice);
}

function readArtifactBootstrapBytes(bytes: Uint8Array, pos: [number], expected: number, name: string): number[] {
  const length = readVarintU64(bytes, pos);
  if (!Number.isSafeInteger(length) || length !== expected || length > ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES) throw artifactBootstrapError(`${name} length does not match metadata`);
  const slice = bytes.subarray(pos[0], pos[0] + length);
  if (slice.length !== length) throw artifactBootstrapError(`${name} is truncated`);
  pos[0] += length;
  return Array.from(slice);
}

function readArtifactBootstrapChunkBytes(bytes: Uint8Array, pos: [number]): number[] {
  const length = readVarintU64(bytes, pos);
  if (!Number.isSafeInteger(length) || length === 0 || length > ARTIFACT_BOOTSTRAP_CHUNK_BYTES) throw artifactBootstrapError("chunk bytes exceed wire limit");
  const slice = bytes.subarray(pos[0], pos[0] + length);
  if (slice.length !== length) throw artifactBootstrapError("chunk bytes are truncated");
  pos[0] += length;
  return Array.from(slice);
}

function decodeArtifactBootstrap(bytes: Uint8Array, pos: [number]): WireArtifactBootstrap {
  const partial: WireArtifactBootstrap = {
    format_version: readVarintU64(bytes, pos),
    descriptor_hash: readHash32(bytes, pos),
    artifact_schema: readArtifactBootstrapString(bytes, pos, "artifact schema"),
    artifact_kind: readArtifactBootstrapString(bytes, pos, "artifact kind"),
    pack_schema_hash: readHash32(bytes, pos),
    baseline_frontier: decodeFrontier(bytes, pos),
    pack_hash: readHash32(bytes, pos),
    spr_hash: readHash32(bytes, pos),
    pack_length: readVarintU64(bytes, pos),
    spr_length: readVarintU64(bytes, pos),
    chunk_count: readVarintU64(bytes, pos),
    aggregate_hash: readHash32(bytes, pos),
    required_tail_frontier: decodeFrontier(bytes, pos),
    inline: null,
  };
  const inline = readBool(bytes, pos);
  validateArtifactBootstrapHeader(partial, inline, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS);
  const decoded = inline ? { ...partial, inline: { pack: readArtifactBootstrapBytes(bytes, pos, partial.pack_length, "inline pack"), spr: readArtifactBootstrapBytes(bytes, pos, partial.spr_length, "inline SPR") } } : partial;
  validateArtifactBootstrap(decoded, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS);
  return decoded;
}

function encodeBootstrap(out: number[], bootstrap: WireBootstrap): void {
  if (bootstrap === "None") {
    out.push(0);
    return;
  }
  if (bootstrap === "Tail") {
    out.push(2);
    return;
  }
  if ("Snapshot" in bootstrap) {
    out.push(1);
    writeHash32(out, bootstrap.Snapshot.pack_hash);
    writeOptBytes(out, bootstrap.Snapshot.inline);
    return;
  }
  out.push(3);
  encodeArtifactBootstrap(out, bootstrap.ArtifactBootstrap);
}
function decodeBootstrap(bytes: Uint8Array, pos: [number]): WireBootstrap {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire bootstrap tag: truncated");
  pos[0] += 1;
  if (tag === 0) return "None";
  if (tag === 2) return "Tail";
  if (tag === 1) return { Snapshot: { pack_hash: readHash32(bytes, pos), inline: readOptBytes(bytes, pos) } };
  if (tag === 3) return { ArtifactBootstrap: decodeArtifactBootstrap(bytes, pos) };
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
  if ("SocketHelloV1" in frame) {
    out.push(7);
    const hello = frame.SocketHelloV1;
    writeVarintU64(out, hello.wire_version);
    writeVarintU64(out, hello.protocol_version);
    writeStr(out, hello.schema);
    writeHash32(out, hello.pack_schema_hash);
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
    case 7:
      frame = {
        SocketHelloV1: {
          wire_version: readVarintU64(bytes, pos),
          protocol_version: readVarintU64(bytes, pos),
          schema: readStr(bytes, pos),
          pack_schema_hash: readHash32(bytes, pos),
          resume_token: readOptStr(bytes, pos),
          frontier: readOptFrontier(bytes, pos),
        },
      };
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
  } else if ("ArtifactBootstrapChunk" in frame) {
    out.push(10);
    writeHash32(out, frame.ArtifactBootstrapChunk.descriptor_hash);
    writeVarintU64(out, frame.ArtifactBootstrapChunk.index);
    writeBytes(out, frame.ArtifactBootstrapChunk.bytes);
  } else if ("ArtifactBootstrapDone" in frame) {
    out.push(11);
    writeHash32(out, frame.ArtifactBootstrapDone.descriptor_hash);
    writeVarintU64(out, frame.ArtifactBootstrapDone.chunk_count);
  } else if ("RebootstrapRequired" in frame) {
    out.push(12);
    writeStr(out, frame.RebootstrapRequired.control.space_id);
    writeStr(out, frame.RebootstrapRequired.control.document_id);
    writeHash32(out, frame.RebootstrapRequired.control.checkpoint_id);
    writeHash32(out, frame.RebootstrapRequired.control.descriptor_hash);
    encodeFrontier(out, frame.RebootstrapRequired.control.baseline_frontier);
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
    case 10:
      frame = { ArtifactBootstrapChunk: { descriptor_hash: readHash32(bytes, pos), index: readVarintU64(bytes, pos), bytes: readArtifactBootstrapChunkBytes(bytes, pos) } };
      break;
    case 11:
      frame = { ArtifactBootstrapDone: { descriptor_hash: readHash32(bytes, pos), chunk_count: readVarintU64(bytes, pos) } };
      break;
    case 12: {
      const control: WireRebootstrapRequired = {
        space_id: readStr(bytes, pos),
        document_id: readStr(bytes, pos),
        checkpoint_id: readHash32(bytes, pos),
        descriptor_hash: readHash32(bytes, pos),
        baseline_frontier: decodeFrontier(bytes, pos),
      };
      if (!control.space_id || !control.document_id || control.space_id.length > 256 || control.document_id.length > 256 || new TextEncoder().encode(control.space_id).length > 256 || new TextEncoder().encode(control.document_id).length > 256 || control.checkpoint_id.every((byte) => byte === 0) || control.descriptor_hash.every((byte) => byte === 0) || control.baseline_frontier.document_id !== control.document_id || !control.baseline_frontier.head_edit_id) {
        throw new Error("artifact bootstrap: rebootstrap control identity is invalid");
      }
      frame = { RebootstrapRequired: { control } };
      break;
    }
    default:
      throw new Error(`wire server-frame tag: unknown tag ${tag}`);
  }
  return { lane, frame };
}

/** 🪡 Extracts and validates the exact causal batch embedded in a server `Commands` frame. */
export function extractServerCommandsDocumentBackboneBatchExact(bytes: Uint8Array): Uint8Array | null {
  if (!(bytes instanceof Uint8Array) || bytes.length < 2) throw new DocumentBackboneBatchError("malformed", "server-frame");
  if (WIRE_BYTE_LANES[bytes[0]] === undefined) throw new DocumentBackboneBatchError("malformed", "server-frame");
  if (bytes[1] !== 3) return null;
  const position: [number] = [2];
  return readDocumentBackboneEnvelopeBatchExact(bytes, position).bytes;
}

/** 🗃️ A durable place a document synchronizes with — mirrors Rust `PersistenceBinding`. `surface`
 * (contract-freeze §C0 "Presence scope") travels out of band on the document WS URL's `?surface=`
 * query param — see `connectHub` in `🧵️backbone-worker.ts`'s `🔖️Hub` region. No `PresencePeer` wire
 * change: its flag byte is full and the file is peer-leased. */
//#endregion 🔖️SyncProtocol

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️artifact-bootstrap-protocol/🟦️.ts");
  await registerTests1(import.meta.vitest, { ArtifactBootstrapAssembler, artifactBootstrapAggregateHash, artifactBootstrapSha256, decodeClientFrame, decodePresencePeer, decodeServerFrame, encodeClientFrame, encodePresencePeer, encodeServerFrame }, { directory: import.meta.dir, url: import.meta.url });
  const { registerTests2 } = await import("./🧪️tests/🧪️document-backbone-envelope-batch/🟦️.ts");
  await registerTests2(import.meta.vitest, { DocumentBackboneBatchError, decodeDocumentBackboneEnvelopeBatchExact, encodeDocumentBackboneEnvelopeBatchExact }, { directory: import.meta.dir, url: import.meta.url });
}
