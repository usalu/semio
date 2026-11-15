/** 🕳️ The even-odd fill rule of the staffage in our SVG and PDF writers against shapely, MuPDF and PDFium.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🕳️fill-rule/🔣️.json — the cases
 */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { DrawingScene, PathCommand } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { flattenPath } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { writePdf } from "../../🔨️modules/📤️export/📕️pdf/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { drawingScene, fixture, pixel, requireTool, STAFFAGE_ROOT, withScratch, type Ppm } from "../🧰️support/🧰️.ts";
import { fillRuleExpected, renderPdf, renderSvg, ringOverlaps } from "../🧰️support/🐍️.ts";
requireTool("uv");

type Case = { readonly name: string; readonly rings: readonly (readonly (readonly [number, number])[])[]; readonly areaMm2: number; readonly samples: readonly (readonly [number, number, boolean])[]; readonly selfCrossing?: boolean };
type Fixture = { readonly widthMm: number; readonly heightMm: number; readonly pixelsPerMm: number; readonly cases: readonly Case[] };
const cases = fixture<Fixture>("🕳️fill-rule");
const FILE = join(STAFFAGE_ROOT, "🧫️fixtures/🕳️fill-rule/🔣️.json");

const sceneOf = (c: Case): DrawingScene => ({
  ...drawingScene(),
  metadata: { ...drawingScene().metadata, representation: { kind: "scale", denominator: 100, requested: 100, exact: true }, resolution: "authored", profile: "fill-rule", omittedValues: 0 },
  widthMm: cases.widthMm,
  heightMm: cases.heightMm,
  legends: [],
  layers: [{ id: "fill", title: { en: "Fill", de: "Füllung" }, items: [{ kind: "path", id: "shape", commands: c.rings.flatMap((ring): PathCommand[] => [...ring.map(([x, y], index): PathCommand => ({ op: index === 0 ? "M" : "L", x, y })), { op: "Z" }]), fill: { color: "#ff0000" } }] }],
});

const painted = (image: Ppm): number => { let count = 0; for (let y = 0; y < image.height; y++) for (let x = 0; x < image.width; x++) { const [r, g, b] = pixel(image, x, y); if (r > 128 && g < 128 && b < 128) count++; } return count / cases.pixelsPerMm ** 2; };
const samples = (image: Ppm, c: Case): boolean[] => c.samples.map(([x, y]) => { const [r, g, b] = pixel(image, Math.round(x * cases.pixelsPerMm), Math.round(y * cases.pixelsPerMm)); return r > 128 && g < 128 && b < 128; });

describe("🕳️ even-odd fill rule", () => {
  const expected = fillRuleExpected(FILE);

  for (const [index, c] of cases.cases.entries()) {
    it(`shapely agrees with the fixture: ${c.name}`, () => {
      expect(expected[index]!.areaMm2).toBeCloseTo(c.areaMm2, 2);
      if (expected[index]!.xorAreaMm2 !== null) expect(expected[index]!.xorAreaMm2).toBeCloseTo(c.areaMm2, 6);
      expect(expected[index]!.samples.map((s) => s[2])).toEqual(c.samples.map((s) => s[2]));
    });

    it(`paints the same area in SVG, compact SVG and PDF: ${c.name}`, async () => {
      await withScratch((directory) => {
        const rasters: Record<string, Ppm> = {};
        for (const [name, options] of [["svg", {}], ["compact", { compact: true }]] as const) {
          const file = join(directory, `${name}.svg`);
          writeFileSync(file, writeSvg(sceneOf(c), options));
          rasters[name] = renderSvg(file, cases.pixelsPerMm);
        }
        const pdf = join(directory, "shape.pdf");
        writeFileSync(pdf, writePdf([sceneOf(c)]));
        rasters.pdf = renderPdf(pdf, cases.pixelsPerMm * 25.4);
        for (const [name, image] of Object.entries(rasters)) {
          expect(painted(image) / c.areaMm2, `${name} painted area`).toBeGreaterThan(0.985);
          expect(painted(image) / c.areaMm2, `${name} painted area`).toBeLessThan(1.015);
          expect(samples(image, c), `${name} samples`).toEqual(c.samples.map((s) => s[2]));
        }
      });
    });
  }
});

describe("🕳️ no filled path of the catalog overlaps itself", () => {
  it("has no two subpaths of one filled shape that overlap without one lying inside the other, in any asset, scale or viewpoint", async () => {
    const root = join(STAFFAGE_ROOT, "🗂️catalog");
    const shapes: { asset: string; scale: string; view: string; id: string; rings: number[][][] }[] = [];
    for (const category of readdirSync(root)) for (const slug of readdirSync(join(root, category), { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name)) {
      const asset = JSON.parse(readFileSync(join(root, category, slug, "🔣️.json"), "utf8")).asset;
      for (const [scale, representation] of Object.entries<any>(asset.representations ?? {})) for (const [view, list] of Object.entries<any>(representation.geometry ?? {})) for (const shape of list as any[]) {
        if (shape.kind !== "path" || shape.filled === false) continue;
        const rings = flattenPath(shape.d).rings;
        if (rings.length > 1) shapes.push({ asset: slug, scale, view, id: shape.id, rings: rings.map((ring) => ring.map((p) => [p.x, p.y])) });
      }
    }
    await withScratch((directory) => {
      const file = join(directory, "rings.json");
      writeFileSync(file, JSON.stringify(shapes));
      expect(ringOverlaps(file)).toEqual([]);
    });
  });
});
