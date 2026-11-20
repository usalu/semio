import { test, expect } from "bun:test";
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";
import cases from "./🔣️.json";

const require = createRequire(import.meta.url);
const canvas = require("@napi-rs/canvas");
for (const name of ["DOMMatrix", "ImageData", "Path2D"]) {
  if (!(name in globalThis)) Object.defineProperty(globalThis, name, { value: canvas[name], configurable: true });
}
const pdf = await import("pdfjs-dist/legacy/build/pdf.mjs");
pdf.GlobalWorkerOptions.workerSrc = pathToFileURL(require.resolve("pdfjs-dist/legacy/build/pdf.worker.mjs")).href;

function onePage(row: (typeof cases)[number]): Uint8Array {
  const crop = row.crop ? `/CropBox [${row.crop.join(" ")}]` : "";
  const objects = [
    "<< /Type /Catalog /Pages 2 0 R >>",
    "<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
    `<< /Type /Page /Parent 2 0 R /Resources << >> /MediaBox [${row.media.join(" ")}] ${crop} /Rotate ${row.angle} /UserUnit ${row.unit} >>`,
  ];
  let text = "%PDF-1.7\n";
  const offsets = objects.map((object, i) => {
    const offset = text.length;
    text += `${i + 1} 0 obj\n${object}\nendobj\n`;
    return offset;
  });
  const xref = text.length;
  text += "xref\n0 4\n0000000000 65535 f \n";
  for (const offset of offsets) text += `${String(offset).padStart(10, "0")} 00000 n \n`;
  text += `trailer\n<< /Size 4 /Root 1 0 R >>\nstartxref\n${xref}\n%%EOF\n`;
  return new TextEncoder().encode(text);
}

test("page coordinates agree with PDF.js on rotation, crop and physical units", async () => {
  for (const row of cases) {
    const task = pdf.getDocument({ data: onePage(row) });
    try {
      const document = await task.promise;
      const view = (await document.getPage(1)).getViewport({ scale: 1, offsetY: row.top });
      expect(view.convertToViewportPoint(row.point[0]!, row.point[1]!)).toEqual(row.display);
      expect([0, row.top, view.width, view.height]).toEqual(row.clip);
    } finally {
      await task.destroy();
    }
  }
});
