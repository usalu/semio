/** 🎛️ App channel command codec: `encodeAppCommand` and `decodeAppCommand` over the tagged app command wire. */

import { decodeLocalInteractionQueryCommand, encodeLocalInteractionQueryCommand, mutationEnvelopeFromWire, mutationEnvelopeToWire, readBytes, readStr, readVarintU64, readVarintU64Exact, readVecBytes, readVecEnvelope, writeBytes, writeStr, writeVarintU64, writeVarintU64Exact, writeVecBytes, writeVecEnvelope } from "@semio-tech/framework-replication";
import { replicationPackCodec } from "../../🎁️packs/🟦️.ts";
import { type AppCommandValue, type MediaExportHandle, readDocumentArchive, readOptU8, readVecChildPackEntry, readWindowConfigPackEntry, writeDocumentArchive, writeOptU8, writeVecChildPackEntry, writeWindowConfigPackEntry } from "../../🪢️wire-contracts/🟦️.ts";
import { APP_COMMAND_TAGS, exactMediaExportFrame, readMediaExportHandle, writeMediaExportHandle } from "../🏷️codec-primitives/🟦️.ts";

//#region 🔖️AppChannelCodec
//#region 🔖️Codec
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
//#endregion 🔖️Codec
//#endregion 🔖️AppChannelCodec
