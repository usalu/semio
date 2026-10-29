/** 🎨️ Structural legend: stops are normalised 0…1 over [min, max], zero grey sits at its normalised position, compression warm, tension cool.
 * @see ../../🔨️modules/🏗️structure/🟦️.ts */
import { describe, expect, it } from "vitest";
import type { StructuralResult } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { STRUCTURAL_COLORS, structuralLegend } from "../../🔨️modules/🏗️structure/🟦️.ts";

const station = (axialN: number) => ({ s: 0, at: { x: 0, y: 0 }, axialN, shearN: 0, momentNm: 0, stressMinPa: null, stressMaxPa: null });
const result = { reactions: [{ anchor: "a", at: { x: 0, y: 0 }, fxN: 0, fyN: 0, momentNm: 0, pressurePa: 5000, active: true }], members: { m: { bone: "m", stations: [station(-300), station(100)], deflection: null } } } as unknown as StructuralResult;

describe("structural legend", () => {
  it("normalises a diverging ramp with grey at the normalised zero", () => {
    const legend = structuralLegend("axial", result, { mode: "auto" });
    expect([legend.min, legend.max]).toEqual([-300, 100]);
    expect(legend.stops.every((s) => s.at >= 0 && s.at <= 1)).toBe(true);
    expect(legend.stops.map((s) => s.at)).toEqual([...legend.stops.map((s) => s.at)].sort((a, b) => a - b));
    expect(legend.stops[0]).toEqual({ at: 0, color: STRUCTURAL_COLORS.compressionStrong });
    expect(legend.stops.at(-1)).toEqual({ at: 1, color: STRUCTURAL_COLORS.tensionStrong });
    expect(legend.stops.find((s) => s.color === STRUCTURAL_COLORS.zero)!.at).toBeCloseTo(0.75, 12);
  });
  it("honours a fixed range and a one-sided pressure ramp", () => {
    const fixed = structuralLegend("axial", result, { mode: "fixed", min: -100, max: 100 });
    expect(fixed.stops.find((s) => s.color === STRUCTURAL_COLORS.zero)!.at).toBeCloseTo(0.5, 12);
    const pressure = structuralLegend("contact-pressure", result, { mode: "auto" });
    expect(pressure.stops[0]).toEqual({ at: 0, color: STRUCTURAL_COLORS.zero });
    expect(pressure.stops.at(-1)).toEqual({ at: 1, color: STRUCTURAL_COLORS.compressionStrong });
    expect(pressure.unit).toBe("Pa");
  });
  it("collapses a zero span to the grey stop", () => {
    const legend = structuralLegend("shear", { reactions: [], members: {} } as unknown as StructuralResult, { mode: "auto" });
    expect(legend.stops).toEqual([{ at: 0, color: STRUCTURAL_COLORS.zero }]);
  });
});
