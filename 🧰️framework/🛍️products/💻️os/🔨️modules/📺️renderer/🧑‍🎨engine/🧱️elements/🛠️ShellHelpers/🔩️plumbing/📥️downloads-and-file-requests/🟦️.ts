/** 📥️ Shell plumbing, downloads and file requests: the media-export and data-url downloads, the backbone file and folder path requests, the file open and selection requests and the bounded blob reads. */
import { mediaExportBytes } from "../../../../../../../../../🔨️modules/🎠️kernel/🎬️media-export/🟦️.ts";
import { wireMediaExportEncoding } from "@semio-tech/framework-actor/wire-turn";
import { segmentedDownloadSinkFactory, type SegmentedDownloadSinkFactory } from "../../../📤️SegmentedDownload/🟦️.ts";

//#region ShellHelpers
export const DOWNLOAD_MEDIA_EXPORT_REVOKE_MS = 10_000;

/** 📥️ Recovers the guest's `encoding` from the wire, whatever shape the WIT `option<string>`
 * arrived in — the SAME reader both renderer doors use (`🖼️wire-turn.ts`). Reading it flat here is
 * what dropped it for every binary export (`📓️io-surface-2026-09-13.md` §7.3). What the string MEANS
 * is `kernel::mediaExportBytes`; a `SEGMENTED_DOWNLOAD_MARKER_PREFIX` value is a handle, not an
 * encoding, and is consumed by the segmented lane before either reaches the bytes. */
export const mediaExportEncodingText = wireMediaExportEncoding;

/** 📥️ Host download of guest `download-media-export`. The bytes come from the kernel's own
 * `(data, encoding)` contract — not a local `atob` branch — so text stays text, base64 becomes bytes,
 * and an encoding no shell knows is a loud typed refusal rather than a corrupt file. The anchor must
 * be in the document and the object URL must outlive the click or the browser drops the file. */
export function downloadMediaExport(filename: string, mimeType: string, data: string, encoding?: string): void {
  if (typeof document === "undefined") return;
  downloadMediaExportBytes(filename, mimeType, mediaExportBytes(data, encoding));
}

/** 📥️ Host delivery of already-assembled bytes — the blob-and-anchor half of {@link downloadMediaExport}, reached by the segmented lane once its chunks are drained. */
export function downloadMediaExportBytes(filename: string, mimeType: string, bytes: Uint8Array): void {
  if (typeof document === "undefined") return;
  // 🧭️ `slice()` because a `Uint8Array` may be backed by a `SharedArrayBuffer`, which is not a
  // `BlobPart`; it answers a fresh `Uint8Array<ArrayBuffer>`, and `Blob` copies its parts anyway.
  const blob = new Blob([bytes.slice()], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.style.display = "none";
  document.body.appendChild(anchor);
  anchor.click();
  window.setTimeout(() => {
    anchor.remove();
    URL.revokeObjectURL(url);
  }, DOWNLOAD_MEDIA_EXPORT_REVOKE_MS);
}

/** 🌊 The sink factory every shell drain uses: assembled chunks delivered as one file through {@link downloadMediaExportBytes}. */
export const shellSegmentedDownloadSinkFactory: SegmentedDownloadSinkFactory = segmentedDownloadSinkFactory(downloadMediaExportBytes);

export function downloadDataUrl(filename: string, dataUrl: string): void {
  if (typeof document === "undefined") return;
  const anchor = document.createElement("a");
  anchor.href = dataUrl;
  anchor.download = filename;
  anchor.click();
}

/** 📤️ Opens the native file picker. Resolves with one entry per selected file, in selection order —
 * always an array (empty on cancel) so single-file callers just read `[0]` and `multiple` callers can
 * fan out over the whole list; single-file behavior (one `<input>`, one resolved entry) is unchanged
 * when `multiple` is false/absent. */
type FileWithNativePath = File & { readonly path?: string };

/** 📂 Opens a single-file picker for backbone `file://` attach — resolves an absolute path when the
 * host exposes one (`File.path` on native shells); otherwise `null` (cancel or path unavailable). */
export function requestBackboneFilePath(accept = ".json,application/json"): Promise<string | null> {
  if (typeof document === "undefined") return Promise.resolve(null);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    input.onchange = () => {
      const file = input.files?.[0] as FileWithNativePath | undefined;
      resolve(file?.path?.trim() ? file.path : null);
    };
    input.oncancel = () => resolve(null);
    input.click();
  });
}

/** 📂 Opens a folder picker for backbone `folder://` attach — resolves an absolute directory path when
 * the host exposes one; otherwise `null`. */
export function requestBackboneFolderPath(): Promise<string | null> {
  if (typeof document === "undefined") return Promise.resolve(null);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.webkitdirectory = true;
    input.onchange = () => {
      const files = input.files;
      const first = files?.[0] as FileWithNativePath | undefined;
      if (!first) {
        resolve(null);
        return;
      }
      const native = first.path?.trim();
      if (native) {
        resolve(native.replace(/[/\\][^/\\]+$/, "") || native);
        return;
      }
      resolve(null);
    };
    input.oncancel = () => resolve(null);
    input.click();
  });
}

