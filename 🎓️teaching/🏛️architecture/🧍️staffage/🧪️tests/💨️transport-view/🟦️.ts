/** 💨️ Transport view: layers are drawn separately and labelled, legends carry units and the model note, colours follow the shared ranges, and both languages are complete. */
import { describe, expect, it } from "vitest";
import { SCENARIOS, scenario } from "../../🧫️fixtures/💨️transport/🟦️.ts";
import type { Viewpoint } from "../../🧬️schema/🟦️.ts";
import { TRANSPORT_LAYERS, type DisplayRange, type Legend } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";
import { Canvas, Labels, type Ctx } from "../../🔨️modules/🎨️depiction/🧱️canvas/🟦️.ts";
import { THEMES } from "../../🔨️modules/🎨️depiction/🖍️palette/🟦️.ts";
import { SHARED_EXCESS_K, transportView } from "../../🔨️modules/🎨️depiction/💨️transport-view/🟦️.ts";

const s = scenario("human");
const field = solveTransport(s.asset, { kind: "pose", pose: "rest", timeS: 2.6 }, SCENARIOS.environments["calm-cold"], s.options);
const k = 10;

function draw(language: "en" | "de", layers: readonly (typeof TRANSPORT_LAYERS)[number][], range: DisplayRange = { mode: "shared" }, labels = true, trailS?: number, viewpoint: Viewpoint = "elevation") {
  const ctx = { state: { asset: s.asset }, options: { viewpoint, annotations: { labels }, ...(trailS === undefined ? {} : { transportStrokeMm: 0.1, transportTrailS: trailS }) }, language, k, theme: "light", colours: THEMES.light, paper: (p: { x: number; y: number }) => ({ x: p.x * k, y: -p.y * k }), labels: new Labels() } as unknown as Ctx;
  const canvas = new Canvas();
  const view = transportView(ctx, field, layers, range);
  view.overlay(canvas);
  const items = canvas.finish(1).layers.flatMap((l) => l.items);
  return { items, legends: view.legends };
}
const ids = (items: { id: string }[], prefix: string) => items.filter((i) => i.id.startsWith(prefix));
const texts = (items: ReturnType<typeof draw>["items"]) => items.flatMap((i) => (i.kind === "text" ? [i.text] : []));

