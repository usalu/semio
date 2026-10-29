/** 🎨️ Thermal legends: centred on air temperature or zero, warm above and cool below, vapour legends show the excess over ambient. */
import { describe, expect, it } from "vitest";
import human from "../../🧫️fixtures/🧍️clothed-human/🔣️.json" with { type: "json" };
import breath from "../../🧫️fixtures/🫁️breath-plume/🔣️.json" with { type: "json" };
import { thermalState, type ThermalFixture } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { plumeLegend, solveThermal, thermalColour, thermalLegend } from "../../🔨️modules/🌡️thermal/🟦️.ts";

const field = solveThermal(thermalState(human as unknown as ThermalFixture));

describe("🎨️ thermal legend", () => {
  it("centres surface temperature on the air temperature and fluxes on zero", () => {
    const t = thermalLegend("surface-temperature", field, { mode: "auto" });
    expect(t.centre).toBe(20);
    expect((t.min + t.max) / 2).toBeCloseTo(20, 9);
    expect(t.unit).toBe("°C");
    expect(thermalLegend("net-flux", field, { mode: "auto" }).centre).toBe(0);
  });

  it("paints warm above and cool below the centre", () => {
    const l = thermalLegend("surface-temperature", field, { mode: "fixed", min: 10, max: 40 });
    expect(thermalColour(l, 40)).toBe("#d7301f");
    expect(thermalColour(l, 10)).toBe("#2c7bb6");
    expect(thermalColour(l, 20)).toBe("#e6e6e6");
    expect(l.stops[0]!.at).toBe(0);
    expect(l.stops[l.stops.length - 1]!.at).toBe(1);
  });

  it("shows the vapour excess over ambient, not the absolute humidity", () => {
    const f = solveThermal(thermalState(breath as unknown as ThermalFixture, { timeS: 2.1 }));
    const l = plumeLegend("airflow", f, { mode: "auto" });
    expect(l.min).toBe(0);
    expect(l.unit).toBe("K");
    const v = plumeLegend("vapour", { ...f, parcels: f.parcels.map((p) => ({ ...p, carries: "vapour" as const })) }, { mode: "auto" });
    expect(v.unit).toBe("g/kg");
    expect(v.max).toBeGreaterThan(0.5);
    expect(v.max).toBeLessThan(30);
    expect(v.interpretation.en).toContain("water vapour");
  });
});
