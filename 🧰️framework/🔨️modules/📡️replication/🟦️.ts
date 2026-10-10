//! 📡️ Replication contract — TypeScript twin of the Rust `protocol` crate.
//!
//! Byte-for-byte identical to `📦️packages/🦀️rust`'s encoders: the 20 frames in `🧫️fixtures/📡️wire`
//! are the shared gate both sides must reproduce. Frame layout is `lane u8`, `frame tag u8`, then
//! fields in declaration order — no length prefix, no per-field tags.

import { artifactBootstrapSha256, artifactBootstrapAggregateHash, ArtifactBootstrapAssembler } from "./🚪️io/📥️bootstrap/🟦️.ts";
export { ARTIFACT_BOOTSTRAP_FORMAT_VERSION, ARTIFACT_BOOTSTRAP_CHUNK_BYTES, ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, ARTIFACT_BOOTSTRAP_MAX_CHUNKS, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, artifactBootstrapSha256, artifactBootstrapAggregateHash, ArtifactBootstrapAssembler } from "./🚪️io/📥️bootstrap/🟦️.ts";
export type { ArtifactBootstrapLimits, ArtifactBootstrapProgress, ArtifactBootstrapControl } from "./🚪️io/📥️bootstrap/🟦️.ts";
import { encodeClientFrame, encodeClientCommandsFrameExact, decodeClientFrame, encodeServerFrame, decodeServerFrame } from "./🚪️io/💾️binary/📡️wire/🟦️.ts";
export { encodeClientFrame, encodeClientCommandsFrameExact, decodeClientFrame, encodeServerFrame, decodeServerFrame, extractServerCommandsDocumentBackboneBatchExact } from "./🚪️io/💾️binary/📡️wire/🟦️.ts";
import { DocumentBackboneBatchError, encodeDocumentBackboneEnvelopeBatchExact, decodeDocumentBackboneEnvelopeBatchExact } from "./🚪️io/💾️binary/🔗️causal/🧮️backbone/🟦️.ts";
export { DOCUMENT_BACKBONE_BATCH_LIMITS, DocumentBackboneBatchError, encodeDocumentBackboneEnvelopeBatchExact, decodeDocumentBackboneEnvelopeBatchExact, readDocumentBackboneEnvelopeBatchExact } from "./🚪️io/💾️binary/🔗️causal/🧮️backbone/🟦️.ts";
export type { DocumentBackboneBatchLimits } from "./🚪️io/💾️binary/🔗️causal/🧮️backbone/🟦️.ts";
import { writeVecEnvelope, readVecEnvelope } from "./🚪️io/💾️binary/🔗️causal/🟦️.ts";
export { writeVecStr, readVecStr, writeVecEnvelope, readVecEnvelope, decodeCausalEnvelopeBatch, encodeCausalEnvelopeBatch, mutationEnvelopeToWire, mutationEnvelopeFromWire } from "./🚪️io/💾️binary/🔗️causal/🟦️.ts";
export type { ReplicationPackCodec } from "./🚪️io/💾️binary/🔗️causal/🟦️.ts";
import { blake3Hex } from "../🔏️hash/🟦️.ts";
import { encodePresencePeer, decodePresencePeer } from "./🚪️io/💾️binary/👥️presence/🟦️.ts";
export { encodePresencePeer, PRESENCE_PEER_WIRE_LIMITS_V1, decodePresencePeer, encodePresenceToolRun, decodePresenceToolRun, encodePresenceHistoryEdit, decodePresenceHistoryEdit, readU8, encodePresenceInteraction, decodePresenceInteraction } from "./🚪️io/💾️binary/👥️presence/🟦️.ts";
import { writeVarintU64, writeStr, writeBytes } from "./🚪️io/💾️binary/🟦️.ts";
export { writeVarintU64, readVarintU64, writeVarintU64Exact, readVarintU64Exact, writeStr, readStr, writeBytes, readBytes, writeHash32, readHash32, writeBool, readBool, writeF64, readF64, writeVecBytes, readVecBytes } from "./🚪️io/💾️binary/🟦️.ts";

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
  readonly transaction?: TransactionRef;
  readonly verb?: string;
};

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
  /** ⏯️ Summary of this peer's tool run (bit 10, ARTIFACT scope), never provisional geometry. */
  readonly toolRun?: ArtifactPresenceToolRun;
  /** 🤖️ What kind of principal this peer is (bit 11). Like `userId`/`role`/`color` the Hub replaces
   * whatever a client sends with the kind it AUTHENTICATED, so a human session can never claim to be
   * an agent and an agent session can never hide as a human. `undefined` is the pre-agent wire shape
   * and a reader must treat it as `"human"`. */
  readonly principalKind?: ArtifactPresencePrincipalKind;
  /** 🛠️ Active editor tool/utility id (bit 12, ARTIFACT scope) — e.g. select/brush/fill. Optional when
   * the peer has not published an active tool. Distinct from `toolRun` (interactive long-running tool). */
  readonly activeTool?: string;
  /** ⏪️ Summary of this peer's open history edit (bit 13, ARTIFACT scope): who edits history, on which mutation, at
   * which stage. */
  readonly historyEdit?: ArtifactPresenceHistoryEdit;
  /** ⌨️ This peer's pending typing runs, one per text window (bit 14, ARTIFACT scope): the ephemeral shared preview of text it
   * typed that has not committed yet — never history. */
  readonly typing?: readonly ArtifactPresenceTyping[];
};

