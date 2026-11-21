// #region 🧲️Header
/** 🧵️ Bounded, operation-owned browser download streaming. */
// #endregion 🧲️Header

import { admitSegmentedDownloadChunk, admitSegmentedDownloadOperationId, SEGMENTED_DOWNLOAD_CONTRACT, SEGMENTED_DOWNLOAD_REFUSAL } from "@semio-tech/framework/actor/shard-client";
export { SEGMENTED_DOWNLOAD_CONTRACT, SEGMENTED_DOWNLOAD_REFUSAL } from "@semio-tech/framework/actor/shard-client";

export const SEGMENTED_DOWNLOAD_MARKER_PREFIX = "semio-segmented-handle-v1:";

export type SegmentedDownloadEncoding = "base64" | "identity";

export type SegmentedDownloadSink = {
  readonly write: (chunk: Uint8Array) => Promise<void>;
  readonly close: () => Promise<void>;
  readonly abort: (reason: unknown) => Promise<void>;
};

export type SegmentedDownloadSinkFactory = (filename: string, mimeType: string) => Promise<SegmentedDownloadSink>;

/** 🪪 Parses only the versioned segmented-handle markers emitted by the Rust runtime. */
export function parseSegmentedDownloadMarker(marker: string | undefined): SegmentedDownloadEncoding {
  if (marker === `${SEGMENTED_DOWNLOAD_MARKER_PREFIX}base64`) return "base64";
  if (marker === `${SEGMENTED_DOWNLOAD_MARKER_PREFIX}identity`) return "identity";
  throw new Error("segmented-download-marker-invalid");
}

/** 🔢 Accepts the runtime's canonical positive decimal operation authority without lossy u64 coercion. */
export function parseSegmentedDownloadOperationId(value: string): bigint {
  if (!/^[1-9][0-9]*$/.test(value)) throw new Error("segmented-download-operation-invalid");
  try {
    return admitSegmentedDownloadOperationId(BigInt(value));
  } catch {
    throw new Error("segmented-download-operation-invalid");
  }
}

function segmentedAbortReason(signal: AbortSignal | undefined): unknown {
  if (!signal?.aborted) return undefined;
  return signal.reason ?? new Error("segmented-download-cancelled");
}

function asciiChunk(chunk: Uint8Array): string {
  let value = "";
  for (const byte of chunk) {
    if (byte > 0x7f) throw new Error("segmented-download-base64-invalid");
    value += String.fromCharCode(byte);
  }
  return value;
}

function decodeBase64Block(value: string, final: boolean): Uint8Array {
  const pattern = final ? /^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/ : /^(?:[A-Za-z0-9+/]{4})*$/;
  if (!pattern.test(value)) throw new Error("segmented-download-base64-invalid");
  return Uint8Array.from(atob(value), (character) => character.charCodeAt(0));
}

/** 🌊 Opens a real browser file stream and fails closed where that streaming authority is absent. */
export async function createSegmentedDownloadSink(filename: string, mimeType: string): Promise<SegmentedDownloadSink> {
  const picker = (globalThis as typeof globalThis & {
    showSaveFilePicker?: (options: { readonly suggestedName: string; readonly types: readonly { readonly description: string; readonly accept: Readonly<Record<string, readonly string[]>> }[] }) => Promise<{
      createWritable: () => Promise<{ write: (chunk: Uint8Array) => Promise<void>; close: () => Promise<void>; abort: (reason?: unknown) => Promise<void> }>;
    }>;
  }).showSaveFilePicker;
  if (!picker) throw new Error("segmented-download-streaming-sink-unavailable");
  const file = await picker.call(globalThis, { suggestedName: filename, types: [{ description: filename, accept: { [mimeType]: [`.${filename.split(".").pop() ?? "bin"}`] } }] });
  const writable = await file.createWritable();
  return { write: (chunk) => writable.write(chunk), close: () => writable.close(), abort: (reason) => writable.abort(reason) };
}

