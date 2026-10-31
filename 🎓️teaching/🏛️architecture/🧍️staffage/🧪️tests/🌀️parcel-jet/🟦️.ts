/** 🌀️ Parcel model: the isothermal momentum jet follows the closed-form top-hat round-jet law, and jet, hot jet and condensing puff match scipy's re-integration of the parcel ODE. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🌀️parcel-jet/🔣️.json" with { type: "json" };
import { SCENARIOS, scenario } from "../../🧫️fixtures/💨️transport/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";

type Row = { ageS: number; x: number; y: number; ux: number; uy: number; radiusM: number; excessK: number; vapourExcessKgPerKg: number; condensateKgPerKg: number };
const expected = fixture.expected as unknown as Record<string, Row[]>;
const calm = SCENARIOS.environments["calm-cold"];
const BIRTH_S = 4 / 120;

function parcelAt(caseId: string, ageS: number) {
  const c = fixture.cases.find((x) => x.id === caseId)!;
  const base = scenario("machine", { operating: { engine: [[0, 1]] } });
  const asset = { ...base.asset, thermal: { ...base.asset.thermal, emitters: [{ ...base.asset.thermal.emitters[0]!, ...c.emitter }] } };
  const field = solveTransport(asset, { kind: "pose", pose: "idle", timeS: BIRTH_S + ageS }, calm, { ...base.options, turbulence: fixture.turbulence, streamlines: false });
  const parcel = field.parcels.find((p) => p.id === "exhaust#0");
  if (parcel === undefined) throw new Error(`parcel exhaust#0 not alive at age ${ageS}`);
  return { parcel, asset };
}

describe("🌀️ parcel jet", () => {
  it("follows the closed-form round-jet law u = U0/(1 + 2αx/r0) and V ∝ b", () => {
    const c = fixture.cases[0]!;
    const r0 = c.emitter.diameterM / 2;
    const u0 = c.emitter.volumeFlowM3S / (Math.PI * r0 * r0);
    for (const age of c.ages) {
      const { parcel } = parcelAt("momentum-jet", age);
      const x = Math.abs(parcel.at.x - 1.8);
      const analytic = (r0 / (2 * fixture.alphaJet)) * (Math.sqrt(1 + (4 * fixture.alphaJet * u0 * age) / r0) - 1);
      expect(Math.abs(x / analytic - 1)).toBeLessThan(fixture.tolerances.analyticRelative);
      const speed = Math.hypot(parcel.velocity.x, parcel.velocity.y);
      expect(Math.abs(speed / (u0 / (1 + (2 * fixture.alphaJet * analytic) / r0)) - 1)).toBeLessThan(fixture.tolerances.analyticRelative);
      expect(Math.abs(parcel.radiusM / (r0 + 2 * fixture.alphaJet * analytic) - 1)).toBeLessThan(fixture.tolerances.analyticRelative);
    }
  });

  it.each(fixture.cases.map((c) => c.id))("matches the solve_ivp re-integration of %s", (id) => {
    const rows = expected[id]!;
    for (const row of rows) {
      const { parcel } = parcelAt(id, row.ageS);
      const rel = fixture.tolerances.relative;
      expect(Math.abs(parcel.at.x - row.x)).toBeLessThan(fixture.tolerances.positionM);
      expect(Math.abs(parcel.at.y - row.y)).toBeLessThan(fixture.tolerances.positionM);
      expect(Math.abs(parcel.radiusM / row.radiusM - 1)).toBeLessThan(rel);
      expect(Math.abs(parcel.excessK - row.excessK)).toBeLessThan(fixture.tolerances.excessK);
      expect(Math.abs(parcel.velocity.x - row.ux)).toBeLessThan(rel * Math.max(1, Math.abs(row.ux)));
      expect(Math.abs(parcel.condensateKgPerKg - row.condensateKgPerKg) * 1000).toBeLessThan(fixture.tolerances.condensateGKg);
    }
  });

  it("condenses the exhaled-like puff and lets the condensate vanish by dilution", () => {
    const early = parcelAt("condensing-puff", 0.3).parcel;
    expect(early.condensateKgPerKg).toBeGreaterThan(1e-4);
    const late = expected["condensing-puff"]!.at(-1)!;
    expect(late.condensateKgPerKg).toBeLessThan(early.condensateKgPerKg);
  });
});
