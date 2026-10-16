/** 🪢️ Wire pieces shared by the backbone and the app channel: backbone size ceilings, app command and frame value types, document-archive pack entries and combinators, and command-rejection codes with their acknowledgement decoders.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | os root}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type { MutationMessage, Severity } from "@semio-tech/framework";
import type { LocalInteractionQueryCommand, LocalInteractionQueryReply, MutationEnvelope } from "@semio-tech/framework-replication";
import {
  readBool,
  readBytes,
  readStr,
  readVarintU64,
  writeBool,
  writeBytes,
  writeStr,
  writeVarintU64
} from "@semio-tech/framework-replication";
import { isPackInteger, packUIntSafeOrNull, type PackValue } from "../🎁️packs/🟦️.ts";

//#region 🔖️Backbone
//#region 🔀️BackboneMessage
export const BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES = 262_144;
export const BACKBONE_GENESIS_MAXIMUM_BYTES = 4 * 1024 * 1024;
export const DOCUMENT_ARCHIVE_MAXIMUM_BYTES = BACKBONE_GENESIS_MAXIMUM_BYTES;
//#endregion 🔀️BackboneMessage
//#endregion 🔖️Backbone

//#region 🔖️BackboneWorkerProtocol
/** 🧾️ Every class of refused command batch, one closed set for every producer: the hub (`hub.refused`, or `hub.unreadable`
 * when its diagnostics do not decode) and the local actor's own refusals before any frame leaves it — mirrors Rust
 * `CommandRejectionCode`.
 * @see ./🔨️modules/🏪️store/🔄️sync/🧬️schema/🔣️command-rejection/🔣️.json */
export const COMMAND_REJECTION_CODES_V1 = ["hub.refused", "hub.unreadable", "local.read-only", "local.queue-full", "local.backbone-capacity", "local.backbone-duplicate", "local.backbone-pair-unavailable", "local.backbone-scope-mismatch", "local.backbone-malformed", "local.socket-frame-ceiling"] as const;

/** 🧾️ One {@link COMMAND_REJECTION_CODES_V1} member. */
export type CommandRejectionCodeV1 = (typeof COMMAND_REJECTION_CODES_V1)[number];

/** 🔢️ The typed counters a local refusal reports: the refused batch's envelopes, the bytes it or its queue holds, and the
 * limit it met — mirrors Rust `CommandRejectionDetail`. */
export type CommandRejectionDetailV1 = { readonly envelopes?: number; readonly bytes?: number; readonly limit?: number };

/** 🚫️ One refused command batch: its class, the producer's English diagnostic `reason` (never shown to a human), the
 * structured `MutationMessage`s the hub graded it with (decoded once from the wire, empty for a local refusal) and the local
 * counters — mirrors Rust `CommandAckOutcome::Rejected`. */
export type CommandRejectionV1 = { readonly kind: "rejected"; readonly code: CommandRejectionCodeV1; readonly reason: string; readonly messages: readonly MutationMessage[]; readonly detail?: CommandRejectionDetailV1 };

/** 📮️ The client-side twin of `protocol_wire::ApplyOutcome`, minus the `Transformed` envelope
 * payload (already delivered separately as a `remoteMutations` event by the time this fires) —
 * mirrors Rust `CommandAckOutcome`. */
export type CommandAckOutcome = { readonly kind: "accepted" } | { readonly kind: "transformed" } | CommandRejectionV1;
//#endregion 🔖️BackboneWorkerProtocol

