/** 🧩️ Semantic parity pixels owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { PARITY_SCENE_LEAF_KINDS, ParityNode, PixelRegionResult } from "../🏗️structure/🟦️.ts";



type OwnedParityImage = { readonly width: number; readonly height: number; readonly data: Uint8Array };

//#endregion 🔖️StructuralCompare

//#region 🔖️PixelCompare
/** 📐️Pixel-region gate covers only structural *containers* (matches the design's "container-level
 * matched node pairs" — navbar/footer/panel-level regions are shell chrome, out of scope for this
 * pass since they're not part of the UiNode tree) plus scene/image leaves, not every leaf text node —
 * bounding pixel-diff cost to O(containers) rather than O(all nodes) per playground. */
const PARITY_PIXEL_REGION_KINDS = new Set(["stack", "field", "section", "group", "tree", "componentScene", "image"]);

const PARITY_PIXEL_THRESHOLD_DEFAULT = 0.005;

const PARITY_PIXEL_THRESHOLD_SCENE = 0.02;

const OWNED_PARITY_AA_MIN_CONTRAST_SQUARED = 0.0225;

const OWNED_PARITY_AA_MAX_COVERAGE_DELTA = 0.35;

const OWNED_PARITY_DIFF_MISMATCH = Object.freeze({ blue: 64, green: 32, red: 255 });

const OWNED_PARITY_DIFF_ANTIALIAS = Object.freeze({ blue: 0, green: 192, red: 255 });

type OwnedParityPixelOptions = {
  readonly ignoreAntialiasing: boolean;
  readonly threshold: number;
};

/** 🌐️ Decodes PNG bytes into exact RGBA pixels in the already-open parity browser page. */
async function decodeParityScreenshot(page: import("playwright").Page, bytes: Uint8Array): Promise<OwnedParityImage> {
  const decoded = await page.evaluate(async (encoded) => {
    const bitmap = await createImageBitmap(new Blob([Uint8Array.from(encoded)], { type: "image/png" }));
    try {
      const canvas = document.createElement("canvas");
      canvas.width = bitmap.width;
      canvas.height = bitmap.height;
      const context = canvas.getContext("2d");
      if (!context) throw new Error("Canvas 2D context is unavailable for parity PNG decoding");
      context.drawImage(bitmap, 0, 0);
      return { width: bitmap.width, height: bitmap.height, data: Array.from(context.getImageData(0, 0, bitmap.width, bitmap.height).data) };
    } finally {
      bitmap.close();
    }
  }, Array.from(bytes));
  return { width: decoded.width, height: decoded.height, data: Uint8Array.from(decoded.data) };
}

/** ✂️ Copies a bounded row-major RGBA crop without retaining or mutating source storage. */
function cropOwnedParityRgba(image: OwnedParityImage, x: number, y: number, width: number, height: number): Uint8Array {
  if (![image.width, image.height, x, y, width, height].every((value) => Number.isSafeInteger(value) && value >= 0)) throw new Error("Owned parity crop dimensions must be non-negative safe integers");
  const imageBytes = image.width * image.height * 4;
  if (!Number.isSafeInteger(imageBytes) || image.data.length !== imageBytes) throw new Error(`Owned parity image must contain exactly ${imageBytes} RGBA bytes`);
  if (x + width > image.width || y + height > image.height) throw new Error("Owned parity crop must stay within image bounds");
  const cropped = new Uint8Array(width * height * 4);
  const rowBytes = width * 4;
  for (let row = 0; row < height; row++) {
    const sourceOffset = ((y + row) * image.width + x) * 4;
    cropped.set(image.data.subarray(sourceOffset, sourceOffset + rowBytes), row * rowBytes);
  }
  return cropped;
}

/** 🖼️ Encodes exact RGBA pixels as a non-byte-contractual diagnostic PNG in the parity page. */
async function encodeParityDiff(page: import("playwright").Page, data: Uint8Array, width: number, height: number): Promise<Uint8Array> {
  if (!Number.isSafeInteger(width) || !Number.isSafeInteger(height) || width <= 0 || height <= 0 || data.length !== width * height * 4) throw new Error("Owned parity diagnostic must contain positive dimensions and exact RGBA bytes");
  const encoded = await page.evaluate(
    async ({ pixels, width: imageWidth, height: imageHeight }) => {
      const canvas = document.createElement("canvas");
      canvas.width = imageWidth;
      canvas.height = imageHeight;
      const context = canvas.getContext("2d");
      if (!context) throw new Error("Canvas 2D context is unavailable for parity PNG encoding");
      context.putImageData(new ImageData(Uint8ClampedArray.from(pixels), imageWidth, imageHeight), 0, 0);
      const blob = await new Promise<Blob>((resolve, reject) => canvas.toBlob((value) => (value ? resolve(value) : reject(new Error("Canvas could not encode the parity diagnostic PNG"))), "image/png"));
      return Array.from(new Uint8Array(await blob.arrayBuffer()));
    },
    { pixels: Array.from(data), width, height },
  );
  return Uint8Array.from(encoded);
}

