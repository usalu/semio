/** 💨️ Transport view: layers are drawn separately and labelled, legends carry units and the model note, colours follow the shared ranges, and both languages are complete. */
import { describe, expect, it } from "vitest";
import { SCENARIOS, scenario } from "../../🧫️fixtures/💨️transport/🟦️.ts";
import { TRANSPORT_LAYERS, type DisplayRange, type Legend } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";
import { Canvas, Labels, type Ctx } from "../../🔨️modules/🎨️depiction/🧱️canvas/🟦️.ts";
import { THEMES } from "../../🔨️modules/🎨️depiction/🖍️palette/🟦️.ts";
import { SHARED_EXCESS_K, transportView } from "../../🔨️modules/🎨️depiction/💨️transport-view/🟦️.ts";

const s = scenario("human");
const field = solveTransport(s.asset, { kind: "pose", pose: "rest", timeS: 2.6 }, SCENARIOS.environments["calm-cold"], s.options);
const k = 10;

function draw(language: "en" | "de", layers: readonly (typeof TRANSPORT_LAYERS)[number][], range: DisplayRange = { mode: "shared" }) {
  const ctx = { state: { asset: s.asset }, options: {}, language, k, theme: "light", colours: THEMES.light, paper: (p: { x: number; y: number }) => ({ x: p.x * k, y: -p.y * k }), labels: new Labels() } as unknown as Ctx;
  const canvas = new Canvas();
  const view = transportView(ctx, field, layers, range);
  view.overlay(canvas);
  const items = canvas.finish(1).layers.flatMap((l) => l.items);
  return { items, legends: view.legends };
}
const ids = (items: { id: string }[], prefix: string) => items.filter((i) => i.id.startsWith(prefix));
const texts = (items: ReturnType<typeof draw>["items"]) => items.flatMap((i) => (i.kind === "text" ? [i.text] : []));

describe("💨️ transport view", () => {
  it("draws every layer as separate items", () => {
    const { items } = draw("en", TRANSPORT_LAYERS);
    expect(ids(items, "parcel:").length).toBe(field.parcels.length);
    expect(ids(items, "pathline:").length).toBe(field.pathlines.length);
    expect(ids(items, "streamline:").length).toBe(field.streamlines.length);
    expect(ids(items, "condensation:").length).toBe(field.condensation.length);
    expect(ids(items, "emitter:").length).toBe(field.emitters.length);
  });

  it("draws only the requested layers", () => {
    const { items, legends } = draw("en", ["condensation"]);
    expect(ids(items, "parcel:").length).toBe(0);
    expect(ids(items, "streamline:").length).toBe(0);
    expect(ids(items, "condensation:").length).toBeGreaterThan(0);
    expect(legends.map((l) => l.quantity)).toEqual(["condensate"]);
  });

  it("labels streamlines and pathlines differently, and condensation as modelled", () => {
    const en = texts(draw("en", TRANSPORT_LAYERS).items);
    expect(en).toContain("instantaneous streamlines");
    expect(en).toContain("particle pathlines");
    expect(en).toContain("modelled visible condensation");
    const de = texts(draw("de", TRANSPORT_LAYERS).items);
    expect(de).toContain("momentane Stromlinien");
    expect(de).toContain("Teilchenbahnen");
    expect(de).toContain("modellierte sichtbare Kondensation");
  });

  it("states units, the model note and complete bilingual texts in every legend", () => {
    const { legends } = draw("en", TRANSPORT_LAYERS);
    expect(legends.map((l: Legend) => l.unit).sort()).toEqual(["K", "g/kg", "m/s", "s"].sort());
    for (const l of legends) {
      for (const t of [l.title, l.convention, l.interpretation, l.conditions!]) {
        expect(t.en.length).toBeGreaterThan(5);
        expect(t.de.length).toBeGreaterThan(5);
        expect(t.de).not.toBe(t.en);
      }
      expect(l.conditions!.en).toMatch(/not CFD/i);
      expect(l.stops.length).toBeGreaterThanOrEqual(2);
    }
  });

  it("uses the shared range for temperature excess and a normalised auto range on request", () => {
    const shared = draw("en", ["parcels"]).legends.find((l) => l.quantity === "excess-temperature")!;
    expect([shared.min, shared.max]).toEqual([-SHARED_EXCESS_K, SHARED_EXCESS_K]);
    expect(shared.range).toBe("shared");
    const auto = draw("en", ["parcels"], { mode: "auto" }).legends.find((l) => l.quantity === "excess-temperature")!;
    expect(auto.range).toBe("auto");
    expect(auto.max).toBeLessThan(SHARED_EXCESS_K);
    expect(auto.interpretation.en).toMatch(/normalised/);
  });

  it("colours warm parcels warm and nearly ambient parcels near neutral", () => {
    const { items } = draw("en", ["parcels"]);
    const colour = (parcelId: string) => {
      const item = items.find((i) => i.id.startsWith(`parcel:${parcelId}`))!;
      return item.kind === "path" ? item.fill!.color : "";
    };
    const warmest = [...field.parcels].sort((a, b) => b.excessK - a.excessK)[0]!;
    const coolest = [...field.parcels].sort((a, b) => a.excessK - b.excessK)[0]!;
    expect(colour(warmest.id)).not.toBe(colour(coolest.id));
    const red = (hex: string) => Number.parseInt(hex.slice(1, 3), 16) - Number.parseInt(hex.slice(5, 7), 16);
    expect(red(colour(warmest.id))).toBeGreaterThan(red(colour(coolest.id)));
  });
});