/**
 * 📡️ TS mirror of the `protocol_channel` crate's `AppCommand`/`AppFrame` binary frame protocol
 * (`tag u8 | fields`, built on `protocol_core`'s varint/string/bytes primitives — the same ones
 * {@link encodeClientFrame}/{@link decodeClientFrame} above use). Channel v12
 * (`📓️design-abi.md` §2's handshake-collapse section) retired the `Hello`/`Bye`/`AttachBackbone`/
 * `DetachBackbone`/`RefreshUi` commands and the `Welcome`/`UiSection`/`Effects`/`Events` frames —
 * the reactor ABI wakes guests on events/timers/`next-wake` and carries lifecycle through
 * `Event::InstanceOpen`/`InstanceClose` instead of a wire handshake, and UI updates are now a
 * revisioned `UiPatch` push rather than a cache-probed `RefreshUi`/`UiSection` round trip.
 * `envelopes`/`config`/`command`/`descriptor`/etc. all stay OPAQUE `bytes` here (never a decoded
 * `protocol_causal::MutationEnvelope` or app-specific payload shape), exactly like
 * {@link WireMutationEnvelope}'s `diff`/`inverse` payloads above. `Option<T>` fields use the same
 * `0x00`/`0x01` presence-byte convention as {@link writeOptStr}/{@link writeOptBytes} elsewhere in
 * this file.
 */

//#region 🔖️AppChannelCodec
//#region 🔖️Types
export type ChildPackEntry = { readonly slot: string; readonly child_id: string; readonly dialect: string; readonly envelope_pack: readonly number[]; readonly owner: string };
/** 🪆️ One owned child's CURRENT head snapshot pack — twin of the Rust `ChildHeadPackEntry` (`AppFrame::ChildHeads`). */
export type ChildHeadPackEntry = { readonly slot: string; readonly child_id: string; readonly dialect: string; readonly head_pack: readonly number[]; readonly owner: string };
export type WindowConfigPackEntry = { readonly window_id: string; readonly window_kind_id: string; readonly envelope_pack: readonly number[] };
export type DocumentArchiveArtifactRef = { readonly artifact_id: string; readonly artifact_kind: string; readonly standard: string; readonly subset: string };
export type DocumentArchiveOwnerRef = { readonly parent: DocumentArchiveArtifactRef; readonly slot: string; readonly child_id: string };
export type OwnedDocumentMemberPackEntry = { readonly ordinal: number; readonly reference: DocumentArchiveArtifactRef; readonly owner: DocumentArchiveOwnerRef; readonly envelope_pack: readonly number[] };
export type DocumentArchivePack = { readonly parent_pack: readonly number[]; readonly parent_spr: readonly number[]; readonly members: readonly OwnedDocumentMemberPackEntry[] };
export type DocumentArchiveLoadState = "pending" | "running" | "ready" | "cancelled" | "fault";
export type DocumentArchiveLoadStatus = { readonly operation: number; readonly state: DocumentArchiveLoadState; readonly completed: number; readonly total: number; readonly ahead: number; readonly fault: readonly number[] };

/** 🎞️ Exact authority of one revision-scoped resumable media export. */
export type MediaExportHandle = { readonly app_instance_id: number; readonly parent_document_id: string; readonly operation_id: bigint; readonly base_revision: bigint; readonly generation: bigint };
export type MediaExportState = "running" | "complete" | "cancelled" | "failed";
export type MediaExportStatus = { readonly handle: MediaExportHandle; readonly state: MediaExportState; readonly applied_progress: bigint; readonly checkpoint_available: boolean; readonly mime_type: string; readonly total_bytes: bigint; readonly detail: string };
export type AppDocumentIdentity = { readonly app_instance_id: number; readonly parent_document_id: string | null };