function ownedParityByteRangesOverlap(left: Uint8Array, right: Uint8Array): boolean {
  if (left.buffer !== right.buffer) return false;
  const leftEnd = left.byteOffset + left.byteLength;
  const rightEnd = right.byteOffset + right.byteLength;
  return left.byteOffset < rightEnd && right.byteOffset < leftEnd;
}

function ownedParityCompositeChannel(pixels: Uint8Array, offset: number, channel: number): number {
  const alpha = pixels[offset + 3]! / 255;
  return 1 - alpha + (pixels[offset + channel]! / 255) * alpha;
}

function ownedParityDistanceSquared(reference: Uint8Array, referenceOffset: number, candidate: Uint8Array, candidateOffset: number): number {
  const alphaCoverageScale = reference[referenceOffset] === candidate[candidateOffset] && reference[referenceOffset + 1] === candidate[candidateOffset + 1] && reference[referenceOffset + 2] === candidate[candidateOffset + 2] ? 0.5 : 1;
  const red = (ownedParityCompositeChannel(reference, referenceOffset, 0) - ownedParityCompositeChannel(candidate, candidateOffset, 0)) * alphaCoverageScale;
  const green = (ownedParityCompositeChannel(reference, referenceOffset, 1) - ownedParityCompositeChannel(candidate, candidateOffset, 1)) * alphaCoverageScale;
  const blue = (ownedParityCompositeChannel(reference, referenceOffset, 2) - ownedParityCompositeChannel(candidate, candidateOffset, 2)) * alphaCoverageScale;
  return 0.299 * red * red + 0.587 * green * green + 0.114 * blue * blue;
}

function ownedParityWriteMuted(reference: Uint8Array, offset: number, diff: Uint8Array): void {
  const luminance = 0.2126 * ownedParityCompositeChannel(reference, offset, 0) + 0.7152 * ownedParityCompositeChannel(reference, offset, 1) + 0.0722 * ownedParityCompositeChannel(reference, offset, 2);
  const muted = Math.round(255 * (0.75 + luminance * 0.25));
  diff[offset] = muted;
  diff[offset + 1] = muted;
  diff[offset + 2] = muted;
  diff[offset + 3] = 255;
}

function ownedParityWriteMarker(diff: Uint8Array, offset: number, marker: Readonly<{ blue: number; green: number; red: number }>): void {
  diff[offset] = marker.red;
  diff[offset + 1] = marker.green;
  diff[offset + 2] = marker.blue;
  diff[offset + 3] = 255;
}

function ownedParityCoverage(reference: Uint8Array, candidate: Uint8Array, offset: number, lowRed: number, lowGreen: number, lowBlue: number, spanRed: number, spanGreen: number, spanBlue: number, spanSquared: number): boolean {
  const referenceRed = ownedParityCompositeChannel(reference, offset, 0) - lowRed;
  const referenceGreen = ownedParityCompositeChannel(reference, offset, 1) - lowGreen;
  const referenceBlue = ownedParityCompositeChannel(reference, offset, 2) - lowBlue;
  const candidateRed = ownedParityCompositeChannel(candidate, offset, 0) - lowRed;
  const candidateGreen = ownedParityCompositeChannel(candidate, offset, 1) - lowGreen;
  const candidateBlue = ownedParityCompositeChannel(candidate, offset, 2) - lowBlue;
  const referenceCoverage = (referenceRed * spanRed + referenceGreen * spanGreen + referenceBlue * spanBlue) / spanSquared;
  const candidateCoverage = (candidateRed * spanRed + candidateGreen * spanGreen + candidateBlue * spanBlue) / spanSquared;
  if (referenceCoverage <= 0.03 || referenceCoverage >= 0.97 || candidateCoverage <= 0.03 || candidateCoverage >= 0.97 || Math.abs(referenceCoverage - candidateCoverage) > OWNED_PARITY_AA_MAX_COVERAGE_DELTA) return false;
  const referenceResidualRed = referenceRed - referenceCoverage * spanRed;
  const referenceResidualGreen = referenceGreen - referenceCoverage * spanGreen;
  const referenceResidualBlue = referenceBlue - referenceCoverage * spanBlue;
  const candidateResidualRed = candidateRed - candidateCoverage * spanRed;
  const candidateResidualGreen = candidateGreen - candidateCoverage * spanGreen;
  const candidateResidualBlue = candidateBlue - candidateCoverage * spanBlue;
  const referenceResidual = referenceResidualRed * referenceResidualRed + referenceResidualGreen * referenceResidualGreen + referenceResidualBlue * referenceResidualBlue;
  const candidateResidual = candidateResidualRed * candidateResidualRed + candidateResidualGreen * candidateResidualGreen + candidateResidualBlue * candidateResidualBlue;
  return referenceResidual <= spanSquared * 0.01 && candidateResidual <= spanSquared * 0.01;
}

