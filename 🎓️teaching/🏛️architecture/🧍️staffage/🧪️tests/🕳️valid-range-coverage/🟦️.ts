/** 🕳️ The drawing-coverage contract (`valid-range-gap`): the valid ranges of an asset's authored representations cover the promised preset bands without a hole.
 *
 * THE REFERENCE is shapely (`🐍️.py`): the same vectors as one-dimensional geometry on the denominator axis, promised bands minus valid ranges.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🕳️valid-range-coverage/🔣️.json — the vectors
 * @see ../../🔨️modules/🗂️catalog/🟦️.ts — `uncoveredDenominators` */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { SCALE_BANDS, SCALE_DENOMINATORS } from "../../🧬️schema/🟦️.ts";
import type { Asset, Representation } from "../../🧬️schema/🟦️.ts";
import { uncoveredDenominators, validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { STAFFAGE_ROOT, fixture } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Range = readonly [number, number] | "unavailable";
type Vectors = { bands: Record<string, [number, number]>; cases: { name: string; ranges: Record<string, Range>; gaps: [number, number, number][] }[] };

const vectors = fixture<Vectors>("🕳️valid-range-coverage");
const sample: Asset = fixture<{ asset: Asset }>("👤️sample-figure").asset;

function withRanges(ranges: Record<string, Range>): Asset {
  const asset = structuredClone(sample) as { -readonly [K in keyof Asset]: Asset[K] };
  const representations: Record<string, Representation> = { ...asset.representations };
  for (const [key, range] of Object.entries(ranges)) {
    const authored = representations[key]!;
    representations[key] =
      range === "unavailable"
        ? ({ status: "unavailable", denominator: authored.denominator, cause: "below-printable-feature", reason: { en: "Vector case.", de: "Vektorfall." }, limitingFeature: { en: "Vector case.", de: "Vektorfall." }, limitingSizeM: 0.01, alternatives: [{ kind: "scale", denominator: 50 }] } as unknown as Representation)
        : ({ ...authored, validRange: { minDenominator: range[0], maxDenominator: range[1] } } as Representation);
  }
  asset.representations = representations as Asset["representations"];
  return asset as Asset;
}

describe("🕳️ valid-range coverage", () => {
  it("pins the bands in schema, implementation and vectors, tiling 1:20…1:1000", () => {
    const schema = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "🧬️schema/🔣️.json"), "utf8"));
    expect(schema.$defs.ScaleBands.const).toEqual(SCALE_BANDS);
    expect(vectors.bands).toEqual(SCALE_BANDS);
    const ordered = [...SCALE_DENOMINATORS].sort((p, q) => p - q).map((denominator) => SCALE_BANDS[`${denominator}`]);
    expect(ordered[0]![0]).toBe(20);
    expect(ordered[ordered.length - 1]![1]).toBe(1000);
    ordered.forEach((band, i) => {
      expect(band[0]).toBeLessThan(band[1]);
      if (i > 0) expect(band[0]).toBe(ordered[i - 1]![1]);
    });
    for (const denominator of SCALE_DENOMINATORS) expect(SCALE_BANDS[`${denominator}`][0] <= denominator && denominator <= SCALE_BANDS[`${denominator}`][1]).toBe(true);
  });

  for (const c of vectors.cases) {
    it(`finds the holes of: ${c.name}`, () => {
      expect(uncoveredDenominators(withRanges(c.ranges)).map((gap) => [gap.from, gap.to, gap.border])).toEqual(c.gaps);
    });
    it(`files one finding per hole for: ${c.name}`, () => {
      const issues = validateAsset(withRanges(c.ranges)).filter((issue) => issue.code === "valid-range-gap");
      expect(issues.map((issue) => [issue.path, issue.data?.from, issue.data?.to])).toEqual(c.gaps.map(([from, to, border]) => [`representations.${border}.validRange`, from, to]));
      for (const issue of issues) expect(issue.message.en.length * issue.message.de.length).toBeGreaterThan(0);
    });
  }

  it("agrees with shapely on every case", () => {
    const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🕳️valid-range-coverage/🐍️.py"), "geometry", [join(STAFFAGE_ROOT, "🧫️fixtures/🕳️valid-range-coverage/🔣️.json")])) as Record<string, [number, number][]>;
    for (const c of vectors.cases) expect(reference[c.name], c.name).toEqual(c.gaps.map(([from, to]) => [from, to]));
  });

  it("finds no hole in the catalog", () => {
    const root = join(STAFFAGE_ROOT, "🗂️catalog");
    const holes: string[] = [];
    for (const category of readdirSync(root).filter((name) => statSync(join(root, name)).isDirectory()))
      for (const slug of readdirSync(join(root, category)).filter((name) => statSync(join(root, category, name)).isDirectory())) {
        const file = join(root, category, slug, "🔣️.json");
        let asset: Asset;
        try {
          asset = (JSON.parse(readFileSync(file, "utf8")) as { asset: Asset }).asset;
        } catch {
          continue;
        }
        if (asset?.representations === undefined) continue;
        for (const gap of uncoveredDenominators(asset)) holes.push(`${asset.category}/${asset.id} 1:${gap.from}…1:${gap.to}`);
      }
    expect(holes).toEqual([]);
  });
});
