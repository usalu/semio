/** 🔤️ Caption fit: text widths agree with PyMuPDF's Helvetica, every text of a scene stays inside the scene and sheet captions wrap to their cell.
 *
 * THE REFERENCE is PyMuPDF (🐍️.py).
 *
 * @see ./🥒️.feature — the scenarios
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { beforeAll, describe, expect, it, vi } from "vitest";
import { VIEWPOINTS, type Asset, type Language } from "../../🧬️schema/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { captionLines } from "../../🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { RepresentationError } from "../../🔨️modules/🖨️representation/🟦️.ts";
import { textWidthMm } from "../../🔨️modules/📐️units/🟦️.ts";
import { STAFFAGE_ROOT, python } from "../🧰️support/🧰️.ts";

type Text = { text: string; size: number; bold: boolean };
vi.setConfig({ testTimeout: 900_000 });
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/🔤️caption-fit/🐍️.py");
let assets: ReadonlyMap<string, Asset>;

beforeAll(async () => {
  assets = new Map((await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT)).assets.map((asset) => [asset.id, asset]));
});

function widths(texts: readonly Text[]): number[] {
  const directory = mkdtempSync(join(tmpdir(), "staffage-"));
  try {
    const file = join(directory, "texts.json");
    writeFileSync(file, JSON.stringify(texts));
    return JSON.parse(python(ORACLE, ["pymupdf"], [file])) as number[];
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

describe("text widths", () => {
  it("agree with PyMuPDF Helvetica within 0.1 %", () => {
    const ascii = Array.from({ length: 95 }, (_, i) => String.fromCharCode(32 + i));
    const german = ["Wandhängender Brennwert-Gaskessel", "Größe in °C · Draufsicht µ", "Schnitt A-A · Dunkle Strichzeichnung", "Außenwand überdämmt"];
    const texts: Text[] = [...ascii, ...german].flatMap((text) => [false, true].map((bold) => ({ text, size: 10, bold })));
    const reference = widths(texts);
    texts.forEach((entry, i) => {
      const mine = textWidthMm(entry.text, entry.size, entry.bold);
      expect(Math.abs(mine - reference[i]!), JSON.stringify(entry)).toBeLessThanOrEqual(Math.max(1e-3 * reference[i]!, 1e-3));
    });
  });
});

describe("scene text", () => {
  const cases: [string, number][] = [["gas-boiler", 200], ["gas-hob", 200], ["gas-hob", 100], ["ceiling-diffuser", 50]];
  it("stays inside every scene", () => {
    let drawn = 0;
    const entries: { name: string; width: number; x: number; anchor: string; text: Text }[] = [];
    for (const [id, denominator] of cases)
      for (const viewpoint of VIEWPOINTS)
        for (const language of ["en", "de"] as Language[]) {
          const asset = assets.get(id)!;
          const selector = { kind: "scale", scale: { denominator } } as const;
          const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, selector);
          const scene = (() => {
            try {
              return depict(state, { ...defaultDepictionOptions("dark-line", language, denominator, viewpoint), representation: selector });
            } catch (error) {
              if (error instanceof RepresentationError) return undefined;
              throw error;
            }
          })();
          if (scene === undefined) continue;
          drawn++;
          for (const layer of scene.layers) for (const item of layer.items) if (item.kind === "text") entries.push({ name: `${id} 1:${denominator} ${viewpoint} ${language} "${item.text}"`, width: scene.widthMm, x: item.x, anchor: item.anchor, text: { text: item.text, size: item.sizeMm, bold: item.weight === "bold" } });
        }
    expect(drawn).toBeGreaterThanOrEqual(12);
    const reference = widths(entries.map((entry) => entry.text));
    entries.forEach((entry, i) => {
      const w = reference[i]!;
      const left = entry.anchor === "start" ? entry.x : entry.anchor === "end" ? entry.x - w : entry.x - w / 2;
      expect(left, entry.name).toBeGreaterThanOrEqual(-1e-6);
      expect(left + w, entry.name).toBeLessThanOrEqual(entry.width + 1e-6);
    });
  });
});

describe("sheet captions", () => {
  it("wrap to the cell width", () => {
    const caption = "light-and-burn@150 · Section A-A through the middle seat and the back cushion";
    for (const cell of [20, 35, 60]) {
      const lines = captionLines(caption, cell);
      const reference = widths(lines.map((text) => ({ text, size: 2.5, bold: false })));
      lines.forEach((line, i) => {
        if (line.includes(" ")) expect(reference[i]!, `${cell} "${line}"`).toBeLessThanOrEqual(cell * 1.001);
      });
      expect(lines.join(" ")).toBe(caption);
    }
  });
});
