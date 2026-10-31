/** 🌡️ Catalog physics: every named pose and the first clip of every catalog asset solves to finite fields under its reference environment; every body sits at its species' core temperature
 * in its default pose; surfaces stay between physical bounds; inactive and passive things stay near the air. */
import { describe, expect, it } from "vitest";
import { catalogAsset, catalogIds } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermal, solveThermalAt } from "../../🔨️modules/🌡️thermal/🟦️.ts";

const CORE: Readonly<Record<string, readonly [number, number]>> = { human: [36.0, 38.0], dog: [37.5, 39.5], cat: [37.5, 39.5], horse: [37.0, 39.2], cow: [37.8, 39.5], deer: [37.5, 40.0], bird: [40.5, 42.5], sheep: [38.5, 40.0], goat: [38.5, 40.0], pig: [38.0, 40.0], fox: [38.0, 40.0], squirrel: [37.5, 39.5], hedgehog: [34.0, 37.5], duck: [40.5, 42.5], gull: [40.5, 42.5] };
/** 🧾️ Content defects reported in the ticket file thermal-v2.md (asset data, not solver): exempt from the surface lower bound until the content worker fixes them. */
const KNOWN_COLD_SURFACE_DEFECTS: ReadonlySet<string> = new Set();
const finite = (value: unknown): boolean => !JSON.stringify(value, (_, v) => (typeof v === "number" && !Number.isFinite(v) ? "NaN" : v)).includes('"NaN"');

describe("🌡️ catalog physics", () => {
  const ids = catalogIds();
  it("has assets to solve", () => expect(ids.length).toBeGreaterThanOrEqual(48));

  it.each(ids)("solves every named pose and a clip instant of %s with finite values", (id) => {
    const asset = catalogAsset(id);
    for (const pose of asset.poses.filter((p) => !p.key)) expect(finite(solveThermal(evaluate(asset, { kind: "pose", pose: pose.id })))).toBe(true);
    const clip = asset.clips[0];
    if (clip) {
      const field = solveThermalAt(asset, { kind: "clip", clip: clip.id, timeS: clip.durationS * 0.37 });
      expect(finite(field)).toBe(true);
      if (!KNOWN_COLD_SURFACE_DEFECTS.has(id)) for (const r of Object.values(field.regions)) expect(r.surfaceTempC).toBeGreaterThan(-60);
    }
  });

  it.each(ids)("keeps the body of %s at its species core temperature in the default pose", (id) => {
    const asset = catalogAsset(id);
    const body = asset.thermal.body;
    if (!body) return;
    const field = solveThermal(evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }));
    const [lo, hi] = CORE[body.species]!;
    expect(field.body!.coreTempC).toBeGreaterThan(lo);
    expect(field.body!.coreTempC).toBeLessThan(hi);
    expect(field.body!.meanSkinTempC).toBeLessThan(field.body!.coreTempC);
    for (const r of Object.values(field.regions)) if (r.skinTempC !== null) expect(r.skinTempC).toBeLessThanOrEqual(field.body!.coreTempC + 1e-9);
  });

  it.each(ids)("keeps the passive and unpowered regions of %s between the radiative extremes of the environment", (id) => {
    const asset = catalogAsset(id);
    const field = solveThermal(evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }));
    const env = field.environment;
    for (const region of asset.thermal.regions) {
      if (region.model !== "passive") continue;
      const r = field.regions[region.id]!;
      expect(r.surfaceTempC).toBeGreaterThan(Math.min(env.airTempC, env.skyTempC ?? env.airTempC) - 25);
      expect(r.surfaceTempC).toBeLessThan(env.airTempC + 45);
    }
  });
});
