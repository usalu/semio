export const REFERENCE_IMAGE_DECODED_BYTE_CAPACITY = 64 * 1024 * 1024;
export const REFERENCE_IMAGE_SOURCE_BYTE_CAPACITY = 64 * 1024 * 1024;
export const REFERENCE_IMAGE_SOURCE_PIXEL_CAPACITY = REFERENCE_IMAGE_SOURCE_BYTE_CAPACITY / 4;
export const REFERENCE_IMAGE_READBACK_BYTE_CAPACITY = 1024 * 1024;

export function referenceImageTargetSize(width: number, height: number): readonly [number, number] {
  const pixels = width * height;
  const budget = REFERENCE_IMAGE_DECODED_BYTE_CAPACITY / 4;
  if (!Number.isSafeInteger(width) || !Number.isSafeInteger(height) || width < 1 || height < 1) throw new Error("reference-image-dimensions");
  if (pixels <= budget) return [width, height];
  const scale = Math.sqrt(budget / pixels);
  return [Math.max(1, Math.floor(width * scale)), Math.max(1, Math.floor(height * scale))];
}

export function referenceImageStripRows(width: number): number {
  const rowBytes = width * 4;
  if (!Number.isSafeInteger(rowBytes) || rowBytes < 4 || rowBytes > REFERENCE_IMAGE_READBACK_BYTE_CAPACITY) throw new Error("reference-image-row-credits");
  return Math.floor(REFERENCE_IMAGE_READBACK_BYTE_CAPACITY / rowBytes);
}

export type ReferenceImageMediaType = "image/png" | "image/jpeg" | "image/gif" | "image/webp" | "image/bmp" | "image/svg+xml";
export type ReferenceImageDimensions = { readonly width: number; readonly height: number; readonly orientation: number; readonly mediaType: ReferenceImageMediaType };

function be16(bytes: Uint8Array, offset: number): number {
  return bytes[offset] * 256 + bytes[offset + 1];
}

function be32(bytes: Uint8Array, offset: number): number {
  return bytes[offset] * 0x1000000 + bytes[offset + 1] * 0x10000 + bytes[offset + 2] * 0x100 + bytes[offset + 3];
}

function le16(bytes: Uint8Array, offset: number): number {
  return bytes[offset] + bytes[offset + 1] * 0x100;
}

function le24(bytes: Uint8Array, offset: number): number {
  return bytes[offset] + bytes[offset + 1] * 0x100 + bytes[offset + 2] * 0x10000;
}

function le32(bytes: Uint8Array, offset: number): number {
  return bytes[offset] + bytes[offset + 1] * 0x100 + bytes[offset + 2] * 0x10000 + bytes[offset + 3] * 0x1000000;
}

function ascii(bytes: Uint8Array, start: number, end: number): string {
  return String.fromCharCode(...bytes.subarray(start, end));
}

function jpegExifOrientation(bytes: Uint8Array, offset: number, length: number): number {
  const start = offset + 2;
  if (length < 16 || start + length - 2 > bytes.length || new TextDecoder("ascii").decode(bytes.subarray(start, start + 6)) !== "Exif\0\0") return 1;
  const tiff = start + 6;
  const little = bytes[tiff] === 0x49 && bytes[tiff + 1] === 0x49;
  const big = bytes[tiff] === 0x4d && bytes[tiff + 1] === 0x4d;
  if (!little && !big) return 1;
  const u16 = (at: number) => little ? bytes[at] + bytes[at + 1] * 0x100 : be16(bytes, at);
  const u32 = (at: number) => little
    ? bytes[at] + bytes[at + 1] * 0x100 + bytes[at + 2] * 0x10000 + bytes[at + 3] * 0x1000000
    : be32(bytes, at);
  if (u16(tiff + 2) !== 42) return 1;
  const ifd = tiff + u32(tiff + 4);
  if (ifd + 2 > start + length - 2) return 1;
  const count = u16(ifd);
  for (let index = 0; index < count; index++) {
    const entry = ifd + 2 + index * 12;
    if (entry + 12 > start + length - 2) return 1;
    if (u16(entry) === 0x0112 && u16(entry + 2) === 3 && u32(entry + 4) === 1) {
      const orientation = u16(entry + 8);
      return orientation >= 1 && orientation <= 8 ? orientation : 1;
    }
  }
  return 1;
}

