/** 🫁️ Respiration: ∫ flow dt = tidal volume (exact), continuous phase over pattern changes and loop boundaries, lung volume, routing, chest motion and respiratory heat against CoolProp and the ASHRAE regression. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🫁️respiration/🔣️.json" with { type: "json" };
import { SCENARIOS } from "../../🧫️fixtures/💨️transport/🟦️.ts";
import { exhaledVolumeM3, respirationOffsets, respirationState, respiratoryHeat, routeShares } from "../../🔨️modules/💨️transport/🫁️respiration/🟦️.ts";

const human = SCENARIOS.assets.human;
const resp = human.thermal.respiration!;
const at = (pose: string, timeS: number) => ({ kind: "pose", pose, timeS }) as const;
const expected = fixture.expected as unknown as Record<string, { inhaleL: number; exhaleL: number; coolprop: { sensibleW: number; latentW: number; dryAirKgS: number }; ashrae: { sensibleW: number; latentW: number } }>;

function integrate(f: (t: number) => number, a: number, b: number, n: number): number {
  const h = (b - a) / n;
  let s = f(a) + f(b);
  for (let i = 1; i < n; i++) s += f(a + i * h) * (i % 2 === 0 ? 2 : 4);
  return (s * h) / 3;
}

describe("🫁️ respiration", () => {
  it.each(resp.patterns.map((p) => p.id))("integrates the flow of pattern %s to the tidal volume", (id) => {
    const pattern = resp.patterns.find((p) => p.id === id)!;
    const period = 60 / pattern.rateBpm;
    const flow = (t: number) => respirationState(human, at(id, t))!.flowM3S;
    const exhale = integrate((t) => Math.max(0, flow(t)), 0, period, 40000);
    const inhale = integrate((t) => Math.max(0, -flow(t)), 0, period, 40000);
    expect(exhale * 1000).toBeCloseTo(pattern.tidalVolumeL, 3);
    expect(inhale * 1000).toBeCloseTo(pattern.tidalVolumeL, 3);
    expect(exhaledVolumeM3(human, at(id, 0), 0, period) * 1000).toBeCloseTo(pattern.tidalVolumeL, 12);
    expect(expected[id]!.exhaleL).toBeCloseTo(pattern.tidalVolumeL, 10);
    expect(expected[id]!.inhaleL).toBeCloseTo(pattern.tidalVolumeL, 10);
  });

  it("adds the exhaled volumes of consecutive emission intervals up to the tidal volumes", () => {
    const q = { kind: "clip", clip: "switch", timeS: 0 } as const;
    let sum = 0;
    for (let k = 0; k < 14 * 30; k++) sum += exhaledVolumeM3(human, q, k / 30, (k + 1) / 30);
    expect(sum * 1000).toBeCloseTo(0.5 * 2 + 1.5 * 2, 9);
  });

  it("keeps lung volume continuous, zero at breath boundaries and the time derivative equal to minus the flow", () => {
    const pattern = resp.patterns[0]!;
    const period = 60 / pattern.rateBpm;
    expect(respirationState(human, at("rest", 0))!.volumeL).toBe(0);
    expect(respirationState(human, at("rest", period - 1e-9))!.volumeL).toBeCloseTo(0, 6);
    const pausePlateau = respirationState(human, at("rest", period * 0.4))!;
    expect(pausePlateau.phase).toBe("pause-after-inhale");
    expect(pausePlateau.volumeL).toBeCloseTo(pattern.tidalVolumeL, 12);
    for (let t = 0.01; t < period * 3; t += 0.0137) {
      const a = respirationState(human, at("rest", t - 1e-6))!;
      const b = respirationState(human, at("rest", t + 1e-6))!;
      expect(Math.abs((b.volumeL - a.volumeL) / 1000 / 2e-6 + respirationState(human, at("rest", t))!.flowM3S)).toBeLessThan(2e-7);
    }
  });

  it("never jumps the phase when the pattern changes and latches changes at breath boundaries", () => {
    const q = (t: number) => ({ kind: "clip", clip: "switch", timeS: t }) as const;
    let previous = respirationState(human, q(0))!;
    const changes: number[] = [];
    for (let t = 0.001; t < 30; t += 0.001) {
      const s = respirationState(human, q(t))!;
      expect(s.cycles).toBeGreaterThanOrEqual(previous.cycles);
      expect(s.cycles - previous.cycles).toBeLessThan((Math.max(s.rateBpm, previous.rateBpm) / 60) * 0.001 * 1.0001 + 1e-9);
      if (s.pattern !== previous.pattern) {
        changes.push(t);
        expect(s.cycles % 1).toBeLessThan(0.01);
      }
      previous = s;
    }
    expect(changes.map((t) => Math.round(t))).toEqual([10, 14]);
  });

  it("integrates the rate in effect: cycles equal the integral of rate/60", () => {
    const q = (t: number) => ({ kind: "clip", clip: "switch", timeS: t }) as const;
    const total = integrate((t) => respirationState(human, q(t))!.rateBpm / 60, 0, 30, 60000);
    expect(Math.abs(total - respirationState(human, q(30))!.cycles)).toBeLessThan(0.02);
  });

  it("continues the phase across loop boundaries of a looping clip on the absolute timeline", () => {
    const q = (t: number) => ({ kind: "clip", clip: "walk", timeS: t }) as const;
    let previous = respirationState(human, q(0))!.cycles;
    for (let t = 0.01; t < 40; t += 0.01) {
      const c = respirationState(human, q(t))!.cycles;
      expect(c).toBeGreaterThanOrEqual(previous);
      expect(c - previous).toBeLessThan((30 / 60) * 0.01 + 1e-9);
      previous = c;
    }
    expect(previous).toBeGreaterThan(5);
  });

  it("routes the exhaled volume by pattern", () => {
    expect(routeShares(resp, resp.patterns[0]!)).toEqual({ nose: 1, mouth: 0 });
    expect(routeShares(resp, resp.patterns[1]!)).toEqual({ nose: 0, mouth: 1 });
    expect(routeShares(resp, resp.patterns[2]!)).toEqual({ nose: 0.5, mouth: 0.5 });
    expect(routeShares({ ...resp, noseEmitter: undefined }, resp.patterns[0]!)).toEqual({ nose: 0, mouth: 1 });
  });

  it("moves chest and abdomen in proportion to the lung volume", () => {
    const full = respirationState(human, at("rest", 0.4 * 5 + 0.0))!;
    expect(full.volumeL).toBeCloseTo(0.5, 12);
    const o = respirationOffsets(human, full);
    expect(o.chest).toEqual({ angleDeg: 1.5, dy: 0.004 });
    expect(o.abdomen!.dy).toBeCloseTo(0.006, 12);
    const running = respirationState(human, at("active", 0.4 * 2 + 0.0))!;
    expect(running.volumeL).toBeCloseTo(1.5, 12);
    expect(respirationOffsets(human, running).chest!.angleDeg).toBeCloseTo(4.5, 12);
    expect(respirationOffsets(human, respirationState(human, at("rest", 0))!).chest).toEqual({ angleDeg: 0, dy: 0 });
  });

  it("returns null without respiration", () => {
    expect(respirationState(SCENARIOS.assets.machine, { kind: "pose", pose: "idle" })).toBeNull();
  });

  it.each(resp.patterns.map((p) => p.id))("matches CoolProp respiratory heat of pattern %s and the ASHRAE magnitude", (id) => {
    const pattern = resp.patterns.find((p) => p.id === id)!;
    const env = { ...human.thermal.environment, ...fixture.heat.environment };
    const ours = respiratoryHeat(resp, pattern, env);
    const e = expected[id]!;
    expect(Math.abs(ours.sensibleW / e.coolprop.sensibleW - 1)).toBeLessThan(fixture.tolerances.heatRel);
    expect(Math.abs(ours.latentW / e.coolprop.latentW - 1)).toBeLessThan(fixture.tolerances.heatRel);
    expect(Math.abs(ours.dryAirKgS / e.coolprop.dryAirKgS - 1)).toBeLessThan(fixture.tolerances.heatRel);
    if (id === "rest") {
      expect(Math.abs(ours.sensibleW / e.ashrae.sensibleW - 1)).toBeLessThan(fixture.tolerances.ashraeSensibleRel);
      expect(Math.abs(ours.latentW / e.ashrae.latentW - 1)).toBeLessThan(fixture.tolerances.ashraeLatentRel);
    }
  });
});

describe("🫁️ passengers", () => {
  const carrier = SCENARIOS.assets.carrier;
  const q = (pose: string, t: number) => ({ kind: "pose", pose, timeS: t }) as const;

  it("addresses each subject with its own pattern, phase and tidal volume", () => {
    const primary = respirationState(carrier, q("rest", 1.3))!;
    const toddler = respirationState(carrier, q("rest", 1.3), "toddler")!;
    expect(primary.rateBpm).toBe(12);
    expect(toddler.rateBpm).toBe(24);
    expect(toddler.tidalVolumeL).toBe(0.12);
    expect(toddler.cycles).toBeCloseTo(1.3 * 0.4, 12);
    expect(respirationState(carrier, q("crying", 1.3), "toddler")!.pattern).toBe("crying");
    expect(respirationState(carrier, q("crying", 1.3))!.pattern).toBe("rest");
    expect(respirationState(carrier, q("rest", 1), "nobody")).toBeNull();
  });

  it("integrates each subject to its own tidal volume", () => {
    expect(exhaledVolumeM3(carrier, q("rest", 0), 0, 2.5, "toddler") * 1000).toBeCloseTo(0.12, 12);
    expect(exhaledVolumeM3(carrier, q("rest", 0), 0, 5) * 1000).toBeCloseTo(0.5, 12);
  });
});