/** ⌨️ Twin of Rust `PresenceTyping`: one peer's pending typing run in one text window — what it deleted and typed so far,
 * excerpts of at most {@link PRESENCE_TYPING_EXCERPT_BYTES} UTF-8 bytes each, cut at a scalar boundary. */
export type ArtifactPresenceTyping = {
  readonly windowId: string;
  readonly deleted: string;
  readonly insert: string;
};

/** 📏️ Twin of Rust `PRESENCE_TYPING_EXCERPT_BYTES`. */
export const PRESENCE_TYPING_EXCERPT_BYTES = 256;

/** ✂️ Twin of Rust `PresenceTyping::excerpt`: `text` cut to {@link PRESENCE_TYPING_EXCERPT_BYTES} UTF-8 bytes at a scalar boundary. */
export function presenceTypingExcerpt(text: string): string {
  let bytes = 0;
  let excerpt = "";
  for (const scalar of text) {
    bytes += new TextEncoder().encode(scalar).length;
    if (bytes > PRESENCE_TYPING_EXCERPT_BYTES) break;
    excerpt += scalar;
  }
  return excerpt;
}

/** ⏪️ Twin of Rust `PresenceHistoryEditStage`: the `HistoryTimeTravelStage` wire spelling in binary tag order. */
export const PRESENCE_HISTORY_EDIT_STAGES = Object.freeze(["editing", "replaying", "reviewing", "choosing", "finalizing"] as const);

/** 🔤️ One history-edit stage wire name. */
export type ArtifactPresenceHistoryEditStage = (typeof PRESENCE_HISTORY_EDIT_STAGES)[number];

/** 👥️ Twin of Rust `PresenceHistoryEdit`: the replica-independent id of the mutation a peer's open history edit drafts
 * (every replica labels it from its own history rows), the stage and the accepted draft count. */
export type ArtifactPresenceHistoryEdit = {
  readonly mutationId: string;
  readonly stage: ArtifactPresenceHistoryEditStage;
  readonly drafts: number;
};

/** 🤖️ Twin of Rust `PresencePrincipalKind`, in binary tag order. An `agent` peer is an AI agent
 * acting under a credential a human delegated to it (`hub.auth`'s `AgentDelegationRecord`); it is a
 * principal in its own right, never the delegating human, so a roster shows it as its own row. */
export const PRESENCE_PRINCIPAL_KINDS = Object.freeze(["human", "agent"] as const);

/** 🔤️ One `PresencePrincipalKind` wire name. */
export type ArtifactPresencePrincipalKind = (typeof PRESENCE_PRINCIPAL_KINDS)[number];

/** ⏳️ Twin of Rust `PresenceToolRunState`: the `ToolRunState` wire spelling (tool run contract §2.2) in binary tag order.
 * Duplicated because the tool run module layers above replication. */
export const PRESENCE_TOOL_RUN_STATES = Object.freeze(["starting", "running", "paused", "complete", "finalizing", "finalized", "aborting", "aborted", "faulted"] as const);

/** 🔤️ One `ToolRunState` wire name. */
export type ArtifactPresenceToolRunState = (typeof PRESENCE_TOOL_RUN_STATES)[number];

/** 📶️ Twin of Rust `PresenceToolRun`: `total` absent means indeterminate and `completed <= total` otherwise. */
export type ArtifactPresenceToolRun = {
  readonly toolId: string;
  readonly state: ArtifactPresenceToolRunState;
  readonly stage: number;
  readonly completed: number;
  readonly total?: number;
};