describe("💨️ transport view", () => {
  for (const viewpoint of ["elevation", "front", "plan"] as const)
    it(`draws every layer as separate items in the ${viewpoint} viewpoint`, () => {
      const { items, legends } = draw("en", TRANSPORT_LAYERS, { mode: "shared" }, true, undefined, viewpoint);
      expect(ids(items, "parcel:").length).toBe(field.parcels.length);
      expect(ids(items, "pathline:").length).toBe(field.pathlines.length);
      expect(ids(items, "streamline:").length).toBe(field.streamlines.length);
      expect(ids(items, "condensation:").length).toBe(field.condensation.length);
      expect(ids(items, "emitter:").length).toBe(field.emitters.length);
      expect(JSON.stringify(legends)).toBe(JSON.stringify(draw("en", TRANSPORT_LAYERS).legends));
    });

  it("keeps only the recent trail of each pathline in fading bands when a trail is requested", () => {
    const whole = draw("en", ["pathlines"], { mode: "shared" }, false).items.filter((i) => i.kind === "path" && i.id.startsWith("pathline:"));
    const { items } = draw("en", ["pathlines"], { mode: "shared" }, false, 0.5);
    const bands = items.filter((i) => i.kind === "path" && /^pathline:.*:[0-2]$/.test(i.id));
    expect(bands.length).toBe(items.length);
    expect(bands.length).toBeGreaterThan(whole.length);
    expect(bands.length).toBeLessThanOrEqual(3 * whole.length);
    const opacity = (parcel: string, band: number) => (items.find((i) => i.id === `pathline:${parcel}:${band}`) as { stroke?: { opacity?: number } } | undefined)?.stroke?.opacity;
    const old = field.pathlines.find((line) => (line.ages[line.ages.length - 1] ?? 0) > 1.5)!;
    const fades = [0, 1, 2].map((band) => opacity(old.parcel, band)) as number[];
    expect(fades.every((value) => value > 0)).toBe(true);
    expect([...fades].sort((a, b) => a - b)).toEqual(fades);
    expect(new Set(fades).size).toBe(3);
    const young = field.pathlines.find((line) => (line.ages[line.ages.length - 1] ?? 0) > 0.15 && (line.ages[line.ages.length - 1] ?? 0) < 0.3)!;
    expect(young).toBeDefined();
    expect(opacity(young.parcel, 0)).toBeUndefined();
    expect(opacity(young.parcel, 2)).toBeGreaterThan(0);
  });

  it("draws only the requested layers", () => {
    const { items, legends } = draw("en", ["condensation"]);
    expect(ids(items, "parcel:").length).toBe(0);
    expect(ids(items, "streamline:").length).toBe(0);
    expect(ids(items, "condensation:").length).toBeGreaterThan(0);
    expect(legends.map((l) => l.quantity)).toEqual(["condensate"]);
  });

  it("draws no emitter markers and no text when labels are off, only the layers", () => {
    const on = draw("en", TRANSPORT_LAYERS).items;
    const off = draw("en", TRANSPORT_LAYERS, { mode: "shared" }, false);
    expect(ids(on, "emitter:").length).toBeGreaterThan(0);
    expect(ids(off.items, "emitter").length).toBe(0);
    expect(texts(off.items)).toEqual([]);
    expect(ids(off.items, "parcel").length).toBe(ids(on, "parcel").length);
    expect(off.legends.length).toBe(draw("en", TRANSPORT_LAYERS).legends.length);
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

  it("declares the parcel layers an explanatory visualization of invisible air and vapour, in both languages, and keeps visible droplets to the condensation layer", () => {
    const { legends } = draw("en", TRANSPORT_LAYERS);
    const parcelLegends = legends.filter((l: Legend) => l.quantity === "excess-temperature" || l.quantity === "vapour-excess");
    expect(parcelLegends.length).toBeGreaterThan(0);
    for (const l of parcelLegends) {
      expect(l.assumptions!.some((a) => /Explanatory visualization/.test(a.en) && /invisible/.test(a.en) && /not smoke or fog/.test(a.en))).toBe(true);
      expect(l.assumptions!.some((a) => /Erklärende Visualisierung/.test(a.de) && /unsichtbar/.test(a.de))).toBe(true);
    }
    expect(legends.find((l: Legend) => l.quantity === "condensate")!.interpretation.en).toMatch(/modelled/);
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

  for (const viewpoint of ["elevation", "front", "plan"] as const)
    it(`draws every layer, the emitter markers and the legends in the ${viewpoint} view`, () => {
      const { items, legends } = draw("en", TRANSPORT_LAYERS, { mode: "shared" }, true, undefined, viewpoint);
      expect(ids(items, "parcel:").length).toBe(field.parcels.length);
      expect(ids(items, "streamline:").length).toBe(field.streamlines.length);
      expect(ids(items, "emitter:").length).toBe(field.emitters.length);
      expect(legends.map((l) => l.quantity)).toEqual(draw("en", TRANSPORT_LAYERS).legends.map((l) => l.quantity));
    });

  it("projects the same parcels to the paper place of their viewpoint, relative to each other", () => {
    const [p, q] = [field.parcels[0]!, field.parcels[field.parcels.length - 1]!];
    const at = (viewpoint: Viewpoint, id: string) => {
      const item = ids(draw("en", ["parcels"], { mode: "shared" }, true, undefined, viewpoint).items, `parcel:${id}:`)[0]! as unknown as { commands: { op: string; x?: number; y?: number }[] };
      const points = item.commands.filter((c) => c.op !== "Z");
      const xs = points.map((c) => c.x!);
      const ys = points.map((c) => c.y!);
      return { x: (Math.min(...xs) + Math.max(...xs)) / 2 / k, y: -(Math.min(...ys) + Math.max(...ys)) / 2 / k };
    };
    const offset = (viewpoint: Viewpoint) => ({ x: at(viewpoint, q.id).x - at(viewpoint, p.id).x, y: at(viewpoint, q.id).y - at(viewpoint, p.id).y });
    expect(offset("elevation").x).toBeCloseTo(q.at.x - p.at.x, 6);
    expect(offset("elevation").y).toBeCloseTo(q.at.y - p.at.y, 6);
    expect(offset("front").x).toBeCloseTo(q.at.z - p.at.z, 6);
    expect(offset("front").y).toBeCloseTo(q.at.y - p.at.y, 6);
    expect(offset("plan").x).toBeCloseTo(q.at.x - p.at.x, 6);
    expect(offset("plan").y).toBeCloseTo(q.at.z - p.at.z, 6);
  });

  it("writes emitter flows with the decimal comma in German and a second hatch cue on hot and cold parcels", () => {
    const de = texts(draw("de", ["parcels"]).items).filter((t) => t.includes(" L/s"));
    expect(de.length).toBeGreaterThan(0);
    for (const t of de) expect(t).toMatch(/\d,\d/u);
    const patterns = draw("en", ["parcels"]).items.flatMap((i) => (i.kind === "path" && i.fill?.pattern !== undefined ? [i.fill.pattern] : []));
    expect(patterns.length).toBeGreaterThan(0);
    expect(patterns.every((p) => p === "hot" || p === "cold" || p === "vapour")).toBe(true);
  });
});
