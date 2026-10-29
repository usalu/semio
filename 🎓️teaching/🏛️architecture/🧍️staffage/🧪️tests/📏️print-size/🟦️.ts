/** 📏️ Paper size, level of detail, line weights and quantity formatting; oracle: d3-scale for the linear metre → millimetre map, d3-format for the number rendering.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/📏️print-size/🔣️.json — the vectors
 */
import { readFileSync } from "node:fs";
import { scaleLinear } from "d3-scale";
import { format } from "d3-format";
import { describe, expect, it } from "vitest";
import type { ShapeRole } from "../../🧬️schema/🟦️.ts";
import { SCALE_PRESETS, formatQuantity, lineWeightMm, lodFor, mmPerMetre, presetIndex, textSizeMm } from "../../🔨️modules/📐️units/🟦️.ts";
import type { QuantityUnit } from "../../🔨️modules/📐️units/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/📏️print-size/🔣️.json", import.meta.url), "utf8"));

describe("📏️ print size", () => {
  it("converts 1.80 m to the paper millimetres of the fixture and agrees with d3-scale", () => {
    for (const { denominator, mm } of fixture.sizesMm) {
      expect(fixture.heightM * mmPerMetre({ denominator })).toBeCloseTo(mm, 9);
      expect(scaleLinear().domain([0, 1]).range([0, 1000 / denominator])(fixture.heightM)).toBeCloseTo(mm, 9);
    }
  });

  it("chooses the level of detail by denominator", () => {
    for (const { denominator, lod } of fixture.lod) expect(lodFor({ denominator })).toBe(lod);
  });

  it("looks up ISO 128 line weights per role and preset", () => {
    for (const [denominator, roles] of Object.entries(fixture.lineWeightsMm) as [string, Record<ShapeRole, number | null>][])
      for (const [role, mm] of Object.entries(roles) as [ShapeRole, number | null][]) expect(lineWeightMm({ denominator: Number(denominator) }, role)).toBe(mm);
  });

  it("snaps custom scales to the nearest preset by log distance", () => {
    for (const { denominator, preset } of fixture.snapping) expect(SCALE_PRESETS[presetIndex({ denominator })]).toBe(preset);
    expect(lineWeightMm({ denominator: 70 }, "body")).toBe(0.5);
    expect(lineWeightMm({ denominator: 320 }, "body")).toBe(0.18);
  });

  it("formats quantities with sensible prefixes in English and German", () => {
    for (const { value, unit, en, de } of fixture.quantities) {
      expect(formatQuantity(value, unit as QuantityUnit, "en")).toBe(en.replace(" ", " "));
      expect(formatQuantity(value, unit as QuantityUnit, "de")).toBe(de.replace(" ", " "));
    }
  });

  it("renders the same digits as d3-format for three significant digits", () => {
    expect(Number(formatQuantity(12.3456, "W", "en").split(" ")[0])).toBe(Number(format(".3r")(12.3456)));
    expect(Number(formatQuantity(0.045678, "W", "en").split(" ")[0])).toBe(Number(format(".3r")(0.045678)));
  });

  it("keeps text sizes constant in paper space", () => {
    expect(textSizeMm("annotation")).toBe(fixture.text.annotationMm);
    expect(textSizeMm("label")).toBe(fixture.text.labelMm);
  });
});
