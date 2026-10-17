/** 🏷️ App channel codec primitives: the media export handle wire, the exact media export frame, the document identity assertion and the command and frame tag tables. */

import { readStr, readVarintU64, readVarintU64Exact, writeStr, writeVarintU64, writeVarintU64Exact } from "@semio-tech/framework-replication";
import { type AppDocumentIdentity, type MediaExportHandle } from "../../🪢️wire-contracts/🟦️.ts";

//#region 🔖️AppChannelCodec
//#region 🔖️Codec
/** 🪪️ Serializes media ownership without rounding its u64 revision lanes. */
export function writeMediaExportHandle(out: number[], handle: MediaExportHandle): void {
  if (!Number.isInteger(handle.app_instance_id) || handle.app_instance_id < 0 || handle.app_instance_id > 0xffffffff) throw new Error("media export: invalid app instance");
  writeVarintU64(out, handle.app_instance_id);
  writeStr(out, handle.parent_document_id);
  writeVarintU64Exact(out, handle.operation_id);
  writeVarintU64Exact(out, handle.base_revision);
  writeVarintU64Exact(out, handle.generation);
}

/** 🎫️ Reads the exact media authority shared with the Rust channel. */
export function readMediaExportHandle(bytes: Uint8Array, pos: [number]): MediaExportHandle {
  const app_instance_id = readVarintU64(bytes, pos);
  if (app_instance_id > 0xffffffff) throw new Error("media export: invalid app instance");
  return { app_instance_id, parent_document_id: readStr(bytes, pos), operation_id: readVarintU64Exact(bytes, pos), base_revision: readVarintU64Exact(bytes, pos), generation: readVarintU64Exact(bytes, pos) };
}

/** 📏️ Media messages consume their complete declared frame. */
export function exactMediaExportFrame<T>(bytes: Uint8Array, pos: [number], value: T): T {
  if (pos[0] !== bytes.length) throw new Error("media export: trailing frame bytes");
  return value;
}

export const MEDIA_EXPORT_STATES = ["running", "complete", "cancelled", "failed"] as const;

export function assertAppDocumentIdentity(identity: AppDocumentIdentity): void {
  if (!Number.isSafeInteger(identity.app_instance_id) || identity.app_instance_id < 0 || identity.app_instance_id > 0xffff_ffff || (identity.parent_document_id !== null && (typeof identity.parent_document_id !== "string" || identity.parent_document_id.length === 0 || [...identity.parent_document_id].length > 512))) throw new Error("document identity: invalid owner");
}

export const APP_COMMAND_TAGS = {
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
export const APP_FRAME_TAGS = {
  Done: 0, Invocation: 1, DocumentChanged: 2, Document: 3,
  Config: 4, ConfigChanged: 5, ContextMenu: 6, Media: 7, MediaFingerprint: 8, Error: 9, Emit: 10, Draft: 11, Children: 12, Ephemeral: 13, HistorySnapshot: 14,
  transactionProposal: 15, transactionPrepared: 16, transactionCommitted: 17, transactionRolledBack: 18,
  MergeReport: 19, Conflicts: 20, UiPatch: 21, UiSnapshotEnd: 22, LocalInteractionQuery: 23, WindowConfigs: 24, OperationCompleted: 25, DocumentArchive: 26, DocumentArchiveLoad: 27,
  MediaExportSubmitted: 28, MediaExportStatus: 29, MediaExportChunk: 30, DocumentIdentity: 31, ChildHeads: 32,
} as const;
//#endregion 🔖️Codec
//#endregion 🔖️AppChannelCodec
