/** 🖼️ Rasterizes a bounded preview using the browser's asynchronous image decoder. */
import { closed, closing, failed } from "../🧵️state/🟦️.ts";
import { macrotask, monitoredSuspension } from "../⏱️steps-and-reporting/🟦️.ts";
import { requestPageIconSvgDecode } from "../🖼️page-image-decode/🟦️.ts";
import { decodeReferenceImage, referenceImageBitmapForStage, streamReferenceImageBitmapRows, REFERENCE_IMAGE_SOURCE_BYTE_CAPACITY, REFERENCE_IMAGE_READBACK_BYTE_CAPACITY } from "../../🖼️reference-image-decode/🟦️.ts";

const PAGE_BYTES = 256 * 1024;
const HEADER_BYTES = 512;

export async function rasterizeIconSvgPreview(
  bytes: number,
  width: number,
  height: number,
  read: (offset: number) => Uint8Array<ArrayBuffer>,
  cancelled: Promise<void>,
): Promise<Uint8Array<ArrayBuffer>> {
  const pixels = width * height * 4;
  if (!Number.isSafeInteger(bytes) || bytes < 1 || bytes > REFERENCE_IMAGE_SOURCE_BYTE_CAPACITY
    || ![width, height].every((edge) => Number.isSafeInteger(edge) && edge > 0 && edge <= 1024)
    || pixels > REFERENCE_IMAGE_READBACK_BYTE_CAPACITY) throw new Error("icon-svg-preview-credits");
  const controller = new AbortController();
  const signal = controller.signal;
  void cancelled.then(() => controller.abort(new DOMException("icon SVG preview retired", "AbortError")));
  const isCurrent = () => !signal.aborted && !closed && !closing && !failed;
  const source = await iconSvgBlob(bytes, width, height, read, isCurrent);
  const dimensions = { width, height, orientation: 1, mediaType: "image/svg+xml" } as const;
  const bitmap = await referenceImageBitmapForStage(
    1,
    () => monitoredSuspension("icon-svg-preview-decode", () => decodeReferenceImage(source, dimensions, isCurrent)),
    () => monitoredSuspension("icon-svg-preview-page-decode", () => {
      if (!isCurrent()) return Promise.reject(new DOMException("icon SVG preview retired", "AbortError"));
      return requestPageIconSvgDecode(async () => new Uint8Array(await source.arrayBuffer()), dimensions, signal);
    }),
  );
  if (!bitmap) throw new Error("icon-svg-preview-bitmap");
  try {
    const rgba = new Uint8Array(pixels);
    await streamReferenceImageBitmapRows(bitmap, width, height, (offset, page) => {
      rgba.set(page, offset);
      return true;
    }, isCurrent, macrotask);
    return rgba;
  } finally {
    bitmap.close();
  }
}

export async function iconSvgBlob(
  bytes: number, width: number, height: number,
  read: (offset: number) => Uint8Array<ArrayBuffer>, isCurrent: () => boolean,
): Promise<Blob> {
  const parts: Blob[] = [];
  for (let offset = 0; offset < bytes;) {
    if (!isCurrent()) throw new DOMException("icon SVG preview retired", "AbortError");
    const page = read(offset);
    if (page.byteLength !== Math.min(PAGE_BYTES, bytes - offset)) throw new Error("icon-svg-preview-source-page");
    if (offset === 0) {
      const end = page.subarray(0, HEADER_BYTES).indexOf(62);
      if (end < 0) throw new Error("icon-svg-preview-header");
      const header = new TextDecoder("utf-8", { fatal: true }).decode(page.subarray(0, end));
      const extent = / width="[0-9]+" height="[0-9]+"$/;
      if (!header.startsWith("<svg ") || !extent.test(header)) throw new Error("icon-svg-preview-header");
      const resized = header.replace(extent, ` width="${width}" height="${height}"`);
      parts.push(new Blob([resized, page.subarray(end)]));
    } else {
      parts.push(new Blob([page]));
    }
    offset += page.byteLength;
    await macrotask();
  }
  if (!isCurrent()) throw new DOMException("icon SVG preview retired", "AbortError");
  return new Blob(parts, { type: "image/svg+xml" });
}
