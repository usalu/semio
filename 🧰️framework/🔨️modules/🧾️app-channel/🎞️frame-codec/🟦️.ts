/** 🎞️ App channel frame codec: `encodeAppFrame` and `decodeAppFrame` over the tagged app frame wire. */

import { decodeLocalInteractionQueryReply, encodeLocalInteractionQueryReply, readBool, readBytes, readStr, readU8, readVarintU64, readVarintU64Exact, readVecBytes, writeBool, writeBytes, writeStr, writeVarintU64, writeVarintU64Exact, writeVecBytes } from "../../📡️replication/📦️packages/🟦️typescript/🟦️.ts";
import { INVOCATION_RESULT_PACK_MAXIMUM_BYTES } from "../../🎁️packs/🟦️.ts";
import { type AppFrameValue, readDocumentArchive, readOptU64, readVecChildHeadPackEntry, readVecChildPackEntry, readVecWindowConfigPackEntry, writeDocumentArchive, writeOptU64, writeVecChildHeadPackEntry, writeVecChildPackEntry, writeVecWindowConfigPackEntry } from "../../🪢️wire-contracts/🟦️.ts";
import { APP_FRAME_TAGS, assertAppDocumentIdentity, exactMediaExportFrame, MEDIA_EXPORT_STATES, readMediaExportHandle, writeMediaExportHandle } from "../🏷️codec-primitives/🟦️.ts";

//#region 🔖️AppChannelCodec
//#region 🔖️Codec
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
    writeVarintU64Exact(out, identity.base_revision);
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
      if (bytes.length > 2077) throw new Error("document identity: frame capacity");
      const in_reply_to = readVarintU64Exact(bytes, pos);
      const app_instance_id = readVarintU64(bytes, pos);
      const presence = readU8(bytes, pos);
      if (presence !== 0 && presence !== 1) throw new Error("document identity: invalid optional document");
      const parent_document_id = presence === 1 ? new TextDecoder("utf-8", { fatal: true }).decode(Uint8Array.from(readBytes(bytes, pos))) : null;
      const base_revision = readVarintU64Exact(bytes, pos);
      const identity = { app_instance_id, parent_document_id, base_revision };
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
