/** 🔤️ Caption fit: text widths agree with PyMuPDF's Helvetica, every text of a scene stays inside the scene and sheet captions wrap to their cell.
 *
 * THE REFERENCE is PyMuPDF (🐍️.py).
 *
 * @see ./🥒️.feature — the scenarios
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { beforeAll, describe, expect, it, vi } from "vitest";
import { PLANAR_VIEWPOINTS, type Asset, type Language } from "../../🧬️schema/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { explanationBox } from "../../🔨️modules/🎨️depiction/🪧️explanation/🟦️.ts";
import { captionLines } from "../../🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { RepresentationError } from "../../🔨️modules/🖨️representation/🟦️.ts";
import { ANNOTATION_TEXT_MM, LABEL_TEXT_MM, textWidthMm } from "../../🔨️modules/📐️units/🟦️.ts";
import { STAFFAGE_ROOT } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Text = { text: string; size: number; bold: boolean; font?: "helvetica" | "anta" };
vi.setConfig({ testTimeout: 900_000 });
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/🔤️caption-fit/🐍️.py");
const ANTA = resolve(STAFFAGE_ROOT, "../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta");
let assets: ReadonlyMap<string, Asset>;

beforeAll(async () => {
  assets = new Map((await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT)).assets.map((asset) => [asset.id, asset]));
});

function widths(texts: readonly Text[]): number[] {
  const directory = mkdtempSync(join(tmpdir(), "staffage-"));
  try {
    const file = join(directory, "texts.json");
    writeFileSync(file, JSON.stringify(texts));
    return JSON.parse(python(ORACLE, "documents", [file, ANTA])) as number[];
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
      const mine = textWidthMm(entry.text, entry.size, entry.bold, "helvetica");
      expect(Math.abs(mine - reference[i]!), JSON.stringify(entry)).toBeLessThanOrEqual(Math.max(1e-3 * reference[i]!, 1e-3));
    });
  });
});

describe("text widths in Anta", () => {
  it("agree with PyMuPDF measuring the Anta files within 0.1 %", () => {
    const ascii = Array.from({ length: 95 }, (_, i) => String.fromCharCode(32 + i));
    const latin1 = Array.from({ length: 96 }, (_, i) => String.fromCharCode(160 + i));
    const symbols = ["–", "—", "‘", "’", "„", "“", "”", "…", "−", "≈", "€", "•"];
    const strings = ["Wandhängender Brennwert-Gaskessel", "1:50 · Draufsicht · 12,5 kPa", "Größe in °C · Schnitt A-A µ ± ×", "0123456789 11:11 00:00", "Außenwand überdämmt"];
    const texts: Text[] = [...ascii, ...latin1, ...symbols, ...strings].flatMap((text) => [false, true].map((bold) => ({ text, size: 10, bold, font: "anta" as const })));
    const reference = widths(texts);
    texts.forEach((entry, i) => {
      const mine = textWidthMm(entry.text, entry.size, entry.bold);
      expect(Math.abs(mine - reference[i]!), JSON.stringify(entry)).toBeLessThanOrEqual(Math.max(1e-3 * reference[i]!, 1e-3));
    });
  });

  it("is the face the SVG writer names first, so a text measured here is the text written there", () => {
    const asset = assets.get("gas-hob")!;
    const selector = { kind: "scale", scale: { denominator: 100 } } as const;
    const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, selector);
    const scene = depict(state, { ...defaultDepictionOptions("dark-line", "en", 100, "plan"), representation: selector });
    const families = new Set([...writeSvg(scene).matchAll(/<text [^>]*font-family="([^"]*)"/gu)].map((match) => match[1]!.split(",")[0]));
    expect([...families]).toEqual(["Anta"]);
    expect(textWidthMm("2000", 10)).toBeGreaterThan(textWidthMm("2000", 10, false, "helvetica"));
  });
});

describe("scene text", () => {
  const cases: [string, number][] = [["gas-boiler", 200], ["gas-hob", 200], ["gas-hob", 100], ["ceiling-diffuser", 50]];
  it("stays inside every scene", () => {
    let drawn = 0;
    const entries: { name: string; width: number; x: number; anchor: string; text: Text }[] = [];
    for (const [id, denominator] of cases)
      for (const viewpoint of PLANAR_VIEWPOINTS)
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
          for (const layer of scene.layers) for (const item of layer.items) if (item.kind === "text") entries.push({ name: `${id} 1:${denominator} ${viewpoint} ${language} "${item.text}"`, width: scene.widthMm, x: item.x, anchor: item.anchor, text: { font: "anta", text: item.text, size: item.sizeMm, bold: item.weight === "bold" } });
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

describe("explanation boxes", () => {
  it("wrap every line inside the frame in the face they are written in", () => {
    const asset = [...assets.values()][0]!;
    const reason = { en: "The 0.15 mm gaps between the 2000 mm slats 100 mm apart are 0.003 mm on paper and close at every printable pen 0000 1111 8888 2000 2000 2000 2000 2000 2000 2000 2000 2000 2000", de: "Die Lücken von 0,15 mm zwischen den 2000 mm langen Latten im Abstand von 100 mm messen 0,003 mm auf dem Papier und schließen sich bei jeder druckbaren Feder 0000 1111 8888 2000 2000 2000 2000 2000 2000 2000 2000 2000 2000" };
    const resolution = { kind: "unavailable", requested: 2000, cause: "outside-valid-range", reason, limitingFeature: { en: "slat 2000 x 8 mm", de: "Latte 2000 x 8 mm" }, limitingSizeM: 0.008, supported: [50, 100, 200], alternatives: [{ kind: "scale", denominator: 200 }] } as const;
    const entries: { name: string; width: number; text: Text }[] = [];
    for (const language of ["en", "de"] as Language[])
      for (const width of [50, 60, 80]) {
        const box = explanationBox(asset, resolution, language, width);
        for (const line of box.lines) entries.push({ name: `${language} ${width} mm "${line.text}"`, width, text: { font: "anta", text: line.text, size: line.bold ? ANNOTATION_TEXT_MM : LABEL_TEXT_MM, bold: line.bold } });
      }
    expect(entries.length).toBeGreaterThan(20);
    const reference = widths(entries.map((entry) => entry.text));
    entries.forEach((entry, i) => {
      if (entry.text.text.includes(" ")) expect(2 + reference[i]!, entry.name).toBeLessThanOrEqual(entry.width - 2 + 1e-6);
    });
  });
});

describe("sheet captions", () => {
  it("wrap to the cell width", () => {
    const caption = "light-and-burn@150 · Section A-A through the middle seat and the back cushion";
    for (const cell of [20, 35, 60]) {
      const lines = captionLines(caption, cell);
      const reference = widths(lines.map((text) => ({ text, size: 2.5, bold: false, font: "helvetica" as const })));
      lines.forEach((line, i) => {
        if (line.includes(" ")) expect(reference[i]!, `${cell} "${line}"`).toBeLessThanOrEqual(cell * 1.001);
      });
      expect(lines.join(" ")).toBe(caption);
    }
  });
});
