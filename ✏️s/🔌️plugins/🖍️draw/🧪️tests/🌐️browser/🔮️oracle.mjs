/** 🔮️ Independent PDF parsing stays behind the Draw browser acceptance oracle. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createCanvas, DOMMatrix, ImageData, Path2D } from "@napi-rs/canvas";

export async function assertPdfPainting(bytes, pages) {
  const previous = { DOMMatrix: globalThis.DOMMatrix, ImageData: globalThis.ImageData, Path2D: globalThis.Path2D };
  Object.assign(globalThis, { DOMMatrix, ImageData, Path2D });
  try {
    const { getDocument, OPS } = await import("pdfjs-dist/legacy/build/pdf.mjs");
    const task = getDocument({ data: Uint8Array.from(bytes), useSystemFonts: true, isEvalSupported: false });
    try {
      const pdf = await task.promise;
      assert.equal(pdf.numPages, pages);
      const sheet = await pdf.getPage(1);
      const operators = await sheet.getOperatorList();
      const painting = new Set([OPS.stroke, OPS.closeStroke, OPS.fill, OPS.eoFill, OPS.fillStroke, OPS.eoFillStroke, OPS.closeFillStroke, OPS.closeEOFillStroke, OPS.shadingFill, OPS.paintImageXObject, OPS.paintInlineImageXObject, OPS.showText, OPS.showSpacedText]);
      assert(operators.fnArray.some((operation, index) => painting.has(operation) || operation === OPS.constructPath && painting.has(operators.argsArray[index]?.[0])), "Export must contain path, image, or text painting");
      const viewport = sheet.getViewport({ scale: 1 });
      const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
      const context = canvas.getContext("2d");
      await sheet.render({ canvas, canvasContext: context, viewport }).promise;
      const pixels = context.getImageData(0, 0, canvas.width, canvas.height).data;
      assert(canvas.width > 0 && canvas.height > 0 && pixels.some((value, index) => index % 4 !== 3 && value < 255 && pixels[index - index % 4 + 3] > 0), "PDF must render visible nonwhite painting");
      return { width: canvas.width, height: canvas.height, digest: createHash("sha256").update(pixels).digest("hex"), pixels };
    } finally { await task.destroy(); }
  } finally { Object.assign(globalThis, previous); }
}

/** 🔬️ Counts independently rendered changed pixels rather than PDF metadata changes. */
export function assertPdfDifference(before, after) {
  let changedPixels = 0;
  for (let index = 0; index < Math.max(before.pixels.length, after.pixels.length); index += 4) {
    if (before.width !== after.width || before.height !== after.height || [0, 1, 2, 3].some(channel => before.pixels[index + channel] !== after.pixels[index + channel])) changedPixels++;
  }
  assert(changedPixels > 0, "PDF pixels must change after the rectangle gesture");
  return { before: { width: before.width, height: before.height, digest: before.digest }, after: { width: after.width, height: after.height, digest: after.digest }, changedPixels };
}
