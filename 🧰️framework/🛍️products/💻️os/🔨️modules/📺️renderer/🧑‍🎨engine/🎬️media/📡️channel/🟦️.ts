import { decodePackValue, faultDisplayMessage, type AppChannelClient, type AppFrameValue, type MediaExportHandle } from "../../../../../🟦️.ts";
import { assertMediaExportAuthority, type MediaTransportPort } from "../🚚️lifecycle/🟦️.ts";

function reply<T>(frames: readonly AppFrameValue[], select: (frame: AppFrameValue) => T | undefined): T {
  const fault = frames.find((frame) => "Error" in frame);
  if (fault && "Error" in fault) throw new Error(`media-export.failed: ${faultDisplayMessage(fault.Error.fault, decodePackValue)}`);
  const replies = frames.map(select).filter((value): value is T => value !== undefined);
  if (replies.length !== 1) throw new Error(replies.length === 0 ? "media-export.missing-reply" : "media-export.ambiguous-reply");
  return replies[0]!;
}

/** 📡️ Adapts the canonical channel replies for every renderer's media runner. */
export function mediaTransportPort(requireChannel: (instanceId: number) => Pick<AppChannelClient, "submitMediaExport" | "pollMediaExport" | "cancelMediaExport" | "takeMediaExportChunk">): MediaTransportPort {
  return {
    submitMediaExport: async (instanceId, port, parentDocumentId, revision) => {
      const result = reply(await requireChannel(instanceId).submitMediaExport(port, parentDocumentId, revision), (frame) => "MediaExportSubmitted" in frame ? frame.MediaExportSubmitted : undefined);
      assertMediaExportAuthority(result.handle, { app_instance_id: instanceId, parent_document_id: parentDocumentId, base_revision: revision });
      return result.handle;
    },
    pollMediaExport: async (instanceId, authority) => {
      assertMediaExportAuthority(authority, { app_instance_id: instanceId });
      const { in_reply_to, ...status } = reply(await requireChannel(instanceId).pollMediaExport(authority), (frame) => "MediaExportStatus" in frame ? frame.MediaExportStatus : undefined);
      assertMediaExportAuthority(status.handle, authority);
      return status;
    },
    cancelMediaExport: async (instanceId, authority) => {
      assertMediaExportAuthority(authority, { app_instance_id: instanceId });
      reply(await requireChannel(instanceId).cancelMediaExport(authority), (frame) => "Done" in frame ? frame.Done : undefined);
    },
    takeMediaExportChunk: async (instanceId, authority) => {
      assertMediaExportAuthority(authority, { app_instance_id: instanceId });
      const result = reply(await requireChannel(instanceId).takeMediaExportChunk(authority), (frame) => "MediaExportChunk" in frame ? frame.MediaExportChunk : undefined);
      assertMediaExportAuthority(result.handle, authority);
      return { handle: result.handle, data: new Uint8Array(result.data), terminal: result.terminal };
    },
  };
}