function ownedParityIsAntialiased(reference: Uint8Array, candidate: Uint8Array, width: number, height: number, x: number, y: number, offset: number, thresholdSquared: number): boolean {
  if (x === 0 || y === 0 || x === width - 1 || y === height - 1) return false;
  let lowBlue = 0;
  let lowGreen = 0;
  let lowLuminance = Number.POSITIVE_INFINITY;
  let lowRed = 0;
  let highBlue = 0;
  let highGreen = 0;
  let highLuminance = Number.NEGATIVE_INFINITY;
  let highRed = 0;
  let stableNeighbors = 0;
  const minimumY = Math.max(0, y - 1);
  const maximumY = Math.min(height - 1, y + 1);
  const minimumX = Math.max(0, x - 1);
  const maximumX = Math.min(width - 1, x + 1);
  for (let neighborY = minimumY; neighborY <= maximumY; neighborY++) {
    for (let neighborX = minimumX; neighborX <= maximumX; neighborX++) {
      const neighborOffset = (neighborY * width + neighborX) * 4;
      if (neighborOffset === offset || ownedParityDistanceSquared(reference, neighborOffset, candidate, neighborOffset) > thresholdSquared) continue;
      stableNeighbors += 1;
      const red = (ownedParityCompositeChannel(reference, neighborOffset, 0) + ownedParityCompositeChannel(candidate, neighborOffset, 0)) * 0.5;
      const green = (ownedParityCompositeChannel(reference, neighborOffset, 1) + ownedParityCompositeChannel(candidate, neighborOffset, 1)) * 0.5;
      const blue = (ownedParityCompositeChannel(reference, neighborOffset, 2) + ownedParityCompositeChannel(candidate, neighborOffset, 2)) * 0.5;
      const luminance = 0.2126 * red + 0.7152 * green + 0.0722 * blue;
      if (luminance < lowLuminance) {
        lowBlue = blue;
        lowGreen = green;
        lowLuminance = luminance;
        lowRed = red;
      }
      if (luminance > highLuminance) {
        highBlue = blue;
        highGreen = green;
        highLuminance = luminance;
        highRed = red;
      }
    }
  }
  if (stableNeighbors < 2) return false;
  const spanRed = highRed - lowRed;
  const spanGreen = highGreen - lowGreen;
  const spanBlue = highBlue - lowBlue;
  const spanSquared = spanRed * spanRed + spanGreen * spanGreen + spanBlue * spanBlue;
  const perceptualSpanSquared = 0.2126 * spanRed * spanRed + 0.7152 * spanGreen * spanGreen + 0.0722 * spanBlue * spanBlue;
  return perceptualSpanSquared >= OWNED_PARITY_AA_MIN_CONTRAST_SQUARED && ownedParityCoverage(reference, candidate, offset, lowRed, lowGreen, lowBlue, spanRed, spanGreen, spanBlue, spanSquared);
}

