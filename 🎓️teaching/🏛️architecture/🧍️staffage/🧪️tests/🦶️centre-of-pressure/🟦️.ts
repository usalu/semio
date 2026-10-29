/** 🦶️ Centre of pressure of a single footprint contact: the sole moves the pressure point under the centre of mass up to half its length, then tips.
 * @see ./🐍️.py — the anastruct oracle that writes `🧫️fixtures/🦶️centre-of-pressure/🔣️.json`
 * @see ./🥒️.feature — the language-agnostic specification */
import { describe, expect, it } from "vitest";
import { buildState, loadCase, near } from "../../🔨️modules/🏗️structure/🧰️harness/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";

const doc = loadCase("🦶️centre-of-pressure");
const values = doc.oracle.values as Record<string, Record<string, number | boolean>>;
const solve = (id: string) => solveStructure(buildState(doc, doc.scenarios.find((s) => s.id === id)!));

describe("centre of pressure against anastruct", () => {
  it("puts the centre of pressure under the centre of mass and balances within half the footprint", () => {
    const result = solve("within");
    const foot = result.reactions[0]!;
    const v = values["within"]!;
    expect(v["clamped"]).toBe(false);
    expect(near(foot.fyN, v["normalN"] as number, 1e-6)).toBe(true);
    expect(near(foot.momentNm, v["baseMomentNm"] as number, 1e-6)).toBe(true);
    expect(near(foot.cop!.x, v["copX"] as number, 1e-6)).toBe(true);
    expect(near(foot.cop!.x, result.centreOfMass.x, 1e-9)).toBe(true);
    expect(Math.hypot(result.residual.fxN, result.residual.fyN)).toBeLessThan(1e-9 * 700);
    expect(Math.abs(result.residual.momentNm)).toBeLessThan(1e-9 * 700);
    expect(near(foot.pressurePa!, (v["normalN"] as number) / (0.25 * 0.1), 1e-6)).toBe(true);
  });
  it("clamps the centre of pressure at the toe beyond half the footprint and reports tipping", () => {
    const result = solve("beyond");
    const foot = result.reactions[0]!;
    const v = values["beyond"]!;
    expect(v["clamped"]).toBe(true);
    expect(near(foot.cop!.x, 0.125, 1e-9)).toBe(true);
    expect(near(foot.cop!.x, v["copX"] as number, 1e-6)).toBe(true);
    expect(near(result.residual.momentNm, -(foot.fyN * ((v["unclampedCopX"] as number) - 0.125)), 1e-6, 1e-6)).toBe(true);
    expect(Math.abs(result.residual.momentNm)).toBeGreaterThan(1);
    expect(result.notes.some((n) => n.en.includes("tips") && n.de.includes("kippt"))).toBe(true);
    expect(result.notes.some((n) => n.en.includes("Unbalanced"))).toBe(true);
  });
  it("states the centre-of-pressure assumption in both languages", () => {
    const notes = solve("within").notes;
    expect(notes.some((n) => n.en.includes("centre of pressure") && n.de.includes("Druckmittelpunkt"))).toBe(true);
  });
});