export type AppCommandValue =
  | { readonly ReadDocumentIdentity: { readonly seq: bigint } }
  | { readonly SubmitMediaExport: { readonly seq: bigint; readonly port: string; readonly expected_parent_document_id: string; readonly expected_base_revision: bigint } }
  | { readonly PollMediaExport: { readonly seq: bigint; readonly handle: MediaExportHandle } }
  | { readonly CancelMediaExport: { readonly seq: bigint; readonly handle: MediaExportHandle } }
  | { readonly TakeMediaExportChunk: { readonly seq: bigint; readonly handle: MediaExportHandle } }
  | { readonly LocalInteractionQuery: { readonly seq: number; readonly command: LocalInteractionQueryCommand } }
  | { readonly ConfigCommand: { readonly seq: number; readonly command: readonly number[] } }
  | { readonly Command: { readonly seq: number; readonly command: readonly number[]; readonly view_state: readonly number[] } }
  | { readonly CommandText: { readonly seq: number; readonly line: string } }
  | { readonly ContextMenu: { readonly seq: number; readonly request: readonly number[] } }
  | { readonly ArtifactCommand: { readonly seq: number; readonly command: readonly number[] } }
  | { readonly ApplyEnvelopes: { readonly seq: number; readonly envelopes: readonly MutationEnvelope[] } }
  | { readonly ReadDocument: { readonly seq: number } }
  | { readonly LoadDocumentArchive: { readonly seq: number; readonly archive: DocumentArchivePack } }
  | { readonly MergeDocumentArchive: { readonly seq: number; readonly archive: DocumentArchivePack } }
  | { readonly ReadDocumentArchive: { readonly seq: number } }
  | { readonly PollDocumentArchiveLoad: { readonly seq: number; readonly operation: number } }
  | { readonly CancelDocumentArchiveLoad: { readonly seq: number; readonly operation: number } }
  | { readonly AcknowledgeDocumentArchiveLoad: { readonly seq: number; readonly operation: number } }
  | { readonly LoadConfig: { readonly seq: number; readonly pack: readonly number[]; readonly spr: readonly number[] } }
  | { readonly ReadConfig: { readonly seq: number } }
  | { readonly LoadWindowConfig: { readonly seq: number; readonly entry: WindowConfigPackEntry } }
  | { readonly ReadWindowConfigs: { readonly seq: number } }
  | { readonly MediaIn: { readonly seq: number; readonly port: string; readonly descriptor: readonly number[]; readonly data: readonly number[] } }
  | { readonly MediaOut: { readonly seq: number; readonly port: string; readonly request: readonly number[] } }
  | { readonly MediaFingerprint: { readonly seq: number; readonly port: string } }
  | { readonly PureCommand: { readonly seq: number; readonly command: readonly number[]; readonly head: readonly number[] } }
  | { readonly LoadChildren: { readonly seq: number; readonly entries: readonly ChildPackEntry[] } }
  | { readonly ReadChildren: { readonly seq: number } }
  | { readonly ReadChildHeads: { readonly seq: number } }
  | { readonly ReadHistory: { readonly seq: number } }
  | {
      readonly transactionPrepare: {
        readonly seq: number;
        readonly txn_id: string;
        readonly mutation_id: string;
        readonly payload: readonly number[];
        readonly prepared_ops: readonly (readonly number[])[];
        readonly origin: readonly number[];
        readonly prepared_child_ops: readonly number[];
      };
    }
  | { readonly transactionCommit: { readonly seq: number; readonly txn_id: string } }
  | { readonly transactionRollback: { readonly seq: number; readonly txn_id: string } }
  | { readonly transactionUndo: { readonly seq: number; readonly group_id: string } }
  | { readonly transactionRedo: { readonly seq: number; readonly group_id: string } }
  | { readonly openArtifact: { readonly seq: number; readonly artifact_ref: string; readonly role: number; readonly plugin_id: string; readonly app_id: string } }
  | {
      readonly setDefaultApp: {
        readonly seq: number;
        readonly artifact_kind: string;
        readonly standard: string;
        readonly subset: string;
        readonly role: number;
        readonly plugin_id: string;
        readonly app_id: string;
      };
    }
  | { readonly clearDefaultApp: { readonly seq: number; readonly artifact_kind: string; readonly standard: string; readonly subset: string; readonly role: number } }
  | { readonly setMergePolicy: { readonly seq: number; readonly policy: number } }
  | { readonly resolveConflict: { readonly seq: number; readonly conflict_id: string; readonly resolution: number } }
  | { readonly readConflicts: { readonly seq: number } }
  /** 👥️ Pushes the document-wide presence roster into this app instance — the ONLY plugin ingress
   * for peers (contract-freeze §C7.6). `own_color` is this actor's hub-assigned palette index
   * (`null` for a folder-only session with no hub); `peers` are `encodePresencePeer` blobs, the
   * whole roster with the wrapper's own actor already dropped. CHANNEL_VERSION 12 wire addition. */
  | { readonly presence: { readonly seq: number; readonly own_color: number | null; readonly peers: readonly (readonly number[])[] } };

