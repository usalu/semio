/** 🧾️ The app channel: command and frame value types, their codecs, channel version admission, the request sequence, the document-archive load host and the app channel client.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | os root}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type {
  Conflict,
  ConflictResolution,
  DispatchReport,
  Fault,
  MergePolicy,
  MergeReport,
  MutationMessage,
  PluginDispatchHintV1,
  PluginWasmHandle,
  TurnOutcome
} from "@semio-tech/framework";
import { conflictResolutionAsU8, mergePolicyAsU8 } from "@semio-tech/framework";
import type {
  ArtifactPresencePeer,
  LocalInteractionIdentity,
  LocalInteractionPage,
  LocalInteractionQueryCommand,
  LocalInteractionQueryReply,
  LocalInteractionQueryToken,
  MutationEnvelope
} from "@semio-tech/framework-replication";
import {
  decodeLocalInteractionQueryCommand,
  decodeLocalInteractionQueryReply,
  encodeLocalInteractionQueryCommand,
  encodeLocalInteractionQueryReply,
  encodePresencePeer,
  localInteractionIdentityEquals,
  mutationEnvelopeFromWire,
  mutationEnvelopeToWire
} from "@semio-tech/framework-replication";
import {
  readBool,
  readBytes,
  readStr,
  readU8,
  readVarintU64,
  readVarintU64Exact,
  readVecBytes,
  readVecEnvelope,
  writeBool,
  writeBytes,
  writeStr,
  writeVarintU64,
  writeVarintU64Exact,
  writeVecBytes,
  writeVecEnvelope
} from "@semio-tech/framework-replication";
import {
  decodePackValue,
  decodePackWire,
  encodePackValue,
  INVOCATION_RESULT_PACK_MAXIMUM_BYTES,
  replicationPackCodec,
  viewContextWireValue
} from "../🎁️packs/🟦️.ts";
import {
  type AppCommandValue,
  type AppDocumentIdentity,
  type AppFrameValue,
  type ChildHeadPackEntry,
  type DocumentArchiveLoadStatus,
  type DocumentArchivePack,
  type MediaExportHandle,
  type OperationCompletionV1,
  readDocumentArchive,
  readOptU64,
  readOptU8,
  readVecChildHeadPackEntry,
  readVecChildPackEntry,
  readVecWindowConfigPackEntry,
  readWindowConfigPackEntry,
  type WindowConfigPackEntry,
  writeDocumentArchive,
  writeOptU64,
  writeOptU8,
  writeVecChildHeadPackEntry,
  writeVecChildPackEntry,
  writeVecWindowConfigPackEntry,
  writeWindowConfigPackEntry
} from "../🪢️wire-contracts/🟦️.ts";

//#region 🔖️AppChannelCodec
//#region 🔖️Codec
/** 🪪️ Serializes media ownership without rounding its u64 revision lanes. */
function writeMediaExportHandle(out: number[], handle: MediaExportHandle): void {
  if (!Number.isInteger(handle.app_instance_id) || handle.app_instance_id < 0 || handle.app_instance_id > 0xffffffff) throw new Error("media export: invalid app instance");
  writeVarintU64(out, handle.app_instance_id);
  writeStr(out, handle.parent_document_id);
  writeVarintU64Exact(out, handle.operation_id);
  writeVarintU64Exact(out, handle.base_revision);
  writeVarintU64Exact(out, handle.generation);
}

/** 🎫️ Reads the exact media authority shared with the Rust channel. */
function readMediaExportHandle(bytes: Uint8Array, pos: [number]): MediaExportHandle {
  const app_instance_id = readVarintU64(bytes, pos);
  if (app_instance_id > 0xffffffff) throw new Error("media export: invalid app instance");
  return { app_instance_id, parent_document_id: readStr(bytes, pos), operation_id: readVarintU64Exact(bytes, pos), base_revision: readVarintU64Exact(bytes, pos), generation: readVarintU64Exact(bytes, pos) };
}

/** 📏️ Media messages consume their complete declared frame. */
function exactMediaExportFrame<T>(bytes: Uint8Array, pos: [number], value: T): T {
  if (pos[0] !== bytes.length) throw new Error("media export: trailing frame bytes");
  return value;
}

const MEDIA_EXPORT_STATES = ["running", "complete", "cancelled", "failed"] as const;

function assertAppDocumentIdentity(identity: AppDocumentIdentity): void {
  if (!Number.isSafeInteger(identity.app_instance_id) || identity.app_instance_id < 0 || identity.app_instance_id > 0xffff_ffff || (identity.parent_document_id !== null && (typeof identity.parent_document_id !== "string" || identity.parent_document_id.length === 0 || [...identity.parent_document_id].length > 512))) throw new Error("document identity: invalid owner");
}

const APP_COMMAND_TAGS = {
  ConfigCommand: 0, Command: 1, CommandText: 2, ContextMenu: 3, ArtifactCommand: 4, ApplyEnvelopes: 5,
  ReadDocument: 7, LoadConfig: 8, ReadConfig: 9, MediaIn: 10, MediaOut: 11,
  MediaFingerprint: 12, PureCommand: 13, LoadChildren: 14, ReadChildren: 15, ReadHistory: 16,
  transactionPrepare: 17, transactionCommit: 18, transactionRollback: 19, transactionUndo: 20, transactionRedo: 21,
  openArtifact: 22, setDefaultApp: 23, clearDefaultApp: 24,
  setMergePolicy: 25, resolveConflict: 26, readConflicts: 27,
  presence: 28, LocalInteractionQuery: 29, LoadWindowConfig: 30, ReadWindowConfigs: 31, LoadDocumentArchive: 32, ReadDocumentArchive: 33,
  PollDocumentArchiveLoad: 34, CancelDocumentArchiveLoad: 35, AcknowledgeDocumentArchiveLoad: 36,
  SubmitMediaExport: 37, PollMediaExport: 38, CancelMediaExport: 39, TakeMediaExportChunk: 40, ReadDocumentIdentity: 41, ReadChildHeads: 42, MergeDocumentArchive: 43,
} as const;
const APP_FRAME_TAGS = {
  Done: 0, Invocation: 1, DocumentChanged: 2, Document: 3,
  Config: 4, ConfigChanged: 5, ContextMenu: 6, Media: 7, MediaFingerprint: 8, Error: 9, Emit: 10, Draft: 11, Children: 12, Ephemeral: 13, HistorySnapshot: 14,
  transactionProposal: 15, transactionPrepared: 16, transactionCommitted: 17, transactionRolledBack: 18,
  MergeReport: 19, Conflicts: 20, UiPatch: 21, UiSnapshotEnd: 22, LocalInteractionQuery: 23, WindowConfigs: 24, OperationCompleted: 25, DocumentArchive: 26, DocumentArchiveLoad: 27,
  MediaExportSubmitted: 28, MediaExportStatus: 29, MediaExportChunk: 30, DocumentIdentity: 31, ChildHeads: 32,
} as const;

/** 📤️ `tag u8 | fields` — the TS twin of `protocol_channel::encode_app_command` (agreed contract). */
export function encodeAppCommand(cmd: AppCommandValue): Uint8Array {
  const out: number[] = [];
  if ("ReadDocumentIdentity" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadDocumentIdentity);
    writeVarintU64Exact(out, cmd.ReadDocumentIdentity.seq);
  } else if ("SubmitMediaExport" in cmd) {
    out.push(APP_COMMAND_TAGS.SubmitMediaExport);
    writeVarintU64Exact(out, cmd.SubmitMediaExport.seq);
    writeStr(out, cmd.SubmitMediaExport.port);
    writeStr(out, cmd.SubmitMediaExport.expected_parent_document_id);
    writeVarintU64Exact(out, cmd.SubmitMediaExport.expected_base_revision);
  } else if ("PollMediaExport" in cmd || "CancelMediaExport" in cmd || "TakeMediaExportChunk" in cmd) {
    const key = "PollMediaExport" in cmd ? "PollMediaExport" : "CancelMediaExport" in cmd ? "CancelMediaExport" : "TakeMediaExportChunk";
    const value = (cmd as Record<typeof key, { readonly seq: bigint; readonly handle: MediaExportHandle }>)[key];
    out.push(APP_COMMAND_TAGS[key]);
    writeVarintU64Exact(out, value.seq);
    writeMediaExportHandle(out, value.handle);
  } else if ("ConfigCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.ConfigCommand);
    writeVarintU64(out, cmd.ConfigCommand.seq);
    writeBytes(out, cmd.ConfigCommand.command);
  } else if ("Command" in cmd) {
    out.push(APP_COMMAND_TAGS.Command);
    writeVarintU64(out, cmd.Command.seq);
    writeBytes(out, cmd.Command.command);
    writeBytes(out, cmd.Command.view_state);
  } else if ("CommandText" in cmd) {
    out.push(APP_COMMAND_TAGS.CommandText);
    writeVarintU64(out, cmd.CommandText.seq);
    writeStr(out, cmd.CommandText.line);
  } else if ("ContextMenu" in cmd) {
    out.push(APP_COMMAND_TAGS.ContextMenu);
    writeVarintU64(out, cmd.ContextMenu.seq);
    writeBytes(out, cmd.ContextMenu.request);
  } else if ("ArtifactCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.ArtifactCommand);
    writeVarintU64(out, cmd.ArtifactCommand.seq);
    writeBytes(out, cmd.ArtifactCommand.command);
  } else if ("ApplyEnvelopes" in cmd) {
    out.push(APP_COMMAND_TAGS.ApplyEnvelopes);
    writeVarintU64(out, cmd.ApplyEnvelopes.seq);
    writeVecEnvelope(
      out,
      cmd.ApplyEnvelopes.envelopes.map((envelope, index) => mutationEnvelopeToWire(envelope, { actor: 0, physical_ms: 0, logical: index + 1 }, replicationPackCodec)),
    );
  } else if ("ReadDocument" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadDocument);
    writeVarintU64(out, cmd.ReadDocument.seq);
  } else if ("LoadConfig" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadConfig);
    writeVarintU64(out, cmd.LoadConfig.seq);
    writeBytes(out, cmd.LoadConfig.pack);
    writeBytes(out, cmd.LoadConfig.spr);
  } else if ("ReadConfig" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadConfig);
    writeVarintU64(out, cmd.ReadConfig.seq);
  } else if ("LoadWindowConfig" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadWindowConfig);
    writeVarintU64(out, cmd.LoadWindowConfig.seq);
    writeWindowConfigPackEntry(out, cmd.LoadWindowConfig.entry);
  } else if ("ReadWindowConfigs" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadWindowConfigs);
    writeVarintU64(out, cmd.ReadWindowConfigs.seq);
  } else if ("LoadDocumentArchive" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadDocumentArchive);
    writeVarintU64(out, cmd.LoadDocumentArchive.seq);
    writeDocumentArchive(out, cmd.LoadDocumentArchive.archive);
  } else if ("MergeDocumentArchive" in cmd) {
    out.push(APP_COMMAND_TAGS.MergeDocumentArchive);
    writeVarintU64(out, cmd.MergeDocumentArchive.seq);
    writeDocumentArchive(out, cmd.MergeDocumentArchive.archive);
  } else if ("ReadDocumentArchive" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadDocumentArchive);
    writeVarintU64(out, cmd.ReadDocumentArchive.seq);
  } else if ("PollDocumentArchiveLoad" in cmd) {
    out.push(APP_COMMAND_TAGS.PollDocumentArchiveLoad);
    writeVarintU64(out, cmd.PollDocumentArchiveLoad.seq);
    writeVarintU64(out, cmd.PollDocumentArchiveLoad.operation);
  } else if ("CancelDocumentArchiveLoad" in cmd) {
    out.push(APP_COMMAND_TAGS.CancelDocumentArchiveLoad);
    writeVarintU64(out, cmd.CancelDocumentArchiveLoad.seq);
    writeVarintU64(out, cmd.CancelDocumentArchiveLoad.operation);
  } else if ("AcknowledgeDocumentArchiveLoad" in cmd) {
    out.push(APP_COMMAND_TAGS.AcknowledgeDocumentArchiveLoad);
    writeVarintU64(out, cmd.AcknowledgeDocumentArchiveLoad.seq);
    writeVarintU64(out, cmd.AcknowledgeDocumentArchiveLoad.operation);
  } else if ("MediaIn" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaIn);
    writeVarintU64(out, cmd.MediaIn.seq);
    writeStr(out, cmd.MediaIn.port);
    writeBytes(out, cmd.MediaIn.descriptor);
    writeBytes(out, cmd.MediaIn.data);
  } else if ("MediaOut" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaOut);
    writeVarintU64(out, cmd.MediaOut.seq);
    writeStr(out, cmd.MediaOut.port);
    writeBytes(out, cmd.MediaOut.request);
  } else if ("MediaFingerprint" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaFingerprint);
    writeVarintU64(out, cmd.MediaFingerprint.seq);
    writeStr(out, cmd.MediaFingerprint.port);
  } else if ("PureCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.PureCommand);
    writeVarintU64(out, cmd.PureCommand.seq);
    writeBytes(out, cmd.PureCommand.command);
    writeBytes(out, cmd.PureCommand.head);
  } else if ("LoadChildren" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadChildren);
    writeVarintU64(out, cmd.LoadChildren.seq);
    writeVecChildPackEntry(out, cmd.LoadChildren.entries);
  } else if ("ReadChildren" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadChildren);
    writeVarintU64(out, cmd.ReadChildren.seq);
  } else if ("ReadChildHeads" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadChildHeads);
    writeVarintU64(out, cmd.ReadChildHeads.seq);
  } else if ("ReadHistory" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadHistory);
    writeVarintU64(out, cmd.ReadHistory.seq);
  } else if ("transactionPrepare" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionPrepare);
    writeVarintU64(out, cmd.transactionPrepare.seq);
    writeStr(out, cmd.transactionPrepare.txn_id);
    writeStr(out, cmd.transactionPrepare.mutation_id);
    writeBytes(out, cmd.transactionPrepare.payload);
    writeVecBytes(out, cmd.transactionPrepare.prepared_ops);
    writeBytes(out, cmd.transactionPrepare.origin);
    writeBytes(out, cmd.transactionPrepare.prepared_child_ops);
  } else if ("transactionCommit" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionCommit);
    writeVarintU64(out, cmd.transactionCommit.seq);
    writeStr(out, cmd.transactionCommit.txn_id);
  } else if ("transactionRollback" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionRollback);
    writeVarintU64(out, cmd.transactionRollback.seq);
    writeStr(out, cmd.transactionRollback.txn_id);
  } else if ("transactionUndo" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionUndo);
    writeVarintU64(out, cmd.transactionUndo.seq);
    writeStr(out, cmd.transactionUndo.group_id);
  } else if ("transactionRedo" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionRedo);
    writeVarintU64(out, cmd.transactionRedo.seq);
    writeStr(out, cmd.transactionRedo.group_id);
  } else if ("openArtifact" in cmd) {
    out.push(APP_COMMAND_TAGS.openArtifact);
    writeVarintU64(out, cmd.openArtifact.seq);
    writeStr(out, cmd.openArtifact.artifact_ref);
    out.push(cmd.openArtifact.role);
    writeStr(out, cmd.openArtifact.plugin_id);
    writeStr(out, cmd.openArtifact.app_id);
  } else if ("setDefaultApp" in cmd) {
    out.push(APP_COMMAND_TAGS.setDefaultApp);
    writeVarintU64(out, cmd.setDefaultApp.seq);
    writeStr(out, cmd.setDefaultApp.artifact_kind);
    writeStr(out, cmd.setDefaultApp.standard);
    writeStr(out, cmd.setDefaultApp.subset);
    out.push(cmd.setDefaultApp.role);
    writeStr(out, cmd.setDefaultApp.plugin_id);
    writeStr(out, cmd.setDefaultApp.app_id);
  } else if ("clearDefaultApp" in cmd) {
    out.push(APP_COMMAND_TAGS.clearDefaultApp);
    writeVarintU64(out, cmd.clearDefaultApp.seq);
    writeStr(out, cmd.clearDefaultApp.artifact_kind);
    writeStr(out, cmd.clearDefaultApp.standard);
    writeStr(out, cmd.clearDefaultApp.subset);
    out.push(cmd.clearDefaultApp.role);
  } else if ("setMergePolicy" in cmd) {
    out.push(APP_COMMAND_TAGS.setMergePolicy);
    writeVarintU64(out, cmd.setMergePolicy.seq);
    out.push(cmd.setMergePolicy.policy);
  } else if ("resolveConflict" in cmd) {
    out.push(APP_COMMAND_TAGS.resolveConflict);
    writeVarintU64(out, cmd.resolveConflict.seq);
    writeStr(out, cmd.resolveConflict.conflict_id);
    out.push(cmd.resolveConflict.resolution);
  } else if ("readConflicts" in cmd) {
    out.push(APP_COMMAND_TAGS.readConflicts);
    writeVarintU64(out, cmd.readConflicts.seq);
  } else if ("LocalInteractionQuery" in cmd) {
    out.push(APP_COMMAND_TAGS.LocalInteractionQuery);
    writeVarintU64(out, cmd.LocalInteractionQuery.seq);
    writeBytes(out, Array.from(encodeLocalInteractionQueryCommand(cmd.LocalInteractionQuery.command)));
  } else if ("presence" in cmd) {
    out.push(APP_COMMAND_TAGS.presence);
    writeVarintU64(out, cmd.presence.seq);
    writeOptU8(out, cmd.presence.own_color);
    writeVecBytes(out, cmd.presence.peers);
  } else {
    throw new Error("encodeAppCommand: unrecognized command variant");
  }
  return new Uint8Array(out);
}

