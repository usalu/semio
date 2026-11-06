import type { MediaExportHandle, MediaExportStatus } from "@semio-tech/framework-os";
import { MEDIA_PLAYBACK_OUTPUT_PORT } from "../../../../🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts";
import { SEGMENTED_DOWNLOAD_CONTRACT } from "../../🧱️elements/📤️SegmentedDownload/🟦️.ts";
import policy from "./🧬️schema/🔣️.json" with { type: "json" };

export const MEDIA_EXPORT_OPERATION_POLICY = policy.const;

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

/** 🪟️ The owner a media host renders for: the app instance and controller that own the window, the window itself and the port its exports travel through. */
export type MediaTransportOwnerV1 = {
  readonly instanceId: number;
  readonly controllerId: string;
  readonly windowId: string;
  readonly port: MediaTransportPort;
  /** ✂️ Dispatches an editor action raised by the transport (a range cut) through the owner app's ordinary action path, so it runs, publishes and undoes through history like any other edit. */
  readonly apply?: (action: string, args: Readonly<Record<string, unknown>>) => void;
};

export type MediaExportOperation = {
  readonly instanceId: number;
  readonly parentDocumentId: string;
  readonly revision: bigint;
  readonly outputPort: string;
  readonly mediaType: string;
  readonly port: MediaTransportPort;
  readonly cancelled: () => boolean;
  readonly progress: (status: MediaExportStatus) => void;
  readonly yieldTurn?: () => Promise<void>;
  readonly now?: () => number;
};

export type MediaTransportOperation = Omit<MediaExportOperation, "outputPort"> & { readonly generation: bigint };

export type MediaTransportBytes = {
  readonly chunks: readonly Uint8Array[];
  readonly mediaType: string;
};

export type MediaExportBytes = MediaTransportBytes & { readonly descriptor: readonly number[] };

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

function exactHandle(handle: MediaExportHandle, operation: MediaExportOperation, generation: bigint | undefined): boolean {
  return handle.app_instance_id === operation.instanceId
    && handle.parent_document_id === operation.parentDocumentId
    && handle.base_revision === operation.revision
    && (generation === undefined || handle.generation === generation);
}

function nextTurn(): Promise<void> {
  return new Promise((resolve) => globalThis.setTimeout(resolve, MEDIA_EXPORT_OPERATION_POLICY.pollIntervalMs));
}

async function cancelRunning(operation: MediaExportOperation, handle: MediaExportHandle): Promise<void> {
  await operation.port.cancelMediaExport(operation.instanceId, handle);
}

async function collectExport(operation: MediaExportOperation, generation?: bigint): Promise<MediaExportBytes | null> {
  const yieldTurn = operation.yieldTurn ?? nextTurn;
  const now = operation.now ?? (() => globalThis.performance.now());
  const started = now();
  let latest = started;
  const checkpoint = (): void => {
    const current = now();
    if (!Number.isFinite(started) || started < 0 || !Number.isFinite(current) || current < latest) throw new Error("media-export.clock");
    latest = current;
    if (current - started >= MEDIA_EXPORT_OPERATION_POLICY.maximumDurationMs) throw new Error("media-export.deadline");
  };
  checkpoint();
  if (operation.cancelled()) return null;
  const handle = await operation.port.submitMediaExport(operation.instanceId, operation.outputPort, operation.parentDocumentId, operation.revision);
  if (!exactHandle(handle, operation, generation)) throw new Error("media-export.owner");
  let status: MediaExportStatus;
  let progress = 0n;
  try {
    checkpoint();
    if (operation.cancelled()) {
      await cancelRunning(operation, handle);
      return null;
    }
    for (;;) {
      checkpoint();
      status = await operation.port.pollMediaExport(operation.instanceId, handle);
      checkpoint();
      if (!sameHandle(status.handle, handle)) throw new Error("media-export.status-owner");
      if (status.state !== "running") break;
      if (status.applied_progress < progress) throw new Error("media-export.progress-regression");
      progress = status.applied_progress;
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
  if (status.descriptor.length === 0 || status.descriptor.length > MEDIA_EXPORT_OPERATION_POLICY.maximumDescriptorBytes) refusal ??= new Error("media-export.descriptor-capacity");
  if (status.total_bytes < 0n || status.total_bytes > BigInt(SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes)) refusal ??= new Error("media-export.capacity");
  const chunks: Uint8Array[] = [];
  let bytes = 0n;
  let drained = false;
  try {
    for (let page = 1; page <= SEGMENTED_DOWNLOAD_CONTRACT.maximumOutstandingChunks; page += 1) {
      checkpoint();
      const chunk = await operation.port.takeMediaExportChunk(operation.instanceId, handle);
      checkpoint();
      if (!sameHandle(chunk.handle, handle)) refusal ??= new Error("media-export.chunk-owner");
      bytes += BigInt(chunk.data.byteLength);
      if (chunk.data.byteLength > SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes || bytes > BigInt(SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes)) refusal ??= new Error("media-export.capacity");
      if (!operation.cancelled() && refusal === null && chunk.data.byteLength > 0) chunks.push(chunk.data);
      if (chunk.terminal) {
        drained = true;
        if (bytes !== status.total_bytes) refusal ??= new Error("media-export.length");
        if (operation.cancelled()) return null;
        if (refusal) throw refusal;
        return { chunks, mediaType: status.mime_type, descriptor: status.descriptor };
      }
      if (chunk.data.byteLength === 0) refusal ??= new Error("media-export.empty-page");
      await yieldTurn();
    }
    throw refusal ?? new Error("media-export.pages");
  } catch (error) {
    if (!drained) await cancelRunning(operation, handle).catch(() => {});
    throw error;
  }
}

/** 📤️ Collects a declared output under one saved document revision and the producer's exact returned handle. */
export function collectMediaExportBytes(operation: MediaExportOperation): Promise<MediaExportBytes | null> {
  return collectExport(operation);
}

/** 🌊 Drives one exact playback generation serially and drains every completed page before returning. */
export function collectMediaTransportBytes(operation: MediaTransportOperation): Promise<MediaTransportBytes | null> {
  return collectExport({ ...operation, outputPort: MEDIA_PLAYBACK_OUTPUT_PORT }, operation.generation);
}
