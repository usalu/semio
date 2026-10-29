/** 🪢️ Tied canopy: pin at the wall plus a roller along the tie-rod direction; the rod tension equals the anastruct truss axial force and the closed form.
 * @see ./🐍️.py — the anastruct oracle that writes `🧫️fixtures/🪢️tied-canopy/🔣️.json`
 * @see ./🥒️.feature — the language-agnostic specification */
import { describe, expect, it } from "vitest";
import { buildState, loadCase, near } from "../../🔨️modules/🏗️structure/🧰️harness/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";

const doc = loadCase("🪢️tied-canopy");
const oracle = doc.oracle.values as Record<string, number>;
const result = solveStructure(buildState(doc));
const rad = (oracle["rodDeg"]! * Math.PI) / 180;
const pin = result.reactions.find((r) => r.anchor === "wall-pin")!;
const rod = result.reactions.find((r) => r.anchor === "tie-rod")!;

describe("tied canopy against anastruct", () => {
  it("pulls the rod with the oracle tension, along the rod only", () => {
    const along = rod.fxN * Math.cos(rad) + rod.fyN * Math.sin(rad);
    const across = -rod.fxN * Math.sin(rad) + rod.fyN * Math.cos(rad);
    expect(near(-along, oracle["rodTensionN"]!, 1e-4)).toBe(true);
    expect(near(-along, oracle["analyticRodTensionN"]!, 1e-9)).toBe(true);
    expect(Math.abs(across)).toBeLessThan(1e-9);
  });
  it("takes the pin reaction of the oracle", () => {
    expect(near(pin.fxN, oracle["pinFxN"]!, 1e-4)).toBe(true);
    expect(near(pin.fyN, oracle["pinFyN"]!, 1e-4)).toBe(true);
  });
  it("is in equilibrium", () => {
    expect(Math.hypot(result.residual.fxN, result.residual.fyN)).toBeLessThan(1e-9 * 3000);
    expect(Math.abs(result.residual.momentNm)).toBeLessThan(1e-9 * 3000);
  });
  it("compresses the canopy by the horizontal rod component", () => {
    const interior = result.members["canopy"]!.stations.filter((s) => s.s > 1e-9 && s.s < 2 - 1e-9);
    for (const s of interior) expect(near(s.axialN, oracle["beamAxialN"]!, 1e-4)).toBe(true);
    expect(interior.every((s) => s.axialN < 0)).toBe(true);
  });
  it("has zero bending moment at both supports", () => {
    const stations = result.members["canopy"]!.stations;
    for (const s of stations.filter((x) => x.s < 1e-9)) expect(Math.abs(s.momentNm)).toBeLessThan(1e-9);
    expect(Math.abs(stations.at(-1)!.momentNm)).toBeLessThan(1e-6);
  });
});
