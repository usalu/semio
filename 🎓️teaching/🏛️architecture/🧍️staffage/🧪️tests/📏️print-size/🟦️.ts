/** 📏️ Paper size and quantity formatting (level of detail and global pen tables no longer exist: pens belong to each representation); oracle: d3-scale for the linear metre → millimetre map, d3-format for the number rendering.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/📏️print-size/🔣️.json — the vectors
 */
import { readFileSync } from "node:fs";
import { scaleLinear } from "d3-scale";
import { format } from "d3-format";
import { describe, expect, it } from "vitest";
import { formatQuantity, mmPerMetre, textSizeMm } from "../../🔨️modules/📐️units/🟦️.ts";
import type { QuantityUnit } from "../../🔨️modules/📐️units/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/📏️print-size/🔣️.json", import.meta.url), "utf8"));

describe("📏️ print size", () => {
  it("converts 1.80 m to the paper millimetres of the fixture and agrees with d3-scale", () => {
    for (const { denominator, mm } of fixture.sizesMm) {
      expect(fixture.heightM * mmPerMetre({ denominator })).toBeCloseTo(mm, 9);
      expect(scaleLinear().domain([0, 1]).range([0, 1000 / denominator])(fixture.heightM)).toBeCloseTo(mm, 9);
    }
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