/** 📥️ Inverse of {@link encodeAppCommand} — the TS twin of `protocol_channel::decode_app_command`. */
export function decodeAppCommand(bytes: Uint8Array): AppCommandValue {
  if (bytes.length === 0) throw new Error("decodeAppCommand: empty frame");
  const pos: [number] = [1];
  switch (bytes[0]) {
    case APP_COMMAND_TAGS.ReadDocumentIdentity: {
      const seq = readVarintU64Exact(bytes, pos);
      if (pos[0] !== bytes.length) throw new Error("document identity: trailing command bytes");
      return { ReadDocumentIdentity: { seq } };
    }
    case APP_COMMAND_TAGS.SubmitMediaExport:
      return exactMediaExportFrame(bytes, pos, { SubmitMediaExport: { seq: readVarintU64Exact(bytes, pos), port: readStr(bytes, pos), expected_parent_document_id: readStr(bytes, pos), expected_base_revision: readVarintU64Exact(bytes, pos) } });
    case APP_COMMAND_TAGS.PollMediaExport:
      return exactMediaExportFrame(bytes, pos, { PollMediaExport: { seq: readVarintU64Exact(bytes, pos), handle: readMediaExportHandle(bytes, pos) } });
    case APP_COMMAND_TAGS.CancelMediaExport:
      return exactMediaExportFrame(bytes, pos, { CancelMediaExport: { seq: readVarintU64Exact(bytes, pos), handle: readMediaExportHandle(bytes, pos) } });
    case APP_COMMAND_TAGS.TakeMediaExportChunk:
      return exactMediaExportFrame(bytes, pos, { TakeMediaExportChunk: { seq: readVarintU64Exact(bytes, pos), handle: readMediaExportHandle(bytes, pos) } });
    case APP_COMMAND_TAGS.ConfigCommand:
      return { ConfigCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.Command: {
      const seq = readVarintU64(bytes, pos);
      const command = readBytes(bytes, pos);
      const view_state = readBytes(bytes, pos);
      return { Command: { seq, command, view_state } };
    }
    case APP_COMMAND_TAGS.CommandText:
      return { CommandText: { seq: readVarintU64(bytes, pos), line: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.ContextMenu:
      return { ContextMenu: { seq: readVarintU64(bytes, pos), request: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.ArtifactCommand:
      return { ArtifactCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.ApplyEnvelopes: {
      const seq = readVarintU64(bytes, pos);
      const wire = readVecEnvelope(bytes, pos);
      return { ApplyEnvelopes: { seq, envelopes: wire.map((envelope) => mutationEnvelopeFromWire(envelope, replicationPackCodec)) } };
    }
    case APP_COMMAND_TAGS.ReadDocument:
      return { ReadDocument: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.LoadConfig: {
      const seq = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      return { LoadConfig: { seq, pack, spr } };
    }
    case APP_COMMAND_TAGS.ReadConfig:
      return { ReadConfig: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.LoadWindowConfig:
      return { LoadWindowConfig: { seq: readVarintU64(bytes, pos), entry: readWindowConfigPackEntry(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadWindowConfigs:
      return { ReadWindowConfigs: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.LoadDocumentArchive:
      return { LoadDocumentArchive: { seq: readVarintU64(bytes, pos), archive: readDocumentArchive(bytes, pos) } };
    case APP_COMMAND_TAGS.MergeDocumentArchive:
      return { MergeDocumentArchive: { seq: readVarintU64(bytes, pos), archive: readDocumentArchive(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadDocumentArchive:
      return { ReadDocumentArchive: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.PollDocumentArchiveLoad:
      return { PollDocumentArchiveLoad: { seq: readVarintU64(bytes, pos), operation: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.CancelDocumentArchiveLoad:
      return { CancelDocumentArchiveLoad: { seq: readVarintU64(bytes, pos), operation: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.AcknowledgeDocumentArchiveLoad:
      return { AcknowledgeDocumentArchiveLoad: { seq: readVarintU64(bytes, pos), operation: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.MediaIn: {
      const seq = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const descriptor = readBytes(bytes, pos);
      const data = readBytes(bytes, pos);
      return { MediaIn: { seq, port, descriptor, data } };
    }
    case APP_COMMAND_TAGS.MediaOut: {
      const seq = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const request = readBytes(bytes, pos);
      return { MediaOut: { seq, port, request } };
    }
    case APP_COMMAND_TAGS.MediaFingerprint:
      return { MediaFingerprint: { seq: readVarintU64(bytes, pos), port: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.PureCommand:
      return { PureCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos), head: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.LoadChildren:
      return { LoadChildren: { seq: readVarintU64(bytes, pos), entries: readVecChildPackEntry(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadChildren:
      return { ReadChildren: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadChildHeads:
      return { ReadChildHeads: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadHistory:
      return { ReadHistory: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionPrepare: {
      const seq = readVarintU64(bytes, pos);
      const txn_id = readStr(bytes, pos);
      const mutation_id = readStr(bytes, pos);
      const payload = readBytes(bytes, pos);
      const prepared_ops = readVecBytes(bytes, pos);
      const origin = readBytes(bytes, pos);
      const prepared_child_ops = readBytes(bytes, pos);
      return { transactionPrepare: { seq, txn_id, mutation_id, payload, prepared_ops, origin, prepared_child_ops } };
    }
    case APP_COMMAND_TAGS.transactionCommit:
      return { transactionCommit: { seq: readVarintU64(bytes, pos), txn_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionRollback:
      return { transactionRollback: { seq: readVarintU64(bytes, pos), txn_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionUndo:
      return { transactionUndo: { seq: readVarintU64(bytes, pos), group_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionRedo:
      return { transactionRedo: { seq: readVarintU64(bytes, pos), group_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.openArtifact: {
      const seq = readVarintU64(bytes, pos);
      const artifact_ref = readStr(bytes, pos);
      const role = bytes[pos[0]]!;
      pos[0] += 1;
      const plugin_id = readStr(bytes, pos);
      const app_id = readStr(bytes, pos);
      return { openArtifact: { seq, artifact_ref, role, plugin_id, app_id } };
    }
    case APP_COMMAND_TAGS.setDefaultApp: {
      const seq = readVarintU64(bytes, pos);
      const artifact_kind = readStr(bytes, pos);
      const standard = readStr(bytes, pos);
      const subset = readStr(bytes, pos);
      const role = bytes[pos[0]]!;
      pos[0] += 1;
      const plugin_id = readStr(bytes, pos);
      const app_id = readStr(bytes, pos);
      return { setDefaultApp: { seq, artifact_kind, standard, subset, role, plugin_id, app_id } };
    }
    case APP_COMMAND_TAGS.clearDefaultApp: {
      const seq = readVarintU64(bytes, pos);
      const artifact_kind = readStr(bytes, pos);
      const standard = readStr(bytes, pos);
      const subset = readStr(bytes, pos);
      const role = bytes[pos[0]]!;
      pos[0] += 1;
      return { clearDefaultApp: { seq, artifact_kind, standard, subset, role } };
    }
    case APP_COMMAND_TAGS.setMergePolicy: {
      const seq = readVarintU64(bytes, pos);
      const policy = bytes[pos[0]]!;
      pos[0] += 1;
      return { setMergePolicy: { seq, policy } };
    }
    case APP_COMMAND_TAGS.resolveConflict: {
      const seq = readVarintU64(bytes, pos);
      const conflict_id = readStr(bytes, pos);
      const resolution = bytes[pos[0]]!;
      pos[0] += 1;
      return { resolveConflict: { seq, conflict_id, resolution } };
    }
    case APP_COMMAND_TAGS.readConflicts:
      return { readConflicts: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.presence: {
      const seq = readVarintU64(bytes, pos);
      const own_color = readOptU8(bytes, pos);
      const peers = readVecBytes(bytes, pos);
      return { presence: { seq, own_color, peers } };
    }
    case APP_COMMAND_TAGS.LocalInteractionQuery: {
      const seq = readVarintU64(bytes, pos);
      const length = readVarintU64(bytes, pos);
      if (length > 142 || pos[0] + length !== bytes.length) throw new Error("local-interaction.command-envelope");
      const command = decodeLocalInteractionQueryCommand(bytes.subarray(pos[0]));
      return { LocalInteractionQuery: { seq, command } };
    }
    default:
      throw new Error(`decodeAppCommand: unknown tag ${bytes[0]}`);
  }
}

/** 📤️ `tag u8 | fields` — the TS twin of `protocol_channel::encode_app_frame` (agreed contract). */
export function encodeAppFrame(frame: AppFrameValue): Uint8Array {
  const out: number[] = [];
  if ("DocumentIdentity" in frame) {
    const { in_reply_to, identity } = frame.DocumentIdentity;
    assertAppDocumentIdentity(identity);
    out.push(APP_FRAME_TAGS.DocumentIdentity);
    writeVarintU64Exact(out, in_reply_to);
    writeVarintU64(out, identity.app_instance_id);
    writeBool(out, identity.parent_document_id !== null);
    if (identity.parent_document_id !== null) writeStr(out, identity.parent_document_id);
  } else if ("MediaExportSubmitted" in frame) {
    out.push(APP_FRAME_TAGS.MediaExportSubmitted);
    writeVarintU64Exact(out, frame.MediaExportSubmitted.in_reply_to);
    writeMediaExportHandle(out, frame.MediaExportSubmitted.handle);
  } else if ("MediaExportStatus" in frame) {
    const value = frame.MediaExportStatus, state = MEDIA_EXPORT_STATES.indexOf(value.state);
    if (state < 0) throw new Error("media export: invalid state");
    out.push(APP_FRAME_TAGS.MediaExportStatus);
    writeVarintU64Exact(out, value.in_reply_to);
    writeMediaExportHandle(out, value.handle);
    out.push(state);
    writeVarintU64Exact(out, value.applied_progress);
    writeBool(out, value.checkpoint_available);
    writeStr(out, value.mime_type);
    writeVarintU64Exact(out, value.total_bytes);
    writeStr(out, value.detail);
  } else if ("MediaExportChunk" in frame) {
    out.push(APP_FRAME_TAGS.MediaExportChunk);
    writeVarintU64Exact(out, frame.MediaExportChunk.in_reply_to);
    writeMediaExportHandle(out, frame.MediaExportChunk.handle);
    writeBytes(out, frame.MediaExportChunk.data);
    writeBool(out, frame.MediaExportChunk.terminal);
  } else if ("Done" in frame) {
    out.push(APP_FRAME_TAGS.Done);
    writeVarintU64(out, frame.Done.in_reply_to);
  } else if ("Invocation" in frame) {
    if (frame.Invocation.mutations.length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES || frame.Invocation.inverse_group.length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES) throw new Error("encodeAppFrame: invocation result pack exceeds command transport authority");
    out.push(APP_FRAME_TAGS.Invocation);
    writeVarintU64(out, frame.Invocation.in_reply_to);
    writeBytes(out, frame.Invocation.output);
    writeBytes(out, frame.Invocation.diagnostics);
    writeBytes(out, frame.Invocation.ui_scope);
    writeBytes(out, frame.Invocation.history_patch);
    writeBytes(out, frame.Invocation.messages);
    writeBytes(out, frame.Invocation.mutations);
    writeBytes(out, frame.Invocation.inverse_group);
  } else if ("DocumentChanged" in frame) {
    out.push(APP_FRAME_TAGS.DocumentChanged);
    writeVecBytes(out, frame.DocumentChanged.envelopes);
    writeStr(out, frame.DocumentChanged.origin);
  } else if ("Document" in frame) {
    out.push(APP_FRAME_TAGS.Document);
    writeVarintU64(out, frame.Document.in_reply_to);
    writeBytes(out, frame.Document.pack);
    writeBytes(out, frame.Document.spr);
    writeStr(out, frame.Document.ops);
  } else if ("DocumentArchive" in frame) {
    out.push(APP_FRAME_TAGS.DocumentArchive);
    writeVarintU64(out, frame.DocumentArchive.in_reply_to);
    writeDocumentArchive(out, frame.DocumentArchive.archive);
  } else if ("DocumentArchiveLoad" in frame) {
    out.push(APP_FRAME_TAGS.DocumentArchiveLoad);
    writeVarintU64(out, frame.DocumentArchiveLoad.in_reply_to);
    writeVarintU64(out, frame.DocumentArchiveLoad.status.operation);
    out.push(["pending", "running", "ready", "cancelled", "fault"].indexOf(frame.DocumentArchiveLoad.status.state));
    writeVarintU64(out, frame.DocumentArchiveLoad.status.completed);
    writeVarintU64(out, frame.DocumentArchiveLoad.status.total);
    writeVarintU64(out, frame.DocumentArchiveLoad.status.ahead);
    writeBytes(out, frame.DocumentArchiveLoad.status.fault);
  } else if ("Config" in frame) {
    out.push(APP_FRAME_TAGS.Config);
    writeVarintU64(out, frame.Config.in_reply_to);
    writeBytes(out, frame.Config.pack);
    writeBytes(out, frame.Config.spr);
    writeStr(out, frame.Config.ops);
  } else if ("WindowConfigs" in frame) {
    out.push(APP_FRAME_TAGS.WindowConfigs);
    writeVarintU64(out, frame.WindowConfigs.in_reply_to);
    writeVecWindowConfigPackEntry(out, frame.WindowConfigs.entries);
  } else if ("ConfigChanged" in frame) {
    out.push(APP_FRAME_TAGS.ConfigChanged);
    writeVecBytes(out, frame.ConfigChanged.envelopes);
    writeStr(out, frame.ConfigChanged.origin);
  } else if ("ContextMenu" in frame) {
    out.push(APP_FRAME_TAGS.ContextMenu);
    writeVarintU64(out, frame.ContextMenu.in_reply_to);
    writeBytes(out, frame.ContextMenu.items);
  } else if ("Media" in frame) {
    out.push(APP_FRAME_TAGS.Media);
    writeVarintU64(out, frame.Media.in_reply_to);
    writeStr(out, frame.Media.port);
    writeBytes(out, frame.Media.descriptor);
    writeBytes(out, frame.Media.data);
  } else if ("MediaFingerprint" in frame) {
    out.push(APP_FRAME_TAGS.MediaFingerprint);
    writeVarintU64(out, frame.MediaFingerprint.in_reply_to);
    writeStr(out, frame.MediaFingerprint.port);
    writeBytes(out, frame.MediaFingerprint.fingerprint);
  } else if ("Error" in frame) {
    out.push(APP_FRAME_TAGS.Error);
    writeOptU64(out, frame.Error.in_reply_to);
    writeBytes(out, frame.Error.fault);
    writeBytes(out, frame.Error.report);
  } else if ("Emit" in frame) {
    out.push(APP_FRAME_TAGS.Emit);
    writeVarintU64(out, frame.Emit.in_reply_to);
    writeBytes(out, frame.Emit.document_ops);
    writeBytes(out, frame.Emit.config_ops);
    writeBytes(out, frame.Emit.draft_ops);
    writeBytes(out, frame.Emit.output);
    writeBytes(out, frame.Emit.diagnostics);
    writeBytes(out, frame.Emit.child_ops);
  } else if ("Draft" in frame) {
    out.push(APP_FRAME_TAGS.Draft);
    writeVarintU64(out, frame.Draft.in_reply_to);
    writeBytes(out, frame.Draft.pack);
    writeBytes(out, frame.Draft.spr);
    writeStr(out, frame.Draft.ops);
  } else if ("Children" in frame) {
    out.push(APP_FRAME_TAGS.Children);
    writeVarintU64(out, frame.Children.in_reply_to);
    writeVecChildPackEntry(out, frame.Children.entries);
  } else if ("ChildHeads" in frame) {
    out.push(APP_FRAME_TAGS.ChildHeads);
    writeVarintU64(out, frame.ChildHeads.in_reply_to);
    writeVecChildHeadPackEntry(out, frame.ChildHeads.entries);
  } else if ("Ephemeral" in frame) {
    out.push(APP_FRAME_TAGS.Ephemeral);
    writeBytes(out, frame.Ephemeral.presence);
    writeVarintU64(out, frame.Ephemeral.presence_generation);
    writeVarintU64(out, frame.Ephemeral.transient_generation);
    writeBytes(out, frame.Ephemeral.interaction);
    writeBytes(out, frame.Ephemeral.tool_run);
    writeBytes(out, frame.Ephemeral.history_edit);
  } else if ("HistorySnapshot" in frame) {
    out.push(APP_FRAME_TAGS.HistorySnapshot);
    writeVarintU64(out, frame.HistorySnapshot.in_reply_to);
    writeBytes(out, frame.HistorySnapshot.history_patch);
  } else if ("transactionProposal" in frame) {
    out.push(APP_FRAME_TAGS.transactionProposal);
    writeVarintU64(out, frame.transactionProposal.in_reply_to);
    writeStr(out, frame.transactionProposal.proposal_id);
    writeVecBytes(out, frame.transactionProposal.local_ops);
    writeVecBytes(out, frame.transactionProposal.foreign);
  } else if ("transactionPrepared" in frame) {
    out.push(APP_FRAME_TAGS.transactionPrepared);
    writeStr(out, frame.transactionPrepared.txn_id);
    writeVecBytes(out, frame.transactionPrepared.foreign);
    writeBytes(out, frame.transactionPrepared.rejection);
  } else if ("transactionCommitted" in frame) {
    out.push(APP_FRAME_TAGS.transactionCommitted);
    writeStr(out, frame.transactionCommitted.txn_id);
    writeStr(out, frame.transactionCommitted.edit_id);
  } else if ("transactionRolledBack" in frame) {
    out.push(APP_FRAME_TAGS.transactionRolledBack);
    writeStr(out, frame.transactionRolledBack.txn_id);
  } else if ("MergeReport" in frame) {
    out.push(APP_FRAME_TAGS.MergeReport);
    writeOptU64(out, frame.MergeReport.in_reply_to);
    writeBytes(out, frame.MergeReport.report);
  } else if ("Conflicts" in frame) {
    out.push(APP_FRAME_TAGS.Conflicts);
    writeOptU64(out, frame.Conflicts.in_reply_to);
    writeBytes(out, frame.Conflicts.conflicts);
  } else if ("UiPatch" in frame) {
    out.push(APP_FRAME_TAGS.UiPatch);
    writeOptU64(out, frame.UiPatch.in_reply_to);
    writeStr(out, frame.UiPatch.surface);
    writeStr(out, frame.UiPatch.kind);
    writeVarintU64(out, frame.UiPatch.revision);
    writeVarintU64(out, frame.UiPatch.base_revision);
    writeBytes(out, frame.UiPatch.ops);
  } else if ("LocalInteractionQuery" in frame) {
    out.push(APP_FRAME_TAGS.LocalInteractionQuery);
    writeBytes(out, Array.from(encodeLocalInteractionQueryReply(frame.LocalInteractionQuery.reply)));
  } else if ("UiSnapshotEnd" in frame) {
    out.push(APP_FRAME_TAGS.UiSnapshotEnd);
    writeVarintU64(out, frame.UiSnapshotEnd.revision);
  } else if ("OperationCompleted" in frame) {
    out.push(APP_FRAME_TAGS.OperationCompleted);
    writeVarintU64Exact(out, frame.OperationCompleted.operation);
    writeVarintU64Exact(out, frame.OperationCompleted.revision);
    writeBytes(out, frame.OperationCompleted.ui_scope);
    writeBytes(out, frame.OperationCompleted.history_patch);
  } else {
    throw new Error("encodeAppFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}

/** 📥️ Inverse of {@link encodeAppFrame} — the TS twin of `protocol_channel::decode_app_frame`. */
export function decodeAppFrame(bytes: Uint8Array): AppFrameValue {
  if (bytes.length === 0) throw new Error("decodeAppFrame: empty frame");
  const pos: [number] = [1];
  switch (bytes[0]) {
    case APP_FRAME_TAGS.DocumentIdentity: {
      if (bytes.length > 2071) throw new Error("document identity: frame capacity");
      const in_reply_to = readVarintU64Exact(bytes, pos);
      const app_instance_id = readVarintU64(bytes, pos);
      const presence = readU8(bytes, pos);
      if (presence !== 0 && presence !== 1) throw new Error("document identity: invalid optional document");
      const parent_document_id = presence === 1 ? new TextDecoder("utf-8", { fatal: true }).decode(Uint8Array.from(readBytes(bytes, pos))) : null;
      const identity = { app_instance_id, parent_document_id };
      assertAppDocumentIdentity(identity);
      if (pos[0] !== bytes.length) throw new Error("document identity: trailing frame bytes");
      return { DocumentIdentity: { in_reply_to, identity } };
    }
    case APP_FRAME_TAGS.MediaExportSubmitted:
      return exactMediaExportFrame(bytes, pos, { MediaExportSubmitted: { in_reply_to: readVarintU64Exact(bytes, pos), handle: readMediaExportHandle(bytes, pos) } });
    case APP_FRAME_TAGS.MediaExportStatus: {
      const in_reply_to = readVarintU64Exact(bytes, pos), handle = readMediaExportHandle(bytes, pos), state = MEDIA_EXPORT_STATES[readU8(bytes, pos)];
      if (state === undefined) throw new Error("media export: invalid state");
      return exactMediaExportFrame(bytes, pos, { MediaExportStatus: { in_reply_to, handle, state, applied_progress: readVarintU64Exact(bytes, pos), checkpoint_available: readBool(bytes, pos), mime_type: readStr(bytes, pos), total_bytes: readVarintU64Exact(bytes, pos), detail: readStr(bytes, pos) } });
    }
    case APP_FRAME_TAGS.MediaExportChunk:
      return exactMediaExportFrame(bytes, pos, { MediaExportChunk: { in_reply_to: readVarintU64Exact(bytes, pos), handle: readMediaExportHandle(bytes, pos), data: readBytes(bytes, pos), terminal: readBool(bytes, pos) } });
    case APP_FRAME_TAGS.Done:
      return { Done: { in_reply_to: readVarintU64(bytes, pos) } };
    case APP_FRAME_TAGS.Invocation: {
      const in_reply_to = readVarintU64(bytes, pos);
      const output = readBytes(bytes, pos);
      const diagnostics = readBytes(bytes, pos);
      const ui_scope = readBytes(bytes, pos);
      const history_patch = readBytes(bytes, pos);
      const messages = readBytes(bytes, pos);
      const readInvocationResultPack = (): readonly number[] => {
        const length = readVarintU64(bytes, pos);
        if (length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES) throw new Error("decodeAppFrame: invocation result pack exceeds command transport authority");
        const end = pos[0] + length;
        if (!Number.isSafeInteger(end) || end > bytes.length) throw new Error("decodeAppFrame: invocation result pack is truncated");
        const value = Array.from(bytes.subarray(pos[0], end));
        pos[0] = end;
        return value;
      };
      const mutations = readInvocationResultPack();
      const inverse_group = readInvocationResultPack();
      return { Invocation: { in_reply_to, output, diagnostics, ui_scope, history_patch, messages, mutations, inverse_group } };
    }
    case APP_FRAME_TAGS.DocumentChanged: {
      const envelopes = readVecBytes(bytes, pos);
      const origin = readStr(bytes, pos);
      return { DocumentChanged: { envelopes, origin } };
    }
    case APP_FRAME_TAGS.Document: {
      const in_reply_to = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      const ops = readStr(bytes, pos);
      return { Document: { in_reply_to, pack, spr, ops } };
    }
    case APP_FRAME_TAGS.DocumentArchive:
      return { DocumentArchive: { in_reply_to: readVarintU64(bytes, pos), archive: readDocumentArchive(bytes, pos) } };
    case APP_FRAME_TAGS.DocumentArchiveLoad: {
      const in_reply_to = readVarintU64(bytes, pos);
      const operation = readVarintU64(bytes, pos);
      const states = ["pending", "running", "ready", "cancelled", "fault"] as const;
      const state = states[bytes[pos[0]++]!];
      if (state === undefined) throw new Error("document archive load state: unknown state");
      return { DocumentArchiveLoad: { in_reply_to, status: { operation, state, completed: readVarintU64(bytes, pos), total: readVarintU64(bytes, pos), ahead: readVarintU64(bytes, pos), fault: readBytes(bytes, pos) } } };
    }
    case APP_FRAME_TAGS.Config: {
      const in_reply_to = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      const ops = readStr(bytes, pos);
      return { Config: { in_reply_to, pack, spr, ops } };
    }
    case APP_FRAME_TAGS.WindowConfigs:
      return { WindowConfigs: { in_reply_to: readVarintU64(bytes, pos), entries: readVecWindowConfigPackEntry(bytes, pos) } };
    case APP_FRAME_TAGS.ConfigChanged: {
      const envelopes = readVecBytes(bytes, pos);
      const origin = readStr(bytes, pos);
      return { ConfigChanged: { envelopes, origin } };
    }
    case APP_FRAME_TAGS.ContextMenu:
      return { ContextMenu: { in_reply_to: readVarintU64(bytes, pos), items: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Media: {
      const in_reply_to = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const descriptor = readBytes(bytes, pos);
      const data = readBytes(bytes, pos);
      return { Media: { in_reply_to, port, descriptor, data } };
    }
    case APP_FRAME_TAGS.MediaFingerprint: {
      const in_reply_to = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const fingerprint = readBytes(bytes, pos);
      return { MediaFingerprint: { in_reply_to, port, fingerprint } };
    }
    case APP_FRAME_TAGS.Error: {
      const in_reply_to = readOptU64(bytes, pos);
      const fault = readBytes(bytes, pos);
      const report = readBytes(bytes, pos);
      return { Error: { in_reply_to, fault, report } };
    }
    case APP_FRAME_TAGS.Emit:
      return { Emit: { in_reply_to: readVarintU64(bytes, pos), document_ops: readBytes(bytes, pos), config_ops: readBytes(bytes, pos), draft_ops: readBytes(bytes, pos), output: readBytes(bytes, pos), diagnostics: readBytes(bytes, pos), child_ops: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Draft:
      return { Draft: { in_reply_to: readVarintU64(bytes, pos), pack: readBytes(bytes, pos), spr: readBytes(bytes, pos), ops: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.Children:
      return { Children: { in_reply_to: readVarintU64(bytes, pos), entries: readVecChildPackEntry(bytes, pos) } };
    case APP_FRAME_TAGS.ChildHeads:
      return { ChildHeads: { in_reply_to: readVarintU64(bytes, pos), entries: readVecChildHeadPackEntry(bytes, pos) } };
    case APP_FRAME_TAGS.Ephemeral:
      return {
        Ephemeral: { presence: readBytes(bytes, pos), presence_generation: readVarintU64(bytes, pos), transient_generation: readVarintU64(bytes, pos), interaction: readBytes(bytes, pos), tool_run: readBytes(bytes, pos), history_edit: readBytes(bytes, pos) },
      };
    case APP_FRAME_TAGS.HistorySnapshot:
      return { HistorySnapshot: { in_reply_to: readVarintU64(bytes, pos), history_patch: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.transactionProposal: {
      const in_reply_to = readVarintU64(bytes, pos);
      const proposal_id = readStr(bytes, pos);
      const local_ops = readVecBytes(bytes, pos);
      const foreign = readVecBytes(bytes, pos);
      return { transactionProposal: { in_reply_to, proposal_id, local_ops, foreign } };
    }
    case APP_FRAME_TAGS.transactionPrepared: {
      const txn_id = readStr(bytes, pos);
      const foreign = readVecBytes(bytes, pos);
      const rejection = readBytes(bytes, pos);
      return { transactionPrepared: { txn_id, foreign, rejection } };
    }
    case APP_FRAME_TAGS.transactionCommitted:
      return { transactionCommitted: { txn_id: readStr(bytes, pos), edit_id: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.transactionRolledBack:
      return { transactionRolledBack: { txn_id: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.MergeReport:
      return { MergeReport: { in_reply_to: readOptU64(bytes, pos), report: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Conflicts:
      return { Conflicts: { in_reply_to: readOptU64(bytes, pos), conflicts: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.UiPatch: {
      const in_reply_to = readOptU64(bytes, pos);
      const surface = readStr(bytes, pos);
      const kind = readStr(bytes, pos);
      const revision = readVarintU64(bytes, pos);
      const base_revision = readVarintU64(bytes, pos);
      const ops = readBytes(bytes, pos);
      return { UiPatch: { in_reply_to, surface, kind, revision, base_revision, ops } };
    }
    case APP_FRAME_TAGS.UiSnapshotEnd:
      return { UiSnapshotEnd: { revision: readVarintU64(bytes, pos) } };
    case APP_FRAME_TAGS.OperationCompleted: {
      const operation = readVarintU64Exact(bytes, pos);
      const revision = readVarintU64Exact(bytes, pos);
      const ui_scope = readBytes(bytes, pos);
      const history_patch = readBytes(bytes, pos);
      return { OperationCompleted: { operation, revision, ui_scope, history_patch } };
    }
    case APP_FRAME_TAGS.LocalInteractionQuery: {
      const length = readVarintU64(bytes, pos);
      if (length > 4256 || pos[0] + length !== bytes.length) throw new Error("local-interaction.reply-envelope");
      return { LocalInteractionQuery: { reply: decodeLocalInteractionQueryReply(bytes.subarray(pos[0])) } };
    }
    default:
      throw new Error(`decodeAppFrame: unknown tag ${bytes[0]}`);
  }
}
//#endregion 🔖️Codec
//#endregion 🔖️AppChannelCodec

//#region 🔖️AppChannelClient
/**
 * 📡️ TS twin of `protocol_channel::CHANNEL_VERSION` (`🔨️modules/📡️protocol/🧵️channel/📦️packages/🦀️rust/📦️lib.rs`)
 * — bump both sides together on a wire-incompatible frame change.
 */
/** 📥️ Decodes a pack-encoded {@link Fault} from an app-channel error frame. */
export function decodeFaultFromWire(faultBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): Fault | null {
  try {
    const raw = decodePackValue(new Uint8Array(faultBytes));
    if (!raw || typeof raw !== "object" || !("message" in raw)) return null;
    return raw as Fault;
  } catch {
    return null;
  }
}

/** 🧯️ One display rule for every app-channel fault: a pack-encoded {@link Fault} reads as
 * `code: message`, and a guest that answered with plain UTF-8 fault text reads as that text. */
export function faultDisplayMessage(faultBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): string {
  const fault = decodeFaultFromWire(faultBytes, decodePackValue);
  if (!fault) {
    const text = new TextDecoder().decode(new Uint8Array(faultBytes)).trim();
    return text.length > 0 ? text : "unknown fault";
  }
  const code = typeof fault.code === "string" ? fault.code : String(fault.code);
  return `${code}: ${fault.message}`;
}

/** 📥️ Decodes a pack-encoded {@link DispatchReport} from an app-channel wire blob —
 * `AppFrame::Invocation.messages` (a successful dispatch's report) or `AppFrame::Error.report` (the
 * rejected dispatch's report, `Fault.code == "app.command.rejected"`). `null` for an empty blob (the
 * trailing field's zero value before every dispatch path was updated to populate it). */
export function decodeDispatchReportFromWire(reportBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): DispatchReport | null {
  if (reportBytes.length === 0) return null;
  try {
    return decodePackValue(new Uint8Array(reportBytes)) as DispatchReport;
  } catch {
    return null;
  }
}

/** 📨️ Decodes an `AppFrame::Error.report` blob into its typed `MutationMessage`s — so a
 * caller reacting to a rejected dispatch (contract-freeze §C8/§C9) gets structured messages instead
 * of parsing {@link faultDisplayMessage}'s prose string. Empty array for an empty/undecodable blob. */
export function faultMessages(reportBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): readonly MutationMessage[] {
  return decodeDispatchReportFromWire(reportBytes, decodePackValue)?.messages ?? [];
}

/** 📥️ Decodes a pack-encoded {@link MergeReport} from an `AppFrame::MergeReport.report`
 * blob — pushed unsolicited after every `ingest_remote`/`resolve_conflict`,
 * alongside `DocumentChanged`. */
export function decodeMergeReportFromWire(reportBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): MergeReport | null {
  if (reportBytes.length === 0) return null;
  try {
    return decodePackValue(new Uint8Array(reportBytes)) as MergeReport;
  } catch {
    return null;
  }
}

/** 📥️ Decodes a pack-encoded {@link Conflict}[] projection from an `AppFrame::
 * Conflicts.conflicts` blob — pushed unsolicited after every ingest (alongside `DocumentChanged`)
 * and in reply to `AppCommand::ReadConflicts`. */
export function decodeConflictsFromWire(conflictsBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): readonly Conflict[] {
  if (conflictsBytes.length === 0) return [];
  try {
    return decodePackValue(new Uint8Array(conflictsBytes)) as readonly Conflict[];
  } catch {
    return [];
  }
}

/** 📡️ TS twin of `protocol_channel::CHANNEL_VERSION`. Both constants are pinned against
 * `🔨️modules/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json`, which owns the number — this one sat at 8 while Rust
 * had moved to 10, so the pin exists to make a half-done bump fail a test instead of a session.
 * Channel v12 retired the `Hello`/`Welcome` handshake this constant used to be carried on; since v21 every host admits a guest
 * against it through {@link admitGuestChannelVersion} before any frame. */
export const APP_CHANNEL_VERSION = 23;

/** 🤝️ Fault code of a guest compiled for another app-channel version than its host — TS twin of Rust `CHANNEL_MISMATCH_CODE`. */
export const CHANNEL_MISMATCH_CODE = "plugin.channel-mismatch";

/** 🤝️ Admits a guest compiled for the host's app-channel version (`reactor.channelVersion()`, read once right after instantiation);
 * any other version, older or newer, is refused before a single frame with the framework fault `plugin.channel-mismatch`, whose
 * notice names `{guest}` and `{host}`. `null` admits. TS twin of Rust `admit_guest_channel_version`, corpus
 * `🧫️fixtures/🧫️channel-handshake`. Self-contained: the materialized jco bridge embeds it verbatim. */
export function admitGuestChannelVersion(guest: number, host: number): Fault | null {
  if (guest === host) return null;
  return { origin: "framework", code: "plugin.channel-mismatch", severity: "error", message: `the guest speaks app channel ${guest}, the host app channel ${host}`, scope: {}, params: { guest: String(guest), host: String(host) }, retryable: false };
}

/** 📡️ The slice of {@link PluginWasmHandle} {@link AppChannelClient} needs — deliberately narrower
 * than the full handle so a caller can hand in any object shaped like it (a real handle, a test
 * double, ...) without importing the rest of `@semio-tech/framework`'s plugin-loading surface. */
export type AppChannelHandle = Pick<PluginWasmHandle, "enqueue" | "outcomes">;

//#region 🏠️LocalQueryOwnership
type LocalInteractionClientQuery = {
  readonly requestId: string;
  readonly cancelSequence: number;
  readonly consume: (page: LocalInteractionPage) => Promise<void>;
  readonly resolve: (identity: LocalInteractionIdentity) => void;
  readonly reject: (error: unknown) => void;
  readonly signal: AbortSignal | undefined;
  readonly abort: () => void;
  token: LocalInteractionQueryToken | null;
  nextOrdinal: bigint;
  consuming: boolean;
  cancelled: boolean;
  terminalConsumed: boolean;
  failure: unknown;
};

function sameLocalInteractionQuery(left: LocalInteractionQueryToken, right: LocalInteractionQueryToken): boolean {
  return left.requestId === right.requestId && left.queryGeneration === right.queryGeneration && localInteractionIdentityEquals(left.identity, right.identity);
}
type AppChannelTransactionReply = { readonly kind: "prepared" | "committed" | "rolledBack"; readonly id: string };

function appChannelTransactionReply(command: AppCommandValue): AppChannelTransactionReply | null {
  if ("transactionPrepare" in command) return { kind: "prepared", id: command.transactionPrepare.txn_id };
  if ("transactionCommit" in command) return { kind: "committed", id: command.transactionCommit.txn_id };
  if ("transactionRollback" in command) return { kind: "rolledBack", id: command.transactionRollback.txn_id };
  return null;
}

function appChannelReplySequence(frame: AppFrameValue): number | null {
  const value = Object.values(frame)[0];
  if (!value || !("in_reply_to" in value)) return null;
  const sequence = value.in_reply_to;
  if (typeof sequence === "bigint") return sequence >= 0n && sequence <= BigInt(Number.MAX_SAFE_INTEGER) ? Number(sequence) : null;
  return typeof sequence === "number" ? sequence : null;
}

function appChannelFrameBelongsTo(frame: AppFrameValue, sequence: number, transaction: AppChannelTransactionReply | null): boolean {
  const replySequence = appChannelReplySequence(frame);
  if (replySequence !== null) return replySequence === sequence;
  if ("DocumentIdentity" in frame || "MediaExportSubmitted" in frame || "MediaExportStatus" in frame || "MediaExportChunk" in frame) return false;
  if ("transactionPrepared" in frame) return transaction?.kind === "prepared" && transaction.id === frame.transactionPrepared.txn_id;
  if ("transactionCommitted" in frame) return transaction?.kind === "committed" && transaction.id === frame.transactionCommitted.txn_id;
  if ("transactionRolledBack" in frame) return transaction?.kind === "rolledBack" && transaction.id === frame.transactionRolledBack.txn_id;
  return true;
}
/** 🪪️ One checked handle-lifetime owner spans every client recreation; query admission reserves its cancellation receipt identity. */
export class AppChannelRequestSequence {
  private sequence: number;
  private request: bigint;
  constructor(sequence = 0, request = 0n) {
    if (!Number.isSafeInteger(sequence) || sequence < 0 || request < 0n || request > 0xffffffffffffffffn) throw new Error("app-channel.invalid-sequence-owner");
    this.sequence = sequence;
    this.request = request;
  }

  nextSequence(): number {
    if (this.sequence === Number.MAX_SAFE_INTEGER) throw new Error("app-channel.sequence-exhausted");
    return ++this.sequence;
  }

  nextQuery(): { readonly sequence: number; readonly cancelSequence: number; readonly request: string } {
    if (this.sequence > Number.MAX_SAFE_INTEGER - 2) throw new Error("app-channel.sequence-exhausted");
    if (this.request === 0xffffffffffffffffn) throw new Error("local-interaction.request-exhausted");
    const sequence = this.sequence + 1;
    this.sequence += 2;
    return { sequence, cancelSequence: this.sequence, request: (++this.request).toString() };
  }

  checkpoint(): { readonly sequence: number; readonly request: string } {
    return { sequence: this.sequence, request: this.request.toString() };
  }
}
//#endregion 🏠️LocalQueryOwnership

//#region 🗃️DocumentArchiveLoadHost
/** 🏁️ How one host-driven whole-document load ended (`📓️api-stepped-document-load.md` §2.3): twin of the Rust
 * `DocumentArchiveLoadOutcome`. `cancelled` and `fault` both left the previous document exactly as it was. */
export type DocumentArchiveLoadOutcome = { readonly kind: "ready" } | { readonly kind: "cancelled" } | { readonly kind: "fault"; readonly fault: readonly number[] };

/** 🧭️ The next command to send, stamped with the sequence the host minted for it, or the outcome once the load is over. */
export type DocumentArchiveLoadStep = { readonly kind: "send"; readonly seq: number; readonly command: AppCommandValue } | { readonly kind: "finished"; readonly outcome: DocumentArchiveLoadOutcome };

/** 📬️ What one answer moved the load to: the polled status (`null` for every other command), or what it cannot go on from —
 * the guest's encoded refusal, or a frame that answers another command. */
export type DocumentArchiveLoadAnswer = { readonly kind: "status"; readonly status: DocumentArchiveLoadStatus | null } | { readonly kind: "refused"; readonly fault: readonly number[] } | { readonly kind: "unanswered" };

/** 🔀️ How one read-back merge ended (design §22.22): `merged` events of the archive joined the program's log and the program
 * holds `ahead` events the archive lacks — or the program cannot merge the archive into the document it shows and `code` is
 * its own refusal ({@link DOCUMENT_ARCHIVE_UNMERGEABLE_CODES}); nothing was touched, and loading the archive is the way on. */
export type DocumentArchiveMergeV1 = { readonly kind: "merged"; readonly merged: number; readonly ahead: number } | { readonly kind: "unmergeable"; readonly code: string };

/** 🧯️ A whole-document load or merge the program refused or faulted. `fault` is the program's own fault when its bytes decode
 * — its code names the notice a person is told — and `message` keeps the engineering text for a log, never for a person. */
export class DocumentArchiveFaultError extends Error {
  readonly fault: Fault | null;

  constructor(message: string, fault: Fault | null) {
    super(message);
    this.name = "DocumentArchiveFaultError";
    this.fault = fault;
  }
}

/** 🚫️ The guest's refusals of a read-back merge that leave a load as the way on: the archive is another document's, or its
 * members differ from the live ones (twins of `DOCUMENT_LOAD_OTHER_DOCUMENT_CODE` and `DOCUMENT_LOAD_MEMBERS_DIFFER_CODE`,
 * `🔨️modules/🔌️plugin/🦀️.rs`). */
export const DOCUMENT_ARCHIVE_UNMERGEABLE_CODES: readonly string[] = ["plugin.document-load.other-document", "plugin.document-load.members-differ"];

type DocumentArchiveLoadSent = "admit" | "poll" | "cancel" | "acknowledge";

/** 🗃️ The host half of the stepped, ACK-owned whole-document load: the TS twin of the Rust `DocumentArchiveLoadHost`
 * (`🔨️modules/📡️spr/🧵️channel/🦀️.rs`), a transport-free state machine both replay against one corpus
 * (`🔨️modules/📡️spr/🧵️channel/🧫️fixtures/🧫️document-archive-load-host/🔣️.json`). The admission's own sequence is the operation, a
 * host's sequence is minted only for a command that is sent, and steps and answers alternate. A cancel before admission sends nothing; after admission it is sent once and polling goes on
 * until the guest's terminal status says whether the previous document was restored (a refused cancel lost the race). A
 * cancelled terminal whose acknowledgement is refused still owns retained input, so it is polled again. A read-back of the
 * document the program already shows is a merge ({@link merging}): the same exchange under its own admission command. */
export class DocumentArchiveLoadHost {
  private archive: DocumentArchivePack | null;
  private admitted: number | null = null;
  private sent: { readonly seq: number; readonly kind: DocumentArchiveLoadSent } | null = null;
  private cancelRequested = false;
  private cancelSent = false;
  private terminal: DocumentArchiveLoadStatus | null = null;
  private outcome: DocumentArchiveLoadOutcome | null = null;
  private merge = false;
  private aheadOfArchive = 0;

  constructor(archive: DocumentArchivePack) {
    this.archive = archive;
  }

  /** 🔀️ A read-back merge of the document the program already shows (design §22.22): admitted with `MergeDocumentArchive`,
   * then polled, cancelled and acknowledged exactly like a load. Its events join the program's log and nothing is
   * replaced; {@link ahead} answers what the program then holds that the archive lacks. */
  static merging(archive: DocumentArchivePack): DocumentArchiveLoadHost {
    const host = new DocumentArchiveLoadHost(archive);
    host.merge = true;
    return host;
  }

  /** 🎞️ A load the guest admitted itself under `operation`, the host sequence of the command that started it (a `MediaIn` of a
   * whole document of the app's own schema answers its pending status): driven from its first poll like an admission this host sent. */
  static admitted(operation: number): DocumentArchiveLoadHost {
    const host = new DocumentArchiveLoadHost({ parent_pack: [], parent_spr: [], members: [] });
    host.archive = null;
    host.admitted = operation;
    return host;
  }

  /** 🔢️ The admitted operation, once the admission was sent. */
  get operation(): number | null {
    return this.admitted;
  }

  /** ⏭️ The events the program holds that the archive lacks, as its acknowledged ready terminal counted them: what a writer
   * persists again for. 0 for a replacement and before the operation ended ready. */
  get ahead(): number {
    return this.aheadOfArchive;
  }

  /** 🛑️ Asks for the load to stop; idempotent, and a no-op once a terminal status is known. */
  requestCancel(): void {
    this.cancelRequested = true;
    if (this.admitted === null && this.outcome === null) {
      this.archive = null;
      this.outcome = { kind: "cancelled" };
    }
  }

  /** ➡️ The next command, stamped with a sequence from `nextSeq`, or the outcome once the load is over. */
  step(nextSeq: () => number): DocumentArchiveLoadStep {
    if (this.outcome) return { kind: "finished", outcome: this.outcome };
    const seq = nextSeq();
    const operation = this.admitted;
    const archive = this.archive ?? { parent_pack: [], parent_spr: [], members: [] };
    const [kind, command]: readonly [DocumentArchiveLoadSent, AppCommandValue] =
      operation === null ? ["admit", this.merge ? { MergeDocumentArchive: { seq, archive } } : { LoadDocumentArchive: { seq, archive } }]
      : this.terminal ? ["acknowledge", { AcknowledgeDocumentArchiveLoad: { seq, operation } }]
      : this.cancelRequested && !this.cancelSent ? ["cancel", { CancelDocumentArchiveLoad: { seq, operation } }]
      : ["poll", { PollDocumentArchiveLoad: { seq, operation } }];
    if (kind === "admit") {
      this.admitted = seq;
      this.archive = null;
    }
    this.sent = { seq, kind };
    return { kind: "send", seq, command };
  }

  /** 📬️ Takes the guest's answer to the command sent under `seq`. */
  answer(seq: number, frame: AppFrameValue): DocumentArchiveLoadAnswer {
    const sent = this.sent;
    const operation = this.admitted;
    this.sent = null;
    if (sent === null || sent.seq !== seq || operation === null) return { kind: "unanswered" };
    if ("Done" in frame && frame.Done.in_reply_to === seq && sent.kind !== "poll") {
      if (sent.kind === "cancel") this.cancelSent = true;
      if (sent.kind === "acknowledge") {
        const terminal = this.terminal;
        if (!terminal) return { kind: "unanswered" };
        this.terminal = null;
        if (terminal.state === "ready") this.aheadOfArchive = terminal.ahead;
        this.outcome = terminal.state === "ready" ? { kind: "ready" } : terminal.state === "cancelled" ? { kind: "cancelled" } : { kind: "fault", fault: terminal.fault };
      }
      return { kind: "status", status: null };
    }
    if ("DocumentArchiveLoad" in frame && sent.kind === "poll" && frame.DocumentArchiveLoad.in_reply_to === seq && frame.DocumentArchiveLoad.status.operation === operation) {
      const status = frame.DocumentArchiveLoad.status;
      if (status.state === "ready" || status.state === "cancelled" || status.state === "fault") this.terminal = status;
      return { kind: "status", status };
    }
    if ("Error" in frame && frame.Error.in_reply_to === seq) {
      if (sent.kind === "cancel") {
        this.cancelSent = true;
        return { kind: "status", status: null };
      }
      if (sent.kind === "acknowledge" && this.terminal?.state === "cancelled") {
        this.terminal = null;
        return { kind: "status", status: null };
      }
      return { kind: "refused", fault: frame.Error.fault };
    }
    return { kind: "unanswered" };
  }
}

/** 🎯️ The one frame of a command's reply that answers `seq`: its refusal first, else its `Done` or archive-load status. */
function documentArchiveLoadReplyV1(frames: readonly AppFrameValue[], seq: number): AppFrameValue | undefined {
  return frames.find((frame) => "Error" in frame && frame.Error.in_reply_to === seq)
    ?? frames.find((frame) => ("Done" in frame && frame.Done.in_reply_to === seq) || ("DocumentArchiveLoad" in frame && frame.DocumentArchiveLoad.in_reply_to === seq));
}
//#endregion 🗃️DocumentArchiveLoadHost

/** ⏳️ Resolves on the next macrotask — where a document-archive load waits between polls, so the page's timers, input
 * and rendering (the Tasks window's Cancel) run while the guest works; a microtask-only re-poll starved them. */
function nextArchivePollTurnV1(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

/** 🎞️ One unsolicited mid-operation `Invocation` frame as {@link AppChannelClient.onOperationProgress} hands it on: the
 * decoded `UiDirtyScope` and `HistoryPatch` it carries, each absent when the frame carried none. */
export type AppChannelOperationProgressV1 = { readonly uiScope?: unknown; readonly historyPatch?: unknown };

/**
 * 📡️ Typed facade over one plugin instance's app channel — encodes an {@link AppCommandValue}, queues
 * it via {@link PluginWasmHandle.enqueue}, and decodes every {@link AppFrameValue} the matching
 * {@link TurnOutcome} carries. This is the ONLY place `AppCommand`/`AppFrame` framing happens on the
 * host side; callers (a React renderer's dispatch/refresh loop, a headless workflow runner) work with
 * decoded frames and plain JS values, never raw bytes or wire tags. `seq` is a handle-owned monotonic
 * counter — the host has no other way to correlate a `Command`/`ConfigCommand`/
 * `ReadDocument`/`LoadConfig`/`ReadConfig` with the `Invocation`/`Document` frame(s) it produced
 * (`AppFrame.*.in_reply_to`). Channel v12 retired the `hello()`/`refreshUi()`/`attachBackbone()`/
 * `detachBackbone()`/`drain()` surface this class used to expose — the handshake, cache-probed UI
 * refresh, and empty-batch drain all disappeared with the reactor ABI (lifecycle now arrives via
 * `Event::InstanceOpen`/`InstanceClose`, UI updates are a `UiPatch` push, and guests wake on
 * events/timers/`next-wake` rather than a poll).
 *
 * `📌️important.md`'s "Replace, never wrap" list: the old handle's synchronous
 * per-call method used to hand this class its reply directly; the handle is now fire-and-forget
 * ({@link PluginWasmHandle.enqueue}) and replies arrive on the handle-wide {@link
 * PluginWasmHandle.outcomes} stream instead, so THIS class is what turns that stream back into the
 * one-reply-per-call shape every method below still returns. One background loop
 * ({@link pumpOutcomes}) owns the handle's async iterator for this instance's lifetime and routes
 * every explicit numeric receipt to its exact waiter. Empty outcomes and uncorrelated notifications
 * cannot complete a command. Transaction payloads accompany their numeric receipt and additionally
 * match the pending transaction identity. One outcome may complete several distinct commands.
 */
export class AppChannelClient {
  private localQuery: LocalInteractionClientQuery | null = null;
  private readonly completionListeners = new Set<(completion: OperationCompletionV1) => void>();
  private readonly progressListeners = new Set<(progress: AppChannelOperationProgressV1) => void>();
  private disposed = false;
  private retired = false;
  private readonly handle: AppChannelHandle;
  private readonly instanceId: number;
  private readonly appId: string;
  private readonly actor: string;
  private readonly sequenceOwner: AppChannelRequestSequence;
  private readonly outcomeIterator: AsyncIterator<TurnOutcome>;
  private readonly pending: { readonly seq: number; readonly queryReceipt: boolean; readonly transaction: AppChannelTransactionReply | null; readonly resolve: (frames: AppFrameValue[]) => void; readonly reject: (error: unknown) => void }[] = [];
  /** 📦️ Per-instance document-pack cache (ticket
   * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS, scout-1 §4: "the
   * browser host keeps NO document pack per instance today"). Populated from BOTH directions —
   * {@link loadDocumentArchive}'s ready root pair and every
   * `AppFrame::Document` reply any sent command's outcome carries (`ReadDocument`, or any future command that
   * happens to include one) — so a transaction coordinator can
   * ask "what does this instance's document look like right now" without a dedicated round trip. */
  private cachedPack: Uint8Array | null = null;
  private cachedSpr: Uint8Array | null = null;
  /** 👥️ The last `AppFrame::Ephemeral` any outcome of this instance carried (every guest exchange appends one,
   * contract-freeze §C7.6) — the presence heartbeat's pack and interaction slice, read without a round trip (the
   * native twin: `ProgramBridge::observe_ephemeral`). */
  private lastEphemeral: Extract<AppFrameValue, { readonly Ephemeral: unknown }>["Ephemeral"] | null = null;

  constructor(handle: AppChannelHandle, sequenceOwner: AppChannelRequestSequence, instanceId: number, appId: string, actor: string = "local") {
    this.handle = handle;
    this.sequenceOwner = sequenceOwner;
    this.instanceId = instanceId;
    this.appId = appId;
    this.actor = actor;
    this.outcomeIterator = handle.outcomes[Symbol.asyncIterator]();
    void this.pumpOutcomes();
  }

  /** 🔁️ Owns this client's subscription against the handle-wide {@link PluginWasmHandle.outcomes}
   * stream for its whole lifetime — runs until {@link dispose} calls the iterator's own `return()`
   * (what breaks this loop) or the handle itself completes it. Every outcome for a DIFFERENT
   * `instanceId` is silently skipped (it belongs to a sibling `AppChannelClient` on the same handle);
   * stale or duplicate numeric receipts do not consume another command's waiter. Query pages retain
   * their separate ACK-owned lifecycle even when coalesced with ordinary replies or notifications. */
  private async pumpOutcomes(): Promise<void> {
    for (;;) {
      const step = await this.outcomeIterator.next();
      if (step.done) {
        this.cachedPack = null;
        this.cachedSpr = null;
        this.finishLocalInteractionQuery(new Error("local-interaction.channel-closed"));
        for (const waiter of this.pending.splice(0)) waiter.reject(new Error("app-channel.closed"));
        return;
      }
      const outcome = step.value;
      if (outcome.instanceId !== this.instanceId) continue;
      if ("error" in outcome) {
        this.finishLocalInteractionQuery(outcome.error);
        this.pending.shift()?.reject(outcome.error);
        continue;
      }
      const frames: AppFrameValue[] = [];
      for (const encoded of outcome.frames) {
        try { frames.push(decodeAppFrame(encoded)); }
        catch (error) {
          this.cancelLocalInteractionQuery(error);
          for (let index = this.pending.length - 1; index >= 0; index -= 1) {
            if (!this.pending[index]!.queryReceipt) this.pending.splice(index, 1)[0]!.reject(error);
          }
        }
      }
      const ordinary: AppFrameValue[] = [];
      for (const frame of frames) {
        if ("Ephemeral" in frame) this.lastEphemeral = frame.Ephemeral;
        if ("LocalInteractionQuery" in frame) this.receiveLocalInteractionQuery(frame.LocalInteractionQuery.reply);
        else if ("OperationCompleted" in frame) this.publishOperationCompletion(frame.OperationCompleted);
        else if ("Invocation" in frame && frame.Invocation.in_reply_to === 0 && (frame.Invocation.ui_scope.length > 0 || frame.Invocation.history_patch.length > 0)) this.publishOperationProgress(frame.Invocation);
        else ordinary.push(frame);
      }
      const correlated = new Set(ordinary.flatMap((frame) => { const sequence = appChannelReplySequence(frame); return sequence === null ? [] : [sequence]; }));
      for (let index = 0; index < this.pending.length;) {
        const waiter = this.pending[index]!;
        if (!correlated.has(waiter.seq)) { index += 1; continue; }
        this.pending.splice(index, 1);
        const reply = ordinary.filter((frame) => appChannelFrameBelongsTo(frame, waiter.seq, waiter.transaction));
        this.captureDocumentFrames(reply);
        waiter.resolve(reply);
      }
      this.finishDisposal();
    }
  }

  /** 👥️ This instance's last published presence pack, interaction slice and tool-run and history-edit summaries (each
   * summary empty while none is open), `null` until the guest published one. */
  ephemeral(): { readonly presence: readonly number[]; readonly presenceGeneration: number; readonly transientGeneration: number; readonly interaction: readonly number[]; readonly toolRun: readonly number[]; readonly historyEdit: readonly number[] } | null {
    const frame = this.lastEphemeral;
    return frame === null ? null : { presence: frame.presence, presenceGeneration: frame.presence_generation, transientGeneration: frame.transient_generation, interaction: frame.interaction, toolRun: frame.tool_run, historyEdit: frame.history_edit };
  }

  /** 🏁️ Subscribes to this instance's unsolicited typed-operation completions. A mounted operation
   * finishes turns after the command that started it already resolved, so its terminal
   * `AppFrame::OperationCompleted` matches no pending waiter and would otherwise be dropped — this is
   * the ONLY delivery path for it. Returns the unsubscribe. */
  onOperationCompleted(listener: (completion: OperationCompletionV1) => void): () => void {
    this.completionListeners.add(listener);
    return () => this.completionListeners.delete(listener);
  }

  /** 🎞️ Subscribes to this instance's unsolicited progress: the `UiDirtyScope` a running operation (a tool run's trace,
   * provisional pieces and progress) and the `HistoryPatch` a running history replay (its stage and `done/total`) hand
   * the shell mid-operation as an `Invocation` frame answering no command. It is best effort — the shell refreshes as
   * often as it keeps up with — and the ONLY delivery path for either once the operation runs on the background drain.
   * Returns the unsubscribe. */
  onOperationProgress(listener: (progress: AppChannelOperationProgressV1) => void): () => void {
    this.progressListeners.add(listener);
    return () => this.progressListeners.delete(listener);
  }

  private publishOperationProgress(frame: { readonly ui_scope: readonly number[]; readonly history_patch: readonly number[] }): void {
    if (this.disposed || this.progressListeners.size === 0) return;
    const progress: AppChannelOperationProgressV1 = {
      ...(frame.ui_scope.length > 0 ? { uiScope: decodePackWire(new Uint8Array(frame.ui_scope), "$.uiScope") } : {}),
      ...(frame.history_patch.length > 0 ? { historyPatch: decodePackWire(new Uint8Array(frame.history_patch), "$.historyPatch") } : {}),
    };
    for (const listener of [...this.progressListeners]) {
      try { listener(progress); }
      catch (error) { console.error("[TRACE] operation progress subscriber failed", error); }
    }
  }

  /** 🏁️ Decodes one completion frame's pack-encoded carriers and fans it out to every subscriber.
   * A throwing subscriber never starves its siblings nor the outcome pump. */
  private publishOperationCompletion(frame: Extract<AppFrameValue, { readonly OperationCompleted: unknown }>["OperationCompleted"]): void {
    if (this.disposed || this.completionListeners.size === 0) return;
    const completion: OperationCompletionV1 = {
      instanceId: this.instanceId,
      operation: frame.operation,
      revision: frame.revision,
      uiScope: frame.ui_scope.length ? decodePackWire(new Uint8Array(frame.ui_scope), "$.uiScope") : undefined,
      historyPatch: frame.history_patch.length ? decodePackWire(new Uint8Array(frame.history_patch), "$.historyPatch") : undefined,
    };
    for (const listener of [...this.completionListeners]) {
      try { listener(completion); }
      catch (error) { console.error("[TRACE] operation completion subscriber failed", error); }
    }
  }

  /** 🪦️ Closes this instance's COMMAND QUEUE and settles everything already in it, WITHOUT ending the
   * background {@link pumpOutcomes} subscription — the first half of `destroyApp`
   * (`PluginRuntime/🟦️.tsx`), run before the actor's close ladder rather than after it.
   *
   * The ladder runs on the actor's own lifecycle lease and never speaks through this channel, so while
   * it ran the queue stayed open: a gesture dispatched into a dying instance was admitted here and only
   * refused one layer down by `requireActorId`, as `no actor for instance N` plus a stack. Retiring the
   * queue first turns every such arrival into this channel's own typed terminal. The subscription is
   * deliberately NOT ended here — {@link dispose} still owns that, so a close the native side REFUSES
   * leaves the iterator exactly where it was and the retry can still retire it. */
  retire(): void {
    if (this.retired) return;
    this.retired = true;
    this.completionListeners.clear();
    this.cachedPack = null;
    this.cachedSpr = null;
    for (let index = this.pending.length - 1; index >= 0; index -= 1) {
      if (!this.pending[index]!.queryReceipt) this.pending.splice(index, 1)[0]!.reject(new Error("app-channel.disposed"));
    }
    if (this.localQuery) this.cancelLocalInteractionQuery(new Error("local-interaction.disposed"));
  }

  /** 🔌️ Ends this client's background {@link pumpOutcomes} subscription — call once from
   * `destroyApp` (`PluginRuntime/🟦️.tsx`) so a torn-down instance doesn't leak a live
   * subscriber against the handle-wide outcome stream for the rest of the handle's lifetime. */
  dispose(): void {
    this.retire();
    this.disposed = true;
    this.finishDisposal();
  }

  private finishDisposal(): void {
    if (this.disposed && this.localQuery === null && this.pending.length === 0) void this.outcomeIterator.return?.();
  }

  private nextSeq(): number {
    return this.sequenceOwner.nextSequence();
  }

  /** 📦️ Scans every frame one sent command's outcome carried for `AppFrame::Document` and refreshes
   * the pack cache — the "every `AppFrame::Document` reply" half of the cache-population contract. */
  private captureDocumentFrames(frames: readonly AppFrameValue[]): void {
    if (this.disposed || frames.some(frame => "Error" in frame)) return;
    for (const frame of frames) {
      if ("Document" in frame) {
        this.cachedPack = new Uint8Array(frame.Document.pack);
        this.cachedSpr = new Uint8Array(frame.Document.spr);
      }
    }
  }

  /** 📦️ The cached `{pack, spr}` for this instance's document, or `null` before any
   * ready {@link loadDocumentArchive} or `AppFrame::Document` reply has been observed. Surfaced to the
   * transaction coordinator through the `PluginWasmHandle` adapter's own `documentPack` accessor
   * (`PluginRuntime/🟦️.tsx`) — a contributor plan call needs the target's current snapshot
   * pack, and this is the only place that snapshot is retained host-side. */
  documentPack(): { readonly pack: Uint8Array; readonly spr: Uint8Array } | null {
    return this.cachedPack && this.cachedSpr ? { pack: this.cachedPack.slice(), spr: this.cachedSpr.slice() } : null;
  }

  /** 🔀️ Queues one encoded command and resolves with every frame its matching {@link TurnOutcome}
   * carries — see this class's own header doc for how the reply gets correlated back to this call.
   * `dispatch` ({@link PluginDispatchHintV1}) is forwarded verbatim to {@link PluginWasmHandle.enqueue};
   * only {@link command} ever passes one — `configure`/`readDocument`/… are maintenance and stay
   * unordered (arrival order). */
  private sendCommand(command: AppCommandValue, dispatch?: PluginDispatchHintV1): Promise<AppFrameValue[]> {
    if (this.disposed || this.retired) return Promise.reject(new Error("app-channel.disposed"));
    return new Promise<AppFrameValue[]>((resolve, reject) => {
      const wireSequence = Object.values(command)[0]!.seq;
      const seq = Number(wireSequence);
      if (!Number.isSafeInteger(seq) || seq < 0) { reject(new Error("app-channel.invalid-sequence-owner")); return; }
      const waiter = { seq, queryReceipt: false, transaction: appChannelTransactionReply(command), resolve, reject };
      this.pending.push(waiter);
      try { this.handle.enqueue(this.instanceId, [encodeAppCommand(command)], dispatch); }
      catch (error) {
        const index = this.pending.indexOf(waiter);
        if (index !== -1) this.pending.splice(index, 1);
        reject(error);
      }
    });
  }

  //#region 🏠️LocalInteractionQuery
  /** 📃️ Each page remains native-owned until its consumer resolves; completion waits for exact native root retirement. */
  readLocalInteractionPages(consume: (page: LocalInteractionPage) => Promise<void>, signal?: AbortSignal): Promise<LocalInteractionIdentity> {
    if (this.disposed || this.retired) return Promise.reject(new Error("app-channel.disposed"));
    if (this.localQuery) return Promise.reject(new Error("local-interaction.busy"));
    if (signal?.aborted) return Promise.reject(new Error("local-interaction.cancelled"));
    let admission: ReturnType<AppChannelRequestSequence["nextQuery"]>;
    try { admission = this.sequenceOwner.nextQuery(); }
    catch (error) { return Promise.reject(error); }
    const requestId = admission.request;
    return new Promise((resolve, reject) => {
      const abort = () => this.cancelLocalInteractionQuery(new Error("local-interaction.cancelled"));
      this.localQuery = { requestId, cancelSequence: admission.cancelSequence, consume, resolve, reject, signal, abort, token: null, nextOrdinal: 0n, consuming: false, cancelled: false, terminalConsumed: false, failure: null };
      signal?.addEventListener("abort", abort, { once: true });
      this.sendLocalInteractionQuery(admission.sequence, { kind: "read", requestId });
    });
  }

  private sendLocalInteractionQuery(seq: number, command: LocalInteractionQueryCommand): void {
    this.pending.push({ seq, queryReceipt: true, transaction: null, resolve: () => {}, reject: (error: unknown) => this.finishLocalInteractionQuery(error) });
    try { this.handle.enqueue(this.instanceId, [encodeAppCommand({ LocalInteractionQuery: { seq, command } })]); }
    catch (error) {
      const index = this.pending.findIndex((waiter) => waiter.seq === seq);
      if (index !== -1) this.pending.splice(index, 1);
      this.finishLocalInteractionQuery(error);
    }
  }

  private cancelLocalInteractionQuery(error: unknown): void {
    const query = this.localQuery;
    if (!query || query.cancelled) return;
    query.cancelled = true;
    query.failure = error;
    if (query.token) this.sendLocalInteractionQuery(query.cancelSequence, { kind: "cancel", token: query.token });
  }

  private finishLocalInteractionQuery(error: unknown = null): void {
    const query = this.localQuery;
    if (!query) return;
    this.localQuery = null;
    for (let index = this.pending.length - 1; index >= 0; index -= 1) {
      if (this.pending[index]!.queryReceipt) this.pending.splice(index, 1);
    }
    query.signal?.removeEventListener("abort", query.abort);
    const failure = query.failure ?? error;
    if (failure !== null) query.reject(failure);
    else if (query.token && query.terminalConsumed) query.resolve(query.token.identity);
    else query.reject(new Error("local-interaction.incomplete-close"));
    this.finishDisposal();
  }

  private receiveLocalInteractionQuery(reply: LocalInteractionQueryReply): void {
    const query = this.localQuery;
    if (!query) return;
    if (reply.kind === "rejected") {
      if (reply.requestId === query.requestId) this.finishLocalInteractionQuery(new Error(`local-interaction.${reply.code}`));
      return;
    }
    if (reply.kind === "started") {
      if (reply.token.requestId !== query.requestId || reply.token.identity.appInstanceId !== this.instanceId || query.token !== null) return;
      query.token = reply.token;
      if (query.cancelled) this.sendLocalInteractionQuery(query.cancelSequence, { kind: "cancel", token: query.token });
      return;
    }
    const token = reply.kind === "page" ? reply.page : reply.token;
    if (!query.token || !sameLocalInteractionQuery(query.token, token)) return;
    if (reply.kind === "closed") {
      if (!query.cancelled && (!query.terminalConsumed || token.ordinal !== query.token.ordinal)) return;
      this.finishLocalInteractionQuery(reply.cancelled ? new Error("local-interaction.cancelled") : null);
      return;
    }
    if (query.cancelled || query.consuming || token.ordinal !== query.nextOrdinal.toString()) return;
    query.token = { requestId: token.requestId, queryGeneration: token.queryGeneration, identity: token.identity, ordinal: token.ordinal };
    query.consuming = true;
    void Promise.resolve().then(() => query.consume(reply.page)).then(() => {
      if (this.localQuery !== query) return;
      query.consuming = false;
      if (query.cancelled) return;
      query.terminalConsumed = reply.page.terminal;
      query.nextOrdinal += 1n;
      let seq: number;
      try { seq = this.nextSeq(); }
      catch (error) { this.cancelLocalInteractionQuery(error); return; }
      this.sendLocalInteractionQuery(seq, { kind: "acknowledge", token: query.token! });
    }, (error: unknown) => {
      if (this.localQuery !== query) return;
      query.consuming = false;
      this.cancelLocalInteractionQuery(error);
    });
  }
  //#endregion 🏠️LocalInteractionQuery

  /** 🎛️ Forwards one opaque app-specific command (already encoded by the caller's own command
   * grammar) plus the current view state; may return several frames (`Invocation` + any dirtied
   * `UiPatch`es) — routing them is the caller's job. `dispatch` ({@link PluginDispatchHintV1}) carries
   * the input ledger's causal `order` for this one command; it steers only the command's position in
   * the actor's queue, never its content. */
  async command(commandBytes: Uint8Array, viewState: unknown, dispatch?: PluginDispatchHintV1): Promise<AppFrameValue[]> {
    return this.sendCommand({
      Command: { seq: this.nextSeq(), command: Array.from(commandBytes), view_state: Array.from(encodePackValue(viewContextWireValue(viewState))) },
    }, dispatch);
  }

  async configure(config: unknown): Promise<AppFrameValue[]> {
    return this.sendCommand({ ConfigCommand: { seq: this.nextSeq(), command: Array.from(encodePackValue(config)) } });
  }

  async readDocument(): Promise<AppFrameValue[]> {
    return this.sendCommand({ ReadDocument: { seq: this.nextSeq() } });
  }

  /** 🪪️ Reads the actor's trusted document envelope identity without exporting its bytes. */
  async readDocumentIdentity(): Promise<AppDocumentIdentity> {
    const seq = BigInt(this.nextSeq());
    const frames = await this.sendCommand({ ReadDocumentIdentity: { seq } });
    const fault = frames.find((frame) => "Error" in frame);
    if (fault && "Error" in fault) throw new Error(`document identity: ${faultDisplayMessage(fault.Error.fault, decodePackValue)}`);
    const replies = frames.filter((frame): frame is Extract<AppFrameValue, { readonly DocumentIdentity: unknown }> => "DocumentIdentity" in frame && frame.DocumentIdentity.in_reply_to === seq);
    if (replies.length !== 1) throw new Error("document identity: missing or ambiguous reply");
    const identity = replies[0]!.DocumentIdentity.identity;
    assertAppDocumentIdentity(identity);
    if (identity.app_instance_id !== this.instanceId) throw new Error("document identity: foreign instance");
    return identity;
  }

  /** 🪆️ Reads every owned child's CURRENT head snapshot pack (`AppCommand::ReadChildHeads` → `AppFrame::ChildHeads`), for a
   * reader that composes parent + children on read (design §20.15). A guest fault rejects with the guest's own fault text. */
  async readChildHeads(): Promise<readonly ChildHeadPackEntry[]> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({ ReadChildHeads: { seq } });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.readChildHeads(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    const heads = frames.find((frame): frame is Extract<AppFrameValue, { readonly ChildHeads: unknown }> => "ChildHeads" in frame && frame.ChildHeads.in_reply_to === seq);
    if (!heads) throw new Error(`AppChannelClient.readChildHeads(${this.appId}): missing ChildHeads frame for seq ${seq}`);
    return heads.ChildHeads.entries;
  }

  /** 📤️ Produces one OUT port's current bytes (`AppCommand::MediaOut` → `AppFrame::Media`) — the
   * agent-facing export lane, which returns the guest's own bytes to the caller instead of handing
   * the human a download. Deliberately the SAME command the headless gateway drives, so an export
   * over the live-shell route and one over the headless route ask the guest the identical question. */
  async mediaOut(port: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ MediaOut: { seq: this.nextSeq(), port, request: [] } });
  }

  /** 📂️ Delivers one typed media artifact to a fresh or existing app instance. */
  async mediaIn(port: string, descriptor: unknown, data: Uint8Array): Promise<AppFrameValue[]> {
    return this.sendCommand({ MediaIn: { seq: this.nextSeq(), port, descriptor: Array.from(encodePackValue(descriptor)), data: Array.from(data) } });
  }

  /** 🎬️ Admits media work only against the caller's exact document and revision. */
  async submitMediaExport(port: string, parentDocumentId: string, revision: bigint): Promise<AppFrameValue[]> {
    return this.sendCommand({ SubmitMediaExport: { seq: BigInt(this.nextSeq()), port, expected_parent_document_id: parentDocumentId, expected_base_revision: revision } });
  }

  /** ⏱️ Advances one bounded media-export slice. */
  async pollMediaExport(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.sendCommand({ PollMediaExport: { seq: BigInt(this.nextSeq()), handle } });
  }

  /** 🛑️ Cancels exactly the submitted media generation. */
  async cancelMediaExport(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.sendCommand({ CancelMediaExport: { seq: BigInt(this.nextSeq()), handle } });
  }

  /** 📦️ Takes one bounded output chunk and observes terminal owner retirement. */
  async takeMediaExportChunk(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.sendCommand({ TakeMediaExportChunk: { seq: BigInt(this.nextSeq()), handle } });
  }

  /** 🗃️ Restores one root envelope and its complete recursive owned-member closure atomically, stepped through
   * {@link DocumentArchiveLoadHost}: `progress` hears every polled status, `signal` cancels. A cancel before admission sends
   * nothing; after it the guest restores the previous document, and the load rejects with the signal's reason — or an
   * `AbortError` when the guest cancelled on its own (a person's Cancel in the history body). A cancel that lost the race
   * resolves, because the document did load. A refusal or a guest fault rejects with the guest's own fault text. Driven by
   * {@link driveDocumentArchive}, like a merge. */
  async loadDocumentArchive(
    archive: DocumentArchivePack,
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<void> {
    const root = { pack: Uint8Array.from(archive.parent_pack), spr: Uint8Array.from(archive.parent_spr) };
    await this.driveDocumentArchive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, progress);
    this.cachedPack = root.pack;
    this.cachedSpr = root.spr;
  }

  /** 🔀️ Merges a read-back of the document this program already shows (design §22.22), stepped through
   * {@link DocumentArchiveLoadHost.merging}: the archive's events join the program's log, nothing is replaced, an open history
   * edit and the program's viewed alternative stay. Resolves `merged` with the events it took and the events the program is
   * `ahead` of the archive by — the cached root pair is dropped when it took any — or `unmergeable` with the guest's code
   * ({@link DOCUMENT_ARCHIVE_UNMERGEABLE_CODES}), which touched nothing. Cancels, refusals and every other fault reject like
   * {@link loadDocumentArchive}'s. */
  async mergeDocumentArchive(
    archive: DocumentArchivePack,
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<DocumentArchiveMergeV1> {
    const merge = DocumentArchiveLoadHost.merging(archive);
    const ended = await this.driveDocumentArchive("mergeDocumentArchive", merge, DOCUMENT_ARCHIVE_UNMERGEABLE_CODES, signal, progress);
    if ("code" in ended) return { kind: "unmergeable", code: ended.code };
    if (ended.completed > 0) {
      this.cachedPack = null;
      this.cachedSpr = null;
    }
    return { kind: "merged", merged: ended.completed, ahead: merge.ahead };
  }

  /** 🚶️ Drives one whole-document load or merge to its acknowledged end: resolves with the count its ready terminal completed,
   * or with the guest's fault code when it is one of `unmergeable`. A cancelled operation rejects with the signal's reason
   * (an `AbortError` when the guest cancelled on its own); a refusal and every other fault reject with a
   * {@link DocumentArchiveFaultError} carrying the guest's own fault, a reply that answers nothing with a plain error. */
  private async driveDocumentArchive(
    name: string,
    host: DocumentArchiveLoadHost,
    unmergeable: readonly string[],
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<{ readonly completed: number } | { readonly code: string }> {
    let completed = 0;
    for (;;) {
      if (signal?.aborted) host.requestCancel();
      const step = host.step(() => this.nextSeq());
      if (step.kind === "finished") {
        if (step.outcome.kind === "ready") return { completed };
        if (step.outcome.kind === "cancelled") throw signal?.reason ?? new DOMException(`AppChannelClient.${name}(${this.appId}): cancelled`, "AbortError");
        const code = decodeFaultFromWire(step.outcome.fault, decodePackValue)?.code;
        if (typeof code === "string" && unmergeable.includes(code)) return { code };
        throw new DocumentArchiveFaultError(`AppChannelClient.${name}(${this.appId}): ${faultDisplayMessage(step.outcome.fault, decodePackValue)}`, decodeFaultFromWire(step.outcome.fault, decodePackValue));
      }
      const seq = step.seq;
      const replyFrames = await this.sendCommand(step.command);
      const frame = documentArchiveLoadReplyV1(replyFrames, seq);
      const answer: DocumentArchiveLoadAnswer = frame ? host.answer(seq, frame) : { kind: "unanswered" };
      if (answer.kind === "refused") throw new DocumentArchiveFaultError(`AppChannelClient.${name}(${this.appId}): ${faultDisplayMessage(answer.fault, decodePackValue)}`, decodeFaultFromWire(answer.fault, decodePackValue));
      if (answer.kind === "unanswered") throw new Error(`AppChannelClient.${name}(${this.appId}): no answer to seq ${seq} of operation ${host.operation}`);
      if (answer.status) {
        completed = answer.status.completed;
        progress?.(answer.status);
      }
      if (answer.status ? answer.status.state === "pending" || answer.status.state === "running" : frame !== undefined && "Error" in frame) await nextArchivePollTurnV1();
    }
  }

  /** 🗃️ Reads one generation-fenced root envelope and its complete recursive owned-member closure. */
  async readDocumentArchive(): Promise<DocumentArchivePack> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({ ReadDocumentArchive: { seq } });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.readDocumentArchive(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    const response = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly DocumentArchive: unknown }> =>
        "DocumentArchive" in frame && frame.DocumentArchive.in_reply_to === seq,
    );
    if (!response) throw new Error(`AppChannelClient.readDocumentArchive(${this.appId}): missing DocumentArchive frame for seq ${seq}`);
    this.cachedPack = Uint8Array.from(response.DocumentArchive.archive.parent_pack);
    this.cachedSpr = Uint8Array.from(response.DocumentArchive.archive.parent_spr);
    return {
      parent_pack: [...response.DocumentArchive.archive.parent_pack],
      parent_spr: [...response.DocumentArchive.archive.parent_spr],
      members: response.DocumentArchive.archive.members.map((entry) => ({
        ordinal: entry.ordinal,
        reference: { ...entry.reference },
        owner: { parent: { ...entry.owner.parent }, slot: entry.owner.slot, child_id: entry.owner.child_id },
        envelope_pack: [...entry.envelope_pack],
      })),
    };
  }

  /** 🪟️ Restores one exact concrete window's persisted-local config envelope. */
  async loadWindowConfig(entry: WindowConfigPackEntry): Promise<void> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({
      LoadWindowConfig: {
        seq,
        entry: { window_id: entry.window_id, window_kind_id: entry.window_kind_id, envelope_pack: Array.from(entry.envelope_pack) },
      },
    });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.loadWindowConfig(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    if (!frames.some((frame) => "Done" in frame && frame.Done.in_reply_to === seq)) {
      throw new Error(`AppChannelClient.loadWindowConfig(${this.appId}): missing Done frame for seq ${seq}`);
    }
  }

  /** 🪟️ Reads every exact concrete window config envelope for persisted-local replay. */
  async readWindowConfigs(): Promise<readonly WindowConfigPackEntry[]> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({ ReadWindowConfigs: { seq } });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.readWindowConfigs(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    const configs = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly WindowConfigs: unknown }> =>
        "WindowConfigs" in frame && frame.WindowConfigs.in_reply_to === seq,
    );
    if (!configs) throw new Error(`AppChannelClient.readWindowConfigs(${this.appId}): missing WindowConfigs frame for seq ${seq}`);
    return configs.WindowConfigs.entries.map((entry) => ({
      window_id: entry.window_id,
      window_kind_id: entry.window_kind_id,
      envelope_pack: Array.from(entry.envelope_pack),
    }));
  }

  /** 🧾️ Retrieves the complete history projection for initial load or cursor-gap recovery. */
  async readHistory(): Promise<AppFrameValue[]> {
    return this.sendCommand({ ReadHistory: { seq: this.nextSeq() } });
  }


  /** 📂️ Opens an artifact in its resolved (or explicitly named) viewer/editor surface —
   * `os.open-artifact` (contract-freeze §3 of
   * `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`). Empty
   * `pluginId`/`appId` means "resolve via the `OpeningResolver`". `role` is `0` Viewer, `1`
   * Editor — declaration order of `AppRole` (kernel `🔖️AppRouter` region). */
  async openArtifact(artifactRef: string, role: number, pluginId = "", appId = ""): Promise<AppFrameValue[]> {
    return this.sendCommand({ openArtifact: { seq: this.nextSeq(), artifact_ref: artifactRef, role, plugin_id: pluginId, app_id: appId } });
  }

  /** 🎚️ Pins a viewer/editor default for one `(artifactKind, standard, subset, role)` coordinate,
   * persisted event-sourced in the OS `os.config.opening` facet — `os.set-default-viewer`/
   * `os.set-default-editor`. */
  async setDefaultApp(artifactKind: string, standard: string, subset: string, role: number, pluginId: string, appId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ setDefaultApp: { seq: this.nextSeq(), artifact_kind: artifactKind, standard, subset, role, plugin_id: pluginId, app_id: appId } });
  }

  /** 🎚️ Clears a previously pinned default, falling back to the `OpeningResolver`'s owner/router
   * order — `os.clear-default-app`. */
  async clearDefaultApp(artifactKind: string, standard: string, subset: string, role: number): Promise<AppFrameValue[]> {
    return this.sendCommand({ clearDefaultApp: { seq: this.nextSeq(), artifact_kind: artifactKind, standard, subset, role } });
  }

  /** 🖱️ On-demand context menu — one `ContextMenu` reply whose `in_reply_to` matches this call's `seq`. */
  async contextMenu(request: unknown): Promise<unknown> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({
      ContextMenu: { seq, request: Array.from(encodePackValue(request)) },
    });
    const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (errorFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    }
    const menuFrame = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly ContextMenu: { readonly in_reply_to: number; readonly items: readonly number[] } }> =>
        "ContextMenu" in frame && frame.ContextMenu.in_reply_to === seq,
    );
    if (!menuFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): missing ContextMenu frame for seq ${seq}`);
    }
    return decodePackValue(new Uint8Array(menuFrame.ContextMenu.items));
  }

  /** 📥️ Force-applies remote `MutationEnvelope`s through `AppCommand::ApplyEnvelopes`. */
  async applyEnvelopes(envelopes: readonly MutationEnvelope[]): Promise<AppFrameValue[]> {
    return this.sendCommand({ ApplyEnvelopes: { seq: this.nextSeq(), envelopes } });
  }

  //#region 🔖️Merge
  /** ⚖️ Sets this instance's local merge-policy authority (`os.set-merge-policy`, C6/C9) — a `Done`
   * reply, never a `MergeReport`/`Conflicts` (those only follow an ingest). */
  async setMergePolicy(policy: MergePolicy): Promise<AppFrameValue[]> {
    return this.sendCommand({ setMergePolicy: { seq: this.nextSeq(), policy: mergePolicyAsU8(policy) } });
  }

  /** ⚔️ Accepts or discards an `Open` {@link Conflict} (`os.resolve-conflict`) — replays it under
   * `LaissezFaire` on `accept` (Quarantined) or acks it in place (Degraded); `discard` on a
   * Quarantined conflict seeds the DAG as already-seen without ever relaying it, on a Degraded
   * conflict it is rejected (never rewrites shared history, C6 §`resolve_conflict`). Returns the
   * authoritative `MergeReport` + `Conflicts` frames. */
  async resolveConflict(conflictId: string, resolution: ConflictResolution): Promise<AppFrameValue[]> {
    return this.sendCommand({ resolveConflict: { seq: this.nextSeq(), conflict_id: conflictId, resolution: conflictResolutionAsU8(resolution) } });
  }

  /** 📖️ Reads the open-conflict projection (`os.read-conflicts`) — one `Conflicts` reply frame. */
  async readConflicts(): Promise<AppFrameValue[]> {
    return this.sendCommand({ readConflicts: { seq: this.nextSeq() } });
  }
  //#endregion 🔖️Merge

  //#region 🔖️Presence
  /** 👥️ Pushes the document-wide presence roster into this instance's plugin app — the ONLY plugin
   * ingress for peers (contract-freeze §C7.6). Encodes each {@link ArtifactPresencePeer} via
   * {@link encodePresencePeer}; the caller has already dropped its own actor from `peers` before
   * calling this (`ownColor` carries this actor's own hub-assigned palette index separately, `null`
   * for a folder-only session with no hub). A plain `Done` reply, never decoded further here. */
  async pushPresence(ownColor: number | null, peers: readonly ArtifactPresencePeer[]): Promise<AppFrameValue[]> {
    return this.sendCommand({ presence: { seq: this.nextSeq(), own_color: ownColor, peers: peers.map((peer) => encodePresencePeer(peer)) } });
  }
  //#endregion 🔖️Presence

  //#region 🔖️Transaction
  /** 🎫️ `TransactionPrepare`, owner-mutation wire form (contract freeze §2/§5.3): `mutationId` +
   * `payload` set, `preparedOps` empty. Sent when `ArtifactMutationRouter` resolves the mutation to
   * its OWNING plugin. */
  async transactionPrepareOwner(txnId: string, mutationId: string, payload: Uint8Array): Promise<AppFrameValue[]> {
    return this.sendCommand({
      transactionPrepare: { seq: this.nextSeq(), txn_id: txnId, mutation_id: mutationId, payload: Array.from(payload), prepared_ops: [], origin: [], prepared_child_ops: [] },
    });
  }

  /** 🎫️ `TransactionPrepare`, pre-planned wire form: `preparedOps`/`origin` set, `mutationId`
   * empty. Sent to a CONTRIBUTED-mutation target (after the host has already called the contributor's
   * `contributor.artifact-mutation-plan`) or to any member the coordinator is re-batching several
   * already-known ops onto in one call — see `PluginRuntime/🟦️.tsx`'s `TransactionCoordinator`. */
  async transactionPreparePlanned(txnId: string, preparedOps: readonly Uint8Array[], origin: Uint8Array): Promise<AppFrameValue[]> {
    return this.sendCommand({
      transactionPrepare: {
        seq: this.nextSeq(),
        txn_id: txnId,
        mutation_id: "",
        payload: [],
        prepared_ops: preparedOps.map((op) => Array.from(op)),
        origin: Array.from(origin),
        prepared_child_ops: [],
      },
    });
  }

  async transactionCommit(txnId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionCommit: { seq: this.nextSeq(), txn_id: txnId } });
  }

  async transactionRollback(txnId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionRollback: { seq: this.nextSeq(), txn_id: txnId } });
  }

  /** 🎁️ Group undo — fans out to every member of `groupId` (contract freeze §5.7); this call is one
   * member's half, the coordinator drives the fan-out. */
  async transactionUndo(groupId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionUndo: { seq: this.nextSeq(), group_id: groupId } });
  }

  async transactionRedo(groupId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionRedo: { seq: this.nextSeq(), group_id: groupId } });
  }
  //#endregion 🔖️Transaction
}
//#endregion 🔖️AppChannelClient
