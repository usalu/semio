/** 📕 Round-trips page count and page size through the TypeScript page snapshot, then checks those facts with PDF.js. */
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";
import { expect, test } from "bun:test";
import { binary64Value } from "@semio-tech/framework/value/ieee754";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework/io/sqlite-snapshot";
import { parsePdfSnapshot, type PdfSnapshot } from "../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import { pdf14SnapshotFromSqliteDatabase, pdf14SnapshotToSqliteDatabase } from "../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
import { PDF17_EDIT_MODE_ID, PDF17_EDITOR_DIALECT } from "../../🟦️.ts";
import { PDF17_MAIN_BODY_KEY, PDF17_MAIN_WINDOW_KIND_ID, type Pdf17DocumentViewModel, type Pdf17SetPage } from "../../🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️.ts";

const require = createRequire(import.meta.url);
const canvas = require("@napi-rs/canvas");
for (const name of ["DOMMatrix", "ImageData", "Path2D"]) {
  if (!(name in globalThis)) Object.defineProperty(globalThis, name, { value: canvas[name], configurable: true });
}
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
pdfjs.GlobalWorkerOptions.workerSrc = pathToFileURL(require.resolve("pdfjs-dist/legacy/build/pdf.worker.mjs")).href;

/** 📤 Writes one page snapshot to SQLite bytes and reads that snapshot back. */
async function roundTrip(snapshot: PdfSnapshot): Promise<PdfSnapshot> {
  const encoded = await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(snapshot));
  return pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(encoded));
}

test("PDF.js page count and page size survive a TypeScript snapshot edit", async () => {
  const bytes = await Bun.file(new URL("../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf", import.meta.url)).bytes();
  const task = pdfjs.getDocument({ data: bytes });
  try {
    const document = await task.promise;
    const measured: { width: number; height: number; text: string }[] = [];
    for (let index = 1; index <= document.numPages; index += 1) {
      const page = await document.getPage(index);
      const box = page.view;
      const content = await page.getTextContent();
      measured.push({
        width: box[2]! - box[0]!,
        height: box[3]! - box[1]!,
        text: content.items.map(item => "str" in item ? item.str : "").join(""),
      });
    }
    const authored = parsePdfSnapshot({ schema: "stdio.pdf", pages: measured });
    const opened = await roundTrip(authored);
    expect(opened).toEqual(authored);
    expect(opened.pages).toHaveLength(document.numPages);
    const draft: Pdf17SetPage = { page: 0, item: 0, revision: "1", text: "Edited page" };
    const edited = parsePdfSnapshot({
      schema: opened.schema,
      pages: opened.pages.map((page, index) => ({
        width: binary64Value(page.width),
        height: binary64Value(page.height),
        text: index === draft.page ? draft.text : page.text,
      })),
    });
    const saved = await roundTrip(edited);
    expect(saved).toEqual(edited);
    saved.pages.forEach((page, index) => {
      expect(binary64Value(page.width)).toBe(measured[index]!.width);
      expect(binary64Value(page.height)).toBe(measured[index]!.height);
    });
    const view: Pdf17DocumentViewModel = {
      windowKindId: PDF17_MAIN_WINDOW_KIND_ID,
      bodyKey: PDF17_MAIN_BODY_KEY,
      pages: saved.pages.map(page => ({ text: page.text })),
    };
    expect(PDF17_EDITOR_DIALECT).toEqual({ artifactKind: "s.stdio.pdf", standard: "1.7", subset: "*" });
    expect(PDF17_EDIT_MODE_ID).toBe("edit");
    expect(view.windowKindId).toBe("framework.window.document");
    expect(view.pages).toHaveLength(document.numPages);
    expect(view.pages[draft.page]?.text).toBe(draft.text);
    expect(view.pages[1]?.text).toBe(measured[1]!.text);
    expect(draft.item).toBe(0);
  } finally {
    await task.destroy();
  }
});
