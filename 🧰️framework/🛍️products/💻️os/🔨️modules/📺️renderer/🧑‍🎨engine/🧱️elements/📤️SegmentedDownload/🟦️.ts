// #region 🧲️Header
/** @emoji 🧵️ Bounded, operation-owned browser download streaming. */
// #endregion 🧲️Header

export const SEGMENTED_DOWNLOAD_MARKER_PREFIX = "semio-segmented-handle-v1:";
export const MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES = 4_096;
export const MAX_SEGMENTED_DOWNLOAD_BYTES = 32 << 20;
const MAX_U64 = (1n << 64n) - 1n;

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
  const operationId = BigInt(value);
  if (operationId > MAX_U64) throw new Error("segmented-download-operation-invalid");
  return operationId;
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

/** 🧵 Drains exactly one capped producer chunk per awaited turn, preserving order and cancellation. */
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
  const maximumBytes = options.maximumBytes ?? MAX_SEGMENTED_DOWNLOAD_BYTES;
  if (!Number.isSafeInteger(maximumBytes) || maximumBytes < 0 || maximumBytes > MAX_SEGMENTED_DOWNLOAD_BYTES) throw new Error("segmented-download-cap-invalid");
  const initialAbort = segmentedAbortReason(options.signal);
  if (initialAbort !== undefined) throw initialAbort;
  const sink = await (options.sinkFactory ?? createSegmentedDownloadSink)(filename, mimeType);
  let sourceBytes = 0;
  let base64Tail = "";
  try {
    while (true) {
      const beforeReadAbort = segmentedAbortReason(options.signal);
      if (beforeReadAbort !== undefined) throw beforeReadAbort;
      const chunk = await takeChunk(operationId);
      const afterReadAbort = segmentedAbortReason(options.signal);
      if (afterReadAbort !== undefined) throw afterReadAbort;
      if (chunk === undefined) break;
      if (Object.prototype.toString.call(chunk) !== "[object Uint8Array]" || chunk.byteLength === 0 || chunk.byteLength > MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES) throw new Error("segmented-download-chunk-limit");
      sourceBytes += chunk.byteLength;
      if (sourceBytes > maximumBytes) throw new Error("segmented-download-total-limit");
      if (encoding === "identity") {
        if (chunk.byteLength > 0) await sink.write(chunk);
        continue;
      }
      const available = base64Tail + asciiChunk(chunk);
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