export type AppFrameValue =
  | { readonly DocumentIdentity: { readonly in_reply_to: bigint; readonly identity: AppDocumentIdentity } }
  | { readonly MediaExportSubmitted: { readonly in_reply_to: bigint; readonly handle: MediaExportHandle } }
  | { readonly MediaExportStatus: MediaExportStatus & { readonly in_reply_to: bigint } }
  | { readonly MediaExportChunk: { readonly in_reply_to: bigint; readonly handle: MediaExportHandle; readonly data: readonly number[]; readonly terminal: boolean } }
  | { readonly LocalInteractionQuery: { readonly reply: LocalInteractionQueryReply } }
  | { readonly Done: { readonly in_reply_to: number } }
  | {
      readonly Invocation: {
        readonly in_reply_to: number;
        readonly output: readonly number[];
        readonly diagnostics: readonly number[];
        readonly ui_scope: readonly number[];
        readonly history_patch: readonly number[];
        readonly messages: readonly number[];
        readonly mutations: readonly number[];
        readonly inverse_group: readonly number[];
      };
    }
  | { readonly DocumentChanged: { readonly envelopes: readonly (readonly number[])[]; readonly origin: string } }
  | { readonly Document: { readonly in_reply_to: number; readonly pack: readonly number[]; readonly spr: readonly number[]; readonly ops: string } }
  | { readonly DocumentArchive: { readonly in_reply_to: number; readonly archive: DocumentArchivePack } }
  | { readonly DocumentArchiveLoad: { readonly in_reply_to: number; readonly status: DocumentArchiveLoadStatus } }
  | { readonly Config: { readonly in_reply_to: number; readonly pack: readonly number[]; readonly spr: readonly number[]; readonly ops: string } }
  | { readonly WindowConfigs: { readonly in_reply_to: number; readonly entries: readonly WindowConfigPackEntry[] } }
  | { readonly ConfigChanged: { readonly envelopes: readonly (readonly number[])[]; readonly origin: string } }
  | { readonly ContextMenu: { readonly in_reply_to: number; readonly items: readonly number[] } }
  | { readonly Media: { readonly in_reply_to: number; readonly port: string; readonly descriptor: readonly number[]; readonly data: readonly number[] } }
  | { readonly MediaFingerprint: { readonly in_reply_to: number; readonly port: string; readonly fingerprint: readonly number[] } }
  | { readonly Error: { readonly in_reply_to: number | null; readonly fault: readonly number[]; readonly report: readonly number[] } }
  | { readonly Emit: { readonly in_reply_to: number; readonly document_ops: readonly number[]; readonly config_ops: readonly number[]; readonly draft_ops: readonly number[]; readonly output: readonly number[]; readonly diagnostics: readonly number[]; readonly child_ops: readonly number[] } }
  | { readonly Draft: { readonly in_reply_to: number; readonly pack: readonly number[]; readonly spr: readonly number[]; readonly ops: string } }
  | { readonly Children: { readonly in_reply_to: number; readonly entries: readonly ChildPackEntry[] } }
  | { readonly ChildHeads: { readonly in_reply_to: number; readonly entries: readonly ChildHeadPackEntry[] } }
  | { readonly Ephemeral: { readonly presence: readonly number[]; readonly presence_generation: number; readonly transient_generation: number; readonly interaction: readonly number[]; readonly tool_run: readonly number[]; readonly history_edit: readonly number[] } }
  | { readonly HistorySnapshot: { readonly in_reply_to: number; readonly history_patch: readonly number[] } }
  | {
      readonly transactionProposal: {
        readonly in_reply_to: number;
        readonly proposal_id: string;
        readonly local_ops: readonly (readonly number[])[];
        readonly foreign: readonly (readonly number[])[];
      };
    }
  | { readonly transactionPrepared: { readonly txn_id: string; readonly foreign: readonly (readonly number[])[]; readonly rejection: readonly number[] } }
  | { readonly transactionCommitted: { readonly txn_id: string; readonly edit_id: string } }
  | { readonly transactionRolledBack: { readonly txn_id: string } }
  | { readonly MergeReport: { readonly in_reply_to: number | null; readonly report: readonly number[] } }
  | { readonly Conflicts: { readonly in_reply_to: number | null; readonly conflicts: readonly number[] } }
  /** 🎨️ Revisioned UI patch batch for one surface — replaces `UiSection`'s cache-probe push.
   * `ops` is `store::pack_rt::encode_wire_value`-encoded `Vec<kernel::PatchOp>` (reused from
   * `semio_framework::kernel`, never redefined here). CHANNEL_VERSION 12 wire addition. */
  | { readonly UiPatch: { readonly in_reply_to: number | null; readonly surface: string; readonly kind: string; readonly revision: number; readonly base_revision: number; readonly ops: readonly number[] } }
  /** 🏁️ Marks the end of one surface's initial full-body snapshot burst. CHANNEL_VERSION 12 wire addition. */
  | { readonly UiSnapshotEnd: { readonly revision: number } }
  /** 🏁️ One mounted typed operation reached its terminal result — unsolicited, correlated by
   * `operation` (the host-allocated operation id) rather than by an `in_reply_to` sequence, because
   * the command that started it returned its "started" `Invocation` long before the retained job
   * finished. `ui_scope`/`history_patch` are `store::pack_rt::encode_wire_value`-encoded exactly like
   * `Invocation`'s same-named fields. CHANNEL_VERSION 15 wire addition. */
  | { readonly OperationCompleted: { readonly operation: bigint; readonly revision: bigint; readonly ui_scope: readonly number[]; readonly history_patch: readonly number[] } };

