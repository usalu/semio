/** 🫁️ Catalog rules of the thermal staffage: adults at rest breathe 12 to 16 times a minute with a longer exhalation, breath emitters sit on head landmarks of the rig, trees transpire (vapour, latent heat) instead of plume, equipment emitters name declared operating keys. */
import { describe, expect, it } from "vitest";
import { catalogAsset, catalogIds } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";

const assets = catalogIds().map(catalogAsset);
const humans = assets.filter((a) => a.thermal.body?.species === "human");
const adults = humans.filter((a) => a.thermal.body!.massKg >= 40);
const RESTING_MET = 1.2;

describe("🫁️ breathing of the catalog", () => {
  it("has human assets to check", () => {
    expect(humans.length).toBeGreaterThanOrEqual(25);
    expect(adults.length).toBeGreaterThanOrEqual(24);
  });

  it.each(adults.map((a) => a.id))("%s: every resting pose (<= 1.2 met) breathes 12 to 16 times a minute", (id) => {
    const asset = adults.find((a) => a.id === id)!;
    const patterns = asset.thermal.respiration!.patterns;
    for (const pose of asset.poses) {
      const met = pose.thermal.activityMet;
      if (met === undefined || met > RESTING_MET) continue;
      const pattern = patterns.find((p) => p.id === (pose.thermal.breathing ?? patterns[0]!.id))!;
      expect(pattern.rateBpm, `${pose.id} (${met} met) uses ${pattern.id}`).toBeGreaterThanOrEqual(12);
      expect(pattern.rateBpm, `${pose.id} (${met} met) uses ${pattern.id}`).toBeLessThanOrEqual(16);
    }
  });

  it.each(humans.map((a) => a.id))("%s: patterns exhale longer than they inhale and sum to one cycle", (id) => {
    const asset = humans.find((a) => a.id === id)!;
    for (const p of asset.thermal.respiration!.patterns) {
      expect(p.exhaleFraction, p.id).toBeGreaterThanOrEqual(p.inhaleFraction);
      expect(p.inhaleFraction + p.pauseAfterInhaleFraction + p.exhaleFraction + p.pauseAfterExhaleFraction, p.id).toBeCloseTo(1, 9);
    }
  });

  it.each(humans.map((a) => a.id))("%s: nose and mouth emitters are breath emitters on head anchors of the rig", (id) => {
    const asset = humans.find((a) => a.id === id)!;
    const bodies = [{ respiration: asset.thermal.respiration!, head: true }, ...asset.thermal.passengers.flatMap((p) => (p.respiration === undefined ? [] : [{ respiration: p.respiration, head: false }]))];
    for (const { respiration, head } of bodies)
      for (const emitterId of [respiration.noseEmitter, respiration.mouthEmitter]) {
        const emitter = asset.thermal.emitters.find((e) => e.id === emitterId);
        expect(emitter?.kind, `${emitterId}`).toBe("breath");
        const anchor = asset.anchors.find((a) => a.id === emitter!.anchor);
        expect(anchor, `anchor ${emitter!.anchor}`).toBeDefined();
        expect(asset.rig.bones.some((b) => b.id === anchor!.bone), `bone ${anchor!.bone}`).toBe(true);
        if (head) expect(anchor!.bone).toMatch(/head/);
      }
  });
});

describe("🌳 vapour and heat sources of the catalog", () => {
  const trees = assets.filter((a) => a.thermal.regions.some((r) => r.model === "leaf"));

  it.each(trees.map((a) => a.id))("%s: leaves release vapour by transpiration emitters and never as a hot plume", (id) => {
    const asset = trees.find((a) => a.id === id)!;
    const leaves = new Set(asset.thermal.regions.filter((r) => r.model === "leaf").map((r) => r.id));
    const transpiration = asset.thermal.emitters.filter((e) => e.kind === "transpiration");
    expect(transpiration.length).toBeGreaterThan(0);
    for (const e of transpiration) expect(leaves.has(e.region!), e.id).toBe(true);
    for (const e of asset.thermal.emitters.filter((x) => x.kind === "plume")) expect(leaves.has(e.region ?? ""), `${e.id} plume over a leaf region`).toBe(false);
  });

  it("names a declared operating key for every equipment emitter that carries one and a real region for every plume or vapour source", () => {
    for (const asset of assets) {
      const keys = new Set(asset.thermal.operating.map((o) => o.id));
      const regions = new Set(asset.thermal.regions.map((r) => r.id));
      for (const e of asset.thermal.emitters) {
        if (e.operatingKey !== undefined) expect(keys.has(e.operatingKey), `${asset.id}/${e.id}`).toBe(true);
        if (e.region !== undefined) expect(regions.has(e.region), `${asset.id}/${e.id}`).toBe(true);
        if (e.kind === "exhaust" || e.kind === "flue" || e.kind === "fan") expect(e.operatingKey, `${asset.id}/${e.id}`).toBeDefined();
      }
    }
  });
});
