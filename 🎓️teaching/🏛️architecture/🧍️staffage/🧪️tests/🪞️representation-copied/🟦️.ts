/** 🪞️ Copy detection between representations and the plausibility of paper quantities; oracle: `structuredClone` + vitest deep equality for the unchanged remainder, hand-made mutations of the fixture figure.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/👤️sample-figure/🔣️.json — the figure
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Asset, AssetDocument, AuthoredRepresentation, Shape } from "../../🧬️schema/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { compareRepresentations } from "../../🔨️modules/🖨️representation/🟦️.ts";

const original = (JSON.parse(readFileSync(new URL("../../🧫️fixtures/👤️sample-figure/🔣️.json", import.meta.url), "utf8")) as AssetDocument).asset;
const codes = (asset: Asset): string[] => validateAsset(asset, {}).map((issue) => issue.code);
const edit = (change: (asset: any) => void): Asset => {
  const copy: any = structuredClone(original);
  change(copy);
  return copy;
};
const rep = (asset: Asset, key: string): AuthoredRepresentation => asset.representations[key as "100"] as AuthoredRepresentation;

const scaleShape = (shape: any, f: number): Shape => {
  const s = { ...shape };
  for (const key of ["x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "x", "y", "w", "h", "r"]) if (typeof s[key] === "number") s[key] *= f;
  if (s.points) s.points = s.points.map(([x, y]: number[]) => [x * f, y * f]);
  if (s.d) s.d = s.d.replace(/-?\d*\.?\d+(?:e-?\d+)?/giu, (n: string) => String(Number(n) * f));
  return s;
};

describe("🪞️ representation copies", () => {
  it("accepts a figure whose scales are separately authored", () => {
    const found = codes(original);
    for (const code of ["representation-copied", "valid-range-overlap", "pen-units", "limiting-size"]) expect(found).not.toContain(code);
    expect(compareRepresentations(rep(original, "200"), rep(original, "100")).kind).toBe("distinct");
  });

  it("flags an identical copy", () => {
    const asset = edit((a) => (a.representations["100"].geometry = structuredClone(a.representations["50"].geometry)));
    expect(compareRepresentations(rep(asset, "100"), rep(asset, "50")).kind).toBe("identical");
    expect(codes(asset)).toContain("representation-copied");
  });

  it("flags a finer drawing with shapes deleted", () => {
    const asset = edit((a) => (a.representations["100"].geometry = { elevation: structuredClone(a.representations["50"].geometry.elevation.slice(0, 6)) }));
    const similarity = compareRepresentations(rep(asset, "100"), rep(asset, "50"));
    expect(similarity.kind).toBe("subset");
    expect(similarity.sharedShapes).toBe(6);
    expect(codes(asset)).toContain("representation-copied");
  });

  it("flags a scaled copy", () => {
    const asset = edit((a) => (a.representations["200"].geometry = { elevation: a.representations["100"].geometry.elevation.map((shape: Shape) => scaleShape(shape, 2)) }));
    expect(compareRepresentations(rep(asset, "200"), rep(asset, "100")).kind).toBe("scaled");
    expect(codes(asset)).toContain("representation-copied");
  });

  it("flags overlapping valid ranges and pens in model metres", () => {
    const asset = edit((a) => {
      a.representations["100"].validRange = { minDenominator: 60, maxDenominator: 200 };
      a.representations["100"].pens.body = 0.0003;
    });
    expect(codes(asset)).toEqual(expect.arrayContaining(["valid-range-overlap", "pen-units"]));
  });

  it("flags an unavailable decision without a real limiting size", () => {
    const asset = edit((a) => (a.representations["500"].limitingSizeM = 0));
    expect(codes(asset)).toContain("limiting-size");
  });
});