/** 🏁️ One typed operation's terminal completion, decoded from `AppFrame::OperationCompleted`.
 * `uiScope` is the operation's final `kernel::UiDirtyScope` and `historyPatch` its command-log delta,
 * both already projected onto exact JSON through {@link decodePackWire} (so `entry.seq` is a number,
 * never a `{kind, value}` integer carrier); `undefined` when the host published neither. */
export type OperationCompletionV1 = Readonly<{
  instanceId: number;
  operation: bigint;
  revision: bigint;
  uiScope: unknown;
  historyPatch: unknown;
}>;
//#endregion 🔖️Types

//#region 🔖️Combinators
/** 🎞️ `presence u8 | varint` — an `Option<u64>` (e.g. `AppFrame.*.in_reply_to`), the same
 * convention {@link writeOptStr}/{@link writeOptBytes} use. */
export function writeOptU64(out: number[], value: number | null): void {
  writeBool(out, value !== null);
  if (value !== null) writeVarintU64(out, value);
}
export function readOptU64(bytes: Uint8Array, pos: [number]): number | null {
  return readBool(bytes, pos) ? readVarintU64(bytes, pos) : null;
}
/** 🎞️ `presence u8 | byte` — an `Option<u8>` (`AppCommand.presence.own_color`), the same
 * presence-byte convention as {@link writeOptU64} above. */
