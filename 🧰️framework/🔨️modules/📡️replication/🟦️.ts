//! 📡️ Replication contract — TypeScript twin of the Rust `protocol` crate.
//!
//! Byte-for-byte identical to `📦️packages/🦀️rust`'s encoders: the 20 frames in `🧪️tests/🧫️fixtures/📡️wire`
//! are the shared gate both sides must reproduce. Frame layout is `lane u8`, `frame tag u8`, then
//! fields in declaration order — no length prefix, no per-field tags.

/// <reference types="vitest/importMeta" />
export { encodeScopedIdentity, decodeScopedIdentity, SCOPED_IDENTITY_MAX_BYTES, SCOPED_IDENTITY_MAX_SEGMENTS } from "./🧾️wire/🪪️scoped-identity/🟦️.ts";
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
import { blake3Hex } from "@semio-tech/framework-hash";
import { encodePresencePeer, decodePresencePeer } from "./🚪️io/💾️binary/👥️presence/🟦️.ts";
export { encodePresencePeer, PRESENCE_PEER_WIRE_LIMITS_V1, decodePresencePeer, encodePresenceToolRun, decodePresenceToolRun, encodePresenceHistoryEdit, decodePresenceHistoryEdit, readU8, encodePresenceInteraction, decodePresenceInteraction } from "./🚪️io/💾️binary/👥️presence/🟦️.ts";
import { writeVarintU64, writeStr, writeBytes } from "./🚪️io/💾️binary/🟦️.ts";
export { writeVarintU64, readVarintU64, writeVarintU64Exact, readVarintU64Exact, writeStr, readStr, writeBytes, readBytes, writeHash32, readHash32, writeBool, readBool, writeF64, readF64, writeVecBytes, readVecBytes } from "./🚪️io/💾️binary/🟦️.ts";

//#region 🔖️SyncProtocol
export * from "./🔖️sync-protocol/🟦️.ts";
import { HISTORY_TRANSITION_DIFF_SCHEMA, type TransactionRef, type WireMutationEnvelope } from "./🔖️sync-protocol/🟦️.ts";

//#region 🔖️HistoryEditing

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
 * (`🎮️mutation/🧪️tests/🧫️fixtures/🧫️outcome-code`). */
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
