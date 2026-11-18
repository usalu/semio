/** ⚡️ Total power of an asset: hand-derived vectors, a cross-check of every sum with d3-array's exact `fsum` and of the metabolic heat with mathjs, the catalog invariants and the manifest agreement.
 * @see ./🥒️.feature — the scenarios */
import { fsum } from "d3-array";
import { evaluate } from "mathjs";
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/⚡️asset-power/🔣️.json" with { type: "json" };
import manifestDocument from "../../🗂️catalog/🔣️.json" with { type: "json" };
import type { Asset, Manifest } from "../../🧬️schema/🟦️.ts";
import { catalogAsset, catalogIds } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { POWER_TOLERANCE_W, powerOf, totalPowerW, validateAsset, validateManifest } from "../../📦️packages/🟦️typescript/🟦️.ts";

const manifest = manifestDocument as unknown as Manifest;
const assets = catalogIds().map(catalogAsset);
const twoDecimals = (value: number): number => Math.round(value * 100) / 100;

const metabolicWithMathjs = (asset: Asset): number => {
  const body = asset.thermal.body!;
  const pose = asset.poses.find((candidate) => candidate.id === asset.presentation.defaultPose)!;
  return evaluate("max(basal, met * metW * area)", { basal: body.basalMetabolismW, met: pose.thermal.activityMet ?? asset.thermal.baseMet ?? 1, metW: body.metWM2, area: body.skinAreaM2 }) as number;
};

describe("⚡️ asset power", () => {
  it.each(fixture.cases.map((c) => c.asset))("%s: matches the hand-derived vector", (id) => {
    const c = fixture.cases.find((x) => x.asset === id)!;
    const power = powerOf(catalogAsset(id));
    expect(Math.abs(power.totalW - c.totalW)).toBeLessThanOrEqual(fixture.toleranceW);
    expect(power.elements.map((element) => [element.id, twoDecimals(element.powerW)])).toEqual(c.parts.map(([part, watts]) => [part, watts]));
  });

  it.each(fixture.cases.map((c) => c.asset))("%s: the total is the exact sum of its elements (d3-array fsum)", (id) => {
    const power = powerOf(catalogAsset(id));
    expect(Math.abs(power.totalW - fsum(power.elements, (element) => element.powerW))).toBeLessThan(1e-9);
    expect(totalPowerW(catalogAsset(id))).toBe(power.totalW);
  });

  it("agrees with mathjs on the metabolic heat of every body", () => {
    const bodies = assets.filter((asset) => asset.thermal.body !== undefined);
    expect(bodies.length).toBeGreaterThanOrEqual(40);
    for (const asset of bodies) {
      const element = powerOf(asset).elements.find((candidate) => candidate.kind === "body")!;
      expect(Math.abs(element.powerW - metabolicWithMathjs(asset)), asset.id).toBeLessThan(1e-9);
    }
  });

  it("adds the heat of passengers to that of their carrier", () => {
    const carriers = assets.filter((asset) => asset.thermal.passengers.length > 0);
    expect(carriers.length).toBeGreaterThanOrEqual(2);
    for (const asset of carriers) expect(powerOf(asset).elements.filter((element) => element.kind === "passenger")).toHaveLength(asset.thermal.passengers.length);
  });

  it("is never negative or non-finite", () => {
    for (const asset of assets) {
      const power = powerOf(asset);
      expect(Number.isFinite(power.totalW), asset.id).toBe(true);
      expect(power.totalW, asset.id).toBeGreaterThanOrEqual(0);
      expect(power.totalW > 0, asset.id).toBe(asset.thermal.body !== undefined || asset.thermal.passengers.length > 0 || asset.thermal.regions.some((region) => (region.ratedPowerW ?? 0) > 0));
    }
  });

  it("states a nameplate for every asset with operating heat or a heating circuit, never derived from released heat", () => {
    for (const asset of assets) {
      const heated = asset.thermal.regions.some((region) => (region.operating !== undefined && region.operating.heatW !== 0) || region.hydronic !== undefined);
      const rated = asset.thermal.regions.some((region) => region.ratedPowerW !== undefined);
      if (heated) expect(rated, `${asset.id} has operating heat but no ratedPowerW`).toBe(true);
    }
  });

  it("gives every rating a basis, and 0 W exactly when it is passive", () => {
    for (const asset of assets)
      for (const region of asset.thermal.regions) {
        if (region.ratedPowerW === undefined) continue;
        expect(region.ratedPowerBasis, `${asset.id}/${region.id}`).toBeDefined();
        expect(region.ratedPowerBasis === "passive", `${asset.id}/${region.id}`).toBe(region.ratedPowerW === 0);
      }
  });

  it("counts engines at their rated output, not their exhaust heat", () => {
    expect(totalPowerW(catalogAsset("wheel-loader"))).toBeGreaterThan(100000);
    expect(totalPowerW(catalogAsset("city-car"))).toBeGreaterThan(50000);
    expect(totalPowerW(catalogAsset("heat-pump-outdoor-unit"))).toBeLessThan(5000);
  });

  it("sorts the elements largest first", () => {
    for (const asset of assets) {
      const watts = powerOf(asset).elements.map((element) => element.powerW);
      expect(watts, asset.id).toEqual([...watts].sort((a, b) => b - a));
    }
  });

  it("keeps the manifest in step with the assets", () => {
    for (const entry of manifest.entries) {
      const asset = assets.find((candidate) => candidate.id === entry.id)!;
      expect(Math.abs(entry.powerW - totalPowerW(asset)), entry.id).toBeLessThanOrEqual(POWER_TOLERANCE_W);
    }
    expect(validateManifest(manifest, assets).filter((issue) => issue.code === "manifest-power-mismatch")).toEqual([]);
  });

  it("reports a manifest entry whose power is stale", () => {
    const stale = { ...manifest, entries: manifest.entries.map((entry) => (entry.id === "electric-kettle" ? { ...entry, powerW: 1900 } : entry)) };
    expect(validateManifest(stale, assets).filter((issue) => issue.code === "manifest-power-mismatch").map((issue) => issue.path)).toHaveLength(1);
  });

  it("refuses a rating without a basis, a basis without a rating, a negative rating and a passive rating above 0", () => {
    const base = catalogAsset("electric-kettle");
    const patch = (regionId: string, change: object): Asset => ({ ...base, thermal: { ...base.thermal, regions: base.thermal.regions.map((region) => (region.id === regionId ? { ...region, ...change } : region)) } });
    const paths = (asset: Asset): string[] => validateAsset(asset).filter((issue) => issue.code === "power-rated-invalid").map((issue) => issue.path);
    expect(paths(patch("handle", { ratedPowerW: 10 }))).toEqual(["thermal.regions.handle.ratedPowerW"]);
    expect(paths(patch("handle", { ratedPowerBasis: "passive" }))).toEqual(["thermal.regions.handle.ratedPowerW"]);
    expect(paths(patch("handle", { ratedPowerW: 5, ratedPowerBasis: "passive" }))).toEqual(["thermal.regions.handle.ratedPowerW"]);
    expect(paths(patch("water", { ratedPowerW: -1, ratedPowerBasis: "electrical-input" }))).toEqual(["thermal.regions.water.ratedPowerW"]);
    expect(paths(patch("handle", { ratedPowerW: 10, ratedPowerBasis: "electrical-input" }))).toEqual([]);
    expect(paths(patch("handle", { ratedPowerW: 0, ratedPowerBasis: "passive" }))).toEqual([]);
  });
});