/** 🎨️ Composites sRGB over white, halves pure alpha-coverage deltas, compares sqrt(0.299·dr² + 0.587·dg² + 0.114·db²), and suppresses only bounded shared-edge coverage. */
function compareOwnedParityPixels(referenceRgba: Uint8Array, candidateRgba: Uint8Array, diffRgba: Uint8Array, width: number, height: number, options: OwnedParityPixelOptions): number {
  if (!Number.isSafeInteger(width) || !Number.isSafeInteger(height) || width < 0 || height < 0) throw new Error("Owned parity pixel dimensions must be non-negative safe integers");
  const pixelCount = width * height;
  if (!Number.isSafeInteger(pixelCount) || pixelCount > Math.floor(Number.MAX_SAFE_INTEGER / 4)) throw new Error("Owned parity pixel dimensions exceed the safe byte range");
  const expectedBytes = pixelCount * 4;
  if (referenceRgba.length !== expectedBytes || candidateRgba.length !== expectedBytes || diffRgba.length !== expectedBytes) throw new Error(`Owned parity pixel buffers must each contain exactly ${expectedBytes} RGBA bytes`);
  if (!Number.isFinite(options.threshold) || options.threshold < 0 || options.threshold > 1) throw new Error("Owned parity pixel threshold must be finite and between zero and one");
  if (ownedParityByteRangesOverlap(diffRgba, referenceRgba) || ownedParityByteRangesOverlap(diffRgba, candidateRgba)) throw new Error("Owned parity pixel diff buffer must not overlap either read input");
  const thresholdSquared = options.threshold * options.threshold;
  let firstDifferentPixel = pixelCount;
  for (let pixel = 0; pixel < pixelCount; pixel++) {
    const offset = pixel * 4;
    if (referenceRgba[offset] !== candidateRgba[offset] || referenceRgba[offset + 1] !== candidateRgba[offset + 1] || referenceRgba[offset + 2] !== candidateRgba[offset + 2] || referenceRgba[offset + 3] !== candidateRgba[offset + 3]) {
      firstDifferentPixel = pixel;
      break;
    }
    ownedParityWriteMuted(referenceRgba, offset, diffRgba);
  }
  if (firstDifferentPixel === pixelCount) return 0;
  let mismatched = 0;
  for (let pixel = firstDifferentPixel; pixel < pixelCount; pixel++) {
    const offset = pixel * 4;
    if (referenceRgba[offset] === candidateRgba[offset] && referenceRgba[offset + 1] === candidateRgba[offset + 1] && referenceRgba[offset + 2] === candidateRgba[offset + 2] && referenceRgba[offset + 3] === candidateRgba[offset + 3]) {
      ownedParityWriteMuted(referenceRgba, offset, diffRgba);
      continue;
    }
    const distanceSquared = ownedParityDistanceSquared(referenceRgba, offset, candidateRgba, offset);
    if (distanceSquared <= thresholdSquared) {
      ownedParityWriteMuted(referenceRgba, offset, diffRgba);
      continue;
    }
    const y = Math.floor(pixel / width);
    const x = pixel - y * width;
    if (options.ignoreAntialiasing && ownedParityIsAntialiased(referenceRgba, candidateRgba, width, height, x, y, offset, thresholdSquared)) {
      ownedParityWriteMarker(diffRgba, offset, OWNED_PARITY_DIFF_ANTIALIAS);
      continue;
    }
    mismatched += 1;
    ownedParityWriteMarker(diffRgba, offset, OWNED_PARITY_DIFF_MISMATCH);
  }
  return mismatched;
}

function parityPixelThreshold(kind: string): number {
  return PARITY_SCENE_LEAF_KINDS.has(kind) ? PARITY_PIXEL_THRESHOLD_SCENE : PARITY_PIXEL_THRESHOLD_DEFAULT;
}

async function compareParityRegion(page: import("playwright").Page, reactPng: OwnedParityImage, wgpuPng: OwnedParityImage, node: ParityNode, outDir: string, variant: string): Promise<PixelRegionResult> {
  const [rx, ry, rw, rh] = node.rect;
  const width = Math.max(1, Math.min(Math.round(rw), reactPng.width - Math.round(rx), wgpuPng.width - Math.round(rx)));
  const height = Math.max(1, Math.min(Math.round(rh), reactPng.height - Math.round(ry), wgpuPng.height - Math.round(ry)));
  const threshold = parityPixelThreshold(node.kind);
  if (width <= 0 || height <= 0 || rx < 0 || ry < 0) return { path: node.path, ratio: 0, threshold };
  const reactCrop = cropOwnedParityRgba(reactPng, Math.round(rx), Math.round(ry), width, height);
  const wgpuCrop = cropOwnedParityRgba(wgpuPng, Math.round(rx), Math.round(ry), width, height);
  const diff = new Uint8Array(width * height * 4);
  const mismatched = compareOwnedParityPixels(reactCrop, wgpuCrop, diff, width, height, { threshold: 0.1, ignoreAntialiasing: true });
  const ratio = mismatched / (width * height);
  let diffPng: string | undefined;
  if (ratio > threshold) {
    diffPng = join(outDir, `diff-${variant}-${node.path.replace(/[^a-zA-Z0-9]+/g, "_")}.png`);
    writeFileSync(diffPng, await encodeParityDiff(page, diff, width, height));
  }
  return { path: node.path, ratio, threshold, diffPng };
}

export { OWNED_PARITY_AA_MAX_COVERAGE_DELTA, OWNED_PARITY_AA_MIN_CONTRAST_SQUARED, OWNED_PARITY_DIFF_ANTIALIAS, OWNED_PARITY_DIFF_MISMATCH, OwnedParityImage, OwnedParityPixelOptions, PARITY_PIXEL_REGION_KINDS, PARITY_PIXEL_THRESHOLD_DEFAULT, PARITY_PIXEL_THRESHOLD_SCENE, compareOwnedParityPixels, compareParityRegion, cropOwnedParityRgba, decodeParityScreenshot, encodeParityDiff, ownedParityByteRangesOverlap, ownedParityCompositeChannel, ownedParityCoverage, ownedParityDistanceSquared, ownedParityIsAntialiased, ownedParityWriteMarker, ownedParityWriteMuted, parityPixelThreshold };