/** 🪟️ Twin of Rust `PresenceWindowView`. */
export type ArtifactPresenceWindowView = {
  readonly windowId: string;
  readonly space: string;
  readonly kind: ArtifactPresenceViewKind;
  readonly size: readonly [number, number];
  readonly pointer?: readonly [number, number, number];
  /** World-space ray origin for 3D presence (Orbit); absent for canvas/geo. */
  readonly rayOrigin?: readonly [number, number, number];
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
  /** 👁️ Advisory, never an ordering constraint: the newest operation of ANOTHER author the author's replica had applied. */
  readonly observed: string | null;
  /** 🎯️ The structured address the operation writes (outermost segment first; empty: the whole artifact). */
  readonly target: readonly string[];
  readonly diff: { readonly schema: string; readonly payload: readonly number[] };
  readonly inverse: { readonly schema: string; readonly payload: readonly number[] };
  readonly timestamp: { readonly actor: number; readonly physical_ms: number; readonly logical: number };
  /** 🧾️ The committed tool transaction that authored the operation (`null`: none, and always for a transition). */
  readonly transaction: TransactionRef | null;
  /** 🏷️ The id of the action or command whose edit carried the operation, never display text (`null`: none, and always
   * for a transition) — a peer resolves it through the authoring app's registry to the same history label. */
  readonly verb: string | null;
  /** 🌿️ The alternative the operation was authored on (`null`: the trunk). */
  readonly line: string | null;
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

/** 🔀️ `diff.schema` of every history-transition envelope — undo, redo, checkpoint commit, branch, checkout, pin — the TS
 * twin of Rust's `HISTORY_TRANSITION_SCHEMA`, pinned by the neutral schema's `diffSchema` const. Operation envelopes carry
 * their artifact's own schema, so this tag alone tells a framework history record from an app mutation.
 * @see ./🔗️causal/🧬️schema/🔣️history-transition/🔣️.json
 * @see ./🔗️causal/🔀️transition/🦀️.rs */
export const HISTORY_TRANSITION_DIFF_SCHEMA = "semio.history.transition";

//#region 🔖️HistoryEditing
/** 🧾️ Twin of Rust `protocol::TransactionRef`: the committed tool transaction (`tx-<hex16>`) and the authoring
 * action/tool id (`<appId>#<toolId>`) stamped on every operation that transaction produced.
 * @see ./🎮️mutation/🦀️.rs */
export type TransactionRef = { readonly id: string; readonly tool: string };

/** 🪪️ Twin of Rust `TransactionRef::mint`: `tx-{hex16(blake3(actor str | hlc.actor varint | hlc.physical_ms varint |
 * hlc.logical varint | tool str))}`, byte-identical to the Rust material. */
export function mintTransactionRef(actor: string, hlc: WireMutationEnvelope["timestamp"], tool: string): TransactionRef {
  const material: number[] = [];
  writeStr(material, actor);
  writeVarintU64(material, hlc.actor);
  writeVarintU64(material, hlc.physical_ms);
  writeVarintU64(material, hlc.logical);
  writeStr(material, tool);
  return { id: `tx-${blake3Hex(new Uint8Array(material)).slice(0, 16)}`, tool };
}

/** 🪪️ Twin of Rust `history_transition_id`: `transition-{hex16(blake3(hlc.actor varint | hlc.physical_ms varint |
 * hlc.logical varint | payload bytes))}`. The free-form actor string is authentication metadata and never enters the
 * content address. */
export function historyTransitionId(hlc: WireMutationEnvelope["timestamp"], payload: readonly number[] | Uint8Array): string {
  const material: number[] = [];
  writeVarintU64(material, hlc.actor);
  writeVarintU64(material, hlc.physical_ms);
  writeVarintU64(material, hlc.logical);
  writeBytes(material, Array.from(payload));
  return `transition-${blake3Hex(new Uint8Array(material)).slice(0, 16)}`;
}

/** 🏷️ Twin of Rust `HistoryTransitionKind`: every transition kind, in wire-tag order. */
export const HISTORY_TRANSITION_KINDS = Object.freeze(["revert", "reinstate", "commit", "branch", "checkout", "repin", "supersede"] as const);

/** 🪪️ One transition kind. */
export type HistoryTransitionKind = (typeof HISTORY_TRANSITION_KINDS)[number];

/** 🗂️ Twin of Rust `HistoryShape`: a `document` history holds every transition, a `config` history only the undo and
 * redo of its edits — no checkpoint, alternative, pin or supersession. */
export type HistoryShape = "document" | "config";

/** 🛂️ Twin of Rust `HistoryShape::admits`. */
export function historyShapeAdmits(shape: HistoryShape, kind: HistoryTransitionKind): boolean {
  return HISTORY_TRANSITION_KINDS.includes(kind) && (shape === "document" || shape === "config" && (kind === "revert" || kind === "reinstate"));
}

/** 🌳️ Twin of Rust `trunk_alternative_id`: the id of `documentId`'s trunk, the implicit root line every document starts
 * on — `trunk-{hex16(blake3(str "semio.history.trunk" | str documentId))}`. The log never names it. */
export function trunkAlternativeId(documentId: string): string {
  const material: number[] = [];
  writeStr(material, "semio.history.trunk");
  writeStr(material, documentId);
  return `trunk-${blake3Hex(new Uint8Array(material)).slice(0, 16)}`;
}

/** ♻️ Twin of Rust `InputReplacement` (`ToValue` shape): an operation's replacement input (the artifact aggregate op's
 * canonical `OpBinary` bytes under `schema`) or a withdrawal that folds the operation as a no-op. */
export type InputReplacement = { readonly kind: "input"; readonly schema: string; readonly payload: readonly number[] } | { readonly kind: "withdrawn" };

/** 🎯️ Twin of Rust `SupersededInput`. */
export type SupersededInput = { readonly target: string; readonly replacement: InputReplacement };

/** ✏️ Twin of Rust `TransitionSupersede` (history transition tag 6). */
export type TransitionSupersede = { readonly scope: string | null; readonly inputs: readonly SupersededInput[] };

/** 🧭️ Twin of Rust `EffectiveSupersession` (`ToValue` shape). */
export type EffectiveSupersession = {
  readonly transitionId: string;
  readonly actor: string;
  readonly timestamp: WireMutationEnvelope["timestamp"];
  readonly scope: string | null;
  readonly replacement: InputReplacement;
};

/** 🔀️ The part of a history transition the supersession fold reads: `branch`/`checkout` move the alternative,
 * `supersede` installs replacements; every other kind leaves both untouched. */
export type SupersessionFoldTransition =
  | { readonly kind: "branch"; readonly alternativeId: string }
  | { readonly kind: "checkout"; readonly alternativeId: string | null }
  | ({ readonly kind: "supersede" } & TransitionSupersede)
  | { readonly kind: "revert" | "reinstate" | "commit" | "repin" };

/** ✉️ One transition event of a document log as the supersession fold sees it. */
export type SupersessionFoldEvent = { readonly id: string; readonly actor: string; readonly timestamp: WireMutationEnvelope["timestamp"]; readonly transition: SupersessionFoldTransition };

/** 🧮️ Twin of the supersession half of Rust `fold_history` for the document `documentId`: events in
 * `(physical_ms, logical, actor, id)` order; an operation's effective supersession is the last `supersede` naming it
 * whose scope is `null` or the final alternative — the trunk's id ({@link trunkAlternativeId}) while the trunk is active
 * (`alternative` is `null` then: a `checkout` naming the trunk or none activates it, a `branch` may not claim it). No
 * ownership rule. A target outside `operations` is refused like Rust's unknown-operation fold error.
 * @see ./🔗️causal/🔀️transition/🦀️.rs */
export function foldSupersessions(documentId: string, operations: ReadonlySet<string>, events: readonly SupersessionFoldEvent[], head?: { readonly lineId: string; readonly checkpointId?: string | null }): Readonly<{ alternative: string | null; trunk: string; supersessions: ReadonlyMap<string, EffectiveSupersession> }> {
  const trunk = trunkAlternativeId(documentId);
  const key = (event: SupersessionFoldEvent): readonly [number, number, number] => [event.timestamp.physical_ms, event.timestamp.logical, event.timestamp.actor];
  const ordered = [...events].sort((left, right) => {
    const [a, b] = [key(left), key(right)];
    return a[0] - b[0] || a[1] - b[1] || a[2] - b[2] || (left.id < right.id ? -1 : left.id > right.id ? 1 : 0);
  });
  const candidates: [string, EffectiveSupersession][] = [];
  for (const event of ordered) {
    const transition = event.transition;
    if (transition.kind === "branch") {
      if (transition.alternativeId === trunk) throw new Error(`history fold: branch claims the trunk alternative ${trunk}`);
    } else if (transition.kind === "supersede") {
      for (const input of transition.inputs) {
        if (!operations.has(input.target)) throw new Error(`history fold: transition references unknown operation ${input.target}`);
        candidates.push([input.target, { transitionId: event.id, actor: event.actor, timestamp: event.timestamp, scope: transition.scope, replacement: input.replacement }]);
      }
    }
  }
  const activeLine = head?.lineId ?? trunk;
  const alternative = activeLine === trunk ? null : activeLine;
  const supersessions = new Map<string, EffectiveSupersession>();
  for (const [target, supersession] of candidates) if (supersession.scope === null || supersession.scope === activeLine) supersessions.set(target, supersession);
  return { alternative, trunk, supersessions };
}

/** 🚦️ Twin of Rust `diagnostic::Severity` (`ToValue` spelling), in level order. */
export const REPLAY_SEVERITIES = Object.freeze(["info", "warning", "error", "fatal"] as const);

/** 🚦️ One `Severity` wire name. */
export type ReplaySeverity = (typeof REPLAY_SEVERITIES)[number];

/** 📖️ Twin of Rust `OUTCOME_CODES`: the frozen outcome-code vocabulary and the one level each code fixes
 * (`🎮️mutation/🧫️fixtures/🧫️outcome-code`). */
export const OUTCOME_CODES = Object.freeze([
  ["mutation.target-missing", "error"],
  ["mutation.target-referenced", "error"],
  ["mutation.target-mismatch", "error"],
  ["mutation.no-op", "warning"],
  ["mutation.partial", "warning"],
  ["mutation.clamped", "warning"],
  ["mutation.precondition-drifted", "warning"],
  ["mutation.duplicate-id", "fatal"],
  ["mutation.invariant", "fatal"],
  ["mutation.inverse-refused", "fatal"],
  ["mutation.cascade", "info"],
] as const satisfies readonly (readonly [string, ReplaySeverity])[]);

/** 🧱️ Twin of Rust `APPLY_OUTCOME_CODE_PREFIX`: the apply-time rejection family, always `fatal`. */
export const APPLY_OUTCOME_CODE_PREFIX = "mutation.apply.";

/** ⚖️ Twin of Rust `outcome_code_level`: the level the vocabulary fixes for `code`, `null` outside it. */
export function outcomeCodeLevel(code: string): ReplaySeverity | null {
  const known = OUTCOME_CODES.find(([candidate]) => candidate === code);
  if (known) return known[1];
  return code.startsWith(APPLY_OUTCOME_CODE_PREFIX) && /^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(code.slice(APPLY_OUTCOME_CODE_PREFIX.length)) ? "fatal" : null;
}

/** 📨️ Twin of Rust `MutationMessage` (`ToValue` shape: `target`/`opIndex` omitted when empty). */
export type ReplayMutationMessage ={ readonly level: ReplaySeverity; readonly code: string; readonly message: string; readonly target?: readonly string[]; readonly opIndex?: number };

/** 🔬️ Twin of Rust `conflict::MutationReplayOutcome`. */
export type MutationReplayOutcome = {
  readonly mutationId: string;
  readonly editId: string;
  readonly opIndex: number;
  readonly worst: ReplaySeverity | null;
  readonly messages: readonly ReplayMutationMessage[];
  readonly superseded: boolean;
  readonly withdrawn: boolean;
};

/** 📋️ Twin of Rust `conflict::ReplayReport`. */
export type ReplayReport = { readonly fromPosition: number; readonly outcomes: readonly MutationReplayOutcome[]; readonly worst: ReplaySeverity | null };

/** 🚧️ Twin of Rust `ReplayReport::blocks_finalize`: any outcome at `error` or `fatal` (the `MergePolicy::Normal` floor). */
export function replayReportBlocksFinalize(report: ReplayReport): boolean {
  return report.outcomes.some((outcome) => outcome.worst === "error" || outcome.worst === "fatal");
}
//#endregion 🔖️HistoryEditing

export const DOCUMENT_BACKBONE_RETENTION_LIMITS = {
  maximumBytes: 1_048_576,
  maximumMessages: 64,
} as const;

export type ExactWireMutationEnvelope = Readonly<{
  mutation_id: string;
  document_id: string;
  actor: string;
  dependencies: readonly string[];
  observed: string | null;
  target: readonly string[];
  diff: Readonly<{ schema: string; payload: Uint8Array }>;
  inverse: Readonly<{ schema: string; payload: Uint8Array }>;
  timestamp: Readonly<{ actor: bigint; physical_ms: bigint; logical: bigint }>;
  transaction: TransactionRef | null;
  verb: string | null;
  line: string | null;
}>;




/** 🗃️ A durable place a document synchronizes with — mirrors Rust `PersistenceBinding`. `surface`
 * (contract-freeze §C0 "Presence scope") travels out of band on the document WS URL's `?surface=`
 * query param — see `connectHub` in `🧵️backbone-worker.ts`'s `🔖️Hub` region. No `PresencePeer` wire
 * change: its flag byte is full and the file is peer-leased. */
//#endregion 🔖️SyncProtocol

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️artifact-bootstrap-protocol/🟦️.ts");
  await registerTests1(import.meta.vitest, { ArtifactBootstrapAssembler, artifactBootstrapAggregateHash, artifactBootstrapSha256, decodeClientFrame, decodePresencePeer, decodeServerFrame, encodeClientFrame, encodePresencePeer, encodeServerFrame }, { directory: import.meta.dir, url: import.meta.url });
  const { registerTests2 } = await import("./🧪️tests/🧪️document-backbone-envelope-batch/🟦️.ts");
  await registerTests2(import.meta.vitest, { DOCUMENT_BACKBONE_RETENTION_LIMITS, DocumentBackboneBatchError, decodeDocumentBackboneEnvelopeBatchExact, encodeDocumentBackboneEnvelopeBatchExact, encodeClientCommandsFrameExact, encodeClientFrame }, { directory: import.meta.dir, url: import.meta.url });
  const { registerTests3 } = await import("./🧪️tests/🧪️history-transition/🟦️.ts");
  await registerTests3(import.meta.vitest, { directory: import.meta.dir, url: import.meta.url }, HISTORY_TRANSITION_DIFF_SCHEMA, { historyTransitionId, trunkAlternativeId, historyShapeAdmits, HISTORY_TRANSITION_KINDS });
  const { registerTests: registerDurableCollaborativeRedoTests } = await import("./🧪️tests/🗄️durable-collaborative-redo/🟦️.ts");
  await registerDurableCollaborativeRedoTests(import.meta.vitest, { directory: import.meta.dir, url: import.meta.url });
  const { registerSupersedeFoldTests } = await import("./🧪️tests/🧪️supersede-fold/🟦️.ts");
  await registerSupersedeFoldTests(import.meta.vitest, { foldSupersessions }, { directory: import.meta.dir, url: import.meta.url });
  const { registerTransactionRefTests } = await import("./🧪️tests/🧪️transaction-ref/🟦️.ts");
  await registerTransactionRefTests(import.meta.vitest, { mintTransactionRef, writeVecEnvelope, readVecEnvelope }, { directory: import.meta.dir, url: import.meta.url });
  const { registerReplayReportTests } = await import("./🧪️tests/🧪️replay-report/🟦️.ts");
  await registerReplayReportTests(import.meta.vitest, { replayReportBlocksFinalize, REPLAY_SEVERITIES }, { directory: import.meta.dir, url: import.meta.url });
  const { registerOutcomeCodeTests } = await import("./🧪️tests/🧪️outcome-code/🟦️.ts");
  await registerOutcomeCodeTests(import.meta.vitest, { OUTCOME_CODES, APPLY_OUTCOME_CODE_PREFIX, outcomeCodeLevel }, { directory: import.meta.dir, url: import.meta.url });

}

export {
  canvasPeerViewportRect,
  canvasPointToScreen,
  orbitPointToScreen,
  orbitFrustumCorners,
  orbitFrustumSegments,
  type OrbitFrustumCorners,
  peerOverlayLabels,
  peerOverlayPath,
  peerMarksFor,
  peersForWindow,
  presenceColorCss,
  PEER_OVERLAY_LABELS,
  type PeerOverlayKind,
  type PeerOverlayLocale,
  type PeerOverlaySpec,
  type PeerView,
  type PresenceDomainInput,
  type PresencePeerInput,
  type PresenceTypingInput,
  type PresenceViewKindInput,
  type PresenceWindowViewInput,
  type UiPeerMark,
} from "./👕️peer-overlay/🟦️.ts";
