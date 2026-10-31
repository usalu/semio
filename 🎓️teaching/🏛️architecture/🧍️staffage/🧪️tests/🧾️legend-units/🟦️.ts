/** 🧾️ Every analytic scene carries legends with unit, sign convention and interpretation in English and German.
 * @see ./🥒️.feature */
import { describe, expect, it } from "vitest";
import { VIEWS } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depictAll } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { sampleState, unchecked } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";

const plainOptions = (...args: Parameters<typeof defaultDepictionOptions>) => unchecked(defaultDepictionOptions(...args));
describe("legend units", () => {
  const scenes = depictAll(sampleState("carry-bag", 50), plainOptions("forces", "de", 50));
  for (const view of ["thermal", "forces", "moments"] as const) {
    it(`states unit, convention and interpretation in ${view}`, () => {
      expect(scenes[view].legends.length).toBeGreaterThan(0);
      for (const l of scenes[view].legends) {
        expect(l.unit.length).toBeGreaterThan(0);
        for (const t of [l.title, l.convention, l.interpretation]) {
          expect(t.en.length).toBeGreaterThan(0);
          expect(t.de.length).toBeGreaterThan(0);
        }
        expect(l.stops.length).toBeGreaterThan(1);
        expect(l.max).toBeGreaterThanOrEqual(l.min);
        for (const s of l.stops) expect(s.at).toBeGreaterThanOrEqual(0), expect(s.at).toBeLessThanOrEqual(1);
      }
      expect(scenes[view].layers.some((l) => l.id === "legend")).toBe(true);
    });
  }
  it("has no legends in plain views", () => {
    for (const view of VIEWS.filter((v) => !["thermal", "forces", "moments"].includes(v))) expect(scenes[view].legends).toEqual([]);
  });
  it("draws the conditions and assumptions of a legend as their own lines", () => {
    const scene = depictAll(sampleState("carry-bag", 50), plainOptions("thermal", "en", 50)).thermal;
    const texts = scene.layers.find((l) => l.id === "legend")!.items.flatMap((i) => (i.kind === "text" ? [i.text] : []));
    const withConditions = scene.legends.filter((l) => l.conditions !== undefined);
    const withAssumptions = scene.legends.flatMap((l) => l.assumptions ?? []);
    expect(withConditions.length + withAssumptions.length).toBeGreaterThan(0);
    if (withConditions.length > 0) expect(texts.some((t) => t.startsWith("Conditions:"))).toBe(true);
    if (withAssumptions.length > 0) expect(texts.some((t) => t.startsWith("Assumption:"))).toBe(true);
  });

  it("writes the legend text in the requested language", () => {
    const texts = scenes.forces.layers.find((l) => l.id === "legend")!.items.flatMap((i) => (i.kind === "text" ? [i.text] : []));
    expect(texts.some((t) => t.includes("Normalkraft"))).toBe(true);
  });
  it("draws analytic views on dark paper with light ink when asked", () => {
    const dark = depictAll(sampleState("carry-bag", 50), { ...plainOptions("forces", "en", 50, "elevation", "dark") });
    for (const view of ["thermal", "forces", "moments"] as const) {
      expect(dark[view].background).toBe("#001117");
      const text = dark[view].layers.flatMap((l) => l.items).find((i) => i.kind === "text");
      expect(text && text.kind === "text" && text.color).toBe("#f7f3e3");
    }
  });
  it("keeps legend ticks at least 6 mm apart", () => {
    const groups = new Map<string, number[]>();
    for (const i of scenes.forces.layers.find((l) => l.id === "legend")!.items)
      if (i.kind === "text" && i.id.includes(":tick-label-")) groups.set(i.id.split(":tick-label-")[0]!, [...(groups.get(i.id.split(":tick-label-")[0]!) ?? []), i.x]);
    expect(groups.size).toBeGreaterThan(0);
    for (const xs of groups.values()) xs.sort((a, b) => a - b).forEach((x, n) => n > 0 && expect(x - xs[n - 1]!).toBeGreaterThanOrEqual(6 - 1e-6));
  });
});