function svgLength(value: string | undefined, fallback: number): number {
  if (value === undefined) return fallback;
  const match = value.trim().match(/^([+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:e[+-]?\d+)?)\s*(%|px|in|cm|mm|pt|pc)?$/i);
  if (!match) return fallback;
  const amount = Number(match[1]);
  const unit = match[2]?.toLowerCase();
  const resolved = unit === "%" ? fallback * amount / 100
    : unit === "in" ? amount * 96
      : unit === "cm" ? amount * 96 / 2.54
        : unit === "mm" ? amount * 96 / 25.4
          : unit === "pt" ? amount * 96 / 72
            : unit === "pc" ? amount * 16
              : amount;
  if (!Number.isFinite(resolved) || resolved <= 0) throw new Error("reference-image-format");
  return resolved;
}

function svgDimensions(bytes: Uint8Array): readonly [number, number] | undefined {
  const text = new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(0, Math.min(bytes.length, 64 * 1024)));
  const start = text.search(/<svg(?=[\s/>])/);
  if (start < 0) return undefined;
  let quote = "";
  let end = start + 4;
  for (; end < text.length; end++) {
    const character = text[end];
    if (quote) {
      if (character === quote) quote = "";
    } else if (character === "\"" || character === "'") {
      quote = character;
    } else if (character === ">") {
      break;
    }
  }
  if (end === text.length) throw new Error("reference-image-format");
  const attributes = new Map<string, string>();
  for (const match of text.slice(start + 4, end).matchAll(/(?:^|\s)([A-Za-z_:][\w:.-]*)\s*=\s*(["'])(.*?)\2/gs)) attributes.set(match[1], match[3]);
  const viewBox = attributes.get("viewBox")?.trim().split(/[\s,]+/).map(Number);
  const validViewBox = viewBox?.length === 4 && viewBox.every(Number.isFinite) && viewBox[2] > 0 && viewBox[3] > 0;
  const fallbackWidth = validViewBox ? viewBox[2] : 300;
  const fallbackHeight = validViewBox ? viewBox[3] : 150;
  return [Math.max(1, Math.round(svgLength(attributes.get("width"), fallbackWidth))), Math.max(1, Math.round(svgLength(attributes.get("height"), fallbackHeight)))];
}

export function referenceImageSourceDimensions(bytes: Uint8Array): ReferenceImageDimensions {
  let width = 0;
  let height = 0;
  let orientation = 1;
  let mediaType: ReferenceImageMediaType | undefined;
  if (bytes.length >= 24 && bytes.subarray(0, 8).every((value, index) => value === [137, 80, 78, 71, 13, 10, 26, 10][index])) {
    width = be32(bytes, 16);
    height = be32(bytes, 20);
    mediaType = "image/png";
  } else if (bytes.length >= 4 && bytes[0] === 0xff && bytes[1] === 0xd8) {
    mediaType = "image/jpeg";
    let offset = 2;
    const frames = new Set([0xc0, 0xc1, 0xc2, 0xc3, 0xc5, 0xc6, 0xc7, 0xc9, 0xca, 0xcb, 0xcd, 0xce, 0xcf]);
    while (offset + 4 <= bytes.length) {
      while (offset < bytes.length && bytes[offset] !== 0xff) offset++;
      while (offset < bytes.length && bytes[offset] === 0xff) offset++;
      if (offset >= bytes.length) break;
      const marker = bytes[offset++];
      if (marker === 0xd9 || marker === 0xda) break;
      if (marker === 0x01 || (marker >= 0xd0 && marker <= 0xd7)) continue;
      if (offset + 2 > bytes.length) break;
      const length = be16(bytes, offset);
      if (length < 2 || offset + length > bytes.length) break;
      if (marker === 0xe1 && orientation === 1) orientation = jpegExifOrientation(bytes, offset, length);
      if (frames.has(marker) && length >= 7) {
        height = be16(bytes, offset + 3);
        width = be16(bytes, offset + 5);
      }
      offset += length;
    }
  } else if (bytes.length >= 10 && (ascii(bytes, 0, 6) === "GIF87a" || ascii(bytes, 0, 6) === "GIF89a")) {
    width = le16(bytes, 6);
    height = le16(bytes, 8);
    mediaType = "image/gif";
  } else if (bytes.length >= 20 && ascii(bytes, 0, 4) === "RIFF" && ascii(bytes, 8, 12) === "WEBP") {
    const fourcc = ascii(bytes, 12, 16);
    if (fourcc === "VP8 " && bytes.length >= 30 && bytes[23] === 0x9d && bytes[24] === 0x01 && bytes[25] === 0x2a) {
      width = le16(bytes, 26) & 0x3fff;
      height = le16(bytes, 28) & 0x3fff;
    } else if (fourcc === "VP8L" && bytes.length >= 25 && bytes[20] === 0x2f) {
      const packed = le32(bytes, 21);
      width = (packed & 0x3fff) + 1;
      height = ((packed >>> 14) & 0x3fff) + 1;
    } else if (fourcc === "VP8X" && bytes.length >= 30) {
      width = le24(bytes, 24) + 1;
      height = le24(bytes, 27) + 1;
    }
    mediaType = "image/webp";
  } else if (bytes.length >= 26 && bytes[0] === 0x42 && bytes[1] === 0x4d) {
    width = Math.abs(new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).getInt32(18, true));
    height = Math.abs(new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).getInt32(22, true));
    mediaType = "image/bmp";
  } else {
    const svg = svgDimensions(bytes);
    if (svg) {
      [width, height] = svg;
      mediaType = "image/svg+xml";
    }
  }
  if (orientation >= 5 && orientation <= 8) [width, height] = [height, width];
  const pixels = width * height;
  if (!mediaType || !Number.isSafeInteger(pixels) || width < 1 || height < 1) throw new Error("reference-image-format");
  if (pixels > REFERENCE_IMAGE_SOURCE_PIXEL_CAPACITY) throw new Error("reference-image-source-pixel-credits");
  return { width, height, orientation, mediaType };
}

export function concatenateReferenceImageSource(chunks: readonly Uint8Array[]): Uint8Array<ArrayBuffer> {
  const length = chunks.reduce((sum, chunk) => sum + chunk.byteLength, 0);
  if (!Number.isSafeInteger(length) || length < 1 || length > 16 * 1024 * 1024) throw new Error("reference-image-source-byte-credits");
  const bytes = new Uint8Array(length);
  let offset = 0;
  for (const chunk of chunks) {
    bytes.set(chunk, offset);
    offset += chunk.byteLength;
  }
  return bytes;
}

export async function decodeReferenceImage(source: Blob, dimensions: ReferenceImageDimensions, isCurrent: () => boolean = () => true): Promise<ImageBitmap> {
  const [width, height] = referenceImageTargetSize(dimensions.width, dimensions.height);
  if (!isCurrent()) throw new DOMException("reference image generation retired", "AbortError");
  const bitmap = await createImageBitmap(source, {
    colorSpaceConversion: "default",
    imageOrientation: "from-image",
    premultiplyAlpha: "none",
    resizeWidth: width,
    resizeHeight: height,
    resizeQuality: "medium",
  });
  if (!isCurrent()) {
    bitmap.close();
    throw new DOMException("reference image generation retired", "AbortError");
  }
  if (bitmap.width !== width || bitmap.height !== height) {
    bitmap.close();
    throw new Error("reference-image-resize");
  }
  return bitmap;
}

export async function referenceImageBitmapForStage(stageMode: number, decode: () => Promise<ImageBitmap>, fallback: () => Promise<ImageBitmap>): Promise<ImageBitmap | undefined> {
  if (stageMode === 2) return undefined;
  if (stageMode !== 1) throw new Error("reference-image-begin-mode");
  try {
    return await decode();
  } catch (error) {
    if (error instanceof DOMException && error.name === "AbortError") throw error;
    return fallback();
  }
}

export async function streamReferenceImageBitmapRows(
  bitmap: ImageBitmap,
  width: number,
  height: number,
  push: (offset: number, pixels: Uint8Array) => boolean,
  isCurrent: () => boolean,
  yieldToHost: () => Promise<void>,
  readStrip: <T>(operation: () => T) => T = (operation) => operation(),
): Promise<void> {
  if (bitmap.width !== width || bitmap.height !== height) throw new Error("reference-image-resize");
  const rowBytes = width * 4;
  const rowsPerStrip = referenceImageStripRows(width);
  const canvas = new OffscreenCanvas(width, Math.min(height, rowsPerStrip));
  const context = canvas.getContext("2d", { alpha: true, colorSpace: "srgb", willReadFrequently: true });
  if (!context) throw new Error("reference-image-context");
  context.globalCompositeOperation = "copy";
  for (let row = 0; row < height; row += rowsPerStrip) {
    if (!isCurrent()) throw new DOMException("reference image generation retired", "AbortError");
    const rows = Math.min(rowsPerStrip, height - row);
    const accepted = readStrip(() => {
      context.drawImage(bitmap, 0, row, width, rows, 0, 0, width, rows);
      const readback = context.getImageData(0, 0, width, rows, { colorSpace: "srgb" }).data;
      const pixels = new Uint8Array(readback.buffer, readback.byteOffset, readback.byteLength);
      if (pixels.byteLength !== rows * rowBytes || pixels.byteLength > REFERENCE_IMAGE_READBACK_BYTE_CAPACITY) throw new Error("reference-image-row-credits");
      return push(row * rowBytes, pixels);
    });
    if (!accepted) throw new Error("reference-image-row-stage-busy");
    await yieldToHost();
    if (!isCurrent()) throw new DOMException("reference image generation retired", "AbortError");
  }
}

export async function decodeReferenceImageOnPage(source: Blob, dimensions: ReferenceImageDimensions, signal: AbortSignal): Promise<ImageBitmap> {
  const [width, height] = referenceImageTargetSize(dimensions.width, dimensions.height);
  if (signal.aborted) throw new DOMException("reference image generation retired", "AbortError");
  const url = URL.createObjectURL(source);
  const image = new Image();
  const abort = () => {
    image.src = "";
  };
  signal.addEventListener("abort", abort, { once: true });
  try {
    image.decoding = "async";
    image.src = url;
    try {
      await image.decode();
    } catch (error) {
      if (signal.aborted) throw new DOMException("reference image generation retired", "AbortError");
      throw error;
    }
    if (signal.aborted) throw new DOMException("reference image generation retired", "AbortError");
    const bitmap = await createImageBitmap(image, {
      colorSpaceConversion: "default",
      imageOrientation: "from-image",
      premultiplyAlpha: "none",
      resizeWidth: width,
      resizeHeight: height,
      resizeQuality: "medium",
    });
    if (!signal.aborted) return bitmap;
    bitmap.close();
    throw new DOMException("reference image generation retired", "AbortError");
  } finally {
    signal.removeEventListener("abort", abort);
    image.src = "";
    URL.revokeObjectURL(url);
  }
}

export async function referenceImageSourceDigest(bytes: Uint8Array<ArrayBuffer>): Promise<string> {
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", bytes));
  return Array.from(digest, (value) => value.toString(16).padStart(2, "0")).join("");
}