export function writeOptU8(out: number[], value: number | null): void {
  writeBool(out, value !== null);
  if (value !== null) out.push(value);
}
export function readOptU8(bytes: Uint8Array, pos: [number]): number | null {
  if (!readBool(bytes, pos)) return null;
  const byte = bytes[pos[0]]!;
  pos[0] += 1;
  return byte;
}
function writeChildPackEntry(out: number[], entry: ChildPackEntry): void {
  writeStr(out, entry.slot);
  writeStr(out, entry.child_id);
  writeStr(out, entry.dialect);
  writeBytes(out, entry.envelope_pack);
  writeStr(out, entry.owner);
}
function readChildPackEntry(bytes: Uint8Array, pos: [number]): ChildPackEntry {
  return { slot: readStr(bytes, pos), child_id: readStr(bytes, pos), dialect: readStr(bytes, pos), envelope_pack: readBytes(bytes, pos), owner: readStr(bytes, pos) };
}
export function writeVecChildPackEntry(out: number[], entries: readonly ChildPackEntry[]): void {
  writeVarintU64(out, entries.length);
  for (const entry of entries) writeChildPackEntry(out, entry);
}
export function readVecChildPackEntry(bytes: Uint8Array, pos: [number]): ChildPackEntry[] {
  const count = readVarintU64(bytes, pos);
  return Array.from({ length: count }, () => readChildPackEntry(bytes, pos));
}
export function writeVecChildHeadPackEntry(out: number[], entries: readonly ChildHeadPackEntry[]): void {
  writeVarintU64(out, entries.length);
  for (const entry of entries) {
    writeStr(out, entry.slot);
    writeStr(out, entry.child_id);
    writeStr(out, entry.dialect);
    writeBytes(out, entry.head_pack);
    writeStr(out, entry.owner);
  }
}
export function readVecChildHeadPackEntry(bytes: Uint8Array, pos: [number]): ChildHeadPackEntry[] {
  const count = readVarintU64(bytes, pos);
  if (count > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS) throw new Error("decodeAppFrame: child head count exceeds the member authority");
  return Array.from({ length: count }, () => ({ slot: readStr(bytes, pos), child_id: readStr(bytes, pos), dialect: readStr(bytes, pos), head_pack: readBytes(bytes, pos), owner: readStr(bytes, pos) }));
}
export const DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS = 1_024;
export function writeDocumentArchive(out: number[], archive: DocumentArchivePack): void {
  if (archive.members.length > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS) throw new Error("document archive exceeds its fixed 1024-member authority");
  writeBytes(out, archive.parent_pack);
  writeBytes(out, archive.parent_spr);
  writeVarintU64(out, archive.members.length);
  for (const entry of archive.members) {
    writeVarintU64(out, entry.ordinal);
    writeStr(out, entry.reference.artifact_id);
    writeStr(out, entry.reference.artifact_kind);
    writeStr(out, entry.reference.standard);
    writeStr(out, entry.reference.subset);
    writeStr(out, entry.owner.parent.artifact_id);
    writeStr(out, entry.owner.parent.artifact_kind);
    writeStr(out, entry.owner.parent.standard);
    writeStr(out, entry.owner.parent.subset);
    writeStr(out, entry.owner.slot);
    writeStr(out, entry.owner.child_id);
    writeBytes(out, entry.envelope_pack);
  }
}
export function readDocumentArchive(bytes: Uint8Array, pos: [number]): DocumentArchivePack {
  const parent_pack = readBytes(bytes, pos);
  const parent_spr = readBytes(bytes, pos);
  const count = readVarintU64(bytes, pos);
  if (count > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS) throw new Error("document archive exceeds its fixed 1024-member authority");
  const members = Array.from({ length: count }, () => {
    const ordinal = readVarintU64(bytes, pos);
    const reference = { artifact_id: readStr(bytes, pos), artifact_kind: readStr(bytes, pos), standard: readStr(bytes, pos), subset: readStr(bytes, pos) };
    const parent = { artifact_id: readStr(bytes, pos), artifact_kind: readStr(bytes, pos), standard: readStr(bytes, pos), subset: readStr(bytes, pos) };
    const owner = { parent, slot: readStr(bytes, pos), child_id: readStr(bytes, pos) };
    return { ordinal, reference, owner, envelope_pack: readBytes(bytes, pos) };
  });
  return { parent_pack, parent_spr, members };
}

/** 🗃️ Canonical persisted recursive-document archive bytes: version byte plus the channel archive body. */
export function encodeDocumentArchiveBytes(archive: DocumentArchivePack): Uint8Array {
  const out = [1];
  writeDocumentArchive(out, archive);
  if (out.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES) throw new Error("document archive exceeds its fixed byte authority");
  return Uint8Array.from(out);
}