/** 🧺 Keeps immutable Blob pages until the anchor receives their ordered representation. */
export function createBufferedDownloadSink(filename: string, mimeType: string, deliver: (filename: string, blob: Blob) => void): SegmentedDownloadSink {
  let blocks: Blob[] | undefined = [];
  return {
    write: async chunk => {
      if (blocks === undefined) throw new Error("segmented-download-sink-retired");
      const bytes = chunk.buffer instanceof ArrayBuffer ? new Uint8Array(chunk.buffer, chunk.byteOffset, chunk.byteLength) : chunk.slice();
      blocks.push(new Blob([bytes]));
    },
    close: async () => {
      if (blocks === undefined) throw new Error("segmented-download-sink-retired");
      const contents = new Blob(blocks, { type: mimeType });
      blocks = undefined;
      deliver(filename, contents);
    },
    abort: async () => {
      blocks = undefined;
    },
  };
}

/** 🌊 Supplies an ordered Blob download without a browser-specific file picker. */
export function segmentedDownloadSinkFactory(deliver: (filename: string, blob: Blob) => void): SegmentedDownloadSinkFactory {
  return async (filename, mimeType) => createBufferedDownloadSink(filename, mimeType, deliver);
}

/** 🧵 Drains exactly one capped producer chunk per awaited turn, preserving order and cancellation.
 *
 * 🧾️ Every bound comes from {@link SEGMENTED_DOWNLOAD_CONTRACT} — the same record the producer slices by
 * and the shard worker admits against — and each taken chunk RETIRES in the producer's outstanding table
 * as it is taken (`🔌️plugin/🦀️.rs`'s `take_chunk` pops the queue and the terminal `None` removes the
 * registry entry), so the loop is bounded by `maximumOutstandingChunks` rather than by the producer's
 * goodwill: a producer that never answers `None` is refused, not spun on forever. */
export async function drainSegmentedMediaExport(
  filename: string,
  mimeType: string,
  operation: string,
  marker: string | undefined,
  takeChunk: (operationId: bigint) => Promise<Uint8Array | undefined>,
  options: { readonly signal?: AbortSignal; readonly sinkFactory?: SegmentedDownloadSinkFactory; readonly maximumBytes?: number } = {},
): Promise<void> {
  const encoding = parseSegmentedDownloadMarker(marker);
  const operationId = parseSegmentedDownloadOperationId(operation);
  const maximumBytes = options.maximumBytes ?? SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes;
  if (!Number.isSafeInteger(maximumBytes) || maximumBytes < 0 || maximumBytes > SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes) throw new Error("segmented-download-cap-invalid");
  const initialAbort = segmentedAbortReason(options.signal);
  if (initialAbort !== undefined) throw initialAbort;
  const sink = await (options.sinkFactory ?? createSegmentedDownloadSink)(filename, mimeType);
  let sourceBytes = 0;
  let taken = 0;
  let base64Tail = "";
  try {
    while (true) {
      const beforeReadAbort = segmentedAbortReason(options.signal);
      if (beforeReadAbort !== undefined) throw beforeReadAbort;
      const chunk = await takeChunk(operationId);
      const afterReadAbort = segmentedAbortReason(options.signal);
      if (afterReadAbort !== undefined) throw afterReadAbort;
      const admitted = admitSegmentedDownloadChunk(chunk);
      if (admitted === undefined) break;
      taken += 1;
      if (taken > SEGMENTED_DOWNLOAD_CONTRACT.maximumOutstandingChunks) throw new Error(SEGMENTED_DOWNLOAD_REFUSAL.outstandingOverCap);
      sourceBytes += admitted.byteLength;
      if (sourceBytes > maximumBytes) throw new Error(SEGMENTED_DOWNLOAD_REFUSAL.totalOverCap);
      if (encoding === "identity") {
        await sink.write(admitted);
        continue;
      }
      const available = base64Tail + asciiChunk(admitted);
      const completeBytes = Math.max(0, available.length - (available.length % 4) - 4);
      if (completeBytes > 0) await sink.write(decodeBase64Block(available.slice(0, completeBytes), false));
      base64Tail = available.slice(completeBytes);
    }
    if (encoding === "base64" && base64Tail.length > 0) await sink.write(decodeBase64Block(base64Tail, true));
    await sink.close();
  } catch (error) {
    await sink.abort(error);
    throw error;
  }
}