export function requestFileOpen(accept: string, readAs?: string, multiple?: boolean): Promise<readonly { contents: string; name: string }[]> {
  if (typeof document === "undefined") return Promise.resolve([]);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    if (multiple) input.multiple = true;
    input.onchange = async () => {
      const files = input.files ? Array.from(input.files) : [];
      if (files.length === 0) {
        resolve([]);
        return;
      }
      const opened: { contents: string; name: string }[] = [];
      for (const file of files) {
        if (readAs === "dataUrl") {
          const contents = await new Promise<string | null>((resolveFile) => {
            const reader = new FileReader();
            reader.onload = () => resolveFile(typeof reader.result === "string" ? reader.result : null);
            reader.onerror = () => resolveFile(null);
            reader.readAsDataURL(file);
          });
          if (contents !== null) opened.push({ contents, name: file.name });
          continue;
        }
        opened.push({ contents: await file.text(), name: file.name });
      }
      resolve(opened);
    };
    input.click();
  });
}

/** 📏️ Maximum browser-owned natural-file buffer admitted before native codec admission. */
export const NATURAL_FILE_BROWSER_MAXIMUM_BYTES_V1 = 512 * 1024 * 1024;

/** 🧩️ Maximum Blob slice materialized by one cancellable natural-file read turn. */
export const NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1 = 256 * 1024;

/** 📂️ Opens a picker without reading the selected files. Cancel always settles with an empty list and
 * every terminal path detaches handlers and the transient input exactly once. */
export function requestFileSelectionV1(accept: string, multiple = false): Promise<readonly File[]> {
  if (typeof document === "undefined") return Promise.resolve([]);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    input.multiple = multiple;
    let settled = false;
    const settle = (files: readonly File[]): void => {
      if (settled) return;
      settled = true;
      input.onchange = null;
      input.oncancel = null;
      input.remove();
      resolve(files);
    };
    input.onchange = () => settle(input.files ? Array.from(input.files) : []);
    input.oncancel = () => settle([]);
    try {
      input.click();
    } catch {
      settle([]);
    }
  });
}

const readBlobSliceV1 = (blob: Blob, signal: AbortSignal): Promise<ArrayBuffer> => {
  if (typeof FileReader === "undefined") {
    let onAbort: (() => void) | undefined;
    const aborted = new Promise<never>((_, reject) => {
      onAbort = () => reject(signal.reason ?? new DOMException("The operation was aborted", "AbortError"));
      signal.addEventListener("abort", onAbort, { once: true });
    });
    return Promise.race([blob.arrayBuffer(), aborted]).finally(() => {
      if (onAbort !== undefined) signal.removeEventListener("abort", onAbort);
    });
  }
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    let settled = false;
    const finish = (result: { readonly ok: true; readonly value: ArrayBuffer } | { readonly ok: false; readonly error: unknown }): void => {
      if (settled) return;
      settled = true;
      signal.removeEventListener("abort", abort);
      reader.onload = null;
      reader.onerror = null;
      reader.onabort = null;
      if (result.ok) resolve(result.value); else reject(result.error);
    };
    const abort = (): void => {
      reader.abort();
      finish({ ok: false, error: signal.reason ?? new DOMException("The operation was aborted", "AbortError") });
    };
    reader.onload = () => reader.result instanceof ArrayBuffer
      ? finish({ ok: true, value: reader.result })
      : finish({ ok: false, error: new Error("natural-file.read-result-invalid") });
    reader.onerror = () => finish({ ok: false, error: reader.error ?? new Error("natural-file.read-failed") });
    reader.onabort = () => finish({ ok: false, error: signal.reason ?? new DOMException("The operation was aborted", "AbortError") });
    signal.addEventListener("abort", abort, { once: true });
    try {
      if (signal.aborted) abort(); else reader.readAsArrayBuffer(blob);
    } catch (error) {
      finish({ ok: false, error });
    }
  });
};

/** 🌊 Reads one Blob through bounded slices with exact byte progress and cancellation between every
 * slice. The final contiguous owner is admitted before any slice is read. */
export async function readBlobBytesBoundedV1(blob: Blob, control: Readonly<{
  signal: AbortSignal;
  maximumBytes: number;
  chunkBytes: number;
  progress: (completed: number, total: number) => void;
}>): Promise<Uint8Array> {
  if (!Number.isSafeInteger(control.maximumBytes) || control.maximumBytes < 0 || !Number.isSafeInteger(control.chunkBytes) || control.chunkBytes < 1)
    throw new Error("natural-file.read-control-invalid");
  if (!Number.isSafeInteger(blob.size) || blob.size < 0 || blob.size > control.maximumBytes) throw new Error("natural-file.read-limit");
  const cancelled = (): never => { throw control.signal.reason ?? new DOMException("The operation was aborted", "AbortError"); };
  if (control.signal.aborted) cancelled();
  const bytes = new Uint8Array(blob.size);
  control.progress(0, blob.size);
  for (let offset = 0; offset < blob.size; offset += control.chunkBytes) {
    if (control.signal.aborted) cancelled();
    const end = Math.min(blob.size, offset + control.chunkBytes);
    const slice = blob.slice(offset, end);
    try {
      const chunk = new Uint8Array(await readBlobSliceV1(slice, control.signal));
      if (chunk.byteLength !== end - offset) throw new Error("natural-file.read-truncated");
      bytes.set(chunk, offset);
    } catch (error) {
      if (control.signal.aborted) cancelled();
      throw new Error("natural-file.read-failed", { cause: error });
    }
    control.progress(end, blob.size);
  }
  return bytes;
}
//#endregion ShellHelpers