/** 🗃️ Decodes one exact persisted recursive-document archive and rejects trailing ownership. */
export function decodeDocumentArchiveBytes(bytes: Uint8Array): DocumentArchivePack {
  if (bytes.length === 0 || bytes.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES) throw new Error("document archive exceeds its fixed byte authority");
  if (bytes[0] !== 1) throw new Error("document archive: unsupported or missing version");
  const pos: [number] = [1];
  const archive = readDocumentArchive(bytes, pos);
  if (pos[0] !== bytes.length) throw new Error("document archive: trailing bytes");
  return archive;
}
export function writeWindowConfigPackEntry(out: number[], entry: WindowConfigPackEntry): void {
  writeStr(out, entry.window_id);
  writeStr(out, entry.window_kind_id);
  writeBytes(out, entry.envelope_pack);
}
export function readWindowConfigPackEntry(bytes: Uint8Array, pos: [number]): WindowConfigPackEntry {
  return { window_id: readStr(bytes, pos), window_kind_id: readStr(bytes, pos), envelope_pack: readBytes(bytes, pos) };
}
export function writeVecWindowConfigPackEntry(out: number[], entries: readonly WindowConfigPackEntry[]): void {
  writeVarintU64(out, entries.length);
  for (const entry of entries) writeWindowConfigPackEntry(out, entry);
}
export function readVecWindowConfigPackEntry(bytes: Uint8Array, pos: [number]): WindowConfigPackEntry[] {
  const count = readVarintU64(bytes, pos);
  return Array.from({ length: count }, () => readWindowConfigPackEntry(bytes, pos));
}
//#endregion 🔖️Combinators
//#endregion 🔖️AppChannelCodec

//#region 🔖️Directory
//#region 🔖️HubBinding
//#region 🚫️CommandRejection
const MUTATION_MESSAGE_LEVELS: ReadonlySet<unknown> = new Set(["info", "warning", "error", "fatal"]);

/** 🧾️ One `MutationMessage` read like Rust's hand-written `FromValue` twin (`📡️replication/🎮️mutation/🦀️.rs`): `level`,
 * `code` and `message` required, `target` and `opIndex` optional (an empty `target` and a `null` `opIndex` read as absent),
 * any other field ignored; `null` when a field has the wrong type. Integers are read from a JSON number or a pack carrier. */
function mutationMessageOfValueV1(value: unknown): MutationMessage | null {
  if (typeof value !== "object" || value === null || Array.isArray(value) || isPackInteger(value)) return null;
  const { level, code, message, target, opIndex } = value as Record<string, PackValue | undefined>;
  if (!MUTATION_MESSAGE_LEVELS.has(level) || typeof code !== "string" || typeof message !== "string") return null;
  if (target !== undefined && (!Array.isArray(target) || !target.every((segment) => typeof segment === "string"))) return null;
  const index = opIndex === undefined || opIndex === null ? null : packUIntSafeOrNull(opIndex);
  if (opIndex !== undefined && opIndex !== null && (index === null || index > 0xffff_ffff)) return null;
  return { level: level as Severity, code, message, ...(target === undefined || target.length === 0 ? {} : { target: [...(target as readonly string[])] }), ...(index === null ? {} : { opIndex: index }) };
}

/** 🧾️ The hub's `ApplyOutcome::Rejected.messages` bytes — `🌎️hub` `encode_messages`: the UTF-8 JSON of one
 * `MutationMessage` array, empty for an admission or transport refusal — decoded once; `null` when they are not that array. */
export function decodeHubRejectionMessagesV1(bytes: ArrayLike<number>): readonly MutationMessage[] | null {
  if (bytes.length === 0) return [];
  let parsed: unknown;
  try {
    parsed = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(Uint8Array.from(bytes)));
  } catch {
    return null;
  }
  if (!Array.isArray(parsed)) return null;
  const messages = parsed.map(mutationMessageOfValueV1);
  return messages.every((message) => message !== null) ? (messages as MutationMessage[]) : null;
}

