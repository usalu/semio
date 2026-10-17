/** 🎬️ Media export encoding: the base64 and UTF-8 encodings, their error and the decoding of an exported payload to bytes.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { base64StandardDecode } from "@semio-tech/framework-io-base64-ts";

// #region 🎠️Kernel
//#region InvocationResponse
//#region ⬇️MediaExportEncoding
/** ⬇️ The only `downloadMediaExport.encoding` value that means "`data` is not text" — the TS twin of
 * Rust `kernel::MEDIA_EXPORT_BASE64_ENCODING`. */
export const MEDIA_EXPORT_BASE64_ENCODING = "base64";

/** ⬇️ The textual encoding a producer may state EXPLICITLY (`puzzle3d`'s `exportSnapshot` does). It
 * means exactly what an absent `encoding` means: `data` IS the file. Twin of Rust
 * `kernel::MEDIA_EXPORT_UTF8_ENCODING`. */
export const MEDIA_EXPORT_UTF8_ENCODING = "utf-8";

/** ⬇️ Why a `downloadMediaExport` envelope could not become bytes — the TS twin of Rust
 * `kernel::MediaExportEncodingError`. Thrown, never swallowed: a shell that quietly saved `data` as
 * text is the defect this contract closes. */
export class MediaExportEncodingError extends Error {
  readonly reason: "unsupported" | "malformed";
  readonly encoding: string;
  constructor(reason: "unsupported" | "malformed", encoding: string, detail: string) {
    super(reason === "unsupported" ? `unsupported media-export encoding "${encoding}"` : `malformed base64 media export (${detail})`);
    this.name = "MediaExportEncodingError";
    this.reason = reason;
    this.encoding = encoding;
  }
}

/** ⬇️ The ONE rule that turns a `downloadMediaExport` envelope into the bytes the user saves — no
 * `encoding` (or {@link MEDIA_EXPORT_UTF8_ENCODING}) means `data` IS the file, and
 * {@link MEDIA_EXPORT_BASE64_ENCODING} means `data` carries the bytes. Twin of Rust `kernel::media_export_bytes`; both drive
 * `🧫️fixtures/⬇️media-export-encoding/🔣️.json`.
 *
 * 🚧️ A segmented-download handle also rides in `encoding` (`SEGMENTED_DOWNLOAD_MARKER_PREFIX`) and is
 * consumed by the segmented lane BEFORE this is reached — it is not an encoding and is refused here. */
export function mediaExportBytes(data: string, encoding?: string): Uint8Array {
  if (encoding === undefined || encoding === MEDIA_EXPORT_UTF8_ENCODING) return new TextEncoder().encode(data);
  if (encoding !== MEDIA_EXPORT_BASE64_ENCODING) throw new MediaExportEncodingError("unsupported", encoding, encoding);
  try {
    return base64StandardDecode(data);
  } catch (error) {
    throw new MediaExportEncodingError("malformed", encoding, error instanceof Error ? error.message : String(error));
  }
}
//#endregion ⬇️MediaExportEncoding
//#endregion InvocationResponse
// #endregion 🎠️Kernel
