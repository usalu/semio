/** 📕️ PDF export reopened with pikepdf (qpdf's engine), PDFium and reportlab's AFM metrics, all from the hash-locked wheels of the oracle environment. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { helveticaWidth, PT_PER_MM, winAnsiBytes, writePdf } from "../../🔨️modules/📤️export/📕️pdf/🟦️.ts";
import { drawingScene, pixel, requireTool, STAFFAGE_ROOT, withScratch } from "../🧰️support/🧰️.ts";
import { inspectPdf, pdfText, python, renderPdf } from "../🧰️support/🐍️.ts";
requireTool("uv");

const second = (): ReturnType<typeof drawingScene> => ({ ...drawingScene(), widthMm: 100, heightMm: 30 });

describe("📕️ writePdf", () => {
  it("passes the qpdf structure and stream check and reports both pages with MediaBox = size in points", async () => {
    await withScratch((directory) => {
      const file = join(directory, "staffage.pdf");
      writeFileSync(file, writePdf([drawingScene(), second()]));
      const report = inspectPdf(file);
      expect(report.problems).toEqual([]);
      expect(report.version).toBe("1.7");
      expect(report.pages).toBe(2);
      expect(report.title).toBe("Walking commuter / Gehender Pendler");
      expect(report.subject).toMatch(/^Staffage walking-commuter, scale 1:100 \(10 mm\/m\), t = 1\.25 s/u);
      expect(report.subject).toMatch(/representation Dark line \/ Dunkle Strichzeichnung, viewpoint [^,\n]+ \/ Seitenansicht, Physical drawing, to scale \/ Physikalische Zeichnung, ma/u);
      expect(report.subject).not.toMatch(/dark-line/u);
      for (const [page, widthMm, heightMm] of [[1, 80, 50], [2, 100, 30]] as const) {
        const box = report.mediaBoxes[page - 1];
        expect(box, `page ${page} MediaBox`).toBeDefined();
        expect(box![0]).toBe(0);
        expect(box![1]).toBe(0);
        expect(box![2]!).toBeCloseTo(widthMm * PT_PER_MM, 2);
        expect(box![3]!).toBeCloseTo(heightMm * PT_PER_MM, 2);
      }
    });
  });

  it("renders a 1.80 m figure at 1:100 as 18 mm within 0.2 mm at 254 dpi", async () => {
    await withScratch((directory) => {
      const file = join(directory, "figure.pdf");
      writeFileSync(file, writePdf([drawingScene()]));
      const image = renderPdf(file, 254);
      expect(Math.abs(image.width - 800)).toBeLessThanOrEqual(1);
      expect(Math.abs(image.height - 500)).toBeLessThanOrEqual(1);
      let top = Infinity, bottom = -Infinity, left = Infinity, right = -Infinity;
      for (let y = 0; y < image.height; y += 1) for (let x = 0; x < image.width; x += 1) {
        const [r, g, b] = pixel(image, x, y);
        if (r > 200 && g < 60 && b < 60 && x < 300) { top = Math.min(top, y); bottom = Math.max(bottom, y); left = Math.min(left, x); right = Math.max(right, x); }
      }
      expect(((bottom - top + 1) / 10)).toBeGreaterThan(17.8);
      expect(((bottom - top + 1) / 10)).toBeLessThan(18.2);
      expect(((right - left + 1) / 10)).toBeGreaterThan(5.8);
      expect(((right - left + 1) / 10)).toBeLessThan(6.2);
      const hatch = new Set<string>();
      for (let y = 160; y < 320; y += 1) for (let x = 410; x < 490; x += 1) hatch.add(pixel(image, x, y).join(","));
      expect(hatch.size, "tiling pattern paints more than one colour").toBeGreaterThan(2);
      const [r, g, b] = pixel(image, 600, 240);
      expect(r).toBeGreaterThan(110);
      expect(r).toBeLessThan(150);
      expect(b).toBeGreaterThan(250);
      expect(g).toBe(r);
    });
  });

  it("keeps · ² ° through WinAnsi and transliterates glyphs WinAnsi lacks", async () => {
    expect(winAnsiBytes("·²°")).toEqual([0xb7, 0xb2, 0xb0]);
    expect(winAnsiBytes("−5 ≤ ü")).toEqual([0x2d, 0x35, 0x20, 0x3c, 0x3d, 0x20, 0xfc]);
    await withScratch((directory) => {
      const file = join(directory, "text.pdf");
      writeFileSync(file, writePdf([drawingScene()]));
      const text = pdfText(file);
      expect(text).toContain("1,80 m · 2,5 m² 20 °C -5 <= ü");
      expect(text).toContain("Mitte");
      expect(text).toContain("Ende");
    });
  });

  it("measures Helvetica advances like reportlab's Adobe metrics", () => {
    const samples: readonly (readonly [string, boolean])[] = [["The quick brown fox 0123456789 (m/s)", false], ["Walking commuter 1:100", true], ["1,80 m · 2,5 m² 20 °C", false], ["W.i-l|l", false]];
    const oracle = JSON.parse(python(`${STAFFAGE_ROOT}/🧪️tests/📕️pdf-reopen/🐍️.py`, "documents", [JSON.stringify(samples)])) as number[];
    samples.forEach(([text, bold], index) => expect(helveticaWidth(text, bold)).toBeCloseTo(oracle[index]!, 6));
  });

  it("is deterministic, reports progress and honours cancellation", () => {
    expect(writePdf([drawingScene()])).toEqual(writePdf([drawingScene()]));
    const seen: number[] = [];
    writePdf([drawingScene(), second()], { progress: (done, total) => seen.push(done * 10 + total) });
    expect(seen).toEqual([12, 22]);
    const controller = new AbortController();
    controller.abort();
    expect(() => writePdf([drawingScene()], { signal: controller.signal })).toThrow();
  });
});
