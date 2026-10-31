/** 📐️ Paper size of a 1.80 m figure at the four presets, parsed back from the written SVG and cross-checked by librsvg pixels.
 * @see ./🥒️.feature */
import { execFileSync, spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { figureState } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/📐️paper-dimensions/🔣️.json", import.meta.url), "utf8")) as { scales: { denominator: number; figureMm: number }[]; pixelsPerMm: number; pixelTolerance: number };
const options = (denominator: number) => {
  const o = defaultDepictionOptions("light-line", "en", denominator);
  return { ...o, annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: false } };
};
const figureD = (svg: string): number[] => (/<g id="shape-figure"[^>]*>(?:<title>[^<]*<\/title>)?<path d="([^"]+)"/u.exec(svg)![1]!.match(/-?\d+(?:\.\d+)?/gu) ?? []).map(Number);
const hasRsvg = spawnSync("rsvg-convert", ["--version"]).status === 0;

describe("paper dimensions", () => {
  for (const { denominator, figureMm } of fixture.scales) {
    it(`draws 1.80 m as ${figureMm} mm at 1:${denominator}`, () => {
      const scene = depict(figureState(denominator), options(denominator));
      const svg = writeSvg(scene);
      const ys = figureD(svg).filter((_, i) => i % 2 === 1);
      expect(Math.max(...ys) - Math.min(...ys)).toBeCloseTo(figureMm, 3);
      const width = Number(/<svg [^>]*width="([\d.]+)mm"/u.exec(svg)![1]);
      expect(width).toBeCloseTo(scene.widthMm, 3);
      expect(/viewBox="0 0 ([\d.]+) ([\d.]+)"/u.exec(svg)!.slice(1).map(Number)).toEqual([scene.widthMm, scene.heightMm].map((v) => Number(v.toFixed(4))));
      expect(scene.heightMm).toBeCloseTo(figureMm + 2 * 5, 3);
    });
    it.skipIf(!hasRsvg)(`rasterises to ${figureMm} mm of pixels at 1:${denominator}`, () => {
      const dir = mkdtempSync(join(tmpdir(), "staffage-"));
      writeFileSync(join(dir, "f.svg"), writeSvg(depict(figureState(denominator), options(denominator))));
      execFileSync("rsvg-convert", ["-i", "shape-figure", "-x", String(fixture.pixelsPerMm), "-y", String(fixture.pixelsPerMm), "-f", "png", "-o", join(dir, "f.png"), join(dir, "f.svg")]);
      const png = readFileSync(join(dir, "f.png"));
      expect(Math.abs(png.readUInt32BE(20) - figureMm * fixture.pixelsPerMm)).toBeLessThanOrEqual(fixture.pixelTolerance);
    });
  }
  it("places the model origin so that (x, y) maps to paper", () => {
    const scene = depict(figureState(100), options(100));
    expect(scene.metadata.origin.xMm).toBeCloseTo(5, 6);
    expect(scene.metadata.origin.yMm).toBeCloseTo(5 + 18, 6);
    expect(scene.metadata.mmPerMetre).toBe(10);
  });
});
