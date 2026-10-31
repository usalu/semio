/** 🎨️ Thermal view: every quantity paints region values computed before colour, merged regions are area-weighted, legends carry quantity, unit, range, sign convention, conditions and assumptions,
 * shared ranges are identical across assets, `auto` is labelled normalised, colours are monotone warm-above-centre. */
import { describe, expect, it } from "vitest";
import environments from "../../🗂️catalog/🌤️environments/🔣️.json" with { type: "json" };
import type { Environment } from "../../🧬️schema/🟦️.ts";
import { THERMAL_QUANTITIES, type DrawingItem, type DrawingScene, type DepictionOptions } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { catalogAsset } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { UNCHECKED_PROFILE } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { QUANTITY_SPECS, solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";

const human = catalogAsset("walking-commuter");
const car = catalogAsset("city-car");
const cold = environments.presets.find((p) => p.id === "winter-overcast")!.environment as Environment;
const items = (s: DrawingScene): DrawingItem[] => s.layers.flatMap((l) => l.items);
const options = (denominator: number, quantity: DepictionOptions["thermalQuantity"], range: DepictionOptions["range"] = { mode: "shared" }): DepictionOptions => ({ ...defaultDepictionOptions("thermal", "en", denominator), profile: UNCHECKED_PROFILE, thermalQuantity: quantity, range, environment: cold });
const scene = (asset = human, denominator = 100, quantity: DepictionOptions["thermalQuantity"] = "surface-temperature", range?: DepictionOptions["range"], pose = "standing") => {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator } });
  const field = solveThermal(state, cold);
  return { state, field, scene: depict(state, options(denominator, quantity, range), { thermal: field }) };
};

describe("🎨️ thermal view", () => {
  it.each(THERMAL_QUANTITIES)("labels %s with its quantity, unit, range, convention, conditions and assumptions", (quantity) => {
    const { scene: s } = scene(human, 100, quantity);
    const legend = s.legends[0]!;
    const spec = QUANTITY_SPECS[quantity];
    expect(legend.quantity).toBe(quantity);
    expect(legend.unit).toBe(spec.unit);
    expect(legend.range).toBe("shared");
    expect(legend.min).toBe(spec.shared.min);
    expect(legend.max).toBe(spec.shared.max);
    expect(legend.conditions!.en).toContain("2.0 °C");
    expect(legend.conditions!.de).toContain("Luft");
    expect(legend.assumptions!.length).toBeGreaterThanOrEqual(2);
    expect(legend.convention.en).toContain("Shared comparison range");
    expect(legend.convention.de).toContain("Gemeinsamer Vergleichsbereich");
    expect(legend.interpretation.en).toContain("area-weighted");
  });

  it("paints values computed before colour: every region-bound fill carries the solver value of its region", () => {
    for (const quantity of THERMAL_QUANTITIES) {
      const { scene: s, field } = scene(human, 100, quantity);
      const spec = QUANTITY_SPECS[quantity];
      const painted = items(s).filter((i): i is Extract<DrawingItem, { kind: "path" }> => i.kind === "path" && i.id.startsWith("shape:") && i.region !== undefined && i.value !== undefined);
      expect(painted.length).toBeGreaterThan(5);
      for (const p of painted) {
        const f = field.regions[p.region!]!;
        expect(p.value!.value).toBeCloseTo(spec.pick(f) ?? f.surfaceTempC, 9);
        expect(p.value!.unit).toBe(spec.unit);
        expect(p.value!.quantity).toBe(quantity);
      }
    }
  });

  it("shows the skin beneath the coat and the coat surface as different values", () => {
    const skin = items(scene(human, 100, "skin-temperature").scene).filter((i) => i.kind === "path" && i.region === "coat" && i.value);
    const surface = items(scene(human, 100, "surface-temperature").scene).filter((i) => i.kind === "path" && i.region === "coat" && i.value);
    expect(skin.length).toBeGreaterThan(0);
    expect((skin[0] as { value: { value: number } }).value.value - (surface[0] as { value: { value: number } }).value.value).toBeGreaterThan(8);
  });

  it("area-weights the merged regions of the 1:500 silhouette and says so", () => {
    const { scene: s, field, state } = scene(human, 500, "surface-temperature");
    const merged = items(s).find((i) => i.kind === "path" && i.id === "shape:sil-torso" && i.value);
    expect(merged).toBeDefined();
    const ids = ["coat", "face-skin", "hair", "scarf", "hand-skin"];
    const regions = state.asset.thermal.regions;
    const weight = ids.reduce((sum, id) => sum + regions.find((r) => r.id === id)!.areaM2, 0);
    const expected = ids.reduce((sum, id) => sum + regions.find((r) => r.id === id)!.areaM2 * field.regions[id]!.surfaceTempC, 0) / weight;
    const value = (merged as { value: { value: number; label: { en: string } } }).value;
    expect(value.value).toBeCloseTo(expected, 9);
    expect(value.label.en).toContain("area-weighted");
  });

  it("splits the net radiative exchange into sky, ground and surroundings in the inspector label", () => {
    const { scene: s } = scene(human, 100, "net-radiative-flux");
    const label = items(s).find((i) => i.kind === "path" && i.region === "coat" && i.value)!;
    expect((label as { value: { label: { en: string } } }).value.label.en).toMatch(/sky .*ground .*surroundings/);
  });

  it("uses the same shared range for different assets and labels the auto range as normalised", () => {
    const a = scene(human, 100, "surface-temperature").scene.legends[0]!;
    const b = scene(car, 100, "surface-temperature", undefined, "parked").scene.legends[0]!;
    expect([a.min, a.max]).toEqual([b.min, b.max]);
    const auto = scene(human, 100, "surface-temperature", { mode: "auto" }).scene.legends[0]!;
    expect(auto.range).toBe("auto");
    expect(auto.convention.en).toContain("AUTO");
    expect(auto.convention.en).toContain("normalised");
    expect(auto.convention.de).toContain("normiert");
    expect(auto.max - auto.min).toBeLessThan(a.max - a.min);
  });

  it("colours warmer values warmer and centres the scale on the air temperature or zero", () => {
    const l = scene(human, 100, "surface-temperature").scene.legends[0]!;
    const centre = l.stops.find((s) => Math.abs(l.min + s.at * (l.max - l.min) - 2) < 1e-9);
    expect(centre).toBeDefined();
    const flux = scene(human, 100, "sensible-heat-flux").scene.legends[0]!;
    expect(flux.stops.some((s) => Math.abs(flux.min + s.at * (flux.max - flux.min)) < 1e-9)).toBe(true);
    expect(l.stops[0]!.at).toBe(0);
    expect(l.stops[l.stops.length - 1]!.at).toBe(1);
    expect(l.stops[0]!.color).not.toBe(l.stops[l.stops.length - 1]!.color);
  });
});
