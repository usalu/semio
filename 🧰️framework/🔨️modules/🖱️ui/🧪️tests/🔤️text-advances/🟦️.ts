/** 🔤️ The text-advance corpus law, React side: Chromium — the engine React's text runs through — measures every committed corpus
 * row, with kerning off and with its default kerning, to exactly the two widths the neutral fixture records, so the fixture the
 * wgpu atlas answers to (`🔬️targets-wgpu-text-unit`) is Chromium's own and never a hand-copied number.
 *
 * 🧫️ Read from `🖱️ui/🧫️fixtures/🔤️text-advances/🔣️.json` (schema `🖱️ui/🧬️schema/🔤️text-advances/🔣️.json`).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import { chromium } from "playwright";
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🔤️text-advances/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔤️text-advances/🔣️.json" with { type: "json" };

const fonts = resolve(dirname(fileURLToPath(import.meta.url)), "../../../🖼️assets/🔤️fonts");
const faceFiles: Record<string, string> = {
  sans: resolve(fonts, "🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf"),
  mono: resolve(fonts, "⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf"),
};

describe("🔤️ text advance corpus", () => {
  it("declares its contract", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("is Chromium's own unkerned and kerned DOM width for every row", async () => {
    const faces = fixture.faces as Record<string, string>;
    const css = Object.entries(faceFiles)
      .map(([face, file]) => `@font-face{font-family:"${faces[face]}";src:url(data:font/ttf;base64,${readFileSync(file).toString("base64")})}`)
      .join("");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      await page.setContent(`<style>${css} span{white-space:pre}</style>`);
      const widths = await page.evaluate(
        async ({ faces, rows }) => {
          for (const family of Object.values(faces)) await document.fonts.load(`16px "${family}"`);
          const width = (row: { face: string; sizePx: number; text: string }, kerning: string) => {
            const span = document.createElement("span");
            span.style.cssText = `font-family:"${faces[row.face]}";font-size:${row.sizePx}px;font-kerning:${kerning}`;
            span.textContent = row.text;
            document.body.appendChild(span);
            const measured = span.getBoundingClientRect().width;
            span.remove();
            return measured;
          };
          return rows.map((row) => ({ unkerned: width(row, "none"), kerned: width(row, "normal") }));
        },
        { faces, rows: fixture.rows },
      );
      fixture.rows.forEach((row, index) => {
        expect(widths[index].unkerned, `unkerned ${row.face} ${row.sizeToken} ${row.text}`).toBeCloseTo(row.unkernedWidthPx, 3);
        expect(widths[index].kerned, `kerned ${row.face} ${row.sizeToken} ${row.text}`).toBeCloseTo(row.kernedWidthPx, 3);
      });
    } finally {
      await browser.close();
    }
  }, 60_000);
});