/** 🚫️ The hub's refusal in the one rejection contract: `hub.refused` with the messages it graded the batch with, or
 * `hub.unreadable` when they do not decode — never a throw, whatever the wire carried. */
export function hubCommandRejectionV1(rejected: { readonly reason: string; readonly messages: ArrayLike<number> }): CommandRejectionV1 {
  const messages = decodeHubRejectionMessagesV1(rejected.messages);
  return { kind: "rejected", code: messages === null ? "hub.unreadable" : "hub.refused", reason: rejected.reason, messages: messages ?? [] };
}

/** 🚫️ A refusal the local actor answers before any frame leaves it: no hub messages, only its typed counters. */
export function localCommandRejectionV1(code: Exclude<CommandRejectionCodeV1, `hub.${string}`>, reason: string, detail?: CommandRejectionDetailV1): CommandRejectionV1 {
  return { kind: "rejected", code, reason, messages: [], ...(detail === undefined ? {} : { detail }) };
}

const COMMAND_REJECTION_CODE_SET: ReadonlySet<unknown> = new Set(COMMAND_REJECTION_CODES_V1);
const COMMAND_REJECTION_DETAIL_FIELDS = ["envelopes", "bytes", "limit"] as const;

/** 🔎️ A `commandOutcome`'s outcome as either actor wires it (Rust `CommandAckOutcome`'s `ToValue`, or this module's own
 * shape through a structured clone), read into the contract; `null` when it is not one. */
export function commandAckOutcomeOfValueV1(value: unknown): CommandAckOutcome | null {
  if (typeof value !== "object" || value === null || Array.isArray(value) || isPackInteger(value)) return null;
  const row = value as Record<string, PackValue | undefined>;
  if (row.kind === "accepted" || row.kind === "transformed") return Object.keys(row).length === 1 ? { kind: row.kind } : null;
  if (row.kind !== "rejected" || !COMMAND_REJECTION_CODE_SET.has(row.code) || typeof row.reason !== "string" || !Array.isArray(row.messages)) return null;
  if (Object.keys(row).some((key) => key !== "kind" && key !== "code" && key !== "reason" && key !== "messages" && key !== "detail")) return null;
  const messages = row.messages.map(mutationMessageOfValueV1);
  if (!messages.every((message) => message !== null)) return null;
  const rejection = { kind: "rejected", code: row.code as CommandRejectionCodeV1, reason: row.reason, messages: messages as MutationMessage[] } as const;
  if (row.detail === undefined || row.detail === null) return rejection;
  if (typeof row.detail !== "object" || Array.isArray(row.detail) || isPackInteger(row.detail)) return null;
  const detailRow = row.detail as Record<string, PackValue | undefined>;
  if (Object.keys(detailRow).some((key) => !(COMMAND_REJECTION_DETAIL_FIELDS as readonly string[]).includes(key))) return null;
  const detail: { envelopes?: number; bytes?: number; limit?: number } = {};
  for (const field of COMMAND_REJECTION_DETAIL_FIELDS) {
    const entry = detailRow[field];
    if (entry === undefined || entry === null) continue;
    const count = packUIntSafeOrNull(entry);
    if (count === null) return null;
    detail[field] = count;
  }
  return { ...rejection, detail };
}

/** 🔢️ A `commandOutcome` batch id: the hub's ids count up from zero, both actors' local refusals count down from -1 — the
 * TypeScript actor as a negative number, the Rust actor as its two's-complement `u64` (`u64::MAX` is -1). */
export function commandBatchIdOfValueV1(value: unknown): number | null {
  if (typeof value === "number") return Number.isSafeInteger(value) ? value : null;
  if (!isPackInteger(value)) return null;
  const signed = value.kind === "uint" ? BigInt.asIntN(64, value.value) : value.value;
  return signed >= BigInt(Number.MIN_SAFE_INTEGER) && signed <= BigInt(Number.MAX_SAFE_INTEGER) ? Number(signed) : null;
}
//#endregion 🚫️CommandRejection
//#endregion 🔖️HubBinding
//#endregion 🔖️Directory
