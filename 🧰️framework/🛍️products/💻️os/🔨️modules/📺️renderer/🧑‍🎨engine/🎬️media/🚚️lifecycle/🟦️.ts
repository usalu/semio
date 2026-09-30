import type { MediaExportHandle, MediaExportStatus } from "@semio-tech/framework-os";
import { MEDIA_PLAYBACK_OUTPUT_PORT } from "../../../../🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts";
import { SEGMENTED_DOWNLOAD_CONTRACT } from "../../🧱️elements/📤️SegmentedDownload/🟦️.ts";

export type MediaExportChunk = {
  readonly handle: MediaExportHandle;
  readonly data: Uint8Array;
  readonly terminal: boolean;
};

export type MediaTransportPort = {
  readonly submitMediaExport: (instanceId: number, port: string, parentDocumentId: string, revision: bigint) => Promise<MediaExportHandle>;
  readonly pollMediaExport: (instanceId: number, handle: MediaExportHandle) => Promise<MediaExportStatus>;
  readonly cancelMediaExport: (instanceId: number, handle: MediaExportHandle) => Promise<void>;
  readonly takeMediaExportChunk: (instanceId: number, handle: MediaExportHandle) => Promise<MediaExportChunk>;
};

export type MediaTransportOperation = {
  readonly instanceId: number;
  readonly parentDocumentId: string;
  readonly revision: bigint;
  readonly generation: bigint;
  readonly mediaType: string;
  readonly port: MediaTransportPort;
  readonly cancelled: () => boolean;
  readonly progress: (status: MediaExportStatus) => void;
  readonly yieldTurn?: () => Promise<void>;
};

export type MediaTransportBytes = {
  readonly chunks: readonly Uint8Array[];
  readonly mediaType: string;
};

/** 🪪️ Verifies every specified field of the media operation authority. */
export function assertMediaExportAuthority(handle: MediaExportHandle, expected: Partial<MediaExportHandle>): void {
  for (const field of Object.keys(expected) as (keyof MediaExportHandle)[]) if (handle[field] !== expected[field]) throw new Error("media-export.authority-mismatch");
}

function sameHandle(left: MediaExportHandle, right: MediaExportHandle): boolean {
  return left.app_instance_id === right.app_instance_id
    && left.parent_document_id === right.parent_document_id
    && left.operation_id === right.operation_id
    && left.base_revision === right.base_revision
    && left.generation === right.generation;
}

function exactHandle(handle: MediaExportHandle, operation: MediaTransportOperation): boolean {
  return handle.app_instance_id === operation.instanceId
    && handle.parent_document_id === operation.parentDocumentId
    && handle.base_revision === operation.revision
    && handle.generation === operation.generation;
}

function nextTurn(): Promise<void> {
  return new Promise((resolve) => globalThis.setTimeout(resolve, 0));
}

async function cancelRunning(operation: MediaTransportOperation, handle: MediaExportHandle): Promise<void> {
  await operation.port.cancelMediaExport(operation.instanceId, handle);
}

/** 🌊 Drives one exact media generation serially and drains every completed page before returning. */
export async function collectMediaTransportBytes(operation: MediaTransportOperation): Promise<MediaTransportBytes | null> {
  const yieldTurn = operation.yieldTurn ?? nextTurn;
  const handle = await operation.port.submitMediaExport(operation.instanceId, MEDIA_PLAYBACK_OUTPUT_PORT, operation.parentDocumentId, operation.revision);
  if (!exactHandle(handle, operation)) throw new Error("media-export.owner");
  let status: MediaExportStatus;
  try {
    for (;;) {
      status = await operation.port.pollMediaExport(operation.instanceId, handle);
      if (!sameHandle(status.handle, handle)) throw new Error("media-export.status-owner");
      if (status.state !== "running") break;
      if (operation.cancelled()) {
        await cancelRunning(operation, handle);
        return null;
      }
      operation.progress(status);
      await yieldTurn();
    }
  } catch (error) {
    await cancelRunning(operation, handle).catch(() => {});
    throw error;
  }
  if (status.state === "cancelled") return null;
  if (status.state === "failed") throw new Error(status.detail || "media-export.failed");

  let refusal: Error | null = null;
  if (status.mime_type !== operation.mediaType) refusal = new Error("media-export.media-type");
  if (status.total_bytes < 0n || status.total_bytes > BigInt(SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes)) refusal ??= new Error("media-export.capacity");
  const chunks: Uint8Array[] = [];
  let bytes = 0n;
  for (let page = 1; page <= SEGMENTED_DOWNLOAD_CONTRACT.maximumOutstandingChunks; page += 1) {
    const chunk = await operation.port.takeMediaExportChunk(operation.instanceId, handle);
    if (!sameHandle(chunk.handle, handle)) refusal ??= new Error("media-export.chunk-owner");
    bytes += BigInt(chunk.data.byteLength);
    if (chunk.data.byteLength > SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes || bytes > BigInt(SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes)) refusal ??= new Error("media-export.capacity");
    if (!operation.cancelled() && refusal === null && chunk.data.byteLength > 0) chunks.push(chunk.data);
    if (chunk.terminal) {
      if (bytes !== status.total_bytes) refusal ??= new Error("media-export.length");
      if (operation.cancelled()) return null;
      if (refusal) throw refusal;
      return { chunks, mediaType: status.mime_type };
    }
    if (chunk.data.byteLength === 0) refusal ??= new Error("media-export.empty-page");
    await yieldTurn();
  }
  throw refusal ?? new Error("media-export.pages");
}
